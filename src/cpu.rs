use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{Byte, Word, MEMORY_RANGE, STACK_HIGH};

#[derive(Debug)]
pub struct MEMORY {
    pub data: [Byte; MEMORY_RANGE],
}
impl MEMORY {
    pub fn new() -> Self {
        Self {
            data: [0; 1024 * 64],
        }
    }

    pub fn reset(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = 0x00;
        }
    }
    // returns byte from 16bit address range
    pub fn get_byte(&self, address: Word) -> Byte {
        self.data[address as usize]
    }
    // sets byte at 16bit address range
    pub fn set_byte(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }
}

#[allow(dead_code)]

pub struct Status {
    pub n: bool, //negative
    pub v: bool, //overflow
    pub u: bool, //unused
    pub b: bool, //break
    pub d: bool, //decimal
    pub i: bool, //interrupt
    pub z: bool, //zero
    pub c: bool, //carry
}

impl Default for Status {
    fn default() -> Self {
        Self {
            n: false,
            v: false,
            u: false,
            b: false,
            d: false,
            i: false,
            z: false,
            c: false,
        }
    }
}

impl Status {
    // bitshifter's paradise
    pub fn to_byte(&self) -> Byte {
        let mut byte = 0x00;
        byte |= (self.n as u8) << 0;
        byte |= (self.v as u8) << 1;
        byte |= (self.u as u8) << 2;
        byte |= (self.b as u8) << 3;
        byte |= (self.d as u8) << 4;
        byte |= (self.i as u8) << 5;
        byte |= (self.z as u8) << 6;
        byte |= (self.c as u8) << 7;
        byte
    }
}

#[allow(dead_code)]
pub struct CPU {
    pub acc: Byte, //accumulator
    pub x: Byte,   //index
    pub y: Byte,   //index

    pub stkptr: Word,
    pub prgmctr: Word,

    pub status: Status,
}

#[derive(Clone, Copy)]
enum InstructionArgs {
    None,
    OneByte(Byte),
    TwoByte(Byte, Byte),
    Address(Word),
}

type InstructionHandler = fn(&mut CPU, &mut MEMORY, Byte, Byte);
trait InstructionExecutor<T> {
    fn execute_with_args(&mut self, memory: &mut MEMORY, args: T);
}

impl InstructionExecutor<InstructionArgs> for CPU {
    fn execute_with_args(&mut self, memory: &mut MEMORY, args: InstructionArgs) {
        match args {
            InstructionArgs::None => {}
            InstructionArgs::OneByte(byte) => {}
            InstructionArgs::TwoByte(byte1, byte2) => {}
            InstructionArgs::Address(address) => {}
        }
    }
}

fn generic_handler<F, T>(
    cpu: &mut CPU,
    memory: &mut MEMORY,
    operand1: Byte,
    operand2: Byte,
    executor: F,
    arg_transformer: fn(Byte, Byte) -> T,
) where
    F: Fn(&mut CPU, &mut MEMORY, T),
{
    let args = arg_transformer(operand1, operand2);
    executor(cpu, memory, args);
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        //sets registers to 0 aside from stack pointer
        //sets stack pointer to 0x01FF
        Self {
            acc: Byte::default(),
            x: Byte::default(),
            y: Byte::default(),
            stkptr: STACK_HIGH,
            prgmctr: Word::default(),
            status: Status::default(),
        }
    }

    const INSTRUCTION_TABLE: [Option<InstructionHandler>; 256] = {
        let mut table = [None; 256];

        // LDA variants
        table[0xA9] = Some(CPU::handle_lda_immediate as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xAD] = Some(CPU::handle_lda_absolute as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xBD] = Some(CPU::handle_lda_absolute_x as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xB9] = Some(CPU::handle_lda_absolute_y as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xA5] = Some(CPU::handle_lda_zero_page as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xB5] = Some(CPU::handle_lda_zero_page_x as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xA1] =
            Some(CPU::handle_lda_indexed_indirect as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xB1] =
            Some(CPU::handle_lda_indirect_indexed as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xA2] = Some(CPU::handle_ldx_immediate as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xA0] = Some(CPU::handle_ldy_immediate as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x4C] = Some(CPU::handle_jmp_absolute as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x48] = Some(CPU::handle_pha as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x68] = Some(CPU::handle_pla as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xEA] = Some(CPU::handle_nop as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x08] = Some(CPU::handle_php as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x28] = Some(CPU::handle_plp as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xAA] = Some(CPU::handle_tax as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x8A] = Some(CPU::handle_txa as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xCA] = Some(CPU::handle_dex as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xE8] = Some(CPU::handle_inx as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x98] = Some(CPU::handle_tya as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x88] = Some(CPU::handle_dey as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xC8] = Some(CPU::handle_iny as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x86] = Some(CPU::handle_stx as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x84] = Some(CPU::handle_sty as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x85] = Some(CPU::handle_sta as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x00] = Some(CPU::handle_brk as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0x9A] = Some(CPU::handle_txs as fn(&mut CPU, &mut MEMORY, Byte, Byte));
        table[0xBA] = Some(CPU::handle_tsx as fn(&mut CPU, &mut MEMORY, Byte, Byte));

        table
    };

    pub fn reset(&mut self) {
        self.acc = Byte::default();
        self.x = Byte::default();
        self.y = Byte::default();

        self.stkptr = STACK_HIGH; // all for the stack !!!
        self.prgmctr = Word::default();

        self.status.v = bool::default();
        self.status.n = bool::default();
        self.status.c = bool::default();
        self.status.z = bool::default();
        self.status.i = bool::default();
        self.status.d = bool::default();
        self.status.b = bool::default();
    }

    fn handle_lda_immediate(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.lda(operand1);
    }
    fn handle_lda_absolute(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        let concat_byte = make_address(operand1, operand2);
        let data = memory.get_byte(concat_byte);
        cpu.lda(data);
    }

    fn handle_lda_absolute_x(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        let concat_byte = make_address(operand1, operand2) + cpu.x as u16;
        let data = memory.get_byte(concat_byte);
        cpu.lda(data);
    }

    fn handle_lda_absolute_y(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        let concat_byte = make_address(operand1, operand2) + cpu.y as u16;
        let data = memory.get_byte(concat_byte);
        cpu.lda(data);
    }

    fn handle_lda_zero_page(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        let data = memory.get_byte(make_address(0x00, operand1));
        cpu.lda(data);
    }

    fn handle_lda_zero_page_x(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        let data = memory.get_byte(make_address(0x00, operand1) + cpu.x as u16);
        cpu.lda(data);
    }
    fn handle_lda_indexed_indirect(
        cpu: &mut CPU,
        memory: &mut MEMORY,
        operand1: Byte,
        operand2: Byte,
    ) {
        cpu.x = cpu.x + operand1;
        let new_operand1 = memory.get_byte(make_address(0x00, cpu.x));
        let new_operand2 = memory.get_byte(make_address(0x00, operand2));
        let data = memory.get_byte(make_address(new_operand1, new_operand2));
        cpu.lda(data);
    }

    fn handle_lda_indirect_indexed(
        cpu: &mut CPU,
        memory: &mut MEMORY,
        operand1: Byte,
        operand2: Byte,
    ) {
        cpu.y = cpu.y + operand1;
        let new_operand1 = memory.get_byte(make_address(0x00, operand1));
        let new_operand2 = memory.get_byte(make_address(0x00, operand2));
        let data = memory.get_byte(make_address(new_operand1, new_operand2));
        cpu.lda(data);
    }

    fn handle_ldx_immediate(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.ldx(operand1);
    }

    fn handle_ldy_immediate(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.ldy(operand1);
    }

    // loads byte into accumulator
    pub fn lda(&mut self, data: Byte) {
        self.prgmctr += 2;
        self.acc = data;
        self.status.n = true;
        self.status.z = true;
    }
    // direct
    pub fn push(&mut self, memory: &mut MEMORY, data: Byte) {
        memory.set_byte(self.stkptr, data);
        self.stkptr -= 1;
        //self.stkptr = xextend(data);
    }

    pub fn pull(&mut self, memory: &mut MEMORY) -> Byte {
        self.stkptr += 1;
        memory.get_byte(self.stkptr)
    }

    fn handle_jmp_absolute(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        let address = make_address(operand1, operand2);
        cpu.jmp(address);
    }

    pub fn jmp(&mut self, data: Word) {
        self.prgmctr = data;
    }

    fn handle_pha(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.pha(memory);
    }

    // push accumulator
    pub fn pha(&mut self, memory: &mut MEMORY) {
        self.prgmctr += 1;
        self.push(memory, self.acc)
    }

    fn handle_pla(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.pla(memory);
    }

    // pull accumulator
    pub fn pla(&mut self, memory: &mut MEMORY) -> Byte {
        self.acc = self.pull(memory);
        self.prgmctr += 1;
        self.acc
    }

    fn handle_nop(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.nop();
    }

    // no operation
    pub fn nop(&mut self) {
        self.prgmctr += 1;
    }

    fn handle_php(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.php(operand1);
    }

    // push processor status
    pub fn php(&mut self, data: Byte) {
        self.status.n = (data & 0b0000_0001) != 0;
        self.status.v = (data & 0b0000_0010) != 0;
        self.status.u = (data & 0b0000_0100) != 0;
        self.status.b = (data & 0b0000_1000) != 0;
        self.status.d = (data & 0b0001_0000) != 0;
        self.status.i = (data & 0b0010_0000) != 0;
        self.status.z = (data & 0b0100_0000) != 0;
        self.status.c = (data & 0b1000_0000) != 0;
        self.prgmctr += 1;
    }

    fn handle_plp(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.plp();
    }

    // pull processor status
    pub fn plp(&mut self) -> Byte {
        self.status.to_byte()
    }

    fn handle_txs(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.txs();
    }

    // transfer accumulator to x
    pub fn txs(&mut self) {
        self.stkptr = xextend(self.x);
    }

    fn handle_tsx(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.tsx();
    }

    // transfer stack pointer to x
    pub fn tsx(&mut self) {
        self.prgmctr += 1;
        self.x = split_address(self.stkptr).1;
    }

    fn handle_tax(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.tax();
    }

    // transfer accumulator to x
    pub fn tax(&mut self) {
        self.prgmctr += 1;
        self.x = self.acc;
    }

    fn handle_txa(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.txa();
    }

    // transfer x to accumulator
    pub fn txa(&mut self) {
        self.prgmctr += 1;
        self.acc = self.x;
    }

    // transfer y to accumulator
    pub fn tya(&mut self) {
        self.prgmctr += 1;
        self.acc = self.y;
    }

    fn handle_tya(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.tya();
    }

    // transfer accumulator to y
    pub fn tay(&mut self) {
        self.prgmctr += 1;
        self.y = self.acc;
    }
    // decrement y

    fn handle_dey(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.dey();
    }

    pub fn dey(&mut self) {
        self.prgmctr += 1;
        self.y -= 1;
    }

    fn handle_iny(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.iny();
    }

    // increment y
    pub fn iny(&mut self) {
        self.prgmctr += 1;
        self.y += 1;
    }

    fn handle_inx(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.inx();
    }

    // increment x
    pub fn inx(&mut self) {
        self.prgmctr += 1;
        self.x += 1;
    }

    fn handle_dex(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.dex();
    }

    // decrement x
    pub fn dex(&mut self) {
        self.prgmctr += 1;
        self.x -= 1;
    }

    fn handle_brk(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.brk();
    }

    // brk
    pub fn brk(&mut self) {
        self.prgmctr += 1;
        self.status.b = true;
    }

    // load x
    pub fn ldx(&mut self, data: Byte) {
        self.status.n = true;
        self.status.z = true;
        self.x = data;
        self.prgmctr += 2;
    }
    // load y
    pub fn ldy(&mut self, data: Byte) {
        self.status.n = true;
        self.status.z = true;
        self.y = data;
        self.prgmctr += 2;
    }

    fn handle_stx(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.stx(memory);
    }

    fn handle_sty(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.sty(memory);
    }

    // store x 0x86
    pub fn stx(&mut self, memory: &mut MEMORY) {
        memory.set_byte(self.prgmctr, self.x);
        self.prgmctr += 2;
    }

    //store y 0x84
    pub fn sty(&mut self, memory: &mut MEMORY) {
        memory.set_byte(self.prgmctr, self.y);
        self.prgmctr += 2;
    }

    fn handle_sta(cpu: &mut CPU, memory: &mut MEMORY, operand1: Byte, operand2: Byte) {
        cpu.sta(memory);
    }

    pub fn sta(&mut self, memory: &mut MEMORY) {
        memory.set_byte(self.prgmctr, self.acc);
    }

    pub fn execute_table(&mut self, m: &mut MEMORY) -> Option<Byte> {
        let instruction = m.get_byte(self.prgmctr);
        let operand1 = m.get_byte(self.prgmctr + 1);
        let operand2 = m.get_byte(self.prgmctr + 2);

        if let Some(handler) = CPU::INSTRUCTION_TABLE[instruction as usize] {
            handler(self, m, operand1, operand2);

            None
        } else {
            None
        }
    }

    // executes and returms an option of the data depending on the instruction
    pub fn execute(&mut self, m: &mut MEMORY) -> Option<Byte> {
        let instruction = m.get_byte(self.prgmctr);
        let operand1 = m.get_byte(self.prgmctr + 1);
        let operand2 = m.get_byte(self.prgmctr + 2);
        match instruction {
            // lda block
            // lda immediate
            0xA9 => {
                self.lda(operand1);

                None
            }
            // lda absolute
            0xAD => {
                let concat_byte = make_address(operand1, operand2);
                let data = m.get_byte(concat_byte);

                self.lda(data);
                None
            }
            // lda x indexed
            0xBD => {
                let concat_byte = make_address(operand1, operand2) + self.x as u16;
                let data = m.get_byte(concat_byte);

                self.lda(data);
                None
            }
            // lda y indexed
            0xB9 => {
                let concat_byte = make_address(operand1, operand2) + self.y as u16;
                let data = m.get_byte(concat_byte);

                self.lda(data);
                None
            }
            // lda zp
            0xA5 => {
                let data = m.get_byte(make_address(0x00, operand1));

                self.lda(data);
                None
            }
            // lda zpx
            0xB5 => {
                let data = m.get_byte(make_address(0x00, operand1) + self.x as u16);

                None
            }
            // lda x zp indexed indirect
            0xA1 => {
                self.x = self.x + operand1;
                let new_operand1 = m.get_byte(make_address(0x00, self.x));
                let new_operand2 = m.get_byte(make_address(0x00, operand2));
                let data = m.get_byte(make_address(new_operand1, new_operand2));
                self.lda(data);

                None
            }

            // lda y zp indirect indexed
            0xB1 => {
                self.y = self.y + operand1;
                let new_operand1 = m.get_byte(make_address(0x00, operand1));
                let new_operand2 = m.get_byte(make_address(0x00, operand2));
                let data = m.get_byte(make_address(new_operand1, new_operand2));
                self.lda(data);

                None
            }
            // ldx
            0xA2 => {
                self.ldx(operand1);
                None
            }
            // ldy
            0xA0 => {
                self.ldy(operand1);

                None
            }
            // jump
            0x4C => {
                self.jmp(make_address(operand1, operand2));

                None
            }
            // pha
            0x48 => {
                self.pha(m);
                None
            }
            // pla
            0x68 => {
                self.pla(m);
                Some(self.acc)
            }
            // txs
            0x9A => {
                self.txs();
                None
            }
            // tsx
            0xBA => {
                self.tsx();
                None
            }
            // nop
            0xEA => {
                self.nop();
                None
            }
            // php
            0x08 => {
                self.php(operand1);
                None
            }
            // plp
            0x28 => Some(self.plp()),
            // tax
            0xAA => {
                self.tax();
                None
            }
            // txa
            0x8A => {
                self.txa();
                None
            }
            // dex
            0xCA => {
                self.dex();
                None
            }
            // inx
            0xE8 => {
                self.inx();
                None
            }
            // tay
            0xA8 => {
                self.tay();
                None
            }
            // tya
            0x98 => {
                self.tya();
                None
            }
            // dey
            0x88 => {
                self.dey();
                None
            }
            // iny
            0xC8 => {
                self.iny();
                None
            }
            0x86 => {
                self.stx(m);
                None
            }
            0x84 => {
                self.sty(m);
                None
            }

            0x85 => {
                self.sta(m);
                None
            }

            0x00 => {
                self.brk();
                None
            }

            _ => None,
        }
    }
    fn set_ctr(&mut self, value: Word) {
        self.prgmctr = value;
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MEMORY {
    fn default() -> Self {
        Self::new()
    }
}

pub fn xextend(x: u8) -> u16 {
    u16::from(x)
}

pub fn make_address(o1: Byte, o2: Byte) -> Word {
    let address: Word = ((o1 as Word) << 8) | o2 as Word;
    address
}

// splits u16 into u8 tuple
pub fn split_address(addr: Word) -> (Byte, Byte) {
    let high_byte: Byte = (addr >> 8) as Byte;
    let low_byte: Byte = addr as Byte;

    // big endian
    (high_byte, low_byte)
}

pub fn save_memory(mem: &MEMORY, file: &str) {
    let mut file = File::create(file).unwrap();
    file.write_all(&mem.data).unwrap();
}

pub fn load_memory(mem: &mut MEMORY, file: &str) {
    let mut file = File::open(file).unwrap();
    file.read_exact(&mut mem.data).unwrap();
}

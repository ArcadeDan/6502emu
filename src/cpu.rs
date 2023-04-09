use std::{fs::File, io::{Write, Read}};

use crate::{MEMORY_RANGE, Byte, Word, STACK_HIGH};


#[allow(dead_code)]
pub enum AddressingModes {
    Accumulator,
    Immediate,
    Implied,
    Relative,
    ZeroPage,
    Indirect,
    Absolute,
}



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
    ///returns byte from 16bit address range
    pub fn get_byte(&self, address: Word) -> Byte {
        self.data[address as usize]
    }
    // sets byte at 16bit address range
    pub fn set_byte(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    // soon to be deprecated
    pub fn set_bytes(&mut self, start: Word, values: &[Byte]) {
        let start = start as usize;
        let end = start + values.len();
        self.data[start..end].copy_from_slice(values);
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
    pub mode: AddressingModes,
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
            mode: AddressingModes::default(),
        }
    }

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

    pub fn jmp(&mut self, data: Word) {
        self.prgmctr = data;
    }
    // push accumulator
    pub fn pha(&mut self, memory: &mut MEMORY) {
        self.prgmctr += 1;
        self.push(memory, self.acc)
    }
    // pull accumulator
    pub fn pla(&mut self, memory: &mut MEMORY) -> Byte {
        self.acc = self.pull(memory);
        self.prgmctr += 1;
        self.acc
    }
    // no operation
    pub fn nop(&mut self) {
        self.prgmctr += 1;
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
    // pull processor status
    pub fn plp(&mut self) -> Byte {
        self.status.to_byte()
    }
    // transfer accumulator to x
    pub fn txs(&mut self) {
        self.stkptr = xextend(self.x);
    }
    // transfer stack pointer to x
    pub fn tsx(&mut self) {
        self.prgmctr += 1;
        self.x = split_address(self.stkptr).1;
    }
    // transfer accumulator to x
    pub fn tax(&mut self) {
        self.prgmctr += 1;
        self.x = self.acc;
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
    // transfer accumulator to y
    pub fn tay(&mut self) {
        self.prgmctr += 1;
        self.y = self.acc;
    }
    // decrement y
    pub fn dey(&mut self) {
        self.prgmctr += 1;
        self.y -= 1;
    }
    // increment y
    pub fn iny(&mut self) {
        self.prgmctr += 1;
        self.y += 1;
    }
    // increment x
    pub fn inx(&mut self) {
        self.prgmctr += 1;
        self.x += 1;
    }
    // decrement x
    pub fn dex(&mut self) {
        self.prgmctr += 1;
        self.x -= 1;
    }

    // executes and returms an option of the data depending on the instruction
    pub fn execute(&mut self, m: &mut MEMORY) -> Option<Byte> {
        let instruction = m.get_byte(self.prgmctr);
        let operand1 = m.get_byte(self.prgmctr + 1);
        let operand2 = m.get_byte(self.prgmctr + 2);
        match instruction {
            // lda
            0xA9 => {
                self.lda(operand1);
                self.mode = AddressingModes::Accumulator;
                None
            }
            // jump
            0x4C => {
                self.jmp(make_address(operand1, operand2));
                self.mode = AddressingModes::Absolute;
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

#[allow(dead_code)]
enum Instruction {
    ADC, // add w/ carry
    AND, // logical AND
    ASL, // arithmetic shift left
    BIT, // bit test
    BRK, // break
    CMP, // compare
    CPY, // compare Y register
    CPX, // compare x register
    DEC, // decrement memory
    EOR, // exclusive or
    CLC, // clear carry flag
    SEC, // set carry flag
    CLI, // clear interrupt disable
    SEI, // set interrupt disable
    CLV, // clear overflow flag
    CLD, // clear decimal mode
    SED, // set decimal flag
    INC, // increment
    JMP, // jump
    JSR, // jump to subroutine
    LDA, // load accumulator
    LDX, // load x register
    LDY, // load y register
    LSR, // logical shift right
    NOP, // no operation
    ORA, // inclusive or
    TAX, // transfer accumulator to x
    TXA, // transfer x to accumulator
    DEX, // decrement X register
    INX, // increment x register
    TAY, // transfer accumulator to y
    DEY, // decrement y register
    INY, // increment y register
    ROL, // rotate left
}

impl Default for AddressingModes {
    fn default() -> Self {
        AddressingModes::Accumulator
    }
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
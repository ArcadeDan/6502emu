use std::{
    io::{stdin, stdout, BufRead, Write},
    ops::Add,
    process::exit,
};

use logos::Logos;
use std::fs;
use std::fs::File;
use std::io::Read;

type Byte = u8;
type Word = u16;

const ADDRESS_LOW: u16 = 0x0000;
const ADDRESS_HIGH: u16 = 0xFFFF;
const MEMORY_RANGE: usize = (ADDRESS_HIGH - ADDRESS_LOW) as usize + 1;

const STACK_LOW: u16 = 0x0100;
const STACK_HIGH: u16 = 0x01FF;

#[derive(Logos, Debug, PartialEq, Clone)]
enum InterpreterInstr {
    // utility
    #[token("reg")]
    Registers,
    #[token("reset")]
    Reset,
    #[token("exit")]
    Exit,
    #[token("status")]
    Status,
    #[token("set_ctr")]
    SetCounter,
    #[token("dump")]
    Dump,
    #[token("load")]
    Load,
    // data
    #[regex(r"(0x+[A-Z \d])\w+")]
    HexValue,

    // instructions
    #[token("setbyte")]
    SetByte,
    #[token("setbytes")]
    SetBytes,
    #[token("getbyte")]
    GetByte,
    #[token("getbytes")]
    GetBytes,
    #[token("jmp")]
    Jump,
    #[token("push")]
    Push,
    #[token("pull")]
    Pull,
    #[token("pha")]
    PushAccumulator,
    #[token("lda")]
    LoadAccumulator,
    #[token("execute")]
    Execute,
    #[error]
    #[regex(r"[\t\n\f ]+", logos::skip)]
    ERROR,
}

fn xextend(x: u8) -> u16 {
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
#[allow(dead_code)]
enum AddressingModes {
    Accumulator,
    Immediate,
    Implied,
    Relative,
    ZeroPage,
    Indirect,
    Absolute,
}

impl Default for AddressingModes {
    fn default() -> Self {
        AddressingModes::Accumulator
    }
}

/// A fixed-size of 65535 bytes
#[derive(Debug)]
struct MEMORY {
    data: [Byte; MEMORY_RANGE],
}
impl MEMORY {
    fn new() -> Self {
        Self {
            data: [0; 1024 * 64],
        }
    }

    fn reset(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = 0x00;
        }
    }
    ///returns byte from 16bit address range
    fn get_byte(&self, address: Word) -> Byte {
        self.data[address as usize]
    }
    // sets byte at 16bit address range
    fn set_byte(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    // soon to be deprecated
    fn set_bytes(&mut self, start: Word, values: &[Byte]) {
        let start = start as usize;
        let end = start + values.len();
        self.data[start..end].copy_from_slice(values);
    }
}

#[allow(dead_code)]

struct Status {
    n: bool, //negative
    v: bool, //overflow
    u: bool, //unused
    b: bool, //break
    d: bool, //decimal
    i: bool, //interrupt
    z: bool, //zero
    c: bool, //carry
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
    fn to_byte(&self) -> Byte {
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
struct CPU {
    acc: Byte, //accumulator
    x: Byte,   //index
    y: Byte,   //index

    stkptr: Word,
    prgmctr: Word,

    status: Status,
    mode: AddressingModes,
}

#[allow(dead_code)]
impl CPU {
    fn new() -> Self {
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

    fn reset(&mut self) {
        self.acc = Byte::default();
        self.x = Byte::default();
        self.y = Byte::default();

        self.stkptr = STACK_HIGH;
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
    fn lda(&mut self, data: Byte) {
        self.prgmctr += 1;
        self.acc = data;
    }
    // direct
    fn push(&mut self, memory: &mut MEMORY, data: Byte) {
        memory.set_byte(self.stkptr, data);
        self.stkptr -= 1;
        //self.stkptr = xextend(data);
    }

    fn pull(&mut self, memory: &mut MEMORY) -> Byte {
        self.stkptr += 1;
        memory.get_byte(self.stkptr)
    }

    fn jmp(&mut self, data: Word) {
        self.prgmctr = data;
    }
    // push accumulator
    fn pha(&mut self, memory: &mut MEMORY) {
        self.prgmctr += 1;
        self.push(memory, self.acc)
    }
    // pull accumulator
    fn pla(&mut self, memory: &mut MEMORY) -> Byte {
        self.acc = self.pull(memory);
        self.prgmctr += 1;
        self.acc
    }

    fn nop(&mut self) {
        self.prgmctr += 1;
    }

    // push processor status
    fn php(&mut self, data: Byte) {
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
    fn plp(&mut self) -> Byte {
        self.status.to_byte()
    }
    // transfer accumulator to x
    fn txs(&mut self) {
        self.stkptr = xextend(self.x);
    }
    // transfer stack pointer to x
    fn tsx(&mut self) {
        self.prgmctr += 1;
        self.x = split_address(self.stkptr).1;
    }
    // transfer accumulator to x
    fn tax(&mut self) {
        self.prgmctr += 1;
        self.x = self.acc;
    }
    // transfer x to accumulator
    fn txa(&mut self) {
        self.prgmctr += 1;
        self.acc = self.x;
    }
    // transfer y to accumulator
    fn tya(&mut self) {
        self.prgmctr += 1;
        self.acc = self.y;
    }
    // transfer accumulator to y
    fn tay(&mut self) {
        self.prgmctr += 1;
        self.y = self.acc;
    }
    // decrement y
    fn dey(&mut self) {
        self.prgmctr += 1;
        self.y -= 1;
    }
    // increment y
    fn iny(&mut self) {
        self.prgmctr += 1;
        self.y += 1;
    }



    // executes and returms an option of the data depending on the instruction
    fn execute(&mut self, m: &mut MEMORY) -> Option<Byte> {
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
            0x9A => {
                self.txs();
                None
            }
            0xBA => {
                self.tsx();
                None
            }
            0xEA => {
                self.nop();
                None
            }
            0x08 => {
                self.php(operand1);
                None
            }
            0x28 => Some(self.plp()),
            0xAA => {
                self.tax();
                None
            }
            0x8A => {
                self.txa();
                None
            }
            0xCA => {
                self.dex();
                None
            }
            0xE8 => {
                self.inx();
                None
            }
            0xA8 => {
                self.tay();
                None
            }
            0x98 => {
                self.tya();
                None
            }
            0x88 => {
                self.dey();
                None
            }
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

// concatenates two operands into a u16 address
fn make_address(o1: Byte, o2: Byte) -> Word {
    let address: Word = ((o1 as Word) << 8) | o2 as Word;
    address
}

// splits u16 into u8 tuple
fn split_address(addr: Word) -> (Byte, Byte) {
    let high_byte: Byte = (addr >> 8) as Byte;
    let low_byte: Byte = addr as Byte;

    // big endian
    (high_byte, low_byte)
}

fn save_memory(mem: &MEMORY, file: &str) {
    let mut file = File::create(file).unwrap();
    file.write_all(&mem.data).unwrap();
}

fn load_memory(mem: &mut MEMORY, file: &str) {
    let mut file = File::open(file).unwrap();
    file.read_exact(&mut mem.data).unwrap();
}

fn main() {
    // test
    let mut _cpu = CPU::default();
    let mut _mem = MEMORY::default();

    // REPL
    for line in stdin().lock().lines() {
        print!("> ");
        stdout().flush();

        let expression = line.unwrap();
        let lexer = InterpreterInstr::lexer(&expression);
        let instructions: Vec<_> = lexer
            .spanned()
            .filter(|x| x.0 != InterpreterInstr::ERROR)
            .collect();

        for instr in instructions.iter() {
            match instr.0 {
                InterpreterInstr::Registers => {
                    println!("acc: {:?}", _cpu.acc);
                    println!("x: {:?}", _cpu.x);
                    println!("y: {:?}", _cpu.y);
                    println!("stkptr: {:?}", _cpu.stkptr);
                    println!("prgmctr: {:?}", _cpu.prgmctr);
                }
                InterpreterInstr::Reset => {
                    _cpu.reset();
                    _mem.reset();
                }
                InterpreterInstr::Status => {
                    println!("v: {:?}", _cpu.status.v);
                    println!("n: {:?}", _cpu.status.n);
                    println!("c: {:?}", _cpu.status.c);
                    println!("z: {:?}", _cpu.status.z);
                    println!("i: {:?}", _cpu.status.i);
                    println!("d: {:?}", _cpu.status.d);
                    println!("b: {:?}", _cpu.status.b);
                }
                InterpreterInstr::Exit => {
                    println!("good bye cruel world...");
                    exit(0)
                }
                InterpreterInstr::GetByte => {
                    let address = expression.split_ascii_whitespace().nth(1).unwrap();
                    let hex = u16::from_str_radix(address, 16).unwrap();

                    println!("{}", _mem.get_byte(hex));
                }
                InterpreterInstr::GetBytes => {
                    let address = expression.split_ascii_whitespace().nth(1).unwrap();
                    let hex = u16::from_str_radix(address, 16).unwrap();

                    println!("{}", _mem.get_byte(hex));
                }
                InterpreterInstr::SetByte => {
                    let address = expression.split_ascii_whitespace().nth(1).unwrap();
                    let hex = u16::from_str_radix(address, 16).unwrap();

                    let value = expression.split_ascii_whitespace().nth(2).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();

                    _mem.set_byte(hex, byte);
                }

                // TODO: fix this
                InterpreterInstr::SetBytes => {
                    let address = expression.split_ascii_whitespace().nth(1).unwrap();
                    let hex = u16::from_str_radix(address, 16).unwrap();

                    let values = expression.split_ascii_whitespace().nth(2).unwrap();
                    let bytes: Vec<u8> = values
                        .split(",")
                        .map(|x| u8::from_str_radix(x, 16).unwrap())
                        .collect();

                    _mem.set_bytes(hex, &bytes);
                }
                InterpreterInstr::Jump => {
                    let address = expression.split_ascii_whitespace().nth(1).unwrap();
                    let hex = u16::from_str_radix(address, 16).unwrap();

                    _cpu.jmp(hex);
                }
                InterpreterInstr::Execute => {
                    _cpu.execute(&mut _mem);
                }
                InterpreterInstr::LoadAccumulator => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();

                    _cpu.lda(byte);
                }
                InterpreterInstr::PushAccumulator => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();
                    _cpu.pha(&mut _mem);
                }
                InterpreterInstr::Push => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();
                    _cpu.push(&mut _mem, byte)
                }
                InterpreterInstr::Dump => {
                    save_memory(&_mem, "memory.dump");
                }
                InterpreterInstr::Load => {
                    load_memory(&mut _mem, "memory.dump");
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_cpu_jmp() {
        let mut cpu = CPU::new();
        let mut mem = MEMORY::new();

        mem.set_byte(0x0000, 0x4C);
        mem.set_byte(0x0001, 0xAA);
        mem.set_byte(0x0002, 0x55);

        cpu.execute(&mut mem);

        assert_eq!(cpu.prgmctr, 0xAA55);
    }

    #[test]
    fn test_cpu_register_reset() {
        let mut cpu = CPU::new();
        cpu.x = 0x50;
        cpu.reset();
        assert_eq!(cpu.x, 0x00);
    }
    #[test]
    fn test_cpu_status_reset() {
        let mut cpu = CPU::new();
        cpu.status.v = true;
        cpu.reset();
        assert_eq!(cpu.status.v, false);
    }
    #[test]
    fn test_cpu_complete_reset() {
        let mut cpu = CPU::new();
        cpu.x = 0x50;
        cpu.reset();
        assert_eq!(cpu.x, 0x00);
        cpu.status.v = true;
        cpu.reset();
        assert_eq!(cpu.status.v, false);
    }

    #[test]
    fn test_memory_reset() {
        let mut memory = MEMORY::new();
        memory.data[0] = 0x40;
        memory.reset();
        assert_eq!(memory.data[0], 0x00);
    }
    #[test]
    fn test_mem_read_at_address() {
        let mut memory = MEMORY::new();
        memory.data[0] = 0x40;
        assert_eq!(memory.get_byte(0x0000), 0x40);
    }
    #[test]
    fn test_mem_write_to_address() {
        let mut memory = MEMORY::new();
        memory.data[0] = 0x40;
        memory.set_byte(0x0000, 0x1A);
        assert_eq!(memory.data[0], 0x1A);
    }

    #[test]
    fn test_mem_get_addr_from_bitwidth() {
        let mut memory = MEMORY::new();
        memory.data[1] = 0x10;
        assert_eq!(memory.get_byte(0x0001), 0x10);
        assert_eq!(memory.get_byte(0x01), 0x10);
        assert_eq!(memory.get_byte(0x1), 0x10);
        //assert_eq!(memory.data[0x0010], 0x10);
    }

    #[test]
    fn test_fn_xextend_addr() {
        assert_eq!(xextend(0xAA), 0x00AA);
    }

    #[test]
    fn test_cpu_lda() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.data[0] = 0xA9;
        memory.data[1] = 0x11;
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x11);
    }

    #[test]
    fn test_fn_make_address() {
        let operand1: u8 = 0xAA;
        let operand2: u8 = 0xFF;
        assert_eq!(make_address(operand1, operand2), 0xAAFF);
    }

    #[test]
    fn test_fn_split_address() {
        let address: u16 = 0xFFAA;
        assert_eq!(split_address(address), (0xFF, 0xAA));
    }

    #[test]
    fn test_cpu_pha() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.acc = 0x11;
        memory.set_byte(0x0000, 0x48);
        cpu.execute(&mut memory);
        assert_eq!(memory.get_byte(0x01FF), 0x11);
    }
    #[test]
    fn test_util_push() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.push(&mut memory, 0xFF);
        assert_eq!(memory.get_byte(0x01FF), 0xFF);
    }
    #[test]
    fn test_util_pull() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.push(&mut memory, 0xFF);
        let value: Byte = cpu.pull(&mut memory);
        assert_eq!(value, 0xFF);
    }
    #[test]
    fn test_cpu_pla() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();

        cpu.push(&mut memory, 0x55);
        let value: Byte = cpu.pla(&mut memory);
        assert_eq!(value, cpu.acc);
    }

    #[test]
    fn test_cpu_stack_descent() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.push(&mut memory, 0x55);
        assert_eq!(memory.get_byte(0x01FF), 0x55);

        cpu.push(&mut memory, 0x66);
        assert_eq!(memory.get_byte(0x01FE), 0x66);

        cpu.push(&mut memory, 0x77);
        assert_eq!(memory.get_byte(0x01FD), 0x77);

        cpu.push(&mut memory, 0x88);
        assert_eq!(memory.get_byte(0x01FC), 0x88);
    }
    #[test]
    fn test_cpu_stack_ascent() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.push(&mut memory, 0x55);
        cpu.push(&mut memory, 0x66);
        cpu.push(&mut memory, 0x77);
        cpu.push(&mut memory, 0x88);

        assert_eq!(cpu.pull(&mut memory), 0x88);
        assert_eq!(cpu.pull(&mut memory), 0x77);
        assert_eq!(cpu.pull(&mut memory), 0x66);
        assert_eq!(cpu.pull(&mut memory), 0x55);
    }

    #[test]
    fn test_cpu_tsx() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.stkptr = 0x55;
        memory.set_byte(0x0000, 0xBA);
        cpu.execute(&mut memory);
        assert_eq!(cpu.x, 0x55);
    }

    #[test]
    fn test_cpu_prgm_counter() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.set_byte(0x0000, 0xEA);
        cpu.execute(&mut memory);
        assert_eq!(cpu.prgmctr, 0x0001);
    }
    #[test]
    fn test_status_register() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();

        cpu.status.n = true;
        cpu.status.c = true;

        let status: Byte = cpu.status.to_byte();
        assert_eq!(status, 0x81);
    }
    #[test]
    fn test_cpu_php() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.set_byte(0x0000, 0x08);
        memory.set_byte(0x0001, 0x81);
        cpu.execute(&mut memory);
        assert_eq!(cpu.status.n, true);
        assert_eq!(cpu.status.c, true);
    }
    #[test]
    fn test_cpu_plp() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.status.n = true;
        cpu.status.c = true;
        memory.set_byte(0x0000, 0x28);
        cpu.execute(&mut memory);
        assert_eq!(cpu.status.to_byte(), 0x81);
    }

    #[test]
    fn test_memory_save() {
        let mut memory = MEMORY::new();
        memory.data[0x0000] = 0x11;
        memory.data[0x0001] = 0x22;
        memory.data[0x0002] = 0x33;
        memory.data[0x0003] = 0x44;
        memory.data[0x0004] = 0x55;
        memory.data[0x0005] = 0x66;
        memory.data[0x0006] = 0x77;
        memory.data[0x0007] = 0x88;
        memory.data[0x0008] = 0x99;
        memory.data[0x0009] = 0xAA;
        memory.data[0x000A] = 0xBB;
        memory.data[0x000B] = 0xCC;
        memory.data[0x000C] = 0xDD;
        memory.data[0x000D] = 0xEE;
        memory.data[0x000E] = 0xFF;
        save_memory(&memory, "test.dump");
        
        let path = Path::new("test.dump");
        assert!(path.exists());

        fs::remove_file("test.dump").unwrap();
    }
    #[test]
    fn test_memory_load() {
        let mut memory = MEMORY::new();
        memory.data[0x0000] = 0x11;
        memory.data[0x0001] = 0x22;
        memory.data[0x0002] = 0x33;
        memory.data[0x0003] = 0x44;
        memory.data[0x0004] = 0x55;
        memory.data[0x0005] = 0x66;
        memory.data[0x0006] = 0x77;
        memory.data[0x0007] = 0x88;
        memory.data[0x0008] = 0x99;
        memory.data[0x0009] = 0xAA;
        memory.data[0x000A] = 0xBB;
        memory.data[0x000B] = 0xCC;
        memory.data[0x000C] = 0xDD;
        memory.data[0x000D] = 0xEE;
        memory.data[0x000E] = 0xFF;
        save_memory(&memory, "test.dump");
        
        let mut memory2 = MEMORY::new();
        load_memory(&mut memory2, "test.dump");
        assert_eq!(memory.data, memory2.data);

        fs::remove_file("test.dump").unwrap();
    }
}

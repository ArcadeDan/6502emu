use std::{
    io::{stdin, stdout, BufRead, Write},
    ops::Add,
    process::exit,
};

use logos::Logos;

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
    fn set_byte(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    fn set_bytes(&mut self, start: Word, values: &[Byte]) {
        let start = start as usize;
        let end = start + values.len();
        self.data[start..end].copy_from_slice(values);
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct Status {
    v: Byte, //overflow
    n: Byte, //negative
    c: Byte, //carry
    z: Byte, //zero
    i: Byte, //interrupt
    d: Byte, //decimal
    b: Byte, //break
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
        //sets ALL to 0
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

        self.stkptr = Word::default();
        self.prgmctr = Word::default();

        self.status.v = Byte::default();
        self.status.n = Byte::default();
        self.status.c = Byte::default();
        self.status.z = Byte::default();
        self.status.i = Byte::default();
        self.status.d = Byte::default();
        self.status.b = Byte::default();
    }

    fn lda(&mut self, data: Byte) {
        self.prgmctr += 1;
        self.acc = data;
    }

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

    fn pha(&mut self, memory: &mut MEMORY) {
        self.prgmctr += 1;
        self.push(memory, self.acc)
    }

    fn pla(&mut self, memory: &mut MEMORY) -> Byte {
        self.acc = self.pull(memory);
        self.prgmctr += 1;
        self.acc
    }

    fn nop(&mut self) {
        self.prgmctr += 1;
    }

    fn txs(&mut self) {
        self.stkptr = xextend(self.x);
    }

    fn tsx(&mut self) {
        self.prgmctr += 1;
        self.x = split_address(self.stkptr).1;
    }

    // init

    // executes and returms an option of the data depending on the instruction
    fn execute(&mut self, m: &mut MEMORY) -> Option<Byte> {
        let instruction = m.get_byte(self.prgmctr);
        let operand1 = m.get_byte(self.prgmctr + 1);
        let operand2 = m.get_byte(self.prgmctr + 2);
        match instruction {
            0xA9 => {
                self.lda(operand1);
                self.mode = AddressingModes::Accumulator;
                None
            }
            0x4C => {
                self.jmp(make_address(operand1, operand2));
                self.mode = AddressingModes::Absolute;
                None
            }
            0x48 => {
                self.pha(m);
                None
            }
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

    (high_byte, low_byte)
}
fn main() {
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
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_JMP() {
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
        cpu.status.v = 0x7A;
        cpu.reset();
        assert_eq!(cpu.status.v, 0x00);
    }
    #[test]
    fn test_cpu_complete_reset() {
        let mut cpu = CPU::new();
        cpu.x = 0x50;
        cpu.reset();
        assert_eq!(cpu.x, 0x00);
        cpu.status.v = 0x7A;
        cpu.reset();
        assert_eq!(cpu.status.v, 0x00);
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
    fn test_cpu_LDA() {
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
    fn test_cpu_PHA() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.acc = 0x11;
        memory.set_byte(0x0000, 0x48);
        cpu.execute(&mut memory);
        assert_eq!(memory.get_byte(0x01FF), 0x11);
    }
    #[test]
    fn test_util_PUSH() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.push(&mut memory, 0xFF);
        assert_eq!(memory.get_byte(0x01FF), 0xFF);
    }
    #[test]
    fn test_util_PULL() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.push(&mut memory, 0xFF);
        let value: Byte = cpu.pull(&mut memory);
        assert_eq!(value, 0xFF);
    }
    #[test]
    fn test_cpu_PLA() {
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

    fn test_cpu_prgm_counter() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.set_byte(0x0000, 0x00);
        memory.set_byte(0x0001, 0x00);
        cpu.execute(&mut memory);
        assert_eq!(cpu.prgmctr, 0x0002);
    }
}

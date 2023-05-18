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
    IndexedIndirectX,
    IndirectIndexedY,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
}

pub enum Instruction {
    LDA,
    STA,
    LDX,
    STX,
    LDY,
    STY,
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,
    PHA,
    PLA,
    PHP,
    PLP,
    AND,
    ORA,
    EOR,
    BIT,
    ADC,
    SBC,
    CMP,
    CPX,
    CPY,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    ASL,
    LSR,
    ROL,
    ROR,
    JMP,
    JSR,
    RTS,
    RTI,
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,
    BRK,
    NOP,
}

pub type DecodedInstruction = (Instruction, AddressingModes);

pub static OPCODES: [Option<(Instruction, AddressingModes)>; 256] = [
    /*0x00*/
    Some((Instruction::BRK, AddressingModes::Implied)),
    /*0x01*/
    Some((Instruction::ORA, AddressingModes::IndexedIndirectX)),
    /*0x02*/
    None,
    /*0x03*/
    None,
    /*0x04*/
    None,
    /*0x05*/
    Some((Instruction::ORA, AddressingModes::ZeroPage)),
    /*0x06*/
    Some((Instruction::ASL, AddressingModes::ZeroPage)),
    /*0x07*/
    None,
    /*0x08*/
    Some((Instruction::PHP, AddressingModes::Implied)),
    /*0x09*/
    Some((Instruction::ORA, AddressingModes::Immediate)),
    /*0x0A*/
    Some((Instruction::ASL, AddressingModes::Accumulator)),
    /*0x0B*/
    None,
    /*0x0C*/
    None,
    /*0x0D*/
    Some((Instruction::ORA, AddressingModes::Absolute)),
    /*0x0E*/
    Some((Instruction::ASL, AddressingModes::Absolute)),
    /*0x0F*/
    None,
    /*0x10*/
    Some((Instruction::BPL, AddressingModes::Relative)),
    /*0x11*/
    Some((Instruction::ORA, AddressingModes::IndirectIndexedY)),
    /*0x12*/
    None,
    /*0x13*/
    None,
    /*0x14*/
    None,
    /*0x15*/
    Some((Instruction::ORA, AddressingModes::ZeroPageX)),
    /*0x16*/
    Some((Instruction::ASL, AddressingModes::ZeroPageX)),
    /*0x17*/
    None,
    /*0x18*/
    Some((Instruction::CLC, AddressingModes::Implied)),
    /*0x19*/
    Some((Instruction::ORA, AddressingModes::AbsoluteY)),
    /*0x1A*/
    None,
    /*0x1B*/
    None,
    /*0x1C*/
    None,
    /*0x1D*/
    Some((Instruction::ORA, AddressingModes::AbsoluteX)),
    /*0x1E*/
    Some((Instruction::ASL, AddressingModes::AbsoluteX)),
    /*0x1F*/
    None,
    /*0x20*/
    Some((Instruction::JSR, AddressingModes::Absolute)),
    /*0x21*/
    Some((Instruction::AND, AddressingModes::IndexedIndirectX)),
    /*0x22*/
    None,
    /*0x23*/
    None,
    /*0x24*/
    Some((Instruction::BIT, AddressingModes::ZeroPage)),
    /*0x25*/
    Some((Instruction::AND, AddressingModes::ZeroPage)),
    /*0x26*/
    Some((Instruction::ROL, AddressingModes::ZeroPage)),
    /*0x27*/
    None,
    /*0x28*/
    Some((Instruction::PLP, AddressingModes::Implied)),
    /*0x29*/
    Some((Instruction::AND, AddressingModes::Immediate)),
    /*0x2A*/
    Some((Instruction::ROL, AddressingModes::Accumulator)),
    /*0x2B*/
    None,
    /*0x2C*/
    Some((Instruction::BIT, AddressingModes::Absolute)),
    /*0x2D*/
    Some((Instruction::AND, AddressingModes::Absolute)),
    /*0x2E*/
    Some((Instruction::ROL, AddressingModes::Absolute)),
    /*0x2F*/
    None,
    /*0x30*/
    Some((Instruction::BMI, AddressingModes::Relative)),
    /*0x31*/
    Some((Instruction::AND, AddressingModes::IndirectIndexedY)),
    /*0x32*/
    None,
    /*0x33*/
    None,
    /*0x34*/
    None,
    /*0x35*/
    Some((Instruction::AND, AddressingModes::ZeroPageX)),
    /*0x36*/
    Some((Instruction::ROL, AddressingModes::ZeroPageX)),
    /*0x37*/
    None,
    /*0x38*/
    Some((Instruction::SEC, AddressingModes::Implied)),
    /*0x39*/
    Some((Instruction::AND, AddressingModes::AbsoluteY)),
    /*0x3A*/
    None,
    /*0x3B*/
    None,
    /*0x3C*/
    None,
    /*0x3D*/
    Some((Instruction::AND, AddressingModes::AbsoluteX)),
    /*0x3E*/
    Some((Instruction::ROL, AddressingModes::AbsoluteX)),
    /*0x3F*/
    None,
    /*0x40*/
    Some((Instruction::RTI, AddressingModes::Implied)),
    /*0x41*/
    Some((Instruction::EOR, AddressingModes::IndexedIndirectX)),
    /*0x42*/
    None,
    /*0x43*/
    None,
    /*0x44*/
    None,
    /*0x45*/
    Some((Instruction::EOR, AddressingModes::ZeroPage)),
    /*0x46*/
    Some((Instruction::LSR, AddressingModes::ZeroPage)),
    /*0x47*/
    None,
    /*0x48*/
    Some((Instruction::PHA, AddressingModes::Implied)),
    /*0x49*/
    Some((Instruction::EOR, AddressingModes::Immediate)),
    /*0x4A*/
    Some((Instruction::LSR, AddressingModes::Accumulator)),
    /*0x4B*/
    None,
    /*0x4C*/
    Some((Instruction::JMP, AddressingModes::Absolute)),
    /*0x4D*/
    Some((Instruction::EOR, AddressingModes::Absolute)),
    /*0x4E*/
    Some((Instruction::LSR, AddressingModes::Absolute)),
    /*0x4F*/
    None,
    /*0x50*/
    Some((Instruction::BVC, AddressingModes::Relative)),
    /*0x51*/
    Some((Instruction::EOR, AddressingModes::IndirectIndexedY)),
    /*0x52*/
    None,
    /*0x53*/
    None,
    /*0x54*/
    None,
    /*0x55*/
    Some((Instruction::EOR, AddressingModes::ZeroPageX)),
    /*0x56*/
    Some((Instruction::LSR, AddressingModes::ZeroPageX)),
    /*0x57*/
    None,
    /*0x58*/
    Some((Instruction::CLI, AddressingModes::Implied)),
    /*0x59*/
    Some((Instruction::EOR, AddressingModes::AbsoluteY)),
    /*0x5A*/
    None,
    /*0x5B*/
    None,
    /*0x5C*/
    None,
    /*0x5D*/
    Some((Instruction::EOR, AddressingModes::AbsoluteX)),
    /*0x5E*/
    Some((Instruction::LSR, AddressingModes::AbsoluteX)),
    /*0x5F*/
    None,
    /*0x60*/
    Some((Instruction::RTS, AddressingModes::Implied)),
    /*0x61*/
    Some((Instruction::ADC, AddressingModes::IndexedIndirectX)),
    /*0x62*/
    None,
    /*0x63*/
    None,
    /*0x64*/
    None,
    /*0x65*/
    Some((Instruction::ADC, AddressingModes::ZeroPage)),
    /*0x66*/
    Some((Instruction::ROR, AddressingModes::ZeroPage)),
    /*0x67*/
    None,
    /*0x68*/
    Some((Instruction::PLA, AddressingModes::Implied)),
    /*0x69*/
    Some((Instruction::ADC, AddressingModes::Immediate)),
    /*0x6A*/
    Some((Instruction::ROR, AddressingModes::Accumulator)),
    /*0x6B*/
    None,
    /*0x6C*/
    Some((Instruction::JMP, AddressingModes::Indirect)),
    /*0x6D*/
    Some((Instruction::ADC, AddressingModes::Absolute)),
    /*0x6E*/
    Some((Instruction::ROR, AddressingModes::Absolute)),
    /*0x6F*/
    None,
    /*0x70*/
    Some((Instruction::BVS, AddressingModes::Relative)),
    /*0x71*/
    Some((Instruction::ADC, AddressingModes::IndirectIndexedY)),
    /*0x72*/
    None,
    /*0x73*/
    None,
    /*0x74*/
    None,
    /*0x75*/
    Some((Instruction::ADC, AddressingModes::ZeroPageX)),
    /*0x76*/
    Some((Instruction::ROR, AddressingModes::ZeroPageX)),
    /*0x77*/
    None,
    /*0x78*/
    Some((Instruction::SEI, AddressingModes::Implied)),
    /*0x79*/
    Some((Instruction::ADC, AddressingModes::AbsoluteY)),
    /*0x7A*/
    None,
    /*0x7B*/
    None,
    /*0x7C*/
    None,
    /*0x7D*/
    Some((Instruction::ADC, AddressingModes::AbsoluteX)),
    /*0x7E*/
    Some((Instruction::ROR, AddressingModes::AbsoluteX)),
    /*0x7F*/
    None,
    /*0x80*/
    None,
    /*0x81*/
    Some((Instruction::STA, AddressingModes::IndexedIndirectX)),
    /*0x82*/
    None,
    /*0x83*/
    None,
    /*0x84*/
    Some((Instruction::STY, AddressingModes::ZeroPage)),
    /*0x85*/
    Some((Instruction::STA, AddressingModes::ZeroPage)),
    /*0x86*/
    Some((Instruction::STX, AddressingModes::ZeroPage)),
    /*0x87*/
    None,
    /*0x88*/
    Some((Instruction::DEY, AddressingModes::Implied)),
    /*0x89*/
    None,
    /*0x8A*/
    Some((Instruction::TXA, AddressingModes::Implied)),
    /*0x8B*/
    None,
    /*0x8C*/
    Some((Instruction::STY, AddressingModes::Absolute)),
    /*0x8D*/
    Some((Instruction::STA, AddressingModes::Absolute)),
    /*0x8E*/
    Some((Instruction::STX, AddressingModes::Absolute)),
    /*0x8F*/
    None,
    /*0x90*/
    Some((Instruction::BCC, AddressingModes::Relative)),
    /*0x91*/
    Some((Instruction::STA, AddressingModes::IndirectIndexedY)),
    /*0x92*/
    None,
    /*0x93*/
    None,
    /*0x94*/
    Some((Instruction::STY, AddressingModes::ZeroPageX)),
    /*0x95*/
    Some((Instruction::STA, AddressingModes::ZeroPageX)),
    /*0x96*/
    Some((Instruction::STX, AddressingModes::ZeroPageY)),
    /*0x97*/
    None,
    /*0x98*/
    Some((Instruction::TYA, AddressingModes::Implied)),
    /*0x99*/
    Some((Instruction::STA, AddressingModes::AbsoluteY)),
    /*0x9A*/
    Some((Instruction::TXS, AddressingModes::Implied)),
    /*0x9B*/
    None,
    /*0x9C*/
    None,
    /*0x9D*/
    Some((Instruction::STA, AddressingModes::AbsoluteX)),
    /*0x9E*/
    None,
    /*0x9F*/
    None,
    /*0xA0*/
    Some((Instruction::LDY, AddressingModes::Immediate)),
    /*0xA1*/
    Some((Instruction::LDA, AddressingModes::IndexedIndirectX)),
    /*0xA2*/
    Some((Instruction::LDX, AddressingModes::Immediate)),
    /*0xA3*/
    None,
    /*0xA4*/
    Some((Instruction::LDY, AddressingModes::ZeroPage)),
    /*0xA5*/
    Some((Instruction::LDA, AddressingModes::ZeroPage)),
    /*0xA6*/
    Some((Instruction::LDX, AddressingModes::ZeroPage)),
    /*0xA7*/
    None,
    /*0xA8*/
    Some((Instruction::TAY, AddressingModes::Implied)),
    /*0xA9*/
    Some((Instruction::LDA, AddressingModes::Immediate)),
    /*0xAA*/
    Some((Instruction::TAX, AddressingModes::Implied)),
    /*0xAB*/
    None,
    /*0xAC*/
    Some((Instruction::LDY, AddressingModes::Absolute)),
    /*0xAD*/
    Some((Instruction::LDA, AddressingModes::Absolute)),
    /*0xAE*/
    Some((Instruction::LDX, AddressingModes::Absolute)),
    /*0xAF*/
    None,
    /*0xB0*/
    Some((Instruction::BCS, AddressingModes::Relative)),
    /*0xB1*/
    Some((Instruction::LDA, AddressingModes::IndirectIndexedY)),
    /*0xB2*/
    None,
    /*0xB3*/
    None,
    /*0xB4*/
    Some((Instruction::LDY, AddressingModes::ZeroPageX)),
    /*0xB5*/
    Some((Instruction::LDA, AddressingModes::ZeroPageX)),
    /*0xB6*/
    Some((Instruction::LDX, AddressingModes::ZeroPageY)),
    /*0xB7*/
    None,
    /*0xB8*/
    Some((Instruction::CLV, AddressingModes::Implied)),
    /*0xB9*/
    Some((Instruction::LDA, AddressingModes::AbsoluteY)),
    /*0xBA*/
    Some((Instruction::TSX, AddressingModes::Implied)),
    /*0xBB*/
    None,
    /*0xBC*/
    Some((Instruction::LDY, AddressingModes::AbsoluteX)),
    /*0xBD*/
    Some((Instruction::LDA, AddressingModes::AbsoluteX)),
    /*0xBE*/
    Some((Instruction::LDX, AddressingModes::AbsoluteY)),
    /*0xBF*/
    None,
    /*0xC0*/
    Some((Instruction::CPY, AddressingModes::Immediate)),
    /*0xC1*/
    Some((Instruction::CMP, AddressingModes::IndexedIndirectX)),
    /*0xC2*/
    None,
    /*0xC3*/
    None,
    /*0xC4*/
    Some((Instruction::CPY, AddressingModes::ZeroPage)),
    /*0xC5*/
    Some((Instruction::CMP, AddressingModes::ZeroPage)),
    /*0xC6*/
    Some((Instruction::DEC, AddressingModes::ZeroPage)),
    /*0xC7*/
    None,
    /*0xC8*/
    Some((Instruction::INY, AddressingModes::Implied)),
    /*0xC9*/
    Some((Instruction::CMP, AddressingModes::Immediate)),
    /*0xCA*/
    Some((Instruction::DEX, AddressingModes::Implied)),
    /*0xCB*/
    None,
    /*0xCC*/
    Some((Instruction::CPY, AddressingModes::Absolute)),
    /*0xCD*/
    Some((Instruction::CMP, AddressingModes::Absolute)),
    /*0xCE*/
    Some((Instruction::DEC, AddressingModes::Absolute)),
    /*0xCF*/
    None,
    /*0xD0*/
    Some((Instruction::BNE, AddressingModes::Relative)),
    /*0xD1*/
    Some((Instruction::CMP, AddressingModes::IndirectIndexedY)),
    /*0xD2*/
    None,
    /*0xD3*/
    None,
    /*0xD4*/
    None,
    /*0xD5*/
    Some((Instruction::CMP, AddressingModes::ZeroPageX)),
    /*0xD6*/
    Some((Instruction::DEC, AddressingModes::ZeroPageX)),
    /*0xD7*/
    None,
    /*0xD8*/
    Some((Instruction::CLD, AddressingModes::Implied)),
    /*0xD9*/
    Some((Instruction::CMP, AddressingModes::AbsoluteY)),
    /*0xDA*/
    None,
    /*0xDB*/
    None,
    /*0xDC*/
    None,
    /*0xDD*/
    Some((Instruction::CMP, AddressingModes::AbsoluteX)),
    /*0xDE*/
    Some((Instruction::DEC, AddressingModes::AbsoluteX)),
    /*0xDF*/
    None,
    /*0xE0*/
    Some((Instruction::CPX, AddressingModes::Immediate)),
    /*0xE1*/
    Some((Instruction::SBC, AddressingModes::IndexedIndirectX)),
    /*0xE2*/
    None,
    /*0xE3*/
    None,
    /*0xE4*/
    Some((Instruction::CPX, AddressingModes::ZeroPage)),
    /*0xE5*/
    Some((Instruction::SBC, AddressingModes::ZeroPage)),
    /*0xE6*/
    Some((Instruction::INC, AddressingModes::ZeroPage)),
    /*0xE7*/
    None,
    /*0xE8*/
    Some((Instruction::INX, AddressingModes::Implied)),
    /*0xE9*/
    Some((Instruction::SBC, AddressingModes::Immediate)),
    /*0xEA*/
    Some((Instruction::NOP, AddressingModes::Implied)),
    /*0xEB*/
    None,
    /*0xEC*/
    Some((Instruction::CPX, AddressingModes::Absolute)),
    /*0xED*/
    Some((Instruction::SBC, AddressingModes::Absolute)),
    /*0xEE*/
    Some((Instruction::INC, AddressingModes::Absolute)),
    /*0xEF*/
    None,
    /*0xF0*/
    Some((Instruction::BEQ, AddressingModes::Relative)),
    /*0xF1*/
    Some((Instruction::SBC, AddressingModes::IndirectIndexedY)),
    /*0xF2*/
    None,
    /*0xF3*/
    None,
    /*0xF4*/
    None,
    /*0xF5*/
    Some((Instruction::SBC, AddressingModes::ZeroPageX)),
    /*0xF6*/
    Some((Instruction::INC, AddressingModes::ZeroPageX)),
    /*0xF7*/
    None,
    /*0xF8*/
    Some((Instruction::SED, AddressingModes::Implied)),
    /*0xF9*/
    Some((Instruction::SBC, AddressingModes::AbsoluteY)),
    /*0xFA*/
    None,
    /*0xFB*/
    None,
    /*0xFC*/
    None,
    /*0xFD*/
    Some((Instruction::SBC, AddressingModes::AbsoluteX)),
    /*0xFE*/
    Some((Instruction::INC, AddressingModes::AbsoluteX)),
    /*0xFF*/
    None,
];

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
    // brk
    pub fn brk(&mut self) {
        self.prgmctr += 1;
        self.status.b = true;
    }

    // load x
    pub fn ldx(&mut self, data: u8) {
        self.status.n = true;
        self.status.z = true;
        self.x = data;
        self.prgmctr += 2;

    }
    // load y
    pub fn ldy(&mut self, data: u8) {
        self.status.n = true;
        self.status.z = true;
        self.y = data;
        self.prgmctr += 2;
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
    pub fn sta(&mut self, memory: &mut MEMORY) {
        memory.set_byte(self.prgmctr, self.acc);
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
            // ldx
            0xA2 => {
                self.ldx(operand1);
                self.mode = AddressingModes::Immediate;
                None
            }
            // ldy
            0xA0 => {
                self.ldy(operand1);
                self.mode = AddressingModes::Immediate;
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
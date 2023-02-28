type Byte = u8;
type Word = u16;


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

/// A fixed-size of 65535 bytes
#[derive(Debug)]
struct MEMORY {
    data: [u8; 1024 * 64],
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
    fn get_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
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
/*
impl Iterator for Status {
    type Item = Byte;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
*/
/*
impl Status {
    fn getflag(self) {
        let f: Vec<u8> = self.collect();

    }
}
*/
#[allow(dead_code)]
struct CPU {
    acc: Byte, //accumulator
    x: Byte,   //index
    y: Byte,   //index

    stkptr: Word,
    prgmctr: Word,

    status: Status,
}

#[allow(dead_code)]
impl CPU {
    fn new() -> Self {
        //sets ALL to 0
        Self {
            acc: Byte::default(),
            x: Byte::default(),
            y: Byte::default(),
            stkptr: Word::default(),
            prgmctr: Word::default(),
            status: Status::default(),
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

fn main() {
    let mut _cpu = CPU::default();
    let mut _mem = MEMORY::default();
    _mem.data[0] = 0x0A;
    dbg!(_mem.get_byte(0x200A));

    println!("good bye cruel world...");
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_fetch_at_address() {
        let mut memory = MEMORY::new();
        memory.data[0] = 0x40;
        assert_eq!(memory.get_byte(0x0000), 0x40);
    }
}

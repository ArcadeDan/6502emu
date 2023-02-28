type Byte = u8;
type Word = u16;

/// A fixed-size of 65535 bytes
/// 
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
        self.data.map(|mut byte| byte = 0x00 as u8);
    }

}
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
struct CPU {
    acc: Byte, //accumulator
    x: Byte,   //index
    y: Byte,   //index

    stkptr: Word,
    prgmctr: Word,

    status: Status,
}

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
        self.acc = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        
        self.stkptr = 0x00;
        self.prgmctr = 0x00;
        
        self.status.v = 0x00;
        self.status.n = 0x00;
        self.status.c = 0x00;
        self.status.z = 0x00;
        self.status.i = 0x00;
        self.status.d = 0x00;
        self.status.b = 0x00;
    }
}

fn main() {
    let mut _cpu = CPU::new();
    let mut _mem = MEMORY::new();
    //println!("{:?}", _cpu.status.getflag());
    println!("good bye cruel world...");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn cpu_register_reset() {
        let mut cpu = CPU::new();
        cpu.x = 0x50;
        cpu.reset();
        assert_eq!(cpu.x, 0x00);
        

    }
    #[test]
    fn cpu_status_reset() {
        let mut cpu = CPU::new();
        cpu.status.v = 0x7A;
        cpu.reset();
        assert_eq!(cpu.status.v, 0x00);
    }
    #[test]
    fn cpu_complete_reset() {
        let mut cpu = CPU::new();
        cpu.x = 0x50;
        cpu.reset();
        assert_eq!(cpu.x, 0x00);
        cpu.status.v = 0x7A;
        cpu.reset();
        assert_eq!(cpu.status.v, 0x00);
    }
}
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
}

fn main() {
    let mut _cpu = CPU::new();
    let mut _mem = MEMORY::new();
    //println!("{:?}", _cpu.status.getflag());
    println!("good bye cruel world...");
}

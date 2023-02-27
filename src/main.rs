type Byte = u8;
type Word = u16;

#[derive(Debug)]
struct MEMORY {
    // max_memory: 65536
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
    let mem = MEMORY::new();

    println!("good bye cruel world...");
}

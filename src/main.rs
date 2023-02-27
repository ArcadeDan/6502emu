

type Byte = u8;
type Word = u16;

#[derive(Debug)]
struct MEMORY {
    data: [u8; 1024 * 64],
}

impl MEMORY {
    fn new() -> Self {
        Self { data: [0; 1024 * 64] }
    }
}



fn main() {

    let mem = MEMORY::new();
  

    println!("good bye cruel world...");
}

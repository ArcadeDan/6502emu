use array_macro::array;


type Byte = u8;
type Word = u16;

struct MEMORY {
    Data: [Byte; 1024 * 64],

}

impl MEMORY {
    fn new() -> Self {
        Self { Data: array!([u8; 1024 * 64]) }
    }


}



fn main() {
    println!("good bye cruel world...");
}

use std::{
    alloc::System,
    io::{stdin, stdout, BufRead, Write},
    ops::Add,
    process::exit,
};

use crate::cpu::{load_memory, make_address, save_memory, split_address, xextend, CPU, MEMORY};

use logos::Logos;
use std::fs;
use std::fs::File;
use std::io::Read;

type Byte = u8;
type Word = u16;

mod cpu;
mod instruction;

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
    #[token("x")]
    X,
    #[token("y")]
    Y,
    #[token("acc")]
    Acc,
    #[token("stkptr")]
    Stkptr,
    #[token("prgmctr")]
    Prgmctr,

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
                    print!("\x1B[2J\n");
                    println!("acc: {:?}", _cpu.acc);
                    println!("x: {:?}", _cpu.x);
                    println!("y: {:?}", _cpu.y);
                    println!("stkptr: {:?}", _cpu.stkptr);
                    println!("prgmctr: {:?}", _cpu.prgmctr);
                }
                InterpreterInstr::X => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();
                    _cpu.x = byte;
                }

                InterpreterInstr::Y => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();
                    _cpu.y = byte;
                }

                InterpreterInstr::Acc => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();
                    _cpu.acc = byte;
                }

                InterpreterInstr::Stkptr => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let word = u16::from_str_radix(value, 16).unwrap();
                    _cpu.stkptr = word;
                }

                InterpreterInstr::Prgmctr => {
                    let value = expression.split_ascii_whitespace().nth(1).unwrap();
                    let word = u16::from_str_radix(value, 16).unwrap();
                    _cpu.prgmctr = word;
                }

                InterpreterInstr::Reset => {
                    _cpu.reset(); //
                    _mem.reset();
                }
                InterpreterInstr::Status => {
                    print!("\x1B[2J");
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

                InterpreterInstr::SetByte => {
                    let address = expression.split_ascii_whitespace().nth(1).unwrap();
                    let hex = u16::from_str_radix(address, 16).unwrap();

                    let value = expression.split_ascii_whitespace().nth(2).unwrap();
                    let byte = u8::from_str_radix(value, 16).unwrap();

                    _mem.set_byte(hex, byte);
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
                    //let value = expression.split_ascii_whitespace().nth(1).unwrap();

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
    fn test_cpu_lda_immediate() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.data[0] = 0xA9;
        memory.data[1] = 0x11;
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x11);
    }
    #[test]
    fn test_cpu_lda_absolute() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.data[0] = 0xAD;
        memory.data[1] = 0xFF;
        memory.data[2] = 0xFF;
        memory.data[0xFFFF] = 0x11;
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x11);
    }

    #[test]
    fn test_cpu_lda_absoluteX() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.x = 0x11;
        memory.data[0] = 0xBD;
        memory.data[1] = 0xAA;
        memory.data[2] = 0xAA;
        let offset = 0xAAAA + cpu.x as u16;
        memory.set_byte(offset, 0x11);

        cpu.execute(&mut memory);
        assert_eq!(memory.get_byte(offset), 0x11);
    }

    #[test]
    fn test_cpu_lda_absoluteY() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.y = 0x11;
        memory.data[0] = 0xB9;
        memory.data[1] = 0xAA;
        memory.data[2] = 0xAA; //
        let offset = 0xAAAA + cpu.y as u16;
        memory.set_byte(offset, 0x11);

        cpu.execute(&mut memory);
        assert_eq!(memory.get_byte(offset), 0x11);
    }

    #[test]
    fn test_cpu_lda_zeropage() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.set_byte(0x00FF, 0x32);
        memory.data[0] = 0xA5;
        memory.data[1] = 0xFF;
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x32);
    }
    #[test]
    fn test_cpu_lda_zeropageX() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.data[0] = 0xB5;
        memory.data[1] = 0x20;
        cpu.x = 0x11;
        memory.set_byte(0x0020 + cpu.x as u16, 0x33);
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x33);
    }

    

    #[test]
    fn test_cpu_lda_zeropage_indirectX() {
        // first add operand1 into accumulator.
        // zp byte at operand1
        // zp byte at operand2
        // make address of zp bytes
        // load byte at address into accumulator

        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.set_byte(0x0000, 0xA1); // lda (zp, x)
        memory.set_byte(0x0001, 0x42); // operand1
        memory.set_byte(0x0042, 0x81); // operand1 byte
        memory.set_byte(0x0002, 0x16); // operand2
        memory.set_byte(0x0016, 0x08); // operand2 byte
        memory.set_byte(0x8108, 0x55);
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x55);
        cpu.execute(&mut memory);
    }
    
    #[test]
    fn test_cpu_lda_zeropageY() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();

        memory.set_byte(0x0000, 0xB1); // lda (zp, x)
        memory.set_byte(0x0001, 0x42); // operand1
        memory.set_byte(0x0042, 0x81); // operand1 byte
        memory.set_byte(0x0002, 0x16); // operand2
        memory.set_byte(0x0016, 0x08); // operand2 byte
        memory.set_byte(0x8108, 0x55);

        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x55);
        cpu.execute(&mut memory);
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

    // linux IO may cause this unit test to fail
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

    // linux IO may cause this unit test to fail
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

    #[test]
    fn test_cpu_tax() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.acc = 0x55;
        memory.set_byte(0x0000, 0xAA);
        cpu.execute(&mut memory);
        assert_eq!(cpu.x, 0x55);
    }
    #[test]
    fn test_cpu_txa() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.x = 0x55;
        memory.set_byte(0x0000, 0x8A);
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x55);
    }
    #[test]
    fn test_cpu_tay() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.acc = 0x55;
        memory.set_byte(0x0000, 0xA8);
        cpu.execute(&mut memory);
        assert_eq!(cpu.y, 0x55);
    }
    #[test]
    fn test_cpu_tya() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.y = 0x55;
        memory.set_byte(0x0000, 0x98);
        cpu.execute(&mut memory);
        assert_eq!(cpu.acc, 0x55);
    }
    #[test]
    fn test_cpu_inx() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.x = 0x55;
        memory.set_byte(0x0000, 0xE8);
        cpu.execute(&mut memory);
        assert_eq!(cpu.x, 0x56);
    }
    #[test]
    fn test_cpu_iny() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.y = 0x55;
        memory.set_byte(0x0000, 0xC8);
        cpu.execute(&mut memory);
        assert_eq!(cpu.y, 0x56);
    }
    #[test]
    fn test_cpu_dex() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.x = 0x55;
        memory.set_byte(0x0000, 0xCA);
        cpu.execute(&mut memory);
        assert_eq!(cpu.x, 0x54);
    }
    #[test]
    fn test_cpu_dey() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        cpu.y = 0x55;
        memory.set_byte(0x0000, 0x88);
        cpu.execute(&mut memory);
        assert_eq!(cpu.y, 0x54);
    }

    // test the cpu so it resets the stack pointer
    #[test]
    fn test_cpu_brk() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.set_byte(0x0000, 0x00);
        cpu.execute(&mut memory);
        assert_eq!(cpu.prgmctr, 0x01);
    }
    #[test]
    fn test_cpu_stx() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();

        memory.set_byte(0x0000, 0x86);
        cpu.x = 0xAA;
        cpu.execute(&mut memory);

        assert_eq!(memory.get_byte(0x0000), 0xAA);
    }

    #[test]
    fn test_cpu_sty() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();

        memory.set_byte(0x0000, 0x84);
        cpu.y = 0xFF;
        cpu.execute(&mut memory);

        assert_eq!(memory.get_byte(0x0000), 0xFF);
    }

    #[test]
    fn test_cpu_sta() {
        let mut memory = MEMORY::new();
        let mut cpu = CPU::new();
        memory.set_byte(0x0000, 0x85);
        cpu.acc = 0x14;
        cpu.execute(&mut memory);
        assert_eq!(memory.get_byte(0x0000), 0x14);
    }
}

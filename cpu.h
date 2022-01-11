#pragma once

#include <cstdint>
using Byte = uint8_t;
using Word = uint16_t;
using u32 = uint32_t;

namespace cpu{
    struct MEM{
        static constexpr u32 MAX_MEM = 1024 * 64;
        Byte Data[MAX_MEM];

        void INIT(){
            for (u32 i = 0; i < MAX_MEM; i++){
                Data[i] = 0;
            }
        }
        //read byte
        Byte operator[](u32 Address) const {
            return Data[Address];
        }
    };
    //6502
    struct CPU{
    // registers
    Byte A;  //accumulator
    Byte X;  //index 
    Byte Y;  //index
    Byte Status;
    Word SP; // stack pointer
    Word PC; //program counter

    Byte C : 1; // CARRY status 
    Byte Z : 1; // ZERO  status 
    Byte I : 1; // INTERRUPT status 
    Byte D : 1; // Decimal status 
    Byte B : 1; // BREAK status 

    Byte V : 1; // OVERFLOW status 
    Byte N : 1; // NEGATIVE status 

        void RESET( cpu::MEM &memory){
            PC = 0xFFFC;
            D = 0;
            SP = 0x00FF;

            A = X = Y = C = Z
            = I = D = B = V = N = 0;
            memory.INIT();
        }
        //byte
        Byte FETCH(u32 &cycles, cpu::MEM &memory ){
            Byte Data = memory[PC];
            PC++;
            cycles--;
            return Data;
        }
        void EXECUTE(u32 cycles, cpu::MEM &memory ){
            while (cycles > 0) {
                Byte instruc = FETCH(cycles, memory);
            }
        }
    };
}

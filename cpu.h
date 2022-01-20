#pragma once
#include <iostream>
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
        Byte &operator[](u32 Address){
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

    /*OP CODES */      // http://www.6502.org/tutorials/6502opcodes.html
        
        /* ADC - Add with Carry */     // FLAGS : N V Z C
        static constexpr Byte OP_ADC = 0x69;    // 2 length, 2 cycles
        static constexpr Byte OP_ADC_ZP = 0x65; // 2 length, 2 cycles
        static constexpr Byte OP_ADC_ZPX = 0x75;// 2 length, 2 cycles
        static constexpr Byte OP_ADC_A = 0x6D;  // 3 length, 4 cycles
        static constexpr Byte OP_ADC_AX = 0x7D; // 3 length, 4+ cycles
        static constexpr Byte OP_ADC_AY = 0x79; // 3 length, 4+ cycles
        static constexpr Byte OP_ADC_IX = 0x61; // 2 length, 6 cycles
        static constexpr Byte OP_ADC_IY = 0x71; // 2 length, 5+ cycles

        /* AND - bitwise AND with accumulator */    // FLAGS : N Z
        static constexpr Byte OP_AND = 0x29;    // 2 length, 2 cycles
        static constexpr Byte OP_AND_ZP = 0x25; // 2 length, 3 cycles
        static constexpr Byte OP_AND_ZPX = 0x35;// 2 length, 4 cycles
        static constexpr Byte OP_AND_A = 0x2D;  // 3 length, 4 cycles
        static constexpr Byte OP_AND_AX = 0x7D; // 3 length, 4+ cycles
        static constexpr Byte OP_AND_AY = 0x79; // 3 length, 4+ cycles
        static constexpr Byte OP_AND_IX = 0x61; // 2 length, 6 cycles
        static constexpr Byte OP_AND_IY = 0x71; // 2 length, 5+ cycles

        /* ASL - Arithmetic Shift Left */   // FLAGS : N Z C
        static constexpr Byte OP_ASL_ACCUMULATOR = 0x0A; // 1 length, 2 cycles
        static constexpr Byte OP_ASL_ZP = 0x06; // 2 length, 5 cycles
        static constexpr Byte OP_ASL_ZPX = 0x16;// 2 length, 6 cycles
        static constexpr Byte OP_ASL_A = 0x0E;  // 3 length, 6 cycles
        static constexpr Byte OP_ASL_AX = 0x1E;  // 3 length, 7 cycles

        /* BIT - test BITS */   // FLAGS : N V Z
        static constexpr Byte OP_BIT_ZP = 0x24; // 2 length, 3 cycles
        static constexpr Byte OP_BIT_A = 0x2C; // 3 length, 4 cycles

        /* BRK - break */
        static constexpr Byte OP_BRK = 0x00; // 1 length, 7 cycles;

        /* CMP - Compare accumulator */ // FLAGS : N Z C
        static constexpr Byte OP_CMP = 0xC9; // 2 length, 2 cycles
        static constexpr Byte OP_CMP_ZP = 0xC5;// 2 length, 3 cycles
        static constexpr Byte OP_CMP_ZPX = 0xD5;// 2 length, 4 cycles
        static constexpr Byte OP_CMP_A = 0xCD; // 3 length,  4 cycles
        static constexpr Byte OP_CMP_AX = 0xDD; // 3 length, 4+ cycles
        static constexpr Byte OP_CMP_AY = 0xD9; // 3 length, 4+ cycles;
        static constexpr Byte OP_CMP_IX = 0xC1; // 2 length, 6 cycles;
        static constexpr Byte OP_CMP_IY = 0xD1; // 2 length, 5+ cycles;

        /* CPY - Compare Y register */ // FLAGS : N Z C
        static constexpr Byte OP_CPY = 0xC0; // 2 length, 2 cycles;
        static constexpr Byte OP_CPY_ZP = 0xC4; // 2 length, 3 cycles;
        static constexpr Byte OP_CPY_A = 0xCC; // 3 length, 4 cycles;

        /* CPX - Compare X register */ // FLAGS : N Z C
        static constexpr Byte OP_CPX = 0xE0; // 2 length, 2 cycles;
        static constexpr Byte OP_CPX_ZP = 0xE4; // 2 length, 3 cycles;
        static constexpr Byte OP_CPX_A = 0xEC; // 3 length, 4 cycles;
        
        /* DEC - Decrement memory */ // FLAGS : N Z
        static constexpr Byte OP_DEC_ZP = 0xC6; // 2 length, 5 cycles

        /* LDA - load accumulator*/     // FLAGS : N Z
        static constexpr Byte OP_LDA = 0xA9;    // 2 length, 2 cycles
        static constexpr Byte OP_LDA_ZP = 0xA5; // 2 length, 2 cycles
        static constexpr Byte OP_LDA_ZPX = 0xB5;// 2 length, 4 cycles
        static constexpr Byte OP_LDA_A = 0xAD;  // 3 length, 4 cycles
        static constexpr Byte OP_LDA_AX = 0xBD; // 3 length, 4+ cycles
        static constexpr Byte OP_LDA_AY = 0xB9; // 3 length, 4+ cycles
        static constexpr Byte OP_LDA_IX = 0xA1; // 2 length, 6 cycles
        static constexpr Byte OP_LDA_IY = 0xB1; // 2 length, 5+ cycles
        
        /* LDX - load x register  */       // FLAGS : N Z
        static constexpr Byte OP_LDX = 0xA2;    // 2 length, 2 cycles
        static constexpr Byte OP_LDX_ZP = 0xA6; // 2 length, 3 cycles 
        static constexpr Byte OP_LDX_ZPY = 0xB6;// 2 length, 4 cycles
        static constexpr Byte OP_LDX_A = 0xAE;  // 3 length, 4 cycles
        static constexpr Byte OP_LDX_AY = 0xBE; // 3 length, 4+ cycles

        /* LDX - load x register  */       // FLAGS : N Z
        static constexpr Byte OP_LDY = 0xA0;    // 2 length, 2 cycles
        static constexpr Byte OP_LDY_ZP = 0xA4; // 2 length, 3 cycles 
        static constexpr Byte OP_LDY_ZPX = 0xB4;// 2 length, 4 cycles
        static constexpr Byte OP_LDY_A = 0xAC;  // 3 length, 4 cycles
        static constexpr Byte OP_LDY_AX = 0xBC; // 3 length, 4+ cycles


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
                Byte instruction = FETCH(cycles, memory);
                switch(instruction){
                    default:
                        std::cout << "Instruction not handled " <<
                         instruction << " \n";
                        break;
                    case OP_LDA:
                        Byte Val = FETCH(cycles, memory);
                        A = Val;
                        Z = (A == 0);
                        N = (A & 0b10000000) > 0;
                        break;
                }
            }
        }
    };
}

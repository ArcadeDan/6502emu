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
        static constexpr Byte OP_BRK = 0x00; // 1 length, 7 cycles

        /* CMP - Compare accumulator */ // FLAGS : N Z C
        static constexpr Byte OP_CMP = 0xC9; // 2 length, 2 cycles
        static constexpr Byte OP_CMP_ZP = 0xC5;// 2 length, 3 cycles
        static constexpr Byte OP_CMP_ZPX = 0xD5;// 2 length, 4 cycles
        static constexpr Byte OP_CMP_A = 0xCD; // 3 length,  4 cycles
        static constexpr Byte OP_CMP_AX = 0xDD; // 3 length, 4+ cycles
        static constexpr Byte OP_CMP_AY = 0xD9; // 3 length, 4+ cycles
        static constexpr Byte OP_CMP_IX = 0xC1; // 2 length, 6 cycles
        static constexpr Byte OP_CMP_IY = 0xD1; // 2 length, 5+ cycles

        /* CPY - Compare Y register */ // FLAGS : N Z C
        static constexpr Byte OP_CPY = 0xC0; // 2 length, 2 cycles
        static constexpr Byte OP_CPY_ZP = 0xC4; // 2 length, 3 cycles
        static constexpr Byte OP_CPY_A = 0xCC; // 3 length, 4 cycles

        /* CPX - Compare X register */ // FLAGS : N Z C
        static constexpr Byte OP_CPX = 0xE0; // 2 length, 2 cycles
        static constexpr Byte OP_CPX_ZP = 0xE4; // 2 length, 3 cycles
        static constexpr Byte OP_CPX_A = 0xEC; // 3 length, 4 cycles
        
        /* DEC - Decrement memory */ // FLAGS : N Z
        static constexpr Byte OP_DEC_ZP = 0xC6; // 2 length, 5 cycles
        static constexpr Byte OP_DEC_ZPX = 0xD6;// 2 length, 6 cycles
        static constexpr Byte OP_DEC_A = 0xCE;  // 3 length, 6 cycles
        static constexpr Byte OP_DEC_AX = 0xDE; // 3 length, 7 cycles

        /* EOR - bitwise exclusive OR */ // FLAGS : N Z
        static constexpr Byte OP_EOR = 0x49;   // 2 length, 2 cycles
        static constexpr Byte OP_EOR_ZP = 0x45; // 2 length, 3 cycles
        static constexpr Byte OP_EOR_ZPX = 0x55; // 2 length, 4 cycles
        static constexpr Byte OP_EOR_A = 0x4D;   // 3 length, 4 cycles
        static constexpr Byte OP_EOR_AX = 0x5D; // 3 length, 4+ cycles
        static constexpr Byte OP_EOR_AY = 0x59; // 3 length, 4+ cycles
        static constexpr Byte OP_EOR_IX = 0x41; // 2 length, 6 cycles
        static constexpr Byte OP_EOR_IY = 0x51; // 2 length, 5+ cycles

        /* Flag (Processor Status) instructions */ // FLAGS : Affects as noted, All have 1 length and two cycles
        static constexpr Byte OP_CLC = 0x18; // clear carry   
        static constexpr Byte OP_SEC = 0x38; // set carry
        static constexpr Byte OP_CLI = 0x58; // clear interrupt
        static constexpr Byte OP_SEI = 0x78; // set interrupt
        static constexpr Byte OP_CLV = 0xB8; // clear overflow
        static constexpr Byte OP_CLD = 0xD8; // clear decimal
        static constexpr Byte OP_SED = 0xF8; // set decimal

        /* INC (Increment memory) */    // FLAGS : N Z
        static constexpr Byte OP_INC_ZP = 0xE6; // 2 length, 5 cycles
        static constexpr Byte OP_INC_ZPX = 0xF6;// 2 length, 6 cycles
        static constexpr Byte OP_INC_A = 0xEE;  // 3 length, 6 cycles
        static constexpr Byte OP_INC_AX = 0xFE; // 3 length, 7 cycles

        /* JMP - jump */    // FLAGS : None
        static constexpr Byte OP_JMP_A = 0x4C;  // 3 length, 3 cycles
        static constexpr Byte OP_JMP_I = 0x6C; // 3 length, 5 cycles

        /* JSR - jump to subroutine */
        static constexpr Byte OP_JSR_A = 0x20; // 3 length, 6 cycles

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

        /* LSR (logical shift right) */

        static constexpr Byte OP_LSR_ACCUMULATOR = 0x4A; // 1 length, 2 cycles
        static constexpr Byte OP_LSR_ZP = 0x46; // 1 length, 2 cycles
        static constexpr Byte OP_LSR_ZPX = 0x56; // 2 length, 6 cycles
        static constexpr Byte OP_LSR_A = 0x4E; // 3 length, 6 cycles
        static constexpr Byte OP_LSR_AX = 0x5E; // 3 length, 7 cycles

        /* NOP - No Operation */

        static constexpr Byte OP_NOP = 0xEA;    // 1 length, 2 cycles
        
        /* ORA - Bitwise OR with Accumulator */ // FLAGS : N Z
        static constexpr Byte ORA = 0x09; // 2 length, 2 cycles
        static constexpr Byte ORA_ZP = 0x05; // 2 length, 3 cycles
        static constexpr Byte ORA_ZPX = 0x15; // 2 length, 4 cycles
        static constexpr Byte ORA_A = 0x0D; // 3 length, 4 cycles
        static constexpr Byte ORA_AX = 0x1D; // 3 length, 4+ cycles
        static constexpr Byte ORA_AY = 0x19; // 3 length, 4+ cycles
        static constexpr Byte ORA_IX = 0x01; // 2 length, 6 cycles
        static constexpr Byte ORA_IY = 0x11; // 2 length, 5+ cycles

        /* Register Instructions */ // FLAGS : N Z     |  1 length, 2 cycles
        static constexpr Byte TAX = 0xAA; // ( Transfer A to X )
        static constexpr Byte TXA = 0x8A; // ( Transfer X to A )
        static constexpr Byte DEX = 0xCA; // ( DEcrement X )
        static constexpr Byte INX = 0xE8; // ( INcrement X )
        static constexpr Byte TAY = 0xA8; // ( Transfer A to Y )
        static constexpr Byte TYA = 0x98; // ( Transfer Y to A )
        static constexpr Byte DEY = 0x88; // ( DEcrement Y )
        static constexpr Byte INY = 0xC8; // ( INcrement Y )

        /* ROL - Rotate Left */ // FLAGS : N Z C
        static constexpr Byte OP_ROL_ACCUMULATOR = 0x2A; // 1 length, 2 cycles
        static constexpr Byte OP_ROL_ZP = 0x26; // 2 length , 5 cycles
        static constexpr Byte OP_ROL_ZPX = 0x36; // 2 length, 6 cycles 
        static constexpr Byte OP_ROL_A = 0x2E; // 3 length, 6 cycles
        static constexpr Byte OP_ROL_AX = 0x3E; //3 length, 7 cycles

        /* ROR - Rotate Right */ // FLAGS : N Z C
        static constexpr Byte OP_ROR_ACCUMULATOR = 0x6A; // 1 length, 2 cycles
        static constexpr Byte OP_ROR_ZP = 0x66; // 2 length , 5 cycles
        static constexpr Byte OP_ROR_ZPX = 0x76; // 2 length, 6 cycles 
        static constexpr Byte OP_ROR_A = 0x6E; // 3 length, 6 cycles
        static constexpr Byte OP_ROR_AX = 0x7E; //3 length, 7 cycles
        
        /* RTI - Return from Interrupt */ // FLAGS : ALL
        static constexpr Byte OP_RTI = 0x40; // 1 length, 6 cycles

        /* RTS - Return from Subroutine */  // FLAGS : NONE
        static constexpr Byte OP_RTS = 0x60; // length 1, 6 cycles

        /* SBC - Subtract with Carry */ // FLAGS : N V Z C
        static constexpr Byte OP_SBC = 0xE9; // 2 length, 2 cycles
        static constexpr Byte OP_SBC_ZP = 0xE5; // 2 length, 2 cycles
        static constexpr Byte OP_SBC_ZPX = 0xF5; // 2 length, 4 cycles
        static constexpr Byte OP_SBC_A = 0xED; //  3 length, 4 cycles
        static constexpr Byte OP_SBC_AX = 0xFD; // 3 length, 4+ cycles
        static constexpr Byte OP_SBC_AY = 0xF9; // 3 length, 4+ cycles
        static constexpr Byte OP_SBC_IX = 0xE1; // 2 length, 6 cycles
        static constexpr Byte OP_SBC_IY = 0xF1; // 2 length, 5+ cycles

        /* STA - Store Accumulator*/    // FLAGS : NONE
        static constexpr Byte OP_STA_ZP = 0x85; // 2 length, 3 cycles
        static constexpr Byte OP_STA_ZPY = 0x95; // 2 length, 3 cycles
        static constexpr Byte OP_STA_A = 0x8D; // 3 length, 4 cycles
        static constexpr Byte OP_STA_AX = 0x9D; // 3 length, 5 cycles
        static constexpr Byte OP_STA_AY = 0x99; // 3 length, 5 cycles
        static constexpr Byte OP_STA_IX = 0x81; // 2 length, 6 cycles
        static constexpr Byte OP_STA_IY = 0x91; // 2 length, 6 cycles

        /* Stack Instructions */    //  FLAGS : N/A     // 1 length
        static constexpr Byte OP_TXS = 0x9A; // (Transfer X to to stack ptr) // 2 cycles
        static constexpr Byte OP_TSX = 0xBA; // (Transfer stack ptr to x) // 2 cycles
        static constexpr Byte OP_PHA = 0x48; // (push accumulator) // 3 cycles
        static constexpr Byte OP_PLA = 0x68; // (pull accumulator status) // 4  cycles
        static constexpr Byte OP_PHP = 0x08; // (PusH processor status) // 3 cycles
        static constexpr Byte OP_PLP = 0x28; // (pull processtor status) // 4 cycles

        /* STX - Store X register */    // FLAGS : NONE
        static constexpr Byte OP_STX_ZP = 0x86; // 2 length, 3 cycles
        static constexpr Byte OP_STX_ZPY = 0x96;// 2 length, 4 cycles
        static constexpr Byte OP_STX_A = 0x8E; // 3 length, 4 cycles

        /* STY - Store Y register */    // FLAGS : NONE
        static constexpr Byte OP_STY_ZP = 0x84; // 2 length, 3 cycles
        static constexpr Byte OP_STY_ZPX = 0x94;// 2 length, 4 cycles
        static constexpr Byte OP_STY_A = 0x8C; // 3 length, 4 cycles



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

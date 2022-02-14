#pragma once
#include <iostream>
#include <array>
#include <vector>

#include "utility.h"

namespace e6502{
    
    struct MEM{
        static constexpr u32 MAX_MEM = 1024 * 64;
        Byte Data[MAX_MEM];

        void INIT(){
            for (u32 i = 0; i < MAX_MEM; i++){
                Data[i] = 0;
            }
        }
        //read byte
        inline Byte operator[](u32 Address) const {
            return Data[Address];
        }
        inline Byte &operator[](u32 Address){
            return Data[Address];
        }
    };

    struct ProcessorStatus{


        Byte V : 1; // OVERFLOW status 
        Byte N : 1; // NEGATIVE status 

        Byte C : 1; // CARRY status 
        Byte Z : 1; // ZERO  status 
        Byte I : 1; // INTERRUPT status 
        Byte D : 1; // Decimal status 
        Byte B : 1; // BREAK status 

    };

    

    //6502
    struct CPU{
        
        InstructionTable table;

        Byte A;     //accumulator
        Byte X;     //index 
        Byte Y;     //index
        
        Word SP;    // stack pointer
        Word PC;    //program counter

        uintmax_t InitCycles = UINTMAX_MAX;
        uintmax_t PostCycles = 0;

        void operator++(){
            PostCycles++;
            InitCycles--;
        }

        union {
            Byte Status;
            ProcessorStatus Flag;
        };

        inline void RESET( e6502::MEM &memory){
            this->PC = 0xFFFC;
            this->Flag.D = 0;
            this->SP = 0x0FF;

            this->A = 0;
            this->X = 0;
            this->Y = 0;
            
            this->Flag.C = 0;
            this->Flag.Z = 0;
            this->Flag.I = 0;
            this->Flag.D = 0; 
            this->Flag.B = 0;
            this->Flag.V = 0;
            this->Flag.N = 0;

            memory.INIT();
            return;
        }
        //byte
        inline Byte FETCH( e6502::MEM &memory ){
            Byte Data = memory[PC];
            PC++;
            this->operator++();
            return Data;
        }

        inline void READ( e6502::MEM &memory ){
            Byte Data = memory[PC];
            //PC++;
            this->operator++();
            return;
        }


        inline void EXECUTE( e6502::MEM &memory ){
            while (InitCycles > 0) {
                Byte instruction = FETCH( memory );
                table[instruction]();
    
            }
        }
    };
}

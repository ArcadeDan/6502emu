#include <iostream>
#include "cpu.h"


int main (int argc, char* argv[]){
    e6502::CPU cpu6502;
    e6502::MEM memory;
    cpu6502.RESET( memory );
    memory[0xFFFC] = e6502::OP_LDA;
    memory[0xFFFD] = 0x32;
    cpu6502.EXECUTE(2, memory);
    return 0;
}
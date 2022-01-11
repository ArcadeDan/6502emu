#include <iostream>
#include "cpu.h"


int main (int argc, char* argv[]){
    cpu::CPU cpu6502;
    cpu::MEM memory;
    cpu6502.RESET( memory );
    cpu6502.EXECUTE(2, memory);
    return 0;
}
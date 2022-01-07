#pragma once

#include <cstdint>


struct CPU{
// registers
    uint8_t A;  //accumulator
    uint8_t X;  //index 
    uint8_t Y;  //index
    uint8_t Status;
    uint8_t SP; // stack pointer
    uint16_t PC; //program counter



};
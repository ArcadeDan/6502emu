# 6502 Emulator
![mos 6502](https://upload.wikimedia.org/wikipedia/commons/4/49/MOS_6502AD_4585_top.jpg)

# REPL Commands

    setbyte: destination, source
	    example: "setbyte 0000 A9"
		    set memory address at 0x0000 to have a byte 0xA9
	
	getbyte: destination
		example: "getbyte 0000"
			retrieves byte at address
	reg:
		example: "reg"
			prints register information
			
	status:
		example: "status"
			prints status register information
	
	execute:
		example: "execute"
			executes instruction at programcounter address
	

# Instructions so far

    LDA : 0xA9
    JMP : 0x4C
    PHA : 0x48
    PLA : 0x68
    TXS : 0x9A
    TSX : 0xBA


# References
* http://www.6502.org/tutorials/6502opcodes.html
* https://www.cs.jhu.edu/~phi/csf/slides/lecture-6502-stack.pdf
* https://www.masswerk.at/6502/6502_instruction_set.html
    
    

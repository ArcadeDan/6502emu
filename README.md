# A MOS6502 Emulator written in Rust
![mos 6502](https://upload.wikimedia.org/wikipedia/commons/4/49/MOS_6502AD_4585_top.jpg)

# Motivations
Hello, I am among a group that really appreciates the mos6502. Super Mario Brothers was the architecture that it was written in.
The programming that got us to the moon in 1969 was written on a 6502 architecture. In short, the merits are there. I believe it also
to be a simple enough architecture to learn. I intend for this to be a project I can learn from.


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
	


# References
* http://www.6502.org/tutorials/6502opcodes.html
* https://www.cs.jhu.edu/~phi/csf/slides/lecture-6502-stack.pdf
* https://www.masswerk.at/6502/6502_instruction_set.html
* https://www.masswerk.at/6502/#
* https://www.masswerk.at/6502/assembler.html
* https://www.pagetable.com/c64ref/6502/?tab=2
* http://www.emulator101.com/6502-addressing-modes.html#:~:text=The%206502%20has%20the%20ability,to%20the%20address%20being%20accessed.&text=This%20addressing%20mode%20makes%20the,register%20to%20an%20absolute%20address.
    

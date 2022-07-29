# 8Bit Virtual Machine

The virtual machine is _not_ based on any real machine.

## CPU

## Registers

The CPU has 6 registers, these can be referenced via cpu::Registers. All registers have a size of 8Bit.

### IP

This register is used to store the instruction pointer, the instruction pointer points to the address at which the next instruction is located.

## ACC

The ACC register stores the last result from an arimethric or logical instruction.

## R1-4

These 4 registers can be used by the developers to store any data.

## Instructions

| Name      | Op-Code | Description                                        |
|-----------|---------|----------------------------------------------------|
| MovLitR1  | 0x10    | Moves a literal to R1                              |
| MovLitR2  | 0x11    | Moves a literal to R2                              |
| MovLitR3  | 0x12    | Moves a literal to R3                              |
| MovLitR4  | 0x13    | Moves a literal to R4                              |
| MovRegR1  | 0x14    | Moves a register to R1                             |
| MovRegR2  | 0x15    | Moves a register to R2                             |
| MovRegR3  | 0x16    | Moves a register to R3                             |
| MovRegR4  | 0x17    | Moves a register to R4                             |
| AddRegReg | 0x20    | Adds two registers                                 |
| AddRegLit | 0x21    | Adds a literal and a register                      |
| OrRegReg  | 0x22    | Performs a logical OR on two registers             |
| OrRegLit  | 0x23    | Performs a logical OR on a register and a literal  |
| AndRegReg | 0x24    | Performs a logical AND on two registers            |
| AndRegLit | 0x25    | Performs a logical AND on a register and a literal |
| XorRegReg | 0x26    | Performs a logical XOR on two registers            |
| XorRegLit | 0x27    | Performs a logical XOR on a register and a literal |
| NotReg    | 0x28    | Performs a logical NOT on a registers              |
| NotLit    | 0x29    | Performs a logical NOT on a literal                |
| SubRegReg | 0x30    | Subtracts the first register from the second       |
| SubRegLit | 0x31    | Subtracts a literal from a register                |
| SubLitReg | 0x32    | Subtracts a register from a literal                |
| MulRegReg | 0x33    | Multiplies two registers                           |
| MulRegLit | 0x34    | Multiplies a register with a literal               |
| DivRegReg | 0x35    | Divides two registers                              |
| DivRegLit | 0x36    | Divides a literal from a register                  |
| DivLitReg | 0x37    | Divides a register from a literal                  |
| ModRegReg | 0x38    | Modulos two registers                              |
| ModRegLit | 0x39    | Modulos a literal from a register                  |
| ModLitReg | 0x40    | Modulos a register from a literal                  |

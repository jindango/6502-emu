use std::collections::HashMap;



struct Opcode {


}


fn gen_opcodes() -> HashMap<u16, u16> {
    let mut opcodes = HashMap::new();
    return opcodes;

}

enum AddressingMode {
    Accumulator,     // Accum
    Imediate,        // IMM
    Absolute,        // Absolute
    ZeroPage,        // ZP
    IndexedZeroPage, // ZP, x or y
    IndexedAbsolute, // ABS, x or y
    Implied,         // Implied
    Relative,        // Relative
    IndexedIndirect, // IND x
    IndirectIndexed, // IND y
    AbsoluteIndirect // Indirect
}

enum CPUFlag { 
    C = 1 << 0, // Carry bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable Interrupts 
    D = 1 << 3, // Decimal mode
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
        V = 1 << 6, // Overflow
    N = 1 << 7 // Negative
}

struct CPU {
    // registers
    a : u8,
    x : u8,
    y : u8,
    sp : u8,
    pc : u16,
    status : u8
}

struct Bus {
    cpu : CPU,
    memory : [u8; 64 * 1024], // 64Kb
}

impl CPU {
    fn ADC() {}
    fn AND() {}
    fn ASL() {}
    fn BCC() {}
    fn BCS() {}
    fn BIT() {}
    fn BMI() {}
    fn BNE() {}
    fn BPL() {}
    fn BRK() {}
    fn BVC() {}
    fn CLC() {}
    fn CLD() {}
    fn CLI() {}
    fn CLV() {}
    fn CMP() {}
    fn CPX() {}
    fn CPY() {}
    fn DEC() {}
    fn DEX() {}
    fn DEY() {}
    fn EOR() {}
    fn INC() {}
    fn INX() {}
    fn INY() {}
    fn JMP() {}
    fn JSR() {}
    fn LDA() {}
    fn LDX() {}
    fn LDY() {}
    fn LSR() {}
    fn NOP() {}
    fn ORA() {}
    fn PHA() {}
    fn PHP() {}
    fn ROL() {}
    fn ROR() {}
    fn RTI() {}
    fn RTS() {}
    fn SBC() {}
    fn SEC() {}
    fn SED() {}
    fn SEI() {}
    fn STA() {}
    fn STX() {}
    fn STY() {}
    fn TAX() {}
    fn TAY() {}
    fn TSX() {}
    fn TXA() {}
    fn TXS() {}
    fn TYA() {}

    fn clock (&self) { 

    }
}

impl Bus {
    fn write(&self, data : u8, address : u8)  {
        
    }
    fn read(&self, address : u16) -> u8 {
        return self.memory[address as usize];
    }
}

fn main() {

}

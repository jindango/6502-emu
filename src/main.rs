use std::collections::HashMap;

struct Opcode {
    instruction : fn(),
    addressing_mode : fn(),
    size : u8,
    cycles : u8
}

fn gen_opcodes() -> HashMap<u16, Opcode> {
    let mut opcodes = HashMap::new(); 
    opcodes.insert(0x00, Opcode{instruction : CPU::BRK,
                                addressing_mode : CPU::Implied,
                                cycles : 1,
                                size : 7});
    opcodes.insert(0x01, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::INDX,
                                cycles : 2,
                                size : 6});
    opcodes.insert(0x05, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::INDX,
                                cycles : 2,
                                size : 6});
    return opcodes;

}

enum CPUFlag { 
    C = 1 << 0, // Carry bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable Interrupts 
    D = 1 << 3, // Decimal mode
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overflow
    N = 1 << 7  // Negative
}

struct CPU {
    // registers
    a : u8,
    x : u8,
    y : u8,
    sp : u8,
    pc : u16,
    status : u8, 

    // Made u
}

struct Bus {
    cpu : CPU,
    // ppu : PPU, // TODO
    // vmemory : [u8; 64 * 1024], // TODO
    memory : [u8; 64 * 1024], // 64Kb
}

impl CPU {
    // Addressing modes
    fn Accum() {}
    fn IMM() {}
    fn ZP() {}
    fn ZPX() {}
    fn ZPY() {}
    fn ABSX() {}
    fn ABSY() {}
    fn Implied() {}
    fn Relative() {}
    fn INDX() {}
    fn INDY() {}
    fn Indirect() {}

    // Instructions
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

    fn reset(&self) {

    }
}

impl Bus {
    fn write(&mut self, data : u8, address : u8)  {
        self.memory[address as usize] = data;
    }

    fn read(&self, address : u16) -> u8 {
        return self.memory[address as usize];
    }
}

fn main() {

}

use std::collections::HashMap;

#[derive(Debug)]
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
    opcodes.insert(0x06, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ZP,
                                cycles : 2,
                                size : 5});
    opcodes.insert(0x08, Opcode{instruction : CPU::PHP,
                                addressing_mode : CPU::Implied,
                                cycles : 1,
                                size : 3});
    opcodes.insert(0x09, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::IMM,
                                cycles : 2,
                                size : 2});
    opcodes.insert(0x0A, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::Accum,
                                cycles : 1,
                                size : 2});
    opcodes.insert(0x0D, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ABS,
                                cycles : 3,
                                size : 4});
    opcodes.insert(0x0E, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ABS,
                                cycles : 3,
                                size : 6});
    opcodes.insert(0x10, Opcode{instruction : CPU::BPL,
                                addressing_mode : CPU::Relative,
                                cycles : 2,
                                size : 2});
    opcodes.insert(0x11, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::INDY,
                                cycles : 2,
                                size : 5});
    opcodes.insert(0x15, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ZPX,
                                cycles : 2,
                                size : 4});
    opcodes.insert(0x17, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ZPX,
                                cycles : 2,
                                size : 6});
    opcodes.insert(0x18, Opcode{instruction : CPU::CLC,
                                addressing_mode : CPU::Implied,
                                cycles : 1,
                                size : 2});
    opcodes.insert(0x19, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ABSY,
                                cycles : 3,
                                size : 4});
    opcodes.insert(0x1D, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ABSX,
                                cycles : 3,
                                size : 4});
    opcodes.insert(0x1E, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ABSX,
                                cycles : 3,
                                size : 7});
    opcodes.insert(0x20, Opcode{instruction : CPU::JSR,
                                addressing_mode : CPU::ABS,
                                cycles : 3,
                                size : 6});
    opcodes.insert(0x21, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::INDX,
                                cycles : 2,
                                size : 6});
    opcodes.insert(0x24, Opcode{instruction : CPU::BIT,
                                addressing_mode : CPU::ZP,
                                cycles : 2,
                                size : 3});
    opcodes.insert(0x25, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ZP,
                                cycles : 2,
                                size : 3});
    opcodes.insert(0x26, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ZP,
                                cycles : 2,
                                size : 5});
    opcodes.insert(0x28, Opcode{instruction : CPU::PLP,
                                addressing_mode : CPU::Implied,
                                cycles : 1,
                                size : 4});
    opcodes.insert(0x29, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::IMM,
                                cycles : 2,
                                size : 2});
    opcodes.insert(0x2A, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::Accum,
                                cycles : 1,
                                size : 2});
    opcodes.insert(0x2C, Opcode{instruction : CPU::BIT,
                                addressing_mode : CPU::ABS,
                                cycles : 3,
                                size : 4});
    opcodes.insert(0x2D, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ABS,
                                cycles : 3,
                                size : 4});
    opcodes.insert(0x2D, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ABS,
                                cycles : 3,
                                size : 6});
    opcodes.insert(0x30, Opcode{instruction : CPU::BMI,
                                addressing_mode : CPU::Relative,
                                cycles : 2,
                                size : 2});
    opcodes.insert(0x31, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::INDY,
                                cycles : 2,
                                size : 5});
    opcodes.insert(0x35, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ZPX,
                                cycles : 2,
                                size : 4});
    opcodes.insert(0x36, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ZPX,
                                cycles : 2,
                                size : 6});
    opcodes.insert(0x38, Opcode{instruction : CPU::SEC,
                                addressing_mode : CPU::Implied,
                                cycles : 1,
                                size : 2});
    opcodes.insert(0x39, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ABSY,
                                cycles : 3,
                                size : 4});
    opcodes.insert(0x3D, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ABSX,
                                cycles : 3,
                                size : 4});
    opcodes.insert(0x3E, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ABSX,
                                cycles : 3,
                                size : 7});


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
    fn ABS() {}
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
    let mut map : HashMap<u16, Opcode>;
    map = gen_opcodes();

    for (value, opcode) in &map {
        (opcode.instruction)();
        (opcode.addressing_mode)();
    }
}

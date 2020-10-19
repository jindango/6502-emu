use std::collections::HashMap;

#[derive(Debug)]
struct Opcode {
    instruction : fn(),
    addressing_mode : fn(),
    size: u8,
    cycles: u8
}

fn gen_opcodes() -> HashMap<u16, Opcode> {
    let mut opcodes = HashMap::new(); 
    opcodes.insert(0x00, Opcode{instruction : CPU::BRK,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 7});
    opcodes.insert(0x01, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::INDX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x05, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::INDX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x06, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x08, Opcode{instruction : CPU::PHP,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 3});
    opcodes.insert(0x09, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::IMM,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x0A, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::Accum,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x0D, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x0E, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 6});
    opcodes.insert(0x10, Opcode{instruction : CPU::BPL,
                                addressing_mode : CPU::Relative,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x11, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::INDY,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x15, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 4});
    opcodes.insert(0x17, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x18, Opcode{instruction : CPU::CLC,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x19, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ABSY,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x1D, Opcode{instruction : CPU::ORA,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x1E, Opcode{instruction : CPU::ASL,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 7});
    opcodes.insert(0x20, Opcode{instruction : CPU::JSR,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 6});
    opcodes.insert(0x21, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::INDX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x24, Opcode{instruction : CPU::BIT,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 3});
    opcodes.insert(0x25, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 3});
    opcodes.insert(0x26, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x28, Opcode{instruction : CPU::PLP,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 4});
    opcodes.insert(0x29, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::IMM,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x2A, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::Accum,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x2C, Opcode{instruction : CPU::BIT,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x2D, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x2D, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 6});
    opcodes.insert(0x30, Opcode{instruction : CPU::BMI,
                                addressing_mode : CPU::Relative,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x31, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::INDY,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x35, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 4});
    opcodes.insert(0x36, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x38, Opcode{instruction : CPU::SEC,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x39, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ABSY,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x3D, Opcode{instruction : CPU::AND,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x3E, Opcode{instruction : CPU::ROL,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 7});
    opcodes.insert(0x40, Opcode{instruction : CPU::RTI,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 6});
    opcodes.insert(0x41, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::INDX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x45, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 3});
    opcodes.insert(0x46, Opcode{instruction : CPU::LSR,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x48, Opcode{instruction : CPU::PHA,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 3});
    opcodes.insert(0x49, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::IMM,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x4A, Opcode{instruction : CPU::LSR,
                                addressing_mode : CPU::Accum,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x4C, Opcode{instruction : CPU::JMP,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 3});
    opcodes.insert(0x4D, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x4E, Opcode{instruction : CPU::LSR,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 6});
    opcodes.insert(0x50, Opcode{instruction : CPU::BVC,
                                addressing_mode : CPU::Relative,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x51, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::INDY,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x55, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 4});
    opcodes.insert(0x56, Opcode{instruction : CPU::LSR,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x58, Opcode{instruction : CPU::CLI,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x59, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::ABSY,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x5D, Opcode{instruction : CPU::EOR,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x5E, Opcode{instruction : CPU::LSR,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 7});
    opcodes.insert(0x60, Opcode{instruction : CPU::RTS,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 6});
    opcodes.insert(0x61, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::INDX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x65, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 3});
    opcodes.insert(0x66, Opcode{instruction : CPU::ROR,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x68, Opcode{instruction : CPU::PLA,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 4});
    opcodes.insert(0x69, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::IMM,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x6A, Opcode{instruction : CPU::ROR,
                                addressing_mode : CPU::Accum,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x6C, Opcode{instruction : CPU::JMP,
                                addressing_mode : CPU::Indirect,
                                size : 3,
                                cycles : 5});
    opcodes.insert(0x6D, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x6E, Opcode{instruction : CPU::ROR,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 6});
    opcodes.insert(0x70, Opcode{instruction : CPU::BVS,
                                addressing_mode : CPU::Relative,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x71, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::INDY,
                                size : 2,
                                cycles : 5});
    opcodes.insert(0x75, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 4});
    opcodes.insert(0x76, Opcode{instruction : CPU::ROR,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x78, Opcode{instruction : CPU::SEI,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x79, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::ABSY,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x7D, Opcode{instruction : CPU::ADC,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x7E, Opcode{instruction : CPU::ROR,
                                addressing_mode : CPU::ABSX,
                                size : 3,
                                cycles : 7});
    opcodes.insert(0x81, Opcode{instruction : CPU::STA,
                                addressing_mode : CPU::INDX,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x84, Opcode{instruction : CPU::STY,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 3});
    opcodes.insert(0x85, Opcode{instruction : CPU::STA,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 3});
    opcodes.insert(0x86, Opcode{instruction : CPU::STX,
                                addressing_mode : CPU::ZP,
                                size : 2,
                                cycles : 3});
    opcodes.insert(0x88, Opcode{instruction : CPU::DEY,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x8A, Opcode{instruction : CPU::TXA,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x8C, Opcode{instruction : CPU::STY,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x8D, Opcode{instruction : CPU::STA,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x8E, Opcode{instruction : CPU::STX,
                                addressing_mode : CPU::ABS,
                                size : 3,
                                cycles : 4});
    opcodes.insert(0x90, Opcode{instruction : CPU::BCC,
                                addressing_mode : CPU::Relative,
                                size : 2,
                                cycles : 2});
    opcodes.insert(0x91, Opcode{instruction : CPU::STA,
                                addressing_mode : CPU::INDY,
                                size : 2,
                                cycles : 6});
    opcodes.insert(0x94, Opcode{instruction : CPU::STY,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 4});
    opcodes.insert(0x95, Opcode{instruction : CPU::STA,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 4});
    opcodes.insert(0x96, Opcode{instruction : CPU::STX,
                                addressing_mode : CPU::ZPX,
                                size : 2,
                                cycles : 4});
    opcodes.insert(0x98, Opcode{instruction : CPU::TYA,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x99, Opcode{instruction : CPU::STA,
                                addressing_mode : CPU::ABSY,
                                size : 3,
                                cycles : 5});
    opcodes.insert(0x9A, Opcode{instruction : CPU::TXS,
                                addressing_mode : CPU::Implied,
                                size : 1,
                                cycles : 2});
    opcodes.insert(0x9D, Opcode{instruction : CPU::STA,
                            addressing_mode : CPU::ABSX,
                            size : 3,
                            cycles : 5});
    opcodes.insert(0xA0, Opcode{instruction : CPU::LDY,
                            addressing_mode : CPU::IMM,
                            size : 2,
                            cycles : 2});
    opcodes.insert(0xA1, Opcode{instruction : CPU::LDA,
                            addressing_mode : CPU::INDX,
                            size : 2,
                            cycles : 6});
    opcodes.insert(0xA2, Opcode{instruction : CPU::LDX,
                        addressing_mode : CPU::IMM,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xA4, Opcode{instruction : CPU::LDY,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 3});
    opcodes.insert(0xA5, Opcode{instruction : CPU::LDA,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 3});
    opcodes.insert(0xA6, Opcode{instruction : CPU::LDX,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 3});
    opcodes.insert(0xA8, Opcode{instruction : CPU::TAY,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xA9, Opcode{instruction : CPU::LDA,
                        addressing_mode : CPU::IMM,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xAA, Opcode{instruction : CPU::TAX,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xAC, Opcode{instruction : CPU::LDY,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xAD, Opcode{instruction : CPU::LDA,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xAE, Opcode{instruction : CPU::LDX,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xAE, Opcode{instruction : CPU::LDX,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xB0, Opcode{instruction : CPU::BCS,
                        addressing_mode : CPU::Relative,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xB1, Opcode{instruction : CPU::LDA,
                        addressing_mode : CPU::INDY,
                        size : 2,
                        cycles : 5});
    opcodes.insert(0xB4, Opcode{instruction : CPU::LDY,
                        addressing_mode : CPU::ZPX,
                        size : 2,
                        cycles : 4});
    opcodes.insert(0xB5, Opcode{instruction : CPU::LDA,
                        addressing_mode : CPU::ZPX,
                        size : 2,
                        cycles : 4});
    opcodes.insert(0xB6, Opcode{instruction : CPU::LDX,
                        addressing_mode : CPU::ZPY,
                        size : 2,
                        cycles : 4});
    opcodes.insert(0xB8, Opcode{instruction : CPU::CLV,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xB9, Opcode{instruction : CPU::LDA,
                        addressing_mode : CPU::ABSY,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xBA, Opcode{instruction : CPU::TSX,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xBC, Opcode{instruction : CPU::LDY,
                        addressing_mode : CPU::ABSX,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xBD, Opcode{instruction : CPU::LDA,
                        addressing_mode : CPU::ABSX,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xBE, Opcode{instruction : CPU::LDX,
                        addressing_mode : CPU::ABSY,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xC0, Opcode{instruction : CPU::CPY,
                        addressing_mode : CPU::IMM,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xC1, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::INDX,
                        size : 2,
                        cycles : 6});
    opcodes.insert(0xC4, Opcode{instruction : CPU::CPY,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 3});
    opcodes.insert(0xC5, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 3});
    opcodes.insert(0xC6, Opcode{instruction : CPU::DEC,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 5});
    opcodes.insert(0xC8, Opcode{instruction : CPU::INY,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xC9, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::IMM,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xCA, Opcode{instruction : CPU::DEX,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xCC, Opcode{instruction : CPU::CPY,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xCD, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xCE, Opcode{instruction : CPU::DEC,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 6});
    opcodes.insert(0xD0, Opcode{instruction : CPU::BNE,
                        addressing_mode : CPU::Relative,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xD1, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::INDY,
                        size : 2,
                        cycles : 5});
    opcodes.insert(0xD5, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::ZPX,
                        size : 2,
                        cycles : 4});
    opcodes.insert(0xD6, Opcode{instruction : CPU::DEC,
                        addressing_mode : CPU::ZPX,
                        size : 2,
                        cycles : 6});
    opcodes.insert(0xD8, Opcode{instruction : CPU::CLD,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xD9, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::ABSY,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xDD, Opcode{instruction : CPU::CMP,
                        addressing_mode : CPU::ABSX,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xDE, Opcode{instruction : CPU::DEC,
                        addressing_mode : CPU::ABSX,
                        size : 3,
                        cycles : 7});
    opcodes.insert(0xE0, Opcode{instruction : CPU::CPX,
                        addressing_mode : CPU::IMM,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xE1, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::INDX,
                        size : 2,
                        cycles : 6});
    opcodes.insert(0xE4, Opcode{instruction : CPU::CPX,
                        addressing_mode : CPU::ZP,
                        size : 3,
                        cycles : 3});
    opcodes.insert(0xE5, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 3});
    opcodes.insert(0xE7, Opcode{instruction : CPU::INC,
                        addressing_mode : CPU::ZP,
                        size : 2,
                        cycles : 5});
    opcodes.insert(0xE8, Opcode{instruction : CPU::INX,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xE9, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::IMM,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xEA, Opcode{instruction : CPU::NOP,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xEC, Opcode{instruction : CPU::CPX,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xED, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xEE, Opcode{instruction : CPU::INC,
                        addressing_mode : CPU::ABS,
                        size : 3,
                        cycles : 6});
    opcodes.insert(0xF0, Opcode{instruction : CPU::BEQ,
                        addressing_mode : CPU::Relative,
                        size : 2,
                        cycles : 2});
    opcodes.insert(0xF1, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::INDY,
                        size : 2,
                        cycles : 5});
    opcodes.insert(0xF5, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::ZPX,
                        size : 2,
                        cycles : 4});
    opcodes.insert(0xF6, Opcode{instruction : CPU::INC,
                        addressing_mode : CPU::ZPX,
                        size : 2,
                        cycles : 6});
    opcodes.insert(0xF8, Opcode{instruction : CPU::SED,
                        addressing_mode : CPU::Implied,
                        size : 1,
                        cycles : 2});
    opcodes.insert(0xF9, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::ABSY,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xFD, Opcode{instruction : CPU::SBC,
                        addressing_mode : CPU::ABSX,
                        size : 3,
                        cycles : 4});
    opcodes.insert(0xFE, Opcode{instruction : CPU::INC,
                        addressing_mode : CPU::ABSX,
                        size : 3,
                        cycles : 7});












































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
    fn BEQ() {}
    fn BCS() {}
    fn BIT() {}
    fn BMI() {}
    fn BNE() {}
    fn BPL() {}
    fn BRK() {}
    fn BVC() {}
    fn BVS() {}
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
    fn PLP() {}
    fn PLA() {}

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

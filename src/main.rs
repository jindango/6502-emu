enum AddressingMode {
    Accumulator, // Accum
    Imediate, // IMM
    Absolute, // Absolute
    ZeroPage, // ZP
    IndexedZeroPage, // ZP, x or y
    IndexedAbsolute, // ABS, x or y
    Implied, // Implied
    Relative, // Relative
    IndexedIndirect, // IND X
    IndirectIndexed, // IND Y
    AbsoluteIndirect // Indirect
}
enum Flag { 
    c = 1 << 0,
    z = 1 << 1,
    i = 1 << 2,
    d = 1 << 3,
    b = 1 << 4,
    u = 1 << 5,
    v = 1 << 6,
    n = 1 << 7
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
struct Instruction {
    raw : u32,
    nm : u8,
    mode : AddressingMode 
}

fn main() {
    let mut memory : [u8; 64 * 1024];
    let mut cpu : CPU;
}

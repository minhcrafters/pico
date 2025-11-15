use crate::cpu::AddressingMode;
use std::sync::LazyLock;

pub struct Opcode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    #[allow(dead_code)] // not yet cycle-accurate
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

impl Opcode {
    pub const fn new(
        code: u8,
        mnemonic: &'static str,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        Opcode {
            code,
            mnemonic,
            bytes,
            cycles,
            addressing_mode,
        }
    }
}

pub struct OpcodeMap {
    opcodes: Vec<Opcode>,
}

impl OpcodeMap {
    pub fn new() -> Self {
        OpcodeMap {
            opcodes: vec![
                // ADC
                Opcode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
                Opcode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
                Opcode::new(0x7D, "ADC", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0x79, "ADC", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0x61, "ADC", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0x71, "ADC", 2, 5, AddressingMode::IndirectY),
                // AND
                Opcode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
                Opcode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
                Opcode::new(0x3D, "AND", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0x39, "AND", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0x31, "AND", 2, 5, AddressingMode::IndirectY),
                // ASL
                Opcode::new(0x0A, "ASL", 1, 2, AddressingMode::Accumulator),
                Opcode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
                Opcode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPageX),
                Opcode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
                Opcode::new(0x1E, "ASL", 3, 7, AddressingMode::AbsoluteX),
                // BCC
                Opcode::new(0x90, "BCC", 2, 2, AddressingMode::Relative),
                // BCS
                Opcode::new(0xB0, "BCS", 2, 2, AddressingMode::Relative),
                // BEQ
                Opcode::new(0xF0, "BEQ", 2, 2, AddressingMode::Relative),
                // BIT
                Opcode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),
                // BMI
                Opcode::new(0x30, "BMI", 2, 2, AddressingMode::Relative),
                // BNE
                Opcode::new(0xD0, "BNE", 2, 2, AddressingMode::Relative),
                // BPL
                Opcode::new(0x10, "BPL", 2, 2, AddressingMode::Relative),
                // BRK
                Opcode::new(0x00, "BRK", 1, 7, AddressingMode::None),
                // BVC
                Opcode::new(0x50, "BVC", 2, 2, AddressingMode::Relative),
                // BVS
                Opcode::new(0x70, "BVS", 2, 2, AddressingMode::Relative),
                // CLC
                Opcode::new(0x18, "CLC", 1, 2, AddressingMode::None),
                // CLD
                Opcode::new(0xD8, "CLD", 1, 2, AddressingMode::None),
                // CLI
                Opcode::new(0x58, "CLI", 1, 2, AddressingMode::None),
                // CLV
                Opcode::new(0xB8, "CLV", 1, 2, AddressingMode::None),
                // CMP
                Opcode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
                Opcode::new(0xDD, "CMP", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0xD9, "CMP", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0xC1, "CMP", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0xD1, "CMP", 2, 5, AddressingMode::IndirectY),
                // CPX
                Opcode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute),
                // CPY
                Opcode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute),
                // DEC
                Opcode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
                Opcode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPageX),
                Opcode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
                Opcode::new(0xDE, "DEC", 3, 7, AddressingMode::AbsoluteX),
                // DEX
                Opcode::new(0xCA, "DEX", 1, 2, AddressingMode::None),
                // DEY
                Opcode::new(0x88, "DEY", 1, 2, AddressingMode::None),
                // EOR
                Opcode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
                Opcode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute),
                Opcode::new(0x5D, "EOR", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0x59, "EOR", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0x41, "EOR", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0x51, "EOR", 2, 5, AddressingMode::IndirectY),
                // INC
                Opcode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),
                Opcode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPageX),
                Opcode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
                Opcode::new(0xFE, "INC", 3, 7, AddressingMode::AbsoluteX),
                // INX
                Opcode::new(0xE8, "INX", 1, 2, AddressingMode::None),
                // INY
                Opcode::new(0xC8, "INY", 1, 2, AddressingMode::None),
                // JMP
                Opcode::new(0x4C, "JMP", 3, 3, AddressingMode::Absolute),
                Opcode::new(0x6C, "JMP", 3, 5, AddressingMode::Indirect),
                // JSR
                Opcode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),
                // LDA
                Opcode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
                Opcode::new(0xBD, "LDA", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0xB9, "LDA", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0xB1, "LDA", 2, 5, AddressingMode::IndirectY),
                // LDX
                Opcode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPageY),
                Opcode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
                Opcode::new(0xBE, "LDX", 3, 4, AddressingMode::AbsoluteY),
                // LDY
                Opcode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
                Opcode::new(0xBC, "LDY", 3, 4, AddressingMode::AbsoluteX),
                // LSR
                Opcode::new(0x4A, "LSR", 1, 2, AddressingMode::Accumulator),
                Opcode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
                Opcode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPageX),
                Opcode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
                Opcode::new(0x5E, "LSR", 3, 7, AddressingMode::AbsoluteX),
                // NOP
                Opcode::new(0xEA, "NOP", 1, 2, AddressingMode::None),
                // ORA
                Opcode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
                Opcode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute),
                Opcode::new(0x1D, "ORA", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0x19, "ORA", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0x01, "ORA", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0x11, "ORA", 2, 5, AddressingMode::IndirectY),
                // PHA
                Opcode::new(0x48, "PHA", 1, 3, AddressingMode::None),
                // PHP
                Opcode::new(0x08, "PHP", 1, 3, AddressingMode::None),
                // PLA
                Opcode::new(0x68, "PLA", 1, 4, AddressingMode::None),
                // PLP
                Opcode::new(0x28, "PLP", 1, 4, AddressingMode::None),
                // ROL
                Opcode::new(0x2A, "ROL", 1, 2, AddressingMode::Accumulator),
                Opcode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
                Opcode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPageX),
                Opcode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
                Opcode::new(0x3E, "ROL", 3, 7, AddressingMode::AbsoluteX),
                // ROR
                Opcode::new(0x6A, "ROR", 1, 2, AddressingMode::Accumulator),
                Opcode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
                Opcode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPageX),
                Opcode::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute),
                Opcode::new(0x7E, "ROR", 3, 7, AddressingMode::AbsoluteX),
                // RTI
                Opcode::new(0x40, "RTI", 1, 6, AddressingMode::None),
                // RTS
                Opcode::new(0x60, "RTS", 1, 6, AddressingMode::None),
                // SBC
                Opcode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
                Opcode::new(0xFD, "SBC", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0xF9, "SBC", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0xE1, "SBC", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0xF1, "SBC", 2, 5, AddressingMode::IndirectY),
                // SEC
                Opcode::new(0x38, "SEC", 1, 2, AddressingMode::None),
                // SED
                Opcode::new(0xF8, "SED", 1, 2, AddressingMode::None),
                // SEI
                Opcode::new(0x78, "SEI", 1, 2, AddressingMode::None),
                // STA
                Opcode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
                Opcode::new(0x9D, "STA", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0x99, "STA", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),
                // STX
                Opcode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),
                Opcode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),
                // STY
                Opcode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),
                // TAX
                Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::None),
                // TAY
                Opcode::new(0xA8, "TAY", 1, 2, AddressingMode::None),
                // TSX
                Opcode::new(0xBA, "TSX", 1, 2, AddressingMode::None),
                // TXA
                Opcode::new(0x8A, "TXA", 1, 2, AddressingMode::None),
                // TXS
                Opcode::new(0x9A, "TXS", 1, 2, AddressingMode::None),
                // TYA
                Opcode::new(0x98, "TYA", 1, 2, AddressingMode::None),
            ],
        }
    }

    pub fn find_by_code(&self, code: u8) -> Option<&Opcode> {
        self.opcodes.iter().find(|opcode| opcode.code == code)
    }

    #[allow(dead_code)]
    pub fn get_opcodes(&self) -> &[Opcode] {
        &self.opcodes
    }
}

pub static CPU_OPCODES: LazyLock<OpcodeMap> = LazyLock::new(|| OpcodeMap::new());

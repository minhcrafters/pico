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
                // LDA
                Opcode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
                Opcode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
                Opcode::new(0xBD, "LDA", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0xB9, "LDA", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0xB1, "LDA", 2, 5, AddressingMode::IndirectY),
                // STA
                Opcode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
                Opcode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
                Opcode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
                Opcode::new(0x9D, "STA", 3, 4, AddressingMode::AbsoluteX),
                Opcode::new(0x99, "STA", 3, 4, AddressingMode::AbsoluteY),
                Opcode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
                Opcode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),
                // Other
                Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::None),
                Opcode::new(0xE8, "INX", 1, 2, AddressingMode::None),
                Opcode::new(0x00, "BRK", 1, 7, AddressingMode::None),
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

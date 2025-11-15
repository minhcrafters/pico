use std::fmt::Debug;

use bitflags::bitflags;

use crate::memory::Memory;
use crate::opcodes::CPU_OPCODES;

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    None,
}

bitflags! {
    pub struct StatusFlags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE = 0b0000_1000;
        const BREAK_COMMAND = 0b0001_0000;
        const UNUSED = 0b0010_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    status: StatusFlags,
    pc: u16,
    sp: u8,
}

impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {:02X}, X: {:02X}, Y: {:02X}, STATUS: {:08b}, PC: {:04X}",
            self.a, self.x, self.y, self.status, self.pc
        )
    }
}

pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                status: StatusFlags::empty(),
                pc: 0x8000,
                sp: 0x00,
            },
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory.read(self.registers.pc);
            self.registers.pc += 1;

            if let Some(opcode_info) = CPU_OPCODES.find_by_code(opcode) {
                match opcode_info.mnemonic {
                    "ADC" => {
                        self.adc(&opcode_info.addressing_mode);
                    }
                    "AND" => {
                        self.and(&opcode_info.addressing_mode);
                    }
                    "ASL" => {
                        self.asl(&opcode_info.addressing_mode);
                    }
                    "BCC" => {
                        self.bcc(&opcode_info.addressing_mode);
                    }
                    "BCS" => {
                        self.bcs(&opcode_info.addressing_mode);
                    }
                    "BEQ" => {
                        self.beq(&opcode_info.addressing_mode);
                    }
                    "BIT" => {
                        self.bit(&opcode_info.addressing_mode);
                    }
                    "BMI" => {
                        self.bmi(&opcode_info.addressing_mode);
                    }
                    "BNE" => {
                        self.bne(&opcode_info.addressing_mode);
                    }
                    "BPL" => {
                        self.bpl(&opcode_info.addressing_mode);
                    }
                    "BRK" => {
                        self.brk(&opcode_info.addressing_mode);
                        return;
                    }
                    "BVC" => {
                        self.bvc(&opcode_info.addressing_mode);
                    }
                    "BVS" => {
                        self.bvs(&opcode_info.addressing_mode);
                    }
                    "CLC" => {
                        self.clc();
                    }
                    "CLD" => {
                        self.cld();
                    }
                    "CLI" => {
                        self.cli();
                    }
                    "CLV" => {
                        self.clv();
                    }
                    "CMP" => {
                        self.cmp(&opcode_info.addressing_mode);
                    }
                    "CPX" => {
                        self.cpx(&opcode_info.addressing_mode);
                    }
                    "CPY" => {
                        self.cpy(&opcode_info.addressing_mode);
                    }
                    "DEC" => {
                        self.dec(&opcode_info.addressing_mode);
                    }
                    "DEX" => {
                        self.dex();
                    }
                    "DEY" => {
                        self.dey();
                    }
                    "EOR" => {
                        self.eor(&opcode_info.addressing_mode);
                    }
                    "INC" => {
                        self.inc(&opcode_info.addressing_mode);
                    }
                    "INX" => {
                        self.inx();
                    }
                    "INY" => {
                        self.iny();
                    }
                    "JMP" => {
                        self.jmp(&opcode_info.addressing_mode);
                    }
                    "JSR" => {
                        self.jsr(&opcode_info.addressing_mode);
                    }
                    "LDA" => {
                        self.lda(&opcode_info.addressing_mode);
                    }
                    "LDX" => {
                        self.ldx(&opcode_info.addressing_mode);
                    }
                    "LDY" => {
                        self.ldy(&opcode_info.addressing_mode);
                    }
                    "LSR" => {
                        self.lsr(&opcode_info.addressing_mode);
                    }
                    "NOP" => {
                        self.nop();
                    }
                    "ORA" => {
                        self.ora(&opcode_info.addressing_mode);
                    }
                    "PHA" => {
                        self.pha();
                    }
                    "PHP" => {
                        self.php();
                    }
                    "PLA" => {
                        self.pla();
                    }
                    "PLP" => {
                        self.plp();
                    }
                    "ROL" => {
                        self.rol(&opcode_info.addressing_mode);
                    }
                    "ROR" => {
                        self.ror(&opcode_info.addressing_mode);
                    }
                    "RTI" => {
                        self.rti();
                    }
                    "RTS" => {
                        self.rts();
                    }
                    "SBC" => {
                        self.sbc(&opcode_info.addressing_mode);
                    }
                    "SEC" => {
                        self.sec();
                    }
                    "SED" => {
                        self.sed();
                    }
                    "SEI" => {
                        self.sei();
                    }
                    "STA" => {
                        self.sta(&opcode_info.addressing_mode);
                    }
                    "STX" => {
                        self.stx(&opcode_info.addressing_mode);
                    }
                    "STY" => {
                        self.sty(&opcode_info.addressing_mode);
                    }
                    "TAX" => {
                        self.tax();
                    }
                    "TAY" => {
                        self.tay();
                    }
                    "TSX" => {
                        self.tsx();
                    }
                    "TXA" => {
                        self.txa();
                    }
                    "TXS" => {
                        self.txs();
                    }
                    "TYA" => {
                        self.tya();
                    }

                    _ => {
                        println!("Unknown mnemonic: {}", opcode_info.mnemonic);
                        break;
                    }
                }

                self.registers.pc += (opcode_info.bytes - 1) as u16;
            } else {
                println!("Unknown opcode: {:02X}", opcode);
                break;
            }
        }
    }

    pub fn reset(&mut self, clear_mem: bool) {
        self.registers.a = 0;
        self.registers.x = 0;
        self.registers.y = 0;
        self.registers.status = StatusFlags::empty();

        if clear_mem {
            self.memory.clear();
        }

        self.registers.pc = self.memory.read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>, start_addr: Option<u16>) {
        let load_addr = start_addr.unwrap_or(0x8000);
        self.memory.load(load_addr, &program);
        self.memory.write_u16(0xFFFC, load_addr);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>, start_addr: Option<u16>) {
        self.load(program, start_addr);
        self.reset(false);
        self.run();
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        let sum = self.registers.a as u16
            + value as u16
            + if self.registers.status.contains(StatusFlags::CARRY) {
                1
            } else {
                0
            };

        if sum > 0xFF {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        let result = sum as u8;

        // Set overflow flag
        if ((self.registers.a ^ result) & (value ^ result) & 0x80) != 0 {
            self.registers.status.insert(StatusFlags::OVERFLOW); // Set overflow flag
        } else {
            self.registers.status.remove(StatusFlags::OVERFLOW); // Clear overflow flag
        }

        self.registers.a = result;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        self.registers.a &= value;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Accumulator {
            let mut value = self.registers.a;

            if value & 0b1000_0000 != 0 {
                self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
            } else {
                self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
            }

            value <<= 1;
            self.registers.a = value;
            self.update_zero_and_negative_flags(self.registers.a);
            return;
        }

        let addr = self.get_operand_address(mode);
        let mut value = self.memory.read(addr);

        if value & 0b1000_0000 != 0 {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        value <<= 1;
        self.memory.write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn bcc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if !self.registers.status.contains(StatusFlags::CARRY) {
            self.registers.pc = addr;
        }
    }

    fn bcs(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if self.registers.status.contains(StatusFlags::CARRY) {
            self.registers.pc = addr;
        }
    }

    fn beq(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if self.registers.status.contains(StatusFlags::ZERO) {
            self.registers.pc = addr;
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        let result = self.registers.a & value;

        if result == 0 {
            self.registers.status.insert(StatusFlags::ZERO); // Set zero flag
        } else {
            self.registers.status.remove(StatusFlags::ZERO); // Clear zero flag
        }

        if value & 0b0100_0000 != 0 {
            self.registers.status.insert(StatusFlags::OVERFLOW); // Set overflow flag
        } else {
            self.registers.status.remove(StatusFlags::OVERFLOW); // Clear overflow flag
        }

        if value & 0b1000_0000 != 0 {
            self.registers.status.insert(StatusFlags::NEGATIVE); // Set negative flag
        } else {
            self.registers.status.remove(StatusFlags::NEGATIVE); // Clear negative flag
        }
    }

    fn bmi(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if self.registers.status.contains(StatusFlags::NEGATIVE) {
            self.registers.pc = addr;
        }
    }

    fn bne(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if !self.registers.status.contains(StatusFlags::ZERO) {
            self.registers.pc = addr;
        }
    }

    fn bpl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if !self.registers.status.contains(StatusFlags::NEGATIVE) {
            self.registers.pc = addr;
        }
    }

    fn brk(&mut self, _mode: &AddressingMode) {
        // TODO: BRK implementation would go here
        self.registers.status.insert(StatusFlags::BREAK_COMMAND); // Set break command flag
    }

    fn bvc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if !self.registers.status.contains(StatusFlags::OVERFLOW) {
            self.registers.pc = addr;
        }
    }

    fn bvs(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        if self.registers.status.contains(StatusFlags::OVERFLOW) {
            self.registers.pc = addr;
        }
    }

    fn clc(&mut self) {
        self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
    }

    fn cld(&mut self) {
        self.registers.status.remove(StatusFlags::DECIMAL_MODE); // Clear decimal flag
    }

    fn cli(&mut self) {
        self.registers.status.remove(StatusFlags::INTERRUPT_DISABLE); // Clear interrupt disable flag
    }

    fn clv(&mut self) {
        self.registers.status.remove(StatusFlags::OVERFLOW); // Clear overflow flag
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        let result = self.registers.a.wrapping_sub(value);

        if self.registers.a >= value {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        self.update_zero_and_negative_flags(result);
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        let result = self.registers.x.wrapping_sub(value);

        if self.registers.x >= value {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        self.update_zero_and_negative_flags(result);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        let result = self.registers.y.wrapping_sub(value);

        if self.registers.y >= value {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        self.update_zero_and_negative_flags(result);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.memory.read(addr);

        value = value.wrapping_sub(1);
        self.memory.write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.registers.x);
    }

    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.registers.y);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        self.registers.a ^= value;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.memory.read(addr);

        value = value.wrapping_add(1);
        self.memory.write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.x);
    }

    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.y);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.registers.pc = addr;
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let return_addr = self.registers.pc.wrapping_sub(1);

        self.memory
            .write_u16(self.registers.sp as u16 + 0x0100 - 2, return_addr);
        self.registers.sp = self.registers.sp.wrapping_sub(2);

        self.registers.pc = addr;
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        self.registers.a = value;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        self.registers.x = value;
        self.update_zero_and_negative_flags(self.registers.x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        self.registers.y = value;
        self.update_zero_and_negative_flags(self.registers.y);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Accumulator {
            let mut value = self.registers.a;

            if value & 0b0000_0001 != 0 {
                self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
            } else {
                self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
            }

            value >>= 1;
            self.registers.a = value;
            self.update_zero_and_negative_flags(self.registers.a);
            return;
        }

        let addr = self.get_operand_address(mode);
        let mut value = self.memory.read(addr);

        if value & 0b0000_0001 != 0 {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        value >>= 1;
        self.memory.write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn nop(&mut self) {
        // No operation
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        self.registers.a |= value;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn pha(&mut self) {
        let sp_addr = self.registers.sp as u16 + 0x0100;
        self.memory.write(sp_addr, self.registers.a);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    fn php(&mut self) {
        let sp_addr = self.registers.sp as u16 + 0x0100;
        self.memory.write(sp_addr, self.registers.status.bits());
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    fn pla(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let sp_addr = self.registers.sp as u16 + 0x0100;
        self.registers.a = self.memory.read(sp_addr);
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn plp(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let sp_addr = self.registers.sp as u16 + 0x0100;
        self.registers.status = StatusFlags::from_bits_truncate(self.memory.read(sp_addr));
    }

    fn rol(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Accumulator {
            let mut value = self.registers.a;
            let carry_in = if self.registers.status.contains(StatusFlags::CARRY) {
                1
            } else {
                0
            };

            if value & 0b1000_0000 != 0 {
                self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
            } else {
                self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
            }

            value = (value << 1) | carry_in;
            self.registers.a = value;
            self.update_zero_and_negative_flags(self.registers.a);
            return;
        }

        let addr = self.get_operand_address(mode);
        let mut value = self.memory.read(addr);
        let carry_in = if self.registers.status.contains(StatusFlags::CARRY) {
            1
        } else {
            0
        };

        if value & 0b1000_0000 != 0 {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        value = (value << 1) | carry_in;
        self.memory.write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Accumulator {
            let mut value = self.registers.a;
            let carry_in = if self.registers.status.contains(StatusFlags::CARRY) {
                0b1000_0000
            } else {
                0
            };

            if value & 0b0000_0001 != 0 {
                self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
            } else {
                self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
            }

            value = (value >> 1) | carry_in;
            self.registers.a = value;
            self.update_zero_and_negative_flags(self.registers.a);
            return;
        }

        let addr = self.get_operand_address(mode);
        let mut value = self.memory.read(addr);
        let carry_in = if self.registers.status.contains(StatusFlags::CARRY) {
            0b1000_0000
        } else {
            0
        };

        if value & 0b0000_0001 != 0 {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        value = (value >> 1) | carry_in;
        self.memory.write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn rti(&mut self) {
        self.plp();
        self.pla();
        self.registers.pc = self.registers.a as u16;
    }

    fn rts(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let sp_addr = self.registers.sp as u16 + 0x0100;
        let lo = self.memory.read(sp_addr) as u16;

        self.registers.sp = self.registers.sp.wrapping_add(1);
        let sp_addr = self.registers.sp as u16 + 0x0100;
        let hi = self.memory.read(sp_addr) as u16;

        self.registers.pc = (hi << 8) | lo;
        self.registers.pc = self.registers.pc.wrapping_add(1);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory.read(addr);

        let carry = if self.registers.status.contains(StatusFlags::CARRY) {
            0
        } else {
            1
        };

        let diff = self.registers.a as i16 - value as i16 - carry as i16;

        if diff >= 0 {
            self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
        } else {
            self.registers.status.remove(StatusFlags::CARRY); // Clear carry flag
        }

        let result = diff as u8;

        // Set overflow flag
        if ((self.registers.a ^ result) & (!(value) ^ result) & 0x80) != 0 {
            self.registers.status.insert(StatusFlags::OVERFLOW); // Set overflow flag
        } else {
            self.registers.status.remove(StatusFlags::OVERFLOW); // Clear overflow flag
        }

        self.registers.a = result;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn sec(&mut self) {
        self.registers.status.insert(StatusFlags::CARRY); // Set carry flag
    }

    fn sed(&mut self) {
        self.registers.status.insert(StatusFlags::DECIMAL_MODE); // Set decimal flag
    }

    fn sei(&mut self) {
        self.registers.status.insert(StatusFlags::INTERRUPT_DISABLE); // Set interrupt disable flag
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.memory.write(addr, self.registers.a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.memory.write(addr, self.registers.x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.memory.write(addr, self.registers.y);
    }

    fn tax(&mut self) {
        self.registers.x = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.x);
    }

    fn tay(&mut self) {
        self.registers.y = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.y);
    }

    fn tsx(&mut self) {
        self.registers.x = self.registers.sp;
        self.update_zero_and_negative_flags(self.registers.x);
    }

    fn txa(&mut self) {
        self.registers.a = self.registers.x;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn txs(&mut self) {
        self.registers.sp = self.registers.x;
    }

    fn tya(&mut self) {
        self.registers.a = self.registers.y;
        self.update_zero_and_negative_flags(self.registers.a);
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        if value == 0 {
            self.registers.status.insert(StatusFlags::ZERO); // Set zero flag
        } else {
            self.registers.status.remove(StatusFlags::ZERO); // Clear zero flag
        }

        if value & 0b1000_0000 != 0 {
            self.registers.status.insert(StatusFlags::NEGATIVE); // Set negative flag
        } else {
            self.registers.status.remove(StatusFlags::NEGATIVE); // Clear negative flag
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.registers.pc,

            AddressingMode::ZeroPage => self.memory.read(self.registers.pc) as u16,

            AddressingMode::Absolute => self.memory.read_u16(self.registers.pc),

            AddressingMode::ZeroPageX => {
                let pos = self.memory.read(self.registers.pc);
                let addr = pos.wrapping_add(self.registers.x) as u16;
                addr
            }
            AddressingMode::ZeroPageY => {
                let pos = self.memory.read(self.registers.pc);
                let addr = pos.wrapping_add(self.registers.y) as u16;
                addr
            }

            AddressingMode::Relative => {
                let offset = self.memory.read(self.registers.pc) as i8;
                let addr = self
                    .registers
                    .pc
                    .wrapping_add(1)
                    .wrapping_add(offset as u16);
                addr
            }

            AddressingMode::AbsoluteX => {
                let base = self.memory.read_u16(self.registers.pc);
                let addr = base.wrapping_add(self.registers.x as u16);
                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.memory.read_u16(self.registers.pc);
                let addr = base.wrapping_add(self.registers.y as u16);
                addr
            }

            AddressingMode::Indirect => {
                let ptr = self.memory.read_u16(self.registers.pc);
                let lo = self.memory.read(ptr);
                let hi = self.memory.read(ptr.wrapping_add(1));
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.registers.pc);

                let ptr: u8 = (base as u8).wrapping_add(self.registers.x);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.memory.read(self.registers.pc);

                let lo = self.memory.read(base as u16);
                let hi = self.memory.read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.registers.y as u16);
                deref
            }

            AddressingMode::None | AddressingMode::Accumulator => {
                panic!("Addressing mode does not use operand address: {:?}", mode);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00], None);
        assert_eq!(cpu.registers.a, 0x05);
        assert!(cpu.registers.status.bits() & 0b0000_0010 == 0b00);
        assert!(cpu.registers.status.bits() & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00], None);
        assert!(cpu.registers.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.registers.a = 10;
        cpu.load(vec![0xaa, 0x00], None);
        cpu.run();

        assert_eq!(cpu.registers.x, 10)
    }

    #[test]
    fn test_0xe8_inx_increment_x() {
        let mut cpu = CPU::new();
        cpu.registers.x = 5;
        cpu.load(vec![0xe8, 0x00], None);
        cpu.run();

        assert_eq!(cpu.registers.x, 6)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00], None);

        assert_eq!(cpu.registers.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0xff;
        cpu.load(vec![0xe8, 0x00], None);
        cpu.run();

        assert_eq!(cpu.registers.x, 0)
    }

    #[test]
    fn test_inx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0xff;
        cpu.load(vec![0xe8, 0x00], None);
        cpu.run();

        assert!(cpu.registers.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_inx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.registers.x = 0xfe;
        cpu.load(vec![0xe8, 0x00], None);
        cpu.run();

        assert!(cpu.registers.status.bits() & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_reset_sets_pc_to_reset_vector() {
        let mut cpu = CPU::new();
        cpu.memory.write_u16(0xFFFC, 0x1234);
        cpu.reset(false);
        assert_eq!(cpu.registers.pc, 0x1234);
    }

    #[test]
    fn test_load_writes_program_to_memory() {
        let mut cpu = CPU::new();
        let program = vec![0xA9, 0x01, 0x00];
        cpu.load(program.clone(), Some(0x8000));
        for (i, &byte) in program.iter().enumerate() {
            assert_eq!(cpu.memory.read(0x8000 + i as u16), byte);
        }
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00], None);

        assert_eq!(cpu.registers.a, 0x55);
    }
}

#[cfg(test)]
mod test_cpu_instrs {
    use super::*;

    #[test]
    fn test_adc_immediate() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x10;

        cpu.load(vec![0x69, 0x05, 0x00], None);
        cpu.run();

        assert_eq!(cpu.registers.a, 0x15);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0xFF;

        cpu.load(vec![0x69, 0x02, 0x00], None);
        cpu.run();

        assert_eq!(cpu.registers.a, 0x01);
        assert!(cpu.registers.status.contains(StatusFlags::CARRY));
    }

    #[test]
    fn test_and_immediate() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b1100_1100;

        cpu.load(vec![0x29, 0b1010_1010, 0x00], None);
        cpu.run();

        assert_eq!(cpu.registers.a, 0b1000_1000);
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0100_0001;

        cpu.load(vec![0x0A, 0x00], None);
        cpu.run();

        assert_eq!(cpu.registers.a, 0b1000_0010);
        assert!(!cpu.registers.status.contains(StatusFlags::CARRY));
    }

    #[test]
    fn test_lda_absolute() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x1234, 0x42);

        cpu.load_and_run(vec![0xad, 0x34, 0x12, 0x00], None);

        assert_eq!(cpu.registers.a, 0x42);
    }
}

use crate::cart::Mirroring;
use crate::mapper::Mapper;

pub struct UxRomMapper {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_bank: u8,
    mirroring: Mirroring,
}

impl UxRomMapper {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        UxRomMapper {
            prg_rom,
            chr_rom,
            prg_bank: 0,
            mirroring,
        }
    }
}

impl Mapper for UxRomMapper {
    fn read_prg(&self, addr: u16) -> u8 {
        let total_prg_banks = self.prg_rom.len() / 16384;

        if addr >= 0x8000 && addr <= 0xBFFF {
            // Lower 16KB: switchable bank (first N-1 banks)
            let bank_addr = (self.prg_bank as usize * 16384) + ((addr - 0x8000) as usize);
            self.prg_rom[bank_addr]
        } else if addr >= 0xC000 && addr <= 0xFFFF {
            // Upper 16KB: fixed to last bank
            let bank_addr = ((total_prg_banks - 1) * 16384) + ((addr - 0xC000) as usize);
            self.prg_rom[bank_addr]
        } else {
            0
        }
    }

    fn write_prg(&mut self, addr: u16, data: u8) {
        // Bank switching at $8000-$FFFF
        if addr >= 0x8000 && addr <= 0xFFFF {
            self.prg_bank = data & 0x0F; // Use only the lower 4 bits
        }
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom[addr as usize]
    }

    fn write_chr(&mut self, _addr: u16, _data: u8) {
        // UxROM CHR is ROM, ignore writes
    }

    fn mirroring(&self) -> Mirroring {
        self.mirroring.clone()
    }
}

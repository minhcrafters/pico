use crate::cart::Mirroring;
use crate::mapper::Mapper;

pub struct CnRomMapper {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    chr_bank: u8,
    mirroring: Mirroring,
}

impl CnRomMapper {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        CnRomMapper {
            prg_rom,
            chr_rom,
            chr_bank: 0,
            mirroring,
        }
    }
}

impl Mapper for CnRomMapper {
    fn read_prg(&self, addr: u16) -> u8 {
        // CnROM has fixed PRG - entire 32KB at $8000-$FFFF
        if addr >= 0x8000 && addr <= 0xFFFF {
            let addr_in_rom = (addr - 0x8000) as usize;
            self.prg_rom[addr_in_rom % self.prg_rom.len()]
        } else {
            0
        }
    }

    fn write_prg(&mut self, _addr: u16, _data: u8) {
        // CnROM has no PRG RAM, ignore writes
    }

    fn read_chr(&self, addr: u16) -> u8 {
        // 8KB CHR bank switching at $0000-$1FFF
        let chr_bank_addr = (self.chr_bank as usize * 8192) + (addr as usize);
        self.chr_rom[chr_bank_addr % self.chr_rom.len()]
    }

    fn write_chr(&mut self, addr: u16, data: u8) {
        // CHR bank switching at $0000-$1FFF  
        if addr <= 0x1FFF {
            self.chr_bank = data & 0x03; // Use only the lower 2 bits for 4 banks
        }
    }

    fn mirroring(&self) -> Mirroring {
        self.mirroring.clone()
    }
}

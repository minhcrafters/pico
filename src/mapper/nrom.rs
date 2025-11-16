use crate::cart::Mirroring;
use crate::mapper::Mapper;

pub struct NromMapper {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mirroring: Mirroring,
}

impl NromMapper {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        NromMapper {
            prg_rom,
            chr_rom,
            mirroring,
        }
    }
}

impl Mapper for NromMapper {
    fn read_prg(&self, addr: u16) -> u8 {
        if !(0x8000..=0xFFFF).contains(&addr) {
            return 0;
        }

        if self.prg_rom.is_empty() {
            return 0;
        }

        let mut offset = (addr - 0x8000) as usize;
        if self.prg_rom.len() == 0x4000 {
            // Mirror 16KB PRG across both $8000-$BFFF and $C000-$FFFF
            offset %= 0x4000;
        }

        self.prg_rom[offset]
    }

    fn write_prg(&mut self, _addr: u16, _data: u8) {
        // NROM has no PRG RAM, ignore writes
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom[addr as usize]
    }

    fn write_chr(&mut self, _addr: u16, _data: u8) {
        // NROM CHR is ROM, ignore writes
    }

    fn mirroring(&self) -> Mirroring {
        self.mirroring.clone()
    }
}

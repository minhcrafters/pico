use crate::cart::Mirroring;
use crate::mapper::Mapper;
use std::cell::Cell;

const PRG_BANK_SIZE: usize = 0x2000;
const CHR_BANK_SIZE_1KB: usize = 0x400;

pub struct Mmc3Mapper {
    prg_rom: Vec<u8>,
    chr: Vec<u8>,
    chr_is_ram: bool,
    prg_ram: Vec<u8>,
    prg_ram_write_protect: bool,
    prg_ram_enable: bool,

    bank_select: u8,
    bank_registers: [u8; 8],
    chr_banks: [usize; 8],
    prg_banks: [usize; 4],

    irq_latch: Cell<u8>,
    irq_counter: Cell<u8>,
    irq_reload: Cell<bool>,
    irq_enabled: Cell<bool>,
    irq_pending: Cell<bool>,
    last_a12: Cell<bool>,

    current_mirroring: Mirroring,
    force_four_screen: bool,
}

impl Mmc3Mapper {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        let chr_is_ram = chr_rom.is_empty();
        let chr = if chr_is_ram {
            vec![0; 0x2000]
        } else {
            chr_rom
        };

        let mut mapper = Mmc3Mapper {
            prg_rom,
            chr,
            chr_is_ram,
            prg_ram: vec![0; 0x2000],
            prg_ram_write_protect: false,
            prg_ram_enable: true,
            bank_select: 0,
            bank_registers: [0; 8],
            chr_banks: [0; 8],
            prg_banks: [0; 4],
            irq_latch: Cell::new(0),
            irq_counter: Cell::new(0),
            irq_reload: Cell::new(false),
            irq_enabled: Cell::new(false),
            irq_pending: Cell::new(false),
            last_a12: Cell::new(false),
            current_mirroring: mirroring.clone(),
            force_four_screen: mirroring == Mirroring::FourScreen,
        };

        mapper.bank_registers = [0, 2, 4, 5, 6, 7, 0, 1];
        mapper.update_chr_banks();
        mapper.update_prg_banks();

        mapper
    }

    fn prg_bank_offset(&self, index: usize) -> usize {
        if self.prg_rom.is_empty() {
            return 0;
        }
        let mut count = self.prg_rom.len() / PRG_BANK_SIZE;
        if count == 0 {
            count = 1;
        }
        let bank = index % count;
        bank * PRG_BANK_SIZE
    }

    fn chr_bank_offset_1k(&self, index: usize) -> usize {
        if self.chr.is_empty() {
            return 0;
        }
        let mut count = self.chr.len() / CHR_BANK_SIZE_1KB;
        if count == 0 {
            count = 1;
        }
        let bank = index % count;
        bank * CHR_BANK_SIZE_1KB
    }

    fn set_chr_2kb_bank(&mut self, slot: usize, value: u8) {
        let bank = (value as usize) & !1;
        self.chr_banks[slot] = self.chr_bank_offset_1k(bank);
        self.chr_banks[slot + 1] = self.chr_bank_offset_1k(bank + 1);
    }

    fn set_chr_1kb_bank(&mut self, slot: usize, value: u8) {
        self.chr_banks[slot] = self.chr_bank_offset_1k(value as usize);
    }

    fn update_chr_banks(&mut self) {
        let invert = self.bank_select & 0x80 != 0;
        if invert {
            self.set_chr_1kb_bank(0, self.bank_registers[2]);
            self.set_chr_1kb_bank(1, self.bank_registers[3]);
            self.set_chr_1kb_bank(2, self.bank_registers[4]);
            self.set_chr_1kb_bank(3, self.bank_registers[5]);
            self.set_chr_2kb_bank(4, self.bank_registers[0]);
            self.set_chr_2kb_bank(6, self.bank_registers[1]);
        } else {
            self.set_chr_2kb_bank(0, self.bank_registers[0]);
            self.set_chr_2kb_bank(2, self.bank_registers[1]);
            self.set_chr_1kb_bank(4, self.bank_registers[2]);
            self.set_chr_1kb_bank(5, self.bank_registers[3]);
            self.set_chr_1kb_bank(6, self.bank_registers[4]);
            self.set_chr_1kb_bank(7, self.bank_registers[5]);
        }
    }

    fn update_prg_banks(&mut self) {
        let mut count = self.prg_rom.len() / PRG_BANK_SIZE;
        if count == 0 {
            count = 1;
        }
        let last_index = count - 1;
        let second_last_index = if count >= 2 { count - 2 } else { count - 1 };
        let fixed_last = self.prg_bank_offset(last_index);
        let fixed_second_last = self.prg_bank_offset(second_last_index);
        let bank6 = self.prg_bank_offset(self.bank_registers[6] as usize);
        let bank7 = self.prg_bank_offset(self.bank_registers[7] as usize);

        if self.bank_select & 0x40 == 0 {
            self.prg_banks[0] = bank6;
            self.prg_banks[1] = bank7;
            self.prg_banks[2] = fixed_second_last;
        } else {
            self.prg_banks[0] = fixed_second_last;
            self.prg_banks[1] = bank7;
            self.prg_banks[2] = bank6;
        }
        self.prg_banks[3] = fixed_last;
    }

    fn map_chr_addr(&self, addr: u16) -> usize {
        if self.chr.is_empty() {
            return 0;
        }
        let bank_slot = ((addr as usize) / CHR_BANK_SIZE_1KB).min(self.chr_banks.len() - 1);
        let offset = (addr as usize) % CHR_BANK_SIZE_1KB;
        let base = self.chr_banks[bank_slot];
        (base + offset) % self.chr.len()
    }

    fn handle_scanline_counter(&self, addr: u16) {
        let a12 = addr & 0x1000 != 0;
        let last = self.last_a12.get();
        if a12 && !last {
            self.clock_irq_counter();
        }
        self.last_a12.set(a12);
    }

    fn clock_irq_counter(&self) {
        let mut counter = self.irq_counter.get();
        if counter == 0 || self.irq_reload.get() {
            counter = self.irq_latch.get();
            self.irq_counter.set(counter);
            self.irq_reload.set(false);
        } else {
            counter = counter.wrapping_sub(1);
            self.irq_counter.set(counter);
        }

        if counter == 0 && self.irq_enabled.get() {
            self.irq_pending.set(true);
        }
    }

    fn read_prg_bank(&self, slot: usize, addr: u16) -> u8 {
        if self.prg_rom.is_empty() {
            return 0;
        }
        let base = self.prg_banks[slot];
        let offset = (addr as usize) & (PRG_BANK_SIZE - 1);
        let index = (base + offset) % self.prg_rom.len();
        self.prg_rom[index]
    }
}

impl Mapper for Mmc3Mapper {
    fn read_prg(&self, addr: u16) -> u8 {
        match addr {
            0x6000..=0x7FFF => {
                if self.prg_ram_enable {
                    self.prg_ram[(addr - 0x6000) as usize]
                } else {
                    0xFF
                }
            }
            0x8000..=0x9FFF => self.read_prg_bank(0, addr - 0x8000),
            0xA000..=0xBFFF => self.read_prg_bank(1, addr - 0xA000),
            0xC000..=0xDFFF => self.read_prg_bank(2, addr - 0xC000),
            0xE000..=0xFFFF => self.read_prg_bank(3, addr - 0xE000),
            _ => 0,
        }
    }

    fn write_prg(&mut self, addr: u16, data: u8) {
        match addr {
            0x6000..=0x7FFF => {
                if self.prg_ram_enable && !self.prg_ram_write_protect {
                    self.prg_ram[(addr - 0x6000) as usize] = data;
                }
            }
            0x8000..=0x9FFF => {
                if addr & 1 == 0 {
                    self.bank_select = data;
                    self.update_chr_banks();
                    self.update_prg_banks();
                } else {
                    let reg = (self.bank_select & 0x07) as usize;
                    let value = match reg {
                        0 | 1 => data & 0xFE,
                        _ => data,
                    };
                    self.bank_registers[reg] = value;
                    if reg <= 5 {
                        self.update_chr_banks();
                    } else {
                        self.update_prg_banks();
                    }
                }
            }
            0xA000..=0xBFFF => {
                if addr & 1 == 0 {
                    if !self.force_four_screen {
                        self.current_mirroring = if data & 0x01 == 0 {
                            Mirroring::Vertical
                        } else {
                            Mirroring::Horizontal
                        };
                    }
                } else {
                    self.prg_ram_write_protect = data & 0x40 != 0;
                    self.prg_ram_enable = data & 0x80 != 0;
                }
            }
            0xC000..=0xDFFF => {
                if addr & 1 == 0 {
                    self.irq_latch.set(data);
                } else {
                    self.irq_reload.set(true);
                }
            }
            0xE000..=0xFFFF => {
                if addr & 1 == 0 {
                    self.irq_enabled.set(false);
                    self.irq_pending.set(false);
                } else {
                    self.irq_enabled.set(true);
                }
            }
            _ => {}
        }
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.handle_scanline_counter(addr);
        if self.chr.is_empty() {
            return 0;
        }
        let index = self.map_chr_addr(addr);
        self.chr[index]
    }

    fn write_chr(&mut self, addr: u16, data: u8) {
        self.handle_scanline_counter(addr);
        if self.chr_is_ram {
            let index = self.map_chr_addr(addr);
            self.chr[index] = data;
        }
    }

    fn mirroring(&self) -> Mirroring {
        self.current_mirroring.clone()
    }

    fn poll_irq(&self) -> Option<u8> {
        if self.irq_pending.get() {
            Some(0)
        } else {
            None
        }
    }
}

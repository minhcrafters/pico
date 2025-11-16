use crate::cart::Mirroring;
use crate::mapper::Mapper;

pub struct Mmc3Mapper {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    
    // Internal registers
    bank_select: u8,
    bank_data: [u8; 8], // Registers 0-7
    
    // IRQ counter and status
    irq_counter: u8,
    irq_reload: u8,
    irq_enabled: bool,
    irq_pending: bool,
    
    // Mirroring
    current_mirroring: Mirroring,
    mirroring_control: u8,
}

impl Mmc3Mapper {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Mmc3Mapper {
            prg_rom,
            chr_rom,
            prg_ram: vec![0; 8192], // 8KB PRG RAM
            
            bank_select: 0,
            bank_data: [0; 8],
            
            irq_counter: 0,
            irq_reload: 0,
            irq_enabled: false,
            irq_pending: false,
            
            current_mirroring: mirroring.clone(),
            mirroring_control: 0,
        }
    }
    
    fn update_prg_banks(&mut self) {
        let total_prg_banks = self.prg_rom.len() / 8192; // 8KB banks
        
        // PRG bank modes
        if self.bank_select & 0x40 == 0 {
            // Mode 0: $8000-$9FFF swappable, $A000-$BFFF fixed to bank 6,
            //         $C000-$DFFF swappable, $E000-$FFFF fixed to last bank
            if total_prg_banks > 6 {
                self.bank_data[6] = (total_prg_banks - 2) as u8;
            } else {
                self.bank_data[6] = 6;
            }
            self.bank_data[7] = (total_prg_banks - 1) as u8;
        } else {
            // Mode 1: $8000-$9FFF fixed to second to last bank,
            //         $A000-$BFFF fixed to bank 6, $C000-$DFFF swappable,
            //         $E000-$FFFF fixed to last bank
            self.bank_data[6] = (total_prg_banks - 2) as u8;
            if total_prg_banks > 6 {
                self.bank_data[7] = (total_prg_banks - 1) as u8;
            } else {
                self.bank_data[7] = 7;
            }
        }
    }
    
    fn update_mirroring(&mut self) {
        self.current_mirroring = if self.bank_select & 0x80 != 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        };
    }
    
    fn process_scanline_counter(&mut self) {
        if self.irq_counter == 0 {
            self.irq_counter = self.irq_reload;
        } else {
            self.irq_counter -= 1;
        }
        
        if self.irq_counter == 0 && self.irq_enabled {
            self.irq_pending = true;
        }
    }
}

impl Mapper for Mmc3Mapper {
    fn read_prg(&self, addr: u16) -> u8 {
        if addr >= 0x6000 && addr <= 0x7FFF {
            // PRG RAM
            self.prg_ram[(addr - 0x6000) as usize]
        } else if addr >= 0x8000 && addr <= 0x9FFF {
            // PRG bank 0 or 2
            let bank = if self.bank_select & 0x40 == 0 { 0 } else { 2 };
            let bank_index = self.bank_data[bank as usize];
            let bank_addr = (bank_index as usize * 8192) + ((addr - 0x8000) as usize);
            self.prg_rom[bank_addr % self.prg_rom.len()]
        } else if addr >= 0xA000 && addr <= 0xBFFF {
            // PRG bank 1 or 3
            let bank = if self.bank_select & 0x40 == 0 { 1 } else { 3 };
            let bank_index = self.bank_data[bank as usize];
            let bank_addr = (bank_index as usize * 8192) + ((addr - 0xA000) as usize);
            self.prg_rom[bank_addr % self.prg_rom.len()]
        } else if addr >= 0xC000 && addr <= 0xDFFF {
            // PRG bank 2 or 0
            let bank = if self.bank_select & 0x40 == 0 { 2 } else { 0 };
            let bank_index = self.bank_data[bank as usize];
            let bank_addr = (bank_index as usize * 8192) + ((addr - 0xC000) as usize);
            self.prg_rom[bank_addr % self.prg_rom.len()]
        } else if addr >= 0xE000 && addr <= 0xFFFF {
            // PRG bank 3 or 1 (fixed)
            let bank = if self.bank_select & 0x40 == 0 { 3 } else { 1 };
            let bank_index = self.bank_data[bank as usize];
            let bank_addr = (bank_index as usize * 8192) + ((addr - 0xE000) as usize);
            self.prg_rom[bank_addr % self.prg_rom.len()]
        } else {
            0
        }
    }

    fn write_prg(&mut self, addr: u16, data: u8) {
        if addr >= 0x6000 && addr <= 0x7FFF {
            // PRG RAM
            self.prg_ram[(addr - 0x6000) as usize] = data;
        } else if addr >= 0x8000 && addr <= 0x9FFF {
            if addr & 0x0001 == 0 {
                // Bank select
                self.bank_select = data;
                self.update_mirroring();
                self.update_prg_banks();
            } else {
                // Bank data
                let reg = self.bank_select & 0x07;
                self.bank_data[reg as usize] = data & 0x3F;
                
                if reg <= 1 {
                    // CHR banks 0-1
                    self.bank_data[reg as usize] = data & 0xFF;
                } else if reg <= 5 {
                    // CHR banks 2-5  
                    self.bank_data[reg as usize] = data & 0xFF;
                } else {
                    // PRG banks 6-7
                    self.bank_data[reg as usize] = data & 0x3F;
                }
            }
        } else if addr >= 0xA000 && addr <= 0xBFFF {
            if addr & 0x0001 == 0 {
                // Mirroring control
                self.mirroring_control = data & 0x01;
            } else {
                // PRG RAM protect
                if data & 0x80 != 0 {
                    self.irq_enabled = data & 0x01 != 0;
                }
            }
        } else if addr >= 0xC000 && addr <= 0xDFFF {
            if addr & 0x0001 == 0 {
                // IRQ reload
                self.irq_reload = data;
            } else {
                // IRQ acknowledge
                self.irq_pending = false;
            }
        } else if addr >= 0xE000 && addr <= 0xFFFF {
            // IRQ disable/enable
            self.irq_enabled = (data & 0x01) != 0;
        }
    }

    fn read_chr(&self, addr: u16) -> u8 {
        let chr_addr = if addr < 0x1000 {
            // CHR banks 0-1 (2KB each)
            let bank = self.bank_data[(addr >> 11) as usize & 0x01]; // 0 or 1
            let bank_addr = (bank as usize * 2048) + ((addr as usize) & 0x7FF);
            self.chr_rom[bank_addr % self.chr_rom.len()]
        } else {
            // CHR banks 2-5 (1KB each)
            let bank_index = self.bank_data[((addr >> 10) as usize & 0x07) + 2]; // 2-5
            let bank_addr = (bank_index as usize * 1024) + ((addr as usize) & 0x3FF);
            self.chr_rom[bank_addr % self.chr_rom.len()]
        };
        chr_addr
    }

    fn write_chr(&mut self, _addr: u16, _data: u8) {
        // CHR RAM not typically used in MMC3, ignore writes
    }

    fn mirroring(&self) -> Mirroring {
        self.current_mirroring.clone()
    }
    
    fn poll_irq(&self) -> Option<u8> {
        if self.irq_pending {
            Some(0) // IRQ acknowledged
        } else {
            None
        }
    }
}

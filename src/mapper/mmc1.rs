use crate::cart::Mirroring;
use crate::mapper::Mapper;

pub struct Mmc1Mapper {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    
    // Internal state
    shift_register: u8,
    shift_count: u8,
    
    // Bank select registers
    prg_bank_mode: u8, // 0-3: PRG bank mode
    chr_bank_mode: u8, // 0-1: CHR bank mode (high/low)
    
    // Bank indices
    prg_bank_0: u8,
    prg_bank_1: u8, 
    prg_bank_2: u8,
    prg_bank_3: u8,
    
    chr_bank_0: u8,
    chr_bank_1: u8,
    chr_bank_2: u8,
    chr_bank_3: u8,
    chr_bank_4: u8,
    chr_bank_5: u8,
    
    // Mirroring
    current_mirroring: Mirroring,
    mirroring_control: u8,
}

impl Mmc1Mapper {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        let total_prg_banks = prg_rom.len() / 16384; // 16KB banks
        
        Mmc1Mapper {
            prg_rom,
            chr_rom,
            prg_ram: vec![0; 8192], // 8KB PRG RAM
            
            shift_register: 0,
            shift_count: 0,
            
            prg_bank_mode: 0,
            chr_bank_mode: 0,
            
            prg_bank_0: 0,
            prg_bank_1: if total_prg_banks > 1 { 1 } else { 0 },
            prg_bank_2: if total_prg_banks > 2 { total_prg_banks as u8 - 2 } else { 0 },
            prg_bank_3: if total_prg_banks > 3 { total_prg_banks as u8 - 1 } else { 0 },
            
            chr_bank_0: 0,
            chr_bank_1: 1,
            chr_bank_2: 2,
            chr_bank_3: 3,
            chr_bank_4: 4,
            chr_bank_5: 5,
            
            current_mirroring: mirroring.clone(),
            mirroring_control: 0,
        }
    }
    
    fn write_control_register(&mut self, value: u8) {
        self.mirroring_control = value & 0x03;
        self.current_mirroring = match value & 0x03 {
            0 => Mirroring::Vertical,
            1 => Mirroring::Horizontal,
            2 => Mirroring::Vertical,
            3 => Mirroring::Horizontal,
            _ => Mirroring::Vertical,
        };
    }
    
    fn write_chr_bank_register(&mut self, reg: u8, value: u8) {
        match reg {
            0 => self.chr_bank_0 = value & 0x1F,
            1 => self.chr_bank_1 = value & 0x1F,
            2 => self.chr_bank_2 = value & 0x1F,
            3 => self.chr_bank_3 = value & 0x1F,
            4 => self.chr_bank_4 = value & 0x1F,
            5 => self.chr_bank_5 = value & 0x1F,
            _ => {},
        }
    }
    
    fn write_prg_bank_register(&mut self, reg: u8, value: u8) {
        match reg {
            0 => self.prg_bank_0 = value & 0x0F,
            1 => self.prg_bank_1 = value & 0x0F,
            2 => self.prg_bank_2 = value & 0x0F,
            3 => self.prg_bank_3 = value & 0x0F,
            _ => {},
        }
    }
    
    fn process_write_data(&mut self, reg: u8, value: u8) {
        let value = value & 0x01;
        
        if value == 0 {
            // Reset command
            self.shift_count = 0;
            self.shift_register = 0;
            
            // Execute command based on last register written
            match self.prg_bank_mode {
                0..=3 => self.write_control_register(self.shift_register),
                4..=5 => self.write_chr_bank_register(self.prg_bank_mode - 4, self.shift_register),
                6..=7 => self.write_prg_bank_register(self.prg_bank_mode - 6, self.shift_register),
                _ => {},
            }
        } else {
            // Shift in data
            self.shift_register >>= 1;
            self.shift_register |= value << 4;
            self.shift_count += 1;
            
            if self.shift_count == 5 {
                // Command complete
                match self.prg_bank_mode {
                    0..=3 => self.write_control_register(self.shift_register),
                    4..=5 => self.write_chr_bank_register(self.prg_bank_mode - 4, self.shift_register),
                    6..=7 => self.write_prg_bank_register(self.prg_bank_mode - 6, self.shift_register),
                    _ => {},
                }
                self.shift_count = 0;
                self.shift_register = 0;
            }
        }
    }
}

impl Mapper for Mmc1Mapper {
    fn read_prg(&self, addr: u16) -> u8 {
        let total_prg_banks = self.prg_rom.len() / 16384;
        
        if addr >= 0x6000 && addr <= 0x7FFF {
            // PRG RAM
            self.prg_ram[(addr - 0x6000) as usize]
        } else if addr >= 0x8000 && addr <= 0xBFFF {
            // Lower 16KB PRG bank
            let bank_index = match self.prg_bank_mode {
                0 => self.prg_bank_0,
                1 => self.prg_bank_0,
                2 => self.prg_bank_0,
                3 => self.prg_bank_0,
                _ => self.prg_bank_0,
            };
            
            let bank_addr = (bank_index as usize * 16384) + ((addr - 0x8000) as usize);
            self.prg_rom[bank_addr % self.prg_rom.len()]
        } else if addr >= 0xC000 && addr <= 0xFFFF {
            // Upper 16KB PRG bank
            let bank_index = match self.prg_bank_mode {
                0 => if total_prg_banks > 1 { total_prg_banks as u8 - 2 } else { 0 },
                1 => if total_prg_banks > 1 { total_prg_banks as u8 - 1 } else { 0 },
                2 => self.prg_bank_2,
                3 => self.prg_bank_3,
                _ => 0,
            };
            
            let bank_addr = (bank_index as usize * 16384) + ((addr - 0xC000) as usize);
            self.prg_rom[bank_addr % self.prg_rom.len()]
        } else {
            0
        }
    }

    fn write_prg(&mut self, addr: u16, data: u8) {
        if addr >= 0x6000 && addr <= 0x7FFF {
            // PRG RAM
            self.prg_ram[(addr - 0x6000) as usize] = data;
        } else if addr >= 0x8000 && addr <= 0xFFFF {
            // Serial write to shift register
            self.prg_bank_mode = ((addr >> 13) & 0x07) as u8; // Bits 13-15 select register
            self.process_write_data(((addr >> 13) & 0x07) as u8, data);
        }
    }

    fn read_chr(&self, addr: u16) -> u8 {
        if addr <= 0x0FFF {
            // Lower CHR page
            let bank_index = self.chr_bank_0;
            let bank_addr = (bank_index as usize * 4096) + (addr as usize);
            self.chr_rom[bank_addr % self.chr_rom.len()]
        } else if addr >= 0x1000 && addr <= 0x1FFF {
            // Upper CHR page  
            let bank_index = self.chr_bank_1;
            let bank_addr = (bank_index as usize * 4096) + ((addr - 0x1000) as usize);
            self.chr_rom[bank_addr % self.chr_rom.len()]
        } else {
            0
        }
    }

    fn write_chr(&mut self, _addr: u16, _data: u8) {
        // MMC1 CHR is ROM, ignore writes
    }

    fn mirroring(&self) -> Mirroring {
        self.current_mirroring.clone()
    }
}

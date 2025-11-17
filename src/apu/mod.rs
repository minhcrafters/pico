use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// Timing reference: https://www.nesdev.org/wiki/APU
const CPU_FREQUENCY: f64 = 1_789_773.0;
const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];
const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];
const TRIANGLE_TABLE: [u8; 32] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15,
];
const NOISE_PERIOD_TABLE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];
const DMC_RATE_TABLE: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 85, 72, 54,
];

pub struct APU {
    square1: SquareChannel,
    square2: SquareChannel,
    triangle: TriangleChannel,
    noise: NoiseChannel,
    dmc: DmcChannel,
    frame_counter: FrameCounter,
    frame_irq: bool,
    sample_timer: f64,
    cycles_per_sample: f64,
    audio_buffer: Arc<Mutex<VecDeque<f32>>>,
    frame_buffer_limit: usize,
    cpu_cycle: u64,
    pending_frame_reset: Option<FrameCounterReset>,
}

impl APU {
    pub fn new(sample_rate: u32, audio_buffer: Arc<Mutex<VecDeque<f32>>>) -> Self {
        let cycles_per_sample = CPU_FREQUENCY / sample_rate as f64;

        APU {
            square1: SquareChannel::new(true),
            square2: SquareChannel::new(false),
            triangle: TriangleChannel::new(),
            noise: NoiseChannel::new(),
            dmc: DmcChannel::new(),
            frame_counter: FrameCounter::new(),
            frame_irq: false,
            sample_timer: 0.0,
            cycles_per_sample,
            audio_buffer,
            frame_buffer_limit: sample_rate as usize * 2,
            cpu_cycle: 1,
            pending_frame_reset: None,
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x4000 => self.square1.write_control(value),
            0x4001 => self.square1.write_sweep(value),
            0x4002 => self.square1.write_timer_low(value),
            0x4003 => self.square1.write_timer_high(value),
            0x4004 => self.square2.write_control(value),
            0x4005 => self.square2.write_sweep(value),
            0x4006 => self.square2.write_timer_low(value),
            0x4007 => self.square2.write_timer_high(value),
            0x4008 => self.triangle.write_control(value),
            0x400A => self.triangle.write_timer_low(value),
            0x400B => self.triangle.write_timer_high(value),
            0x400C => self.noise.write_control(value),
            0x400E => self.noise.write_period(value),
            0x400F => self.noise.write_length(value),
            0x4010 => self.dmc.write_control(value),
            0x4011 => self.dmc.write_direct_load(value),
            0x4012 => self.dmc.write_sample_address(value),
            0x4013 => self.dmc.write_sample_length(value),
            _ => {}
        }
    }

    pub fn write_status(&mut self, value: u8) {
        self.square1.set_enabled(value & 0x01 != 0);
        self.square2.set_enabled(value & 0x02 != 0);
        self.triangle.set_enabled(value & 0x04 != 0);
        self.noise.set_enabled(value & 0x08 != 0);
        self.dmc.set_enabled(value & 0x10 != 0);

        if !self.square1.enabled {
            self.square1.clear_length();
        }
        if !self.square2.enabled {
            self.square2.clear_length();
        }
        if !self.triangle.enabled {
            self.triangle.clear_length();
        }
        if !self.noise.enabled {
            self.noise.clear_length();
        }
        if !self.dmc.enabled {
            self.dmc.clear_bytes();
        } else {
            self.dmc.restart_if_needed();
        }

        self.frame_irq = false;
        self.dmc.clear_irq();
    }

    pub fn write_frame_counter(&mut self, value: u8) {
        // See https://www.nesdev.org/wiki/APU_Frame_Counter for the delayed reset semantics.
        let mode = if value & 0x80 != 0 {
            FrameCounterMode::FiveStep
        } else {
            FrameCounterMode::FourStep
        };
        let irq_inhibit = value & 0x40 != 0;
        self.frame_counter.set_irq_inhibit(irq_inhibit);
        self.frame_irq = false;

        let delay = if self.cpu_cycle & 1 == 0 { 4 } else { 3 };
        self.pending_frame_reset = Some(FrameCounterReset { delay, mode });
    }

    pub fn read_status(&mut self) -> u8 {
        let mut status = 0u8;
        if self.square1.length_counter > 0 {
            status |= 0x01;
        }
        if self.square2.length_counter > 0 {
            status |= 0x02;
        }
        if self.triangle.length_counter > 0 {
            status |= 0x04;
        }
        if self.noise.length_counter > 0 {
            status |= 0x08;
        }
        if self.dmc.bytes_remaining > 0 {
            status |= 0x10;
        }
        if self.frame_irq {
            status |= 0x40;
        }
        if self.dmc.irq_pending {
            status |= 0x80;
        }

        self.frame_irq = false;
        self.dmc.clear_irq();

        status
    }

    pub fn poll_irq(&mut self) -> Option<u8> {
        if self.dmc.irq_pending {
            self.dmc.clear_irq();
            return Some(0);
        }
        if self.frame_irq {
            self.frame_irq = false;
            return Some(0);
        }
        None
    }

    pub fn clock(&mut self) -> Option<u16> {
        self.tick_frame_reset();

        let dmc_request = self.dmc.clock();

        if self.cpu_cycle & 1 == 0 {
            self.square1.clock_timer();
            self.square2.clock_timer();
            self.noise.clock_timer();
        }
        self.triangle.clock_timer();

        let frame_events = self.frame_counter.tick();
        if frame_events.quarter {
            self.clock_quarter_frame();
        }
        if frame_events.half {
            self.clock_half_frame();
        }
        if frame_events.irq {
            self.set_frame_irq();
        }

        self.sample_timer += 1.0;
        if self.sample_timer >= self.cycles_per_sample {
            self.sample_timer -= self.cycles_per_sample;
            self.queue_sample();
        }

        let request = dmc_request;

        self.cpu_cycle = self.cpu_cycle.wrapping_add(1);

        request
    }

    pub fn provide_dmc_sample(&mut self, value: u8) {
        self.dmc.provide_sample(value);
    }

    fn queue_sample(&mut self) {
        let pulse_out = f64::from(self.square1.output() + self.square2.output());
        let t_out = f64::from(self.triangle.output());
        let n_out = f64::from(self.noise.output());
        let d_out = f64::from(self.dmc.output());

        let pulse_mix = if pulse_out > 0.0 {
            95.88 / ((8128.0 / pulse_out) + 100.0)
        } else {
            0.0
        };
        let tnd_input = (t_out / 8227.0) + (n_out / 12241.0) + (d_out / 22638.0);
        let tnd_mix = if tnd_input > 0.0 {
            159.79 / ((1.0 / tnd_input) + 100.0)
        } else {
            0.0
        };

        let mut sample = pulse_mix + tnd_mix;
        sample = (sample * 2.0 - 1.0).clamp(-1.0, 1.0);
        let final_sample = sample as f32;

        if let Ok(mut buffer) = self.audio_buffer.lock() {
            if buffer.len() >= self.frame_buffer_limit {
                buffer.pop_front();
            }
            buffer.push_back(final_sample);
        }
    }

    fn clock_quarter_frame(&mut self) {
        self.square1.clock_envelope();
        self.square2.clock_envelope();
        self.triangle.clock_linear_counter();
        self.noise.clock_envelope();
    }

    fn clock_half_frame(&mut self) {
        self.square1.clock_length_counter();
        self.square2.clock_length_counter();
        self.triangle.clock_length_counter();
        self.noise.clock_length_counter();
        self.square1.clock_sweep();
        self.square2.clock_sweep();
    }

    fn set_frame_irq(&mut self) {
        self.frame_irq = true;
    }

    fn tick_frame_reset(&mut self) {
        if let Some(mut reset) = self.pending_frame_reset.take() {
            if reset.delay > 0 {
                reset.delay -= 1;
            }

            if reset.delay == 0 {
                self.frame_counter.set_mode(reset.mode);
                self.frame_counter.reset();

                if matches!(reset.mode, FrameCounterMode::FiveStep) {
                    // In five-step mode the quarter and half clocks fire immediately (nesdev reference).
                    self.clock_quarter_frame();
                    self.clock_half_frame();
                }
            } else {
                self.pending_frame_reset = Some(reset);
            }
        }
    }
}

struct Envelope {
    constant_volume: bool,
    loop_flag: bool,
    volume: u8,
    start_flag: bool,
    divider: u8,
    decay_level: u8,
}

impl Envelope {
    fn new() -> Self {
        Envelope {
            constant_volume: false,
            loop_flag: false,
            volume: 0,
            start_flag: false,
            divider: 0,
            decay_level: 0,
        }
    }

    fn set_control(&mut self, data: u8) {
        self.constant_volume = data & 0x10 != 0;
        self.loop_flag = data & 0x20 != 0;
        self.volume = data & 0x0F;
    }

    fn restart(&mut self) {
        self.start_flag = true;
    }

    fn divider_period(&self) -> u8 {
        (self.volume & 0x0F).saturating_add(1)
    }

    fn reload_divider(&mut self) {
        self.divider = self.divider_period();
    }

    fn clock(&mut self) {
        if self.start_flag {
            self.start_flag = false;
            self.decay_level = 15;
            self.reload_divider();
        } else if self.divider == 0 {
            self.reload_divider();
            if self.decay_level == 0 {
                if self.loop_flag {
                    self.decay_level = 15;
                }
            } else {
                self.decay_level -= 1;
            }
        } else {
            self.divider -= 1;
        }
    }

    fn output(&self) -> u8 {
        if self.constant_volume {
            self.volume
        } else {
            self.decay_level
        }
    }

    fn halt(&self) -> bool {
        self.loop_flag
    }
}

struct SweepUnit {
    enabled: bool,
    negate: bool,
    shift: u8,
    period: u8,
    divider: u8,
    reload: bool,
    ones_complement: bool,
    muted: bool,
}

impl SweepUnit {
    fn new(ones_complement: bool) -> Self {
        SweepUnit {
            enabled: false,
            negate: false,
            shift: 0,
            period: 0,
            divider: 0,
            reload: false,
            ones_complement,
            muted: false,
        }
    }

    fn write(&mut self, data: u8) {
        self.enabled = data & 0x80 != 0;
        self.period = ((data >> 4) & 0x07) + 1;
        self.negate = data & 0x08 != 0;
        self.shift = data & 0x07;
        self.reload = true;
        self.divider = self.period;
        self.muted = false;
    }

    fn step(&mut self, timer_period: &mut u16) {
        let mut apply_change = false;
        if self.divider == 0 {
            self.divider = self.period;
            if self.enabled && self.shift > 0 && !self.is_muted(*timer_period) {
                apply_change = true;
            }
        } else if self.divider > 0 {
            self.divider -= 1;
        }

        if apply_change {
            if let Some(target) = self.calculate_target(*timer_period) {
                *timer_period = target;
            }
        }

        if self.reload {
            self.reload = false;
            self.divider = self.period;
        }
    }

    fn calculate_target(&mut self, timer_period: u16) -> Option<u16> {
        let change = timer_period >> self.shift;
        let adjustment = if self.negate && self.ones_complement {
            1
        } else {
            0
        };
        let target = if self.negate {
            timer_period.wrapping_sub(change + adjustment)
        } else {
            timer_period.wrapping_add(change)
        };

        if target <= 0x7FF {
            self.muted = false;
            Some(target)
        } else {
            self.muted = true;
            None
        }
    }

    fn is_muted(&self, timer_period: u16) -> bool {
        self.muted || timer_period < 8
    }

    fn reset_mute(&mut self) {
        self.muted = false;
    }
}

struct SquareChannel {
    envelope: Envelope,
    sweep: SweepUnit,
    timer_period: u16,
    timer_value: u16,
    duty_mode: u8,
    duty_position: u8,
    pub length_counter: u8,
    pub enabled: bool,
}

impl SquareChannel {
    fn new(ones_complement: bool) -> Self {
        SquareChannel {
            envelope: Envelope::new(),
            sweep: SweepUnit::new(ones_complement),
            timer_period: 0,
            timer_value: 0,
            duty_mode: 0,
            duty_position: 0,
            length_counter: 0,
            enabled: false,
        }
    }

    fn write_control(&mut self, data: u8) {
        self.duty_mode = (data >> 6) & 0x03;
        self.envelope.set_control(data);
    }

    fn write_sweep(&mut self, data: u8) {
        self.sweep.write(data);
    }

    fn write_timer_low(&mut self, data: u8) {
        self.sweep.reset_mute();
        self.timer_period = (self.timer_period & 0xFF00) | data as u16;
    }

    fn write_timer_high(&mut self, data: u8) {
        self.sweep.reset_mute();
        self.timer_period = (self.timer_period & 0x00FF) | (((data & 0x07) as u16) << 8);
        self.length_counter = LENGTH_TABLE[((data >> 3) & 0x1F) as usize];
        self.envelope.restart();
        self.duty_position = 0;
        self.timer_value = self.timer_period + 1;
    }

    fn clock_timer(&mut self) {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period + 1;
            self.duty_position = (self.duty_position + 1) % 8;
        } else {
            self.timer_value = self.timer_value.saturating_sub(1);
        }
    }

    fn output(&self) -> u8 {
        if !self.enabled
            || self.length_counter == 0
            || self.timer_period < 8
            || DUTY_TABLE[self.duty_mode as usize][self.duty_position as usize] == 0
            || self.sweep.is_muted(self.timer_period)
        {
            0
        } else {
            self.envelope.output()
        }
    }

    fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    fn clock_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope.halt() {
            self.length_counter -= 1;
        }
    }

    fn clock_sweep(&mut self) {
        self.sweep.step(&mut self.timer_period);
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn clear_length(&mut self) {
        self.length_counter = 0;
    }
}

struct TriangleChannel {
    pub enabled: bool,
    length_counter: u8,
    timer_period: u16,
    timer_value: u16,
    sequence_idx: u8,
    linear_counter: u8,
    linear_reload_value: u8,
    linear_control_flag: bool,
    linear_reload_flag: bool,
}

impl TriangleChannel {
    fn new() -> Self {
        TriangleChannel {
            enabled: false,
            length_counter: 0,
            timer_period: 0,
            timer_value: 0,
            sequence_idx: 0,
            linear_counter: 0,
            linear_reload_value: 0,
            linear_control_flag: false,
            linear_reload_flag: false,
        }
    }

    fn write_control(&mut self, data: u8) {
        self.linear_control_flag = data & 0x80 != 0;
        self.linear_reload_value = data & 0x7F;
    }

    fn write_timer_low(&mut self, data: u8) {
        self.timer_period = (self.timer_period & 0xFF00) | data as u16;
    }

    fn write_timer_high(&mut self, data: u8) {
        self.timer_period = (self.timer_period & 0x00FF) | (((data & 0x07) as u16) << 8);
        self.length_counter = LENGTH_TABLE[((data >> 3) & 0x1F) as usize];
        self.linear_reload_flag = true;
        self.sequence_idx = 0;
        self.timer_value = self.timer_period + 1;
    }

    fn clock_timer(&mut self) {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period + 1;
            if self.length_counter > 0 && self.linear_counter > 0 && self.timer_period >= 2 {
                self.sequence_idx = (self.sequence_idx + 1) % 32;
            }
        } else {
            self.timer_value = self.timer_value.saturating_sub(1);
        }
    }

    fn clock_linear_counter(&mut self) {
        if self.linear_reload_flag {
            self.linear_counter = self.linear_reload_value;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }

        if !self.linear_control_flag {
            self.linear_reload_flag = false;
        }
    }

    fn clock_length_counter(&mut self) {
        if self.length_counter > 0 && !self.linear_control_flag {
            self.length_counter -= 1;
        }
    }

    fn output(&self) -> u8 {
        if !self.enabled
            || self.length_counter == 0
            || self.linear_counter == 0
            || self.timer_period < 2
        {
            0
        } else {
            TRIANGLE_TABLE[self.sequence_idx as usize]
        }
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn clear_length(&mut self) {
        self.length_counter = 0;
    }
}

struct NoiseChannel {
    pub enabled: bool,
    envelope: Envelope,
    length_counter: u8,
    timer_period: u16,
    timer_value: u16,
    mode: bool,
    shift_register: u16,
}

impl NoiseChannel {
    fn new() -> Self {
        NoiseChannel {
            enabled: false,
            envelope: Envelope::new(),
            length_counter: 0,
            timer_period: 0,
            timer_value: 0,
            mode: false,
            shift_register: 1,
        }
    }

    fn write_control(&mut self, data: u8) {
        self.envelope.set_control(data);
    }

    fn write_period(&mut self, data: u8) {
        self.mode = data & 0x80 != 0;
        let index = (data & 0x0F) as usize;
        self.timer_period = NOISE_PERIOD_TABLE[index];
        self.timer_value = self.timer_period;
    }

    fn write_length(&mut self, data: u8) {
        self.length_counter = LENGTH_TABLE[((data >> 3) & 0x1F) as usize];
        self.envelope.restart();
    }

    fn clock_timer(&mut self) {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period;
            let feedback = if self.mode {
                (self.shift_register & 1) ^ ((self.shift_register >> 6) & 1)
            } else {
                (self.shift_register & 1) ^ ((self.shift_register >> 1) & 1)
            };

            self.shift_register >>= 1;
            self.shift_register |= feedback << 14;
        } else {
            self.timer_value = self.timer_value.saturating_sub(1);
        }
    }

    fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    fn clock_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope.halt() {
            self.length_counter -= 1;
        }
    }

    fn output(&self) -> u8 {
        if !self.enabled || self.length_counter == 0 || self.shift_register & 1 == 1 {
            0
        } else {
            self.envelope.output()
        }
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn clear_length(&mut self) {
        self.length_counter = 0;
    }
}

struct DmcChannel {
    pub enabled: bool,
    irq_enabled: bool,
    irq_pending: bool,
    loop_flag: bool,
    timer_period: u16,
    timer_value: u16,
    sample_address: u16,
    sample_length: u16,
    current_address: u16,
    bytes_remaining: u16,
    sample_buffer: Option<u8>,
    pending_fetch: bool,
    shift_register: u8,
    bits_remaining: u8,
    silence: bool,
    output_level: u8,
}

impl DmcChannel {
    fn new() -> Self {
        DmcChannel {
            enabled: false,
            irq_enabled: false,
            irq_pending: false,
            loop_flag: false,
            timer_period: DMC_RATE_TABLE[0],
            timer_value: DMC_RATE_TABLE[0],
            sample_address: 0xC000,
            sample_length: 1,
            current_address: 0xC000,
            bytes_remaining: 0,
            sample_buffer: None,
            pending_fetch: false,
            shift_register: 0,
            bits_remaining: 8,
            silence: true,
            output_level: 0,
        }
    }

    fn write_control(&mut self, data: u8) {
        let rate_idx = (data & 0x0F) as usize;
        self.timer_period = DMC_RATE_TABLE[rate_idx];
        self.timer_value = self.timer_period;
        self.loop_flag = data & 0x40 != 0;
        self.irq_enabled = data & 0x80 != 0;
        if !self.irq_enabled {
            self.irq_pending = false;
        }
    }

    fn write_direct_load(&mut self, data: u8) {
        self.output_level = data & 0x7F;
    }

    fn write_sample_address(&mut self, data: u8) {
        self.sample_address = 0xC000 | ((data as u16) << 6);
    }

    fn write_sample_length(&mut self, data: u8) {
        self.sample_length = (data as u16) * 16 + 1;
    }

    fn clock(&mut self) -> Option<u16> {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period;
            if !self.silence {
                if self.shift_register & 1 == 1 {
                    if self.output_level <= 125 {
                        self.output_level += 2;
                    }
                } else if self.output_level >= 2 {
                    self.output_level -= 2;
                }
            }

            self.shift_register >>= 1;
            self.bits_remaining -= 1;

            if self.bits_remaining == 0 {
                self.bits_remaining = 8;
                if let Some(sample) = self.sample_buffer.take() {
                    self.shift_register = sample;
                    self.silence = false;
                } else {
                    self.silence = true;
                }
            }
        } else {
            self.timer_value -= 1;
        }

        if !self.pending_fetch
            && self.sample_buffer.is_none()
            && self.bytes_remaining > 0
            && self.enabled
        {
            self.pending_fetch = true;
            let addr = self.current_address;
            self.current_address = self.current_address.wrapping_add(1);
            if self.current_address == 0 {
                self.current_address = 0x8000;
            }
            return Some(addr);
        }

        None
    }

    fn provide_sample(&mut self, value: u8) {
        if !self.pending_fetch {
            return;
        }

        self.sample_buffer = Some(value);
        self.pending_fetch = false;

        if self.bytes_remaining > 0 {
            self.bytes_remaining -= 1;
        }

        if self.bytes_remaining == 0 {
            if self.loop_flag {
                self.reload_sample();
            } else if self.irq_enabled {
                self.irq_pending = true;
            }
        }
    }

    fn reload_sample(&mut self) {
        self.current_address = self.sample_address;
        self.bytes_remaining = self.sample_length;
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn restart_if_needed(&mut self) {
        if self.enabled && self.bytes_remaining == 0 {
            self.reload_sample();
        }
    }

    fn clear_bytes(&mut self) {
        self.bytes_remaining = 0;
        self.pending_fetch = false;
        self.sample_buffer = None;
    }

    fn output(&self) -> u8 {
        self.output_level
    }

    fn clear_irq(&mut self) {
        self.irq_pending = false;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameCounterMode {
    FourStep,
    FiveStep,
}

struct FrameStep {
    cycle: usize,
    clock_quarter: bool,
    clock_half: bool,
}

const FOUR_STEP_SEQ: [FrameStep; 4] = [
    FrameStep {
        cycle: 3729,
        clock_quarter: true,
        clock_half: false,
    },
    FrameStep {
        cycle: 7457,
        clock_quarter: true,
        clock_half: true,
    },
    FrameStep {
        cycle: 11186,
        clock_quarter: true,
        clock_half: false,
    },
    FrameStep {
        cycle: 14916,
        clock_quarter: true,
        clock_half: true,
    },
];

const FIVE_STEP_SEQ: [FrameStep; 5] = [
    FrameStep {
        cycle: 3729,
        clock_quarter: true,
        clock_half: false,
    },
    FrameStep {
        cycle: 7457,
        clock_quarter: true,
        clock_half: true,
    },
    FrameStep {
        cycle: 11186,
        clock_quarter: true,
        clock_half: false,
    },
    FrameStep {
        cycle: 14916,
        clock_quarter: false,
        clock_half: false,
    },
    FrameStep {
        cycle: 18641,
        clock_quarter: true,
        clock_half: true,
    },
];

#[derive(Default, Clone, Copy)]
struct FrameEvents {
    quarter: bool,
    half: bool,
    irq: bool,
}

struct FrameCounterReset {
    delay: u8,
    mode: FrameCounterMode,
}

struct FrameCounter {
    mode: FrameCounterMode,
    step_index: usize,
    cycle_counter: usize,
    irq_inhibit: bool,
}

impl FrameCounter {
    fn new() -> Self {
        FrameCounter {
            mode: FrameCounterMode::FourStep,
            step_index: 0,
            cycle_counter: 0,
            irq_inhibit: false,
        }
    }

    fn set_mode(&mut self, mode: FrameCounterMode) {
        self.mode = mode;
    }

    fn set_irq_inhibit(&mut self, irq_inhibit: bool) {
        self.irq_inhibit = irq_inhibit;
    }

    fn reset(&mut self) {
        self.step_index = 0;
        self.cycle_counter = 0;
    }

    fn tick(&mut self) -> FrameEvents {
        self.cycle_counter += 1;
        let sequence = match self.mode {
            FrameCounterMode::FourStep => &FOUR_STEP_SEQ[..],
            FrameCounterMode::FiveStep => &FIVE_STEP_SEQ[..],
        };

        let mut events = FrameEvents::default();

        if self.step_index < sequence.len() && self.cycle_counter == sequence[self.step_index].cycle
        {
            let step = &sequence[self.step_index];
            events.quarter = step.clock_quarter;
            events.half = step.clock_half;

            self.step_index += 1;
            match self.mode {
                FrameCounterMode::FourStep => {
                    if self.step_index == sequence.len() {
                        if !self.irq_inhibit {
                            events.irq = true;
                        }
                        self.step_index = 0;
                        self.cycle_counter = 0;
                    }
                }
                FrameCounterMode::FiveStep => {
                    if self.step_index == sequence.len() {
                        self.step_index = 0;
                        self.cycle_counter = 0;
                    }
                }
            }
        }

        events
    }
}

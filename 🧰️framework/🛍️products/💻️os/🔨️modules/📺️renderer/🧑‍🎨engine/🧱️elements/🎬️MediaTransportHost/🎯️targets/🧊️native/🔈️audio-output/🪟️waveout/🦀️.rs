//! 🪟️ The Windows output: the multimedia service `winmm` (`waveOut`, a system library), linked directly, no crate. 16-bit PCM in four 50 ms
//! headers that are refilled whenever the caller writes or asks for the queue depth; no callback thread, so nothing runs while the caller is idle.

use super::{f32_to_i16, AudioFormat, AudioOutputError, AudioOutputPort, AudioOutputStats};
use std::collections::VecDeque;
use std::ffi::c_void;

type WaveOut = *mut c_void;

#[repr(C, packed(1))]
struct WaveFormat {
    tag: u16,
    channels: u16,
    samples_per_second: u32,
    bytes_per_second: u32,
    block_align: u16,
    bits_per_sample: u16,
    extra_size: u16,
}

#[repr(C)]
struct WaveHeader {
    data: *mut u8,
    length: u32,
    recorded: u32,
    user: usize,
    flags: u32,
    loops: u32,
    next: *mut WaveHeader,
    reserved: usize,
}

#[link(name = "winmm")]
extern "system" {
    fn waveOutOpen(device: *mut WaveOut, id: u32, format: *const WaveFormat, callback: usize, instance: usize, flags: u32) -> u32;
    fn waveOutPrepareHeader(device: WaveOut, header: *mut WaveHeader, size: u32) -> u32;
    fn waveOutUnprepareHeader(device: WaveOut, header: *mut WaveHeader, size: u32) -> u32;
    fn waveOutWrite(device: WaveOut, header: *mut WaveHeader, size: u32) -> u32;
    fn waveOutPause(device: WaveOut) -> u32;
    fn waveOutRestart(device: WaveOut) -> u32;
    fn waveOutReset(device: WaveOut) -> u32;
    fn waveOutClose(device: WaveOut) -> u32;
}

const WAVE_MAPPER: u32 = u32::MAX;
const FORMAT_PCM: u16 = 1;
const HEADER_DONE: u32 = 1;
const HEADER_COUNT: usize = 4;
const PENDING_SECONDS_DIVISOR: usize = 2;

struct Slot {
    header: WaveHeader,
    samples: Vec<i16>,
    in_flight: bool,
}

/// 🪟️ An open `waveOut` device.
pub struct WaveOutput {
    format: AudioFormat,
    device: WaveOut,
    slots: Vec<Box<Slot>>,
    pending: VecDeque<i16>,
    started: bool,
    dry: bool,
    stats: AudioOutputStats,
}

unsafe impl Send for WaveOutput {}

fn status(call: &str, code: u32) -> Result<(), AudioOutputError> {
    if code == 0 { Ok(()) } else { Err(AudioOutputError::DeviceLost(format!("{call} answered {code}"))) }
}

pub(super) fn open(format: AudioFormat) -> Result<WaveOutput, AudioOutputError> {
    if format.sample_rate == 0 || format.channels == 0 {
        return Err(AudioOutputError::Format("sample rate and channels must be positive".into()));
    }
    let block_align = format.channels * 2;
    let description = WaveFormat { tag: FORMAT_PCM, channels: format.channels, samples_per_second: format.sample_rate, bytes_per_second: format.sample_rate * u32::from(block_align), block_align, bits_per_sample: 16, extra_size: 0 };
    let mut device: WaveOut = std::ptr::null_mut();
    let code = unsafe { waveOutOpen(&mut device, WAVE_MAPPER, &description, 0, 0, 0) };
    if code != 0 {
        return Err(AudioOutputError::Unavailable(format!("waveOutOpen answered {code}")));
    }
    unsafe { waveOutPause(device) };
    let frames = (format.sample_rate / 20).max(64) as usize;
    let slots = (0..HEADER_COUNT)
        .map(|_| Box::new(Slot { header: WaveHeader { data: std::ptr::null_mut(), length: 0, recorded: 0, user: 0, flags: 0, loops: 0, next: std::ptr::null_mut(), reserved: 0 }, samples: Vec::with_capacity(frames * usize::from(format.channels)), in_flight: false }))
        .collect();
    Ok(WaveOutput { format, device, slots, pending: VecDeque::new(), started: false, dry: false, stats: AudioOutputStats::default() })
}

impl WaveOutput {
    fn frame_samples(&self) -> usize {
        (self.format.sample_rate / 20).max(64) as usize * usize::from(self.format.channels)
    }

    fn reap(&mut self) {
        let channels = u64::from(self.format.channels);
        for slot in &mut self.slots {
            if slot.in_flight && slot.header.flags & HEADER_DONE != 0 {
                unsafe { waveOutUnprepareHeader(self.device, &mut slot.header, std::mem::size_of::<WaveHeader>() as u32) };
                self.stats.played_frames += slot.samples.len() as u64 / channels;
                slot.in_flight = false;
            }
        }
    }

    fn pump(&mut self) -> Result<(), AudioOutputError> {
        self.reap();
        let wanted = self.frame_samples();
        for index in 0..self.slots.len() {
            if self.slots[index].in_flight || self.pending.is_empty() {
                continue;
            }
            let take = wanted.min(self.pending.len());
            let device = self.device;
            let slot = &mut self.slots[index];
            slot.samples.clear();
            slot.samples.extend(self.pending.drain(..take));
            slot.header.data = slot.samples.as_mut_ptr() as *mut u8;
            slot.header.length = (slot.samples.len() * 2) as u32;
            slot.header.flags = 0;
            let size = std::mem::size_of::<WaveHeader>() as u32;
            status("waveOutPrepareHeader", unsafe { waveOutPrepareHeader(device, &mut slot.header, size) })?;
            status("waveOutWrite", unsafe { waveOutWrite(device, &mut slot.header, size) })?;
            slot.in_flight = true;
        }
        let idle = self.slots.iter().all(|slot| !slot.in_flight) && self.pending.is_empty();
        if self.started && idle && !self.dry {
            self.stats.underruns += 1;
        }
        self.dry = idle;
        Ok(())
    }
}

impl AudioOutputPort for WaveOutput {
    fn format(&self) -> AudioFormat {
        self.format
    }
    fn write(&mut self, samples: &[f32]) -> Result<usize, AudioOutputError> {
        if self.device.is_null() {
            return Err(AudioOutputError::Closed);
        }
        let channels = usize::from(self.format.channels);
        let limit = self.format.sample_rate as usize * channels / PENDING_SECONDS_DIVISOR;
        let frames = (samples.len() / channels).min(limit.saturating_sub(self.pending.len()) / channels);
        self.pending.extend(f32_to_i16(&samples[..frames * channels]));
        self.stats.submitted_frames += frames as u64;
        self.pump()?;
        Ok(frames)
    }
    fn queued_frames(&self) -> usize {
        let in_flight: usize = self.slots.iter().filter(|slot| slot.in_flight && slot.header.flags & HEADER_DONE == 0).map(|slot| slot.samples.len()).sum();
        (self.pending.len() + in_flight) / usize::from(self.format.channels)
    }
    fn start(&mut self) -> Result<(), AudioOutputError> {
        if self.device.is_null() {
            return Err(AudioOutputError::Closed);
        }
        self.started = true;
        status("waveOutRestart", unsafe { waveOutRestart(self.device) })?;
        self.pump()
    }
    fn pause(&mut self) -> Result<(), AudioOutputError> {
        if self.device.is_null() {
            return Err(AudioOutputError::Closed);
        }
        self.started = false;
        status("waveOutPause", unsafe { waveOutPause(self.device) })
    }
    fn flush(&mut self) -> Result<(), AudioOutputError> {
        if self.device.is_null() {
            return Err(AudioOutputError::Closed);
        }
        self.pending.clear();
        status("waveOutReset", unsafe { waveOutReset(self.device) })?;
        self.reap();
        if !self.started {
            unsafe { waveOutPause(self.device) };
        }
        Ok(())
    }
    fn stats(&self) -> AudioOutputStats {
        self.stats
    }
    fn close(&mut self) {
        if self.device.is_null() {
            return;
        }
        unsafe { waveOutReset(self.device) };
        self.reap();
        unsafe { waveOutClose(self.device) };
        self.device = std::ptr::null_mut();
    }
}

impl Drop for WaveOutput {
    fn drop(&mut self) {
        self.close();
    }
}

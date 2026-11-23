//! 🍎️ The macOS output: AudioToolbox's `AudioQueue` (a system framework), linked directly, no crate. A ring of `f32` frames sits between the
//! caller's `write` and the queue's completion callback, which refills each buffer from the ring and pads a dry ring with silence.

use super::{AudioFormat, AudioOutputError, AudioOutputPort, AudioOutputStats};
use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, MutexGuard};

type QueueRef = *mut c_void;

#[repr(C)]
struct StreamDescription {
    sample_rate: f64,
    format_id: u32,
    format_flags: u32,
    bytes_per_packet: u32,
    frames_per_packet: u32,
    bytes_per_frame: u32,
    channels_per_frame: u32,
    bits_per_channel: u32,
    reserved: u32,
}

#[repr(C)]
struct QueueBuffer {
    capacity: u32,
    data: *mut c_void,
    byte_size: u32,
    user_data: *mut c_void,
    packet_capacity: u32,
    packet_descriptions: *mut c_void,
    packet_count: u32,
}

type Callback = extern "C" fn(*mut c_void, QueueRef, *mut QueueBuffer);

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioQueueNewOutput(format: *const StreamDescription, callback: Callback, user_data: *mut c_void, run_loop: *mut c_void, run_loop_mode: *mut c_void, flags: u32, queue: *mut QueueRef) -> i32;
    fn AudioQueueAllocateBuffer(queue: QueueRef, bytes: u32, buffer: *mut *mut QueueBuffer) -> i32;
    fn AudioQueueEnqueueBuffer(queue: QueueRef, buffer: *mut QueueBuffer, descriptions: u32, packet_descriptions: *const c_void) -> i32;
    fn AudioQueueStart(queue: QueueRef, start_time: *const c_void) -> i32;
    fn AudioQueuePause(queue: QueueRef) -> i32;
    fn AudioQueueStop(queue: QueueRef, immediate: u8) -> i32;
    fn AudioQueueDispose(queue: QueueRef, immediate: u8) -> i32;
}

const LINEAR_PCM: u32 = 0x6C70_636D;
const FLOAT_PACKED: u32 = 1 | 8;
const BUFFER_COUNT: usize = 4;

#[derive(Default)]
struct Shared {
    ring: VecDeque<f32>,
    started: bool,
    stats: AudioOutputStats,
}

struct Context {
    shared: Mutex<Shared>,
    channels: usize,
}

fn lock(context: &Context) -> MutexGuard<'_, Shared> {
    context.shared.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

extern "C" fn refill(user_data: *mut c_void, queue: QueueRef, buffer: *mut QueueBuffer) {
    let context = unsafe { &*(user_data as *const Context) };
    let buffer = unsafe { &mut *buffer };
    let wanted = buffer.capacity as usize / 4;
    let out = unsafe { std::slice::from_raw_parts_mut(buffer.data as *mut f32, wanted) };
    let mut shared = lock(context);
    let take = wanted.min(shared.ring.len());
    for (slot, sample) in out.iter_mut().zip(shared.ring.drain(..take)) {
        *slot = sample;
    }
    out[take..].fill(0.0);
    if shared.started {
        shared.stats.played_frames += (take / context.channels) as u64;
        if take < wanted {
            shared.stats.underruns += 1;
        }
    }
    drop(shared);
    buffer.byte_size = buffer.capacity;
    unsafe { AudioQueueEnqueueBuffer(queue, buffer, 0, std::ptr::null()) };
}

/// 🍎️ An open `AudioQueue`.
pub struct AudioQueueOutput {
    format: AudioFormat,
    queue: QueueRef,
    context: *const Context,
    closed_stats: AudioOutputStats,
}

unsafe impl Send for AudioQueueOutput {}

fn status(call: &str, code: i32) -> Result<(), AudioOutputError> {
    if code == 0 { Ok(()) } else { Err(AudioOutputError::Unavailable(format!("{call} answered {code}"))) }
}

pub(super) fn open(format: AudioFormat) -> Result<AudioQueueOutput, AudioOutputError> {
    if format.sample_rate == 0 || format.channels == 0 {
        return Err(AudioOutputError::Format("sample rate and channels must be positive".into()));
    }
    let bytes_per_frame = u32::from(format.channels) * 4;
    let description = StreamDescription { sample_rate: f64::from(format.sample_rate), format_id: LINEAR_PCM, format_flags: FLOAT_PACKED, bytes_per_packet: bytes_per_frame, frames_per_packet: 1, bytes_per_frame, channels_per_frame: u32::from(format.channels), bits_per_channel: 32, reserved: 0 };
    let context = Box::into_raw(Box::new(Context { shared: Mutex::new(Shared::default()), channels: usize::from(format.channels) }));
    let mut queue: QueueRef = std::ptr::null_mut();
    let created = status("AudioQueueNewOutput", unsafe { AudioQueueNewOutput(&description, refill, context as *mut c_void, std::ptr::null_mut(), std::ptr::null_mut(), 0, &mut queue) });
    if let Err(error) = created {
        drop(unsafe { Box::from_raw(context) });
        return Err(error);
    }
    let bytes = (format.sample_rate / 50).max(64) * bytes_per_frame;
    for _ in 0..BUFFER_COUNT {
        let mut buffer: *mut QueueBuffer = std::ptr::null_mut();
        let allocated = status("AudioQueueAllocateBuffer", unsafe { AudioQueueAllocateBuffer(queue, bytes, &mut buffer) });
        if let Err(error) = allocated {
            unsafe { AudioQueueDispose(queue, 1) };
            drop(unsafe { Box::from_raw(context) });
            return Err(error);
        }
        unsafe { (*buffer).capacity = bytes };
        refill(context as *mut c_void, queue, buffer);
    }
    Ok(AudioQueueOutput { format, queue, context, closed_stats: AudioOutputStats::default() })
}

impl AudioQueueOutput {
    fn context(&self) -> &Context {
        unsafe { &*self.context }
    }
}

impl AudioOutputPort for AudioQueueOutput {
    fn format(&self) -> AudioFormat {
        self.format
    }
    fn write(&mut self, samples: &[f32]) -> Result<usize, AudioOutputError> {
        if self.queue.is_null() {
            return Err(AudioOutputError::Closed);
        }
        let channels = usize::from(self.format.channels);
        let mut shared = lock(self.context());
        let room = (usize::from(self.format.channels) * self.format.sample_rate as usize / 2).saturating_sub(shared.ring.len()) / channels;
        let frames = (samples.len() / channels).min(room);
        shared.ring.extend(&samples[..frames * channels]);
        shared.stats.submitted_frames += frames as u64;
        Ok(frames)
    }
    fn queued_frames(&self) -> usize {
        if self.queue.is_null() { return 0 }
        lock(self.context()).ring.len() / usize::from(self.format.channels)
    }
    fn start(&mut self) -> Result<(), AudioOutputError> {
        if self.queue.is_null() {
            return Err(AudioOutputError::Closed);
        }
        lock(self.context()).started = true;
        status("AudioQueueStart", unsafe { AudioQueueStart(self.queue, std::ptr::null()) }).map_err(|error| AudioOutputError::DeviceLost(error.to_string()))
    }
    fn pause(&mut self) -> Result<(), AudioOutputError> {
        if self.queue.is_null() {
            return Err(AudioOutputError::Closed);
        }
        lock(self.context()).started = false;
        status("AudioQueuePause", unsafe { AudioQueuePause(self.queue) }).map_err(|error| AudioOutputError::DeviceLost(error.to_string()))
    }
    fn flush(&mut self) -> Result<(), AudioOutputError> {
        if self.queue.is_null() {
            return Err(AudioOutputError::Closed);
        }
        {
            let mut shared = lock(self.context());
            shared.ring.clear();
            shared.started = false;
        }
        status("AudioQueueStop", unsafe { AudioQueueStop(self.queue, 1) }).map_err(|error| AudioOutputError::DeviceLost(error.to_string()))
    }
    fn stats(&self) -> AudioOutputStats {
        if self.queue.is_null() { return self.closed_stats }
        lock(self.context()).stats
    }
    fn close(&mut self) {
        if self.queue.is_null() {
            return;
        }
        unsafe { AudioQueueDispose(self.queue, 1) };
        let stats = lock(self.context()).stats;
        self.closed_stats = stats;
        self.queue = std::ptr::null_mut();
        drop(unsafe { Box::from_raw(self.context as *mut Context) });
    }
}

impl Drop for AudioQueueOutput {
    fn drop(&mut self) {
        self.close();
    }
}

//! 🐧️ The Linux (and other Unix) output: ALSA's `libasound`, a system library opened at run time with `dlopen`, so a machine without it builds
//! and runs and answers silence. Non-blocking interleaved 16-bit writes sized by `snd_pcm_avail_update`; an xrun is recovered and counted.

use super::{f32_to_i16, AudioFormat, AudioOutputError, AudioOutputPort, AudioOutputStats};
use std::collections::VecDeque;
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, CString};

#[cfg_attr(target_os = "linux", link(name = "dl"))]
extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

const RTLD_NOW: c_int = 2;
const PLAYBACK: c_int = 0;
const NONBLOCK: c_int = 1;
const FORMAT_S16_LE: c_int = 2;
const ACCESS_RW_INTERLEAVED: c_int = 3;
const EPIPE: c_long = -32;
const LATENCY_MICROSECONDS: c_uint = 100_000;

struct Api {
    library: *mut c_void,
    open: unsafe extern "C" fn(*mut *mut c_void, *const c_char, c_int, c_int) -> c_int,
    set_params: unsafe extern "C" fn(*mut c_void, c_int, c_int, c_uint, c_uint, c_int, c_uint) -> c_int,
    writei: unsafe extern "C" fn(*mut c_void, *const c_void, c_ulong) -> c_long,
    avail_update: unsafe extern "C" fn(*mut c_void) -> c_long,
    delay: unsafe extern "C" fn(*mut c_void, *mut c_long) -> c_int,
    recover: unsafe extern "C" fn(*mut c_void, c_int, c_int) -> c_int,
    pause: unsafe extern "C" fn(*mut c_void, c_int) -> c_int,
    drop_frames: unsafe extern "C" fn(*mut c_void) -> c_int,
    prepare: unsafe extern "C" fn(*mut c_void) -> c_int,
    close: unsafe extern "C" fn(*mut c_void) -> c_int,
}

macro_rules! symbol {
    ($library:expr, $name:literal) => {{
        let name = CString::new($name).map_err(|_| AudioOutputError::Unavailable("symbol name".into()))?;
        let address = unsafe { dlsym($library, name.as_ptr()) };
        if address.is_null() {
            unsafe { dlclose($library) };
            return Err(AudioOutputError::Unavailable(concat!("libasound lacks ", $name).into()));
        }
        unsafe { std::mem::transmute(address) }
    }};
}

fn load() -> Result<Api, AudioOutputError> {
    let library = ["libasound.so.2", "libasound.so"].iter().find_map(|name| {
        let name = CString::new(*name).ok()?;
        let handle = unsafe { dlopen(name.as_ptr(), RTLD_NOW) };
        (!handle.is_null()).then_some(handle)
    });
    let library = library.ok_or_else(|| AudioOutputError::Unavailable("libasound is not installed".into()))?;
    Ok(Api {
        library,
        open: symbol!(library, "snd_pcm_open"),
        set_params: symbol!(library, "snd_pcm_set_params"),
        writei: symbol!(library, "snd_pcm_writei"),
        avail_update: symbol!(library, "snd_pcm_avail_update"),
        delay: symbol!(library, "snd_pcm_delay"),
        recover: symbol!(library, "snd_pcm_recover"),
        pause: symbol!(library, "snd_pcm_pause"),
        drop_frames: symbol!(library, "snd_pcm_drop"),
        prepare: symbol!(library, "snd_pcm_prepare"),
        close: symbol!(library, "snd_pcm_close"),
    })
}

/// 🐧️ An open ALSA playback device.
pub struct AlsaOutput {
    format: AudioFormat,
    api: Api,
    pcm: *mut c_void,
    pending: VecDeque<i16>,
    started: bool,
    stats: AudioOutputStats,
}

unsafe impl Send for AlsaOutput {}

pub(super) fn open(format: AudioFormat) -> Result<AlsaOutput, AudioOutputError> {
    if format.sample_rate == 0 || format.channels == 0 {
        return Err(AudioOutputError::Format("sample rate and channels must be positive".into()));
    }
    let api = load()?;
    let mut pcm: *mut c_void = std::ptr::null_mut();
    let name = CString::new("default").map_err(|_| AudioOutputError::Unavailable("device name".into()))?;
    let code = unsafe { (api.open)(&mut pcm, name.as_ptr(), PLAYBACK, NONBLOCK) };
    if code < 0 {
        unsafe { dlclose(api.library) };
        return Err(AudioOutputError::Unavailable(format!("snd_pcm_open answered {code}")));
    }
    let code = unsafe { (api.set_params)(pcm, FORMAT_S16_LE, ACCESS_RW_INTERLEAVED, c_uint::from(format.channels), format.sample_rate, 1, LATENCY_MICROSECONDS) };
    if code < 0 {
        unsafe { (api.close)(pcm) };
        unsafe { dlclose(api.library) };
        return Err(AudioOutputError::Format(format!("snd_pcm_set_params answered {code}")));
    }
    Ok(AlsaOutput { format, api, pcm, pending: VecDeque::new(), started: false, stats: AudioOutputStats::default() })
}

impl AlsaOutput {
    fn pump(&mut self) -> Result<(), AudioOutputError> {
        if !self.started {
            return Ok(());
        }
        let channels = usize::from(self.format.channels);
        let mut room = unsafe { (self.api.avail_update)(self.pcm) };
        if room < 0 {
            if unsafe { (self.api.recover)(self.pcm, room as c_int, 1) } < 0 {
                return Err(AudioOutputError::DeviceLost(format!("snd_pcm_avail_update answered {room}")));
            }
            self.stats.underruns += 1;
            room = unsafe { (self.api.avail_update)(self.pcm) }.max(0);
        }
        let frames = (room as usize).min(self.pending.len() / channels);
        if frames == 0 {
            return Ok(());
        }
        let block: Vec<i16> = self.pending.drain(..frames * channels).collect();
        let written = unsafe { (self.api.writei)(self.pcm, block.as_ptr() as *const c_void, frames as c_ulong) };
        if written == EPIPE || written < 0 {
            if unsafe { (self.api.recover)(self.pcm, written as c_int, 1) } < 0 {
                return Err(AudioOutputError::DeviceLost(format!("snd_pcm_writei answered {written}")));
            }
            self.stats.underruns += 1;
            return Ok(());
        }
        self.stats.played_frames += written as u64;
        Ok(())
    }
}

impl AudioOutputPort for AlsaOutput {
    fn format(&self) -> AudioFormat {
        self.format
    }
    fn write(&mut self, samples: &[f32]) -> Result<usize, AudioOutputError> {
        if self.pcm.is_null() {
            return Err(AudioOutputError::Closed);
        }
        let channels = usize::from(self.format.channels);
        let limit = self.format.sample_rate as usize * channels / 2;
        let frames = (samples.len() / channels).min(limit.saturating_sub(self.pending.len()) / channels);
        self.pending.extend(f32_to_i16(&samples[..frames * channels]));
        self.stats.submitted_frames += frames as u64;
        self.pump()?;
        Ok(frames)
    }
    fn queued_frames(&self) -> usize {
        if self.pcm.is_null() {
            return 0;
        }
        let mut delay: c_long = 0;
        let device = if unsafe { (self.api.delay)(self.pcm, &mut delay) } == 0 { delay.max(0) as usize } else { 0 };
        self.pending.len() / usize::from(self.format.channels) + device
    }
    fn start(&mut self) -> Result<(), AudioOutputError> {
        if self.pcm.is_null() {
            return Err(AudioOutputError::Closed);
        }
        if !self.started {
            unsafe { (self.api.pause)(self.pcm, 0) };
        }
        self.started = true;
        self.pump()
    }
    fn pause(&mut self) -> Result<(), AudioOutputError> {
        if self.pcm.is_null() {
            return Err(AudioOutputError::Closed);
        }
        self.started = false;
        unsafe { (self.api.pause)(self.pcm, 1) };
        Ok(())
    }
    fn flush(&mut self) -> Result<(), AudioOutputError> {
        if self.pcm.is_null() {
            return Err(AudioOutputError::Closed);
        }
        self.pending.clear();
        unsafe { (self.api.drop_frames)(self.pcm) };
        let code = unsafe { (self.api.prepare)(self.pcm) };
        if code < 0 { Err(AudioOutputError::DeviceLost(format!("snd_pcm_prepare answered {code}"))) } else { Ok(()) }
    }
    fn stats(&self) -> AudioOutputStats {
        self.stats
    }
    fn close(&mut self) {
        if self.pcm.is_null() {
            return;
        }
        unsafe { (self.api.drop_frames)(self.pcm) };
        unsafe { (self.api.close)(self.pcm) };
        unsafe { dlclose(self.api.library) };
        self.pcm = std::ptr::null_mut();
    }
}

impl Drop for AlsaOutput {
    fn drop(&mut self) {
        self.close();
    }
}

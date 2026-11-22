//! 🔈️ The audio output port of the native media host: the one interface the presenter plays decoded blocks through. The operating system's
//! audio services are system libraries behind it (macOS AudioToolbox `AudioQueue`, Windows `winmm` `waveOut`, Linux ALSA `libasound`, opened at
//! run time), never a crate. A machine with no usable device answers [`NullAudioOutput`] with the reason, so playback degrades to silence with
//! a visible position instead of failing. The browser overlay's twin is the `HTMLMediaElement`'s own output.

#[cfg(target_os = "macos")]
#[path = "🍎️audio-queue/🦀️.rs"]
mod audio_queue;
#[cfg(target_os = "windows")]
#[path = "🪟️waveout/🦀️.rs"]
mod waveout;
#[cfg(all(unix, not(target_os = "macos")))]
#[path = "🐧️alsa/🦀️.rs"]
mod alsa;

/// 🎚️ What a stream plays: interleaved `f32` frames.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
}

/// 🚨️ Why an output stopped short.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioOutputError {
    Unavailable(String),
    Format(String),
    DeviceLost(String),
    Closed,
}

impl AudioOutputError {
    /// 🔤️ The stable code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Unavailable(_) => "unavailable",
            Self::Format(_) => "format",
            Self::DeviceLost(_) => "device-lost",
            Self::Closed => "closed",
        }
    }
}

impl std::fmt::Display for AudioOutputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(reason) | Self::Format(reason) | Self::DeviceLost(reason) => write!(formatter, "{}: {reason}", self.code()),
            Self::Closed => formatter.write_str("closed: the audio output was closed"),
        }
    }
}

impl std::error::Error for AudioOutputError {}

/// 📊️ What an output has done: frames accepted, frames the device took, and the times the device ran dry while started.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AudioOutputStats {
    pub submitted_frames: u64,
    pub played_frames: u64,
    pub underruns: u64,
}

/// 🔈️ One open audio stream. `write` never blocks: it takes as many whole frames as the output has room for and answers that count, so a
/// caller that is cancelled or paused simply stops writing. `queued_frames` is what was accepted and not yet played, the caller's latency.
pub trait AudioOutputPort: Send {
    fn format(&self) -> AudioFormat;
    fn write(&mut self, samples: &[f32]) -> Result<usize, AudioOutputError>;
    fn queued_frames(&self) -> usize;
    fn start(&mut self) -> Result<(), AudioOutputError>;
    fn pause(&mut self) -> Result<(), AudioOutputError>;
    fn flush(&mut self) -> Result<(), AudioOutputError>;
    fn stats(&self) -> AudioOutputStats;
    fn close(&mut self);
}

/// 🔇️ Opens a stream, or says why it cannot. The signature the presenter holds so a host (or a law) chooses the device.
pub type AudioOutputOpener = Box<dyn FnMut(AudioFormat) -> Box<dyn AudioOutputPort> + Send>;

/// 🔇️ The silent output: accepts everything, plays at the speed the caller's clock runs, remembers why it is silent.
#[derive(Debug)]
pub struct NullAudioOutput {
    format: AudioFormat,
    reason: String,
    started: bool,
    closed: bool,
    stats: AudioOutputStats,
}

impl NullAudioOutput {
    pub fn new(format: AudioFormat, reason: impl Into<String>) -> Self {
        Self { format, reason: reason.into(), started: false, closed: false, stats: AudioOutputStats::default() }
    }

    /// 💬️ Why this output is silent.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl AudioOutputPort for NullAudioOutput {
    fn format(&self) -> AudioFormat {
        self.format
    }
    fn write(&mut self, samples: &[f32]) -> Result<usize, AudioOutputError> {
        if self.closed {
            return Err(AudioOutputError::Closed);
        }
        let frames = samples.len() / usize::from(self.format.channels.max(1));
        self.stats.submitted_frames += frames as u64;
        if self.started {
            self.stats.played_frames += frames as u64;
        }
        Ok(frames)
    }
    fn queued_frames(&self) -> usize {
        0
    }
    fn start(&mut self) -> Result<(), AudioOutputError> {
        self.started = true;
        Ok(())
    }
    fn pause(&mut self) -> Result<(), AudioOutputError> {
        self.started = false;
        Ok(())
    }
    fn flush(&mut self) -> Result<(), AudioOutputError> {
        Ok(())
    }
    fn stats(&self) -> AudioOutputStats {
        self.stats
    }
    fn close(&mut self) {
        self.closed = true;
        self.started = false;
    }
}

/// 🧪️ A first-party output that keeps every sample it is handed, in order, for laws: it has room for `capacity_frames` queued frames and
/// plays them out when [`Self::drain`] is called, so back-pressure, flush and underruns are observable without a device.
#[derive(Debug)]
pub struct RecordingAudioOutput {
    format: AudioFormat,
    capacity_frames: usize,
    queue: std::collections::VecDeque<f32>,
    pub played: Vec<f32>,
    pub events: Vec<&'static str>,
    started: bool,
    closed: bool,
    stats: AudioOutputStats,
}

impl RecordingAudioOutput {
    pub fn new(format: AudioFormat, capacity_frames: usize) -> Self {
        Self { format, capacity_frames, queue: Default::default(), played: Vec::new(), events: Vec::new(), started: false, closed: false, stats: AudioOutputStats::default() }
    }

    /// 🔊️ Plays `frames` frames out of the queue like a device would; asking for more than is queued while started counts one underrun.
    pub fn drain(&mut self, frames: usize) {
        if !self.started {
            return;
        }
        let channels = usize::from(self.format.channels.max(1));
        let take = (frames * channels).min(self.queue.len());
        self.played.extend(self.queue.drain(..take));
        self.stats.played_frames += (take / channels) as u64;
        if take < frames * channels {
            self.stats.underruns += 1;
        }
    }
}

impl AudioOutputPort for RecordingAudioOutput {
    fn format(&self) -> AudioFormat {
        self.format
    }
    fn write(&mut self, samples: &[f32]) -> Result<usize, AudioOutputError> {
        if self.closed {
            return Err(AudioOutputError::Closed);
        }
        let channels = usize::from(self.format.channels.max(1));
        let room = self.capacity_frames.saturating_sub(self.queue.len() / channels);
        let frames = (samples.len() / channels).min(room);
        self.queue.extend(&samples[..frames * channels]);
        self.stats.submitted_frames += frames as u64;
        Ok(frames)
    }
    fn queued_frames(&self) -> usize {
        self.queue.len() / usize::from(self.format.channels.max(1))
    }
    fn start(&mut self) -> Result<(), AudioOutputError> {
        self.started = true;
        self.events.push("start");
        Ok(())
    }
    fn pause(&mut self) -> Result<(), AudioOutputError> {
        self.started = false;
        self.events.push("pause");
        Ok(())
    }
    fn flush(&mut self) -> Result<(), AudioOutputError> {
        self.queue.clear();
        self.events.push("flush");
        Ok(())
    }
    fn stats(&self) -> AudioOutputStats {
        self.stats
    }
    fn close(&mut self) {
        self.closed = true;
        self.started = false;
        self.events.push("close");
    }
}

/// 🔊️ Opens the operating system's default output for `format`: AudioToolbox on macOS, `waveOut` on Windows, ALSA elsewhere on Unix. Any
/// failure (no library, no device, a format the device refuses) answers a [`NullAudioOutput`] carrying the error text.
pub fn open_system_audio_output(format: AudioFormat) -> Box<dyn AudioOutputPort> {
    open_platform(format).unwrap_or_else(|error| Box::new(NullAudioOutput::new(format, error.to_string())))
}

#[cfg(target_os = "macos")]
fn open_platform(format: AudioFormat) -> Result<Box<dyn AudioOutputPort>, AudioOutputError> {
    audio_queue::open(format).map(|output| Box::new(output) as Box<dyn AudioOutputPort>)
}

#[cfg(target_os = "windows")]
fn open_platform(format: AudioFormat) -> Result<Box<dyn AudioOutputPort>, AudioOutputError> {
    waveout::open(format).map(|output| Box::new(output) as Box<dyn AudioOutputPort>)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn open_platform(format: AudioFormat) -> Result<Box<dyn AudioOutputPort>, AudioOutputError> {
    alsa::open(format).map(|output| Box::new(output) as Box<dyn AudioOutputPort>)
}

#[cfg(not(any(unix, target_os = "windows")))]
fn open_platform(_format: AudioFormat) -> Result<Box<dyn AudioOutputPort>, AudioOutputError> {
    Err(AudioOutputError::Unavailable("no audio service on this platform".into()))
}

/// 🎚️ `f32` frames to signed 16-bit little-endian samples, clamped; shared by the integer-only device paths.
#[cfg_attr(target_os = "macos", allow(dead_code))]
pub(crate) fn f32_to_i16(samples: &[f32]) -> Vec<i16> {
    samples.iter().map(|sample| (sample.clamp(-1.0, 1.0) * 32767.0).round() as i16).collect()
}

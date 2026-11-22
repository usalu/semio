//! 🔊️ RIFF/WAVE audio, first-party: PCM (8, 16, 24 and 32 bit), IEEE float (32 and 64 bit), A-law and µ-law to interleaved `f32`
//! samples, and PCM16 back to bytes. The native twin of what the browser's media element decodes for `audio/wav`.
//! <https://www.mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html> · <https://www.itu.int/rec/T-REC-G.711>

/// 🚨️ Everything the WAV decoder refuses; `code()` is the language-agnostic vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WavError {
    NotWav,
    Truncated,
    Malformed { what: &'static str },
    Unsupported { format: u16, bits: u16 },
    TooLarge { bytes: u64 },
    Cancelled,
}

impl WavError {
    /// 🔤️ The refusal's stable code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotWav => "notWav",
            Self::Truncated => "truncated",
            Self::Malformed { .. } => "malformed",
            Self::Unsupported { .. } => "unsupported",
            Self::TooLarge { .. } => "tooLarge",
            Self::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for WavError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotWav => formatter.write_str("audio data is not a RIFF/WAVE file"),
            Self::Truncated => formatter.write_str("audio data ends before its structure does"),
            Self::Malformed { what } => write!(formatter, "audio data is malformed: {what}"),
            Self::Unsupported { format, bits } => write!(formatter, "WAVE format tag {format} at {bits} bits is not decoded natively"),
            Self::TooLarge { bytes } => write!(formatter, "audio data of {bytes} bytes exceeds the {WAV_MAXIMUM_DATA_BYTES} byte decode ceiling"),
            Self::Cancelled => formatter.write_str("audio decoding was cancelled"),
        }
    }
}

impl std::error::Error for WavError {}

/// 📏️ The most sample bytes one decode admits.
pub const WAV_MAXIMUM_DATA_BYTES: u64 = 1 << 30;

/// 🎚️ Decoded audio: interleaved `f32` samples in `-1.0..1.0`.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedAudio {
    pub sample_rate: u32,
    pub channels: u16,
    pub samples: Vec<f32>,
}

impl DecodedAudio {
    /// 🔢️ Sample frames (one sample per channel each).
    pub fn frames(&self) -> usize {
        self.samples.len() / usize::from(self.channels.max(1))
    }

    /// ⏱️ Playing time in milliseconds, rounded half up.
    pub fn duration_milliseconds(&self) -> u64 {
        (self.frames() as u64 * 1000 + u64::from(self.sample_rate) / 2) / u64::from(self.sample_rate.max(1))
    }

    /// 🎯️ The frame at `milliseconds`, clamped to the frame count.
    pub fn frame_at_milliseconds(&self, milliseconds: u64) -> usize {
        ((milliseconds.saturating_mul(u64::from(self.sample_rate)) / 1000) as usize).min(self.frames())
    }

    /// 📖️ The interleaved samples of `count` frames from frame `start`, as many as remain.
    pub fn read(&self, start: usize, count: usize) -> &[f32] {
        let channels = usize::from(self.channels.max(1));
        let from = (start.min(self.frames())) * channels;
        let to = (start.saturating_add(count).min(self.frames())) * channels;
        &self.samples[from..to]
    }
}

fn alaw(byte: u8) -> i32 {
    let value = byte ^ 0x55;
    let segment = i32::from((value & 0x70) >> 4);
    let mut magnitude = i32::from(value & 0x0F) << 4;
    magnitude += if segment == 0 { 8 } else { 0x108 };
    if segment > 1 {
        magnitude <<= segment - 1;
    }
    if value & 0x80 != 0 { magnitude } else { -magnitude }
}

fn mulaw(byte: u8) -> i32 {
    let value = !byte;
    let magnitude = ((i32::from(value & 0x0F) << 3) + 0x84) << ((value & 0x70) >> 4);
    if value & 0x80 != 0 { 0x84 - magnitude } else { magnitude - 0x84 }
}

#[derive(Clone, Copy)]
enum Layout {
    Pcm(u16),
    Float(u16),
    ALaw,
    MuLaw,
}

struct Format {
    layout: Layout,
    channels: u16,
    sample_rate: u32,
    bytes_per_sample: usize,
}

fn le16(data: &[u8], at: usize) -> Result<u16, WavError> {
    Ok(u16::from_le_bytes(data.get(at..at + 2).ok_or(WavError::Truncated)?.try_into().map_err(|_| WavError::Truncated)?))
}

fn le32(data: &[u8], at: usize) -> Result<u32, WavError> {
    Ok(u32::from_le_bytes(data.get(at..at + 4).ok_or(WavError::Truncated)?.try_into().map_err(|_| WavError::Truncated)?))
}

fn parse_format(body: &[u8]) -> Result<Format, WavError> {
    let (mut tag, channels, sample_rate, block_align, bits) = (le16(body, 0)?, le16(body, 2)?, le32(body, 4)?, le16(body, 12)?, le16(body, 14)?);
    if tag == 0xFFFE {
        if le16(body, 16)? < 22 {
            return Err(WavError::Malformed { what: "extensible format extension" });
        }
        tag = le16(body, 24)?;
    }
    if channels == 0 || channels > 32 || sample_rate == 0 || bits == 0 || bits % 8 != 0 {
        return Err(WavError::Malformed { what: "format fields" });
    }
    let layout = match (tag, bits) {
        (1, 8 | 16 | 24 | 32) => Layout::Pcm(bits),
        (3, 32 | 64) => Layout::Float(bits),
        (6, 8) => Layout::ALaw,
        (7, 8) => Layout::MuLaw,
        _ => return Err(WavError::Unsupported { format: tag, bits }),
    };
    let bytes_per_sample = usize::from(bits / 8);
    if usize::from(block_align) != bytes_per_sample * usize::from(channels) {
        return Err(WavError::Malformed { what: "block alignment" });
    }
    Ok(Format { layout, channels, sample_rate, bytes_per_sample })
}

fn sample(layout: Layout, bytes: &[u8]) -> f32 {
    match layout {
        Layout::Pcm(8) => (f32::from(bytes[0]) - 128.0) / 128.0,
        Layout::Pcm(16) => f32::from(i16::from_le_bytes([bytes[0], bytes[1]])) / 32768.0,
        Layout::Pcm(24) => (i32::from_le_bytes([0, bytes[0], bytes[1], bytes[2]]) >> 8) as f32 / 8_388_608.0,
        Layout::Pcm(_) => (f64::from(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) / 2_147_483_648.0) as f32,
        Layout::Float(32) => f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        Layout::Float(_) => f64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]) as f32,
        Layout::ALaw => alaw(bytes[0]) as f32 / 32768.0,
        Layout::MuLaw => mulaw(bytes[0]) as f32 / 32768.0,
    }
}

/// 🔓️ Decodes a RIFF/WAVE file, reporting `(bytes decoded, bytes in all)` about every 64 Ki frames and stopping with
/// [`WavError::Cancelled`] as soon as `cancelled` answers true.
pub fn decode_wav(bytes: &[u8], progress: &mut dyn FnMut(u64, u64), cancelled: &dyn Fn() -> bool) -> Result<DecodedAudio, WavError> {
    if bytes.get(..4) != Some(b"RIFF") || bytes.get(8..12) != Some(b"WAVE") {
        return Err(WavError::NotWav);
    }
    let (mut format, mut data, mut at) = (None, None, 12usize);
    while at + 8 <= bytes.len() && data.is_none() {
        let (kind, size) = (&bytes[at..at + 4], le32(bytes, at + 4)? as usize);
        let body = at + 8;
        let end = body.saturating_add(size).min(bytes.len());
        match kind {
            b"fmt " => format = Some(parse_format(bytes.get(body..end).ok_or(WavError::Truncated)?)?),
            b"data" => data = Some(&bytes[body..end]),
            _ => {}
        }
        at = body.saturating_add(size).saturating_add(size & 1);
    }
    let format = format.ok_or(WavError::Malformed { what: "no fmt chunk before the data" })?;
    let data = data.ok_or(WavError::Malformed { what: "no data chunk" })?;
    if data.len() as u64 > WAV_MAXIMUM_DATA_BYTES {
        return Err(WavError::TooLarge { bytes: data.len() as u64 });
    }
    let frame_bytes = format.bytes_per_sample * usize::from(format.channels);
    let usable = data.len() / frame_bytes * frame_bytes;
    let mut samples = Vec::with_capacity(usable / format.bytes_per_sample);
    for (index, chunk) in data[..usable].chunks(frame_bytes * 65_536).enumerate() {
        if cancelled() {
            return Err(WavError::Cancelled);
        }
        samples.extend(chunk.chunks_exact(format.bytes_per_sample).map(|bytes| sample(format.layout, bytes)));
        progress(((index + 1) * frame_bytes * 65_536).min(usable) as u64, usable as u64);
    }
    Ok(DecodedAudio { sample_rate: format.sample_rate, channels: format.channels, samples })
}

/// 🔏️ Encodes `audio` as a 16-bit PCM WAV file (the inverse of [`decode_wav`] for every 16-bit sample value).
pub fn encode_wav_pcm16(audio: &DecodedAudio) -> Vec<u8> {
    let data_bytes = (audio.samples.len() * 2) as u32;
    let block_align = audio.channels * 2;
    let mut out = Vec::with_capacity(44 + data_bytes as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_bytes).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&audio.channels.to_le_bytes());
    out.extend_from_slice(&audio.sample_rate.to_le_bytes());
    out.extend_from_slice(&(audio.sample_rate * u32::from(block_align)).to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_bytes.to_le_bytes());
    for value in &audio.samples {
        out.extend_from_slice(&((value * 32768.0).round().clamp(-32768.0, 32767.0) as i16).to_le_bytes());
    }
    out
}

//! 🎞️ Native playback of the media the framework decodes itself: WAV audio and the MP4 its own AVC encoder writes. Every other format
//! stays with the browser's media element ([`NATIVE_MEDIA_BROWSER_ONLY`]); the player says so with a typed refusal instead of failing silently.

use super::transport::{MediaKeyCommand, MediaTransport, MediaTransportEvent};
use super::wav::{decode_wav, DecodedAudio, WavError};
use semio_framework_raster::video::decode::{decode_avc_mp4, DecodedVideo, VideoDecodeError};

/// 🌐️ What only a browser (or a platform decoder) plays today: no in-repo decoder exists for these. The operating system's speakers are
/// reached through [`super::audio_output::AudioOutputPort`].
pub const NATIVE_MEDIA_BROWSER_ONLY: &[&str] = &[
    "AAC, MP3, Opus, Vorbis, FLAC and ALAC audio",
    "H.264 beyond the I_PCM and P_Skip subset the framework encoder writes (transform, prediction, CABAC, B pictures)",
    "HEVC, VP8, VP9 and AV1 video",
    "WebM, Matroska, Ogg and fragmented MP4 containers",
];

/// 🔎️ The containers the native decoders read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeMediaFormat {
    Wav,
    Mp4,
}

/// 🔎️ The container of `bytes` by signature, `None` for everything the native decoders do not read.
pub fn sniff_native_media(bytes: &[u8]) -> Option<NativeMediaFormat> {
    if bytes.get(..4) == Some(b"RIFF") && bytes.get(8..12) == Some(b"WAVE") {
        Some(NativeMediaFormat::Wav)
    } else if bytes.get(4..8) == Some(b"ftyp") {
        Some(NativeMediaFormat::Mp4)
    } else {
        None
    }
}

/// 🎟️ Whether the native decoders may play a media type, and why not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeMediaSupport {
    Ready(NativeMediaFormat),
    Unsupported { reason: String },
}

/// 🎟️ The native capability for a media type: `audio/wav` (and its aliases) and `video/mp4` are decoded here.
pub fn native_media_support(media_type: &str) -> NativeMediaSupport {
    let essence = media_type.split(';').next().unwrap_or_default().trim().to_ascii_lowercase();
    match essence.as_str() {
        "audio/wav" | "audio/x-wav" | "audio/wave" | "audio/vnd.wave" => NativeMediaSupport::Ready(NativeMediaFormat::Wav),
        "video/mp4" => NativeMediaSupport::Ready(NativeMediaFormat::Mp4),
        _ => NativeMediaSupport::Unsupported { reason: format!("{essence} is decoded by the browser only") },
    }
}

/// 🚨️ Why a native load stopped short of a player.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeMediaFault {
    Unrecognised,
    Wav(WavError),
    Video(VideoDecodeError),
    Cancelled,
}

impl NativeMediaFault {
    /// 🔤️ The fault's stable code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Unrecognised => "unrecognised",
            Self::Wav(error) => error.code(),
            Self::Video(error) => error.code(),
            Self::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for NativeMediaFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unrecognised => formatter.write_str("media is neither WAV audio nor first-party MP4 video"),
            Self::Wav(error) => error.fmt(formatter),
            Self::Video(error) => error.fmt(formatter),
            Self::Cancelled => formatter.write_str("media loading was cancelled"),
        }
    }
}

impl std::error::Error for NativeMediaFault {}

/// 🎬️ Decoded media.
#[derive(Clone, Debug, PartialEq)]
pub enum NativeMedia {
    Audio(DecodedAudio),
    Video(DecodedVideo),
}

/// 📥️ Decodes `bytes` by signature, reporting `(units done, units in all)` (bytes for audio, samples for video) and stopping with
/// [`NativeMediaFault::Cancelled`] as soon as `cancelled` answers true.
pub fn load_native_media(bytes: &[u8], progress: &mut dyn FnMut(u64, u64), cancelled: &dyn Fn() -> bool) -> Result<NativeMedia, NativeMediaFault> {
    match sniff_native_media(bytes).ok_or(NativeMediaFault::Unrecognised)? {
        NativeMediaFormat::Wav => decode_wav(bytes, progress, cancelled).map(NativeMedia::Audio).map_err(|error| if error == WavError::Cancelled { NativeMediaFault::Cancelled } else { NativeMediaFault::Wav(error) }),
        NativeMediaFormat::Mp4 => decode_avc_mp4(bytes, &mut |done, total| progress(u64::from(done), u64::from(total)), cancelled).map(NativeMedia::Video).map_err(|error| if error == VideoDecodeError::Cancelled { NativeMediaFault::Cancelled } else { NativeMediaFault::Video(error) }),
    }
}

/// 🖼️ The picture a video shows now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeVideoFrame<'a> {
    pub index: usize,
    pub width: u32,
    pub height: u32,
    pub rgba: &'a [u8],
}

/// ▶️ A decoded medium under its transport: the clock advances the position of a video, the audio cursor drives the position of audio.
#[derive(Clone, Debug, PartialEq)]
pub struct NativeMediaPlayer {
    pub media: NativeMedia,
    pub transport: MediaTransport,
    audio_cursor: usize,
}

impl NativeMediaPlayer {
    /// 🏗️ A paused player at the start of `media`.
    pub fn new(media: NativeMedia) -> Self {
        let duration = match &media {
            NativeMedia::Audio(audio) => audio.duration_milliseconds(),
            NativeMedia::Video(video) => video.duration_milliseconds(),
        };
        Self { media, transport: MediaTransport::new(Some(duration)), audio_cursor: 0 }
    }

    fn sync_cursor(&mut self) {
        if let NativeMedia::Audio(audio) = &self.media {
            self.audio_cursor = audio.frame_at_milliseconds(self.transport.position_ms);
        }
    }

    /// 🎯️ Moves to `position_ms`.
    pub fn seek(&mut self, position_ms: u64) {
        self.transport.seek(position_ms);
        self.sync_cursor();
    }

    /// ⌨️ Applies a keyboard command.
    pub fn apply(&mut self, command: MediaKeyCommand) {
        self.transport.apply(command);
        self.sync_cursor();
    }

    /// ⏱️ Advances a video's clock by `delta_ms`; an audio player is advanced by [`Self::pull_audio`] instead.
    pub fn tick(&mut self, delta_ms: u64) -> MediaTransportEvent {
        match self.media {
            NativeMedia::Video(_) => self.transport.advance(delta_ms),
            NativeMedia::Audio(_) => MediaTransportEvent::None,
        }
    }

    /// 🔊️ Hands the next `frames` frames of audio to a sink and advances the position by exactly them; empty unless playing. Reaching
    /// the end pauses the transport.
    pub fn pull_audio(&mut self, frames: usize) -> (Vec<f32>, MediaTransportEvent) {
        let NativeMedia::Audio(audio) = &self.media else { return (Vec::new(), MediaTransportEvent::None) };
        if !self.transport.playing {
            return (Vec::new(), MediaTransportEvent::None);
        }
        let block = audio.read(self.audio_cursor, frames).to_vec();
        self.audio_cursor += block.len() / usize::from(audio.channels.max(1));
        self.transport.position_ms = (self.audio_cursor as u64 * 1000 + u64::from(audio.sample_rate) / 2) / u64::from(audio.sample_rate.max(1));
        let ended = self.audio_cursor >= audio.frames();
        if ended {
            self.transport.position_ms = audio.duration_milliseconds();
            self.transport.playing = false;
        }
        (block, if ended { MediaTransportEvent::Ended } else { MediaTransportEvent::None })
    }

    /// 🖼️ The video picture at the current position.
    pub fn video_frame(&self) -> Option<NativeVideoFrame<'_>> {
        let NativeMedia::Video(video) = &self.media else { return None };
        let index = video.frame_at_milliseconds(self.transport.position_ms);
        Some(NativeVideoFrame { index, width: video.width, height: video.height, rgba: video.frame_rgba(index)? })
    }
}

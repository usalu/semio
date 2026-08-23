//! 🔊️ Audio oracles: RIFF/PCM WAVE creation, mutation and projection.
//!
//! The `semantic-audio-v1` profile compares the format block (channels, sample rate, bit depth) and
//! the DECODED samples. Chunk padding, LIST/INFO metadata, chunk order and the total byte length are
//! writer choices.
//!
//! @see 📇️registry/🔣️component.json — the approved oracle registry these functions implement.

use crate::protocol::Json;

//#region 🔖️AudioSpec
/// 🔊️ Owned description of a PCM waveform — the one input both producers are given.
#[derive(Debug, Clone)]
pub struct AudioSpec {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub samples: Vec<i32>,
}

impl AudioSpec {
    /// 🔊️ A deterministic sawtooth of the requested length, in 16-bit mono at 8 kHz by default.
    pub fn sawtooth(channels: u16, sample_rate: u32, frames: usize) -> AudioSpec {
        let samples = (0..frames * channels as usize).map(|index| ((index % 256) as i32 * 128) - 16384).collect();
        AudioSpec { channels, sample_rate, bits_per_sample: 16, samples }
    }

    /// 🔊️ Reads a spec out of a scenario's owned JSON payload.
    pub fn from_json(value: &Json) -> AudioSpec {
        let number = |key: &str, fallback: f64| match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        };
        AudioSpec::sawtooth(number("channels", 1.0) as u16, number("sampleRate", 8000.0) as u32, number("frames", 64.0) as usize)
    }

    /// 🔁️ The projection every audio producer is compared through.
    pub fn projection(&self) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String("wav".to_string())),
            ("channels".to_string(), Json::Number(self.channels as f64)),
            ("sampleRate".to_string(), Json::Number(self.sample_rate as f64)),
            ("bitsPerSample".to_string(), Json::Number(self.bits_per_sample as f64)),
            ("frameCount".to_string(), Json::Number((self.samples.len() / self.channels.max(1) as usize) as f64)),
            ("samples".to_string(), Json::Array(self.samples.iter().map(|sample| Json::Number(*sample as f64)).collect())),
        ])
    }
}
//#endregion 🔖️AudioSpec

//#region 🔖️Wav
/// 🔮️ Creates a RIFF/PCM WAVE file with the registered `hound` reference implementation.
/// @see https://github.com/ruuda/hound
#[cfg(feature = "oracles")]
pub fn oracle_create_wav(spec: &AudioSpec) -> Result<Vec<u8>, String> {
    let header = hound::WavSpec { channels: spec.channels, sample_rate: spec.sample_rate, bits_per_sample: spec.bits_per_sample, sample_format: hound::SampleFormat::Int };
    let mut cursor = std::io::Cursor::new(Vec::new());
    {
        let mut writer = hound::WavWriter::new(&mut cursor, header).map_err(|error| format!("wav header: {}", error))?;
        for sample in &spec.samples {
            writer.write_sample(*sample as i16).map_err(|error| format!("wav sample: {}", error))?;
        }
        writer.finalize().map_err(|error| format!("wav finalize: {}", error))?;
    }
    Ok(cursor.into_inner())
}

/// 🔮️ Rewrites an existing WAVE at a new sample rate, keeping every sample — the reference
/// implementation of "change the declared rate without resampling".
#[cfg(feature = "oracles")]
pub fn oracle_retune_wav(input: &[u8], sample_rate: u32) -> Result<Vec<u8>, String> {
    let mut spec = read_wav(input)?;
    spec.sample_rate = sample_rate;
    oracle_create_wav(&spec)
}

/// 👁️ Projects WAVE bytes with the INDEPENDENT reader onto the owned `semantic-audio-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_wav(input: &[u8]) -> Result<Json, String> {
    Ok(read_wav(input)?.projection())
}

#[cfg(feature = "oracles")]
fn read_wav(input: &[u8]) -> Result<AudioSpec, String> {
    let mut reader = hound::WavReader::new(std::io::Cursor::new(input.to_vec())).map_err(|error| format!("independent reader could not parse the WAVE: {}", error))?;
    let header = reader.spec();
    let samples = reader.samples::<i16>().collect::<Result<Vec<i16>, _>>().map_err(|error| format!("independent reader could not decode WAVE samples: {}", error))?;
    Ok(AudioSpec { channels: header.channels, sample_rate: header.sample_rate, bits_per_sample: header.bits_per_sample, samples: samples.into_iter().map(i32::from).collect() })
}
//#endregion 🔖️Wav

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature nothing here is linked, and every entry point fails loudly.
#[cfg(not(feature = "oracles"))]
mod unavailable {
    use super::{AudioSpec, Json};
    const MESSAGE: &str = "the `oracles` feature is disabled — this host was not built with the registered reference implementations";

    pub fn create_wav(_spec: &AudioSpec) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn retune_wav(_input: &[u8], _sample_rate: u32) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_wav(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{create_wav as oracle_create_wav, project_wav, retune_wav as oracle_retune_wav};
//#endregion 🔖️Unavailable

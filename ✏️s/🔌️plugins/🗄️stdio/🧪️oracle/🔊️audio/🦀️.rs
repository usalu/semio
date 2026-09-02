//! 🔊️ Audio oracles: RIFF/PCM WAVE creation, mutation and projection.
//!
//! The `semantic-audio-v1` profile compares the format block (channels, sample rate, bit depth) and
//! the DECODED samples. Chunk padding, LIST/INFO metadata, chunk order and the total byte length are
//! writer choices.
//!
//! @see 📇️registry/🔣️.json — the approved oracle registry these functions implement.

use semio_repo_test_host::Json;

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

//#region 🔖️PcmWav
/// 📐️ Owned PCM format block shared by every WAVE oracle entry point.
#[derive(Debug, Clone, Copy)]
pub struct PcmWavFormat {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
}

/// 🌊️ Owned semantic WAVE model; auxiliary chunks stay opaque and ordered.
#[derive(Debug, Clone)]
pub struct PcmWav {
    pub format: PcmWavFormat,
    pub samples: Vec<i16>,
    pub other_chunks: Vec<(String, Vec<u8>)>,
}

fn le_u16(bytes: &[u8], offset: usize, label: &str) -> Result<u16, String> {
    let word = bytes.get(offset..offset + 2).ok_or_else(|| format!("wav: truncated {label}"))?;
    Ok(u16::from_le_bytes([word[0], word[1]]))
}

fn le_u32(bytes: &[u8], offset: usize, label: &str) -> Result<u32, String> {
    let word = bytes.get(offset..offset + 4).ok_or_else(|| format!("wav: truncated {label}"))?;
    Ok(u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
}

fn validate_format(format: PcmWavFormat) -> Result<u16, String> {
    if format.channels == 0 {
        return Err("wav: PCM requires at least one channel".to_string());
    }
    if format.sample_rate == 0 {
        return Err("wav: PCM requires a positive sample rate".to_string());
    }
    if format.bits_per_sample != 16 {
        return Err(format!("wav: owned oracle supports 16-bit PCM, found {} bits", format.bits_per_sample));
    }
    format.channels.checked_mul(2).ok_or_else(|| "wav: block alignment overflow".to_string())
}

/// 📥️ Decodes RIFF/WAVE PCM16 without sharing the subject codec.
pub fn decode_pcm16_wav(input: &[u8]) -> Result<PcmWav, String> {
    if input.len() < 12 || &input[0..4] != b"RIFF" || &input[8..12] != b"WAVE" {
        return Err("wav: missing RIFF/WAVE magic".to_string());
    }
    let declared_len = le_u32(input, 4, "RIFF size")? as usize + 8;
    if declared_len != input.len() {
        return Err(format!("wav: RIFF size declares {declared_len} byte(s), found {}", input.len()));
    }
    let mut position = 12usize;
    let mut format = None;
    let mut data = None;
    let mut other_chunks = Vec::new();
    while position < input.len() {
        let header = input.get(position..position + 8).ok_or_else(|| "wav: truncated chunk header".to_string())?;
        let size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;
        let body_start = position + 8;
        let body_end = body_start.checked_add(size).ok_or_else(|| "wav: chunk size overflow".to_string())?;
        let body = input.get(body_start..body_end).ok_or_else(|| format!("wav: chunk {:?} overruns file", String::from_utf8_lossy(&header[0..4])))?;
        match &header[0..4] {
            b"fmt " => {
                if format.is_some() {
                    return Err("wav: duplicate fmt chunk".to_string());
                }
                if body.len() < 16 {
                    return Err("wav: fmt chunk is shorter than 16 bytes".to_string());
                }
                let audio_format = le_u16(body, 0, "audio format")?;
                if audio_format != 1 {
                    return Err(format!("wav: owned oracle supports PCM format 1, found {audio_format}"));
                }
                let parsed = PcmWavFormat { channels: le_u16(body, 2, "channels")?, sample_rate: le_u32(body, 4, "sample rate")?, bits_per_sample: le_u16(body, 14, "bits per sample")? };
                let block_align = validate_format(parsed)?;
                let declared_byte_rate = le_u32(body, 8, "byte rate")?;
                let expected_byte_rate = parsed.sample_rate.checked_mul(u32::from(block_align)).ok_or_else(|| "wav: byte rate overflow".to_string())?;
                if declared_byte_rate != expected_byte_rate {
                    return Err(format!("wav: byte rate is {declared_byte_rate}, expected {expected_byte_rate}"));
                }
                let declared_block_align = le_u16(body, 12, "block alignment")?;
                if declared_block_align != block_align {
                    return Err(format!("wav: block alignment is {declared_block_align}, expected {block_align}"));
                }
                format = Some(parsed);
            }
            b"data" => {
                if data.replace(body.to_vec()).is_some() {
                    return Err("wav: duplicate data chunk".to_string());
                }
            }
            fourcc => other_chunks.push((String::from_utf8_lossy(fourcc).into_owned(), body.to_vec())),
        }
        position = body_end + (size % 2);
        if position > input.len() {
            return Err("wav: missing odd-chunk padding byte".to_string());
        }
    }
    let format = format.ok_or_else(|| "wav: missing fmt chunk".to_string())?;
    let data = data.ok_or_else(|| "wav: missing data chunk".to_string())?;
    let block_align = validate_format(format)? as usize;
    if data.len() % block_align != 0 {
        return Err(format!("wav: {} PCM byte(s) do not fill a {block_align}-byte frame", data.len()));
    }
    let samples = data.chunks_exact(2).map(|word| i16::from_le_bytes([word[0], word[1]])).collect();
    Ok(PcmWav { format, samples, other_chunks })
}

/// 📤️ Encodes the owned PCM16 model into one canonical RIFF/WAVE byte stream.
pub fn encode_pcm16_wav(wav: &PcmWav) -> Result<Vec<u8>, String> {
    let block_align = validate_format(wav.format)?;
    if wav.samples.len() % wav.format.channels as usize != 0 {
        return Err(format!("wav: {} sample(s) do not fill {} channel(s)", wav.samples.len(), wav.format.channels));
    }
    let data_len = wav.samples.len().checked_mul(2).ok_or_else(|| "wav: sample byte length overflow".to_string())?;
    let auxiliary_len = wav.other_chunks.iter().try_fold(0usize, |total, (fourcc, body)| {
        if fourcc.as_bytes().len() != 4 {
            return Err(format!("wav: chunk id {fourcc:?} is not four bytes"));
        }
        total.checked_add(8 + body.len() + body.len() % 2).ok_or_else(|| "wav: auxiliary chunk length overflow".to_string())
    })?;
    let total_len = 44usize.checked_add(data_len).and_then(|size| size.checked_add(auxiliary_len)).ok_or_else(|| "wav: file length overflow".to_string())?;
    let riff_size = u32::try_from(total_len - 8).map_err(|_| "wav: file is larger than RIFF permits".to_string())?;
    let data_size = u32::try_from(data_len).map_err(|_| "wav: sample data is larger than RIFF permits".to_string())?;
    let byte_rate = wav.format.sample_rate.checked_mul(u32::from(block_align)).ok_or_else(|| "wav: byte rate overflow".to_string())?;
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&riff_size.to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&wav.format.channels.to_le_bytes());
    out.extend_from_slice(&wav.format.sample_rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&wav.format.bits_per_sample.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_size.to_le_bytes());
    for sample in &wav.samples {
        out.extend_from_slice(&sample.to_le_bytes());
    }
    for (fourcc, body) in &wav.other_chunks {
        out.extend_from_slice(fourcc.as_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(body);
        if body.len() % 2 == 1 {
            out.push(0);
        }
    }
    Ok(out)
}

/// 🔮️ Creates a canonical RIFF/PCM WAVE file through the owned oracle boundary.
#[cfg(feature = "oracles")]
pub fn oracle_create_wav(spec: &AudioSpec) -> Result<Vec<u8>, String> {
    encode_pcm16_wav(&PcmWav { format: PcmWavFormat { channels: spec.channels, sample_rate: spec.sample_rate, bits_per_sample: spec.bits_per_sample }, samples: spec.samples.iter().map(|sample| *sample as i16).collect(), other_chunks: Vec::new() })
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
    let wav = decode_pcm16_wav(input)?;
    Ok(AudioSpec { channels: wav.format.channels, sample_rate: wav.format.sample_rate, bits_per_sample: wav.format.bits_per_sample, samples: wav.samples.into_iter().map(i32::from).collect() })
}
//#endregion 🔖️PcmWav

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::{decode_pcm16_wav, encode_pcm16_wav, PcmWav, PcmWavFormat};

    fn hex(value: &str) -> Vec<u8> {
        value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("ascii hex"), 16).expect("hex byte")).collect()
    }

    #[test]
    fn owned_pcm16_codec_matches_the_frozen_hound_golden() {
        let wav = PcmWav { format: PcmWavFormat { channels: 1, sample_rate: 8000, bits_per_sample: 16 }, samples: vec![-32768, -1, 0, 1, 32767], other_chunks: Vec::new() };
        let expected = hex("524946462e00000057415645666d74201000000001000100401f0000803e000002001000646174610a0000000080ffff00000100ff7f");
        let encoded = encode_pcm16_wav(&wav).expect("encode hostile lanes");
        assert_eq!(encoded, expected);
        let decoded = decode_pcm16_wav(&encoded).expect("decode hostile lanes");
        assert_eq!(decoded.format.channels, 1);
        assert_eq!(decoded.format.sample_rate, 8000);
        assert_eq!(decoded.format.bits_per_sample, 16);
        assert_eq!(decoded.samples, wav.samples);
    }

    #[test]
    fn owned_pcm16_codec_preserves_ordered_odd_auxiliary_chunks() {
        let wav = PcmWav { format: PcmWavFormat { channels: 2, sample_rate: 44_100, bits_per_sample: 16 }, samples: vec![-32768, 32767, -1, 1], other_chunks: vec![("JUNK".to_string(), vec![0, 127, 255]), ("fact".to_string(), vec![2, 0, 0, 0])] };
        let decoded = decode_pcm16_wav(&encode_pcm16_wav(&wav).expect("encode auxiliary chunks")).expect("decode auxiliary chunks");
        assert_eq!(decoded.samples, wav.samples);
        assert_eq!(decoded.other_chunks, wav.other_chunks);
    }

    #[test]
    fn owned_pcm16_decoder_rejects_hostile_framing_and_format_fields() {
        let hostile = [
            "000000000000000000000000",
            "524946461000000057415645666d74200400000001000100",
            "524946462400000057415645666d74201000000003000100401f0000803e0000020010006461746100000000",
            "524946462400000057415645666d74201000000001000000401f000000000000000010006461746100000000",
            "524946462400000057415645666d74201000000001000100401f000001000000020010006461746100000000",
            "524946462400000057415645666d74201000000001000100401f0000803e0000040010006461746100000000",
            "524946462500000057415645666d74201000000001000100401f0000803e000002001000646174610100000000",
            "524946462400000057415645666d74201000000001000100401f0000803e00000200100064617461040000000000",
        ];
        for bytes in hostile {
            assert!(decode_pcm16_wav(&hex(bytes)).is_err(), "accepted hostile WAVE {bytes}");
        }
    }

    #[test]
    fn owned_pcm16_encoder_rejects_incomplete_frames_and_invalid_chunk_ids() {
        let format = PcmWavFormat { channels: 2, sample_rate: 44_100, bits_per_sample: 16 };
        assert!(encode_pcm16_wav(&PcmWav { format, samples: vec![1], other_chunks: Vec::new() }).is_err());
        assert!(encode_pcm16_wav(&PcmWav { format, samples: vec![1, 2], other_chunks: vec![("wide!".to_string(), Vec::new())] }).is_err());
    }
}
//#endregion 🧪️Tests

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

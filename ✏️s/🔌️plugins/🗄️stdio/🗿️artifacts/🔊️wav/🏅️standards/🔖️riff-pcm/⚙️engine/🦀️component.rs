//! ⚙️ Wav (riff-pcm) engine — 🚧 scaffolded by W1b: a REAL RIFF/WAVE chunk walker + typed
//! `fmt ` chunk decode + magic sniff. `data` chunk bytes are retained typed-raw (PCM sample
//! interpretation lands in W3).

#[derive(Clone, Debug, PartialEq)]
pub struct WavFmtSummary { pub audio_format: u16, pub channels: u16, pub sample_rate: u32, pub bits_per_sample: u16 }

pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE"
}

pub fn parse_fmt_chunk(bytes: &[u8]) -> Result<WavFmtSummary, String> {
    if !sniff_real_bytes(bytes) { return Err("wav: missing RIFF/WAVE magic".into()); }
    let mut pos = 12usize;
    while pos + 8 <= bytes.len() {
        let fourcc = &bytes[pos..pos + 4];
        let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().map_err(|_| "wav: bad chunk size".to_string())?) as usize;
        let body_start = pos + 8;
        if fourcc == b"fmt " {
            if body_start + 16 > bytes.len() { return Err("wav: truncated fmt chunk".into()); }
            let b = &bytes[body_start..body_start + 16];
            return Ok(WavFmtSummary {
                audio_format: u16::from_le_bytes([b[0], b[1]]),
                channels: u16::from_le_bytes([b[2], b[3]]),
                sample_rate: u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
                bits_per_sample: u16::from_le_bytes([b[14], b[15]]),
            });
        }
        pos = body_start + size + (size % 2);
    }
    Err("wav: no fmt chunk found".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_wav() -> Vec<u8> {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend_from_slice(&36u32.to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&8000u32.to_le_bytes());
        bytes.extend_from_slice(&16000u32.to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&16u16.to_le_bytes());
        bytes
    }

    #[test]
    fn sniffs_and_parses_a_synthetic_fmt_chunk() {
        let bytes = synthetic_wav();
        assert!(sniff_real_bytes(&bytes));
        let fmt = parse_fmt_chunk(&bytes).expect("fmt");
        assert_eq!(fmt.audio_format, 1);
        assert_eq!(fmt.channels, 1);
        assert_eq!(fmt.sample_rate, 8000);
        assert_eq!(fmt.bits_per_sample, 16);
    }

    #[test]
    fn sniff_rejects_non_wave_riff() {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend_from_slice(&4u32.to_le_bytes());
        bytes.extend_from_slice(b"AVI ");
        assert!(!sniff_real_bytes(&bytes));
    }
}

//#region 🔖️Register
/// 📌️ Registers this standard's single (✳️any) subset. Real magic-byte `sniff_real_bytes`/
/// `parse_minimal` above are used by the subset's analyzer; schema descriptor + document codec
/// registration is the subset composer's own job (see that module).
pub fn register() {
    crate::artifacts::wav::standards::riff_pcm::subsets::any::composer::register();
}
//#endregion 🔖️Register

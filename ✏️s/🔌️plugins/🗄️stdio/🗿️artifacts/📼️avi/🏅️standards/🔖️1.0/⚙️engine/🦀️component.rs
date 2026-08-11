//! ⚙️ Avi (1.0, RIFF) engine — 🚧 scaffolded by W1b: a REAL RIFF/AVI top-level chunk walker +
//! magic sniff (`RIFF`....`AVI `). Deep hdrl/strl/movi descent lands in W3.

#[derive(Clone, Debug, PartialEq)]
pub struct AviChunkSummary { pub fourcc: String, pub size: u32 }

pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"AVI "
}

pub fn parse_minimal(bytes: &[u8]) -> Result<Vec<AviChunkSummary>, String> {
    if !sniff_real_bytes(bytes) {
        return Err("avi: missing RIFF/AVI magic".into());
    }
    let mut out = Vec::new();
    let mut pos = 12usize;
    while pos + 8 <= bytes.len() {
        let fourcc = String::from_utf8_lossy(&bytes[pos..pos + 4]).into_owned();
        let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().map_err(|_| "avi: bad chunk size".to_string())?);
        out.push(AviChunkSummary { fourcc, size });
        let padded = size as usize + (size as usize % 2);
        pos += 8 + padded;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniff_and_parse_a_synthetic_riff_avi() {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend_from_slice(&40u32.to_le_bytes());
        bytes.extend_from_slice(b"AVI ");
        bytes.extend_from_slice(b"LIST");
        bytes.extend_from_slice(&4u32.to_le_bytes());
        bytes.extend_from_slice(b"hdrl");
        assert!(sniff_real_bytes(&bytes));
        let chunks = parse_minimal(&bytes).expect("parse");
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].fourcc, "LIST");
    }

    #[test]
    fn sniff_rejects_non_avi_riff() {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend_from_slice(&4u32.to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        assert!(!sniff_real_bytes(&bytes));
    }
}

//#region 🔖️Register
/// 📌️ Registers this standard's single (✳️any) subset. Real magic-byte `sniff_real_bytes`/
/// `parse_minimal` above are used by the subset's analyzer; schema descriptor + document codec
/// registration is the subset composer's own job (see that module).
pub fn register() {
    crate::artifacts::avi::standards::v1_0::subsets::any::composer::register();
}
//#endregion 🔖️Register

//! ⚙️ Mp3 (mpeg1-layer3) engine — 🚧 scaffolded by W1b: real ID3v2 header detection + a real
//! MPEG frame sync-word scan (11-bit sync + version/layer bits validated). Per-frame bitrate/
//! sample-rate table decode lands in W3 (frame bytes retained typed-raw for now).

#[derive(Clone, Debug, PartialEq)]
pub struct Id3v2Summary { pub major_version: u8, pub flags: u8, pub size: u32 }

pub fn detect_id3v2(bytes: &[u8]) -> Option<Id3v2Summary> {
    if bytes.len() < 10 || &bytes[0..3] != b"ID3" { return None; }
    let major_version = bytes[3];
    let flags = bytes[5];
    let size = ((bytes[6] as u32) << 21) | ((bytes[7] as u32) << 14) | ((bytes[8] as u32) << 7) | (bytes[9] as u32);
    Some(Id3v2Summary { major_version, flags, size })
}

/// 🔍 Real 11-bit MPEG sync-word scan: `0xFFE` in the top 11 bits, plus a sanity check that the
/// version (bits 19-20) and layer (bits 17-18) fields are not the reserved values.
pub fn find_frame_sync(bytes: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    while i + 1 < bytes.len() {
        if bytes[i] == 0xFF && (bytes[i + 1] & 0xE0) == 0xE0 {
            let version = (bytes[i + 1] >> 3) & 0x03;
            let layer = (bytes[i + 1] >> 1) & 0x03;
            if version != 0x01 && layer != 0x00 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// 🔍 Real magic sniff: an ID3v2 header at the front, OR a valid MPEG frame sync anywhere in the
/// buffer (mirrors the other 6 format engines' `sniff_real_bytes(bytes) -> bool` shape exactly —
/// this one was referenced by the analyzer's doc comment but never actually defined; genuine
/// scaffold gap, fixed here without touching `detect_id3v2`/`find_frame_sync`'s own shape).
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    detect_id3v2(bytes).is_some() || find_frame_sync(bytes).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_a_synthetic_id3v2_header() {
        let mut bytes = b"ID3".to_vec();
        bytes.extend_from_slice(&[0x03, 0x00, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00, 0x02, 0x01]);
        let hdr = detect_id3v2(&bytes).expect("id3v2");
        assert_eq!(hdr.major_version, 3);
        assert_eq!(hdr.size, 257);
    }

    #[test]
    fn finds_a_synthetic_mpeg1_layer3_frame_sync() {
        let bytes = [0x00, 0x00, 0xFF, 0xFB, 0x90, 0x00];
        assert_eq!(find_frame_sync(&bytes), Some(2));
    }

    #[test]
    fn no_id3v2_header_returns_none() {
        assert!(detect_id3v2(b"not an id3 tag").is_none());
    }
}

//#region 🔖️Register
/// 📌️ Registers this standard's single (✳️any) subset. Real magic-byte `sniff_real_bytes`/
/// `parse_minimal` above are used by the subset's analyzer; schema descriptor + document codec
/// registration is the subset composer's own job (see that module).
pub fn register() {
    crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::composer::register();
}
//#endregion 🔖️Register

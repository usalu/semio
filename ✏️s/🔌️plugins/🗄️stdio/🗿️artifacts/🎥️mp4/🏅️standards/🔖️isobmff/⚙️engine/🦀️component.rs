//! ⚙️ Mp4 (isobmff) engine — 🚧 scaffolded by W1b: a REAL minimal ISO-BMFF top-level box walker
//! (size+type framing, `ftyp` typed, everything else typed-raw) + a real magic sniff. Full
//! box-tree descent (moov/trak/...) lands in W3. Moved wholesale from remodel's video engine per
//! the master plan's extraction map (5,163 LOC) starting in W3 — this file is the fresh seed.

#[derive(Clone, Debug, PartialEq)]
pub struct Mp4BoxSummary { pub box_type: String, pub size: u64 }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mp4Summary {
    pub major_brand: Option<String>,
    pub minor_version: u32,
    pub boxes: Vec<Mp4BoxSummary>,
}

/// 🔍 True when `bytes` starts with a plausible ISO-BMFF top-level box header whose 4-byte type
/// is ASCII (real structural check, not a fixed byte-string match).
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    if bytes.len() < 8 { return false; }
    let size = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let box_type = &bytes[4..8];
    let ascii_type = box_type.iter().all(|&b| b.is_ascii_alphanumeric() || b == b' ');
    ascii_type && (size as usize >= 8 || size == 0 || size == 1)
}

/// 📦 Walks top-level boxes, typing `ftyp`; everything else is a type+size summary only (no
/// payload retention here — this is a structural sniff/summary pass, not the snapshot decoder,
/// which stays JSON-passthrough in W1b per the subset's own module doc comment).
pub fn parse_minimal(bytes: &[u8]) -> Result<Mp4Summary, String> {
    let mut summary = Mp4Summary::default();
    let mut pos = 0usize;
    while pos + 8 <= bytes.len() {
        let size32 = u32::from_be_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]]) as u64;
        let box_type = String::from_utf8_lossy(&bytes[pos + 4..pos + 8]).into_owned();
        let (header_len, box_size) = if size32 == 1 {
            if pos + 16 > bytes.len() { return Err("mp4: truncated 64-bit box size".into()); }
            let big = u64::from_be_bytes(bytes[pos + 8..pos + 16].try_into().map_err(|_| "mp4: bad 64-bit size".to_string())?);
            (16usize, big)
        } else if size32 == 0 {
            (8usize, (bytes.len() - pos) as u64)
        } else {
            (8usize, size32)
        };
        if box_size < header_len as u64 || pos as u64 + box_size > bytes.len() as u64 {
            return Err(format!("mp4: box {box_type:?} has an out-of-range size"));
        }
        if box_type == "ftyp" && summary.major_brand.is_none() {
            let body_start = pos + header_len;
            if body_start + 8 <= bytes.len() {
                summary.major_brand = Some(String::from_utf8_lossy(&bytes[body_start..body_start + 4]).into_owned());
                summary.minor_version = u32::from_be_bytes(bytes[body_start + 4..body_start + 8].try_into().unwrap());
            }
        }
        summary.boxes.push(Mp4BoxSummary { box_type, size: box_size });
        pos += box_size as usize;
    }
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn box_bytes(box_type: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&((8 + body.len()) as u32).to_be_bytes());
        out.extend_from_slice(box_type);
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn sniff_and_parse_a_synthetic_ftyp_plus_free_box() {
        let mut body = b"isom".to_vec();
        body.extend_from_slice(&0u32.to_be_bytes());
        body.extend_from_slice(b"isomiso2");
        let ftyp = box_bytes(b"ftyp", &body);
        let free = box_bytes(b"free", &[0, 0, 0, 0]);
        let mut bytes = ftyp.clone();
        bytes.extend_from_slice(&free);
        assert!(sniff_real_bytes(&bytes));
        let summary = parse_minimal(&bytes).expect("parse");
        assert_eq!(summary.major_brand.as_deref(), Some("isom"));
        assert_eq!(summary.boxes.len(), 2);
        assert_eq!(summary.boxes[0].box_type, "ftyp");
        assert_eq!(summary.boxes[1].box_type, "free");
    }

    #[test]
    fn sniff_rejects_non_ascii_box_type() {
        let bytes = [0u8, 0, 0, 8, 0xff, 0xfe, 0xfd, 0xfc];
        assert!(!sniff_real_bytes(&bytes));
    }
}

//#region 🔖️Register
/// 📌️ Registers this standard's single (✳️any) subset. Real magic-byte `sniff_real_bytes`/
/// `parse_minimal` above are used by the subset's analyzer; schema descriptor + document codec
/// registration is the subset composer's own job (see that module).
pub fn register() {
    crate::artifacts::mp4::standards::isobmff::subsets::any::composer::register();
}
//#endregion 🔖️Register

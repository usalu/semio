//! 🔡️ Base64 codec of the foundation crate: Gateway vocabulary: errors, handles, audit lane, reports, local session files and delegated credentials.

//#region 🔖️Base64

/// 🔤️ The byte fields this codec carries (`ops` lanes, an artifact's pack/spr, exported media) are
/// opaque binary. JSON has no byte type, so they travel as standard base64 — 1.33× rather than the
/// ~4× a number array costs against a bridge outbox whose per-frame credit is fixed.
pub const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(BASE64_ALPHABET[(b0 >> 2) as usize] as char);
        out.push(BASE64_ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 { BASE64_ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { BASE64_ALPHABET[(b2 & 0x3f) as usize] as char } else { '=' });
    }
    out
}

//#endregion 🔖️Base64

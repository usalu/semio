//! 💾️ resize-page native binary payload owner.

use super::super::{
    binary::{put_index, Reader},
    PdfMutation,
};
use super::ResizePage;

//#region 🔖️Codec
pub const TAG: u8 = 3;

pub fn encode(mutation: &PdfMutation) -> Option<Result<Vec<u8>, String>> {
    let PdfMutation::ResizePage(payload) = mutation else {
        return None;
    };
    Some(encode_payload(payload))
}

fn encode_payload(payload: &ResizePage) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    put_index(payload.index, &mut bytes)?;
    if !payload.width.is_finite() {
        return Err("Non-finite geometry".into());
    }
    bytes.extend_from_slice(&payload.width.to_le_bytes());
    if !payload.height.is_finite() {
        return Err("Non-finite geometry".into());
    }
    bytes.extend_from_slice(&payload.height.to_le_bytes());
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<PdfMutation, String> {
    let mut reader = Reader::new(bytes);
    let payload = ResizePage { index: reader.index()?, width: reader.number()?, height: reader.number()? };
    reader.finish()?;
    Ok(PdfMutation::ResizePage(payload))
}
//#endregion 🔖️Codec

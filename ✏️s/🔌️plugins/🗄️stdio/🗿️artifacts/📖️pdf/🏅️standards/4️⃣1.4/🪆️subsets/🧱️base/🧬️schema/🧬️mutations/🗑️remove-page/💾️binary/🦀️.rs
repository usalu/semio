//! 💾️ remove-page native binary payload owner.

use super::super::{
    binary::{put_index, Reader},
    PdfMutation,
};
use super::RemovePage;

//#region 🔖️Codec
pub const TAG: u8 = 1;

pub fn encode(mutation: &PdfMutation) -> Option<Result<Vec<u8>, String>> {
    let PdfMutation::RemovePage(payload) = mutation else {
        return None;
    };
    Some(encode_payload(payload))
}

fn encode_payload(payload: &RemovePage) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    put_index(payload.index, &mut bytes)?;
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<PdfMutation, String> {
    let mut reader = Reader::new(bytes);
    let payload = RemovePage { index: reader.index()? };
    reader.finish()?;
    Ok(PdfMutation::RemovePage(payload))
}
//#endregion 🔖️Codec

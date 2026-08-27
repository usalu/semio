//! 💾️ move-page native binary payload owner.

use super::super::{
    binary::{put_index, Reader},
    PdfMutation,
};
use super::MovePage;

//#region 🔖️Codec
pub const TAG: u8 = 2;

pub fn encode(mutation: &PdfMutation) -> Option<Result<Vec<u8>, String>> {
    let PdfMutation::MovePage(payload) = mutation else {
        return None;
    };
    Some(encode_payload(payload))
}

fn encode_payload(payload: &MovePage) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    put_index(payload.from, &mut bytes)?;
    put_index(payload.to, &mut bytes)?;
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<PdfMutation, String> {
    let mut reader = Reader::new(bytes);
    let payload = MovePage { from: reader.index()?, to: reader.index()? };
    reader.finish()?;
    Ok(PdfMutation::MovePage(payload))
}
//#endregion 🔖️Codec

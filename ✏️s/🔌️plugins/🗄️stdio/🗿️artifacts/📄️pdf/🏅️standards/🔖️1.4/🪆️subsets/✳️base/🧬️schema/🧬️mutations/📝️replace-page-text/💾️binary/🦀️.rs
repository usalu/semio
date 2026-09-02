//! 💾️ replace-page-text native binary payload owner.

use super::super::{
    binary::{put_index, put_text, Reader},
    PdfMutation,
};
use super::ReplacePageText;

//#region 🔖️Codec
pub const TAG: u8 = 4;

pub fn encode(mutation: &PdfMutation) -> Option<Result<Vec<u8>, String>> {
    let PdfMutation::ReplacePageText(payload) = mutation else {
        return None;
    };
    Some(encode_payload(payload))
}

fn encode_payload(payload: &ReplacePageText) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    put_index(payload.index, &mut bytes)?;
    put_text(&payload.text, &mut bytes)?;
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<PdfMutation, String> {
    let mut reader = Reader::new(bytes);
    let payload = ReplacePageText { index: reader.index()?, text: reader.text()? };
    reader.finish()?;
    Ok(PdfMutation::ReplacePageText(payload))
}
//#endregion 🔖️Codec

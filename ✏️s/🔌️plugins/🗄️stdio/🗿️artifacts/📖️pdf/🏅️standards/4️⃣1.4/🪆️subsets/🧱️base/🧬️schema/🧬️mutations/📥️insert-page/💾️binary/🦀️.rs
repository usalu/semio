//! 💾️ insert-page native binary payload owner.

use super::super::{
    binary::{put_index, put_text, Reader},
    PdfMutation,
};
use super::InsertPage;

//#region 🔖️Codec
pub const TAG: u8 = 0;

pub fn encode(mutation: &PdfMutation) -> Option<Result<Vec<u8>, String>> {
    let PdfMutation::InsertPage(payload) = mutation else {
        return None;
    };
    Some(encode_payload(payload))
}

fn encode_payload(payload: &InsertPage) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    put_index(payload.index, &mut bytes)?;
    if !payload.page.width.is_finite() || !payload.page.height.is_finite() {
        return Err("Non-finite geometry".into());
    }
    bytes.extend_from_slice(&payload.page.width.to_le_bytes());
    bytes.extend_from_slice(&payload.page.height.to_le_bytes());
    put_text(&payload.page.text, &mut bytes)?;
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<PdfMutation, String> {
    let mut reader = Reader::new(bytes);
    let payload = InsertPage { index: reader.index()?, page: crate::artifacts::pdf::standards::v1_4::subsets::base::schema::snapshot::PageDoc { width: reader.number()?, height: reader.number()?, text: reader.text()? } };
    reader.finish()?;
    Ok(PdfMutation::InsertPage(payload))
}
//#endregion 🔖️Codec

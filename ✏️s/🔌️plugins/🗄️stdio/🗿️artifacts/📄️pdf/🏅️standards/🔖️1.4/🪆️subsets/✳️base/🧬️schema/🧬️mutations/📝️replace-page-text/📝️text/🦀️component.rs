//! 📝️ replace-page-text text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::ReplacePageText;

//#region 🔖️Codec
pub const OPCODE: &str = "replace-page-text";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::ReplacePageText(payload) = mutation else {
        return None;
    };
    Some(hex(&serde_json::to_vec(payload).expect("A direct payload serializes")))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    serde_json::from_slice::<ReplacePageText>(&unhex(payload)?).map(PdfMutation::ReplacePageText).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

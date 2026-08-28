//! 📝️ remove-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::RemovePage;

//#region 🔖️Codec
pub const OPCODE: &str = "remove-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::RemovePage(payload) = mutation else {
        return None;
    };
    Some(hex(&serde_json::to_vec(payload).expect("A direct payload serializes")))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    serde_json::from_slice::<RemovePage>(&unhex(payload)?).map(PdfMutation::RemovePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//! 📝️ move-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::MovePage;

//#region 🔖️Codec
pub const OPCODE: &str = "move-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::MovePage(payload) = mutation else {
        return None;
    };
    Some(hex(&serde_json::to_vec(payload).expect("A direct payload serializes")))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    serde_json::from_slice::<MovePage>(&unhex(payload)?).map(PdfMutation::MovePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

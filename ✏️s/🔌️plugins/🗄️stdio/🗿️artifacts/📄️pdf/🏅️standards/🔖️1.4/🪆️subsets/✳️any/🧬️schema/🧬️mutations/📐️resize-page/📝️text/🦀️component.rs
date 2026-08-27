//! 📝️ resize-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::ResizePage;

//#region 🔖️Codec
pub const OPCODE: &str = "resize-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::ResizePage(payload) = mutation else {
        return None;
    };
    Some(hex(&serde_json::to_vec(payload).expect("A direct payload serializes")))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    serde_json::from_slice::<ResizePage>(&unhex(payload)?).map(PdfMutation::ResizePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

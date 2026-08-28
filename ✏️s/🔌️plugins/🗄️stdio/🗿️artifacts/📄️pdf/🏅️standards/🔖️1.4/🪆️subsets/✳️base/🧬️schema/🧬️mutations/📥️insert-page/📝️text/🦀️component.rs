//! 📝️ insert-page text payload owner.

use super::super::{
    text::{hex, unhex},
    PdfMutation,
};
use super::InsertPage;

//#region 🔖️Codec
pub const OPCODE: &str = "insert-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::InsertPage(payload) = mutation else {
        return None;
    };
    Some(hex(&serde_json::to_vec(payload).expect("A direct payload serializes")))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    serde_json::from_slice::<InsertPage>(&unhex(payload)?).map(PdfMutation::InsertPage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

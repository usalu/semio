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
    Some(hex(&pack::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    let bytes = unhex(payload)?;
    let parsed = pack::parse_json_bytes(&bytes).map_err(|error| error.to_string())?;
    <MovePage as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map(PdfMutation::MovePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

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
    Some(hex(&pack::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, String> {
    let bytes = unhex(payload)?;
    let parsed = pack::parse_json_bytes(&bytes).map_err(|error| error.to_string())?;
    <RemovePage as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map(PdfMutation::RemovePage).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

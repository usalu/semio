//! 🧽️ Direct text codec for `remove-trim-box`.

use super::RemoveTrimBox;

//#region 🔖️Identity
pub const OPCODE: &str = "remove-trim-box";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &RemoveTrimBox) -> Result<String, String> {
    Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(payload))))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<RemoveTrimBox, String> {
    let parsed = pack::parse_json(text).map_err(|error| error.to_string())?;
    <RemoveTrimBox as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = RemoveTrimBox { page_index: 0 };
        assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests

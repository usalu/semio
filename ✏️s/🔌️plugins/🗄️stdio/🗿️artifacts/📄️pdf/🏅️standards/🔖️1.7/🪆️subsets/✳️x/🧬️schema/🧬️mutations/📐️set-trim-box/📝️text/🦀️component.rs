//! 📐️ Direct text codec for `set-trim-box`.

use super::SetTrimBox;

//#region 🔖️Identity
pub const OPCODE: &str = "set-trim-box";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &SetTrimBox) -> Result<String, String> {
    Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(payload))))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<SetTrimBox, String> {
    let parsed = pack::parse_json(text).map_err(|error| error.to_string())?;
    <SetTrimBox as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = SetTrimBox { page_index: 0, trim_box: [0.0, 0.0, 100.0, 100.0] };
        assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests

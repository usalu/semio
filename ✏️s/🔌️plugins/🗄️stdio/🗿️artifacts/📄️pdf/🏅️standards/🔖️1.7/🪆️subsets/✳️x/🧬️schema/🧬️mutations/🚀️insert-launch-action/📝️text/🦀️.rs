//! 🚀️ Direct text codec for `insert-launch-action`.

use super::InsertLaunchAction;

//#region 🔖️Identity
pub const OPCODE: &str = "insert-launch-action";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &InsertLaunchAction) -> Result<String, String> {
    Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(payload))))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<InsertLaunchAction, String> {
    let parsed = pack::parse_json(text).map_err(|error| error.to_string())?;
    <InsertLaunchAction as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = InsertLaunchAction { target: "sample".to_string() };
        assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests

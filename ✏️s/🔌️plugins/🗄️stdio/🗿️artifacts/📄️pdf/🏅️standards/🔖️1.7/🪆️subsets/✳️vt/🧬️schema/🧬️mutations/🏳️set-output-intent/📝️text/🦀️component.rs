//! 🏳️ Direct text codec for `set-output-intent`.

use super::SetOutputIntent;

//#region 🔖️Identity
pub const OPCODE: &str = "set-output-intent";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &SetOutputIntent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<SetOutputIntent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = SetOutputIntent { identifier: "sample".to_string() };
        assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests

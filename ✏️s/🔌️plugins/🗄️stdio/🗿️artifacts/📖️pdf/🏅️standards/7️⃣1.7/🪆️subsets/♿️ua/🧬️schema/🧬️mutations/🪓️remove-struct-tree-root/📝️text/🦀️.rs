//! 🪓️ Direct text codec for `remove-struct-tree-root`.

use super::RemoveStructTreeRoot;

//#region 🔖️Identity
pub const OPCODE: &str = "remove-struct-tree-root";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &RemoveStructTreeRoot) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<RemoveStructTreeRoot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = RemoveStructTreeRoot {  };
        assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests

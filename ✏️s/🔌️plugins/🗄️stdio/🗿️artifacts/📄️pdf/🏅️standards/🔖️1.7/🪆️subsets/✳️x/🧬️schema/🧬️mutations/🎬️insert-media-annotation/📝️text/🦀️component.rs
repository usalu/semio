//! 🎬️ Direct text codec for `insert-media-annotation`.

use super::InsertMediaAnnotation;

//#region 🔖️Identity
pub const OPCODE: &str = "insert-media-annotation";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &InsertMediaAnnotation) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<InsertMediaAnnotation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "sample".to_string() };
        assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests

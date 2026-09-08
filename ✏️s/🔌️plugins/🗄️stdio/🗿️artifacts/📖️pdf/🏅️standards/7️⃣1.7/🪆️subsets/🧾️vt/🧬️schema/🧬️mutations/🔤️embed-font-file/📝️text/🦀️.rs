//! 🔤️ Direct text codec for `embed-font-file`.

use super::EmbedFontFile;

//#region 🔖️Identity
pub const OPCODE: &str = "embed-font-file";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &EmbedFontFile) -> Result<String, String> {
    Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(payload))))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<EmbedFontFile, String> {
    let parsed = pack::parse_json(text).map_err(|error| error.to_string())?;
    <EmbedFontFile as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::ObjRef { num: 1, gen: 0 } };
        assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests

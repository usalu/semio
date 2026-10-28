//! ⚡️ EnergyPlus IDF model serialization over the positional interchange codec.

#[path = "🧬️.rs"]
mod codec;
pub use codec::*;

/// 🏭️ Converts a native model to runnable positional IDF with the epJSON export refusal policy.
pub fn serialize(snapshot: &crate::snapshot::EnergyModelSnapshot) -> Result<String, String> {
    let document = crate::epjson::serialize(snapshot).map_err(|error| error.to_string())?;
    write_records(&epjson_to_records(&document, DICTIONARY)?)
}

/// 📨️ Exports UTF-8 IDF bytes through the same checked serializer.
pub fn serialize_bytes(snapshot: &crate::snapshot::EnergyModelSnapshot) -> Result<Vec<u8>, String> {
    Ok(serialize(snapshot)?.into_bytes())
}

/// 🌉️ Native model JSON to EnergyPlus IDF for application and independent test bridges.
pub fn idf_from_model_json(model_json: &str) -> Result<String, String> {
    let epjson = crate::epjson::epjson_from_model_json(model_json)?;
    let document = semio_framework_pack_json::parse(&epjson, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    write_records(&epjson_to_records(&document, DICTIONARY)?)
}

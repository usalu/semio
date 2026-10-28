//! 📩️ EnergyPlus 25.2 IDF import through the native epJSON physical-model decoder.

/// 🧬️ Parses official IDD field positions before applying the model decoder's diagnostics.
pub fn import_bytes(bytes: &[u8]) -> Result<crate::epjson::EpJsonImport, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("IDF: invalid UTF-8: {error}"))?;
    let records = crate::export::idf::parse_records(text)?;
    let document = crate::export::idf::records_to_epjson(&records, crate::export::idf::DICTIONARY)?;
    crate::epjson::decode_model(&document)
}

/// 📦️ Imports a snapshot only when every object is representable without semantic loss.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<crate::snapshot::EnergyModelSnapshot, String> {
    let import = import_bytes(bytes)?;
    if !import.diagnostics.is_empty() {
        return Err(format!("IDF import requires resolving diagnostics: {}", import.diagnostics.iter().map(|diagnostic| format!("{} ({})", diagnostic.message, diagnostic.subject)).collect::<Vec<_>>().join("; ")));
    }
    Ok(crate::snapshot::EnergyModelSnapshot { model: import.model, ..Default::default() })
}

/// 🌉️ Positional IDF to native model JSON plus the explicit import diagnostics.
pub fn model_json_from_idf(idf: &str) -> Result<(String, String), String> {
    let records = crate::export::idf::parse_records(idf)?;
    let document = crate::export::idf::records_to_epjson(&records, crate::export::idf::DICTIONARY)?;
    crate::epjson::model_json_from_epjson(&semio_framework_pack_json::to_string(&document))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_model_idf_roundtrip_retains_exported_semantics() {
        let source = crate::bestest::model_json("600").unwrap();
        let idf = crate::export::idf::idf_from_model_json(&source).unwrap();
        let (model, diagnostics) = model_json_from_idf(&idf).unwrap();
        assert_eq!(diagnostics, "[]");
        assert_eq!(crate::export::idf::idf_from_model_json(&model).unwrap(), idf);
    }
}

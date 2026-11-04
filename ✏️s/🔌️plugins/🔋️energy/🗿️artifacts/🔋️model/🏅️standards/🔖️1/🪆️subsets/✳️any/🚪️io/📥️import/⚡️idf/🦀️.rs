//! 📩️ EnergyPlus 25.2 IDF import through the native epJSON physical-model decoder.

/// 🧬️ Parses official IDD field positions before applying the model decoder's diagnostics.
pub fn import_bytes(bytes: &[u8]) -> Result<crate::epjson::EpJsonImport, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("IDF: invalid UTF-8: {error}"))?;
    let records = crate::export::idf::parse_records(text)?;
    let document = crate::export::idf::records_to_epjson(&records, crate::export::idf::DICTIONARY)?;
    crate::epjson::decode_model(&document)
}

/// 🚧️ Why an IDF cannot open as an energy model, with the reason in both languages (English first, no default language). The diagnostics keep their own wording verbatim inside the localized frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdfImportError {
    NotUtf8(String),
    Malformed(String),
    Diagnostics(Vec<String>),
    Unrepresentable(String),
}

impl IdfImportError {
    /// 🔑️ The stable fault code of the refusal.
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotUtf8(_) => "energy.idf.not-utf8",
            Self::Malformed(_) => "energy.idf.malformed",
            Self::Diagnostics(_) => "energy.idf.diagnostics",
            Self::Unrepresentable(_) => "energy.idf.unrepresentable",
        }
    }

    /// 🇬🇧 The refusal in English.
    pub fn message_en(&self) -> String {
        match self {
            Self::NotUtf8(detail) => format!("IDF: invalid UTF-8: {detail}"),
            Self::Malformed(detail) => format!("IDF: {detail}"),
            Self::Diagnostics(items) => format!("IDF import requires resolving diagnostics: {}", items.join("; ")),
            Self::Unrepresentable(detail) => format!("IDF: the model cannot represent this file: {detail}"),
        }
    }

    /// 🇩🇪 The refusal in German.
    pub fn message_de(&self) -> String {
        match self {
            Self::NotUtf8(detail) => format!("IDF: ungültiges UTF-8: {detail}"),
            Self::Malformed(detail) => format!("IDF: ungültige Eingabe: {detail}"),
            Self::Diagnostics(items) => format!("Der IDF-Import erfordert die Behebung von Diagnosen: {}", items.join("; ")),
            Self::Unrepresentable(detail) => format!("IDF: das Modell kann diese Datei nicht darstellen: {detail}"),
        }
    }
}

impl std::fmt::Display for IdfImportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[{}] {} / {}", self.code(), self.message_en(), self.message_de())
    }
}

impl std::error::Error for IdfImportError {}

/// 📦️ Imports a snapshot only when every object is representable without semantic loss.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<crate::snapshot::EnergyModelSnapshot, IdfImportError> {
    std::str::from_utf8(bytes).map_err(|error| IdfImportError::NotUtf8(error.to_string()))?;
    let import = import_bytes(bytes).map_err(IdfImportError::Malformed)?;
    if !import.diagnostics.is_empty() {
        return Err(IdfImportError::Diagnostics(import.diagnostics.iter().map(|diagnostic| format!("{} ({})", diagnostic.message, diagnostic.subject)).collect()));
    }
    crate::snapshot::EnergyModelSnapshot::from_model(&import.model).map_err(|error| IdfImportError::Unrepresentable(error.to_string()))
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
    fn refusals_carry_a_code_and_both_languages_with_the_diagnostics_verbatim() {
        let error = IdfImportError::Diagnostics(vec!["unsupported field (Zone,Z1)".into()]);
        assert_eq!(error.code(), "energy.idf.diagnostics");
        assert!(error.message_en().contains("IDF import requires resolving diagnostics: unsupported field (Zone,Z1)"));
        assert!(error.message_de().contains("Der IDF-Import erfordert die Behebung von Diagnosen: unsupported field (Zone,Z1)"));
        assert!(error.to_string().starts_with("[energy.idf.diagnostics] "));
        assert!(matches!(deserialize_bytes(&[0xff]), Err(IdfImportError::NotUtf8(_))));
    }

    #[test]
    fn native_model_idf_roundtrip_retains_exported_semantics() {
        let source = crate::bestest::model_json("600").unwrap();
        let idf = crate::export::idf::idf_from_model_json(&source).unwrap();
        let (model, diagnostics) = model_json_from_idf(&idf).unwrap();
        assert_eq!(diagnostics, "[]");
        assert_eq!(crate::export::idf::idf_from_model_json(&model).unwrap(), idf);
    }
}

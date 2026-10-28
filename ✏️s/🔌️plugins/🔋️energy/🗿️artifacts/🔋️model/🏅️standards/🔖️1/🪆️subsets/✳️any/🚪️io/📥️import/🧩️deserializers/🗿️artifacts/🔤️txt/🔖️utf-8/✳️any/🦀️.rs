//! 🏋️ Energy text ingress admits native DSL and EnergyPlus IDF through their typed importers.
use crate::snapshot::EnergyModelSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("energy←txt: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    match <EnergyModelSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
        Ok(snapshot) => Ok(snapshot),
        Err(dsl_error) => {
            let is_idf = crate::export::idf::parse_records(text).is_ok_and(|records| {
                records.first().is_some_and(|record| crate::export::idf::DICTIONARY.iter().any(|object| object.kind.eq_ignore_ascii_case(&record.kind)))
            });
            if !is_idf { return Err(dsl_error); }
            crate::idf::deserialize_bytes(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn text_carrier_imports_energyplus_idf() {
        let source = crate::bestest::model_json("600").unwrap();
        let idf = crate::export::idf::idf_from_model_json(&source).unwrap();
        let snapshot = super::deserialize_bytes(idf.as_bytes()).unwrap();
        assert_eq!(crate::export::idf::serialize(&snapshot).unwrap(), idf);
    }

    #[test]
    fn text_carrier_preserves_idf_diagnostics() {
        let error = super::deserialize_bytes(b"Version,25.2;UnknownObject,Named;").unwrap_err();
        assert!(error.to_string().contains("unsupported object type UnknownObject"));
    }
}

//! 🌉️ The serializer leaf registration and the JSON bridge entry points.

use super::*;

//#region 🔖️Leaf
// 🚫️async: E1 pure codec registration hook, no io — see R9.
pub fn register() {}

/// 🚨️ The diagnostics that make a document unusable rather than merely lossy: a refusal is
/// something EnergyPlus would have had to be told and this subset cannot say, so the export stops.
/// A lossy-but-runnable note (a dropped surface multiplier, a dropped frame conductance) is
/// reported and the document is still written.
pub fn refusals(diagnostics: &[EpJsonDiagnostic]) -> Vec<&EpJsonDiagnostic> {
    diagnostics.iter().filter(|diagnostic| diagnostic.code.ends_with("-unsupported") || diagnostic.code.ends_with("dangling-zone")).collect()
}

/// 📤️ The io-leaf entry point: this artifact's snapshot as an epJSON document tree.
pub fn serialize(snapshot: &EnergyModelSnapshot) -> Result<Value, semio_framework_diagnostic::TextError> {
    let (document, diagnostics) = encode_model_with_diagnostics(&snapshot.model);
    let refused = refusals(&diagnostics);
    if !refused.is_empty() {
        let detail = refused.iter().map(|diagnostic| format!("{} ({})", diagnostic.message, diagnostic.subject)).collect::<Vec<_>>().join("; ");
        return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("model -> epJSON: {detail}"), semio_framework_diagnostic::TextSpan::at(1, 1)));
    }
    Ok(document)
}

/// 📤️ The io-leaf byte entry point: pretty-printed epJSON, the form EnergyPlus reads.
pub fn serialize_bytes(snapshot: &EnergyModelSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(format!("{}\n", semio_framework_pack_json::to_string_pretty(&serialize(snapshot)?)).into_bytes())
}
//#endregion 🔖️Leaf

//#region 🌉️Bridge
/// 🌉️ Text in, text out. The out-of-crate test adapter for
/// `🧪️tests/🏛️export-epjson-runs-in-energyplus` cannot name [`Model`], [`EnergyModelSnapshot`] or
/// [`Value`], so the case's one committed input — `🧫️fixtures/🏛️bestest-<case>/🔋️model.json` — goes
/// in as its own canonical JSON and the epJSON comes back as the exact bytes EnergyPlus is handed.
/// Same shape and same reason as `bestest::model_json`.
pub fn epjson_from_model_json(model_json: &str) -> Result<String, String> {
    let model: Model = semio_framework_pack_json::from_json_str(model_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed model does not decode: {error}"))?;
    let (document, diagnostics) = encode_model_with_diagnostics(&model);
    let refused = refusals(&diagnostics);
    if !refused.is_empty() {
        return Err(format!("model -> epJSON refused: {}", refused.iter().map(|diagnostic| format!("{} ({})", diagnostic.message, diagnostic.subject)).collect::<Vec<_>>().join("; ")));
    }
    Ok(format!("{}\n", semio_framework_pack_json::to_string_pretty(&document)))
}

/// 🌉️ The same export's full diagnostic list as a JSON array, so a scenario can assert that a
/// document carrying no refusals also dropped nothing quietly.
pub fn epjson_diagnostics_json(model_json: &str) -> Result<String, String> {
    let model: Model = semio_framework_pack_json::from_json_str(model_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed model does not decode: {error}"))?;
    let (_, diagnostics) = encode_model_with_diagnostics(&model);
    Ok(semio_framework_pack_json::to_string(&Value::Array(diagnostics.iter().map(EpJsonDiagnostic::to_json).collect())))
}
//#endregion 🌉️Bridge

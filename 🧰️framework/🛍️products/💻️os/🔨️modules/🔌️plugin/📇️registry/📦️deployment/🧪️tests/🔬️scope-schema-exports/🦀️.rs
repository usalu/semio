//! 📇️ Shared publication exports resolve exactly their schema-declared formats.
use super::*;
use semio_framework_schema_registry::{resolve_schema_export, SchemaFormat};

#[test]
fn plugin_module_schema_exports_match_declared_formats() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).expect("shared publication schema");
    let declared: std::collections::BTreeSet<_> = schema["$defs"].as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(declared, EXPORTS.iter().map(|export| export.id).collect());
    register_scope_exports();
    for export in &EXPORTS {
        let formats: Vec<_> = schema["$defs"][export.id]["x-semio-formats"].as_array().unwrap().iter().map(|format| SchemaFormat::parse(format.as_str().unwrap()).unwrap()).collect();
        for format in SchemaFormat::ALL {
            assert_eq!(resolve_schema_export(SCHEMA_SCOPE, export.id, format).is_ok(), formats.contains(&format), "{} {format}", export.id);
        }
    }
    println!("plugin module scope exports={}", EXPORTS.len());
}

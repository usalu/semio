//! 🧬️ Validates and renders the versioned owned semantic UI schema projection.
#![cfg(feature = "typegen")]

use semio_framework_ui_contract::schema_metadata;

#[test]
fn exports_typescript_bindings() {
    schema_metadata::validate().unwrap();
    assert_eq!(schema_metadata::TYPES.len(), 79);
    let rendered = schema_metadata::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, include_str!("../../../../../🛂️manifest/🤖️generated/📜️ui-contract.ts"));
    }
}

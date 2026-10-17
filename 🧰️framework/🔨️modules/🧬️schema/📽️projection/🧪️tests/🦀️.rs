//! 🧬️ The TypeScript bindings rendered from the owned schema metadata equal the checked-in generated file.
use super::*;
use semio_framework_test_fixtures::fixture_text;

#[test]
fn exports_typescript_bindings() {
    crate::schema_metadata::validate().unwrap();
    let rendered = crate::schema_metadata::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, fixture_text!("../../🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"));
    }
}

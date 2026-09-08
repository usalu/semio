
use super::*;

/// 🧬️ The gate of this module: `🔣️.json`'s `$defs` key set and the Rust registry's exported
/// type set are the SAME set. `bun nx run @semio-tech/framework-os-shell-rs:schema-check` runs
/// this test; the TypeScript side of that target cross-checks the rendered mirror.
#[test]
fn owned_json_schema_defs_match_registry() {
    schema_registry::validate().unwrap();
    let mut declared = schema_registry::owned_json_schema_def_ids().unwrap();
    let mut exported: Vec<String> = schema_registry::export_ids().into_iter().map(str::to_string).collect();
    declared.sort();
    exported.sort();
    let missing_in_json: Vec<&String> = exported.iter().filter(|name| !declared.contains(name)).collect();
    let missing_in_registry: Vec<&String> = declared.iter().filter(|name| !exported.contains(name)).collect();
    assert!(missing_in_json.is_empty(), "registry exports absent from 🔣️.json `$defs`: {missing_in_json:?}");
    assert!(missing_in_registry.is_empty(), "🔣️.json `$defs` absent from the Rust registry: {missing_in_registry:?}");
    assert_eq!(declared, exported);
}

/// 🟦️ Renders the owned TypeScript mirror. With `SEMIO_TYPEGEN_OUT` set (the `typegen` nx
/// target) it WRITES `../🤖️generated/🟦️.ts`; without it, it asserts the committed mirror is
/// exactly what this registry renders today.
#[cfg(feature = "typegen")]
#[test]
fn exports_typescript_bindings() {
    schema_registry::validate().unwrap();
    let rendered = schema_registry::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, include_str!("../../../🤖️generated/🟦️.ts"));
    }
}

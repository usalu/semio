use super::*;

#[test]
fn every_named_export_resolves_in_every_declared_format() {
    register_scope_exports();
    for export in EXPORTS {
        for format in [semio_framework_schema::SchemaFormat::JsonSchema, semio_framework_schema::SchemaFormat::Rust, semio_framework_schema::SchemaFormat::Typescript, semio_framework_schema::SchemaFormat::Graphql] {
            let leaf = semio_framework_schema::resolve_schema_export("framework.interaction", export.id, format).expect("resolve");
            assert!(!leaf.is_empty());
        }
        assert!(semio_framework_schema::resolve_schema_export("framework.interaction", export.id, semio_framework_schema::SchemaFormat::Protobuf).is_err());
        assert!(LEAVES.json_schema.contains(export.id));
    }
    assert!(semio_framework_schema::scope_schema_exports_registered("framework.interaction"));
}

/// 🏷️ Resolving to a non-empty leaf only proves the FILE exists. Execution contract §A defines export
/// presence per format as a same-named entity in that format's own vocabulary, so each leaf is asked
/// for the entity by name — the exact question `schema check`/`test schema` asks from outside.
#[test]
fn every_declared_format_leaf_declares_the_export_by_name() {
    for export in EXPORTS {
        assert!(LEAVES.json_schema.contains(&format!("\"{}\":", export.id)), "🔣️.json declares no $defs entry {}", export.id);
        assert!(LEAVES.rust.contains(&format!("pub type {} =", export.id)), "🦀️.rs declares no pub type {}", export.id);
        assert!(LEAVES.typescript.contains(&format!("export type {} =", export.id)), "🟦️.ts declares no export type {}", export.id);
        assert!(LEAVES.typescript.contains(&format!("export function parse{}(", export.id)), "🟦️.ts declares no parse{}() entry point", export.id);
        let graphql = ["type ", "input ", "enum ", "interface ", "union ", "scalar "].iter().any(|keyword| LEAVES.graphql.contains(&format!("{}{} ", keyword, export.id)) || LEAVES.graphql.contains(&format!("{}{} {{", keyword, export.id)));
        assert!(graphql, "🔗️.graphql declares no type-system entity {}", export.id);
    }
}

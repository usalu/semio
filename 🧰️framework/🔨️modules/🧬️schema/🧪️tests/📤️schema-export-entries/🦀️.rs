//! 📤️ Runtime dump of `schema_export_catalog_entries()` for catalog parity: the authority the
//! generated `🔣️schema-catalog.json` is cross-checked against by `schema verify --rust-entries`.
//!
//! Runs in its own test binary so the dump carries exactly the scopes the linked crates register and
//! nothing a sibling unit test happened to put in the process-wide catalog. When
//! `SEMIO_SCHEMA_EXPORT_ENTRIES_OUT` names a path the entries are written there as
//! `{ "entries": [ { "scope", "export", "format" } ] }`, sorted by `(scope, export, format)`, with
//! the ascii `SchemaFormat::id()` spelling of the format.

use semio_framework_schema::{register_scope_schema_exports, schema_export_catalog_entries, FacetLeaves, SchemaExport, SchemaFormat, ScopeSchemaExports};
use std::path::Path;

fn entries_json() -> String {
    let mut entries: Vec<(&'static str, &'static str, &'static str)> = schema_export_catalog_entries().into_iter().map(|entry| (entry.scope, entry.export, entry.format.id())).collect();
    entries.sort_unstable();
    entries.dedup();
    format!("{{\n  \"entries\": [{}\n  ]\n}}\n", entries.iter().map(|(scope, export, format)| format!("\n    {{ \"scope\": \"{scope}\", \"export\": \"{export}\", \"format\": \"{format}\" }}")).collect::<Vec<_>>().join(","))
}

#[test]
fn schema_export_catalog_entries_dump_is_sorted_and_matches_the_declared_contract() {
    const LEAVES: FacetLeaves = FacetLeaves {
        rust: "pub struct SchemaExportEntry;",
        typescript: "export type SchemaExportEntry = {};",
        graphql: "",
        json_schema: "{\"$schema\":\"http://json-schema.org/draft-07/schema#\",\"$id\":\"https://semio.tech/schema/framework/schema/entries/schema.json\",\"type\":\"object\"}",
        proto: "",
    };
    const EXPORTS: [SchemaExport; 1] = [SchemaExport { id: "SchemaExportEntry", leaves: LEAVES }];
    register_scope_schema_exports(ScopeSchemaExports { scope: "framework.schema.entries", exports: &EXPORTS }).expect("scope schema exports");

    let entries = schema_export_catalog_entries();
    for format in [SchemaFormat::Rust, SchemaFormat::Typescript, SchemaFormat::JsonSchema] {
        assert!(
            entries.iter().any(|entry| entry.scope == "framework.schema.entries" && entry.export == "SchemaExportEntry" && entry.format == format),
            "expected a {format} entry for the registered scope"
        );
    }
    for format in [SchemaFormat::Graphql, SchemaFormat::Protobuf] {
        assert!(entries.iter().all(|entry| !(entry.scope == "framework.schema.entries" && entry.format == format)), "an empty {format} leaf must not become a catalog entry");
    }

    let json = entries_json();
    assert!(json.starts_with("{\n  \"entries\": ["), "unexpected dump shape:\n{json}");
    assert!(json.contains("{ \"scope\": \"framework.schema.entries\", \"export\": \"SchemaExportEntry\", \"format\": \"jsonschema\" }"), "unexpected entry shape:\n{json}");

    if let Ok(destination) = std::env::var("SEMIO_SCHEMA_EXPORT_ENTRIES_OUT") {
        if let Some(parent) = Path::new(&destination).parent() {
            std::fs::create_dir_all(parent).expect("entries directory");
        }
        std::fs::write(&destination, &json).expect("entries");
    }
    println!("[schema-export-entries] {}", json.replace('\n', ""));
}

//! 📤️ Runtime dump of `schema_export_catalog_entries()` for catalog parity: the authority the
//! generated `🔣️schema-catalog.json` is cross-checked against by `schema verify --rust-entries`.
//!
//! Runs in its own test binary so the dump carries exactly the scopes the linked crates register and
//! nothing a sibling unit test happened to put in the process-wide catalog. The rendered dump is the
//! `schema-export-registry-entries-v1` shape
//! `{ "contractId", "generator", "entries": [ { "scope", "export", "format" } ] }`, entries sorted by
//! `(scope, export, format)` with the ascii `SchemaFormat::id()` spelling. `contractId` is the
//! taxonomy `schemaExportResolution.rustEntriesContractId`; `SchemaRustEntryDump` in
//! `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` is the consumer.
//!
//! The dump is checked three ways: against the committed fixture
//! `🧫️fixtures/📤️schema-export-entries-dump.json` (regenerate with
//! `SEMIO_SCHEMA_EXPORT_ENTRIES_OUT=<that path>`), against the `framework.schema` JSON Schema facet
//! through the owned draft-07 validator, and — from the same fixture — against `ajv` in
//! `🧪️tests/📤️schema-export-entries/🟦️.ts`.

use semio_framework_schema::{register_framework_schema_exports, register_scope_schema_exports, schema_export_catalog_entries, structural_validator_for, FacetLeaves, SchemaExport, SchemaExportEntries, SchemaFormat, ScopeSchemaExports, FRAMEWORK_SCHEMA_SCOPE};
use std::path::Path;

const RUST_ENTRIES_GENERATOR: &str = "cargo test -p semio-framework-schema --test schema-export-entries";
const COMMITTED_DUMP: &str = include_str!("../../🧫️fixtures/📤️schema-export-entries-dump.json");

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
    register_framework_schema_exports().expect("framework.schema scope schema exports");

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

    let dump = SchemaExportEntries::from_catalog(RUST_ENTRIES_GENERATOR);
    assert_eq!(dump.contract_id, SchemaExportEntries::CONTRACT_ID);
    let mut sorted = dump.entries.clone();
    sorted.sort_unstable_by_key(|entry| (entry.scope, entry.export, entry.format.id()));
    assert_eq!(dump.entries, sorted, "entries must be sorted by (scope, export, format)");
    let json = dump.to_json();

    let validator = structural_validator_for(FRAMEWORK_SCHEMA_SCOPE, "SchemaExportEntries").expect("SchemaExportEntries validator");
    if let Err(error) = validator.validate_json(&json) {
        panic!("the runtime dump must satisfy the framework.schema facet: {error}\n{json}");
    }

    if let Ok(destination) = std::env::var("SEMIO_SCHEMA_EXPORT_ENTRIES_OUT") {
        if let Some(parent) = Path::new(&destination).parent() {
            std::fs::create_dir_all(parent).expect("entries directory");
        }
        std::fs::write(&destination, &json).expect("entries");
    }
    assert_eq!(json, COMMITTED_DUMP, "the committed dump fixture drifted — regenerate it with SEMIO_SCHEMA_EXPORT_ENTRIES_OUT");
}

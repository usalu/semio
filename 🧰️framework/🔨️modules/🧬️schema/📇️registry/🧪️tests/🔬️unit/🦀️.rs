
use super::*;

const EXPORT_LEAVES: FacetLeaves =
    FacetLeaves { rust: "pub struct Thing;", typescript: "export type Thing = {};", graphql: "type Thing { id: String! }", json_schema: r#"{"$id":"https://semio.tech/schema/test/thing.json","type":"object"}"#, proto: "" };
const EMPTY_LEAVES: FacetLeaves = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
const NAMED_EXPORTS: [SchemaExport; 2] = [SchemaExport { id: "Thing", leaves: EXPORT_LEAVES }, SchemaExport { id: "Other", leaves: EXPORT_LEAVES }];

fn facets() -> [FacetLeaves; 4] {
    [EXPORT_LEAVES, EMPTY_LEAVES, EMPTY_LEAVES, EMPTY_LEAVES]
}

#[test]
fn schema_format_ids_mirror_the_taxonomy_schema_formats_keys() {
    assert_eq!(SchemaFormat::ALL.map(|format| format.id()), ["rust", "typescript", "graphql", "jsonschema", "protobuf"]);
    assert_eq!(SchemaFormat::ALL.map(|format| format.taxonomy_key()), ["🦀️rust", "🟦️typescript", "🔗️graphql", "🔣️jsonschema", "🛰️protobuf"]);
    for format in SchemaFormat::ALL {
        assert_eq!(SchemaFormat::parse(format.id()), Some(format));
        assert_eq!(SchemaFormat::parse(format.taxonomy_key()), Some(format));
    }
    assert_eq!(SchemaFormat::parse("📜️wit"), None);
}

#[test]
fn schema_export_registration_is_duplicate_safe_and_conflict_fatal() {
    const OTHER: [SchemaExport; 1] = [SchemaExport { id: "Thing", leaves: EXPORT_LEAVES }];
    let mut registry = SchemaExportRegistry::new();
    registry.register_facet_leaves("test.scope", facets()).expect("first facets");
    registry.register_facet_leaves("test.scope", facets()).expect("exact duplicate facets");
    registry.register_exports(ScopeSchemaExports { scope: "test.scope", exports: &NAMED_EXPORTS }).expect("first exports");
    registry.register_exports(ScopeSchemaExports { scope: "test.scope", exports: &NAMED_EXPORTS }).expect("exact duplicate exports");
    assert_eq!(registry.register_exports(ScopeSchemaExports { scope: "test.scope", exports: &OTHER }), Err(SchemaExportRegistryError::ConflictingScope { scope: "test.scope".to_string() }));
    assert_eq!(registry.register_facet_leaves("test.scope", [EMPTY_LEAVES; 4]), Err(SchemaExportRegistryError::ConflictingScope { scope: "test.scope".to_string() }));
}

#[test]
fn schema_export_registration_rejects_a_duplicate_export_id_inside_one_scope() {
    const DUPLICATED: [SchemaExport; 2] = [SchemaExport { id: "Thing", leaves: EXPORT_LEAVES }, SchemaExport { id: "Thing", leaves: EXPORT_LEAVES }];
    let mut registry = SchemaExportRegistry::new();
    assert_eq!(registry.register_exports(ScopeSchemaExports { scope: "test.duplicate", exports: &DUPLICATED }), Err(SchemaExportRegistryError::DuplicateExportId { scope: "test.duplicate".to_string(), export: "Thing".to_string() }));
}

#[test]
fn schema_export_resolution_distinguishes_every_failure() {
    const SHADOWING: [SchemaExport; 1] = [SchemaExport { id: "snapshot", leaves: EXPORT_LEAVES }];
    let mut registry = SchemaExportRegistry::new();
    registry.register_facet_leaves("test.resolve", facets()).expect("facets");
    registry.register_exports(ScopeSchemaExports { scope: "test.resolve", exports: &NAMED_EXPORTS }).expect("exports");
    assert_eq!(registry.resolve("test.resolve", "Thing", SchemaFormat::Rust), Ok("pub struct Thing;"));
    assert_eq!(registry.resolve("test.resolve", "artifact", SchemaFormat::Graphql), Ok("type Thing { id: String! }"));
    assert_eq!(registry.resolve("test.missing", "Thing", SchemaFormat::Rust), Err(SchemaResolveError::UnknownScope { scope: "test.missing".to_string() }));
    assert_eq!(registry.resolve("test.resolve", "Absent", SchemaFormat::Rust), Err(SchemaResolveError::UnknownExport { scope: "test.resolve".to_string(), export: "Absent".to_string() }));
    assert_eq!(registry.resolve("test.resolve", "Thing", SchemaFormat::Protobuf), Err(SchemaResolveError::FormatAbsent { scope: "test.resolve".to_string(), export: "Thing".to_string(), format: SchemaFormat::Protobuf }));
    assert_eq!(registry.resolve("test.resolve", "snapshot", SchemaFormat::Rust), Err(SchemaResolveError::FormatAbsent { scope: "test.resolve".to_string(), export: "snapshot".to_string(), format: SchemaFormat::Rust }));
    let mut shadowed = SchemaExportRegistry::new();
    shadowed.register_facet_leaves("test.shadow", facets()).expect("facets");
    shadowed.register_exports(ScopeSchemaExports { scope: "test.shadow", exports: &SHADOWING }).expect("exports");
    assert_eq!(shadowed.resolve("test.shadow", "snapshot", SchemaFormat::Rust), Err(SchemaResolveError::AmbiguousScope { scope: "test.shadow".to_string(), export: "snapshot".to_string() }));
}

#[test]
fn schema_export_registry_reports_every_scope_with_its_exports() {
    let mut registry = SchemaExportRegistry::new();
    registry.register_facet_leaves("test.alpha", facets()).expect("alpha facets");
    registry.register_exports(ScopeSchemaExports { scope: "test.alpha", exports: &NAMED_EXPORTS }).expect("alpha exports");
    registry.register_exports(ScopeSchemaExports { scope: "test.beta", exports: &NAMED_EXPORTS }).expect("beta exports");
    assert_eq!(registry.scopes(), vec!["test.alpha", "test.beta"]);
    assert_eq!(registry.exports("test.alpha").expect("alpha"), vec!["artifact", "snapshot", "diff", "mutations", "Thing", "Other"]);
    assert_eq!(registry.exports("test.beta").expect("beta"), vec!["Thing", "Other"]);
    assert_eq!(registry.exports("test.absent"), Err(SchemaResolveError::UnknownScope { scope: "test.absent".to_string() }));
    let entries: Vec<SchemaExportEntry> = registry.entries().collect();
    assert!(entries.contains(&SchemaExportEntry { scope: "test.alpha", export: "artifact", format: SchemaFormat::Rust }));
    assert!(!entries.iter().any(|entry| entry.format == SchemaFormat::Protobuf));
}

#[test]
fn entries_dump_renders_the_declared_contract_and_escapes_the_generator() {
    let dump = SchemaExportEntries { contract_id: SchemaExportEntries::CONTRACT_ID, generator: "cargo test \"quoted\"\n".to_string(), entries: vec![SchemaExportEntry { scope: "test.dump", export: "Thing", format: SchemaFormat::JsonSchema }] };
    assert_eq!(
        dump.to_json(),
        "{\n  \"contractId\": \"schema-export-registry-entries-v1\",\n  \"generator\": \"cargo test \\\"quoted\\\"\\n\",\n  \"entries\": [\n    { \"scope\": \"test.dump\", \"export\": \"Thing\", \"format\": \"jsonschema\" }\n  ]\n}\n"
    );
}

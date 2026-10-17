
use super::*;

const EXPORT_LEAVES: FacetLeaves =
    FacetLeaves { rust: "pub struct Thing;", typescript: "export type Thing = {};", graphql: "type Thing { id: String! }", json_schema: r#"{"$id":"https://json.schemas.assets.semio-tech.com/test/thing.json","type":"object"}"#, proto: "" };
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

struct MemoryLeaves;

impl SchemaLeafReader for MemoryLeaves {
    fn read(&self, path: &str) -> Option<&'static str> {
        match path {
            "memory/leaf.ts" => Some("export type Served = {};"),
            "memory/first.ts" => Some("export type First = {};\n"),
            "memory/second.ts" => Some("export type Second = {};\n"),
            _ => None,
        }
    }
}

static MEMORY_LEAVES: MemoryLeaves = MemoryLeaves;

#[test]
fn served_leaf_documents_resolve_when_the_registering_crate_embeds_no_body() {
    let _ = install_schema_leaf_reader(&MEMORY_LEAVES);
    let document = SchemaLeafDocument { scope: "test.served", export: "artifact", format: SchemaFormat::Typescript, path: "memory/leaf.ts" };
    register_schema_leaf_documents(&[document]);
    register_schema_leaf_documents(&[document]);
    let embedded = FacetLeaves { rust: "pub struct Thing;", typescript: "", graphql: "", json_schema: "{}", proto: "" };
    let mut registry = SchemaExportRegistry::new();
    registry.register_facet_leaves("test.served", [embedded, EMPTY_LEAVES, EMPTY_LEAVES, EMPTY_LEAVES]).expect("served facets");
    assert_eq!(registry.resolve("test.served", "artifact", SchemaFormat::Typescript), Ok("export type Served = {};"));
    assert_eq!(registry.resolve("test.served", "artifact", SchemaFormat::Rust), Ok("pub struct Thing;"));
    assert_eq!(
        registry.resolve("test.served", "artifact", SchemaFormat::Graphql),
        Err(SchemaResolveError::FormatAbsent { scope: "test.served".to_string(), export: "artifact".to_string(), format: SchemaFormat::Graphql })
    );
    assert!(registry.entries().any(|entry| entry.scope == "test.served" && entry.format == SchemaFormat::Typescript));
}

#[test]
fn several_served_documents_of_one_leaf_resolve_as_their_concatenation_in_registration_order() {
    let _ = install_schema_leaf_reader(&MEMORY_LEAVES);
    let first = SchemaLeafDocument { scope: "test.joined", export: "mutations", format: SchemaFormat::Typescript, path: "memory/first.ts" };
    let second = SchemaLeafDocument { scope: "test.joined", export: "mutations", format: SchemaFormat::Typescript, path: "memory/second.ts" };
    register_schema_leaf_documents(&[first, second]);
    register_schema_leaf_documents(&[first, second]);
    let mut registry = SchemaExportRegistry::new();
    registry.register_facet_leaves("test.joined", [EMPTY_LEAVES, EMPTY_LEAVES, EMPTY_LEAVES, EMPTY_LEAVES]).expect("joined facets");
    let expected = "export type First = {};\nexport type Second = {};\n";
    assert_eq!(registry.resolve("test.joined", "mutations", SchemaFormat::Typescript), Ok(expected));
    assert_eq!(registry.resolve("test.joined", "mutations", SchemaFormat::Typescript), Ok(expected));
    let missing = SchemaLeafDocument { scope: "test.joined", export: "mutations", format: SchemaFormat::Graphql, path: "memory/first.ts" };
    let absent = SchemaLeafDocument { scope: "test.joined", export: "mutations", format: SchemaFormat::Graphql, path: "memory/never.ts" };
    register_schema_leaf_documents(&[missing, absent]);
    assert_eq!(
        registry.resolve("test.joined", "mutations", SchemaFormat::Graphql),
        Err(SchemaResolveError::FormatAbsent { scope: "test.joined".to_string(), export: "mutations".to_string(), format: SchemaFormat::Graphql })
    );
}

#[test]
fn a_direct_reader_resolves_an_empty_inline_body_through_the_served_document() {
    let _ = install_schema_leaf_reader(&MEMORY_LEAVES);
    let served = SchemaLeafDocument { scope: "test.direct", export: "snapshot", format: SchemaFormat::Typescript, path: "memory/first.ts" };
    register_schema_leaf_documents(&[served]);
    let inline = FacetLeaves { typescript: "export type Inline = {};\n", ..EMPTY_LEAVES };
    assert_eq!(resolved_leaf_body("test.direct", "snapshot", SchemaFormat::Typescript, &EMPTY_LEAVES), "export type First = {};\n");
    assert_eq!(resolved_leaf_body("test.direct", "snapshot", SchemaFormat::Typescript, &inline), "export type Inline = {};\n", "an inline body wins over a served document");
    assert_eq!(resolved_leaf_body("test.direct", "snapshot", SchemaFormat::Graphql, &EMPTY_LEAVES), "", "an unserved format stays empty");
}

fn delivered_package_asset(name: &str) -> Option<&'static [u8]> {
    match name {
        "memory/delivered.json" => Some(br#"{"$id":"delivered"}"#),
        "memory/not-utf8.json" => Some(&[0xff, 0xfe]),
        _ => None,
    }
}

#[test]
fn package_asset_leaves_answer_delivered_documents_and_refuse_everything_else() {
    let reader = PackageAssetSchemaLeaves::new(delivered_package_asset);
    assert_eq!(reader.read("memory/delivered.json"), Some(r#"{"$id":"delivered"}"#));
    assert_eq!(reader.read("memory/delivered.json"), Some(r#"{"$id":"delivered"}"#));
    assert_eq!(reader.read("memory/never-delivered.json"), None, "a host that delivered nothing under the name answers none");
    assert_eq!(reader.read("memory/not-utf8.json"), None, "a document that is not text is refused, not guessed");
}

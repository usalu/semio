//! 📇️ The process-wide catalog seen from outside the crate: a scope that links **only**
//! `semio-framework-schema-registry` — the position `semio-framework-ui-contract` and every other
//! dependency-restricted scope is in — registers its exports, resolves them, and appears in the
//! `schema-export-registry-entries-v1` dump. Own test binary so the catalog carries exactly what this
//! file registers.

use semio_framework_schema_registry::{
    register_scope_facet_leaves, register_scope_schema_exports, resolve_schema_export, schema_export_catalog_entries, scope_schema_exports_registered, scope_schema_facets_registered, with_schema_export_registry, FacetLeaves, SchemaExport, SchemaExportEntries, SchemaExportEntry,
    SchemaFormat, SchemaResolveError, ScopeSchemaExports,
};

const LEAVES: FacetLeaves = FacetLeaves {
    rust: "pub struct UiSnapshot;",
    typescript: "export type UiSnapshot = {};",
    graphql: "",
    json_schema: r#"{"$id":"https://semio.tech/schema/framework/ui/contract/schema.json","type":"object"}"#,
    proto: "",
};
const EMPTY: FacetLeaves = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
const EXPORTS: [SchemaExport; 2] = [SchemaExport { id: "UiSnapshot", leaves: LEAVES }, SchemaExport { id: "UiPatch", leaves: LEAVES }];

#[test]
fn a_dependency_restricted_scope_registers_resolves_and_dumps_through_the_registry_crate_alone() {
    register_scope_schema_exports(ScopeSchemaExports { scope: "framework.ui.contract", exports: &EXPORTS }).expect("register named exports");
    register_scope_schema_exports(ScopeSchemaExports { scope: "framework.ui.contract", exports: &EXPORTS }).expect("exact duplicate registration is accepted");
    register_scope_facet_leaves("framework.ui.contract", [LEAVES, EMPTY, EMPTY, EMPTY]).expect("register fixed facets");

    assert!(scope_schema_exports_registered("framework.ui.contract"));
    assert!(scope_schema_facets_registered("framework.ui.contract"));
    assert!(!scope_schema_exports_registered("framework.ui.absent"));

    for export in EXPORTS {
        for format in [SchemaFormat::Rust, SchemaFormat::Typescript, SchemaFormat::JsonSchema] {
            let leaf = resolve_schema_export("framework.ui.contract", export.id, format).expect("resolve declared format");
            assert!(!leaf.trim().is_empty(), "{} {} resolved to an empty leaf", export.id, format);
        }
        assert_eq!(
            resolve_schema_export("framework.ui.contract", export.id, SchemaFormat::Graphql),
            Err(SchemaResolveError::FormatAbsent { scope: "framework.ui.contract".to_string(), export: export.id.to_string(), format: SchemaFormat::Graphql })
        );
    }
    assert_eq!(resolve_schema_export("framework.ui.contract", "artifact", SchemaFormat::Rust), Ok("pub struct UiSnapshot;"));
    assert_eq!(resolve_schema_export("framework.ui.absent", "UiSnapshot", SchemaFormat::Rust), Err(SchemaResolveError::UnknownScope { scope: "framework.ui.absent".to_string() }));

    with_schema_export_registry(|registry| {
        assert_eq!(registry.scopes(), vec!["framework.ui.contract"]);
        assert_eq!(registry.exports("framework.ui.contract").expect("exports"), vec!["artifact", "snapshot", "diff", "mutations", "UiSnapshot", "UiPatch"]);
    });

    let entries = schema_export_catalog_entries();
    assert!(entries.contains(&SchemaExportEntry { scope: "framework.ui.contract", export: "UiSnapshot", format: SchemaFormat::Typescript }));
    assert!(!entries.iter().any(|entry| entry.format == SchemaFormat::Protobuf), "no scope declares a proto leaf: {entries:?}");

    let dump = SchemaExportEntries::from_catalog("cargo test -p semio-framework-schema-registry --test schema-registry-catalog");
    assert_eq!(dump.contract_id, SchemaExportEntries::CONTRACT_ID);
    let mut sorted = dump.entries.clone();
    sorted.sort_unstable_by_key(|entry| (entry.scope, entry.export, entry.format.id()));
    assert_eq!(dump.entries, sorted, "entries are sorted by (scope, export, format)");
    assert!(dump.to_json().starts_with("{\n  \"contractId\": \"schema-export-registry-entries-v1\",\n  \"generator\": \"cargo test -p semio-framework-schema-registry --test schema-registry-catalog\",\n  \"entries\": ["), "unexpected dump shape:\n{}", dump.to_json());
}

//! 🧭️ Declaration registration laws: the declaration tree registration of every plugin, the atomic declaration registration and the derived subset declaration ids. See [`super`].

use super::*;

// 🌳️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-C Task 4 — laws for the new
// declaration tree (`super::declarations`). Every law commits through the REAL
// `PluginBuilder::declare_artifact`/`try_build` path (or, for the pure id-derivation law, no
// registration at all) rather than reaching into `declarations::commit_artifact_declarations`
// directly, so a passing law is proof the whole public surface works end to end.
/// 🌳️ Builds a plugin from `declaration` and asserts every subset ends up registered in the
/// schema registry, the io registry (when it declares any `IoEntry` rows), and the app router
/// (manifest apps) — "nothing is silently dropped" (Task 4, law 1/3).
pub async fn assert_declaration_tree_registers_all<PA: PluginApp>(plugin_id: &str, declaration: semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration<PA>) {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let expected_schema_ids: Vec<&'static str> = declaration.standards.iter().flat_map(|standard| standard.subsets.iter().map(|subset| subset.schema.descriptor.id)).collect();
    let mut expected_io_pairs: Vec<(String, String)> = Vec::new();
    for entry in declaration.standards.iter().flat_map(|standard| standard.subsets.iter()).flat_map(|subset| subset.io.entries.iter()) {
        expected_io_pairs.push((semio_framework_artifact_reference::ArtifactDialect::from(entry.from).to_coordinate(), semio_framework_artifact_reference::ArtifactDialect::from(entry.into).to_coordinate()));
    }
    let expected_surface_ids: Vec<String> = declaration.standards.iter().flat_map(|standard| standard.subsets.iter()).flat_map(|subset| [subset.editor.definition.id.clone(), subset.viewer.definition.id.clone()]).collect();

    let plugin = semio_framework_plugin_builder::PluginBuilder::new(plugin_id).label(plugin_id).version("0.0.1").package_id(format!("semio:{plugin_id}")).declare_artifact(declaration).try_build().expect("a well-formed declaration tree must register");

    for id in &expected_schema_ids {
        assert!(::semio_framework_schema_registry::artifact_schema_descriptor_registered(id), "schema descriptor {id:?} must be registered");
    }
    let live_io_entries = semio_framework_os_kernel::io::io_mechanism::io_entries();
    for (from, into) in &expected_io_pairs {
        let mut found = false;
        for entry in live_io_entries.iter() {
            if &entry.from.to_coordinate() == from && &entry.into.to_coordinate() == into {
                found = true;
                break;
            }
        }
        assert!(found, "io entry {from} -> {into} must be registered");
    }
    let live_app_ids: Vec<&str> = plugin.manifest.apps.iter().map(|app| app.id.as_str()).collect();
    for id in &expected_surface_ids {
        assert!(live_app_ids.contains(&id.as_str()), "surface {id:?} must be registered on the app router");
    }
}

/// 🌳️ `invalid` must fail preflight by construction (e.g. two subsets whose schema
/// descriptors share an id with different content) — asserts the schema AND io registries are
/// byte-for-byte unchanged after the rejection: a preflight-failing declaration leaves ZERO
/// rows behind (Task 4, law 2/3).
pub async fn assert_declaration_registration_is_atomic<PA: PluginApp>(plugin_id: &str, invalid: semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration<PA>) {
    let schema_count_before = ::semio_framework_schema_registry::with_artifact_schema_registry(|registry| registry.len());
    let io_count_before = semio_framework_os_kernel::io::io_mechanism::io_entries().len();
    let result = semio_framework_plugin_builder::PluginBuilder::new(plugin_id).label(plugin_id).version("0.0.1").package_id(format!("semio:{plugin_id}")).declare_artifact(invalid).try_build();
    assert!(result.is_err(), "a declaration that fails preflight must not build a plugin");
    assert_eq!(::semio_framework_schema_registry::with_artifact_schema_registry(|registry| registry.len()), schema_count_before, "schema registry must be unchanged after a rejected declaration");
    assert_eq!(semio_framework_os_kernel::io::io_mechanism::io_entries().len(), io_count_before, "io registry must be unchanged after a rejected declaration");
}

/// 🌳️ Every subset's editor/viewer surface id must equal `surface_app_id(&dialect, role)` —
/// pure, no registration (Task 4, law 3/3).
pub async fn assert_subset_declaration_ids_are_derived<PA: PluginApp>(declaration: &semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration<PA>) {
    for standard in &declaration.standards {
        for subset in &standard.subsets {
            let dialect: semio_framework_artifact_reference::ArtifactDialect = subset.dialect.into();
            assert_eq!(subset.editor.definition.id, semio_framework::surface_app_id(&dialect, semio_framework::AppRole::Editor), "editor surface id must be derived via surface_app_id");
            assert_eq!(subset.viewer.definition.id, semio_framework::surface_app_id(&dialect, semio_framework::AppRole::Viewer), "viewer surface id must be derived via surface_app_id");
        }
    }
}

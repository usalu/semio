//! 🔗️ The authored concrete-forest example's kind catalog names only wire and edge kinds the compiled `puzzle3d-default` graph manifest registers.

use semio_s_artifact_puzzle_3d_model::KindCatalogBundle;

/// 🔗️ Keeps the example scene_snapshot's scene-authored kind catalog in sync with the compile-time
/// `puzzle3d-default` manifest.
#[test]
fn concrete_forest_kind_catalog_matches_puzzle3d_default_manifest() {
    let scene_snapshot = semio_s_artifact_puzzle_3d_model::io::text::snapshot::parse_dsl(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio")).expect("committed example reads")).expect("concrete-forest example parses as dsl");
    let catalogs: KindCatalogBundle = serde_json::from_value(serde_json::to_value(&scene_snapshot.meta.kind_catalogs).unwrap()).unwrap();
    let manifest = crate::graph_manifest::manifest_by_id("puzzle3d-default").expect("puzzle3d-default manifest must be registered");
    let wire_kind_ids: std::collections::BTreeSet<_> = manifest.wire_kinds.iter().map(|row| row.id.as_str()).collect();
    let edge_kind_ids: std::collections::BTreeSet<_> = manifest.edge_kinds.iter().map(|row| row.id.as_str()).collect();
    for vortex in &catalogs.vortices {
        if let Some(default_cable_kind) = &vortex.default_cable_kind {
            assert!(wire_kind_ids.contains(default_cable_kind.as_str()), "vortex kind {:?} references unknown wire kind {default_cable_kind:?}", vortex.id);
        }
    }
    for cable in &catalogs.cables {
        if let Some(default_attraction_kind) = &cable.default_attraction_kind {
            assert!(edge_kind_ids.contains(default_attraction_kind.as_str()), "cable kind {:?} references unknown edge kind {default_attraction_kind:?}", cable.id);
        }
    }
}

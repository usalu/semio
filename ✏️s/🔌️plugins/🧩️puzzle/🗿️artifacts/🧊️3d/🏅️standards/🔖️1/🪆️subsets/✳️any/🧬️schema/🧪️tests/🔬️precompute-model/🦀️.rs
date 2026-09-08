
use super::*;

/// 🔗️ Keeps the example fixture's scene-authored kind catalog in sync with the compile-time
/// `puzzle3d-default` manifest.
#[test]
fn concrete_forest_kind_catalog_matches_puzzle3d_default_manifest() {
    let fixture = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("concrete-forest example parses as dsl");
    let catalogs: KindCatalogBundle = serde_json::from_value(serde_json::to_value(&fixture.meta.kind_catalogs).unwrap()).unwrap();
    let manifest = semio_framework_graph::manifest::manifest_by_id("puzzle3d-default").expect("puzzle3d-default manifest must be registered");
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

/// 🪪️ A vortex id that already carries its owner's prefix is passed through untouched.
#[test]
fn vortex_full_id_prefixes_only_bare_ids() {
    assert_eq!(puzzle3d_vortex_full_id("host", "v0"), "host:v0");
    assert_eq!(puzzle3d_vortex_full_id("host", "other:v0"), "other:v0");
}

#[test]
fn brush_preview_state_converts_into_a_placement_payload() {
    let preview = BrushPreviewState {
        target_vortex_full_id: "host:v0".into(),
        object_kind_id: "Kind".into(),
        source_vortex_index: 2,
        mesh_url: "/mesh.glb".into(),
        origin: [1.0, 2.0, 3.0],
        orientation: [0.0, 0.0, 0.0, 1.0],
        scale: Some(dsl::DslValue::float(2.0)),
    };
    let payload = BrushPlacePayload::from(preview);
    assert_eq!(payload.target_vortex_full_id, "host:v0");
    assert_eq!(payload.object_kind_id, "Kind");
    assert_eq!(payload.source_vortex_index, 2);
    assert_eq!(payload.origin, [1.0, 2.0, 3.0]);
    assert_eq!(payload.scale, Some(dsl::DslValue::float(2.0)));
}

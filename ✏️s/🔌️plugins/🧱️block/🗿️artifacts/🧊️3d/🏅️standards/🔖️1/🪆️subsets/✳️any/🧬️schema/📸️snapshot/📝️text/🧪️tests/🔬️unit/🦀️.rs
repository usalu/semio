
use super::*;
use crate::{Block3dVortexKind, Block3dVortexTemplate};
use crate::{BlockCamera3d, BlockKindIdentity, BlockRepresentation};

pub fn nakagin_capsule() -> Block3dSnapshot {
    let mut definition = Block3dSnapshot {
        object_kind: BlockKindIdentity { id: "Capsule J".into(), name: "Capsule J".into(), label: "Capsule J".into(), ..Default::default() },
        camera3d: BlockCamera3d { position: [10.0, -10.0, 6.0], target: [0.0, 0.0, 1.0], zoom: 1.0 },
        ..Block3dSnapshot::default()
    };
    definition.representations.push(BlockRepresentation {
        id: "r0".into(),
        name: "Full Detail".into(),
        mesh_url: Some("/mesh/🧊️capsule_J.glb".into()),
        tags: vec!["full".into()],
        lod: Some("full".into()),
        description: String::new(),
        attributes: Vec::new(),
    });
    crate::set_vortex_kinds(&mut definition, &[Block3dVortexKind { id: "door".into(), name: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() }]);
    definition.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [0.0, -1.6, 1.2], direction: [0.0, -1.0, 0.0], radius: 0.3, label: Some("door".into()) });
    definition
}

#[semio_framework_async_macros::async_test]
async fn block3d_definition_dsl_round_trips() {
    let empty = Block3dSnapshot::default();
    store::os_store::test_support::assert_dsl_round_trip(&empty);
    store::os_store::test_support::assert_dsl_pack_equivalence(&empty);
    let definition = nakagin_capsule();
    store::os_store::test_support::assert_dsl_round_trip(&definition);
    store::os_store::test_support::assert_dsl_pack_equivalence(&definition);
}

/// ⚖️ LAW: the boot document is never empty and every representation it carries names a mesh —
/// `world_meshes_json` silently drops a representation whose `mesh_url` is `None`, so a boot
/// document with representations but no mesh urls would still paint an empty `World3d` scene.
#[semio_framework_async_macros::async_test]
async fn block3d_boot_snapshot_carries_at_least_one_resolvable_representation() {
    let boot = block3d_boot_snapshot();
    assert!(!boot.representations.is_empty(), "the boot document must render a non-empty world");
    assert!(boot.representations.iter().all(|representation| representation.mesh_url.as_deref().is_some_and(|url| !url.is_empty())), "every boot representation must name a mesh url");
    assert_eq!(boot.object_kind.id, "Hexagonal Cut Concrete Forest Left");
}

/// ⚖️ LAW: no example fixture may reference a mesh the delivery catalog cannot resolve — the
/// `nakagin-capsule` `"1:500"` representation used to name `/mesh/capsule_J.1to500.glb`, which no
/// catalog ships, and `resolveMeshAsset` throws on it the moment the example loads.
#[semio_framework_async_macros::async_test]
async fn block3d_example_fixtures_only_name_catalogued_meshes() {
    let catalogued = ["/mesh/🧊️capsule_J.glb", "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb"];
    for dsl_text in [BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT, BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT] {
        let definition = parse_dsl(dsl_text).expect("example fixture parses as dsl");
        for representation in &definition.representations {
            let url = representation.mesh_url.as_deref().expect("example representation names a mesh url");
            assert!(catalogued.contains(&url), "example representation {} names uncatalogued mesh {url}", representation.id);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn block3d_example_fixtures_parse_and_round_trip_as_dsl() {
    for dsl_text in [BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT, BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT] {
        let definition = parse_dsl(dsl_text).expect("example fixture parses as dsl");
        store::os_store::test_support::assert_dsl_round_trip(&definition);
    }
}

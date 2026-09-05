//! 📜️ Block 3D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::block3d::Block3dSnapshot;

/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block3d` DSL — the `ObjectKind` half
/// of semio_compose_rs's metabolism-kit `Capsule` type.
pub const BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio");
/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block3d` DSL.
pub const BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio");

/// 📖️ Parses `.block3d` DSL text into a `Block3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block3dSnapshot, store::TextError> {
    <Block3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block3dSnapshot` back to `.block3d` DSL text.
pub fn print_dsl(document: &Block3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

/// 🚀️ The document every `block3d` surface boots with — `hexagonal-cut-concrete-forest-left`, the only
/// built-in example whose every representation `mesh_url` resolves against the delivery catalog
/// (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json` and its nested `🌱️metabolism` collection),
/// so the `World3d` surface renders a real mesh instead of an empty scene on first paint. Falls back to
/// the empty document if the embedded fixture ever stops parsing — a boot must never fault on a fixture.
pub fn block3d_boot_snapshot() -> Block3dSnapshot {
    parse_dsl(BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block3d::{Block3dVortexKind, Block3dVortexTemplate};
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
        crate::artifacts::block3d::set_vortex_kinds(&mut definition, vec![Block3dVortexKind { id: "door".into(), name: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() }]);
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
}
//#endregion 🧪️Tests

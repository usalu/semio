//! 📜️ Block 3D artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block3d::Block3dSnapshot;

/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block3d` DSL — the `ObjectKind` half
/// of semio_compose_rs's metabolism-kit `Capsule` type.
pub const BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️nakagin-capsule/🖼️assets/🗣️nakagin-capsule.dsl.semio");
/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block3d` DSL.
pub const BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️hexagonal-cut-concrete-forest-left/🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio");

/// 📖️ Parses `.block3d` DSL text into a `Block3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block3dSnapshot, store::TextError> {
    <Block3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block3dSnapshot` back to `.block3d` DSL text.
pub fn print_dsl(document: &Block3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
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
        definition.representations.push(BlockRepresentation {
            id: "r1".into(),
            name: "1:500".into(),
            mesh_url: Some("/mesh/capsule_J.1to500.glb".into()),
            tags: vec!["1to500".into()],
            lod: Some("low".into()),
            description: String::new(),
            attributes: Vec::new(),
        });
        crate::artifacts::block3d::set_vortex_kinds(&mut definition, vec![Block3dVortexKind { id: "door".into(), name: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() }]);
        definition.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [0.0, -1.6, 1.2], direction: [0.0, -1.0, 0.0], radius: 0.3, label: Some("door".into()) });
        definition
    }

    #[test]
    fn block3d_definition_dsl_round_trips() {
        let empty = Block3dSnapshot::default();
        store::os_store::test_support::assert_dsl_round_trip(&empty);
        store::os_store::test_support::assert_dsl_pack_equivalence(&empty);
        let definition = nakagin_capsule();
        store::os_store::test_support::assert_dsl_round_trip(&definition);
        store::os_store::test_support::assert_dsl_pack_equivalence(&definition);
    }

    #[test]
    fn block3d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT, BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT] {
            let definition = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::os_store::test_support::assert_dsl_round_trip(&definition);
        }
    }
}
//#endregion 🧪️Tests

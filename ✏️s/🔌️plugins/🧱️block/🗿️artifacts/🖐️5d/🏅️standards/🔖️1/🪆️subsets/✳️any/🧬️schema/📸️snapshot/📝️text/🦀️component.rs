//! 📜️ Block 5D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::block5d::Block5dSnapshot;

/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block5d` DSL —
/// the `PartKind` slice of `s/plugin/puzzle/app/5d/example/🧩️concrete-forest.puzzle5d`.
pub const BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️hexagonal-cut-concrete-forest-left/🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio");
/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block5d` DSL.
pub const BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️nakagin-capsule/🖼️assets/🗣️nakagin-capsule.dsl.semio");

/// 📖️ Parses `.block5d` DSL text into a `Block5dSnapshot`.
pub async fn parse_dsl(text: &str) -> Result<Block5dSnapshot, store::TextError> {
    <Block5dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block5dSnapshot` back to `.block5d` DSL text.
pub async fn print_dsl(document: &Block5dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block5d::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d};
    use crate::{BlockCamera2d, BlockCamera3d, BlockKindIdentity, BlockRepresentation};

    pub async fn hexagonal_cut_concrete_forest_left() -> Block5dSnapshot {
        let mut definition = Block5dSnapshot {
            part_kind: BlockKindIdentity { id: "Hexagonal Cut Concrete Forest Left".into(), name: "Hexagonal Cut Concrete Forest Left".into(), label: "Hexagonal Cut Concrete Forest Left".into(), ..Default::default() },
            part_2d: Block5dPart2d { shape: Some("circle".into()), radius: Some(20.0), ..Default::default() },
            part_3d: Block5dPart3d { orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None },
            camera2d: BlockCamera2d { x: 230.7, y: 93.5, zoom: 2.0 },
            camera3d: BlockCamera3d { position: [30.0, -30.0, 20.0], target: [7.0, 0.0, 3.0], zoom: 3.0 },
            ..Block5dSnapshot::default()
        };
        definition.representations.push(BlockRepresentation {
            id: "r0".into(),
            name: "default".into(),
            mesh_url: Some("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb".into()),
            tags: Vec::new(),
            lod: None,
            description: String::new(),
            attributes: Vec::new(),
        });
        definition.grip_kinds.push(Block5dGripKind { id: "b-l".into(), name: "b-l".into(), label: "b-l".into(), color: "hsl(206 52% 48%)".into(), default_rope_kind: "rope.link".into() });
        definition.grips.push(Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -0.1, radius_2d: 3.0, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 });
        definition
    }

    #[semio_framework_async_macros::async_test]
    async fn block5d_definition_dsl_round_trips() {
        let empty = Block5dSnapshot::default();
        store::os_store::test_support::assert_dsl_round_trip(&empty);
        store::os_store::test_support::assert_dsl_pack_equivalence(&empty);
        let definition = hexagonal_cut_concrete_forest_left();
        store::os_store::test_support::assert_dsl_round_trip(&definition);
        store::os_store::test_support::assert_dsl_pack_equivalence(&definition);
    }

    #[semio_framework_async_macros::async_test]
    async fn block5d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT, BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT] {
            let definition = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::os_store::test_support::assert_dsl_round_trip(&definition);
        }
    }
}
//#endregion 🧪️Tests

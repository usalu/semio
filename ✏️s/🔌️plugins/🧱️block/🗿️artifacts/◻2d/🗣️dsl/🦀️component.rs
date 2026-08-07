//! 📜️ Block 2D artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block2d::Block2dDefinition;

/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block2d` DSL —
/// the `NodeKind` half of `s/plugin/puzzle/app/2d/manifest/🛂️manifest.jsonconcrete-forest.manifest.json`.
pub const BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../📚️examples/hexagonal-cut-concrete-forest-left/🗣️dsls/♻️reuse/🧬️component.block.block2d.dsl.semio");
/// 📄️ The `hexagonal-cut-concrete-forest-right` example fixture, handcrafted in the `.block2d` DSL.
pub const BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT: &str = include_str!("../📚️examples/hexagonal-cut-concrete-forest-right/🗣️dsls/♻️reuse/🧬️component.block.block2d.dsl.semio");

/// 📖️ Parses `.block2d` DSL text into a `Block2dDefinition`.
pub fn parse_dsl(text: &str) -> Result<Block2dDefinition, store::TextError> {
    <Block2dDefinition as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block2dDefinition` back to `.block2d` DSL text.
pub fn print_dsl(document: &Block2dDefinition) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::{Block2dHandleKind, Block2dHandleTemplate};
    use crate::core::{BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity};

    /// 🌱️ Builds the `Hexagonal Cut Concrete Forest Left` node kind — the representative fixture also
    /// used to (re)generate the handcrafted example file via `print_dsl`.
    // 🧯️ `approx_constant`: the first angle is genuinely `-π/2` to enough digits that clippy flags it
    // against `f64::consts::FRAC_PI_2` — these are handcrafted fixture values copied verbatim from the
    // real puzzle example file, not meant to reference the named constant.
    #[allow(clippy::approx_constant)]
    pub fn hexagonal_cut_concrete_forest_left() -> Block2dDefinition {
        let mut definition = Block2dDefinition {
            node_kind: BlockKindIdentity { id: "Hexagonal Cut Concrete Forest Left".into(), name: "Hexagonal Cut Concrete Forest Left".into(), label: "Hexagonal Cut Concrete Forest Left".into(), ..Default::default() },
            camera2d: BlockCamera2d { x: 230.7, y: 93.5, zoom: 2.0 },
            ..Block2dDefinition::default()
        };
        for (id, name, color) in
            [("b-l", "b-l", "hsl(206 52% 48%)"), ("b-l-m", "b-l-m", "hsl(290 52% 48%)"), ("b-s", "b-s", "hsl(55 52% 48%)"), ("b-s-m", "b-s-m", "hsl(124 52% 48%)"), ("c-b", "c-b", "hsl(37 52% 48%)"), ("c-t", "c-t", "hsl(169 52% 48%)")]
        {
            definition.handle_kinds.push(Block2dHandleKind { id: id.into(), name: name.into(), label: name.into(), color: color.into(), default_wire_kind: "cable.link".into() });
        }
        let angles: [(&str, f64); 11] = [
            ("b-l", -1.5707963267948966),
            ("b-l-m", -0.9995976625058433),
            ("b-l", -0.42839899821678995),
            ("b-s-m", 0.14279966607226324),
            ("b-s", 0.7139983303613167),
            ("b-s-m", 1.28519699465037),
            ("b-s", 1.856395658939423),
            ("c-b", 2.4275943232284765),
            ("c-t", 2.99879298751753),
            ("c-b", 3.569991651806583),
            ("c-t", 4.141190316095637),
        ];
        for (index, (handle_kind, angle)) in angles.into_iter().enumerate() {
            definition.handles.push(Block2dHandleTemplate { id: format!("h{index}"), handle_kind: handle_kind.into(), angle, radius: 0.36 });
        }
        definition.compatibility.push(BlockCompatibilityRule { id: "compat0".into(), source: "b-l".into(), target: "b-l".into(), bidirectional: true });
        definition
    }

    #[test]
    fn block2d_definition_dsl_round_trips() {
        let empty = Block2dDefinition::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let definition = hexagonal_cut_concrete_forest_left();
        store::test_support::assert_dsl_round_trip(&definition);
        store::test_support::assert_dsl_pack_equivalence(&definition);
    }

    #[test]
    fn block2d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT, BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT] {
            let definition = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::test_support::assert_dsl_round_trip(&definition);
        }
    }
}
//#endregion 🧪️Tests

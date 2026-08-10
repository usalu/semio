//! 🗣️ Puzzle 2d artifact — the textual `.puzzle2d` document grammar surface and its laws, plus the
//! two handcrafted example fixtures the play app's example picker loads.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle2d` DSL.
pub const PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle2d` DSL.
pub const PUZZLE2D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio");

/// 📖️ Parses `.puzzle2d` DSL text into a `Puzzle2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Puzzle2dSnapshot, store::TextError> {
    <Puzzle2dSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle2dSnapshot` back to `.puzzle2d` DSL text.
pub fn print_dsl(document: &Puzzle2dSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle2d::{Puzzle2dCamera, Puzzle2dCompatSpecificity, Puzzle2dEdge, Puzzle2dHandle, Puzzle2dKindCompatibility, Puzzle2dMeta, Puzzle2dNode};

    /// 📜️ Both real example fixtures (migrated from the legacy `.2d.json` shape — see ticket
    /// 🎫️convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle2d` DSL text and round-trip
    /// through `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle2d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT, PUZZLE2D_NAKAGIN_EXAMPLE_TEXT] {
            let projection = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
            semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
        }
    }

    #[test]
    fn puzzle2d_projection_dsl_round_trips() {
        let empty = Puzzle2dSnapshot::default();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&empty);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut with_content = Puzzle2dSnapshot { camera: Puzzle2dCamera { x: 12.0, y: -4.0, zoom: 2.5 }, ..Puzzle2dSnapshot::default() };
        with_content.nodes.push(Puzzle2dNode {
            id: "n1".into(),
            node_kind: Some("seed".into()),
            shape: Some("circle".into()),
            x: 1.0,
            y: 2.0,
            radius: Some(24.0),
            width: None,
            height: None,
            text: Some("Seed".into()),
            icon_kind: None,
            root: None,
            scale: None,
            visible: None,
            locked: None,
            anchor: Default::default(),
            handles: vec![Puzzle2dHandle { id: "n1:v0".into(), handle_kind: Some("b-l".into()), angle: 0.0, radius: Some(3.0), color: None, icon_kind: None, scale: None, visible: None, locked: None }],
        });
        with_content.nodes.push(Puzzle2dNode {
            id: "n2".into(),
            node_kind: Some("seed".into()),
            shape: Some("rectangle".into()),
            x: 30.0,
            y: 40.0,
            radius: None,
            width: Some(48.0),
            height: Some(24.0),
            text: None,
            icon_kind: Some("door".into()),
            root: Some(true),
            scale: Some(1.5),
            visible: Some(false),
            locked: Some(true),
            anchor: Default::default(),
            handles: Vec::new(),
        });
        with_content.edges.push(Puzzle2dEdge { id: "e1".into(), source: "n1:v0".into(), target: "n2".into(), edge_kind: Some("edge.link".into()), source_tip: None, target_tip: Some("arrow".into()), visible: None, locked: None, ..Default::default() });
        with_content.meta = Puzzle2dMeta {
            manifest_id: Some("concrete-forest".into()),
            kind_compatibility: vec![Puzzle2dKindCompatibility { source: "b-l".into(), target: "b-l".into(), bidirectional: true, important: false, specificity: Puzzle2dCompatSpecificity::Vortex }],
            kind_catalogs: None,
        };
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&with_content);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&with_content);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle2dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle2d::op::Puzzle2dMutation;
        use crate::artifacts::puzzle2d::spr::Puzzle2dStore;
        use crate::artifacts::puzzle2d::PUZZLE_2D_SCHEMA;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None));
        let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
        store.dispatch(DocumentCommand::Apply { mutations: vec![Puzzle2dMutation::SetNode { index: 0, node }], description: None }).expect("apply");
        let edit: &Edit<Puzzle2dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests

    #[test]
    fn puzzle2d_dsl_parses_edge_with_all_connection_params() {
        let text = r#"semio puzzle.puzzle2d.dsl v1
schema=puzzle.2d.fixture
camera {
  x=0 y=0 zoom=1
}
meta {
}
nodes [id:TEXT x:NUM y:NUM] {
  n1 0 0
  n2 10 0
}
edges [id:TEXT source:TEXT target:TEXT gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM x:NUM y:NUM] {
  e1 n1 n2 1 2 3 10 20 30 4 5
}
"#;
        let snapshot = parse_dsl(text).expect("minimal edge with 8 params parses");
        assert_eq!(snapshot.edges.len(), 1);
        let edge = &snapshot.edges[0];
        assert_eq!(edge.gap, 1.0);
        assert_eq!(edge.shift, 2.0);
        assert_eq!(edge.rise, 3.0);
        assert_eq!(edge.rotation, 10.0);
        assert_eq!(edge.turn, 20.0);
        assert_eq!(edge.tilt, 30.0);
        assert_eq!(edge.x, 4.0);
        assert_eq!(edge.y, 5.0);
    }
}
//#endregion 🧪️Tests

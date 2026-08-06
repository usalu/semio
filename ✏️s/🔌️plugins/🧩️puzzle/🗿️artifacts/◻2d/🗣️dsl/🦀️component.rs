//! 🗣️ Puzzle 2d artifact — the textual `.puzzle2d` document grammar surface and its laws, plus the
//! two handcrafted example fixtures the play app's example picker loads.

use crate::artifacts::puzzle2d::Puzzle2dProjection;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle2d` DSL.
pub const PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.puzzle.puzzle2d.dsl.semio");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle2d` DSL.
pub const PUZZLE2D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.puzzle.puzzle2d.dsl.semio");

/// 📖️ Parses `.puzzle2d` DSL text into a `Puzzle2dProjection`.
pub fn parse_dsl(text: &str) -> Result<Puzzle2dProjection, store::TextError> {
    <Puzzle2dProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle2dProjection` back to `.puzzle2d` DSL text.
pub fn print_dsl(document: &Puzzle2dProjection) -> String {
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
            store::test_support::assert_dsl_round_trip(&projection);
            store::test_support::assert_dsl_pack_equivalence(&projection);
        }
    }

    #[test]
    fn puzzle2d_projection_dsl_round_trips() {
        let empty = Puzzle2dProjection::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut with_content = Puzzle2dProjection { camera: Puzzle2dCamera { x: 12.0, y: -4.0, zoom: 2.5 }, ..Puzzle2dProjection::default() };
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
            handles: Vec::new(),
        });
        with_content.edges.push(Puzzle2dEdge { id: "e1".into(), source: "n1:v0".into(), target: "n2".into(), edge_kind: Some("edge.link".into()), source_tip: None, target_tip: Some("arrow".into()), visible: None, locked: None });
        with_content.meta = Puzzle2dMeta {
            manifest_id: Some("concrete-forest".into()),
            kind_compatibility: vec![Puzzle2dKindCompatibility { bidirectional: true, specificity: Puzzle2dCompatSpecificity::Vortex, source: "b-l".into(), target: "b-l".into() }],
            kind_catalogs: None,
        };
        store::test_support::assert_dsl_round_trip(&with_content);
        store::test_support::assert_dsl_pack_equivalence(&with_content);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle2dOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle2d::op::Puzzle2dOperation;
        use crate::artifacts::puzzle2d::spr::Puzzle2dStore;
        use crate::artifacts::puzzle2d::PUZZLE_2D_SCHEMA;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dProjection::default(), None));
        let node = Puzzle2dNode { id: "n1".into(), node_kind: None, shape: None, x: 0.0, y: 0.0, radius: None, width: None, height: None, text: None, icon_kind: None, root: None, scale: None, visible: None, locked: None, handles: Vec::new() };
        store.dispatch(DocumentCommand::Apply { operations: vec![Puzzle2dOperation::SetNode { index: 0, node }], description: None }).expect("apply");
        let edit: &Edit<Puzzle2dOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<Puzzle2dProjection, Puzzle2dOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

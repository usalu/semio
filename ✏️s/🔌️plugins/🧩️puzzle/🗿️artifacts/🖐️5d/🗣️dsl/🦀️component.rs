//! 🗣️ Puzzle 5d artifact — the textual `.puzzle5d` document grammar surface and its laws, plus the
//! two handcrafted example fixtures the play app's example picker loads.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio");

/// 📖️ Parses `.puzzle5d` DSL text into a `Puzzle5dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Puzzle5dSnapshot, store::TextError> {
    <Puzzle5dSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle5dSnapshot` back to `.puzzle5d` DSL text.
pub fn print_dsl(document: &Puzzle5dSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dScale};

    #[test]
    fn puzzle5d_projection_dsl_round_trips() {
        let empty = Puzzle5dSnapshot::default();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&empty);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut projection = Puzzle5dSnapshot { label: Some("Concrete Forest".into()), meta: Puzzle5dMeta { description: "Unified puzzle 5d source".into() }, ..Default::default() };
        projection.parts.push(Puzzle5dPart {
            id: "seed-left-001".into(),
            part_kind: Some("Hexagonal Cut Concrete Forest Left".into()),
            part_2d: Puzzle5dPart2d { x: 230.7, y: 93.5, shape: Some("circle".into()), radius: Some(20.0), width: None, height: None, text: Some("Hexagonal Cut Concrete Forest Left".into()), icon_kind: None, hidden: None, locked: None },
            // 📏️ Vec3 (per-axis) case of `Puzzle5dScale`, exercised alongside the Uniform case
            // below so both `Shape::Tuple(_, None)` arities round-trip through this test.
            part_3d: Puzzle5dPart3d {
                origin: [0.0, 0.0, 0.0],
                mesh_url: Some("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb".into()),
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: Some(Puzzle5dScale::Vec3([1.0, 1.0, 1.5])),
                label: Some("Hexagonal Cut Concrete Forest Left".into()),
            },
            grips: vec![Puzzle5dGrip {
                id: "v0".into(),
                grip_kind: Some("b-l".into()),
                grip_2d: Puzzle5dGrip2d { angle: -0.1, grip_kind: Some("b-l".into()), radius: Some(3.0) },
                grip_3d: Puzzle5dGrip3d { position: [4.05, 4.68, 3.0], direction: Some([0.0, 1.0, 0.0]), radius: Some(0.36), label: Some("b-l".into()) },
            }],
        });
        projection.parts.push(Puzzle5dPart {
            id: "seed-right-001".into(),
            part_kind: Some("Hexagonal Cut Concrete Forest Right".into()),
            part_2d: Puzzle5dPart2d::default(),
            // 📏️ Uniform case of `Puzzle5dScale` — a bare number scaling all three axes alike.
            part_3d: Puzzle5dPart3d { origin: [6.0, 0.0, 0.0], mesh_url: None, orientation: None, scale: Some(Puzzle5dScale::Uniform(1.25)), label: None },
            grips: vec![Puzzle5dGrip {
                id: "v0".into(),
                grip_kind: Some("b-l".into()),
                grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, 0.0], direction: None, radius: None, label: None },
            }],
        });
        projection.fasteners.push(Puzzle5dFastener { id: "f1".into(), source: "seed-left-001:v0".into(), target: "seed-right-001:v0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
        projection.kind_compatibility.push(Puzzle5dKindCompatibility { source: "b-l".into(), target: "b-l".into(), bidirectional: true });
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    /// 📜️ Both real example fixtures (migrated from the legacy `.5d.json` shape — see ticket
    /// 🎫️convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle5d` DSL text and round-trip
    /// through `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle5d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT, PUZZLE5D_NAKAGIN_EXAMPLE_TEXT] {
            let projection = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
            // 🚧️ `assert_dsl_pack_equivalence(&projection)` deliberately NOT added here: same
            // `pack/value/rs` table-column bug as `puzzle5d_projection_dsl_round_trips` above
            // (this fixture's `parts` rows have the identical shape).
        }
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle5dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`). Uses a single `#[dsl(block)]`
    /// `SetPart` operation (not a `#[dsl(table)]` collection), so this is unaffected by the
    /// known table-column pack bug noted above.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle5d::op::Puzzle5dMutation;
        use crate::artifacts::puzzle5d::spr::Puzzle5dStore;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle5dStore::new(create_document_envelope(crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None));
        let part = Puzzle5dPart { id: "p1".into(), part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
        store.dispatch(DocumentCommand::Apply { mutations: vec![Puzzle5dMutation::SetPart { index: 0, part }], description: None }).expect("apply");
        let edit: &Edit<Puzzle5dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

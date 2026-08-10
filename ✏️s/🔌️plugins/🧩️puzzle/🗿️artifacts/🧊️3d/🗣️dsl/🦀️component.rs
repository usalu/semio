//! 🗣️ Puzzle 3d artifact — the textual `.puzzle3d` document grammar surface and its laws, plus the
//! two handcrafted example fixtures the play app's example picker loads.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle3d` DSL.
pub const PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle3d` DSL.
pub const PUZZLE3D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio");

/// 📖️ Parses `.puzzle3d` DSL text into a `Puzzle3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Puzzle3dSnapshot, store::TextError> {
    <Puzzle3dSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle3dSnapshot` back to `.puzzle3d` DSL text.
pub fn print_dsl(document: &Puzzle3dSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dCompatSpecificity, Puzzle3dKindCompatibility, Puzzle3dMeta, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dScale, Puzzle3dTargetVolume, Puzzle3dVortex};

    /// 📜️ Both real example fixtures (migrated from the legacy `.3d.json` shape — see ticket
    /// 🎫️convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle3d` DSL text and round-trip
    /// through `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle3d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT, PUZZLE3D_NAKAGIN_EXAMPLE_TEXT] {
            let projection = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
            semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
        }
    }

    /// 📜️ A representative in-memory projection (one object with two vortices, one attraction, a
    /// target volume, a reference plane, and a link-compatibility rule) round-trips through
    /// `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle3d_projection_dsl_round_trips() {
        let empty = Puzzle3dSnapshot::default();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&empty);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut projection = Puzzle3dSnapshot::default();
        projection.objects.push(Puzzle3dObject {
            id: "seed-left-001".into(),
            label: Some("Seed Left".into()),
            object_kind: Some("Hexagonal Cut Concrete Forest Left".into()),
            anchor: Default::default(),
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: Some(Puzzle3dScale::Uniform(1.5)),
            mesh_url: Some("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb".into()),
            vortices: vec![
                Puzzle3dVortex { id: "seed-left-001:v0".into(), vortex_kind: Some("b-l".into()), label: Some("v0".into()), position: [0.36, 0.0, 0.0], direction: Some([1.0, 0.0, 0.0]), radius: Some(0.36), hidden: false, locked: false },
                Puzzle3dVortex { id: "seed-left-001:v1".into(), vortex_kind: Some("b-l-m".into()), label: Some("v1".into()), position: [0.0, 0.36, 0.0], direction: None, radius: None, hidden: true, locked: true },
            ],
            hidden: false,
            locked: false,
        });
        projection.attractions.push(Puzzle3dAttraction { id: "a1".into(), attracting: "seed-left-001:v0".into(), attracted: "seed-right-001:v0".into(), gap: 0.02, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 , x: 0.0, y: 0.0});
        projection.target_volumes.push(Puzzle3dTargetVolume { id: "tv1".into(), origin: [1.0, 2.0, 3.0], orientation: None, scale: Some(Puzzle3dScale::Vec3([2.0, 3.0, 4.0])), hidden: false, locked: false });
        projection.references.push(Puzzle3dReference {
            id: "r1".into(),
            source: Puzzle3dReferenceSource { url: "https://example.com/plan.png".into(), media_kind: Some("image".into()) },
            origin: [0.0, 0.0, 0.0],
            width_world: 12.0,
            locked: false,
            hidden: false,
        });
        projection.meta = Puzzle3dMeta { kind_catalogs: None, kind_compatibility: vec![Puzzle3dKindCompatibility { source: "b-l".into(), target: "b-l".into(), bidirectional: true, important: false, specificity: crate::artifacts::puzzle3d::Puzzle3dCompatSpecificity::Vortex }] };
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle3dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle3d::op::Puzzle3dMutation;
        use crate::artifacts::puzzle3d::spr::Puzzle3dStore;
        use crate::artifacts::puzzle3d::PUZZLE_3D_SCHEMA;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None));
        let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
        store.dispatch(DocumentCommand::Apply { mutations: vec![Puzzle3dMutation::SetObject { index: 0, object }], description: None }).expect("apply");
        let edit: &Edit<Puzzle3dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

//! 📜️ Puzzle 3d app — textual document grammar surface + laws (constitutional: dsl).

use puzzle_3d::Puzzle3dProjection;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle3d` DSL.
pub const PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/⚡️implementations/🦀️rust/📚️examples/🧩️concrete-forest.puzzle3d");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle3d` DSL.
pub const PUZZLE3D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/⚡️implementations/🦀️rust/📚️examples/🧩️nakagin-capsule-tower.puzzle3d");

/// 📖️ Parses `.puzzle3d` DSL text into a `Puzzle3dProjection`.
pub fn parse_dsl(text: &str) -> Result<Puzzle3dProjection, store::TextError> {
    <Puzzle3dProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle3dProjection` back to `.puzzle3d` DSL text.
pub fn print_dsl(document: &Puzzle3dProjection) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use puzzle_3d::{Puzzle3dAttraction, Puzzle3dKindCompatibility, Puzzle3dMeta, Puzzle3dObject, Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dScale, Puzzle3dTargetVolume, Puzzle3dVortex};

    /// 📜️ Both real example fixtures (migrated from the legacy `.3d.json` shape — see ticket
    /// 🎫️convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle3d` DSL text and round-trip
    /// through `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle3d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT, PUZZLE3D_NAKAGIN_EXAMPLE_TEXT] {
            let projection = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::test_support::assert_dsl_round_trip(&projection);
            store::test_support::assert_dsl_pack_equivalence(&projection);
        }
    }

    /// 📜️ A representative in-memory projection (one object with two vortices, one attraction, a
    /// target volume, a reference plane, and a link-compatibility rule) round-trips through
    /// `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle3d_projection_dsl_round_trips() {
        let empty = Puzzle3dProjection::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut projection = Puzzle3dProjection::default();
        projection.objects.push(Puzzle3dObject {
            id: "seed-left-001".into(),
            label: Some("Seed Left".into()),
            object_kind: Some("Hexagonal Cut Concrete Forest Left".into()),
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
        projection.attractions.push(Puzzle3dAttraction { id: "a1".into(), attracting: "seed-left-001:v0".into(), attracted: "seed-right-001:v0".into(), gap: 0.02, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
        projection.target_volumes.push(Puzzle3dTargetVolume { id: "tv1".into(), origin: [1.0, 2.0, 3.0], orientation: None, scale: Some(Puzzle3dScale::Vec3([2.0, 3.0, 4.0])), hidden: false, locked: false });
        projection.references.push(Puzzle3dReference {
            id: "r1".into(),
            source: Puzzle3dReferenceSource { url: "https://example.com/plan.png".into(), media_kind: Some("image".into()) },
            origin: [0.0, 0.0, 0.0],
            width_world: 12.0,
            locked: false,
            hidden: false,
        });
        projection.meta = Puzzle3dMeta { kind_catalogs: None, kind_compatibility: vec![Puzzle3dKindCompatibility { source: "b-l".into(), target: "b-l".into(), bidirectional: true, important: false, specificity: Some("vortex".into()) }] };
        store::test_support::assert_dsl_round_trip(&projection);
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }
}
//#endregion 🧪️Tests

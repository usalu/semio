use crate::io::text::snapshot::*;
use {crate::Puzzle3dAttraction, crate::Puzzle3dKindCompatibility, crate::Puzzle3dMeta, crate::Puzzle3dObject, crate::Puzzle3dReference, crate::Puzzle3dReferenceSource, crate::Puzzle3dScale, crate::Puzzle3dTargetVolume, crate::Puzzle3dVortex};

/// 📜️ Both real example fixtures (migrated from the legacy `.3d.json` shape — see ticket
/// 🎫️convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle3d` DSL text and round-trip
/// through `print_dsl`/`parse_dsl` exactly.
#[test]
fn puzzle3d_example_fixtures_parse_and_round_trip_as_dsl() {
    for dsl_text in [std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio")).expect("committed example reads"), std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio")).expect("committed example reads")] {
        let projection = parse_dsl(&dsl_text).expect("example scene_snapshot parses as dsl");
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
    projection.attractions.push(Puzzle3dAttraction { id: "a1".into(), attracting: "seed-left-001:v0".into(), attracted: "seed-right-001:v0".into(), gap: 0.02, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    projection.target_volumes.push(Puzzle3dTargetVolume { id: "tv1".into(), origin: [1.0, 2.0, 3.0], orientation: None, scale: Some(Puzzle3dScale::Vec3([2.0, 3.0, 4.0])), hidden: false, locked: false });
    projection.references.push(Puzzle3dReference {
        id: "r1".into(),
        source: Puzzle3dReferenceSource { url: "https://example.com/plan.png".into(), media_kind: Some("image".into()) },
        origin: [0.0, 0.0, 0.0],
        width_world: 12.0,
        locked: false,
        hidden: false,
    });
    projection.meta =
        Puzzle3dMeta { kind_catalogs: None, kind_compatibility: vec![Puzzle3dKindCompatibility { source: "b-l".into(), target: "b-l".into(), bidirectional: true, important: false, specificity: crate::Puzzle3dCompatSpecificity::Vortex }] };
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
}

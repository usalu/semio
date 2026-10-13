use crate::primitives::{make_box, make_planar_face_from_points};
use semio_framework_3d_brep_representation::topology::{history, Body};
use semio_framework_3d_brep_representation::vector::Pnt3;

/// 🌱 Law A from the W3a-0 design: `Body::from_seed(&seed).to_seed() == seed`. Exercised against a
/// real closed solid (a box built exclusively through the checked euler editors) rather than a
/// hand-assembled fixture, so the seed under test has the same shape a real diff constructor's
/// extraction would produce.
#[semio_framework_async_macros::async_test]
async fn from_seed_round_trips_a_closed_box_through_to_seed() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();

    let seed = body.to_seed();
    assert_eq!(seed.vertices.len(), 8);
    assert_eq!(seed.edges.len(), 12);
    assert_eq!(seed.faces.len(), 6);
    assert_eq!(seed.shells.len(), 1);
    assert_eq!(seed.solids.len(), 1);

    let rebuilt = Body::from_seed(&seed);
    let round_tripped = rebuilt.to_seed();
    assert_eq!(seed, round_tripped, "Body::from_seed(seed).to_seed() must equal seed");
}

/// 🌱 The same law on a simpler, loop-free-of-holes single face — guards the `outer`/`inners`
/// index bookkeeping independently of a full closed solid's shell/solid wrapping.
#[semio_framework_async_macros::async_test]
async fn from_seed_round_trips_a_loose_planar_face() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], &mut rec).unwrap();

    let seed = body.to_seed();
    let rebuilt = Body::from_seed(&seed);
    assert_eq!(rebuilt.to_seed(), seed);
}

/// 🌱 `LabelSource` determinism (the frozen W1 seed contract, "from_seed(s) equals from_seed(s)
/// for byte-identical s"): rebuilding the same seed twice must not re-mint or collide labels.
#[semio_framework_async_macros::async_test]
async fn from_seed_is_deterministic_for_identical_seeds() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let seed = body.to_seed();

    let a = Body::from_seed(&seed);
    let b = Body::from_seed(&seed);
    assert_eq!(a.to_seed(), b.to_seed());
}

/// 🌱 The seed's `next_label` must survive `build`, not reset to 0 — otherwise two independent
/// diff-constructor calls against the same `base` mint colliding labels the instant they merge
/// (the exact defect §2 of the design flags for a `LabelSource` that restarts at 0 every build).
#[semio_framework_async_macros::async_test]
async fn from_seed_preserves_the_label_high_water_mark() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let seed = body.to_seed();
    assert!(seed.next_label > 0, "a box mints more than zero labels");

    let rebuilt = Body::from_seed(&seed);
    assert_eq!(rebuilt.labels.next(), seed.next_label);
}

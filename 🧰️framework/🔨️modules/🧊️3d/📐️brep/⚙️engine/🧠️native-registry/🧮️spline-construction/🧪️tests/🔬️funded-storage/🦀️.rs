//! 🔬️ Oracle vectors for the retained spline storage shared by solid publications.

use super::{nurbs_cuboid::{Payload, Recipe}, quadratic_arc::{ArcBuffers, SectionArc}};
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, matrix::Frame3};

fn vector(kind: &str) -> [f64; 14] {
    include_str!("🔣️.csv").lines().skip(1).find_map(|line| {
        let (key, data) = line.split_once(',')?;
        (key == kind).then(|| data.split(',').map(|field| field.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap())
    }).unwrap()
}

fn check_point(point: Pnt3, expected: [f64; 3]) {
    for (actual, expected) in point.to_array().into_iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-13);
    }
}

/// 🧊️ Each funded array precedes publication of the degree-one cuboid edge.
#[test]
fn cuboid_edge_preserves_funded_samples() {
    let [t, x0, y0, z0, x1, y1, z1, _, _, _, _, x, y, z] = vector("cuboid-edge");
    let mut payload = Payload::default();
    let mut steps = 0;
    while let Some(need) = payload.demand() {
        let progress = payload.prepare(Recipe::Edge {
            knots: [0., 0., 1., 1.],
            controls: [Pnt3::new(x0, y0, z0), Pnt3::new(x1, y1, z1)],
        }).unwrap();
        assert_eq!(progress.copied_bytes, need.copy_bytes);
        assert_eq!(progress.retained_capacity_bytes, need.capacity_bytes);
        steps += 1;
    }
    assert_eq!(steps, 4);
    let Payload::Edge(curve) = payload else { panic!("completed cuboid spline edge") };
    check_point(curve.eval(t), [x, y, z]);
}

/// 🪢️ The section arc restores its original owner and releases each funded array in turn.
#[test]
fn section_arc_round_trip_and_retirement_preserve_the_rational_sample() {
    let [t, x0, y0, _, x1, y1, _, x2, y2, _, weight, x, y, z] = vector("section-arc");
    let arc = SectionArc {
        controls: [Pnt2::new(x0, y0), Pnt2::new(x1, y1), Pnt2::new(x2, y2)],
        middle_weight: weight,
    };
    let mut buffers = ArcBuffers::default();
    let mut funded = 0;
    while buffers.next_bytes(true) > 0 {
        let need = buffers.next_bytes(true);
        let progress = buffers.prepare(arc, Frame3::WORLD, true).unwrap();
        assert_eq!(progress.copied_bytes, need);
        assert_eq!(progress.retained_capacity_bytes, need);
        funded += need;
    }
    let curve = buffers.curve();
    assert_eq!(buffers.release_bytes(), 0);
    check_point(curve.eval(t), [x, y, z]);
    buffers.restore_curve(curve);
    let mut released = 0;
    while buffers.release_bytes() > 0 {
        released += buffers.release_bytes();
        buffers.release();
    }
    assert_eq!(released, funded);
}

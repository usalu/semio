
use super::*;

#[test]
fn identity_rotation_leaves_vectors_unchanged() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(UnitQuaternion::identity().apply(v), v);
}

#[test]
fn rotation_between_parallel_is_identity() {
    let a = Vector3::new(1.0, 0.0, 0.0);
    assert_eq!(UnitQuaternion::rotation_between(a, a * 2.0), Some(UnitQuaternion::identity()));
}

#[test]
fn rotation_between_anti_parallel_is_none() {
    let a = Vector3::new(1.0, 0.0, 0.0);
    assert_eq!(UnitQuaternion::rotation_between(a, -a), None);
}

#[test]
fn rotation_between_quarter_turn_carries_from_onto_to() {
    let from = Vector3::new(1.0, 0.0, 0.0);
    let to = Vector3::new(0.0, 1.0, 0.0);
    let rotation = UnitQuaternion::rotation_between(from, to).expect("defined axis");
    let rotated = rotation.apply(from);
    assert!((rotated.x - to.x).abs() < 1e-5 && (rotated.y - to.y).abs() < 1e-5 && (rotated.z - to.z).abs() < 1e-5);
}

#[test]
fn isometry_inverse_round_trips_a_point() {
    let pose = Isometry3::from_parts(Vector3::new(3.0, -2.0, 5.0), UnitQuaternion::rotation_between(Vector3::new(0.0, 0.0, 1.0), Vector3::new(1.0, 0.0, 0.0)).expect("defined axis"));
    let point = Point3::new(1.0, 1.0, 1.0);
    let round_tripped = pose.inverse().transform_point(pose.transform_point(point));
    assert!((round_tripped.x - point.x).abs() < 1e-5 && (round_tripped.y - point.y).abs() < 1e-5 && (round_tripped.z - point.z).abs() < 1e-5);
}

#[test]
fn isometry_compose_matches_sequential_application() {
    let a = Isometry3::from_parts(Vector3::new(1.0, 0.0, 0.0), UnitQuaternion::identity());
    let b = Isometry3::from_parts(Vector3::new(0.0, 2.0, 0.0), UnitQuaternion::identity());
    let point = Point3::new(0.0, 0.0, 0.0);
    let composed = a.compose(b).transform_point(point);
    let sequential = a.transform_point(b.transform_point(point));
    assert_eq!(composed, sequential);
}

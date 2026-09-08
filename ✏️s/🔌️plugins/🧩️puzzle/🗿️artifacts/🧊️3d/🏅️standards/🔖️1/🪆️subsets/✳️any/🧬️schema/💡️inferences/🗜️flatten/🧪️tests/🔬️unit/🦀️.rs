
use super::*;
use crate::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dVortex};

fn vortex(id: &str, position: [f64; 3], direction: [f64; 3]) -> Puzzle3dVortex {
    Puzzle3dVortex { id: id.into(), vortex_kind: None, label: None, position, direction: Some(direction), radius: None, hidden: false, locked: false }
}

fn object(id: &str, origin: [f64; 3], vortices: Vec<Puzzle3dVortex>) -> Puzzle3dObject {
    Puzzle3dObject { id: id.into(), label: None, object_kind: None, anchor: Puzzle3dObjectAnchor::Fixed, origin, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, vortices, hidden: false, locked: false }
}

#[test]
fn fixed_root_keeps_stored_plane() {
    let objects = vec![object("a", [1.0, 2.0, 3.0], vec![])];
    let poses = flatten_objects(&objects, &[], None);
    let pose = poses.get("a").expect("a");
    assert_eq!(pose.plane.origin, [1.0, 2.0, 3.0]);
}

#[test]
fn derived_root_resets_plane_to_default() {
    let mut objects = vec![object("a", [1.0, 2.0, 3.0], vec![])];
    objects[0].anchor = Puzzle3dObjectAnchor::Derived;
    let poses = flatten_objects(&objects, &[], None);
    let pose = poses.get("a").expect("a");
    assert_eq!(pose.plane.origin, [0.0, 0.0, 0.0]);
}

#[test]
fn child_plane_is_deterministic_for_vertical_stack() {
    let parent = object("p", [0.0, 0.0, 0.0], vec![vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
    let mut child = object("c", [0.0, 0.0, 0.0], vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0])]);
    child.anchor = Puzzle3dObjectAnchor::Derived;
    let attraction = Puzzle3dAttraction { id: "a1".into(), attracting: "p:top".into(), attracted: "c:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 270.0, turn: 0.0, tilt: 0.0, x: 1.5, y: 2.5 };
    let poses = flatten_objects(&[parent, child], &[attraction], None);
    let child_pose = poses.get("c").expect("c");
    let parent_pose = poses.get("p").expect("p");
    assert_eq!(parent_pose.plane.origin, [0.0, 0.0, 0.0]);
    // Parent center at origin → compose circle rule with t=0 → (0, DIAGRAM_RADIUS).
    assert_eq!(child_pose.center, [0.0, DIAGRAM_RADIUS]);
    assert!(child_pose.plane.origin[2].is_finite());
}

#[test]
fn identity_orientation_from_default_plane() {
    let q = plane_to_orientation(FlattenPlane::default());
    assert!((q[0]).abs() < 1e-9 && (q[1]).abs() < 1e-9 && (q[2]).abs() < 1e-9);
    assert!((q[3] - 1.0).abs() < 1e-9);
}

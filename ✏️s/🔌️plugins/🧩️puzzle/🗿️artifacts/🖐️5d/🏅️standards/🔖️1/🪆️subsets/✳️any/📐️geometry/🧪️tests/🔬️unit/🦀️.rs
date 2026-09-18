//! 📐️ The 5D flatten solver against the numbers the React target produces — ticket
//! 26/09/17/WGPU-RENDERER-REACT-PARITY packet W2k. The load-bearing oracle is React's own single
//! test (`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧪️compose5d-preparetopologymodel/🟦️.tsx:9`): a `fixed` root
//! at `[1,2,3]` with a `derived` child fastened through one z-facing grip pair, fastener `x: 0.5`,
//! `y: 0.25` — the child must land at world x ≈ 1 and diagram x ≈ `1.5 × 48`.

use super::*;
use crate::{Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dPart2d, Puzzle5dPart3d};

fn grip(id: &str, position: [f64; 3], direction: [f64; 3], angle: f64) -> Puzzle5dGrip {
    Puzzle5dGrip { id: id.to_string(), grip_kind: None, grip_2d: Puzzle5dGrip2d { angle, grip_kind: None, radius: None }, grip_3d: Puzzle5dGrip3d { position, direction: Some(direction), radius: None, label: None } }
}

fn part(id: &str, anchor: Puzzle5dPartAnchor, x: f64, y: f64, origin: [f64; 3], grips: Vec<Puzzle5dGrip>) -> Puzzle5dPart {
    Puzzle5dPart {
        id: id.to_string(),
        part_kind: None,
        anchor,
        part_2d: Puzzle5dPart2d { x, y, ..Puzzle5dPart2d::default() },
        part_3d: Puzzle5dPart3d { origin, orientation: Some([0.0, 0.0, 0.0, 1.0]), ..Puzzle5dPart3d::default() },
        grips,
    }
}

fn fastener(id: &str, source: &str, target: &str, x: f64, y: f64) -> Puzzle5dFastener {
    Puzzle5dFastener { id: id.to_string(), source: source.to_string(), target: target.to_string(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x, y }
}

fn document(parts: Vec<Puzzle5dPart>, fasteners: Vec<Puzzle5dFastener>) -> Puzzle5dSnapshot {
    Puzzle5dSnapshot { schema: crate::PUZZLE_5D_SCHEMA.to_string(), domain: "architecture".to_string(), parts, fasteners, ..Puzzle5dSnapshot::default() }
}

fn react_oracle_document() -> Puzzle5dSnapshot {
    document(
        vec![
            part("root", Puzzle5dPartAnchor::Fixed, 1.0, 2.0, [1.0, 2.0, 3.0], vec![grip("conn-a", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
            part("child", Puzzle5dPartAnchor::Derived, 0.0, 0.0, [0.0, 0.0, 0.0], vec![grip("conn-a", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
        ],
        vec![fastener("link-1", "root:conn-a", "child:conn-a", 0.5, 0.25)],
    )
}

#[test]
fn a_fixed_root_keeps_its_authored_frame_and_a_derived_child_is_posed_from_it() {
    let poses = flatten_poses(&react_oracle_document());
    let root = poses.get("root").expect("the fixed root is posed");
    assert_eq!(root.plane.origin, [1.0, 2.0, 3.0], "a Fixed root keeps its authored origin");
    let child = poses.get("child").expect("the derived child is posed");
    assert!((child.plane.origin[0] - 1.0).abs() < 0.01, "React's own oracle pins the child's world x at ~1, got {}", child.plane.origin[0]);
}

#[test]
fn the_topology_diagram_centre_scales_into_board_pixels_exactly_as_react_scales_it() {
    let document = react_oracle_document();
    let flat = flatten_poses(&document);
    let prepared = prepare_topology_poses(&document);
    let child_flat = flat.get("child").expect("the child is posed").center;
    let child_prepared = prepared.get("child").expect("the child is posed").center;
    assert!((child_prepared[0] - 1.5 * PUZZLE_5D_TOPOLOGY_ICON_WIDTH).abs() < 0.01, "React's oracle pins the child's diagram x at 1.5 × 48, got {}", child_prepared[0]);
    assert_eq!(child_prepared, [child_flat[0] * PUZZLE_5D_TOPOLOGY_ICON_WIDTH, child_flat[1] * PUZZLE_5D_TOPOLOGY_ICON_WIDTH]);
    assert_eq!(prepared.get("root").expect("the root is posed").plane, flat.get("root").expect("the root is posed").plane, "scaling touches only the diagram half");
}

#[test]
fn a_derived_root_is_reset_to_the_identity_plane_while_a_fixed_one_is_not() {
    let derived_only = document(vec![part("solo", Puzzle5dPartAnchor::Derived, 4.0, 5.0, [9.0, 9.0, 9.0], vec![])], Vec::new());
    let poses = flatten_poses(&derived_only);
    let solo = poses.get("solo").expect("an unfastened part still gets a pose");
    assert_eq!(solo.plane, FlattenPlane::IDENTITY, "React resets a derived root's plane and ignores its authored origin");
    assert_eq!(solo.center, [4.0, 5.0], "the authored board position still seeds the diagram centre");
    let fixed_only = document(vec![part("solo", Puzzle5dPartAnchor::Fixed, 4.0, 5.0, [9.0, 9.0, 9.0], vec![])], Vec::new());
    assert_eq!(flatten_poses(&fixed_only).get("solo").expect("posed").plane.origin, [9.0, 9.0, 9.0]);
}

#[test]
fn every_fixed_part_roots_the_walk_before_any_derived_island_does() {
    let mut document = document(
        vec![
            part("derived-island", Puzzle5dPartAnchor::Derived, 0.0, 0.0, [7.0, 7.0, 7.0], vec![]),
            part("anchor", Puzzle5dPartAnchor::Fixed, 0.0, 0.0, [1.0, 0.0, 0.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
            part("attached", Puzzle5dPartAnchor::Derived, 0.0, 0.0, [0.0, 0.0, 0.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
        ],
        vec![fastener("f", "anchor:g", "attached:g", 0.0, 0.0)],
    );
    document.parts.reverse();
    let poses = flatten_poses(&document);
    assert_eq!(poses.len(), 3, "every part is posed, island or not");
    assert_eq!(poses.get("derived-island").expect("posed").plane, FlattenPlane::IDENTITY);
    assert!((poses.get("attached").expect("posed").plane.origin[0] - 1.0).abs() < 0.01, "the attached child is solved off the FIXED anchor, whatever the declaration order");
}

#[test]
fn an_unaddressable_endpoint_or_a_missing_grip_degrades_that_one_hop_and_never_drops_a_part() {
    let no_colon = document(
        vec![
            part("a", Puzzle5dPartAnchor::Fixed, 0.0, 0.0, [1.0, 1.0, 1.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
            part("b", Puzzle5dPartAnchor::Derived, 0.0, 0.0, [0.0, 0.0, 0.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
        ],
        vec![fastener("f", "a", "b", 0.0, 0.0)],
    );
    let poses = flatten_poses(&no_colon);
    assert_eq!(poses.len(), 2, "an unaddressable fastener drops the edge, not the parts");
    assert_eq!(poses.get("b").expect("posed").plane, FlattenPlane::IDENTITY);

    let missing_grip = document(
        vec![
            part("a", Puzzle5dPartAnchor::Fixed, 0.0, 0.0, [1.0, 1.0, 1.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
            part("b", Puzzle5dPartAnchor::Derived, 0.0, 0.0, [0.0, 0.0, 0.0], vec![]),
        ],
        vec![fastener("f", "a:g", "b:nope", 0.0, 0.0)],
    );
    let degraded = flatten_poses(&missing_grip);
    assert_eq!(degraded.get("b").expect("posed").plane, FlattenPlane::IDENTITY);
    assert_eq!(degraded.get("b").expect("posed").center, [0.0, 0.0], "React sends a degraded hop to the diagram origin");
}

#[test]
fn a_fastener_is_walked_undirected_so_a_child_declared_as_the_source_is_still_solved() {
    let reversed = document(
        vec![
            part("anchor", Puzzle5dPartAnchor::Fixed, 0.0, 0.0, [2.0, 0.0, 0.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
            part("leaf", Puzzle5dPartAnchor::Derived, 0.0, 0.0, [0.0, 0.0, 0.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
        ],
        vec![fastener("f", "leaf:g", "anchor:g", 0.0, 0.0)],
    );
    let poses = flatten_poses(&reversed);
    assert!((poses.get("leaf").expect("posed").plane.origin[0] - 2.0).abs() < 0.01, "the fixed end is the parent regardless of which endpoint field it sits in");
}

#[test]
fn an_empty_document_solves_to_no_poses_at_all() {
    assert!(flatten_poses(&document(Vec::new(), Vec::new())).is_empty());
    assert!(prepare_topology_poses(&document(Vec::new(), Vec::new())).is_empty());
}

#[test]
fn the_diagram_seeds_a_first_child_on_reacts_own_circle_by_the_grips_rim_parameter() {
    let quarter_turn = std::f64::consts::PI / 2.0;
    let seeded = document(
        vec![
            part("root", Puzzle5dPartAnchor::Fixed, 0.0, 0.0, [0.0, 0.0, 0.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], quarter_turn)]),
            part("child", Puzzle5dPartAnchor::Derived, 0.0, 0.0, [0.0, 0.0, 0.0], vec![grip("g", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0)]),
        ],
        vec![fastener("f", "root:g", "child:g", 9.0, 9.0)],
    );
    let center = flatten_poses(&seeded).get("child").expect("posed").center;
    assert!((center[0] - 2.697).abs() < 0.001, "a parent still at the diagram origin ignores the fastener offset and uses DIAGRAM_RADIUS × sin(2πt), got {}", center[0]);
    assert!(center[1].abs() < 0.001, "…and DIAGRAM_RADIUS × cos(2πt), which is 0 at a quarter turn");
}

#[test]
fn a_collinear_grip_pair_takes_the_degenerate_alignment_branch_without_producing_nans() {
    let same_direction = Attraction::default();
    let plane = compute_child_plane(&FlattenPlane::IDENTITY, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, -1.0], &same_direction);
    assert!(plane.origin.iter().all(|value| value.is_finite()), "the antiparallel branch must not divide by a zero cross product");
    assert!(plane.x_axis.iter().chain(plane.y_axis.iter()).all(|value| value.is_finite()));
    let horizontal = compute_child_plane(&FlattenPlane::IDENTITY, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], &same_direction);
    assert!(horizontal.origin.iter().all(|value| value.is_finite()), "the |z| < TOLERANCE branch is the other degenerate case React carves out");
}

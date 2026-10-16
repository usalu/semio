//! 📦️ Unit suite of the clearance twin, boxes and places: the constants of the design, the box of a body around its feet, and the free places among obstacles.
//!
//! @see ../🧰️clearance-tools/🦀️.rs — the tools of the cases

use super::*;

#[test]
fn constants_are_the_starting_values_of_the_design() {
    assert_eq!([MARGIN, SEAM], [4.0, 1.0 / 64.0]);
    assert_eq!(FOREVER, (1_i64 << 32) - 1);
    assert_eq!((PUSHES, PATIENCE, DELAY), (4, 40, 32));
    assert_eq!([SLIDE_OFF_SPEED, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, LEAN, LANE_LIFT, SCOOT_HASTE], [32.0, 2.0, 128.0, 0.25, 0.9, 1.5]);
    assert_eq!(STEERINGS, [0.0, 30.0, -30.0, 60.0, -60.0, 90.0, -90.0, 130.0, -130.0, 180.0, -180.0, 240.0, -240.0]);
    assert_eq!(TALLY, Tally::default());
}

#[test]
fn the_box_is_built_around_the_feet_from_size_hover_posture_and_margin() {
    let size = Size { width: 40.0, height: 48.0 };
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, UPRIGHT, 4.0), body("a", 76.0, 248.0, 124.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 290.0), size, 10.0, UPRIGHT, 4.0).extent, rect(76.0, 238.0, 124.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, leaning(48.0, 1.0), 4.0).extent, rect(64.0, 248.0, 136.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, canopied(size, Size { width: 64.0, height: 28.0 }), 4.0).extent, rect(64.0, 220.0, 136.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, Posture { left: 3.0, right: 14.0, above: 0.0 }, 0.0).extent, rect(77.0, 252.0, 134.0, 300.0));
}

#[test]
fn a_leaning_body_widens_by_a_quarter_of_its_height_times_the_sine_and_a_canopied_one_by_the_overhang() {
    let size = Size { width: 40.0, height: 48.0 };
    assert_eq!(leaning(48.0, 1.0), Posture { left: 12.0, right: 12.0, above: 0.0 });
    assert_eq!(leaning(48.0, -0.5), Posture { left: 6.0, right: 6.0, above: 0.0 });
    assert_eq!(leaning(48.0, 0.0), UPRIGHT);
    assert_eq!(canopied(size, Size { width: 64.0, height: 28.0 }), Posture { left: 12.0, right: 12.0, above: 28.0 });
    assert_eq!(canopied(size, Size { width: 30.0, height: 20.0 }), Posture { left: 0.0, right: 0.0, above: 20.0 });
    assert_eq!(leaning(48.0, 1.0).left, LEAN * 48.0);
}

#[test]
fn neighbours_with_the_comfortable_gap_touch_without_overlapping() {
    let size = Size { width: 40.0, height: 48.0 };
    let small = Size { width: 30.0, height: 40.0 };
    let one = body_of("a", feet(100.0, 300.0), size, 0.0, UPRIGHT, MARGIN);
    let apart = body_of("b", feet(100.0 + (40.0 + 30.0) / 2.0 + 2.0 * MARGIN, 300.0), small, 0.0, UPRIGHT, MARGIN);
    let closer = body_of("b", feet(100.0 + (40.0 + 30.0) / 2.0 + 2.0 * MARGIN - 1.0 / 1024.0, 300.0), small, 0.0, UPRIGHT, MARGIN);
    assert_eq!(apart.extent.x0, one.extent.x1);
    assert_eq!(overlaps(&[one.clone(), apart]), Vec::new());
    assert_eq!(overlaps(&[one, closer]), vec![pair("a", "b")]);
}

#[test]
fn boxes_meet_only_where_open_boxes_intersect() {
    let one = rect(0.0, 0.0, 10.0, 10.0);
    assert!(!meets(one, rect(10.0, 0.0, 20.0, 10.0)));
    assert!(!meets(one, rect(0.0, 10.0, 10.0, 20.0)));
    assert!(meets(one, rect(9.999, 9.999, 20.0, 20.0)));
    assert!(meets(one, rect(2.0, 2.0, 3.0, 3.0)));
    assert!(meets(rect(2.0, 2.0, 3.0, 3.0), one));
    assert!(meets(one, rect(-5.0, 4.0, 15.0, 6.0)));
    assert!(!meets(one, rect(11.0, 0.0, 20.0, 10.0)));
    assert!(meets(one, one));
}

#[test]
fn boxes_move_grow_and_unite() {
    assert_eq!(shifted(rect(1.0, 2.0, 3.0, 4.0), 10.0, -1.0), rect(11.0, 1.0, 13.0, 3.0));
    assert_eq!(grown(rect(1.0, 2.0, 3.0, 4.0), 0.5), rect(0.5, 1.5, 3.5, 4.5));
    assert_eq!(united(rect(1.0, 2.0, 3.0, 4.0), rect(-1.0, 3.0, 2.0, 9.0)), rect(-1.0, 2.0, 3.0, 9.0));
}

#[test]
fn every_overlapping_pair_is_named_by_the_first_and_then_the_second_in_the_list() {
    let bodies = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 5.0, 5.0, 15.0, 15.0), body("c", 8.0, 8.0, 20.0, 20.0), body("d", 100.0, 100.0, 110.0, 110.0)];
    assert_eq!(overlaps(&bodies), vec![pair("a", "b"), pair("a", "c"), pair("b", "c")]);
    assert_eq!(overlaps(&[bodies[3].clone(), bodies[2].clone(), bodies[0].clone()]), vec![pair("c", "a")]);
    assert_eq!(overlaps(&[]), Vec::new());
    assert_eq!(overlaps(&bodies[..1]), Vec::new());
}

#[test]
fn near_misses_are_the_pairs_that_are_apart_but_closer_than_the_margin() {
    let bodies = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 11.0, 0.0, 20.0, 10.0), body("c", 5.0, 5.0, 8.0, 8.0), body("e", 11.0, 11.0, 20.0, 20.0)];
    assert_eq!(near_misses(&bodies, 2.0), vec![pair("a", "b"), pair("a", "e"), pair("b", "e")]);
    assert_eq!(near_misses(&bodies, 1.5), near_misses(&bodies, 2.0));
    assert_eq!(near_misses(&bodies, 1.0), Vec::new());
    assert_eq!(near_misses(&bodies, 0.0), Vec::new());
}

#[test]
fn overlaps_agree_with_the_area_of_the_intersection_on_generated_bodies() {
    overlaps_agree_with_areas(40);
}

/// 🧩️ The bodies and claims of the cases of free places.
fn free_scene() -> ([Body; 2], [Claim; 2]) {
    let bodies = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 20.0, 0.0, 30.0, 10.0)];
    let claims = [
        Claim { owner: "c".to_string(), slices: vec![Slice { from: 5, until: 5, extent: rect(40.0, 0.0, 50.0, 10.0) }, Slice { from: 6, until: 9, extent: rect(60.0, 0.0, 70.0, 10.0) }], rest: Some(rect(80.0, 0.0, 90.0, 10.0)) },
        Claim { owner: "a".to_string(), slices: vec![Slice { from: 5, until: 9, extent: rect(200.0, 0.0, 210.0, 10.0) }], rest: Some(rect(220.0, 0.0, 230.0, 10.0)) },
    ];
    (bodies, claims)
}

#[test]
fn obstacles_are_the_bodies_of_the_others_the_corridors_that_are_left_and_the_rests() {
    let (bodies, claims) = free_scene();
    assert_eq!(obstacles_of("a", &bodies, &claims, 5), vec![rect(20.0, 0.0, 30.0, 10.0), rect(40.0, 0.0, 50.0, 10.0), rect(60.0, 0.0, 70.0, 10.0), rect(80.0, 0.0, 90.0, 10.0)]);
    assert_eq!(obstacles_of("a", &bodies, &claims, 6), vec![rect(20.0, 0.0, 30.0, 10.0), rect(60.0, 0.0, 70.0, 10.0), rect(80.0, 0.0, 90.0, 10.0)]);
    assert_eq!(obstacles_of("a", &bodies, &claims, 10), vec![rect(20.0, 0.0, 30.0, 10.0), rect(80.0, 0.0, 90.0, 10.0)]);
    assert_eq!(obstacles_of("c", &bodies, &claims, 10), vec![rect(0.0, 0.0, 10.0, 10.0), rect(20.0, 0.0, 30.0, 10.0), rect(220.0, 0.0, 230.0, 10.0)]);
    assert_eq!(obstacles_of("nobody", &[], &[], 0), Vec::new());
}

#[test]
fn a_place_is_free_when_no_body_no_corridor_that_is_left_and_no_rest_of_another_owner_overlaps_it() {
    let (bodies, claims) = free_scene();
    assert!(free_at(rect(10.0, 0.0, 20.0, 10.0), "x", &bodies, &claims, 5));
    assert!(!free_at(rect(5.0, 0.0, 15.0, 10.0), "x", &bodies, &claims, 5));
    assert!(free_at(rect(5.0, 0.0, 15.0, 10.0), "a", &bodies, &claims, 5));
    assert!(!free_at(rect(45.0, 0.0, 55.0, 10.0), "x", &bodies, &claims, 5));
    assert!(free_at(rect(45.0, 0.0, 55.0, 10.0), "x", &bodies, &claims, 6));
    assert!(!free_at(rect(85.0, 0.0, 95.0, 10.0), "x", &bodies, &claims, 1000));
    assert!(free_at(rect(85.0, 0.0, 95.0, 10.0), "c", &bodies, &claims, 1000));
    assert!(free_among(rect(10.0, 0.0, 20.0, 10.0), &[rect(0.0, 0.0, 10.0, 10.0), rect(20.0, 0.0, 30.0, 10.0)]));
    assert!(!free_among(rect(10.0, 0.0, 20.5, 10.0), &[rect(0.0, 0.0, 10.0, 10.0), rect(20.0, 0.0, 30.0, 10.0)]));
}

#[test]
fn the_nearest_place_keeps_a_seam_from_every_obstacle_of_its_height() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 260.0, 150.0, 300.0)]), Some(90.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(60.0, 260.0, 90.0, 300.0)]), Some(110.0 + SEAM));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(90.0, 260.0, 110.0, 300.0)]), Some(70.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 100.0, 150.0, 200.0)]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 200.0, 150.0, 250.0)]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 300.0, 150.0, 350.0)]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 200.0, 150.0, 250.25)]), Some(90.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 150.0, 400.0, &[]), Some(150.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 60.0, &[]), Some(60.0));
}

#[test]
fn a_single_point_is_answered_when_that_is_all_there_is_and_nothing_without_room() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let obstacle = [rect(110.0, 260.0, 150.0, 300.0)];
    assert_eq!(slot_in(100.0, extent, 90.0 - SEAM, 160.0, &obstacle), Some(90.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 90.0, 110.0, &obstacle), None);
    assert_eq!(slot_in(100.0, extent, 90.0, 170.0, &obstacle), None);
    assert_eq!(slot_in(100.0, extent, 90.0, 170.0 + SEAM, &obstacle), Some(170.0 + SEAM));
    assert_eq!(slot_in(100.0, extent, 120.0, 110.0, &[]), None);
    assert_eq!(slot_in(100.0, extent, f64::NAN, 110.0, &[]), None);
}

#[test]
fn the_footprint_stays_on_the_perch_and_a_landing_looks_down_the_whole_column() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let perch = Perch { surface: "s".to_string(), x0: 50.0, x1: 300.0, y: 300.0 };
    assert_eq!(spot_on(&perch, 100.0, extent, 16.0, &[]), Some(100.0));
    assert_eq!(spot_on(&perch, 40.0, shifted(extent, -60.0, 0.0), 16.0, &[]), Some(66.0));
    assert_eq!(spot_on(&perch, 400.0, shifted(extent, 300.0, 0.0), 16.0, &[]), Some(284.0));
    assert_eq!(spot_on(&Perch { surface: "s".to_string(), x0: 50.0, x1: 70.0, y: 300.0 }, 60.0, shifted(extent, -40.0, 0.0), 16.0, &[]), None);
    assert_eq!(spot_on(&perch, 100.0, extent, 16.0, &[rect(110.0, 260.0, 150.0, 300.0)]), Some(90.0 - SEAM));
    let high = rect(80.0, 100.0, 120.0, 150.0);
    let below = [rect(110.0, 200.0, 150.0, 250.0)];
    assert_eq!(spot_on(&perch, 100.0, high, 16.0, &below), Some(100.0));
    assert_eq!(column_over(&perch, 100.0, high, 150.0, 16.0, &below), Some(90.0 - SEAM));
    assert_eq!(column_over(&perch, 100.0, high, 50.0, 16.0, &below), Some(100.0));
    assert_eq!(column_over(&perch, 100.0, high, -30.0, 16.0, &below), Some(100.0));
}

#[test]
fn free_places_agree_with_a_search_over_every_place_of_a_lattice() {
    slots_agree_with_the_lattice(40);
}

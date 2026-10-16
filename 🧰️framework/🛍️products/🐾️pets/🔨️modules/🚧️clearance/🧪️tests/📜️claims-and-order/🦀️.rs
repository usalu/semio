//! 📜️ Unit suite of the clearance twin, claims and order: planned paths cut into slices and compared tick by tick, and strides that keep walkers apart and in their order on a perch.
//!
//! @see ../🧰️clearance-tools/🦀️.rs — the tools of the cases

use super::*;

#[test]
fn a_path_is_cut_into_slices_of_a_span_each_the_smallest_box_around_its_ticks() {
    let path = marching(0.0, 0.0, 10.0, 1.0, 10);
    let exact = claim_of("a", 100, &path, 1, Some(path[9]));
    assert_eq!(exact.owner, "a");
    assert_eq!(exact.slices.len(), 10);
    assert_eq!(exact.slices[3], Slice { from: 103, until: 103, extent: path[3] });
    assert_eq!(exact.rest, Some(path[9]));
    let coarse = claim_of("a", 100, &path, 4, None);
    assert_eq!(coarse.slices, vec![Slice { from: 100, until: 103, extent: rect(0.0, 0.0, 70.0, 53.0) }, Slice { from: 104, until: 107, extent: rect(40.0, 4.0, 110.0, 57.0) }, Slice { from: 108, until: 109, extent: rect(80.0, 8.0, 130.0, 59.0) }]);
    assert_eq!(coarse.rest, None);
    assert_eq!(claim_of("a", 7, &path, 0, None).slices.len(), 10);
    assert_eq!(claim_of("a", 7, &path, -3, None).slices.len(), 10);
    assert_eq!(claim_of("a", 7, &path, 2, None).slices.len(), 5);
    assert_eq!(claim_of("a", 7, &[], 3, None).slices, Vec::new());
}

#[test]
fn a_claim_answers_where_it_has_its_owner_at_a_tick() {
    let path = marching(0.0, 0.0, 10.0, 0.0, 6);
    let claim = claim_of("a", 10, &path, 2, Some(rect(500.0, 0.0, 540.0, 50.0)));
    assert_eq!(slice_at(&claim, 9), None);
    assert_eq!(slice_at(&claim, 10), Some(rect(0.0, 0.0, 50.0, 50.0)));
    assert_eq!(slice_at(&claim, 13), Some(rect(20.0, 0.0, 70.0, 50.0)));
    assert_eq!(slice_at(&claim, 15), Some(rect(40.0, 0.0, 90.0, 50.0)));
    assert_eq!(slice_at(&claim, 16), Some(rect(500.0, 0.0, 540.0, 50.0)));
    assert_eq!(slice_at(&claim_of("a", 10, &path, 2, None), 16), None);
    assert_eq!(slice_at(&claim_of("a", 10, &[], 2, Some(rect(0.0, 0.0, 1.0, 1.0))), 16), None);
}

#[test]
fn a_plan_that_runs_into_a_body_that_stays_is_refused_and_the_owners_own_body_ignored() {
    let path = marching(0.0, 0.0, 10.0, 0.0, 20);
    let plan = claim_of("a", 0, &path, 1, Some(path[19]));
    assert!(claim_clear(&plan, &[body("a", 0.0, 0.0, 40.0, 50.0)], &[]));
    assert!(claim_clear(&plan, &[body("a", 0.0, 0.0, 40.0, 50.0), body("b", 100.0, 60.0, 140.0, 110.0)], &[]));
    assert!(!claim_clear(&plan, &[body("a", 0.0, 0.0, 40.0, 50.0), body("b", 100.0, 40.0, 140.0, 90.0)], &[]));
    assert!(claim_clear(&plan, &[body("b", 230.0, 0.0, 270.0, 50.0)], &[]));
    assert!(!claim_clear(&plan, &[body("b", 229.0, 0.0, 269.0, 50.0)], &[]));
}

#[test]
fn two_plans_may_cross_one_place_at_different_ticks_but_not_at_the_same_tick() {
    let across = claim_of("a", 20, &marching(0.0, 100.0, 10.0, 0.0, 21), 1, Some(rect(200.0, 100.0, 240.0, 150.0)));
    let down = |from: Ticks| claim_of("b", from, &marching(100.0, -100.0, 0.0, 10.0, 31), 1, Some(rect(100.0, 200.0, 140.0, 250.0)));
    let crossing = std::slice::from_ref(&across);
    assert!(claim_clear(&down(0), &[], crossing));
    assert!(!claim_clear(&down(10), &[], crossing));
    assert!(claim_clear(&down(30), &[], crossing));
    assert!(claim_clear(&across, &[], &[down(0)]));
    assert!(!claim_clear(&across, &[], &[down(10)]));
    assert!(claim_clear(&across, &[], &[down(30)]));
}

#[test]
fn a_rest_where_somebody_arrives_later_and_a_path_through_somebodys_rest_are_refused() {
    let late = claim_of("a", 0, &marching(0.0, 0.0, 10.0, 0.0, 51), 1, Some(rect(500.0, 0.0, 540.0, 50.0)));
    let early = claim_of("b", 0, &marching(500.0, -300.0, 0.0, 10.0, 31), 1, Some(rect(500.0, 0.0, 540.0, 50.0)));
    let arriving = std::slice::from_ref(&late);
    assert!(!claim_clear(&early, &[], arriving));
    assert!(claim_clear(&claim_of("b", 0, &marching(500.0, -300.0, 0.0, 10.0, 31), 1, None), &[], arriving));
    assert!(claim_clear(&claim_of("b", 0, &marching(560.0, -300.0, 0.0, 10.0, 31), 1, Some(rect(560.0, 0.0, 600.0, 50.0))), &[], arriving));
    let through = claim_of("b", 100, &marching(500.0, -300.0, 0.0, 10.0, 61), 1, Some(rect(500.0, 300.0, 540.0, 350.0)));
    assert!(!claim_clear(&through, &[], arriving));
    assert!(claim_clear(&through, &[], &[claim_of("a", 0, &marching(0.0, 0.0, 10.0, 0.0, 51), 1, None)]));
}

#[test]
fn a_planned_body_is_where_its_claim_says_and_where_it_stands_until_that_claim_begins() {
    let leaving = claim_of("a", 0, &marching(100.0, 0.0, 10.0, 0.0, 31), 1, Some(rect(400.0, 0.0, 440.0, 50.0)));
    let landing = claim_of("b", 0, &marching(100.0, -300.0, 0.0, 10.0, 31), 1, Some(rect(100.0, 0.0, 140.0, 50.0)));
    let standing = [body("a", 100.0, 0.0, 140.0, 50.0)];
    assert!(claim_clear(&landing, &standing, std::slice::from_ref(&leaving)));
    assert!(!claim_clear(&landing, &standing, &[]));
    let later = claim_of("a", 40, &marching(300.0, 0.0, 10.0, 0.0, 11), 1, Some(rect(400.0, 0.0, 440.0, 50.0)));
    assert!(!claim_clear(&landing, &standing, std::slice::from_ref(&later)));
    assert!(claim_clear(&landing, &[], std::slice::from_ref(&later)));
    assert!(claim_clear(&landing, &standing, &[claim_of("c", 0, &marching(900.0, 0.0, 0.0, 0.0, 5), 1, None), leaving.clone()]));
    assert!(!claim_clear(&landing, &standing, &[leaving, claim_of("a", 0, &marching(100.0, 0.0, 0.0, 0.0, 40), 1, None)]));
}

#[test]
fn claims_agree_with_a_comparison_of_every_tick_of_every_time_line() {
    claims_agree_with_time_lines(40);
}

#[test]
fn the_claims_of_an_owner_are_released_and_what_is_over_is_pruned() {
    let one = claim_of("a", 0, &marching(0.0, 0.0, 1.0, 0.0, 10), 5, Some(rect(9.0, 0.0, 49.0, 50.0)));
    let two = claim_of("b", 3, &marching(0.0, 0.0, 1.0, 0.0, 4), 2, None);
    let both = [one.clone(), two.clone()];
    assert_eq!(released(&both, "a"), vec![two.clone()]);
    assert_eq!(released(&both, "c"), both.to_vec());
    assert_eq!(pruned(&both, 0), both.to_vec());
    assert_eq!(pruned(&both, 5), vec![Claim { owner: "a".to_string(), slices: vec![one.slices[1]], rest: one.rest }, Claim { owner: "b".to_string(), slices: vec![two.slices[1]], rest: None }]);
    assert_eq!(pruned(&both, 7), vec![Claim { owner: "a".to_string(), slices: vec![one.slices[1]], rest: one.rest }]);
    assert_eq!(pruned(&both, 10), Vec::new());
}

#[test]
fn a_stride_is_cut_short_a_seam_before_the_first_obstacle_ahead_that_shares_its_height() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    assert_eq!(guarded_stride(extent, 5.0, &[]), 5.0);
    assert_eq!(guarded_stride(extent, -5.0, &[]), -5.0);
    assert_eq!(guarded_stride(extent, 0.0, &[rect(0.0, 0.0, 1000.0, 1000.0)]), 0.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(123.0, 260.0, 160.0, 300.0)]), 3.0 - SEAM);
    assert_eq!(guarded_stride(extent, 2.0, &[rect(123.0, 260.0, 160.0, 300.0)]), 2.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(120.01, 260.0, 160.0, 300.0)]), 0.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(40.0, 260.0, 77.0, 300.0)]), SEAM - 3.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(123.0, 260.0, 160.0, 300.0)]), -5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(40.0, 260.0, 77.0, 300.0)]), 5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(123.0, 100.0, 160.0, 250.0)]), 5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(123.0, 300.0, 160.0, 350.0)]), 5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(200.0, 260.0, 240.0, 300.0), rect(123.0, 260.0, 160.0, 300.0), rect(122.0, 100.0, 160.0, 200.0)]), 3.0 - SEAM);
}

#[test]
fn a_body_is_let_out_of_an_obstacle_it_overlaps_never_further_in() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let inside = [rect(110.0, 260.0, 150.0, 300.0)];
    assert_eq!(guarded_stride(extent, 5.0, &inside), 0.0);
    assert_eq!(guarded_stride(extent, -5.0, &inside), -5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(60.0, 260.0, 100.0, 300.0)]), 5.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(60.0, 260.0, 100.0, 300.0)]), 0.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(60.0, 260.0, 140.0, 300.0)]), 5.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(60.0, 260.0, 140.0, 300.0)]), -5.0);
}

#[test]
fn a_stride_stops_before_the_corridors_and_the_rests_of_claims_as_before_bodies() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let claims = [Claim { owner: "c".to_string(), slices: vec![Slice { from: 5, until: 5, extent: rect(122.0, 0.0, 160.0, 300.0) }, Slice { from: 6, until: 9, extent: rect(124.0, 0.0, 160.0, 300.0) }], rest: Some(rect(126.0, 250.0, 160.0, 300.0)) }];
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("a", &[], &claims, 5)), 2.0 - SEAM);
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("a", &[], &claims, 6)), 4.0 - SEAM);
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("a", &[], &claims, 10)), 6.0 - SEAM);
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("c", &[], &claims, 5)), 9.0);
}

#[test]
fn strides_agree_with_a_walk_along_a_lattice_that_stops_at_the_first_place_too_close() {
    strides_agree_with_the_lattice(40);
}

#[test]
fn walkers_on_one_line_stay_apart_and_in_their_order_which_nothing_but_the_guard_does() {
    walkers_kept_in_order(400);
}

#[test]
fn bodies_are_ranked_from_left_to_right_and_an_order_is_kept_or_not() {
    let bodies = [body("c", 50.0, 0.0, 60.0, 10.0), body("a", 0.0, 0.0, 10.0, 10.0), body("b", 20.0, 0.0, 40.0, 10.0), body("d", 25.0, 0.0, 35.0, 10.0)];
    assert_eq!(order_of(&bodies), slugs(&["a", "b", "d", "c"]));
    assert_eq!(order_of(&[]), Vec::<Slug>::new());
    assert!(order_kept(&slugs(&["a", "b", "c"]), &slugs(&["a", "b", "c"])));
    assert!(order_kept(&slugs(&["a", "b", "c"]), &slugs(&["a", "x", "c"])));
    assert!(!order_kept(&slugs(&["a", "b", "c"]), &slugs(&["c", "y", "a"])));
    assert!(!order_kept(&slugs(&["a", "b", "c"]), &slugs(&["b", "a"])));
    assert!(order_kept(&slugs(&["a", "b", "c"]), &[]));
    assert!(order_kept(&[], &slugs(&["a"])));
}

#[test]
fn a_swap_by_a_hop_needs_a_rise_that_lifts_the_hopper_over_its_neighbour() {
    let hopper = rect(80.0, 250.0, 120.0, 304.0);
    let hurdle = rect(130.0, 244.0, 170.0, 304.0);
    assert!(vaults(hopper, 60.0 + SEAM, hurdle));
    assert!(!vaults(hopper, 60.0, hurdle));
    assert!(!vaults(hopper, 84.0, rect(130.0, 219.0, 170.0, 304.0)));
    assert!(vaults(hopper, 84.0, rect(130.0, 221.0, 170.0, 304.0)));
}

#[test]
fn no_hop_over_a_neighbour_is_clear_whose_rise_does_not_vault_it() {
    let size = Size { width: 30.0, height: 40.0 };
    let from = feet(100.0, 300.0);
    let (mut vaulted, mut refused) = (0, 0);
    for dx in (60..=160).step_by(5).map(f64::from) {
        let to = feet(from.x + dx, 300.0);
        let Some(hop) = hop_of(from, to) else {
            continue;
        };
        let mut path = vec![body_of("h", from, size, 0.0, UPRIGHT, MARGIN).extent];
        let mut flight = Flight { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
        for left in (1..=hop.ticks).rev() {
            flight = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
            path.push(body_of("h", feet(flight.x, flight.y), size, 0.0, UPRIGHT, MARGIN).extent);
        }
        let plan = claim_of("h", 0, &path, 1, Some(path[path.len() - 1]));
        let rise = path[0].y1 - path.iter().map(|extent| extent.y1).fold(f64::INFINITY, f64::min);
        for height in (4..=60).step_by(4).map(f64::from) {
            for width in (4..=40).step_by(12).map(f64::from) {
                let hurdle = body_of("n", feet(from.x + dx / 2.0, 300.0), Size { width, height }, 0.0, UPRIGHT, MARGIN);
                if claim_clear(&plan, std::slice::from_ref(&hurdle), &[]) {
                    vaulted += 1;
                    assert!(vaults(path[0], rise, hurdle.extent), "dx {dx} height {height} width {width}");
                } else {
                    refused += 1;
                }
            }
        }
    }
    assert!(vaulted > 20);
    assert!(refused > 20);
}

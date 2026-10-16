//! 🪑️ Unit suite of the clearance twin, seating and heads: the nearest valid seating of pooled neighbours, landings on heads and slides off them, held bodies pushed out of obstacles and the last resort of a pet without a clear plan.
//!
//! @see ../🧰️clearance-tools/🦀️.rs — the tools of the cases

use super::*;

/// 🪑️ The perch of the cases of seating.
fn seating_perch() -> Perch {
    Perch { surface: "s".to_string(), x0: 100.0, x1: 400.0, y: 300.0 }
}

/// 💺️ The boxes after a seating, in the order of its seats.
fn seated(bodies: &[Body], seating: &Seating) -> Vec<Body> {
    seating.seats.iter().map(|seat| Body { owner: seat.owner.clone(), extent: shifted(bodies.iter().find(|entry| entry.owner == seat.owner).expect("a body").extent, seat.shift, 0.0) }).collect()
}

/// 🪧️ A seat by its owner and shift.
fn seat(owner: &str, shift: f64) -> Seat {
    Seat { owner: owner.to_string(), shift }
}

#[test]
fn a_valid_seating_is_left_exactly_as_it_is() {
    let bodies = [body("b", 220.0, 248.0, 268.0, 304.0), body("a", 150.0, 248.0, 198.0, 304.0), body("c", 268.0 + SEAM, 248.0, 300.0, 304.0)];
    assert_eq!(seat_of(&bodies, &seating_perch(), MARGIN), Seating { seats: vec![seat("a", 0.0), seat("b", 0.0), seat("c", 0.0)], leavers: Vec::new() });
    assert_eq!(seat_of(&[], &seating_perch(), MARGIN), Seating::default());
}

#[test]
fn two_bodies_that_overlap_move_apart_by_the_same_amount_a_seam_between_them() {
    let seating = seat_of(&[body("a", 150.0, 248.0, 198.0, 304.0), body("b", 190.0, 248.0, 238.0, 304.0)], &seating_perch(), MARGIN);
    assert_eq!(seating, Seating { seats: vec![seat("a", -(4.0 + SEAM / 2.0)), seat("b", 4.0 + SEAM / 2.0)], leavers: Vec::new() });
}

#[test]
fn a_whole_run_of_violators_is_pooled_into_one_block() {
    let bodies = [body("a", 200.0, 248.0, 240.0, 304.0), body("b", 210.0, 248.0, 250.0, 304.0), body("c", 220.0, 248.0, 260.0, 304.0), body("d", 300.0, 248.0, 340.0, 304.0)];
    let after = seated(&bodies, &seat_of(&bodies, &seating_perch(), MARGIN));
    assert_eq!(after.iter().map(|entry| entry.extent.x0).collect::<Vec<_>>(), vec![170.0 - SEAM, 210.0, 250.0 + SEAM, 300.0]);
}

#[test]
fn bodies_are_brought_back_onto_the_perch_their_margin_beyond_its_ends() {
    assert_eq!(seat_of(&[body("a", 60.0, 248.0, 108.0, 304.0)], &seating_perch(), MARGIN).seats, vec![seat("a", 36.0)]);
    assert_eq!(seat_of(&[body("a", 380.0, 248.0, 428.0, 304.0)], &seating_perch(), MARGIN).seats, vec![seat("a", -24.0)]);
    let bodies = [body("a", 300.0, 248.0, 348.0, 304.0), body("b", 352.0, 248.0, 400.0, 304.0)];
    let short = Perch { surface: "s".to_string(), x0: 100.0, x1: 380.0, y: 300.0 };
    assert_eq!(seat_of(&bodies, &short, MARGIN).seats, vec![seat("a", -(12.0 + SEAM)), seat("b", -16.0)]);
    assert_eq!(seated(&bodies, &seat_of(&bodies, &short, MARGIN))[1].extent.x1, short.x1 + MARGIN);
}

#[test]
fn whoever_does_not_fit_is_named_for_leaving_the_last_in_the_list_first() {
    let narrow = Perch { surface: "s".to_string(), x0: 100.0, x1: 200.0, y: 300.0 };
    let bodies = [body("c", 180.0, 248.0, 228.0, 304.0), body("a", 90.0, 248.0, 138.0, 304.0), body("b", 130.0, 248.0, 178.0, 304.0)];
    let seating = seat_of(&bodies, &narrow, MARGIN);
    assert_eq!(seating.leavers, slugs(&["b"]));
    assert_eq!(seating.seats.iter().map(|seat| seat.owner.clone()).collect::<Vec<_>>(), slugs(&["a", "c"]));
    assert_eq!(overlaps(&seated(&bodies, &seating)), Vec::new());
    assert_eq!(seat_of(&[body("a", 90.0, 248.0, 210.0, 304.0)], &narrow, MARGIN), Seating { seats: Vec::new(), leavers: slugs(&["a"]) });
    assert_eq!(seat_of(&[body("a", 90.0, 248.0, 198.0, 304.0)], &narrow, MARGIN).leavers, Vec::<Slug>::new());
    let crowd = [body("a", 0.0, 248.0, 60.0, 304.0), body("b", 0.0, 248.0, 60.0, 304.0), body("c", 0.0, 248.0, 60.0, 304.0), body("d", 0.0, 248.0, 60.0, 304.0)];
    assert_eq!(seat_of(&crowd, &narrow, MARGIN).leavers, slugs(&["d", "c", "b"]));
}

#[test]
fn a_seating_is_the_nearest_valid_one_found_by_a_search_over_every_way_to_pool_neighbours() {
    seatings_agree_with_every_pooling(40);
}

#[test]
fn a_group_scoots_by_one_and_the_same_share_of_the_way_which_never_overlaps_within_the_group() {
    assert_eq!(scoot_fraction(&[3.0, -6.0, 0.0], 1.5), 0.25);
    assert_eq!(scoot_fraction(&[3.0, -6.0, 0.0], 10.0), 1.0);
    assert_eq!(scoot_fraction(&[3.0, -6.0, 0.0], 6.0), 1.0);
    assert_eq!(scoot_fraction(&[0.0, 0.0], 1.5), 1.0);
    assert_eq!(scoot_fraction(&[], 1.5), 1.0);
    assert_eq!(scoot_fraction(&[3.0], 0.0), 0.0);
    assert_eq!(scoot_fraction(&[3.0], -2.0), 0.0);
    assert_eq!(scoot_fraction(&[0.0], -2.0), 1.0);
    assert_eq!(scooted(10.0, 20.0, 0.25), 12.5);
    assert_eq!(scooted(10.0, 20.0, 0.0), 10.0);
    assert_eq!(scooted(10.0, 20.0, 1.0), 20.0);
    assert_eq!(scooted(10.0, 20.0, 1.5), 20.0);
    scoots_stay_apart(40);
}

/// 🎩️ The host of the cases of heads.
fn host() -> Body {
    body("h", 100.0, 240.0, 150.0, 304.0)
}

#[test]
fn a_descending_body_lands_on_the_top_it_comes_down_onto_like_a_perch_crossing() {
    let host = host();
    let hosts = std::slice::from_ref(&host);
    let before = rect(90.0, 180.0, 130.0, 236.0);
    let after = rect(90.0, 190.0, 130.0, 246.0);
    assert_eq!(head_under(before, after, "f", hosts), Some(&host));
    assert_eq!(lift_onto(after, host.extent), -(6.0 + SEAM));
    assert_eq!(shifted(after, 0.0, lift_onto(after, host.extent)).y1, 240.0 - SEAM);
    assert_eq!(head_under(before, after, "h", hosts), None);
    assert_eq!(head_under(shifted(before, -50.0, 0.0), shifted(after, -50.0, 0.0), "f", hosts), None);
    assert_eq!(head_under(shifted(before, -30.0, 0.0), shifted(after, -29.75, 0.0), "f", hosts), Some(&host));
    assert_eq!(head_under(shifted(before, -30.0, 0.0), shifted(after, -30.0, 0.0), "f", hosts), None);
    assert_eq!(head_under(after, before, "f", hosts), None);
    assert_eq!(head_under(shifted(before, 0.0, 14.0), shifted(after, 0.0, 14.0), "f", hosts), None);
    assert_eq!(head_under(before, shifted(before, 0.0, 3.0), "f", hosts), None);
    assert_eq!(head_under(before, shifted(before, 0.0, 4.0), "f", hosts), Some(&host));
    assert_eq!(head_under(shifted(before, 0.0, 4.0), shifted(before, 0.0, 4.25), "f", hosts), Some(&host));
    assert_eq!(head_under(before, after, "f", &[]), None);
}

#[test]
fn the_highest_head_is_chosen_and_the_first_one_among_equals() {
    let before = rect(90.0, 100.0, 180.0, 200.0);
    let after = rect(90.0, 160.0, 180.0, 260.0);
    let taller = body("t", 150.0, 220.0, 200.0, 304.0);
    let twin = body("w", 60.0, 220.0, 95.0, 304.0);
    assert_eq!(head_under(before, after, "f", &[host(), taller.clone()]), Some(&taller));
    assert_eq!(head_under(before, after, "f", &[taller.clone(), host()]), Some(&taller));
    assert_eq!(head_under(before, after, "f", &[host(), twin.clone(), taller]), Some(&twin));
}

#[test]
fn a_rider_still_stands_on_its_host_or_not() {
    let host = host().extent;
    let rider = rect(90.0, 180.0, 130.0, 240.0 - SEAM);
    assert!(rests_on(rider, host));
    assert!(rests_on(shifted(rider, 0.0, SEAM), host));
    assert!(rests_on(shifted(rider, 0.0, -SEAM), host));
    assert!(!rests_on(shifted(rider, 0.0, -2.0 * SEAM), host));
    assert!(!rests_on(shifted(rider, 0.0, 2.0 * SEAM), host));
    assert!(!rests_on(shifted(rider, -30.0, 0.0), host));
    assert!(rests_on(shifted(rider, -29.75, 0.0), host));
    assert!(!rests_on(shifted(rider, 60.0, 0.0), host));
}

#[test]
fn a_rider_slides_off_to_the_nearer_side_faster_and_faster_up_to_a_limit() {
    let host = host().extent;
    assert_eq!(slide_side(rect(110.0, 0.0, 150.0, 10.0), host), Facing::Right);
    assert_eq!(slide_side(rect(105.0, 0.0, 145.0, 10.0), host), Facing::Right);
    assert_eq!(slide_side(rect(104.0, 0.0, 145.0, 10.0), host), Facing::Left);
    assert_eq!(slide_stride(0), 0.5);
    assert_eq!(slide_stride(10), 0.8125);
    assert_eq!(slide_stride(48), 2.0);
    assert_eq!(slide_stride(1000), 2.0);
    for tick in 1..100 {
        assert!(slide_stride(tick) >= slide_stride(tick - 1));
    }
}

#[test]
fn a_slide_is_guarded_kept_on_the_stage_and_turned_round_where_it_cannot_move() {
    let rider = rect(90.0, 180.0, 130.0, 240.0 - SEAM);
    let right = Facing::Right;
    let left = Facing::Left;
    assert_eq!(slide_of(rider, right, 0, 0.0, 960.0, &[host().extent]), Slide { stride: 0.5, side: right });
    assert_eq!(slide_of(rider, left, 10, 0.0, 960.0, &[host().extent]), Slide { stride: -0.8125, side: left });
    assert_eq!(slide_of(rider, right, 0, 0.0, 960.0, &[rect(130.25, 100.0, 200.0, 235.0)]), Slide { stride: 0.25 - SEAM, side: right });
    assert_eq!(slide_of(rider, right, 0, 0.0, 960.0, &[rect(130.0 + SEAM, 100.0, 200.0, 235.0)]), Slide { stride: 0.0, side: left });
    assert_eq!(slide_of(rider, left, 0, 0.0, 960.0, &[rect(40.0, 100.0, 90.0, 235.0)]), Slide { stride: 0.0, side: right });
    assert_eq!(slide_of(rider, right, 0, 0.0, 130.0, &[]), Slide { stride: 0.0, side: left });
    assert_eq!(slide_of(rider, right, 0, 0.0, 130.25, &[]), Slide { stride: 0.25, side: right });
    assert_eq!(slide_of(rider, left, 0, 90.0, 960.0, &[]), Slide { stride: 0.0, side: right });
    assert_eq!(slide_of(rider, left, 0, 89.75, 960.0, &[]), Slide { stride: -0.25, side: left });
}

#[test]
fn a_held_body_takes_the_shortest_way_out_of_an_obstacle_a_seam_to_spare() {
    let extent = rect(100.0, 100.0, 140.0, 150.0);
    let at = |x: f64, y: f64| Some(Point { x, y });
    assert_eq!(pushed_out(extent, &[], PUSHES), at(0.0, 0.0));
    assert_eq!(pushed_out(extent, &[rect(140.0, 100.0, 200.0, 150.0), rect(0.0, 150.0, 300.0, 200.0)], PUSHES), at(0.0, 0.0));
    assert_eq!(pushed_out(extent, &[rect(130.0, 90.0, 200.0, 160.0)], PUSHES), at(-(10.0 + SEAM), 0.0));
    assert_eq!(pushed_out(extent, &[rect(40.0, 90.0, 108.0, 160.0)], PUSHES), at(8.0 + SEAM, 0.0));
    assert_eq!(pushed_out(extent, &[rect(90.0, 140.0, 150.0, 200.0)], PUSHES), at(0.0, -(10.0 + SEAM)));
    assert_eq!(pushed_out(extent, &[rect(90.0, 60.0, 150.0, 105.0)], PUSHES), at(0.0, 5.0 + SEAM));
    assert_eq!(pushed_out(extent, &[rect(110.0, 0.0, 130.0, 300.0)], PUSHES), at(-(30.0 + SEAM), 0.0));
    assert_eq!(pushed_out(extent, &[rect(0.0, 115.0, 300.0, 135.0)], PUSHES), at(0.0, -(35.0 + SEAM)));
}

#[test]
fn a_held_body_goes_on_from_obstacle_to_obstacle_and_gives_up_when_its_rounds_are_over() {
    let extent = rect(100.0, 100.0, 140.0, 150.0);
    let pair = [rect(130.0, 90.0, 200.0, 160.0), rect(80.0, 60.0, 125.0, 105.0)];
    let freed = Some(Point { x: -(10.0 + SEAM), y: 5.0 + SEAM });
    assert_eq!(pushed_out(extent, &pair, PUSHES), freed);
    assert_eq!(pushed_out(extent, &pair, 1), freed);
    assert_eq!(pushed_out(extent, &[pair[1], pair[0]], 1), freed);
    assert_eq!(pushed_out(extent, &[rect(130.0, 90.0, 200.0, 160.0), rect(60.0, 90.0, 95.0, 160.0)], PUSHES), None);
    let walls = [rect(0.0, 0.0, 105.0, 300.0), rect(135.0, 0.0, 300.0, 300.0)];
    assert_eq!(pushed_out(extent, &walls, PUSHES), None);
    assert_eq!(pushed_out(extent, &walls, 100), None);
    assert_eq!(pushed_out(extent, &[rect(130.0, 90.0, 200.0, 160.0)], 0), None);
    assert_eq!(pushed_out(extent, &[rect(140.0, 90.0, 200.0, 160.0)], 0), Some(Point { x: 0.0, y: 0.0 }));
}

#[test]
fn a_push_answers_a_free_place_or_nothing_on_generated_crowds() {
    pushes_free_or_nothing(40);
}

/// 🧯️ The bodies and plans of the cases of the last resort.
fn resort_scene() -> ([Body; 2], Claim, Claim) {
    let bodies = [body("a", 0.0, 0.0, 40.0, 50.0), body("b", 100.0, 0.0, 140.0, 50.0)];
    let clear = claim_of("a", 0, &marching(0.0, 0.0, 0.0, 10.0, 10), 1, Some(rect(0.0, 90.0, 40.0, 140.0)));
    let blocked = claim_of("a", 0, &marching(0.0, 0.0, 10.0, 0.0, 10), 1, Some(rect(90.0, 0.0, 130.0, 50.0)));
    (bodies, clear, blocked)
}

#[test]
fn a_pet_may_be_as_long_as_it_may_stay_or_one_of_its_plans_is_clear() {
    let (bodies, clear, blocked) = resort_scene();
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[], &bodies, &[], 0));
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), std::slice::from_ref(&blocked), &bodies, &[], 0));
    assert!(!must_poof("a", None, &[blocked.clone(), clear.clone()], &bodies, &[], 0));
    assert!(!must_poof("a", Some(rect(70.0, 0.0, 110.0, 50.0)), &[blocked, clear], &bodies, &[], 0));
}

#[test]
fn a_pet_that_may_not_stay_and_has_no_clear_plan_poofs() {
    let (bodies, clear, blocked) = resort_scene();
    assert!(must_poof("a", None, &[], &bodies, &[], 0));
    assert!(must_poof("a", None, std::slice::from_ref(&blocked), &bodies, &[], 0));
    assert!(must_poof("a", Some(rect(70.0, 0.0, 110.0, 50.0)), std::slice::from_ref(&blocked), &bodies, &[], 0));
    let corridor = [claim_of("c", 5, &marching(0.0, -100.0, 0.0, 20.0, 6), 1, None)];
    assert!(must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[], &bodies, &corridor, 5));
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[], &bodies, &corridor, 11));
    assert!(must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), std::slice::from_ref(&blocked), &bodies, &corridor, 5));
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[blocked, clear], &bodies, &corridor, 5));
}

#[test]
fn whoever_a_change_from_outside_carried_into_somebody_who_stays_is_evicted() {
    let (bodies, _, _) = resort_scene();
    let with = |extra: &[Body]| bodies.iter().cloned().chain(extra.iter().cloned()).collect::<Vec<_>>();
    assert_eq!(evicted(&bodies, &[], 0), Vec::<Slug>::new());
    assert_eq!(evicted(&with(&[body("c", 30.0, 0.0, 70.0, 50.0)]), &[], 0), slugs(&["c"]));
    assert_eq!(evicted(&[body("c", 30.0, 0.0, 70.0, 50.0), bodies[0].clone(), bodies[1].clone()], &[], 0), slugs(&["a"]));
    assert_eq!(evicted(&with(&[body("c", 30.0, 0.0, 70.0, 50.0), body("d", 60.0, 0.0, 90.0, 50.0)]), &[], 0), slugs(&["c"]));
    assert_eq!(evicted(&with(&[body("c", 30.0, 0.0, 110.0, 50.0), body("d", 60.0, 0.0, 90.0, 50.0)]), &[], 0), slugs(&["c"]));
    assert_eq!(evicted(&with(&[body("d", 60.0, 0.0, 90.0, 50.0), body("c", 30.0, 0.0, 110.0, 50.0)]), &[], 0), slugs(&["c"]));
}

#[test]
fn a_body_without_a_plan_is_evicted_from_the_corridors_and_rests_of_those_who_stay() {
    let (bodies, _, _) = resort_scene();
    let plan = claim_of("b", 0, &marching(100.0, 0.0, 20.0, 0.0, 10), 1, Some(rect(300.0, 0.0, 340.0, 50.0)));
    let rider = body("c", 200.0, 0.0, 240.0, 50.0);
    let with = |extra: Body| bodies.iter().cloned().chain([extra]).collect::<Vec<_>>();
    assert_eq!(evicted(&with(rider.clone()), std::slice::from_ref(&plan), 0), slugs(&["c"]));
    assert_eq!(evicted(&with(rider.clone()), std::slice::from_ref(&plan), 8), Vec::<Slug>::new());
    assert_eq!(evicted(&with(body("c", 310.0, 0.0, 350.0, 50.0)), std::slice::from_ref(&plan), 100), slugs(&["c"]));
    assert_eq!(evicted(&with(rider.clone()), &[plan.clone(), claim_of("c", 0, &marching(200.0, 0.0, 0.0, 0.0, 3), 1, None)], 0), Vec::<Slug>::new());
    assert_eq!(evicted(&[body("a", 0.0, 0.0, 40.0, 50.0), body("b", 30.0, 0.0, 70.0, 50.0), rider], &[plan], 0), slugs(&["b"]));
}

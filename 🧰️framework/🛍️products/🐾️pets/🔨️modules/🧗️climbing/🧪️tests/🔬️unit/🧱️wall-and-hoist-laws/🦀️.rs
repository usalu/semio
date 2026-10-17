//! 🧱️ The stage cut, line of sight, the hoist and the wall holds, crowns, takes and surveys of the climbing unit laws. See [`super`].

use super::*;

#[test]
fn the_stage_of_this_suite_is_what_the_terrain_cuts_out_of_a_card_that_stands_on_the_floor() {
    let walls =
        [Wall { id: "card-left".to_string(), surface: "card".to_string(), side: Facing::Left, x: 300.0, y0: 300.0, y1: 480.0 }, Wall { id: "card-right".to_string(), surface: "card".to_string(), side: Facing::Right, x: 500.0, y0: 300.0, y1: 480.0 }];
    assert_eq!(walls_of(&walls, &[card_box()], 640.0, 480.0, 48.0, 72.0), [left(), right()]);
}

#[test]
fn a_line_is_sighted_past_every_keep_out_it_misses_and_every_one_that_holds_one_of_its_ends() {
    let (from, to) = (point(0.0, 0.0), point(100.0, 100.0));
    let boxed = keepout(40.0, 40.0, 20.0, 20.0);
    assert!(!sighted(from, to, &[boxed], 0.0, from, to));
    assert!(sighted(from, to, &[boxed], 0.0, point(50.0, 50.0), to) && sighted(from, to, &[boxed], 0.0, from, point(40.0, 60.0)));
    assert!(sighted(from, to, &[keepout(102.0, 95.0, 10.0, 10.0)], 0.0, from, to) && !sighted(from, to, &[keepout(102.0, 95.0, 10.0, 10.0)], 3.0, from, to));
    assert!(sighted(from, to, &[], 0.0, from, to) && sighted(from, to, &[keepout(40.0, 40.0, 0.0, 20.0)], 0.0, from, to));
}

#[test]
fn hoist_begins_on_its_start_and_ends_on_its_end_exactly_and_stays_there_beyond() {
    let (from, to) = (point(280.0, 340.8), point(328.0, 300.0));
    assert_eq!([hoist_path(from, to, 48.0, 0.0), hoist_path(from, to, 48.0, -0.5)], [from, from]);
    assert_eq!([hoist_path(from, to, 48.0, 1.0), hoist_path(from, to, 48.0, 1.5)], [to, to]);
}

#[test]
fn hoist_rises_before_it_crosses_over_tops_its_end_by_the_hump_and_settles_onto_it() {
    let (from, to) = (point(280.0, 340.8), point(328.0, 300.0));
    let path: Vec<Point> = (0..=MANTLE_TICKS).map(|tick| hoist_path(from, to, 48.0, tick as f64 / MANTLE_TICKS as f64)).collect();
    for tick in 1..path.len() {
        assert!(path[tick].x >= path[tick - 1].x, "tick {tick}");
    }
    for (tick, at) in path.iter().enumerate() {
        if tick as f64 / MANTLE_TICKS as f64 <= 0.35 {
            assert_eq!(at.x, from.x, "tick {tick}");
        }
    }
    let apex = path.iter().fold(f64::INFINITY, |least, at| least.min(at.y));
    assert!(apex > to.y - HOIST_HUMP * 48.0 - 1e-9 && apex < to.y - HOIST_HUMP * 48.0 + 0.2, "{apex}");
    let top = path.iter().position(|at| at.y == apex).unwrap_or_default();
    for tick in 1..=top {
        assert!(path[tick].y < path[tick - 1].y, "tick {tick}");
    }
    for tick in top + 1..path.len() {
        assert!(path[tick].y > path[tick - 1].y, "tick {tick}");
    }
}

#[test]
fn hoist_is_the_three_eased_parts_of_its_definition_at_every_phase_mirrors_with_the_side_and_leads_down_as_well_as_up() {
    let (from, to) = (point(280.0, 340.8), point(328.0, 300.0));
    for step in 0..=200 {
        let phase = f64::from(step) / 200.0;
        let found = hoist_path(from, to, 48.0, phase);
        let (rise, across, settle) = (smoothstep(phase / 0.65), smoothstep((phase - 0.35) / 0.65), smoothstep((phase - 0.65) / 0.35));
        assert!((found.x - (from.x + (to.x - from.x) * across)).abs() < 1e-9, "phase {phase}");
        assert!((found.y - (from.y + (to.y - 4.8 - from.y) * rise + 4.8 * settle)).abs() < 1e-9, "phase {phase}");
    }
    let mirrored = hoist_path(point(520.0, 340.8), point(472.0, 300.0), 48.0, 0.6);
    let found = hoist_path(from, to, 48.0, 0.6);
    assert!((mirrored.x - 400.0 - (400.0 - found.x)).abs() < 5e-10);
    assert_eq!(mirrored.y, found.y);
    let down = hoist_path(point(100.0, 200.0), point(130.0, 260.0), 40.0, 0.5);
    assert!(down.y > 200.0 && down.y < 260.0);
}

#[test]
fn a_wall_is_clung_to_half_a_width_out_on_the_air_side_between_the_rim_and_the_foot_of_the_pitch() {
    let (left, right) = (left(), right());
    assert_eq!([cling_of(&left, PET), cling_of(&right, PET)], [280.0, 520.0]);
    assert_eq!([ledge_of(&left, PET), ledge_of(&right, PET)], [300.0 + 20.0 + MANTLE_INSET, 500.0 - 20.0 - MANTLE_INSET]);
    assert_eq!(rim_of(&left, PET), 300.0 + HAND_HEIGHT * 48.0);
    assert_eq!(foot_of(&left, PET), 480.0 - GRIP_BITE + HAND_HEIGHT * 48.0);
    assert!(clings(&left, rim_of(&left, PET), PET) && clings(&left, foot_of(&left, PET), PET) && clings(&left, 400.0, PET));
    assert!(!clings(&left, rim_of(&left, PET) - 0.25, PET) && !clings(&left, foot_of(&left, PET) + 0.25, PET));
}

#[test]
fn a_wall_is_crowned_by_the_perch_of_its_surface_at_its_top_that_carries_the_ledge() {
    let (left, right, card) = (left(), right(), card());
    assert!(crowns(&card, &left, PET) && crowns(&card, &right, PET));
    assert!(!crowns(&floor(), &left, PET));
    assert!(!crowns(&perch("card", 306.0, 494.0, 299.5), &left, PET));
    assert!(!crowns(&perch("other", 306.0, 494.0, 300.0), &left, PET));
    assert!(crowns(&perch("card", 328.0, 494.0, 300.0), &left, PET));
    assert!(!crowns(&perch("card", 328.5, 494.0, 300.0), &left, PET));
    assert!(!crowns(&card, &pitch("card-left", "card", Facing::Left, 300.0, 320.0, 480.0), PET));
    let perches = [floor(), perch("card", 400.0, 494.0, 300.0), card];
    assert!(rim_for(&left, &perches, PET).is_some_and(|found| std::ptr::eq(found, &perches[2])));
    assert_eq!(rim_for(&left, &perches[..2], PET), None);
    assert_eq!(rim_for(&left, &[], PET), None);
}

#[test]
fn a_wall_is_taken_from_a_perch_at_its_foot_where_the_actor_can_stand_beside_it_with_its_hands_on_it() {
    let (left, right) = (left(), right());
    let hold = |x: f64, y: f64| Some(WallHold { x, y, over: false });
    assert_eq!(grip_for(&floor(), &left, PET), hold(280.0, 480.0));
    assert_eq!(grip_for(&floor(), &right, PET), hold(520.0, 480.0));
    assert_eq!(grip_for(&perch("shelf", 100.0, 280.0, 420.0), &left, PET), hold(280.0, 420.0));
    assert_eq!(grip_for(&perch("shelf", 100.0, 279.5, 420.0), &left, PET), None);
    assert_eq!(grip_for(&perch("shelf", 280.5, 296.0, 420.0), &left, PET), None);
    assert_eq!(grip_for(&perch("floor", 0.0, 640.0, 512.8), &left, PET), hold(280.0, 512.8));
    assert_eq!(grip_for(&perch("floor", 0.0, 640.0, 513.0), &left, PET), None);
    assert_eq!(grip_for(&perch("shelf", 100.0, 290.0, rim_of(&left, PET)), &left, PET), hold(280.0, rim_of(&left, PET)));
    assert_eq!(grip_for(&perch("shelf", 100.0, 290.0, 340.0), &left, PET), None);
    assert_eq!(grip_for(&floor(), &left, LARGE), hold(277.0, 480.0));
}

#[test]
fn a_wall_is_taken_over_the_rim_from_the_perch_that_crowns_it_when_the_pitch_is_long_enough_to_hold() {
    let over = Some(WallHold { x: 280.0, y: 340.8, over: true });
    assert_eq!(grip_for(&card(), &left(), PET), over);
    assert_eq!(grip_for(&card(), &right(), PET), Some(WallHold { x: 520.0, y: 340.8, over: true }));
    assert_eq!(grip_for(&card(), &pitch("card-left", "card", Facing::Left, 300.0, 300.0, 308.0), PET), over);
    assert_eq!(grip_for(&card(), &pitch("card-left", "card", Facing::Left, 300.0, 300.0, 307.5), PET), None);
    assert_eq!(grip_for(&perch("card", 340.0, 494.0, 300.0), &left(), PET), None);
    assert_eq!(grip_for(&card(), &pitch("card-left", "card", Facing::Left, 300.0, 330.0, 480.0), PET), None);
}

#[test]
fn a_wall_carries_its_climber_through_a_survey_while_it_stays_within_the_tolerance_and_free_under_its_hands_and_throws_off_whoever_lost_it() {
    let left = left();
    let moved = |dx: f64, y0: f64, y1: f64| vec![right(), pitch("card-left", "card", Facing::Left, 300.0 + dx, y0, y1)];
    assert_eq!(wall_holds(&left, &moved(0.0, 300.0, 480.0), 400.0, PET), Some(&left));
    assert_eq!(wall_holds(&left, &moved(WALL_FOLLOW, 300.0, 480.0), 400.0, PET), Some(&pitch("card-left", "card", Facing::Left, 306.0, 300.0, 480.0)));
    assert!(wall_holds(&left, &moved(-WALL_FOLLOW, 300.0, 480.0), 400.0, PET).is_some());
    assert_eq!(wall_holds(&left, &moved(WALL_FOLLOW + 0.5, 300.0, 480.0), 400.0, PET), None);
    assert_eq!(wall_holds(&left, &moved(0.0, 300.0, 360.0), 400.0, PET), None);
    assert!(wall_holds(&left, &moved(0.0, 300.0, 367.2), 400.0, PET).is_some());
    assert_eq!(wall_holds(&left, &moved(0.0, 360.0, 480.0), 400.0, PET), None);
    assert_eq!(wall_holds(&left, &[right()], 400.0, PET), None);
    assert_eq!(wall_holds(&left, &[pitch("card-left", "card", Facing::Right, 300.0, 300.0, 480.0)], 400.0, PET), None);
    let split = [pitch("card-left", "card", Facing::Left, 300.0, 300.0, 340.0), pitch("card-left", "card", Facing::Left, 300.0, 350.0, 480.0)];
    assert!(wall_holds(&left, &split, 400.0, PET).is_some_and(|found| std::ptr::eq(found, &split[1])));
    assert_eq!(wall_holds(&left, &[], 400.0, PET), None);
    assert_eq!(slip_of(&left), Toss { vx: -SLIP_PUSH, vy: -SLIP_LIFT });
    assert_eq!(slip_of(&right()), Toss { vx: SLIP_PUSH, vy: -SLIP_LIFT });
}

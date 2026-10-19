//! 🪜️ The ladder lean, rise, passes, generated stages, climb, carry, toppling and following of the climbing unit laws. See [`super`].

use super::*;

#[test]
fn a_ladder_stands_at_the_lean_of_real_ladders_with_its_top_a_little_under_the_rim() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let ladder = ladder_for(&floor(), &shelf, &wall, &boxed, PET).unwrap_or_else(|| panic!("no ladder"));
    assert_eq!(ladder, LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(300.0 - LADDER_LEAN * 93.0, 480.0), top: point(300.0, 380.0 + LADDER_TUCK) });
    assert_eq!(ladder_lean(&ladder), LADDER_LEAN);
    assert!((ladder_length(&ladder) - 23.25_f64.hypot(93.0)).abs() < 1e-12);
    assert_eq!(ladder_rungs(&ladder), 9);
    let mirrored = ladder_for(&floor(), &shelf, &pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0), &boxed, PET).unwrap_or_else(|| panic!("no mirrored ladder"));
    assert_eq!((mirrored.foot, mirrored.side), (point(500.0 + LADDER_LEAN * 93.0, 480.0), Facing::Right));
}

#[test]
fn a_ladder_moves_its_foot_as_near_to_that_lean_as_its_perch_lets_it_within_the_steepest_and_the_flattest_lean() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let short = |x0: f64, x1: f64| ladder_for(&perch("floor", x0, x1, 480.0), &shelf, &wall, &boxed, PET);
    let foot = |x0: f64, x1: f64| short(x0, x1).map(|ladder| ladder.foot.x);
    assert_eq!([foot(0.0, 640.0), foot(0.0, 275.0), foot(0.0, 273.0), foot(270.0, 640.0), foot(276.5, 640.0), foot(260.0, 280.0)], [Some(276.75), Some(275.0 - LADDER_FOOTING), Some(263.0), Some(270.0 + LADDER_FOOTING), Some(286.5), Some(270.0)]);
    assert!((short(0.0, 275.0).map_or(f64::NAN, |ladder| ladder_lean(&ladder)) - 35.0 / 93.0).abs() < 5e-13);
    assert!((short(270.0, 640.0).map_or(f64::NAN, |ladder| ladder_lean(&ladder)) - 20.0 / 93.0).abs() < 5e-13);
    assert_eq!([foot(0.0, 272.5), foot(277.5, 640.0), foot(260.0, 279.5), foot(310.0, 640.0)], [None; 4]);
}

#[test]
fn a_ladder_needs_a_rise_between_the_shortest_and_the_tallest_ladder_of_its_owner_and_a_pitch_that_reaches_down_to_its_contact() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let from = |y: f64, size: Size| ladder_for(&perch("floor", 0.0, 640.0, y), &shelf, &wall, &[], size).is_some();
    assert!(from(387.0 + LADDER_SHORT * 48.0 + 0.5, PET) && !from(387.0 + LADDER_SHORT * 48.0 - 0.5, PET));
    assert!(from(387.0 + LADDER_TALL * 48.0 - 0.5, PET) && !from(387.0 + LADDER_TALL * 48.0 + 0.5, PET));
    assert!(!from(387.0 + LADDER_SHORT * 48.0 + 0.5, LARGE) && from(387.0 + LADDER_SHORT * 56.0 + 0.5, LARGE));
    assert!(from(387.0 + LADDER_SHORT * 40.0 + 0.5, SMALL) && from(387.0 + LADDER_TALL * 48.0 + 0.5, LARGE));
    assert!(!from(380.0, PET) && !from(300.0, PET));
    assert_eq!(ladder_for(&floor(), &perch("shelf", 340.0, 494.0, 380.0), &wall, &[], PET), None);
    assert_eq!(ladder_for(&floor(), &perch("other", 306.0, 494.0, 380.0), &wall, &[], PET), None);
    assert_eq!(ladder_for(&floor(), &shelf, &pitch("shelf-left", "shelf", Facing::Left, 300.0, 390.0, 480.0), &[], PET), None);
    assert!(ladder_for(&floor(), &shelf, &pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 387.0), &[], PET).is_some());
    assert_eq!(ladder_for(&floor(), &shelf, &pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 386.5), &[], PET), None);
}

#[test]
fn a_ladder_passes_over_what_it_stands_on_and_leans_against_and_is_blocked_by_anything_else_within_its_girth() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let stage = |blocker: Rect| ladder_for(&floor(), &shelf, &wall, &[solid(300.0, 500.0, 380.0, 480.0), keepout(0.0, 480.0, 640.0, 40.0), blocker], PET);
    let ladder = stage(keepout(0.0, 0.0, 0.0, 0.0)).unwrap_or_else(|| panic!("no ladder"));
    assert!(stage(keepout(280.0, 400.0, 10.0, 30.0)).is_none() && stage(keepout(240.0, 400.0, 30.0, 30.0)).is_some());
    let reach = along(ladder.foot, ladder.top, 430.0);
    assert!(stage(keepout(reach - LADDER_GIRTH - 20.0, 420.0, 20.0, 20.0)).is_none() && stage(keepout(reach - LADDER_GIRTH - 40.0, 420.0, 20.0, 20.0)).is_some());
    assert!(stage(keepout(200.0, 440.0, 60.0, 34.0)).is_some() && stage(keepout(200.0, 440.0, 71.5, 34.5)).is_none());
}

#[test]
fn a_ladder_keeps_every_law_on_generated_stages() {
    let mut next = stream(41);
    let (mut granted, mut refused) = (0, 0);
    for stage in 0..STAGES {
        let size = [PET, SMALL, LARGE][next(3) as usize];
        let side = if next(2) == 0.0 { Facing::Left } else { Facing::Right };
        let x = 200.0 + next(800) / 4.0;
        let rim = 150.0 + next(600) / 4.0;
        let high = if side == Facing::Left { perch("high", x + 6.0, x + 180.0, rim) } else { perch("high", x - 180.0, x - 6.0, rim) };
        let face = pitch("high-side", "high", side, x, rim, rim + 20.0 + next(800) / 4.0);
        let low = perch("low", x - 150.0 + next(600) / 4.0, x + next(600) / 4.0, rim + 20.0 + next(900) / 4.0);
        let far = x - side.sign() * 186.0;
        let mut keepouts = vec![solid(x.min(far), x.max(far), rim, rim + 300.0)];
        for _ in 0..next(3) as usize {
            keepouts.push(keepout(x - 120.0 + next(960) / 4.0, rim - 40.0 + next(1200) / 4.0, next(160) / 4.0, next(160) / 4.0));
        }
        let Some(ladder) = ladder_for(&low, &high, &face, &keepouts, size) else {
            refused += 1;
            continue;
        };
        granted += 1;
        let rise = ladder.foot.y - ladder.top.y;
        assert_eq!((ladder.top, ladder.foot.y), (point(x, rim + LADDER_TUCK), low.y), "stage {stage}");
        assert!(ladder.foot.x >= low.x0 + LADDER_FOOTING && ladder.foot.x <= low.x1 - LADDER_FOOTING, "stage {stage}");
        assert!((ladder.foot.x - x) * side.sign() > 0.0, "stage {stage}");
        assert!(rise >= LADDER_SHORT * size.height && rise <= LADDER_TALL * size.height, "stage {stage}");
        assert!(ladder_lean(&ladder) >= LADDER_STEEP - 1e-12 && ladder_lean(&ladder) <= LADDER_FLAT + 1e-12, "stage {stage}");
        assert!((ladder_length(&ladder) - (ladder.foot.x - x).hypot(rise)).abs() < 1e-9, "stage {stage}");
        assert_eq!(f64::from(ladder_rungs(&ladder)), ((ladder.foot.x - x).hypot(rise) / RUNG_SPACING).floor(), "stage {stage}");
        for blocker in &keepouts {
            if !holding(blocker, ladder.foot) && !holding(blocker, ladder.top) {
                assert!(!segment_hits(ladder.foot, ladder.top, blocker, LADDER_GIRTH), "stage {stage}");
            }
        }
        assert_eq!(ladder_holds(&ladder, &[low, high], &[face], &keepouts, size).as_ref(), Some(&ladder), "stage {stage}");
    }
    assert!(granted > STAGES / 12 && refused > STAGES / 4, "{granted} granted, {refused} refused");
}

#[test]
fn a_ladder_is_climbed_with_a_gathered_speed_up_slower_than_down_ending_on_its_goal_exactly() {
    let ladder = LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(276.75, 480.0), top: point(300.0, 387.0) };
    let length = 23.25_f64.hypot(93.0);
    let exit = ladder_exit(&ladder, PET);
    assert!((exit - (length - LADDER_EXIT * 48.0)).abs() < 1e-12);
    let (mut travel, mut ticks) = (0.0, 0);
    while travel != exit && ticks < 1000 {
        let next = ladder_step(travel, exit, ticks);
        assert!((next - travel - ((LADDER_RISE * smoothstep((ticks + 1) as f64 / 6.0)) / 64.0).min(exit - travel)).abs() < 1e-9, "tick {ticks}");
        travel = next;
        ticks += 1;
    }
    assert_eq!(travel, exit);
    assert_eq!(ticks, 6 + ((exit - (3.5 * LADDER_RISE) / 64.0) / (LADDER_RISE / 64.0)).ceil() as Ticks);
    assert_eq!(ladder_step(exit, exit, ticks), exit);
    let mut down = 0;
    travel = exit;
    while travel != 0.0 && down < 1000 {
        travel = ladder_step(travel, 0.0, down);
        down += 1;
    }
    assert_eq!(travel, 0.0);
    assert_eq!(down, 6 + ((exit - (3.5 * LADDER_DESCENT) / 64.0) / (LADDER_DESCENT / 64.0)).ceil() as Ticks);
    assert!(down < ticks);
}

#[test]
fn a_ladder_carries_its_climber_along_the_rails_puts_hands_and_feet_on_the_rungs_and_lets_it_off_inside_the_rim() {
    let ladder = LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(276.75, 480.0), top: point(300.0, 387.0) };
    let length = 23.25_f64.hypot(93.0);
    assert_eq!([ladder_at(&ladder, 0.0), ladder_at(&ladder, -3.0), ladder_at(&ladder, length), ladder_at(&ladder, length + 10.0)], [ladder.foot, ladder.foot, ladder.top, ladder.top]);
    let mut travel = 0.5;
    while travel < length {
        let feet = ladder_at(&ladder, travel);
        assert!((distance(feet, ladder.foot) - travel).abs() < 1e-9 && (distance(feet, ladder.top) - (length - travel)).abs() < 1e-9, "{travel}");
        travel += 3.7;
    }
    assert_eq!(ladder_exit(&LadderStand { foot: point(298.0, 395.0), ..ladder.clone() }, PET), 0.0);
    assert_eq!([ladder_phase(0.0), ladder_phase(RUNG_SPACING), ladder_phase(2.0 * RUNG_SPACING), ladder_phase(5.25), ladder_phase(47.25)], [0.0, 0.5, 0.0, 0.25, 0.25]);
    let mut travel = 0.0;
    while travel < 120.0 {
        assert!((0.0..1.0).contains(&ladder_phase(travel)));
        travel += 0.41;
    }
    assert_eq!(ladder_landing(&ladder, PET), point(300.0 + 20.0 + MANTLE_INSET, 380.0));
    assert_eq!(ladder_landing(&ladder, LARGE), point(300.0 + 23.0 + MANTLE_INSET, 380.0));
    assert_eq!(ladder_landing(&LadderStand { side: Facing::Right, top: point(500.0, 387.0), ..ladder }, PET), point(500.0 - 20.0 - MANTLE_INSET, 380.0));
}

#[test]
fn a_toppling_ladder_drops_whoever_is_high_on_it_and_lets_whoever_is_low_step_off() {
    let ladder = LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(276.75, 480.0), top: point(300.0, 387.0) };
    assert_eq!([spill_of(&ladder, 480.0, PET), spill_of(&ladder, 480.0 - TOPPLE_STEP * 48.0 + 0.5, PET)], [None, None]);
    assert_eq!(spill_of(&ladder, 480.0 - TOPPLE_STEP * 48.0, PET), Some(Toss { vx: -TOPPLE_PUSH, vy: 0.0 }));
    assert_eq!(spill_of(&LadderStand { side: Facing::Right, ..ladder.clone() }, 400.0, PET), Some(Toss { vx: TOPPLE_PUSH, vy: 0.0 }));
    assert_eq!(spill_of(&ladder, 480.0 - TOPPLE_STEP * 48.0, LARGE), None);
}

#[test]
fn a_standing_ladder_follows_its_wall_and_its_perch_within_the_tolerances_and_topples_beyond_them() {
    let (shelf, wall, boxed) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), solid(300.0, 500.0, 380.0, 480.0));
    let ladder = ladder_for(&floor(), &shelf, &wall, &[boxed], PET).unwrap_or_else(|| panic!("no ladder"));
    let moved = |dx: f64, dy: f64| {
        ladder_holds(&ladder, &[floor(), perch("shelf", 306.0 + dx, 494.0 + dx, 380.0 + dy)], &[pitch("shelf-left", "shelf", Facing::Left, 300.0 + dx, 380.0 + dy, 480.0 + dy)], &[solid(300.0 + dx, 500.0 + dx, 380.0 + dy, 480.0 + dy)], PET)
    };
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&ladder));
    for (dx, dy) in [(3.0, 0.0), (0.0, -5.0), (-4.5, 6.0), (0.0, LADDER_FOLLOW), (LADDER_FOLLOW, 0.0)] {
        assert_eq!(moved(dx, dy), Some(LadderStand { top: point(300.0 + dx, 387.0 + dy), ..ladder.clone() }), "{dx} {dy}");
    }
    for (dx, dy) in [(0.0, LADDER_FOLLOW + 0.5), (LADDER_FOLLOW + 0.5, 0.0), (6.0, 6.0), (-40.0, 0.0)] {
        assert_eq!(moved(dx, dy), None, "{dx} {dy}");
    }
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], &[], &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], &[pitch("shelf-left", "shelf", Facing::Right, 300.0, 380.0, 480.0)], &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], &[pitch("other", "shelf", Facing::Left, 300.0, 380.0, 480.0)], &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor()], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&ladder));
    assert_eq!(ladder_holds(&ladder, &[floor(), perch("shelf", 400.0, 494.0, 380.0)], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&ladder));
    let steep = LadderStand { foot: point(300.0 - LADDER_STEEP * 93.0 - 0.5, 480.0), ..ladder.clone() };
    assert_eq!(ladder_holds(&steep, &[floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&steep));
    assert_eq!(ladder_holds(&steep, &[floor(), perch("shelf", 305.0, 493.0, 380.0)], &[pitch("shelf-left", "shelf", Facing::Left, 299.0, 380.0, 480.0)], &[solid(299.0, 499.0, 380.0, 480.0)], PET), None);
    let on = |low: Perch| ladder_holds(&ladder, &[low, shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET);
    assert_eq!(on(perch("floor", 0.0, 640.0, 480.0 + LADDER_SHIFT)), Some(LadderStand { foot: point(ladder.foot.x, 492.0), ..ladder.clone() }));
    assert_eq!(on(perch("floor", 0.0, 640.0, 474.0)), Some(LadderStand { foot: point(ladder.foot.x, 474.0), ..ladder.clone() }));
    assert_eq!(on(perch("floor", 0.0, 640.0, 480.0 + LADDER_SHIFT + 0.5)), None);
    assert_eq!(on(perch("floor", 0.0, ladder.foot.x + LADDER_FOOTING, 480.0)).as_ref(), Some(&ladder));
    assert_eq!(on(perch("floor", 0.0, ladder.foot.x + LADDER_FOOTING - 0.5, 480.0)), None);
    assert_eq!(on(perch("ground", 0.0, 640.0, 480.0)), None);
    assert_eq!(ladder_holds(&ladder, std::slice::from_ref(&shelf), std::slice::from_ref(&wall), &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[perch("floor", 0.0, 100.0, 480.0), floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&ladder));
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed, keepout(270.0, 400.0, 30.0, 30.0)], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf], &[wall], &[boxed, keepout(100.0, 400.0, 30.0, 30.0)], PET).as_ref(), Some(&ladder));
}

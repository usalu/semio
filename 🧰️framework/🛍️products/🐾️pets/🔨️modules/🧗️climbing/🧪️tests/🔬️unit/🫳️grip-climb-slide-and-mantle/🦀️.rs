//! 🫳️ Grip, the climbing speed and clip, sliding and the mantle of the climbing unit laws. See [`super`].

use super::*;

#[test]
fn grip_spends_one_per_tick_of_climbing_a_quarter_per_tick_of_hanging_never_falls_below_nothing_and_comes_back_two_per_tick_of_rest_up_to_the_budget() {
    assert_eq!([grip_step(GRIP_BUDGET, Effort::Climb), grip_step(GRIP_BUDGET, Effort::Hang), grip_step(0.5, Effort::Climb), grip_step(0.0, Effort::Hang)], [383.0, 383.75, 0.0, 0.0]);
    let drained = |effort: Effort| {
        let (mut grip, mut ticks) = (GRIP_BUDGET, 0.0);
        while grip > 0.0 {
            grip = grip_step(grip, effort);
            ticks += 1.0;
        }
        ticks
    };
    assert_eq!([drained(Effort::Climb), drained(Effort::Hang)], [GRIP_BUDGET, 4.0 * GRIP_BUDGET]);
    assert_eq!([grip_step(100.0, Effort::Rest), grip_step(GRIP_BUDGET - 1.0, Effort::Rest), grip_step(GRIP_BUDGET, Effort::Rest)], [102.0, GRIP_BUDGET, GRIP_BUDGET]);
    let (mut grip, mut ticks) = (0.0, 0.0);
    while grip < GRIP_BUDGET {
        grip = grip_step(grip, Effort::Rest);
        ticks += 1.0;
    }
    assert_eq!(ticks, GRIP_BUDGET / 2.0);
}

#[test]
fn climbing_gathers_its_speed_over_the_ramp_then_climbs_at_the_full_speed_up_slower_than_down() {
    let (up, down) = (climbed(480.0, 340.8), climbed(340.8, 480.0));
    for tick in 0..12 {
        let eased = smoothstep((tick + 1) as f64 / CLIMB_RAMP as f64);
        let (up_before, down_before) = if tick == 0 { (480.0, 340.8) } else { (up[tick - 1], down[tick - 1]) };
        assert!((up_before - up[tick] - (CLIMB_RISE * eased) / 64.0).abs() < 1e-9, "up {tick}");
        assert!((down[tick] - down_before - (CLIMB_DESCENT * eased) / 64.0).abs() < 1e-9, "down {tick}");
    }
    assert!((480.0 - up[5] - (3.5 * CLIMB_RISE) / 64.0).abs() < 1e-9);
    assert!(up.len() > down.len());
}

#[test]
fn climbing_never_overshoots_ends_on_its_goal_exactly_and_stays_there() {
    for (from, goal) in [(480.0, 340.8), (340.8, 480.0), (400.0, 399.9), (400.0, 400.3), (512.8, 340.8)] {
        let heights = climbed(from, goal);
        assert_eq!(heights.last().copied(), Some(goal));
        assert!(heights.iter().all(|height| (height - goal) * (from - goal) >= 0.0), "{from} → {goal}");
        assert_eq!(climb_step(goal, goal, heights.len() as Ticks), goal);
        assert_eq!(climb_ticks(from, goal), heights.len() as Ticks);
    }
    assert_eq!(climb_ticks(400.0, 400.0), 0);
}

#[test]
fn climbing_takes_as_long_as_its_closed_form_says_and_gives_up_beyond_the_grip_of_a_rested_actor() {
    for distance in [0.1, 1.5, 1.640625, 2.0, 10.0, 92.0, 139.2, 150.0] {
        let ramp = (3.5 * CLIMB_RISE) / 64.0;
        let expected = if distance <= ramp { climbed(480.0, 480.0 - distance).len() as Ticks } else { CLIMB_RAMP + ((distance - ramp) / (CLIMB_RISE / 64.0) - 1e-9).ceil() as Ticks };
        assert_eq!(climb_ticks(480.0, 480.0 - distance), expected, "distance {distance}");
    }
    let budget = GRIP_BUDGET as Ticks;
    assert_eq!(climb_ticks(480.0, 480.0 - 178.8), budget);
    assert_eq!([climb_ticks(480.0, 480.0 - 181.0), climb_ticks(480.0, -10000.0), climb_ticks(480.0, f64::NEG_INFINITY), climb_ticks(300.0, f64::NAN)], [budget + 1; 4]);
}

#[test]
fn the_climbing_clip_runs_one_cycle_per_two_holds_counted_up_the_wall_and_advances_by_the_distance_climbed() {
    let left = left();
    let stride = 2.0 * GRIP_SPACING * 48.0;
    let turned = |y: f64, expected: f64| (((climb_phase(&left, y, PET) - expected + 1.5) % 1.0) - 0.5).abs();
    assert_eq!(climb_phase(&left, 300.0, PET), 0.0);
    assert!(turned(300.0 + stride, 0.0) < 1e-12 && turned(300.0 + 3.0 * stride, 0.0) < 1e-12);
    assert!(turned(300.0 + stride / 4.0, 0.75) < 1e-12 && turned(300.0 + stride / 2.0, 0.5) < 1e-12 && turned(300.0 - stride / 4.0, 0.25) < 1e-12);
    assert!(turned(300.0 + stride / 4.0, 0.25) > 0.4);
    let mut y = 250.0;
    while y < 520.0 {
        let phase = climb_phase(&left, y, PET);
        assert!((0.0..1.0).contains(&phase), "{y}");
        y += 0.37;
    }
    let mut heights = vec![480.0];
    heights.extend(climbed(480.0, 340.8));
    for tick in 1..heights.len() {
        let advanced = climb_phase(&left, heights[tick], PET) - climb_phase(&left, heights[tick - 1], PET);
        let expected = (heights[tick - 1] - heights[tick]) / (2.0 * GRIP_SPACING * 48.0);
        assert!((advanced - advanced.floor() - expected).abs() < 1e-9, "tick {tick}");
    }
    assert_eq!(climb_phase(&pitch("moved", "card", Facing::Left, 304.0, 310.0, 490.0), 410.0, PET), climb_phase(&left, 400.0, PET));
}

#[test]
fn sliding_starts_at_the_slip_speed_gains_speed_up_to_the_fastest_slide_and_ends_on_its_floor_exactly() {
    assert_eq!(slide_step(340.8, 0.0, 480.0), Fall { y: 340.8 + (SLIDE_START + SLIDE_GAIN / 64.0) / 64.0, vy: SLIDE_START + SLIDE_GAIN / 64.0 });
    assert_eq!(slide_step(340.8, 100.0, 480.0), Fall { y: 340.8 + 109.375 / 64.0, vy: 109.375 });
    assert_eq!(slide_step(340.8, SLIDE_SPEED, 480.0), Fall { y: 340.8 + 2.5, vy: SLIDE_SPEED });
    assert_eq!(slide_step(340.8, 5000.0, 480.0), Fall { y: 340.8 + 2.5, vy: SLIDE_SPEED });
    let (mut slide, mut ticks) = (Fall { y: 340.8, vy: 0.0 }, 0.0);
    while slide.vy < SLIDE_SPEED {
        slide = slide_step(slide.y, slide.vy, 10000.0);
        ticks += 1.0;
        if slide.vy < SLIDE_SPEED {
            assert!((slide.vy - (SLIDE_START + (ticks * SLIDE_GAIN) / 64.0)).abs() < 1e-9);
        }
    }
    assert_eq!(ticks, (((SLIDE_SPEED - SLIDE_START) * 64.0) / SLIDE_GAIN).ceil());
    let (mut slide, mut ticks) = (Fall { y: 340.8, vy: 0.0 }, 0);
    while slide.y != 480.0 && ticks < 500 {
        let next = slide_step(slide.y, slide.vy, 480.0);
        assert!(next.y > slide.y && next.y <= 480.0);
        slide = next;
        ticks += 1;
    }
    assert!(slide.y == 480.0 && ticks > 56 && ticks < 70, "{ticks}");
    assert_eq!(slide_step(480.0, slide.vy, 480.0).y, 480.0);
}

#[test]
fn the_mantle_leads_from_the_highest_hold_over_the_rim_onto_the_ledge_on_either_side() {
    let (left, right) = (left(), right());
    assert_eq!([mantle_path(&left, PET, 0.0), mantle_path(&left, PET, 1.0)], [point(280.0, 340.8), point(328.0, 300.0)]);
    assert_eq!([mantle_path(&right, PET, 0.0), mantle_path(&right, PET, 1.0)], [point(520.0, 340.8), point(472.0, 300.0)]);
    for tick in 0..=MANTLE_TICKS {
        let phase = tick as f64 / MANTLE_TICKS as f64;
        let (on_left, on_right) = (mantle_path(&left, PET, phase), mantle_path(&right, PET, phase));
        assert_eq!(on_left, hoist_path(point(280.0, 340.8), point(328.0, 300.0), 48.0, phase));
        assert!((on_right.x - 400.0 - (400.0 - on_left.x)).abs() < 1e-9 && on_right.y == on_left.y, "tick {tick}");
        assert!(on_left.x <= 300.0 || on_left.y <= 300.5, "tick {tick}");
    }
}

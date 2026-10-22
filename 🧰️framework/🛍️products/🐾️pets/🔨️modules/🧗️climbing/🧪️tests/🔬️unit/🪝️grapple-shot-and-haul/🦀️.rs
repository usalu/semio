//! 🪝️ The grapple shot, its perches and ropes, the flying hook, the haul, the swing and the landing of the climbing unit laws. See [`super`].

use super::*;

#[test]
fn a_shot_aims_from_the_muzzle_in_front_of_the_feet_at_a_point_just_above_the_edge_nearest_to_the_feet() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let boxed = [solid(300.0, 500.0, 380.0, 420.0)];
    let from = |x: f64| shot_for(point(x, 480.0), &[floor(), shelf.clone()], &boxed, PET).unwrap_or_else(|| panic!("no shot from {x}"));
    let shot = from(280.0);
    assert_eq!((shot.surface.as_str(), shot.facing), ("shelf", Facing::Right));
    assert_eq!((shot.muzzle, shot.hook), (point(280.0 + MUZZLE_FORWARD * 40.0, 480.0 - MUZZLE_HEIGHT * 48.0), point(306.0 + HOOK_INSET, 380.0 - HOOK_LIFT)));
    assert!((shot.length - distance(shot.hook, shot.muzzle)).abs() < 1e-12);
    let mirrored = from(520.0);
    assert_eq!((mirrored.facing, mirrored.muzzle, mirrored.hook), (Facing::Left, point(520.0 - MUZZLE_FORWARD * 40.0, 480.0 - MUZZLE_HEIGHT * 48.0), point(494.0 - HOOK_INSET, 379.0)));
    assert!((mirrored.length - shot.length).abs() < 5e-10);
    assert_eq!((from(400.0).hook, from(400.0).facing, from(400.0).reel), (point(400.0, 379.0), Facing::Right, Reeling::Zip));
    assert_eq!([from(440.0).hook, from(311.0).hook, from(309.0).hook], [point(440.0, 379.0), point(310.0, 379.0), point(310.0, 379.0)]);
    assert_eq!(from(311.0).facing, Facing::Left);
    let stub = [perch("stub", 380.0, 420.0, 380.0)];
    assert_eq!(shot_for(point(400.0, 480.0), &stub, &[], PET).map(|shot| shot.hook), Some(point(384.0, 379.0)));
    assert_eq!(shot_for(point(300.0, 480.0), &stub, &[], PET).map(|shot| shot.hook), Some(point(384.0, 379.0)));
    let (steep, slanted) = (from(310.0), from(240.0));
    assert!((steep.hook.x - steep.muzzle.x).abs() <= ZIP_SLANT * (steep.muzzle.y - steep.hook.y) && steep.reel == Reeling::Zip);
    assert!((slanted.hook.x - slanted.muzzle.x).abs() > ZIP_SLANT * (slanted.muzzle.y - slanted.hook.y) && slanted.reel == Reeling::Swing);
}

#[test]
fn a_shot_needs_a_perch_high_enough_a_rope_of_the_right_length_and_steep_enough_and_a_clear_line_but_for_what_lies_under_its_hook() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let boxed = solid(300.0, 500.0, 380.0, 420.0);
    let high = |y: f64, size: Size| shot_for(point(400.0, 480.0), &[perch("stub", 380.0, 420.0, y)], &[], size);
    assert!(high(480.0 - ROPE_RISE * 48.0 + 0.5, PET).is_none());
    assert!(high(413.0, PET).is_some_and(|shot| shot.length >= ROPE_SHORT * 48.0) && high(414.0, PET).is_none());
    assert!(high(66.0, PET).is_some_and(|shot| shot.length <= ROPE_LONG * 48.0) && high(65.0, PET).is_none());
    assert!(high(480.0 - MUZZLE_HEIGHT * 56.0 - 420.0, LARGE).is_some() && high(480.0 - MUZZLE_HEIGHT * 56.0 - 420.0, PET).is_none());
    assert!(high(480.0, PET).is_none() && high(520.0, PET).is_none());
    let from = |x: f64, keepouts: &[Rect]| shot_for(point(x, 480.0), &[floor(), shelf.clone()], keepouts, PET);
    assert!(from(240.0, &[boxed]).is_some() && from(140.0, &[boxed]).is_none());
    assert!(shot_for(point(100.0, 420.0), &[perch("shelf", 180.0, 494.0, 380.0)], &[], PET).is_none());
    for x in [200.0, 240.0, 280.0, 330.0, 400.0, 470.0, 520.0, 560.0] {
        if let Some(shot) = from(x, &[boxed]) {
            assert!(shot.length >= ROPE_SHORT * 48.0 && shot.length <= ROPE_LONG * 48.0 && shot.muzzle.y - shot.hook.y >= ROPE_ELEVATION * shot.length, "{x}");
        }
    }
    let clear = from(280.0, &[boxed]).unwrap_or_else(|| panic!("no shot"));
    assert!(segment_hits(clear.muzzle, clear.hook, &boxed, ROPE_MARGIN));
    assert_eq!(from(280.0, &[boxed, keepout(310.0, 380.0, 100.0, 30.0)]).as_ref(), Some(&clear));
    assert_eq!(from(280.0, &[boxed, keepout(310.0, 384.0, 100.0, 30.0)]), None);
    assert_eq!(from(280.0, &[boxed, keepout(300.0, 420.0, 16.0, 30.0)]), None);
    assert_eq!(from(280.0, &[boxed, keepout(312.0, 340.0, 100.0, 80.0)]).as_ref(), Some(&clear));
    assert_eq!(from(280.0, &[boxed, keepout(311.5, 340.0, 100.0, 80.0)]), None);
    let reach = along(clear.muzzle, clear.hook, 422.0);
    assert_eq!(from(280.0, &[boxed, keepout(reach - ROPE_MARGIN - 0.5 - 20.0, 400.0, 20.0, 20.0)]).as_ref(), Some(&clear));
    assert_eq!(from(280.0, &[boxed, keepout(reach - ROPE_MARGIN + 0.5 - 20.0, 400.0, 20.0, 20.0)]), None);
}

#[test]
fn a_shot_chooses_the_least_rope_and_detour_among_all_perches_the_first_one_among_equals() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let upper = perch("upper", 306.0, 494.0, 350.0);
    let twin = perch("twin", 306.0, 494.0, 380.0);
    let boxed = [solid(300.0, 500.0, 380.0, 420.0)];
    let surface = |x: f64, perches: &[Perch], keepouts: &[Rect]| shot_for(point(x, 480.0), perches, keepouts, PET).map(|shot| shot.surface);
    assert_eq!(surface(400.0, &[floor(), upper.clone(), shelf.clone()], &[]).as_deref(), Some("shelf"));
    assert_eq!(surface(400.0, &[floor(), shelf.clone(), upper.clone()], &[]).as_deref(), Some("shelf"));
    assert_eq!(surface(400.0, std::slice::from_ref(&upper), &[]).as_deref(), Some("upper"));
    assert_eq!(surface(400.0, &[upper.clone(), shelf.clone()], &boxed).as_deref(), Some("shelf"));
    assert_eq!(surface(400.0, &[upper], &boxed), None);
    assert_eq!(surface(400.0, &[twin.clone(), shelf.clone()], &[]).as_deref(), Some("twin"));
    assert_eq!(surface(400.0, &[shelf, twin], &[]).as_deref(), Some("shelf"));
    let (near, far) = (perch("left", 306.0, 400.0, 380.0), perch("right", 560.0, 654.0, 380.0));
    assert_eq!(surface(480.0, &[near.clone(), far.clone()], &[]).as_deref(), Some("left"));
    assert_eq!(surface(480.0, &[far.clone(), near.clone()], &[]).as_deref(), Some("right"));
    assert_eq!(surface(481.0, &[near, far], &[]).as_deref(), Some("right"));
    assert_eq!(surface(400.0, &[], &[]), None);
    assert_eq!(surface(400.0, &[floor()], &[]), None);
}

#[test]
fn a_shot_keeps_every_law_on_generated_stages() {
    let mut next = stream(43);
    let (mut granted, mut refused, mut swings) = (0, 0, 0);
    for stage in 0..STAGES {
        let size = [PET, SMALL, LARGE][next(3) as usize];
        let feet = point(100.0 + next(1600) / 4.0, 300.0 + next(800) / 4.0);
        let mut perches = Vec::new();
        for index in 0..1 + next(3) as usize {
            let x0 = next(2000) / 4.0;
            perches.push(perch(&format!("p{index}"), x0, x0 + 20.0 + next(800) / 4.0, 100.0 + next(1400) / 4.0));
        }
        let mut keepouts: Vec<Rect> = perches.iter().map(|edge| solid(edge.x0, edge.x1, edge.y, edge.y + 40.0)).collect();
        for _ in 0..next(3) as usize {
            keepouts.push(keepout(next(2400) / 4.0, 100.0 + next(1600) / 4.0, next(240) / 4.0, next(240) / 4.0));
        }
        let Some(shot) = shot_for(feet, &perches, &keepouts, size) else {
            refused += 1;
            continue;
        };
        granted += 1;
        if shot.reel == Reeling::Swing {
            swings += 1;
        }
        let target = perches.iter().find(|edge| edge.surface == shot.surface).unwrap_or_else(|| panic!("stage {stage}: no target"));
        let inset = HOOK_INSET;
        assert!(feet.y - target.y >= ROPE_RISE * size.height && shot.hook.y == target.y - HOOK_LIFT, "stage {stage}");
        if target.x1 - target.x0 >= 2.0 * inset {
            assert!(shot.hook.x >= target.x0 + inset && shot.hook.x <= target.x1 - inset, "stage {stage}");
        } else {
            assert_eq!(shot.hook.x, (target.x0 + target.x1) / 2.0, "stage {stage}");
        }
        assert_eq!(shot.muzzle, point(feet.x + shot.facing.sign() * MUZZLE_FORWARD * size.width, feet.y - MUZZLE_HEIGHT * size.height), "stage {stage}");
        assert_eq!(shot.facing, if shot.hook.x < feet.x { Facing::Left } else { Facing::Right }, "stage {stage}");
        let mut dropped = false;
        for blocker in &keepouts {
            if !holding(blocker, point(shot.hook.x, target.y)) {
                assert!(!segment_hits(shot.muzzle, shot.hook, blocker, ROPE_MARGIN), "stage {stage}");
                dropped = dropped || segment_hits(shot.hook, point(shot.hook.x, shot.hook.y + shot.length), blocker, ROPE_MARGIN);
            }
        }
        assert_eq!(shot.reel, if (shot.hook.x - shot.muzzle.x).abs() <= ZIP_SLANT * (shot.muzzle.y - shot.hook.y) || dropped { Reeling::Zip } else { Reeling::Swing }, "stage {stage}");
        assert!((shot.length - distance(shot.hook, shot.muzzle)).abs() < 1e-9, "stage {stage}");
        assert!(shot.length >= ROPE_SHORT * size.height && shot.length <= ROPE_LONG * size.height && shot.muzzle.y - shot.hook.y >= ROPE_ELEVATION * shot.length, "stage {stage}");
        assert_eq!(shot_holds(&shot, &perches, &keepouts).as_ref(), Some(&shot), "stage {stage}");
    }
    assert!(granted > STAGES / 12 && swings > STAGES / 60 && refused > STAGES / 4, "{granted} granted, {swings} swings, {refused} refused");
}

#[test]
fn the_hook_flies_on_a_straight_line_at_a_constant_speed_ends_on_its_target_exactly_and_comes_back_twice_as_fast() {
    let (muzzle, hook) = (point(292.0, 448.8), point(330.0, 379.0));
    let length = 38.0_f64.hypot(69.8);
    let ticks = hook_ticks(muzzle, hook, HOOK_SPEED);
    assert_eq!(ticks, (length / 10.0).ceil() as Ticks);
    assert_eq!([hook_step(muzzle, hook, HOOK_SPEED, 0), hook_step(muzzle, hook, HOOK_SPEED, -2)], [muzzle, muzzle]);
    for tick in 1..ticks {
        let tip = hook_step(muzzle, hook, HOOK_SPEED, tick);
        assert!((distance(tip, muzzle) - 10.0 * tick as f64).abs() < 1e-9, "tick {tick}");
        assert!(((tip.x - muzzle.x) * (hook.y - muzzle.y) - (tip.y - muzzle.y) * (hook.x - muzzle.x)).abs() < 1e-9, "tick {tick}");
    }
    assert_eq!([hook_step(muzzle, hook, HOOK_SPEED, ticks), hook_step(muzzle, hook, HOOK_SPEED, ticks + 5)], [hook, hook]);
    let tip = hook_step(muzzle, hook, HOOK_SPEED, 5);
    let back = hook_ticks(tip, muzzle, HOOK_RETURN);
    assert_eq!(back, 3);
    assert!((distance(hook_step(tip, muzzle, HOOK_RETURN, 1), tip) - 20.0).abs() < 5e-10);
    assert_eq!(hook_step(tip, muzzle, HOOK_RETURN, back), muzzle);
    assert_eq!(hook_ticks(muzzle, muzzle, HOOK_SPEED), 0);
    assert_eq!([hook_step(muzzle, muzzle, HOOK_SPEED, 0), hook_step(muzzle, muzzle, HOOK_SPEED, 1)], [muzzle, muzzle]);
    assert_eq!([hook_ticks(point(0.0, 0.0), point(6.0, 8.0), HOOK_SPEED), hook_ticks(point(0.0, 0.0), point(6.0, 8.5), HOOK_SPEED), hook_ticks(point(0.0, 0.0), point(60.0, 80.0), HOOK_SPEED)], [1, 2, 10]);
    assert_eq!(hook_step(point(0.0, 0.0), point(60.0, 80.0), HOOK_SPEED, 9), point(54.0, 72.0));
}

#[test]
fn the_haul_begins_at_rest_where_the_actor_stood_and_zips_at_the_gathered_speed_along_the_taut_line_to_where_the_hoist_begins() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let shot = shot_for(point(400.0, 520.0), std::slice::from_ref(&shelf), &[], PET).unwrap_or_else(|| panic!("no shot"));
    assert_eq!(shot.reel, Reeling::Zip);
    let start = haul_of(&shot, shot.length, PET);
    assert_eq!((start.rope, start.before), (shot.length, start.hand));
    assert!(distance(start.hand, shot.muzzle) < 1e-9 && (start.x - 400.0).abs() < 1e-9 && (start.y - 520.0).abs() < 1e-9);
    let hung = haul_of(&shot, 0.0, PET);
    assert_eq!(hung, Haul { rope: 0.0, hand: shot.hook, before: shot.hook, x: shot.hook.x - MUZZLE_FORWARD * 40.0, y: shot.hook.y + MUZZLE_HEIGHT * 48.0 });
    let half = haul_of(&shot, shot.length / 2.0, PET);
    assert!((half.x - (start.x + hung.x) / 2.0).abs() < 1e-9 && (half.y - (start.y + hung.y) / 2.0).abs() < 1e-9);
    let mirrored = shot_for(point(420.0, 520.0), &[perch("stub", 380.0, 420.0, 380.0)], &[], PET).unwrap_or_else(|| panic!("no mirrored shot"));
    assert_eq!(mirrored.facing, Facing::Left);
    assert_eq!(haul_of(&mirrored, 0.0, PET).x, mirrored.hook.x + MUZZLE_FORWARD * 40.0);
    let end = REEL_LEAST * 48.0;
    let (mut haul, mut ticks) = (start, 0);
    while haul.rope > end && ticks < 500 {
        let next = zip_step(&shot, haul, ticks, PET);
        assert!((haul.rope - next.rope - ((ZIP_SPEED * smoothstep((ticks + 1) as f64 / ZIP_RAMP as f64)) / 64.0).min(haul.rope - end)).abs() < 1e-9, "tick {ticks}");
        assert_eq!(next, Haul { before: haul.hand, ..haul_of(&shot, next.rope, PET) });
        assert_eq!(haul_step(&shot, haul, ticks, PET), next);
        haul = next;
        ticks += 1;
    }
    assert_eq!(haul.rope, end);
    assert_eq!(ticks, haul_ticks(&shot, PET));
    assert_eq!(ticks, ZIP_RAMP + ((shot.length - end - (5.5 * ZIP_SPEED) / 64.0) / (ZIP_SPEED / 64.0)).ceil() as Ticks);
    assert_eq!(zip_step(&shot, haul, ticks, PET).rope, end);
    let short = shot_for(point(400.0, 450.2), std::slice::from_ref(&shelf), &[], PET).unwrap_or_else(|| panic!("no short shot"));
    assert!(short.length < REEL_LEAST * 48.0);
    assert_eq!(haul_ticks(&short, PET), 0);
    assert_eq!(zip_step(&short, haul_of(&short, short.length, PET), 0, PET).rope, short.length);
    assert_eq!([zip_step(&shot, haul_of(&shot, 20.0, PET), 30, PET).rope, sway_step(&shot, haul_of(&shot, 20.0, PET), 30, PET).rope], [20.0, 20.0]);
}

#[test]
fn the_haul_swings_on_a_slanted_rope_as_the_pendulum_of_the_swing_module_the_feet_under_the_hands() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let slanted = shot_for(point(250.0, 480.0), &[floor(), shelf], &[], PET).unwrap_or_else(|| panic!("no shot"));
    let least = REEL_LEAST * 48.0;
    assert_eq!(slanted.reel, Reeling::Swing);
    let mut haul = haul_of(&slanted, slanted.length, PET);
    let (mut ticks, mut lowest, mut crossed) = (0, haul.hand.y, false);
    while haul.rope > least && ticks < 500 {
        let reel = reel_step(slanted.hook, haul.hand, haul.before, haul.rope, least, ticks);
        let next = sway_step(&slanted, haul, ticks, PET);
        assert_eq!(next, Haul { rope: reel.length, hand: reel.bob, before: haul.hand, x: reel.bob.x - MUZZLE_FORWARD * 40.0, y: reel.bob.y + MUZZLE_HEIGHT * 48.0 }, "tick {ticks}");
        assert_eq!(haul_step(&slanted, haul, ticks, PET), next);
        assert!(distance(next.hand, slanted.hook) < next.rope + 1e-9 && distance(next.hand, haul.hand) < REEL_CAP / 64.0 + 1e-9, "tick {ticks}");
        assert!((haul.rope - next.rope - ((REEL_SPEED * ((ticks + 1).min(REEL_RAMP)) as f64) / REEL_RAMP as f64 / 64.0).min(haul.rope - least)).abs() < 5e-10, "tick {ticks}");
        lowest = lowest.max(next.hand.y);
        crossed = crossed || next.hand.x > slanted.hook.x;
        haul = next;
        ticks += 1;
    }
    assert_eq!(haul.rope, least);
    assert_eq!(ticks, haul_ticks(&slanted, PET));
    assert_eq!(ticks, REEL_RAMP - 1 + ((slanted.length - least - (4.5 * REEL_SPEED) / 64.0) / (REEL_SPEED / 64.0)).ceil() as Ticks);
    assert!(crossed && lowest > slanted.muzzle.y && lowest < slanted.hook.y + slanted.length);
}

#[test]
fn the_landing_lies_inside_the_hook_towards_the_middle_of_the_perch_and_a_meant_miss_past_its_nearer_end() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let shot = shot_for(point(400.0, 520.0), std::slice::from_ref(&shelf), &[], PET).unwrap_or_else(|| panic!("no shot"));
    let hooked = |x: f64| Shot { hook: point(x, 379.0), ..shot.clone() };
    assert_eq!(landing_for(&shot, &shelf, PET), point(400.0 + 20.0 + MANTLE_INSET, 380.0));
    assert_eq!([landing_for(&hooked(330.0), &shelf, PET), landing_for(&hooked(470.0), &shelf, PET)], [point(358.0, 380.0), point(442.0, 380.0)]);
    let stub = perch("stub", 380.0, 420.0, 380.0);
    assert_eq!([landing_for(&hooked(400.0), &stub, PET), landing_for(&hooked(401.0), &stub, PET)], [point(420.0, 380.0), point(380.0, 380.0)]);
    assert_eq!(miss_of(&hooked(330.0), &shelf), point(306.0 - ROPE_MISS_OVERSHOOT, 379.0));
    assert_eq!(miss_of(&hooked(470.0), &shelf), point(494.0 + ROPE_MISS_OVERSHOOT, 379.0));
    assert_eq!(miss_of(&hooked(400.0), &shelf), point(292.0, 379.0));
}

#[test]
fn a_shot_holds_as_it_is_follows_its_edge_up_and_down_and_loses_an_edge_that_moved_too_far_left_the_hook_vanished_or_was_blocked() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let boxed = solid(300.0, 500.0, 380.0, 420.0);
    let shot = shot_for(point(280.0, 480.0), &[floor(), shelf.clone()], &[boxed], PET).unwrap_or_else(|| panic!("no shot"));
    assert_eq!(shot_holds(&shot, &[floor(), shelf.clone()], &[boxed]).as_ref(), Some(&shot));
    for dy in [-ROPE_FOLLOW, -3.0, 5.0, ROPE_FOLLOW] {
        let held = shot_holds(&shot, &[floor(), perch("shelf", 306.0, 494.0, 380.0 + dy)], &[solid(300.0, 500.0, 380.0 + dy, 420.0 + dy)]).unwrap_or_else(|| panic!("{dy}"));
        assert_eq!(held.hook, point(310.0, 379.0 + dy));
        assert!((held.length - distance(point(310.0, 379.0 + dy), shot.muzzle)).abs() < 1e-12);
        assert_eq!(Shot { hook: shot.hook, length: shot.length, ..held }, shot);
    }
    let lost = |perches: &[Perch]| shot_holds(&shot, perches, &[]).is_none();
    assert!(lost(&[floor(), perch("shelf", 306.0, 494.0, 380.0 + ROPE_FOLLOW + 0.5)]) && lost(&[floor(), perch("shelf", 306.0, 494.0, 380.0 - ROPE_FOLLOW - 0.5)]));
    assert!(lost(&[floor(), perch("shelf", 310.5, 494.0, 380.0)]) && !lost(&[floor(), perch("shelf", 310.0, 494.0, 380.0)]));
    assert!(lost(&[floor(), perch("other", 306.0, 494.0, 380.0)]) && lost(&[floor()]));
    assert_eq!(shot_holds(&shot, &[floor(), shelf.clone()], &[boxed, keepout(300.0, 420.0, 16.0, 30.0)]), None);
    assert_eq!(shot_holds(&shot, &[floor(), shelf], &[boxed, keepout(100.0, 420.0, 16.0, 30.0)]).as_ref(), Some(&shot));
}

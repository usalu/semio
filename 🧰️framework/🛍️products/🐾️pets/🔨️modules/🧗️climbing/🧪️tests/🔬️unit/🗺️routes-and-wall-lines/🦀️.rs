//! 🗺️ The routes up, over and down, and the lunges, pitches and ways of the wall lines of the climbing unit laws. See [`super`].

use super::*;

#[test]
fn routes_offer_every_way_the_gear_allows_a_standing_ladder_first_then_the_wall_then_a_ladder_of_its_own_then_the_rope() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let (left, right) = (pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let pitches = [left.clone(), right.clone()];
    let standing = [ladder_for(&floor(), &shelf, &right, &boxed, LARGE).unwrap_or_else(|| panic!("no standing ladder"))];
    let up = |gear: &[Gear], ladders: &[LadderStand], grip: f64, x: f64| route_of(x, &floor(), &shelf, gear, PET, grip, &pitches, ladders, &boxed).map(|legs| means(&legs));
    let every = [Gear::Climb, Gear::Ladder, Gear::Grapple];
    let legs = route_of(200.0, &floor(), &shelf, &every, PET, GRIP_BUDGET, &pitches, &[], &boxed).unwrap_or_else(|| panic!("no legs"));
    assert_eq!(means(&legs), ["wall", "wall", "raise", "raise", "grapple"]);
    assert_eq!(legs[0], Leg::Wall { at: 280.0, pitch: &pitches[0], hold: WallHold { x: 280.0, y: 480.0, over: false }, exit: &pitches[0], goal: rim_of(&left, PET) });
    assert_eq!(legs[1], Leg::Wall { at: 520.0, pitch: &pitches[1], hold: WallHold { x: 520.0, y: 480.0, over: false }, exit: &pitches[1], goal: rim_of(&right, PET) });
    assert_eq!(legs[2], Leg::Raise { at: 300.0 - LADDER_LEAN * 93.0, ladder: ladder_for(&floor(), &shelf, &left, &boxed, PET).unwrap_or_else(|| panic!("no ladder")) });
    assert_eq!(legs[4], Leg::Grapple { at: 200.0, shot: shot_for(point(200.0, 480.0), std::slice::from_ref(&shelf), &boxed, PET).unwrap_or_else(|| panic!("no shot")) });
    let joined = route_of(200.0, &floor(), &shelf, &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed).unwrap_or_else(|| panic!("no legs"));
    assert_eq!(means(&joined), ["ladder", "wall", "wall", "grapple"]);
    assert!(matches!(joined[0], Leg::Ladder { at, ladder, up: true } if at == standing[0].foot.x && std::ptr::eq(ladder, &standing[0])));
    assert_eq!(up(&[Gear::Climb], &[], GRIP_BUDGET, 200.0), Some(vec!["wall", "wall"]));
    assert_eq!(up(&[Gear::Ladder], &[], GRIP_BUDGET, 200.0), Some(vec!["raise", "raise"]));
    assert_eq!(up(&[Gear::Grapple], &[], GRIP_BUDGET, 200.0), Some(vec!["grapple"]));
    assert_eq!(up(&[Gear::Grapple], &standing, GRIP_BUDGET, 200.0), Some(vec!["ladder", "grapple"]));
    assert_eq!([up(&[Gear::Parachute], &standing, GRIP_BUDGET, 200.0), up(&[], &standing, GRIP_BUDGET, 200.0), up(&[Gear::Parachute], &[], GRIP_BUDGET, 200.0)], [None, None, None]);
}

#[test]
fn routes_shoot_from_where_the_actor_stands_when_that_works_else_from_the_point_of_its_perch_nearest_to_the_target_and_refuse_a_wall_the_grip_does_not_last_for() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let (left, right) = (pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let pitches = [left.clone(), right];
    let shot = |x: f64, from: &Perch| {
        route_of(x, from, &shelf, &[Gear::Grapple], PET, GRIP_BUDGET, &[], &[], &boxed).map(|legs| match &legs[0] {
            Leg::Grapple { at, .. } => *at,
            _ => f64::NAN,
        })
    };
    assert_eq!(
        route_of(400.0, &floor(), &shelf, &[Gear::Grapple], PET, GRIP_BUDGET, &pitches, &[], &boxed).map(|legs| legs[0].clone()),
        Some(Leg::Grapple { at: 400.0, shot: shot_for(point(400.0, 480.0), std::slice::from_ref(&shelf), &boxed, PET).unwrap_or_else(|| panic!("no shot")) })
    );
    assert_eq!(shot(100.0, &floor()), Some(306.0));
    assert_eq!(shot(100.0, &perch("floor", 0.0, 150.0, 480.0)), None);
    assert_eq!([shot(100.0, &perch("floor", 0.0, 250.0, 480.0)), shot(600.0, &perch("floor", 540.0, 640.0, 480.0)), shot(630.0, &perch("floor", 540.0, 640.0, 480.0))], [Some(250.0), Some(600.0), Some(540.0)]);
    let need = (WALL_GRAB_TICKS + climb_ticks(480.0, rim_of(&left, PET)) + MANTLE_TICKS) as f64;
    let climbs = |gear: &[Gear], grip: f64| route_of(200.0, &floor(), &shelf, gear, PET, grip, &pitches, &[], &boxed).map(|legs| means(&legs));
    assert_eq!(climbs(&[Gear::Climb], need), Some(vec!["wall", "wall"]));
    assert_eq!(climbs(&[Gear::Climb], need - 1.0), None);
    assert_eq!(climbs(&[Gear::Climb, Gear::Grapple], 10.0), Some(vec!["grapple"]));
}

#[test]
fn routes_lead_down_over_the_rim_by_the_wall_or_a_standing_ladder_never_by_a_rope_or_a_new_ladder_and_find_nothing_where_nothing_joins() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let (left, right) = (pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let pitches = [left.clone(), right.clone()];
    let standing = [ladder_for(&floor(), &shelf, &right, &boxed, LARGE).unwrap_or_else(|| panic!("no standing ladder"))];
    let every = [Gear::Climb, Gear::Ladder, Gear::Grapple];
    let legs = route_of(400.0, &shelf, &floor(), &every, PET, GRIP_BUDGET, &pitches, &[], &boxed).unwrap_or_else(|| panic!("no legs"));
    assert_eq!(
        legs,
        [
            Leg::Wall { at: 328.0, pitch: &pitches[0], hold: WallHold { x: 280.0, y: rim_of(&left, PET), over: true }, exit: &pitches[0], goal: 480.0 },
            Leg::Wall { at: 472.0, pitch: &pitches[1], hold: WallHold { x: 520.0, y: rim_of(&right, PET), over: true }, exit: &pitches[1], goal: 480.0 }
        ]
    );
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Ladder, Gear::Grapple], PET, GRIP_BUDGET, &pitches, &[], &boxed), None);
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Ladder], PET, GRIP_BUDGET, &pitches, &standing, &boxed), Some(vec![Leg::Ladder { at: ledge_of(&right, PET), ladder: &standing[0], up: false }]));
    let need = (WALL_HANG_TICKS + climb_ticks(rim_of(&left, PET), 480.0)) as f64;
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Climb], PET, need, &pitches, &[], &boxed).map(|legs| legs.len()), Some(2));
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Climb], PET, need - 1.0, &pitches, &[], &boxed), None);
    let far = perch("far", 20.0, 120.0, 200.0);
    assert_eq!(route_of(200.0, &floor(), &far, &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed), None);
    assert_eq!(route_of(400.0, &shelf, &shelf, &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed), None);
    assert_eq!(route_of(200.0, &floor(), &floor(), &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed), None);
    assert_eq!(route_of(200.0, &floor(), &shelf, &[Gear::Climb, Gear::Ladder], PET, GRIP_BUDGET, &[], &[], &boxed), None);
}

fn column() -> [Pitch; 2] {
    [pitch("top-left", "top", Facing::Left, 300.0, 300.0, 380.0), pitch("low-left", "low", Facing::Left, 300.0, 410.0, 480.0)]
}

#[test]
fn a_lunge_crosses_between_two_pitches_of_one_wall_line_one_above_the_other_within_its_reach() {
    let [top, low] = column();
    assert!(crossable(&low, &top, PET) && crossable(&top, &low, PET));
    assert!(!crossable(&top, &top, PET));
    assert!(!crossable(&low, &pitch("top-right", "top", Facing::Right, 300.0, 300.0, 380.0), PET));
    assert!(crossable(&low, &pitch("top-left", "top", Facing::Left, 300.0 + WALL_FOLLOW, 300.0, 380.0), PET));
    assert!(!crossable(&low, &pitch("top-left", "top", Facing::Left, 300.0 + WALL_FOLLOW + 0.5, 300.0, 380.0), PET));
    assert!(!crossable(&low, &pitch("top-left", "top", Facing::Left, 300.0, 300.0, 411.0), PET));
    let reach = CROSS_REACH * 48.0 - GRIP_BITE;
    assert!(crossable(&pitch("low-left", "low", Facing::Left, 300.0, 380.0 + reach, 480.0), &top, PET));
    assert!(!crossable(&pitch("low-left", "low", Facing::Left, 300.0, 380.0 + reach + 0.5, 480.0), &top, PET));
    assert!(!crossable(&low, &pitch("stub", "top", Facing::Left, 300.0, 372.5, 380.0), PET));
    assert!(crossable(&low, &pitch("stub", "top", Facing::Left, 300.0, 372.0, 380.0), PET));
}

#[test]
fn a_lunge_leads_from_the_nearest_hold_of_one_pitch_to_the_nearest_hold_of_the_other() {
    let [top, low] = column();
    assert_eq!([lunge_path(&low, &top, PET, 0.0), lunge_path(&low, &top, PET, 1.0)], [point(280.0, rim_of(&low, PET)), point(280.0, foot_of(&top, PET))]);
    assert_eq!([lunge_path(&top, &low, PET, 0.0), lunge_path(&top, &low, PET, 1.0)], [point(280.0, foot_of(&top, PET)), point(280.0, rim_of(&low, PET))]);
    for step in 0..=LUNGE_TICKS {
        let phase = step as f64 / LUNGE_TICKS as f64;
        assert_eq!(lunge_path(&low, &top, PET, phase), hoist_path(point(280.0, rim_of(&low, PET)), point(280.0, foot_of(&top, PET)), 48.0, phase));
    }
}

#[test]
fn a_wall_line_holds_every_pitch_reached_by_lunges_from_the_top_down_the_nearest_first_among_equals() {
    let [top, low] = column();
    let far = pitch("far-left", "far", Facing::Left, 300.0, 100.0, 200.0);
    let twin = pitch("twin-left", "twin", Facing::Left, 302.0, 300.0, 380.0);
    let pitches = [far, low.clone(), top.clone(), twin];
    let names = |chain: Vec<&Pitch>| chain.iter().map(|member| member.wall.clone()).collect::<Vec<_>>();
    assert_eq!(names(chain_of(&pitches[1], &pitches, PET)), ["top-left", "low-left"]);
    assert!(std::ptr::eq(chain_of(&pitches[1], &pitches, PET)[0], &pitches[2]));
    assert_eq!(names(chain_of(&pitches[2], &pitches, PET)), ["top-left", "low-left"]);
    assert_eq!(names(chain_of(&pitches[3], &pitches, PET)), ["twin-left", "low-left"]);
    assert_eq!(names(chain_of(&pitches[0], &pitches, PET)), ["far-left"]);
    let higher = pitch("higher-left", "top", Facing::Left, 300.0, 300.0, 370.0);
    let ordered = [higher, top, low.clone()];
    assert_eq!(names(chain_of(&ordered[2], &ordered, PET)), ["top-left", "low-left"]);
    assert_eq!(names(chain_of(&low, &[], PET)), ["low-left"]);
}

#[test]
fn a_way_along_a_wall_line_takes_hold_climbs_lunges_and_mantles_tick_by_tick_and_costs_every_tick() {
    let pitches = column();
    let chain: Vec<&Pitch> = pitches.iter().collect();
    let goal = rim_of(&pitches[0], PET);
    let path = wall_path(&chain, 1, Entry::Grab, 270.0, 480.0, 0, goal, true, PET);
    let works: Vec<Handwork> = path.iter().map(|clamber| clamber.work).collect();
    let count = |work: Handwork| works.iter().filter(|&&each| each == work).count() as Ticks;
    assert_eq!([count(Handwork::Grab), count(Handwork::Lunge), count(Handwork::Mantle), count(Handwork::Hang)], [WALL_GRAB_TICKS, LUNGE_TICKS, MANTLE_TICKS, 0]);
    assert_eq!(path[WALL_GRAB_TICKS as usize - 1], Clamber { x: 280.0, y: 480.0, hold: 1, work: Handwork::Grab });
    assert!(path[..WALL_GRAB_TICKS as usize].windows(2).all(|pair| pair[0].x <= pair[1].x));
    let climbed: Vec<f64> = path.iter().filter(|clamber| clamber.work == Handwork::Climb && clamber.hold == 1).map(|clamber| clamber.y).collect();
    assert_eq!(climbed, climbed_to(480.0, rim_of(&pitches[1], PET)));
    let lunged = path.iter().position(|clamber| clamber.work == Handwork::Lunge).unwrap_or_default();
    assert_eq!(path[lunged + LUNGE_TICKS as usize - 1], Clamber { x: 280.0, y: foot_of(&pitches[0], PET), hold: 0, work: Handwork::Lunge });
    let upper: Vec<f64> = path.iter().filter(|clamber| clamber.work == Handwork::Climb && clamber.hold == 0).map(|clamber| clamber.y).collect();
    assert_eq!(upper, climbed_to(foot_of(&pitches[0], PET), goal));
    assert_eq!(path.last().copied(), Some(Clamber { x: ledge_of(&pitches[0], PET), y: 300.0, hold: 0, work: Handwork::Mantle }));
    assert_eq!(wall_cost(&path), path.len() as f64 * GRIP_CLIMB);
    let single = [left()];
    let alone: Vec<&Pitch> = single.iter().collect();
    let up = wall_path(&alone, 0, Entry::Grab, 280.0, 480.0, 0, rim_of(&single[0], PET), true, PET);
    assert_eq!(wall_cost(&up), (WALL_GRAB_TICKS + climb_ticks(480.0, rim_of(&single[0], PET)) + MANTLE_TICKS) as f64 * GRIP_CLIMB);
    let down = wall_path(&alone, 0, Entry::Hang, 0.0, 0.0, 0, 480.0, false, PET);
    assert_eq!(wall_cost(&down), (WALL_HANG_TICKS + climb_ticks(rim_of(&single[0], PET), 480.0)) as f64 * GRIP_CLIMB);
    assert_eq!(down[0], Clamber { x: mantle_path(&single[0], PET, 0.9).x, y: mantle_path(&single[0], PET, 0.9).y, hold: 0, work: Handwork::Hang });
    assert_eq!(down[WALL_HANG_TICKS as usize - 1], Clamber { x: 280.0, y: rim_of(&single[0], PET), hold: 0, work: Handwork::Hang });
    let still = wall_path(&alone, 0, Entry::Cling, 0.0, 400.0, 0, 400.0, false, PET);
    assert!(still.is_empty());
}

fn climbed_to(from: f64, goal: f64) -> Vec<f64> {
    let mut heights = Vec::new();
    let mut height = from;
    let mut tick = 0;
    while height != goal {
        height = climb_step(height, goal, tick);
        heights.push(height);
        tick += 1;
    }
    heights
}

#[test]
fn routes_climb_a_wall_line_up_and_down_and_name_the_pitch_their_climb_ends_on() {
    let pitches = column();
    let (floor, top, low) = (floor(), perch("top", 306.0, 494.0, 300.0), perch("low", 306.0, 494.0, 410.0));
    let up = route_of(200.0, &floor, &top, &[Gear::Climb], PET, GRIP_BUDGET, &pitches, &[], &[]).unwrap_or_else(|| panic!("no way up"));
    assert_eq!(up, [Leg::Wall { at: 280.0, pitch: &pitches[1], hold: WallHold { x: 280.0, y: 480.0, over: false }, exit: &pitches[0], goal: rim_of(&pitches[0], PET) }]);
    let down = route_of(400.0, &top, &floor, &[Gear::Climb], PET, GRIP_BUDGET, &pitches, &[], &[]).unwrap_or_else(|| panic!("no way down"));
    assert_eq!(down, [Leg::Wall { at: ledge_of(&pitches[0], PET), pitch: &pitches[0], hold: WallHold { x: 280.0, y: rim_of(&pitches[0], PET), over: true }, exit: &pitches[1], goal: 480.0 }]);
    let chain: Vec<&Pitch> = pitches.iter().collect();
    let need = wall_cost(&wall_path(&chain, 1, Entry::Grab, 280.0, 480.0, 0, rim_of(&pitches[0], PET), true, PET));
    assert!(route_of(200.0, &floor, &top, &[Gear::Climb], PET, need, &pitches, &[], &[]).is_some());
    assert!(route_of(200.0, &floor, &top, &[Gear::Climb], PET, need - 1.0, &pitches, &[], &[]).is_none());
    assert_eq!(route_of(200.0, &floor, &low, &[Gear::Climb], PET, GRIP_BUDGET, &pitches, &[], &[]).map(|legs| legs.len()), Some(1));
}

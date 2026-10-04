//! 👁️ Unit tests of attention: turning round (and turning back in the middle of a turn), the see-through presence under the pointer, a walker that turns before it sets out, an idle pet that turns to a pointer clearly behind it — once a second at most, and never while it concentrates, stands still, walks or sleeps —, the head that leans after the gaze, and the greeting of a pointer that has come to rest.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, a meadow to stand on, the events and a pet at rest

use super::*;
use crate::clock::SHY_STEP;
use crate::draft::{draft_of, sealed};
use crate::locomotion::stroll;
use crate::population::CENTRED;
use crate::rig::{rest_pose, solve_rig};
use crate::schema::{Hushed, Menagerie, Over, PetMode, Pointed, Rate, StageEvent, Ticked, Tuned, Unpointed};
use crate::stage::tests::{meadow, plain, pointed, rested, run, ticked, troupe, LEVEL};
use crate::stage::{advance, frame_of};

/// 🫡️ The stage after its only actor greeted towards `x`.
fn greeted(menagerie: &Menagerie, stage: Stage, x: f64) -> Stage {
    let mut draft = draft_of(menagerie, stage);
    let now = draft.stage.tick;
    hail(&mut draft, 0, x, now);
    sealed(draft)
}

#[test]
fn an_actor_that_is_asked_to_turn_again_in_the_middle_of_a_turn_turns_back_from_where_its_drawing_is() {
    let menagerie = plain(&troupe());
    let mut stage = rested(&menagerie, 0.0);
    stage.actors[0].clip = None;
    let stage = greeted(&menagerie, stage, 530.0);
    assert_eq!((stage.actors[0].facing, stage.actors[0].faced, stage.actors[0].activity), (Facing::Left, stage.tick + 8, Activity::Greet));
    let stage = run(&menagerie, stage, 3, |_| {});
    let before = frame_of(&menagerie, &stage).actors[0].clone();
    assert!(before.bones[0] == 2.0 * smoothstep(3.0 / 8.0) - 1.0);
    let stage = greeted(&menagerie, stage, 570.0);
    assert_eq!((stage.actors[0].facing, stage.actors[0].faced), (Facing::Right, stage.tick + 3));
    let after = frame_of(&menagerie, &stage).actors[0].clone();
    assert!((after.bones[0] * after.facing.sign() - before.bones[0] * before.facing.sign()).abs() < 1e-12);
    let stage = run(&menagerie, stage, 3, |_| {});
    assert!(frame_of(&menagerie, &stage).actors[0].bones[0] == 1.0);
}

#[test]
fn an_actor_turns_see_through_while_the_pointer_rests_on_it() {
    let menagerie = troupe();
    let kind = &menagerie.species[0];
    let stage = advance(&menagerie, meadow(&menagerie, 6, &["mossy"]), &ticked(32));
    let mossy = stage.actors[0].clone();
    let over = |dx: f64, dy: f64| Some(Point { x: mossy.x + dx, y: mossy.y + dy });
    assert!(presence_of(None, &mossy, kind) == 1.0 && presence_of(over(24.5, -10.0), &mossy, kind) == 1.0 && presence_of(over(0.0, -34.5), &mossy, kind) == 1.0 && presence_of(over(-24.5, 0.0), &mossy, kind) == 1.0);
    assert!(presence_of(over(24.0, -10.0), &mossy, kind) == SHY_OPACITY && presence_of(over(0.0, 4.0), &mossy, kind) == SHY_OPACITY && presence_of(over(-24.0, -34.0), &mossy, kind) == SHY_OPACITY);
    let pointed = StageEvent::Pointed(Pointed { x: mossy.x, y: mossy.y - 10.0, over: Over::Free });
    let shy = advance(&menagerie, stage.clone(), &[pointed.clone(), StageEvent::Ticked(Ticked { ticks: 1 })]);
    assert!(shy.actors[0].opacity == 1.0 - SHY_STEP);
    assert_eq!(frame_of(&menagerie, &shy).rate, Rate::Full);
    let seen = advance(&menagerie, shy, &ticked(40));
    assert!(seen.actors[0].opacity == SHY_OPACITY);
    let back = advance(&menagerie, seen, &[StageEvent::Unpointed(Unpointed {}), StageEvent::Ticked(Ticked { ticks: 11 })]);
    assert!(back.actors[0].opacity == 1.0);
    let still = advance(&menagerie, stage, &[StageEvent::Tuned(Tuned { mode: PetMode::Still }), pointed]);
    let frame = frame_of(&menagerie, &still);
    assert!(still.actors[0].opacity == 1.0 && frame.actors[0].opacity == SHY_OPACITY);
    assert_eq!((frame.rate, frame.wake), (Rate::Rest, None));
}

#[test]
fn a_walker_turns_round_before_it_sets_out() {
    let menagerie = troupe();
    let mut draft = draft_of(&menagerie, advance(&menagerie, meadow(&menagerie, 6, &["mossy"]), &ticked(32)));
    let x = draft.stage.actors[0].x;
    draft.stage.actors[0].facing = Facing::Right;
    stroll(&mut draft, 0, x - 30.0, Some("stroll".to_string()), 32);
    assert_eq!(heading_of(&draft.stage, &draft.stage.actors, &draft.kinds, 0, 33), Some(Facing::Left));
    let begun = advance(&menagerie, sealed(draft), &ticked(1));
    assert_eq!((begun.actors[0].facing, begun.actors[0].faced, begun.actors[0].since), (Facing::Left, 41, 32));
    assert!(begun.actors[0].x == x && frame_of(&menagerie, &begun).actors[0].facing == Facing::Left);
    let halfway = advance(&menagerie, begun, &ticked(4));
    assert!(halfway.actors[0].x == x && halfway.actors[0].facing == Facing::Left);
    let frame = frame_of(&menagerie, &halfway);
    assert_eq!(frame.rate, Rate::Full);
    assert!(frame.actors[0].bones.iter().step_by(2).all(|entry| *entry == 0.0));
    let turned = advance(&menagerie, halfway, &ticked(4));
    assert_eq!((turned.actors[0].facing, turned.actors[0].faced, turned.actors[0].since), (Facing::Left, 41, 41));
    assert!(turned.actors[0].x == x && squeeze_of(&turned.actors[0], turned.tick) == 1.0);
    let striding = advance(&menagerie, turned, &ticked(1));
    assert!(striding.actors[0].x == x - 36.0 / 64.0);
}

#[test]
fn an_idle_actor_turns_round_to_a_pointer_that_is_clearly_behind_it() {
    let menagerie = troupe();
    let facings = |x: f64| {
        let mut seen = Vec::new();
        run(&menagerie, advance(&menagerie, rested(&menagerie, 0.0), &[pointed(x)]), 128, |after| seen.push(after.actors[0].facing));
        seen
    };
    assert!(facings(550.0 - 20.0 - 11.0).iter().all(|&facing| facing == Facing::Right));
    assert!(facings(550.0 + 300.0).iter().all(|&facing| facing == Facing::Right));
    assert!(facings(550.0 - 20.0 - 13.0).iter().all(|&facing| facing == Facing::Left));
    let mut stage = advance(&menagerie, rested(&menagerie, 0.0), &[pointed(100.0), StageEvent::Ticked(Ticked { ticks: 1 })]);
    assert_eq!((stage.actors[0].facing, stage.actors[0].faced, stage.actors[0].activity), (Facing::Left, stage.tick + 8, Activity::Idle));
    assert!(stage.actors[0].x == 550.0);
    let mut widths = Vec::new();
    for step in 0..=8 {
        let frame = frame_of(&menagerie, &stage);
        assert_eq!(frame.rate, if step < 8 { Rate::Full } else { Rate::Half }, "{step}");
        widths.push(frame.actors[0].bones[0]);
        stage = advance(&menagerie, stage, &ticked(1));
    }
    assert!(widths[0] < 0.0 && widths[4].abs() < 1e-12 && widths[8] > 0.0);
    assert!(widths.windows(2).all(|pair| pair[1] > pair[0]));
}

#[test]
fn a_pointer_that_crosses_over_and_back_turns_an_actor_once_a_second_at_most() {
    let menagerie = troupe();
    let mut stage = rested(&menagerie, 0.0);
    let mut turns = Vec::new();
    for tick in 0..256 {
        let before = stage.actors[0].facing;
        stage = advance(&menagerie, stage, &[pointed(if tick % 4 < 2 { 100.0 } else { 1000.0 }), StageEvent::Ticked(Ticked { ticks: 1 })]);
        if stage.actors[0].facing != before {
            turns.push(stage.tick);
        }
    }
    assert!((2..=4).contains(&turns.len()), "{turns:?}");
    assert!(turns.windows(2).all(|pair| pair[1] - pair[0] >= 64), "{turns:?}");
}

#[test]
fn a_pointer_turns_nobody_who_concentrates_stands_still_walks_or_sleeps_or_once_it_is_old_news() {
    let menagerie = troupe();
    let pointed_at = |stage: Stage| advance(&menagerie, stage, &[pointed(100.0), StageEvent::Ticked(Ticked { ticks: 128 })]);
    let standing = rested(&menagerie, 0.0);
    assert_eq!(pointed_at(standing.clone()).actors[0].facing, Facing::Left);
    let hushed = pointed_at(advance(&menagerie, standing.clone(), &[StageEvent::Hushed(Hushed { quiet: true })]));
    assert_eq!(hushed.actors[0].facing, Facing::Right);
    assert_eq!(pointed_at(advance(&menagerie, standing.clone(), &[StageEvent::Tuned(Tuned { mode: PetMode::Still })])).actors[0].facing, Facing::Right);
    let mut asleep = standing.clone();
    (asleep.actors[0].activity, asleep.actors[0].clip) = (Activity::Sleep, None);
    assert_eq!(pointed_at(asleep).actors[0].facing, Facing::Right);
    let mut walking = standing;
    (walking.actors[0].activity, walking.actors[0].goal) = (Activity::Walk, 780.0);
    let walked = pointed_at(walking);
    assert_eq!((walked.actors[0].facing, walked.actors[0].activity), (Facing::Right, Activity::Walk));
    let stale = advance(&menagerie, hushed, &[StageEvent::Ticked(Ticked { ticks: 192 }), StageEvent::Hushed(Hushed { quiet: false }), StageEvent::Ticked(Ticked { ticks: 128 })]);
    assert_eq!(stale.actors[0].facing, Facing::Right);
}

#[test]
fn the_bone_that_carries_the_first_eye_leans_after_the_gaze() {
    let menagerie = troupe();
    let kind = &menagerie.species[0];
    let bone = kind.bones.iter().position(|bone| bone.id == kind.face.eyes[0].bone).unwrap_or_else(|| panic!("the first eye of {} sits on no bone", kind.id));
    let bones = |menagerie: &Menagerie, stage: &Stage| frame_of(menagerie, stage).actors[0].bones.clone();
    let upright = |stage: &Stage| {
        let mut centred = stage.clone();
        centred.actors[0].gaze = CENTRED;
        bones(&menagerie, &centred)
    };
    let stage = advance(&menagerie, rested(&menagerie, 0.0), &[pointed(1100.0), StageEvent::Ticked(Ticked { ticks: 64 })]);
    let gaze = stage.actors[0].gaze;
    assert!(gaze.x > 0.8);
    assert_ne!(bones(&menagerie, &stage), upright(&stage));
    let bare = plain(&menagerie);
    let mut pose = rest_pose(kind);
    pose[bone] = BonePose { x: pose[bone].x + 1.5 * gaze.x, y: pose[bone].y + gaze.y, rotation: pose[bone].rotation + 5.0 * gaze.x, scale_x: 1.0, scale_y: 1.0 };
    let mut clipless = stage.clone();
    clipless.actors[0].clip = None;
    assert_eq!(bones(&bare, &clipless), solve_rig(kind, &pose));
    assert_eq!(bones(&menagerie, &advance(&menagerie, stage.clone(), &[StageEvent::Hushed(Hushed { quiet: true })])), upright(&stage));
    assert_eq!(bones(&menagerie, &advance(&menagerie, stage.clone(), &[StageEvent::Tuned(Tuned { mode: PetMode::Still })])), solve_rig(kind, &rest_pose(kind)));
    let (mut mirrored, mut settled) = (stage.clone(), stage);
    (mirrored.actors[0].facing, mirrored.actors[0].gaze) = (Facing::Left, Gaze { x: 0.0 - gaze.x, y: gaze.y, vx: 0.0, vy: 0.0 });
    settled.actors[0].gaze = Gaze { x: gaze.x, y: gaze.y, vx: 0.0, vy: 0.0 };
    assert_eq!(bones(&menagerie, &mirrored), bones(&menagerie, &settled));
}

#[test]
fn an_idle_actor_greets_a_pointer_that_has_come_to_rest_beside_it_and_pays_with_curiosity() {
    let menagerie = troupe();
    let beside = 550.0 + 20.0 + 20.0;
    let stage = advance(&menagerie, rested(&menagerie, 1.0), &[pointed(beside)]);
    let since = stage.tick;
    let waiting = advance(&menagerie, stage.clone(), &ticked(31));
    assert_eq!(waiting.actors[0].activity, Activity::Idle);
    let greeting = advance(&menagerie, waiting, &ticked(1));
    let body = &greeting.actors[0];
    assert_eq!((greeting.tick, body.activity, body.since, body.facing, body.partner.as_ref(), body.clip.as_deref()), (since + 32, Activity::Greet, since + 32, Facing::Right, None, Some("wave")));
    assert!(body.x == 550.0 && body.until - greeting.tick >= 128 && body.until - greeting.tick < 320 && (body.needs.curiosity - 0.4).abs() < 1e-9);
    assert_eq!(frame_of(&menagerie, &greeting).actors[0].activity, Activity::Greet);
    assert_eq!(advance(&menagerie, stage, &ticked(32)), greeting);
    let rests = |x: f64, y: f64, quiet: bool| advance(&menagerie, rested(&menagerie, 1.0), &[StageEvent::Hushed(Hushed { quiet }), StageEvent::Pointed(Pointed { x, y, over: Over::Free }), StageEvent::Ticked(Ticked { ticks: 40 })]).actors[0].activity;
    assert_eq!(rests(beside, LEVEL, false), Activity::Greet);
    assert_eq!(rests(550.0, LEVEL, false), Activity::Idle);
    assert_eq!(rests(550.0 + 4.0 * 30.0, LEVEL, false), Activity::Idle);
    assert_eq!(rests(beside, LEVEL, true), Activity::Idle);
    let dull = advance(&menagerie, rested(&menagerie, 0.45), &[pointed(beside), StageEvent::Ticked(Ticked { ticks: 40 })]);
    assert_eq!(dull.actors[0].activity, Activity::Idle);
    let spent = advance(&menagerie, rested(&menagerie, 0.5), &[pointed(beside), StageEvent::Ticked(Ticked { ticks: 32 })]);
    assert!(spent.actors[0].activity == Activity::Greet && spent.actors[0].needs.curiosity == 0.0);
    let mut moving = rested(&menagerie, 1.0);
    for tick in 0..128 {
        moving = advance(&menagerie, moving, &[pointed(beside + f64::from(tick % 20)), StageEvent::Ticked(Ticked { ticks: 1 })]);
        assert_eq!(moving.actors[0].activity, Activity::Idle, "{tick}");
    }
}

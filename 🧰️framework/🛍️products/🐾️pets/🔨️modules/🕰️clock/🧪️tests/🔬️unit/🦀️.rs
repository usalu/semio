//! 🕰️ Unit tests of the clock: the lulls it jumps over — a quiet hour of sleep, and a resting pointer up to the ticks on which it can change something — end in the very stage that ticking through them yields, and a frame at rate 0 wakes exactly when its picture changes.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, a meadow to stand on, the events and a pet at rest

use super::*;
use crate::draft::draft_of;
use crate::population::CENTRED;
use crate::schema::{Glanced, Hushed, Over, Point, Pointed, Rate, StageEvent, Ticked, Unpointed};
use crate::stage::tests::{meadow, plain, pointed, rested, run, ticked, troupe};
use crate::stage::{advance, frame_of};

#[test]
fn a_quiet_hour_of_sleep_is_jumped_over() {
    let menagerie = troupe();
    let mut stage = advance(&menagerie, meadow(&menagerie, 2, &["pebble"]), &[StageEvent::Hushed(Hushed { quiet: true }), StageEvent::Ticked(Ticked { ticks: 64 })]);
    let sleeper = &mut stage.actors[0];
    (sleeper.activity, sleeper.clip, sleeper.since, sleeper.until, sleeper.gaze) = (Activity::Sleep, Some("doze".to_string()), 0, 64 + 64 * 3600, CENTRED);
    let draft = draft_of(&menagerie, stage.clone());
    assert_eq!(lull(&menagerie, &draft, 1_000_000), 64 * 3600 - 1);
    assert_eq!(frame_of(&menagerie, &stage).rate, Rate::Quarter);
    let later = advance(&menagerie, stage.clone(), &ticked(64 * 3600 - 1));
    assert_eq!((later.tick, &later.actors), (64 * 3600 + 63, &stage.actors));
    let near = advance(&menagerie, stage.clone(), &[StageEvent::Pointed(Pointed { x: stage.actors[0].x, y: stage.actors[0].y, over: Over::Free }), StageEvent::Ticked(Ticked { ticks: 1 })]);
    assert_eq!(near.actors[0].activity, Activity::Idle);
    let away = advance(&menagerie, near, &[StageEvent::Unpointed(Unpointed {}), StageEvent::Glanced(Glanced { points: vec![Point { x: 0.0, y: 0.0 }] })]);
    assert_eq!((away.pointer, away.glances.len()), (None, 1));
}

#[test]
fn a_resting_pointer_is_jumped_over_up_to_the_ticks_on_which_it_can_change_something() {
    let troupe = troupe();
    for menagerie in [plain(&troupe), troupe] {
        for (curiosity, x) in [(1.0, 590.0), (0.0, 100.0), (0.0, 590.0)] {
            let mut start = rested(&menagerie, curiosity);
            if menagerie.species[0].clips.is_empty() {
                start.actors[0].clip = None;
            }
            let start = advance(&menagerie, start, &[pointed(x)]);
            for chunk in [1, 31, 32, 33, 200, 2000] {
                assert_eq!(advance(&menagerie, start.clone(), &ticked(chunk)), run(&menagerie, start.clone(), chunk as Ticks, |_| {}), "{x} in one chunk of {chunk}");
            }
            let mut stage = start;
            let mut step = 0;
            while step < 384 {
                let frame = frame_of(&menagerie, &stage);
                let ticks = match frame.wake {
                    Some(wake) if frame.rate == Rate::Rest => Ticks::min(wake - stage.tick, 384 - step),
                    _ => 1,
                };
                assert!(ticks > 0);
                let woken = advance(&menagerie, stage.clone(), &ticked(ticks.unsigned_abs()));
                assert_eq!(woken, run(&menagerie, stage.clone(), ticks, |_| {}), "{x} woken {ticks} ticks after {}", stage.tick);
                if frame.rate == Rate::Rest {
                    assert_eq!(frame_of(&menagerie, &run(&menagerie, stage.clone(), ticks - 1, |_| {})).actors, frame.actors);
                }
                stage = woken;
                step += ticks;
            }
        }
    }
}

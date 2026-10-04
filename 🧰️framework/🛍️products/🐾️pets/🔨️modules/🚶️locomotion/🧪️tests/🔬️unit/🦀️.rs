//! 🏃️ Unit tests of locomotion: a hopping gait that finds somebody in its way in the middle of a hop finishes that hop on the spot and comes to rest at its end.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, the events and time passing

use super::*;
use crate::draft::{draft_of, sealed};
use crate::schema::{Facing, StageEvent, Ticked};
use crate::stage::tests::{actor_of, summoned, surveyed, ticked, troupe};
use crate::stage::{advance, open_stage};

#[test]
fn a_hopper_that_finds_somebody_in_its_way_in_mid_hop_finishes_the_hop_on_the_spot() {
    let menagerie = troupe();
    let floor = surveyed(1280.0, 400.0, &[("floor", 0.0, 1280.0, 400.0)], &[]);
    let mut draft = draft_of(&menagerie, advance(&menagerie, open_stage(3), &[floor, summoned(&["mossy", "sparky"]), StageEvent::Ticked(Ticked { ticks: 32 })]));
    assert_eq!(beat_of(draft.kinds[1], clip_of(draft.kinds[1], Some("bounce"))), 32);
    (draft.stage.actors[1].x, draft.stage.actors[1].facing) = (300.0, Facing::Right);
    draft.stage.actors[0].x = 300.0 + shoulders(draft.kinds[0], draft.kinds[1]) + MEET_GAP + 3.0;
    stroll(&mut draft, 1, 600.0, Some("bounce".to_string()), 32);
    assert!(!hindered(&draft, 1, 1) && hindered(&draft, 1, 32));
    let stopped = advance(&menagerie, sealed(draft), &ticked(5));
    let sparky = actor_of(&stopped, "sparky");
    assert!(sparky.x == 303.0 && sparky.goal == 303.0);
    assert_eq!((sparky.activity, sparky.since), (Activity::Walk, 32));
    let rested = advance(&menagerie, stopped, &ticked(27));
    let sparky = actor_of(&rested, "sparky");
    assert!(sparky.x == 303.0 && sparky.activity == Activity::Idle && sparky.since == 64);
}

//! 🗓️ Unit tests of the schedule: a pair may be drawn on whole seconds, after the warm-up or the gap of the mode, while two actors are sociable and the stage is neither quiet nor still.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, a meadow to stand on and time passing

use super::*;
use crate::stage::advance;
use crate::stage::tests::{meadow, ticked, troupe};

#[test]
fn pairs_are_drawn_on_whole_seconds_after_the_warm_up() {
    let menagerie = troupe();
    let stage = advance(&menagerie, meadow(&menagerie, 8, &["mossy", "sparky"]), &ticked(16));
    assert_eq!(pairing_tick(&stage, &stage.actors, 17), WARMUP);
    assert_eq!(pairing_tick(&stage, &stage.actors, WARMUP + 1), WARMUP + 64);
    let mut met = stage.clone();
    met.met = 1000;
    assert_eq!(pairing_tick(&met, &met.actors, 17), 6784);
    let mut quiet = stage.clone();
    quiet.quiet = true;
    assert_eq!(pairing_tick(&quiet, &quiet.actors, 17), -1);
    let mut alone = stage.clone();
    alone.actors.truncate(1);
    assert_eq!(pairing_tick(&alone, &alone.actors, 17), -1);
    let mut still = stage;
    still.mode = PetMode::Still;
    assert_eq!(pairing_tick(&still, &still.actors, 17), -1);
}

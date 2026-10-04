//! 📽️ Unit tests of the projection: a frame draws its actors back to front — the one higher up first, at the same height the species id that is smaller by UTF-16 code units, as the TypeScript twin compares strings — whatever the order of the actors in the stage.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, a meadow to stand on and time passing

use super::*;
use crate::stage::advance;
use crate::stage::tests::{meadow, ticked, troupe, TROUPE};

#[test]
fn a_frame_is_drawn_back_to_front() {
    let menagerie = troupe();
    let stage = advance(&menagerie, meadow(&menagerie, 13, &TROUPE), &ticked(40));
    let frame = frame_of(&menagerie, &stage);
    assert_eq!(frame.actors.len(), 5);
    for pair in frame.actors.windows(2) {
        assert!(pair[0].y < pair[1].y || (pair[0].y == pair[1].y && pair[0].species < pair[1].species), "{} before {}", pair[0].species, pair[1].species);
    }
    let mut reversed = stage.clone();
    reversed.actors.reverse();
    assert_eq!(frame_of(&menagerie, &reversed), frame);
    let mut one = stage.actors[0].clone();
    let mut other = stage.actors[0].clone();
    (one.species, other.species) = ("\u{ff5e}".to_string(), "\u{1f600}".to_string());
    assert!(behind(&other, &one) && !behind(&one, &other) && one.species < other.species);
    other.y = f64::NAN;
    assert!(behind(&other, &one));
    one.y -= 1.0;
    other.y = one.y + 1.0;
    assert!(behind(&one, &other) && !behind(&other, &one));
}

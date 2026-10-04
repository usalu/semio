//! ✏️ Unit tests of the draft: one actor per known species in `menagerie.species` order, stretches carved by open intervals, and the small expressions every part of the stage shares — units, blinks, facings, clips and hovers — as the TypeScript twin writes them.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe and a meadow to stand on

use super::*;
use crate::stage::tests::{meadow, troupe};

#[test]
fn a_draft_keeps_one_actor_per_known_species() {
    let menagerie = troupe();
    let mut stage = meadow(&menagerie, 9, &["thorny", "mossy"]);
    let mut stranger = stage.actors[0].clone();
    stranger.species = "stranger".to_string();
    let mut twin = stage.actors[1].clone();
    twin.x += 100.0;
    let first = stage.actors[1].x;
    stage.actors.extend([stranger, twin]);
    let draft = draft_of(&menagerie, stage);
    assert_eq!(draft.stage.actors.iter().map(|actor| actor.species.as_str()).collect::<Vec<_>>(), ["mossy", "thorny"]);
    assert_eq!(draft.streams, [0, 2]);
    assert_eq!(draft.kinds.iter().map(|kind| kind.id.as_str()).collect::<Vec<_>>(), ["mossy", "thorny"]);
    assert!(draft.stage.actors[1].x == first);
}

#[test]
fn stretches_are_carved_by_open_intervals() {
    assert_eq!(carve(&[[0.0, 10.0]], 3.0, 5.0), [[0.0, 3.0], [5.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0]], -5.0, 0.0), [[0.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0]], 10.0, 12.0), [[0.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0]], -1.0, 4.0), [[4.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0], [20.0, 30.0]], 8.0, 22.0), [[0.0, 8.0], [22.0, 30.0]]);
    assert!(carve(&[[0.0, 10.0]], 0.0, 10.0).is_empty());
}

#[test]
fn units_dwells_and_facings_are_the_expressions_of_the_twin() {
    assert!(unit_of(0) == 0.0 && unit_of(u32::MAX) == 4_294_967_295.0 / 4_294_967_296.0 && unit_of(0x8000_0000) == 0.5);
    assert_eq!((blink_at(100, 0.0), blink_at(100, 0.5), blink_at(100, unit_of(u32::MAX))), (228, 356, 483));
    assert_eq!((facing_to(1.0, 1.0), facing_to(0.0, 1.0), facing_to(f64::NAN, 1.0)), (Facing::Right, Facing::Left, Facing::Left));
    let kind = &troupe().species[0];
    assert_eq!(clip_at(kind, Activity::Fidget, 0.0).as_deref(), Some("wiggle"));
    assert_eq!(clip_at(kind, Activity::Fidget, 0.5).as_deref(), Some("stretch"));
    assert_eq!(clip_at(kind, Activity::Fidget, 1.0).as_deref(), Some("stretch"));
    assert_eq!(clip_at(kind, Activity::Hop, 0.3).as_deref(), Some("breathe"));
    assert_eq!(clips_of(&troupe().species[1], Activity::Walk), ["bounce"]);
    assert_eq!(breath_of(kind).map(|clip| clip.id.as_str()), Some("breathe"));
    assert!(hover_of(kind) == 0.0 && hover_of(&troupe().species[4]) == 14.0);
}

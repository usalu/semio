//! 💞️ Unit tests of sociability: a sulk that runs out mends the rapport of the pair once the partner has stopped sulking, and lets go of the partner.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, a meadow to stand on and time passing

use super::*;
use crate::stage::advance;
use crate::stage::tests::{actor_of, meadow, ticked, troupe};

#[test]
fn a_sulk_that_runs_out_mends_the_rapport_and_lets_go_of_the_partner() {
    let menagerie = troupe();
    let mut stage = advance(&menagerie, meadow(&menagerie, 21, &["mossy", "thorny"]), &ticked(32));
    let now = stage.tick;
    for actor in &mut stage.actors {
        if actor.species == "mossy" {
            (actor.activity, actor.since, actor.until, actor.partner) = (Activity::Sulk, now - 64, now + 1, Some("thorny".to_string()));
        } else {
            (actor.activity, actor.since, actor.until, actor.partner) = (Activity::Idle, now, now + 100_000, None);
        }
    }
    let (first, second) = if menagerie.species.iter().position(|kind| kind.id == "mossy") < menagerie.species.iter().position(|kind| kind.id == "thorny") { ("mossy", "thorny") } else { ("thorny", "mossy") };
    stage.rapports = vec![Rapport { between: [first.to_string(), second.to_string()], drift: -0.15 }];
    stage.met = now;
    let mended = advance(&menagerie, stage, &ticked(1));
    let sulker = actor_of(&mended, "mossy");
    assert_eq!((sulker.activity, sulker.partner.as_deref()), (Activity::Idle, None));
    assert_eq!(mended.rapports.len(), 1);
    assert_eq!(mended.rapports[0].drift, rapport_after(rapport_faded(-0.15, 1), Activity::Sulk));
    assert!(mended.rapports[0].drift > -0.15);
}

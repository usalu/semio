//! 💞️ Unit tests of sociability: a sulk that runs out mends the rapport of the pair once the partner has stopped sulking, and lets go of the partner.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, a meadow to stand on and time passing

use super::*;
use crate::stage::advance;
use crate::stage::tests::{actor_of, meadow, ticked, troupe};
use semio_framework_pets_contract::schema::{Feeling, Mood, Played, Stage, StageEvent};
use semio_framework_pets_contract::testkit::{entries, fixture, typed};

/// 🪑️ One free actor at rest with hand interactions permitted.
fn keyboard_stage(menagerie: &Menagerie, species: &str, state: &str, grumpy: bool) -> Stage {
    let mut stage = meadow(menagerie, 7, &[species]);
    stage.play = true;
    let body = &mut stage.actors[0];
    body.state = state.to_string();
    body.state_since = 0;
    body.activity = Activity::Idle;
    body.since = 0;
    body.until = 100000;
    body.partner = None;
    body.leaving = false;
    body.feeling = Feeling { mood: if grumpy { Mood::Grumpy } else { Mood::Content }, intensity: if grumpy { 0.9 } else { 0.25 }, since: 0 };
    stage
}

/// 🎬️ A keyboard gesture and the end of its authored clip, if any.
fn keyboard_gesture(menagerie: &Menagerie, stage: Stage, species: &str, deed: Deed) -> Stage {
    let asked = advance(menagerie, stage, &[StageEvent::Played(Played { species: species.to_string(), deed })]);
    if asked.actors[0].activity != Activity::Trick { return asked; }
    let ticks = asked.actors[0].until - asked.tick;
    advance(menagerie, asked, &ticked(ticks))
}

#[test]
fn keyboard_gestures_follow_the_shared_neutral_vectors_and_natural_state_route() {
    let menagerie: Menagerie = typed(&fixture("chemistry-rules")["menagerie"]);
    let vectors = fixture("keyboard-gestures");
    for vector in entries(&vectors["vectors"]) {
        let species = vector["species"].as_str().unwrap();
        for grumpy in [false, true] {
            let stage = keyboard_stage(&menagerie, species, vector["state"].as_str().unwrap(), grumpy);
            let deed: Deed = typed(&vector["deed"]);
            let asked = advance(&menagerie, stage.clone(), &[StageEvent::Played(Played { species: species.to_string(), deed })]);
            assert_eq!(serde_json::to_value(&asked.actors[0].trick).unwrap(), vector["trick"]);
            assert_eq!(asked.actors[0].warmth, stage.actors[0].warmth);
            assert_eq!(keyboard_gesture(&menagerie, stage, species, deed).actors[0].state, vector["after"].as_str().unwrap());
        }
    }
    let route = &vectors["route"];
    let species = route["species"].as_str().unwrap();
    let mut stage = keyboard_stage(&menagerie, species, route["state"].as_str().unwrap(), false);
    for (index, deed) in entries(&route["deeds"]).iter().enumerate() {
        stage = keyboard_gesture(&menagerie, stage, species, typed(deed));
        assert_eq!(stage.actors[0].state, route["states"][index].as_str().unwrap());
    }
}

#[test]
fn keyboard_gestures_keep_actor_and_stage_permissions() {
    let menagerie: Menagerie = typed(&fixture("chemistry-rules")["menagerie"]);
    let vectors = fixture("keyboard-gestures");
    for deed in entries(&vectors["deeds"]) {
        let deed: Deed = typed(deed);
        let stage = keyboard_stage(&menagerie, "glow", "bright", false);
        for changes in entries(&vectors["blocked"]) {
            let mut value = serde_json::to_value(&stage).unwrap();
            for (key, value_) in changes.as_object().unwrap() { value["actors"][0][key.as_str()] = value_.clone(); }
            let blocked: Stage = typed(&value);
            assert_eq!(advance(&menagerie, blocked.clone(), &[StageEvent::Played(Played { species: "glow".to_string(), deed })]), blocked);
        }
        let mut blocked = stage.clone();
        blocked.play = false;
        assert_eq!(advance(&menagerie, blocked.clone(), &[StageEvent::Played(Played { species: "glow".to_string(), deed })]), blocked);
        blocked = stage;
        blocked.mode = PetMode::Still;
        assert_eq!(advance(&menagerie, blocked.clone(), &[StageEvent::Played(Played { species: "glow".to_string(), deed })]), blocked);
    }
    let mut stage = keyboard_stage(&menagerie, "mist", "fluffy", false);
    stage.actors[0].activity = Activity::Purr;
    let asked = advance(&menagerie, stage, &[StageEvent::Played(Played { species: "mist".to_string(), deed: Deed::Stroke })]);
    assert_eq!(asked.actors[0].activity, Activity::Purr);
    assert_eq!(asked.actors[0].trick, None);
}

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

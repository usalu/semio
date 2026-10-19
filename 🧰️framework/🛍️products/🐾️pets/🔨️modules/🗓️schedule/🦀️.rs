//! 🗓️ The schedule of the stage: the changes that are due at a known tick and depend on more than one actor — the next whole second on which a pair may be drawn (`pairing_tick`) and the next on which somebody who waits off stage may arrive (`arrival_tick`).
//!
//! The clock jumps to these ticks (`lull`) and the frame reports them (`wake`). A new time horizon of the stage is a function here and a line in both of them.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🗓️schedule/🟦️.ts — the TypeScript twin

use crate::draft::index_of;
use semio_framework_pets_character::behavior::MODE_LIMITS;
use semio_framework_pets_contract::schema::{Activity, Actor, Menagerie, PetMode, Stage, Ticks, TICKS_PER_SECOND};
use std::borrow::Borrow;
use semio_framework_pets_contract::schema::Footing;
use semio_framework_pets_contract::schema::PressPhase;
use semio_framework_pets_contract::schema::Facing;
use semio_framework_pets_contract::schema::Gear;
use semio_framework_pets_contract::schema::Gait;
use semio_framework_pets_contract::schema::Mood;
use semio_framework_pets_character::feeling::Sighting;
use semio_framework_pets_contract::schema::Pitch;
use semio_framework_pets_contract::schema::Perch;
use semio_framework_pets_contract::schema::Slug;
use semio_framework_pets_contract::schema::Fixture;
use semio_framework_pets_contract::schema::Species;

//#region 🔖️Constants
const WARMUP: Ticks = 1280;
//#endregion 🔖️Constants

//#region 🔖️Schedule
/// 🙌️ Whether an actor can be drawn into an encounter: standing idle and whole on a perch, without a partner, not leaving.
pub fn sociable(actor: &Actor, tick: Ticks) -> bool {
    actor.activity == Activity::Idle && actor.partner.is_none() && !actor.leaving && actor.opacity == 1.0 && actor.footing == Footing::Perch && !circled(&actor.hover, tick)
}

/// 📅️ The first whole second at or after tick `from` on which the stage may draw a pair, or −1 while it may not: the mode has encounters, it is not a quiet time, nobody has a partner, at least two actors are sociable, and the warm-up (20 s before the first encounter) or the gap of the mode since the last change of a rapport has passed.
pub fn pairing_tick<A: Borrow<Actor>>(stage: &Stage, actors: &[A], from: Ticks) -> Ticks {
    let limits = MODE_LIMITS[stage.mode];
    if limits.encounter_gap == 0 || stage.quiet {
        return -1;
    }
    let mut free = 0;
    for actor in actors {
        let actor: &Actor = actor.borrow();
        if actor.partner.is_some() {
            return -1;
        }
        if sociable(actor, from) {
            free += 1;
        }
    }
    if free < 2 {
        return -1;
    }
    let open = if stage.met == 0 { WARMUP } else { stage.met + limits.encounter_gap };
    let first = if from > open { from } else { open };
    (first + TICKS_PER_SECOND - 1).div_euclid(TICKS_PER_SECOND) * TICKS_PER_SECOND
}

/// 🚏️ The first whole second at or after tick `from` on which a species that is wanted but not on stage may arrive, or −1 while nobody waits, no perch exists or the stage is still (a still stage lets them arrive with its events).
pub fn arrival_tick<A: Borrow<Actor>>(menagerie: &Menagerie, stage: &Stage, actors: &[A], from: Ticks) -> Ticks {
    if stage.mode == PetMode::Still || stage.perches.is_empty() {
        return -1;
    }
    if menagerie.species.iter().any(|kind| stage.wanted.contains(&kind.id) && index_of(actors, &kind.id).is_none()) {
        return (from + TICKS_PER_SECOND - 1).div_euclid(TICKS_PER_SECOND) * TICKS_PER_SECOND;
    }
    -1
}
//#endregion 🔖️Schedule

/// 👀️ Read actors through their closed-form state and mood at the chemistry beat.
pub(crate) fn sightings_of(menagerie: &Menagerie, stage: &Stage, tick: Ticks) -> Vec<Sighting> {
    stage.actors.iter().filter_map(|actor| {
        let kind = menagerie.species.iter().find(|kind| kind.id == actor.species)?;
        if actor.leaving { return None; }
        let standing = state_at(kind, &actor.state, actor.state_since, tick);
        let feeling = settled(actor.feeling, kind.mood, tick);
        Some(Sighting { species: actor.species.clone(), state: standing.state, held: tick - standing.since, mood: feeling.mood, intensity: feeling.intensity, activity: actor.activity, trick: actor.trick.clone(), x: actor.x, y: actor.y, width: kind.size.width, height: kind.size.height })
    }).collect()
}

/// ⏭️ The first multiple of a beat at or after a tick.
fn beat_after(from: Ticks, beat: Ticks) -> Ticks {
    (from + beat - 1).div_euclid(beat) * beat
}

/// ⚗️ The earliest due reaction, cooling expiry or held-state threshold.
fn due_tick(menagerie: &Menagerie, stage: &Stage, from: Ticks) -> Ticks {
    let sightings = sightings_of(menagerie, stage, from);
    if !trials_of(menagerie, &sightings, &stage.coolings, from, &stage.rapports).is_empty() { return from; }
    let mut due = -1;
    for cooling in &stage.coolings {
        if cooling.until > from && (due < 0 || cooling.until < due) { due = cooling.until; }
    }
    for reaction in &menagerie.chemistry {
        for wanted in [&reaction.when, &reaction.near].into_iter().chain(reaction.unless.iter()) {
            if let Some(held) = wanted.held {
                for sighting in &sightings {
                    let ripe = from - sighting.held + held_ticks(held);
                    if ripe > from && (due < 0 || ripe < due) { due = ripe; }
                }
            }
        }
    }
    due
}

/// 🥁️ The first beat on which contagion or chemistry can change a resting stage.
pub(crate) fn beat_tick(menagerie: &Menagerie, stage: &Stage, from: Ticks) -> Ticks {
    if stage.mode == PetMode::Still { return -1; }
    let mut staying = 0;
    let mut stirred = false;
    for actor in &stage.actors {
        if actor.leaving { continue; }
        staying += 1;
        if let Some(kind) = menagerie.species.iter().find(|kind| kind.id == actor.species) {
            if MOOD_SPREADS[MOODS.iter().position(|mood| *mood == actor.feeling.mood).unwrap_or(0)] > 0.0 && calms_at(actor.feeling, kind.mood) > from { stirred = true; }
        }
    }
    if staying < 2 { return -1; }
    let contagion = if stirred { beat_after(from, CONTAGION_BEAT) } else { -1 };
    let due = if !menagerie.chemistry.is_empty() && !stage.quiet { due_tick(menagerie, stage, from) } else { -1 };
    let chemistry = if due < 0 { -1 } else { beat_after(due, CHEMISTRY_BEAT) };
    if contagion < 0 { chemistry } else if chemistry < 0 || contagion < chemistry { contagion } else { chemistry }
}

/// 🖱️ A newly moved pointer or scroll must reach the gesture recogniser on the next tick.
pub(crate) fn sampling(stage: &Stage, from: Ticks) -> bool {
    stage.play && stage.pointer.is_some() && stage.press.phase == PressPhase::Idle && stage.mode != PetMode::Still && !stage.quiet && (from - stage.pointed <= 1 || from - stage.scrolled <= 1)
}

//#region 🔖️Mischief
/// 📮️ A pusher's working position beside a marked element.
#[derive(Clone)]
pub(crate) enum Post { Wall { pitch: Pitch, y: f64 }, Perch { perch: Perch, x: f64 } }

/// 🔭️ One actor and fixture with a feasible working position.
pub(crate) struct Prospect { pub actor: Slug, pub fixture: Fixture, pub post: Post, pub side: Facing, pub room: f64 }

/// 🛎️ Choose the post offering most room, then least travel, preserving survey order.
fn post_of(kind: &Species, fixture: &Fixture, x: f64, y: f64, perches: &[Perch], pitches: &[Pitch], width: f64) -> Option<(Post, Facing, f64)> {
    let half = kind.size.width / 2.0;
    let bottom = fixture.y + fixture.height;
    let mut best: Option<(Post, Facing, f64)> = None;
    let mut farness = 0.0;
    for perch in perches {
        let Some(station) = station_for(fixture, &[], std::slice::from_ref(perch), width) else { continue; };
        if perch.x1 - perch.x0 < kind.size.width { continue; }
        let stand = clamp(station.x - station.side.sign() * half, perch.x0 + half, perch.x1 - half);
        let room = smaller(station.room, kind.size.width);
        let far = (stand - x).abs() + (perch.y - hover_of(kind) - y).abs();
        if (if station.side == Facing::Right { stand + half > fixture.x } else { stand - half < fixture.x + fixture.width }) || best.as_ref().is_some_and(|(_, _, previous)| room < *previous || (room == *previous && far >= farness)) { continue; }
        best = Some((Post::Perch { perch: perch.clone(), x: stand }, station.side, room));
        farness = far;
    }
    if !kind.gear.contains(&Gear::Climb) || kind.locomotion.gait == Gait::Float { return best; }
    for pitch in pitches {
        let rim = rim_of(pitch, kind.size);
        let foot = foot_of(pitch, kind.size);
        let feet = clamp(bottom, rim, foot);
        if rim > foot || !(feet - kind.size.height < bottom && feet > fixture.y) { continue; }
        let Some(wall) = station_for(fixture, std::slice::from_ref(pitch), &[], width) else { continue; };
        let room = smaller(wall.room, kind.size.width);
        let far = (cling_of(pitch, kind.size) - x).abs() + (feet - y).abs();
        if best.as_ref().is_some_and(|(_, _, previous)| room < *previous || (room == *previous && far >= farness)) { continue; }
        best = Some((Post::Wall { pitch: pitch.clone(), y: feet }, wall.side, room));
        farness = far;
    }
    best
}

/// 🗳️ Collect eligible actors and their covered fixtures in stable authored order.
pub(crate) fn prospects_of(menagerie: &Menagerie, stage: &Stage, tick: Ticks) -> Vec<Prospect> {
    let mut prospects = Vec::new();
    if stage.fixtures.is_empty() { return prospects; }
    let movers = stage.actors.iter().filter(|actor| matches!(actor.activity, Activity::Walk | Activity::Hop) || stage.trips.iter().any(|trip| trip.owner == actor.species)).count();
    if movers >= MODE_LIMITS[stage.mode].movers { return prospects; }
    for actor in &stage.actors {
        let Some(kind) = menagerie.species.iter().find(|kind| kind.id == actor.species) else { continue; };
        if !sociable(actor, tick) || actor.perch.is_none() || stage.trips.iter().any(|trip| trip.owner == actor.species) { continue; }
        for fixture in fixture_for(&kind.grounds, &stage.fixtures) {
            if let Some((post, side, room)) = post_of(kind, fixture, actor.x, actor.y, &stage.perches, &stage.pitches, stage.width) { prospects.push(Prospect { actor: actor.species.clone(), fixture: fixture.clone(), post, side, room }); }
        }
    }
    prospects
}

/// 🚥️ The event-anchored conditions that gate the next prank.
fn circumstances_of(stage: &Stage, tick: Ticks) -> Circumstances {
    Circumstances { permitted: stage.mischief, fine: true, width: stage.width, mode: stage.mode, quiet: stage.quiet, lifting: stage.lift.is_some(), tick, stirred: std::cmp::max(std::cmp::max(stage.stirred, stage.pointed), stage.scrolled), rested: stage.rested }
}

/// 🪄️ The next prank transition, including cooldown, push readiness and sheepish recovery.
pub(crate) fn prank_tick(menagerie: &Menagerie, stage: &Stage, from: Ticks) -> Ticks {
    let Some(lift) = &stage.lift else {
        let Some(opens) = allowed_from(circumstances_of(stage, from)) else { return -1; };
        if prospects_of(menagerie, stage, from).is_empty() { return -1; }
        return std::cmp::max(opens, from);
    };
    let pusher = index_of(&stage.actors, &lift.pusher).map(|index| &stage.actors[index]);
    let age = from - lift.since;
    if age > LIFT_TICKS {
        let Some((pusher, kind)) = pusher.zip(menagerie.species.iter().find(|kind| kind.id == lift.pusher)) else { return from; };
        if matches!(pusher.footing, Footing::Air | Footing::Chute | Footing::Hand) { return -1; }
        let feeling = settled(pusher.feeling, kind.mood, from);
        let rank = |mood| MOOD_PRIORITIES[MOODS.iter().position(|candidate| *candidate == mood).unwrap()];
        if feeling.intensity <= MOOD_REST || rank(feeling.mood) <= rank(Mood::Sad) { return from; }
        return std::cmp::max(calms_at(pusher.feeling, kind.mood), from);
    }
    if age == LIFT_TICKS || age == 0 { return from; }
    if age < 0 { return if pusher.is_none_or(|pusher| pusher.leaving || (age < -1 && !stage.trips.iter().any(|trip| trip.owner == lift.pusher))) { from } else { lift.since }; }
    if pusher.is_none_or(|pusher| pusher.leaving || pusher.activity != Activity::Push) && age < LIFT_RETURNS { from } else { lift.since + LIFT_TICKS }
}
//#endregion 🔖️Mischief

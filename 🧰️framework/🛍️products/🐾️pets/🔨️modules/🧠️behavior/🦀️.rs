//! 🧠️ The character of a pet in numbers: what a mode allows, how much a pet feels like each activity, how long it stays with it, how two pets meet, and how their bond and their drives move. Pure functions over the schema; the stage folds them.
//!
//! Every table is an array in `ACTIVITIES` order (idle, fidget, walk, hop, fall, land, sleep, greet, cuddle, squabble,
//! sulk) and is read at `activity as usize`, every constant is the TypeScript twin's literal and every expression the
//! twin's, term for term and in its order (no reassociation, no `mul_add`), so both cores yield the same bits. Time is
//! whole ticks; rates are per second and meet time as `rate × (ticks ÷ 64)`.
//!
//! Only `idle` decides freely: an idle pet draws its next activity from [`activity_weights`] (idle again, fidget,
//! walk, hop or sleep); every other activity is entered by what happens on stage and ends where [`followers_of`]
//! says. Encounters are drawn from [`encounter_shares`]; a squabble always ends in a sulk and a sulk in mending.
//!
//! Where the twin answers `-1` ("no weight is positive") this one answers `None`; the twin's record `MODE_LIMITS[mode]`
//! is [`ModeLimits`] indexed by a [`PetMode`]; a capacity and an epoch are whole numbers here, so the twin's `floor`
//! of them is the type.
//!
//! @see ../🎪️stage/🦀️.rs — the fold that calls all of this
//! @see ../🎲️randomness/🦀️.rs — `random_words`, `weighted_index`, the reserved streams
//! @see ../📐️trigonometry/🦀️.rs — `clamp`
//! @see ../../🧬️schema/🦀️.rs — `Activity`, `Actor`, `Cast`, `Menagerie`, `Needs`, `PetMode`, `Rapport`, `Species`, `Temperament`
//! @see ../🧠️behavior/🟦️.ts — the TypeScript twin

use crate::randomness::{random_words, weighted_index, CAST_STREAM};
use crate::schema::{Activity, Actor, Cast, Menagerie, Needs, PetMode, Rapport, Slug, Species, Temperament, Ticks, ACTIVITIES, TICKS_PER_SECOND};
use crate::trigonometry::clamp;
use serde::{Deserialize, Serialize};
use std::ops::Index;

//#region 🔖️Modes
/// 🚦️ What a mode allows and how eager it makes a pet: how many actors may walk or hop at once (`movers`) and fidget at once (`fidgeters`), the range of an idle dwell in ticks, the weights of fidget, walk, hop and sleep beside an idle weight of 1, the longest walk in body widths (`stroll`), the least ticks between two encounters (`encounter_gap`, 0 = never) and the chance of one per second for a pair of full sociability (`encounter_rate`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Limits {
    pub movers: usize,
    pub fidgeters: usize,
    pub idle_low: Ticks,
    pub idle_high: Ticks,
    pub fidget: f64,
    pub walk: f64,
    pub hop: f64,
    pub sleep: f64,
    pub stroll: f64,
    pub encounter_gap: Ticks,
    pub encounter_rate: f64,
}

/// 🗂️ The limits of every mode by its name: the TypeScript twin's record, read with `MODE_LIMITS[mode]`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModeLimits {
    pub still: Limits,
    pub calm: Limits,
    pub lively: Limits,
}

impl Index<PetMode> for ModeLimits {
    type Output = Limits;

    /// 🗝️ The limits of a mode.
    fn index(&self, mode: PetMode) -> &Limits {
        match mode {
            PetMode::Still => &self.still,
            PetMode::Calm => &self.calm,
            PetMode::Lively => &self.lively,
        }
    }
}

/// 🎚️ The limits of every mode: `still` allows nothing, `calm` is slightly active (one mover, long idle dwells, a fidget now and then, a short walk now and then, an encounter every couple of minutes), `lively` is busy but not frantic.
pub const MODE_LIMITS: ModeLimits = ModeLimits {
    still: Limits { movers: 0, fidgeters: 0, idle_low: 0, idle_high: 0, fidget: 0.0, walk: 0.0, hop: 0.0, sleep: 0.0, stroll: 0.0, encounter_gap: 0, encounter_rate: 0.0 },
    calm: Limits { movers: 1, fidgeters: 1, idle_low: 384, idle_high: 1280, fidget: 0.4, walk: 0.2, hop: 0.12, sleep: 6.0, stroll: 4.0, encounter_gap: 5760, encounter_rate: 0.04 },
    lively: Limits { movers: 2, fidgeters: 2, idle_low: 192, idle_high: 640, fidget: 1.0, walk: 0.6, hop: 0.3, sleep: 3.0, stroll: 6.0, encounter_gap: 1920, encounter_rate: 0.12 },
};
//#endregion 🔖️Modes

//#region 🔖️Choice
/// 🧭️ What an idle pet finds around it when it decides: the mode, whether it is a time of concentration, how many other actors walk or hop and how many fidget right now, whether its perch has room for a walk (`roam`), whether another perch is open to it — in reach of a hop, or with room to wander off to (`hops`) —, how many others stand on its surface (`crowd`) and whether the pointer is close enough to keep it awake (`watched`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Situation {
    pub mode: PetMode,
    pub quiet: bool,
    pub movers: usize,
    pub fidgeters: usize,
    pub roam: bool,
    pub hops: bool,
    pub crowd: usize,
    pub watched: bool,
}

const QUIET_DROWSE: f64 = 3.0;
const DROWSY: f64 = 0.6;

/// ⚖️ How much an idle actor feels like each activity, in `ACTIVITIES` order; 0 = not eligible.
///
/// `idle` weighs 1. With `drive = 0.5 + 0.5 × energy` and `urge = 0.25 + curiosity` of the actor's needs and the
/// limits of the mode: `fidget = limits.fidget × drive × (0.5 + curiosity)` while fewer than `limits.fidgeters` others
/// fidget and the species has a fidget clip; `walk = limits.walk × drive × urge` while its perch has room and fewer
/// than `limits.movers` others move; `hop = limits.hop × energy × urge × (1 + crowd)` while another perch is open to
/// it and fewer than `limits.movers` others move — company on its surface makes a pet restless, so a crowd thins out
/// by itself where there is room elsewhere; `sleep = limits.sleep × tired²` unless watched, with
/// `tired = (0.6 − energy) ÷ 0.6` held in [0, 1] (a pet with more than 0.6 of its energy never dozes off). Quiet
/// removes fidget, walk and hop and triples sleep. Everything else is entered by events and weighs 0.
pub fn activity_weights(actor: &Actor, species: &Species, situation: Situation) -> [f64; ACTIVITIES.len()] {
    let limits = MODE_LIMITS[situation.mode];
    let energy = actor.needs.energy;
    let curiosity = actor.needs.curiosity;
    let drive = 0.5 + 0.5 * energy;
    let urge = 0.25 + curiosity;
    let tired = clamp((DROWSY - energy) / DROWSY, 0.0, 1.0);
    let awake = !situation.quiet;
    let free = situation.movers < limits.movers;
    let fidgets = species.repertoire.fidget.as_ref().is_some_and(|clips| !clips.is_empty());
    let fidget = if awake && situation.fidgeters < limits.fidgeters && fidgets { limits.fidget * drive * (0.5 + curiosity) } else { 0.0 };
    let walk = if awake && free && situation.roam { limits.walk * drive * urge } else { 0.0 };
    let hop = if awake && free && situation.hops { limits.hop * energy * urge * (1.0 + situation.crowd as f64) } else { 0.0 };
    let sleep = if situation.watched { 0.0 } else { limits.sleep * tired * tired * if situation.quiet { QUIET_DROWSE } else { 1.0 } };
    [1.0, fidget, walk, hop, 0.0, 0.0, sleep, 0.0, 0.0, 0.0, 0.0]
}

const DWELL_LOW: [Ticks; ACTIVITIES.len()] = [0, 96, 1920, 96, 640, 19, 1280, 128, 128, 128, 192];
const DWELL_HIGH: [Ticks; ACTIVITIES.len()] = [0, 96, 1920, 96, 640, 19, 3840, 320, 320, 320, 384];

/// ⏳️ How many ticks an activity lasts for a unit draw: `low + floor((high − low) × unit)`.
///
/// `idle` takes its range from the mode (6…20 s calm, 3…10 s lively, 0 still); `sleep` lasts 20…60 s, `greet`,
/// `cuddle` and `squabble` 2…5 s, `sulk` 3…6 s and `land` 0.3 s. `fidget` (1.5 s) is the span of a fidget whose clip
/// loops, and `walk` (30 s), `hop` (1.5 s) and `fall` (10 s) are the patience of the stage with motion that ends itself.
pub fn dwell_of(activity: Activity, mode: PetMode, unit: f64) -> Ticks {
    let index = activity as usize;
    let low = if activity == Activity::Idle { MODE_LIMITS[mode].idle_low } else { DWELL_LOW[index] };
    let high = if activity == Activity::Idle { MODE_LIMITS[mode].idle_high } else { DWELL_HIGH[index] };
    low + ((high - low) as f64 * unit).floor() as Ticks
}

const MOODS: [f64; ACTIVITIES.len()] = [0.3, 0.5, 0.4, 0.5, -0.2, 0.2, 0.1, 0.7, 1.0, -0.8, -0.6];

/// 🙂️ The mood an activity eases an actor towards, from −1 (sad) to 1 (happy): cuddle 1, greet 0.7, fidget and hop 0.5, walk 0.4, idle 0.3, land 0.2, sleep 0.1, fall −0.2, sulk −0.6, squabble −0.8.
pub fn mood_of(activity: Activity) -> f64 {
    MOODS[activity as usize]
}
//#endregion 🔖️Choice

//#region 🔖️Graph
const AFTER_IDLE: [Activity; 9] = [Activity::Idle, Activity::Fidget, Activity::Walk, Activity::Hop, Activity::Fall, Activity::Sleep, Activity::Greet, Activity::Cuddle, Activity::Squabble];
const AFTER_FIDGET: [Activity; 4] = [Activity::Idle, Activity::Walk, Activity::Fall, Activity::Greet];
const AFTER_WALK: [Activity; 5] = [Activity::Idle, Activity::Fall, Activity::Greet, Activity::Cuddle, Activity::Squabble];
const AFTER_HOP: [Activity; 3] = [Activity::Idle, Activity::Fall, Activity::Land];
const AFTER_FALL: [Activity; 2] = [Activity::Idle, Activity::Land];
const AFTER_LAND: [Activity; 4] = [Activity::Idle, Activity::Walk, Activity::Fall, Activity::Greet];
const AFTER_SLEEP: [Activity; 4] = [Activity::Idle, Activity::Walk, Activity::Fall, Activity::Greet];
const AFTER_GREET: [Activity; 3] = [Activity::Idle, Activity::Walk, Activity::Fall];
const AFTER_CUDDLE: [Activity; 3] = [Activity::Idle, Activity::Walk, Activity::Fall];
const AFTER_SQUABBLE: [Activity; 4] = [Activity::Idle, Activity::Walk, Activity::Fall, Activity::Sulk];
const AFTER_SULK: [Activity; 4] = [Activity::Idle, Activity::Walk, Activity::Fall, Activity::Greet];

/// 🕸️ The activities that may follow an activity on stage, in `ACTIVITIES` order: the activity graph. Every activity is reachable from every other one, and `idle` follows them all (a stage that turns still freezes every actor in it).
///
/// `idle` → idle, fidget, walk, hop, sleep by its own choice, greet by a poke, greet, cuddle or squabble when a
/// partner has arrived, walk towards a partner or off the stage, fall when its perch vanishes. `walk` → idle at its
/// goal, or the encounter it walked into. `hop` → land, or fall when its target is gone. `fall` → land. `squabble` →
/// sulk. `sulk`, `sleep`, `fidget`, `land` → idle, or greet by a poke. Whoever is summoned away walks off.
pub fn followers_of(activity: Activity) -> &'static [Activity] {
    match activity {
        Activity::Idle => &AFTER_IDLE,
        Activity::Fidget => &AFTER_FIDGET,
        Activity::Walk => &AFTER_WALK,
        Activity::Hop => &AFTER_HOP,
        Activity::Fall => &AFTER_FALL,
        Activity::Land => &AFTER_LAND,
        Activity::Sleep => &AFTER_SLEEP,
        Activity::Greet => &AFTER_GREET,
        Activity::Cuddle => &AFTER_CUDDLE,
        Activity::Squabble => &AFTER_SQUABBLE,
        Activity::Sulk => &AFTER_SULK,
    }
}
//#endregion 🔖️Graph

//#region 🔖️Encounters
/// 🫱️ What two pets can do when they meet, in the order of [`encounter_shares`].
pub const ENCOUNTERS: [Activity; 3] = [Activity::Greet, Activity::Cuddle, Activity::Squabble];

/// 🤗️ One of [`ENCOUNTERS`].
pub type Encounter = Activity;

const FRIENDS: f64 = 0.4;
const RIVALS: f64 = -0.3;

/// 🥧️ The chances `[greet, cuddle, squabble]` of an encounter at an affinity; they sum to 1.
///
/// Friends (affinity ≥ 0.4) mostly cuddle: `0.5 + 0.4 × affinity` (0.66 … 0.9). Rivals (affinity ≤ −0.3) mostly
/// squabble: `0.55 − 0.5 × affinity`, at most 0.95 (0.7 … 0.85 down to the affinity floor). In between a pair mostly
/// greets, with a cuddle now and then when it leans friendly (`0.5 × affinity`) and a small dispute now and then when
/// it does not (`0.08 − 0.5 × affinity`, never below 0). Friends never squabble and rivals never cuddle.
pub fn encounter_shares(affinity: f64) -> [f64; ENCOUNTERS.len()] {
    let cuddle = if affinity >= FRIENDS {
        0.5 + 0.4 * affinity
    } else if affinity > 0.0 {
        0.5 * affinity
    } else {
        0.0
    };
    let squabble = if affinity <= RIVALS { clamp(0.55 - 0.5 * affinity, 0.0, 0.95) } else { clamp(0.08 - 0.5 * affinity, 0.0, 0.95) };
    [1.0 - cuddle - squabble, cuddle, squabble]
}

/// 🎭️ The encounter a unit draw picks at an affinity: [`weighted_index`] of [`encounter_shares`].
pub fn encounter_of(affinity: f64, unit: f64) -> Encounter {
    ENCOUNTERS[weighted_index(&encounter_shares(affinity), unit).unwrap_or(0)]
}
//#endregion 🔖️Encounters

//#region 🔖️Bonds
/// 🧱️ The lowest effective affinity of any pair: disputes stay small however often two rivals have squabbled.
pub const AFFINITY_FLOOR: f64 = -0.6;

/// 📏️ How far shared history can shift a bond away from its authored affinity, in both directions.
pub const RAPPORT_SPAN: f64 = 0.5;

const RAPPORT_STEPS: [f64; ACTIVITIES.len()] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.05, 0.1, -0.15, 0.1];
const RAPPORT_FADE_STEP: f64 = 0.1;
const RAPPORT_FADE_TICKS: f64 = 38400.0;

/// 🔗️ Whether a pair joins the two species, in either order.
fn joins(pair: &[Slug; 2], a: &str, b: &str) -> bool {
    (pair[0] == a && pair[1] == b) || (pair[0] == b && pair[1] == a)
}

/// 💞️ How two species feel about each other right now: the authored affinity of their bond (0 when unlisted) plus the drift of their rapport (0 when they have no history), held inside `[AFFINITY_FLOOR, 1]`; 0 for a species and itself.
pub fn affinity_of(menagerie: &Menagerie, rapports: &[Rapport], a: &str, b: &str) -> f64 {
    if a == b {
        return 0.0;
    }
    let affinity = menagerie.bonds.iter().find(|bond| joins(&bond.between, a, b)).map_or(0.0, |bond| bond.affinity);
    let drift = rapports.iter().find(|rapport| joins(&rapport.between, a, b)).map_or(0.0, |rapport| rapport.drift);
    clamp(affinity + drift, AFFINITY_FLOOR, 1.0)
}

/// 🪢️ The drift of a rapport after an encounter: a greet adds 0.05, a cuddle 0.1, a squabble takes 0.15 and the end of a sulk gives 0.1 back (mending); any other activity leaves it. The result stays inside `[−RAPPORT_SPAN, RAPPORT_SPAN]`.
pub fn rapport_after(drift: f64, activity: Activity) -> f64 {
    clamp(drift + RAPPORT_STEPS[activity as usize], 0.0 - RAPPORT_SPAN, RAPPORT_SPAN)
}

/// 🍂️ The drift of a rapport `ticks` later without an encounter: it moves towards 0 by `0.1 × ticks ÷ 38400` (a tenth in ten minutes) and stops there.
pub fn rapport_faded(drift: f64, ticks: Ticks) -> f64 {
    let step = (RAPPORT_FADE_STEP * ticks as f64) / RAPPORT_FADE_TICKS;
    if drift > step {
        drift - step
    } else if drift < 0.0 - step {
        drift + step
    } else {
        0.0
    }
}
//#endregion 🔖️Bonds

//#region 🔖️Needs
const ENERGY_RATES: [f64; ACTIVITIES.len()] = [-0.001, -0.01, -0.012, -0.02, 0.0, 0.0, 0.02, -0.006, -0.004, -0.012, -0.001];
const SOCIABILITY_RATES: [f64; ACTIVITIES.len()] = [0.002, 0.002, 0.002, 0.002, 0.0, 0.002, 0.002, -0.12, -0.12, -0.12, -0.02];
const CURIOSITY_RATES: [f64; ACTIVITIES.len()] = [0.01, -0.08, -0.06, -0.1, 0.0, 0.01, 0.01, 0.0, 0.0, 0.0, 0.01];

/// 🌱️ The drives a pet arrives with: energy `0.5 + 0.5 × temperament.energy`, sociability and curiosity as its temperament says.
pub fn needs_of(temperament: Temperament) -> Needs {
    Needs { energy: 0.5 + 0.5 * temperament.energy, sociability: temperament.sociability, curiosity: temperament.curiosity }
}

/// 🔋️ The drives of an actor after `ticks` of an activity, each held inside [0, 1]: `need + rate × (ticks ÷ 64)` with a rate per second and activity.
///
/// Energy drains with everything but sleep (idle 0.001, walk 0.012, hop 0.02 … per second), slower for an energetic
/// species (`× (1.5 − temperament.energy)`), and returns asleep (0.02). Sociability grows alone (0.002, `× (0.5 +
/// temperament.sociability)`) and is spent in company (0.12 while greeting, cuddling or squabbling). Curiosity grows
/// at rest (0.01, `× (0.5 + temperament.curiosity)`) and is spent on the move (walk 0.06, fidget 0.08, hop 0.1).
pub fn needs_after(needs: Needs, activity: Activity, ticks: Ticks, temperament: Temperament) -> Needs {
    let index = activity as usize;
    let seconds = ticks as f64 / TICKS_PER_SECOND as f64;
    let energy = ENERGY_RATES[index];
    let sociability = SOCIABILITY_RATES[index];
    let curiosity = CURIOSITY_RATES[index];
    Needs {
        energy: clamp(needs.energy + (if energy < 0.0 { energy * (1.5 - temperament.energy) } else { energy }) * seconds, 0.0, 1.0),
        sociability: clamp(needs.sociability + (if sociability > 0.0 { sociability * (0.5 + temperament.sociability) } else { sociability }) * seconds, 0.0, 1.0),
        curiosity: clamp(needs.curiosity + (if curiosity > 0.0 { curiosity * (0.5 + temperament.curiosity) } else { curiosity }) * seconds, 0.0, 1.0),
    }
}
//#endregion 🔖️Needs

//#region 🔖️Casting
/// 🎠️ The members of a ring that are on at an epoch, `seats` of them at most: read one after the other from the place a word picks (`word mod count`), moved on by one per epoch, so everybody gets its turn and a change of epoch swaps one member.
fn turns_of(ring: &[&Slug], seats: usize, word: u32, epoch: i64) -> Vec<Slug> {
    let count = ring.len();
    let mut taken = Vec::new();
    if count == 0 {
        return taken;
    }
    let turn = epoch.rem_euclid(count as i64) as usize;
    let start = word as usize % count + turn;
    for index in 0..count {
        if index >= seats {
            break;
        }
        taken.push(ring[(start + index) % count].clone());
    }
    taken
}

/// 🎟️ The species of a cast that are on stage at an epoch: its core first, then its visitors.
///
/// The rotation visits: it gets the seats the core leaves free, and one seat whenever the stage holds at least two —
/// also when the core alone would fill or exceed the capacity —, so every species of a cast can appear. The core gets
/// the other seats: all of it, in its order, while it fits; otherwise it takes turns itself. Whoever takes turns is
/// read off a ring ([`turns_of`]) from a start the seed picks (`random_words([seed, CAST_STREAM, 0], 2)`: word 0 for
/// the core, word 1 for the rotation) and every epoch moves on by one. A species is listed once, the core wins.
pub fn cast_of(cast: &Cast, capacity: usize, epoch: i64, seed: u32) -> Vec<Slug> {
    let mut core: Vec<&Slug> = Vec::with_capacity(cast.core.len());
    for species in &cast.core {
        if !core.contains(&species) {
            core.push(species);
        }
    }
    let mut pool: Vec<&Slug> = Vec::with_capacity(cast.rotation.len());
    for species in &cast.rotation {
        if !core.contains(&species) && !pool.contains(&species) {
            pool.push(species);
        }
    }
    let spare = capacity.saturating_sub(core.len());
    let least = usize::from(capacity >= 2);
    let open = if spare > least { spare } else { least };
    let guests = if open < pool.len() { open } else { pool.len() };
    let seats = capacity - guests;
    let words = random_words(&[seed, CAST_STREAM, 0], 2);
    let mut troupe: Vec<Slug> = if core.len() <= seats { core.iter().map(|&species| species.clone()).collect() } else { turns_of(&core, seats, words[0], epoch) };
    troupe.extend(turns_of(&pool, guests, words[1], epoch));
    troupe
}
//#endregion 🔖️Casting

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

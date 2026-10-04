//! ✨️ Particles of the pets as pure functions of time — the Rust twin of the effects module: what an emitter shows at a tick follows from the emitter, the tick it began, the tick it stopped, a 32-bit key and the tick asked for; nothing is stored between frames, so a frame can be recomputed, skipped or replayed and every language draws the same sparks.
//!
//! An emitter keeps at most `count` particles alive, each lives `life` seconds, leaves at `speed` pixels per second,
//! and the directions scatter over `spread` of a full turn around the direction its motion prefers: down for `fall`,
//! up for `rise` and `burst`, ahead for `drift`; an `orbit` spaces its particles over `spread` of its ring.
//! Randomness is a hash, not a stream: particle `index` of the emitter keyed `key` reads its lane `lane` as
//! `unit(mix(mix(key, index), lane))`, so no draw of the stage is spent and no order matters.
//!
//! The hash is the twin's 32-bit arithmetic on `u32` (`wrapping_mul` ≙ `Math.imul`, `>>` ≙ `>>>`, `wrapping_add` ≙
//! `+ … >>> 0`); an index or a tick enters it by its low 32 bits, as the twin's `| 0` reads it. Every float expression
//! is the twin's, term for term and in its order (no `mul_add`, no reassociation); `facing` multiplies as ±1, so a
//! negative zero falls where the twin's does; `Math.min` is the terrain's `smaller`. Whole ticks are `i64` and
//! counts `u32`; the twin's `null` is `None`.
//!
//! @see <https://nullprogram.com/blog/2018/07/31/> — `lowbias32`, the 32-bit mixing hash
//! @see <https://theorangeduck.com/page/spring-roll-call#exactdamper> — the rational decay behind the drag of a burst
//! @see ../✨️effects/🟦️.ts — the TypeScript twin
//! @see ../../🧬️schema/🦀️.rs — `Emitter`, `Drift`, `Facing`
//! @see ../📐️trigonometry/🦀️.rs — `sin_turns`, `cos_turns`, `smoothstep`, `fast_neg_exp`
//! @see ../🎲️randomness/🦀️.rs — `unit_of`, the one place a word becomes a unit

use crate::randomness::unit_of;
use crate::schema::{Drift, Emitter, Facing, Point, Ticks, Turns, TICKS_PER_SECOND};
use crate::terrain::smaller;
use crate::trigonometry::{cos_turns, fast_neg_exp, sin_turns, smoothstep};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🧢️ The most particles one emitter keeps alive, whatever its `count` says: the schema's upper bound of `count`.
pub const EMITTER_CAP: u32 = 32;

/// 🎪️ The most particles a whole stage shows at once ([`capped`] keeps the youngest).
pub const STAGE_CAP: usize = 160;

/// ⭕️ The radians of a full turn, as the trigonometry rounds them (`6.283185307179586`, the twin's literal): turns a speed along a ring into its radius.
pub const TURN_RADIANS: f64 = std::f64::consts::TAU;

/// ➡️ The direction ahead in turns: the way the pet faces; `drift` scatters around it.
pub const AHEAD: Turns = 0.0;

/// ⬇️ The direction down the screen in turns; `fall` scatters around it.
pub const DOWN: Turns = 0.25;

/// ⬆️ The direction up the screen in turns; `rise` and `burst` scatter around it.
pub const UP: Turns = 0.75;

/// 🎂️ The lane of a particle that delays its birth within its period.
pub const LANE_BIRTH: u32 = 0;

/// 🧭️ The lane of a particle that scatters its direction.
pub const LANE_HEADING: u32 = 1;

/// 👟️ The lane of a particle that scatters its speed.
pub const LANE_PACE: u32 = 2;

/// 🌗️ The lane of a particle that sets where its sideways swing begins.
pub const LANE_PHASE: u32 = 3;

/// 👗️ The lane of a particle that scatters its size and its strength.
pub const LANE_LOOK: u32 = 4;

/// 🍂️ How far a slowly falling particle swings sideways, in pixels; a fast one hardly swings (see [`FALL_SWAY_SPEED`]).
pub const FALL_SWAY: f64 = 3.0;

/// 🪶️ The falling speed in pixels per second at which the sideways swing has shrunk to about a third: snow swings, rain does not.
pub const FALL_SWAY_SPEED: f64 = 60.0;

/// 🎐️ How many times per second a falling particle swings to and fro.
pub const FALL_SWAY_RATE: f64 = 0.5;

/// 🌅️ The ticks over which a falling particle appears.
pub const FALL_FADE_IN: Ticks = 4;

/// 🎈️ How far a rising particle wanders sideways per unit of `spread`, in pixels, once it is old.
pub const RISE_WANDER: f64 = 24.0;

/// 🫧️ How many times per second a rising particle wanders to and fro.
pub const RISE_WANDER_RATE: f64 = 0.35;

/// 🛶️ How far a rising particle rocks per unit of `spread`, in turns.
pub const RISE_ROCK: f64 = 0.1;

/// 🪂️ The seconds after which the air has taken most of the speed of a thrown particle: it gets `speed × BURST_DRAG` pixels far at most.
pub const BURST_DRAG: f64 = 0.4;

/// 🍎️ How fast a thrown particle sinks, in pixels per second squared.
pub const BURST_GRAVITY: f64 = 120.0;

/// 🥏️ The height of a ring over its width: an orbit is seen from slightly above.
pub const ORBIT_SQUASH: f64 = 0.35;

/// 🔭️ How much larger a circling particle is at the front of its ring and how much smaller at the back.
pub const ORBIT_DEPTH: f64 = 0.15;

/// 🌫️ The ticks over which circling particles appear at `since` and vanish from `until`.
pub const ORBIT_FADE: Ticks = 8;

/// 🐍️ How far a drifting particle meanders across its path, in pixels.
pub const DRIFT_MEANDER: f64 = 3.0;

/// 🐌️ How many times per second a drifting particle meanders to and fro.
pub const DRIFT_RATE: f64 = 0.5;
//#endregion 🔖️Constants

//#region 🔖️Hash
/// 🧂️ Wellons' `lowbias32`: an unsigned 32-bit word stirred so that every input bit reaches every output bit; 0 stays 0, which is why [`mix`] adds a constant first.
pub fn lowbias32(word: u32) -> u32 {
    let first = (word ^ (word >> 16)).wrapping_mul(0x7feb_352d);
    let second = (first ^ (first >> 15)).wrapping_mul(0x846c_a68b);
    second ^ (second >> 16)
}

/// 🥣️ Two unsigned 32-bit words hashed into one: `lowbias32((a xor 0x9e3779b9) + (b + 1)·0x85ebca6b)`, all of it wrapping at 32 bits. Nested, it keys anything: `mix(mix(key, index), lane)`.
pub fn mix(a: u32, b: u32) -> u32 {
    lowbias32((a ^ 0x9e37_79b9).wrapping_add(b.wrapping_add(1).wrapping_mul(0x85eb_ca6b)))
}

/// 🪙️ A word as a number in [0, 1): the randomness module's one division by 2³², under the name the effects use.
pub use crate::randomness::unit_of as unit;

/// 🎲️ The number in [0, 1) that particle `index` of the emitter keyed `key` reads from its lane `lane`: `unit(mix(mix(key, index), lane))`, the index entering the hash by its low 32 bits.
pub fn scattered(key: u32, index: i64, lane: u32) -> f64 {
    unit_of(mix(mix(key, index as u32), lane))
}

/// 🔑️ The key of one run of an emitter: the stage's seed, the index of the species in the menagerie, the index of the emitter in the species and the tick the run began, hashed together — two runs never share their scatter, and no draw of the stage is spent.
pub fn emitter_key(seed: u32, species: usize, emitter: usize, since: Ticks) -> u32 {
    mix(mix(mix(seed, species as u32), emitter as u32), since as u32)
}
//#endregion 🔖️Hash

//#region 🔖️Births
/// 🚿️ The numbers of an emitter its particles follow from (the twin's `Pick<Emitter, "motion" | "count" | "life" | "speed" | "spread">`; every emitter gives one).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Emission {
    pub motion: Drift,
    pub count: u32,
    pub life: f64,
    pub speed: f64,
    pub spread: f64,
}

impl From<&Emitter> for Emission {
    /// 🚰️ The numbers of an emitter its particles follow from.
    fn from(emitter: &Emitter) -> Self {
        Self { motion: emitter.motion, count: emitter.count, life: emitter.life, speed: emitter.speed, spread: emitter.spread }
    }
}

/// 🔹️ One particle as drawn: where the origin of its shape lies on the stage, how large it is, how far it is turned, how strong it is, and for how many ticks it has been alive.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub rotation: Turns,
    pub opacity: f64,
    pub age: Ticks,
}

/// ⏳️ The ticks a particle of the emitter lives: `⌊life × 64 + ½⌋`, at least one.
pub fn life_ticks(emission: Emission) -> Ticks {
    let ticks = (emission.life * TICKS_PER_SECOND as f64 + 0.5).floor();
    if ticks > 1.0 {
        ticks as Ticks
    } else {
        1
    }
}

/// 👥️ How many particles the emitter keeps alive at most: its whole `count`, never above [`EMITTER_CAP`].
pub fn swarm_of(emission: Emission) -> u32 {
    emission.count.min(EMITTER_CAP)
}

/// 🥁️ The ticks between two births of a continuous emitter: `⌈life ÷ count⌉`, at least one (an emitter without particles counts as one here), so that a lifetime holds about `count` births.
pub fn period_of(emission: Emission) -> Ticks {
    let swarm = Ticks::from(swarm_of(emission).max(1));
    ((life_ticks(emission) + swarm - 1) / swarm).max(1)
}

/// 🐣️ The tick particle `index` of a continuous emitter is born: its slot `since + index·period`, delayed by its birth lane within the period, so births keep the order of their indices.
pub fn born_at(emission: Emission, since: Ticks, key: u32, index: i64) -> Ticks {
    let period = period_of(emission);
    since + index * period + (scattered(key, index, LANE_BIRTH) * period as f64).floor() as Ticks
}
//#endregion 🔖️Births

//#region 🔖️Motions
/// 🧾️ What every particle of one emitter at one tick shares: its numbers, its point on the stage, the way its pet faces as ±1, its key, the ticks a particle lives and how many it keeps alive.
struct Source {
    emission: Emission,
    origin: Point,
    facing: f64,
    key: u32,
    life: Ticks,
    swarm: u32,
}

/// 🌧️ A falling particle `age` ticks after its birth: it leaves at `speed × (0.9 + 0.2·u)` in a direction within `spread` around straight down and keeps that velocity; the slower it is, the more it swings sideways. It appears over [`FALL_FADE_IN`] ticks and vanishes over the last quarter of its life; its shape leans as its path does.
fn fallen(source: &Source, index: i64, age: Ticks) -> Particle {
    let seconds = age as f64 / TICKS_PER_SECOND as f64;
    let share = age as f64 / source.life as f64;
    let look = scattered(source.key, index, LANE_LOOK);
    let heading = DOWN + (scattered(source.key, index, LANE_HEADING) - 0.5) * source.emission.spread;
    let pace = source.emission.speed * (0.9 + 0.2 * scattered(source.key, index, LANE_PACE));
    let sway = FALL_SWAY * fast_neg_exp(pace / FALL_SWAY_SPEED) * sin_turns(FALL_SWAY_RATE * seconds + scattered(source.key, index, LANE_PHASE));
    Particle {
        x: source.origin.x + source.facing * (pace * cos_turns(heading) * seconds + sway),
        y: source.origin.y + pace * sin_turns(heading) * seconds,
        scale: 0.8 + 0.4 * look,
        rotation: source.facing * (heading - DOWN),
        opacity: smaller(1.0, age as f64 / FALL_FADE_IN as f64) * (0.55 + 0.35 * look) * (1.0 - smoothstep((share - 0.75) / 0.25)),
        age,
    }
}

/// ♨️ A rising particle `age` ticks after its birth: it leaves at `speed × (0.7 + 0.3·u)` in a direction within `spread` around straight up, wanders sideways ever further (`RISE_WANDER × spread × sin × (0.3 + share)`) and rocks with it. It pops in (half size to full over 8 ticks, opacity over 6) and fades over the last two fifths of its life.
fn risen(source: &Source, index: i64, age: Ticks) -> Particle {
    let seconds = age as f64 / TICKS_PER_SECOND as f64;
    let share = age as f64 / source.life as f64;
    let heading = UP + (scattered(source.key, index, LANE_HEADING) - 0.5) * source.emission.spread;
    let pace = source.emission.speed * (0.7 + 0.3 * scattered(source.key, index, LANE_PACE));
    let swing = RISE_WANDER_RATE * seconds + scattered(source.key, index, LANE_PHASE);
    let wander = RISE_WANDER * source.emission.spread * sin_turns(swing) * (0.3 + share);
    Particle {
        x: source.origin.x + source.facing * (pace * cos_turns(heading) * seconds + wander),
        y: source.origin.y + pace * sin_turns(heading) * seconds,
        scale: 0.5 + 0.5 * smoothstep(age as f64 / 8.0),
        rotation: source.facing * RISE_ROCK * source.emission.spread * cos_turns(swing),
        opacity: smoothstep(age as f64 / 6.0) * (1.0 - smoothstep((share - 0.6) / 0.4)),
        age,
    }
}

/// 🎆️ A thrown particle `age` ticks after the burst: particle `index` of the swarm leaves at `speed × (0.45 + 0.55·u)` in its own slice of the fan of `spread` around straight up, the air brakes it (`BURST_DRAG × (1 − fast_neg_exp(seconds ÷ BURST_DRAG))` seconds of flight at full speed) and it sinks with [`BURST_GRAVITY`]. It shrinks to two fifths, fades as `1 − share²` and points the way it left.
fn thrown(source: &Source, index: i64, age: Ticks) -> Particle {
    let seconds = age as f64 / TICKS_PER_SECOND as f64;
    let share = age as f64 / source.life as f64;
    let heading = UP + ((index as f64 + scattered(source.key, index, LANE_HEADING)) / f64::from(source.swarm) - 0.5) * source.emission.spread;
    let pace = source.emission.speed * (0.45 + 0.55 * scattered(source.key, index, LANE_PACE));
    let reach = BURST_DRAG * (1.0 - fast_neg_exp(seconds / BURST_DRAG));
    Particle {
        x: source.origin.x + source.facing * (pace * cos_turns(heading) * reach),
        y: source.origin.y + pace * sin_turns(heading) * reach + 0.5 * BURST_GRAVITY * seconds * seconds,
        scale: 1.0 - 0.6 * share,
        rotation: if source.facing > 0.0 { heading } else { 0.5 - heading },
        opacity: 1.0 - share * share,
        age,
    }
}

/// 💫️ A circling particle `elapsed` ticks after its emitter began: particle `index` of the swarm sits `index ÷ swarm × spread` of a turn along a ring that takes `life` ticks per lap at `speed`, so its radius is `speed × seconds per lap ÷ 2π`; the ring is squashed to [`ORBIT_SQUASH`] of its width, turns clockwise for a pet facing right and mirrored for one facing left, and a particle is larger at the front than at the back. Everything fades in over [`ORBIT_FADE`] ticks and out over as many from `until`.
fn circled(source: &Source, until: Option<Ticks>, tick: Ticks, index: i64, elapsed: Ticks) -> Particle {
    let radius = (source.emission.speed * (source.life as f64 / TICKS_PER_SECOND as f64)) / TURN_RADIANS;
    let turned = (index as f64 / f64::from(source.swarm)) * source.emission.spread + elapsed as f64 / source.life as f64;
    let angle = if source.facing > 0.0 { turned } else { 0.5 - turned };
    let depth = sin_turns(angle);
    Particle {
        x: source.origin.x + radius * cos_turns(angle),
        y: source.origin.y + radius * ORBIT_SQUASH * depth,
        scale: 1.0 + ORBIT_DEPTH * depth,
        rotation: 0.0,
        opacity: smoothstep(elapsed as f64 / ORBIT_FADE as f64) * until.map_or(1.0, |until| 1.0 - smoothstep((tick - until) as f64 / ORBIT_FADE as f64)),
        age: elapsed,
    }
}

/// 🍃️ A drifting particle `age` ticks after its birth: it leaves at `speed × (0.5 + 0.5·u)` in a direction within `spread` around straight ahead and meanders [`DRIFT_MEANDER`] pixels across its path. It fades in over the first three tenths of its life and out over the last three, and points the way it travels.
fn wandered(source: &Source, index: i64, age: Ticks) -> Particle {
    let seconds = age as f64 / TICKS_PER_SECOND as f64;
    let share = age as f64 / source.life as f64;
    let look = scattered(source.key, index, LANE_LOOK);
    let heading = AHEAD + (scattered(source.key, index, LANE_HEADING) - 0.5) * source.emission.spread;
    let pace = source.emission.speed * (0.5 + 0.5 * scattered(source.key, index, LANE_PACE));
    let meander = DRIFT_MEANDER * sin_turns(DRIFT_RATE * seconds + scattered(source.key, index, LANE_PHASE));
    let along = cos_turns(heading);
    let across = sin_turns(heading);
    Particle {
        x: source.origin.x + source.facing * (pace * along * seconds - meander * across),
        y: source.origin.y + pace * across * seconds + meander * along,
        scale: 0.6 + 0.6 * look,
        rotation: if source.facing > 0.0 { heading } else { 0.5 - heading },
        opacity: smoothstep(share / 0.3) * (1.0 - smoothstep((share - 0.7) / 0.3)) * (0.6 + 0.4 * look),
        age,
    }
}
//#endregion 🔖️Motions

//#region 🔖️Particles
/// 🎇️ The particles of one emitter that are alive at `tick`, oldest first: the emitter's point `origin` on the stage, the way its pet faces, the tick it began (`since`), the tick it stopped or `None` while it runs (`until`), and the key of this run ([`emitter_key`]).
///
/// A continuous emitter (`fall`, `rise`, `drift`) looks only at the births that can still be alive — the indices from
/// `⌊(tick − since − life) ÷ period⌋` to `⌊(tick − since) ÷ period⌋` — and keeps those born before `until` and
/// younger than `life`; when one more than its `count` is alive, the oldest is left out. A `burst` shows its `count`
/// particles for `life` ticks from `since`. An `orbit` shows its `count` particles from `since` until [`ORBIT_FADE`]
/// ticks after `until`. Nothing is alive before `since`.
pub fn particles_of(emission: Emission, origin: Point, facing: Facing, since: Ticks, until: Option<Ticks>, tick: Ticks, key: u32) -> Vec<Particle> {
    let life = life_ticks(emission);
    let swarm = swarm_of(emission);
    let elapsed = tick - since;
    if swarm < 1 || elapsed < 0 {
        return Vec::new();
    }
    let source = Source { emission, origin, facing: facing.sign(), key, life, swarm };
    if emission.motion == Drift::Burst {
        if elapsed >= life {
            return Vec::new();
        }
        return (0..i64::from(swarm)).map(|index| thrown(&source, index, elapsed)).collect();
    }
    if emission.motion == Drift::Orbit {
        if until.is_some_and(|until| tick >= until + ORBIT_FADE) {
            return Vec::new();
        }
        return (0..i64::from(swarm)).map(|index| circled(&source, until, tick, index, elapsed)).collect();
    }
    let period = period_of(emission);
    let first = (elapsed - life).div_euclid(period).max(0);
    let last = elapsed.div_euclid(period);
    let mut particles = Vec::new();
    for index in first..=last {
        let born = born_at(emission, since, key, index);
        let age = tick - born;
        if age < 0 || age >= life {
            continue;
        }
        if until.is_some_and(|until| born >= until) {
            continue;
        }
        particles.push(match emission.motion {
            Drift::Fall => fallen(&source, index, age),
            Drift::Rise => risen(&source, index, age),
            Drift::Burst | Drift::Orbit | Drift::Drift => wandered(&source, index, age),
        });
    }
    let alive = swarm as usize;
    if particles.len() > alive {
        particles.split_off(particles.len() - alive)
    } else {
        particles
    }
}

/// ✂️ At most `cap` of the particles, in their order: the youngest stay — whoever is nearest to its birth is dropped last —, and among particles of the same age those earlier in the list stay, so the emitters listed first keep theirs. A stage passes all its particles in the order of its emitters and [`STAGE_CAP`]; under the cap the very list comes back.
pub fn capped(particles: Vec<Particle>, cap: usize) -> Vec<Particle> {
    if particles.len() <= cap {
        return particles;
    }
    if cap == 0 {
        return Vec::new();
    }
    let mut ages: Vec<Ticks> = particles.iter().map(|particle| particle.age).collect();
    ages.sort_unstable();
    let eldest = ages[cap - 1];
    let mut peers = cap - ages.iter().filter(|&&age| age < eldest).count();
    let mut kept = Vec::with_capacity(cap);
    for particle in particles {
        if particle.age < eldest {
            kept.push(particle);
        } else if particle.age == eldest && peers > 0 {
            kept.push(particle);
            peers -= 1;
        }
    }
    kept
}

/// 🏁️ The first tick from which nothing of the emitter is alive any more — the horizon a stage has to stay awake for —, or `None` while the emitter runs without an end: a `burst` ends `life` ticks after `since`; an `orbit` [`ORBIT_FADE`] ticks after `until`; a continuous emitter `life − 1` ticks after `until` (its last birth is one tick before `until` at the latest); never before `since`, and at `since` when the emitter stopped before it could begin.
pub fn emitter_ends(emission: Emission, since: Ticks, until: Option<Ticks>) -> Option<Ticks> {
    let life = life_ticks(emission);
    if swarm_of(emission) < 1 {
        return Some(since);
    }
    if emission.motion == Drift::Burst {
        return Some(since + life);
    }
    let until = until?;
    if emission.motion == Drift::Orbit {
        return Some(since.max(until + ORBIT_FADE));
    }
    Some(if until <= since { since } else { until + life - 1 })
}
//#endregion 🔖️Particles

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

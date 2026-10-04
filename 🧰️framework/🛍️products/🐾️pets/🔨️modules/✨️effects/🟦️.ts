/** ✨️ Particles of the pets as pure functions of time: what an emitter shows at a tick follows from the emitter, the tick it began, the tick it stopped, a 32-bit key and the tick asked for — nothing is stored between frames, so a frame can be recomputed, skipped or replayed and every language draws the same sparks.
 *
 * An emitter (`Emitter` of the schema) keeps at most `count` particles alive, each lives `life` seconds, leaves at
 * `speed` pixels per second, and the directions scatter over `spread` of a full turn around the direction its motion
 * prefers: down for `fall`, up for `rise` and `burst`, ahead for `drift`; an `orbit` spaces its particles over
 * `spread` of its ring. Randomness is a hash, not a stream: particle `index` of the emitter with key `key` reads
 * its lane `lane` as `unit(mix(mix(key, index), lane))`, so no draw of the stage is spent and no order matters.
 *
 * - `fall`, `rise`, `drift` are continuous: particle `index` is born at `since + index·period + ⌊jitter·period⌋`
 *   with `period = ⌈life ÷ count⌉` ticks, as long as that tick lies before `until`; it lives `life` ticks.
 * - `burst` throws `count` particles at `since`, once, whatever `until` says.
 * - `orbit` keeps `count` particles circling from `since` on and fades them out over {@link ORBIT_FADE} ticks from `until`.
 *
 * Angles are turns on screen (y points down): 0 is ahead (the way the pet faces), ¼ is down, ¾ is up. Only
 * `+ − × ÷`, `floor`, `min`, `max`, comparisons, 32-bit integer operations and the owned `sinTurns`, `cosTurns`,
 * `smoothstep` and `fastNegExp` are used.
 *
 * @see https://nullprogram.com/blog/2018/07/31/ — `lowbias32`, the 32-bit mixing hash
 * @see https://theorangeduck.com/page/spring-roll-call#exactdamper — the rational decay behind the drag of a burst
 * @see ../../🧬️schema/🟦️.ts — `Emitter`, `Drift`
 * @see ../📐️trigonometry/🟦️.ts — `sinTurns`, `cosTurns`, `smoothstep`, `fastNegExp`
 * @see ../🎲️randomness/🟦️.ts — `unitOf`, the one place a word becomes a unit
 */

import type { Emitter, Point, Ticks, Turns } from "../../🧬️schema/🟦️.ts";
import { TICKS_PER_SECOND } from "../../🧬️schema/🟦️.ts";
import { unitOf } from "../🎲️randomness/🟦️.ts";
import { cosTurns, fastNegExp, sinTurns, smoothstep } from "../📐️trigonometry/🟦️.ts";

//#region 🔖️Constants
/** 🧢️ The most particles one emitter keeps alive, whatever its `count` says: the schema's upper bound of `count`. */
export const EMITTER_CAP = 32;

/** 🎪️ The most particles a whole stage shows at once ({@link capped} keeps the youngest). */
export const STAGE_CAP = 160;

/** ⭕️ The radians of a full turn, as the trigonometry rounds them: turns a speed along a ring into its radius. */
export const TURN_RADIANS = 6.283185307179586;

/** ➡️ The direction ahead in turns: the way the pet faces; `drift` scatters around it. */
export const AHEAD = 0;

/** ⬇️ The direction down the screen in turns; `fall` scatters around it. */
export const DOWN = 0.25;

/** ⬆️ The direction up the screen in turns; `rise` and `burst` scatter around it. */
export const UP = 0.75;

/** 🎂️ The lane of a particle that delays its birth within its period. */
export const LANE_BIRTH = 0;

/** 🧭️ The lane of a particle that scatters its direction. */
export const LANE_HEADING = 1;

/** 👟️ The lane of a particle that scatters its speed. */
export const LANE_PACE = 2;

/** 🌗️ The lane of a particle that sets where its sideways swing begins. */
export const LANE_PHASE = 3;

/** 👗️ The lane of a particle that scatters its size and its strength. */
export const LANE_LOOK = 4;

/** 🍂️ How far a slowly falling particle swings sideways, in pixels; a fast one hardly swings (see {@link FALL_SWAY_SPEED}). */
export const FALL_SWAY = 3;

/** 🪶️ The falling speed in pixels per second at which the sideways swing has shrunk to about a third: snow swings, rain does not. */
export const FALL_SWAY_SPEED = 60;

/** 🎐️ How many times per second a falling particle swings to and fro. */
export const FALL_SWAY_RATE = 0.5;

/** 🌅️ The ticks over which a falling particle appears. */
export const FALL_FADE_IN = 4;

/** 🎈️ How far a rising particle wanders sideways per unit of `spread`, in pixels, once it is old. */
export const RISE_WANDER = 24;

/** 🫧️ How many times per second a rising particle wanders to and fro. */
export const RISE_WANDER_RATE = 0.35;

/** 🛶️ How far a rising particle rocks per unit of `spread`, in turns. */
export const RISE_ROCK = 0.1;

/** 🪂️ The seconds after which the air has taken most of the speed of a thrown particle: it gets `speed × BURST_DRAG` pixels far at most. */
export const BURST_DRAG = 0.4;

/** 🍎️ How fast a thrown particle sinks, in pixels per second squared. */
export const BURST_GRAVITY = 120;

/** 🥏️ The height of a ring over its width: an orbit is seen from slightly above. */
export const ORBIT_SQUASH = 0.35;

/** 🔭️ How much larger a circling particle is at the front of its ring and how much smaller at the back. */
export const ORBIT_DEPTH = 0.15;

/** 🌫️ The ticks over which circling particles appear at `since` and vanish from `until`. */
export const ORBIT_FADE = 8;

/** 🐍️ How far a drifting particle meanders across its path, in pixels. */
export const DRIFT_MEANDER = 3;

/** 🐌️ How many times per second a drifting particle meanders to and fro. */
export const DRIFT_RATE = 0.5;
//#endregion 🔖️Constants

//#region 🔖️Hash
/** 🧂️ Wellons' `lowbias32`: an unsigned 32-bit word stirred so that every input bit reaches every output bit; 0 stays 0, which is why {@link mix} adds a constant first. */
export function lowbias32(word: number): number {
  const first = Math.imul((word >>> 0) ^ (word >>> 16), 0x7feb352d);
  const second = Math.imul(first ^ (first >>> 15), 0x846ca68b);
  return (second ^ (second >>> 16)) >>> 0;
}

/** 🥣️ Two unsigned 32-bit words hashed into one: `lowbias32((a xor 0x9e3779b9) + (b + 1)·0x85ebca6b)`, all of it wrapping at 32 bits. Nested, it keys anything: `mix(mix(key, index), lane)`. */
export function mix(a: number, b: number): number {
  return lowbias32(((a ^ 0x9e3779b9) + Math.imul((b + 1) | 0, 0x85ebca6b)) >>> 0);
}

/** 🪙️ A word as a number in [0, 1): the randomness module's one division by 2³², under the name the effects use. */
export { unitOf as unit };

/** 🎲️ The number in [0, 1) that particle `index` of the emitter keyed `key` reads from its lane `lane`: `unit(mix(mix(key, index), lane))`. */
export function scattered(key: number, index: number, lane: number): number {
  return unitOf(mix(mix(key, index), lane));
}

/** 🔑️ The key of one run of an emitter: the stage's seed, the index of the species in the menagerie, the index of the emitter in the species and the tick the run began, hashed together — two runs never share their scatter, and no draw of the stage is spent. */
export function emitterKey(seed: number, species: number, emitter: number, since: Ticks): number {
  return mix(mix(mix(seed, species), emitter), since);
}
//#endregion 🔖️Hash

//#region 🔖️Births
/** 🚿️ The numbers of an emitter its particles follow from. */
export type Emission = Pick<Emitter, "motion" | "count" | "life" | "speed" | "spread">;

/** 🔹️ One particle as drawn: where the origin of its shape lies on the stage, how large it is, how far it is turned, how strong it is, and for how many ticks it has been alive. */
export type Particle = { readonly x: number; readonly y: number; readonly scale: number; readonly rotation: Turns; readonly opacity: number; readonly age: Ticks };

/** ⏳️ The ticks a particle of the emitter lives: `⌊life × 64 + ½⌋`, at least one. */
export function lifeTicks(emitter: Emission): Ticks {
  return Math.max(1, Math.floor(emitter.life * TICKS_PER_SECOND + 0.5));
}

/** 👥️ How many particles the emitter keeps alive at most: its whole `count`, never above {@link EMITTER_CAP} and never below zero. */
export function swarmOf(emitter: Emission): number {
  return Math.min(Math.max(Math.floor(emitter.count), 0), EMITTER_CAP);
}

/** 🥁️ The ticks between two births of a continuous emitter: `⌈life ÷ count⌉`, at least one (an emitter without particles counts as one here), so that a lifetime holds about `count` births. */
export function periodOf(emitter: Emission): Ticks {
  const swarm = Math.max(1, swarmOf(emitter));
  return Math.max(1, Math.floor((lifeTicks(emitter) + swarm - 1) / swarm));
}

/** 🐣️ The tick particle `index` of a continuous emitter is born: its slot `since + index·period`, delayed by its birth lane within the period, so births keep the order of their indices. */
export function bornAt(emitter: Emission, since: Ticks, key: number, index: number): Ticks {
  const period = periodOf(emitter);
  return since + index * period + Math.floor(scattered(key, index, LANE_BIRTH) * period);
}
//#endregion 🔖️Births

//#region 🔖️Motions
/** 🌧️ A falling particle `age` ticks after its birth: it leaves at `speed × (0.9 + 0.2·u)` in a direction within `spread` around straight down and keeps that velocity; the slower it is, the more it swings sideways. It appears over {@link FALL_FADE_IN} ticks and vanishes over the last quarter of its life; its shape leans as its path does. */
function fallen(emitter: Emission, origin: Point, facing: 1 | -1, key: number, index: number, age: Ticks, life: Ticks): Particle {
  const seconds = age / TICKS_PER_SECOND;
  const share = age / life;
  const look = scattered(key, index, LANE_LOOK);
  const heading = DOWN + (scattered(key, index, LANE_HEADING) - 0.5) * emitter.spread;
  const pace = emitter.speed * (0.9 + 0.2 * scattered(key, index, LANE_PACE));
  const sway = FALL_SWAY * fastNegExp(pace / FALL_SWAY_SPEED) * sinTurns(FALL_SWAY_RATE * seconds + scattered(key, index, LANE_PHASE));
  return {
    x: origin.x + facing * (pace * cosTurns(heading) * seconds + sway),
    y: origin.y + pace * sinTurns(heading) * seconds,
    scale: 0.8 + 0.4 * look,
    rotation: facing * (heading - DOWN),
    opacity: Math.min(1, age / FALL_FADE_IN) * (0.55 + 0.35 * look) * (1 - smoothstep((share - 0.75) / 0.25)),
    age,
  };
}

/** ♨️ A rising particle `age` ticks after its birth: it leaves at `speed × (0.7 + 0.3·u)` in a direction within `spread` around straight up, wanders sideways ever further (`RISE_WANDER × spread × sin × (0.3 + share)`) and rocks with it. It pops in (half size to full over 8 ticks, opacity over 6) and fades over the last two fifths of its life. */
function risen(emitter: Emission, origin: Point, facing: 1 | -1, key: number, index: number, age: Ticks, life: Ticks): Particle {
  const seconds = age / TICKS_PER_SECOND;
  const share = age / life;
  const heading = UP + (scattered(key, index, LANE_HEADING) - 0.5) * emitter.spread;
  const pace = emitter.speed * (0.7 + 0.3 * scattered(key, index, LANE_PACE));
  const swing = RISE_WANDER_RATE * seconds + scattered(key, index, LANE_PHASE);
  const wander = RISE_WANDER * emitter.spread * sinTurns(swing) * (0.3 + share);
  return {
    x: origin.x + facing * (pace * cosTurns(heading) * seconds + wander),
    y: origin.y + pace * sinTurns(heading) * seconds,
    scale: 0.5 + 0.5 * smoothstep(age / 8),
    rotation: facing * RISE_ROCK * emitter.spread * cosTurns(swing),
    opacity: smoothstep(age / 6) * (1 - smoothstep((share - 0.6) / 0.4)),
    age,
  };
}

/** 🎆️ A thrown particle `age` ticks after the burst: particle `index` of `swarm` leaves at `speed × (0.45 + 0.55·u)` in its own slice of the fan of `spread` around straight up, the air brakes it (`BURST_DRAG × (1 − fastNegExp(seconds ÷ BURST_DRAG))` seconds of flight at full speed) and it sinks with {@link BURST_GRAVITY}. It shrinks to two fifths, fades as `1 − share²` and points the way it left. */
function thrown(emitter: Emission, origin: Point, facing: 1 | -1, key: number, index: number, age: Ticks, life: Ticks, swarm: number): Particle {
  const seconds = age / TICKS_PER_SECOND;
  const share = age / life;
  const heading = UP + ((index + scattered(key, index, LANE_HEADING)) / swarm - 0.5) * emitter.spread;
  const pace = emitter.speed * (0.45 + 0.55 * scattered(key, index, LANE_PACE));
  const reach = BURST_DRAG * (1 - fastNegExp(seconds / BURST_DRAG));
  return {
    x: origin.x + facing * (pace * cosTurns(heading) * reach),
    y: origin.y + pace * sinTurns(heading) * reach + 0.5 * BURST_GRAVITY * seconds * seconds,
    scale: 1 - 0.6 * share,
    rotation: facing > 0 ? heading : 0.5 - heading,
    opacity: 1 - share * share,
    age,
  };
}

/** 💫️ A circling particle `elapsed` ticks after its emitter began: particle `index` of `swarm` sits `index ÷ swarm × spread` of a turn along a ring that takes `life` ticks per lap at `speed`, so its radius is `speed × seconds per lap ÷ 2π`; the ring is squashed to {@link ORBIT_SQUASH} of its width, turns clockwise for a pet facing right and mirrored for one facing left, and a particle is larger at the front than at the back. Everything fades in over {@link ORBIT_FADE} ticks and out over as many from `until`. */
function circled(emitter: Emission, origin: Point, facing: 1 | -1, until: Ticks | null, tick: Ticks, index: number, elapsed: Ticks, life: Ticks, swarm: number): Particle {
  const radius = (emitter.speed * (life / TICKS_PER_SECOND)) / TURN_RADIANS;
  const turned = (index / swarm) * emitter.spread + elapsed / life;
  const angle = facing > 0 ? turned : 0.5 - turned;
  const depth = sinTurns(angle);
  return {
    x: origin.x + radius * cosTurns(angle),
    y: origin.y + radius * ORBIT_SQUASH * depth,
    scale: 1 + ORBIT_DEPTH * depth,
    rotation: 0,
    opacity: smoothstep(elapsed / ORBIT_FADE) * (until === null ? 1 : 1 - smoothstep((tick - until) / ORBIT_FADE)),
    age: elapsed,
  };
}

/** 🍃️ A drifting particle `age` ticks after its birth: it leaves at `speed × (0.5 + 0.5·u)` in a direction within `spread` around straight ahead and meanders {@link DRIFT_MEANDER} pixels across its path. It fades in over the first three tenths of its life and out over the last three, and points the way it travels. */
function wandered(emitter: Emission, origin: Point, facing: 1 | -1, key: number, index: number, age: Ticks, life: Ticks): Particle {
  const seconds = age / TICKS_PER_SECOND;
  const share = age / life;
  const look = scattered(key, index, LANE_LOOK);
  const heading = AHEAD + (scattered(key, index, LANE_HEADING) - 0.5) * emitter.spread;
  const pace = emitter.speed * (0.5 + 0.5 * scattered(key, index, LANE_PACE));
  const meander = DRIFT_MEANDER * sinTurns(DRIFT_RATE * seconds + scattered(key, index, LANE_PHASE));
  const along = cosTurns(heading);
  const across = sinTurns(heading);
  return {
    x: origin.x + facing * (pace * along * seconds - meander * across),
    y: origin.y + pace * across * seconds + meander * along,
    scale: 0.6 + 0.6 * look,
    rotation: facing > 0 ? heading : 0.5 - heading,
    opacity: smoothstep(share / 0.3) * (1 - smoothstep((share - 0.7) / 0.3)) * (0.6 + 0.4 * look),
    age,
  };
}
//#endregion 🔖️Motions

//#region 🔖️Particles
/** 🎇️ The particles of one emitter that are alive at `tick`, oldest first: the emitter's point `origin` on the stage, the way its pet faces, the tick it began (`since`), the tick it stopped or `null` while it runs (`until`), and the key of this run ({@link emitterKey}).
 *
 * A continuous emitter (`fall`, `rise`, `drift`) looks only at the births that can still be alive — the indices from
 * `⌊(tick − since − life) ÷ period⌋` to `⌊(tick − since) ÷ period⌋` — and keeps those born before `until` and
 * younger than `life`; when one more than its `count` is alive, the oldest is left out. A `burst` shows its `count`
 * particles for `life` ticks from `since`. An `orbit` shows its `count` particles from `since` until
 * {@link ORBIT_FADE} ticks after `until`. Nothing is alive before `since`.
 */
export function particlesOf(emitter: Emission, origin: Point, facing: 1 | -1, since: Ticks, until: Ticks | null, tick: Ticks, key: number): Particle[] {
  const life = lifeTicks(emitter);
  const swarm = swarmOf(emitter);
  const elapsed = tick - since;
  const particles: Particle[] = [];
  if (swarm < 1 || elapsed < 0) return particles;
  if (emitter.motion === "burst") {
    if (elapsed >= life) return particles;
    for (let index = 0; index < swarm; index++) particles.push(thrown(emitter, origin, facing, key, index, elapsed, life, swarm));
    return particles;
  }
  if (emitter.motion === "orbit") {
    if (until !== null && tick >= until + ORBIT_FADE) return particles;
    for (let index = 0; index < swarm; index++) particles.push(circled(emitter, origin, facing, until, tick, index, elapsed, life, swarm));
    return particles;
  }
  const period = periodOf(emitter);
  const first = Math.max(0, Math.floor((elapsed - life) / period));
  const last = Math.floor(elapsed / period);
  for (let index = first; index <= last; index++) {
    const born = since + index * period + Math.floor(scattered(key, index, LANE_BIRTH) * period);
    const age = tick - born;
    if (age < 0 || age >= life) continue;
    if (until !== null && born >= until) continue;
    particles.push(emitter.motion === "fall" ? fallen(emitter, origin, facing, key, index, age, life) : emitter.motion === "rise" ? risen(emitter, origin, facing, key, index, age, life) : wandered(emitter, origin, facing, key, index, age, life));
  }
  return particles.length > swarm ? particles.slice(particles.length - swarm) : particles;
}

/** ✂️ At most `cap` of the particles, in their order: the youngest stay — whoever is nearest to its birth is dropped last —, and among particles of the same age those earlier in the list stay, so the emitters listed first keep theirs. A stage passes all its particles in the order of its emitters and {@link STAGE_CAP}. */
export function capped(particles: readonly Particle[], cap: number): readonly Particle[] {
  const room = Math.floor(cap);
  if (particles.length <= room) return particles;
  if (!(room > 0)) return [];
  const ages = particles.map((particle) => particle.age).sort((left, right) => left - right);
  const eldest = ages[room - 1]!;
  let peers = room;
  for (const age of ages) if (age < eldest) peers -= 1;
  const kept: Particle[] = [];
  for (const particle of particles) {
    if (particle.age < eldest) kept.push(particle);
    else if (particle.age === eldest && peers > 0) {
      kept.push(particle);
      peers -= 1;
    }
  }
  return kept;
}

/** 🏁️ The first tick from which nothing of the emitter is alive any more — the horizon a stage has to stay awake for —, or `null` while the emitter runs without an end: a `burst` ends `life` ticks after `since`; an `orbit` {@link ORBIT_FADE} ticks after `until`; a continuous emitter `life − 1` ticks after `until` (its last birth is one tick before `until` at the latest); never before `since`, and at `since` when the emitter stopped before it could begin. */
export function emitterEnds(emitter: Emission, since: Ticks, until: Ticks | null): Ticks | null {
  const life = lifeTicks(emitter);
  if (swarmOf(emitter) < 1) return since;
  if (emitter.motion === "burst") return since + life;
  if (until === null) return null;
  if (emitter.motion === "orbit") return Math.max(since, until + ORBIT_FADE);
  return until <= since ? since : until + life - 1;
}
//#endregion 🔖️Particles

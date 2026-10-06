/** 🎪️ Subject adapter of the stage-trace case: the pets stage replays every committed script tick by tick, digests every frame and holds itself to the committed trace and to the laws of the stage.
 *
 * The digest is FNV-1a (32 bit) over the IEEE-754 bit patterns (little-endian doubles) of every number of every
 * frame, in this order: tick, rate, wake (−1 for none), the number of actors, then per actor in frame order the
 * index of its species in the menagerie, x, y, facing, the index of its activity in `ACTIVITIES`, opacity, every
 * bone number, per eye x, y and lid, spirits, the index of its footing in `FOOTINGS`, the index of its state among
 * the states of its species, the index of its mood in `MOODS`, intensity, tilt, pivot x and y, the number of its
 * tools and per tool the index of its kind (chute 0, rope 1, hook 2, gun 3, ladder 4) and its numbers in the order of
 * the schema, and its body x, y, width and height; then the number of ladders and per ladder x0, y0, x1, y1, rungs and
 * opacity; the number of particles and per particle the index of its species, the index of its emitter among the
 * emitters of that species, x, y, scale, rotation and opacity; the number of lifts and per lift dx, dy, tilt and
 * opacity; the number of puffs of dust and per puff x, y, width, height and phase; and the index of the species held
 * (−1 for none). Only the id of a lifted fixture, a string, is left out.
 * It runs on from frame to frame, so a checkpoint vouches for every frame before it.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🎪️stage/🟦️.ts
 * @see ../../🔨️modules/🧠️behavior/🟦️.ts — `MODE_LIMITS`, `followersOf`
 */
import { AssertionError } from "node:assert";
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { ACTIVITIES, FOOTINGS, MOODS, type Activity, type Extent, type Footing, type Frame, type Menagerie, type Slug, type Stage, type StageEvent, type ToolFrame } from "../../🧬️schema/🟦️.ts";
import { overlaps } from "../../🔨️modules/🚧️clearance/🟦️.ts";
import { bodiesOf, extentOf } from "../../🔨️modules/📏️spacing/🟦️.ts";
import { MODE_LIMITS, followersOf } from "../../🔨️modules/🧠️behavior/🟦️.ts";
import { advance, frameOf, openStage } from "../../🔨️modules/🎪️stage/🟦️.ts";
import { HOP_TICKS } from "../../🔨️modules/🏞️terrain/🟦️.ts";

const VECTORS = "shared://🎪️stage-trace/🔣️.json";
const FNV_OFFSET_BASIS = 2166136261;
const FNV_PRIME = 16777619;
const CHUNKS = [1, 7, 64, 3, 500, 2, 19, 1000];

/** 🪜️ The events that happen at one tick of a script, before that tick passes. */
export type Step = { readonly at: number; readonly events: readonly StageEvent[] };

/** 👁️ One actor as a checkpoint records it: who, what carries it (its perch, the wall of the pitch it clings to), where its feet are, what it does, with whom, whether it leaves, how visible it is and its body (the solid box no other body may overlap). */
export type Sighting = { readonly species: Slug; readonly perch: string | null; readonly wall: string | null; readonly footing: Footing; readonly x: number; readonly y: number; readonly activity: Activity; readonly partner: Slug | null; readonly leaving: boolean; readonly opacity: number; readonly body: Extent };

/** 📍️ What a checkpoint records: the tick, the digest of every frame up to it, the mode and the actors on stage. */
export type Checkpoint = { readonly tick: number; readonly digest: number; readonly mode: string; readonly actors: readonly Sighting[]; readonly drawn: Scenery };

/** 🎆️ What the frame of a checkpoint shows beside the actors: how many particles, standing ladders, lifted copies and puffs of dust, and who is held. */
export type Scenery = { readonly particles: number; readonly ladders: number; readonly lifts: number; readonly puffs: number; readonly held: Slug | null };

/** ⚖️ The laws every tick of every script keeps: actors on a perch stand on it, actors on a wall cling to a pitch of a surveyed wall and actors on a ladder ride one that stands (`perched`; one that scoots to a new seat stands at the height of its surface and on its way there, at most two of its widths off its perch) outside every keep-out (`clear`; but for a scooter on its way out of one), the bodies of no two actors overlap, wherever they are — on perches, in the air, under a parachute, in the hand, on a head (`apart`, the invariant of `🚧️clearance`) —, no more actors walk or hop than the mode allows (`paced`), at most one pair has partners (`paired`), activities follow the activity graph (`graphed`), and frames are well-formed — opacity in [0, 1], a known rate, a wake tick only at rate 0 and in the future, actors back to front (`whole`). */
export type Laws = { readonly perched: boolean; readonly clear: boolean; readonly apart: boolean; readonly paced: boolean; readonly paired: boolean; readonly graphed: boolean; readonly whole: boolean };

/** 🎞️ The trace of a script: how many frames it has, the digest of all of them, the checkpoints and the laws. */
export type Trace = { readonly frames: number; readonly digest: number; readonly checkpoints: readonly Checkpoint[]; readonly laws: Laws };

/** 📜️ A scripted event log: the seed of the stage, how many ticks pass, how often a checkpoint is taken, the events, and the committed trace. */
export type Script = { readonly id: string; readonly seed: number; readonly ticks: number; readonly every: number; readonly steps: readonly Step[]; readonly expected?: Trace };

type Vectors = { readonly menagerie: Menagerie; readonly scripts: readonly Script[] };

const BYTES = new DataView(new ArrayBuffer(8));

/** 🔢️ FNV-1a over the eight bytes of a double, little-endian. */
function fold(hash: number, value: number): number {
  BYTES.setFloat64(0, value, true);
  let folded = hash;
  for (let index = 0; index < 8; index++) folded = Math.imul(folded ^ BYTES.getUint8(index), FNV_PRIME) >>> 0;
  return folded;
}

/** 🧰️ The numbers of a tool in the order the digest folds them: the index of its kind (chute 0, rope 1, hook 2, gun 3, ladder 4), then its own numbers in the order of the schema. */
function toolNumbers(tool: ToolFrame): readonly number[] {
  if (tool.kind === "chute") return [0, tool.open, tool.sway];
  if (tool.kind === "rope") return [1, tool.x, tool.y, tool.slack];
  if (tool.kind === "hook") return [2, tool.x, tool.y];
  if (tool.kind === "gun") return [3, tool.aim];
  return [4, tool.lean, tool.length];
}

/** 🧮️ The digest after one more frame. */
function foldFrame(hash: number, menagerie: Menagerie, frame: Frame): number {
  let folded = fold(fold(fold(fold(hash, frame.tick), frame.rate), frame.wake === null ? -1 : frame.wake), frame.actors.length);
  for (const actor of frame.actors) {
    const kind = menagerie.species.find((species) => species.id === actor.species);
    folded = fold(folded, menagerie.species.findIndex((species) => species.id === actor.species));
    folded = fold(fold(fold(fold(fold(folded, actor.x), actor.y), actor.facing), ACTIVITIES.indexOf(actor.activity)), actor.opacity);
    for (const number of actor.bones) folded = fold(folded, number);
    for (const eye of actor.eyes) folded = fold(fold(fold(folded, eye.x), eye.y), eye.lid);
    folded = fold(folded, actor.spirits);
    folded = fold(fold(fold(fold(folded, FOOTINGS.indexOf(actor.footing)), kind === undefined ? -1 : kind.states.findIndex((state) => state.id === actor.state)), MOODS.indexOf(actor.mood)), actor.intensity);
    folded = fold(fold(fold(folded, actor.tilt), actor.pivot.x), actor.pivot.y);
    folded = fold(folded, actor.tools.length);
    for (const tool of actor.tools) for (const number of toolNumbers(tool)) folded = fold(folded, number);
    folded = fold(fold(fold(fold(folded, actor.body.x), actor.body.y), actor.body.width), actor.body.height);
  }
  folded = fold(folded, frame.ladders.length);
  for (const ladder of frame.ladders) folded = fold(fold(fold(fold(fold(fold(folded, ladder.x0), ladder.y0), ladder.x1), ladder.y1), ladder.rungs), ladder.opacity);
  folded = fold(folded, frame.particles.length);
  for (const particle of frame.particles) {
    const kind = menagerie.species.find((species) => species.id === particle.species);
    folded = fold(fold(folded, menagerie.species.findIndex((species) => species.id === particle.species)), kind === undefined ? -1 : kind.emitters.findIndex((emitter) => emitter.id === particle.emitter));
    folded = fold(fold(fold(fold(fold(folded, particle.x), particle.y), particle.scale), particle.rotation), particle.opacity);
  }
  folded = fold(folded, frame.lifts.length);
  for (const lift of frame.lifts) folded = fold(fold(fold(fold(folded, lift.dx), lift.dy), lift.tilt), lift.opacity);
  folded = fold(folded, frame.puffs.length);
  for (const puff of frame.puffs) folded = fold(fold(fold(fold(fold(folded, puff.x), puff.y), puff.width), puff.height), puff.phase);
  return fold(folded, frame.held === null ? -1 : menagerie.species.findIndex((species) => species.id === frame.held));
}

/** 🗂️ The events of a script by the tick they happen at. */
function eventsByTick(script: Script): Map<number, StageEvent[]> {
  const byTick = new Map<number, StageEvent[]>();
  for (const step of script.steps) byTick.set(step.at, [...(byTick.get(step.at) ?? []), ...step.events]);
  return byTick;
}

/** 🕸️ Whether every actor that is on stage before and after changed its activity along the activity graph. */
function graphed(before: Stage, after: Stage): boolean {
  for (const actor of after.actors) {
    const earlier = before.actors.find((candidate) => candidate.species === actor.species);
    if (earlier !== undefined && earlier.activity !== actor.activity && !followersOf(earlier.activity).includes(actor.activity)) return false;
  }
  return true;
}

/** 🧾️ The laws of one stage and its frame, given the tick of the last change of mode. */
function lawsOf(menagerie: Menagerie, stage: Stage, frame: Frame, tuned: number): Laws {
  let perched = true;
  let clear = true;
  let whole = (frame.rate === 0 || frame.rate === 16 || frame.rate === 32 || frame.rate === 64) && (frame.wake === null || (frame.rate === 0 && frame.wake > frame.tick)) && frame.tick === stage.tick && frame.actors.length === stage.actors.length;
  let movers = 0;
  let partners = 0;
  for (const actor of stage.actors) {
    const species = menagerie.species.find((candidate) => candidate.id === actor.species)!;
    const hover = species.locomotion.gait === "float" && species.locomotion.hover !== undefined ? species.locomotion.hover : 0;
    if ((actor.activity === "walk" || actor.activity === "hop") && !actor.leaving) movers++;
    if (actor.partner !== null) {
      partners++;
      if (!stage.actors.some((candidate) => candidate.species === actor.partner)) partners += 2;
    }
    if (!(actor.opacity >= 0 && actor.opacity <= 1)) whole = false;
    if ((actor.footing === "perch") !== (actor.perch !== null)) perched = false;
    const pitch = actor.pitch;
    if ((actor.footing === "wall") !== (pitch !== null) || (pitch !== null && !stage.walls.some((wall) => wall.id === pitch.wall && wall.side === pitch.side && wall.x === pitch.x))) perched = false;
    if (actor.footing === "ladder" && !stage.ladders.some((ladder) => ladder.rider === actor.species)) perched = false;
    if (actor.perch === null) continue;
    const scoots = actor.activity === "scoot";
    const slack = scoots ? 2 * species.size.width : 0;
    if (!stage.perches.some((perch) => perch.surface === actor.perch && perch.x0 - slack <= actor.x && actor.x <= perch.x1 + slack && actor.y === perch.y - hover)) perched = false;
    const left = actor.x - species.size.width / 2;
    const right = actor.x + species.size.width / 2;
    const top = actor.y - species.size.height;
    for (const keepout of stage.keepouts) if (!scoots && keepout.width > 0 && keepout.height > 0 && Math.max(left, keepout.x) < Math.min(right, keepout.x + keepout.width) && Math.max(top, keepout.y) < Math.min(actor.y, keepout.y + keepout.height)) clear = false;
    if (actor.x < 0 || actor.x > stage.width) whole = false;
  }
  for (let index = 1; index < frame.actors.length; index++) {
    const lower = frame.actors[index - 1]!;
    const upper = frame.actors[index]!;
    if (lower.y > upper.y || (lower.y === upper.y && !(lower.species < upper.species))) whole = false;
  }
  const apart = overlaps(bodiesOf(stage.actors, stage.actors.map((actor) => menagerie.species.find((candidate) => candidate.id === actor.species)!))).length === 0;
  return { perched, clear, apart, paced: movers <= MODE_LIMITS[stage.mode].movers || stage.tick - tuned <= HOP_TICKS + 1, paired: partners <= 2, graphed: true, whole };
}

/** 🎬️ The trace of a script over a menagerie, replayed tick by tick: the events of a tick are folded first, then the tick passes, then the frame is digested and the laws are checked. */
export function traceOf(menagerie: Menagerie, script: Script, seed: number = script.seed): { trace: Trace; stage: Stage } {
  const byTick = eventsByTick(script);
  const checkpoints: Checkpoint[] = [];
  let stage = openStage(seed);
  let hash = FNV_OFFSET_BASIS;
  let tuned = -HOP_TICKS - 2;
  let laws: Laws = { perched: true, clear: true, apart: true, paced: true, paired: true, graphed: true, whole: true };
  for (let tick = 1; tick <= script.ticks; tick++) {
    const events = byTick.get(tick - 1) ?? [];
    if (events.some((event) => event.kind === "tuned")) tuned = tick - 1;
    const evented = advance(menagerie, stage, events);
    const ticked = advance(menagerie, evented, [{ kind: "ticked", ticks: 1 }]);
    const frame = frameOf(menagerie, ticked);
    const kept = lawsOf(menagerie, ticked, frame, tuned);
    laws = { perched: laws.perched && kept.perched, clear: laws.clear && kept.clear, apart: laws.apart && kept.apart, paced: laws.paced && kept.paced, paired: laws.paired && kept.paired, graphed: laws.graphed && graphed(stage, evented) && graphed(evented, ticked), whole: laws.whole && kept.whole };
    hash = foldFrame(hash, menagerie, frame);
    stage = ticked;
    if (tick % script.every !== 0 && tick !== script.ticks) continue;
    checkpoints.push({ tick, digest: hash, mode: stage.mode, actors: stage.actors.map((actor) => ({ species: actor.species, perch: actor.perch, wall: actor.pitch === null ? null : actor.pitch.wall, footing: actor.footing, x: actor.x, y: actor.y, activity: actor.activity, partner: actor.partner, leaving: actor.leaving, opacity: actor.opacity, body: extentOf(actor, menagerie.species.find((candidate) => candidate.id === actor.species)!) })), drawn: { particles: frame.particles.length, ladders: frame.ladders.length, lifts: frame.lifts.length, puffs: frame.puffs.length, held: frame.held } });
  }
  return { trace: { frames: script.ticks, digest: hash, checkpoints, laws }, stage };
}

/** ✂️ The stage a script ends in when its ticks are not passed one by one but in the uneven chunks of `CHUNKS`, as far as the events allow. */
function chunkedStage(menagerie: Menagerie, script: Script): Stage {
  const byTick = eventsByTick(script);
  let stage = openStage(script.seed);
  let tick = 0;
  let turn = 0;
  while (tick < script.ticks) {
    stage = advance(menagerie, stage, byTick.get(tick) ?? []);
    let next = script.ticks;
    for (const at of byTick.keys()) if (at > tick && at < next) next = at;
    const ticks = Math.min(CHUNKS[turn % CHUNKS.length]!, next - tick);
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks }]);
    tick += ticks;
    turn++;
  }
  return stage;
}

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

const replayed = new Map<string, { trace: Trace; stage: Stage }>();

/** 🗃️ The trace of a committed script, replayed once per run. */
function replay(menagerie: Menagerie, script: Script): { trace: Trace; stage: Stage } {
  const known = replayed.get(script.id);
  if (known !== undefined) return known;
  const fresh = traceOf(menagerie, script);
  replayed.set(script.id, fresh);
  return fresh;
}

/** 📏️ Holds a replayed trace to the committed one, exactly. */
function conforming(script: Script, trace: Trace): Trace {
  if (JSON.stringify(trace) !== JSON.stringify(script.expected)) throw new AssertionError({ message: `stage-trace/${script.id}: the replayed trace differs from the committed one`, actual: trace, expected: script.expected });
  return trace;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    traces: {
      subject: (ctx) => {
        const { menagerie, scripts } = vectors(ctx);
        return { projection: Object.fromEntries(scripts.map((script) => [script.id, conforming(script, replay(menagerie, script).trace)])) };
      },
    },
    laws: {
      subject: (ctx) => {
        const { menagerie, scripts } = vectors(ctx);
        return { projection: Object.fromEntries(scripts.map((script) => [script.id, replay(menagerie, script).trace.laws])) };
      },
    },
    determinism: {
      subject: (ctx) => {
        const { menagerie, scripts } = vectors(ctx);
        return {
          projection: Object.fromEntries(
            scripts.map((script) => {
              const first = replay(menagerie, script);
              const again = traceOf(menagerie, script);
              const other = traceOf(menagerie, script, script.seed + 1);
              return [script.id, { repeatable: JSON.stringify(again) === JSON.stringify(first), seeded: other.trace.digest !== first.trace.digest, chunked: JSON.stringify(chunkedStage(menagerie, script)) === JSON.stringify(first.stage) }];
            }),
          ),
        };
      },
    },
  },
});

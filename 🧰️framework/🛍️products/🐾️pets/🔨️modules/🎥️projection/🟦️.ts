/** 📽️ The projection of a stage into a frame: the pose of every actor (the idle loop underneath in the look of its state, the clip of its activity on top of it or laying it to rest, the posture of its mood), its face, tilt, tools and body, the particles of every plume, the ladders that stand, the lifted copy and who is held, the rate all of it needs (`paceOf`, `plumePace`, `sceneryPace`), and `frameOf` — the actors back to front, the rest of the scenery, the rate of the frame and the tick it must wake at.
 *
 * Whatever a render target is to draw is projected here, from the stage alone; particles, lifts and ladders are pure
 * functions of the stage's tick, so a frame can be recomputed, skipped or replayed.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ./🦀️.rs — the Rust twin
 */

import { TICKS_PER_SECOND, type Activity, type Actor, type ActorFrame, type Clip, type EyeFrame, type Frame, type LadderFrame, type LiftFrame, type Menagerie, type ParticleFrame, type Point, type PuffFrame, type Slug, type Species, type Stage, type Ticks, type ToolFrame, type Turns } from "../../🧬️schema/🟦️.ts";
import { BLINK_TICKS, blendPose, clipTicks, lidAt, sampleClip } from "../🎞️animation/🟦️.ts";
import { PERK_LINGER, POINTER_TICKS, TURN_REST, gazeRests, leant, presenceOf, squeezeOf } from "../👀️attention/🟦️.ts";
import { STAGE_CAP, capped, emitterEnds, emitterKey, particlesOf, type Particle } from "../✨️effects/🟦️.ts";
import { hoverBusy, pressDue } from "../👆️gesture/🟦️.ts";
import { atRest, faceOf, settled, settlesAt, spiritsOf, stateAt, stateEnds } from "../💗️feeling/🟦️.ts";
import { PUFF_TICKS } from "../🚶️locomotion/🟦️.ts";
import { atanTurns, clamp, cosTurns, sinTurns, smoothstep } from "../📐️trigonometry/🟦️.ts";
import { breathOf, clipOf } from "../📝️draft/🟦️.ts";
import { extentOf, wallLeanOf } from "../📏️spacing/🟦️.ts";
import { arrivalTick, beatTick, pairingTick, prankTick } from "../🗓️schedule/🟦️.ts";
import { HOOK_RETURN, HOOK_SPEED, MUZZLE_FORWARD, MUZZLE_HEIGHT, climbPhase, hookStep, hookTicks, ladderLength, ladderPhase, ladderRungs } from "../🧗️climbing/🟦️.ts";
import { LIFT_TICKS, liftAt, liftEnds, liftWake } from "../🪄️mischief/🟦️.ts";
import { CHUTE_ROD, leanOf } from "../🪢️swing/🟦️.ts";
import { pupilReach, restPose, solveRig, type BonePose, type Pose } from "../🦴️rig/🟦️.ts";

//#region 🔖️Constants
const BLEND_TICKS = 8;
const BREATH_STAGGER = 37;
const NO_TOOLS: readonly ToolFrame[] = [];

/** 🔀️ The ticks over which the look of a state blends in from the look of the state before it, channel by channel; the tint of the new state shows from the middle of them. */
export const STATE_BLEND = 16;

/** 🧺️ How far a carried ladder leans from upright towards the way its carrier faces, in turns: nearly lying. */
export const CARRY_LEAN = 0.22;

/** 📏️ How long a carried ladder is, in heights of its carrier. */
export const CARRY_LENGTH = 2;

/** 🔫️ How far above level an actor that aims without a shot holds its grappling gun, in turns. */
export const AIM_RISE = 0.125;

/** 🌁️ The ticks over which a standing ladder fades in after it stood up and out before it is taken away. */
export const LADDER_FADE = 8;

/** 🏎️ The speed in pixels per second from which the particles of an emitter move so far per tick that they need every tick (rate 64); slower ones drift well at every other tick (rate 32). A burst always needs every tick. */
export const BRISK = 32;
//#endregion 🔖️Constants

//#region 🔖️Pose
/** 🧱️ Whether the clip of an activity replaces the idle loop underneath it (cross-fade) instead of being added on top of it: the gaits (walking, scooting, carrying, climbing, sliding, mantling), the flights (hop, fall, tumble), the landing and sleep bring their own body motion. The look of a state stays underneath every one of them. */
function replaces(activity: Activity): boolean {
  return activity === "walk" || activity === "hop" || activity === "fall" || activity === "land" || activity === "sleep" || activity === "tumble" || activity === "scoot" || activity === "carry" || activity === "climb" || activity === "slide" || activity === "mantle";
}

/** 🎛️ How much of its clip an actor shows: it fades in over the first 8 ticks of the activity and out over the last 8 — of its span, or of the way to its goal when it walks or scoots; flights end hard; a climber on a wall or a ladder shows all of it, climbing or resting in its pose. */
function weightOf(kind: Species, actor: Actor, tick: Ticks): number {
  const activity = actor.activity;
  if (clinging(actor)) return 1;
  const into = (tick - actor.since) / BLEND_TICKS;
  const out = activity === "hop" || activity === "fall" || activity === "tumble" ? 1 : activity === "walk" || activity === "scoot" ? (Math.abs(actor.goal - actor.x) * TICKS_PER_SECOND) / (Math.max(kind.locomotion.speed, 1) * BLEND_TICKS) : (actor.until - tick) / BLEND_TICKS;
  return clamp(Math.min(into, out), 0, 1);
}

/** 🫳️ Whether an actor climbs on a wall or a ladder, or rests there in the pose of its climb. */
function clinging(actor: Actor): boolean {
  return actor.activity === "climb" && (actor.footing === "wall" || actor.footing === "ladder");
}

/** 🎡️ Where in its clip an actor is at a tick, in whole ticks: a climber on a wall or a ladder ({@link clinging}) turns its clip with the distance it climbed — one cycle per two holds of its pitch (`climbPhase`) or per two rungs of the ladder it rides (`ladderPhase`) —, so the clip stands still while it rests and its hands meet the holds; every other clip plays from the tick its activity began. */
function clipTime(stage: Stage, kind: Species, actor: Actor, clip: Clip, tick: Ticks): Ticks {
  if (!clinging(actor)) return tick - actor.since;
  if (actor.footing === "wall") return actor.pitch === null ? 0 : Math.floor(climbPhase(actor.pitch, actor.y, kind.size) * clipTicks(clip));
  const ladder = stage.ladders.find((entry) => entry.rider === actor.species);
  if (ladder === undefined) return 0;
  const share = clamp((actor.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0, 1);
  return Math.floor(ladderPhase(share * ladderLength(ladder)) * clipTicks(clip));
}

/** 🛬️ How far a replacing clip has laid the idle loop to rest: as far as the clip weighs, except on a landing, which begins with the loop at rest — where the flight before it left the body — and hands it back only as it ends, so the body never jumps between a flight and its landing. */
function restingOf(actor: Actor, tick: Ticks, weight: number): number {
  return actor.activity === "land" ? clamp((actor.until - tick) / BLEND_TICKS, 0, 1) : weight;
}

/** 🎭️ The clip a state of a species shows itself with (its look), or `null` for a state without one or a state the species does not have. */
function lookOf(kind: Species, state: Slug): Clip | null {
  for (const entry of kind.states) if (entry.id === state) return entry.clip === undefined ? null : clipOf(kind, entry.clip);
  return null;
}

/** 🗝️ Which channels of which bones a clip keys: five flags per bone in rig order, in `CHANNELS` order (x, y, rotation, scaleX, scaleY); `null` for no clip. */
function keysOf(kind: Species, clip: Clip | null): boolean[] | null {
  if (clip === null) return null;
  const keyed = new Array<boolean>(kind.bones.length * 5).fill(false);
  for (const track of clip.tracks) {
    const bone = kind.bones.findIndex((entry) => entry.id === track.bone);
    if (bone < 0) continue;
    keyed[bone * 5 + (track.channel === "x" ? 0 : track.channel === "y" ? 1 : track.channel === "rotation" ? 2 : track.channel === "scaleX" ? 3 : 4)] = true;
  }
  return keyed;
}

/** 🎚️ One channel of the pose of a bone, by its place in `CHANNELS`. */
function channelOf(bone: BonePose, channel: number): number {
  return channel === 0 ? bone.x : channel === 1 ? bone.y : channel === 2 ? bone.rotation : channel === 3 ? bone.scaleX : bone.scaleY;
}

/** 🌗️ The underlay `under` (the breathing pose) in the look of a state, `blend` (0…1) of the way from the look of the former state to the look of the present one: every channel the clip of a state keys is that clip's (sampled on the clock of the stage, at `phase`, like the idle loop), every other channel stays the underlay's — so a look that stops a spinning part holds it, while what it leaves alone keeps breathing —, and the two looks are blended channel by channel. Without a look, and once the blend is done without one, the underlay is returned as it is. */
function lookedOf(kind: Species, under: Pose, present: Clip | null, former: Clip | null, blend: number, phase: Ticks): Pose {
  const done = blend >= 1 || former === present;
  if (present === null && (done || former === null)) return under;
  const now = present === null ? null : sampleClip(kind, present, phase);
  const nowKeys = keysOf(kind, present);
  const was = done || former === null ? null : sampleClip(kind, former, phase);
  const wasKeys = done ? null : keysOf(kind, former);
  return under.map((bone, index) => {
    const values: number[] = [];
    for (let channel = 0; channel < 5; channel++) {
      const base = channelOf(bone, channel);
      const to = now !== null && nowKeys![index * 5 + channel]! ? channelOf(now[index]!, channel) : base;
      const from = was !== null && wasKeys![index * 5 + channel]! ? channelOf(was[index]!, channel) : base;
      values.push(done ? to : from + (to - from) * blend);
    }
    return { x: values[0]!, y: values[1]!, rotation: values[2]!, scaleX: values[3]!, scaleY: values[4]! };
  });
}

/** 🪶️ The pose with the bone that carries the first eye sunk by `drop` pixels (lifted while `drop` is negative): the posture a mood gives the face (`faceOf`). */
function drooped(kind: Species, pose: Pose, drop: number): Pose {
  const eye = kind.face.eyes[0];
  if (eye === undefined || drop === 0) return pose;
  return pose.map((bone, index) => (kind.bones[index]!.id === eye.bone ? { x: bone.x, y: bone.y + drop, rotation: bone.rotation, scaleX: bone.scaleX, scaleY: bone.scaleY } : bone));
}

/** 🚩️ The state an actor shows at a tick, the tick it began and the state it blends from: `stateAt` of the state it entered, and as the former one the state it entered from (`Actor.former`) — or, when a lasting state has given way since, the state that gave way. */
function lookingOf(kind: Species, actor: Actor, tick: Ticks): { readonly state: Slug; readonly since: Ticks; readonly former: Slug } {
  const standing = stateAt(kind, actor.state, actor.stateSince, tick);
  return { state: standing.state, since: standing.since, former: standing.since === actor.stateSince ? actor.former : stateAt(kind, actor.state, actor.stateSince, standing.since - 1).state };
}

/** 🧩️ One pose on top of another: offsets and rotations add, scale factors multiply. */
function layer(under: Pose, over: Pose): Pose {
  return under.map((bone, index) => {
    const other = over[index]!;
    return { x: bone.x + other.x, y: bone.y + other.y, rotation: bone.rotation + other.rotation, scaleX: bone.scaleX * other.scaleX, scaleY: bone.scaleY * other.scaleY };
  });
}

/** 🤸️ The pose of an actor at a tick: the first idle clip of its species loops underneath on the clock of the stage (staggered by 37 ticks per stream, so a pet that stands still is never frozen and no two breathe in step), in the look of its state ({@link lookedOf}: blended in from the former look over {@link STATE_BLEND} ticks, `blend`), and the clip of its activity at {@link clipTime}, weighted by {@link weightOf} from the rest pose, lies on top of it or lays the idle loop to rest ({@link restingOf}) — never the look. A walker that still turns round has not set out yet: its walk weighs nothing. */
function poseOf(stage: Stage, kind: Species, actor: Actor, tick: Ticks, stream: number, present: Clip | null, former: Clip | null, blend: number): Pose {
  const rest = restPose(kind);
  const breath = breathOf(kind);
  const top = clipOf(kind, actor.clip);
  const phase = tick + stream * BREATH_STAGGER;
  const idle = breath === null ? rest : sampleClip(kind, breath, phase);
  if (top === null || (breath !== null && top.id === breath.id)) return lookedOf(kind, idle, present, former, blend, phase);
  const weight = tick < actor.faced && (actor.activity === "walk" || actor.activity === "scoot") ? 0 : weightOf(kind, actor, tick);
  const over = blendPose(rest, sampleClip(kind, top, clipTime(stage, kind, actor, top, tick)), weight);
  const under = replaces(actor.activity) ? blendPose(idle, rest, restingOf(actor, tick, weight)) : idle;
  return layer(lookedOf(kind, under, present, former, blend, phase), over);
}

//#endregion 🔖️Pose

//#region 🔖️Gear
/** 🧷️ The point the drawing of an actor turns about, in the coordinates of its rig (its feet at the origin, x mirrored with the actor): its hands while it hangs on a rope (where the rope holds it), else its scruff (`Species.grip` above the feet), about which the stage tilts a body that hangs, tumbles or rights itself. */
function pivotOf(kind: Species, actor: Actor): Point {
  return actor.footing === "rope" ? { x: MUZZLE_FORWARD * kind.size.width, y: 0 - MUZZLE_HEIGHT * kind.size.height } : { x: 0, y: -kind.grip };
}

/** 🦎️ The tilt of the drawing of an actor in turns, clockwise on screen: the tilt of its body — the tilt the stage gave it (hanging from the hand, tumbling, righting itself) and, while it clings to a wall, its lean towards that wall, given up as its feet leave the line it clings to (`wallLeanOf`) —, so the body the stage keeps apart always holds the drawing. */
function tiltOf(kind: Species, actor: Actor): Turns {
  return actor.tilt + wallLeanOf(kind, actor.footing, actor.pitch, actor.x);
}

/** 📍️ Where the origin of an actor's rig is placed before its drawing turns by `tilt` about `pivot` (rig coordinates), so that the turn carries the feet of the rig onto `feet`: the feet themselves while it is upright. A render target places the rig there and turns it about the pivot (`translate(x, y) scale(±1, 1)`, then the turn about the pivot inside the mirrored drawing). */
function placed(feet: Point, facing: 1 | -1, tilt: Turns, pivot: Point): Point {
  if (tilt === 0) return feet;
  const sine = sinTurns(tilt);
  const cosine = cosTurns(tilt);
  const across = facing * pivot.x;
  return { x: feet.x + (cosine * across - sine * pivot.y) - across, y: feet.y + (sine * across + cosine * pivot.y) - pivot.y };
}

/** 🗺️ Where the point (`x`, `y`) of an actor's rig lies on the stage as a render target draws it: mirrored with the actor, turned by `tilt` about `pivot` and placed at `origin` ({@link placed}). */
function staged(origin: Point, facing: 1 | -1, tilt: Turns, pivot: Point, x: number, y: number): Point {
  const across = facing * x;
  if (tilt === 0) return { x: origin.x + across, y: origin.y + y };
  const sine = sinTurns(tilt);
  const cosine = cosTurns(tilt);
  const turn = facing * pivot.x;
  const dx = across - turn;
  const dy = y - pivot.y;
  return { x: origin.x + turn + (cosine * dx - sine * dy), y: origin.y + pivot.y + (sine * dx + cosine * dy) };
}

/** 🎒️ What an actor holds or hangs from, in the order a render target layers them (behind its drawing the parachute and the ladder, in front of it the rope, the hook and the gun): its parachute while it is out — open as far as the stage has it, canopy and cords leaning as the cords lean from the feet up to the canopy (`leanOf` over `CHUTE_ROD` heights) —; the ladder it carries (leaning {@link CARRY_LEAN} the way it faces, {@link CARRY_LENGTH} heights long); its grappling line — while the hook flies the rope paid out to the hook and the gun aimed at the edge, then the taut rope to the hook it hangs from; after a miss the hook flies past the edge and is pulled back to the gun (`HOOK_RETURN`), the gun aimed the whole time —; and while it aims without a shot the gun raised {@link AIM_RISE} the way it faces. */
function toolsOf(kind: Species, actor: Actor, tick: Ticks): readonly ToolFrame[] {
  const chute = actor.chute;
  const rope = actor.rope;
  if (chute === null && rope === null && actor.activity !== "carry" && actor.activity !== "aim") return NO_TOOLS;
  const tools: ToolFrame[] = [];
  if (chute !== null) tools.push({ kind: "chute", open: chute.open, sway: leanOf(chute.canopy, { x: actor.x, y: actor.y }, CHUTE_ROD * kind.size.height) });
  if (actor.activity === "carry") tools.push({ kind: "ladder", lean: actor.facing * CARRY_LEAN, length: CARRY_LENGTH * kind.size.height });
  if (rope !== null) {
    const shot = rope.shot;
    const flown = tick - rope.since;
    const out = hookTicks(shot.muzzle, shot.hook, HOOK_SPEED);
    const flying = flown < out || !rope.caught;
    const hook = flown < out ? hookStep(shot.muzzle, shot.hook, HOOK_SPEED, flown) : rope.caught ? shot.hook : hookStep(shot.hook, shot.muzzle, HOOK_RETURN, flown - out);
    tools.push({ kind: "rope", x: hook.x, y: hook.y, slack: 0 }, { kind: "hook", x: hook.x, y: hook.y });
    if (flying) tools.push({ kind: "gun", aim: atanTurns(shot.hook.y - shot.muzzle.y, shot.hook.x - shot.muzzle.x) });
  } else if (actor.activity === "aim") tools.push({ kind: "gun", aim: actor.facing > 0 ? 0 - AIM_RISE : AIM_RISE - 0.5 });
  return tools;
}

/** ✊️ The actor in the learner's hand, or `null`: the first, in the order of the menagerie, that hangs from the hand. */
function heldOf(actors: readonly Actor[]): Slug | null {
  for (const actor of actors) if (actor.hang !== null) return actor.species;
  return null;
}
//#endregion 🔖️Gear

//#region 🔖️Scenery
/** 🎇️ One particle on its way into the frame: the particle, the species and the emitter it belongs to and the opacity of its actor. */
type Spark = { readonly particle: Particle; readonly species: Slug; readonly emitter: Slug; readonly opacity: number };

/** 💥️ The particles of every plume an actor runs, appended to `sparks`: each emitter at its bone — the point `x`, `y` of the emitter carried by the bone's matrix of the frame (`bones`), mirrored, turned and placed like the drawing ({@link staged}) —, its particles as pure functions of the plume's begin and end and the tick (`particlesOf`), keyed by the stage's seed, the stream of the species, the place of the emitter and the plume's begin (`emitterKey`). A plume of an emitter the species does not have shows nothing. */
function sparksOf(stage: Stage, kind: Species, actor: Actor, stream: number, bones: readonly number[], origin: Point, tilt: Turns, pivot: Point, opacity: number, sparks: Spark[]): void {
  for (const plume of actor.emitters) {
    const index = kind.emitters.findIndex((entry) => entry.id === plume.emitter);
    if (index < 0) continue;
    const emitter = kind.emitters[index]!;
    const bone = Math.max(kind.bones.findIndex((entry) => entry.id === emitter.bone), 0) * 6;
    const x = bones[bone]! * emitter.x + bones[bone + 2]! * emitter.y + bones[bone + 4]!;
    const y = bones[bone + 1]! * emitter.x + bones[bone + 3]! * emitter.y + bones[bone + 5]!;
    const at = staged(origin, actor.facing, tilt, pivot, x, y);
    for (const particle of particlesOf(emitter, at, actor.facing, plume.since, plume.until, stage.tick, emitterKey(stage.seed, stream, index, plume.since))) sparks.push({ particle, species: actor.species, emitter: emitter.id, opacity });
  }
}

/** 🎆️ The particles of a frame: the sparks of every actor in drawing order, at most {@link STAGE_CAP} of them — the youngest stay (`capped`) —, each as strong as its particle times the opacity of its actor. */
function particlesFrom(sparks: readonly Spark[]): ParticleFrame[] {
  const kept = capped(
    sparks.map((spark) => spark.particle),
    STAGE_CAP,
  );
  const chosen = kept.length === sparks.length ? null : new Set(kept);
  const particles: ParticleFrame[] = [];
  for (const spark of sparks) {
    const particle = spark.particle;
    if (chosen !== null && !chosen.has(particle)) continue;
    particles.push({ species: spark.species, emitter: spark.emitter, x: particle.x, y: particle.y, scale: particle.scale, rotation: particle.rotation, opacity: particle.opacity * spark.opacity });
  }
  return particles;
}

/** 💨️ How many ticks per second the plumes of an actor need at a tick: 64 while a brisk one (a burst, or one whose particles leave at {@link BRISK} pixels per second or faster) has begun and not yet ended (`emitterEnds`), 32 while only slower ones have, 0 when none has. */
function plumePace(kind: Species, actor: Actor, tick: Ticks): 0 | 32 | 64 {
  let pace: 0 | 32 | 64 = 0;
  for (const plume of actor.emitters) {
    const emitter = kind.emitters.find((entry) => entry.id === plume.emitter);
    if (emitter === undefined || tick < plume.since) continue;
    const ends = emitterEnds(emitter, plume.since, plume.until);
    if (ends !== null && tick >= ends) continue;
    if (emitter.motion === "burst" || emitter.speed >= BRISK) return 64;
    pace = 32;
  }
  return pace;
}

/** 🪜️ The ladders that stand on the stage as drawn: from foot to top, one rung per `RUNG_SPACING` of their length (`ladderRungs`), fading in over {@link LADDER_FADE} ticks after they stood up and out over as many before they are taken away; on a still stage whole while they stand. */
function laddersOf(stage: Stage, still: boolean): LadderFrame[] {
  const ladders: LadderFrame[] = [];
  for (const ladder of stage.ladders) {
    const opacity = still ? (stage.tick >= ladder.since && stage.tick < ladder.until ? 1 : 0) : Math.min(smoothstep((stage.tick - ladder.since) / LADDER_FADE), smoothstep((ladder.until - stage.tick) / LADDER_FADE));
    if (!(opacity > 0)) continue;
    ladders.push({ x0: ladder.foot.x, y0: ladder.foot.y, x1: ladder.top.x, y1: ladder.top.y, rungs: ladderRungs(ladder), opacity });
  }
  return ladders;
}

/** 🪞️ The lifted fixture as drawn: the offset, tilt and opacity of its copy at the tick (`liftAt`) from the tick the lift began until it is over (`LIFT_TICKS`); nothing on a still stage. */
function liftsOf(stage: Stage, still: boolean): LiftFrame[] {
  const lift = stage.lift;
  if (lift === null || still || stage.tick < lift.since || stage.tick >= lift.since + LIFT_TICKS) return [];
  const copy = liftAt(lift.since, stage.tick, lift.side, lift.room, lift.span, lift.unit);
  return [{ fixture: lift.fixture, dx: copy.dx, dy: copy.dy, tilt: copy.tilt, opacity: copy.opacity }];
}

/** 🌫️ The dust where pets vanished, as drawn: the middle and size of each vanished body and how far its dust has spread, `(tick − vanished) ÷ PUFF_TICKS`, while that is below 1; nothing on a still stage. */
function puffsOf(stage: Stage, still: boolean): PuffFrame[] {
  const puffs: PuffFrame[] = [];
  if (still) return puffs;
  for (const puff of stage.puffs) {
    const phase = (stage.tick - puff.tick) / PUFF_TICKS;
    if (phase >= 0 && phase < 1) puffs.push({ x: puff.x, y: puff.y, width: puff.width, height: puff.height, phase });
  }
  return puffs;
}

/** 🎠️ How many ticks per second the things on stage beside the actors need: 64 while the lifted copy moves (`liftWake` is the next tick), a standing ladder fades in or out or the dust of a vanished pet spreads, 0 otherwise. */
function sceneryPace(stage: Stage): 0 | 64 {
  const tick = stage.tick;
  const lift = stage.lift;
  if (lift !== null && liftWake(lift.since, tick) === tick + 1) return 64;
  for (const puff of stage.puffs) if (tick >= puff.tick && tick < puff.tick + PUFF_TICKS) return 64;
  for (const ladder of stage.ladders) if ((tick >= ladder.since && tick < ladder.since + LADDER_FADE) || (tick >= ladder.until - LADDER_FADE && tick < ladder.until)) return 64;
  return 0;
}

/** ⏰️ The first tick after `tick` at which the scenery or a plume changes while nothing moves, or `Infinity`: a plume that has not begun yet or not yet ended (`emitterEnds`), the lifted copy setting out, resting until it is put back or ending (`liftWake`, `liftEnds`), a ladder that stands up or begins to fade out. */
function sceneryWake(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], tick: Ticks): number {
  let horizon = Infinity;
  for (let index = 0; index < actors.length; index++) {
    for (const plume of actors[index]!.emitters) {
      const emitter = kinds[index]!.emitters.find((entry) => entry.id === plume.emitter);
      if (emitter === undefined) continue;
      const ends = emitterEnds(emitter, plume.since, plume.until);
      if (plume.since > tick) horizon = Math.min(horizon, plume.since);
      else if (ends !== null && ends > tick) horizon = Math.min(horizon, ends);
    }
  }
  const lift = stage.lift;
  if (lift !== null) {
    const next = liftWake(lift.since, tick);
    if (next !== null) horizon = Math.min(horizon, next);
    if (liftEnds(lift.since) > tick) horizon = Math.min(horizon, liftEnds(lift.since));
  }
  for (const ladder of stage.ladders) {
    if (ladder.since > tick) horizon = Math.min(horizon, ladder.since);
    else if (ladder.until - LADDER_FADE > tick) horizon = Math.min(horizon, ladder.until - LADDER_FADE);
  }
  return horizon;
}
//#endregion 🔖️Scenery

//#region 🔖️Pace
/** 🎼️ How many ticks per second the motion of one actor needs at a tick: 64 while it fades (in, out, or see-through in front of a keep-out and back), turns round, is on a trip with its gear, walks, scoots, carries, flies, lands, stands on anything but a perch, a wall or a ladder, has its grappling line out, plays a clip that does not loop, blends a clip in or out, blends from one look of its state to another, runs a brisk plume ({@link plumePace}) or while a gesture of the pointer is under way round it; 32 while a loop (the idle loop, a looping clip, a looping look), a blink, its pupils, its feeling (until it `settlesAt` its rest) or a slow plume move; 16 while it only sleeps; 0 when nothing of it moves. A climber that rests on a wall or a ladder holds its clip still ({@link clipTime}) and its pose lays the idle loop to rest: only its look, its eyes, its feeling and its plumes move. */
function paceOf(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number): 0 | 16 | 32 | 64 {
  const actor = actors[index]!;
  const kind = kinds[index]!;
  const tick = stage.tick;
  const activity = actor.activity;
  const held = actor.footing === "perch" || actor.footing === "wall" || actor.footing === "ladder";
  if (!held || actor.leaving || actor.opacity !== presenceOf(stage.keepouts, actor, kind) || tick < actor.faced || stage.trips.some((trip) => trip.owner === actor.species) || activity === "walk" || activity === "hop" || activity === "fall" || activity === "land" || activity === "scoot" || activity === "carry" || actor.rope !== null || hoverBusy(actor.hover)) return 64;
  const resting = clinging(actor);
  const breath = breathOf(kind);
  const top = clipOf(kind, actor.clip);
  const layered = top !== null && (breath === null || top.id !== breath.id) && !resting;
  if (layered && ((!top.loop && tick - actor.since < clipTicks(top)) || tick - actor.since < BLEND_TICKS || actor.until - tick < BLEND_TICKS)) return 64;
  const looking = lookingOf(kind, actor, tick);
  const look = lookOf(kind, looking.state);
  if (tick - looking.since < STATE_BLEND && looking.former !== looking.state && (look !== null || lookOf(kind, looking.former) !== null)) return 64;
  const plumes = plumePace(kind, actor, tick);
  if (plumes === 64) return 64;
  const restless = settlesAt(actor.feeling, kind.mood) > tick || !gazeRests(stage, actors, kinds, index, tick + 1) || plumes === 32;
  const looping = (look !== null && look.loop) || (breath !== null && !resting);
  if (activity === "sleep") return restless ? 32 : layered || looping ? 16 : 0;
  if (restless || (tick >= actor.blink && tick < actor.blink + BLINK_TICKS)) return 32;
  return (layered && top.loop) || looping ? 32 : 0;
}
//#endregion 🔖️Pace

//#region 🔖️Frame
/** 🎥️ What a render target draws for a stage: the actors back to front (by `y`, then species id), the ladders that stand, the particles, the lifted fixture and who is held; the rate the motion needs and, at rate 0, the tick of the next scheduled change.
 *
 * Per actor: where its rig is placed (`x`, `y`: its feet, unless its drawing is tilted — then the place from which the
 * turn by `tilt` about `pivot` carries the feet of the rig onto its feet, {@link placed}), the way it faces, its
 * activity and opacity; one matrix per bone (`solveRig` of {@link poseOf} — the idle loop in the look of its state,
 * the clip of its activity on top —, sunk by the posture of its mood ({@link drooped}), leaning after the gaze
 * ({@link leant}) outside a time of concentration, squeezed across by {@link squeezeOf} while it turns round); its
 * eyes (pupil offset = gaze × `pupilReach`, mirrored with the actor as drawn; the lid at the resting height its mood
 * gives it with the blink on top, shut asleep); its footing; its state (the one it blends from until half of
 * {@link STATE_BLEND} has passed, so the tint changes in the middle of the blend); the mood it feels, its intensity
 * and its spirits (`spiritsOf`, the bend of the mouth — the slant of its lids is `faceOf` of mood and intensity); the
 * tilt of its drawing ({@link tiltOf}) about its pivot ({@link pivotOf}); its tools ({@link toolsOf}); and its body,
 * the solid box the stage keeps apart from every other (`extentOf`), for hit tests.
 *
 * Particles: every plume of every actor in drawing order ({@link sparksOf}), the youngest {@link STAGE_CAP} of them
 * ({@link particlesFrom}); ladders by {@link laddersOf}, the lifted copy by {@link liftsOf}; `held` is the actor in
 * the learner's hand.
 *
 * A still stage shows every actor at rest in the look of its state, whole on its perch, with open eyes, centred pupils
 * and the face of its resting mood, without tilt, tools, particles or lift, at rate 0 without a wake tick — and so does
 * an empty one that nobody waits to enter. At rate 0
 * otherwise (species without an idle loop, pupils at rest, between blinks) `wake` is the earliest of the next blink,
 * the end of an activity, the pointer losing its interest, having come to rest for half a second or being allowed to
 * turn an actor again, the tick a lasting state gives way, the next whole second on which a pair may be drawn, the
 * next whole second on which somebody who waits off stage may arrive, the next beat on which moods may travel or the
 * chemistry may act, the next tick on which the mischief of the stage does something (`prankTick`: a prank picked,
 * begun, put back or ended), the press coming due, a puff of dust clearing, and the scenery ({@link sceneryWake}).
 */
export function frameOf(menagerie: Menagerie, stage: Stage): Frame {
  const actors: Actor[] = [];
  const kinds: Species[] = [];
  const streams: number[] = [];
  for (let stream = 0; stream < menagerie.species.length; stream++) {
    const kind = menagerie.species[stream]!;
    for (const actor of stage.actors) {
      if (actor.species !== kind.id) continue;
      actors.push(actor);
      kinds.push(kind);
      streams.push(stream);
      break;
    }
  }
  const tick = stage.tick;
  const still = stage.mode === "still";
  const frames: ActorFrame[] = actors.map((actor, index) => {
    const kind = kinds[index]!;
    const looking = lookingOf(kind, actor, tick);
    const present = lookOf(kind, looking.state);
    const feeling = still ? atRest(kind.mood, tick) : settled(actor.feeling, kind.mood, tick);
    const face = faceOf(feeling);
    const lid = still ? 0 : actor.activity === "sleep" ? 1 : face.lid + (1 - face.lid) * lidAt(tick - actor.blink);
    const squeeze = still ? 1 : squeezeOf(actor, tick);
    const forward = squeeze < 0 ? actor.facing === -1 : actor.facing === 1;
    const across = forward ? actor.gaze.x : 0 - actor.gaze.x;
    const eyes: EyeFrame[] = kind.face.eyes.map((eye) => {
      const span = pupilReach(eye);
      return { x: across * span, y: actor.gaze.y * span, lid };
    });
    const pose = still ? lookedOf(kind, restPose(kind), present, null, 1, 0) : drooped(kind, poseOf(stage, kind, actor, tick, streams[index]!, present, lookOf(kind, looking.former), smoothstep((tick - looking.since) / STATE_BLEND)), face.drop);
    const bones = solveRig(kind, still || stage.quiet ? pose : leant(kind, pose, across, actor.gaze.y));
    if (squeeze !== 1) for (let entry = 0; entry < bones.length; entry += 2) bones[entry] = bones[entry]! * squeeze;
    const tilt = still ? 0 : tiltOf(kind, actor);
    const pivot = pivotOf(kind, actor);
    const origin = placed({ x: actor.x, y: actor.y }, actor.facing, tilt, pivot);
    const extent = extentOf(actor, kind);
    return {
      species: actor.species,
      x: origin.x,
      y: origin.y,
      facing: actor.facing,
      activity: actor.activity,
      opacity: actor.opacity,
      bones,
      eyes,
      footing: actor.footing,
      state: !still && tick - looking.since < STATE_BLEND / 2 ? looking.former : looking.state,
      mood: feeling.mood,
      intensity: feeling.intensity,
      spirits: spiritsOf(feeling),
      tilt,
      pivot,
      tools: still ? NO_TOOLS : toolsOf(kind, actor, tick),
      body: { x: extent.x0, y: extent.y0, width: extent.x1 - extent.x0, height: extent.y1 - extent.y0 },
    };
  });
  const order = frames.map((_, index) => index).sort((a, b) => frames[a]!.y - frames[b]!.y || (frames[a]!.species < frames[b]!.species ? -1 : frames[a]!.species > frames[b]!.species ? 1 : 0));
  const sparks: Spark[] = [];
  if (!still) for (const index of order) sparksOf(stage, kinds[index]!, actors[index]!, streams[index]!, frames[index]!.bones, frames[index]!, frames[index]!.tilt, frames[index]!.pivot, frames[index]!.opacity, sparks);
  let rate: 0 | 16 | 32 | 64 = 0;
  if (!still) {
    for (let index = 0; index < actors.length; index++) {
      const pace = paceOf(stage, actors, kinds, index);
      if (pace > rate) rate = pace;
    }
    if (sceneryPace(stage) > rate) rate = 64;
    if (stage.press.phase !== "idle" && pressDue(stage.press) === null) rate = 64;
  }
  let wake: Ticks | null = null;
  const arrival = arrivalTick(menagerie, { ...stage, actors }, tick + 1);
  if (rate === 0 && !still && (actors.length > 0 || arrival >= 0 || stage.ladders.length > 0 || stage.lift !== null)) {
    let horizon = Infinity;
    for (const actor of actors) {
      horizon = Math.min(horizon, actor.until);
      if (actor.activity !== "sleep") horizon = Math.min(horizon, actor.blink > tick ? actor.blink : actor.blink + BLINK_TICKS);
      if (stage.pointer !== null && tick + 1 < actor.faced + TURN_REST) horizon = Math.min(horizon, actor.faced + TURN_REST);
    }
    if (actors.length > 0 && stage.pointer !== null && tick + 1 <= stage.pointed + PERK_LINGER) horizon = Math.min(horizon, stage.pointed + PERK_LINGER);
    if (actors.length > 0 && stage.pointer !== null && tick + 1 - stage.pointed < POINTER_TICKS) horizon = Math.min(horizon, stage.pointed + POINTER_TICKS);
    for (let index = 0; index < actors.length; index++) {
      const looking = lookingOf(kinds[index]!, actors[index]!, tick);
      const ends = stateEnds(kinds[index]!, looking.state, looking.since);
      if (ends !== null) horizon = Math.min(horizon, ends);
    }
    horizon = Math.min(horizon, sceneryWake(stage, actors, kinds, tick));
    const pairing = pairingTick({ ...stage, actors }, tick + 1);
    if (pairing >= 0) horizon = Math.min(horizon, pairing);
    if (arrival >= 0) horizon = Math.min(horizon, arrival);
    const beating = beatTick(menagerie, { ...stage, actors }, tick + 1);
    if (beating >= 0) horizon = Math.min(horizon, beating);
    const pranking = prankTick(menagerie, { ...stage, actors }, tick + 1);
    if (pranking >= 0) horizon = Math.min(horizon, pranking);
    const due = pressDue(stage.press);
    if (due !== null) horizon = Math.min(horizon, due);
    for (const puff of stage.puffs) horizon = Math.min(horizon, puff.tick + PUFF_TICKS);
    wake = horizon > tick ? horizon : tick + 1;
  }
  return { tick, actors: order.map((index) => frames[index]!), rate, wake, ladders: laddersOf(stage, still), particles: particlesFrom(sparks), lifts: liftsOf(stage, still), puffs: puffsOf(stage, still), held: still ? null : heldOf(actors) };
}
//#endregion 🔖️Frame

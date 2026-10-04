/** 🎪️ The stage: the pure fold that makes pets alive. `advance` folds events into a stage, `frameOf` projects a stage into what a render target draws; the same seed and the same events yield the same frames, bit for bit.
 *
 * NORMATIVE ORDER (the Rust twin follows it step by step). Actors are kept in `menagerie.species` order. One tick:
 * 1. per actor, in that order: fade (out while leaving and not walking, gone at 0; otherwise towards its presence —
 *    whole, or see-through while it flies or climbs in front of a keep-out) → turning round (an actor that does not
 *    face its heading faces it at once and its drawing follows over 8 ticks, up to `faced`; a walker strides only once
 *    it faces its goal squarely) → motion (`walk`: on every beat the end at the goal or before a neighbour, else
 *    `strideTo`; `hop`: `hopStep` + `landingOf`, a floating gait glides straight; `fall`: `fallStep` + `landingOf`; a
 *    sleeper wakes when the pointer is near; an idle one perks up when the pointer has come to rest on it or beside
 *    it) → gaze spring
 *    → blink schedule → mood → the end of its activity when `tick ≥ until`;
 * 2. the stage: a pair whose partners both wait begins its encounter; on every whole second a new pair may be drawn,
 *    and then whoever is wanted but not on stage arrives when a perch has room.
 *
 * DISTANCE. Actors of one surface never stand in each other. Whoever arrives, lands or picks a goal keeps a
 * comfortable gap (8 px between the bodies); a walker stops before a neighbour (6 px); a ride on a perch that shrank
 * sets the actors apart again; and whoever does not fit — a perch holds as many as fit with the comfortable gap — is
 * crowded out: it fades where it stood and, while it is wanted, arrives anew on a perch with room. Newcomers spread
 * out: they arrive on a perch that holds the fewest, on the ground — the lowest perches of the stage — only when no
 * higher one is as empty. And when a survey brings new ground, the scenery has changed: whoever stood on ground that
 * vanished is gone with it and arrives anew (without new ground it falls), and whoever shares a perch moves to one
 * that holds nobody.
 *
 * THE POINTER. Outside a time of concentration a pet that stands idle by itself attends to the pointer while it is
 * worth a look (it moved within the last 4 s): its pupils follow it, the bone that carries its first eye leans
 * after the pupils, it turns round when the pointer is clearly behind it (at most once a second), and when the
 * pointer has come to rest beside it for half a second and it is curious enough it greets it, which costs curiosity.
 * Pupils and lean follow the gaze, which reaches half of its way at 32 px from the eyes: `lookOffset(…, 32)`.
 *
 * DRAWS. One key per draw, `[seed, stream, counter]`, words in the order listed:
 * - actor (`stream` = index of the species, `counter` = `actor.draws`, which starts at the tick of its arrival):
 *   arrival and thaw `[idle dwell, idle clip, first blink]`; coming to rest `[idle dwell, idle clip]`; decision
 *   `randomPick` on word 0, then `[·, dwell, clip, goal or target]`; blink `[gap, double]`; perking up `[dwell, clip]`;
 *   sulk `[dwell, clip]`;
 * - stage (`stream` = `STAGE_STREAM`, `counter` = `stage.draws`): arrival `[perch, place, facing]`; pairing
 *   `randomPick` on word 0, then `[·, chance]`; encounter `[kind, span, clip of the first, clip of the second]`.
 *
 * LAZINESS. Needs are settled when an activity ends (`needsAfter` over its whole span) and rapports when they are
 * touched (`rapportFaded` since `stage.met`, the tick of the last change of any rapport), so neither depends on how
 * time is cut into `ticked` events. Ticks in which nothing can change are jumped over: an actor at rest (no motion,
 * no turn, no fade, gaze and mood on their targets) only has scheduled changes — the end of its activity, the end of
 * its blink, the pointer losing its interest, the next whole second on which a pair may be drawn or somebody who
 * waits off stage may arrive.
 *
 * PARTS. This file is the façade of the stage — `openStage`, `advance` and `frameOf` are all the package exports of
 * it — over ten modules that share the working copy of a stage (the draft): the gaps between actors, the schedule of
 * the stage, attention, locomotion, sociability, choice, population, the clock and the projection. The order above is
 * the order of the clock.
 *
 * @see ../📝️draft/🟦️.ts — the working copy of a stage, its lookups and the smallest changes of an actor
 * @see ../📏️spacing/🟦️.ts — the gaps between grounded actors
 * @see ../🗓️schedule/🟦️.ts — the whole seconds on which a pair may be drawn or somebody may arrive
 * @see ../👀️attention/🟦️.ts — eyes, head and facing: presence, turning round, gaze, blinks, mood, perking up
 * @see ../🚶️locomotion/🟦️.ts — walking, hopping, gliding, falling, landing, leaving
 * @see ../💞️sociability/🟦️.ts — pairing, encounters, sulks, rapport
 * @see ../🎯️choice/🟦️.ts — what an actor does next when its time is up
 * @see ../👥️population/🟦️.ts — surveys, arrivals, spreading out, summons, liveliness
 * @see ../🕰️clock/🟦️.ts — one tick in the normative order, the lulls, time passing
 * @see ../🎥️projection/🟦️.ts — poses, the rate and `frameOf`
 * @see ../🧠️behavior/🟦️.ts — limits, weights, dwells, encounters, needs, rapport
 * @see ../🏞️terrain/🟦️.ts — perches, strides, falls, hops
 * @see ../🎞️animation/🟦️.ts — clips, springs, blinks
 * @see ../🦴️rig/🟦️.ts — poses and matrices
 * @see ../🎲️randomness/🟦️.ts — counter-based draws
 * @see ../../🧬️schema/🟦️.ts — `Stage`, `Actor`, `StageEvent`, `Frame`
 * @see ./🦀️.rs — the Rust twin
 */

import { type Menagerie, type Stage, type StageEvent } from "../../🧬️schema/🟦️.ts";
import { grasp, tossed, touchedAt, unhovered } from "../👀️attention/🟦️.ts";
import { IDLE, noShaking, pressStep } from "../👆️gesture/🟦️.ts";
import { summon, survey, tune } from "../👥️population/🟦️.ts";
import { clicked, played } from "../💞️sociability/🟦️.ts";
import { draftOf, sealed, type Draft } from "../📝️draft/🟦️.ts";
import { pass } from "../🕰️clock/🟦️.ts";
import { reclaimed } from "../🎯️choice/🟦️.ts";
import { unlifted } from "../🚶️locomotion/🟦️.ts";

export { frameOf } from "../🎥️projection/🟦️.ts";

//#region 🔖️Stage
/** 🏟️ An empty stage for a seed: tick 0, mode `calm`, not quiet, nothing surveyed, nobody summoned, nothing permitted (the shell's `permitted` grants play and mischief), no press, no ladder, no lift, no trip. */
export function openStage(seed: number): Stage {
  return {
    seed: seed >>> 0,
    tick: 0,
    mode: "calm",
    quiet: false,
    width: 0,
    height: 0,
    pointer: null,
    pointed: 0,
    over: "free",
    glances: [],
    surfaces: [],
    keepouts: [],
    walls: [],
    fixtures: [],
    perches: [],
    pitches: [],
    wanted: [],
    actors: [],
    rapports: [],
    met: 0,
    draws: 0,
    play: false,
    mischief: false,
    stirred: 0,
    scrolled: 0,
    press: IDLE,
    touched: null,
    shaking: noShaking(0),
    trail: [],
    coolings: [],
    pledges: [],
    ladders: [],
    lift: null,
    rested: 0,
    poofs: 0,
    puffs: [],
    claims: [],
    courses: [],
    trips: [],
    origin: null,
  };
}

/** 📨️ One event folded into the draft. What the learner permits, what the pointer is over and the ticks of the learner's last input and of the last scroll are kept; a pointer that leaves the stage takes every gesture with it; a deed is the pet's to answer (`played`). A press, a drag, a release and a cancellation step the press of the stage (a drag and a release also move the pointer): a release that the press machine calls a click is the click of the actor the press began on (`clicked`); a pick-up, a drop and an abort are the hand's (`grasp`), and so is a toss (`tossed`). Play that is no longer permitted lets go of the press, and a pet in the hand is given back; mischief that is no longer permitted ends the prank quietly (`unlifted`). A reclaimed fixture whose copy is out ends its lift at once and throws its pusher off (`reclaimed`). */
function apply(menagerie: Menagerie, draft: Draft, event: StageEvent): void {
  if (event.kind === "ticked") pass(menagerie, draft, event.ticks);
  else if (event.kind === "pointed") {
    draft.pointer = { x: event.x, y: event.y };
    draft.pointed = draft.tick;
    draft.over = event.over;
  } else if (event.kind === "unpointed") {
    draft.pointer = null;
    unhovered(draft, draft.tick);
  } else if (event.kind === "pressed") {
    const step = pressStep(draft.press, event, draft.tick, { control: false, scrolled: false, quiet: draft.quiet, still: draft.mode === "still" });
    grasp(draft, step.signal, draft.touched, draft.tick);
    draft.press = step.state;
    draft.touched = draft.press.phase === "idle" ? null : touchedAt(draft, event.x, event.y);
  } else if (event.kind === "dragged" || event.kind === "released" || event.kind === "cancelled") {
    if (event.kind !== "cancelled") {
      draft.pointer = { x: event.x, y: event.y };
      draft.pointed = draft.tick;
    }
    const step = pressStep(draft.press, event, draft.tick, { control: false, scrolled: false, quiet: draft.quiet, still: draft.mode === "still" });
    draft.press = step.state;
    const touched = draft.touched;
    grasp(draft, step.signal, touched, draft.tick);
    if (step.state.phase === "idle") draft.touched = null;
    if (step.signal === "click" && touched !== null && event.kind === "released") clicked(draft, touched, event.x);
  } else if (event.kind === "played") {
    if (event.deed === "toss") tossed(draft, event.species, draft.tick);
    else played(draft, event.species, event.deed);
  }
  else if (event.kind === "glanced") draft.glances = event.points;
  else if (event.kind === "surveyed") survey(menagerie, draft, event);
  else if (event.kind === "summoned") summon(menagerie, draft, event.species);
  else if (event.kind === "tuned") tune(menagerie, draft, event.mode);
  else if (event.kind === "hushed") draft.quiet = event.quiet;
  else if (event.kind === "permitted") {
    draft.play = event.play;
    draft.mischief = event.mischief;
    if (!event.mischief) unlifted(draft, draft.tick);
    if (!event.play && draft.press.phase !== "idle") {
      grasp(draft, "abort", draft.touched, draft.tick);
      draft.press = IDLE;
      draft.touched = null;
    }
  } else if (event.kind === "stirred") draft.stirred = draft.tick;
  else if (event.kind === "scrolled") draft.scrolled = draft.tick;
  else if (event.kind === "reclaimed") reclaimed(draft, event.fixture, draft.tick);
}

/** 🧵️ The stage after the events, folded in order; the stage passed in is never changed. */
export function advance(menagerie: Menagerie, stage: Stage, events: readonly StageEvent[]): Stage {
  if (events.length === 0) return stage;
  const draft = draftOf(menagerie, stage);
  for (const event of events) apply(menagerie, draft, event);
  return sealed(draft);
}
//#endregion 🔖️Stage

/** 🕰️ The clock of the stage: one tick of one actor and one tick of the stage in the normative order (`act`, `step`), the lull in which nothing changes but the tick (`lull`), and time passing over both (`pass`).
 *
 * Whatever moves, fades, turns or is scheduled must be known in three places, or it is drawn at the wrong rate or jumped over: `lull` here, and `paceOf` and the `wake` of `frameOf` in the projection.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ./🦀️.rs — the Rust twin
 */

import { type Menagerie, type Ticks } from "../../🧬️schema/🟦️.ts";
import { BLINK_TICKS } from "../🎞️animation/🟦️.ts";
import { beat, conclude, mischief } from "../🎯️choice/🟦️.ts";
import { PERK_LINGER, POINTER_TICKS, TURN_REST, gazeRests, handle, headingOf, hovers, look, perk, presenceOf, swivel, watched, wink } from "../👀️attention/🟦️.ts";
import { pruned } from "../🚧️clearance/🟦️.ts";
import { hoverBusy, pressDue } from "../👆️gesture/🟦️.ts";
import { calmsAt, stateEnds } from "../💗️feeling/🟦️.ts";
import { spawn } from "../👥️population/🟦️.ts";
import { meet, pair } from "../💞️sociability/🟦️.ts";
import { feel, remove, settle, stand, type Draft } from "../📝️draft/🟦️.ts";
import { arrivalTick, beatTick, pairingTick, prankTick, sampling } from "../🗓️schedule/🟦️.ts";
import { PUFF_TICKS, cling, fly, scoot, slip, stride, travel, tripOf } from "../🚶️locomotion/🟦️.ts";

//#region 🔖️Constants
const FADE_STEP = 0.0625;
const SHY_STEP = 0.03125;
//#endregion 🔖️Constants

//#region 🔖️Time
/** 🎬️ One tick of one actor, in the normative order; `false` when the actor is gone. */
function act(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  stand(draft, index, now);
  if (body.leaving) {
    if (body.activity !== "walk") {
      const opacity = body.opacity - FADE_STEP;
      if (opacity <= 0) {
        remove(draft, index);
        return false;
      }
      body.opacity = opacity;
    }
  } else {
    const presence = presenceOf(draft.keepouts, body, draft.kinds[index]!);
    if (body.opacity < presence) body.opacity = body.opacity + FADE_STEP < presence ? body.opacity + FADE_STEP : presence;
    else if (body.opacity > presence) body.opacity = body.opacity - SHY_STEP > presence ? body.opacity - SHY_STEP : presence;
  }
  const turns = swivel(draft, index, now);
  if (tripOf(draft.trips, body.species) !== null) {
    if (!travel(draft, index, now)) return false;
  } else if (body.footing === "air" || body.footing === "chute" || body.footing === "rope") {
    if (!fly(draft, index, now)) return false;
  } else if (body.footing === "head") {
    if (!slip(draft, index, now)) return false;
  } else if (body.footing === "wall" || body.footing === "ladder") {
    if (!cling(draft, index, now)) return false;
  } else if (body.activity === "walk") {
    if (!turns) stride(draft, index, now);
  } else if (body.activity === "sleep") {
    if (watched(draft.pointer, body, draft.kinds[index]!)) {
      feel(draft, index, "woken", now);
      settle(draft, index, now);
    }
  } else if (body.activity === "idle") perk(draft, index, now);
  look(draft, index, now);
  wink(draft, index, now);
  if (!body.leaving && now >= body.until) conclude(draft, index, now);
  return true;
}

/** 💤️ How many of the next `left` ticks change nothing but the tick: 0 while any actor moves (a trip with gear included), turns or is about to, fades, has pupils off their target or a gesture of the pointer under way round it, and on the tick after the pointer moved or the page scrolled while gestures are followed — an actor that rests on a wall or a ladder rests like one on a perch —; else the ticks before the earliest scheduled change — among them the tick a standing ladder is taken away, while a pointer is on stage, the tick an actor may turn to it again and the tick it has come to rest for half a second, the tick a lasting state gives way, the tick a mood stops showing (the manners of the gaze change with it), the next beat on which moods travel or the chemistry is looked at, and the next tick on which the mischief of the stage does something (a prank picked, begun, put back or ended). */
function lull(menagerie: Menagerie, draft: Draft, left: Ticks): Ticks {
  const next = draft.tick + 1;
  let horizon = next + left;
  if (sampling(draft, next) || draft.trips.length > 0) return 0;
  const due = pressDue(draft.press);
  if (draft.press.phase !== "idle" && due === null) return 0;
  if (due !== null) horizon = Math.min(horizon, due);
  for (const puff of draft.puffs) horizon = Math.min(horizon, puff.tick + PUFF_TICKS);
  for (const ladder of draft.ladders) if (ladder.until >= next) horizon = Math.min(horizon, ladder.until);
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    const kind = draft.kinds[index]!;
    const activity = body.activity;
    if ((body.footing !== "perch" && body.footing !== "wall" && body.footing !== "ladder") || body.leaving || body.opacity !== presenceOf(draft.keepouts, body, kind) || activity === "walk" || activity === "hop" || activity === "fall" || activity === "scoot") return 0;
    if (next <= body.faced || hoverBusy(body.hover) || !gazeRests(draft, draft.actors, draft.kinds, index, next)) return 0;
    const heading = headingOf(draft, draft.actors, draft.kinds, index, next);
    if (heading !== 0 && heading !== body.facing) return 0;
    if (activity === "sleep") {
      if (watched(draft.pointer, body, kind)) return 0;
    } else horizon = Math.min(horizon, body.blink + BLINK_TICKS);
    horizon = Math.min(horizon, body.until);
    if (draft.pointer !== null && next < body.faced + TURN_REST) horizon = Math.min(horizon, body.faced + TURN_REST);
    const ends = stateEnds(kind, body.state, body.stateSince);
    if (ends !== null) horizon = Math.min(horizon, ends);
    const calm = calmsAt(body.feeling, kind.mood);
    if (calm >= next) horizon = Math.min(horizon, calm);
  }
  if (draft.pointer !== null && next <= draft.pointed + PERK_LINGER) horizon = Math.min(horizon, draft.pointed + PERK_LINGER);
  if (draft.pointer !== null && next - draft.pointed < POINTER_TICKS) horizon = Math.min(horizon, draft.pointed + POINTER_TICKS);
  const pairing = pairingTick(draft, next);
  if (pairing >= 0) horizon = Math.min(horizon, pairing);
  const arrival = arrivalTick(menagerie, draft, next);
  if (arrival >= 0) horizon = Math.min(horizon, arrival);
  const beating = beatTick(menagerie, draft, next);
  if (beating >= 0) horizon = Math.min(horizon, beating);
  const pranking = prankTick(menagerie, draft, next);
  if (pranking >= 0) horizon = Math.min(horizon, pranking);
  const skip = horizon - next;
  return skip > 0 ? Math.min(skip, left) : 0;
}

/** 🥁️ One tick of the stage, in the normative order: the corridors that are over are dropped, the dust that has settled and the ladders whose time is up and that nobody climbs; the hand (the press sees the tick pass, a held or tossed pet follows it); the scooters of every surface move to their seats together; the actors; the gestures of the pointer round them, the beat (moods travelling, chemistry), encounters, pairing, mischief with the page, arrivals. */
function step(menagerie: Menagerie, draft: Draft): void {
  const now = draft.tick + 1;
  draft.tick = now;
  if (draft.claims.length > 0) draft.claims = pruned(draft.claims, now);
  if (draft.puffs.some((puff) => puff.tick + PUFF_TICKS <= now)) draft.puffs = draft.puffs.filter((puff) => puff.tick + PUFF_TICKS > now);
  if (draft.ladders.some((ladder) => ladder.until <= now && ladder.rider === null)) draft.ladders = draft.ladders.filter((ladder) => ladder.until > now || ladder.rider !== null);
  handle(draft, now);
  scoot(draft, now);
  for (let index = 0; index < draft.actors.length; ) if (act(draft, index, now)) index++;
  hovers(draft, now);
  beat(menagerie, draft, now);
  meet(menagerie, draft, now);
  pair(menagerie, draft, now);
  mischief(menagerie, draft, now);
  if (arrivalTick(menagerie, draft, now) === now) spawn(menagerie, draft);
}

/** ⏭️ Time passes: tick by tick while something can change, in one jump over every lull, over a still stage and over an empty one that nobody waits to enter and whose last prank is over. */
export function pass(menagerie: Menagerie, draft: Draft, ticks: Ticks): void {
  let left = ticks > 0 ? Math.floor(ticks) : 0;
  while (left > 0) {
    if (draft.mode === "still" || (draft.actors.length === 0 && draft.lift === null && arrivalTick(menagerie, draft, draft.tick + 1) < 0)) {
      draft.tick = draft.tick + left;
      return;
    }
    const skip = lull(menagerie, draft, left);
    if (skip > 0) {
      draft.tick = draft.tick + skip;
      left = left - skip;
      continue;
    }
    step(menagerie, draft);
    left = left - 1;
  }
}
//#endregion 🔖️Time

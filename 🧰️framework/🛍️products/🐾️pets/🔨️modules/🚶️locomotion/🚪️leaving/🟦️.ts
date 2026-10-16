/** 🚪️ Locomotion, leaving: a pet that leaves the stage after halting wherever it is.
 *
 * @see ../🧳️trips/🟦️.ts — the halt that comes first
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { type Ticks } from "../../../🧬️schema/🟦️.ts";
import { perchAt } from "../../🏞️terrain/🟦️.ts";
import { COMFORT_GAP, clearway } from "../../📏️spacing/🟦️.ts";
import { clipAt, release, shift, type Draft } from "../../📝️draft/🟦️.ts";
import { LEAVE_REACH } from "../📐️constants/🟦️.ts";
import { paced, stroll } from "../🛫️departure/🟦️.ts";
import { halt } from "../🧳️trips/🟦️.ts";

//#region 🔖️Leaving
/** 👋️ An actor starts to leave: it lets go of its partner and walks to the nearer end of its perch when that is within three of its widths and nobody stands in the way, then fades; farther away, hemmed in or off its perch, it fades where it is — on a trip with its gear it goes on with it while it fades, unless it is still on its perch: then it gives the trip up ({@link halt}). */
export function leave(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  release(draft, index, now);
  body.leaving = true;
  if (body.footing !== "perch" || body.perch === null) return;
  halt(draft, index, now);
  const perch = perchAt(draft.perches, body.perch, body.x);
  const half = kind.size.width / 2;
  const low = perch === null ? body.x : perch.x0 + half;
  const high = perch === null ? body.x : perch.x1 - half;
  const end = body.x - low <= high - body.x ? low : high;
  const [clearLow, clearHigh] = clearway(draft, index, COMFORT_GAP, -1);
  const clip = clipAt(kind, "walk", 0);
  const goal = paced(kind, clip, body.x, end);
  if (goal !== body.x && end >= clearLow && end <= clearHigh && Math.abs(end - body.x) <= LEAVE_REACH * kind.size.width) {
    stroll(draft, index, goal, clip, now);
    return;
  }
  shift(draft, index, "idle", now);
  body.clip = clipAt(kind, "idle", 0);
  body.goal = body.x;
}
//#endregion 🔖️Leaving

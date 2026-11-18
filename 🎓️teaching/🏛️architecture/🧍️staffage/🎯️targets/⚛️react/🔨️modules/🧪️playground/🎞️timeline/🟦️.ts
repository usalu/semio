/** 🎞️ The shared timeline of the playgrounds: pure rules that say, for one timeline instant, where every placement is in its clip.
 *
 * A placement plays inside a window of the timeline: it begins at `startS`, lasts `durationS` (`null`: to the end of the timeline) and runs its clip `speed` times as fast, starting at the clip time `offsetS`. Before its window it shows the first frame of its window (the clip at `offsetS`),
 * after it the last; with `loop` off the clip also holds its first and last frame instead of repeating; a `holdS` pauses the placement at one clip time whatever the timeline does. A pose has no time. The three playgrounds read these rules and nothing else, so one instant gives one state in every view.
 *
 * @see ../../💾️persistence/🟦️.ts — {@link Track}, the values these rules read
 * @see ../../📍️placed/🟦️.tsx — the 2D figure that draws a reading
 * @see ../🧊️model/🎞️retime/🟦️.ts — the same rules baked into the animation of the 3D scene
 */
import type { Asset, Clip } from "@teaching/staffage";
import type { Placement, Selection, Track } from "../../💾️persistence/🟦️.ts";

/** 🎬️ Where a placement is relative to its window. */
export type TrackPhase = "static" | "waiting" | "running" | "finished" | "held";

/** 📖️ One reading of a placement: the clip time that is drawn (`clipTimeS`, not wrapped, so analytic views keep their absolute time), the same inside one cycle (`cycleS`), and where the placement is. */
export type TrackReading = { readonly clipTimeS: number; readonly cycleS: number; readonly phase: TrackPhase };

/** 🧾️ What a reading needs of a placement. */
export type TrackInput = Track & { readonly offsetS: number };

/** 🎯️ The clip a selection plays: the chosen clip, the asset's default clip when nothing is chosen, nothing for a pose; also nothing for a clip the asset (in its current revision) no longer has. */
export function clipOf(asset: Asset, pick: Selection | null): Clip | undefined {
  if (pick?.kind === "pose") return undefined;
  const wanted = pick?.kind === "clip" ? pick.clip : asset.presentation.defaultClip;
  return asset.clips.find((clip) => clip.id === wanted);
}

/** ❓️ Whether a selection names a pose or clip the asset no longer has. */
export const isMissing = (asset: Asset, pick: Selection | null): boolean => (pick?.kind === "pose" ? !asset.poses.some((pose) => pose.id === pick.pose) : pick?.kind === "clip" && !asset.clips.some((clip) => clip.id === pick.clip));

/** 🎞️ The selection that is really played: the given one, else (a pose or clip that is gone) the default. */
export const playable = (asset: Asset, pick: Selection | null): Selection | null => (isMissing(asset, pick) ? null : pick);

/** ⏱️ The duration of the clip a selection plays in seconds; 0 for a pose. */
export const clipDurationS = (asset: Asset, pick: Selection | null): number => clipOf(asset, playable(asset, pick))?.durationS ?? 0;

/** 📏️ The seconds of the timeline one pass of the clip takes in a window (the clip's duration over the speed), 0 for a pose. */
export const passS = (durationS: number, track: Pick<Track, "speed">): number => (durationS <= 0 ? 0 : durationS / track.speed);

/** 🪟️ Where the window of a placement ends on the timeline (`timelineS` when it runs to the end). */
export const windowEndS = (track: Pick<Track, "startS" | "durationS">, timelineS: number): number => (track.durationS === null ? Math.max(timelineS, track.startS) : track.startS + track.durationS);

/** 📐️ The length of the timeline: the reader's value, else the longest window (one pass of the clip for a window that runs to the end), at least one second. */
export function timelineLengthS(placements: readonly (Track & { readonly clipS: number })[], chosen: number | null): number {
  if (chosen !== null) return Math.max(chosen, 1);
  return Math.max(1, ...placements.map((placement) => placement.startS + (placement.durationS ?? passS(placement.clipS, placement))));
}

const wrap = (value: number, period: number): number => (value >= 0 && value < period ? value : ((value % period) + period) % period);

/** 📖️ Reads a placement at the timeline instant `timeS`; `clipS` is the duration of its clip (0 for a pose). */
export function readTrack(input: TrackInput, clipS: number, timeS: number, timelineS: number): TrackReading {
  if (clipS <= 0) return { clipTimeS: 0, cycleS: 0, phase: "static" };
  if (input.holdS !== null) {
    const held = input.loop ? input.holdS : Math.min(input.holdS, clipS);
    return { clipTimeS: held, cycleS: wrap(held, clipS), phase: "held" };
  }
  const end = windowEndS(input, timelineS);
  const phase: TrackPhase = timeS < input.startS ? "waiting" : timeS > end ? "finished" : "running";
  const local = (Math.min(Math.max(timeS, input.startS), end) - input.startS) * input.speed;
  const raw = input.offsetS + local;
  const clipTimeS = input.loop ? Math.max(0, raw) : Math.min(Math.max(raw, 0), clipS);
  return { clipTimeS, cycleS: input.loop ? wrap(clipTimeS, clipS) : clipTimeS, phase };
}

/** 🧮️ The reading of a placement of an asset at a timeline instant. */
export const readPlacement = (asset: Asset, placement: Placement, timeS: number, timelineS: number): TrackReading => readTrack(placement, clipDurationS(asset, placement.pick), timeS, timelineS);

/** ⏸️ The track of a placement paused where it is now (`holdS` = its clip time), or playing again. */
export function paused(reading: TrackReading, pause: boolean): Pick<Track, "holdS"> {
  return { holdS: pause ? reading.cycleS : null };
}

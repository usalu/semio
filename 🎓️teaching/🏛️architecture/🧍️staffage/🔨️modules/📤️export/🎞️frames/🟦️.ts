/** 🎞️ Deterministic frame schedule of a clip and the PNG-sequence ZIP with its manifest.
 *
 * Frame `n` is sampled at `t = n / fps`; a looping clip has `round(duration · fps)` frames (the last instant equals the first),
 * a non-looping clip `round(duration · fps) + 1` (both end points). The ZIP uses stored members through the first-party writer of the
 * downloads module (PNG is already compressed), so every frame file is byte-identical to the rendered PNG.
 * <https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT> */
import type { JobControl, View } from "../../🔗️contracts/🟦️.ts";
import type { Slug } from "../../../🧬️schema/🟦️.ts";
import { writeZip, type ZipEntry } from "../../../🎯️targets/⚛️react/🔨️modules/📤️downloads/🗜️zip/🟦️.ts";

/** 📅️ The times at which a clip is sampled. */
export type FrameSchedule = { readonly fps: number; readonly durationS: number; readonly loop: boolean; readonly count: number; readonly times: readonly number[] };

/** 📅️ The deterministic schedule of a clip of `durationS` seconds at an integer `fps`. */
export function frameSchedule(clip: { readonly durationS: number; readonly loop: boolean }, fps: number): FrameSchedule {
  if (!Number.isInteger(fps) || fps < 1 || fps > 120) throw new RangeError(`fps must be an integer in 1..120, got ${fps}`);
  if (!(clip.durationS > 0) || !Number.isFinite(clip.durationS)) throw new RangeError(`clip duration must be positive, got ${clip.durationS}`);
  const count = Math.round(clip.durationS * fps) + (clip.loop ? 0 : 1);
  return { fps, durationS: clip.durationS, loop: clip.loop, count, times: Array.from({ length: count }, (_, frame) => frame / fps) };
}

/** 📛️ The file name of frame `index`. */
export const frameName = (index: number): string => `frame-${String(index).padStart(6, "0")}.png`;

/** 🧾️ What the manifest of a PNG sequence states. */
export type FrameManifestInput = { readonly asset: Slug; readonly clip: Slug; readonly view: View; readonly scaleDenominator: number; readonly widthPx: number; readonly heightPx: number; readonly transparent: boolean };

/** 🧾️ The manifest written next to the frames. */
export function frameManifest(schedule: FrameSchedule, input: FrameManifestInput): Readonly<Record<string, unknown>> {
  return {
    schema: "semio.staffage.frames/v1",
    asset: input.asset,
    clip: input.clip,
    view: input.view,
    scale: { denominator: input.scaleDenominator, text: `1:${input.scaleDenominator}` },
    fps: schedule.fps,
    durationS: schedule.durationS,
    loop: schedule.loop,
    frameCount: schedule.count,
    size: { widthPx: input.widthPx, heightPx: input.heightPx },
    alpha: input.transparent,
    frames: schedule.times.map((timeS, index) => ({ index, timeS, file: frameName(index) })),
  };
}

/** 🗜️ Packs rendered PNG frames and the manifest into a ZIP (manifest first); honours `control` between frames. */
export async function writePngSequenceZip(schedule: FrameSchedule, input: FrameManifestInput, frames: AsyncIterable<Uint8Array>, control?: JobControl): Promise<Uint8Array> {
  const entries: ZipEntry[] = [{ name: "manifest.json", bytes: new TextEncoder().encode(JSON.stringify(frameManifest(schedule, input), null, 2) + "\n") }];
  let index = 0;
  for await (const png of frames) {
    control?.signal?.throwIfAborted();
    entries.push({ name: frameName(index), bytes: png });
    index += 1;
    control?.progress?.(index, schedule.count, { en: "Packing PNG frames", de: "PNG-Bilder werden gepackt" });
  }
  if (index !== schedule.count) throw new Error(`PNG sequence has ${index} frames, the schedule needs ${schedule.count}`);
  return writeZip(entries);
}

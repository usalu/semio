/** 🧊️ Cast levels of detail: which frames of a cast are sampled and which shapes, patterns and transport particles are drawn, resolved from data before any asset is loaded or drawn.
 *
 * A level never changes units, origin, anchors, scenario or the clock of a cast. It samples the same analytical state at canonical times (a thermal history is a pure function of asset, query and environment, so a skipped frame costs nothing and a sampled frame equals the full-detail frame at that time) and draws a subset of the same authored shapes.
 * Non-looping samples include both endpoints, looping samples wrap in whole periods, so `playbackFrame` keeps mapping canonical time to a frame.
 * @see ./🔣️.json — the levels, normative
 * @see ./🔣️.schema.json — their schema
 * @see ../🟦️.ts — the exporter */
import type { ShapeRole } from "../../../../🧬️schema/🟦️.ts";
import { canonicalJson, sha256Hex } from "../../../🏷️revision/🟦️.ts";
import type { FrameSchedule } from "../../🎞️frames/🟦️.ts";
import levels from "./🔣️.json" with { type: "json" };

/** 🆔️ The level ids a render profile can ask for. */
export const LOD_IDS = ["full", "preview", "poster"] as const;

/** 🧊️ One of {@link LOD_IDS}. */
export type LodId = (typeof LOD_IDS)[number];

/** 🧊️ One level of the levels file. */
export type LodLevel = {
  readonly title: { readonly en: string; readonly de: string };
  readonly frames: { readonly outputFpsFraction: number | null; readonly maxFrames: number; readonly phase: number };
  readonly roles: readonly ShapeRole[];
  readonly patterns: boolean;
  readonly transportDensity: number;
  readonly strokeMin: number;
};

/** 🧊️ A level with the output frame rate of the job applied: the exact parameters a cached cast was drawn with. `revision` is eight hex digits of their canonical-JSON hash. */
export type ResolvedLod = { readonly id: LodId; readonly level: LodLevel; readonly fpsCap: number | null; readonly revision: string };

const LEVELS = levels.levels as Readonly<Record<LodId, LodLevel>>;

/** 🔎️ Whether `value` is a level id. */
export const isLodId = (value: unknown): value is LodId => (LOD_IDS as readonly unknown[]).includes(value);

/** 🧊️ Resolves a level id for a job that outputs `outputFps` frames per second; throws an actionable error for an unknown level or a non-positive rate. */
export function resolveLod(id: string, outputFps: number): ResolvedLod {
  if (!isLodId(id)) throw new Error(`unknown level of detail ${JSON.stringify(id)}; levels: ${LOD_IDS.join(", ")}`);
  if (!(Number.isFinite(outputFps) && outputFps > 0 && outputFps <= 240)) throw new RangeError(`the output frame rate must be a number in (0, 240], got ${outputFps}`);
  const level = LEVELS[id];
  const fpsCap = level.frames.outputFpsFraction === null ? null : level.frames.outputFpsFraction * outputFps;
  const revision = sha256Hex(new TextEncoder().encode(canonicalJson({ id, level, fpsCap }))).slice(0, 8);
  return { id, level, fpsCap, revision };
}

/** 📅️ The samples a level takes of the canonical schedule of a cast: the schedule itself when the level keeps every frame, otherwise at most `maxFrames` samples at `fpsCap` frames per second of canonical time; a single sample sits at `phase` of the duration. The index `fps` is the effective rate, so a sample `k` plays at `k / fps`. */
export function lodSchedule(canonical: FrameSchedule, lod: ResolvedLod): FrameSchedule {
  const { maxFrames, phase } = lod.level.frames;
  const { count, durationS, loop } = canonical;
  const rate = lod.fpsCap === null ? canonical.fps : Math.min(canonical.fps, lod.fpsCap);
  const wanted = Math.round(durationS * rate) + (loop ? 0 : 1);
  const kept = Math.min(count, wanted, maxFrames);
  const samples = count >= 2 && maxFrames >= 2 ? Math.max(2, kept) : kept;
  if (samples >= count) return canonical;
  if (samples === 1) return { fps: canonical.fps, durationS, loop, count: 1, times: [phase * durationS] };
  const intervals = loop ? samples : samples - 1;
  return { fps: intervals / durationS, durationS, loop, count: samples, times: Array.from({ length: samples }, (_, frame) => (frame * durationS) / intervals) };
}

/** ⏱️ Progress and cancellation of the model exports: the staffage `JobControl` (signal, bilingual progress) adapted to the kernel `Operation` (fraction, cancelled), split into weighted phases so one bar runs from 0 to 1 over modelling, tessellation, edges and encoding, and the tessellation quality of an asset.
 *
 * Cancellation throws the signal's reason (an `AbortError` when the signal carries none), as every other staffage export does; the kernel's own cancellation errors are translated.
 *
 * @see ../../🔗️contracts/🟦️.ts — `JobControl`
 */
import type { Asset, Text } from "../../../🧬️schema/🟦️.ts";
import { CancelledError, type Operation, type Quality } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { JobControl } from "../../🔗️contracts/🟦️.ts";

/** 🧩️ One weighted phase of a job. */
export type Phase = { readonly label: Text; readonly weight: number };

/** ⏳️ A job split into phases: `operation(i)` is the kernel operation of phase `i`; progress is reported as permille of the whole. */
export type Job = { readonly operation: (phase: number) => Operation; readonly finish: () => void };

const PERMILLE = 1000;

/** 📤️ The {@link Job} of a control over weighted phases. */
export function jobOf(control: JobControl, phases: readonly Phase[]): Job {
  const total = phases.reduce((sum, p) => sum + p.weight, 0);
  const before = (index: number): number => phases.slice(0, index).reduce((sum, p) => sum + p.weight, 0) / total;
  return {
    operation: (index) => ({
      report: (fraction) => control.progress?.(Math.round((before(index) + (Math.max(0, Math.min(1, fraction)) * phases[index]!.weight) / total) * PERMILLE), PERMILLE, phases[index]!.label),
      cancelled: () => control.signal?.aborted === true,
    }),
    finish: () => control.progress?.(PERMILLE, PERMILLE, phases[phases.length - 1]!.label),
  };
}

/** 🛑️ Runs `work` and turns the kernel's cancellation errors into the signal's reason. */
export async function cancellable<T>(control: JobControl, work: () => Promise<T>): Promise<T> {
  try {
    return await work();
  } catch (error) {
    if (error instanceof CancelledError || (error instanceof Error && error.name === "FbxCancelledError")) throw control.signal?.reason ?? new DOMException("This operation was aborted", "AbortError");
    throw error;
  }
}

/** 📏️ The roundness rule of the 3D models: the smallest feature of a scale is `paperFeatureM` of paper (0.15 mm: 7.5, 15, 30 and 75 mm at 1:50, 1:100, 1:200 and 1:500); the chord deviation is at most `chordPerFeature` of that feature and at most 1/`largestPerChord` of the largest dimension of the asset (a quarter of a pixel when the asset fills 800 pixels); a full circle whose radius exceeds one feature has at least `roundSegments` segments and no circle fewer than `minSegments`; the validators measure at a chord of 1/`referencePerChord` of the largest dimension, clamped to `referenceChordM`. */
export const ROUNDNESS = { paperFeatureM: 0.00015, chordPerFeature: 0.02, largestPerChord: 3200, roundSegments: 24, minSegments: 12, referencePerChord: 800, referenceChordM: [0.0005, 0.02] } as const;

/** 🧢️ The most segments of one full circle at a scale: 96 up to 1:20, 64 up to 1:50, 48 up to 1:100, 40 up to 1:200 and 32 beyond, so that a coarser scale never has more triangles than a finer one and the triangle budgets hold. */
export const maxSegmentsAt = (scale: number): number => (scale <= 20 ? 96 : scale <= 50 ? 64 : scale <= 100 ? 48 : scale <= 200 ? 40 : 32);

/** 🎚️ The tessellation quality of an asset at a scale, the one rule of the viewer, every download and the budget oracle ({@link ROUNDNESS}): adaptive to curvature and size, the chord deviation is the smaller of 2 % of the smallest feature of the scale and 1/3200 of the largest dimension, a circle larger than one feature never has fewer than 24 segments, and the segments are capped by {@link maxSegmentsAt}. Without a scale it is the reference of the validators that measure silhouettes, areas and volumes: a chord of 1/800 of the largest dimension (0.5 to 20 mm) with 24 to 64 segments on every circle. */
export function qualityOf(asset: Asset, scale?: number): Quality {
  const largest = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
  if (scale === undefined) return { chordM: Math.min(ROUNDNESS.referenceChordM[1], Math.max(ROUNDNESS.referenceChordM[0], largest / ROUNDNESS.referencePerChord)), minSegments: ROUNDNESS.roundSegments, maxSegments: 64 };
  const featureM = ROUNDNESS.paperFeatureM * scale;
  return { chordM: Math.min(largest / ROUNDNESS.largestPerChord, ROUNDNESS.chordPerFeature * featureM), minSegments: ROUNDNESS.minSegments, maxSegments: maxSegmentsAt(scale), round: { radiusM: featureM, segments: ROUNDNESS.roundSegments } };
}

/** 🏷️ The phases every export shares. */
export const PHASES = {
  model: { en: "Building the model", de: "Modell wird aufgebaut" },
  tessellate: { en: "Tessellating solids", de: "Körper werden vernetzt" },
  edges: { en: "Finding feature edges", de: "Kanten werden gesucht" },
  encode: { en: "Encoding", de: "Kodiere" },
} as const satisfies Record<string, Text>;

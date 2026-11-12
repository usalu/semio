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

/** 🎚️ The tessellation quality of an asset: a chord deviation of 1/800 of its largest dimension between 0.5 and 20 mm, 12 to 64 segments per circle. */
export function qualityOf(asset: Asset): Quality {
  const largest = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
  return { chordM: Math.min(0.02, Math.max(0.0005, largest / 800)), minSegments: 12, maxSegments: 64 };
}

/** 🏷️ The phases every export shares. */
export const PHASES = {
  model: { en: "Building the model", de: "Modell wird aufgebaut" },
  tessellate: { en: "Tessellating solids", de: "Körper werden vernetzt" },
  edges: { en: "Finding feature edges", de: "Kanten werden gesucht" },
  encode: { en: "Encoding", de: "Kodiere" },
} as const satisfies Record<string, Text>;

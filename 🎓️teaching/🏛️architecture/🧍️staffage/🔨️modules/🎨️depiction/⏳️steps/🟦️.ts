/** ⏳️ The steps of a long depiction: a generator of {@link Checkpoint}s run to its result with progress and cancellation, synchronously ({@link drive}) or yielding to the event loop between the steps ({@link driveAsync}).
 *
 * @see ../🟦️.ts — `depict` and `depictAsync` drive their steps with these
 * @see ../🧊️axonometric/🟦️.ts — the one depiction whose steps are long */
import type { Text } from "../../../🧬️schema/🟦️.ts";

/** 📍️ A point of progress of the depiction: `done` of `total` permille and what is being done. */
export type Checkpoint = { readonly done: number; readonly total: number; readonly label: Text };

/** 🎛️ How a run is controlled: an abort signal checked before every step and a progress callback. */
export type StepControl = { readonly signal?: AbortSignal; readonly progress?: (done: number, total: number, label: Text) => void };

/** 🦄 Runs a generator of checkpoints to its result, reporting progress and honouring cancellation: the signal is checked before every step (a cancelled run throws the reason of the signal, as every staffage export does). */
export function drive<T>(steps: Generator<Checkpoint, T>, control?: StepControl): T {
  for (;;) {
    control?.signal?.throwIfAborted();
    const next = steps.next();
    if (next.done === true) return next.value;
    control?.progress?.(next.value.done, next.value.total, next.value.label);
  }
}

/** 🐙 The asynchronous form of {@link drive}: yields to the event loop between steps, so a long depiction neither blocks the thread nor a cancellation. */
export async function driveAsync<T>(steps: Generator<Checkpoint, T>, control?: StepControl): Promise<T> {
  for (;;) {
    control?.signal?.throwIfAborted();
    const next = steps.next();
    if (next.done === true) return next.value;
    control?.progress?.(next.value.done, next.value.total, next.value.label);
    await new Promise<void>((resume) => setTimeout(resume, 0));
  }
}

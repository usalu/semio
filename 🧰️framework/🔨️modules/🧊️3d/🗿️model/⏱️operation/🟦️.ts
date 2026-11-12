/** ⏱️ Progress and cancellation plumbing of the kernel: the error every cancelled operation throws, the operation that never reports, and {@link Pace}, the bounded-interval check hot loops call.
 *
 * `Operation.cancelled()` is polled, never pushed: a synchronous function sees a cancellation only when the callback's answer can change while it runs (a deadline, a shared flag); an asynchronous one also sees what happens while it yields to the event loop.
 * @see ../🧬️schema/🟦️.ts#Operation */
import type { Operation } from "../🧬️schema/🟦️.ts";

/** 🛑️ Thrown by every operation whose `Operation.cancelled()` became true; no partial result is produced. */
export class CancelledError extends Error {
  constructor() {
    super("cancelled");
    this.name = "CancelledError";
  }
}

/** 🔇️ An operation that never reports and never cancels. */
export const silentOperation: Operation = { report: () => {}, cancelled: () => false };

/** 🚦️ Default number of work units between two checks of `cancelled()`. */
export const PACE_INTERVAL = 2048;

/** 🥁️ Counts work units of a hot loop and asks the operation about cancellation every `interval` units, so the time between two checks stays bounded no matter how large one item of work is. */
export class Pace {
  private units = 0;

  constructor(readonly operation: Operation = silentOperation, readonly interval: number = PACE_INTERVAL) {}

  /** ⏳️ Adds `units` of work; when `interval` units accumulated, throws {@link CancelledError} if cancelled and reports `fraction` when given. */
  tick(units = 1, fraction?: number): void {
    this.units += units;
    if (this.units < this.interval) return;
    this.units = 0;
    this.check();
    if (fraction !== undefined) this.operation.report(fraction);
  }

  /** ✋️ Throws {@link CancelledError} when the operation is cancelled, regardless of the count. */
  check(): void {
    if (this.operation.cancelled()) throw new CancelledError();
  }

  /** 📈️ Reports `fraction` after checking for cancellation. */
  step(fraction: number): void {
    this.check();
    this.operation.report(fraction);
  }
}

/** 🧵️ The video render job: encoder tier, job events, outcomes and rows, and the ledger that applies them in order.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region InvocationResponse
//#region 🧵️VideoRenderJob
/** 🧩️ Which encoder produced a finished render — Rust `kernel::VideoRenderEncoderTier`. */
export type VideoRenderEncoderTier = "platform" | "first-party";

/** 🏁️ How one host video render job ended — Rust `kernel::VideoRenderJobOutcome` (tagged by `status`). */
export type VideoRenderJobOutcome =
  | { readonly status: "done"; readonly bytes: number; readonly tier: VideoRenderEncoderTier }
  | { readonly status: "cancelled"; readonly completed: number }
  | { readonly status: "refused"; readonly code: string }
  | { readonly status: "failed"; readonly reason: string };

/** 🧵️ One fact of a host video render job's life — Rust `kernel::VideoRenderJobEvent` (tagged by `kind`). The task list
 * is the fold of these ({@link VideoRenderJobLedger}); nothing mutates it directly. */
export type VideoRenderJobEvent =
  | { readonly kind: "started"; readonly job: number; readonly owner: string; readonly filename: string; readonly frames: number; readonly atMs: number }
  | { readonly kind: "progressed"; readonly job: number; readonly completed: number }
  | { readonly kind: "cancelRequested"; readonly job: number }
  | { readonly kind: "finished"; readonly job: number; readonly outcome: VideoRenderJobOutcome };

/** 🧵️ One running job as a task list shows it — Rust `kernel::VideoRenderJobRow`. */
export interface VideoRenderJobRow {
  readonly job: number;
  readonly owner: string;
  readonly filename: string;
  readonly frames: number;
  readonly completed: number;
  readonly cancelling: boolean;
  readonly startedAtMs: number;
}

/** 🚨️ The fixture's refusal vocabulary — Rust `VideoRenderJobEventError::code`. */
export type VideoRenderJobEventErrorCode = "staleJob" | "unknownJob" | "progressRegressed" | "progressOverrun" | "alreadyCancelling";

/** 📒️ The fold of a host's {@link VideoRenderJobEvent} log — twin of Rust `kernel::VideoRenderJobLedger`. `running()` keeps
 * its identity until an event changes it, so a `useSyncExternalStore` reader re-renders only on real change. */
export class VideoRenderJobLedger {
  #lastJob = 0;
  #running: readonly VideoRenderJobRow[] = [];

  /** 📒️ Folds a whole log; a refusal names the index of the first event refused. */
  static fold(events: readonly VideoRenderJobEvent[]): VideoRenderJobLedger | { readonly index: number; readonly error: VideoRenderJobEventErrorCode } {
    const ledger = new VideoRenderJobLedger();
    for (const [index, event] of events.entries()) {
      const error = ledger.apply(event);
      if (error !== null) return { index, error };
    }
    return ledger;
  }

  /** ➕️ Applies one event (`null`), or refuses it with a code and leaves the ledger as it was. */
  apply(event: VideoRenderJobEvent): VideoRenderJobEventErrorCode | null {
    if (event.kind === "started") {
      if (event.job <= this.#lastJob) return "staleJob";
      this.#lastJob = event.job;
      this.#running = [...this.#running, { job: event.job, owner: event.owner, filename: event.filename, frames: event.frames, completed: 0, cancelling: false, startedAtMs: event.atMs }];
      return null;
    }
    const row = this.#running.find((candidate) => candidate.job === event.job);
    if (!row) return "unknownJob";
    if (event.kind === "progressed") {
      if (event.completed < row.completed) return "progressRegressed";
      if (event.completed > row.frames) return "progressOverrun";
      if (event.completed !== row.completed) this.#running = this.#running.map((candidate) => (candidate === row ? { ...row, completed: event.completed } : candidate));
      return null;
    }
    if (event.kind === "cancelRequested") {
      if (row.cancelling) return "alreadyCancelling";
      this.#running = this.#running.map((candidate) => (candidate === row ? { ...row, cancelling: true } : candidate));
      return null;
    }
    this.#running = this.#running.filter((candidate) => candidate !== row);
    return null;
  }

  /** 🧵️ Every running job, in start order. */
  running(): readonly VideoRenderJobRow[] {
    return this.#running;
  }

  /** 🔎️ Running job `job`, if any. */
  row(job: number): VideoRenderJobRow | undefined {
    return this.#running.find((candidate) => candidate.job === job);
  }

  /** 🔢️ The last job id issued (`0` before the first). */
  lastJob(): number {
    return this.#lastJob;
  }
}
//#endregion 🧵️VideoRenderJob
//#endregion InvocationResponse
// #endregion 🎠️Kernel

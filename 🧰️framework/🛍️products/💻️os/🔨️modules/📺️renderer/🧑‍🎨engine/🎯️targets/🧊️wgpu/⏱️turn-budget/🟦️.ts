// #region ⏱️Ceiling
/** @emoji ⏱️ The browser's twin of the guest step authority: what one turn of a browser isolate may
 * cost, how an overrun is priced, and who may be degraded for it. BOTH isolates the wgpu surface runs
 * in are priced here — the UI isolate against {@link UI_TURN_BUDGET_MS}, the frame Worker against
 * {@link WORKER_STEP_BUDGET_MS} — because neither of them publishes a per-thread CPU clock and the
 * attribution problem is therefore identical in both.
 *
 * 🪞️ Mirrors `semio_framework_trace`'s `INTERACTIVE_STEP_CEILING_US` /
 * `SUSTAINED_OVERRUN_QUARANTINE_STEPS` / `StepOverrunLedger` law
 * (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs` region `📒️OverrunLedger`) across the language boundary. The
 * two cannot share a declaration, so `🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts` and
 * `🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts` pin the constants against the Rust ones (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END). */

/** @emoji ⏱️ What "interactive" means for ONE UI turn. Unchanged in meaning by the ledger below: every
 * breach is still measured and still recorded — what changed is that a breach is no longer a verdict. */
export const UI_TURN_BUDGET_MS = 2;

/** @emoji 🧵️ What "interactive" means for ONE frame-Worker step — a boot step, a frame step, an
 * interactive-job admission, an asset pump step, a close step. Four times {@link UI_TURN_BUDGET_MS}
 * because the Worker owns the whole frame while the UI isolate only has to stay responsive within it.
 *
 * 🧯️ Priced by {@link TurnLedger} against EXECUTING spans exactly like the UI ceiling, and for the same
 * measured reason: `plugin-graph` executes for 187 µs natively (median of 9, `🧪️wboot-measure.ts`) and
 * read **8.300 ms** of wall time in a hidden pane on a box at load ~20 — a 44× scheduling artefact that
 * terminated the Worker, took the OffscreenCanvas with it, and left the surface with no frame path at
 * all. A breach of this ceiling is a measurement; only a throw is a fault. */
export const WORKER_STEP_BUDGET_MS = 8;

/** @emoji 🔁️ How many CONSECUTIVE UI turns must each breach [`UI_TURN_BUDGET_MS`] before the overrun is
 * attributed to the turn's own work instead of to the machine that scheduled it.
 *
 * 🧮️ Why a consecutive count and not thread CPU time: the browser publishes no per-thread CPU clock at
 * all — `performance.now()` is wall time, and a UI isolate in a hidden pane is additionally subject to
 * background throttling and to an OS that is scheduling ~20 other WASM workers. A synchronous
 * `status.textContent = …` measured 5.000 ms of wall time on a box at load ~20 while executing for
 * microseconds. A run of four separate scheduler decisions is the only signal available on this target
 * that separates "this turn is expensive" from "this isolate was not running", which is exactly the
 * reasoning `SUSTAINED_OVERRUN_QUARANTINE_STEPS` already settled for the Rust side. */
export const SUSTAINED_TURN_OVERRUN_TURNS = 4;

/** @emoji 📊️ Fixed telemetry ring — no allocation on the measured path. */
export const TURN_SAMPLE_CAPACITY = 64;

/** @emoji 🩺️ Arms the recorded-overrun traces. 🪞️ The same key `🏛️ShellHost/🟦️.tsx`'s
 * `RUNTIME_DIAGNOSTICS_KEY` and the guest's `RUNTIME_DIAGNOSTICS_ENV` use; declared here rather than
 * imported because the wgpu boot bundle must not pull the React shell into the UI isolate.
 *
 * 🔒️ This module deliberately reads NO web storage. It is bundled into `🎞️frame-worker.js`, whose
 * carrier census (`📦️packages/🦀️rust/📜️script.ts` `checkFrameWorkerCarrierCensus`) forbids every
 * storage and credential carrier outright. The UI isolate resolves a stored preference itself and hands
 * the answer to {@link setTurnDiagnostics}. */
export const TURN_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";

/** @emoji ⚖️ What one measured UI turn means for the surface that produced it.
 * `admitted` — inside the ceiling. `recorded-overrun` — one breach, counted, work continues.
 * `sustained-overrun` — {@link SUSTAINED_TURN_OVERRUN_TURNS} breaches in a row, so the cost is the
 * turn's own: the owner degrades to deferred cadence and still continues. `clock-fault` — the reading
 * itself is unusable (absent, negative, or non-finite), so nothing was measured; also non-fatal, because
 * a broken clock is never evidence about the work. */
export type TurnVerdict = "admitted" | "recorded-overrun" | "sustained-overrun" | "clock-fault";

export type TurnOutcome = {
  readonly site: string;
  readonly verdict: TurnVerdict;
  readonly executingMs: number;
  readonly consecutive: number;
};

export type TurnLedgerSnapshot = {
  readonly recordedOverruns: number;
  readonly sustainedOverruns: number;
  readonly consecutive: number;
  readonly longestRun: number;
  readonly worstExecutingMs: number;
  readonly worstSite: string;
  readonly degraded: boolean;
  readonly p99Ms: number;
};
// #endregion ⏱️Ceiling

// #region 🩺️Diagnostics
let diagnosticsOverride: boolean | undefined;
let diagnosticsResolved: boolean | undefined;

function diagnosticsArmed(value: unknown): boolean {
  return typeof value === "string" && ["1", "true", "on", "yes"].includes(value.trim().toLowerCase());
}

/** @emoji 🩺️ Arms or disarms the UI-turn traces for this isolate, outranking build env and stored
 * preference — the transport's counterpart to `setRuntimeDiagnostics`. */
export function setTurnDiagnostics(enabled: boolean | undefined): void {
  diagnosticsOverride = enabled;
  diagnosticsResolved = undefined;
}

/** @emoji 🩺️ Whether recorded overruns may print. Resolved once per isolate: an explicit override wins,
 * otherwise the build's `VITE_SEMIO_RUNTIME_DIAGNOSTICS`. The reader is wrapped because a Worker has no
 * `import.meta.env` at all. */
export function turnDiagnosticsEnabled(): boolean {
  if (diagnosticsOverride !== undefined) return diagnosticsOverride;
  if (diagnosticsResolved !== undefined) return diagnosticsResolved;
  let armed = false;
  try {
    armed = diagnosticsArmed((import.meta as { readonly env?: Record<string, unknown> }).env?.[`VITE_${TURN_DIAGNOSTICS_KEY}`]);
  } catch {
    armed = false;
  }
  diagnosticsResolved = armed;
  return armed;
}
// #endregion 🩺️Diagnostics

// #region ⏳️ExecutingClock
/** @emoji ⏳️ Charges only the spans a UI turn is actually EXECUTING. 🪞️ The browser twin of the guest
 * reactor's `TURN_EXECUTION` accumulator (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` region `⏱️TurnExecution`):
 * a turn that yields — an `await`, a deferred macrotask, a rAF hand-off — accrues nothing across the gap,
 * so hidden-tab throttling and macrotask latency can never be billed to the hook that yielded. Spans
 * nest: an inner turn suspends the outer one for exactly its own duration, so a hook that observes a
 * nested hook is charged once, not twice. */
export class TurnClock {
  private depth = 0;
  private spanStartedAt = 0;
  private charged = 0;
  private clockLost = false;

  constructor(private readonly now: () => number) {}

  /** @emoji ▶️ Opens a turn (or a nested span of the open turn). */
  enter(): void {
    if (this.depth === 0) {
      this.charged = 0;
      this.clockLost = false;
      this.spanStartedAt = this.reading();
    } else {
      this.chargeSpan();
    }
    this.depth++;
  }

  /** @emoji ⏸️ Stops charging while the turn is provably not running. */
  suspend(): void {
    if (this.depth === 0) return;
    this.chargeSpan();
  }

  /** @emoji ▶️ Resumes charging after a suspension gap that accrued nothing. */
  resume(): void {
    if (this.depth === 0) return;
    this.spanStartedAt = this.reading();
  }

  /** @emoji ⏹️ Closes the turn and answers the executing milliseconds, or `undefined` when the clock
   * never produced a usable reading. */
  leave(): number | undefined {
    if (this.depth === 0) return undefined;
    this.depth--;
    if (this.depth > 0) return undefined;
    this.chargeSpan();
    return this.clockLost ? undefined : this.charged;
  }

  private chargeSpan(): void {
    const now = this.reading();
    const started = this.spanStartedAt;
    if (!Number.isFinite(now) || !Number.isFinite(started) || now < started) {
      this.clockLost = true;
      return;
    }
    this.charged += now - started;
    this.spanStartedAt = now;
  }

  private reading(): number {
    try {
      const value = this.now();
      return typeof value === "number" ? value : Number.NaN;
    } catch {
      return Number.NaN;
    }
  }
}
// #endregion ⏳️ExecutingClock

// #region 📒️Ledger
/** @emoji 📒️ One isolate's fixed-capacity turn ledger. Every measured turn is admitted here exactly
 * once, in turn order, and the ledger answers whether the isolate has now overrun
 * {@link SUSTAINED_TURN_OVERRUN_TURNS} turns in a row. Any admitted turn resets the run, so an
 * isolated descheduling spike is recorded and forgotten while a genuinely expensive turn degrades the
 * owner's cadence. Nothing here ever fails a surface: this module owns no fault path at all.
 *
 * ⚖️ `budgetMs` is the owning isolate's ceiling — {@link UI_TURN_BUDGET_MS} on the UI thread,
 * {@link WORKER_STEP_BUDGET_MS} in the frame Worker. The attribution law above it is the same one in
 * both, which is exactly why it is declared once. */
export class TurnLedger {
  private readonly samples = new Float64Array(TURN_SAMPLE_CAPACITY);
  private sampleCount = 0;
  private consecutive = 0;
  private longestRun = 0;
  private recorded = 0;
  private sustained = 0;
  private worstExecutingMs = 0;
  private worstSite = "";
  private degradedUntilAdmitted = false;

  constructor(
    private readonly budgetMs: number = UI_TURN_BUDGET_MS,
    private readonly scope: string = "ui-turn",
  ) {}

  admit(site: string, executingMs: number | undefined): TurnOutcome {
    if (executingMs === undefined || !Number.isFinite(executingMs) || executingMs < 0) {
      this.consecutive = 0;
      return { site, verdict: "clock-fault", executingMs: 0, consecutive: 0 };
    }
    this.samples[this.sampleCount % TURN_SAMPLE_CAPACITY] = executingMs;
    this.sampleCount++;
    if (executingMs < this.budgetMs) {
      this.consecutive = 0;
      this.degradedUntilAdmitted = false;
      return { site, verdict: "admitted", executingMs, consecutive: 0 };
    }
    this.consecutive++;
    this.longestRun = Math.max(this.longestRun, this.consecutive);
    this.recorded++;
    if (executingMs > this.worstExecutingMs) {
      this.worstExecutingMs = executingMs;
      this.worstSite = site;
    }
    if (this.consecutive < SUSTAINED_TURN_OVERRUN_TURNS) {
      this.trace(site, "recorded-overrun", executingMs);
      return { site, verdict: "recorded-overrun", executingMs, consecutive: this.consecutive };
    }
    this.sustained++;
    this.degradedUntilAdmitted = true;
    this.trace(site, "sustained-overrun", executingMs);
    return { site, verdict: "sustained-overrun", executingMs, consecutive: this.consecutive };
  }

  /** @emoji 🐢️ Whether the owner should run its next turns on deferred cadence. Latches on a sustained
   * run and clears on the first admitted turn, so a surface recovers on its own. */
  degraded(): boolean {
    return this.degradedUntilAdmitted;
  }

  snapshot(): TurnLedgerSnapshot {
    return {
      recordedOverruns: this.recorded,
      sustainedOverruns: this.sustained,
      consecutive: this.consecutive,
      longestRun: this.longestRun,
      worstExecutingMs: this.worstExecutingMs,
      worstSite: this.worstSite,
      degraded: this.degradedUntilAdmitted,
      p99Ms: this.p99Ms(),
    };
  }

  p99Ms(): number {
    const count = Math.min(this.sampleCount, TURN_SAMPLE_CAPACITY);
    if (count === 0) return 0;
    const ordered = Array.from(this.samples.subarray(0, count)).sort((left, right) => left - right);
    return ordered[Math.min(count - 1, Math.ceil(count * 0.99) - 1)]!;
  }

  private trace(site: string, verdict: TurnVerdict, executingMs: number): void {
    if (!turnDiagnosticsEnabled()) return;
    console.debug(`[DEBUG] ${this.scope} ${verdict} site=${site} executing=${executingMs.toFixed(3)}ms budget=${this.budgetMs}ms consecutive=${this.consecutive}/${SUSTAINED_TURN_OVERRUN_TURNS}`);
  }
}
// #endregion 📒️Ledger

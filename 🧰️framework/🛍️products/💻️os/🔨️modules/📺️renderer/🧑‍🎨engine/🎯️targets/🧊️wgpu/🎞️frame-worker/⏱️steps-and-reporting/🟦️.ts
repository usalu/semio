import { BOOT_HEARTBEAT_MS, BOOT_LIVENESS_INTERVAL_MS, bootDeclarationsOpen, bootPhaseStack, closed, closing, failed, lifecycle, scope, setLastProgressValue, setLastStepOutcome, stepClock, stepLedger, suspensionLedger } from "../🧵️state/🟦️.ts";
import { TurnLedger } from "../../⏱️turn-budget/🟦️.ts";
import type { BrowserFrameWorkerMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";

/** ⏱️ Runs one owned step inside the executing clock and admits it to its ledger. The measurement
 * happens whether the callback returns or throws; the THROW is the only thing that propagates. */
export function ownedStep<T>(stage: string, callback: () => T, ledger: TurnLedger = stepLedger): T {
  stepClock.enter();
  try {
    return callback();
  } finally {
    setLastStepOutcome(ledger.admit(stage, stepClock.leave()));
  }
}

/** 🧭️ One SYNCHRONOUS owned step long enough to be worth declaring. A blocking Rust bootstrap phase
 * (`font-atlas` measured at 12 002 ms) posts nothing while it runs — the same blindness a browser-owned
 * suspension has — so it declares itself to the UI-isolate watchdog first and withdraws afterwards, and its
 * real cost rides out on the withdrawal. */
export function declaredStep<T>(stage: string, callback: () => T, ledger: TurnLedger = stepLedger): T {
  const startedAt = performance.now();
  declarePhase(stage, "enter", 0);
  try {
    return ownedStep(stage, callback, ledger);
  } finally {
    declarePhase(stage, "leave", performance.now() - startedAt);
  }
}

/** 🐢️ Whether this Worker's owned steps have sustained a run of overruns and the boot should hand
 * the isolate back between chunks. Latches on the run and clears itself on the first admitted step. */
function stepsDegraded(): boolean {
  return stepLedger.degraded();
}

/** 📊️ What the UI isolate is told about this Worker's own step ledger on every boot report. */
function stepLedgerReport(): { readonly degraded: boolean; readonly recordedOverruns: number; readonly sustainedOverruns: number; readonly worstStepMs: number; readonly worstStepSite: string } {
  const snapshot = stepLedger.snapshot();
  return { degraded: snapshot.degraded, recordedOverruns: snapshot.recordedOverruns, sustainedOverruns: snapshot.sustainedOverruns, worstStepMs: snapshot.worstExecutingMs, worstStepSite: snapshot.worstSite };
}

/** ⏸️ Awaits one browser-owned operation. The synchronous prologue is an owned step; the blocking
 * gap the heartbeat measures is admitted to {@link suspensionLedger} as an observation — a Worker that did
 * not run is the machine's report about the machine, never evidence about this step's own work, so it can
 * no longer end the boot. `FRAME_WORKER_BOOT_TIMEOUT_MS` in the UI isolate remains the outer bound and the
 * liveness heartbeat keeps it honest. */
export async function monitoredSuspension<T>(stage: string, operation: () => Promise<T>, ledger: TurnLedger = suspensionLedger): Promise<T> {
  const declaredAt = performance.now();
  declarePhase(stage, "enter", 0);
  let lastBeat = performance.now();
  let lastLivenessAt = performance.now();
  let maximumBlockMs = 0;
  const heartbeat = setInterval(() => {
    const now = performance.now();
    maximumBlockMs = Math.max(maximumBlockMs, now - lastBeat - BOOT_HEARTBEAT_MS);
    lastBeat = now;
    if (now - lastLivenessAt >= BOOT_LIVENESS_INTERVAL_MS) {
      lastLivenessAt = now;
      if (!closed && !closing && !failed) post({ kind: "boot-liveness", lifecycle });
    }
  }, BOOT_HEARTBEAT_MS);
  try {
    stepClock.suspend();
    const result = await ownedStep(`${stage}:start`, operation, ledger);
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
    if (closed || closing) throw new Error(`worker-boot-cancelled: ${stage}`);
    ledger.admit(`${stage}:block`, maximumBlockMs);
    return result;
  } finally {
    clearInterval(heartbeat);
    stepClock.resume();
    declarePhase(stage, "leave", performance.now() - declaredAt);
  }
}

export async function macrotask(): Promise<void> {
  stepClock.suspend();
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
  stepClock.resume();
  if (closed || closing) throw new Error("worker-boot-cancelled");
}

/** 🧩️ A unit of boot work that can be performed one bounded chunk at a time — the shape
 * `PlaygroundBootPlanner` and the Rust `BrowserRendererBootstrap` phase machine both take. */
type ResumableBootUnit = { stage(): string; completion(): number; step(): boolean };

/** ⏭️ Drives one resumable unit to completion, one chunk per owned step, reporting each chunk as
 * boot progress and handing the isolate back between chunks whenever the ledger says this Worker's own
 * steps are running long. Chunk size is the unit's own business; the ceiling is never the unit's. */
export async function driveChunks(unit: ResumableBootUnit, base: number, span: number): Promise<void> {
  for (;;) {
    const stage = unit.stage();
    const more = ownedStep(stage, () => unit.step());
    progress(stage, base + unit.completion() * span);
    if (!more) return;
    if (stepsDegraded()) await macrotask();
  }
}

/** 📣️ Reports one boot stage AND this Worker's own step ledger with it, so the boot UI shows the
 * degraded state instead of only the stage that happened to be running when the isolate was descheduled. */
export function progress(stage: string, value: number): void {
  setLastProgressValue(value);
  if (!closed && !closing && !failed) post({ kind: "boot-progress", lifecycle, stage, progress: value, worker: stepLedgerReport() });
}

function declarePhase(phase: string, state: "enter" | "leave", elapsedMs: number): void {
  if (!bootDeclarationsOpen || closed || closing || failed) return;
  if (state === "enter") {
    bootPhaseStack.push(phase);
    post({ kind: "boot-phase", lifecycle, phase, state, elapsedMs });
    return;
  }
  post({ kind: "boot-phase", lifecycle, phase, state, elapsedMs });
  const top = bootPhaseStack[bootPhaseStack.length - 1];
  if (top === phase) bootPhaseStack.pop();
  const parent = bootPhaseStack[bootPhaseStack.length - 1];
  if (parent) post({ kind: "boot-phase", lifecycle, phase: parent, state: "enter", elapsedMs: 0 });
}

/** 🧭️ Nested `boot-phase` under an already-declared parent. Re-enters the parent on leave so the watchdog never sees an undeclared Worker. */
export function declareBootSubphase(phase: string, state: "enter" | "leave", elapsedMs: number): void {
  declarePhase(phase, state, elapsedMs);
}

export function post(message: BrowserFrameWorkerMessage, transfer: Transferable[] = []): void {
  scope.postMessage(message, transfer);
}

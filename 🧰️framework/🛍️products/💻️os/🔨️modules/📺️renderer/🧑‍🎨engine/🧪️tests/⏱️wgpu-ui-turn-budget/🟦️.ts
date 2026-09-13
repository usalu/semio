import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  SUSTAINED_TURN_OVERRUN_TURNS,
  UI_TURN_BUDGET_MS,
  TURN_DIAGNOSTICS_KEY,
  TurnClock,
  TurnLedger,
  setTurnDiagnostics,
} from "../../🎯️targets/🧊️wgpu/⏱️turn-budget/🟦️.ts";
import { BrowserFrameTransport, type BrowserFrameUiMessage, type BrowserFrameWorkerMessage, type BrowserFrameWorkerPort, type BrowserFrameWorkerStepReport } from "../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";

/** @emoji 🧵️ A frame Worker whose own steps all fit their ceiling — this suite measures the UI isolate. */
const ADMITTED_WORKER_STEPS: BrowserFrameWorkerStepReport = { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" };

const REPO_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "..", "..", "..", "..", "..");
const TRACE_RS = join(REPO_ROOT, "🧰️framework", "🔨️modules", "⏱️trace", "🦀️.rs");
const SHELL_HOST_TSX = join(REPO_ROOT, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📺️renderer", "🧑‍🎨engine", "🧱️elements", "🏛️ShellHost", "🟦️.tsx");

class FakeWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];
  terminated = false;

  postMessage(message: BrowserFrameUiMessage): void {
    this.messages.push(message);
  }

  terminate(): void {
    this.terminated = true;
  }

  reply(message: BrowserFrameWorkerMessage): void {
    this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>);
  }
}

type Harness = {
  readonly worker: FakeWorker;
  readonly transport: BrowserFrameTransport;
  readonly stages: string[];
  readonly faults: string[];
  drainContinuations(): void;
};

/** @emoji 🧪️ One transport whose zero-delay continuations are drained by hand, so the boot-stall timer
 * (the only other timer this seam arms) can never be mistaken for one. */
function harness(options: { now?: () => number; onProgress?: (stage: string) => void } = {}): Harness {
  const worker = new FakeWorker();
  const stages: string[] = [];
  const faults: string[] = [];
  const continuations: Array<() => void> = [];
  const transport = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, pluginVariant: "generation3d", locale: "en", appRole: "editor", appMode: "", appExample: "" },
    ...(options.now ? { now: options.now } : {}),
    setTimer: (callback, delayMs) => (delayMs === 0 ? continuations.push(callback) : 0),
    clearTimer: () => {},
    onProgress: (stage) => {
      stages.push(stage);
      options.onProgress?.(stage);
    },
    onFault: (code) => faults.push(code),
  });
  return {
    worker,
    transport,
    stages,
    faults,
    drainContinuations() {
      for (const callback of continuations.splice(0, continuations.length)) callback();
    },
  };
}

function spinMs(milliseconds: number): void {
  const until = performance.now() + milliseconds;
  while (performance.now() < until) {
    /* a genuinely executing turn */
  }
}

describe("wgpu UI-turn budget", () => {
  it("pins the ceiling and the attribution law to their Rust twins", () => {
    const trace = readFileSync(TRACE_RS, "utf8");
    const sustained = /pub const SUSTAINED_OVERRUN_QUARANTINE_STEPS: u32 = (\d+);/.exec(trace);
    expect(sustained?.[1]).toBeDefined();
    expect(SUSTAINED_TURN_OVERRUN_TURNS).toBe(Number(sustained![1]));
    expect(UI_TURN_BUDGET_MS).toBe(2);
    expect(readFileSync(SHELL_HOST_TSX, "utf8")).toContain(`RUNTIME_DIAGNOSTICS_KEY = "${TURN_DIAGNOSTICS_KEY}"`);
  });

  it("charges executing time only — a turn parked across a real 50 ms suspension measures nothing", async () => {
    const clock = new TurnClock(() => performance.now());
    const wallStartedAt = performance.now();
    clock.enter();
    clock.suspend();
    await new Promise<void>((resolve) => setTimeout(resolve, 50));
    clock.resume();
    const executingMs = clock.leave();
    expect(performance.now() - wallStartedAt).toBeGreaterThanOrEqual(45);
    expect(executingMs).toBeDefined();
    expect(executingMs!).toBeLessThan(UI_TURN_BUDGET_MS);
  });

  it("charges a real 50 ms spin and reaches the sustained verdict, never a fatal one", () => {
    const ledger = new TurnLedger();
    const clock = new TurnClock(() => performance.now());
    const verdicts: string[] = [];
    for (let turn = 0; turn < SUSTAINED_TURN_OVERRUN_TURNS; turn++) {
      clock.enter();
      spinMs(50);
      verdicts.push(ledger.admit("spin", clock.leave()).verdict);
    }
    expect(verdicts.slice(0, -1).every((verdict) => verdict === "recorded-overrun")).toBe(true);
    expect(verdicts.at(-1)).toBe("sustained-overrun");
    expect(ledger.degraded()).toBe(true);
    expect(ledger.admit("cheap", 0).verdict).toBe("admitted");
    expect(ledger.degraded()).toBe(false);
  });

  it("treats an unusable clock reading as a clock fault, never as an overrun", () => {
    const ledger = new TurnLedger();
    for (const reading of [undefined, Number.NaN, Number.POSITIVE_INFINITY, -1]) expect(ledger.admit("lost", reading).verdict).toBe("clock-fault");
    expect(ledger.snapshot().recordedOverruns).toBe(0);
  });

  it("completes shell construction when a progress-hook turn sleeps 50 ms of wall time", () => {
    let now = 0;
    const subject = harness({ now: () => now, onProgress: () => { now += 50; } });
    subject.worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "shell-construct", progress: 0.845, worker: ADMITTED_WORKER_STEPS });
    expect(subject.stages).toEqual(["shell-construct"]);
    expect(subject.transport.status).toBe("booting");
    expect(subject.transport.fault).toBeUndefined();
    expect(subject.faults).toEqual([]);
    subject.worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.transport.status).toBe("ready");
  });

  it("degrades to deferred cadence and keeps reporting progress when the hook spins past the ceiling", () => {
    const subject = harness({ onProgress: () => spinMs(50) });
    for (let turn = 0; turn < SUSTAINED_TURN_OVERRUN_TURNS; turn++) subject.worker.reply({ kind: "boot-progress", lifecycle: 1, stage: `stage-${turn}`, progress: 0.1 * turn, worker: ADMITTED_WORKER_STEPS });
    expect(subject.transport.degraded()).toBe(true);
    expect(subject.transport.status).toBe("booting");
    expect(subject.faults).toEqual([]);
    expect(subject.stages).toHaveLength(SUSTAINED_TURN_OVERRUN_TURNS);
    subject.worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "deferred-stage", progress: 0.9, worker: ADMITTED_WORKER_STEPS });
    expect(subject.stages).toHaveLength(SUSTAINED_TURN_OVERRUN_TURNS);
    subject.drainContinuations();
    expect(subject.stages.at(-1)).toBe("deferred-stage");
    expect(subject.transport.status).toBe("booting");
    subject.worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.transport.status).toBe("ready");
  });

  it("reports the real fallback state instead of a static no-fallback claim", () => {
    const subject = harness();
    subject.worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.transport.fallbackState()).toMatchObject({ surface: "ready", uiThreadFrames: "unavailable-offscreen-transferred", workerTerminated: false, inputAccepted: true, deferredCadence: false });
    subject.worker.reply({ kind: "fault", lifecycle: 1, code: "runtime", detail: "broken" });
    const faulted = subject.transport.fallbackState();
    expect(faulted).toMatchObject({ surface: "faulted", workerTerminated: true, inputAccepted: false });
    expect(faulted.uiTurns.recordedOverruns).toBe(0);
  });

  it("keeps recorded-overrun traces behind the runtime diagnostics switch", () => {
    const printed: string[] = [];
    const original = console.debug;
    console.debug = (...args: unknown[]) => { printed.push(args.join(" ")); };
    try {
      setTurnDiagnostics(false);
      new TurnLedger().admit("silent", 50);
      expect(printed).toEqual([]);
      setTurnDiagnostics(true);
      new TurnLedger().admit("loud", 50);
      expect(printed.some((line) => line.startsWith("[DEBUG] ui-turn recorded-overrun site=loud"))).toBe(true);
    } finally {
      console.debug = original;
      setTurnDiagnostics(undefined);
    }
  });
});

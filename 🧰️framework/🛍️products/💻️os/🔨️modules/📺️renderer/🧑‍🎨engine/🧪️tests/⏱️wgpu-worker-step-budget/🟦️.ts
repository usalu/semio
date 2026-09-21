import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { PlaygroundBootPlanner, PLUGIN_GRAPH_CHUNK_ROWS, resolvePlaygroundBoot } from "@semio-tech/framework";
import { SUSTAINED_TURN_OVERRUN_TURNS, TurnClock, TurnLedger, WORKER_STEP_BUDGET_MS } from "../../🎯️targets/🧊️wgpu/⏱️turn-budget/🟦️.ts";
import { FrameTurnScheduler, WorkerTurnTaskQueue, nextFrameSequence } from "../../🎯️targets/🧊️wgpu/🧵️frame-turn-scheduler/🟦️.ts";
import { PLUGIN_CATALOG } from "../../../../🔌️plugin/📇️registry/🟦️.ts";

const ENGINE_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const FRAME_WORKER_TS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🎞️frame-worker", "🟦️.ts");
const FRAME_JOB_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧵️frame-job", "🦀️.rs");
const BROWSER_WORKER_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🌐️browser-worker", "🦀️.rs");
const RENDERER_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧊️renderer", "🦀️.rs");
const FRAME_TURN_FIXTURE = join(ENGINE_ROOT, "🧫️fixtures", "🧵️frame-turn-scheduling", "🔣️.json");

/** @emoji 🔥️ A genuinely EXECUTING span — the only thing the ceiling is allowed to charge for. */
function spinMs(milliseconds: number): void {
  const until = performance.now() + milliseconds;
  while (performance.now() < until) {
    /* burning the isolate's own time */
  }
}

/** @emoji 🧵️ One frame-Worker boot step, driven the way `🎞️frame-worker/🟦️.ts` drives one: priced on the
 * executing clock, admitted to the ledger, yielded after when the ledger says the Worker is running long,
 * and NEVER able to end the boot. */
async function workerBootStep<T>(ledger: TurnLedger, clock: TurnClock, stage: string, run: () => T, yields: string[]): Promise<T> {
  clock.enter();
  let value: T;
  try {
    value = run();
  } finally {
    ledger.admit(stage, clock.leave());
  }
  if (ledger.degraded()) {
    yields.push(stage);
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  }
  return value;
}

describe("wgpu frame-Worker step budget", () => {
  it("runs one retained frame unit per Worker callback, admits ingress between turns, and closes in a bounded callback", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly requests: readonly [{ readonly remainingTurns: number }];
      readonly expected: { readonly callbackOwners: readonly string[]; readonly frameSequences: readonly number[]; readonly closeOwners: readonly string[] };
    };
    const callbacks: (() => void)[] = [];
    const owners: string[] = [];
    const sequences: number[] = [];
    const closeOwners: string[] = [];
    const between: string[] = [];
    let remaining = fixture.requests[0].remainingTurns;
    let sequence = 0;
    const scheduler = new FrameTurnScheduler(
      (callback) => callbacks.push(callback),
      () => {
        owners.push("frame");
        sequence = nextFrameSequence(sequence);
        sequences.push(sequence);
        remaining -= 1;
        return remaining > 0;
      },
      () => {
        closeOwners.push("frame");
        return true;
      },
    );
    scheduler.request();
    scheduler.request();
    expect(callbacks).toHaveLength(1);
    expect(owners).toEqual([]);
    callbacks.shift()!();
    between.push("input-admitted");
    expect(callbacks).toHaveLength(1);
    expect(owners).toEqual(["frame"]);
    callbacks.shift()!();
    expect(between).toEqual(["input-admitted"]);
    expect(owners).toEqual(fixture.expected.callbackOwners);
    expect(sequences).toEqual(fixture.expected.frameSequences);
    scheduler.beginClose();
    expect(callbacks).toHaveLength(1);
    callbacks.shift()!();
    expect(closeOwners).toEqual(fixture.expected.closeOwners);
    expect(scheduler.terminalIsEmpty()).toBe(true);
    expect(() => nextFrameSequence(Number.MAX_SAFE_INTEGER)).toThrow("frame output sequence exhausted");
  });

  it("keeps an actual Worker task armed while the retained runtime frame remains pending", async () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly runtimeTurns: readonly { readonly phase: string; readonly requestFrame: boolean; readonly continueFrame: boolean }[];
      readonly nonRunnableTurns: readonly { readonly owner: string; readonly requestFrame: boolean; readonly continueFrame: false }[];
      readonly textIngressTurns: readonly { readonly phase: string; readonly committed: boolean; readonly retiring: boolean; readonly continueFrame: boolean }[];
      readonly expected: { readonly runtimePhases: readonly string[]; readonly nonRunnableCallbacks: readonly string[]; readonly runnableTextPhases: readonly string[] };
    };
    const workerSource = readFileSync(FRAME_WORKER_TS, "utf8");
    const turnSource = workerSource.slice(workerSource.indexOf("function runFrameTurn"), workerSource.indexOf("function answerIntrospection"));
    expect(turnSource).toContain("return result.continueFrame");
    expect(turnSource).not.toContain("return result.requestFrame");
    expect(workerSource).toContain('new WorkerTurnTaskQueue()');
    expect(workerSource).not.toContain('new FrameTurnScheduler((callback) => setTimeout(callback, 0)');
    const browserWorkerSource = readFileSync(BROWSER_WORKER_RS, "utf8");
    const continuationSource = browserWorkerSource.slice(browserWorkerSource.indexOf("let continue_frame ="), browserWorkerSource.indexOf("encode_tick_timed(generation, BrowserTickOutput {", browserWorkerSource.indexOf("let continue_frame =")));
    expect(continuationSource).toContain("host.frame_build.has_live_session()");
    expect(continuationSource).not.toContain("next_deadline");
    expect(continuationSource).not.toContain("hub_status_pending");
    expect(browserWorkerSource).toContain("request_frame: continue_frame || host.scheduler.next_deadline().is_some()");
    const rendererSource = readFileSync(RENDERER_RS, "utf8");
    const textContinuationAt = rendererSource.lastIndexOf("fn has_pending_text_work");
    const textContinuation = rendererSource.slice(textContinuationAt, rendererSource.indexOf("fn drive_text_operation", textContinuationAt));
    expect(textContinuation).toContain("text_buffer.runnable_work_pending()");
    expect(textContinuation).not.toContain("reserved_bytes()");
    expect(textContinuation).not.toContain("text_streams.iter()");
    const tasks = new WorkerTurnTaskQueue();
    const turns = [...fixture.runtimeTurns];
    const phases: string[] = [];
    await new Promise<void>((complete) => {
      const scheduler = new FrameTurnScheduler(
        tasks.schedule,
        () => {
          const turn = turns.shift();
          if (!turn) throw new Error("runtime frame turn credits exhausted");
          phases.push(turn.phase);
          if (!turn.continueFrame) complete();
          return turn.continueFrame;
        },
        () => true,
      );
      scheduler.request();
    });
    tasks.close();
    expect(phases).toEqual(fixture.expected.runtimePhases);
    expect(turns).toEqual([]);
    expect(() => tasks.schedule(() => {})).toThrow("frame turn task owner is closed");
    const capacityTasks = new WorkerTurnTaskQueue();
    capacityTasks.schedule(() => {});
    expect(() => capacityTasks.schedule(() => {})).toThrow("frame turn task credits exceeded");
    capacityTasks.close();
    const nonRunnableCallbacks: string[] = [];
    for (const turn of fixture.nonRunnableTurns) {
      const ownerTasks = new WorkerTurnTaskQueue();
      await new Promise<void>((complete) => {
        const scheduler = new FrameTurnScheduler(ownerTasks.schedule, () => {
          nonRunnableCallbacks.push(turn.owner);
          complete();
          return turn.continueFrame;
        }, () => true);
        scheduler.request();
      });
      ownerTasks.close();
    }
    expect(nonRunnableCallbacks).toEqual(fixture.expected.nonRunnableCallbacks);
    const runnableTextPhases = fixture.textIngressTurns.filter((turn) => turn.committed || turn.retiring).map((turn) => turn.phase);
    expect(fixture.textIngressTurns.map((turn) => turn.continueFrame)).toEqual(fixture.textIngressTurns.map((turn) => turn.committed || turn.retiring));
    expect(runnableTextPhases).toEqual(fixture.expected.runnableTextPhases);
  });

  it("acknowledges ingress before the private frame callback and never runs tick in the batch handler", () => {
    const worker = readFileSync(FRAME_WORKER_TS, "utf8");
    const handler = worker.slice(worker.indexOf('ownedStep("frame-ingress"'), worker.indexOf("function runFrameTurn"));
    expect(handler).toContain('post({ kind: "batch-accepted"');
    expect(handler).toContain("frameTurns?.request()");
    expect(handler).not.toContain("runtime!.tick(");
    expect(worker.slice(worker.indexOf("function runFrameTurn"), worker.indexOf("function answerIntrospection"))).toContain("runtime!.tick(");
    const frameJob = readFileSync(FRAME_JOB_RS, "utf8");
    const wasmOwner = frameJob.slice(frameJob.indexOf('#[cfg(target_arch = "wasm32")]\n    pub(crate) fn poll_runtime_and_resubmit'), frameJob.indexOf("    /// 🧵️ Whether a frame build is admitted right now"));
    expect(wasmOwner).toContain("try_step_on_worker");
    expect(wasmOwner).not.toContain("try_step_on_caller");
    expect(wasmOwner).not.toContain("BROWSER_FRAME_BUILD_DRIVE_US");
    expect(wasmOwner).not.toContain("loop {");
  });

  it("pins the Worker ceiling and the attribution law it shares with the UI isolate and the guest", () => {
    expect(WORKER_STEP_BUDGET_MS).toBe(8);
    expect(SUSTAINED_TURN_OVERRUN_TURNS).toBe(4);
    const worker = readFileSync(FRAME_WORKER_TS, "utf8");
    expect(worker).toContain('new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step")');
    expect(worker).not.toContain("worker-boot-step-overrun");
    expect(worker).not.toContain("interactive-job-overrun");
    expect(worker).not.toContain("worker-close-overrun");
    expect(worker).not.toMatch(/>= *WORKER_STEP_BUDGET_MS/);
  });

  it("completes a boot step that spins 50 ms of EXECUTING time — it yields and continues instead of ending the boot", async () => {
    const ledger = new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step");
    const clock = new TurnClock(() => performance.now());
    const yields: string[] = [];
    const completed: string[] = [];
    for (let index = 0; index < SUSTAINED_TURN_OVERRUN_TURNS + 1; index++) {
      completed.push(await workerBootStep(ledger, clock, `plugin-graph#${index}`, () => (spinMs(50), `plugin-graph#${index}`), yields));
    }
    expect(completed).toHaveLength(SUSTAINED_TURN_OVERRUN_TURNS + 1);
    const snapshot = ledger.snapshot();
    expect(snapshot.recordedOverruns).toBe(SUSTAINED_TURN_OVERRUN_TURNS + 1);
    expect(snapshot.sustainedOverruns).toBeGreaterThanOrEqual(1);
    expect(snapshot.worstExecutingMs).toBeGreaterThanOrEqual(45);
    expect(yields.length).toBeGreaterThanOrEqual(2);
    const recovered = await workerBootStep(ledger, clock, "cheap", () => "cheap", yields);
    expect(recovered).toBe("cheap");
    expect(ledger.degraded()).toBe(false);
  });

  it("completes a boot step suspended for 500 ms of WALL time with nothing recorded — descheduling is never the step's own cost", async () => {
    const ledger = new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step");
    const clock = new TurnClock(() => performance.now());
    const wallStartedAt = performance.now();
    clock.enter();
    clock.suspend();
    await new Promise<void>((resolve) => setTimeout(resolve, 500));
    clock.resume();
    const outcome = ledger.admit("shell-boot", clock.leave());
    expect(performance.now() - wallStartedAt).toBeGreaterThanOrEqual(450);
    expect(outcome.verdict).toBe("admitted");
    expect(outcome.executingMs).toBeLessThan(WORKER_STEP_BUDGET_MS);
    expect(ledger.snapshot().recordedOverruns).toBe(0);
    expect(ledger.degraded()).toBe(false);
  });

  it("chunks the plugin graph so no single chunk needs the ceiling, and answers exactly what the one-turn resolver answers", () => {
    const planner = new PlaygroundBootPlanner(PLUGIN_CATALOG, "generation3d");
    const chunks: { readonly stage: string; readonly executingMs: number }[] = [];
    for (;;) {
      const stage = planner.stage();
      const startedAt = performance.now();
      const more = planner.step();
      chunks.push({ stage, executingMs: performance.now() - startedAt });
      if (!more) break;
    }
    const plan = planner.finish();
    expect(chunks.length).toBeGreaterThanOrEqual(3);
    expect(chunks.some((chunk) => chunk.stage.startsWith("plugin-graph:rows"))).toBe(true);
    expect(chunks.some((chunk) => chunk.stage === "plugin-graph:closure")).toBe(true);
    expect(chunks.some((chunk) => chunk.stage === "plugin-graph:order")).toBe(true);
    for (const chunk of chunks) expect(chunk.executingMs).toBeLessThan(WORKER_STEP_BUDGET_MS);
    expect(PLUGIN_GRAPH_CHUNK_ROWS).toBeGreaterThan(0);
    expect(chunks.filter((chunk) => chunk.stage.startsWith("plugin-graph:rows"))).toHaveLength(Math.ceil((PLUGIN_CATALOG.plugins.length + PLUGIN_CATALOG.extensions.length) / PLUGIN_GRAPH_CHUNK_ROWS));
    const reference = resolvePlaygroundBoot(PLUGIN_CATALOG, "generation3d");
    expect(plan.plugins.map((entry) => entry.pluginId)).toEqual(reference.plugins.map((entry) => entry.pluginId));
    expect(plan.defaultAppId).toBe(reference.defaultAppId);
    expect(plan.dependencyErrors).toEqual(reference.dependencyErrors);
  });

  it("reports what every Rust bootstrap phase executed for, so a long phase is measurable instead of invisible", () => {
    const rust = readFileSync(BROWSER_WORKER_RS, "utf8");
    expect(rust).toContain("elapsed_us: u32");
    expect(rust).toContain("fn worker_now_ms() -> f64");
    for (const stage of ["font-atlas", "icon-atlas", "font-upload", "icon-upload", "plugin-parse", "shell-construct", "shell-boot", "runtime-ready"]) {
      expect(rust).toContain(`stage: "${stage}"`);
    }
    expect(readFileSync(FRAME_WORKER_TS, "utf8")).toContain("phaseUs=${step.elapsedUs}");
  });
});

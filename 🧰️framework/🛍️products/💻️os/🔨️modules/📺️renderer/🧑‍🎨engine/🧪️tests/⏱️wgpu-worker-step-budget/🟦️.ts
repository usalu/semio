import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { PlaygroundBootPlanner, PLUGIN_GRAPH_CHUNK_ROWS, resolvePlaygroundBoot } from "@semio-tech/framework";
import { SUSTAINED_TURN_OVERRUN_TURNS, TurnClock, TurnLedger, WORKER_STEP_BUDGET_MS } from "../../🎯️targets/🧊️wgpu/⏱️turn-budget/🟦️.ts";
import { PLUGIN_CATALOG } from "../../../../🔌️plugin/📇️registry/🟦️.ts";

const ENGINE_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const FRAME_WORKER_TS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🎞️frame-worker", "🟦️.ts");
const BROWSER_WORKER_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🌐️browser-worker", "🦀️.rs");

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

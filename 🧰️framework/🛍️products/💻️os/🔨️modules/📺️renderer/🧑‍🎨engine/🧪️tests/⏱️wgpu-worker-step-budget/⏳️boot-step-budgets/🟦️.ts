/** ⏳️ The boot step executing time, wall time, plugin graph chunk and bootstrap phase tests. */
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
import { expect, it } from "vitest";
import { PlaygroundBootPlanner, PLUGIN_GRAPH_CHUNK_ROWS, resolvePlaygroundBoot } from "@semio-tech/framework";
import { SUSTAINED_TURN_OVERRUN_TURNS, TurnClock, TurnLedger, WORKER_STEP_BUDGET_MS } from "@semio-tech/framework-renderer-wgpu/turn-budget";
import { readFrameWorkerDirectory } from "../../📨️browser-frame-transport/📖️frame-worker-source/🟦️.ts";
import { BROWSER_WORKER_RS, FRAME_WORKER_TS, catalog, spinMs, workerBootStep } from "../🧰️step-budget-fixtures/🟦️.ts";

/** 🚦 Registers the boot step executing and wall time budgets, the plugin graph chunking and the bootstrap phase attribution tests. */
export function registerBootStepBudgetTests(): void {
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
    const planner = new PlaygroundBootPlanner(catalog, "canvas");
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
    expect(chunks.filter((chunk) => chunk.stage.startsWith("plugin-graph:rows"))).toHaveLength(Math.ceil((catalog.plugins.length + catalog.extensions.length) / PLUGIN_GRAPH_CHUNK_ROWS));
    const reference = resolvePlaygroundBoot(catalog, "canvas");
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
    expect(readFrameWorkerDirectory(dirname(FRAME_WORKER_TS))).toContain("phaseUs=${step.elapsedUs}");
  });
}

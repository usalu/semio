/** 🧵️ The armed Worker task, ingress acknowledgement and attribution ceiling tests. */
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
import { expect, it } from "vitest";
import { SUSTAINED_TURN_OVERRUN_TURNS, WORKER_STEP_BUDGET_MS } from "@semio-tech/framework-renderer-wgpu/turn-budget";
import { FrameTurnScheduler, WorkerTurnTaskQueue } from "@semio-tech/framework-renderer-wgpu/frame-turn-scheduler";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { readFrameWorkerDirectory } from "../../📨️browser-frame-transport/📖️frame-worker-source/🟦️.ts";
import { BROWSER_WORKER_RS, FRAME_JOB_RS, FRAME_TURN_FIXTURE, FRAME_WORKER_TS, RENDERER_RS } from "../🧰️step-budget-fixtures/🟦️.ts";

/** 📥 Registers the armed Worker task, ingress before frame callback and Worker ceiling attribution tests. */
export function registerWorkerTaskAndIngressTests(): void {
  it("keeps an actual Worker task armed while the retained runtime frame remains pending", async () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly runtimeTurns: readonly { readonly phase: string; readonly requestFrame: boolean; readonly continueFrame: boolean }[];
      readonly nonRunnableTurns: readonly { readonly owner: string; readonly requestFrame: boolean; readonly continueFrame: false }[];
      readonly textIngressTurns: readonly { readonly phase: string; readonly committed: boolean; readonly retiring: boolean; readonly continueFrame: boolean }[];
      readonly expected: { readonly runtimePhases: readonly string[]; readonly nonRunnableCallbacks: readonly string[]; readonly runnableTextPhases: readonly string[] };
    };
    const workerSource = readFrameWorkerDirectory(dirname(FRAME_WORKER_TS));
    const turnSource = workerSource.slice(workerSource.indexOf("function runFrameTurn"), workerSource.indexOf("function answerIntrospection"));
    expect(turnSource).toContain("return result.continueFrame");
    expect(turnSource).not.toContain("return result.requestFrame");
    expect(workerSource).toContain('new WorkerTurnTaskQueue()');
    expect(workerSource).not.toContain('new FrameTurnScheduler((callback) => setTimeout(callback, 0)');
    const browserWorkerSource = readFileSync(BROWSER_WORKER_RS, "utf8").replace(/\s+/gu, "");
    const continuationStart = browserWorkerSource.indexOf("letcontinue_frame=");
    const continuationSource = browserWorkerSource.slice(continuationStart, browserWorkerSource.indexOf("letoutput=BrowserTickOutput{", continuationStart));
    expect(continuationSource).toContain("host.frame_build.has_live_session()");
    expect(continuationSource).not.toContain("next_deadline");
    expect(continuationSource).not.toContain("hub_status_pending");
    expect(browserWorkerSource).toContain("request_frame:continue_frame,");
    expect(browserWorkerSource).toContain("next_deadline_delay_ms:host.scheduler.next_deadline()");
    expect(turnSource).toContain("nextDeadlineDelayMs: result.nextDeadlineDelayMs");
    const rendererSource = readModuleTreeSource(RENDERER_RS);
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
    const worker = readFrameWorkerDirectory(dirname(FRAME_WORKER_TS));
    const handler = worker.slice(worker.indexOf('ownedStep("frame-ingress"'), worker.indexOf("function runFrameTurn"));
    expect(handler).toContain('post({ kind: "batch-accepted"');
    expect(handler).toContain("frameTurns?.request()");
    expect(handler).not.toContain("originalFrameSource!.tick(");
    expect(worker.slice(worker.indexOf("function runFrameTurn"), worker.indexOf("function answerIntrospection"))).toContain("originalFrameSource!.tick(");
    const frameJob = readFileSync(FRAME_JOB_RS, "utf8");
    const originalOwner = frameJob.slice(frameJob.indexOf("pub fn poll_runtime_on_original_caller"), frameJob.indexOf("    /// ♻️ Returns pending admission and checked-out custody"));
    expect(originalOwner).toContain("try_step_on_original_caller");
    expect(originalOwner).not.toContain("try_step_on_caller");
    expect(originalOwner).not.toContain("BROWSER_FRAME_BUILD_DRIVE_US");
    expect(originalOwner).not.toContain("loop {");
  });
  it("pins the Worker ceiling and the attribution law it shares with the UI isolate and the guest", () => {
    expect(WORKER_STEP_BUDGET_MS).toBe(8);
    expect(SUSTAINED_TURN_OVERRUN_TURNS).toBe(4);
    const worker = readFrameWorkerDirectory(dirname(FRAME_WORKER_TS));
    expect(worker).toContain('new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step")');
    expect(worker).not.toContain("worker-boot-step-overrun");
    expect(worker).not.toContain("interactive-job-overrun");
    expect(worker).not.toContain("worker-close-overrun");
    expect(worker).not.toMatch(/>= *WORKER_STEP_BUDGET_MS/);
  });
}

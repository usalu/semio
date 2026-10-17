/** ⏱️ Retained clock deadlines: the retained browser clock deadline laws for animation, control and input timers against the shared render deadline fixture. */
import { fixtureOriginalWorkStock } from "../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { BrowserFrameTransport } from "../../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { uiRenderDeadlineFixtureUrl } from "../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts";
import { FakeWorker, TEST_HOST_APPEARANCE, TEST_HOST_PLATFORM, TEST_HOST_STORAGE, testBootDescriptor } from "../🧰️transport-test-support/🟦️.ts";

describe("retained browser clock deadlines", () => {
  function harness() {
    const worker = new FakeWorker();
    const raf = new Map<number, FrameRequestCallback>();
    let rafSequence = 0;
    let frameSequence = 0;
    const setTimer = vi.fn((callback: () => void, delay: number) => setTimeout(callback, delay) as unknown as number);
    const clearTimer = vi.fn((handle: number) => clearTimeout(handle));
    const subject = new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer_bg.wasm", canvas: {} as OffscreenCanvas, width: 8, height: 8, dpr: 1, locale: "en", descriptor: testBootDescriptor("s"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE, catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] } },
      now: () => Date.now(),
      setTimer,
      clearTimer,
      requestAnimationFrame: callback => { raf.set(++rafSequence, callback); return rafSequence; },
      cancelAnimationFrame: handle => { raf.delete(handle); },
    });
    const flushRaf = (acknowledge = true) => {
      const callbacks = [...raf.values()];
      raf.clear();
      for (const callback of callbacks) callback(Date.now());
      const batch = worker.messages.at(-1);
      if (acknowledge && batch?.kind === "batch") worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: batch.inputSequence, generation: batch.generation });
    };
    worker.reply({ kind: "booted", lifecycle: 1 });
    flushRaf();
    const reply = (delay: number | null, generation = 0) => worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: ++frameSequence, generation, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: delay, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    return { worker, subject, raf, flushRaf, reply, setTimer, clearTimer };
  }

  const fixture = JSON.parse(readFileSync(uiRenderDeadlineFixtureUrl, "utf8")) as { cases: { id: string; steps: ({ key: number; dueMs: number | null } | { nowMs: number; fired: boolean; nextMs: number | null })[] }[] };
  for (const row of fixture.cases.filter(row => row.steps.every(step => !("key" in step) || step.key === 1))) it(`waits without rAF and replays ${row.id}`, () => {
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const context = harness();
    try {
      for (const step of row.steps) {
        if ("key" in step) context.reply(step.dueMs === null ? null : step.dueMs - Date.now());
        else {
          vi.advanceTimersByTime(step.nowMs - Date.now());
          expect(context.raf.size > 0, `${row.id}: ${step.nowMs}`).toBe(step.fired);
          context.flushRaf();
        }
      }
    } finally {
      context.subject.close();
      vi.useRealTimers();
    }
  });

  it("revokes animation and expired control timers while a presenter checkout waits for handback", () => {
    const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json"), "utf8"));
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const context = harness();
    try {
      const batches = () => context.worker.messages.filter(message => message.kind === "batch").length;
      context.reply(fixture.presenterWait.deadlineTurns[0].nextMs);
      context.reply(null);
      const before = batches();
      vi.advanceTimersByTime(3000);
      context.flushRaf();
      expect(batches()).toBe(before);
      expect(context.raf.size).toBe(0);
      context.worker.reply({ kind: "wake", lifecycle: 1 });
      context.flushRaf();
      expect(batches()).toBe(before + 1);
      context.reply(0);
      vi.advanceTimersByTime(0);
      expect(context.raf.size).toBe(1);
      const source = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../../🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs"), "utf8");
      const snapshot = source.slice(source.indexOf("fn present_snapshot("), source.indexOf("fn semio_cursor_to_request("));
      expect(snapshot).toContain("sync_presented_deadlines(");
      expect(snapshot).toContain("self.presenter.awaiting_runtime()");
      expect(snapshot).toMatch(/#\[cfg\(not\(target_arch = "wasm32"\)\)\]\s*if self.hot_swap.is_due/);
    } finally {
      context.subject.close();
      vi.useRealTimers();
    }
  });

  it("cancels a pending wake for input and ignores a stale frame's deadline", () => {
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const context = harness();
    try {
      context.reply(600);
      context.subject.enqueueLossless({ kind: "text", text: "x" });
      context.flushRaf();
      context.reply(1, 0);
      context.flushRaf();
      context.reply(800, 1);
      vi.advanceTimersByTime(600);
      expect(context.raf.size).toBe(0);
      vi.advanceTimersByTime(200);
      expect(context.raf.size).toBe(1);
    } finally {
      context.subject.close();
      vi.useRealTimers();
    }
  });

  it("retains a due timer wake until the in-flight input batch is acknowledged", () => {
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const context = harness();
    try {
      context.subject.requestFrame();
      context.flushRaf(false);
      const batch = context.worker.messages.at(-1);
      if (batch?.kind !== "batch") throw new Error("input batch");
      const batches = () => context.worker.messages.filter(message => message.kind === "batch").length;
      expect(batches()).toBe(2);
      context.reply(10);
      vi.advanceTimersByTime(10);
      expect(context.raf.size).toBe(1);
      context.flushRaf(false);
      expect(batches()).toBe(2);
      expect(context.raf.size).toBe(0);
      context.worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: batch.inputSequence, generation: batch.generation });
      expect(context.raf.size).toBe(1);
      context.flushRaf();
      expect(batches()).toBe(3);
    } finally {
      context.subject.close();
      vi.useRealTimers();
    }
  });

  for (const terminal of ["close", "fault", "quarantine"] as const) it(`retires its timer on ${terminal}`, () => {
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const context = harness();
    try {
      context.reply(600);
      const timer = context.setTimer.mock.results.at(-1)!.value;
      if (terminal === "close") context.subject.close();
      if (terminal === "fault") context.worker.reply({ kind: "fault", lifecycle: 1, code: "test", detail: "test" });
      if (terminal === "quarantine") context.worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 2, generation: 0, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted", quarantined: true, faultCode: "present-failed" });
      expect(context.clearTimer).toHaveBeenCalledWith(timer);
      vi.advanceTimersByTime(600);
      expect(context.raf.size).toBe(0);
    } finally {
      context.subject.close();
      vi.useRealTimers();
    }
  });
});

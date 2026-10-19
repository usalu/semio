/** 🧱️ Generations, faults and hooks: the pointer slot authority, stale generations, close, protocol faults, overruns, the external UI hooks and the structured clone budget. */
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import { describe, expect, it } from "vitest";
import { BrowserFrameTransport } from "@semio-tech/framework-renderer-wgpu/browser-frame-transport";
import { FRAME_WORKER_POINTER_CAPACITY } from "@semio-tech/framework-renderer-wgpu/browser-frame-transport/contract";
import { FakeWorker, TEST_HOST_APPEARANCE, TEST_HOST_PLATFORM, TEST_HOST_STORAGE, testBootDescriptor, transport } from "../🧰️transport-test-support/🟦️.ts";

describe("browser frame worker transport", () => {
  it("fails closed when distinct pointer identities exceed fixed slot authority", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    for (let pointerId = 0; pointerId < FRAME_WORKER_POINTER_CAPACITY; pointerId++) {
      expect(subject.enqueueReplaceable({ kind: "pointer-move", pointerId, pointerKind: "touch", x: pointerId, y: 0, shift: false, ctrl: false, alt: false, meta: false })).toBe(true);
    }
    expect(subject.enqueueReplaceable({ kind: "pointer-move", pointerId: FRAME_WORKER_POINTER_CAPACITY, pointerKind: "touch", x: 0, y: 0, shift: false, ctrl: false, alt: false, meta: false })).toBe(false);
    expect(subject.fault?.code).toBe("replaceable-overflow");
  });

  it("rejects stale generations and presents only the current generation", () => {
    const worker = new FakeWorker();
    const directives: number[] = [];
    const subject = transport(worker, { directives });
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.enqueueLossless({ kind: "text", text: "a" });
    subject.flush(1);
    subject.enqueueLossless({ kind: "text", text: "b" });
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: 1, generation: 1 });
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 1, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(directives).toEqual([]);
    expect(subject.flush(2)).toBe(true);
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 2, generation: 2, cursor: "text", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(directives).toEqual([2]);
  });

  it("terminates on close and rejects late Worker messages", () => {
    const worker = new FakeWorker();
    const directives: number[] = [];
    const subject = transport(worker, { directives });
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.close();
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 1, generation: 0, cursor: "pointer", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(subject.status).toBe("closed");
    expect(worker.terminated).toBe(false);
    worker.reply({ kind: "closed", lifecycle: 1 });
    expect(worker.terminated).toBe(true);
    expect(directives).toEqual([]);
  });

  it("faults instead of deadlocking on a protocol-corrupt future generation", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.flush(1);
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 1, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(subject.status).toBe("faulted");
    expect(subject.fault?.code).toBe("protocol-violation");
  });

  it("maps Worker faults according to lifecycle phase", () => {
    const bootWorker = new FakeWorker();
    const bootSubject = transport(bootWorker);
    bootWorker.reply({ kind: "fault", lifecycle: 1, code: "module", detail: "missing" });
    expect(bootSubject.fault?.code).toBe("worker-boot-failed");

    const runtimeWorker = new FakeWorker();
    const runtimeSubject = transport(runtimeWorker);
    runtimeWorker.reply({ kind: "booted", lifecycle: 1 });
    runtimeWorker.reply({ kind: "fault", lifecycle: 1, code: "runtime", detail: "broken" });
    expect(runtimeSubject.fault?.code).toBe("worker-runtime-failed");
  });

  it("quarantines a Worker overrun without terminating the last valid surface owner", () => {
    const worker = new FakeWorker();
    const directives: number[] = [];
    const subject = transport(worker, { directives });
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.flush(1);
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 1, generation: 0, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 40, workerExecutingMs: 9, workerStepVerdict: "sustained-overrun", quarantined: true, faultCode: "worker-step-overrun", faultDetail: "frame step executed 9.000 ms for 4 consecutive steps" });
    expect(subject.status).toBe("quarantined");
    expect(worker.terminated).toBe(false);
    expect(directives).toEqual([]);
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 2, generation: 0, cursor: "pointer", fullscreen: true, requestFrame: true, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(directives).toEqual([]);
  });

  it("measures external UI hooks centrally and keeps the surface alive on hook overrun", () => {
    const worker = new FakeWorker();
    const overruns: string[] = [];
    let now = 0;
    const subject = new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, locale: "en", descriptor: testBootDescriptor("s"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE, catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] } },
      now: () => now,
      setTimer: () => 1,
      clearTimer: () => {},
      onReady: () => { now += 50; },
      onUiTurn: (outcome) => overruns.push(`${outcome.verdict}:${outcome.site}`),
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.status).toBe("ready");
    expect(subject.fault).toBeUndefined();
    expect(overruns).toEqual(["recorded-overrun:ready-hook"]);
    expect(subject.flush()).toBe(true);
    expect(worker.messages.filter((message) => message.kind === "batch")).toHaveLength(1);
  });

  it("fails the surface only when a UI hook throws, under its own fault code", () => {
    const worker = new FakeWorker();
    const faults: string[] = [];
    const subject = new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, locale: "en", descriptor: testBootDescriptor("s"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE, catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] } },
      now: () => 0,
      setTimer: () => 1,
      clearTimer: () => {},
      onReady: () => { throw new Error("hook exploded"); },
      onFault: (code) => faults.push(code),
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.status).toBe("faulted");
    expect(subject.fault?.code).toBe("ui-hook-failed");
    expect(faults).toEqual(["ui-hook-failed"]);
  });

  it("records rather than fails when the bounded structured clone consumes the UI turn budget", () => {
    const worker = new FakeWorker();
    let now = 0;
    const original = worker.postMessage.bind(worker);
    worker.postMessage = (message) => {
      original(message);
      if (message.kind === "batch") now += 50;
    };
    const subject = new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, locale: "en", descriptor: testBootDescriptor("s"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE, catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] } },
      now: () => now,
      setTimer: () => 1,
      clearTimer: () => {},
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.flush()).toBe(true);
    expect(subject.fault).toBeUndefined();
    expect(subject.status).toBe("ready");
    expect(subject.fallbackState().uiTurns.recordedOverruns).toBeGreaterThan(0);
  });
});

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import {
  BrowserFrameTransport,
  FRAME_WORKER_ACCESSIBILITY_ID_BYTES,
  FRAME_WORKER_BYTE_CAPACITY,
  FRAME_WORKER_INTROSPECTION_CAPACITY,
  FRAME_WORKER_HUB_DOCUMENT_CAPACITY,
  FRAME_WORKER_IMAGE_DECODE_BYTE_CAPACITY,
  FRAME_WORKER_LOSSLESS_ITEM_CAPACITY,
  FRAME_WORKER_MESSAGE_BYTE_CAPACITY,
  FRAME_WORKER_POINTER_CAPACITY,
  type BrowserFrameFallbackState,
  type BrowserFrameUiMessage,
  type BrowserFrameWorkerMessage,
  type BrowserFrameWorkerPort,
} from "../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import {
  FRAME_WORKER_BOOT_LIVENESS_POLICY,
  bootPhaseCeilingMs,
  describeBrowserBootPhase,
  describeBrowserBootSilence,
  evaluateBrowserBootLiveness,
} from "../../🎯️targets/🧊️wgpu/🫀️boot-liveness/🟦️.ts";
import { evictCachedRendererModule, readCachedRendererModule, rendererArtifactTag, writeCachedRendererModule } from "../../🎯️targets/🧊️wgpu/🗄️wasm-module-cache/🟦️.ts";
import { resolveWgpuBootDescriptor, resolveWgpuHostPlatform, type WgpuBootDescriptor, type WgpuHostAppearance, type WgpuHostStorageSnapshot } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";

/** @emoji 🧭️ One resolved boot descriptor for a fixture transport — the shared resolver, never a hand
 * rolled literal, so these fixtures cannot drift from the shape the three real doors produce
 * (`🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts`). */
function testBootDescriptor(variant: string): WgpuBootDescriptor {
  return resolveWgpuBootDescriptor({ defaultVariant: variant });
}

/** @emoji 🌓️ The appearance a realm that read nothing publishes — React's own no-window default. */
const TEST_HOST_APPEARANCE: WgpuHostAppearance = { preference: "", systemDark: false };
const TEST_HOST_PLATFORM = "MacIntel";
/** @emoji 🗄️ The storage snapshot a realm that persisted nothing publishes. */
const TEST_HOST_STORAGE: WgpuHostStorageSnapshot = {};

class FakeWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];
  readonly transfers: Transferable[][] = [];
  terminated = false;

  postMessage(message: BrowserFrameUiMessage, transfer: Transferable[] = []): void {
    this.messages.push(message);
    this.transfers.push(transfer);
  }

  terminate(): void {
    this.terminated = true;
  }

  reply(message: BrowserFrameWorkerMessage): void {
    this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>);
  }
}

function transport(worker: FakeWorker, hooks: { directives?: number[]; faults?: string[]; turns?: string[]; now?: () => number } = {}): BrowserFrameTransport {
  return new BrowserFrameTransport({
    worker,
    boot: {
      bindingsModuleUrl: "renderer.js",
      bindingsWasmUrl: "renderer_bg.wasm",
      canvas: {} as OffscreenCanvas,
      width: 800,
      height: 600,
      dpr: 2,
      locale: "en",
      descriptor: testBootDescriptor("s"),
      appearance: TEST_HOST_APPEARANCE,
      platform: TEST_HOST_PLATFORM,
      storage: TEST_HOST_STORAGE,
    },
    setTimer: () => 1,
    clearTimer: () => {},
    now: hooks.now,
    onDirectives: (value) => hooks.directives?.push(value.generation),
    onUiTurn: (outcome) => hooks.turns?.push(`${outcome.verdict}:${outcome.site}`),
    onFault: (code) => hooks.faults?.push(code),
  });
}

describe("browser frame worker transport", () => {
  it("posts one transferable boot and remains fail-closed until the Worker acknowledges", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    expect(subject.status).toBe("booting");
    expect(worker.messages[0]?.kind).toBe("boot");
    expect(subject.flush()).toBe(false);
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.status).toBe("ready");
    expect(subject.flush(10)).toBe(true);
  });

  it("coalesces pointer, wheel, and resize storms into one bounded batch", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.enqueueReplaceable({ kind: "pointer-move", pointerId: 7, pointerKind: "mouse", x: 1, y: 2, shift: true, ctrl: false, alt: false, meta: false });
    subject.enqueueReplaceable({ kind: "pointer-move", pointerId: 7, pointerKind: "mouse", x: 3, y: 4, shift: false, ctrl: false, alt: false, meta: false });
    subject.enqueueReplaceable({ kind: "wheel", x: 3, y: 4, deltaX: 1, deltaY: 2, shift: false, ctrl: true, alt: false, meta: false });
    subject.enqueueReplaceable({ kind: "wheel", x: 4, y: 5, deltaX: 3, deltaY: 4, shift: false, ctrl: false, alt: true, meta: false });
    subject.enqueueReplaceable({ kind: "resize", width: 10, height: 20, dpr: 1 });
    subject.enqueueReplaceable({ kind: "resize", width: 30, height: 40, dpr: 2 });
    expect(subject.flush(11)).toBe(true);
    const batch = worker.messages.at(-1);
    expect(batch?.kind).toBe("batch");
    if (batch?.kind !== "batch") return;
    expect(batch.replaceable).toHaveLength(3);
    expect(batch.replaceable).toContainEqual(expect.objectContaining({ kind: "pointer-move", x: 3, y: 4, shift: false }));
    expect(batch.replaceable).toContainEqual(expect.objectContaining({ kind: "wheel", x: 4, y: 5, deltaX: 4, deltaY: 6, ctrl: false, alt: true }));
    expect(batch.replaceable).toContainEqual(expect.objectContaining({ kind: "resize", width: 30, height: 40, dpr: 2 }));
  });

  it("releases the input lease on batch acknowledgement and orders multiple frame turns independently", () => {
    const worker = new FakeWorker();
    const directives: number[] = [];
    const subject = transport(worker, { directives });
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.enqueueLossless({ kind: "text", text: "a" });
    expect(subject.flush(1)).toBe(true);
    const first = worker.messages.at(-1);
    if (first?.kind !== "batch") throw new Error("first input batch");
    expect(subject.flush(2)).toBe(false);
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: first.inputSequence, generation: first.generation });
    subject.requestFrame();
    expect(subject.flush(2)).toBe(true);
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 1, generation: first.generation, cursor: "default", fullscreen: null, requestFrame: false, progress: 0.5, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 2, generation: first.generation, cursor: "text", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(directives).toEqual([first.generation, first.generation]);
  });

  it("coalesces Hub status by document, refuses only a new key at capacity, and releases admission after transfer", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    const key = "hub:space-a/document-a";
    expect(subject.publishHubDocumentStatus(key, { kind: "connecting" })).toBe(true);
    expect(subject.publishHubDocumentStatus(key, { kind: "live", peerCount: 4 })).toBe(true);
    expect(subject.publishHubDocumentStatus(key, null)).toBe(true);
    for (let index = 1; index < FRAME_WORKER_HUB_DOCUMENT_CAPACITY; index++) expect(subject.publishHubDocumentStatus(`hub:space-${index}/document`, { kind: "connecting" })).toBe(true);
    const refused = "hub:overflow/document";
    expect(subject.publishHubDocumentStatus(refused, { kind: "live", peerCount: 9 })).toBe(false);
    expect(subject.status).toBe("ready");
    expect(subject.flush(1)).toBe(true);
    const first = worker.messages.at(-1);
    expect(first?.kind).toBe("batch");
    if (first?.kind !== "batch") return;
    expect(first.lossless.filter((event) => "documentKey" in event && event.documentKey === key)).toEqual([expect.objectContaining({ kind: "hub-document-close" })]);
    expect(subject.publishHubDocumentStatus(refused, { kind: "live", peerCount: 9 })).toBe(true);
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: first.inputSequence, generation: first.generation });
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: first.inputSequence, generation: first.generation, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    const delivered: unknown[] = [...first.lossless];
    let sequence = first.inputSequence;
    while (subject.flush(++sequence)) {
      const batch = worker.messages.at(-1);
      if (batch?.kind !== "batch") break;
      delivered.push(...batch.lossless);
      worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: batch.inputSequence, generation: batch.generation });
      worker.reply({ kind: "frame", lifecycle: 1, frameSequence: batch.inputSequence, generation: batch.generation, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    }
    expect(delivered.filter((event) => typeof event === "object" && event !== null && "documentKey" in event && event.documentKey === key)).toEqual([expect.objectContaining({ kind: "hub-document-close" })]);
    expect(delivered).toContainEqual(expect.objectContaining({ kind: "hub-document-status", documentKey: refused, remote: { kind: "live", peerCount: 9 } }));
  });

  it("fails closed rather than dropping a lossless event when item credits are exhausted", () => {
    const worker = new FakeWorker();
    const faults: string[] = [];
    const subject = transport(worker, { faults });
    for (let index = 0; index < FRAME_WORKER_LOSSLESS_ITEM_CAPACITY; index++) {
      expect(subject.enqueueLossless({ kind: "text", text: "x" })).toBe(true);
    }
    expect(subject.enqueueLossless({ kind: "text", text: "overflow" })).toBe(false);
    expect(subject.status).toBe("faulted");
    expect(faults).toEqual(["lossless-overflow"]);
    expect(worker.terminated).toBe(false);
    expect(worker.messages.at(-1)?.kind).toBe("close");
  });

  it("transfers node-addressed accessibility focus activation and value events exactly once", () => {
    const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json"), "utf8")) as { readonly events: readonly { readonly wire: Parameters<BrowserFrameTransport["enqueueLossless"]>[0] }[] };
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    for (const row of fixture.events) expect(subject.enqueueLossless(row.wire)).toBe(true);
    expect(subject.flush(23)).toBe(true);
    const batch = worker.messages.at(-1);
    expect(batch?.kind).toBe("batch");
    if (batch?.kind !== "batch") return;
    expect(batch.lossless.map((event) => {
      const { timestampMs: _, ...wire } = event as typeof event & { readonly timestampMs: number };
      return wire;
    })).toEqual(fixture.events.map((row) => row.wire));
    for (const event of batch.lossless) {
      if (!event.kind.startsWith("accessibility-")) continue;
      expect(event).not.toHaveProperty("x");
      expect(event).not.toHaveProperty("y");
    }
  });

  it("rejects accessibility addresses and values outside their fixed transport credits", () => {
    const address = { windowId: "procedural-main", windowGeneration: 7, nodeId: 2, nodeKey: "#apply" };
    for (const event of [
      { kind: "accessibility-focus" as const, ...address, nodeKey: "é".repeat(FRAME_WORKER_ACCESSIBILITY_ID_BYTES / 2 + 1) },
      { kind: "accessibility-activate" as const, ...address, windowId: "bad\u0000window" },
      { kind: "accessibility-value" as const, ...address, value: "x".repeat(1025) },
    ]) {
      const worker = new FakeWorker();
      const subject = transport(worker);
      expect(subject.enqueueLossless(event)).toBe(false);
      expect(subject.fault?.code).toBe("lossless-overflow");
      expect(worker.messages.map((message) => message.kind)).toEqual(["boot", "close"]);
    }
  });

  it("fails closed on byte-credit exhaustion", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    expect(subject.enqueueLossless({ kind: "paste", text: "x".repeat(FRAME_WORKER_BYTE_CAPACITY / 2) })).toBe(false);
    expect(subject.fault?.code).toBe("lossless-overflow");
  });

  it("rejects oversized IME before transfer so the Worker never sees a nonfinal composition", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    expect(subject.enqueueLossless({ kind: "ime-update", text: "x".repeat(1025), cursor: 1025 })).toBe(false);
    expect(subject.fault?.code).toBe("lossless-overflow");
    expect(worker.messages.map((message) => message.kind)).toEqual(["boot", "close"]);
  });

  it("chunks a near-256-KiB paste so every UI structured clone stays hard bounded", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.enqueueLossless({ kind: "paste", text: "x".repeat(Math.floor((FRAME_WORKER_BYTE_CAPACITY - 256) / 3)) })).toBe(true);
    expect(subject.flush(1)).toBe(true);
    const batch = worker.messages.at(-1);
    expect(batch?.kind).toBe("batch");
    if (batch?.kind !== "batch") return;
    expect(batch.lossless.length).toBeGreaterThan(0);
    expect(batch.lossless.length).toBeLessThanOrEqual(16);
    expect(batch.lossless[0]).toEqual(expect.objectContaining({ kind: "text-chunk", target: "paste", final: false }));
    expect(JSON.stringify(batch).length).toBeLessThan(FRAME_WORKER_MESSAGE_BYTE_CAPACITY);
  });

  it("preserves the image-data-url paste target through the bounded text stream", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.enqueueLossless({ kind: "paste-image-data-url", text: "data:image/png;base64,iVBORw0KGgo=" })).toBe(true);
    expect(subject.flush(1)).toBe(true);
    const batch = worker.messages.at(-1);
    expect(batch?.kind).toBe("batch");
    if (batch?.kind !== "batch") return;
    expect(batch.lossless).toContainEqual(expect.objectContaining({ kind: "text-chunk", target: "paste-image-data-url", final: true }));
  });

  it("never slices an astral code point across Worker messages", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    const text = `${"x".repeat(1023)}🚀${"y".repeat(1024)}`;
    expect(subject.enqueueLossless({ kind: "paste", text })).toBe(true);
    expect(subject.flush(1)).toBe(true);
    const batch = worker.messages.at(-1);
    if (batch?.kind !== "batch") return;
    const chunk = batch.lossless.find((event) => event.kind === "text-chunk");
    expect(chunk?.kind).toBe("text-chunk");
    if (chunk?.kind === "text-chunk") expect(chunk.text.endsWith("\ud83d")).toBe(false);
  });

  it("keeps worst-case JSON escaping within the message clone cap", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.enqueueLossless({ kind: "paste", text: "\u0000\\\"".repeat(1024) })).toBe(true);
    expect(subject.flush(1)).toBe(true);
    const batch = worker.messages.at(-1);
    if (batch?.kind === "batch") expect(JSON.stringify(batch).length).toBeLessThan(FRAME_WORKER_MESSAGE_BYTE_CAPACITY);
  });

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
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 1, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(directives).toEqual([]);
    expect(subject.flush(2)).toBe(true);
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 2, generation: 2, cursor: "text", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(directives).toEqual([2]);
  });

  it("terminates on close and rejects late Worker messages", () => {
    const worker = new FakeWorker();
    const directives: number[] = [];
    const subject = transport(worker, { directives });
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.close();
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 1, generation: 0, cursor: "pointer", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
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
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 1, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
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
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 1, generation: 0, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 40, workerExecutingMs: 9, workerStepVerdict: "sustained-overrun", quarantined: true, faultCode: "worker-step-overrun", faultDetail: "frame step executed 9.000 ms for 4 consecutive steps" });
    expect(subject.status).toBe("quarantined");
    expect(worker.terminated).toBe(false);
    expect(directives).toEqual([]);
    worker.reply({ kind: "frame", lifecycle: 1, frameSequence: 2, generation: 0, cursor: "pointer", fullscreen: true, requestFrame: true, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    expect(directives).toEqual([]);
  });

  it("measures external UI hooks centrally and keeps the surface alive on hook overrun", () => {
    const worker = new FakeWorker();
    const overruns: string[] = [];
    let now = 0;
    const subject = new BrowserFrameTransport({
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, locale: "en", descriptor: testBootDescriptor("s"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE },
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
    const subject = new BrowserFrameTransport({
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, locale: "en", descriptor: testBootDescriptor("s"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE },
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
    const subject = new BrowserFrameTransport({
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, locale: "en", descriptor: testBootDescriptor("s"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE },
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

  it("routes an introspection dump across the Worker seam behind a flushed frame", async () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    const pending = subject.introspect("structure");
    expect(worker.messages.map((message) => message.kind)).toEqual(["boot", "batch", "introspect"]);
    const request = worker.messages[2] as Extract<BrowserFrameUiMessage, { kind: "introspect" }>;
    expect(request.probe).toBe("structure");
    worker.reply({ kind: "introspection", lifecycle: 1, requestId: request.requestId, probe: "structure", json: '{"nodes":[]}' });
    await expect(pending).resolves.toBe('{"nodes":[]}');
  });

  it("answers null rather than faulting when introspection is unavailable, over-subscribed, or abandoned", async () => {
    const worker = new FakeWorker();
    const faults: string[] = [];
    const subject = transport(worker, { faults });
    await expect(subject.introspect("structure")).resolves.toBeNull();
    worker.reply({ kind: "booted", lifecycle: 1 });
    const inFlight = Array.from({ length: FRAME_WORKER_INTROSPECTION_CAPACITY }, () => subject.introspect("frame-stats"));
    await expect(subject.introspect("frame-stats")).resolves.toBeNull();
    subject.close();
    for (const answer of inFlight) await expect(answer).resolves.toBeNull();
    expect(faults).toEqual([]);
  });

  it("admits one page image decode, refuses a concurrent source, and cancels the exact live request", async () => {
    let rejectDecode: ((error: Error) => void) | undefined;
    class PendingImage {
      decoding = "auto";
      private value = "";
      get src(): string { return this.value; }
      set src(value: string) {
        this.value = value;
        if (value === "") rejectDecode?.(new DOMException("cancelled", "AbortError"));
      }
      decode(): Promise<void> {
        return new Promise((_, reject) => { rejectDecode = reject; });
      }
    }
    vi.stubGlobal("Image", PendingImage);
    vi.spyOn(URL, "createObjectURL").mockReturnValue("blob:reference");
    vi.spyOn(URL, "revokeObjectURL").mockImplementation(() => {});
    try {
      const worker = new FakeWorker();
      const subject = transport(worker);
      worker.reply({ kind: "booted", lifecycle: 1 });
      const dimensions = { width: 2, height: 3, orientation: 1, mediaType: "image/svg+xml" as const };
      worker.reply({ kind: "image-decode", lifecycle: 1, requestId: 7, source: new Uint8Array([1]), dimensions });
      worker.reply({ kind: "image-decode", lifecycle: 1, requestId: 8, source: new Uint8Array(FRAME_WORKER_IMAGE_DECODE_BYTE_CAPACITY + 1), dimensions });
      await vi.waitFor(() => expect(worker.messages.some((message) => message.kind === "image-decode-result" && message.requestId === 8 && message.bitmap === null && message.detail === "reference-image-decode-credits")).toBe(true));
      worker.reply({ kind: "image-decode-cancel", lifecycle: 1, requestId: 7 });
      await vi.waitFor(() => expect(worker.messages.some((message) => message.kind === "image-decode-result" && message.requestId === 7 && message.bitmap === null && message.detail?.startsWith("AbortError:") === true)).toBe(true));
      subject.close();
    } finally {
      vi.unstubAllGlobals();
      vi.restoreAllMocks();
    }
  });

  /** ♿️ LAW: the two host channels are not interchangeable. `onDirectives` is the FRAME channel and
   * fires for every accepted frame; `onUiTurn` is the BUDGET channel and fires only for a turn that
   * breached its ceiling. A healthy shell overruns once at boot and never again, so anything hung on
   * `onUiTurn` runs exactly once in a healthy session — which is how the ARIA mirror came to carry
   * whatever a single boot-time race captured (`nodeCount 0` on 6118). */
  it("raises the frame channel for every accepted frame while the budget channel stays silent", () => {
    const worker = new FakeWorker();
    const directives: number[] = [];
    const turns: string[] = [];
    const subject = transport(worker, { directives, turns, now: () => 0 });
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.enqueueLossless({ kind: "text", text: "x" });
    for (let sequence = 1; sequence <= 5; sequence++) {
      subject.requestFrame();
      expect(subject.flush(sequence)).toBe(true);
      worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: sequence, generation: 1 });
      worker.reply({ kind: "frame", lifecycle: 1, frameSequence: sequence, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    }
    expect(directives).toEqual([1, 1, 1, 1, 1]);
    expect(turns).toEqual([]);
  });

  /** ♿️ LAW: the ARIA mirror is driven by the frame channel, and the pull never runs inside the hook. */
  it("refreshes the accessibility mirror from the frame channel, never from the overrun channel", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8");
    const mirrorSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts"), "utf8");
    const directiveHook = bootSource.slice(bootSource.indexOf("onDirectives: ({ cursor, fullscreen })"), bootSource.indexOf("onFault:"));
    const turnHook = bootSource.slice(bootSource.indexOf("onUiTurn: (outcome)"), bootSource.indexOf("onReady: () =>"));
    expect(directiveHook).toContain("accessibility?.refresh()");
    expect(turnHook).not.toContain("accessibility");
    const refresh = mirrorSource.slice(mirrorSource.indexOf("const refresh = (): void =>"), mirrorSource.indexOf("return { refresh,"));
    expect(refresh).toContain("window.setTimeout");
    expect(refresh).not.toMatch(/\n\s*void pull\(\);\n\s*return;/);
    expect(mirrorSource).toContain("paint(dump.windows ?? [])");
    expect(mirrorSource).toContain('element.dataset.window = surface.windowId');
    expect(mirrorSource).toContain("parent.appendChild(projected.description)");
    expect(mirrorSource).not.toContain("element.appendChild(description)");
  });

  it("wakes the frame owner after a handled reference refusal so the next asset is polled", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const workerSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");
    const pump = workerSource.slice(workerSource.indexOf("async function pumpAsset"), workerSource.indexOf("function progress"));
    const caught = pump.slice(pump.lastIndexOf("  } catch (error) {"), pump.lastIndexOf("  } finally {"));
    const reject = caught.indexOf('ownedStep("asset-reject"');
    const wake = caught.indexOf('post({ kind: "wake", lifecycle })');
    expect(reject).toBeGreaterThan(-1);
    expect(wake).toBeGreaterThan(reject);
    expect(caught.match(/post\(\{ kind: "wake", lifecycle \}\)/g)).toHaveLength(1);
    expect(caught).toContain("activeRequest?.referenceImage && !closing && !closed && !failed");
  });

  it("streams decoded references as cancellable row-aligned strips before the exact seal", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const workerSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");
    const decoderSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts"), "utf8");
    const rustSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"), "utf8");
    const pump = workerSource.slice(workerSource.indexOf("async function pumpAsset"), workerSource.indexOf("function progress"));
    const begin = pump.indexOf('ownedStep("reference-image-begin"');
    const stage = pump.indexOf("referenceImageBitmapForStage(");
    const push = pump.indexOf("streamReferenceImageBitmapRows(");
    const seal = pump.indexOf('ownedStep("reference-image-seal"');
    expect(begin).toBeGreaterThan(-1);
    expect(stage).toBeGreaterThan(begin);
    expect(push).toBeGreaterThan(stage);
    expect(seal).toBeGreaterThan(push);
    expect(pump).toContain("stageMode !== 1 && stageMode !== 2");
    expect(workerSource).not.toContain("ReferenceImageDecodeCache");
    expect(pump.indexOf('ownedStep("reference-image-begin"')).toBeLessThan(pump.indexOf('monitoredSuspension("reference-image-decode"'));
    expect(pump).toContain('monitoredSuspension("reference-image-page-decode"');
    expect(pump.match(/streamReferenceImageBitmapRows\(/g)).toHaveLength(1);
    expect(pump).toContain("runtime!.pushReferenceImageRows(offset, pixels)");
    expect(pump).toContain('(operation) => ownedStep("reference-image-row-strip", operation)');
    expect(pump).toContain('post({ kind: "wake", lifecycle })');
    expect(pump).toContain("await macrotask()");
    expect(decoderSource).toContain("new OffscreenCanvas(width, Math.min(height, rowsPerStrip))");
    expect(decoderSource).toContain('context.getImageData(0, 0, width, rows, { colorSpace: "srgb" })');
    expect(decoderSource).toContain("pixels.byteLength > REFERENCE_IMAGE_READBACK_BYTE_CAPACITY");
    expect(decoderSource).toContain("if (stageMode === 2) return undefined");
    expect(decoderSource).not.toContain("getImageData(0, 0, width, height");
    expect(decoderSource).not.toContain("Uint8Array.from(context.getImageData");
    for (const binding of ["beginReferenceImage", "pushReferenceImageRows", "sealReferenceImage"]) expect(rustSource).toContain(`js_name = ${binding}`);
    expect(rustSource).toContain("Result<u8, JsValue>");
    expect(workerSource).not.toContain("stageReferenceImage");
  });

  it("publishes the introspection hooks on the UI isolate only after the Worker reports booted", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8");
    const workerSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");
    expect(bootSource).toContain("host.semioWgpuIntrospection = { dumpStructure:");
    expect(bootSource).not.toContain("wasmBindings =");
    expect(bootSource.indexOf("detachIntrospection = attachIntrospectionBindings(transport)")).toBeGreaterThan(bootSource.indexOf("onReady: () => {"));
    for (const mapping of ['message.probe === "structure" ? bindings.dumpStructure', 'message.probe === "accessibility" ? bindings.dumpAccessibility', 'message.probe === "mesh-stats" ? bindings.dumpMeshStats', 'message.probe === "chrome" ? bindings.dumpChrome', "bindings.dumpFrameStats"]) expect(workerSource).toContain(mapping);
    expect(bootSource).toContain('dumpChrome: probe("chrome")');
    expect(workerSource).toContain("INTROSPECTION_STEP_BUDGET_MS");
  });

  it("carries the page realm's platform read across the boot seam", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8");
    const workerSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");
    const descriptorSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"), "utf8");
    // ⌨️ The renderer answers `mod` with `cfg!(target_os = "macos")`, which is FALSE in every wasm
    // build, so a macOS browser painted `Ctrl+Alt+E` where React painted `⌘️⌥️E`. `userAgentData`
    // exists only on the page thread, so the read is made there and forwarded with the boot.
    expect(bootSource).toContain("platform: hostPlatform()");
    expect(workerSource).toContain("loaded.semioWgpuSetHostPlatform?.(message.platform)");
    expect(descriptorSource).toContain("export function resolveWgpuHostPlatform");
    expect(resolveWgpuHostPlatform({ navigator: { platform: "MacIntel" } as Navigator })).toBe("MacIntel");
    expect(resolveWgpuHostPlatform({ navigator: { platform: "Win32", userAgentData: { platform: "macOS" } } as unknown as Navigator })).toBe("macOS");
    expect(resolveWgpuHostPlatform({})).toBe("");
  });

  // 🏷️ W15f — the brand REGISTRY row rides the SAME descriptor `locks`/`defaults` do. The catalogue
  // stays TypeScript (`🧑‍💻dev/🏷️brand/🟦️.ts`'s `resolveShellBrandById`); only its resolved row crosses
  // into the renderer, so the shell can key the tour's seen flag the way React does
  // (`"<brandId>:<appId>"`) and honour `ephemeral`/`replayIntroductionOnLoad` at all.
  it("carries the resolved shell brand row on the boot descriptor", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const serveSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🌐️server/🟦️.ts"), "utf8");
    const rendererSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"), "utf8");

    const unbranded = resolveWgpuBootDescriptor({ defaultVariant: "s" });
    expect(unbranded.brandId).toBe("");
    expect(unbranded.brand).toEqual({ windowTitle: "", ephemeral: false, replayIntroductionOnLoad: false });

    const meta: Record<string, string> = {
      "semio-brand": "entwerfen-mit-bestand-aggregator",
      "semio-brand-window-title": "Aggregator",
      "semio-brand-ephemeral": "true",
      "semio-brand-replay-introduction": "true",
    };
    const branded = resolveWgpuBootDescriptor({ defaultVariant: "s", meta: (name) => meta[name] ?? "" });
    expect(branded.brandId).toBe("entwerfen-mit-bestand-aggregator");
    expect(branded.brand).toEqual({ windowTitle: "Aggregator", ephemeral: true, replayIntroductionOnLoad: true });

    // 🎛️ An override beats the page, exactly as every other axis does — the embeddable door's path.
    expect(resolveWgpuBootDescriptor({ defaultVariant: "s", meta: (name) => meta[name] ?? "", overrides: { brand: { ephemeral: false } } }).brand.ephemeral).toBe(false);

    // 🧭️ All three doors: the serve injects the meta tags, the native side reads the env twins.
    for (const tag of ["semio-brand-window-title", "semio-brand-ephemeral", "semio-brand-replay-introduction"]) expect(serveSource).toContain(`["${tag}"`);
    for (const variable of ["SEMIO_BRAND_WINDOW_TITLE", "SEMIO_BRAND_EPHEMERAL", "SEMIO_BRAND_REPLAY_INTRODUCTION"]) expect(rendererSource).toContain(variable);
  });

  it("keeps product discovery and native UI capability out of the UI/Worker seams", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8");
    const workerSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");
    const rustSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"), "utf8");
    expect(bootSource).not.toContain("PLUGIN_CATALOG");
    expect(bootSource).not.toContain("resolvePlaygroundBoot");
    expect(bootSource).not.toContain("performance.getEntriesByType");
    expect(workerSource).toContain("new PlaygroundBootPlanner(PLUGIN_CATALOG");
    expect(workerSource).toContain('monitoredSuspension("renderer-module", () => import');
    expect(workerSource).toContain('closeOwner === "runtime"');
    expect(workerSource).toContain("interactiveJobs?.close()");
    expect(workerSource).toContain('getReader({ mode: "byob" })');
    expect(workerSource).toContain("new Uint8Array(ASSET_RESPONSE_PAGE_BYTES)");
    expect(workerSource).toContain("declared ?? ASSET_RESPONSE_BYTE_CAPACITY");
    expect(workerSource).toContain("declared !== undefined && received !== declared");
    expect(workerSource).toContain("ASSET_RESPONSE_PAGE_BYTES");
    expect(workerSource).toContain("assetAbort?.abort()");
    expect(workerSource).not.toContain("response.arrayBuffer()");
    expect(rustSource).toContain("OffscreenPresentToken::mint_for_dedicated_worker");
    expect(rustSource).toContain("WorldAssetResponsePage::try_from_owned");
    expect(rustSource).not.toContain("UiPresentToken");
    const admittedRetirement = (source: string): boolean => {
      const handoff = source.slice(source.indexOf("if self.close_phase == 5 {"), source.indexOf("self.close_phase = 6;"));
      const retirement = source.slice(source.indexOf("if let Some(retired) = self.retired_host.as_mut()"), source.indexOf("fn ensure_live("));
      return handoff.includes("match host.try_into_retirement()")
        && handoff.includes("Ok(retirement) => self.retired_host = Some(retirement)")
        && handoff.includes("Err(host) => {")
        && handoff.includes("self.host = Some(host);")
        && handoff.includes('return Err(js_error("host-close", "host retirement abandonment registry refused admission"))')
        && retirement.includes("if !retired.close_step()")
        && retirement.includes("if !retired.terminal_is_empty()")
        && retirement.indexOf("if !retired.terminal_is_empty()") < retirement.indexOf("self.retired_host = None;");
    };
    expect(admittedRetirement(rustSource)).toBe(true);
    for (const required of ["match host.try_into_retirement()", "self.host = Some(host);", "if !retired.terminal_is_empty()", "if !retired.close_step()"])
      expect(admittedRetirement(rustSource.replace(required, "unqualified_retirement"))).toBe(false);
    expect(rustSource).not.toContain("drop(self.host.take())");
    expect(rustSource).not.toContain("forget(host)");
    expect(rustSource).toContain("DispatchEvent::TextEditStart");
    expect(rustSource).toContain("DispatchEvent::TextEditChunk");
    expect(rustSource).toContain("DispatchEvent::TextEditCommit");
    expect(rustSource).toContain("pending_discrete.saturating_add(discrete_commits)");
    expect(rustSource).not.toContain("stream.text.push_str");
    // 🧭️ The boot-query capacity guard lives in `🧭️boot-descriptor/🟦️.ts` (the schema-owned parser
    // both doors share) rather than inline in the page boot, so the law follows it there: the bound
    // is applied to `location.search` BEFORE the string reaches `URLSearchParams`.
    const descriptorSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"), "utf8");
    expect(bootSource).not.toContain("new URLSearchParams");
    expect(descriptorSource.indexOf("WGPU_BOOT_LOCATION_CAPACITY")).toBeLessThan(descriptorSource.indexOf("new URLSearchParams"));
    expect(descriptorSource).toContain('new URLSearchParams(boundedLocation(input.search ?? "", "location.search"))');
  });
});

type VirtualTimer = { readonly id: number; readonly callback: () => void; readonly dueAtMs: number };

/** @emoji ⏱️ A virtual clock + timer queue, so a 120 s boot is decided in microseconds and the watchdog's
 * verdict is a function of the timeline rather than of how loaded the machine running the suite is. */
function bootHarness(tongue: "en" | "de" = "en") {
  const worker = new FakeWorker();
  const timers = new Map<number, VirtualTimer>();
  const faults: { readonly code: string; readonly detail: string; readonly fallback: BrowserFrameFallbackState }[] = [];
  let nextTimerId = 1;
  let nowMs = 0;
  const subject = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer_bg.wasm", canvas: {} as OffscreenCanvas, width: 8, height: 8, dpr: 1, locale: tongue, descriptor: testBootDescriptor("generation3d"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE },
    now: () => nowMs,
    setTimer: (callback, delayMs) => {
      const id = nextTimerId++;
      timers.set(id, { id, callback, dueAtMs: nowMs + delayMs });
      return id;
    },
    clearTimer: (handle) => void timers.delete(handle),
    onFault: (code, detail, fallback) => faults.push({ code, detail, fallback }),
  });
  const advance = (deltaMs: number): void => {
    const targetMs = nowMs + deltaMs;
    for (;;) {
      let due: VirtualTimer | undefined;
      for (const timer of timers.values()) if (timer.dueAtMs <= targetMs && (due === undefined || timer.dueAtMs < due.dueAtMs)) due = timer;
      if (due === undefined) break;
      timers.delete(due.id);
      nowMs = Math.max(nowMs, due.dueAtMs);
      due.callback();
    }
    nowMs = targetMs;
  };
  return { advance, faults, subject, worker, now: () => nowMs };
}

describe("wgpu boot liveness watchdog", () => {
  it("re-arms for the exact remaining window and never terminates a declared phase inside its ceiling", () => {
    const silenceTimeoutMs = FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs;
    expect(evaluateBrowserBootLiveness({ nowMs: 40_000, lastLivenessAtMs: 10_000, silenceTimeoutMs, phase: undefined })).toEqual({ terminate: false, rearmInMs: 30_000, silentForMs: 30_000, phaseElapsedMs: 0 });
    expect(evaluateBrowserBootLiveness({ nowMs: 70_000, lastLivenessAtMs: 10_000, silenceTimeoutMs, phase: undefined }).terminate).toBe(true);
    expect(evaluateBrowserBootLiveness({ nowMs: 0, lastLivenessAtMs: Number.NEGATIVE_INFINITY, silenceTimeoutMs, phase: undefined }).silentForMs).toBe(Number.POSITIVE_INFINITY);
    const phase = { phase: "gpu-platform", ceilingMs: bootPhaseCeilingMs("gpu-platform"), enteredAtMs: 1_000 };
    const busy = evaluateBrowserBootLiveness({ nowMs: 601_000, lastLivenessAtMs: 1_000, silenceTimeoutMs, phase });
    expect(busy.terminate).toBe(false);
    expect(busy.phaseElapsedMs).toBe(600_000);
    expect(busy.rearmInMs).toBe(Math.min(silenceTimeoutMs, phase.ceilingMs - 600_000));
    expect(evaluateBrowserBootLiveness({ nowMs: 1_000 + phase.ceilingMs, lastLivenessAtMs: 1_000, silenceTimeoutMs, phase }).terminate).toBe(true);
  });

  it("prices a phase family by the segment before its colon and bounds every unnamed phase", () => {
    expect(bootPhaseCeilingMs("plugin:generation3d")).toBe(FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs["plugin"]);
    expect(bootPhaseCeilingMs("shell-boot")).toBe(FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs["shell-boot"]);
    expect(bootPhaseCeilingMs("a-phase-nobody-declared")).toBe(FRAME_WORKER_BOOT_LIVENESS_POLICY.defaultPhaseCeilingMs);
    expect(bootPhaseCeilingMs("a-phase-nobody-declared")).toBeGreaterThan(0);
  });

  it("keeps a Worker that declared a long phase alive across 120 s of total wall silence", () => {
    const { advance, faults, subject, worker } = bootHarness();
    worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "renderer-runtime", progress: 0.65, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
    worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "enter" });
    advance(120_000);
    expect(faults).toEqual([]);
    expect(subject.status).toBe("booting");
    expect(worker.terminated).toBe(false);
    expect(subject.fallbackState().bootPhase?.phase).toBe("gpu-platform");
    expect(subject.fallbackState().bootPhaseElapsedMs).toBe(120_000);
    worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "leave", elapsedMs: 120_000 });
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.status).toBe("ready");
  });

  it("keeps a heartbeating Worker alive indefinitely and still terminates one that goes silent with no phase declared", () => {
    const { advance, faults, subject, worker } = bootHarness();
    worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "plugin:generation3d", progress: 0.4, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
    for (let beat = 0; beat < 240; beat++) {
      advance(FRAME_WORKER_BOOT_LIVENESS_POLICY.livenessIntervalMs);
      worker.reply({ kind: "boot-liveness", lifecycle: 1 });
    }
    expect(faults).toEqual([]);
    expect(subject.status).toBe("booting");
    advance(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs + 1);
    expect(subject.status).toBe("faulted");
    expect(faults).toHaveLength(1);
    expect(faults[0]?.code).toBe("worker-boot-timeout");
    expect(faults[0]?.detail).toContain("declared no long phase");
    expect(faults[0]?.detail).toContain("plugin:generation3d");
    expect(worker.messages.some((message) => message.kind === "close")).toBe(true);
  });

  it("terminates a declared phase only once it blows its OWN ceiling, naming the phase, its elapsed and that ceiling", () => {
    const { advance, faults, subject, worker } = bootHarness();
    worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "renderer-runtime", progress: 0.65, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
    worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "enter" });
    const ceilingMs = bootPhaseCeilingMs("gpu-platform");
    advance(ceilingMs - 1);
    expect(subject.status).toBe("booting");
    advance(2);
    expect(subject.status).toBe("faulted");
    expect(faults[0]?.code).toBe("worker-boot-timeout");
    expect(faults[0]?.detail).toContain('"gpu-platform"');
    expect(faults[0]?.detail).toContain(`${ceilingMs} ms ceiling`);
    expect(faults[0]?.detail).toContain('last reported stage "renderer-runtime"');
    expect(faults[0]?.fallback.bootPhase?.phase).toBe("gpu-platform");
    expect(faults[0]?.fallback.bootStage).toBe("renderer-runtime");
  });

  it("names a Worker that never sent a single message, and says the same thing in German", () => {
    const { advance, faults, subject } = bootHarness("de");
    advance(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs + 1);
    expect(subject.status).toBe("faulted");
    expect(faults[0]?.detail).toContain("hat nie eine einzige Nachricht gesendet");
    expect(faults[0]?.detail).toContain("Ereignisschleife hängt");
    const report = { heard: true, lastStage: "renderer-runtime", silentForMs: 61_000, silenceTimeoutMs: 60_000, phase: { phase: "wasm-compile", ceilingMs: 900_000, enteredAtMs: 0 }, phaseElapsedMs: 901_000 };
    expect(describeBrowserBootSilence(report, "en")).toBe('The frame Worker\'s declared long phase "wasm-compile" ran 901000 ms against its 900000 ms ceiling (last reported stage "renderer-runtime", silent for 61000 ms).');
    expect(describeBrowserBootSilence(report, "de")).toContain("Die erklärte lange Phase „wasm-compile“");
    expect(describeBrowserBootPhase(report.phase, 901_000, "en")).toBe('Long boot phase: "wasm-compile" for 901000 ms (ceiling 900000 ms)');
    expect(describeBrowserBootPhase(undefined, 0, "de")).toBe("Lange Boot-Phase: keine erklärt");
  });

  it("declares every browser-owned phase BEFORE it blocks, and brings the renderer wasm up in cached, reporting phases", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const workerSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");
    const bootSource = readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8");
    const suspension = workerSource.slice(workerSource.indexOf("async function monitoredSuspension"), workerSource.indexOf("async function macrotask"));
    expect(suspension.indexOf('declarePhase(stage, "enter", 0)')).toBeGreaterThan(-1);
    expect(suspension.indexOf('declarePhase(stage, "enter", 0)')).toBeLessThan(suspension.indexOf("stepClock.suspend()"));
    expect(suspension).toContain('declarePhase(stage, "leave"');
    expect(workerSource).toContain('declaredStep("renderer-bootstrap"');
    expect(workerSource).toContain("WebAssembly.compileStreaming");
    expect(workerSource).toContain("readCachedRendererModule");
    expect(workerSource).toContain("writeCachedRendererModule");
    expect(workerSource).not.toContain('monitoredSuspension("wasm-instance"');
    expect(bootSource).toContain("describeBrowserBootPhase(state.bootPhase, state.bootPhaseElapsedMs, tongue)");
    expect(workerSource).toContain("if (!bootDeclarationsOpen || closed || closing || failed) return;");
    expect(workerSource.indexOf("bootDeclarationsOpen = false;")).toBeLessThan(workerSource.indexOf('post({ kind: "booted", lifecycle })'));
  });
});

describe("wgpu boot liveness watchdog, replayed on a third-party clock", () => {
  /** @emoji 🧪️ The same 120 s timeline decided by vitest's own fake timers (`@sinonjs/fake-timers`) driving
   * the real `setTimeout`/`clearTimeout`, instead of this file's hand-rolled queue — an INDEPENDENT clock
   * implementation reaching the same verdicts, so a bug in the harness cannot pass for a passing law. */
  it("survives a declared 120 s phase and dies on 60 s of undeclared silence under vitest's fake timers", () => {
    vi.useFakeTimers();
    try {
      const worker = new FakeWorker();
      const faults: string[] = [];
      const subject = new BrowserFrameTransport({
        worker,
        boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer_bg.wasm", canvas: {} as OffscreenCanvas, width: 8, height: 8, dpr: 1, locale: "en", descriptor: testBootDescriptor("generation3d"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE },
        now: () => Date.now(),
        setTimer: (callback, delayMs) => setTimeout(callback, delayMs) as unknown as number,
        clearTimer: (handle) => clearTimeout(handle),
        onFault: (code, detail) => faults.push(`${code}: ${detail}`),
      });
      worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "renderer-runtime", progress: 0.65, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
      worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "enter" });
      vi.advanceTimersByTime(120_000);
      expect(faults).toEqual([]);
      expect(subject.status).toBe("booting");
      worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "leave", elapsedMs: 120_000 });
      vi.advanceTimersByTime(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs + 1);
      expect(subject.status).toBe("faulted");
      expect(faults[0]).toContain("worker-boot-timeout");
      expect(faults[0]).toContain("declared no long phase");
    } finally {
      vi.useRealTimers();
    }
  });
});

describe("wgpu renderer module cache", () => {
  it("reads the artifact's server-asserted identity and disables itself when the server offers none", async () => {
    const original = globalThis.fetch;
    try {
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 200, headers: { etag: 'W/"7ab1"' } }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe('etag:W/"7ab1"');
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 200, headers: { "last-modified": "Thu, 10 Sep 2026 09:29:09 GMT", "content-length": "76048601" } }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("mtime:Thu, 10 Sep 2026 09:29:09 GMT:76048601");
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 200 }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("");
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 404 }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("");
      globalThis.fetch = stubFetch(async () => { throw new Error("offline"); });
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("");
    } finally {
      globalThis.fetch = original;
    }
  });

  it("fails soft to no cache wherever IndexedDB or a taggable artifact is unavailable", async () => {
    expect((globalThis as { indexedDB?: unknown }).indexedDB).toBeUndefined();
    expect(await readCachedRendererModule("http://host/renderer_bg.wasm", "")).toBeUndefined();
    expect(await readCachedRendererModule("http://host/renderer_bg.wasm", "etag:1")).toBeUndefined();
    expect(await writeCachedRendererModule("http://host/renderer_bg.wasm", "", {} as WebAssembly.Module, 1, 0)).toBe(false);
    expect(await writeCachedRendererModule("http://host/renderer_bg.wasm", "etag:1", {} as WebAssembly.Module, 1, 0)).toBe(false);
    await expect(evictCachedRendererModule("http://host/renderer_bg.wasm")).resolves.toBeUndefined();
  });
});

/** 🌊️ Input batches and credits: the bounded input batches, the lease release, the Hub status and lossless credits, the accessibility events and the byte credit exhaustion. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { BrowserFrameTransport, FRAME_WORKER_ACCESSIBILITY_ID_BYTES, FRAME_WORKER_BYTE_CAPACITY, FRAME_WORKER_HUB_DOCUMENT_CAPACITY, FRAME_WORKER_LOSSLESS_ITEM_CAPACITY } from "../../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { FakeWorker, transport } from "../🧰️transport-test-support/🟦️.ts";

describe("browser frame worker transport", () => {
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
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 1, generation: first.generation, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 0.5, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 2, generation: first.generation, cursor: "text", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
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
    worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: first.inputSequence, generation: first.generation, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    const delivered: unknown[] = [...first.lossless];
    let sequence = first.inputSequence;
    while (subject.flush(++sequence)) {
      const batch = worker.messages.at(-1);
      if (batch?.kind !== "batch") break;
      delivered.push(...batch.lossless);
      worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: batch.inputSequence, generation: batch.generation });
      worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: batch.inputSequence, generation: batch.generation, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
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

  it("admits an accessibility address by its credits alone: a document root (node id 0) is an address, liveness is the renderer's", () => {
    const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json"), "utf8")) as { readonly transportCredits: Record<"admitted" | "refused", readonly { readonly id: string; readonly wire: Parameters<BrowserFrameTransport["enqueueLossless"]>[0] }[]> };
    for (const row of fixture.transportCredits.admitted) {
      const worker = new FakeWorker();
      const subject = transport(worker);
      worker.reply({ kind: "booted", lifecycle: 1 });
      expect(subject.enqueueLossless(row.wire), row.id).toBe(true);
      expect(subject.fault, row.id).toBeUndefined();
    }
    for (const row of fixture.transportCredits.refused) {
      const worker = new FakeWorker();
      const subject = transport(worker);
      expect(subject.enqueueLossless(row.wire), row.id).toBe(false);
      expect(subject.fault?.code, row.id).toBe("lossless-overflow");
    }
  });

  it("fails closed on byte-credit exhaustion", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    expect(subject.enqueueLossless({ kind: "paste", text: "x".repeat(FRAME_WORKER_BYTE_CAPACITY / 2) })).toBe(false);
    expect(subject.fault?.code).toBe("lossless-overflow");
  });
});

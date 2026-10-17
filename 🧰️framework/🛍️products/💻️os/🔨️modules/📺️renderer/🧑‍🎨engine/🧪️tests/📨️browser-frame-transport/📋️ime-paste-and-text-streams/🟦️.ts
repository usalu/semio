/** 📋️ IME, paste and text streams: oversized IME, the chunked paste, the image data url paste target, astral code points and the JSON escaping bound. */
import { describe, expect, it } from "vitest";
import { FRAME_WORKER_BYTE_CAPACITY, FRAME_WORKER_MESSAGE_BYTE_CAPACITY } from "../../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { FakeWorker, transport } from "../🧰️transport-test-support/🟦️.ts";

describe("browser frame worker transport", () => {
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
});

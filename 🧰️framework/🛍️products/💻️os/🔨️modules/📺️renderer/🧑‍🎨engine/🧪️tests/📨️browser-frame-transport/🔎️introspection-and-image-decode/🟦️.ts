/** 🔎️ Introspection and image decode: the introspection dump, the single page image decode, the retired component response and handback, the aborted stream continuation and the asset polling gate. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { FRAME_WORKER_INTROSPECTION_CAPACITY, FRAME_WORKER_IMAGE_DECODE_BYTE_CAPACITY, type BrowserFrameUiMessage } from "@semio-tech/framework-renderer-wgpu/browser-frame-transport/contract";
import { BrowserAssetCancellationCursor, assertBrowserAssetResponseContinuation } from "@semio-tech/framework-renderer-wgpu/frame-worker/asset-cancellation";
import { FakeWorker, transport } from "../🧰️transport-test-support/🟦️.ts";
import { readFrameWorkerSource } from "../📖️frame-worker-source/🟦️.ts";

describe("browser frame worker transport", () => {
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

  it("aborts a retired component response, retries its exact handback, then admits the next asset", async () => {
    const fixtureDirectory = join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🛑️browser-component-asset-cancellation");
    const fixture = JSON.parse(readFileSync(join(fixtureDirectory, "🔣️.json"), "utf8")) as {
      readonly owners: readonly { readonly id: string; readonly responseToken: number; readonly bytes: readonly number[] }[];
      readonly handback: readonly ["busy", "returned"];
      readonly expected: { readonly aborted: string; readonly decodeCancellationOwner: string; readonly publishedOwners: readonly string[]; readonly pollOrder: readonly string[] };
    };
    
    
    const cursor = new BrowserAssetCancellationCursor();
    const controller = new AbortController();
    const polled: string[] = [fixture.owners[0]!.id];
    const decodeCancelled: string[] = [];
    const published: string[] = [];
    const returns = [...fixture.handback];
    let current = false;
    let decodeOwner: string | undefined = fixture.owners[0]!.id;
    const port = {
      hasFetch: () => !controller.signal.aborted,
      hasPageImageDecode: () => decodeOwner !== undefined,
      responseCurrent: () => current,
      abortFetch: () => controller.abort(),
      cancelPageImageDecode: () => {
        if (decodeOwner) decodeCancelled.push(decodeOwner);
        decodeOwner = undefined;
      },
      returnResponseOwner: () => returns.shift() === "returned",
    };
    expect(cursor.step(port)).toBe("waiting");
    expect(controller.signal.aborted).toBe(true);
    expect(decodeCancelled).toEqual([fixture.expected.decodeCancellationOwner]);
    expect(polled).toEqual([fixture.owners[0]!.id]);
    expect(cursor.step(port)).toBe("returned");
    polled.push(fixture.owners[1]!.id);
    current = true;
    published.push(fixture.owners[1]!.id);
    expect(polled).toEqual(fixture.expected.pollOrder);
    expect(published).toEqual(fixture.expected.publishedOwners);
    expect(fixture.expected.aborted).toBe(fixture.owners[0]!.id);
  });

  it("refuses an aborted stream continuation before mutating its returned response owner", async () => {
    const controller = new AbortController();
    const cursor = new BrowserAssetCancellationCursor();
    const ownerCalls: string[] = [];
    const faults: string[] = [];
    let current = true;
    const mutate = (name: string): void => {
      assertBrowserAssetResponseContinuation(controller, () => current);
      ownerCalls.push(name);
    };
    mutate("reserve-busy");
    await Promise.resolve();
    current = false;
    expect(cursor.step({
      hasFetch: () => true,
      hasPageImageDecode: () => false,
      responseCurrent: () => current,
      abortFetch: () => controller.abort(),
      cancelPageImageDecode: () => undefined,
      returnResponseOwner: () => true,
    })).toBe("returned");
    try {
      mutate("push-after-return");
    } catch (error) {
      if ((error as { readonly name?: string }).name !== "AbortError") faults.push(String(error));
    }
    expect(ownerCalls).toEqual(["reserve-busy"]);
    expect(faults).toEqual([]);
  });

  it("keeps asset polling closed while the exact retired response handback remains busy", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const workerSource = readFrameWorkerSource(root);
    const frameTurn = workerSource.slice(workerSource.indexOf("function runFrameTurn"), workerSource.indexOf("function runAssetDecodeTurn"));
    expect(frameTurn).toContain('if (assetCancellationStep === "idle") scheduleAssetPump()');
    expect(frameTurn).toContain('assetCancellationStep === "returned" && !assetPumping');
    expect(frameTurn).not.toMatch(/assetCancellationStep === "waiting"[\s\S]*scheduleAssetPump/);
    const schedule = workerSource.slice(workerSource.indexOf("function scheduleAssetPump"), workerSource.indexOf("async function pumpAsset"));
    expect(schedule).toContain("!assetCancellation.pollAdmitted()");
    const cursor = new BrowserAssetCancellationCursor();
    let handbackAttempts = 0;
    const port = {
      hasFetch: () => true,
      hasPageImageDecode: () => false,
      responseCurrent: () => false,
      abortFetch: () => undefined,
      cancelPageImageDecode: () => undefined,
      returnResponseOwner: () => ++handbackAttempts === 3,
    };
    expect(cursor.step(port)).toBe("waiting");
    expect(cursor.pollAdmitted()).toBe(false);
    expect(cursor.step(port)).toBe("waiting");
    expect(cursor.pollAdmitted()).toBe(false);
    expect(cursor.step(port)).toBe("returned");
    expect(cursor.pollAdmitted()).toBe(false);
    expect(handbackAttempts).toBe(3);
    expect(cursor.releaseReturned()).toBe(true);
    expect(cursor.pollAdmitted()).toBe(true);
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
      worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: sequence, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    }
    expect(directives).toEqual([1, 1, 1, 1, 1]);
    expect(turns).toEqual([]);
  });
});

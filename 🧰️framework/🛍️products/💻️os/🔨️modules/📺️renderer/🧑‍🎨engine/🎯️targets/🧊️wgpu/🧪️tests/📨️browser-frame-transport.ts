import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  BrowserFrameTransport,
  FRAME_WORKER_BYTE_CAPACITY,
  FRAME_WORKER_LOSSLESS_ITEM_CAPACITY,
  FRAME_WORKER_MESSAGE_BYTE_CAPACITY,
  FRAME_WORKER_POINTER_CAPACITY,
  type BrowserFrameUiMessage,
  type BrowserFrameWorkerMessage,
  type BrowserFrameWorkerPort,
} from "../🚚️browser-frame-transport/🟦️.ts";

class FakeWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];
  terminated = false;

  postMessage(message: BrowserFrameUiMessage): void {
    this.messages.push(message);
  }

  terminate(): void {
    this.terminated = true;
  }

  reply(message: BrowserFrameWorkerMessage): void {
    this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>);
  }
}

function transport(worker: FakeWorker, hooks: { directives?: number[]; faults?: string[] } = {}): BrowserFrameTransport {
  return new BrowserFrameTransport({
    worker,
    boot: {
      bindingsModuleUrl: "renderer.js",
      bindingsWasmUrl: "renderer_bg.wasm",
      canvas: {} as OffscreenCanvas,
      width: 800,
      height: 600,
      dpr: 2,
      pluginVariant: "s",
      locale: "en",
      appRole: "editor",
    },
    setTimer: () => 1,
    clearTimer: () => {},
    onDirectives: (value) => hooks.directives?.push(value.generation),
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
    subject.enqueueReplaceable({ kind: "pointer-move", pointerId: 7, pointerKind: "mouse", x: 1, y: 2 });
    subject.enqueueReplaceable({ kind: "pointer-move", pointerId: 7, pointerKind: "mouse", x: 3, y: 4 });
    subject.enqueueReplaceable({ kind: "wheel", x: 3, y: 4, deltaX: 1, deltaY: 2 });
    subject.enqueueReplaceable({ kind: "wheel", x: 4, y: 5, deltaX: 3, deltaY: 4 });
    subject.enqueueReplaceable({ kind: "resize", width: 10, height: 20, dpr: 1 });
    subject.enqueueReplaceable({ kind: "resize", width: 30, height: 40, dpr: 2 });
    expect(subject.flush(11)).toBe(true);
    const batch = worker.messages.at(-1);
    expect(batch?.kind).toBe("batch");
    if (batch?.kind !== "batch") return;
    expect(batch.replaceable).toHaveLength(3);
    expect(batch.replaceable).toContainEqual(expect.objectContaining({ kind: "pointer-move", x: 3, y: 4 }));
    expect(batch.replaceable).toContainEqual(expect.objectContaining({ kind: "wheel", x: 4, y: 5, deltaX: 4, deltaY: 6 }));
    expect(batch.replaceable).toContainEqual(expect.objectContaining({ kind: "resize", width: 30, height: 40, dpr: 2 }));
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
      expect(subject.enqueueReplaceable({ kind: "pointer-move", pointerId, pointerKind: "touch", x: pointerId, y: 0 })).toBe(true);
    }
    expect(subject.enqueueReplaceable({ kind: "pointer-move", pointerId: FRAME_WORKER_POINTER_CAPACITY, pointerKind: "touch", x: 0, y: 0 })).toBe(false);
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
    worker.reply({ kind: "frame", lifecycle: 1, sequence: 1, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1 });
    expect(directives).toEqual([]);
    expect(subject.flush(2)).toBe(true);
    worker.reply({ kind: "frame", lifecycle: 1, sequence: 2, generation: 2, cursor: "text", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1 });
    expect(directives).toEqual([2]);
  });

  it("terminates on close and rejects late Worker messages", () => {
    const worker = new FakeWorker();
    const directives: number[] = [];
    const subject = transport(worker, { directives });
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.close();
    worker.reply({ kind: "frame", lifecycle: 1, sequence: 1, generation: 0, cursor: "pointer", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1 });
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
    worker.reply({ kind: "frame", lifecycle: 1, sequence: 1, generation: 1, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 1 });
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
    worker.reply({ kind: "frame", lifecycle: 1, sequence: 1, generation: 0, cursor: "default", fullscreen: null, requestFrame: false, progress: 1, workerDurationMs: 8 });
    expect(subject.status).toBe("quarantined");
    expect(worker.terminated).toBe(false);
    expect(directives).toEqual([]);
    worker.reply({ kind: "frame", lifecycle: 1, sequence: 2, generation: 0, cursor: "pointer", fullscreen: true, requestFrame: true, progress: 1, workerDurationMs: 1 });
    expect(directives).toEqual([]);
  });

  it("measures external UI hooks centrally and stops immediately on hook overrun", () => {
    const worker = new FakeWorker();
    let now = 0;
    const subject = new BrowserFrameTransport({
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, pluginVariant: "s", locale: "en", appRole: "editor" },
      now: () => now,
      setTimer: () => 1,
      clearTimer: () => {},
      onReady: () => { now = 2; },
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.status).toBe("quarantined");
    expect(worker.messages.filter((message) => message.kind === "batch")).toHaveLength(0);
  });

  it("fails closed when the bounded structured clone consumes the UI turn budget", () => {
    const worker = new FakeWorker();
    let now = 0;
    const original = worker.postMessage.bind(worker);
    worker.postMessage = (message) => {
      original(message);
      if (message.kind === "batch") now = 2;
    };
    const subject = new BrowserFrameTransport({
      worker,
      boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer.wasm", canvas: {} as OffscreenCanvas, width: 1, height: 1, dpr: 1, pluginVariant: "s", locale: "en", appRole: "editor" },
      now: () => now,
      setTimer: () => 1,
      clearTimer: () => {},
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.flush()).toBe(false);
    expect(subject.fault?.code).toBe("ui-turn-overrun");
  });

  it("keeps product discovery and native UI capability out of the UI/Worker seams", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = readFileSync(join(root, "../🚀️browser-boot/🟦️.ts"), "utf8");
    const workerSource = readFileSync(join(root, "../🎞️frame-worker/🟦️.ts"), "utf8");
    const rustSource = readFileSync(join(root, "../🌐️browser-worker/🦀️.rs"), "utf8");
    expect(bootSource).not.toContain("PLUGIN_CATALOG");
    expect(bootSource).not.toContain("resolvePlaygroundBoot");
    expect(bootSource).not.toContain("performance.getEntriesByType");
    expect(workerSource).toContain("resolvePlaygroundBoot(PLUGIN_CATALOG");
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
    expect(bootSource.indexOf("location.search.length")).toBeLessThan(bootSource.indexOf("new URLSearchParams"));
  });
});

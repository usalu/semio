// #region 🔖️Protocol
/** @emoji 🧵️ Browser UI-to-frame-Worker protocol with bounded lossless and latest-wins lanes. */

import { BrowserInteractiveJobPort, type InteractiveJobUiMessage, type InteractiveJobWorkerMessage } from "../🔌️browser-interactive-job-port/🟦️.ts";

export const FRAME_WORKER_LOSSLESS_ITEM_CAPACITY = 64;
export const FRAME_WORKER_BYTE_CAPACITY = 256 * 1024;
/** @emoji ⏳️ A boot STALL bound, not a total-boot deadline: every `boot-progress` the Worker reports is
 * proof of liveness and rearms it. The `s` boot plan mounts 57 plugin modules one macrotask at a time, so a
 * healthy cold boot legitimately outruns any fixed total budget, while a Worker that stops reporting still
 * fails closed within this window. */
export const FRAME_WORKER_BOOT_STALL_TIMEOUT_MS = 15_000;
export const FRAME_WORKER_POINTER_CAPACITY = 16;
export const FRAME_WORKER_MESSAGE_BYTE_CAPACITY = 4 * 1024;
export const FRAME_WORKER_TEXT_CHUNK_CODE_UNITS = 1024;
export const FRAME_UI_TURN_BUDGET_MS = 2;
export const FRAME_WORKER_INTROSPECTION_CAPACITY = 4;
export const FRAME_WORKER_INTROSPECTION_TIMEOUT_MS = 10_000;

export type BrowserFrameWorkerFaultCode =
  | "worker-unavailable"
  | "offscreen-canvas-unavailable"
  | "offscreen-transfer-failed"
  | "worker-construction-failed"
  | "worker-boot-timeout"
  | "worker-boot-failed"
  | "worker-runtime-failed"
  | "worker-step-overrun"
  | "worker-present-failed"
  | "worker-input-failed"
  | "worker-message-failed"
  | "ui-turn-overrun"
  | "protocol-violation"
  | "replaceable-overflow"
  | "lossless-overflow"
  | "transport-closed";

export type BrowserFrameWorkerStatus = "booting" | "ready" | "quarantined" | "faulted" | "closed";

export type BrowserFramePointer = {
  readonly pointerId: number;
  readonly pointerKind: "mouse" | "touch" | "pen" | "eraser";
  readonly x: number;
  readonly y: number;
  readonly pressure?: number;
  readonly tiltX?: number;
  readonly tiltY?: number;
};

export type BrowserFrameReplaceableEvent =
  | ({ readonly kind: "pointer-move" } & BrowserFramePointer)
  | { readonly kind: "wheel"; readonly x: number; readonly y: number; readonly deltaX: number; readonly deltaY: number }
  | { readonly kind: "resize"; readonly width: number; readonly height: number; readonly dpr: number };

export type BrowserFrameLosslessEvent =
  | ({ readonly kind: "pointer-down" | "pointer-up"; readonly button: "primary" | "secondary" | "middle" } & BrowserFramePointer)
  | { readonly kind: "key-down" | "key-up"; readonly key: string; readonly shift: boolean; readonly ctrl: boolean; readonly alt: boolean; readonly meta: boolean }
  | { readonly kind: "text" | "paste"; readonly text: string }
  | { readonly kind: "ime-start" | "ime-cancel" }
  | { readonly kind: "ime-update"; readonly text: string; readonly cursor: number }
  | { readonly kind: "ime-commit"; readonly text: string };

export type BrowserFrameWorkerBoot = {
  readonly kind: "boot";
  readonly lifecycle: number;
  readonly bindingsModuleUrl: string;
  readonly bindingsWasmUrl: string;
  readonly canvas: OffscreenCanvas;
  readonly width: number;
  readonly height: number;
  readonly dpr: number;
  readonly pluginVariant: string;
  readonly locale: "en" | "de";
  readonly appRole: string;
  readonly hub?: { readonly hubUrl: string; readonly user: string; readonly dataDir: string };
};

export type BrowserFrameWorkerBatch = {
  readonly kind: "batch";
  readonly lifecycle: number;
  readonly sequence: number;
  readonly generation: number;
  readonly timestampMs: number;
  readonly replaceable: readonly BrowserFrameReplaceableEvent[];
  readonly lossless: readonly BrowserFrameWireLosslessEvent[];
};

export type BrowserFrameWireLosslessEvent =
  | Exclude<BrowserFrameLosslessEvent, { readonly kind: "text" | "paste" | "ime-update" | "ime-commit" }>
  | { readonly kind: "text-chunk"; readonly streamId: number; readonly target: "text" | "paste" | "ime-update" | "ime-commit"; readonly text: string; readonly totalBytes: number; readonly final: boolean; readonly cursor?: number };

/** @emoji 🔬️ The renderer's `#[wasm_bindgen]` introspection exports (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
 * region `🔬️IntrospectionExports`) read `UI_ENGINE`, a thread-local that lives inside `semio-frame-worker`.
 * The UI isolate therefore cannot call them directly and asks across the same fail-closed seam every other
 * frame message uses. Read-only by construction: no probe mutates renderer state. */
export type BrowserFrameIntrospectionProbe = "structure" | "frame-stats";

export type BrowserFrameWorkerIntrospect = { readonly kind: "introspect"; readonly lifecycle: number; readonly requestId: number; readonly probe: BrowserFrameIntrospectionProbe };

/** @emoji 🧵️ One shard worker the UI isolate spawned on the frame worker's behalf, handed back as a
 * `MessagePort`. Nested dedicated workers are unavailable in some embedded browsers, so the frame worker
 * never constructs a shard `Worker` itself — see `shardWorkerPortBridge` in the wgpu plugin bridge. */
export type BrowserFrameShardPort = { readonly kind: "shard-port"; readonly shardIndex: number; readonly port: MessagePort };

export type BrowserFrameUiMessage = BrowserFrameWorkerBoot | BrowserFrameWorkerBatch | BrowserFrameWorkerIntrospect | InteractiveJobUiMessage | BrowserFrameShardPort | { readonly kind: "close"; readonly lifecycle: number };

export type BrowserFrameWorkerMessage =
  | { readonly kind: "boot-progress"; readonly lifecycle: number; readonly stage: string; readonly progress: number }
  | { readonly kind: "booted"; readonly lifecycle: number }
  | { readonly kind: "wake"; readonly lifecycle: number }
  | {
      readonly kind: "frame";
      readonly lifecycle: number;
      readonly sequence: number;
      readonly generation: number;
      readonly cursor: string;
      readonly fullscreen: boolean | null;
      readonly requestFrame: boolean;
      readonly progress: number;
      readonly workerDurationMs: number;
      readonly quarantined?: boolean;
      readonly faultCode?: string;
      readonly faultDetail?: string;
    }
  | { readonly kind: "introspection"; readonly lifecycle: number; readonly requestId: number; readonly probe: BrowserFrameIntrospectionProbe; readonly json: string | null; readonly detail?: string }
  | { readonly kind: "fault"; readonly lifecycle: number; readonly code: string; readonly detail: string }
  | { readonly kind: "closed"; readonly lifecycle: number }
  | { readonly kind: "shard-spawn"; readonly shardIndex: number; readonly url: string }
  | { readonly kind: "shard-terminate"; readonly shardIndex: number }
  | InteractiveJobWorkerMessage;

export interface BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null;
  onmessageerror: ((event: MessageEvent) => void) | null;
  onerror: ((event: ErrorEvent) => void) | null;
  postMessage(message: BrowserFrameUiMessage, transfer?: Transferable[]): void;
  terminate(): void;
}

export type BrowserFrameDirectives = {
  readonly cursor: string;
  readonly fullscreen: boolean | null;
  readonly generation: number;
  readonly workerDurationMs: number;
};

export type BrowserFrameTransportOptions = {
  readonly worker: BrowserFrameWorkerPort;
  readonly boot: Omit<BrowserFrameWorkerBoot, "kind" | "lifecycle">;
  readonly now?: () => number;
  readonly setTimer?: (callback: () => void, delayMs: number) => number;
  readonly clearTimer?: (handle: number) => void;
  readonly onReady?: () => void;
  readonly onProgress?: (stage: string, progress: number) => void;
  readonly onDirectives?: (directives: BrowserFrameDirectives) => void;
  readonly onFault?: (code: BrowserFrameWorkerFaultCode, detail: string) => void;
  readonly requestAnimationFrame?: (callback: FrameRequestCallback) => number;
  readonly cancelAnimationFrame?: (handle: number) => void;
};

type QueuedLossless = {
  readonly event: BrowserFrameLosslessEvent;
  readonly bytes: number;
  readonly streamId: number;
  readonly timestampMs: number;
  cursor: number;
};
// #endregion 🔖️Protocol

// #region 📮️Transport
/** @emoji 📮️ Owns one fail-closed frame Worker lifecycle and its bounded admission state. */
export class BrowserFrameTransport {
  readonly lifecycle = 1;
  readonly interactiveJobs: BrowserInteractiveJobPort;
  status: BrowserFrameWorkerStatus = "booting";
  fault: { readonly code: BrowserFrameWorkerFaultCode; readonly detail: string } | undefined;
  private readonly worker: BrowserFrameWorkerPort;
  private readonly shardWorkers = new Map<number, Worker>();
  private readonly now: () => number;
  private readonly clearTimer: (handle: number) => void;
  private readonly setTimer: (callback: () => void, delayMs: number) => number;
  private readonly onReady?: () => void;
  private readonly onProgress?: (stage: string, progress: number) => void;
  private readonly onDirectives?: (directives: BrowserFrameDirectives) => void;
  private readonly onFault?: (code: BrowserFrameWorkerFaultCode, detail: string) => void;
  private readonly requestRaf?: (callback: FrameRequestCallback) => number;
  private readonly cancelRaf?: (handle: number) => void;
  private readonly pointerIds = new Array<number>(FRAME_WORKER_POINTER_CAPACITY);
  private readonly pointerMoves = new Array<BrowserFrameReplaceableEvent | undefined>(FRAME_WORKER_POINTER_CAPACITY);
  private pointerCount = 0;
  private wheel: BrowserFrameReplaceableEvent | undefined;
  private resize: BrowserFrameReplaceableEvent | undefined;
  private lossless: QueuedLossless[] = [];
  private losslessBytes = 0;
  private nextStreamId = 1;
  private generation = 0;
  private sequence = 0;
  private acceptedSequence = 0;
  private inFlight = false;
  private frameRequested = false;
  private rafHandle: number | undefined;
  private bootTimer: number | undefined;
  private closeRequested = false;
  private readonly uiTurnSamples = new Float64Array(64);
  private uiTurnSampleCount = 0;
  private readonly introspections = new Map<number, { readonly resolve: (json: string | null) => void; readonly timer: number }>();
  private nextIntrospectionId = 1;

  constructor(options: BrowserFrameTransportOptions) {
    this.worker = options.worker;
    this.now = options.now ?? (() => performance.now());
    const setTimer = options.setTimer ?? ((callback, delayMs) => window.setTimeout(callback, delayMs));
    this.setTimer = setTimer;
    this.clearTimer = options.clearTimer ?? ((handle) => window.clearTimeout(handle));
    this.onReady = options.onReady;
    this.onProgress = options.onProgress;
    this.onDirectives = options.onDirectives;
    this.onFault = options.onFault;
    this.requestRaf = options.requestAnimationFrame;
    this.cancelRaf = options.cancelAnimationFrame;
    this.interactiveJobs = new BrowserInteractiveJobPort(this.lifecycle, (message) => this.worker.postMessage(message), this.now, (detail) => this.quarantine("ui-turn-overrun", detail), (callback) => void this.setTimer(callback, 0));
    this.worker.onmessage = (event) => this.receive(event.data);
    this.worker.onerror = (event) => this.fail("worker-message-failed", event.message || "Worker error");
    this.worker.onmessageerror = () => this.fail("worker-message-failed", "Worker message could not be decoded");
    this.armBootStallTimer();
    try {
      this.worker.postMessage({ kind: "boot", lifecycle: this.lifecycle, ...options.boot }, [options.boot.canvas]);
    } catch (error) {
      this.fail("worker-boot-failed", error instanceof Error ? error.message : String(error));
    }
  }

  /** @emoji 🫧 Coalesces replaceable input without consuming lossless credits. */
  enqueueReplaceable(event: BrowserFrameReplaceableEvent): boolean {
    if (!this.accepting()) return false;
    this.generation++;
    if (event.kind === "pointer-move") {
      let slot = -1;
      for (let index = 0; index < this.pointerCount; index++) {
        if (this.pointerIds[index] === event.pointerId) {
          slot = index;
          break;
        }
      }
      if (slot < 0) {
        if (this.pointerCount === FRAME_WORKER_POINTER_CAPACITY) {
          this.fail("replaceable-overflow", `pointer lane exceeded ${FRAME_WORKER_POINTER_CAPACITY} active identities`);
          return false;
        }
        slot = this.pointerCount++;
        this.pointerIds[slot] = event.pointerId;
      }
      this.pointerMoves[slot] = { ...event, timestampMs: this.now() } as BrowserFrameReplaceableEvent;
    }
    if (event.kind === "wheel") {
      const prior = this.wheel?.kind === "wheel" ? this.wheel : undefined;
      this.wheel = prior ? { ...event, deltaX: prior.deltaX + event.deltaX, deltaY: prior.deltaY + event.deltaY, timestampMs: this.now() } as BrowserFrameReplaceableEvent : { ...event, timestampMs: this.now() } as BrowserFrameReplaceableEvent;
    }
    if (event.kind === "resize") this.resize = { ...event, timestampMs: this.now() } as BrowserFrameReplaceableEvent;
    this.requestFrame();
    return true;
  }

  /** @emoji 🔒 Admits a lossless input only while both item and byte credits remain. */
  enqueueLossless(event: BrowserFrameLosslessEvent): boolean {
    if (!this.accepting()) return false;
    if ((event.kind === "key-down" || event.kind === "key-up") && event.key.length > FRAME_WORKER_TEXT_CHUNK_CODE_UNITS) {
      this.fail("lossless-overflow", `key payload exceeded ${FRAME_WORKER_TEXT_CHUNK_CODE_UNITS} code units`);
      return false;
    }
    if ((event.kind === "ime-update" || event.kind === "ime-commit") && event.text.length > FRAME_WORKER_TEXT_CHUNK_CODE_UNITS) {
      this.fail("lossless-overflow", `IME payload exceeded ${FRAME_WORKER_TEXT_CHUNK_CODE_UNITS} code units`);
      return false;
    }
    const bytes = admittedBytes(event);
    if (this.lossless.length >= FRAME_WORKER_LOSSLESS_ITEM_CAPACITY || this.losslessBytes + bytes > FRAME_WORKER_BYTE_CAPACITY) {
      this.fail("lossless-overflow", `lossless lane exceeded ${FRAME_WORKER_LOSSLESS_ITEM_CAPACITY} items or ${FRAME_WORKER_BYTE_CAPACITY} bytes`);
      return false;
    }
    this.lossless.push({ event, bytes, streamId: this.nextStreamId++, timestampMs: this.now(), cursor: 0 });
    this.losslessBytes += bytes;
    this.generation++;
    this.requestFrame();
    return true;
  }

  /** @emoji 🎞️ Coalesces frame requests and schedules at most one UI rAF directive turn. */
  requestFrame(): void {
    if (!this.accepting()) return;
    this.frameRequested = true;
    if (this.requestRaf && this.rafHandle === undefined) {
      this.rafHandle = this.requestRaf((timestampMs) => {
        this.rafHandle = undefined;
        this.flush(timestampMs);
      });
    }
  }

  /** @emoji 📤 Transfers one bounded batch; never executes frame work on the caller. */
  flush(timestampMs = this.now()): boolean {
    if (this.status !== "ready" || this.inFlight || !this.frameRequested) return false;
    const replaceable: BrowserFrameReplaceableEvent[] = [];
    for (let index = 0; index < this.pointerCount; index++) {
      const event = this.pointerMoves[index];
      if (event) replaceable.push(event);
    }
    if (this.wheel) replaceable.push(this.wheel);
    if (this.resize) replaceable.push(this.resize);
    const lossless = this.takeLosslessWireBatch();
    this.pointerMoves.fill(undefined);
    this.pointerCount = 0;
    this.wheel = undefined;
    this.resize = undefined;
    this.frameRequested = this.lossless.length > 0;
    const sequence = ++this.sequence;
    this.inFlight = true;
    try {
      const startedAt = this.now();
      this.worker.postMessage({ kind: "batch", lifecycle: this.lifecycle, sequence, generation: this.generation, timestampMs, replaceable, lossless });
      const duration = this.now() - startedAt;
      if (!this.observeUiTurn("frame-transfer", duration)) return false;
      return true;
    } catch (error) {
      this.fail("worker-message-failed", error instanceof Error ? error.message : String(error));
      return false;
    }
  }

  /** @emoji 🔬️ Requests one read-only introspection dump from the frame Worker's renderer thread-local.
   * Resolves `null` — never rejects and never faults the surface — when the Worker is not ready, when the
   * fixed in-flight credit is exhausted, or when the answer misses `FRAME_WORKER_INTROSPECTION_TIMEOUT_MS`,
   * so a diagnostic can distinguish "no hooks" from "empty dump" without ever taking the shell down. The
   * batch flushed first is what makes the answer meaningful: message order guarantees the Worker has ticked
   * at least one frame before it reads the retained tree. */
  introspect(probe: BrowserFrameIntrospectionProbe): Promise<string | null> {
    if (this.status !== "ready" || this.introspections.size >= FRAME_WORKER_INTROSPECTION_CAPACITY) return Promise.resolve(null);
    const requestId = this.nextIntrospectionId++;
    this.requestFrame();
    this.flush();
    return new Promise<string | null>((resolve) => {
      const timer = this.setTimer(() => {
        this.introspections.delete(requestId);
        resolve(null);
      }, FRAME_WORKER_INTROSPECTION_TIMEOUT_MS);
      this.introspections.set(requestId, { resolve, timer });
      try {
        this.worker.postMessage({ kind: "introspect", lifecycle: this.lifecycle, requestId, probe });
      } catch {
        this.introspections.delete(requestId);
        this.clearTimer(timer);
        resolve(null);
      }
    });
  }

  /** @emoji 🛑 Cancels queued work and terminates the dedicated Worker. */
  close(): void {
    if (this.status === "closed") return;
    if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
    if (this.rafHandle !== undefined) this.cancelRaf?.(this.rafHandle);
    this.requestWorkerClose();
    this.interactiveJobs.close();
    this.drainInteractiveJobs();
    this.clearQueues();
    this.status = "closed";
  }

  /** @emoji ⏱️ Records fixed-ring callback telemetry and quarantines the surface owner on budget breach. */
  observeUiTurn(site: string, durationMs: number): boolean {
    this.uiTurnSamples[this.uiTurnSampleCount % this.uiTurnSamples.length] = durationMs;
    this.uiTurnSampleCount++;
    if (durationMs < FRAME_UI_TURN_BUDGET_MS) return true;
    if (this.status === "quarantined" || this.status === "faulted" || this.status === "closed") return false;
    const detail = `${site} UI turn took ${durationMs.toFixed(3)} ms`;
    if (this.status === "ready") this.quarantine("ui-turn-overrun", detail);
    else this.fail("ui-turn-overrun", detail);
    return false;
  }

  /** @emoji 📊 Returns bounded fixed-ring p99 telemetry outside the event callback path. */
  uiTurnP99Ms(): number {
    const count = Math.min(this.uiTurnSampleCount, this.uiTurnSamples.length);
    if (count === 0) return 0;
    const samples = Array.from(this.uiTurnSamples.subarray(0, count)).sort((left, right) => left - right);
    return samples[Math.min(count - 1, Math.ceil(count * 0.99) - 1)]!;
  }

  private armBootStallTimer(): void {
    if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
    this.bootTimer = this.setTimer(() => this.fail("worker-boot-timeout", `Worker reported no boot progress for ${FRAME_WORKER_BOOT_STALL_TIMEOUT_MS} ms`), FRAME_WORKER_BOOT_STALL_TIMEOUT_MS);
  }

  private accepting(): boolean {
    return this.status === "booting" || this.status === "ready";
  }

  /** @emoji 🧵️ Spawns one shard worker HERE, on the UI isolate, and hands the frame worker a
   * `MessagePort` onto it. The frame worker cannot construct these itself: a nested dedicated worker
   * fails to load outright in some embedded browsers (measured — even a one-line worker), which
   * surfaced as four shards dying with a message-less `error` event and `create_app promise failed:
   * shard 0 terminated`. Messages are relayed verbatim in both directions, and a worker-level `error`
   * is forwarded as a `shard-worker-error` frame so `ShardClient`'s own failure ladder still runs. */
  private spawnShardWorker(shardIndex: number, url: string): void {
    this.terminateShardWorker(shardIndex);
    const worker = new Worker(url, { type: "module" });
    const channel = new MessageChannel();
    worker.onmessage = (event: MessageEvent) => channel.port1.postMessage(event.data);
    worker.onerror = (event: ErrorEvent) => channel.port1.postMessage({ kind: "shard-worker-error", message: event.message ?? "", filename: event.filename ?? "", lineno: event.lineno ?? 0 });
    channel.port1.onmessage = (event: MessageEvent) => worker.postMessage(event.data);
    channel.port1.start();
    this.shardWorkers.set(shardIndex, worker);
    this.worker.postMessage({ kind: "shard-port", shardIndex, port: channel.port2 }, [channel.port2]);
  }

  private terminateShardWorker(shardIndex: number): void {
    const existing = this.shardWorkers.get(shardIndex);
    if (!existing) return;
    this.shardWorkers.delete(shardIndex);
    existing.onmessage = null;
    existing.onerror = null;
    existing.terminate();
  }

  private receive(message: BrowserFrameWorkerMessage): void {
    if (message.kind === "shard-spawn") {
      this.spawnShardWorker(message.shardIndex, message.url);
      return;
    }
    if (message.kind === "shard-terminate") {
      this.terminateShardWorker(message.shardIndex);
      return;
    }
    if (message.lifecycle !== this.lifecycle) return;
    if (message.kind === "job-input-pull" || message.kind === "job-output-page" || message.kind === "job-terminal") {
      this.interactiveJobs.receive(message);
      return;
    }
    if (message.kind === "introspection") {
      const pending = this.introspections.get(message.requestId);
      if (!pending) return;
      this.introspections.delete(message.requestId);
      this.clearTimer(pending.timer);
      pending.resolve(message.json);
      return;
    }
    if (message.kind === "closed") {
      this.worker.terminate();
      return;
    }
    if (this.status === "closed" || this.status === "faulted" || this.status === "quarantined") return;
    if (message.kind === "booted") {
      if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
      this.bootTimer = undefined;
      this.status = "ready";
      this.interactiveJobs.ready();
      if (!this.runUiHook("ready-hook", () => this.onReady?.())) return;
      this.requestFrame();
      return;
    }
    if (message.kind === "boot-progress") {
      if (this.status !== "booting") return;
      this.armBootStallTimer();
      this.runUiHook("progress-hook", () => this.onProgress?.(message.stage, message.progress));
      return;
    }
    if (message.kind === "wake") {
      this.requestFrame();
      return;
    }
    if (message.kind === "fault") {
      this.fail(this.status === "booting" ? "worker-boot-failed" : "worker-runtime-failed", `${message.code}: ${message.detail}`);
      return;
    }
    if (message.generation > this.generation) {
      this.fail("protocol-violation", `Worker returned future generation ${message.generation} while UI generation is ${this.generation}`);
      return;
    }
    if (message.sequence <= this.acceptedSequence) return;
    this.inFlight = false;
    if (message.quarantined || message.workerDurationMs >= 8) {
      const code = message.faultCode === "present-failed" ? "worker-present-failed" : message.faultCode === "text-input-failed" ? "worker-input-failed" : "worker-step-overrun";
      this.quarantine(code, message.faultDetail ?? `worker frame step took ${message.workerDurationMs.toFixed(3)} ms`);
      return;
    }
    if (message.generation === this.generation) {
      this.acceptedSequence = message.sequence;
      if (!this.runUiHook("directive-hook", () => this.onDirectives?.({ cursor: message.cursor, fullscreen: message.fullscreen, generation: message.generation, workerDurationMs: message.workerDurationMs }))) return;
    }
    if (message.requestFrame || this.frameRequested || message.generation < this.generation) this.requestFrame();
  }

  private fail(code: BrowserFrameWorkerFaultCode, detail: string): void {
    if (this.status === "faulted" || this.status === "closed") return;
    if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
    if (this.rafHandle !== undefined) this.cancelRaf?.(this.rafHandle);
    this.requestWorkerClose();
    this.interactiveJobs.close();
    this.drainInteractiveJobs();
    this.clearQueues();
    this.fault = { code, detail };
    this.status = "faulted";
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail));
  }

  private quarantine(code: BrowserFrameWorkerFaultCode, detail: string): void {
    if (this.status !== "ready") return;
    if (this.rafHandle !== undefined) this.cancelRaf?.(this.rafHandle);
    this.interactiveJobs.quarantineFromOwner();
    this.drainInteractiveJobs();
    this.requestWorkerClose();
    this.clearQueues();
    this.fault = { code, detail };
    this.status = "quarantined";
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail));
  }

  private runUiHook(site: string, callback: () => void): boolean {
    const startedAt = this.now();
    try {
      callback();
    } catch (error) {
      const detail = `${site} threw: ${error instanceof Error ? error.message : String(error)}`;
      if (this.status === "ready") this.quarantine("ui-turn-overrun", detail);
      else if (this.status !== "quarantined" && this.status !== "faulted" && this.status !== "closed") this.fail("ui-turn-overrun", detail);
      return false;
    }
    return this.observeUiTurn(site, this.now() - startedAt);
  }

  private clearQueues(): void {
    for (const pending of this.introspections.values()) {
      this.clearTimer(pending.timer);
      pending.resolve(null);
    }
    this.introspections.clear();
    this.pointerMoves.fill(undefined);
    this.pointerCount = 0;
    this.wheel = undefined;
    this.resize = undefined;
    const retiredLossless = this.lossless;
    this.lossless = [];
    const drain = () => {
      retiredLossless.pop();
      if (retiredLossless.length > 0) this.setTimer(drain, 0);
    };
    if (retiredLossless.length > 0) this.setTimer(drain, 0);
    this.losslessBytes = 0;
    this.frameRequested = false;
    this.inFlight = false;
  }

  private drainInteractiveJobs(): void {
    if (this.interactiveJobs.closeStep()) return;
    this.setTimer(() => this.drainInteractiveJobs(), 0);
  }

  private requestWorkerClose(): void {
    if (this.closeRequested) return;
    this.closeRequested = true;
    try {
      this.worker.postMessage({ kind: "close", lifecycle: this.lifecycle });
    } catch {}
  }

  private takeLosslessWireBatch(): BrowserFrameWireLosslessEvent[] {
    const batch: BrowserFrameWireLosslessEvent[] = [];
    let budget = FRAME_WORKER_MESSAGE_BYTE_CAPACITY - 2048;
    while (batch.length < 16 && this.lossless.length > 0 && budget > 256) {
      const queued = this.lossless[0]!;
      const event = queued.event;
      if (event.kind !== "text" && event.kind !== "paste" && event.kind !== "ime-update" && event.kind !== "ime-commit") {
        batch.push({ ...event, timestampMs: queued.timestampMs } as BrowserFrameWireLosslessEvent);
        budget -= Math.min(queued.bytes, 512);
        this.lossless.shift();
        this.losslessBytes -= queued.bytes;
        continue;
      }
      const remaining = event.text.length - queued.cursor;
      const take = Math.min(remaining, FRAME_WORKER_TEXT_CHUNK_CODE_UNITS, Math.max(1, Math.floor((budget - 512) / 6)));
      let end = queued.cursor + take;
      if (end < event.text.length && isHighSurrogate(event.text.charCodeAt(end - 1)) && isLowSurrogate(event.text.charCodeAt(end))) end--;
      const final = end === event.text.length;
      batch.push({ kind: "text-chunk", streamId: queued.streamId, target: event.kind, text: event.text.slice(queued.cursor, end), totalBytes: 3 * event.text.length, final, timestampMs: queued.timestampMs, ...(event.kind === "ime-update" ? { cursor: event.cursor } : {}) } as BrowserFrameWireLosslessEvent);
      queued.cursor = end;
      budget -= 6 * take + 512;
      if (final) {
        this.lossless.shift();
        this.losslessBytes -= queued.bytes;
      }
    }
    return batch;
  }
}

function admittedBytes(event: BrowserFrameLosslessEvent): number {
  if (event.kind === "text" || event.kind === "paste" || event.kind === "ime-update" || event.kind === "ime-commit") return 3 * event.text.length + 128;
  if (event.kind === "key-down" || event.kind === "key-up") return 2 * event.key.length + 128;
  return 128;
}

function isHighSurrogate(value: number): boolean {
  return value >= 0xd800 && value <= 0xdbff;
}

function isLowSurrogate(value: number): boolean {
  return value >= 0xdc00 && value <= 0xdfff;
}
// #endregion 📮️Transport

import { BrowserInteractiveJobPort } from "../🔌️browser-interactive-job-port/🟦️.ts";
import { type BrowserFrameDiagnostic, type BrowserFrameDirectives, type BrowserFrameFallbackState, type BrowserFrameIntrospectionProbe, type BrowserFrameLosslessEvent, type BrowserFrameReplaceableEvent, type BrowserFrameTransportOptions, type BrowserFrameWireLosslessEvent, type BrowserFrameWorkerFaultCode, type BrowserFrameWorkerMessage, type BrowserFrameWorkerPort, type BrowserFrameWorkerStatus, type BrowserFrameWorkerStepReport, type BrowserHubDocumentRemote, FRAME_WORKER_ACCESSIBILITY_ID_BYTES, FRAME_WORKER_BYTE_CAPACITY, FRAME_WORKER_HUB_DOCUMENT_CAPACITY, FRAME_WORKER_IMAGE_DECODE_BYTE_CAPACITY, FRAME_WORKER_INTROSPECTION_CAPACITY, FRAME_WORKER_INTROSPECTION_TIMEOUT_MS, FRAME_WORKER_LOSSLESS_ITEM_CAPACITY, FRAME_WORKER_MESSAGE_BYTE_CAPACITY, FRAME_WORKER_POINTER_CAPACITY, FRAME_WORKER_TEXT_CHUNK_CODE_UNITS, type QueuedLossless } from "./📋️contract/🟦️.ts";
import { TurnClock, TurnLedger, type TurnOutcome } from "../⏱️turn-budget/🟦️.ts";
import { BROWSER_MEDIA_CAPACITY, type BrowserMediaOperation, type BrowserMediaValue, mediaSlotAuthorityKey, parsePresentedMediaSlots, type PresentedMediaSlot } from "@semio-tech/framework-os/modules/renderer/engine/media/browser";
import { bootPhaseCeilingMs, type BrowserBootPhase, describeBrowserBootSilence, evaluateBrowserBootLiveness, FRAME_WORKER_BOOT_LIVENESS_POLICY } from "../🫀️boot-liveness/🟦️.ts";
import { OriginalBootstrapRootPort } from "@semio-tech/framework-actor/retained-turn/bootstrap/port";
import type { MediaTransportPort } from "@semio-tech/framework-os/modules/renderer/engine/media/lifecycle";
import { nextFrameSequence } from "../🧵️frame-turn-scheduler/🟦️.ts";
import type { WgpuHostAppearance, WgpuHostStorageSnapshot } from "../🧭️boot-descriptor/🟦️.ts";
import { stampShardWorkerDiagnostics } from "@semio-tech/framework-actor/diagnostics";
import { decodeReferenceImageOnPage } from "../🖼️reference-image-decode/🟦️.ts";

// #endregion 🔖️Protocol

// #region 📮️Transport
/** 📮️ Owns one fail-closed frame Worker lifecycle and its bounded admission state. */
export class BrowserFrameTransport {
  private resolveRetirement: () => void = () => {};
  readonly retirement = new Promise<void>(resolve => { this.resolveRetirement = resolve; });
  readonly lifecycle = 1;
  readonly interactiveJobs: BrowserInteractiveJobPort;
  status: BrowserFrameWorkerStatus = "booting";
  fault: { readonly code: BrowserFrameWorkerFaultCode; readonly detail: string;readonly originalFailure?:{readonly format:"pack";readonly bytes:Uint8Array} } | undefined;
  private readonly worker: BrowserFrameWorkerPort;
  private readonly shardWorkers = new Map<number, Worker>();
  private readonly shardChannels = new Map<number, MessagePort>();
  private workerRetired = false;
  private readonly now: () => number;
  private readonly clearTimer: (handle: number) => void;
  private readonly setTimer: (callback: () => void, delayMs: number) => number;
  private readonly onReady?: () => void;
  private readonly onProgress?: (stage: string, progress: number, worker: BrowserFrameWorkerStepReport) => void;
  private readonly hostIo?: (requestJson: string, bytes: Uint8Array | null) => Promise<string>;
  /** 🧵️ The frame Worker's last reported step ledger — a measurement the boot UI renders. */
  private workerSteps: BrowserFrameWorkerStepReport = { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" };
  /** 🧵️ Frame steps the Worker attributed to its OWN work (a sustained run, not one wall sample). */
  private workerStepOverruns = 0;
  private readonly onDirectives?: (directives: BrowserFrameDirectives) => void;
  private readonly onDiagnostic?: (diagnostic: BrowserFrameDiagnostic) => void;
  private readonly onFault?: (code: BrowserFrameWorkerFaultCode, detail: string, fallback: BrowserFrameFallbackState,originalFailure?:{readonly format:"pack";readonly bytes:Uint8Array}) => void;
  private readonly onUiTurn?: (outcome: TurnOutcome) => void;
  private readonly requestRaf?: (callback: FrameRequestCallback) => number;
  private readonly cancelRaf?: (handle: number) => void;
  private readonly pointerIds = new Array<number>(FRAME_WORKER_POINTER_CAPACITY);
  private readonly pointerMoves = new Array<BrowserFrameReplaceableEvent | undefined>(FRAME_WORKER_POINTER_CAPACITY);
  private pointerCount = 0;
  private wheel: BrowserFrameReplaceableEvent | undefined;
  private resize: BrowserFrameReplaceableEvent | undefined;
  private lossless: QueuedLossless[] = [];
  private losslessBytes = 0;
  private readonly hubDocuments = new Map<string, BrowserHubDocumentRemote | null>();
  private nextStreamId = 1;
  private generation = 0;
  private inputSequence = 0;
  private acceptedInputSequence = 0;
  private acceptedFrameSequence = 0;
  private inFlight = false;
  private frameRequested = false;
  private mediaSlots = new Map<string, PresentedMediaSlot>();
  private mediaRequestId = 0;
  private readonly mediaPending = new Map<number, { readonly token: string; readonly resolve: (value: BrowserMediaValue) => void; readonly reject: (error: Error) => void; readonly timer: number }>();
  private rafHandle: number | undefined;
  private deadlineTimer: number | undefined;
  private bootTimer: number | undefined;
  /** 🫀️ The last instant ANY message arrived from the frame Worker — the clock silence is measured
   * against. `NEGATIVE_INFINITY` until the Worker speaks for the first time, so "never sent a single
   * message" stays distinguishable from "went quiet". */
  private lastLivenessAtMs = Number.NEGATIVE_INFINITY;
  private bootStage = "";
  private bootPhase: BrowserBootPhase | undefined;
  private bootStartedAtMs = 0;
  private readonly locale: "en" | "de";
  private closeRequested = false;
  private readonly uiTurns = new TurnLedger();
  private readonly uiTurnClock: TurnClock;
  private deferredWork: (() => void)[] = [];
  private deferredScheduled = false;
  private readonly introspections = new Map<number, { readonly resolve: (json: string | null) => void; readonly timer: number }>();
  private nextIntrospectionId = 1;
  private readonly imageDecodes = new Map<number, AbortController>();

  readonly #originalWork: OriginalBootstrapRootPort;
  constructor(options: BrowserFrameTransportOptions) {
    this.#originalWork = new OriginalBootstrapRootPort(options.originalWorkStock, response => this.worker.postMessage(response));
    this.worker = options.worker;
    this.now = options.now ?? (() => performance.now());
    this.locale = options.boot.locale === "de" ? "de" : "en";
    this.bootStartedAtMs = this.now();
    const setTimer = options.setTimer ?? ((callback, delayMs) => window.setTimeout(callback, delayMs));
    this.setTimer = setTimer;
    this.clearTimer = options.clearTimer ?? ((handle) => window.clearTimeout(handle));
    this.onReady = options.onReady;
    this.onProgress = options.onProgress;
    this.onDirectives = options.onDirectives;
    this.onDiagnostic = options.onDiagnostic;
    this.onFault = options.onFault;
    this.onUiTurn = options.onUiTurn;
    this.uiTurnClock = new TurnClock(this.now);
    this.requestRaf = options.requestAnimationFrame;
    this.cancelRaf = options.cancelAnimationFrame;
    this.hostIo = options.hostIo;
    this.interactiveJobs = new BrowserInteractiveJobPort(this.lifecycle, (message) => this.worker.postMessage(message), this.now, (detail) => this.quarantine("interactive-job-violation", detail), (callback) => void this.setTimer(callback, 0));
    this.worker.onmessage = (event) => this.receive(event.data);
    this.worker.onerror = (event) => { this.fail("worker-message-failed", event.message || "Worker error"); this.retireWorker(); };
    this.worker.onmessageerror = () => this.fail("worker-message-failed", "Worker message could not be decoded");
    this.armBootWatchdog(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs);
    try {
      this.worker.postMessage({ kind: "boot", lifecycle: this.lifecycle, ...options.boot }, [options.boot.canvas]);
    } catch (error) {
      this.fail("worker-boot-failed", error instanceof Error ? error.message : String(error));
    }
  }

  /** 🫧 Coalesces replaceable input without consuming lossless credits. */
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

  /** 🔒 Admits a lossless input only while both item and byte credits remain. */
  enqueueLossless(event: BrowserFrameLosslessEvent): boolean {
    if (!this.accepting()) return false;
    if (event.kind === "hub-document-status") return this.publishHubDocumentStatus(event.documentKey, event.remote);
    if (event.kind === "hub-document-close") return this.publishHubDocumentStatus(event.documentKey, null);
    if ((event.kind === "key-down" || event.kind === "key-up") && event.key.length > FRAME_WORKER_TEXT_CHUNK_CODE_UNITS) {
      this.fail("lossless-overflow", `key payload exceeded ${FRAME_WORKER_TEXT_CHUNK_CODE_UNITS} code units`);
      return false;
    }
    if ((event.kind === "ime-update" || event.kind === "ime-commit") && event.text.length > FRAME_WORKER_TEXT_CHUNK_CODE_UNITS) {
      this.fail("lossless-overflow", `IME payload exceeded ${FRAME_WORKER_TEXT_CHUNK_CODE_UNITS} code units`);
      return false;
    }
    if (event.kind === "accessibility-focus" || event.kind === "accessibility-blur" || event.kind === "accessibility-activate" || event.kind === "accessibility-value") {
      const invalidText = (value: string) => value.length === 0 || new TextEncoder().encode(value).byteLength > FRAME_WORKER_ACCESSIBILITY_ID_BYTES || /[\u0000-\u001f\u007f]/u.test(value);
      if (invalidText(event.windowId) || invalidText(event.nodeKey) || !Number.isSafeInteger(event.windowGeneration) || event.windowGeneration < 0 || !Number.isSafeInteger(event.nodeId) || event.nodeId < 0) {
        this.fail("lossless-overflow", "accessibility address is outside fixed identity credits");
        return false;
      }
      if (event.kind === "accessibility-value" && event.value.length > FRAME_WORKER_TEXT_CHUNK_CODE_UNITS) {
        this.fail("lossless-overflow", `accessibility value exceeded ${FRAME_WORKER_TEXT_CHUNK_CODE_UNITS} code units`);
        return false;
      }
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

  publishHubDocumentStatus(documentKey: string, remote: BrowserHubDocumentRemote | null): boolean {
    if (!this.accepting() || documentKey.length === 0 || documentKey.length > 512 || /[\u0000-\u001f\u007f]/u.test(documentKey)) return false;
    if (!this.hubDocuments.has(documentKey) && this.hubDocuments.size >= FRAME_WORKER_HUB_DOCUMENT_CAPACITY) return false;
    this.hubDocuments.set(documentKey, remote);
    this.generation++;
    this.requestFrame();
    return true;
  }

  /** 🎬️ Creates the serial runner's bounded port for an accepted resource token. */
  mediaPort(slotToken: string): MediaTransportPort {
    const send = (instanceId: number, operation: BrowserMediaOperation): Promise<BrowserMediaValue> => {
      const slot = this.mediaSlots.get(slotToken);
      if (this.status !== "ready" || !slot || slot.appInstanceId !== instanceId) return Promise.reject(new Error("browser-media.retired"));
      if ([...this.mediaPending.values()].some((pending) => pending.token === slotToken)) return Promise.reject(new Error("browser-media.in-flight"));
      if (this.mediaPending.size >= BROWSER_MEDIA_CAPACITY.slots) return Promise.reject(new Error("browser-media.capacity"));
      const requestId = ++this.mediaRequestId;
      return new Promise((resolve, reject) => {
        const timer = this.setTimer(() => {
          this.mediaPending.delete(requestId);
          this.mediaSlots.delete(slotToken);
          try { this.worker.postMessage({ kind: "media-release", lifecycle: this.lifecycle, slotToken }); } catch {}
          reject(new Error("browser-media.timeout"));
        }, BROWSER_MEDIA_CAPACITY.commandTimeoutMs);
        this.mediaPending.set(requestId, { token: slotToken, resolve, reject, timer });
        try { this.worker.postMessage({ kind: "media-command", lifecycle: this.lifecycle, requestId, slotToken, ...operation }); }
        catch (error) { this.mediaPending.delete(requestId); this.clearTimer(timer); reject(error instanceof Error ? error : new Error(String(error))); }
      });
    };
    return {
      submitMediaExport: async (instanceId, port, parentDocumentId, revision) => {
        const slot = this.mediaSlots.get(slotToken);
        if (!slot?.props.resource || port !== slot.props.resource.outputPort || parentDocumentId !== slot.parentDocumentId || revision !== BigInt(slot.props.revision)) throw new Error("browser-media.owner");
        return await send(instanceId, { operation: "submit" }) as Awaited<ReturnType<MediaTransportPort["submitMediaExport"]>>;
      },
      pollMediaExport: async (instanceId, handle) => await send(instanceId, { operation: "poll", handle }) as Awaited<ReturnType<MediaTransportPort["pollMediaExport"]>>,
      cancelMediaExport: async (instanceId, handle) => { await send(instanceId, { operation: "cancel", handle }); },
      takeMediaExportChunk: async (instanceId, handle) => await send(instanceId, { operation: "take", handle }) as Awaited<ReturnType<MediaTransportPort["takeMediaExportChunk"]>>,
    };
  }

  private acceptMediaSlots(value: unknown): readonly PresentedMediaSlot[] {
    const slots = parsePresentedMediaSlots(value);
    const next = new Map(slots.map((slot) => [slot.token, slot]));
    for (const [requestId, pending] of this.mediaPending) {
      const before = this.mediaSlots.get(pending.token);
      const after = next.get(pending.token);
      if (!before || !after || mediaSlotAuthorityKey(before) !== mediaSlotAuthorityKey(after)) {
        this.mediaPending.delete(requestId);
        this.clearTimer(pending.timer);
        pending.reject(new Error("browser-media.retired"));
      }
    }
    this.mediaSlots = next;
    return slots;
  }

  /** 🎞️ Coalesces frame requests and schedules at most one UI rAF directive turn. */
  requestFrame(): void {
    if (!this.accepting()) return;
    this.clearDeadlineTimer();
    this.frameRequested = true;
    if (this.requestRaf && this.rafHandle === undefined) {
      this.rafHandle = this.requestRaf((timestampMs) => {
        this.rafHandle = undefined;
        this.flush(timestampMs);
      });
    }
  }

  /** 📤 Transfers one bounded batch; never executes frame work on the caller. */
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
    this.frameRequested = this.lossless.length > 0 || this.hubDocuments.size > 0;
    try {
      const inputSequence = nextFrameSequence(this.inputSequence);
      this.inputSequence = inputSequence;
      this.inFlight = true;
      this.uiTurnClock.enter();
      this.worker.postMessage({ kind: "batch", lifecycle: this.lifecycle, inputSequence, generation: this.generation, timestampMs, replaceable, lossless });
      this.observeUiTurn("frame-transfer", this.uiTurnClock.leave());
      return true;
    } catch (error) {
      this.uiTurnClock.leave();
      this.fail("worker-message-failed", error instanceof Error ? error.message : String(error));
      return false;
    }
  }

  /** 🎓️ Publishes the owning host's live introduction policy and schedules its next frame. */
  setHostIntroductionSuppressed(suppressed: boolean): void {
    if (this.status === "faulted" || this.status === "closed") return;
    this.worker.postMessage({ kind: "host-introduction-policy", lifecycle: this.lifecycle, suppressed });
    this.requestFrame();
  }

  /** 🌓️ Republishes the page realm's appearance reads. Fire-and-forget by construction: a
   * theme flip must never fault a surface, and the Worker simply keeps the last value it was given.
   * The frame it requests afterwards is what makes the change visible — the renderer re-resolves its
   * theme every frame build, so nothing else has to be invalidated. */
  setHostAppearance(appearance: WgpuHostAppearance): void {
    if (this.status === "faulted" || this.status === "closed") return;
    try {
      this.worker.postMessage({ kind: "host-appearance", lifecycle: this.lifecycle, appearance });
      this.requestFrame();
    } catch {}
  }

  /** 🫥️ Tells the Worker the page was hidden. Posted at once, never batched: a hidden page gets no animation frame, so
   * the Worker runs its own frame turn to send every typing run's `hidden` commit. Fire-and-forget like {@link setHostAppearance}. */
  setHostPageHidden(): void {
    if (this.status === "faulted" || this.status === "closed") return;
    try {
      this.worker.postMessage({ kind: "host-page-hidden", lifecycle: this.lifecycle });
    } catch {}
  }

  /** 🫥️ Tells the Worker the page's `window` lost focus. Posted at once, never batched, fire-and-forget like
   * {@link setHostPageHidden}. */
  setHostWindowBlur(): void {
    if (this.status === "faulted" || this.status === "closed") return;
    try {
      this.worker.postMessage({ kind: "host-window-blur", lifecycle: this.lifecycle });
    } catch {}
  }

  /** 🛰️ Hands the page's latest agent-bridge offer to the Worker. Fire-and-forget like {@link setHostAppearance}:
   * an offer that cannot cross leaves the bridge `Disabled`, never a faulted surface. */
  setHostAgentBridge(offer: { readonly url: string; readonly admissionProof: string } | null): void {
    if (this.status === "faulted" || this.status === "closed") return;
    try {
      this.worker.postMessage({ kind: "host-agent-bridge", lifecycle: this.lifecycle, offer });
      this.requestFrame();
    } catch {}
  }

  /** 🗄️ Re-seeds the Worker's synchronous preference cache from the page's store. Fire-and-forget
   * for the same reason {@link setHostAppearance} is: a preference another tab changed must never fault
   * a surface. It requests no frame — nothing repaints on a storage change by itself; the next read of
   * the affected key is what observes it. */
  setHostStorage(storage: WgpuHostStorageSnapshot): void {
    if (this.status === "faulted" || this.status === "closed") return;
    try {
      this.worker.postMessage({ kind: "host-storage", lifecycle: this.lifecycle, storage });
    } catch {}
  }

  /** 🔬️ Requests one read-only introspection dump from the frame Worker's renderer thread-local.
   * Resolves `null` — never rejects and never faults the surface — when the Worker is not ready, when the
   * fixed in-flight credit is exhausted, or when the answer misses `FRAME_WORKER_INTROSPECTION_TIMEOUT_MS`,
   * so a diagnostic can distinguish "no hooks" from "empty dump" without ever taking the shell down. The
   * batch flushed first is what makes the answer meaningful: message order guarantees the Worker has ticked
   * at least one frame before it reads the retained tree. */
  introspect(probe: BrowserFrameIntrospectionProbe, windowId?: string): Promise<string | null> {
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
        this.worker.postMessage({ kind: "introspect", lifecycle: this.lifecycle, requestId, probe, ...(windowId === undefined ? {} : { windowId }) });
      } catch {
        this.introspections.delete(requestId);
        this.clearTimer(timer);
        resolve(null);
      }
    });
  }

  /** 🛑 Cancels queued work and terminates the dedicated Worker. */
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

  /** ⏱️ Prices ONE UI turn and records it. Never a verdict: an overrun cannot quarantine, fail or
   * close this surface, because on the UI isolate a wall reading over the ceiling is as often the machine
   * descheduling a hidden pane as it is the turn's own work — the attribution lives in `TurnLedger`'s
   * sustained-run law, and its only consequence is deferred cadence. Answers whether the turn was
   * admitted, so a caller may choose to defer its remaining work to the next frame. */
  observeUiTurn(site: string, executingMs: number | undefined): boolean {
    const outcome = this.uiTurns.admit(site, executingMs);
    if (outcome.verdict !== "admitted" && outcome.verdict !== "clock-fault") this.onUiTurn?.(outcome);
    return outcome.verdict === "admitted" || outcome.verdict === "clock-fault";
  }

  /** 🐢️ Whether the surface is running its turns on deferred cadence after a sustained run of
   * overruns. Clears itself on the first turn that fits the ceiling again. */
  degraded(): boolean {
    return this.uiTurns.degraded();
  }

  /** 🪂️ The surface's real fallback state — what a fault banner must report instead of claiming
   * a UI-thread frame path was or was not attempted. */
  fallbackState(): BrowserFrameFallbackState {
    return {
      surface: this.status,
      uiThreadFrames: "unavailable-offscreen-transferred",
      workerTerminated: this.workerRetired,
      inputAccepted: this.accepting(),
      deferredCadence: this.uiTurns.degraded(),
      uiTurns: this.uiTurns.snapshot(),
      workerSteps: { ...this.workerSteps, sustainedOverruns: this.workerSteps.sustainedOverruns + this.workerStepOverruns },
      bootPhase: this.bootPhase,
      bootPhaseElapsedMs: this.bootPhase ? Math.max(0, this.now() - this.bootPhase.enteredAtMs) : 0,
      bootStage: this.bootStage,
      bootSilentForMs: Math.max(0, this.now() - (Number.isFinite(this.lastLivenessAtMs) ? this.lastLivenessAtMs : this.bootStartedAtMs)),
    };
  }

  /** 📊 Returns bounded fixed-ring p99 telemetry outside the event callback path. */
  uiTurnP99Ms(): number {
    return this.uiTurns.p99Ms();
  }

  /** ⏭️ Yields and continues: hands the remainder of an overrunning turn to the next macrotask so
   * the isolate can paint and pump input between the pieces. Bounded by the caller's own queue — the
   * transport enqueues at most one continuation per hook site. */
  private deferToNextTurn(work: () => void): void {
    this.deferredWork.push(work);
    if (this.deferredScheduled) return;
    this.deferredScheduled = true;
    void this.setTimer(() => {
      this.deferredScheduled = false;
      const pending = this.deferredWork;
      this.deferredWork = [];
      for (const item of pending) {
        if (this.status === "closed") return;
        this.uiTurnClock.enter();
        try {
          item();
        } catch (error) {
          this.uiTurnClock.leave();
          this.fail("ui-hook-failed", `deferred UI turn threw: ${error instanceof Error ? error.message : String(error)}`);
          return;
        }
        this.observeUiTurn("deferred-hook", this.uiTurnClock.leave());
      }
    }, 0);
  }

  /** 🫀️ Arms ONE self-rescheduling watchdog wake. Nothing re-arms it on every inbound message any
   * more: a message only stamps {@link lastLivenessAtMs}, and the wake below re-reads the whole window and
   * schedules itself for the exact remaining time. One timer for the whole boot instead of one per
   * `boot-progress`, and the deadline is exact rather than a whole window late. */
  private armBootWatchdog(delayMs: number): void {
    if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
    this.bootTimer = this.setTimer(() => this.judgeBootLiveness(), Math.max(0, delayMs));
  }

  /** ⚖️ One watchdog window, decided by `../🫀️boot-liveness/🟦️.ts` and never here — so the same
   * verdict replays from a timeline with no transport in the picture. A declared long phase inside its own
   * ceiling is BUSY and re-arms; only an undeclared silence past the ceiling is a wedged event loop, and
   * the fault it raises names the phase, its elapsed and its ceiling in the reader's tongue. */
  private judgeBootLiveness(): void {
    this.bootTimer = undefined;
    if (this.status !== "booting") return;
    const nowMs = this.now();
    const window = { nowMs, lastLivenessAtMs: this.lastLivenessAtMs, silenceTimeoutMs: FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs, phase: this.bootPhase };
    const decision = evaluateBrowserBootLiveness(window);
    if (!decision.terminate) {
      this.armBootWatchdog(decision.rearmInMs);
      return;
    }
    const detail = describeBrowserBootSilence({ heard: Number.isFinite(this.lastLivenessAtMs), lastStage: this.bootStage, silentForMs: decision.silentForMs, silenceTimeoutMs: window.silenceTimeoutMs, phase: this.bootPhase, phaseElapsedMs: decision.phaseElapsedMs }, this.locale);
    this.fail("worker-boot-timeout", detail);
  }

  /** 🫀️ Stamps the instant the Worker last proved it was running. */
  private witnessWorker(): void {
    this.lastLivenessAtMs = this.now();
  }

  private accepting(): boolean {
    return this.status === "booting" || this.status === "ready";
  }

  /** 🧵️ Spawns one shard worker HERE, on the UI isolate, and hands the frame worker a
   * `MessagePort` onto it. The frame worker cannot construct these itself: a nested dedicated worker
   * fails to load outright in some embedded browsers (measured — even a one-line worker), which
   * surfaced as four shards dying with a message-less `error` event and `create_app promise failed:
   * shard 0 terminated`. Messages are relayed verbatim in both directions, and a worker-level `error`
   * is forwarded as a `shard-worker-error` frame so `ShardClient`'s own failure ladder still runs. */
  private spawnShardWorker(shardIndex: number, url: string): void {
    this.terminateShardWorker(shardIndex);
    const worker = new Worker(stampShardWorkerDiagnostics(url), { type: "module" });
    const channel = new MessageChannel();
    worker.onmessage = (event: MessageEvent) => channel.port1.postMessage(event.data);
    worker.onerror = (event: ErrorEvent) => channel.port1.postMessage({ kind: "shard-worker-error", message: event.message ?? "", filename: event.filename ?? "", lineno: event.lineno ?? 0 });
    channel.port1.onmessage = (event: MessageEvent) => worker.postMessage(event.data);
    channel.port1.start();
    this.shardWorkers.set(shardIndex, worker);
    this.shardChannels.set(shardIndex, channel.port1);
    this.worker.postMessage({ kind: "shard-port", shardIndex, port: channel.port2 }, [channel.port2]);
  }

  private terminateShardWorker(shardIndex: number): void {
    const existing = this.shardWorkers.get(shardIndex);
    if (!existing) return;
    this.shardWorkers.delete(shardIndex);
    const channel = this.shardChannels.get(shardIndex);
    this.shardChannels.delete(shardIndex);
    if (channel) { channel.onmessage = null; channel.close(); }
    existing.onmessage = null;
    existing.onerror = null;
    existing.terminate();
  }

  /** 🚪️ Runs one Worker-issued file-door request on THIS isolate — the only one with a `document` — and
   * answers it by `requestId`. A refusal is named on the wire so the shell can log why the user saw
   * nothing, instead of the silent `None` its own `web_sys::window()` used to return inside the Worker. */
  private answerHostIo(message: Extract<BrowserFrameWorkerMessage, { readonly kind: "host-io" }>): void {
    const reply = (json: string | null, detail?: string): void => {
      try {
        this.worker.postMessage({ kind: "host-io-result", lifecycle: this.lifecycle, requestId: message.requestId, json, detail });
      } catch {}
    };
    if (!this.hostIo) {
      reply(null, "wgpu-host-io: this transport was constructed without a page host-io door");
      return;
    }
    void this.hostIo(message.request, message.bytes).then(
      (json) => reply(json),
      (error: unknown) => reply(null, error instanceof Error ? error.message : String(error)),
    );
  }

  private answerImageDecode(message: Extract<BrowserFrameWorkerMessage, { readonly kind: "image-decode" }>): void {
    const reply = (bitmap: ImageBitmap | null, detail?: string): void => {
      if (!this.accepting()) {
        bitmap?.close();
        return;
      }
      try {
        this.worker.postMessage({ kind: "image-decode-result", lifecycle: this.lifecycle, requestId: message.requestId, bitmap, ...(detail === undefined ? {} : { detail }) }, bitmap ? [bitmap] : []);
      } catch {
        bitmap?.close();
      }
    };
    if (!this.accepting()) return;
    const expected = Number.isSafeInteger(message.dimensions.width) && Number.isSafeInteger(message.dimensions.height)
      ? message.dimensions.width * message.dimensions.height * 4
      : Number.POSITIVE_INFINITY;
    if (this.imageDecodes.size >= 1 || message.source.byteLength < 1 || message.source.byteLength > FRAME_WORKER_IMAGE_DECODE_BYTE_CAPACITY || expected < 4 || expected > 64 * 1024 * 1024) {
      reply(null, "reference-image-decode-credits");
      return;
    }
    const controller = new AbortController();
    this.imageDecodes.set(message.requestId, controller);
    const source = new Blob([message.source], { type: message.dimensions.mediaType });
    void decodeReferenceImageOnPage(source, message.dimensions, controller.signal).then(
      (bitmap) => reply(bitmap),
      (error: unknown) => reply(null, error instanceof Error ? `${error.name}: ${error.message}` : String(error)),
    ).finally(() => {
      if (this.imageDecodes.get(message.requestId) === controller) this.imageDecodes.delete(message.requestId);
    });
  }

  private receive(message: BrowserFrameWorkerMessage): void {
    if (message.kind === "original-stock-request") { void this.#originalWork.receive(message); return; }
    if (message.kind === "shard-spawn") {
      if (this.status !== "booting" && this.status !== "ready") return;
      this.runUiHook("shard-spawn", () => this.spawnShardWorker(message.shardIndex, message.url));
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
    if (message.kind === "host-io") {
      this.answerHostIo(message);
      return;
    }
    if (message.kind === "image-decode") {
      this.answerImageDecode(message);
      return;
    }
    if (message.kind === "image-decode-cancel") {
      this.imageDecodes.get(message.requestId)?.abort();
      return;
    }
    if (message.kind === "media-result") {
      const pending = this.mediaPending.get(message.requestId);
      if (!pending || pending.token !== message.slotToken) return;
      this.mediaPending.delete(message.requestId);
      this.clearTimer(pending.timer);
      if (message.fault !== undefined) pending.reject(new Error(message.fault));
      else if (message.result !== undefined) pending.resolve(message.result);
      else pending.reject(new Error("browser-media.missing-result"));
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
    if (message.kind === "diagnostic") {
      this.runUiHook("diagnostic-hook", () => this.onDiagnostic?.({ channel: message.channel, generation: message.generation, frameSequence: message.frameSequence, json: message.json }));
      return;
    }
    if (message.kind === "closed") {
      this.retireWorker();
      return;
    }
    if (this.status === "closed" || this.status === "faulted" || this.status === "quarantined") return;
    if (message.kind === "booted") {
      if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
      this.bootTimer = undefined;
      this.bootPhase = undefined;
      this.witnessWorker();
      this.status = "ready";
      this.interactiveJobs.ready();
      if (!this.runUiHook("ready-hook", () => this.onReady?.())) return;
      this.requestFrame();
      return;
    }
    if (message.kind === "boot-liveness") {
      if (this.status === "booting") this.witnessWorker();
      return;
    }
    if (message.kind === "boot-phase") {
      if (this.status !== "booting") return;
      this.witnessWorker();
      if (message.state === "enter") this.bootPhase = { phase: message.phase, ceilingMs: bootPhaseCeilingMs(message.phase), enteredAtMs: this.now() };
      else if (this.bootPhase?.phase === message.phase) this.bootPhase = undefined;
      return;
    }
    if (message.kind === "boot-progress") {
      if (this.status !== "booting") return;
      this.witnessWorker();
      this.bootStage = message.stage;
      this.workerSteps = message.worker;
      const report = () => this.onProgress?.(message.stage, message.progress, message.worker);
      if (this.degraded() || message.worker.degraded) this.deferToNextTurn(report);
      else this.runUiHook("progress-hook", report);
      return;
    }
    if (message.kind === "wake") {
      this.requestFrame();
      return;
    }
    if (message.kind === "fault") {
      this.fail(this.status === "booting" ? "worker-boot-failed" : "worker-runtime-failed", `${message.code}: ${message.detail}`,message.originalFailure);
      return;
    }
    if (message.kind === "batch-accepted") {
      if (message.generation > this.generation || message.inputSequence > this.inputSequence) {
        this.fail("protocol-violation", `Worker accepted future input ${message.inputSequence}/${message.generation} while UI is ${this.inputSequence}/${this.generation}`);
        return;
      }
      if (message.inputSequence <= this.acceptedInputSequence) return;
      this.acceptedInputSequence = message.inputSequence;
      if (message.inputSequence === this.inputSequence) this.inFlight = false;
      if (this.frameRequested || message.generation < this.generation) this.requestFrame();
      return;
    }
    if (message.generation > this.generation) {
      this.fail("protocol-violation", `Worker returned future generation ${message.generation} while UI generation is ${this.generation}`);
      return;
    }
    if (message.frameSequence <= this.acceptedFrameSequence) return;
    this.acceptedFrameSequence = message.frameSequence;
    if (message.workerStepVerdict === "sustained-overrun") this.workerStepOverruns++;
    if (message.quarantined) {
      const code = message.faultCode === "present-failed" ? "worker-present-failed" : message.faultCode === "text-input-failed" ? "worker-input-failed" : message.faultCode === "frame-credits" ? "worker-frame-failed" : "worker-step-overrun";
      this.quarantine(code, message.faultDetail ?? `worker frame step executed ${message.workerExecutingMs.toFixed(3)} ms`);
      return;
    }
    if (message.generation === this.generation) {
      let mediaSlots: readonly PresentedMediaSlot[];
      try { mediaSlots = this.acceptMediaSlots(message.mediaSlots); }
      catch (error) { this.fail("protocol-violation", error instanceof Error ? error.message : String(error)); return; }
      if (!this.runUiHook("directive-hook", () => this.onDirectives?.({ mediaSlots, cursor: message.cursor, fullscreen: message.fullscreen, generation: message.generation, workerDurationMs: message.workerDurationMs }))) return;
    }
    if (message.requestFrame || this.frameRequested || message.generation < this.generation) this.requestFrame();
    else if (message.generation === this.generation) this.replaceDeadlineTimer(message.nextDeadlineDelayMs);
  }

  private clearDeadlineTimer(): void {
    if (this.deadlineTimer !== undefined) this.clearTimer(this.deadlineTimer);
    this.deadlineTimer = undefined;
  }

  private replaceDeadlineTimer(delayMs: number | null): void {
    this.clearDeadlineTimer();
    if (delayMs === null || !Number.isFinite(delayMs) || delayMs < 0) return;
    this.deadlineTimer = this.setTimer(() => {
      this.deadlineTimer = undefined;
      this.requestFrame();
    }, Math.min(Math.ceil(delayMs), 2_147_483_647));
  }

  private fail(code: BrowserFrameWorkerFaultCode, detail: string,originalFailure?:{readonly format:"pack";readonly bytes:Uint8Array}): void {
    if (this.status === "faulted" || this.status === "closed") return;
    if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
    if (this.rafHandle !== undefined) this.cancelRaf?.(this.rafHandle);
    this.requestWorkerClose();
    this.interactiveJobs.close();
    this.drainInteractiveJobs();
    this.clearQueues();
    this.fault = { code, detail,originalFailure };
    this.status = "faulted";
    const fallback = this.fallbackState();
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail, fallback,originalFailure));
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
    const fallback = this.fallbackState();
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail, fallback));
  }

  /** 🪝️ Runs one external UI hook inside the executing clock. A THROW is a real defect and still
   * fails the surface (`ui-hook-failed`); a budget breach is only recorded, and the answer says whether
   * the turn fitted so the caller can defer what is left. */
  private runUiHook(site: string, callback: () => void): boolean {
    this.uiTurnClock.enter();
    try {
      callback();
    } catch (error) {
      this.uiTurnClock.leave();
      const detail = `${site} threw: ${error instanceof Error ? error.message : String(error)}`;
      if (this.status !== "faulted" && this.status !== "closed") this.fail("ui-hook-failed", detail);
      return false;
    }
    this.observeUiTurn(site, this.uiTurnClock.leave());
    return true;
  }

  private clearQueues(): void {
    for (const pending of this.mediaPending.values()) { this.clearTimer(pending.timer); pending.reject(new Error("browser-media.closed")); }
    this.mediaPending.clear();
    this.mediaSlots.clear();
    this.clearDeadlineTimer();
    for (const controller of this.imageDecodes.values()) controller.abort();
    this.imageDecodes.clear();
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
    this.hubDocuments.clear();
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
    } catch { this.retireWorker(); }
  }

  private retireWorker(): void {
    if (this.workerRetired) return;
    this.workerRetired = true;
    this.worker.terminate();
    const drain = () => {
      const next = this.shardWorkers.keys().next();
      if (next.done) { this.resolveRetirement(); return; }
      this.terminateShardWorker(next.value);
      this.setTimer(drain, 0);
    };
    drain();
  }

  private takeLosslessWireBatch(): BrowserFrameWireLosslessEvent[] {
    const batch: BrowserFrameWireLosslessEvent[] = [];
    let budget = FRAME_WORKER_MESSAGE_BYTE_CAPACITY - 2048;
    for (const [documentKey, remote] of this.hubDocuments) {
      const bytes = 2 * documentKey.length + 192;
      if (batch.length >= 16 || budget <= bytes) break;
      batch.push(remote === null ? { kind: "hub-document-close", documentKey, timestampMs: this.now() } : { kind: "hub-document-status", documentKey, remote, timestampMs: this.now() });
      this.hubDocuments.delete(documentKey);
      budget -= bytes;
    }
    while (batch.length < 16 && this.lossless.length > 0 && budget > 256) {
      const queued = this.lossless[0]!;
      const event = queued.event;
      if (event.kind !== "text" && event.kind !== "paste" && event.kind !== "paste-image-data-url" && event.kind !== "ime-update" && event.kind !== "ime-commit") {
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
  if (event.kind === "text" || event.kind === "paste" || event.kind === "paste-image-data-url" || event.kind === "ime-update" || event.kind === "ime-commit") return 3 * event.text.length + 128;
  if (event.kind === "key-down" || event.kind === "key-up") return 2 * event.key.length + 128;
  if (event.kind === "hub-document-status" || event.kind === "hub-document-close") return 2 * event.documentKey.length + 192;
  if (event.kind === "accessibility-value") return 2 * (event.windowId.length + event.nodeKey.length + event.value.length) + 192;
  if (event.kind === "accessibility-focus" || event.kind === "accessibility-blur" || event.kind === "accessibility-activate") return 2 * (event.windowId.length + event.nodeKey.length) + 192;
  return 128;
}

function isHighSurrogate(value: number): boolean {
  return value >= 0xd800 && value <= 0xdbff;
}

function isLowSurrogate(value: number): boolean {
  return value >= 0xdc00 && value <= 0xdfff;
}

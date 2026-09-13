// #region 🔖️Protocol
/** @emoji 🧵️ Browser UI-to-frame-Worker protocol with bounded lossless and latest-wins lanes. */

import { BrowserInteractiveJobPort, type InteractiveJobUiMessage, type InteractiveJobWorkerMessage } from "../🔌️browser-interactive-job-port/🟦️.ts";
import { TurnClock, TurnLedger, UI_TURN_BUDGET_MS, type TurnLedgerSnapshot, type TurnOutcome, type TurnVerdict } from "../⏱️turn-budget/🟦️.ts";
import { FRAME_WORKER_BOOT_LIVENESS_POLICY, bootPhaseCeilingMs, describeBrowserBootSilence, evaluateBrowserBootLiveness, type BrowserBootPhase } from "../🫀️boot-liveness/🟦️.ts";

export const FRAME_WORKER_LOSSLESS_ITEM_CAPACITY = 64;
export const FRAME_WORKER_BYTE_CAPACITY = 256 * 1024;
/** @emoji ⏳️ Outer bound on a Worker that is silent AND has declared no long phase — the only state that
 * is really a wedged event loop. It is NOT a total-boot deadline and no longer a bare stall bound either:
 * a Worker blocked inside a browser-owned compile cannot post anything, so silence alone was never
 * evidence of death. The law, its per-phase ceilings and its diagnosis live in `../🫀️boot-liveness/🟦️.ts`;
 * this mirror exists so a reader of the transport sees the ceiling its watchdog is priced against. */
export const FRAME_WORKER_BOOT_STALL_TIMEOUT_MS = FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs;
export const FRAME_WORKER_POINTER_CAPACITY = 16;
export const FRAME_WORKER_MESSAGE_BYTE_CAPACITY = 4 * 1024;
export const FRAME_WORKER_TEXT_CHUNK_CODE_UNITS = 1024;
/** @emoji ⏱️ Re-exported so a reader of the transport sees the ceiling its turns are priced against;
 * the law itself — executing-time pricing and the sustained-run attribution — lives in
 * `../⏱️turn-budget/🟦️.ts`. */
export const FRAME_UI_TURN_BUDGET_MS = UI_TURN_BUDGET_MS;
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
  | "ui-hook-failed"
  | "interactive-job-violation"
  | "protocol-violation"
  | "replaceable-overflow"
  | "lossless-overflow"
  | "transport-closed";

export type BrowserFrameWorkerStatus = "booting" | "ready" | "quarantined" | "faulted" | "closed";

/** @emoji 🪂️ The surface's REAL fallback state, reported instead of asserting one. `uiThreadFrames` is
 * `unavailable-offscreen-transferred` by construction on this path: `transferControlToOffscreen()`
 * detaches the canvas from the UI isolate, so no UI-thread frame path exists to attempt afterwards —
 * a fact the fault banner must state rather than imply a fallback was skipped by choice. The remaining
 * fields say what the surface actually did: whether the Worker still holds the canvas, whether input is
 * still admitted, and the surface's own UI-turn ledger. */
export type BrowserFrameFallbackState = {
  readonly surface: BrowserFrameWorkerStatus;
  readonly uiThreadFrames: "unavailable-offscreen-transferred";
  readonly workerTerminated: boolean;
  readonly inputAccepted: boolean;
  readonly deferredCadence: boolean;
  readonly uiTurns: TurnLedgerSnapshot;
  /** @emoji 🧵️ The frame Worker's own step ledger, so a banner reports which isolate ran long. */
  readonly workerSteps: BrowserFrameWorkerStepReport;
  /** @emoji 🧭️ The long boot phase the Worker had declared and not yet left, with how long it had been in
   * flight — so a fault card names the work that was actually running instead of only the last stage that
   * happened to fit into a `boot-progress` before the isolate blocked. */
  readonly bootPhase: BrowserBootPhase | undefined;
  readonly bootPhaseElapsedMs: number;
  readonly bootStage: string;
  readonly bootSilentForMs: number;
};

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

/** @emoji 🖱️ One DOM input as the UI isolate observes it on `#semio-wgpu-canvas`, reduced to the fields
 * the wire carries. Deliberately NOT `PointerEvent`/`WheelEvent`/`KeyboardEvent`: this shape is what the
 * language-neutral oracle `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` names, so the projection below
 * can be exercised without a DOM and the Rust law can read the same rows. */
export type BrowserFrameDomEvent =
  | { readonly type: "pointermove" | "pointerdown" | "pointerup"; readonly pointerId: number; readonly pointerType: string; readonly offsetX: number; readonly offsetY: number; readonly pressure?: number; readonly tiltX?: number; readonly tiltY?: number; readonly button?: number }
  | { readonly type: "wheel"; readonly offsetX: number; readonly offsetY: number; readonly deltaX: number; readonly deltaY: number }
  | { readonly type: "keydown" | "keyup"; readonly key: string; readonly shift: boolean; readonly ctrl: boolean; readonly alt: boolean; readonly meta: boolean }
  | { readonly type: "resize"; readonly clientWidth: number; readonly clientHeight: number };

/** @emoji 📏️ The ONE place a CSS pixel becomes a physical pixel. `offsetX`/`offsetY` are CSS pixels
 * relative to the canvas; the renderer's layout, hit registry and dock plan are all physical. Scroll
 * deltas are already device-independent and are never scaled. */
function physical(css: number, devicePixelRatio: number): number {
  return css * devicePixelRatio;
}

/** @emoji 🖱️ Names a DOM `button` integer. The wire carries a name so neither side has to agree on
 * the DOM's numbering, and an unknown button is the primary one rather than a dropped event. */
function pointerButtonName(button: number | undefined): "primary" | "secondary" | "middle" {
  return button === 2 ? "secondary" : button === 1 ? "middle" : "primary";
}

function pointerFields(event: Extract<BrowserFrameDomEvent, { type: "pointermove" | "pointerdown" | "pointerup" }>, devicePixelRatio: number): BrowserFramePointer {
  return {
    pointerId: event.pointerId,
    pointerKind: event.pointerType === "touch" || event.pointerType === "pen" || event.pointerType === "eraser" ? event.pointerType : "mouse",
    x: physical(event.offsetX, devicePixelRatio),
    y: physical(event.offsetY, devicePixelRatio),
    ...(event.pressure ? { pressure: event.pressure } : {}),
    ...(event.tiltX ? { tiltX: event.tiltX } : {}),
    ...(event.tiltY ? { tiltY: event.tiltY } : {}),
  };
}

/** @emoji 🎮️ Projects one observed DOM input onto the single wire event the frame Worker decodes.
 *
 * This is the UI isolate's whole share of input semantics — the listeners in `🚀️browser-boot/🟦️.ts`
 * add nothing but focus/capture and the lossless-versus-replaceable lane choice. Keeping the projection
 * HERE, beside the wire types it produces and inside the module the transport's own vitest suite already
 * imports, is what lets `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` be answered without a browser and
 * by the Rust law on the other side of the same fixture.
 *
 * A `resize` is expressed in physical pixels by the caller's own surface measurement, so it carries
 * `dpr` and is not scaled again here. */
export function browserFrameEventFromDom(event: BrowserFrameDomEvent, devicePixelRatio: number): BrowserFrameReplaceableEvent | BrowserFrameLosslessEvent {
  if (event.type === "pointermove") return { kind: "pointer-move", ...pointerFields(event, devicePixelRatio) };
  if (event.type === "pointerdown" || event.type === "pointerup") return { kind: event.type === "pointerdown" ? "pointer-down" : "pointer-up", ...pointerFields(event, devicePixelRatio), button: pointerButtonName(event.button) };
  if (event.type === "wheel") return { kind: "wheel", x: physical(event.offsetX, devicePixelRatio), y: physical(event.offsetY, devicePixelRatio), deltaX: event.deltaX, deltaY: event.deltaY };
  if (event.type === "resize") return { kind: "resize", width: Math.max(1, Math.round(physical(event.clientWidth, devicePixelRatio))), height: Math.max(1, Math.round(physical(event.clientHeight, devicePixelRatio))), dpr: devicePixelRatio };
  return { kind: event.type === "keydown" ? "key-down" : "key-up", key: event.key, shift: event.shift, ctrl: event.ctrl, alt: event.alt, meta: event.meta };
}

/** @emoji 🫧 Whether a projected wire event belongs to the coalescing lane (`enqueueReplaceable`) rather
 * than the credit-bearing lossless lane. A pointer MOVE, a wheel and a resize are each "the newest one
 * wins"; a button transition and a key transition are not, because dropping one latches shell state. */
export function browserFrameEventIsReplaceable(event: BrowserFrameReplaceableEvent | BrowserFrameLosslessEvent): event is BrowserFrameReplaceableEvent {
  return event.kind === "pointer-move" || event.kind === "wheel" || event.kind === "resize";
}

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
  /** @emoji 🎭️ `?mode=`'s value, or `""` when the url named none — the boot-time mode axis beside `appRole`. */
  readonly appMode: string;
  /** @emoji 📚️ `?example=`'s value, or `""` when the url named none — the boot-time example axis. An id
   * the open dialect does not author is dropped by the shell, never a boot failure. */
  readonly appExample: string;
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
 * frame message uses. Read-only by construction: no probe mutates renderer state.
 *
 * ♿️ `accessibility` is the one probe that is NOT a test hook: it carries the window's accessibility
 * tree out of the isolate so the UI thread can mirror it into a real ARIA subtree beside the canvas.
 * A DOM renderer writes those attributes onto the elements it already renders; a GPU canvas has no
 * elements, so the tree has to cross this seam as data (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export type BrowserFrameIntrospectionProbe = "structure" | "frame-stats" | "accessibility";

export type BrowserFrameWorkerIntrospect = { readonly kind: "introspect"; readonly lifecycle: number; readonly requestId: number; readonly probe: BrowserFrameIntrospectionProbe; readonly windowId?: string };

/** @emoji 🧵️ One shard worker the UI isolate spawned on the frame worker's behalf, handed back as a
 * `MessagePort`. Nested dedicated workers are unavailable in some embedded browsers, so the frame worker
 * never constructs a shard `Worker` itself — see `shardWorkerPortBridge` in the wgpu plugin bridge. */
export type BrowserFrameShardPort = { readonly kind: "shard-port"; readonly shardIndex: number; readonly port: MessagePort };

export type BrowserFrameUiMessage = BrowserFrameWorkerBoot | BrowserFrameWorkerBatch | BrowserFrameWorkerIntrospect | InteractiveJobUiMessage | BrowserFrameShardPort | { readonly kind: "close"; readonly lifecycle: number };

/** @emoji 🧵️ The frame Worker's own step ledger, as the UI isolate sees it. The Worker prices its steps
 * against `WORKER_STEP_BUDGET_MS` with the same executing-span law the UI isolate uses for its turns
 * (`../⏱️turn-budget/🟦️.ts`), so this is a MEASUREMENT the boot UI renders — never a verdict. */
export type BrowserFrameWorkerStepReport = {
  readonly degraded: boolean;
  readonly recordedOverruns: number;
  readonly sustainedOverruns: number;
  readonly worstStepMs: number;
  readonly worstStepSite: string;
};

export type BrowserFrameWorkerMessage =
  | { readonly kind: "boot-progress"; readonly lifecycle: number; readonly stage: string; readonly progress: number; readonly worker: BrowserFrameWorkerStepReport }
  | { readonly kind: "boot-liveness"; readonly lifecycle: number }
  /** @emoji 🧭️ The Worker DECLARING that it is about to block on one browser-owned phase, posted while its
   * event loop still runs so the declaration always arrives — and withdrawing it when the phase ends. This
   * is what lets the watchdog tell a busy Worker from a wedged one: a `WebAssembly` compile of the 76 MB
   * renderer, `semioWgpuWorkerBootstrap`'s synchronous prologue and one Rust bootstrap phase all stop the
   * Worker's liveness ticker dead, so silence during a declared phase proves nothing. */
  | { readonly kind: "boot-phase"; readonly lifecycle: number; readonly phase: string; readonly state: "enter" | "leave"; readonly elapsedMs?: number }
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
      /** @emoji ⏳️ The step's EXECUTING milliseconds — what the Worker's own ledger priced. */
      readonly workerExecutingMs: number;
      readonly workerStepVerdict: TurnVerdict;
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
  readonly onProgress?: (stage: string, progress: number, worker: BrowserFrameWorkerStepReport) => void;
  readonly onDirectives?: (directives: BrowserFrameDirectives) => void;
  readonly onFault?: (code: BrowserFrameWorkerFaultCode, detail: string, fallback: BrowserFrameFallbackState) => void;
  /** @emoji 🐢️ Reported for every UI turn that breached its ceiling — a measured signal, never a verdict. */
  readonly onUiTurn?: (outcome: TurnOutcome) => void;
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
  private readonly onProgress?: (stage: string, progress: number, worker: BrowserFrameWorkerStepReport) => void;
  /** @emoji 🧵️ The frame Worker's last reported step ledger — a measurement the boot UI renders. */
  private workerSteps: BrowserFrameWorkerStepReport = { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" };
  /** @emoji 🧵️ Frame steps the Worker attributed to its OWN work (a sustained run, not one wall sample). */
  private workerStepOverruns = 0;
  private readonly onDirectives?: (directives: BrowserFrameDirectives) => void;
  private readonly onFault?: (code: BrowserFrameWorkerFaultCode, detail: string, fallback: BrowserFrameFallbackState) => void;
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
  private nextStreamId = 1;
  private generation = 0;
  private sequence = 0;
  private acceptedSequence = 0;
  private inFlight = false;
  private frameRequested = false;
  private rafHandle: number | undefined;
  private bootTimer: number | undefined;
  /** @emoji 🫀️ The last instant ANY message arrived from the frame Worker — the clock silence is measured
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

  constructor(options: BrowserFrameTransportOptions) {
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
    this.onFault = options.onFault;
    this.onUiTurn = options.onUiTurn;
    this.uiTurnClock = new TurnClock(this.now);
    this.requestRaf = options.requestAnimationFrame;
    this.cancelRaf = options.cancelAnimationFrame;
    this.interactiveJobs = new BrowserInteractiveJobPort(this.lifecycle, (message) => this.worker.postMessage(message), this.now, (detail) => this.quarantine("interactive-job-violation", detail), (callback) => void this.setTimer(callback, 0));
    this.worker.onmessage = (event) => this.receive(event.data);
    this.worker.onerror = (event) => this.fail("worker-message-failed", event.message || "Worker error");
    this.worker.onmessageerror = () => this.fail("worker-message-failed", "Worker message could not be decoded");
    this.armBootWatchdog(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs);
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
      this.uiTurnClock.enter();
      this.worker.postMessage({ kind: "batch", lifecycle: this.lifecycle, sequence, generation: this.generation, timestampMs, replaceable, lossless });
      this.observeUiTurn("frame-transfer", this.uiTurnClock.leave());
      return true;
    } catch (error) {
      this.uiTurnClock.leave();
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

  /** @emoji ⏱️ Prices ONE UI turn and records it. Never a verdict: an overrun cannot quarantine, fail or
   * close this surface, because on the UI isolate a wall reading over the ceiling is as often the machine
   * descheduling a hidden pane as it is the turn's own work — the attribution lives in `TurnLedger`'s
   * sustained-run law, and its only consequence is deferred cadence. Answers whether the turn was
   * admitted, so a caller may choose to defer its remaining work to the next frame. */
  observeUiTurn(site: string, executingMs: number | undefined): boolean {
    const outcome = this.uiTurns.admit(site, executingMs);
    if (outcome.verdict !== "admitted" && outcome.verdict !== "clock-fault") this.onUiTurn?.(outcome);
    return outcome.verdict === "admitted" || outcome.verdict === "clock-fault";
  }

  /** @emoji 🐢️ Whether the surface is running its turns on deferred cadence after a sustained run of
   * overruns. Clears itself on the first turn that fits the ceiling again. */
  degraded(): boolean {
    return this.uiTurns.degraded();
  }

  /** @emoji 🪂️ The surface's real fallback state — what a fault banner must report instead of claiming
   * a UI-thread frame path was or was not attempted. */
  fallbackState(): BrowserFrameFallbackState {
    return {
      surface: this.status,
      uiThreadFrames: "unavailable-offscreen-transferred",
      workerTerminated: this.status === "faulted" || this.status === "closed",
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

  /** @emoji 📊 Returns bounded fixed-ring p99 telemetry outside the event callback path. */
  uiTurnP99Ms(): number {
    return this.uiTurns.p99Ms();
  }

  /** @emoji ⏭️ Yields and continues: hands the remainder of an overrunning turn to the next macrotask so
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

  /** @emoji 🫀️ Arms ONE self-rescheduling watchdog wake. Nothing re-arms it on every inbound message any
   * more: a message only stamps {@link lastLivenessAtMs}, and the wake below re-reads the whole window and
   * schedules itself for the exact remaining time. One timer for the whole boot instead of one per
   * `boot-progress`, and the deadline is exact rather than a whole window late. */
  private armBootWatchdog(delayMs: number): void {
    if (this.bootTimer !== undefined) this.clearTimer(this.bootTimer);
    this.bootTimer = this.setTimer(() => this.judgeBootLiveness(), Math.max(0, delayMs));
  }

  /** @emoji ⚖️ One watchdog window, decided by `../🫀️boot-liveness/🟦️.ts` and never here — so the same
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

  /** @emoji 🫀️ Stamps the instant the Worker last proved it was running. */
  private witnessWorker(): void {
    this.lastLivenessAtMs = this.now();
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
      this.fail(this.status === "booting" ? "worker-boot-failed" : "worker-runtime-failed", `${message.code}: ${message.detail}`);
      return;
    }
    if (message.generation > this.generation) {
      this.fail("protocol-violation", `Worker returned future generation ${message.generation} while UI generation is ${this.generation}`);
      return;
    }
    if (message.sequence <= this.acceptedSequence) return;
    this.inFlight = false;
    if (message.workerStepVerdict === "sustained-overrun") this.workerStepOverruns++;
    if (message.quarantined) {
      const code = message.faultCode === "present-failed" ? "worker-present-failed" : message.faultCode === "text-input-failed" ? "worker-input-failed" : "worker-step-overrun";
      this.quarantine(code, message.faultDetail ?? `worker frame step executed ${message.workerExecutingMs.toFixed(3)} ms`);
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
    const fallback = this.fallbackState();
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail, fallback));
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

  /** @emoji 🪝️ Runs one external UI hook inside the executing clock. A THROW is a real defect and still
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

import type { PluginCatalogRowsV1 } from "@semio-tech/framework/plugin-catalog";
import { type BrowserBootPhase, FRAME_WORKER_BOOT_LIVENESS_POLICY } from "../../🫀️boot-liveness/🟦️.ts";
import { type TurnLedgerSnapshot, type TurnOutcome, type TurnVerdict, UI_TURN_BUDGET_MS } from "../../⏱️turn-budget/🟦️.ts";
import type { NativeCatalogV1 } from "@semio-tech/plugin-registry/runtime";
import type { WgpuPluginModule, WgpuPluginRegistrySelection } from "../../🧩️plugin-modules/🛂️admission/🟦️.ts";
import type { WgpuBootDescriptor, WgpuHostAppearance, WgpuHostPlatform, WgpuHostStorageSnapshot } from "../../🧭️boot-descriptor/🟦️.ts";
import type { OriginalBootstrapRequest, OriginalBootstrapResponse } from "@semio-tech/framework-actor/retained-turn/bootstrap/port";
import type { BrowserMediaCommand, BrowserMediaRelease, BrowserMediaResult, PresentedMediaSlot } from "../../../../🎬️media/🌐️browser/🟦️.ts";
import type { InteractiveJobUiMessage, InteractiveJobWorkerMessage } from "../../🔌️browser-interactive-job-port/🟦️.ts";
import type { ReferenceImageDimensions } from "../../🖼️reference-image-decode/🟦️.ts";
import type { OriginalBootstrapOwner } from "../../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts";

export const FRAME_WORKER_LOSSLESS_ITEM_CAPACITY = 64;

export const FRAME_WORKER_HUB_DOCUMENT_CAPACITY = 64;

export const FRAME_WORKER_BYTE_CAPACITY = 256 * 1024;

/** ⏳️ Outer bound on a Worker that is silent AND has declared no long phase — the only state that
 * is really a wedged event loop. It is NOT a total-boot deadline and no longer a bare stall bound either:
 * a Worker blocked inside a browser-owned compile cannot post anything, so silence alone was never
 * evidence of death. The law, its per-phase ceilings and its diagnosis live in `../🫀️boot-liveness/🟦️.ts`;
 * this mirror exists so a reader of the transport sees the ceiling its watchdog is priced against. */
export const FRAME_WORKER_BOOT_STALL_TIMEOUT_MS = FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs;

export const FRAME_WORKER_POINTER_CAPACITY = 16;

export const FRAME_WORKER_MESSAGE_BYTE_CAPACITY = 4 * 1024;

export const FRAME_WORKER_TEXT_CHUNK_CODE_UNITS = 1024;

export const FRAME_WORKER_ACCESSIBILITY_ID_BYTES = 512;

/** ⏱️ Re-exported so a reader of the transport sees the ceiling its turns are priced against;
 * the law itself — executing-time pricing and the sustained-run attribution — lives in
 * `../⏱️turn-budget/🟦️.ts`. */
export const FRAME_UI_TURN_BUDGET_MS = UI_TURN_BUDGET_MS;

export const FRAME_WORKER_INTROSPECTION_CAPACITY = 4;

export const FRAME_WORKER_INTROSPECTION_TIMEOUT_MS = 10_000;

export const FRAME_WORKER_IMAGE_DECODE_BYTE_CAPACITY = 16 * 1024 * 1024;

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
  | "worker-frame-failed"
  | "worker-message-failed"
  | "ui-hook-failed"
  | "interactive-job-violation"
  | "protocol-violation"
  | "replaceable-overflow"
  | "lossless-overflow"
  | "transport-closed";

export type BrowserFrameWorkerStatus = "booting" | "ready" | "quarantined" | "faulted" | "closed";

/** 🪂️ The surface's REAL fallback state, reported instead of asserting one. `uiThreadFrames` is
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
  /** 🧵️ The frame Worker's own step ledger, so a banner reports which isolate ran long. */
  readonly workerSteps: BrowserFrameWorkerStepReport;
  /** 🧭️ The long boot phase the Worker had declared and not yet left, with how long it had been in
   * flight — so a fault card names the work that was actually running instead of only the last stage that
   * happened to fit into a `boot-progress` before the isolate blocked. */
  readonly bootPhase: BrowserBootPhase | undefined;
  readonly bootPhaseElapsedMs: number;
  readonly bootStage: string;
  readonly bootSilentForMs: number;
};

export type BrowserFrameModifiers = {
  readonly shift: boolean;
  readonly ctrl: boolean;
  readonly alt: boolean;
  readonly meta: boolean;
};

export type BrowserFramePointer = BrowserFrameModifiers & {
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
  | ({ readonly kind: "wheel"; readonly x: number; readonly y: number; readonly deltaX: number; readonly deltaY: number } & BrowserFrameModifiers)
  | { readonly kind: "resize"; readonly width: number; readonly height: number; readonly dpr: number };

export type BrowserHubDocumentRemote =
  | { readonly kind: "detached" }
  | { readonly kind: "connecting" }
  | { readonly kind: "live"; readonly peerCount: number }
  | { readonly kind: "backoff"; readonly retryInMs: number };

export type BrowserFrameAccessibilityAddress = {
  readonly windowId: string;
  readonly windowGeneration: number;
  readonly nodeId: number;
  readonly nodeKey: string;
};

export type BrowserFrameLosslessEvent =
  | ({ readonly kind: "pointer-cancel" } & BrowserFramePointer)
  | ({ readonly kind: "pointer-down" | "pointer-up"; readonly button: "primary" | "secondary" | "middle" } & BrowserFramePointer)
  | { readonly kind: "key-down" | "key-up"; readonly key: string; readonly shift: boolean; readonly ctrl: boolean; readonly alt: boolean; readonly meta: boolean }
  | { readonly kind: "text" | "paste" | "paste-image-data-url"; readonly text: string }
  | { readonly kind: "ime-start" | "ime-cancel" }
  | { readonly kind: "ime-update"; readonly text: string; readonly cursor: number }
  | { readonly kind: "ime-commit"; readonly text: string }
  | ({ readonly kind: "accessibility-focus" | "accessibility-blur" | "accessibility-activate" } & BrowserFrameAccessibilityAddress)
  | ({ readonly kind: "accessibility-value"; readonly value: string } & BrowserFrameAccessibilityAddress)
  // ⏱️ These two are minted straight into a wire batch by `takeLosslessWireBatch` rather than enqueued
  // as input, so unlike every other member they carry the batch stamp on the type instead of
  // acquiring it at flush.
  | { readonly kind: "hub-document-status"; readonly documentKey: string; readonly remote: BrowserHubDocumentRemote; readonly timestampMs: number }
  | { readonly kind: "hub-document-close"; readonly documentKey: string; readonly timestampMs: number };

export type BrowserFrameWorkerBoot = {
  readonly catalog: PluginCatalogRowsV1;
  readonly nativeCatalog?: NativeCatalogV1;
  readonly plugins?: readonly WgpuPluginModule[];
  readonly pluginRegistrySelection?: WgpuPluginRegistrySelection;
  readonly kind: "boot";
  readonly lifecycle: number;
  readonly bindingsModuleUrl: string;
  readonly bindingsWasmUrl: string;
  readonly canvas: OffscreenCanvas;
  readonly width: number;
  readonly height: number;
  readonly dpr: number;
  readonly locale: "en" | "de";
  /** 🧭️ Every boot axis in ONE shape (`../🧭️boot-descriptor/🟦️.ts`), forwarded verbatim to the
   * renderer wasm's `semioWgpuSetBootDescriptor`. It used to be four loose fields here, which is how
   * the three wgpu doors came to carry three different subsets of the same vocabulary. */
  readonly descriptor: WgpuBootDescriptor;
  /** 🌓️ The page realm's appearance reads (`../🧭️boot-descriptor/🟦️.ts`), forwarded to the
   * renderer's `semioWgpuSetHostAppearance`. An environment axis beside `locale`/`dpr`, never a boot
   * axis: the Worker cannot make either read itself, and both keep changing after boot. */
  readonly appearance: WgpuHostAppearance;
  /** ⌨️ The page realm's platform read (`../🧭️boot-descriptor/🟦️.ts`), forwarded to the
   * renderer's `semioWgpuSetHostPlatform` so `mod` formats as `⌘️` on Apple and `Ctrl` elsewhere.
   * Constant for the life of a navigation, so unlike `appearance` it never gets a live message. */
  readonly platform: WgpuHostPlatform;
  /** 🗄️ The page realm's read of every durable preference key the shell owns
   * (`../🧭️boot-descriptor/🟦️.ts`'s 🗄️HostStorage census), forwarded to the renderer's
   * `semioWgpuSetHostStorage`. It travels WITH the boot so the shell's first frame answers
   * appearance, `ui.introduction.seen.*` and the dock skeleton synchronously instead of painting a
   * default and correcting it a frame later. Later writes cross the host-io door one at a time. */
  readonly storage: WgpuHostStorageSnapshot;
  readonly introductionSuppressed?: boolean;
};

/** 🗄️ One live storage change made OUTSIDE this page — another tab rewrote a shared key, which
 * is the only way the page learns of one (`storage` never fires for the writer's own document). Carries
 * the whole re-read snapshot rather than a delta: the page's store IS the authority and a delta would
 * need a second ordering guarantee against the door's own write-through. */
export type BrowserFrameHostStorage = { readonly kind: "host-storage"; readonly lifecycle: number; readonly storage: WgpuHostStorageSnapshot };

/** 🎓️ One independently rooted host can suppress its own introduction without affecting peers. */
export type BrowserFrameHostIntroductionPolicy = { readonly kind: "host-introduction-policy"; readonly lifecycle: number; readonly suppressed: boolean };

export type BrowserFrameHostAppearance = { readonly kind: "host-appearance"; readonly lifecycle: number; readonly appearance: WgpuHostAppearance };

/** 🫥️ The page was hidden (`visibilitychange` to hidden, which a closing tab fires before `pagehide` retires the mount): the Worker ends every open text-editor typing run
 * (`hidden`, design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) so the keystrokes of its last idle bound are never lost. */
export type BrowserFrameHostPageHidden = { readonly kind: "host-page-hidden"; readonly lifecycle: number };

/** 🫥️ The page's `window` lost the system focus (the person switched applications or windows): the Worker ends every open
 * text-editor typing run (`blur`) and blurs the focused pane for its program (`hostEvent{kind: blur}`), React's pane `onBlur`. */
export type BrowserFrameHostWindowBlur = { readonly kind: "host-window-blur"; readonly lifecycle: number };

/** 🛰️ The local supervisor's agent-bridge offer as the page last read it (`🔗️AgentBridge/🛰️offer`), `null` once no
 * gateway offers one. The Worker owns no page and dials nothing it discovered itself: the page reads the offer, the
 * Worker hands it to the renderer's `semioWgpuSetAgentBridgeConfig`, and the bridge socket goes through the page door. */
export type BrowserFrameHostAgentBridge = { readonly kind: "host-agent-bridge"; readonly lifecycle: number; readonly offer: { readonly url: string; readonly admissionProof: string } | null };

export type BrowserFrameWorkerBatch = {
  readonly kind: "batch";
  readonly lifecycle: number;
  readonly inputSequence: number;
  readonly generation: number;
  readonly timestampMs: number;
  readonly replaceable: readonly BrowserFrameReplaceableEvent[];
  readonly lossless: readonly BrowserFrameWireLosslessEvent[];
};

export type BrowserFrameWireLosslessEvent =
  | (Exclude<BrowserFrameLosslessEvent, { readonly kind: "text" | "paste" | "paste-image-data-url" | "ime-update" | "ime-commit" }> & { readonly timestampMs: number })
  | { readonly kind: "text-chunk"; readonly streamId: number; readonly target: "text" | "paste" | "paste-image-data-url" | "ime-update" | "ime-commit"; readonly text: string; readonly totalBytes: number; readonly final: boolean; readonly timestampMs: number; readonly cursor?: number };

/** 🔬️ The renderer's `#[wasm_bindgen]` introspection exports (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
 * region `🔬️IntrospectionExports`) read `UI_ENGINE`, a thread-local that lives inside `semio-frame-worker`.
 * The UI isolate therefore cannot call them directly and asks across the same fail-closed seam every other
 * frame message uses. Read-only by construction: no probe mutates renderer state.
 *
 * ♿️ `accessibility` is the one probe that is NOT a test hook: it carries the window's accessibility
 * tree out of the isolate so the UI thread can mirror it into a real ARIA subtree beside the canvas.
 * A DOM renderer writes those attributes onto the elements it already renders; a GPU canvas has no
 * elements, so the tree has to cross this seam as data (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * 🛰️ `agent-bridge-scope` is production too: the shell's agent-bridge offer scope (hub origin, space, its human's agent
 * principals — never the session capability) for the page's offer watcher (ticket 26/09/23, G12 × WG11 session 14c). */
export type BrowserFrameIntrospectionProbe = "structure" | "frame-stats" | "accessibility" | "mesh-stats" | "board2d" | "chrome" | "agent-bridge-scope";

export type BrowserFrameWorkerIntrospect = { readonly kind: "introspect"; readonly lifecycle: number; readonly requestId: number; readonly probe: BrowserFrameIntrospectionProbe; readonly windowId?: string };

/** 🧵️ One shard worker the UI isolate spawned on the frame worker's behalf, handed back as a
 * `MessagePort`. Nested dedicated workers are unavailable in some embedded browsers, so the frame worker
 * never constructs a shard `Worker` itself — see `shardWorkerPortBridge` in the wgpu plugin bridge. */
export type BrowserFrameShardPort = { readonly kind: "shard-port"; readonly shardIndex: number; readonly port: MessagePort };

/** 🚪️ The page's answer to one {@link BrowserFrameWorkerHostIo} request — the JSON the shell's
 * `semioWgpuHostIo` call resolves with, or a `detail` the shell refuses on. Addressed by `requestId`
 * alone, exactly like `introspection`, so no host-side request table beyond the pending map exists. */
export type BrowserFrameHostIoResult = { readonly kind: "host-io-result"; readonly lifecycle: number; readonly requestId: number; readonly json: string | null; readonly detail?: string };

export type BrowserFrameImageDecodeResult = { readonly kind: "image-decode-result"; readonly lifecycle: number; readonly requestId: number; readonly bitmap: ImageBitmap | null; readonly detail?: string };

export type BrowserFrameUiMessage =
  | OriginalBootstrapResponse | BrowserMediaRelease | BrowserMediaCommand | BrowserFrameWorkerBoot | BrowserFrameWorkerBatch | BrowserFrameWorkerIntrospect | InteractiveJobUiMessage | BrowserFrameShardPort | BrowserFrameHostIoResult | BrowserFrameImageDecodeResult | BrowserFrameHostAppearance | BrowserFrameHostStorage | BrowserFrameHostAgentBridge | BrowserFrameHostIntroductionPolicy | BrowserFrameHostPageHidden | BrowserFrameHostWindowBlur | { readonly kind: "close"; readonly lifecycle: number };

/** 🧵️ The frame Worker's own step ledger, as the UI isolate sees it. The Worker prices its steps
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
  | OriginalBootstrapRequest
  | BrowserMediaResult
  | { readonly kind: "boot-progress"; readonly lifecycle: number; readonly stage: string; readonly progress: number; readonly worker: BrowserFrameWorkerStepReport }
  | { readonly kind: "boot-liveness"; readonly lifecycle: number }
  /** 🧭️ The Worker DECLARING that it is about to block on one browser-owned phase, posted while its
   * event loop still runs so the declaration always arrives — and withdrawing it when the phase ends. This
   * is what lets the watchdog tell a busy Worker from a wedged one: a `WebAssembly` compile of the 76 MB
   * renderer, `semioWgpuWorkerBootstrap`'s synchronous prologue and one Rust bootstrap phase all stop the
   * Worker's liveness ticker dead, so silence during a declared phase proves nothing. */
  | { readonly kind: "boot-phase"; readonly lifecycle: number; readonly phase: string; readonly state: "enter" | "leave"; readonly elapsedMs?: number }
  | { readonly kind: "booted"; readonly lifecycle: number }
  | { readonly kind: "wake"; readonly lifecycle: number }
  | { readonly kind: "batch-accepted"; readonly lifecycle: number; readonly inputSequence: number; readonly generation: number }
  | {
      readonly kind: "frame";
      readonly mediaSlots: readonly PresentedMediaSlot[];
      readonly lifecycle: number;
      readonly frameSequence: number;
      readonly generation: number;
      readonly cursor: string;
      readonly fullscreen: boolean | null;
      readonly requestFrame: boolean;
      readonly nextDeadlineDelayMs: number | null;
      readonly progress: number;
      readonly workerDurationMs: number;
      /** ⏳️ The step's EXECUTING milliseconds — what the Worker's own ledger priced. */
      readonly workerExecutingMs: number;
      readonly workerStepVerdict: TurnVerdict;
      readonly quarantined?: boolean;
      readonly faultCode?: string;
      readonly faultDetail?: string;
    }
  | { readonly kind: "introspection"; readonly lifecycle: number; readonly requestId: number; readonly probe: BrowserFrameIntrospectionProbe; readonly json: string | null; readonly detail?: string }
  | { readonly kind: "diagnostic"; readonly lifecycle: number; readonly channel: "world3d-accepted-frame"; readonly generation: number; readonly frameSequence: number; readonly json: string }
  | { readonly kind: "fault"; readonly lifecycle: number; readonly code: string; readonly detail: string;readonly originalFailure?:{readonly format:"pack";readonly bytes:Uint8Array} }
  | { readonly kind: "closed"; readonly lifecycle: number }
  | { readonly kind: "shard-spawn"; readonly shardIndex: number; readonly url: string }
  | { readonly kind: "shard-terminate"; readonly shardIndex: number }
  /** 🚪️ The Worker asking the PAGE for the one thing it cannot do itself: a `<a download>` or an
   * `<input type="file">`. A dedicated Worker has no `window` and no `document`, so the shell's own
   * browser halves used to answer `None` and return silently — an export produced its bytes and handed
   * them to nobody, an import opened nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  | { readonly kind: "host-io"; readonly lifecycle: number; readonly requestId: number; readonly request: string; readonly bytes: Uint8Array | null }
  | { readonly kind: "image-decode"; readonly lifecycle: number; readonly requestId: number; readonly source: Uint8Array<ArrayBuffer>; readonly dimensions: ReferenceImageDimensions }
  | { readonly kind: "image-decode-cancel"; readonly lifecycle: number; readonly requestId: number }
  | InteractiveJobWorkerMessage;

export interface BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null;
  onmessageerror: ((event: MessageEvent) => void) | null;
  onerror: ((event: ErrorEvent) => void) | null;
  postMessage(message: BrowserFrameUiMessage, transfer?: Transferable[]): void;
  terminate(): void;
}

export type BrowserFrameDirectives = {
  readonly mediaSlots: readonly PresentedMediaSlot[];
  readonly cursor: string;
  readonly fullscreen: boolean | null;
  readonly generation: number;
  readonly workerDurationMs: number;
};

export type BrowserFrameDiagnostic = {
  readonly channel: "world3d-accepted-frame";
  readonly generation: number;
  readonly frameSequence: number;
  readonly json: string;
};

export type BrowserFrameTransportOptions = {
  readonly originalWorkStock: OriginalBootstrapOwner;
  readonly worker: BrowserFrameWorkerPort;
  readonly boot: Omit<BrowserFrameWorkerBoot, "kind" | "lifecycle">;
  readonly now?: () => number;
  readonly setTimer?: (callback: () => void, delayMs: number) => number;
  readonly clearTimer?: (handle: number) => void;
  readonly onReady?: () => void;
  readonly onProgress?: (stage: string, progress: number, worker: BrowserFrameWorkerStepReport) => void;
  readonly onDirectives?: (directives: BrowserFrameDirectives) => void;
  readonly onDiagnostic?: (diagnostic: BrowserFrameDiagnostic) => void;
  readonly onFault?: (code: BrowserFrameWorkerFaultCode, detail: string, fallback: BrowserFrameFallbackState,originalFailure?:{readonly format:"pack";readonly bytes:Uint8Array}) => void;
  /** 🐢️ Reported for every UI turn that breached its ceiling — a measured signal, never a verdict. */
  readonly onUiTurn?: (outcome: TurnOutcome) => void;
  readonly requestAnimationFrame?: (callback: FrameRequestCallback) => number;
  readonly cancelAnimationFrame?: (handle: number) => void;
  /** 🚪️ The PAGE half of the shell's file door (`🚪️host-io/🟦️.ts`). A dedicated Worker has no
   * `document`, so the shell's `<a download>` and `<input type="file">` live on this side of the wire.
   * Absent, a `host-io` request is refused with a named detail rather than silently ignored — a silent
   * ignore is exactly how both io journeys died on this renderer. */
  readonly hostIo?: (requestJson: string, bytes: Uint8Array | null) => Promise<string>;
};

export type QueuedLossless = {
  readonly event: Exclude<BrowserFrameLosslessEvent, { readonly kind: "hub-document-status" | "hub-document-close" }>;
  readonly bytes: number;
  readonly streamId: number;
  readonly timestampMs: number;
  cursor: number;
};

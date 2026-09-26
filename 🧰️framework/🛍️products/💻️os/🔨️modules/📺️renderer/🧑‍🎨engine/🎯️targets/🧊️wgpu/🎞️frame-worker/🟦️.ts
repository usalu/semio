/// <reference lib="webworker" />

import { PlaygroundBootPlanner, pluginGraphErrorMessage } from "@semio-tech/framework";
import { TurnClock, TurnLedger, WORKER_STEP_BUDGET_MS, setTurnDiagnostics, stampedTurnDiagnostics, type TurnOutcome } from "../⏱️turn-budget/🟦️.ts";
import { FRAME_WORKER_BOOT_LIVENESS_POLICY } from "../🫀️boot-liveness/🟦️.ts";
import { evictCachedRendererModule, readCachedRendererModule, rendererArtifactTag, writeCachedRendererModule } from "../🗄️wasm-module-cache/🟦️.ts";
import { PLUGIN_CATALOG } from "../../../../../🔌️plugin/📇️registry/🟦️.ts";
import type { BrowserFrameUiMessage, BrowserFrameWorkerMessage } from "../🚚️browser-frame-transport/🟦️.ts";
import { INTERACTIVE_WORKER_DESCRIPTORS, InteractiveWorkerScheduler } from "../📇️interactive-job-registry/🟦️.ts";
import { loadPluginModule, pluginHandleForBridge, primeContributionManifest } from "../🐚️plugin-bridge/🟦️.ts";
import { createLazyPluginInstallDoor } from "./🧩️lazy-install/🟦️.ts";
import { meshAssetTransportUrl } from "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🟦️.ts";
import { concatenateReferenceImageSource, decodeReferenceImage, referenceImageBitmapForStage, referenceImageSourceDigest, referenceImageSourceDimensions, referenceImageTargetSize, streamReferenceImageBitmapRows, type ReferenceImageDimensions } from "../🖼️reference-image-decode/🟦️.ts";
import { FrameTurnScheduler, WorkerTurnTaskQueue, nextFrameSequence } from "../🧵️frame-turn-scheduler/🟦️.ts";
import { BrowserAssetCancellationCursor, assertBrowserAssetResponseContinuation } from "./🧩️asset-cancellation/🟦️.ts";

//#region 🔖️Bindings
/** @emoji 🔢️ `generation` and `sequence` are `u64` on the renderer's own `#[wasm_bindgen]` exports
 * (`🌐️browser-worker/🦀️.rs` `enqueue_batch`/`tick`), and wasm-bindgen lowers a `u64` parameter
 * straight into the wasm `i64` slot — a JS `number` there throws
 * `TypeError: Cannot convert <n> to a BigInt` out of the generated glue, faulting the Worker on its
 * FIRST frame batch (`frame-runtime-fault`). Declaring them `bigint` here is what makes that ABI
 * checkable at the seam instead of at runtime; the transport's own counters stay `number`, which is
 * what a structured-cloned protocol message carries. */
type BrowserRendererWorkerHandle = {
  enqueueBatch(eventsJson: string, generation: bigint): void;
  tick(timestampMs: number, sequence: bigint, generation: bigint): string;
  assetDecodeStep(): string;
  pollAssetRequest(): string;
  assetResponseCurrent(): boolean;
  beginReferenceImage(width: number, height: number, digest: string): number;
  pushReferenceImageRows(offset: number, pixels: Uint8Array): boolean;
  sealReferenceImage(): boolean;
  reserveAssetResponse(byteCredits: number): boolean;
  pushAssetResponsePage(bytes: Uint8Array): void;
  sealAssetResponse(): boolean;
  abortAssetResponse(): boolean;
  rejectAssetResponse(): boolean;
  closeStep(): boolean;
};

type BrowserRendererBootstrapHandle = {
  step(): string;
  bootShell(): Promise<BrowserRendererBootstrapHandle>;
  finish(): BrowserRendererWorkerHandle;
};

type BrowserRendererBootStep = { readonly stage: string; readonly progress: number; readonly shellBoot: boolean; readonly complete: boolean; readonly elapsedUs: number };
type RendererAssetDecodeStep = { readonly kind: "pending" | "published" | "cancelled" | "idle" | "fault"; readonly pending: boolean; readonly detail?: string };

type RendererBindings = {
  default?: (moduleOrPath?: WebAssembly.Module | RequestInfo | URL) => Promise<unknown>;
  dumpStructure?: (windowId?: string) => string;
  dumpFrameStats?: (windowId?: string) => string;
  /** ♿️ The window's accessibility tree, for the UI isolate's ARIA mirror — production, not a probe. */
  dumpAccessibility?: (windowId?: string) => string;
  /** 🧊️ Every World3d surface's PUBLISHED mesh payload, per role — the oracle a committed fixture is compared against. */
  dumpMeshStats?: (windowId?: string) => string;
  /** 🎯️ The shell chrome's pointer registry and its dispatched-action ledger — the chrome twin of the DOM a
   * React parity probe reads. Diagnostics-gated in the renderer, so an unarmed page answers `armed: false`. */
  dumpChrome?: (windowId?: string) => string;
  /** 🧭️ The ONE boot-axis door of the renderer wasm (`🧊️renderer/🦀️.rs` `WgpuBootDescriptor`), fed the
   * JSON `../🧭️boot-descriptor/🟦️.ts` resolved. It replaced four per-axis setters, which is what let the
   * three wgpu entry points carry three different subsets of the same vocabulary. */
  semioWgpuSetBootDescriptor?: (descriptorJson: string) => void;
  semioWgpuSetHubEnv?: (hubUrl: string, user: string, dataDir: string) => void;
  /** 🌓️ The appearance door (`🧊️renderer/🦀️.rs`, region 🌓️HostAppearance). This realm owns no
   * `window`, so it can neither read `prefers-color-scheme` nor replay the persisted
   * `os.config.ui-preferences` log; the UI isolate makes both reads and hands them here — at boot and
   * again on every change. Without it every browser boot resolved DARK where React resolved LIGHT. */
  semioWgpuSetHostAppearance?: (preference: string, systemDark: boolean) => void;
  /** 🗣️ The locale door (`🧊️renderer/🦀️.rs`, region 🗣️HostLocale). This realm's own `navigator.language`
   * is not the page's promise, so the UI isolate's read (the boot message's `locale`) crosses here once,
   * before the shell resolves its tongue: a lock, then the stored preference, then this. */
  semioWgpuSetHostLocale?: (locale: string) => void;
  /** 🛰️ The agent-bridge door (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `semioWgpuSetAgentBridgeConfig`): the page's discovered
   * offer; an empty url clears it and parks the bridge in `Disabled`. */
  semioWgpuSetAgentBridgeConfig?: (url: string, admissionProof: string) => void;
  /** ⌨️ The platform door (`🧊️renderer/🦀️.rs`, region ⌨️HostPlatform). The renderer's own answer is
   * `cfg!(target_os = "macos")`, which is FALSE in every wasm build, so a macOS browser formatted `mod`
   * as `Ctrl` where React formatted `⌘️`. `userAgentData.platform` is page-thread-only, so the UI
   * isolate makes the read and hands it here once, with the boot. */
  semioWgpuSetHostPlatform?: (platform: string) => void;
  /** 🗄️ The preference door (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, region 🚪️StorageDoor). This realm owns
   * no `localStorage`, so every `prefs_get`/`prefs_set` used to read empty and write to nothing. The UI
   * isolate hands the whole carried census over here BEFORE the shell boots — which is what makes the
   * first frame's appearance, tour-seen and dock-skeleton reads synchronous — and the shell writes back
   * through the host-io door. */
  semioWgpuSetHostStorage?: (snapshotJson: string) => void;
  /** 🩺️ Arms the renderer's own per-frame `[DEBUG]` dumps — `semio_framework_trace::set_runtime_diagnostics`,
   * the wasm door of the `SEMIO_RUNTIME_DIAGNOSTICS` switch a native process reads from its environment
   * and React reads from `localStorage`. */
  semioWgpuSetRuntimeDiagnostics?: (enabled: boolean) => void;
  semioWgpuWorkerBootstrap?: (
    canvas: OffscreenCanvas,
    plugins: readonly { readonly pluginId: string; readonly handle: ReturnType<typeof pluginHandleForBridge> }[],
    pluginFilter: string,
    width: number,
    height: number,
    dpr: number,
    wake: () => void,
  ) => Promise<BrowserRendererBootstrapHandle>;
};
//#endregion 🔖️Bindings

//#region ⏱️StepAuthority
/** @emoji ⏱️ This Worker's step law, declared once for both browser isolates in
 * `../⏱️turn-budget/🟦️.ts`. A step is priced against the EXECUTING spans it actually ran for, an
 * isolated breach is recorded and the work continues, {@link SUSTAINED_TURN_OVERRUN_TURNS} breaches in
 * a row degrade the boot to yielding cadence — and NOTHING here may terminate the Worker. Only a throw
 * is a fault, because only a throw is evidence of a defect.
 *
 * 🩸️ What this replaces: a single `performance.now()` wall delta that THREW. `plugin-graph` executes for
 * 187 µs (native median of 9) and read 8.300 ms of wall time in a hidden pane on a box at load ~20; the
 * throw became `worker-boot-failed`, which closed the Worker and took the transferred OffscreenCanvas —
 * the surface's only frame path — with it. The UI isolate's ledger recorded `0/0` overruns across that
 * same boot, which is the measurement that settles the attribution. */
const BOOT_HEARTBEAT_MS = 2;
/** @emoji 💓️ How often a still-running browser-owned suspension re-posts liveness to the UI isolate, from
 * the shared law in `../🫀️boot-liveness/🟦️.ts`. Posts `boot-liveness`, NOT `boot-progress`: the latter runs
 * the UI isolate's `progress-hook`, whose DOM work overran `FRAME_UI_TURN_BUDGET_MS` (2 ms) once it fired
 * every second — no longer fatal there (`../⏱️turn-budget/🟦️.ts` records and degrades instead of failing),
 * but still pointless work to manufacture.
 *
 * 🩸️ This ticker is NOT the boot's proof of life and never was: it is a `setInterval` on this Worker's own
 * event loop, so it cannot fire while that loop is blocked — which is exactly what compiling the 76 MB
 * renderer, `semioWgpuWorkerBootstrap`'s synchronous prologue and every Rust bootstrap phase do. The proof
 * of life for those is the `boot-phase` DECLARATION {@link monitoredSuspension} posts before it blocks. */
const BOOT_LIVENESS_INTERVAL_MS = FRAME_WORKER_BOOT_LIVENESS_POLICY.livenessIntervalMs;
let lastProgressValue = 0;
/** @emoji 🧭️ Whether phase declarations are still meaningful. They exist for ONE reader — the UI isolate's
 * boot watchdog — which stops listening the moment it sees `booted`. `monitoredSuspension` is also the
 * asset pump's suspension wrapper (`asset-fetch`, `asset-stream-read`, once per 16 KiB page), so leaving
 * declarations open past boot would post two ignored messages per page across the whole session's hot
 * asset path. Closed exactly where the watchdog stops caring. */
let bootDeclarationsOpen = true;
/** @emoji 🧮️ Fixed boot credit taken from the generated catalog itself. A boot plan is by construction a
 * subset of the catalog's own plugin and extension rows, so no legitimate plan can exceed it, and unlike a
 * magic number it cannot go stale as the product grows — a hardcoded 32 rejected the `s` plan's 57 rows
 * outright and made every wgpu boot impossible. */
const PLUGIN_BOOT_CAPACITY = PLUGIN_CATALOG.plugins.length + PLUGIN_CATALOG.extensions.length;
const ASSET_RESPONSE_BYTE_CAPACITY = 16 * 1024 * 1024;
const ASSET_RESPONSE_PAGE_BYTES = 16 * 1024;
/** @emoji 🔁️ How many macrotasks one response seal may wait for the renderer to free its interaction
 * state. Bounded on purpose: a seal that never lands is a renderer defect, not something to spin on. */
const ASSET_SEAL_ATTEMPTS = 64;
/** @emoji 🔬️ Introspection walks the whole retained tree, so it earns a wider turn than a frame step —
 * and a breach is reported on the answer instead of faulting the shell, because a diagnostic must never
 * be the thing that takes the surface down. */
const INTROSPECTION_STEP_BUDGET_MS = 64;
/** @emoji 🧱️ Ceiling for the boot stages whose blocking time is spent inside the browser itself — the module
 * loader, the WebAssembly compiler, and the GPU driver. `WORKER_STEP_BUDGET_MS` prices turns *this* Worker
 * owns and can slice; a `WebAssembly.instantiate` of the renderer module or a `requestDevice` is neither
 * ours nor sliceable (measured: a COLD instantiate of the multi-MB renderer 1093 ms, the font-atlas
 * `renderer-bootstrap` phase 12002 ms, on a machine at load ~36 across 10 cores), so charging it to the
 * interactive ceiling would latch the boot into degraded cadence for work no chunking can shorten. It is
 * therefore a SEPARATE ledger, not a separate verdict: since this lane no budget in this file can end a
 * boot, and the UI isolate's `FRAME_WORKER_BOOT_STALL_TIMEOUT_MS` — which fires on SILENCE, kept honest by
 * `monitoredSuspension`'s liveness heartbeat — is the one remaining outer bound on a wedged instantiate. */
const BROWSER_OWNED_SUSPENSION_BUDGET_MS = 30_000;

const stepClock = new TurnClock(() => performance.now());
/** @emoji 📒️ Prices the turns this Worker OWNS and can slice, against {@link WORKER_STEP_BUDGET_MS}. */
const stepLedger = new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step");
/** @emoji 🧱️ Prices the stages whose blocking time belongs to the browser itself — the module loader,
 * the WebAssembly compiler, the GPU driver. None of it is ours to slice, so it is measured against the
 * wedge ceiling instead of the interactive one and never degrades the boot's cadence. */
const suspensionLedger = new TurnLedger(BROWSER_OWNED_SUSPENSION_BUDGET_MS, "browser-owned-suspension");

/** @emoji 🧾️ The verdict the most recently CLOSED owned step earned, so a caller that must report its own
 * step's price — the frame reply — reads the executing measurement instead of taking a second wall sample. */
let lastStepOutcome: TurnOutcome | undefined;

/** @emoji ⏱️ Runs one owned step inside the executing clock and admits it to its ledger. The measurement
 * happens whether the callback returns or throws; the THROW is the only thing that propagates. */
function ownedStep<T>(stage: string, callback: () => T, ledger: TurnLedger = stepLedger): T {
  stepClock.enter();
  try {
    return callback();
  } finally {
    lastStepOutcome = ledger.admit(stage, stepClock.leave());
  }
}

/** @emoji 🧭️ One SYNCHRONOUS owned step long enough to be worth declaring. A blocking Rust bootstrap phase
 * (`font-atlas` measured at 12 002 ms) posts nothing while it runs — the same blindness a browser-owned
 * suspension has — so it declares itself to the UI-isolate watchdog first and withdraws afterwards, and its
 * real cost rides out on the withdrawal. */
function declaredStep<T>(stage: string, callback: () => T, ledger: TurnLedger = stepLedger): T {
  const startedAt = performance.now();
  declarePhase(stage, "enter", 0);
  try {
    return ownedStep(stage, callback, ledger);
  } finally {
    declarePhase(stage, "leave", performance.now() - startedAt);
  }
}

/** @emoji 🐢️ Whether this Worker's owned steps have sustained a run of overruns and the boot should hand
 * the isolate back between chunks. Latches on the run and clears itself on the first admitted step. */
function stepsDegraded(): boolean {
  return stepLedger.degraded();
}

/** @emoji 📊️ What the UI isolate is told about this Worker's own step ledger on every boot report. */
function stepLedgerReport(): { readonly degraded: boolean; readonly recordedOverruns: number; readonly sustainedOverruns: number; readonly worstStepMs: number; readonly worstStepSite: string } {
  const snapshot = stepLedger.snapshot();
  return { degraded: snapshot.degraded, recordedOverruns: snapshot.recordedOverruns, sustainedOverruns: snapshot.sustainedOverruns, worstStepMs: snapshot.worstExecutingMs, worstStepSite: snapshot.worstSite };
}

/** @emoji ⏸️ Awaits one browser-owned operation. The synchronous prologue is an owned step; the blocking
 * gap the heartbeat measures is admitted to {@link suspensionLedger} as an observation — a Worker that did
 * not run is the machine's report about the machine, never evidence about this step's own work, so it can
 * no longer end the boot. `FRAME_WORKER_BOOT_TIMEOUT_MS` in the UI isolate remains the outer bound and the
 * liveness heartbeat keeps it honest. */
async function monitoredSuspension<T>(stage: string, operation: () => Promise<T>, ledger: TurnLedger = suspensionLedger): Promise<T> {
  const declaredAt = performance.now();
  declarePhase(stage, "enter", 0);
  let lastBeat = performance.now();
  let lastLivenessAt = performance.now();
  let maximumBlockMs = 0;
  const heartbeat = setInterval(() => {
    const now = performance.now();
    maximumBlockMs = Math.max(maximumBlockMs, now - lastBeat - BOOT_HEARTBEAT_MS);
    lastBeat = now;
    if (now - lastLivenessAt >= BOOT_LIVENESS_INTERVAL_MS) {
      lastLivenessAt = now;
      if (!closed && !closing && !failed) post({ kind: "boot-liveness", lifecycle });
    }
  }, BOOT_HEARTBEAT_MS);
  try {
    stepClock.suspend();
    const result = await ownedStep(`${stage}:start`, operation, ledger);
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
    if (closed || closing) throw new Error(`worker-boot-cancelled: ${stage}`);
    ledger.admit(`${stage}:block`, maximumBlockMs);
    return result;
  } finally {
    clearInterval(heartbeat);
    stepClock.resume();
    declarePhase(stage, "leave", performance.now() - declaredAt);
  }
}

async function macrotask(): Promise<void> {
  stepClock.suspend();
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
  stepClock.resume();
  if (closed || closing) throw new Error("worker-boot-cancelled");
}

/** @emoji 🧩️ A unit of boot work that can be performed one bounded chunk at a time — the shape
 * `PlaygroundBootPlanner` and the Rust `BrowserRendererBootstrap` phase machine both take. */
type ResumableBootUnit = { stage(): string; completion(): number; step(): boolean };

/** @emoji ⏭️ Drives one resumable unit to completion, one chunk per owned step, reporting each chunk as
 * boot progress and handing the isolate back between chunks whenever the ledger says this Worker's own
 * steps are running long. Chunk size is the unit's own business; the ceiling is never the unit's. */
async function driveChunks(unit: ResumableBootUnit, base: number, span: number): Promise<void> {
  for (;;) {
    const stage = unit.stage();
    const more = ownedStep(stage, () => unit.step());
    progress(stage, base + unit.completion() * span);
    if (!more) return;
    if (stepsDegraded()) await macrotask();
  }
}

//#endregion ⏱️StepAuthority

//#region 🧵️Worker
const scope = self as DedicatedWorkerGlobalScope;

/** @emoji 🩺️ The diagnostics preference the UI isolate stamped on THIS worker's url — the only channel
 * that exists before the boot message, and the one the renderer wasm's own per-frame `[DEBUG]` dumps are
 * armed from. A Worker owns no storage, so the page resolves `SEMIO_RUNTIME_DIAGNOSTICS` and stamps;
 * `undefined` (no stamp) leaves the build-time switch in charge. */
const diagnosticsStamp = stampedTurnDiagnostics(scope.location.search);
setTurnDiagnostics(diagnosticsStamp);

/** @emoji 🧭️ A wasm panic spends V8's default ten stack frames entirely inside the panic machinery
 * (`capacity_overflow` → `panic_fmt` → `rust_begin_unwind` → the hook), so the Rust frame that
 * actually asked for the allocation is always the one cut off. The renderer's panic hook mints an
 * `Error` to capture that stack (`🌐️browser-worker/🦀️.rs` `install_worker_panic_trace`), and this is
 * what makes it reach past the machinery. */
Error.stackTraceLimit = 64;
let lifecycle = 0;
let runtime: BrowserRendererWorkerHandle | undefined;
let bindings: RendererBindings | undefined;
let interactiveJobs: InteractiveWorkerScheduler | undefined;
let frameTurns: FrameTurnScheduler | undefined;
let frameTurnTasks: WorkerTurnTaskQueue | undefined;
let frameInput: { readonly timestampMs: number; readonly generation: number } | undefined;
let frameSequence = 0;
let closed = false;
let closing = false;
let failed = false;
let quarantined: { readonly code: string; readonly detail: string } | undefined;
let lastFrame = { cursor: "default", fullscreen: null as boolean | null };
let pendingFault: { readonly code: string; readonly detail: string } | undefined;
let runtimeCloseComplete = false;
let jobsCloseComplete = false;
let closeOwner: "runtime" | "jobs" = "runtime";
let assetPumping = false;
let assetAbort: AbortController | undefined;
const assetCancellation = new BrowserAssetCancellationCursor();
let nextImageDecodeRequestId = 1;
let pageImageDecode: { readonly requestId: number; readonly resolve: (bitmap: ImageBitmap) => void; readonly reject: (error: Error) => void } | undefined;

scope.onmessage = (event: MessageEvent<BrowserFrameUiMessage>) => void receive(event.data);

/** @emoji 🧯️ Last-resort seam for a throw that escapes every awaited step — a trap raised inside a wasm
 * callback the renderer scheduled itself, or a rejection nothing awaited. Without this the UI isolate only
 * sees `worker.onerror`'s bare message as `worker-message-failed`, with no stack and no code to triage; the
 * protocol already carries a typed fault, so route it there instead and let the close ladder run. */
scope.onerror = (event) => {
  const error = event instanceof ErrorEvent ? event : undefined;
  const stack = error?.error instanceof Error ? `\n${error.error.stack ?? ""}` : "";
  requestFault("worker-uncaught", `${error?.message ?? "uncaught worker error"} (${error?.filename ?? "?"}:${error?.lineno ?? 0}:${error?.colno ?? 0})${stack}`);
  return true;
};

scope.onunhandledrejection = (event) => {
  const reason = (event as PromiseRejectionEvent).reason;
  requestFault("worker-unhandled-rejection", reason instanceof Error ? `${reason.message}\n${reason.stack ?? ""}` : String(reason));
};

async function receive(message: BrowserFrameUiMessage): Promise<void> {
  if (message.kind === "boot") {
    await boot(message);
    return;
  }
  if (message.kind === "shard-port") return;
  if (message.kind === "image-decode-result" && message.lifecycle !== lifecycle) {
    message.bitmap?.close();
    return;
  }
  if (message.lifecycle !== lifecycle) return;
  if (message.kind === "close") {
    if (closed || closing) return;
    beginClose();
    return;
  }
  if (message.kind === "introspect") {
    answerIntrospection(message);
    return;
  }
  if (message.kind === "host-io-result") {
    settleHostIo(message);
    return;
  }
  if (message.kind === "image-decode-result") {
    settlePageImageDecode(message);
    return;
  }
  if (message.kind === "host-appearance") {
    ownedStep("host-appearance", () => bindings?.semioWgpuSetHostAppearance?.(message.appearance.preference, message.appearance.systemDark));
    return;
  }
  if (message.kind === "host-storage") {
    ownedStep("host-storage", () => bindings?.semioWgpuSetHostStorage?.(JSON.stringify(message.storage)));
    return;
  }
  if (message.kind === "host-agent-bridge") {
    ownedStep("host-agent-bridge", () => bindings?.semioWgpuSetAgentBridgeConfig?.(message.offer?.url ?? "", message.offer?.admissionProof ?? ""));
    return;
  }
  if (closed || closing || failed || quarantined) return;
  if (message.kind === "job-submit" || message.kind === "job-input-page" || message.kind === "job-cancel") {
    if (!interactiveJobs) {
      fault("interactive-job-not-ready", "interactive job arrived before Worker boot completed");
      return;
    }
    ownedStep(`interactive-job:${message.kind}`, () => interactiveJobs!.receive(message));
    return;
  }
  if (!runtime) {
    fault("worker-not-booted", "frame batch arrived before renderer boot completed");
    return;
  }
  try {
    ownedStep("frame-ingress", () => runtime!.enqueueBatch(JSON.stringify({ replaceable: message.replaceable, lossless: message.lossless }), BigInt(message.generation)));
    frameInput = { timestampMs: message.timestampMs, generation: message.generation };
    post({ kind: "batch-accepted", lifecycle, inputSequence: message.inputSequence, generation: message.generation });
    frameTurns?.request();
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
  }
}

function runFrameTurn(): boolean {
  if (!runtime || !frameInput || closed || closing || failed || quarantined) return false;
  const startedAt = performance.now();
  try {
    frameSequence = nextFrameSequence(frameSequence);
    const input = frameInput;
    const result = ownedStep("frame-step", () => JSON.parse(runtime!.tick(input.timestampMs, BigInt(frameSequence), BigInt(input.generation)))) as Omit<Extract<BrowserFrameWorkerMessage, { kind: "frame" }>, "kind" | "lifecycle" | "frameSequence" | "generation" | "workerDurationMs"> & { readonly continueFrame: boolean };
    const outcome = lastStepOutcome;
    lastFrame = { cursor: result.cursor, fullscreen: result.fullscreen };
    const assetCancellationStep = assetCancellation.step({
      hasFetch: () => assetAbort !== undefined,
      hasPageImageDecode: () => pageImageDecode !== undefined,
      responseCurrent: () => runtime!.assetResponseCurrent(),
      abortFetch: () => assetAbort?.abort(),
      cancelPageImageDecode,
      returnResponseOwner: () => runtime!.abortAssetResponse(),
    });
    if (result.quarantined) quarantined = { code: result.faultCode ?? "renderer-quarantine", detail: result.faultDetail ?? "renderer quarantined its own frame step" };
    const sustained = outcome?.verdict === "sustained-overrun";
    const degrade = quarantined ?? (sustained ? { code: "worker-step-overrun", detail: `frame step executed ${outcome!.executingMs.toFixed(3)} ms for ${outcome!.consecutive} consecutive steps` } : undefined);
    post({ kind: "frame", lifecycle, frameSequence, generation: input.generation, cursor: result.cursor, fullscreen: result.fullscreen, requestFrame: result.requestFrame, progress: result.progress, workerDurationMs: performance.now() - startedAt, workerExecutingMs: outcome?.executingMs ?? 0, workerStepVerdict: outcome?.verdict ?? "clock-fault", quarantined: degrade !== undefined, faultCode: degrade?.code, faultDetail: degrade?.detail });
    if (quarantined) requestFault(quarantined.code, quarantined.detail);
    else if (assetCancellationStep === "idle") scheduleAssetPump();
    else if (assetCancellationStep === "returned" && !assetPumping) {
      assetCancellation.releaseReturned();
      scheduleAssetPump();
    }
    return result.continueFrame;
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
    return false;
  }
}

function runAssetDecodeTurn(): boolean {
  if (!runtime || closed || closing || failed || quarantined) return false;
  try {
    const result = ownedStep("asset-decode-step", () => JSON.parse(runtime!.assetDecodeStep()) as RendererAssetDecodeStep);
    if (!(["pending", "published", "cancelled", "idle", "fault"] as const).includes(result.kind) || typeof result.pending !== "boolean") {
      throw new Error("asset-decode-protocol: invalid step result");
    }
    if (result.kind === "published") post({ kind: "wake", lifecycle });
    if (result.kind === "fault") {
      fault("asset-decode-fault", result.detail ?? "renderer asset decode faulted without detail");
      return false;
    }
    return result.pending;
  } catch (error) {
    fault("asset-decode-fault", error instanceof Error ? error.message : String(error));
    return false;
  }
}

/** @emoji 🔬️ Answers one read-only introspection request from the renderer's own thread-local, which only
 * exists in this isolate. It never faults the Worker: a missing export, a throwing export, or a turn wider
 * than `INTROSPECTION_STEP_BUDGET_MS` all come back as `json: null` plus a `detail`, so a probe can tell
 * "no hooks" from "empty dump" while the surface keeps running. */
function answerIntrospection(message: Extract<BrowserFrameUiMessage, { kind: "introspect" }>): void {
  const respond = (json: string | null, detail?: string) => post({ kind: "introspection", lifecycle, requestId: message.requestId, probe: message.probe, json, ...(detail === undefined ? {} : { detail }) });
  if (closed) return;
  if (!runtime || !bindings) {
    respond(null, "renderer bindings are not mounted in this Worker");
    return;
  }
  const hook = message.probe === "structure" ? bindings.dumpStructure : message.probe === "accessibility" ? bindings.dumpAccessibility : message.probe === "mesh-stats" ? bindings.dumpMeshStats : message.probe === "chrome" ? bindings.dumpChrome : bindings.dumpFrameStats;
  if (!hook) {
    respond(null, `renderer bindings expose no ${message.probe} introspection export`);
    return;
  }
  const startedAt = performance.now();
  try {
    const json = hook(message.windowId);
    const duration = performance.now() - startedAt;
    respond(json, duration >= INTROSPECTION_STEP_BUDGET_MS ? `${message.probe} introspection took ${duration.toFixed(3)} ms` : undefined);
  } catch (error) {
    respond(null, error instanceof Error ? error.message : String(error));
  }
}

async function closeRuntime(): Promise<void> {
  for (;;) {
    ownedStep("close-step", () => {
      if (closeOwner === "runtime" && !runtimeCloseComplete) {
        runtimeCloseComplete = runtime ? runtime.closeStep() : true;
        closeOwner = "jobs";
      } else if (!jobsCloseComplete) {
        jobsCloseComplete = interactiveJobs ? interactiveJobs.closeStep() : true;
        closeOwner = "runtime";
      } else if (!runtimeCloseComplete) {
        closeOwner = "runtime";
      }
    });
    if (runtimeCloseComplete && jobsCloseComplete && (frameTurns?.terminalIsEmpty() ?? true)) break;
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  }
  if (pendingFault) post({ kind: "fault", lifecycle, code: pendingFault.code, detail: pendingFault.detail });
  post({ kind: "closed", lifecycle });
  frameTurnTasks?.close();
  frameTurnTasks = undefined;
  closed = true;
  scope.close();
}

function beginClose(): void {
  if (closed || closing) return;
  closing = true;
  failed = pendingFault !== undefined;
  runtimeCloseComplete = runtime === undefined;
  jobsCloseComplete = interactiveJobs === undefined;
  frameTurns?.beginClose();
  assetAbort?.abort();
  assetAbort = undefined;
  cancelPageImageDecode();
  if (runtime) {
    try {
      ownedStep("asset-abort", () => runtime!.abortAssetResponse());
    } catch (error) {
      pendingFault ??= { code: "asset-abort-fault", detail: error instanceof Error ? error.message : String(error) };
    }
  }
  interactiveJobs?.close();
  void closeRuntime();
}

type PluginHandleMount = { readonly pluginId: string; readonly handle: ReturnType<typeof pluginHandleForBridge> };

/** @emoji 🧩️ Mounts every plugin the boot plan names, isolating each one. A module that fails to load, or
 * whose manifest overruns its fixed credits, is reported as a `plugin-fault:` boot-progress stage and
 * skipped — the same per-plugin isolation the React shell's router gives a descriptor fault, rather than
 * taking the whole surface down. The catalogue cache legitimately carries stale or missing descriptors
 * while plugin cores are rebuilt, and a shell that boots only when all of them are fresh never boots.
 * Cancellation (`closed`/`closing`) is re-thrown, never swallowed; an empty result is still fatal upstream. */
async function mountPluginHandles(targets: readonly { readonly pluginId: string; readonly moduleUrl: string }[]): Promise<PluginHandleMount[]> {
  const mounted: PluginHandleMount[] = [];
  for (let index = 0; index < targets.length; index++) {
    const target = targets[index]!;
    const share = 0.3 + 0.3 * (index / Math.max(1, targets.length));
    progress(`plugin:${target.pluginId}`, share);
    await macrotask();
    try {
      const module = await monitoredSuspension(`plugin:${target.pluginId}`, () => loadPluginModule(target.pluginId, target.moduleUrl), suspensionLedger);
      mounted.push(ownedStep(`plugin-handle:${target.pluginId}`, () => ({ pluginId: target.pluginId, handle: pluginHandleForBridge(module) })));
    } catch (error) {
      if (closed || closing) throw error;
      progress(`plugin-fault:${target.pluginId}: ${error instanceof Error ? error.message : String(error)}`, share);
    }
  }
  return mounted;
}

/** @emoji 📶️ How many buckets the 76 MB download reports itself in — one `boot-progress` every 2 %, which
 * is real streamed progress rather than a stage name that sits still for the whole transfer, and is far
 * below the rate at which the UI isolate's `progress-hook` would become the expensive thing. */
const WASM_FETCH_PROGRESS_BUCKETS = 50;

/** @emoji 🧱️ Compiles the renderer wasm WHILE it downloads, reporting the transfer as it goes.
 * `compileStreaming` over a counting `TransformStream` keeps the browser's streaming compilation — the
 * bytes are never buffered into one 76 MB array — while every 2 % of the body posts a `boot-progress`, so
 * the longest phase of the boot stops being a stage name that sits still. Answers `undefined` for every
 * failure (a browser that will not `compileStreaming` a constructed `Response`, a transport fault), which
 * hands the caller back to wasm-bindgen's own URL path rather than failing the boot. */
async function compileRendererModule(url: string): Promise<{ readonly module: WebAssembly.Module; readonly byteLength: number } | undefined> {
  try {
    const response = await fetch(url);
    if (!response.ok || !response.body) return undefined;
    const total = Number(response.headers.get("content-length") ?? 0);
    let received = 0;
    let reported = -1;
    const counted = response.body.pipeThrough(
      new TransformStream<Uint8Array, Uint8Array>({
        transform(chunk, controller) {
          received += chunk.byteLength;
          const share = total > 0 ? received / total : 0;
          const bucket = Math.floor(share * WASM_FETCH_PROGRESS_BUCKETS);
          if (total > 0 && bucket > reported) {
            reported = bucket;
            progress(`wasm-compile ${Math.round(share * 100)} %`, 0.06 + 0.09 * share);
          }
          controller.enqueue(chunk);
        },
      }),
    );
    const module = await WebAssembly.compileStreaming(new Response(counted, { headers: { "content-type": "application/wasm" } }));
    return { module, byteLength: received };
  } catch {
    return undefined;
  }
}

/** @emoji 🧱️ Brings the renderer wasm up in DECLARED phases instead of one opaque `init(url)`.
 *
 * 🩸️ What this replaces: `loaded.default(url)` — one call that fetched, compiled, instantiated and linked
 * 76 048 601 B behind a single stage name, reported nothing while it ran, and re-did all of it on every
 * reload. The four phases here each declare themselves to the UI-isolate watchdog before they block
 * (`monitoredSuspension`), the compile reports its own transfer, and the compiled module is kept in
 * IndexedDB (`../🗄️wasm-module-cache/🟦️.ts`) so the second boot on an unchanged artifact skips the compile
 * entirely and goes straight to instantiation. A cache miss costs one `HEAD`; a browser that cannot cache
 * a `WebAssembly.Module` at all costs the same `HEAD` and nothing else. */
async function instantiateRendererWasm(bindings: RendererBindings, url: string): Promise<void> {
  const tag = await monitoredSuspension("wasm-artifact", () => rendererArtifactTag(url), suspensionLedger);
  const cached = tag ? await monitoredSuspension("wasm-cache-read", () => readCachedRendererModule(url, tag), suspensionLedger) : undefined;
  if (cached) {
    progress(`wasm-instantiate:cached ${(cached.byteLength / 1_048_576).toFixed(1)} MB`, 0.18);
    await monitoredSuspension("wasm-instantiate", () => bindings.default!(cached.module), suspensionLedger);
    return;
  }
  progress("wasm-compile", 0.06);
  const compiled = await monitoredSuspension("wasm-compile", () => compileRendererModule(url), suspensionLedger);
  progress("wasm-instantiate", 0.18);
  if (!compiled) {
    await monitoredSuspension("wasm-instantiate", () => bindings.default!(url), suspensionLedger);
    return;
  }
  await monitoredSuspension("wasm-instantiate", () => bindings.default!(compiled.module), suspensionLedger);
  if (tag) void writeCachedRendererModule(url, tag, compiled.module, compiled.byteLength, performance.now());
  else void evictCachedRendererModule(url);
}

async function boot(message: Extract<BrowserFrameUiMessage, { kind: "boot" }>): Promise<void> {
  if (runtime || lifecycle !== 0) {
    fault("duplicate-boot", "the frame Worker accepts exactly one boot lifecycle");
    return;
  }
  lifecycle = message.lifecycle;
  try {
    progress("renderer-module", 0.05);
    const loaded = await monitoredSuspension("renderer-module", () => import(/* @vite-ignore */ message.bindingsModuleUrl) as Promise<RendererBindings>, suspensionLedger);
    bindings = loaded;
    if (loaded.default) await instantiateRendererWasm(loaded, message.bindingsWasmUrl);
    if (!loaded.semioWgpuWorkerBootstrap) throw new Error("renderer bindings missing semioWgpuWorkerBootstrap");
    ownedStep("runtime-environment", () => {
      loaded.semioWgpuSetRuntimeDiagnostics?.(diagnosticsStamp === true);
      loaded.semioWgpuSetBootDescriptor?.(JSON.stringify(message.descriptor));
      loaded.semioWgpuSetHostStorage?.(JSON.stringify(message.storage));
      loaded.semioWgpuSetHostAppearance?.(message.appearance.preference, message.appearance.systemDark);
      loaded.semioWgpuSetHostPlatform?.(message.platform);
      loaded.semioWgpuSetHostLocale?.(message.locale);
      if (message.descriptor.hub) loaded.semioWgpuSetHubEnv?.(message.descriptor.hub.hubUrl, message.descriptor.hub.user, message.descriptor.hub.dataDir);
    }, suspensionLedger);
    progress("plugin-graph", 0.25);
    const planner = new PlaygroundBootPlanner(PLUGIN_CATALOG, message.descriptor.pluginVariant);
    await driveChunks(planner, 0.25, 0.05);
    const bootPlan = ownedStep("plugin-graph:finish", () => planner.finish());
    if (bootPlan.plugins.length > PLUGIN_BOOT_CAPACITY) throw new Error(`plugin-credits: boot plan exceeds ${PLUGIN_BOOT_CAPACITY} plugins`);
    for (const error of bootPlan.dependencyErrors) progress(pluginGraphErrorMessage(error, message.locale), 0.3);
    const plugins = await mountPluginHandles(bootPlan.plugins);
    await Promise.all(bootPlan.plugins.map((target) => primeContributionManifest(target.pluginId, target.moduleUrl).catch((error) => {
    })));
    if (plugins.length === 0) throw new Error(`no wasm plugin modules found for variant ${message.descriptor.pluginVariant}`);
    progress("renderer-runtime", 0.65);
    let bootstrap = await monitoredSuspension("gpu-platform", () => loaded.semioWgpuWorkerBootstrap!(message.canvas, plugins, bootPlan.variant, message.width, message.height, message.dpr, () => frameTurns?.requestRuntimeWake()), suspensionLedger);
    while (true) {
      await macrotask();
      const step = declaredStep("renderer-bootstrap", () => JSON.parse(bootstrap.step()) as BrowserRendererBootStep, suspensionLedger);
      if (diagnosticsStamp === true) console.debug(`[DEBUG] wgpu boot stage=${step.stage} phaseUs=${step.elapsedUs} progress=${step.progress}`);
      progress(step.stage, 0.65 + step.progress * 0.3);
      if (step.shellBoot) {
        bootstrap = await monitoredSuspension("shell-boot", () => bootstrap.bootShell(), suspensionLedger);
        continue;
      }
      if (step.complete) break;
    }
    runtime = declaredStep("renderer-finish", () => bootstrap.finish(), suspensionLedger);
    frameTurnTasks = declaredStep("frame-turn-task-owner", () => new WorkerTurnTaskQueue(), suspensionLedger);
    frameTurns = declaredStep("frame-turn-scheduler", () => new FrameTurnScheduler(frameTurnTasks!.schedule, runFrameTurn, () => {
      frameInput = undefined;
      return true;
    }, runAssetDecodeTurn), suspensionLedger);
    interactiveJobs = declaredStep("interactive-job-registry", () => new InteractiveWorkerScheduler(lifecycle, INTERACTIVE_WORKER_DESCRIPTORS, post, (callback) => setTimeout(callback, 0), () => performance.now(), (detail) => fault("interactive-job-fault", detail)), suspensionLedger);
    progress("ready", 1);
    bootDeclarationsOpen = false;
    post({ kind: "booted", lifecycle });
    scheduleAssetPump();
  } catch (error) {
    fault("worker-boot-failed", error instanceof Error ? error.message : String(error));
  }
}

type AssetRequest = {
  readonly available: boolean;
  readonly url?: string;
  readonly referenceImage?: boolean;
  readonly responseByteCapacity?: number;
  readonly pageByteCapacity?: number;
};

function requestPageImageDecode(source: Uint8Array<ArrayBuffer>, dimensions: ReferenceImageDimensions): Promise<ImageBitmap> {
  if (pageImageDecode) return Promise.reject(new Error("reference-image-decode-credits"));
  const requestId = nextImageDecodeRequestId++;
  return new Promise<ImageBitmap>((resolve, reject) => {
    pageImageDecode = { requestId, resolve, reject };
    try {
      post({ kind: "image-decode", lifecycle, requestId, source, dimensions }, [source.buffer]);
    } catch (error) {
      pageImageDecode = undefined;
      reject(error instanceof Error ? error : new Error(String(error)));
    }
  });
}

function settlePageImageDecode(message: Extract<BrowserFrameUiMessage, { readonly kind: "image-decode-result" }>): void {
  const pending = pageImageDecode;
  if (!pending || pending.requestId !== message.requestId) {
    message.bitmap?.close();
    return;
  }
  pageImageDecode = undefined;
  if (message.bitmap) pending.resolve(message.bitmap);
  else pending.reject(new Error(message.detail ?? "reference-image-decode-refused"));
}

function cancelPageImageDecode(): void {
  const pending = pageImageDecode;
  if (!pending) return;
  pageImageDecode = undefined;
  try {
    post({ kind: "image-decode-cancel", lifecycle, requestId: pending.requestId });
  } catch {}
  pending.reject(new DOMException("reference image generation retired", "AbortError"));
}

function scheduleAssetPump(): void {
  if (!assetCancellation.pollAdmitted() || assetPumping || !runtime || closed || closing || failed || quarantined) return;
  assetPumping = true;
  setTimeout(() => void pumpAsset(), 0);
}

async function pumpAsset(): Promise<void> {
  let activeRequest: AssetRequest | undefined;
  let responseController: AbortController | undefined;
  try {
    if (!runtime || closed || closing || failed || quarantined) return;
    const request = ownedStep("asset-request", () => JSON.parse(runtime!.pollAssetRequest()) as AssetRequest);
    activeRequest = request;
    if (!request.available) return;
    if (!request.url || request.responseByteCapacity !== ASSET_RESPONSE_BYTE_CAPACITY || request.pageByteCapacity !== ASSET_RESPONSE_PAGE_BYTES) {
      throw new Error("asset-request-protocol: request descriptor did not match fixed Worker credits");
    }
    responseController = new AbortController();
    assetAbort = responseController;
    const responseCurrent = (): void => assertBrowserAssetResponseContinuation(responseController!, () => ownedStep("asset-response-current", () => runtime!.assetResponseCurrent()));
    const response = await monitoredSuspension("asset-fetch", () => fetch(meshAssetTransportUrl(request.url!), { signal: responseController!.signal }));
    responseCurrent();
    if (!response.ok || !response.body) throw new Error(`asset-fetch-status: ${response.status}`);
    const declaredHeader = ownedStep("asset-response-headers", () => response.headers.get("content-length"));
    const declared = declaredHeader === null ? undefined : Number(declaredHeader);
    if (declared !== undefined && (!Number.isSafeInteger(declared) || declared < 0 || declared > ASSET_RESPONSE_BYTE_CAPACITY)) throw new Error("asset-response-length: Content-Length exceeded fixed aggregate credits");
    let reserved = false;
    for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && !reserved; attempt++) {
      responseCurrent();
      reserved = ownedStep("asset-response-reserve", () => runtime!.reserveAssetResponse(declared ?? ASSET_RESPONSE_BYTE_CAPACITY));
      if (!reserved) {
        await macrotask();
        responseCurrent();
      }
    }
    if (!reserved) throw new Error("asset-reserve-busy: the renderer never freed its interaction state for the response credits");
    const reader = ownedStep("asset-stream-reader", () => response.body!.getReader({ mode: "byob" }) as ReadableStreamBYOBReader);
    let received = 0;
    const referenceChunks: Uint8Array<ArrayBuffer>[] = [];
    for (;;) {
      responseCurrent();
      const pageOwner = ownedStep("asset-page-owner", () => new Uint8Array(ASSET_RESPONSE_PAGE_BYTES));
      const chunk = await monitoredSuspension("asset-stream-read", () => reader.read(pageOwner));
      responseCurrent();
      if (chunk.done) break;
      const bytes = chunk.value;
      if (bytes.byteLength === 0 || bytes.byteLength > ASSET_RESPONSE_PAGE_BYTES) throw new Error("asset-response-page: stream violated fixed BYOB page credits");
      received += bytes.byteLength;
      if (received > (declared ?? ASSET_RESPONSE_BYTE_CAPACITY)) throw new Error("asset-response-overflow: stream exceeded admitted bytes");
      responseCurrent();
      ownedStep("asset-page", () => runtime!.pushAssetResponsePage(bytes));
      if (request.referenceImage) referenceChunks.push(Uint8Array.from(bytes));
      await macrotask();
      responseCurrent();
    }
    ownedStep("asset-stream-release", () => reader.releaseLock());
    if (declared !== undefined && received !== declared) throw new Error("asset-response-short-read: stream ended before declared bytes");
    if (request.referenceImage) {
      responseCurrent();
      const source = ownedStep("reference-image-source", () => concatenateReferenceImageSource(referenceChunks));
      const dimensions = ownedStep("reference-image-dimensions", () => referenceImageSourceDimensions(source));
      const digest = await monitoredSuspension("reference-image-digest", () => referenceImageSourceDigest(source), suspensionLedger);
      responseCurrent();
      const [width, height] = referenceImageTargetSize(dimensions.width, dimensions.height);
      let stageMode = 0;
      for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && stageMode === 0; attempt++) {
        responseCurrent();
        stageMode = ownedStep("reference-image-begin", () => runtime!.beginReferenceImage(width, height, digest));
        if (stageMode === 0) {
          post({ kind: "wake", lifecycle });
          await macrotask();
          responseCurrent();
        }
      }
      if (stageMode === 0) throw new Error("reference-image-begin-busy");
      if (stageMode !== 1 && stageMode !== 2) throw new Error("reference-image-begin-mode");
      const bitmap = await referenceImageBitmapForStage(
        stageMode,
        () => monitoredSuspension("reference-image-decode", () => decodeReferenceImage(new Blob([source], { type: dimensions.mediaType }), dimensions, () => runtime?.assetResponseCurrent() === true), suspensionLedger),
        () => monitoredSuspension("reference-image-page-decode", () => requestPageImageDecode(source, dimensions), suspensionLedger),
      );
      responseCurrent();
      if (bitmap) {
        try {
          await streamReferenceImageBitmapRows(
            bitmap,
            width,
            height,
            (offset, pixels) => {
              responseCurrent();
              return runtime!.pushReferenceImageRows(offset, pixels);
            },
            () => {
              responseCurrent();
              return true;
            },
            async () => {
              post({ kind: "wake", lifecycle });
              await macrotask();
              responseCurrent();
            },
            (operation) => ownedStep("reference-image-row-strip", operation),
          );
        } finally {
          ownedStep("reference-image-bitmap-close", () => bitmap.close());
        }
      }
      responseCurrent();
      if (!ownedStep("reference-image-seal", () => runtime!.sealReferenceImage())) throw new Error("reference-image-seal-busy");
    }
    // 🔏️ The seal needs the renderer's own interaction state, which a live apply owns for the length
    // of that apply. `sealAssetResponse` answers `false` for exactly that — back-pressure, not a
    // refusal — and leaves the request untouched, so this comes back on the next macrotask instead of
    // raising `asset-stream-fault` and quarantining the whole shell over a lock that is already free
    // (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
    let sealed = false;
    for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && !sealed; attempt++) {
      responseCurrent();
      sealed = ownedStep("asset-seal", () => runtime!.sealAssetResponse());
      if (!sealed) {
        await macrotask();
        responseCurrent();
      }
    }
    if (!sealed) throw new Error("asset-seal-busy: the renderer never freed its interaction state for the sealed response");
    frameTurns?.request("assetDecode");
  } catch (error) {
    const cancelled = error instanceof DOMException && error.name === "AbortError";
    if (runtime && !cancelled) {
      try {
        if (activeRequest?.referenceImage && !cancelled) {
          let rejected = false;
          for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && !rejected; attempt++) {
            rejected = ownedStep("asset-reject", () => runtime!.rejectAssetResponse());
            if (!rejected) await macrotask();
          }
          if (!rejected) ownedStep("asset-abort", () => runtime!.abortAssetResponse());
        } else ownedStep("asset-abort", () => runtime!.abortAssetResponse());
      } catch {}
    }
    if (activeRequest?.referenceImage && !closing && !closed && !failed) post({ kind: "wake", lifecycle });
    if (!activeRequest?.referenceImage && !cancelled && !closing && !closed) fault("asset-stream-fault", error instanceof Error ? error.message : String(error));
  } finally {
    if (assetAbort === responseController) assetAbort = undefined;
    assetPumping = false;
    if (assetCancellation.releaseReturned()) scheduleAssetPump();
  }
}

/** @emoji 📣️ Reports one boot stage AND this Worker's own step ledger with it, so the boot UI shows the
 * degraded state instead of only the stage that happened to be running when the isolate was descheduled. */
function progress(stage: string, value: number): void {
  lastProgressValue = value;
  if (!closed && !closing && !failed) post({ kind: "boot-progress", lifecycle, stage, progress: value, worker: stepLedgerReport() });
}

/** @emoji 🧭️ DECLARES that this Worker is about to block on one browser-owned phase — or that it has left
 * it. Posted while the event loop still runs, which is the whole point: a `postMessage` issued a tick
 * before a multi-second `WebAssembly.compile` reaches the UI isolate, a `boot-liveness` issued DURING it
 * never does. The UI-isolate watchdog measures a declared phase against its own ceiling
 * (`../🫀️boot-liveness/🟦️.ts`) instead of against silence, so it can no longer terminate a Worker that is
 * merely busy, and `elapsedMs` on the withdrawal is the per-phase cost the next boot is measured by. */
const bootPhaseStack: string[] = [];

function declarePhase(phase: string, state: "enter" | "leave", elapsedMs: number): void {
  if (!bootDeclarationsOpen || closed || closing || failed) return;
  if (state === "enter") {
    bootPhaseStack.push(phase);
    post({ kind: "boot-phase", lifecycle, phase, state, elapsedMs });
    return;
  }
  post({ kind: "boot-phase", lifecycle, phase, state, elapsedMs });
  const top = bootPhaseStack[bootPhaseStack.length - 1];
  if (top === phase) bootPhaseStack.pop();
  const parent = bootPhaseStack[bootPhaseStack.length - 1];
  if (parent) post({ kind: "boot-phase", lifecycle, phase: parent, state: "enter", elapsedMs: 0 });
}

/** @emoji 🧭️ Nested `boot-phase` under an already-declared parent. Re-enters the parent on leave so the watchdog never sees an undeclared Worker. */
function declareBootSubphase(phase: string, state: "enter" | "leave", elapsedMs: number): void {
  declarePhase(phase, state, elapsedMs);
}

(globalThis as { semioDeclareBootSubphase?: typeof declareBootSubphase }).semioDeclareBootSubphase = declareBootSubphase;

//#region 🚪️HostIo
/** 🚪️ The shell's file door, bridged to the isolate that owns a `document`.
 *
 * 🐛️ This Worker owns the `OffscreenCanvas` and therefore the whole wgpu shell — but it has no `window`
 * and no `document`, so the shell's own `<a download>`/`<input type="file">` halves answered
 * `web_sys::window() == None` and returned silently. `Export Document…` produced real bytes and handed
 * them to nobody; `Import Document…` opened nothing at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-end-to-end-verification-2026-09-14.md` §C). The page half is `🚪️host-io/🟦️.ts`, installed by
 * `🚀️browser-boot/🟦️.ts` on the transport; the shell calls the SAME `semioWgpuHostIo` global in both
 * isolates and never learns which one it is in.
 *
 * 📤️ The download's bytes are TRANSFERRED, never copied: an export is the whole document. */
const pendingHostIo = new Map<number, { readonly resolve: (json: string) => void; readonly reject: (error: Error) => void }>();
let hostIoSeq = 0;

function requestHostIo(requestJson: string, bytes: Uint8Array | null): Promise<string> {
  if (closed || closing || failed) return Promise.reject(new Error("wgpu-host-io: the frame Worker is closing"));
  hostIoSeq += 1;
  const requestId = hostIoSeq;
  return new Promise<string>((resolve, reject) => {
    pendingHostIo.set(requestId, { resolve, reject });
    try {
      const carried = bytes ? new Uint8Array(bytes) : null;
      scope.postMessage({ kind: "host-io", lifecycle, requestId, request: requestJson, bytes: carried }, carried ? [carried.buffer] : []);
    } catch (error) {
      pendingHostIo.delete(requestId);
      reject(error instanceof Error ? error : new Error(String(error)));
    }
  });
}

function settleHostIo(message: Extract<BrowserFrameUiMessage, { readonly kind: "host-io-result" }>): void {
  const pending = pendingHostIo.get(message.requestId);
  if (!pending) return;
  pendingHostIo.delete(message.requestId);
  if (message.json === null) pending.reject(new Error(message.detail ?? "wgpu-host-io refused"));
  else pending.resolve(message.json);
}

(globalThis as { semioWgpuHostIo?: typeof requestHostIo }).semioWgpuHostIo = requestHostIo;
//#endregion 🚪️HostIo

//#region 🧩️LazyPluginInstall
/** 🧩️ The door the browser wgpu shell's `install_plugin` calls (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 * `install_js_plugin`). It mounts through the SAME `loadPluginModule`/`pluginHandleForBridge` pair
 * {@link mountPluginHandles} uses, under the same `monitoredSuspension` watchdog declaration, so a
 * lazily installed plugin is indistinguishable from an eagerly mounted one once it lands. The module
 * url comes from the catalog rather than the boot plan on purpose: the whole point is reaching a
 * plugin the boot plan did NOT name. */
const lazyPluginInstalls = createLazyPluginInstallDoor({
  moduleUrl: (pluginId) => (PLUGIN_CATALOG.plugins.some((row) => row.pluginId === pluginId) ? PLUGIN_CATALOG.moduleUrl(pluginId) : PLUGIN_CATALOG.extensions.some((row) => row.pluginId === pluginId) ? PLUGIN_CATALOG.extensionModuleUrl(pluginId) : undefined),
  mount: async (pluginId, moduleUrl) => {
    if (closed || closing || failed) throw new Error("plugin-install.closing: the frame Worker is closing");
    const module = await monitoredSuspension(`plugin-install:${pluginId}`, () => loadPluginModule(pluginId, moduleUrl), suspensionLedger);
    const handle = pluginHandleForBridge(module);
    void primeContributionManifest(pluginId, moduleUrl).catch(() => {});
    return handle;
  },
  progress: (pluginId, phase) => {
    if (diagnosticsStamp === true) console.debug(`[DEBUG] wgpu lazy plugin install ${pluginId} ${phase}`);
  },
});

(globalThis as { semioWgpuInstallPlugin?: (pluginId: string) => Promise<unknown> }).semioWgpuInstallPlugin = (pluginId) => lazyPluginInstalls.install(pluginId);
(globalThis as { semioWgpuCancelPluginInstall?: (pluginId: string) => boolean }).semioWgpuCancelPluginInstall = (pluginId) => lazyPluginInstalls.cancel(pluginId);
//#endregion 🧩️LazyPluginInstall

function post(message: BrowserFrameWorkerMessage, transfer: Transferable[] = []): void {
  scope.postMessage(message, transfer);
}

function fault(code: string, detail: string): void {
  requestFault(code, detail);
}

function requestFault(code: string, detail: string): void {
  if (closed || pendingFault) return;
  pendingFault = { code, detail };
  failed = true;
  beginClose();
}
//#endregion 🧵️Worker

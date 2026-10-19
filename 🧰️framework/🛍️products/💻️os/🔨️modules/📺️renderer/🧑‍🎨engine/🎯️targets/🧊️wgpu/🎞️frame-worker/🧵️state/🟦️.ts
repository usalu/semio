import type { PluginCatalog } from "@semio-tech/framework";
import { FRAME_WORKER_BOOT_LIVENESS_POLICY } from "../../🫀️boot-liveness/🟦️.ts";
import { setTurnDiagnostics, stampedTurnDiagnostics, TurnClock, TurnLedger, type TurnOutcome, WORKER_STEP_BUDGET_MS } from "../../⏱️turn-budget/🟦️.ts";
import type { BrowserFrameUiMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";
import type { BrowserRendererWorkerHandle, RendererBindings } from "../🧬️bindings-types/🟦️.ts";
import { InteractiveWorkerScheduler } from "../../📇️interactive-job-registry/🟦️.ts";
import { loadPluginModule } from "../../🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts";
import { BrowserMediaRegistry } from "../../../../🎬️media/🌐️browser/🟦️.ts";
import { FrameTurnScheduler, WorkerTurnTaskQueue } from "../../🧵️frame-turn-scheduler/🟦️.ts";
import type { WgpuPluginModule } from "../../🧩️plugin-modules/🟦️.ts";
import { BrowserAssetCancellationCursor } from "../🧩️asset-cancellation/🟦️.ts";
import { OriginalBrowserFrameSource } from "../🎟️original/🟦️.ts";

//#region ⏱️StepAuthority
/** ⏱️ This Worker's step law, declared once for both browser isolates in
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
export const BOOT_HEARTBEAT_MS = 2;

/** 💓️ How often a still-running browser-owned suspension re-posts liveness to the UI isolate, from
 * the shared law in `../🫀️boot-liveness/🟦️.ts`. Posts `boot-liveness`, NOT `boot-progress`: the latter runs
 * the UI isolate's `progress-hook`, whose DOM work overran `FRAME_UI_TURN_BUDGET_MS` (2 ms) once it fired
 * every second — no longer fatal there (`../⏱️turn-budget/🟦️.ts` records and degrades instead of failing),
 * but still pointless work to manufacture.
 *
 * 🩸️ This ticker is NOT the boot's proof of life and never was: it is a `setInterval` on this Worker's own
 * event loop, so it cannot fire while that loop is blocked — which is exactly what compiling the 76 MB
 * renderer, `semioWgpuWorkerBootstrap`'s synchronous prologue and every Rust bootstrap phase do. The proof
 * of life for those is the `boot-phase` DECLARATION {@link monitoredSuspension} posts before it blocks. */
export const BOOT_LIVENESS_INTERVAL_MS = FRAME_WORKER_BOOT_LIVENESS_POLICY.livenessIntervalMs;

export let lastProgressValue = 0;

/** 🧭️ Whether phase declarations are still meaningful. They exist for ONE reader — the UI isolate's
 * boot watchdog — which stops listening the moment it sees `booted`. `monitoredSuspension` is also the
 * asset pump's suspension wrapper (`asset-fetch`, `asset-stream-read`, once per 16 KiB page), so leaving
 * declarations open past boot would post two ignored messages per page across the whole session's hot
 * asset path. Closed exactly where the watchdog stops caring. */
export let bootDeclarationsOpen = true;

/** 🧮️ Fixed boot credit taken from the generated catalog itself. A boot plan is by construction a
 * subset of the catalog's own plugin and extension rows, so no legitimate plan can exceed it, and unlike a
 * magic number it cannot go stale as the product grows — a hardcoded 32 rejected the `s` plan's 57 rows
 * outright and made every wgpu boot impossible. */
export const ASSET_RESPONSE_BYTE_CAPACITY = 16 * 1024 * 1024;

export const ASSET_RESPONSE_PAGE_BYTES = 16 * 1024;

/** 🔁️ How many macrotasks one response seal may wait for the renderer to free its interaction
 * state. Bounded on purpose: a seal that never lands is a renderer defect, not something to spin on. */
export const ASSET_SEAL_ATTEMPTS = 64;

/** 🔬️ Introspection walks the whole retained tree, so it earns a wider turn than a frame step —
 * and a breach is reported on the answer instead of faulting the shell, because a diagnostic must never
 * be the thing that takes the surface down. */
export const INTROSPECTION_STEP_BUDGET_MS = 64;

/** 🧱️ Ceiling for the boot stages whose blocking time is spent inside the browser itself — the module
 * loader, the WebAssembly compiler, and the GPU driver. `WORKER_STEP_BUDGET_MS` prices turns *this* Worker
 * owns and can slice; a `WebAssembly.instantiate` of the renderer module or a `requestDevice` is neither
 * ours nor sliceable (measured: a COLD instantiate of the multi-MB renderer 1093 ms, the font-atlas
 * `renderer-bootstrap` phase 12002 ms, on a machine at load ~36 across 10 cores), so charging it to the
 * interactive ceiling would latch the boot into degraded cadence for work no chunking can shorten. It is
 * therefore a SEPARATE ledger, not a separate verdict: since this lane no budget in this file can end a
 * boot, and the UI isolate's `FRAME_WORKER_BOOT_STALL_TIMEOUT_MS` — which fires on SILENCE, kept honest by
 * `monitoredSuspension`'s liveness heartbeat — is the one remaining outer bound on a wedged instantiate. */
const BROWSER_OWNED_SUSPENSION_BUDGET_MS = 30_000;

export const stepClock = new TurnClock(() => performance.now());

/** 📒️ Prices the turns this Worker OWNS and can slice, against {@link WORKER_STEP_BUDGET_MS}. */
export const stepLedger = new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step");

/** 🧱️ Prices the stages whose blocking time belongs to the browser itself — the module loader,
 * the WebAssembly compiler, the GPU driver. None of it is ours to slice, so it is measured against the
 * wedge ceiling instead of the interactive one and never degrades the boot's cadence. */
export const suspensionLedger = new TurnLedger(BROWSER_OWNED_SUSPENSION_BUDGET_MS, "browser-owned-suspension");

/** 🧾️ The verdict the most recently CLOSED owned step earned, so a caller that must report its own
 * step's price — the frame reply — reads the executing measurement instead of taking a second wall sample. */
export let lastStepOutcome: TurnOutcome | undefined;

//#endregion ⏱️StepAuthority

/** 🧵️ The members of a dedicated worker's global scope this worker reads and assigns (the program is not compiled with the `webworker` library, so the scope is declared where it is used). */
type DedicatedWorkerScope = {
  readonly location: Location;
  onmessage: ((event: MessageEvent<BrowserFrameUiMessage>) => void) | null;
  onerror: ((event: Event | string) => boolean | void) | null;
  onunhandledrejection: ((event: PromiseRejectionEvent) => void) | null;
  close(): void;
  postMessage(message: unknown, transfer?: Transferable[]): void;
};

export const scope = self as unknown as DedicatedWorkerScope;

/** 🩺️ The diagnostics preference the UI isolate stamped on THIS worker's url — the only channel
 * that exists before the boot message, and the one the renderer wasm's own per-frame `[TRACE]` dumps are
 * armed from. A Worker owns no storage, so the page resolves `SEMIO_RUNTIME_DIAGNOSTICS` and stamps;
 * `undefined` (no stamp) leaves the build-time switch in charge. */
export const diagnosticsStamp = stampedTurnDiagnostics(scope.location.search);

setTurnDiagnostics(diagnosticsStamp);

/** 🧭️ A wasm panic spends V8's default ten stack frames entirely inside the panic machinery
 * (`capacity_overflow` → `panic_fmt` → `rust_begin_unwind` → the hook), so the Rust frame that
 * actually asked for the allocation is always the one cut off. The renderer's panic hook mints an
 * `Error` to capture that stack (`🌐️browser-worker/🦀️.rs` `install_worker_panic_trace`), and this is
 * what makes it reach past the machinery. */
Error.stackTraceLimit = 64;

export let lifecycle = 0;

export let runtime: BrowserRendererWorkerHandle | undefined;

export let bindings: RendererBindings | undefined;

export let interactiveJobs: InteractiveWorkerScheduler | undefined;

export const typedMediaPlugins = new Map<string, Awaited<ReturnType<typeof loadPluginModule>>>();

export const mediaRegistry = new BrowserMediaRegistry((pluginId) => typedMediaPlugins.get(pluginId));

export let mediaRetirement: Promise<void> | undefined;

export let frameTurns: FrameTurnScheduler | undefined;

export let frameTurnTasks: WorkerTurnTaskQueue | undefined;

export let frameInput: { readonly timestampMs: number; readonly generation: number } | undefined;

export let frameSequence = 0;

export let closed = false;

export let closing = false;

export let failed = false;

export let quarantined: { readonly code: string; readonly detail: string } | undefined;

export let lastFrame = { cursor: "default", fullscreen: null as boolean | null };

export let lastWorld3dAcceptedFrameDiagnostic: string | undefined;

export let pendingFault: { readonly code: string; readonly detail: string;readonly originalFailure?:{readonly format:"pack";readonly bytes:Uint8Array} } | undefined;

export let runtimeCloseComplete = false;

export let jobsCloseComplete = false;

export let closeOwner: "runtime" | "jobs" = "runtime";

export let assetPumping = false;

export let assetAbort: AbortController | undefined;

export const bootAbort = new AbortController();

export let explicitPluginModules: readonly WgpuPluginModule[] | undefined;

export let workerCatalog: PluginCatalog | undefined;

export const consumedTopics = (id: string) => [...(workerCatalog?.plugins ?? []), ...(workerCatalog?.extensions ?? [])].find(row => row.pluginId === id)?.consumes ?? [];

export const assetCancellation = new BrowserAssetCancellationCursor();

export let nextImageDecodeRequestId = 1;

export let pageImageDecode: { readonly requestId: number; readonly resolve: (bitmap: ImageBitmap) => void; readonly reject: (error: Error) => void } | undefined;

export let originalFrameSource:OriginalBrowserFrameSource|undefined;

/** 🧭️ DECLARES that this Worker is about to block on one browser-owned phase — or that it has left
 * it. Posted while the event loop still runs, which is the whole point: a `postMessage` issued a tick
 * before a multi-second `WebAssembly.compile` reaches the UI isolate, a `boot-liveness` issued DURING it
 * never does. The UI-isolate watchdog measures a declared phase against its own ceiling
 * (`../🫀️boot-liveness/🟦️.ts`) instead of against silence, so it can no longer terminate a Worker that is
 * merely busy, and `elapsedMs` on the withdrawal is the per-phase cost the next boot is measured by. */
export const bootPhaseStack: string[] = [];

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
export const pendingHostIo = new Map<number, { readonly resolve: (json: string) => void; readonly reject: (error: Error) => void }>();

export let hostIoSeq = 0;

/** 🧷 Setter for the module-level `lastStepOutcome`: importers read the live binding and write through this call. */
export function setLastStepOutcome(value: typeof lastStepOutcome): typeof lastStepOutcome {
  lastStepOutcome = value;
  return value;
}

/** 🧷 Setter for the module-level `lastProgressValue`: importers read the live binding and write through this call. */
export function setLastProgressValue(value: typeof lastProgressValue): typeof lastProgressValue {
  lastProgressValue = value;
  return value;
}

/** 🧷 Setter for the module-level `nextImageDecodeRequestId`: importers read the live binding and write through this call. */
export function setNextImageDecodeRequestId(value: typeof nextImageDecodeRequestId): typeof nextImageDecodeRequestId {
  nextImageDecodeRequestId = value;
  return value;
}

/** 🧷 Setter for the module-level `pageImageDecode`: importers read the live binding and write through this call. */
export function setPageImageDecode(value: typeof pageImageDecode): typeof pageImageDecode {
  pageImageDecode = value;
  return value;
}

/** 🧷 Setter for the module-level `assetPumping`: importers read the live binding and write through this call. */
export function setAssetPumping(value: typeof assetPumping): typeof assetPumping {
  assetPumping = value;
  return value;
}

/** 🧷 Setter for the module-level `assetAbort`: importers read the live binding and write through this call. */
export function setAssetAbort(value: typeof assetAbort): typeof assetAbort {
  assetAbort = value;
  return value;
}

/** 🧷 Setter for the module-level `hostIoSeq`: importers read the live binding and write through this call. */
export function setHostIoSeq(value: typeof hostIoSeq): typeof hostIoSeq {
  hostIoSeq = value;
  return value;
}

/** 🧷 Setter for the module-level `runtimeCloseComplete`: importers read the live binding and write through this call. */
export function setRuntimeCloseComplete(value: typeof runtimeCloseComplete): typeof runtimeCloseComplete {
  runtimeCloseComplete = value;
  return value;
}

/** 🧷 Setter for the module-level `closeOwner`: importers read the live binding and write through this call. */
export function setCloseOwner(value: typeof closeOwner): typeof closeOwner {
  closeOwner = value;
  return value;
}

/** 🧷 Setter for the module-level `jobsCloseComplete`: importers read the live binding and write through this call. */
export function setJobsCloseComplete(value: typeof jobsCloseComplete): typeof jobsCloseComplete {
  jobsCloseComplete = value;
  return value;
}

/** 🧷 Setter for the module-level `frameTurnTasks`: importers read the live binding and write through this call. */
export function setFrameTurnTasks(value: typeof frameTurnTasks): typeof frameTurnTasks {
  frameTurnTasks = value;
  return value;
}

/** 🧷 Setter for the module-level `closed`: importers read the live binding and write through this call. */
export function setClosed(value: typeof closed): typeof closed {
  closed = value;
  return value;
}

/** 🧷 Setter for the module-level `closing`: importers read the live binding and write through this call. */
export function setClosing(value: typeof closing): typeof closing {
  closing = value;
  return value;
}

/** 🧷 Setter for the module-level `mediaRetirement`: importers read the live binding and write through this call. */
export function setMediaRetirement(value: typeof mediaRetirement): typeof mediaRetirement {
  mediaRetirement = value;
  return value;
}

/** 🧷 Setter for the module-level `failed`: importers read the live binding and write through this call. */
export function setFailed(value: typeof failed): typeof failed {
  failed = value;
  return value;
}

/** 🧷 Setter for the module-level `pendingFault`: importers read the live binding and write through this call. */
export function setPendingFault(value: typeof pendingFault): typeof pendingFault {
  pendingFault = value;
  return value;
}

/** 🧷 Setter for the module-level `frameSequence`: importers read the live binding and write through this call. */
export function setFrameSequence(value: typeof frameSequence): typeof frameSequence {
  frameSequence = value;
  return value;
}

/** 🧷 Setter for the module-level `lastFrame`: importers read the live binding and write through this call. */
export function setLastFrame(value: typeof lastFrame): typeof lastFrame {
  lastFrame = value;
  return value;
}

/** 🧷 Setter for the module-level `quarantined`: importers read the live binding and write through this call. */
export function setQuarantined(value: typeof quarantined): typeof quarantined {
  quarantined = value;
  return value;
}

/** 🧷 Setter for the module-level `lastWorld3dAcceptedFrameDiagnostic`: importers read the live binding and write through this call. */
export function setLastWorld3dAcceptedFrameDiagnostic(value: typeof lastWorld3dAcceptedFrameDiagnostic): typeof lastWorld3dAcceptedFrameDiagnostic {
  lastWorld3dAcceptedFrameDiagnostic = value;
  return value;
}

/** 🧷 Setter for the module-level `lifecycle`: importers read the live binding and write through this call. */
export function setLifecycle(value: typeof lifecycle): typeof lifecycle {
  lifecycle = value;
  return value;
}

/** 🧷 Setter for the module-level `bindings`: importers read the live binding and write through this call. */
export function setBindings(value: typeof bindings): typeof bindings {
  bindings = value;
  return value;
}

/** 🧷 Setter for the module-level `workerCatalog`: importers read the live binding and write through this call. */
export function setWorkerCatalog(value: typeof workerCatalog): typeof workerCatalog {
  workerCatalog = value;
  return value;
}

/** 🧷 Setter for the module-level `explicitPluginModules`: importers read the live binding and write through this call. */
export function setExplicitPluginModules(value: typeof explicitPluginModules): typeof explicitPluginModules {
  explicitPluginModules = value;
  return value;
}

/** 🧷 Setter for the module-level `runtime`: importers read the live binding and write through this call. */
export function setRuntime(value: typeof runtime): typeof runtime {
  runtime = value;
  return value;
}

/** 🧷 Setter for the module-level `originalFrameSource`: importers read the live binding and write through this call. */
export function setOriginalFrameSource(value: typeof originalFrameSource): typeof originalFrameSource {
  originalFrameSource = value;
  return value;
}

/** 🧷 Setter for the module-level `frameTurns`: importers read the live binding and write through this call. */
export function setFrameTurns(value: typeof frameTurns): typeof frameTurns {
  frameTurns = value;
  return value;
}

/** 🧷 Setter for the module-level `frameInput`: importers read the live binding and write through this call. */
export function setFrameInput(value: typeof frameInput): typeof frameInput {
  frameInput = value;
  return value;
}

/** 🧷 Setter for the module-level `interactiveJobs`: importers read the live binding and write through this call. */
export function setInteractiveJobs(value: typeof interactiveJobs): typeof interactiveJobs {
  interactiveJobs = value;
  return value;
}

/** 🧷 Setter for the module-level `bootDeclarationsOpen`: importers read the live binding and write through this call. */
export function setBootDeclarationsOpen(value: typeof bootDeclarationsOpen): typeof bootDeclarationsOpen {
  bootDeclarationsOpen = value;
  return value;
}

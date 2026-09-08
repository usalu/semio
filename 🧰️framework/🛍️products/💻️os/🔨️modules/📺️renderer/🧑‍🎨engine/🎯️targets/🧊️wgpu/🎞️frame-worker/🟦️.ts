/// <reference lib="webworker" />

import { pluginGraphErrorMessage, resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../../../../🔌️plugin/📇️registry/🟦️.ts";
import type { BrowserFrameUiMessage, BrowserFrameWorkerMessage } from "../🚚️browser-frame-transport/🟦️.ts";
import { INTERACTIVE_WORKER_DESCRIPTORS, InteractiveWorkerScheduler } from "../📇️interactive-job-registry/🟦️.ts";
import { loadPluginModule, pluginHandleForBridge } from "../📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts";
import { meshAssetTransportUrl } from "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🟦️.ts";

//#region 🔖️Bindings
type BrowserRendererWorkerHandle = {
  enqueueBatch(eventsJson: string, generation: number): void;
  tick(timestampMs: number, sequence: number, generation: number): string;
  pollAssetRequest(): string;
  reserveAssetResponse(byteCredits: number): void;
  pushAssetResponsePage(bytes: Uint8Array): void;
  sealAssetResponse(): void;
  abortAssetResponse(): void;
  closeStep(): boolean;
};

type BrowserRendererBootstrapHandle = {
  step(): string;
  bootShell(): Promise<BrowserRendererBootstrapHandle>;
  finish(): BrowserRendererWorkerHandle;
};

type BrowserRendererBootStep = { readonly stage: string; readonly progress: number; readonly shellBoot: boolean; readonly complete: boolean };

type RendererBindings = {
  default?: (moduleOrPath?: WebAssembly.Module | RequestInfo | URL) => Promise<unknown>;
  dumpStructure?: () => string;
  dumpFrameStats?: () => string;
  semioWgpuSetAppRole?: (role: string) => void;
  semioWgpuSetHubEnv?: (hubUrl: string, user: string, dataDir: string) => void;
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
const WORKER_STEP_BUDGET_MS = 8;
const BOOT_HEARTBEAT_MS = 2;
/** @emoji 💓️ How often a still-running browser-owned suspension re-posts its stage to the UI isolate.
 * `FRAME_WORKER_BOOT_TIMEOUT_MS` fires on SILENCE, not on slowness, and a cold `shell-boot` of a 5.4 MB-manifest
 * plugin legitimately outlasts it while yielding normally — so liveness is reported rather than the watchdog
 * loosened, and a genuinely wedged worker still trips it. */
const BOOT_LIVENESS_INTERVAL_MS = 1_000;
let lastProgressValue = 0;
/** @emoji 🧮️ Fixed boot credit taken from the generated catalog itself. A boot plan is by construction a
 * subset of the catalog's own plugin and extension rows, so no legitimate plan can exceed it, and unlike a
 * magic number it cannot go stale as the product grows — a hardcoded 32 rejected the `s` plan's 57 rows
 * outright and made every wgpu boot impossible. */
const PLUGIN_BOOT_CAPACITY = PLUGIN_CATALOG.plugins.length + PLUGIN_CATALOG.extensions.length;
const ASSET_RESPONSE_BYTE_CAPACITY = 16 * 1024 * 1024;
const ASSET_RESPONSE_PAGE_BYTES = 16 * 1024;
/** @emoji 🔬️ Introspection walks the whole retained tree, so it earns a wider turn than a frame step —
 * and a breach is reported on the answer instead of faulting the shell, because a diagnostic must never
 * be the thing that takes the surface down. */
const INTROSPECTION_STEP_BUDGET_MS = 64;
/** @emoji 🧱️ Ceiling for the boot stages whose blocking time is spent inside the browser itself — the module
 * loader, the WebAssembly compiler, and the GPU driver. `WORKER_STEP_BUDGET_MS` bounds turns *this* Worker
 * owns and can slice; a `WebAssembly.instantiate` of the renderer module or a `requestDevice` is neither
 * ours nor sliceable, so measuring it against the frame budget only ever reports the browser's own cost as a
 * Worker fault (measured: instantiating the debug renderer blocks ~62 ms, which failed the 8 ms frame
 * budget and killed every boot). The UI isolate's `FRAME_WORKER_BOOT_TIMEOUT_MS` remains the outer bound. */
const BROWSER_OWNED_SUSPENSION_BUDGET_MS = 1_000;

function ownedStep<T>(stage: string, callback: () => T, budgetMs: number = WORKER_STEP_BUDGET_MS): T {
  const startedAt = performance.now();
  const value = callback();
  const duration = performance.now() - startedAt;
  if (duration >= budgetMs) throw new Error(`worker-boot-step-overrun: ${stage} took ${duration.toFixed(3)} ms against a ${budgetMs} ms budget`);
  return value;
}

async function monitoredSuspension<T>(stage: string, operation: () => Promise<T>, blockBudgetMs: number = WORKER_STEP_BUDGET_MS): Promise<T> {
  let lastBeat = performance.now();
  let lastLivenessAt = performance.now();
  let maximumBlockMs = 0;
  const heartbeat = setInterval(() => {
    const now = performance.now();
    maximumBlockMs = Math.max(maximumBlockMs, now - lastBeat - BOOT_HEARTBEAT_MS);
    lastBeat = now;
    if (now - lastLivenessAt >= BOOT_LIVENESS_INTERVAL_MS) {
      lastLivenessAt = now;
      progress(stage, lastProgressValue);
    }
  }, BOOT_HEARTBEAT_MS);
  try {
    const result = await ownedStep(`${stage}:start`, operation, blockBudgetMs);
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
    if (closed || closing) throw new Error(`worker-boot-cancelled: ${stage}`);
    if (maximumBlockMs >= blockBudgetMs) throw new Error(`worker-boot-step-overrun: ${stage} blocked the Worker for ${maximumBlockMs.toFixed(3)} ms against a ${blockBudgetMs} ms budget`);
    return result;
  } finally {
    clearInterval(heartbeat);
  }
}

async function macrotask(): Promise<void> {
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
  if (closed || closing) throw new Error("worker-boot-cancelled");
}

//#endregion ⏱️StepAuthority

//#region 🧵️Worker
const scope = self as DedicatedWorkerGlobalScope;
let lifecycle = 0;
let runtime: BrowserRendererWorkerHandle | undefined;
let bindings: RendererBindings | undefined;
let interactiveJobs: InteractiveWorkerScheduler | undefined;
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
  if (closed || closing || failed || quarantined) return;
  if (message.kind === "job-submit" || message.kind === "job-input-page" || message.kind === "job-cancel") {
    if (!interactiveJobs) {
      fault("interactive-job-not-ready", "interactive job arrived before Worker boot completed");
      return;
    }
    const startedAt = performance.now();
    interactiveJobs.receive(message);
    const duration = performance.now() - startedAt;
    if (duration >= WORKER_STEP_BUDGET_MS) fault("interactive-job-overrun", `interactive job admission turn took ${duration.toFixed(3)} ms`);
    return;
  }
  if (!runtime) {
    fault("worker-not-booted", "frame batch arrived before renderer boot completed");
    return;
  }
  const startedAt = performance.now();
  try {
    runtime.enqueueBatch(JSON.stringify({ replaceable: message.replaceable, lossless: message.lossless }), message.generation);
    const result = JSON.parse(runtime.tick(message.timestampMs, message.sequence, message.generation)) as Omit<Extract<BrowserFrameWorkerMessage, { kind: "frame" }>, "kind" | "lifecycle" | "sequence" | "generation" | "workerDurationMs">;
    const duration = performance.now() - startedAt;
    lastFrame = { cursor: result.cursor, fullscreen: result.fullscreen };
    if (result.quarantined || duration >= WORKER_STEP_BUDGET_MS) quarantined = { code: result.faultCode ?? "worker-step-overrun", detail: result.faultDetail ?? `frame step took ${duration.toFixed(3)} ms` };
    post({ kind: "frame", lifecycle, sequence: message.sequence, generation: message.generation, cursor: result.cursor, fullscreen: result.fullscreen, requestFrame: result.requestFrame, progress: result.progress, workerDurationMs: duration, quarantined: quarantined !== undefined, faultCode: quarantined?.code, faultDetail: quarantined?.detail });
    if (quarantined) requestFault(quarantined.code, quarantined.detail);
    else scheduleAssetPump();
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
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
  const hook = message.probe === "structure" ? bindings.dumpStructure : bindings.dumpFrameStats;
  if (!hook) {
    respond(null, `renderer bindings expose no ${message.probe} introspection export`);
    return;
  }
  const startedAt = performance.now();
  try {
    const json = hook();
    const duration = performance.now() - startedAt;
    respond(json, duration >= INTROSPECTION_STEP_BUDGET_MS ? `${message.probe} introspection took ${duration.toFixed(3)} ms` : undefined);
  } catch (error) {
    respond(null, error instanceof Error ? error.message : String(error));
  }
}

async function closeRuntime(): Promise<void> {
  for (;;) {
    const startedAt = performance.now();
    if (closeOwner === "runtime" && !runtimeCloseComplete) {
      runtimeCloseComplete = runtime ? runtime.closeStep() : true;
      closeOwner = "jobs";
    } else if (!jobsCloseComplete) {
      jobsCloseComplete = interactiveJobs ? interactiveJobs.closeStep() : true;
      closeOwner = "runtime";
    } else if (!runtimeCloseComplete) {
      closeOwner = "runtime";
    }
    if (performance.now() - startedAt >= WORKER_STEP_BUDGET_MS) {
      pendingFault ??= { code: "worker-close-overrun", detail: "Worker close turn exceeded the Worker budget" };
    }
    if (runtimeCloseComplete && jobsCloseComplete) break;
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  }
  if (pendingFault) post({ kind: "fault", lifecycle, code: pendingFault.code, detail: pendingFault.detail });
  post({ kind: "closed", lifecycle });
  closed = true;
  scope.close();
}

function beginClose(): void {
  if (closed || closing) return;
  closing = true;
  failed = pendingFault !== undefined;
  runtimeCloseComplete = runtime === undefined;
  jobsCloseComplete = interactiveJobs === undefined;
  assetAbort?.abort();
  assetAbort = undefined;
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
      const module = await monitoredSuspension(`plugin:${target.pluginId}`, () => loadPluginModule(target.pluginId, target.moduleUrl), BROWSER_OWNED_SUSPENSION_BUDGET_MS);
      mounted.push(ownedStep(`plugin-handle:${target.pluginId}`, () => ({ pluginId: target.pluginId, handle: pluginHandleForBridge(module) })));
    } catch (error) {
      if (closed || closing) throw error;
      progress(`plugin-fault:${target.pluginId}: ${error instanceof Error ? error.message : String(error)}`, share);
    }
  }
  return mounted;
}

async function boot(message: Extract<BrowserFrameUiMessage, { kind: "boot" }>): Promise<void> {
  if (runtime || lifecycle !== 0) {
    fault("duplicate-boot", "the frame Worker accepts exactly one boot lifecycle");
    return;
  }
  lifecycle = message.lifecycle;
  try {
    progress("renderer-module", 0.05);
    const loaded = await monitoredSuspension("renderer-module", () => import(/* @vite-ignore */ message.bindingsModuleUrl) as Promise<RendererBindings>, BROWSER_OWNED_SUSPENSION_BUDGET_MS);
    bindings = loaded;
    if (loaded.default) {
      progress("wasm-instance", 0.15);
      await monitoredSuspension("wasm-instance", () => loaded.default!(message.bindingsWasmUrl), BROWSER_OWNED_SUSPENSION_BUDGET_MS);
    }
    if (!loaded.semioWgpuWorkerBootstrap) throw new Error("renderer bindings missing semioWgpuWorkerBootstrap");
    ownedStep("runtime-environment", () => {
      loaded.semioWgpuSetAppRole?.(message.appRole);
      if (message.hub) loaded.semioWgpuSetHubEnv?.(message.hub.hubUrl, message.hub.user, message.hub.dataDir);
    }, BROWSER_OWNED_SUSPENSION_BUDGET_MS);
    progress("plugin-graph", 0.25);
    const bootPlan = ownedStep("plugin-graph", () => resolvePlaygroundBoot(PLUGIN_CATALOG, message.pluginVariant));
    if (bootPlan.plugins.length > PLUGIN_BOOT_CAPACITY) throw new Error(`plugin-credits: boot plan exceeds ${PLUGIN_BOOT_CAPACITY} plugins`);
    for (const error of bootPlan.dependencyErrors) progress(pluginGraphErrorMessage(error, message.locale), 0.3);
    const plugins = await mountPluginHandles(bootPlan.plugins);
    if (plugins.length === 0) throw new Error(`no wasm plugin modules found for variant ${message.pluginVariant}`);
    progress("renderer-runtime", 0.65);
    let bootstrap = await monitoredSuspension("gpu-platform", () => loaded.semioWgpuWorkerBootstrap!(message.canvas, plugins, bootPlan.variant, message.width, message.height, message.dpr, () => post({ kind: "wake", lifecycle })), BROWSER_OWNED_SUSPENSION_BUDGET_MS);
    while (true) {
      await macrotask();
      const bootstrapStartedAt = performance.now();
      const step = ownedStep("renderer-bootstrap", () => JSON.parse(bootstrap.step()) as BrowserRendererBootStep, BROWSER_OWNED_SUSPENSION_BUDGET_MS);
      console.log(`[DEBUG] renderer-bootstrap stage=${step.stage} took ${(performance.now() - bootstrapStartedAt).toFixed(1)}ms`);
      progress(step.stage, 0.65 + step.progress * 0.3);
      if (step.shellBoot) {
        bootstrap = await monitoredSuspension("shell-boot", () => bootstrap.bootShell(), BROWSER_OWNED_SUSPENSION_BUDGET_MS);
        continue;
      }
      if (step.complete) break;
    }
    runtime = ownedStep("renderer-finish", () => bootstrap.finish(), BROWSER_OWNED_SUSPENSION_BUDGET_MS);
    interactiveJobs = ownedStep("interactive-job-registry", () => new InteractiveWorkerScheduler(lifecycle, INTERACTIVE_WORKER_DESCRIPTORS, post, (callback) => setTimeout(callback, 0), () => performance.now(), (detail) => fault("interactive-job-fault", detail)), BROWSER_OWNED_SUSPENSION_BUDGET_MS);
    progress("ready", 1);
    post({ kind: "booted", lifecycle });
    scheduleAssetPump();
  } catch (error) {
    fault("worker-boot-failed", error instanceof Error ? error.message : String(error));
  }
}

type AssetRequest = {
  readonly available: boolean;
  readonly url?: string;
  readonly responseByteCapacity?: number;
  readonly pageByteCapacity?: number;
};

function scheduleAssetPump(): void {
  if (assetPumping || !runtime || closed || closing || failed || quarantined) return;
  assetPumping = true;
  setTimeout(() => void pumpAsset(), 0);
}

async function pumpAsset(): Promise<void> {
  try {
    if (!runtime || closed || closing || failed || quarantined) return;
    const request = ownedStep("asset-request", () => JSON.parse(runtime!.pollAssetRequest()) as AssetRequest);
    if (!request.available) return;
    if (!request.url || request.responseByteCapacity !== ASSET_RESPONSE_BYTE_CAPACITY || request.pageByteCapacity !== ASSET_RESPONSE_PAGE_BYTES) {
      throw new Error("asset-request-protocol: request descriptor did not match fixed Worker credits");
    }
    assetAbort = new AbortController();
    const response = await monitoredSuspension("asset-fetch", () => fetch(meshAssetTransportUrl(request.url!), { signal: assetAbort!.signal }));
    if (!response.ok || !response.body) throw new Error(`asset-fetch-status: ${response.status}`);
    const declaredHeader = ownedStep("asset-response-headers", () => response.headers.get("content-length"));
    const declared = declaredHeader === null ? undefined : Number(declaredHeader);
    if (declared !== undefined && (!Number.isSafeInteger(declared) || declared < 0 || declared > ASSET_RESPONSE_BYTE_CAPACITY)) throw new Error("asset-response-length: Content-Length exceeded fixed aggregate credits");
    ownedStep("asset-response-reserve", () => runtime!.reserveAssetResponse(declared ?? ASSET_RESPONSE_BYTE_CAPACITY));
    const reader = ownedStep("asset-stream-reader", () => response.body!.getReader({ mode: "byob" }) as ReadableStreamBYOBReader);
    let received = 0;
    for (;;) {
      const pageOwner = ownedStep("asset-page-owner", () => new Uint8Array(ASSET_RESPONSE_PAGE_BYTES));
      const chunk = await monitoredSuspension("asset-stream-read", () => reader.read(pageOwner));
      if (chunk.done) break;
      const bytes = chunk.value;
      if (bytes.byteLength === 0 || bytes.byteLength > ASSET_RESPONSE_PAGE_BYTES) throw new Error("asset-response-page: stream violated fixed BYOB page credits");
      received += bytes.byteLength;
      if (received > (declared ?? ASSET_RESPONSE_BYTE_CAPACITY)) throw new Error("asset-response-overflow: stream exceeded admitted bytes");
      ownedStep("asset-page", () => runtime!.pushAssetResponsePage(bytes));
      await macrotask();
    }
    ownedStep("asset-stream-release", () => reader.releaseLock());
    if (declared !== undefined && received !== declared) throw new Error("asset-response-short-read: stream ended before declared bytes");
    ownedStep("asset-seal", () => runtime!.sealAssetResponse());
    post({ kind: "wake", lifecycle });
  } catch (error) {
    if (runtime) {
      try {
        ownedStep("asset-abort", () => runtime!.abortAssetResponse());
      } catch {}
    }
    if (!closing && !closed) fault("asset-stream-fault", error instanceof Error ? error.message : String(error));
  } finally {
    assetAbort = undefined;
    assetPumping = false;
  }
}

function progress(stage: string, value: number): void {
  lastProgressValue = value;
  if (!closed && !closing && !failed) post({ kind: "boot-progress", lifecycle, stage, progress: value });
}

function post(message: BrowserFrameWorkerMessage): void {
  scope.postMessage(message);
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

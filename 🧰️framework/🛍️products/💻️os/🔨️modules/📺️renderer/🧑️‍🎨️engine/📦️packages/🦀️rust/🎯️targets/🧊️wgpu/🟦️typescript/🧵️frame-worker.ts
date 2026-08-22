/// <reference lib="webworker" />

import { pluginGraphErrorMessage, resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../../../../../../🔌️plugin/📇️registry/🟦️catalog.ts";
import type { BrowserFrameUiMessage, BrowserFrameWorkerMessage } from "./🧵️browser-frame-transport.ts";
import { INTERACTIVE_WORKER_DESCRIPTORS, InteractiveWorkerScheduler } from "./🧵️interactive-job-registry.ts";
import { loadPluginModule, pluginHandleForBridge } from "./🐚️plugin-bridge.ts";

//#region 🔖️Bindings
type BrowserRendererWorkerHandle = {
  enqueueBatch(eventsJson: string, generation: number): void;
  tick(timestampMs: number, sequence: number, generation: number): string;
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
const PLUGIN_BOOT_CAPACITY = 32;
const PLUGIN_MANIFEST_CODE_UNIT_CAPACITY = 64 * 1024;

function ownedStep<T>(stage: string, callback: () => T): T {
  const startedAt = performance.now();
  const value = callback();
  const duration = performance.now() - startedAt;
  if (duration >= WORKER_STEP_BUDGET_MS) throw new Error(`worker-boot-step-overrun: ${stage} took ${duration.toFixed(3)} ms`);
  return value;
}

async function monitoredSuspension<T>(stage: string, operation: () => Promise<T>): Promise<T> {
  let lastBeat = performance.now();
  let maximumBlockMs = 0;
  const heartbeat = setInterval(() => {
    const now = performance.now();
    maximumBlockMs = Math.max(maximumBlockMs, now - lastBeat - BOOT_HEARTBEAT_MS);
    lastBeat = now;
  }, BOOT_HEARTBEAT_MS);
  try {
    const result = await ownedStep(`${stage}:start`, operation);
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
    if (closed || closing) throw new Error(`worker-boot-cancelled: ${stage}`);
    if (maximumBlockMs >= WORKER_STEP_BUDGET_MS) throw new Error(`worker-boot-step-overrun: ${stage} blocked the Worker for ${maximumBlockMs.toFixed(3)} ms`);
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

scope.onmessage = (event: MessageEvent<BrowserFrameUiMessage>) => void receive(event.data);

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
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
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
  interactiveJobs?.close();
  void closeRuntime();
}

async function boot(message: Extract<BrowserFrameUiMessage, { kind: "boot" }>): Promise<void> {
  if (runtime || lifecycle !== 0) {
    fault("duplicate-boot", "the frame Worker accepts exactly one boot lifecycle");
    return;
  }
  lifecycle = message.lifecycle;
  try {
    progress("renderer-module", 0.05);
    const bindings = await monitoredSuspension("renderer-module", () => import(/* @vite-ignore */ message.bindingsModuleUrl) as Promise<RendererBindings>);
    if (bindings.default) {
      progress("wasm-instance", 0.15);
      await monitoredSuspension("wasm-instance", () => bindings.default!(message.bindingsWasmUrl));
    }
    if (!bindings.semioWgpuWorkerBootstrap) throw new Error("renderer bindings missing semioWgpuWorkerBootstrap");
    ownedStep("runtime-environment", () => {
      bindings.semioWgpuSetAppRole?.(message.appRole);
      if (message.hub) bindings.semioWgpuSetHubEnv?.(message.hub.hubUrl, message.hub.user, message.hub.dataDir);
    });
    progress("plugin-graph", 0.25);
    const bootPlan = ownedStep("plugin-graph", () => resolvePlaygroundBoot(PLUGIN_CATALOG, message.pluginVariant));
    if (bootPlan.plugins.length > PLUGIN_BOOT_CAPACITY) throw new Error(`plugin-credits: boot plan exceeds ${PLUGIN_BOOT_CAPACITY} plugins`);
    for (const error of bootPlan.dependencyErrors) progress(pluginGraphErrorMessage(error, message.locale), 0.3);
    const plugins: { pluginId: string; handle: ReturnType<typeof pluginHandleForBridge> }[] = [];
    for (let index = 0; index < bootPlan.plugins.length; index++) {
      const target = bootPlan.plugins[index]!;
      progress(`plugin:${target.pluginId}`, 0.3 + 0.3 * (index / Math.max(1, bootPlan.plugins.length)));
      await macrotask();
      const module = await monitoredSuspension(`plugin:${target.pluginId}`, () => loadPluginModule(target.pluginId, target.moduleUrl));
      ownedStep(`plugin-manifest:${target.pluginId}`, () => {
        const manifest = JSON.stringify(module.manifest);
        if (manifest.length > PLUGIN_MANIFEST_CODE_UNIT_CAPACITY) throw new Error(`plugin-manifest-credits: ${target.pluginId} exceeds ${PLUGIN_MANIFEST_CODE_UNIT_CAPACITY} code units`);
      });
      plugins.push(ownedStep(`plugin-handle:${target.pluginId}`, () => ({ pluginId: target.pluginId, handle: pluginHandleForBridge(module) })));
    }
    if (plugins.length === 0) throw new Error(`no wasm plugin modules found for variant ${message.pluginVariant}`);
    progress("renderer-runtime", 0.65);
    let bootstrap = await monitoredSuspension("gpu-platform", () => bindings.semioWgpuWorkerBootstrap!(message.canvas, plugins, bootPlan.variant, message.width, message.height, message.dpr, () => post({ kind: "wake", lifecycle })));
    while (true) {
      await macrotask();
      const step = ownedStep("renderer-bootstrap", () => JSON.parse(bootstrap.step()) as BrowserRendererBootStep);
      progress(step.stage, 0.65 + step.progress * 0.3);
      if (step.shellBoot) {
        bootstrap = await monitoredSuspension("shell-boot", () => bootstrap.bootShell());
        continue;
      }
      if (step.complete) break;
    }
    runtime = ownedStep("renderer-finish", () => bootstrap.finish());
    interactiveJobs = ownedStep("interactive-job-registry", () => new InteractiveWorkerScheduler(lifecycle, INTERACTIVE_WORKER_DESCRIPTORS, post, (callback) => setTimeout(callback, 0), () => performance.now(), (detail) => fault("interactive-job-fault", detail)));
    progress("ready", 1);
    post({ kind: "booted", lifecycle });
  } catch (error) {
    fault("worker-boot-failed", error instanceof Error ? error.message : String(error));
  }
}

function progress(stage: string, value: number): void {
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

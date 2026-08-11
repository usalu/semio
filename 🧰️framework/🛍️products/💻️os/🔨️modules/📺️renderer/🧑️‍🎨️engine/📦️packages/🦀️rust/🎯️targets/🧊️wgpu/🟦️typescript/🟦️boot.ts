// #region 🧲️Header
/** @emoji 🧊️ Trunk boot glue — loads wasm programs and starts the wgpu renderer. */
// #endregion 🧲️Header

import { parseInvocationResponse, type PluginRegistryEntry } from "@semio-tech/framework";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../../../../../../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️catalog.ts";
import { PLAYGROUND_SESSION } from "../../../../../../../🧑️‍💻️dev/🤖️generated/🟦️session.ts";

await new Promise<void>((resolve) => {
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  } else {
    resolve();
  }
});

//#region PluginTypes
type PluginModuleHandle = {
  pluginId: string;
  manifest: unknown;
  createApp: (appId: string) => Promise<number>;
  destroyApp: (instanceId: number) => Promise<void>;
  handleAction: (instanceId: number, actionJson: string, viewState: unknown) => Promise<unknown>;
  handleCommand: (instanceId: number, commandJson: string, viewState: unknown) => Promise<unknown>;
  render: (instanceId: number, bodyKey: string, viewState: unknown) => Promise<unknown>;
  renderWithDocument?: (instanceId: number, bodyKey: string, viewState: unknown, documentJson: string) => Promise<unknown>;
  utilities: (instanceId: number, viewState: unknown) => Promise<unknown>;
  windowEngagements: (instanceId: number, viewState: unknown) => Promise<unknown>;
  windowMeasures: (instanceId: number, viewState: unknown) => Promise<unknown>;
};

type PluginWorkerMessageType = "init" | "manifest" | "createApp" | "handleAction" | "handleCommand" | "render" | "destroy" | "utilities" | "windowEngagements" | "windowMeasures" | "error";

/** ⏱️ Only `init`/`manifest` (boot calls, expected to be fast) restart the worker on timeout. */
const PLUGIN_WORKER_BOOT_TIMEOUT_MS = 5000;
/** 🐢️ Every other call (handleAction/handleCommand/render/...) only logs past this point — see `request`. */
const PLUGIN_WORKER_SLOW_CALL_WARN_MS = 2000;
const PLUGIN_WORKER_BOOT_MESSAGE_TYPES: readonly PluginWorkerMessageType[] = ["init", "manifest"];
//#endregion PluginTypes

async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginModuleHandle> {
  return loadPluginModuleViaWorker(pluginId, moduleUrl);
}

function pluginWorkerUrl(moduleUrl: string): string {
  return moduleUrl.replace(/\/[^/]+\.js$/, "/🟨️plugin-worker.js");
}

class PluginWorkerClient {
  private worker: Worker | null = null;
  private readonly pending = new Map<string, { resolve: (value: unknown) => void; reject: (error: Error) => void; timer: number }>();

  constructor(
    private readonly pluginId: string,
    private readonly moduleUrl: string,
  ) {}

  private clearPending(error: Error): void {
    for (const [requestId, entry] of this.pending) {
      window.clearTimeout(entry.timer);
      entry.reject(error);
      this.pending.delete(requestId);
    }
  }

  private terminateWorker(): void {
    if (!this.worker) return;
    this.worker.terminate();
    this.worker = null;
  }

  private attachWorker(worker: Worker): void {
    worker.onmessage = (event: MessageEvent) => {
      const message = event.data as {
        requestId?: string;
        type?: PluginWorkerMessageType;
        message?: string;
        value?: string;
        instanceId?: number;
        ok?: boolean;
      };
      const requestId = message.requestId;
      if (!requestId) return;
      const entry = this.pending.get(requestId);
      if (!entry) return;
      window.clearTimeout(entry.timer);
      this.pending.delete(requestId);
      if (message.type === "error") {
        entry.reject(new Error(message.message ?? `program worker ${this.pluginId} error`));
        return;
      }
      entry.resolve(message);
    };
    worker.onerror = (error) => {
      console.error(`[DEBUG] program worker ${this.pluginId} crashed`, error);
      this.terminateWorker();
      this.clearPending(new Error(`program worker ${this.pluginId} crashed`));
    };
  }

  private async spawnWorker(): Promise<void> {
    this.terminateWorker();
    this.clearPending(new Error(`program worker ${this.pluginId} restarted`));
    const worker = new Worker(pluginWorkerUrl(this.moduleUrl), { type: "module" });
    this.attachWorker(worker);
    this.worker = worker;
    await this.request("init", { moduleUrl: this.moduleUrl });
  }

  private async restartWorker(reason: string): Promise<void> {
    console.warn(`[DEBUG] restarting program worker ${this.pluginId}: ${reason}`);
    await this.spawnWorker();
  }

  private request(type: PluginWorkerMessageType, payload: Record<string, unknown>): Promise<Record<string, unknown>> {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error(`program worker ${this.pluginId} is not running`));
        return;
      }
      const requestId = crypto.randomUUID();
      const isBoot = PLUGIN_WORKER_BOOT_MESSAGE_TYPES.includes(type);
      const startedAt = Date.now();
      const timer = window.setTimeout(() => {
        if (isBoot) {
          this.pending.delete(requestId);
          void this.restartWorker(`timeout:${type}`).catch((error) => {
            console.error(`[DEBUG] program worker ${this.pluginId} restart failed`, error);
          });
          reject(new Error(`program worker ${this.pluginId} timeout: ${type}`));
          return;
        }
        // 🐢️ A long-running call (e.g. a fill-plan `setFillCount`/`render` doing catch-up work) is expected
        // and must never restart the worker — a restart destroys the running app instance (document, fill
        // plan, meshes), turning one slow call into total, minutes-long unresponsiveness while everything
        // replans from zero. Only a genuine crash (`worker.onerror`) restarts a non-boot call.
        console.warn(`[DEBUG] program worker ${this.pluginId} slow ${type} call: still waiting after ${Date.now() - startedAt}ms`);
      }, isBoot ? PLUGIN_WORKER_BOOT_TIMEOUT_MS : PLUGIN_WORKER_SLOW_CALL_WARN_MS);
      this.pending.set(requestId, { resolve, reject, timer });
      this.worker.postMessage({ type, requestId, ...payload });
    });
  }

  async start(): Promise<void> {
    await this.spawnWorker();
  }

  async manifest(): Promise<string> {
    const response = await this.request("manifest", {});
    return String(response.value ?? "");
  }

  async createApp(appId: string): Promise<number> {
    const response = await this.request("createApp", { appId });
    return Number(response.instanceId);
  }

  async destroyApp(instanceId: number): Promise<void> {
    await this.request("destroy", { instanceId });
  }

  async handleAction(instanceId: number, actionJson: string, viewState: unknown): Promise<string> {
    const contextJson = JSON.stringify({ viewState, actor: "local" });
    const response = await this.request("handleAction", { instanceId, actionJson, contextJson });
    return String(response.value ?? "{}");
  }

  async handleCommand(instanceId: number, commandJson: string, viewState: unknown): Promise<string> {
    const contextJson = JSON.stringify({ viewState, actor: "local" });
    const response = await this.request("handleCommand", { instanceId, commandJson, contextJson });
    return String(response.value ?? "{}");
  }

  async render(instanceId: number, bodyKey: string, viewStateJson: string, documentJson?: string): Promise<string> {
    const response = await this.request("render", { instanceId, bodyKey, viewStateJson, documentJson });
    return String(response.value ?? "{}");
  }

  async utilities(instanceId: number, viewStateJson: string): Promise<string> {
    const response = await this.request("utilities", { instanceId, viewStateJson });
    return String(response.value ?? "[]");
  }

  async windowEngagements(instanceId: number, viewStateJson: string): Promise<string> {
    const response = await this.request("windowEngagements", { instanceId, viewStateJson });
    return String(response.value ?? "{}");
  }

  async windowMeasures(instanceId: number, viewStateJson: string): Promise<string> {
    const response = await this.request("windowMeasures", { instanceId, viewStateJson });
    return String(response.value ?? "{}");
  }

  dispose(): void {
    this.clearPending(new Error(`program worker ${this.pluginId} disposed`));
    this.terminateWorker();
  }
}

function validatePluginManifest(pluginId: string, manifest: unknown): void {
  const apps = (manifest as { apps?: unknown }).apps;
  if (!Array.isArray(apps) || apps.length === 0) {
    throw new Error(`[DEBUG] program ${pluginId} manifest has no apps`);
  }
  for (const app of apps as { windowKinds?: unknown }[]) {
    const windowKinds = app.windowKinds;
    if (!Array.isArray(windowKinds) || windowKinds.length === 0) continue;
    for (const kind of windowKinds as { surfaceKind?: unknown }[]) {
      if (!kind.surfaceKind) {
        throw new Error(`[DEBUG] program ${pluginId} manifest window kind missing surfaceKind`);
      }
    }
  }
}

async function loadPluginModuleViaWorker(pluginId: string, moduleUrl: string): Promise<PluginModuleHandle> {
  const client = new PluginWorkerClient(pluginId, moduleUrl);
  await client.start();
  const manifest = JSON.parse(await client.manifest());
  validatePluginManifest(pluginId, manifest);
  return {
    pluginId,
    manifest,
    createApp: (appId: string) => client.createApp(appId),
    destroyApp: (instanceId: number) => client.destroyApp(instanceId),
    handleAction: async (instanceId: number, actionJson: string, viewState: unknown) => parseInvocationResponse(await client.handleAction(instanceId, actionJson, viewState)),
    handleCommand: async (instanceId: number, commandJson: string, viewState: unknown) => parseInvocationResponse(await client.handleCommand(instanceId, commandJson, viewState)),
    render: async (instanceId: number, bodyKey: string, viewState: unknown) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState))),
    renderWithDocument: async (instanceId: number, bodyKey: string, viewState: unknown, documentJson: string) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState), documentJson)),
    utilities: async (instanceId: number, viewState: unknown) => JSON.parse(await client.utilities(instanceId, JSON.stringify(viewState))),
    windowEngagements: async (instanceId: number, viewState: unknown) => JSON.parse(await client.windowEngagements(instanceId, JSON.stringify(viewState))),
    windowMeasures: async (instanceId: number, viewState: unknown) => JSON.parse(await client.windowMeasures(instanceId, JSON.stringify(viewState))),
  };
}
//#region PluginWorkerLoad
function pluginHandleForBridge(handle: PluginModuleHandle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId: string) => handle.createApp(appId),
    destroyApp: (instanceId: number) => handle.destroyApp(instanceId),
    handleAction: (instanceId: number, actionJson: string, viewStateJson: string) => handle.handleAction(instanceId, actionJson, JSON.parse(viewStateJson)).then((result) => JSON.stringify(result)),
    handleCommand: (instanceId: number, commandJson: string, viewStateJson: string) => handle.handleCommand(instanceId, commandJson, JSON.parse(viewStateJson)).then((result) => JSON.stringify(result)),
    render: (instanceId: number, bodyKey: string, viewStateJson: string) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    renderWithDocument: handle.renderWithDocument
      ? (instanceId: number, bodyKey: string, viewStateJson: string, documentJson: string) => handle.renderWithDocument!(instanceId, bodyKey, JSON.parse(viewStateJson), documentJson).then((node) => JSON.stringify(node))
      : undefined,
    utilities: (instanceId: number, viewStateJson: string) => handle.utilities(instanceId, JSON.parse(viewStateJson)).then((nodes) => JSON.stringify(nodes)),
    windowEngagements: (instanceId: number, viewStateJson: string) => handle.windowEngagements(instanceId, JSON.parse(viewStateJson)).then((engagements) => JSON.stringify(engagements)),
    windowMeasures: (instanceId: number, viewStateJson: string) => handle.windowMeasures(instanceId, JSON.parse(viewStateJson)).then((measures) => JSON.stringify(measures)),
  };
}

const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, PLAYGROUND_SESSION.variant, PLAYGROUND_SESSION);
const pluginTargets: PluginRegistryEntry[] = boot.plugins.map((entry) => ({
  pluginId: entry.pluginId,
  moduleUrl: entry.moduleUrl,
  contributes: entry.contributes,
  consumes: entry.consumes,
}));
const pluginFilter = boot.variant;

async function pluginModuleAvailable(moduleUrl: string): Promise<boolean> {
  try {
    const response = await fetch(moduleUrl, { method: "HEAD" });
    return response.ok;
  } catch {
    return false;
  }
}

function renderBootErrorBanner(message: string): void {
  console.error(`[DEBUG] wgpu boot failed: ${message}`);
  const root = document.getElementById("root");
  if (!root) return;
  const banner = document.createElement("div");
  banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0a;color:#ffb4b4;font-family:monospace;font-size:14px;white-space:pre-wrap;overflow:auto;z-index:9999;";
  banner.textContent = `wgpu renderer boot failed:\n\n${message}`;
  root.appendChild(banner);
}

try {
  const availableTargets: PluginRegistryEntry[] = [];
  for (const entry of pluginTargets) {
    if (await pluginModuleAvailable(entry.moduleUrl)) {
      availableTargets.push(entry);
    }
  }
  if (availableTargets.length === 0) {
    throw new Error(`[DEBUG] no wasm plugin modules found for filter ${pluginFilter}`);
  }

  const handles = await Promise.all(
    availableTargets.map(async (entry) => ({
      pluginId: entry.pluginId,
      handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl)),
    })),
  );

  const bindings = await new Promise<Record<string, unknown>>((resolve, reject) => {
    const host = window as { wasmBindings?: Record<string, unknown> };
    const finish = () => {
      if (!host.wasmBindings) {
        reject(new Error("[DEBUG] trunk wasm bindings missing"));
        return;
      }
      resolve(host.wasmBindings);
    };
    if (host.wasmBindings) {
      finish();
      return;
    }
    const timeout = window.setTimeout(() => reject(new Error("[DEBUG] trunk wasm bindings timeout")), 30000);
    const done = () => {
      window.clearTimeout(timeout);
      window.clearInterval(poll);
      finish();
    };
    window.addEventListener("TrunkApplicationStarted", done, { once: true });
    const poll = window.setInterval(() => {
      if (host.wasmBindings) done();
    }, 50);
  });

  if (!bindings.semioWgpuMount) throw new Error("[DEBUG] missing semioWgpuMount");
  const root = document.getElementById("root");
  if (!root) throw new Error("[DEBUG] missing #root");
  const canvas = document.createElement("canvas");
  canvas.style.display = "block";
  canvas.style.width = "100%";
  canvas.style.height = "100%";
  canvas.style.touchAction = "none";
  canvas.style.outline = "none";
  root.replaceChildren(canvas);
  (bindings.semioWgpuMount as (canvas: HTMLCanvasElement, handles: typeof handles, pluginFilter: string) => void)(canvas, handles, pluginFilter);
} catch (error) {
  renderBootErrorBanner(error instanceof Error ? error.message : String(error));
  throw error;
}

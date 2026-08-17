// #region 🧲️Header
/** @emoji 🧊️ Trunk boot glue — loads wasm programs and starts the wgpu renderer. */
// #endregion 🧲️Header

import { parseInvocationResponse, pluginGraphErrorMessage, type PluginRegistryEntry, type ShellLocale } from "@semio-tech/framework";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../../../../../../🔌️plugin/📇️registry/🟦️catalog.ts";

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

  private readonly pluginId: string;
  private readonly moduleUrl: string;

  constructor(
    pluginId: string,
    moduleUrl: string,
  ) {
    this.pluginId = pluginId;
    this.moduleUrl = moduleUrl;
  }

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

const bootVariant = new URLSearchParams(window.location.search).get("plugin") ?? "s";
const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariant);
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

/** 🌐️ No shell locale selector exists this early in boot (before any app/config has loaded) —
 * `navigator.language` is the best signal available, English/German only per the repo's
 * `ShellLocale` axis. */
function resolveBootLocale(): ShellLocale {
  return typeof navigator !== "undefined" && navigator.language.toLowerCase().startsWith("de") ? "de" : "en";
}

/** 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: `VITE_SEMIO_APP_ROLE`,
 * values `"viewer"`/`"editor"`, default `"editor"`. This target is Trunk-served (`Trunk.toml`), not
 * Vite-bundled, so `import.meta.env.VITE_SEMIO_APP_ROLE` is read defensively (a harmless `undefined`
 * unless a deployment wraps this boot module through a Vite dev server) — a `?plugin=`-style URL
 * param is the always-available fallback for this shell, mirroring `bootVariant`'s own
 * `URLSearchParams` idiom a few lines below. */
function resolveBootAppRole(): string {
  const viteEnv = (import.meta as unknown as { env?: Record<string, string | undefined> }).env?.VITE_SEMIO_APP_ROLE;
  const urlRole = new URLSearchParams(window.location.search).get("role") ?? undefined;
  return viteEnv === "viewer" || urlRole === "viewer" ? "viewer" : "editor";
}

/** 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0 — `S_HUB_URL`/
 * `S_USER`/`S_DATA_DIR` for the browser wgpu build. Same defensive-read posture as
 * `resolveBootAppRole` right above (this target is Trunk-served, not Vite-bundled, so
 * `import.meta.env.VITE_S_*` only resolves when a deployment wraps this boot module through a Vite
 * dev server) with `?hub=`/`?user=`/`?dataDir=` URL-param fallbacks mirroring `resolveBootAppRole`'s
 * own `?role=` idiom. `undefined` hub url ⇒ no hub env at all ⇒ `semioWgpuSetHubEnv` is never called
 * ⇒ the Rust side's `resolve_identity_env` stays `None` ⇒ unchanged local-only behaviour. */
function resolveBootHubEnv(): { hubUrl: string; user: string; dataDir: string } | undefined {
  const viteEnv = (import.meta as unknown as { env?: Record<string, string | undefined> }).env;
  const params = new URLSearchParams(window.location.search);
  const hubUrl = viteEnv?.VITE_S_HUB_URL ?? params.get("hub") ?? undefined;
  if (!hubUrl) return undefined;
  const user = viteEnv?.VITE_S_USER ?? params.get("user") ?? "";
  const dataDir = viteEnv?.VITE_S_DATA_DIR ?? params.get("dataDir") ?? "";
  return { hubUrl, user, dataDir };
}

/** 🌐️ Surfaces a missing/incompatible plugin dependency (ticket
 * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS) as a real, localized
 * banner instead of only a console error — non-fatal, since `boot.plugins` already excludes the
 * blocked entries and every OTHER plugin still boots (contract freeze §4 rule 5's fail-soft posture). */
function renderDependencyFaultBanner(messages: readonly string[]): void {
  const root = document.getElementById("root");
  if (!root) return;
  const banner = document.createElement("div");
  banner.style.cssText = "position:fixed;top:0;left:0;right:0;padding:12px 24px;background:#4a2a00;color:#ffd8a8;font-family:monospace;font-size:13px;white-space:pre-wrap;z-index:9998;";
  banner.textContent = messages.join("\n");
  root.appendChild(banner);
}

if (boot.dependencyErrors.length > 0) {
  const locale = resolveBootLocale();
  const messages = boot.dependencyErrors.map((error) => pluginGraphErrorMessage(error, locale));
  for (const message of messages) console.error(`[DEBUG] plugin dependency fault: ${message}`);
  renderDependencyFaultBanner(messages);
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

  // 🎯️ Loaded SEQUENTIALLY, in `boot.plugins`'s already dependency-ordered sequence (scout-2 §4:
  // "boot must walk the dependency order... instead of relying on array order") — a concurrent
  // `Promise.all` gives no guarantee a dependency finishes loading before its dependent starts.
  const handles: { readonly pluginId: string; readonly handle: ReturnType<typeof pluginHandleForBridge> }[] = [];
  for (const entry of availableTargets) {
    handles.push({ pluginId: entry.pluginId, handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl)) });
  }

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
  // 👁️✏️ Contract freeze §5: boot role, applied before mount so the very first `Shell::set_window_layout`
  // already carries it. Guarded — `semioWgpuSetAppRole` is new (this ticket) and a stale wasm build
  // predating it simply skips role chrome rather than throwing, same fail-soft posture as every other
  // optional binding this file checks.
  if (bindings.semioWgpuSetAppRole) {
    (bindings.semioWgpuSetAppRole as (role: string) => void)(resolveBootAppRole());
  }
  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§1 — same guarded,
  // fail-soft posture as `semioWgpuSetAppRole` right above: a stale wasm build predating this ticket
  // simply skips identity/directory wiring rather than throwing.
  const hubEnv = resolveBootHubEnv();
  if (hubEnv && bindings.semioWgpuSetHubEnv) {
    (bindings.semioWgpuSetHubEnv as (hubUrl: string, user: string, dataDir: string) => void)(hubEnv.hubUrl, hubEnv.user, hubEnv.dataDir);
  }
  (bindings.semioWgpuMount as (canvas: HTMLCanvasElement, handles: typeof handles, pluginFilter: string) => void)(canvas, handles, pluginFilter);
} catch (error) {
  renderBootErrorBanner(error instanceof Error ? error.message : String(error));
  throw error;
}

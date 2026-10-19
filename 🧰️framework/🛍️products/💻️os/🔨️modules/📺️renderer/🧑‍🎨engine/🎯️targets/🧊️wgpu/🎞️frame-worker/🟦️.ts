import { bootAbort, closed, closing, consumedTopics, diagnosticsStamp, explicitPluginModules, failed, scope, suspensionLedger, typedMediaPlugins, workerCatalog } from "./🧵️state/🟦️.ts";
import type { BrowserFrameUiMessage } from "../🚚️browser-frame-transport/📋️contract/🟦️.ts";
import { receive } from "./📨️receive/🟦️.ts";
import { requestFault } from "./🧯️close-and-fault/🟦️.ts";
import { declareBootSubphase, monitoredSuspension } from "./⏱️steps-and-reporting/🟦️.ts";
import { requestHostIo } from "./📦️assets-and-host-io/🟦️.ts";
import { createLazyPluginInstallDoor } from "./🧩️lazy-install/🟦️.ts";
import { loadPluginModule } from "../🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts";
import { pluginHandleForBridge } from "../🐚️plugin-bridge/🟦️.ts";
import { primeContributionManifest } from "../🐚️plugin-bridge/🎛️contributions-and-effects/🟦️.ts";
import { installWgpuDynamicExtensionDoor } from "../🧩️dynamic-extension/🟦️.ts";

scope.onmessage = (event: MessageEvent<BrowserFrameUiMessage>) => void receive(event.data);

/** 🧯️ Last-resort seam for a throw that escapes every awaited step — a trap raised inside a wasm
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

(globalThis as { semioDeclareBootSubphase?: typeof declareBootSubphase }).semioDeclareBootSubphase = declareBootSubphase;

(globalThis as { semioWgpuHostIo?: typeof requestHostIo }).semioWgpuHostIo = requestHostIo;

/** 🧩️ The door the browser wgpu shell's `install_plugin` calls (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 * `install_js_plugin`). It mounts through the SAME `loadPluginModule`/`pluginHandleForBridge` pair
 * {@link mountPluginHandles} uses, under the same `monitoredSuspension` watchdog declaration, so a
 * lazily installed plugin is indistinguishable from an eagerly mounted one once it lands. The module
 * url comes from the catalog rather than the boot plan on purpose: the whole point is reaching a
 * plugin the boot plan did NOT name. */
const lazyPluginInstalls = createLazyPluginInstallDoor({
  moduleUrl: (pluginId) => explicitPluginModules?.find(row => row.pluginId === pluginId)?.moduleUrl ?? workerCatalog?.moduleUrl(pluginId) ?? workerCatalog?.extensionModuleUrl(pluginId),
  mount: async (pluginId, moduleUrl) => {
    if (closed || closing || failed) throw new Error("plugin-install.closing: the frame Worker is closing");
    const module = await monitoredSuspension(`plugin-install:${pluginId}`, () => loadPluginModule(pluginId, moduleUrl, bootAbort.signal, undefined, consumedTopics(pluginId)), suspensionLedger);
    typedMediaPlugins.set(pluginId, module);
    const handle = pluginHandleForBridge(module);
    void primeContributionManifest(pluginId, moduleUrl).catch(() => {});
    return handle;
  },
  progress: (pluginId, phase) => {
    if (diagnosticsStamp === true) console.debug(`[TRACE] wgpu lazy plugin install ${pluginId} ${phase}`);
  },
});

(globalThis as { semioWgpuInstallPlugin?: (pluginId: string) => Promise<unknown> }).semioWgpuInstallPlugin = (pluginId) => lazyPluginInstalls.install(pluginId);

(globalThis as { semioWgpuCancelPluginInstall?: (pluginId: string) => boolean }).semioWgpuCancelPluginInstall = (pluginId) => lazyPluginInstalls.cancel(pluginId);

const dynamicExtensionModules = new Map<string, Awaited<ReturnType<typeof loadPluginModule>>>();

installWgpuDynamicExtensionDoor(globalThis, async (record) => {
  if (closed || closing || failed) throw new Error("extension-install.closing: the frame Worker is closing");
  const previous = dynamicExtensionModules.get(record.extensionId);
  dynamicExtensionModules.delete(record.extensionId);
  await previous?.dispose();
  const module = await monitoredSuspension(`extension-install:${record.extensionId}`, () => loadPluginModule(record.extensionId, record.moduleUrl, bootAbort.signal, undefined, consumedTopics(record.extensionId)), suspensionLedger);
  dynamicExtensionModules.set(record.extensionId, module);
  void primeContributionManifest(record.extensionId, record.moduleUrl).catch(() => {});
  return { handle: pluginHandleForBridge(module), manifest: module.manifest };
}, async (extensionId) => {
  const module = dynamicExtensionModules.get(extensionId);
  if (!module) return;
  dynamicExtensionModules.delete(extensionId);
  await module.dispose();
});

import { resolveEmbeddedWgpuBoot } from "./🧩️selection/🟦️.ts";
// #region 🧲️Header
/** 🧊️ `@semio-tech/framework-renderer-wgpu` — raw wgpu WASM renderer boot for declarative Rust program UI trees. */
// #endregion 🧲️Header

import { type WgpuBootDefaults, type WgpuBootHub, type WgpuBootLocks } from "../🧭️boot-descriptor/🟦️.ts";
import { mountWgpuBrowserHost, type WgpuBrowserMount, type WgpuBootProgress } from "../🌐️browser-host/🟦️.ts";
import { Locale } from "../../../../../../../../🔨️modules/🖱️ui/🌐️locale/🟦️.ts";

/** 🧊️ The embeddable wgpu boot door. Field-for-field React's `FrameworkOsBootOptions`
 * (`🧱️elements/🐚️Shell/🟦️.tsx`), so one call site can swap renderers without dropping options: it
 * used to carry `rootId`/`plugin`/`plugins`/`rendererModuleUrl` alone, which silently discarded
 * `appId`/`appRole`/`locks`/`defaults`/`brand` on this target. Like React's own library door it reads
 * NO `?query=` — an embedder says what it wants — but it does read the one-shot `#semio-broker=` proof
 * the way `🏛️ShellHost/🟦️.tsx` does, because that hash addresses the page, not the mount.
 *
 * `surfaceSessionFactories` is React-only by construction (a JS `AppSurfaceSessionFactory` closure
 * cannot cross into the renderer wasm); the wgpu twin of that seam is the plugin bridge. */
export type FrameworkOsWgpuBootOptions = {
  readonly rootId?: string;
  readonly plugin?: string;
  readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  readonly appRole?: "viewer" | "editor";
  readonly appMode?: string;
  readonly appExample?: string;
  readonly locks?: Partial<WgpuBootLocks>;
  readonly defaults?: Partial<WgpuBootDefaults>;
  readonly brand?: string;
  readonly hub?: WgpuBootHub;
  readonly rendererModuleUrl?: string;
};

/** 🎬️ Owns host resource loading and lifecycle capabilities for one renderer mount. */
export type FrameworkOsWgpuBootExecution = {
  readonly rendererWasmUrl?: string;
  readonly frameWorkerUrl?: string;
  readonly suppressAutoIntroduction?: boolean;
  readonly signal?: AbortSignal;
  readonly onProgress?: (event: WgpuBootProgress) => void;
};

const DEFAULT_RENDERER_MODULE_URL = "/renderer-modules/wgpu/semio-framework-os-renderer-wgpu.js";

/** 🪆️ Boots one independently rooted frame Worker and retires only that mount. */
export async function bootFrameworkOsWgpu(options: FrameworkOsWgpuBootOptions = {}, execution: FrameworkOsWgpuBootExecution = {}): Promise<WgpuBrowserMount> {
  const rootId = options.rootId ?? "root";
  const root = document.getElementById(rootId);
  if (!root) throw new Error("missing #" + rootId);
  const { plugins, descriptor, pluginRegistrySelection } = resolveEmbeddedWgpuBoot({
    modules: options.plugins ?? [], baseUrl: window.location.href, hash: window.location.hash,
    overrides: { plugin: options.plugin, appId: options.appId, appRole: options.appRole, appMode: options.appMode, appExample: options.appExample, brandId: options.brand, locks: options.locks, defaults: options.defaults, hub: options.hub },
  });
  const moduleUrl = new URL(options.rendererModuleUrl ?? DEFAULT_RENDERER_MODULE_URL, window.location.href).href;
  const wasmUrl = execution.rendererWasmUrl ?? moduleUrl.replace(/\.js(?=[?#]|$)/u, "_bg.wasm");
  const locale = Locale.fromLanguageTag(descriptor.locks.locale || navigator.language).id as "en" | "de";
  return mountWgpuBrowserHost(root, { descriptor, locale, mountId: rootId, rendererModuleUrl: moduleUrl, rendererWasmUrl: wasmUrl, plugins, pluginRegistrySelection, frameWorkerUrl: execution.frameWorkerUrl, suppressAutoIntroduction: execution.suppressAutoIntroduction, signal: execution.signal, onProgress: execution.onProgress });
}

export type { WgpuBootProgress } from "../🌐️browser-host/🟦️.ts";

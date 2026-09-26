// #region 🧲️Header
/** @emoji 🧊️ `@semio-tech/framework-renderer-wgpu` — raw wgpu WASM renderer boot for declarative Rust program UI trees. */
// #endregion 🧲️Header

import { watchAgentBridgeOffer } from "../../../🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";
import { ICON_NAMES, ICONS } from "@semio-tech/assets";
import { loadPluginModule, pluginHandleForBridge } from "../🐚️plugin-bridge/🟦️.ts";
import { installWgpuPageHostIo } from "../🚪️host-io/🟦️.ts";
import { installWgpuDynamicExtensionDoor } from "../🧩️dynamic-extension/🟦️.ts";
import { WGPU_PREFERS_DARK_MEDIA_QUERY, readWgpuHostStorageSnapshot, resolveWgpuBootDescriptor, resolveWgpuHostAppearance, resolveWgpuHostPlatform, type WgpuBootDefaults, type WgpuBootHub, type WgpuBootLocks } from "../🧭️boot-descriptor/🟦️.ts";

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

const DEFAULT_RENDERER_MODULE_URL = "/renderer-modules/wgpu/semio-framework-os-renderer-wgpu.js";

/** 🧩️ The variant an embedder that names none boots — the host shell, as this door always defaulted. */
const DEFAULT_EMBEDDED_VARIANT = "s";

const SEMIO_LOGO_SVG = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 350 350"><path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117"/><path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"/><g fill="#ff344f" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z"/></g><g fill="#34d1bf" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z"/></g></svg>`;

const ICON_SIZE = 24;
const ATLAS_COLS = 16;
const ICON_ATLAS_TEXTURE_SIZE = 2048;

async function rasterizeSvg(svg: string): Promise<ImageData | null> {
  const blob = new Blob([svg], { type: "image/svg+xml" });
  const url = URL.createObjectURL(blob);
  try {
    const image = await new Promise<HTMLImageElement>((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = reject;
      img.src = url;
    });
    const canvas = document.createElement("canvas");
    canvas.width = ICON_SIZE;
    canvas.height = ICON_SIZE;
    const ctx = canvas.getContext("2d");
    if (!ctx) return null;
    ctx.clearRect(0, 0, ICON_SIZE, ICON_SIZE);
    ctx.drawImage(image, 0, 0, ICON_SIZE, ICON_SIZE);
    return ctx.getImageData(0, 0, ICON_SIZE, ICON_SIZE);
  } finally {
    URL.revokeObjectURL(url);
  }
}

async function rasterizeIcon(id: string): Promise<ImageData | null> {
  const svg = ICONS[id as keyof typeof ICONS];
  if (!svg) return null;
  const image = await rasterizeSvg(svg);
  if (!image || id === "semio-logo") return image;
  return iconTintMask(image);
}

/** Converts rasterized stroke icons into a white mask so GPU tint multiply matches React `currentColor`. */
function iconTintMask(image: ImageData): ImageData {
  const out = new ImageData(image.width, image.height);
  for (let i = 0; i < image.data.length; i += 4) {
    const a = image.data[i + 3] ?? 0;
    out.data[i] = 255;
    out.data[i + 1] = 255;
    out.data[i + 2] = 255;
    out.data[i + 3] = a;
  }
  return out;
}

export async function buildIconAtlas(): Promise<{
  width: number;
  height: number;
  pixels: Uint8Array;
  entries: Record<string, [number, number, number, number]>;
}> {
  const loaded = await Promise.all([...ICON_NAMES.map(async (id) => ({ id, image: await rasterizeIcon(id) })), { id: "semio-logo", image: await rasterizeSvg(SEMIO_LOGO_SVG) }]);
  const rows = Math.ceil(loaded.length / ATLAS_COLS);
  const width = ATLAS_COLS * ICON_SIZE;
  const height = rows * ICON_SIZE;
  const pixels = new Uint8Array(width * height * 4);
  const entries: Record<string, [number, number, number, number]> = {};
  for (const [index, item] of loaded.entries()) {
    if (!item.image) continue;
    const col = index % ATLAS_COLS;
    const row = Math.floor(index / ATLAS_COLS);
    const ox = col * ICON_SIZE;
    const oy = row * ICON_SIZE;
    for (let y = 0; y < ICON_SIZE; y++) {
      for (let x = 0; x < ICON_SIZE; x++) {
        const src = (y * ICON_SIZE + x) * 4;
        const dst = ((oy + y) * width + (ox + x)) * 4;
        pixels[dst] = item.image.data[src] ?? 0;
        pixels[dst + 1] = item.image.data[src + 1] ?? 0;
        pixels[dst + 2] = item.image.data[src + 2] ?? 0;
        pixels[dst + 3] = item.image.data[src + 3] ?? 0;
      }
    }
    entries[item.id] = [ox / ICON_ATLAS_TEXTURE_SIZE, oy / ICON_ATLAS_TEXTURE_SIZE, (ox + ICON_SIZE) / ICON_ATLAS_TEXTURE_SIZE, (oy + ICON_SIZE) / ICON_ATLAS_TEXTURE_SIZE];
  }
  return { width, height, pixels, entries };
}

/**
 * 🧊️ Boots the real wgpu renderer WASM into `options.rootId`'s container (default `"root"`, kept as a
 * single-app-per-page convenience default, not a hardcoded assumption). Unlike the old
 * `semioRendererBoot` (which always looked up `#root` itself, cleared it via `set_inner_html("")`, and
 * created+appended its own fixed `#semio-wgpu-canvas` — a second boot call would wipe the first mount's
 * canvas), this side now owns creating and placing the canvas and hands it to the Rust
 * `semioWgpuMount(canvas, plugins, pluginFilter)` entry point, so several independently-rooted mounts can
 * coexist on one page.
 *
 * Returns a dispose callback for hosts (e.g. Storybook, the multi-shell harness) that need to unmount —
 * it detaches the canvas and disposes this call's `loadPluginModule` actors (`ShardClient.dispose`,
 * MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME `wgpu-web-shard` — replaces the deleted `acquirePluginModule`
 * lease/release pair). The underlying wasm event loop still has no JS-visible stop handle (see the doc
 * comment on `semio_wgpu_mount` in `../🧊️renderer/🦀️.rs` for why a real one isn't wired up yet), so this remains a
 * best-effort cleanup, not a full runtime teardown: the mount keeps rendering into a detached canvas
 * until the page unloads.
 */
export async function bootFrameworkOsWgpu(options: FrameworkOsWgpuBootOptions = {}): Promise<() => Promise<void>> {
  const rootId = options.rootId ?? "root";
  const root = document.getElementById(rootId);
  if (!root) throw new Error(`missing #${rootId}`);
  const descriptor = resolveWgpuBootDescriptor({
    hash: typeof window === "undefined" ? "" : window.location.hash,
    defaultVariant: DEFAULT_EMBEDDED_VARIANT,
    overrides: { plugin: options.plugin, appId: options.appId, appRole: options.appRole, appMode: options.appMode, appExample: options.appExample, brandId: options.brand, locks: options.locks, defaults: options.defaults, hub: options.hub },
  });
  // 🚪️ This variant mounts the shell on the PAGE, so it installs the file door directly rather than
  // through the Worker bridge — one `semioWgpuHostIo` binding, two installs, so neither io journey can
  // work on one variant and vanish on the other (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  installWgpuPageHostIo();
  const dynamicHandles: Awaited<ReturnType<typeof loadPluginModule>>[] = [];
  const dynamicExtensions = new Map<string, Awaited<ReturnType<typeof loadPluginModule>>>();
  installWgpuDynamicExtensionDoor(globalThis, async (record) => {
    const previous = dynamicExtensions.get(record.extensionId);
    dynamicExtensions.delete(record.extensionId);
    if (previous) {
      const index = dynamicHandles.indexOf(previous);
      if (index >= 0) dynamicHandles.splice(index, 1);
      await previous.dispose();
    }
    const module = await loadPluginModule(record.extensionId, record.moduleUrl);
    dynamicHandles.push(module);
    dynamicExtensions.set(record.extensionId, module);
    return { handle: pluginHandleForBridge(module), manifest: module.manifest };
  }, async (extensionId) => {
    const module = dynamicExtensions.get(extensionId);
    if (!module) return;
    dynamicExtensions.delete(extensionId);
    const index = dynamicHandles.indexOf(module);
    if (index >= 0) dynamicHandles.splice(index, 1);
    await module.dispose();
  });

  const canvas = document.createElement("canvas");
  canvas.style.display = "block";
  canvas.style.width = "100%";
  canvas.style.height = "100%";
  canvas.style.touchAction = "none";
  canvas.style.outline = "none";
  root.replaceChildren(canvas);

  const pluginEntries = options.plugins ?? [];
  const [loadedHandles, iconAtlas] = await Promise.all([
    Promise.all(pluginEntries.map((entry) => loadPluginModule(entry.pluginId, entry.moduleUrl))),
    buildIconAtlas(),
  ]);
  const handles = loadedHandles.map((handle, index) => ({ pluginId: pluginEntries[index]!.pluginId, handle: pluginHandleForBridge(handle) }));

  const rendererUrl = options.rendererModuleUrl ?? DEFAULT_RENDERER_MODULE_URL;
  const rendererModule = (await import(/* @vite-ignore */ rendererUrl)) as {
    default?: (input?: WebAssembly.Module | BufferSource | Response) => Promise<void>;
    semioWgpuMount?: (canvas: HTMLCanvasElement, plugins: { pluginId: string; handle: ReturnType<typeof pluginHandleForBridge> }[], pluginFilter: string) => void;
    semioWgpuSetBootDescriptor?: (descriptorJson: string) => void;
    semioWgpuSetHubEnv?: (hubUrl: string, user: string, dataDir: string) => void;
    semioWgpuSetHostAppearance?: (preference: string, systemDark: boolean) => void;
    semioWgpuSetHostPlatform?: (platform: string) => void;
    semioWgpuSetHostLocale?: (locale: string) => void;
    semioWgpuSetHostStorage?: (snapshotJson: string) => void;
    semioWgpuSetAgentBridgeConfig?: (url: string, admissionProof: string) => void;
    uploadIconAtlas?: (width: number, height: number, pixels: Uint8Array, entriesJson: string) => void;
  };
  if (rendererModule.default) await rendererModule.default();
  if (!rendererModule.semioWgpuMount) {
    throw new Error("wgpu renderer module missing semioWgpuMount");
  }
  // 🧭️ Boot axes cross BEFORE the mount, the same ordering the frame Worker keeps: `ShellState::boot`
  // reads them while it opens the session, so a descriptor applied afterwards would be read by nobody.
  rendererModule.semioWgpuSetBootDescriptor?.(JSON.stringify(descriptor));
  if (descriptor.hub) rendererModule.semioWgpuSetHubEnv?.(descriptor.hub.hubUrl, descriptor.hub.user, descriptor.hub.dataDir);
  // 🌓️ This door runs ON the page, so unlike the frame Worker it CAN read both appearance inputs —
  // it forwards them through the same one hook, and keeps the `system` half live for as long as the
  // mount lives (React's own shared `matchMedia` listener, per shell).
  const publishAppearance = () => {
    const appearance = resolveWgpuHostAppearance(window);
    rendererModule.semioWgpuSetHostAppearance?.(appearance.preference, appearance.systemDark);
  };
  // 🗄️ Same story for the durable preference census: this door runs ON the page, so it seeds the
  // renderer's synchronous cache itself before the mount reads a single key, and re-seeds whenever
  // another document on this origin rewrites one.
  const publishHostStorage = () => rendererModule.semioWgpuSetHostStorage?.(JSON.stringify(readWgpuHostStorageSnapshot(window)));
  publishHostStorage();
  publishAppearance();
  // ⌨️ Constant for the life of a navigation, so it is published once and never listened to — unlike
  // the two above, no user action can change which machine this is.
  rendererModule.semioWgpuSetHostPlatform?.(resolveWgpuHostPlatform(window));
  // 🗣️ Read once, as React's `ShellHost` reads `navigator.language` once per scope.
  rendererModule.semioWgpuSetHostLocale?.(window.navigator?.language ?? "");
  const darkQuery = window.matchMedia?.(WGPU_PREFERS_DARK_MEDIA_QUERY);
  darkQuery?.addEventListener("change", publishAppearance);
  window.addEventListener("storage", publishAppearance);
  window.addEventListener("storage", publishHostStorage);
  rendererModule.semioWgpuMount(canvas, handles, descriptor.pluginVariant);
  const stopAgentBridgeOffer = watchAgentBridgeOffer((offer) => rendererModule.semioWgpuSetAgentBridgeConfig?.(offer?.url ?? "", offer?.admissionProof ?? ""));
  if (rendererModule.uploadIconAtlas) {
    rendererModule.uploadIconAtlas(iconAtlas.width, iconAtlas.height, iconAtlas.pixels, JSON.stringify(iconAtlas.entries));
  }
  await new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
  return async () => {
    stopAgentBridgeOffer();
    darkQuery?.removeEventListener("change", publishAppearance);
    window.removeEventListener("storage", publishAppearance);
    window.removeEventListener("storage", publishHostStorage);
    root.replaceChildren();
    const results = await Promise.allSettled([...loadedHandles, ...dynamicHandles].map((handle) => handle.dispose()));
    const failures = results.filter((result): result is PromiseRejectedResult => result.status === "rejected");
    if (failures.length) throw new AggregateError(failures.map((result) => result.reason), "wgpu-renderer.retirement-failed");
  };
}

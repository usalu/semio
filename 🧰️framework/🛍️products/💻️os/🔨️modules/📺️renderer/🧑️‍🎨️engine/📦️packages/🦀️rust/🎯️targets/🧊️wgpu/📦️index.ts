// #region 🧲️Header
/** @emoji 🧊️ `@semio-tech/framework-renderer-wgpu` — raw wgpu WASM renderer boot for declarative Rust program UI trees. */
// #endregion 🧲️Header

import { ICON_NAMES, ICONS } from "@semio-tech/assets";
import { acquirePluginModule, pluginHandleForBridge } from "@semio-tech/framework";

export type FrameworkOsWgpuBootOptions = {
  readonly rootId?: string;
  readonly plugin?: string;
  readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly rendererModuleUrl?: string;
};

const DEFAULT_RENDERER_MODULE_URL = "/renderer-modules/wgpu/semio_framework_renderer_wgpu.js";

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
 * it detaches the canvas and releases this call's `acquirePluginModule` leases. The underlying wasm event
 * loop still has no JS-visible stop handle (see the doc comment on `semio_wgpu_mount` in `📦️glue.rs` for
 * why a real one isn't wired up yet), so this remains a best-effort cleanup, not a full runtime teardown:
 * the mount keeps rendering into a detached canvas until the page unloads.
 */
export async function bootFrameworkOsWgpu(options: FrameworkOsWgpuBootOptions = {}): Promise<() => void> {
  const rootId = options.rootId ?? "root";
  const root = document.getElementById(rootId);
  if (!root) throw new Error(`missing #${rootId}`);

  const canvas = document.createElement("canvas");
  canvas.style.display = "block";
  canvas.style.width = "100%";
  canvas.style.height = "100%";
  canvas.style.touchAction = "none";
  canvas.style.outline = "none";
  root.replaceChildren(canvas);

  const pluginEntries = options.plugins ?? [];
  const [leases, iconAtlas] = await Promise.all([
    Promise.all(pluginEntries.map((entry) => acquirePluginModule(entry.pluginId, entry.moduleUrl))),
    buildIconAtlas(),
  ]);
  const handles = leases.map((lease, index) => ({ pluginId: pluginEntries[index]!.pluginId, handle: pluginHandleForBridge(lease.handle) }));

  const rendererUrl = options.rendererModuleUrl ?? DEFAULT_RENDERER_MODULE_URL;
  const rendererModule = (await import(/* @vite-ignore */ rendererUrl)) as {
    default?: (input?: WebAssembly.Module | BufferSource | Response) => Promise<void>;
    semioWgpuMount?: (canvas: HTMLCanvasElement, plugins: { pluginId: string; handle: ReturnType<typeof pluginHandleForBridge> }[], pluginFilter: string) => void;
    uploadIconAtlas?: (width: number, height: number, pixels: Uint8Array, entriesJson: string) => void;
  };
  if (rendererModule.default) await rendererModule.default();
  if (!rendererModule.semioWgpuMount) {
    throw new Error("wgpu renderer module missing semioWgpuMount");
  }
  rendererModule.semioWgpuMount(canvas, handles, options.plugin ?? "s");
  if (rendererModule.uploadIconAtlas) {
    rendererModule.uploadIconAtlas(iconAtlas.width, iconAtlas.height, iconAtlas.pixels, JSON.stringify(iconAtlas.entries));
  }
  await new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
  return () => {
    root.replaceChildren();
    for (const lease of leases) lease.release();
  };
}

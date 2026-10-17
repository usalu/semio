/** 🔌️ The TypeScript side of Canvas's runtime asset port (`🦀️.rs`). Canvas embeds no font and no icon: a browser host serves
 * the directories in {@link RUNTIME_ASSET_CHANNELS} at their routes and hands the seven fetched files to the wasm module through
 * {@link installRuntimeAssets} before the module resolves its first icon or label. Every browser wasm module that links Canvas
 * exports the same door, `semioSetRuntimeAssets`. */
import { COMPILER_FONTS_DIRECTORY, COMPILER_FONT_FILES } from "@semio-tech/framework-compiler-ts/world";

/** 📦️ One directory a host serves at one route; its files are fetched relative to that route. */
export interface RuntimeAssetChannel {
  readonly id: string;
  readonly route: string;
  readonly directory: string;
  readonly files: readonly string[];
}

/** 🏷️ The asset directories in the order of the Rust `RuntimeAsset::ALL` (five compiler fonts, the label face, the icon pack). */
export const RUNTIME_ASSET_CHANNELS: readonly RuntimeAssetChannel[] = [
  { id: "compiler-fonts", route: "/renderer-modules/compiler-fonts", directory: COMPILER_FONTS_DIRECTORY, files: COMPILER_FONT_FILES },
  { id: "canvas-fonts", route: "/renderer-modules/canvas-fonts", directory: "🧰️framework/🔨️modules/🖼️canvas/🖼️assets", files: ["🔤️MapLabelSans.ttf"] },
  { id: "icon-pack", route: "/renderer-modules/icon-pack", directory: "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes", files: ["🔣️.json"] },
];

/** 📋️ Every asset file with the route it is served at, in `RuntimeAsset::ALL` order. */
export const RUNTIME_ASSET_FILES: readonly { readonly channel: string; readonly route: string; readonly file: string }[] = RUNTIME_ASSET_CHANNELS.flatMap(({ id, route, files }) => files.map(file => ({ channel: id, route, file })));

/** 📦️ The assets a `wasm32-wasip2` guest uses, the last three of {@link RUNTIME_ASSET_FILES} (the emoji face, the label face and the icon
 * pack): what Rust's `RuntimeAsset::GUEST` names and `PluginBuilder::canvas_runtime_assets` declares as package assets. */
export const GUEST_RUNTIME_ASSET_FILES: readonly { readonly channel: string; readonly route: string; readonly file: string }[] = RUNTIME_ASSET_FILES.slice(-3);

/** 🚏️ The static-directory deliveries a dev server mounts so every channel answers at its route. */
export function runtimeAssetDeliveries(): readonly { readonly kind: "static-dir"; readonly route: string; readonly root: string }[] {
  return RUNTIME_ASSET_CHANNELS.map(({ route, directory }) => ({ kind: "static-dir" as const, route, root: directory }));
}

/** 🔌️ The `semioSetRuntimeAssets` door, absent on wasm modules that do not link Canvas. */
export interface RuntimeAssetBindings {
  readonly semioSetRuntimeAssets?: (math: Uint8Array, serif: Uint8Array, serifItalic: Uint8Array, mono: Uint8Array, emoji: Uint8Array, labelFont: Uint8Array, iconPack: Uint8Array) => void;
}

/** 📥️ Fetches the seven asset files relative to `base` and installs them in one call, so the module never resolves an icon or
 * label without them. Answers `false` without fetching when the bindings carry no door; any failed fetch rejects. */
export async function installRuntimeAssets(bindings: object, base: string, fetchAsset: (url: string) => Promise<Response>): Promise<boolean> {
  const install = (bindings as RuntimeAssetBindings).semioSetRuntimeAssets;
  if (!install) return false;
  const [math, serif, serifItalic, mono, emoji, labelFont, iconPack] = await Promise.all(RUNTIME_ASSET_FILES.map(async ({ route, file }) => {
    const url = new URL(`${route}/${encodeURIComponent(file)}`, base).href;
    const response = await fetchAsset(url);
    if (!response.ok) throw new Error(`runtime-assets: ${response.status} for ${url}`);
    return new Uint8Array(await response.arrayBuffer());
  }));
  install(math!, serif!, serifItalic!, mono!, emoji!, labelFont!, iconPack!);
  return true;
}

/** 🌐️ [`installRuntimeAssets`] for a page host: fetches from the page's own origin and reports a failure instead of failing the
 * module, because only icons and labels depend on the assets. */
export async function installPageRuntimeAssets(bindings: object): Promise<void> {
  try {
    await installRuntimeAssets(bindings, globalThis.location.href, (url) => fetch(url));
  } catch (error) {
    console.error(`[runtime-assets] icons and labels will not render: ${error instanceof Error ? error.message : String(error)}`);
  }
}

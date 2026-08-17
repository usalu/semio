import {readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioBrandHtmlVitePlugins, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, semioViteProductionBuild, staticDirVitePlugin, semioAssetsVitePlugin } from "../../../../../../🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts";
import { DEFAULT_HOST_VARIANT, PLAYGROUND_BUILD_TARGETS } from "../../../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { isHostPluginFilter } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts";
import { resolveShellBrandById } from "../../🏷️brand/📦️index.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin, semioPluginHotSwapVitePlugin } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts";
import { defaultExtensionInstallRoot, semioExtensionStoreVitePlugin } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🏪️store/📜️store.ts";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "../..");
const repoRoot = path.resolve(playDir, "../../../../..");
const pluginModulesDir = path.join(playDir, "🔌️plugin-modules");
const installedExtensionsDir = defaultExtensionInstallRoot(repoRoot);
const rendererModulesDir = path.join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/📺️renderer-modules");
const renderer = process.env.SEMIO_RENDERER ?? "react";
const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT;
const brandId = process.env.SEMIO_BRAND ?? PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin || target.aliases.includes(plugin))?.brand;
const brand = resolveShellBrandById(brandId);

//#region 🔖️RegistryDrivenAssetsAndEngines
/** @emoji 🔌️ Framework engine crates every react-renderer dev session needs regardless of the active
 * plugin (the node-graph/editor host engines back shared studio chrome, not any one app) — kept as a
 * literal baseline rather than per-plugin metadata, mirroring the equally-unconditional pre-registry
 * build in `os/dev/script.ts`'s `buildEngineWasm`. */
const FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE = ["@semio-tech/framework-surface-node-graph-rs", "@semio-tech/framework-surface-board-2d-rs", "@semio-tech/framework-editor-rs", "@semio-tech/flow-core"];

/** @emoji 📦️ Maps a registry `engines` crate path (e.g. `framework/module/surface/tiled-map/rs`) to its wasm-pack
 * npm package name — read from the crate's own sibling `package.json`, not derived from its path, so a
 * crate keeps optimizing correctly across restructures/moves without touching this file. */
function engineNpmPackage(cratePath: string): string {
  const direct = path.join(repoRoot, cratePath, "package.json");
  const nested = path.join(repoRoot, cratePath, "pkg", "package.json");
  const manifestPath = existsSync(direct) ? direct : nested;
  const name = JSON.parse(readFileSync(manifestPath, "utf8")).name as string | undefined;
  if (!name) throw new Error(`missing "name" in ${manifestPath}`);
  return name;
}

const registryEngineOptimizeDepsExclude = [...new Set(PLAYGROUND_BUILD_TARGETS.flatMap((target) => target.engines))].map(engineNpmPackage);

/** @emoji 🗄️ Isolates dependency-optimizer state for concurrent playground variants and renderers. */
const playgroundCacheDir = path.join(repoRoot, "node_modules/.vite-os-dev", `${plugin}-${renderer}`);

/** @emoji 🚫️ Keeps Node-only browser automation packages outside Vite's browser dependency optimizer. */
const nodeOnlyOptimizeDepsExclude = ["playwright", "playwright-core", "chromium-bidi", "fsevents"];

/** @emoji 🗂️ The active playground's declared asset needs — every playground's assets when unfiltered
 * (the "s" studio hub can open any app, so it needs every app's dev-time asset routes available), else
 * just the resolved variant's own `assets` row. */
const resolvedPlaygroundAssets = isHostPluginFilter(plugin) ? PLAYGROUND_BUILD_TARGETS.flatMap((target) => target.assets) : (PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin)?.assets ?? []);

/** @emoji 🔌️ The wasm plugin crate(s) a production build's `dist/plugin-modules/` needs to actually ship
 * — the "s" studio hub can open any app so it needs every built plugin crate; a single-variant build
 * (e.g. the Aggregator's "aggregator" → `puzzle`) needs only its own, plus the shared `_vendor` shim
 * deps every plugin's `🟨️host-shim.js` imports. Falls back to "every crate" for an unresolved/unknown
 * filter rather than shipping nothing. */
const resolvedPluginId = PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin || target.aliases.includes(plugin))?.pluginId;
const pluginModuleDirNames = isHostPluginFilter(plugin) || !resolvedPluginId ? undefined : ["_vendor", resolvedPluginId];
//#endregion 🔖️RegistryDrivenAssetsAndEngines

export default defineConfig({
  root: playDir,
  cacheDir: playgroundCacheDir,
  publicDir: path.join(playDir, "public"),
  assetsInclude: ["**/*.wasm"],
  // 🏷️ A brand's own `distDir` (e.g. the Aggregator's `♻️/aggregator/dist`) keeps its build output
  // self-contained alongside its brand config/assets instead of the shared playground `dist/`.
  build: {
    ...semioViteProductionBuild(),
    ...(brand?.distDir ? { outDir: path.join(repoRoot, brand.distDir) } : {}),
  },
  resolve: {
    alias: [
      ...playgroundSceneHostResolveAliases(repoRoot),
      { find: "@semio-tech/ui-react", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx") },
      { find: "@semio-tech/assets", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/ui-styling", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️glue.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️glue.tsx") },
      { find: "@semio-tech/framework-renderer-react", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx") },
      { find: "@semio-tech/framework-renderer-wgpu", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️index.ts") },
      { find: "@semio-tech/framework", replacement: path.resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts") },
      { find: "@semio-tech/framework-os", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts") },
      { find: "@semio-tech/framework-surface-board-2d-rs", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/pkg") },
      { find: "/plugin-modules", replacement: pluginModulesDir },
      { find: "/extensions", replacement: installedExtensionsDir },
      { find: "/renderer-modules", replacement: rendererModulesDir },
    ],
    dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
  },
  server: {
    port: Number(process.env.S_OS_PORT ?? 6066),
    strictPort: true,
    fs: { allow: [repoRoot, pluginModulesDir, installedExtensionsDir, rendererModulesDir] },
    watch: {
      // Generated registry rewrites must not bounce Vite (write playgrounds.ts → restart → rewrite…).
      ignored: ["**/📇️registry/🤖️generated/**", "**/🤖️generated/**", "**/.vscode/launch.json"],
    },
  },
  plugins: [
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "semio · os",
      // 🌐️ MUST be root-relative (`/…`), not `./…`: `semioHostHtmlString` renders this into a `<script
      // src>` on every request via `transformIndexHtml` (`🟦️vite-elements-assets.ts`), including SPA
      // deep-link fallbacks like `/spaces/{id}` — a `./`-relative entry resolves against the CURRENT
      // path there, 404ing on any nested route (26/08/16 HUB-SPACES lane 4-I: this is why user2's hard
      // navigation to `/spaces/{id}` never rendered — the browser requested `/spaces/🟦️component.ts`).
      entry: "/🟦️component.ts",
    }),
    semioEmojiIndexHtmlVitePlugin(playDir),
    playgroundFlowWasmDevStubPlugin(repoRoot),
    semioBackboneVitePlugin(),
    semioBlobVitePlugin(),
    semioPluginHotSwapVitePlugin(),
    semioExtensionStoreVitePlugin({ installRoot: installedExtensionsDir, repoRoot }),
    ...semioAssetsVitePlugin(repoRoot),
    // 🔌️ `resolve.alias`'s `/plugin-modules` entry above only covers *bundler* resolution (static imports
    // Vite can inline) — the shell also fetches wasm plugin modules at runtime via plain absolute-URL
    // `import()`s, which a production build never bundles. Without an explicit static-dir copy, a
    // production `dist/` simply never contains `plugin-modules/` at all, so every wasm plugin 404s once
    // deployed (only ever worked in dev, where the dev server can serve arbitrary filesystem paths).
    // Copied per-crate-dir rather than the whole tree so a single-variant build doesn't ship every plugin
    // crate this shared dev directory happens to have accumulated from other variants' past builds — see
    // `pluginModuleDirNames` above. (`renderer-modules/wgpu` needs no equivalent copy: it's a wholly
    // separate wgpu-renderer trunk build — `BuildScript.run` skips `vite build` entirely for that
    // renderer — and nothing in the react app ever fetches `/renderer-modules` at runtime.)
    ...(pluginModuleDirNames
      ? pluginModuleDirNames.flatMap((name) => staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/plugin-modules/${name}`, root: path.relative(repoRoot, path.join(pluginModulesDir, name)) }))
      : staticDirVitePlugin(repoRoot, { kind: "static-dir", route: "/plugin-modules", root: path.relative(repoRoot, pluginModulesDir) })),
    staticDirVitePlugin(repoRoot, { kind: "static-dir", route: "/extensions", root: path.relative(repoRoot, installedExtensionsDir) }),
    // 🏷️ A brand's own static assets (e.g. the Aggregator's funding/partner logos) mount at `/<assetsDir>`
    // alongside the shared `framework/ui/asset` mount above.
    ...(brand?.assetsDir ? staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/${brand.assetsDir}`, root: brand.assetsDir }) : []),
    ...semioBrandHtmlVitePlugins(repoRoot, brand),
    ...playgroundAssetVitePlugins(repoRoot, resolvedPlaygroundAssets, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)),
    ...(renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()]),
  ],
  optimizeDeps: {
    entries: [path.join(playDir, "🌐️index.html")],
    include: ["react-reconciler", "react-reconciler/constants", "three", "@react-three/fiber", "fuse.js"],
    exclude: [...nodeOnlyOptimizeDepsExclude, ...(renderer === "wgpu" ? ["@semio-tech/framework-renderer-react"] : []), ...FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE, ...registryEngineOptimizeDepsExclude],
  },
  define: {
    "import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? DEFAULT_HOST_VARIANT),
    "import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
    "import.meta.env.VITE_SEMIO_BRAND": JSON.stringify(brand?.id ?? ""),
    // 👥️ Hub identity passthrough for collaborative dev sessions (contract freeze §C0/§C3) — unset for
    // the plain single-user `s` launcher, populated for the `s` `users` launchers (`S_HUB_URL`/`S_USER`/
    // `S_DATA_DIR` env, see `.vscode/🧩️launch.seed.jsonc`'s `devLaunchers.s.users`).
    "import.meta.env.VITE_S_HUB_URL": JSON.stringify(process.env.S_HUB_URL ?? ""),
    "import.meta.env.VITE_S_USER": JSON.stringify(process.env.S_USER ?? ""),
    "import.meta.env.VITE_S_DATA_DIR": JSON.stringify(process.env.S_DATA_DIR ?? ""),
  },
});

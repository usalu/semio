import {readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioBrandHtmlVitePlugins, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, semioViteProductionBuild, staticDirVitePlugin, semioAssetsVitePlugin } from "../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { DEFAULT_HOST_VARIANT, PLAYGROUND_BUILD_TARGETS } from "../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { isHostPlaygroundFilter } from "../../../🔌️plugin/📇️registry/🟦️.ts";
import { resolveShellBrandById } from "../../🏷️brand/🟦️.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin, semioDescriptorRouteGuardVitePlugin, semioActivationVitePlugin, semioProductionTestBoundaryVitePlugin, semioSourceWatchVitePlugin } from "../../🔌️vite-plugins/🟦️.ts";
import { semioExtensionStoreVitePlugin } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts";
import { developmentRuntimeRoot, playgroundSessionViteAlias, pluginModulesRoot, readActivationReceipt } from "../../♻️activation/🟦️.ts";
import { resolveTestBrowserHostRootsV1 } from "../../♻️activation/🌐️browser-host/🟦️.ts";
import { productionBrowserArtifactsVitePlugin, selectProductionBrowserComponents } from "../../🚚️distribution/🔌️components/🟦️.ts";
import { DISTRIBUTION_LAYOUT, distributionChunkName, distributionAssetName } from "../../🚚️distribution/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "../..");
const repoRoot = path.resolve(playDir, "../../../../..");
const rendererModulesDir = path.join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/📺️renderer-modules");
export default defineConfig(async ({ command }) => {
const renderer = process.env.SEMIO_RENDERER ?? "react";
const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT;
const profile = command === "build" || process.env.SEMIO_BUILD_MODE === "ship" ? "release" : "dev";
const runtimeRoot = developmentRuntimeRoot(configDir, plugin, profile);
const testBrowserHost = command === "serve" ? resolveTestBrowserHostRootsV1(process.env) : undefined;
const receiptDirectory = testBrowserHost?.activationRoot ?? path.join(runtimeRoot, "activation");
const activated = command === "serve" ? readActivationReceipt(receiptDirectory) : undefined;
const pluginModulesDir = testBrowserHost?.moduleRoot ?? pluginModulesRoot(profile);
const installedExtensionsDir = command === "build" ? pluginModulesDir : testBrowserHost ? path.join(testBrowserHost.browserHostRoot, "extensions") : path.join(runtimeRoot, "extensions");
const fontsDir = path.resolve(configDir, "../../../♾️infinite/📦️packages/🦀️rust/dist/fonts");
const sessionRoot = path.resolve(configDir, "../../../🔌️plugin/📇️registry/dist/sessions");
const sessionAlias = playgroundSessionViteAlias(sessionRoot, plugin);
const sessionPath = sessionAlias.replacement;
const brandId = process.env.SEMIO_BRAND ?? PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin || target.aliases.includes(plugin))?.brand;
const brand = resolveShellBrandById(brandId);
const distributionSource = (source: string) => source === "\0vite/preload-helper.js" ? "virtual/vite/preload-helper.js" : path.relative(repoRoot, path.resolve(playDir, source)).replaceAll("\\", "/");
const distributionRollupOutput = {
  entryFileNames: (chunk: { facadeModuleId: string | null; moduleIds: string[] }) => distributionChunkName(DISTRIBUTION_LAYOUT, { facadeModuleId: chunk.facadeModuleId === null ? null : distributionSource(chunk.facadeModuleId), moduleIds: chunk.moduleIds.map(distributionSource) }),
  chunkFileNames: (chunk: { facadeModuleId: string | null; moduleIds: string[] }) => distributionChunkName(DISTRIBUTION_LAYOUT, { facadeModuleId: chunk.facadeModuleId === null ? null : distributionSource(chunk.facadeModuleId), moduleIds: chunk.moduleIds.map(distributionSource) }),
  assetFileNames: (asset: { originalFileNames: string[]; names: string[] }) => distributionAssetName(DISTRIBUTION_LAYOUT, { originalFileNames: asset.originalFileNames.map(distributionSource), names: asset.names }),
};

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

const registryEngineOptimizeDepsExclude = [...new Set((isHostPlaygroundFilter(plugin) ? PLAYGROUND_BUILD_TARGETS : PLAYGROUND_BUILD_TARGETS.filter((target) => target.variant === plugin)).flatMap((target) => target.engines))].map(engineNpmPackage);

/** @emoji 🗄️ Isolates dependency-optimizer state for concurrent playground variants, renderers and
 * profiles under the ONE shared cache root, so disk is bounded by build history rather than by
 * `node_modules`. The profile belongs in the key: a `dev` and a `release` serve of the same variant run
 * side by side, and sharing one `deps/` directory means whichever re-optimizes last rewrites the modules
 * the other has already handed to a browser. */
const playgroundCacheDir = repoCacheDirectory(repoRoot, "vite", "os-dev", `${plugin}-${renderer}-${profile}`);

/** @emoji 🚫️ Keeps Node-only browser automation packages outside Vite's browser dependency optimizer. */
const nodeOnlyOptimizeDepsExclude = ["playwright", "playwright-core", "chromium-bidi", "fsevents"];

/** @emoji 🗂️ The active playground's declared asset needs — every playground's assets when unfiltered
 * (the "s" studio hub can open any app, so it needs every app's dev-time asset routes available), else
 * just the resolved variant's own `assets` row. */
const resolvedPlaygroundAssets = isHostPlaygroundFilter(plugin) ? PLAYGROUND_BUILD_TARGETS.flatMap((target) => target.assets) : (PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin)?.assets ?? []);

/** @emoji 🔌️ The wasm plugin crate(s) a production build's `dist/🔌️plugin-modules/` needs to actually ship
 * — the "s" studio hub can open any app so it needs every built plugin crate; a single-variant build
 * (e.g. the Aggregator's "aggregator" → `puzzle`) needs only its own, plus the shared `🪞️vendor` shim
 * dependencies. Unknown identities are rejected before selecting physical copy roots. */
const resolvedPluginId = PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin || target.aliases.includes(plugin))?.pluginId;
// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (wgpu-web-shard): `🧵️shard` is `🟦️.ts`'s
// generated `🟨️shard-worker.js` bundle — every actor of every plugin now activates through the ONE pooled
// shard-worker pool (`ShardClient`/`ActivationRegistry`, design-runtime.md §1/§3), not a per-plugin
// worker, so a single-variant production build needs this directory copied regardless of which plugin
// `resolvedPluginId` names. Omitting it meant `dist/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js` was never
// copied and every single-variant production build 404s the shard worker at first plugin activation.
if (!resolvedPluginId) throw new Error(`Unknown playground module identity: ${plugin}`);
const extensionIds = new Set(EXTENSION_TARGETS.map((target) => target.pluginId));
const productionComponents = command === "build" ? selectProductionBrowserComponents((await import(pathToFileURL(sessionPath).href)).PLAYGROUND_SESSION, plugin, resolvedPluginId, [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS]) : undefined;
const pluginModuleDirNames = [MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, ...(activated?.plugins ?? []).filter((row) => !extensionIds.has(row.pluginId)).map((row) => moduleDirectoryName(row.pluginId))];

/** @emoji 🔎️ The components the activation-receipt watcher checks for staleness — every declared build
 * target, with the owner tree whose newest source mtime decides whether the staged module is behind
 * (`<cratePath>/../..`, the same owner root `stagePluginDescriptor` publishes descriptors from). */
const activationComponents = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map((target) => ({
  pluginId: target.pluginId,
  directoryName: moduleDirectoryName(target.pluginId),
  role: target.role === "extension" ? ("extension" as const) : ("plugin" as const),
  sourceRoot: path.resolve(repoRoot, target.cratePath, "..", ".."),
}));
//#endregion 🔖️RegistryDrivenAssetsAndEngines

return {
  root: playDir,
  cacheDir: playgroundCacheDir,
  publicDir: path.join(playDir, "public"),
  assetsInclude: ["**/*.wasm"],
  worker: { format: "es", plugins: () => [semioProductionTestBoundaryVitePlugin()], rollupOptions: { output: distributionRollupOutput } },
  // 🏷️ A brand's own `distDir` (e.g. the Aggregator's `♻️/aggregator/dist`) keeps its build output
  // self-contained alongside its brand config/assets instead of the shared playground `dist/`.
  build: {
    ...semioViteProductionBuild(),
    outDir: path.join(playDir, DISTRIBUTION_LAYOUT.directory),
    assetsDir: DISTRIBUTION_LAYOUT.bundles,
    emptyOutDir: false,
    rollupOptions: { output: distributionRollupOutput },
    ...(brand?.distDir ? { outDir: path.join(repoRoot, brand.distDir) } : {}),
  },
  resolve: {
    alias: [
      sessionAlias,
      ...playgroundSceneHostResolveAliases(repoRoot),
      { find: "@semio-tech/ui-react/test", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts") },
      { find: "@semio-tech/ui-react/runtime", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎠️runtime/🟦️.ts") },
      { find: "@semio-tech/ui-react", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/assets", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/ui-styling", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/framework-renderer-react", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/framework-renderer-wgpu", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📚️library/🟦️.ts") },
      { find: "@semio-tech/framework", replacement: path.resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/framework-os", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/framework-surface-board-2d-rs", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings") },
      { find: MODULE_PLUGIN_ROUTE, replacement: pluginModulesDir },
      { find: MODULE_EXTENSION_ROUTE, replacement: installedExtensionsDir },
      { find: "/renderer-modules", replacement: rendererModulesDir },
    ],
    dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
  },
  server: {
    host: "127.0.0.1",
    port: Number(process.env.S_OS_PORT ?? 6066),
    strictPort: true,
    ...(process.env.SEMIO_VITE_HMR === "0" ? { hmr: false } : {}),
    ...(process.env.S_LOCAL_RELAY_URL ? {
      proxy: {
        "/_semio": {
          target: process.env.S_LOCAL_RELAY_URL,
          changeOrigin: false,
          headers: process.env.S_LOCAL_RELAY_SECRET ? { "x-semio-local-relay": process.env.S_LOCAL_RELAY_SECRET } : undefined,
        },
      },
    } : {}),
    fs: { allow: [repoRoot, pluginModulesDir, installedExtensionsDir, rendererModulesDir] },
    // 👁️ `semioSourceWatchVitePlugin` owns file watching (see its docstring): Vite's own chokidar
    // watcher watches `root` plus every module-graph file outside it, which on macOS consolidates into
    // ONE FSEvents stream over the whole repository and then pays `events × watched paths` per event —
    // a concurrent cargo build in the shared cache wedges the server. `server.watch.ignored` cannot fix
    // that (chokidar reads it only after those per-path filters have run), so there is no watcher here
    // to configure; the replacement watches the source roots and never sees cache writes at all.
    watch: null,
  },
  plugins: [
    semioProductionTestBoundaryVitePlugin(),
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "semio · os",
      // 🌐️ MUST be root-relative (`/…`), not `./…`: `semioHostHtmlString` renders this into a `<script
      // src>` on every request via `transformIndexHtml` (`🟦️.ts`), including SPA
      // deep-link fallbacks like `/spaces/{id}` — a `./`-relative entry resolves against the CURRENT
      // path there, 404ing on any nested route (26/08/16 HUB-SPACES lane 4-I: this is why user2's hard
      // navigation to `/spaces/{id}` never rendered — the browser requested `/spaces/🟦️.ts`).
      entry: "/🟦️.ts",
    }),
    semioEmojiIndexHtmlVitePlugin(playDir),
    playgroundFlowWasmDevStubPlugin(repoRoot),
    semioDescriptorRouteGuardVitePlugin([
      { route: MODULE_PLUGIN_ROUTE, root: pluginModulesDir, directoryNames: new Set(PLUGIN_BUILD_TARGETS.filter((target) => target.role === "plugin").map((target) => moduleDirectoryName(target.pluginId))) },
      { route: MODULE_EXTENSION_ROUTE, root: installedExtensionsDir, directoryNames: new Set(EXTENSION_TARGETS.map((target) => moduleDirectoryName(target.pluginId))) },
    ]),
    semioBackboneVitePlugin(),
    semioBlobVitePlugin(),
    ...(command === "serve" ? [semioSourceWatchVitePlugin({ repoRoot }), semioActivationVitePlugin({ receiptDirectory, moduleRoot: pluginModulesDir, installRoot: installedExtensionsDir, components: activationComponents }), semioExtensionStoreVitePlugin({ installRoot: installedExtensionsDir, repoRoot })] : []),
    ...semioAssetsVitePlugin(repoRoot),
    ...(productionComponents ? [productionBrowserArtifactsVitePlugin(repoRoot, productionComponents)] : [
      ...pluginModuleDirNames.flatMap((name) => staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `${MODULE_PLUGIN_ROUTE}/${name}`, root: path.relative(repoRoot, path.join(pluginModulesDir, name)) })),
      staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `${MODULE_PLUGIN_ROUTE}/${MODULE_VENDOR_DIRECTORY}`, root: path.relative(repoRoot, fontsDir) }),
      staticDirVitePlugin(repoRoot, { kind: "static-dir", route: MODULE_EXTENSION_ROUTE, root: path.relative(repoRoot, installedExtensionsDir) }),
    ]),
    // 🏷️ A brand's own static assets (e.g. the Aggregator's funding/partner logos) mount at `/<assetsDir>`
    // alongside the shared `framework/ui/asset` mount above.
    ...(brand?.assetsDir ? staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/${brand.assetsDir}`, root: brand.assetsDir }) : []),
    ...semioBrandHtmlVitePlugins(repoRoot, brand),
    ...playgroundAssetVitePlugins(repoRoot, resolvedPlaygroundAssets, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)),
    ...(renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()]),
  ],
  optimizeDeps: {
    entries: [path.join(playDir, "🌐️.html")],
    include: ["three", "@react-three/fiber"],
    exclude: [...nodeOnlyOptimizeDepsExclude, ...(renderer === "wgpu" ? ["@semio-tech/framework-renderer-react"] : []), ...FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE, ...registryEngineOptimizeDepsExclude],
  },
  define: {
    "import.meta.vitest": "undefined",
    "import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? DEFAULT_HOST_VARIANT),
    "import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
    "import.meta.env.VITE_SEMIO_BRAND": JSON.stringify(brand?.id ?? ""),
    // 👥️ Non-secret collaborative endpoint metadata; authority stays inside the local relay.
    "import.meta.env.VITE_S_HUB_URL": JSON.stringify(process.env.S_HUB_URL ?? ""),
    "import.meta.env.VITE_S_DATA_DIR": JSON.stringify(process.env.S_DATA_DIR ?? ""),
  },
};
});

/** 🧩️ Semantic plugin build plan owner. */

import { ACTIVATION_RECEIPT_FILE, PLAYGROUND_SESSION_OUTPUT_ROOT_ENV, developmentRuntimeRoot, newestComponentSourceMtime, nextActivationReceipt, playgroundSessionOutputPath, pluginModulesRoot, publishActivationReceipt, readActivationReceipt, stagedModuleMtime, stagedModuleReportLines, stagedModuleVerdict, type StagedModuleFacts, type StagedModuleVerdict } from "../../../🧑‍💻dev/♻️activation/🟦️.ts";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { isHostPlaygroundFilter } from "../../📇️registry/🟦️.ts";

import type { PluginRegistryEntry } from "../../📇️registry/🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();



/** @emoji 🎚️ The staging profile this process produces and serves — `ship` builds the release tree, every
 * other mode the dev tree. Paired with {@link pluginWasmProfile}, which selects the matching cargo profile. */
function devStagingProfile(): "dev" | "release" { return semioBuildMode() === "ship" ? "release" : "dev"; }

const pluginOutRoot = pluginModulesRoot(devStagingProfile());

/** @emoji 🧊️ The one wgpu renderer package the dev router delegates `serve`/`wasm`/`native` to — a single
 * constant so the ship, dev and bench call sites can never drift onto different (or extinct) paths. */
const WGPU_PACKAGE_ROOT = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust");

const WGPU_SCRIPT_PATH = join(WGPU_PACKAGE_ROOT, "📜️script.ts");

const PLUGIN_WASM_TARGET = "wasm32-wasip2";

const PLUGIN_WASM_STACK_BYTES = 8 * 1024 * 1024;

/** @emoji 🎭️ Playwright is a TEST-ONLY dependency, loaded lazily by the harness commands below. The
 * specifier is held in a constant so no bundler can statically resolve it: `⚙️vite.config.ts` imports
 * this module for its Vite plugins, and a literal `import("playwright")` makes bun follow the dynamic
 * import while LOADING THE CONFIG — dragging `playwright-core` into a browser build and failing on its
 * uninstalled optional `chromium-bidi`. Runtime behaviour is identical. */
const PLAYWRIGHT_MODULE_SPECIFIER = "playwright";

/** @emoji 🎯 Ensures the wasip2 rustc target is installed for plugin component builds. */
function ensureWasmTarget(): void {
  const probe = runProbe("rustup", ["target", "list", "--installed"]);
  if (!probe.stdout.includes(PLUGIN_WASM_TARGET)) {
    runCmd("rustup", ["target", "add", PLUGIN_WASM_TARGET]);
  }
}

/** 🪶️ Uses only WASI link profiles; arbitrary native-profile overrides fail closed. */
function pluginWasmProfile(mode = semioBuildMode(), override: string | null = process.env.SEMIO_PLUGIN_PROFILE ?? null): "wasm-dev" | "wasm-release" {
  return selectComponentWasmProfile(mode, override ?? undefined);
}

function pluginCargoArgs(packageName: string, profile: string): string[] {
  selectComponentWasmProfile("dev", profile);
  const args = ["rustc", "-p", packageName, "--target", PLUGIN_WASM_TARGET, "--profile", profile, "--", "-C", `link-arg=-zstack-size=${PLUGIN_WASM_STACK_BYTES}`];
  if (process.env.SEMIO_PLUGIN_SYMBOLS === "1") args.push("-C", "strip=none");
  return args;
}

function resolvePluginBuildTargets(entries: readonly PluginRegistryEntry[], filterPlugin?: string): readonly PluginRegistryEntry[] {
  const only = process.env.SEMIO_PLUGIN_ONLY?.trim();
  if (only) {
    const matched = entries.filter((entry) => entry.pluginId === only);
    if (matched.length === 0) throw new Error(`SEMIO_PLUGIN_ONLY=${JSON.stringify(only)} matched no plugin crates`);
    return matched;
  }
  if (!filterPlugin || isHostPlaygroundFilter(filterPlugin)) return entries;
  if (entries.length === 0) throw new Error(`no program build targets for filter ${JSON.stringify(filterPlugin)}`);
  return entries;
}

//#region 🔖️PlaygroundVariantResolution
/** @emoji 📚️ Generated playground catalog (variant -> crate pluginId + optional app id), loaded once for this process via `@semio-tech/repo-lib`'s `loadFrameworkOsPlaygroundCatalog` (backed by `framework/plugin/registry/generated/🎮️playgrounds/🟦️.ts`). */
const playgroundCatalog = loadFrameworkOsPlaygroundCatalog();

/** @emoji 🧭️ A resolved playground filter: the crate pluginId to build/load, plus the app id and shell brand id to inject when the filter matched a catalog variant row. */
type ResolvedPlaygroundFilter = {
  readonly pluginId: string;
  readonly appId?: string;
  readonly brand?: string;
};

/**
 * 🧭️ Resolves `filterPlugin` (a playground variant id like "puzzle5d", or already a bare crate
 * pluginId like "note") against the generated playground catalog: a matching variant row yields
 * its crate pluginId, app id, and brand id, otherwise `filterPlugin` is treated as already being a
 * bare pluginId (existing behavior for single-app crates where variant === pluginId).
 */
function resolvePlaygroundFilter(filterPlugin: string): ResolvedPlaygroundFilter {
  const row = playgroundCatalog.find((entry) => entry.variant === filterPlugin);
  return row ? { pluginId: row.pluginId, appId: row.app, brand: row.brand } : { pluginId: filterPlugin };
}

/** @emoji 🎯️ Resolves a raw filter to the crate pluginId `generatePluginRegistry`'s `filterPlaygroundPlugin` option expects, or `undefined` for the unfiltered/studio case. */
function resolveCatalogFilterPluginId(filterPlugin?: string): string | undefined {
  return filterPlugin && !isHostPlaygroundFilter(filterPlugin) ? resolvePlaygroundFilter(filterPlugin).pluginId : undefined;
}

export { PLAYWRIGHT_MODULE_SPECIFIER, PLUGIN_WASM_STACK_BYTES, PLUGIN_WASM_TARGET, ResolvedPlaygroundFilter, WGPU_PACKAGE_ROOT, WGPU_SCRIPT_PATH, devStagingProfile, ensureWasmTarget, playgroundCatalog, pluginCargoArgs, pluginOutRoot, pluginWasmProfile, resolveCatalogFilterPluginId, resolvePlaygroundFilter, resolvePluginBuildTargets };

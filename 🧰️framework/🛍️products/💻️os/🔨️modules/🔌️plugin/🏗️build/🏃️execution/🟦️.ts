/** 🧩️ Semantic plugin build execution owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

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

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../../📇️registry/📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../📇️registry/🔎️discovery/🟦️.ts";

import { isHostPlaygroundFilter } from "../../📇️registry/🟦️.ts";

import { DEFAULT_HOST_VARIANT } from "../../📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { ensureWasmTarget, pluginOutRoot, resolveCatalogFilterPluginId, resolvePlaygroundFilter, resolvePluginBuildTargets } from "../📋️plan/🟦️.ts";

import { assertExtensionOutputsFresh, syncBuiltExtensionsToInstallRoot } from "../📥️installation/🟦️.ts";

import { buildPluginCargo, ensureGuestSlimTypstFontsAsset, ensurePreview2ShimVendor, materializePlugin, publishShardWorker, rewriteExistingPluginShimImports } from "../📦️materialization/🟦️.ts";

import { assertPluginCatalogComplete, buildPluginCatalog, ensurePluginRegistry } from "../../📇️registry/🔄️refresh/🟦️.ts";

import { ensureAppleDeveloperDir } from "../../../🧑‍💻dev/⚙️engine/📤️publication/🟦️.ts";



/** @emoji 🎯️ Builds exactly one target end to end (cargo then materialize then the shared shard-worker
 * publish) — used where only one crate is being built at a time, so there is no concurrency to bound:
 * the file-watch rebuild loop (`watchPluginRebuilds`, which deliberately serializes overlapping rebuild
 * requests onto the SAME `target/` cargo lock) and the two-crate collab-e2e prebuild. The full-catalog
 * entry points (`buildPlugins`/`buildPluginsStreaming`) go through `buildPluginCatalog` instead, which
 * pipelines this same pair of stages across many targets. */
async function buildPlugin(target: PluginRegistryEntry): Promise<void> {
  const { artifact } = await buildPluginCargo(target);
  await materializePlugin(target, artifact);
  publishShardWorker();
}

/** @emoji 🧵️ Minimal counting semaphore bounding how many `fn()` calls run concurrently. Local to this
 * file rather than promoted to the shared repo-lib — this packet's ownership (`📌️important.md`
 * registrar-only list) is scoped to `📜️script.ts` and `🟦️.ts` only. FIFO wakeup,
 * never reorders which caller gets the next free slot. */
function createConcurrencyLimiter(limit: number): { run: <T>(fn: () => Promise<T>) => Promise<T> } {
  let active = 0;
  const queue: Array<() => void> = [];
  async function acquire(): Promise<void> {
    if (active < limit) {
      active++;
      return;
    }
    await new Promise<void>((wake) => queue.push(wake));
    active++;
  }
  function release(): void {
    active--;
    queue.shift()?.();
  }
  return {
    async run<T>(fn: () => Promise<T>): Promise<T> {
      await acquire();
      try {
        return await fn();
      } finally {
        release();
      }
    },
  };
}

/** @emoji 🧵️ Concurrency cap for the MATERIALIZE stage only (see `buildPluginCatalog`) — jco transpile
 * and `wasm-opt` are each single-process, mostly-single-threaded-per-invocation CPU-bound subprocesses,
 * and each holds a decoded wasm module plus jco's own intermediate JS AST in memory while running. 4 is
 * a deliberately small constant, not tied to `hardwareConcurrency`: unlike the cargo stage (one process
 * for the whole build, sharing rustc's own parallelism internally), materialize concurrency is
 * ~N-processes-at-once, and an unbounded `Promise.all` over a ~20-58-plugin catalog risks the same class
 * of machine-saturation `📌️important.md` records for parallel cargo (174 concurrent processes, 40
 * minutes, nothing produced) — just with jco/wasm-opt instead of rustc. `SEMIO_MATERIALIZE_CONCURRENCY`
 * overrides it for measurement/tuning. */
function materializeConcurrencyLimit(): number {
  const override = process.env.SEMIO_MATERIALIZE_CONCURRENCY;
  if (override) {
    const parsed = Number.parseInt(override, 10);
    if (Number.isFinite(parsed) && parsed > 0) return parsed;
  }
  return 4;
}

/** 🛡️Rejects stale public output without deleting another task's retained files. */
function assertNoStalePublicPluginOutputs(path: string): void {
  if (existsSync(path)) throw new Error(`Unexpected public plugin output preserved at ${path}; inspect its owner before running @semio-tech/framework-os-dev:plugin.`);
}

/** @emoji 🎯️ Shared setup for every plugin-build entry point below: registry regeneration, output dirs,
 * vendor shims, read-only stale-output checks, and the resolved+logged target list — everything a build needs
 * that isn't itself a `cargo build`. Split out of the old monolithic `buildPlugins` so the dev runner's
 * streaming variant can run this fast (no-cargo) prep synchronously before Vite starts, then stream the
 * slow per-crate builds in afterward instead of blocking the first byte on all of them. */
async function preparePluginBuildTargets(filterPlugin?: string): Promise<readonly PluginRegistryEntry[]> {
  ensureWasmTarget();
  await ensurePluginRegistry(filterPlugin);
  const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
  const catalogEntries = filterProjectedPluginRegistry(readGeneratedCatalogProjection(), filterPluginId);
  mkdirSync(pluginOutRoot, { recursive: true });
  ensurePreview2ShimVendor();
  ensureGuestSlimTypstFontsAsset();
  rewriteExistingPluginShimImports();
  const stalePublicPlugins = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/public/plugin-modules");
  assertNoStalePublicPluginOutputs(stalePublicPlugins);
  assertExtensionOutputsFresh();
  const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin);
  syncBuiltExtensionsToInstallRoot(targets);
  if (filterPlugin && !isHostPlaygroundFilter(filterPlugin)) {
    console.log(`program build scope: ${targets.map((target) => target.pluginId).join(", ")}`);
  } else {
    console.log(`program build scope: all (${targets.length} plugin crates)`);
  }
  return targets;
}

/** @emoji 🎪️ Exported so a multi-variant host (e.g. the mit-bestand demonstrator, which needs every one
 * of its six panes' plugin crates built into the SAME shared `🔌️plugin-modules/` dir rather than one
 * variant's own isolated dev/build) can call this directly per variant instead of shelling out to this
 * script's own CLI once per variant.
 *
 * Every target is attempted before the summary reports failures. Unlike the streaming dev build,
 * an incomplete explicit build rejects so callers cannot mistake stale artifacts for fresh outputs. */
export async function buildPlugins(filterPlugin?: string): Promise<void> {
  ensureAppleDeveloperDir();
  const targets = await preparePluginBuildTargets(filterPlugin);
  const { failedPluginIds } = await buildPluginCatalog(targets);
  const builtCount = targets.length - failedPluginIds.length;
  console.log(`plugin catalog build summary: ${builtCount}/${targets.length} crate(s) produced .wasm`);
  if (failedPluginIds.length > 0) {
    console.log(`plugin catalog build failures (${failedPluginIds.length}): ${failedPluginIds.join(", ")}`);
  }
  assertPluginCatalogComplete(failedPluginIds);
}

/** @emoji 🌊️ Host-plugin-first, best-effort variant of `buildPlugins` for the dev runner's streaming
 * boot (`DevScript`, react renderer only): the shell's boot effect gates only on the host/primary
 * plugin (see os-core's `hostConfig` path), so building it first (and cargo-building it before any
 * other crate) gets the shell out of its "waiting for host program" state fastest — every other crate
 * streams in afterward via the `♻️hot-swap.json`/SSE channel, in whatever order the registry lists them.
 * `buildPluginCatalog` keeps the cargo stage itself serial (concurrent `cargo build`s just contend on
 * the shared `target/` lock) but overlaps each target's MATERIALIZE stage with the next target's cargo
 * build (T-P8) — a single broken crate no longer aborts the rest of the catalog either way. */
export async function buildPluginsStreaming(filterPlugin?: string): Promise<void> {
  const targets = await preparePluginBuildTargets(filterPlugin);
  const hostPluginId = resolvePlaygroundFilter(filterPlugin ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT).pluginId;
  const ordered = [...targets].sort((a, b) => (a.pluginId === hostPluginId ? -1 : b.pluginId === hostPluginId ? 1 : 0));
  await buildPluginCatalog(ordered);
}

class PluginBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
  }
}

export { PluginBuildScript, assertNoStalePublicPluginOutputs, buildPlugin, createConcurrencyLimiter, materializeConcurrencyLimit, preparePluginBuildTargets };

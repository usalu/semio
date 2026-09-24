/** 🧩️ Semantic activation execution owner. */

import { artifactFiles } from "../../../🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";

import { ACTIVATION_RECEIPT_FILE, PLAYGROUND_SESSION_OUTPUT_ROOT_ENV, developmentRuntimeRoot, healthyPreparedComponents, newestComponentSourceMtime, nextActivationReceipt, playgroundSessionOutputPath, pluginModulesRoot, preparedComponentReportLines, preparedComponentVerdict, publishActivationReceipt, readActivationReceipt, stagedModuleMtime, stagedModuleReportLines, readStagedSourceContentHash, writeStagedSourceContentHash, writeStagedSourceFreshness, stagedModuleVerdict, type StagedModuleFacts, type StagedModuleVerdict } from "../🟦️.ts";

import { FONT_ASSET, validateFontAsset } from "../../../♾️infinite/🖼️canvas/🔤️fonts/🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { fileURLToPath, pathToFileURL } from "node:url";

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

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";

import {
  ensurePreview2ShimVendorAt,
  hostShimSource,
  PLUGIN_HOST_SHIM_FILE,
  PREVIEW2_VENDOR_RELATIVE,
  GUESTSLIM_FONT_RELATIVE,
  pluginComponentBridgeSource,
  rewriteJcoComponentAssetUrls,
  SHARD_WORKER_FILE,
  shardWorkerSource,
  rewriteJcoAsyncResultLifting,
  rewritePreview2ShimImportSource,
  rewritePreview2ShimImports,
  transpilePluginComponentAsync,
  type PluginWebMaterializeContext,
} from "../../../🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { activationFilesDigest, publishActivatedExtension } from "../📥️installation/🟦️.ts";

import { stagedComponentFacts } from "../🔍️freshness/🟦️.ts";

import { playgroundCatalog } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";




class ActivationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, renderer, selectedProfile] = args, profile = selectedProfile as "dev" | "release";
    if (args.length !== 3 || !["react", "wgpu"].includes(renderer) || !["dev", "release"].includes(profile) || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant)) throw new Error("activate <variant> react|wgpu <dev|release>");
    const moduleRoot = pluginModulesRoot(profile);
    const sessionPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions", variant, "🎮️playground-session", "🟦️.ts");
    const session = (await import(pathToFileURL(sessionPath).href)).PLAYGROUND_SESSION;
    if (session.variant !== variant) throw new Error("Activation session identity mismatch");
    const runtime = developmentRuntimeRoot(this.root, variant, profile, renderer as "react" | "wgpu"), receiptRoot = join(runtime, "activation");
    const catalog = new Map(readGeneratedCatalogProjection().entries.map((entry) => [entry.pluginId, entry]));
    const controller = new AbortController(), cancel = (): void => controller.abort();
    for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, cancel);
    try {
      const support = new Map<string, string>();
      for (const directory of [PREVIEW2_VENDOR_RELATIVE, MODULE_SHARD_DIRECTORY]) for (const [name, path] of artifactFiles(join(moduleRoot, directory))) support.set(join(directory, name), path);
      support.set(FONT_ASSET, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts", FONT_ASSET));
      const supportDigest = await activationFilesDigest(support, controller.signal), completed = [];
      // 🩺️ HEALTHY SET: the receipt names what is ACTIVATED, so a component that never staged must be
      // absent from it rather than carry a digest of nothing — that absence is what makes the serve's
      // own freshness pass report it `unactivated` instead of the host pretending it is there.
      const verdicts = session.plugins.map((plugin: { readonly pluginId: string }) => preparedComponentVerdict(stagedComponentFacts(moduleRoot, plugin.pluginId)));
      const healthy = healthyPreparedComponents(verdicts, playgroundCatalog.find((row) => row.variant === variant)?.pluginId ?? variant);
      for (const line of preparedComponentReportLines(healthy.excluded, `bun nx run @semio-tech/framework-os-dev:activate-${variant}-${renderer}-${profile}`)) console.warn(line);
      if (healthy.refusal) throw new Error(healthy.refusal);
      // Content-hash warm path: when every prepared module already records the current
      // plugin-owner source hash and the prior receipt agrees, reuse the receipt without
      // re-digesting artifact bytes (warm `served` stays seconds, not a full republish).
      const previousWarm = existsSync(join(receiptRoot, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(receiptRoot) : undefined;
      if (previousWarm && previousWarm.variant === variant && previousWarm.profile === profile) {
        const prior = new Map(previousWarm.plugins.map((row) => [row.pluginId, row]));
        let warm = healthy.prepared.length > 0 && prior.size === healthy.prepared.length;
        for (const pluginId of healthy.prepared) {
          const moduleDirectory = join(moduleRoot, moduleDirectoryName(pluginId));
          const staged = readStagedSourceContentHash(moduleDirectory);
          const row = prior.get(pluginId);
          // Trust receipt ↔ staged-marker agreement; live source re-hash is the serve freshness pass.
          if (!row?.sourceContentSha256 || staged !== row.sourceContentSha256) { warm = false; break; }
        }
        if (warm) {
          console.log(`Activated ${variant} ${renderer} ${profile}: ${healthy.prepared.length} completed components (warm content-hash unchanged)`);
          return;
        }
      }
      for (const pluginId of healthy.prepared) {
        controller.signal.throwIfAborted();
        const moduleDirectory = join(moduleRoot, moduleDirectoryName(pluginId));
        const digest = await activationFilesDigest(artifactFiles(moduleDirectory), controller.signal);
        const catalogEntry = catalog.get(pluginId);
        const sourceRoot = catalogEntry ? join(repoRoot, catalogEntry.cratePath, "..", "..") : moduleDirectory;
        const sourceContentSha256 = writeStagedSourceFreshness(moduleDirectory, sourceRoot);
        completed.push({ pluginId, artifactSha256: createHash("sha256").update(supportDigest + digest).digest("hex"), sourceContentSha256 });
      }
      const previous = existsSync(join(receiptRoot, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(receiptRoot) : undefined;
      const receipt = nextActivationReceipt(variant, profile, completed, previous);
      for (const plugin of receipt.plugins) {
        controller.signal.throwIfAborted();
        const target = catalog.get(plugin.pluginId);
        if (!target) throw new Error(`Missing activation catalog entry: ${plugin.pluginId}`);
        if (target.role === "extension") await publishActivatedExtension(target, join(moduleRoot, moduleDirectoryName(plugin.pluginId)), join(runtime, "extensions"), plugin.artifactSha256, plugin.rebuiltAt, controller.signal);
      }
      controller.signal.throwIfAborted();
      const changed = publishActivationReceipt(receiptRoot, receipt);
      console.log(`Activated ${variant} ${renderer} ${profile}: ${receipt.plugins.length} completed components (${changed ? "changed" : "unchanged"})`);
    } finally { for (const signal of ["SIGINT", "SIGTERM"] as const) process.removeListener(signal, cancel); }
  }
}

/** @emoji ♻️ Brings one playground variant's `renderer` runtime up to a publishable activation state by
 * running the Nx target that OWNS that closure — `activate-<variant>-<renderer>-<profile>`, whose declared
 * `dependsOn` (`…🦑️repo/🔨️modules/📚️library/🟨️.mjs` `playgroundPreparationTargets`) is the single
 * source of truth for the chain: every selected plugin's `component-<profile>` → `materialize-<profile>`,
 * `@semio-tech/framework-plugin-web:support-<profile>`, `semio-framework-os-infinite:fonts`, the renderer's
 * own `wasm` producer(s), `@semio-tech/plugin-registry:session-<variant>`, then `prepare` and `activate`.
 * Delegating rather than re-listing that closure here is what makes "reuse whatever is already fresh"
 * Nx's cache decision instead of a second, drifting freshness rule — react and wgpu share this one path. */
async function activatePlaygroundRuntime(variant: string, profile: "dev" | "release", renderer: "react" | "wgpu" = "react"): Promise<void> {
  const target = `@semio-tech/framework-os-dev:activate-${variant}-${renderer}-${profile}`;
  console.log(`[dev] activating ${variant} ${renderer} ${profile} via ${target}`);
  if (runCmdStatus("bun", ["nx", "run", target], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error(`Playground activation failed: ${target}`);
}

export { ActivationScript, activatePlaygroundRuntime };

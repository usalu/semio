/** 🧩️ Semantic activation serve owner. */

import { ACTIVATION_RECEIPT_FILE, PLAYGROUND_SESSION_OUTPUT_ROOT_ENV, developmentRuntimeRoot, newestComponentSourceMtime, nextActivationReceipt, playgroundSessionOutputPath, pluginModulesRoot, publishActivationReceipt, readActivationReceipt, stagedModuleMtime, stagedModuleReportLines, stagedModuleVerdict, type StagedModuleFacts, type StagedModuleVerdict } from "../🟦️.ts";

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

import { playgroundCatalog, resolvePlaygroundFilter } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { reportServeStagedModuleFreshness } from "../🔍️freshness/🟦️.ts";



class ServeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, renderer, selectedProfile, ...serverArgs] = args;
    if (renderer !== "react" || !["dev", "release"].includes(selectedProfile)) throw new Error("serve <variant> react <dev|release> [server options]");
    const profile = selectedProfile as "dev" | "release", runtime = developmentRuntimeRoot(this.root, variant, profile, "react");
    const receipt = readActivationReceipt(join(runtime, "activation"));
    if (receipt.variant !== variant || receipt.profile !== profile) throw new Error("Server activation identity mismatch");
    reportServeStagedModuleFreshness(variant, "react", profile, runtime, receipt);
    const resolved = resolvePlaygroundFilter(variant);
    await runViteBunxDev(this.root, serverArgs, { config: "../../🏗️builder/🌐️vite/🟦️.ts", portEnv: "S_OS_PORT", defaultPort: String(frameworkOsPlaygroundDefaultPort(playgroundCatalog, variant, renderer)), fixedPort: true, env: { SEMIO_PLUGIN: variant, SEMIO_RENDERER: renderer, SEMIO_BUILD_MODE: profile === "release" ? "ship" : "dev", SEMIO_BRAND: resolved.brand ?? "", VITE_SEMIO_PLUGIN: variant, VITE_SEMIO_RENDERER: renderer, VITE_SEMIO_APP_ID: resolved.appId ?? "", ...frameworkOsLockedPrefsEnv() } });
  }
}

export { ServeScript };

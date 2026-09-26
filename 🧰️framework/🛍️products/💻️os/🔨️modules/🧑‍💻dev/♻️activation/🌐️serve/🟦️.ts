/** 🧩️ Semantic activation serve owner. */

import { ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, readActivationReceipt } from "../🟦️.ts";

import { existsSync } from "node:fs";

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

import { ActivationScript } from "../🏃️execution/🟦️.ts";
import { reportServeStagedModuleFreshness } from "../🔍️freshness/🟦️.ts";
import { ensureDevLocalHub, DEV_LOCAL_HUB_DATA_ENV, DEV_LOCAL_HUB_PROFILE_ENV } from "../../🚀️local-hub/🏃️execution/🟦️.ts";



class ServeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, renderer, selectedProfile, ...serverArgs] = args;
    if (renderer !== "react" || !["dev", "release"].includes(selectedProfile)) throw new Error("serve <variant> react <dev|release> [server options]");
    const profile = selectedProfile as "dev" | "release", runtime = developmentRuntimeRoot(this.root, variant, profile, "react");
    const receiptRoot = join(runtime, "activation");
    // activate only PUBLISHES (never builds). Warm `served` skips activate when a matching receipt
    // already names the host — content-hash freshness is reported read-only below without re-publishing.
    if (!existsSync(join(receiptRoot, ACTIVATION_RECEIPT_FILE))) {
      await new ActivationScript(this.root).run([variant, "react", profile]);
    }
    let receipt = readActivationReceipt(receiptRoot);
    if (receipt.variant !== variant || receipt.profile !== profile) throw new Error("Server activation identity mismatch");
    const hostId = playgroundCatalog.find((row) => row.variant === variant)?.pluginId ?? variant;
    if (!receipt.plugins.some((row) => row.pluginId === hostId)) {
      await new ActivationScript(this.root).run([variant, "react", profile]);
      receipt = readActivationReceipt(receiptRoot);
    }
    void reportServeStagedModuleFreshness(variant, "react", profile, runtime, receipt);
    const resolved = resolvePlaygroundFilter(variant);
    const hub = await ensureDevLocalHub(this.repoRoot, { hubUrl: process.env.S_HUB_URL || undefined });
    const localHubEnv = hub ? { S_HUB_URL: hub.hubUrl, [DEV_LOCAL_HUB_DATA_ENV]: hub.dataDir, [DEV_LOCAL_HUB_PROFILE_ENV]: hub.profileId } : {};
    await runViteBunxDev(this.root, serverArgs, { config: "../../🏗️builder/🌐️vite/🟦️.ts", portEnv: "S_OS_PORT", defaultPort: String(frameworkOsPlaygroundDefaultPort(playgroundCatalog, variant, renderer)), fixedPort: true, env: { SEMIO_PLUGIN: variant, SEMIO_RENDERER: renderer, SEMIO_BUILD_MODE: profile === "release" ? "ship" : "dev", SEMIO_BRAND: resolved.brand ?? "", VITE_SEMIO_PLUGIN: variant, VITE_SEMIO_RENDERER: renderer, VITE_SEMIO_APP_ID: resolved.appId ?? "", ...frameworkOsLockedPrefsEnv(), ...localHubEnv } });
  }
}

export { ServeScript };

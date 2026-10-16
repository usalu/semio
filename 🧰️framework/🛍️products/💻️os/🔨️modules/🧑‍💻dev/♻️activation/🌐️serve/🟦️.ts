import {frameworkOsEngineFacetV1} from "../../🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {repositoryCachePartitionRootDataV1} from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";

import {frameworkOsViteArtifactInputsV1} from "../../🏗️builder/🌐️vite/📥️artifacts/🟦️.ts";



import {loadDevContribution, resolveDevContributionFile} from "../../🧩️contribution/📥️loading/🟦️.ts";

/** 🧩️ Semantic activation serve owner. */

import {ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, readActivationReceipt} from "../🟦️.ts";


import {existsSync} from "node:fs";


import {join} from "node:path";


import {runViteBunxDev} from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🌐️service/🟦️.ts";
import {frameworkOsLockedPrefsEnv} from "../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🟦️.ts";
import {BundleScript} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";



import {playgroundCatalog, resolvePlaygroundFilter} from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";


import {ActivationScript} from "../🏃️execution/🟦️.ts";

import {reportServeStagedModuleFreshness} from "../🔍️freshness/🟦️.ts";

import {ensureDevLocalHub} from "../../🚀️local-hub/🏃️execution/🟦️.ts";

import {DEV_LOCAL_HUB_DATA_ENV, DEV_LOCAL_HUB_PROFILE_ENV} from "../../🚀️local-hub/🧬️schema/🟦️.ts";




class ServeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, renderer, selectedProfile, ...serverArgs] = args;
    if (!variant || renderer !== "react" || (selectedProfile !== "prototype" && selectedProfile !== "release")) throw new Error("serve <variant> react <prototype|release> [server options]");
    const profile = selectedProfile as "prototype" | "release", runtime = developmentRuntimeRoot(this.repoRoot, variant, profile, "react");
    const receiptRoot = join(runtime, "activation");
    // activate only PUBLISHES (never builds). Warm `served` skips activate when a matching receipt
    // already names the host — content-hash freshness is reported read-only below without re-publishing.
    if (!existsSync(join(receiptRoot, ACTIVATION_RECEIPT_FILE))) {
      await new ActivationScript(this.root, this.repoRoot, this.invocation).run([variant, "react", profile]);
    }
    let receipt = readActivationReceipt(receiptRoot);
    if (receipt.variant !== variant || receipt.profile !== profile) throw new Error("Server activation identity mismatch");
    const selected = playgroundCatalog().find(row => row.variant === variant);
    if(!selected)throw Error("Declared playground variant required");
    const hostId = selected.pluginId;
    if (!receipt.plugins.some((row) => row.pluginId === hostId)) {
      await new ActivationScript(this.root, this.repoRoot, this.invocation).run([variant, "react", profile]);
      receipt = readActivationReceipt(receiptRoot);
    }
    void reportServeStagedModuleFreshness(variant, "react", profile, runtime, receipt);
    const resolved = resolvePlaygroundFilter(variant);
    const contribution = loadDevContribution(this.repoRoot, resolved);
    const boot = new AbortController();
    const cancelBoot = () => boot.abort();
    process.once("SIGINT", cancelBoot);
    process.once("SIGTERM", cancelBoot);
    let hub: Awaited<ReturnType<typeof ensureDevLocalHub>>;
    try { hub = await ensureDevLocalHub(this.repoRoot, { provider: contribution?.localHub, hubUrl: process.env.S_HUB_URL || undefined, signal: boot.signal }); }
    finally { process.removeListener("SIGINT", cancelBoot); process.removeListener("SIGTERM", cancelBoot); }
    if (boot.signal.aborted) return;
    const localHubEnv = hub ? { S_HUB_URL: hub.hubUrl, [DEV_LOCAL_HUB_DATA_ENV]: hub.dataDir, [DEV_LOCAL_HUB_PROFILE_ENV]: hub.profileId } : {};
    await runViteBunxDev(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"vite:activation",0), this.root, serverArgs, { artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootDataV1(this.repoRoot, "products"), {version:1,mode:profile,renderer:"react",command:"serve",engines:frameworkOsEngineFacetV1(contribution?.browserSessionFactories??[])}), config: contribution ? resolveDevContributionFile(this.repoRoot, contribution.viteConfig, contribution.ownerRoot) : "../../🏗️builder/🌐️vite/🟦️.ts", portEnv: "S_OS_PORT", defaultPort: String(selected.ports.react), fixedPort: true, env: { SEMIO_PLUGIN: variant, SEMIO_RENDERER: renderer, SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_BRAND: resolved.brand ?? "", VITE_SEMIO_PLUGIN: variant, VITE_SEMIO_RENDERER: renderer, VITE_SEMIO_APP_ID: resolved.appId ?? "", ...frameworkOsLockedPrefsEnv(), ...localHubEnv } });
  }
}

export { ServeScript };

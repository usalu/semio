import {frameworkOsEngineFacetV1} from "../../🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import { repositoryCachePartitionRootDataV1 } from "@semio-tech/repo-lib/storage";

import {frameworkOsViteArtifactInputsV1} from "../../🏗️builder/🌐️vite/📥️artifacts/🟦️.ts";



import {loadDevContribution, resolveDevContributionFile} from "../../🧩️contribution/📥️loading/🟦️.ts";

/** 🧩️ Semantic activation serve owner. */

import {ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, readActivationReceipt} from "../🟦️.ts";


import {existsSync} from "node:fs";


import {join} from "node:path";


import { runViteBunxDev, frameworkOsLockedPrefsEnv } from "@semio-tech/repo-lib/dev";
import {BundleScript} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";



import {playgroundCatalog, resolvePlaygroundFilter} from "@semio-tech/framework-plugin/build/plan";


import {ActivationScript} from "../🏃️execution/🟦️.ts";

import {reportServeStagedModuleFreshness} from "../🔍️freshness/🟦️.ts";

import {withDevLocalHubSessionV1} from "../../🚀️local-hub/🎫️session/🟦️.ts";






class ServeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, renderer, selectedProfile, ...serverArgs] = args;
    if (!variant || renderer !== "react" || (selectedProfile !== "prototype" && selectedProfile !== "release")) throw new Error("serve <variant> react <prototype|release> [server options]");
    const profile = selectedProfile as "prototype" | "release", runtime = developmentRuntimeRoot(this.repoRoot, variant, profile, "react");
    const receiptRoot = join(runtime, "activation");
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
    const phase=scriptCompilerAdmissionControlV1(this.invocation,"vite:activation",0);
    await withDevLocalHubSessionV1(this.invocation,phase,{localOnly:process.env.S_LOCAL_ONLY==="1"||process.env.S_LOCAL_ONLY==="true",...(process.env.S_HUB_URL?{hubUrl:process.env.S_HUB_URL}:{}),...(contribution?.localHub?{provider:contribution.localHub}:{})},async(hub,actualPhase)=>{
    const localHubEnv={S_HUB_URL:"",...hub?.environment};
    await runViteBunxDev(this.invocation,actualPhase, this.root, serverArgs, {runtimeChannels:hub?.runtimeChannels??[], artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootDataV1(this.repoRoot, "products"), {version:1,mode:profile,renderer:"react",command:"serve",engines:frameworkOsEngineFacetV1(contribution?.browserSessionFactories??[])}), configLoader: "native", config: contribution ? resolveDevContributionFile(this.repoRoot, contribution.viteConfig, contribution.ownerRoot) : "../../🏗️builder/🌐️vite/🟦️.ts", portEnv: "S_OS_PORT", defaultPort: String(selected.ports.react), fixedPort: true, env: { SEMIO_PLUGIN: variant, SEMIO_RENDERER: renderer, SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_BRAND: resolved.brand ?? "", VITE_SEMIO_PLUGIN: variant, VITE_SEMIO_RENDERER: renderer, VITE_SEMIO_APP_ID: resolved.appId ?? "", ...frameworkOsLockedPrefsEnv(), ...localHubEnv } });
    });
  }
}

export { ServeScript };

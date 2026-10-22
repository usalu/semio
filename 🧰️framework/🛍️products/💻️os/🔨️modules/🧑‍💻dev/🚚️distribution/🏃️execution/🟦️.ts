import {withDistributionPlanV1} from "../🏗️compiler/📥️capture/🟦️.ts";



/** 🧩️ Semantic distribution execution owner. */





import {join, relative, resolve} from "node:path";



import {BundleScript} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";



import {DISTRIBUTION_LAYOUT} from "../🟦️.ts";




import {distributionPreflight, distributionPreviewProtocol, distributionProgress} from "../📋️plan/🟦️.ts";


import {publishDistributionBundle} from "../📤️publication/🟦️.ts";


import {distributionPathOrder} from "../📥️source/🟦️.ts";


import {checkDistributionBundle} from "../🔍️freshness/🟦️.ts";




/** 🚚️ Publishes, previews or byte-checks only this distribution's compiler-owned partition. */
class DistributionBundleScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const command = segments[0];
    if (segments.length !== 1 || !["generate", "preview", "check"].includes(command ?? "")) throw new Error("distribution expects exactly generate, preview or check");
    const destination = resolve(import.meta.dirname, "../..", DISTRIBUTION_LAYOUT.directory);
    distributionProgress("compiling bounded distribution through an owned carried child");
    return withDistributionPlanV1(this.repoRoot,this.invocation,async(plan,artifactRoot)=>{
    distributionProgress(`witnessed ${plan.manifest.inputs.length} compiler inputs and ${plan.manifest.outputs.length} outputs`);
    if (command === "check") {
      const findings = await checkDistributionBundle(plan, DISTRIBUTION_LAYOUT, destination);
      if (findings.length) throw new Error(`Distribution outputs are stale: ${findings.join(", ")}`);
      console.log("distribution check: fresh");
    } else if (command === "preview") {
      const previous = await distributionPreflight(plan, DISTRIBUTION_LAYOUT, destination), protocol = distributionPreviewProtocol(plan, this.repoRoot, destination);
      protocol.staleRemovals = (previous?.outputs ?? []).filter(row => !plan.files.has(row.path)).map(row => relative(this.repoRoot, join(destination, row.path)).replaceAll("\\", "/")).sort(distributionPathOrder);
      process.stdout.write(JSON.stringify(protocol) + "\n");
    } else {
      const result = await publishDistributionBundle(plan, DISTRIBUTION_LAYOUT, destination, artifactRoot);
      console.log(JSON.stringify({ outputs: plan.manifest.outputs.length, retired: result.retired, recovery: result.recovery }));
    }
    });
  }
}

export { DistributionBundleScript };

/** 🧭️ Trusted catalog commands: preflight and bootstrap of the selected packages.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { withRepositoryDevelopmentStateV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/💾️development/🟦️.ts";
import { repositoryCargoNetworkV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { verifyCapturedCargoArtifactsV1 } from "@semio-tech/process/native-build";
import { join } from "node:path";
import { buildRepositoryCargoArtifacts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withPublishedDevelopmentLocalHubHostV1 } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { validateAndPublishTrustedStdioGisCandidate, withMaterializedTrustedCatalogBundleV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import { TRUSTED_BOOTSTRAP_DEFAULT_PACKAGES, type TrustedBootstrapPackageSpecV1, trustedBootstrapPreflightComponentsV1, trustedBootstrapPreflightDescriptorsV1, trustedBootstrapPreflightSelectionV1, trustedBootstrapSelectPackages } from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";

/** 🧾️ Parses the one `[--packages <plugin,plugin,…|all>]` argument the trusted catalog verbs take. */
function trustedCatalogPackagesArgument(segments: readonly string[], verb: string): readonly TrustedBootstrapPackageSpecV1[] {
  const flag = segments.indexOf("--packages");
  const list = flag === -1 ? TRUSTED_BOOTSTRAP_DEFAULT_PACKAGES : segments[flag + 1];
  if ((flag === -1 && segments.length !== 0) || (flag !== -1 && (flag !== 0 || segments.length !== 2)) || typeof list !== "string")
    throw new Error(`usage: ${verb} [--packages <plugin,plugin,…|all>]`);
  return trustedBootstrapSelectPackages(list);
}

export class TrustedCatalogPreflightScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const selection = trustedCatalogPackagesArgument(segments, "trusted-catalog-preflight");
    trustedBootstrapPreflightSelectionV1(this.repoRoot, selection);
    trustedBootstrapPreflightDescriptorsV1(this.repoRoot, selection);
    trustedBootstrapPreflightComponentsV1(this.repoRoot, selection);
  }
}

export class TrustedCatalogBootstrapScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const selection = trustedCatalogPackagesArgument(segments, "trusted-catalog-bootstrap");
    trustedBootstrapPreflightSelectionV1(this.repoRoot, selection);
    trustedBootstrapPreflightDescriptorsV1(this.repoRoot, selection);
    trustedBootstrapPreflightComponentsV1(this.repoRoot, selection);
    const phaseControl=exactCargoCallerControlV1(this.invocation,"hub:trusted-bootstrap",0);
    await buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"),["--bin","os-hub"],this.repoRoot,{producer:"artifact",invocation:this.invocation,control:phaseControl,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),consume:verifyCapturedCargoArtifactsV1});
    return withRepositoryDevelopmentStateV1(this.invocation,phaseControl,(state,statePhase)=>withPublishedDevelopmentLocalHubHostV1(this.invocation,statePhase,state,(host,context)=>withMaterializedTrustedCatalogBundleV1(host,context.control,selection,async(receipt,actualPhase)=>{
      await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt);
      console.log(`trusted-catalog-bootstrap-receipt: ${JSON.stringify(receipt)}`);
      console.log(`trusted-catalog-bootstrap: immutable ${selection.length}-package generation (${selection.map(spec=>spec.pluginId).join(",")}) loaded by a candidate and published current after readiness; client execution remains separate`);
    })));
  }
}


/** 🧬️ Shared stdio artifact contract package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runContributionChecks } from "../../🧪️tests/📇️contributions/🟦️.ts";
import { runDefinitionHierarchyChecks } from "../../🧪️tests/🪜️definition-hierarchy/🟦️.ts";
import {runOriginalCopyCloseChecks} from "../../✏️editing/♻️close/🧪️tests/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOriginalOwnershipScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** ♻️ Executes original decoder/copy custody with one actual executable and exact physical laws. */
class OriginalOwnershipScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runOriginalOwnershipScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-original-ownership", OriginalOwnershipScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-contract", { packages: ["semio-s-artifact-stdio-contract", "semio-s-artifact-stdio-contract-wire", "semio-s-artifact-stdio-contract-window-kit", "semio-s-artifact-stdio-contract-assembly", "semio-s-artifact-stdio-contract-editing", "semio-s-artifact-stdio-contract-editing-patch", "semio-s-artifact-stdio-contract-editing-details", "semio-s-artifact-stdio-contract-part21", "semio-s-artifact-stdio-contract-list-delta", "semio-s-artifact-stdio-contract-media-export", "semio-s-artifact-stdio-contract-native-print", "semio-s-artifact-stdio-contract-testing"], commands: router, twins: [{name:"original-owner-close",run:runOriginalCopyCloseChecks},{ name: "definition-hierarchy", run: runDefinitionHierarchyChecks }, { name: "contribution-removal", run: runContributionChecks }] });

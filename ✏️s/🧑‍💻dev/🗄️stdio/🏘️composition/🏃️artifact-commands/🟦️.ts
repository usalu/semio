import { BundleScript } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { testStdioArtifactPackageContract, testStdioArtifactPackageGraph, verifyStdioCommandOwnership } from "../../🧪️tests/📦️artifact-package-graph/🟦️.ts";

export class StdioArtifactPackageContractScript extends BundleScript {
  async run(): Promise<void> {
    await verifyStdioCommandOwnership(this.repoRoot);
    await testStdioArtifactPackageContract(this.repoRoot,this.invocation);
  }
}

export class StdioArtifactPackageGraphScript extends BundleScript {
  async run(): Promise<void> {
    await testStdioArtifactPackageGraph(this.repoRoot,this.invocation);
  }
}

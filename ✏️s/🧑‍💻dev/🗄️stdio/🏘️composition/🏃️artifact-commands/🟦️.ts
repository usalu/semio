import { BundleScript } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { testStdioArtifactPackageContract, testStdioArtifactPackageGraph, verifyStdioCommandOwnership } from "../../🧪️tests/📦️artifact-package-graph/🟦️.ts";

export class StdioArtifactPackageContractScript extends BundleScript {
  async run(): Promise<void> {
    await verifyStdioCommandOwnership(this.repoRoot);
    await testStdioArtifactPackageContract(this.repoRoot);
  }
}

export class StdioArtifactPackageGraphScript extends BundleScript {
  async run(): Promise<void> {
    await testStdioArtifactPackageGraph(this.repoRoot);
  }
}

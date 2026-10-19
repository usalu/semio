import { BundleScript } from "@semio-tech/process/routing";
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

/** 🥡️ Owned `verify` routes of the stdio composition; the semio member-store producer source lane runs here so the neutral root imports no plugin test. */
export class StdioVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] !== "semio-member-store-producer-source" || segments.length !== 1) throw new Error("Usage: verify semio-member-store-producer-source");
    const { verifySemioMemberStoreProducerSource } = await import("../../🧪️tests/🧿️semio-member-store-producer-source/🟦️.ts");
    console.log(`semio member store producer source: ${verifySemioMemberStoreProducerSource(this.repoRoot)} self-tests passed.`);
  }
}

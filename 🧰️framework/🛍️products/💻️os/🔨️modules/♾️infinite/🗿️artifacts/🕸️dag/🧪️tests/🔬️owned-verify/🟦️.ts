import { BundleScript } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCargo } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🧪️ Runs the verification lanes owned by the infinite-dag domain: dag-demo-ownership. */
export class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "dag-demo-ownership") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-artifact-infinite-dag", "--lib", "--", "--nocapture"], this.repoRoot);
      return;
    }
    
    
    
    throw new Error(`Unknown owned verification ${segments.join(" ")}`);
  }
}

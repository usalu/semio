import { BundleScript } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCmd, runCargo } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runtimeExecutableV1 } from "../../../⚙️runtime/🏷️identity/🟦️.ts";
import { join } from "node:path";

/** 🧪️ Runs the verification lanes owned by the replication domain: shared-value-clone, shared-map-delta, framework-paged-list-ownership. */
export class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "shared-value-clone") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "shared_value_clone", "--", "--nocapture"], this.repoRoot);
      return;
    }
    if (segments[0] === "shared-map-delta") {
      const { testSharedMapDeltaOracle } = await import("../../📦️packages/🟦️typescript/🎮️mutation/🗂️map/🧪️testing/🧪️shared-map-delta-source/🟦️.ts");
      testSharedMapDeltaOracle();
      if (segments[1] === "oracle") return;
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "shared_map_delta", "--", "--nocapture"], this.repoRoot);
      return;
    }
    
    
    if (segments[0] === "framework-paged-list-ownership") {
      const testRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list");
      const { testPagedListOwnership } = await import(`${testRoot}/🟦️.ts`);
      testPagedListOwnership();
      runCmd(runtimeExecutableV1(), [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.repoRoot });
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "value::list::", "--", "--nocapture"], this.repoRoot);
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-ui-contract", "--lib", "fixed_list_", "--", "--nocapture"], this.repoRoot);
      return;
    }
    throw new Error(`Unknown owned verification ${segments.join(" ")}`);
  }
}

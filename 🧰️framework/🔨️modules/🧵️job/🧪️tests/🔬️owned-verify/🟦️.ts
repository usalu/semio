import { BundleScript } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCmd, runCargo } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runtimeExecutableV1 } from "../../../⚙️runtime/🏷️identity/🟦️.ts";
import { join } from "node:path";

/** 🧪️ Runs the verification lanes owned by the job domain: framework-job-physical-close. */
export class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "framework-job-physical-close") {
      const testRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close");
      const { testJobPayloadPhysicalClose } = await import(`${testRoot}/🟦️.ts`);
      testJobPayloadPhysicalClose();
      runCmd(runtimeExecutableV1(), [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.repoRoot });
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-job", "--lib", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
      return;
    }
    throw new Error(`Unknown owned verification ${segments.join(" ")}`);
  }
}

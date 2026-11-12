import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCargo } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🧪️ Runs the verification lanes owned by the block-3d domain: block3d-window-transient. */
export class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "block3d-window-transient") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-block-3d", "--lib", "preview_", "--", "--nocapture"], this.repoRoot);
      return;
    }
    throw new Error(`Unknown owned verification ${segments.join(" ")}`);
  }
}

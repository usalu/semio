import { BundleScript } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCargo } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🧪️ Runs the verification lanes owned by the value domain: shared-dynamic-value, framework-variant-field-casing. */
export class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "shared-dynamic-value") {
      const { testSharedDynamicValueOracle } = await import("../../📦️packages/🟦️typescript/🧪️testing/🔣️json-projection/🟦️.ts");
      testSharedDynamicValueOracle();
      return;
    }
    if (segments[0] === "framework-variant-field-casing") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-value-derive", "--test", "variant_field_casing", "--", "--nocapture"], this.repoRoot);
      return;
    }
    throw new Error(`Unknown owned verification ${segments.join(" ")}`);
  }
}

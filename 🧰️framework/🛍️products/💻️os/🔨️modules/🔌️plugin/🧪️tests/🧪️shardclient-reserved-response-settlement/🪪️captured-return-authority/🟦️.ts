/** 🪪️ The captured return authority describe block: its wrapper, shared fixtures and parts. */
import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";
import { createCapturedReturnHelpers } from "../🛠️captured-return-helpers/🟦️.ts";
import { registerCapturedReturnAdmissionTests } from "../🔓️captured-return-admission/🟦️.ts";
import { registerKernelReturnInputBindingTests } from "../🔗️kernel-return-input-binding/🟦️.ts";
import { registerKernelReturnInputSettlementTests } from "../🧵️kernel-return-input-settlement/🟦️.ts";
import { registerCapturedReturnRoutingTests } from "../🔀️captured-return-routing/🟦️.ts";

/** 🛡 Registers the captured return authority describe block from its helpers and parts. */
export function registerCapturedReturnAuthorityTests(kit: ShardClientSettlementTestKit): void {
  const { describe } = kit;
  describe("ShardClient captured return authority", () => {
    const helpers = createCapturedReturnHelpers(kit);
    registerCapturedReturnAdmissionTests(kit, helpers);
    registerKernelReturnInputBindingTests(kit, helpers);
    registerKernelReturnInputSettlementTests(kit, helpers);
    registerCapturedReturnRoutingTests(kit, helpers);
  });
}

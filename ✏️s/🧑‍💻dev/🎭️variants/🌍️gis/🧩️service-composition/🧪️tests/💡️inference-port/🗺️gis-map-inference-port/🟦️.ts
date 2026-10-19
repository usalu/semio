/** 🗺️ The gis map inference port describe block: its wrapper, shared fixtures and parts. */
import type { GisInferencePortTestKit } from "../🧰️inference-port-test-kit/🟦️.ts";
import { createInferencePortHelpers } from "../🛠️inference-port-helpers/🟦️.ts";
import { registerReceiptAndLeaseAdmissionTests } from "../🪪️receipt-and-lease-admission/🟦️.ts";
import { registerSealedOwnerRetentionTests } from "../🔒️sealed-owner-retention/🟦️.ts";
import { registerBrokerSessionAuthorityTests } from "../🔑️broker-session-authority/🟦️.ts";
import { registerBrokerPortRetirementTests } from "../🔌️broker-port-retirement-and-queue/🟦️.ts";
import { registerCancelStaleApprovalTests } from "../🏁️cancel-stale-approval-and-vocabulary/🟦️.ts";

/** 🛰 Registers the gis map inference port describe block from its helpers and parts. */
export function registerGisMapInferencePortTests(kit: GisInferencePortTestKit): void {
  const { describe } = kit;
  describe("gis map inference port", () => {
    const helpers = createInferencePortHelpers(kit);
    registerReceiptAndLeaseAdmissionTests(kit, helpers);
    registerSealedOwnerRetentionTests(kit, helpers);
    registerBrokerSessionAuthorityTests(kit, helpers);
    registerBrokerPortRetirementTests(kit, helpers);
    registerCancelStaleApprovalTests(kit, helpers);
  });
}

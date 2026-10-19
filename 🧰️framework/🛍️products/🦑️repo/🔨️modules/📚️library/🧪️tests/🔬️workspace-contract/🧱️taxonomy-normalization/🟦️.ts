import { testLevelAtLeast } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { describe } from "bun:test";
import { registerCatalogMappingAndOwnershipTests } from "./🗺️catalog-mappings-and-ownership/🟦️.ts";
import { registerProjectionApplyAndResidualChildTests } from "./🔄️projection-apply-and-residual-children/🟦️.ts";
import { registerSourceAdmissionAndCanonicalJsonTests } from "./🎫️source-admission-and-canonical-json/🟦️.ts";
import { registerScopedInventoryAndSymlinkTests } from "./🔍️scoped-inventory-and-symlinks/🟦️.ts";
import { registerInventoryOrderCliAndTicketScopeTests } from "./📋️inventory-order-cli-and-ticket-scope/🟦️.ts";
import { registerPlansBoundariesAndAdapterTests } from "./🏗️plans-boundaries-and-adapters/🟦️.ts";
import { registerPreimageRollbackAndCancellationTests } from "./↩️preimage-rollback-and-cancellation/🟦️.ts";

describe.if(testLevelAtLeast("long"))("taxonomy normalization", () => {
  registerCatalogMappingAndOwnershipTests();
  registerProjectionApplyAndResidualChildTests();
  registerSourceAdmissionAndCanonicalJsonTests();
  registerScopedInventoryAndSymlinkTests();
  registerInventoryOrderCliAndTicketScopeTests();
  registerPlansBoundariesAndAdapterTests();
  registerPreimageRollbackAndCancellationTests();
});

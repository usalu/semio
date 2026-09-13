import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyAbstractionOwnershipBreaches } from "../../../../📏️ownership/🏛️abstraction/⚖️law/🟦️.ts";
import { policyAppSchemaConfigFidelityBreaches } from "../🪞️config-fidelity/🟦️.ts";
import { policyAppSchemaConfigRelocationBreaches } from "../🚚️config-relocation/🟦️.ts";
import { policyAppSchemaFacetCompletenessBreaches } from "../🧩️facet-completeness/🟦️.ts";
import { policyAppSchemaFieldParityBreaches } from "../📏️field-parity/🟦️.ts";
import { policyAppSchemaStatePurityBreaches } from "../💧️state-purity/🟦️.ts";
import { policyAppSchemaTypeNameParityBreaches } from "../🏷️type-name-parity/🟦️.ts";

/** ⚖️ Aggregates surface-schema and abstraction-ownership laws. */
export function policyAppSchemaBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  return [
    ...policyAbstractionOwnershipBreaches(repoRoot, operations),
    ...policyAppSchemaFacetCompletenessBreaches(repoRoot, operations),
    ...policyAppSchemaFieldParityBreaches(repoRoot, operations),
    ...policyAppSchemaConfigFidelityBreaches(repoRoot, operations),
    ...policyAppSchemaStatePurityBreaches(repoRoot, operations),
    ...policyAppSchemaTypeNameParityBreaches(repoRoot, operations),
    ...policyAppSchemaConfigRelocationBreaches(repoRoot, operations),
  ];
}

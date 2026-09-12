import { POLICY_SOURCE_OPERATIONS, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyDiscoverArtifactSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";
import { policyArtifactSchemaFacetCompletenessBreaches } from "../🧩️facet-completeness/🟦️.ts";
import { policyArtifactSchemaFieldParityBreaches } from "../📏️field-parity/🟦️.ts";
import { policyArtifactSchemaStateParityBreaches } from "../💾️state-parity/🟦️.ts";
import { policyArtifactSchemaDiffCoverageBreaches } from "../🔺️diff-coverage/🟦️.ts";
import { policyArtifactSchemaTypeNameParityBreaches } from "../🏷️type-name-parity/🟦️.ts";

/** ⚖️ Aggregates the separate artifact-schema discovery, completeness, parity and coverage laws. */
export function policyArtifactSchemaBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS) {
  const owners = policyDiscoverArtifactSchemaOwners(repoRoot, operations);
  return [
    ...policyArtifactSchemaFacetCompletenessBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaFieldParityBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaStateParityBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaDiffCoverageBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaTypeNameParityBreaches(repoRoot, owners, operations),
  ];
}

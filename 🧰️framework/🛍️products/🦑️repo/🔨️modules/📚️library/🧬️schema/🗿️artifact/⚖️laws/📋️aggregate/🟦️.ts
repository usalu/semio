import { POLICY_SOURCE_OPERATIONS, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyDiscoverArtifactSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";
import { policyArtifactSchemaFacetCompletenessBreaches } from "../🧩️facet-completeness/🟦️.ts";
import { policyArtifactSchemaFieldParityBreaches } from "../📏️field-parity/🟦️.ts";
import { policyArtifactSchemaStateParityBreaches } from "../💾️state-parity/🟦️.ts";
import { policyArtifactSchemaDiffCoverageBreaches } from "../🔺️diff-coverage/🟦️.ts";
import { policyArtifactSchemaTypeNameParityBreaches } from "../🏷️type-name-parity/🟦️.ts";

/** ⚖️ Aggregates the separate artifact-schema discovery, completeness, parity and coverage laws. */
export function policyArtifactSchemaBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS) {
  const discovery = policyDiscoverArtifactSchemaOwners(repoRoot, operations),
    owners = discovery.owners;
  return [
    ...discovery.issues.map((issue) => ({
      id: `artifact-schema-source-unreadable-${issue.path}`,
      summary: `"${issue.path}" cannot be admitted for artifact-schema owner discovery: ${issue.state}`,
      kind: "artifact-schema/source-unreadable",
      scope: issue.path,
      priority: "high" as const,
      reason: "A source root or subtree that cannot be read without following links is unresolved evidence and cannot certify an empty owner set.",
      solution: `Restore readable no-follow directory access to ${issue.path}/.`,
    })),
    ...policyArtifactSchemaFacetCompletenessBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaFieldParityBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaStateParityBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaDiffCoverageBreaches(repoRoot, owners, operations),
    ...policyArtifactSchemaTypeNameParityBreaches(repoRoot, owners, operations),
  ];
}

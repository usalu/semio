import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_APP_SCHEMA_FACET } from "../../🧱️contract/🟦️.ts";
import { policyDiscoverAppSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";
import { policyLoadAppSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";
/**
 * 📏️State purity: every config-facet field is `config`; every presence-facet field is `presence`.
 */
export function policyAppSchemaStatePurityBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot, operations)) {
    const checks: { facetAbs: string; expectedState: string; expectedTypeName: string; label: string }[] = [
      {
        facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`,
        expectedState: "config",
        expectedTypeName: owner.configType,
        label: "config",
      },
      {
        facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}`,
        expectedState: "presence",
        expectedTypeName: owner.presenceType,
        label: "presence",
      },
    ];
    for (const { facetAbs, expectedState, expectedTypeName, label } of checks) {
      const facetSource = policySourceDirectory(repoRoot, facetAbs, operations);
      if (facetSource.state === "missing") continue;
      if (facetSource.state !== "directory") throw new Error(`Surface schema facet ${facetAbs} is ${facetSource.state}.`);
      const jsonLeaf = policyLoadAppSchemaFacetLeaves(repoRoot, facetAbs, expectedTypeName, operations).find((l) => l.formatId === "🔣️jsonschema");
      if (!jsonLeaf?.extract) continue;
      for (const field of jsonLeaf.extract.fields) {
        if (field.state === expectedState) continue;
        breaches.push({
          id: `app-schema-state-purity-${facetAbs}-${field.name}`,
          summary: `${label} facet field "${field.name}" must be ${expectedState} (got ${field.state || "missing"})`,
          kind: "app-schema/state-purity",
          scope: owner.ownerRel,
          priority: "high",
          reason: `App ${label} facet fields are by definition ${expectedState}; other state classes belong elsewhere.`,
          solution: `Set x-semio-state (and the matching per-format state annotation) for "${field.name}" in ${jsonLeaf.relPath} to ${expectedState}.`,
        });
      }
    }
  }
  return breaches;
}

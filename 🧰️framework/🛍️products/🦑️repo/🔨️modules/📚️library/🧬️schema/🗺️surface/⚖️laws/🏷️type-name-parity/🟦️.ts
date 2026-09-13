import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_APP_SCHEMA_FACET } from "../../🧱️contract/🟦️.ts";
import { policyDiscoverAppSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";
import { policyLoadAppSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";
/**
 * 📏️Type-name parity: `XConfig` / `XPresence` spelled identically across all five leaves of their facet;
 * `XPresence` is derived from the owner's `type Config` binding (trailing `Config` → `Presence`).
 */
export function policyAppSchemaTypeNameParityBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot, operations)) {
    const facets: { facetAbs: string; expected: string }[] = [
      { facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`, expected: owner.configType },
      { facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}`, expected: owner.presenceType },
    ];
    for (const { facetAbs, expected } of facets) {
      const facetSource = policySourceDirectory(repoRoot, facetAbs, operations);
      if (facetSource.state === "missing") continue;
      if (facetSource.state !== "directory") throw new Error(`Surface schema facet ${facetAbs} is ${facetSource.state}.`);
      const leaves = policyLoadAppSchemaFacetLeaves(repoRoot, facetAbs, expected, operations);
      for (const leaf of leaves) {
        if (!leaf.extract) continue;
        if (!leaf.extract.typeName) {
          breaches.push({
            id: `app-schema-type-name-missing-${leaf.relPath}`,
            summary: `"${leaf.relPath}" does not declare top-level type ${expected}`,
            kind: "app-schema/type-name-parity",
            scope: owner.ownerRel,
            priority: "high",
            reason: `Every leaf of this facet must declare the same top-level type name ${expected}.`,
            solution: `Declare ${expected} as the top-level type in ${leaf.relPath}.`,
          });
          continue;
        }
        if (leaf.extract.typeName !== expected) {
          breaches.push({
            id: `app-schema-type-name-${leaf.relPath}`,
            summary: `"${leaf.relPath}" declares ${leaf.extract.typeName} but expects ${expected}`,
            kind: "app-schema/type-name-parity",
            scope: owner.ownerRel,
            priority: "high",
            reason: `Type-name parity requires ${expected} in all five leaves (from the app's type Config binding).`,
            solution: `Rename the top-level type in ${leaf.relPath} to ${expected}.`,
          });
        }
      }
    }
  }
  return breaches;
}

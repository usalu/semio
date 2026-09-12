import { canonicalPrimaryFilenameForKind, loadTaxonomy, type BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_SCHEMA_FACET_RELS, policyLoadSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";
import { policyDeclaredSchemaExportName } from "../../🏷️export-identity/🟦️.ts";

/** 🏷️ Requires every representation to use the export id declared by normative JSON Schema. */
export function policyArtifactSchemaTypeNameParityBreaches(repoRoot: string, owners: readonly string[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const taxonomy = loadTaxonomy(),
    breaches: BreachRecord[] = [];
  for (const artRel of owners) {
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (policySourceDirectory(repoRoot, facetAbs, operations).state !== "directory") continue;
      const expected = policyDeclaredSchemaExportName(repoRoot, facetAbs, operations);
      if (!expected) {
        breaches.push({
          id: `artifact-schema-export-undeclared-${facetAbs}`,
          summary: `"${facetAbs}" declares no export id in its normative JSON Schema`,
          kind: "artifact-schema/type-name-parity",
          scope: artRel,
          priority: "high",
          reason: "A scope names its exports itself: the facet's JSON Schema title is the root export id every other format must spell.",
          solution: `Declare the PascalCase root export as "title" in ${facetAbs}/${canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🔣️jsonschema"]!.fileKindId)}.`,
        });
        continue;
      }
      for (const leaf of policyLoadSchemaFacetLeaves(repoRoot, facetAbs, operations)) {
        if (!leaf.extract || leaf.extract.typeName === expected) continue;
        breaches.push({
          id: `artifact-schema-type-name-missing-${leaf.relPath}`,
          summary: `"${leaf.relPath}" does not declare the facet export ${expected}`,
          kind: "artifact-schema/type-name-parity",
          scope: artRel,
          priority: "high",
          reason: `Every leaf of facet ${facetRel} must declare the export id its JSON Schema title names (${expected}).`,
          solution: `Declare ${expected} as the top-level type in ${leaf.relPath}.`,
        });
      }
    }
  }
  return breaches;
}

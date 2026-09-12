import { canonicalPrimaryFilenameForKind, loadTaxonomy, type BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyLoadSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";

/** 🔺️ Requires every non-transient artifact field, and no transient field, in the diff facet. */
export function policyArtifactSchemaDiffCoverageBreaches(repoRoot: string, owners: readonly string[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const normative = canonicalPrimaryFilenameForKind(loadTaxonomy().semanticManifestFileKindId),
    breaches: BreachRecord[] = [];
  for (const artRel of owners) {
    const artifactFacet = `${artRel}/🧬️schema`,
      diffFacet = `${artRel}/🧬️schema/🔺️diff`;
    if (policySourceDirectory(repoRoot, artifactFacet, operations).state !== "directory" || policySourceDirectory(repoRoot, diffFacet, operations).state !== "directory") continue;
    const artJson = policyLoadSchemaFacetLeaves(repoRoot, artifactFacet, operations).find((leaf) => leaf.formatId === "🔣️jsonschema")?.extract,
      diffJson = policyLoadSchemaFacetLeaves(repoRoot, diffFacet, operations).find((leaf) => leaf.formatId === "🔣️jsonschema")?.extract;
    if (!artJson || !diffJson) continue;
    const diffNames = new Set(diffJson.fields.map((field) => field.name));
    for (const field of artJson.fields) {
      if (field.state === "transient") {
        if (diffNames.has(field.name)) {
          breaches.push({
            id: `artifact-schema-diff-effect-${artRel}-${field.name}`,
            summary: `Diff facet must not cover transient field "${field.name}"`,
            kind: "artifact-schema/diff-coverage",
            scope: artRel,
            priority: "high",
            reason: "Transient fields are ephemeral local-only UI state and must not appear in XDiff.",
            solution: `Remove "${field.name}" from ${diffFacet}/${normative}.`,
          });
        }
        continue;
      }
      if (!diffNames.has(field.name)) {
        breaches.push({
          id: `artifact-schema-diff-coverage-${artRel}-${field.name}`,
          summary: `Diff facet is missing entry for non-transient artifact field "${field.name}"`,
          kind: "artifact-schema/diff-coverage",
          scope: artRel,
          priority: "high",
          reason: "Every artifact field whose state lane is not transient must have a same-named diff entry.",
          solution: `Add sparse diff entry "${field.name}" to ${diffFacet}/${normative} matching the declared cardinality rules.`,
        });
      }
    }
  }
  return breaches;
}

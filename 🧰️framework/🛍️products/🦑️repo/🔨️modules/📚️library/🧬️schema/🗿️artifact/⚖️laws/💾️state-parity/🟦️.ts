import { canonicalPrimaryFilenameForKind, loadTaxonomy, type BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyLoadSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";

/** 💾 Requires snapshot fields to equal the artifact-state fields of the document facet. */
export function policyArtifactSchemaStateParityBreaches(repoRoot: string, owners: readonly string[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const taxonomy = loadTaxonomy(),
    breaches: BreachRecord[] = [];
  for (const artRel of owners) {
    const artifactFacet = `${artRel}/🧬️schema`,
      snapshotFacet = `${artRel}/🧬️schema/📸️snapshot`;
    if (policySourceDirectory(repoRoot, artifactFacet, operations).state !== "directory" || policySourceDirectory(repoRoot, snapshotFacet, operations).state !== "directory") continue;
    const artJson = policyLoadSchemaFacetLeaves(repoRoot, artifactFacet, operations).find((leaf) => leaf.formatId === "🔣️jsonschema")?.extract,
      snapJson = policyLoadSchemaFacetLeaves(repoRoot, snapshotFacet, operations).find((leaf) => leaf.formatId === "🔣️jsonschema")?.extract;
    if (!artJson || !snapJson) continue;
    const persistent = artJson.fields.filter((field) => field.state === "artifact"),
      snapMap = new Map(snapJson.fields.map((field) => [field.name, field])),
      persistentMap = new Map(persistent.map((field) => [field.name, field])),
      normative = canonicalPrimaryFilenameForKind(taxonomy.semanticManifestFileKindId);
    for (const field of persistent) {
      const snapshot = snapMap.get(field.name);
      if (!snapshot) {
        breaches.push({
          id: `artifact-schema-state-parity-missing-${artRel}-${field.name}`,
          summary: `Snapshot facet is missing artifact-lane field "${field.name}"`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: "XSnapshot must equal exactly the artifact-lane fields of XArtifact (equality, not subset).",
          solution: `Add "${field.name}" to ${snapshotFacet}/${normative} and the other representation leaves.`,
        });
        continue;
      }
      if (snapshot.optional !== field.optional || snapshot.cardinality !== field.cardinality) {
        breaches.push({
          id: `artifact-schema-state-parity-shape-${artRel}-${field.name}`,
          summary: `Snapshot field "${field.name}" shape differs from the artifact-lane field`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: `Artifact-lane field "${field.name}" is optional=${field.optional}, cardinality=${field.cardinality}; snapshot has optional=${snapshot.optional}, cardinality=${snapshot.cardinality}.`,
          solution: `Align "${field.name}" in ${snapshotFacet}/${normative} with ${artifactFacet}/${normative}.`,
        });
      }
    }
    for (const name of snapMap.keys()) {
      if (persistentMap.has(name)) continue;
      breaches.push({
        id: `artifact-schema-state-parity-extra-${artRel}-${name}`,
        summary: `Snapshot facet has non-artifact-lane field "${name}"`,
        kind: "artifact-schema/state-parity",
        scope: artRel,
        priority: "high",
        reason: "XSnapshot may only contain the artifact-lane fields of XArtifact.",
        solution: `Remove "${name}" from ${snapshotFacet}/${normative}, or move it into the artifact lane on the artifact facet if it belongs there.`,
      });
    }
  }
  return breaches;
}

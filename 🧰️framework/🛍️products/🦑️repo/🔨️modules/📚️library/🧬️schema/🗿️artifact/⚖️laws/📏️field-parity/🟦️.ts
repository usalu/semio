import { type BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_SCHEMA_FACET_RELS, policyLoadSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";

/** 📏 Compares every representation with normative JSON Schema fields and expressible shapes. */
export function policyArtifactSchemaFieldParityBreaches(repoRoot: string, owners: readonly string[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of owners) {
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (policySourceDirectory(repoRoot, facetAbs, operations).state !== "directory") continue;
      const leaves = policyLoadSchemaFacetLeaves(repoRoot, facetAbs, operations);
      if (leaves.some((leaf) => leaf.extract === null)) continue;
      const jsonLeaf = leaves.find((leaf) => leaf.formatId === "🔣️jsonschema");
      if (!jsonLeaf?.extract) continue;
      const truth = new Map(jsonLeaf.extract.fields.map((field) => [field.name, field]));
      for (const leaf of leaves) {
        if (leaf.formatId === "🔣️jsonschema" || !leaf.extract) continue;
        const seen = new Map(leaf.extract.fields.map((field) => [field.name, field]));
        for (const [name, truthField] of truth) {
          const other = seen.get(name);
          if (!other) {
            breaches.push({
              id: `artifact-schema-field-parity-missing-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" is missing field "${name}" present in normative JSON Schema`,
              kind: "artifact-schema/field-parity",
              scope: artRel,
              priority: "high",
              reason: `Field parity requires identical canonical fields across all five leaves; JSON Schema is normative (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
              solution: `Add field "${name}" to ${leaf.relPath} matching ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}, scalar=${truthField.scalar}).`,
            });
            continue;
          }
          const optionalityComparable = !(leaf.formatId === "🛰️protobuf" && truthField.cardinality === "map");
          const cardinalityComparable = !(truthField.cardinality === "fixedList" && other.cardinality === "list" && (leaf.formatId === "🟦️typescript" || leaf.formatId === "🔗️graphql" || leaf.formatId === "🛰️protobuf"));
          if ((optionalityComparable && other.optional !== truthField.optional) || (cardinalityComparable && other.cardinality !== truthField.cardinality)) {
            breaches.push({
              id: `artifact-schema-field-parity-shape-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" field "${name}" disagrees with normative JSON Schema optionality/cardinality`,
              kind: "artifact-schema/field-parity",
              scope: artRel,
              priority: "high",
              reason: `Normative ${jsonLeaf.relPath} declares "${name}" as optional=${truthField.optional}, cardinality=${truthField.cardinality}; ${leaf.formatId} has optional=${other.optional}, cardinality=${other.cardinality}.`,
              solution: `Change "${name}" in ${leaf.relPath} to match ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
            });
          }
        }
        for (const name of seen.keys()) {
          if (truth.has(name)) continue;
          breaches.push({
            id: `artifact-schema-field-parity-extra-${leaf.relPath}-${name}`,
            summary: `"${leaf.relPath}" declares extra field "${name}" absent from normative JSON Schema`,
            kind: "artifact-schema/field-parity",
            scope: artRel,
            priority: "high",
            reason: `JSON Schema at ${jsonLeaf.relPath} is normative; extra fields in other formats break cross-format identity.`,
            solution: `Remove "${name}" from ${leaf.relPath}, or add it to ${jsonLeaf.relPath} if it is a real artifact field.`,
          });
        }
      }
    }
  }
  return breaches;
}

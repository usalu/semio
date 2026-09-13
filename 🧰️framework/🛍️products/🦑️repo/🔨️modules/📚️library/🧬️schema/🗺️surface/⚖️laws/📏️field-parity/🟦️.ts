import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_APP_SCHEMA_FACET } from "../../🧱️contract/🟦️.ts";
import { policyDiscoverAppSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";
import { policyLoadAppSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";
/**
 * 📏️Field parity: all five leaves of one facet declare the identical canonical field set with identical
 * optionality and cardinality; JSON Schema is the truth when others disagree. Optionality of a `map`
 * field is exempt for protobuf only (proto3 rejects `optional` on a map entry field).
 * @see https://protobuf.dev/programming-guides/proto3/#maps
 */
export function policyAppSchemaFieldParityBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot, operations)) {
    const facets: { facetAbs: string; expectedTypeName: string }[] = [
      { facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`, expectedTypeName: owner.configType },
      { facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}`, expectedTypeName: owner.presenceType },
    ];
    for (const { facetAbs, expectedTypeName } of facets) {
      const facetSource = policySourceDirectory(repoRoot, facetAbs, operations);
      if (facetSource.state === "missing") continue;
      if (facetSource.state !== "directory") throw new Error(`Surface schema facet ${facetAbs} is ${facetSource.state}.`);
      const leaves = policyLoadAppSchemaFacetLeaves(repoRoot, facetAbs, expectedTypeName, operations);
      if (leaves.some((l) => l.extract === null)) continue;
      const jsonLeaf = leaves.find((l) => l.formatId === "🔣️jsonschema");
      if (!jsonLeaf?.extract) continue;
      const truth = new Map(jsonLeaf.extract.fields.map((f) => [f.name, f]));
      for (const leaf of leaves) {
        if (leaf.formatId === "🔣️jsonschema" || !leaf.extract) continue;
        const seen = new Map(leaf.extract.fields.map((f) => [f.name, f]));
        for (const [name, truthField] of truth) {
          const other = seen.get(name);
          if (!other) {
            breaches.push({
              id: `app-schema-field-parity-missing-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" is missing field "${name}" present in normative JSON Schema`,
              kind: "app-schema/field-parity",
              scope: owner.ownerRel,
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
              id: `app-schema-field-parity-shape-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" field "${name}" disagrees with normative JSON Schema optionality/cardinality`,
              kind: "app-schema/field-parity",
              scope: owner.ownerRel,
              priority: "high",
              reason: `Normative ${jsonLeaf.relPath} declares "${name}" as optional=${truthField.optional}, cardinality=${truthField.cardinality}; ${leaf.formatId} has optional=${other.optional}, cardinality=${other.cardinality}.`,
              solution: `Change "${name}" in ${leaf.relPath} to match ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
            });
          }
        }
        for (const name of seen.keys()) {
          if (truth.has(name)) continue;
          breaches.push({
            id: `app-schema-field-parity-extra-${leaf.relPath}-${name}`,
            summary: `"${leaf.relPath}" declares extra field "${name}" absent from normative JSON Schema`,
            kind: "app-schema/field-parity",
            scope: owner.ownerRel,
            priority: "high",
            reason: `JSON Schema at ${jsonLeaf.relPath} is normative; extra fields in other formats break cross-format identity.`,
            solution: `Remove "${name}" from ${leaf.relPath}, or add it to ${jsonLeaf.relPath} if it is a real app field.`,
          });
        }
      }
    }
  }
  return breaches;
}

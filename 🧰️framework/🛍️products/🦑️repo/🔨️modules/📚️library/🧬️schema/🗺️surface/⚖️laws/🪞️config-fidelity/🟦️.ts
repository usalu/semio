import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../🔍️field-discovery/🦀️rust/🟦️.ts";
import { POLICY_APP_SCHEMA_FACET } from "../../🧱️contract/🟦️.ts";
import { policyDiscoverAppSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";
import { policyLoadAppSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";

/** 🪞️ Compares the normative config facet with its authored Rust config type. */
export function policyAppSchemaConfigFidelityBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot, operations)) {
    const configRel = `${owner.ownerRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
      configSource = policySourceText(repoRoot, configRel, operations),
      facetRel = `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`,
      facetSource = policySourceDirectory(repoRoot, facetRel, operations);
    if (configSource.state === "missing" || facetSource.state === "missing") continue;
    if (configSource.state !== "file" || facetSource.state !== "directory") {
      const path = configSource.state !== "file" ? configRel : facetRel,
        state = configSource.state !== "file" ? configSource.state : facetSource.state;
      breaches.push({
        id: `app-schema-source-unreadable-${path}`,
        summary: `"${path}" is ${state}`,
        kind: "app-schema/source-unreadable",
        scope: owner.ownerRel,
        priority: "high",
        reason: "Config fidelity requires admitted readable config and schema sources.",
        solution: `Restore a readable regular source at ${path}.`,
      });
      continue;
    }
    const real = policyExtractRustSchemaFields(configSource.text, owner.configType),
      jsonLeaf = policyLoadAppSchemaFacetLeaves(repoRoot, facetRel, owner.configType, operations).find((leaf) => leaf.formatId === "🔣️jsonschema");
    if (!jsonLeaf?.extract) continue;
    const truth = new Map(real.fields.map((field) => [field.name, field])),
      seen = new Map(jsonLeaf.extract.fields.map((field) => [field.name, field]));
    for (const [name, realField] of truth) {
      const facetField = seen.get(name);
      if (!facetField) {
        breaches.push({
          id: `app-schema-config-fidelity-missing-${owner.ownerRel}-${name}`,
          summary: `Config facet is missing field "${name}" from real ${owner.configType}`,
          kind: "app-schema/config-fidelity",
          scope: owner.ownerRel,
          priority: "high",
          reason: `The config facet must document exactly the fields of ${owner.configType} in ${configRel}.`,
          solution: `Add "${name}" to ${jsonLeaf.relPath} and the other four leaves matching ${configRel} (optional=${realField.optional}, cardinality=${realField.cardinality}).`,
        });
        continue;
      }
      if (facetField.optional !== realField.optional || facetField.cardinality !== realField.cardinality) {
        breaches.push({
          id: `app-schema-config-fidelity-shape-${owner.ownerRel}-${name}`,
          summary: `Config facet field "${name}" disagrees with real ${owner.configType}`,
          kind: "app-schema/config-fidelity",
          scope: owner.ownerRel,
          priority: "high",
          reason: `Real ${owner.configType}.${name} is optional=${realField.optional}, cardinality=${realField.cardinality}; facet has optional=${facetField.optional}, cardinality=${facetField.cardinality}.`,
          solution: `Align "${name}" in ${jsonLeaf.relPath} with ${configRel}.`,
        });
      }
    }
    for (const name of seen.keys()) {
      if (truth.has(name)) continue;
      breaches.push({
        id: `app-schema-config-fidelity-extra-${owner.ownerRel}-${name}`,
        summary: `Config facet declares extra field "${name}" absent from real ${owner.configType}`,
        kind: "app-schema/config-fidelity",
        scope: owner.ownerRel,
        priority: "high",
        reason: `The config facet may not invent fields beyond ${owner.configType} in ${configRel}.`,
        solution: `Remove "${name}" from ${jsonLeaf.relPath}, or add it to ${configRel} if it belongs on the real struct.`,
      });
    }
  }
  return breaches;
}

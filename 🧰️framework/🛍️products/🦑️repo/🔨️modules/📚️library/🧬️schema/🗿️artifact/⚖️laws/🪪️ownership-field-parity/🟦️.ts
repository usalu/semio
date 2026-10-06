import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policySchemaFieldDifferences } from "../../../🔍️field-discovery/⚖️comparison/🟦️.ts";
import { POLICY_SCHEMA_FACET_RELS, policyLoadSchemaFacetLeaves } from "../../📚️facet-leaves/🟦️.ts";
import { policyDiscoverArtifactSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";

/** 🪪️ Audits authored document, snapshot and diff declarations through their semantic owners. */
export function policyArtifactOwnershipFieldParity(root: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): { path: string; missing: string[]; extra: string[] }[] {
  const discovery = policyDiscoverArtifactSchemaOwners(root, operations);
  if (discovery.issues.length) throw new Error("Artifact owner discovery is unresolved: " + JSON.stringify(discovery.issues));
  const breaches: { path: string; missing: string[]; extra: string[] }[] = [];
  for (const owner of discovery.owners) for (const facet of POLICY_SCHEMA_FACET_RELS) {
    const path = owner + "/" + facet, source = policySourceDirectory(root, path, operations);
    if (source.state === "missing") continue;
    if (source.state !== "directory") throw new Error("Artifact schema " + path + " is " + source.state);
    const leaves = policyLoadSchemaFacetLeaves(root, path, operations), normative = leaves.find(leaf => leaf.formatId === "🔣️jsonschema");
    if (!normative?.extract) {
      breaches.push({ path: normative?.relPath ?? path + "/🔣️.json", missing: ["source:" + (normative?.sourceState ?? "missing")], extra: [] });
      continue;
    }
    for (const leaf of leaves) {
      if (leaf === normative) continue;
      const difference = policySchemaFieldDifferences(normative.extract.fields.map(field => field.name), leaf.extract?.fields.map(field => field.name) ?? []);
      if (leaf.sourceState !== "file") difference.missing.unshift("source:" + leaf.sourceState);
      else if (!leaf.extract?.typeName) difference.missing.unshift("declaration:" + normative.extract.typeName);
      if (difference.missing.length || difference.extra.length) breaches.push({ path: leaf.relPath, ...difference });
    }
  }
  return breaches.sort((left, right) => left.path.localeCompare(right.path));
}

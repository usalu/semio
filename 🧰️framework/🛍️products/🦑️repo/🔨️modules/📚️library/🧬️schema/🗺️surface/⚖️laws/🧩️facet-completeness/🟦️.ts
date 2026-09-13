import { canonicalPrimaryFilenameForKind, schemaFacetFormatEntries } from "../../../../🔍️discovery/🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { loadTaxonomy, type BreachRecord } from "../../../../🟦️.ts";
import { POLICY_APP_SCHEMA_FACET, policyAppSchemaFacetRole } from "../../🧱️contract/🟦️.ts";
import { policyDiscoverAppSchemaOwners } from "../../🔍️owner-discovery/🟦️.ts";

/** 🧩️ Reports missing or unavailable config and presence facets and their configured schema leaves. */
export function policyAppSchemaFacetCompletenessBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const taxonomy = loadTaxonomy(),
    normativeByFacet = taxonomy.surfaceSchemaSpecFileKinds ?? {},
    breaches: BreachRecord[] = [],
    sourceBreach = (scope: string, path: string, state: string): BreachRecord => ({
      id: `app-schema-source-unreadable-${path}`,
      summary: `"${path}" is ${state}`,
      kind: "app-schema/source-unreadable",
      scope,
      priority: "high",
      reason: "Surface schema laws require admitted regular directories and files and never follow symbolic links.",
      solution: `Restore a readable regular source at ${path}.`,
    });
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot, operations)) {
    const facets: { kind: "config" | "presence"; facetAbs: string }[] = [
      { kind: "config", facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}` },
      { kind: "presence", facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}` },
    ];
    for (const { kind, facetAbs } of facets) {
      const facetSource = policySourceDirectory(repoRoot, facetAbs, operations);
      if (facetSource.state === "missing") {
        breaches.push({
          id: `app-schema-facet-missing-${facetAbs}`,
          summary: `"${owner.ownerRel}" is missing required ${kind} schema facet ${facetAbs}/`,
          kind: "app-schema/facet-completeness",
          scope: owner.ownerRel,
          priority: "high",
          reason: "Every app-schema owner must expose 🎚️config/🧬️schema and 👥️presence/🧬️schema facets.",
          solution: `Create ${facetAbs}/ with all five schemaFormats leaves (and the normative ${canonicalPrimaryFilenameForKind(taxonomy.semanticManifestFileKindId)}).`,
        });
        continue;
      }
      if (facetSource.state !== "directory") {
        breaches.push(sourceBreach(owner.ownerRel, facetAbs, facetSource.state));
        continue;
      }
      for (const [formatId, format] of schemaFacetFormatEntries(facetAbs, taxonomy)) {
        const leafFilename = canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy),
          leafRel = `${facetAbs}/${leafFilename}`,
          leafSource = policySourceText(repoRoot, leafRel, operations);
        if (leafSource.state === "file") continue;
        if (leafSource.state !== "missing") {
          breaches.push(sourceBreach(owner.ownerRel, leafRel, leafSource.state));
          continue;
        }
        breaches.push({
          id: `app-schema-leaf-missing-${leafRel}`,
          summary: `"${facetAbs}" is missing schemaFormats leaf ${leafFilename} (${formatId})`,
          kind: "app-schema/facet-completeness",
          scope: owner.ownerRel,
          priority: "high",
          reason: "Each schema facet must carry every schemaFormats leaf for its facet kind from 🔣️taxonomy.json.",
          solution: `Add handcrafted ${leafRel}.`,
        });
      }
      const role = policyAppSchemaFacetRole(kind),
        normativeFileKindId = normativeByFacet[role];
      if (!normativeFileKindId) throw new Error(`[taxonomy] no normative surface schema file kind for ${role}.`);
      const normative = canonicalPrimaryFilenameForKind(normativeFileKindId, taxonomy),
        normativeRel = `${facetAbs}/${normative}`,
        normativeSource = policySourceText(repoRoot, normativeRel, operations);
      if (normativeSource.state === "file") continue;
      if (normativeSource.state !== "missing") {
        breaches.push(sourceBreach(owner.ownerRel, normativeRel, normativeSource.state));
        continue;
      }
      breaches.push({
        id: `app-schema-normative-missing-${normativeRel}`,
        summary: `"${facetAbs}" is missing normative surfaceSchemaSpecFilenames leaf ${normative}`,
        kind: "app-schema/facet-completeness",
        scope: owner.ownerRel,
        priority: "high",
        reason: `Within a facet the ${canonicalPrimaryFilenameForKind(taxonomy.semanticManifestFileKindId)} JSON Schema leaf is normative; the other four mirror it.`,
        solution: `Add ${normativeRel} as the source of truth for this facet's fields.`,
      });
    }
  }
  return breaches;
}

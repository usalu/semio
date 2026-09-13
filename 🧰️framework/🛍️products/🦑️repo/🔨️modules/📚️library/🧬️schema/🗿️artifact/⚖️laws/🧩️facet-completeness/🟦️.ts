import { canonicalPrimaryFilenameForKind, loadTaxonomy, schemaFacetFormatEntries, type BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_SCHEMA_FACET_RELS } from "../../📚️facet-leaves/🟦️.ts";

/** 📏 Reports absent facets, configured leaves and normative schema leaves without hiding unreadable sources. */
export function policyArtifactSchemaFacetCompletenessBreaches(repoRoot: string, owners: readonly string[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const taxonomy = loadTaxonomy(),
    normativeByFacet = taxonomy.artifactSchemaSpecFileKinds ?? {},
    breaches: BreachRecord[] = [];
  for (const artRel of owners) {
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`,
        facet = policySourceDirectory(repoRoot, facetAbs, operations);
      if (facet.state !== "directory") {
        const unavailable = facet.state !== "missing";
        breaches.push({
          id: `${unavailable ? "artifact-schema-source-unreadable" : "artifact-schema-facet-missing"}-${facetAbs}`,
          summary: unavailable ? `"${facetAbs}" cannot be admitted: ${facet.state}` : `"${artRel}" is missing required schema facet ${facetRel}/`,
          kind: unavailable ? "artifact-schema/source-unreadable" : "artifact-schema/facet-completeness",
          scope: artRel,
          priority: "high",
          reason: unavailable
            ? "A schema facet that is unreadable, linked, or the wrong source kind is unresolved evidence and cannot be certified as complete."
            : "Every artifact standard/subset must expose 🧬️schema, 🧬️schema/📸️snapshot, and 🧬️schema/🔺️diff facets.",
          solution: unavailable
            ? `Restore readable no-follow directory access to ${facetAbs}/.`
            : `Create ${facetAbs}/ with every configured schema-format leaf and its normative ${canonicalPrimaryFilenameForKind(taxonomy.semanticManifestFileKindId)} schema.`,
        });
        continue;
      }
      for (const [formatId, format] of schemaFacetFormatEntries(facetAbs, taxonomy)) {
        const leafFilename = canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy),
          leafRel = `${facetAbs}/${leafFilename}`,
          source = policySourceText(repoRoot, leafRel, operations);
        if (source.state === "file") continue;
        const unavailable = source.state !== "missing";
        breaches.push({
          id: `${unavailable ? "artifact-schema-source-unreadable" : "artifact-schema-leaf-missing"}-${leafRel}`,
          summary: unavailable ? `"${leafRel}" cannot be admitted: ${source.state}` : `"${facetAbs}" is missing schemaFormats leaf ${leafFilename} (${formatId})`,
          kind: unavailable ? "artifact-schema/source-unreadable" : "artifact-schema/facet-completeness",
          scope: artRel,
          priority: "high",
          reason: unavailable
            ? "A schema leaf that is unreadable, linked, or the wrong source kind is unresolved evidence and cannot be treated as an empty or conforming declaration."
            : "Each schema facet must carry every schemaFormats leaf for its facet kind from 🔣️taxonomy.json.",
          solution: unavailable ? `Restore readable no-follow file access to ${leafRel}.` : `Add handcrafted ${leafRel}.`,
        });
      }
      const normativeFileKindId = normativeByFacet[facetRel];
      if (!normativeFileKindId) continue;
      const normative = canonicalPrimaryFilenameForKind(normativeFileKindId, taxonomy),
        normativeRel = `${facetAbs}/${normative}`,
        normativeSource = policySourceText(repoRoot, normativeRel, operations);
      if (normativeSource.state !== "missing") continue;
      breaches.push({
        id: `artifact-schema-normative-missing-${normativeRel}`,
        summary: `"${facetAbs}" is missing normative artifactSchemaSpecFilenames leaf ${normative}`,
        kind: "artifact-schema/normative-leaf",
        scope: artRel,
        priority: "high",
        reason: `Within a facet the ${canonicalPrimaryFilenameForKind(taxonomy.semanticManifestFileKindId)} JSON Schema leaf is normative; the other four mirror it.`,
        solution: `Add ${normativeRel} as the source of truth for this facet's fields.`,
      });
    }
  }
  return breaches;
}

import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";
import { validateSchemaScopeVocabulary } from "../🪜️audit-schema-scope-vocabulary/🟦️.ts";

/** 📘️ Audits the schema formats, the schema facet kinds and the schema scope vocabulary. */
export function auditSchemaFormatsAndFacets(audit: TaxonomyAudit): void {
  const { taxonomy, problems } = audit;
  for (const [formatId, format] of Object.entries(taxonomy.schemaFormats ?? {})) {
    if ("leafFilename" in (format as unknown as Record<string, unknown>) || "extension" in (format as unknown as Record<string, unknown>)) problems.push(`schemaFormats[${JSON.stringify(formatId)}] contains removed filename fields.`);
    if (!taxonomy.fileKinds[format.fileKindId]) problems.push(`schemaFormats[${JSON.stringify(formatId)}].fileKindId is missing.`);
    if (!["snake", "camel", "kebab"].includes(format.fieldCasing)) problems.push(`schemaFormats[${JSON.stringify(formatId)}].fieldCasing is invalid.`);
  }
  const declaredFacetKindPaths = new Set<string>();
  for (const [kindId, kind] of Object.entries(taxonomy.schemaFacetKinds ?? {})) {
    if (!kind.formats.includes(kind.normativeFormat)) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].formats must include its normative format.`);
    for (const formatId of kind.formats) if (!taxonomy.schemaFormats[formatId]) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}] references missing format ${JSON.stringify(formatId)}.`);
    if (!Array.isArray(kind.facetPathIdentities)) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].facetPathIdentities must be an array of exact facet module paths.`);
    else for (const facetPath of kind.facetPathIdentities) {
      if (typeof facetPath !== "string" || !facetPath || facetPath !== facetPath.normalize("NFC") || facetPath.startsWith("/") || /[\\*?]/u.test(facetPath) || facetPath.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].facetPathIdentities must name exact repository-relative facet modules.`);
      else if (declaredFacetKindPaths.has(facetPath)) problems.push(`schemaFacetKinds declares ${JSON.stringify(facetPath)} under more than one facet kind.`);
      else declaredFacetKindPaths.add(facetPath);
    }
  }
  problems.push(...validateSchemaScopeVocabulary(audit));
}

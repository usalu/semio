import { canonicalPrimaryFilenameForKind, loadTaxonomy, resolveSchemaFacetKind } from "../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../🔍️discovery/📖️source-access/🟦️.ts";

/** 🏷️ Reads the export id declared by a facet's normative JSON Schema. */
export function policyDeclaredSchemaExportName(repoRoot: string, facetRel: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): string | null {
  const taxonomy = loadTaxonomy(),
    facetKind = resolveSchemaFacetKind(facetRel, taxonomy);
  if (!facetKind) return null;
  const facet = taxonomy.schemaFacetKinds?.[facetKind];
  if (!facet) return null;
  const format = taxonomy.schemaFormats[facet.normativeFormat];
  if (!format) return null;
  const source = policySourceText(repoRoot, `${facetRel}/${canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy)}`, operations);
  if (source.state !== "file") return null;
  try {
    const parsed = JSON.parse(source.text) as { title?: unknown };
    return typeof parsed.title === "string" && parsed.title ? parsed.title : null;
  } catch {
    return null;
  }
}

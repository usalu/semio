import { join } from "node:path";
import { canonicalPrimaryFilenameForKind, loadTaxonomy, schemaFacetFormatEntries } from "../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations, type PolicySourceTextResult } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import type { PolicySchemaLeafExtract } from "../../🔍️field-discovery/🧱️contract/🟦️.ts";
import { policyExtractGraphqlSchemaFields } from "../../🔍️field-discovery/🔗️graphql/🟦️.ts";
import { policyExtractJsonSchemaFields } from "../../🔍️field-discovery/🔣️json-schema/🟦️.ts";
import { policyExtractProtobufSchemaFields } from "../../🔍️field-discovery/🛰️protobuf/🟦️.ts";
import { policyExtractTypescriptSchemaFile } from "../../🔍️field-discovery/🟦️typescript/📂️module-resolution/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../🔍️field-discovery/🦀️rust/🟦️.ts";
import { policyDeclaredSchemaExportName } from "../🏷️export-identity/🟦️.ts";

export const POLICY_SCHEMA_FACET_RELS = ["🧬️schema", "🧬️schema/📸️snapshot", "🧬️schema/🔺️diff"] as const;

export type PolicyArtifactSchemaLeaf = Readonly<{
  formatId: string;
  leafFilename: string;
  fieldCasing: string;
  relPath: string;
  sourceState: PolicySourceTextResult["state"];
  extract: PolicySchemaLeafExtract | null;
}>;

/** 🗂️ Loads every configured schema-format leaf while retaining missing and unreadable source state. */
export function policyLoadSchemaFacetLeaves(repoRoot: string, facetRel: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicyArtifactSchemaLeaf[] {
  const taxonomy = loadTaxonomy(),
    expected = policyDeclaredSchemaExportName(repoRoot, facetRel, operations),
    out: PolicyArtifactSchemaLeaf[] = [];
  for (const [formatId, format] of schemaFacetFormatEntries(facetRel, taxonomy)) {
    const leafFilename = canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy),
      relPath = `${facetRel}/${leafFilename}`,
      source = policySourceText(repoRoot, relPath, operations);
    if (source.state !== "file") {
      out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, sourceState: source.state, extract: null });
      continue;
    }
    let extract: PolicySchemaLeafExtract;
    switch (formatId) {
      case "🦀️rust":
        extract = policyExtractRustSchemaFields(source.text, expected);
        break;
      case "🟦️typescript":
        extract = policyExtractTypescriptSchemaFile(join(repoRoot, relPath), source.text, expected);
        break;
      case "🔗️graphql":
        extract = policyExtractGraphqlSchemaFields(source.text, expected);
        break;
      case "🔣️jsonschema":
        extract = policyExtractJsonSchemaFields(source.text);
        break;
      case "🛰️protobuf":
        extract = policyExtractProtobufSchemaFields(source.text, expected);
        break;
      default:
        extract = { typeName: "", fields: [] };
        break;
    }
    out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, sourceState: source.state, extract });
  }
  return out;
}

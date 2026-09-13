import { canonicalPrimaryFilenameForKind } from "../../../🔍️discovery/🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations, type PolicySourceTextResult } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyExtractGraphqlSchemaFields } from "../../🔍️field-discovery/🔗️graphql/🟦️.ts";
import { policyExtractJsonSchemaFields } from "../../🔍️field-discovery/🔣️json-schema/🟦️.ts";
import { policyExtractProtobufSchemaFields } from "../../🔍️field-discovery/🛰️protobuf/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../🔍️field-discovery/🦀️rust/🟦️.ts";
import { policyExtractTypescriptSchemaFile } from "../../🔍️field-discovery/🟦️typescript/📂️module-resolution/🟦️.ts";
import type { PolicySchemaLeafExtract } from "../../🔍️field-discovery/🧱️contract/🟦️.ts";
import { loadTaxonomy } from "../../../🟦️.ts";

export type PolicyAppSchemaFacetLeaf = Readonly<{
  formatId: string;
  leafFilename: string;
  fieldCasing: string;
  relPath: string;
  sourceState: PolicySourceTextResult["state"];
  extract: PolicySchemaLeafExtract | null;
}>;

/** 📚️ Loads configured schema leaves through their accepted format-specific field parsers. */
export function policyLoadAppSchemaFacetLeaves(repoRoot: string, facetRel: string, expectedTypeName: string | null, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicyAppSchemaFacetLeaf[] {
  const taxonomy = loadTaxonomy(),
    formats = taxonomy.schemaFormats ?? {},
    leaves: PolicyAppSchemaFacetLeaf[] = [];
  for (const [formatId, format] of Object.entries(formats)) {
    const leafFilename = canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy),
      relPath = `${facetRel}/${leafFilename}`,
      source = policySourceText(repoRoot, relPath, operations);
    if (source.state !== "file") {
      leaves.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, sourceState: source.state, extract: null });
      continue;
    }
    let extract: PolicySchemaLeafExtract;
    switch (formatId) {
      case "🦀️rust":
        extract = policyExtractRustSchemaFields(source.text, expectedTypeName);
        break;
      case "🟦️typescript":
        extract = policyExtractTypescriptSchemaFile(`${repoRoot}/${relPath}`, source.text, expectedTypeName);
        break;
      case "🔗️graphql":
        extract = policyExtractGraphqlSchemaFields(source.text, expectedTypeName);
        break;
      case "🔣️jsonschema":
        extract = policyExtractJsonSchemaFields(source.text);
        break;
      case "🛰️protobuf":
        extract = policyExtractProtobufSchemaFields(source.text, expectedTypeName);
        break;
      default:
        extract = { typeName: "", fields: [] };
        break;
    }
    leaves.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, sourceState: "file", extract });
  }
  return leaves;
}

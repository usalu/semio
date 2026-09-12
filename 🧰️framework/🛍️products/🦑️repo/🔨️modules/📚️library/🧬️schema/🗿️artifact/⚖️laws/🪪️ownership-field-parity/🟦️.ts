import { dirname, join } from "node:path";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policySchemaFieldDifferences } from "../../../🔍️field-discovery/⚖️comparison/🟦️.ts";
import { policyExtractGraphqlSchemaFields } from "../../../🔍️field-discovery/🔗️graphql/🟦️.ts";
import { policyExtractProtobufSchemaFields } from "../../../🔍️field-discovery/🛰️protobuf/🟦️.ts";
import { policyExtractTypescriptSchemaFields } from "../../../🔍️field-discovery/🟦️typescript/🟦️.ts";
import { policyExtractTypescriptSchemaFile } from "../../../🔍️field-discovery/🟦️typescript/📂️module-resolution/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../🔍️field-discovery/🦀️rust/🟦️.ts";

/** 🪪️ Audits exact document, snapshot and diff fields across authored representations. */
export function policyArtifactOwnershipFieldParity(root: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): { path: string; missing: string[]; extra: string[] }[] {
  const fixtureSource = policySourceText(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🧪️abstraction-ownership/🔣️.json", operations);
  if (fixtureSource.state !== "file") throw new Error(`Artifact ownership fixture is ${fixtureSource.state}.`);
  const fixture = JSON.parse(fixtureSource.text) as { artifactSchemas: string[] },
    paths = new Set(fixture.artifactSchemas);
  for (const path of fixture.artifactSchemas) if (!path.includes("/🔺️diff/")) paths.add(join(dirname(path), "📸️snapshot/🔣️.json"));
  const representations = [
      ["🦀️.rs", policyExtractRustSchemaFields],
      ["🟦️.ts", policyExtractTypescriptSchemaFields],
      ["🔗️.graphql", policyExtractGraphqlSchemaFields],
      ["🛰️.proto", policyExtractProtobufSchemaFields],
    ] as const,
    breaches: { path: string; missing: string[]; extra: string[] }[] = [];
  for (const path of paths) {
    const schemaSource = policySourceText(root, path, operations);
    if (schemaSource.state !== "file") throw new Error(`Artifact ownership schema ${path} is ${schemaSource.state}.`);
    const schema = JSON.parse(schemaSource.text) as { title: string; properties: Record<string, unknown> };
    for (const [filename, extract] of representations) {
      const source = join(dirname(path), filename),
        sourceRead = policySourceText(root, source, operations),
        text = sourceRead.state === "file" ? sourceRead.text : "",
        declaration = filename === "🟦️.ts" ? policyExtractTypescriptSchemaFile(join(root, source), text, schema.title) : extract(text, schema.title),
        difference = policySchemaFieldDifferences(
          Object.keys(schema.properties ?? {}),
          declaration.fields.map((field) => field.name),
        );
      if (!declaration.typeName) difference.missing.unshift(`declaration:${schema.title}`);
      if (difference.missing.length || difference.extra.length) breaches.push({ path: source, ...difference });
    }
  }
  return breaches.sort((left, right) => left.path.localeCompare(right.path));
}

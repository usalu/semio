import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyExtractGraphqlSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🔗️graphql/🟦️.ts";
import { policyExtractProtobufSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🛰️protobuf/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";
import { policyExtractTypescriptSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🟦️typescript/🟦️.ts";
import { policyExtractTypescriptSchemaFile } from "../../../🧬️schema/🔍️field-discovery/🟦️typescript/📂️module-resolution/🟦️.ts";
import { abstractionOwnershipSchema, abstractionOwnershipViolations, type AbstractionOwnership } from "../🧱️contract/🟦️.ts";
import { abstractionOwnershipRustCommands, abstractionOwnershipSchemaFields } from "../🔍️source-projection/🟦️.ts";

/** 🧪️ Compares language-independent ownership vectors with Ajv and real schema representations. */
export function abstractionOwnershipChecks(root: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): number {
  const schema = abstractionOwnershipSchema(root, operations),
    fixturePath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🧪️abstraction-ownership/🔣️.json",
    fixtureSource = policySourceText(root, fixturePath, operations);
  if (fixtureSource.state !== "file") throw new Error(`Abstraction ownership fixture ${fixturePath} is ${fixtureSource.state}.`);
  const fixture = JSON.parse(fixtureSource.text) as {
      cases: (AbstractionOwnership & { name: string; expected: string[] })[];
      artifactSchemas: string[];
      schemaCases: { name: string; schema: Record<string, unknown>; expected: string[] }[];
      sourceCases: { name: string; source: string; expected: string[] }[];
    },
    Ajv = createRequire(import.meta.url)("ajv"),
    validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const row of fixture.cases) {
    const declaration = { owner: row.owner, fields: row.fields, commands: row.commands },
      actual = abstractionOwnershipViolations(declaration, schema);
    if (JSON.stringify(actual) !== JSON.stringify(row.expected)) throw new Error(`[verify abstraction-ownership] ${row.name}: expected ${JSON.stringify(row.expected)}, got ${JSON.stringify(actual)}.`);
    if (validate(declaration) !== (actual.length === 0)) throw new Error(`[verify abstraction-ownership] ${row.name}: implementation disagrees with Ajv.`);
  }
  console.log(`[verify abstraction-ownership] ${fixture.cases.length} ownership vectors agree with Ajv.`);
  for (const row of fixture.schemaCases) {
    const declaration: AbstractionOwnership = { owner: "surface", fields: abstractionOwnershipSchemaFields(row.schema), commands: [] },
      actual = abstractionOwnershipViolations(declaration, schema);
    if (JSON.stringify(actual) !== JSON.stringify(row.expected)) throw new Error(`[verify abstraction-ownership] ${row.name}: expected ${JSON.stringify(row.expected)}, got ${JSON.stringify(actual)}.`);
    if (validate(declaration) !== (actual.length === 0)) throw new Error(`[verify abstraction-ownership] ${row.name}: schema discovery disagrees with Ajv.`);
  }
  console.log(`[verify abstraction-ownership] ${fixture.schemaCases.length} nested schema ownership vectors agree with Ajv.`);
  for (const row of fixture.sourceCases) {
    const declaration: AbstractionOwnership = { owner: "surface", fields: [], commands: abstractionOwnershipRustCommands(row.source) },
      actual = abstractionOwnershipViolations(declaration, schema);
    if (JSON.stringify(actual) !== JSON.stringify(row.expected)) throw new Error(`[verify abstraction-ownership] ${row.name}: expected ${JSON.stringify(row.expected)}, got ${JSON.stringify(actual)}.`);
    if (validate(declaration) !== (actual.length === 0)) throw new Error(`[verify abstraction-ownership] ${row.name}: source declaration disagrees with Ajv.`);
  }
  console.log(`[verify abstraction-ownership] ${fixture.sourceCases.length} command source ownership vectors agree with Ajv.`);
  const validateArtifactFields = new Ajv({ strict: true, allErrors: true }).compile({
      type: "object",
      propertyNames: { not: { enum: schema.$defs.ArtifactExcludedField.enum } },
      additionalProperties: { type: "object", required: ["x-semio-state"], properties: { "x-semio-state": { const: "artifact" } } },
    }),
    artifactRepresentations = [
      ["🦀️.rs", policyExtractRustSchemaFields],
      ["🟦️.ts", policyExtractTypescriptSchemaFields],
      ["🔗️.graphql", policyExtractGraphqlSchemaFields],
      ["🛰️.proto", policyExtractProtobufSchemaFields],
    ] as const,
    typescript = new (globalThis as any).Bun.Transpiler({ loader: "ts" });
  for (const path of fixture.artifactSchemas) {
    const artifactSource = policySourceText(root, path, operations);
    if (artifactSource.state !== "file") throw new Error(`[verify abstraction-ownership] ${path}: ${artifactSource.state}.`);
    const artifact = JSON.parse(artifactSource.text);
    if (!validateArtifactFields(artifact.properties)) throw new Error(`[verify abstraction-ownership] ${path}: app/window state leaks into the artifact contract: ${JSON.stringify(validateArtifactFields.errors)}.`);
    for (const [filename, extract] of artifactRepresentations) {
      const pathName = `${dirname(path)}/${filename}`,
        source = policySourceText(root, pathName, operations);
      if (source.state !== "file") throw new Error(`[verify abstraction-ownership] ${pathName}: ${source.state}.`);
      const declaration = filename === "🟦️.ts" ? policyExtractTypescriptSchemaFile(join(root, pathName), source.text, artifact.title) : extract(source.text, artifact.title);
      if (filename === "🟦️.ts") typescript.scan(source.text);
      if (!declaration.typeName) throw new Error(`[verify abstraction-ownership] ${pathName}: missing document declaration ${artifact.title}.`);
      const misplaced = declaration.fields.filter((field) => field.state && field.state !== "artifact");
      if (misplaced.length) throw new Error(`[verify abstraction-ownership] ${pathName}: non-document field ownership ${JSON.stringify(misplaced)}.`);
      const excluded = abstractionOwnershipViolations({ owner: "artifact", fields: declaration.fields.map((field) => field.name), commands: [] }, schema);
      if (excluded.length) throw new Error(`[verify abstraction-ownership] ${pathName}: live UI/computed state ${JSON.stringify(excluded)}.`);
    }
  }
  console.log(`[verify abstraction-ownership] ${fixture.artifactSchemas.length} artifact contracts expose document state only across five schema formats.`);
  return fixture.cases.length;
}

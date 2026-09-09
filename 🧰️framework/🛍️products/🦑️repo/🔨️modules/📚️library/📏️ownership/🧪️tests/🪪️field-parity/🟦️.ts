import assert from "node:assert/strict";
import Ajv from "ajv";
import ts from "typescript";
import glob from "fast-glob";
import { resolve } from "node:path";
import fixture from "./🔣️.json" with { type: "json" };
import { policyExtractRustSchemaFields, policyExtractTypescriptSchemaFields, policySchemaFieldDifferences, policyArtifactSchemaBreaches, policyDiscoverArtifactSchemaOwners } from "../../../../../../../../📜️script.ts";

/** 🪪️ Checks native field discovery against independent TypeScript AST and Ajv evaluators. */
export function testArtifactFieldParityOracle(): void {
  for (const row of fixture.cases) {
    const source = ts.createSourceFile("fixture.ts", row.typescript, ts.ScriptTarget.Latest, true);
    const declaration = source.statements.find((node) => ts.isInterfaceDeclaration(node) && node.name.text === row.typeName) as ts.InterfaceDeclaration;
    const expected = declaration.members.map((member) => ts.isStringLiteral(member.name!) ? member.name.text : member.name!.getText(source));
    assert.deepEqual(expected, row.expected, row.name);
    const actual = policyExtractRustSchemaFields(row.rust, row.typeName);
    assert.deepEqual(actual.fields.map((field) => field.name), expected, row.name);
    assert.deepEqual(actual.fields.map((field) => field.state), row.state, row.name);
    assert.deepEqual(policyExtractTypescriptSchemaFields(row.typescript, row.typeName).fields.map((field) => field.name), expected, row.name);
  }
  const ajv = new Ajv({ strict: true });
  for (const row of fixture.parity) {
    const actual = policySchemaFieldDifferences(row.reference, row.candidate);
    assert.deepEqual(actual, row.expected);
    const validate = ajv.compile({ type: "object", properties: Object.fromEntries(row.reference.map((name) => [name, {}])), required: row.reference, additionalProperties: false });
    assert.equal(validate(Object.fromEntries(row.candidate.map((name) => [name, null]))), actual.missing.length === 0 && actual.extra.length === 0);
  }
  const root = resolve(import.meta.dir, "../../../../../../../../");
  const owners = glob.sync(fixture.discovery.pattern, { cwd: root, onlyDirectories: true, ignore: fixture.discovery.ignore }).sort();
  assert.deepEqual(policyDiscoverArtifactSchemaOwners(root), owners);
  assert.equal(policyArtifactSchemaBreaches(root).some((breach) => breach.id.startsWith("artifact-schema-diff-artifact-entry-")), false);
  for (const breach of policyArtifactSchemaBreaches(root)) assert(owners.includes(breach.scope), `schema policy inspected misplaced owner ${breach.scope}`);
  console.log(`[DEBUG] schema policy discovered ${owners.length} standard/subset owners matching independent fast-glob discovery`);
  console.log("[DEBUG] schema field discovery matched independent TypeScript AST names; missing/extra field reports matched Ajv exact-record validation");
}

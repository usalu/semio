import assert from "node:assert/strict";
import Ajv from "ajv";
import ts from "typescript";
import glob from "fast-glob";
import { dirname, resolve } from "node:path";
import fixture from "../../🧫️fixtures/🪪️field-parity/🔣️.json" with { type: "json" };
import { policyExtractRustSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";
import { policyExtractTypescriptSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🟦️typescript/🟦️.ts";
import { policyExtractGraphqlSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🔗️graphql/🟦️.ts";
import { policySchemaFieldDifferences } from "../../../🧬️schema/🔍️field-discovery/⚖️comparison/🟦️.ts";
import { policyArtifactSchemaBreaches, policyArtifactOwnershipFieldParity, policyDiscoverArtifactSchemaOwners } from "../../../../../../../../📜️script.ts";

/** 🪪️ Checks native field discovery against independent TypeScript AST and Ajv evaluators. */
export function testArtifactFieldParityOracle(): void {
  for (const row of fixture.graphql) {
    const source = ts.createSourceFile("fixture.ts", row.typescript, ts.ScriptTarget.Latest, true);
    const declaration = source.statements.find(ts.isInterfaceDeclaration)!;
    const names = declaration.members.map((member) => member.name!.getText(source));
    assert.deepEqual(names, row.expected, row.name);
    const actual = policyExtractGraphqlSchemaFields(row.source, "Fixture");
    assert.deepEqual(actual.fields.map((field) => field.name), names, row.name);
    assert.deepEqual(actual.fields.map((field) => field.state), row.state, row.name);
  }
  for (const row of fixture.typescriptModules) {
    const files: Record<string, string> = row.files;
    const host = ts.createCompilerHost({ noLib: true });
    host.getSourceFile = (name, languageVersion) => files[name] === undefined ? undefined : ts.createSourceFile(name, files[name], languageVersion, true);
    host.fileExists = (name) => files[name] !== undefined;
    host.readFile = (name) => files[name];
    host.resolveModuleNames = (names, from) => names.map((name) => {
      const resolvedFileName = resolve(dirname(from), name);
      return files[resolvedFileName] === undefined ? undefined : { resolvedFileName, extension: ts.Extension.Ts };
    });
    const program = ts.createProgram(["/root.ts"], { noLib: true }, host);
    const checker = program.getTypeChecker();
    const source = program.getSourceFile("/root.ts")!;
    const module = checker.getSymbolAtLocation(source)!;
    const exported = checker.getExportsOfModule(module).find((symbol) => symbol.name === "Fixture");
    const symbol = exported && (exported.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(exported) : exported);
    const names = symbol ? checker.getPropertiesOfType(checker.getDeclaredTypeOfSymbol(symbol)).map((field) => field.name) : [];
    assert.deepEqual(names, row.expected, `independent TypeScript compiler: ${row.name}`);
    const actual = policyExtractTypescriptSchemaFields(files["/root.ts"]!, "Fixture", (specifier, from) => {
      const moduleId = resolve(dirname(from), specifier);
      return files[moduleId] === undefined ? null : { moduleId, text: files[moduleId] };
    }, "/root.ts");
    assert.deepEqual(actual.fields.map((field) => field.name), names, row.name);
    assert.equal(actual.typeName, names.length ? "Fixture" : "", row.name);
    if ("states" in row && row.states) {
      const states = checker.getPropertiesOfType(checker.getDeclaredTypeOfSymbol(symbol!)).map((field) => field.getJsDocTags(checker).find((tag) => tag.name === "state")?.text?.map((part) => part.text).join("").trim() ?? "");
      assert.deepEqual(states, row.states, row.name);
      assert.deepEqual(actual.fields.map((field) => field.state), states, row.name);
    }
  }
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
  const breaches = policyArtifactSchemaBreaches(root);
  assert.equal(breaches.some((breach) => breach.id.startsWith("artifact-schema-diff-artifact-entry-")), false);
  for (const breach of breaches) assert(owners.includes(breach.scope), `schema policy inspected misplaced owner ${breach.scope}`);
  assert.deepEqual(policyArtifactOwnershipFieldParity(root).filter((breach) => breach.path.endsWith("/🟦️.ts") && breach.missing.some((field) => field.startsWith("declaration:"))), []);
  console.log(`[DEBUG] schema policy discovered ${owners.length} standard/subset owners matching independent fast-glob discovery`);
  console.log(`[DEBUG] ${fixture.typescriptModules.length} alias/module graphs matched TypeScript compiler symbols; ${fixture.graphql.length} GraphQL metadata cases matched equivalent TypeScript AST field sets`);
  console.log("[DEBUG] schema field discovery matched independent TypeScript AST names; missing/extra field reports matched Ajv exact-record validation");
}

import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";
import { semioSchemaAjvV1 } from "../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import { parseFeature } from "../../🥒️gherkin/🟦️.ts";
import type { AdapterContext, TestAdapter } from "../🟦️.ts";
import fixture from "../../🧫️fixtures/🔌️adapter-ownership/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔌️adapter-ownership/🔣️.json" with { type: "json" };
import protocol from "../../🧬️schema/🔣️.json" with { type: "json" };
const root = resolve(import.meta.dir, "../../../../..");
const scenarioSchema = { $defs: protocol.$defs, $ref: "#/$defs/Scenario" };
const ajv = semioSchemaAjvV1({ strictRequired: false });
const validScenario = ajv.compile(scenarioSchema);

test("portable neutral ownership corpus agrees with the independent schema oracle", () => {
  expect(ajv.validate(schema, fixture)).toBe(true);
  expect(validateJsonSchemaSubset(schema, fixture)).toEqual([]);
  expect(protocol.$id).toBe("https://json.schemas.assets.semio-tech.com/framework/test/schema.json");
  expect(protocol["x-semio"].owner).toBe("🧰️framework/🔨️modules/🧪️test");
  for (const file of fixture.removedFiles) expect(existsSync(resolve(root, file)), file).toBe(false);
  for (const row of fixture.removedExports) {
    const source = readFileSync(resolve(root, row.source), "utf8"), ast = ts.createSourceFile(row.source, source, ts.ScriptTarget.Latest, true);
    const exports = new Set<string>();
    for (const node of ast.statements) {
      if (ts.isExportDeclaration(node) && node.exportClause && ts.isNamedExports(node.exportClause)) for (const item of node.exportClause.elements) exports.add(item.name.text);
      if (!(ts.canHaveModifiers(node) && ts.getModifiers(node)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword))) continue;
      if (ts.isVariableStatement(node)) {
        for (const declaration of node.declarationList.declarations) if (ts.isIdentifier(declaration.name)) exports.add(declaration.name.text);
      } else if ("name" in node && node.name && ts.isIdentifier(node.name as ts.Node)) exports.add((node.name as ts.Identifier).text);
    }
    for (const name of row.names) expect(exports.has(name), name).toBe(false);
  }
});

for (const row of fixture.cases) test(`real feature vectors preserve neutral adapter behavior: ${row.directory}`, async () => {
  const source = readFileSync(resolve(root, row.directory, "🥒️.feature"), "utf8"), feature = parseFeature(source);
  expect(feature.errors).toEqual([]);
  expect(feature.scenarios.map((scenario) => scenario.id)).toEqual(row.scenarios);
  const adapter = (await import(pathToFileURL(resolve(root, row.directory, "🟦️.ts")).href)).default as TestAdapter;
  expect(adapter.implementation).toBe("typescript");
  expect(Object.keys(adapter.scenarios)).toEqual(row.scenarios);
  for (const scenario of feature.scenarios) {
    const value = JSON.parse(JSON.stringify(scenario));
    expect(validScenario(value), JSON.stringify(ajv.errors)).toBe(true);
    expect(validateJsonSchemaSubset(scenarioSchema, value)).toEqual([]);
    for (const invalid of [{ ...value, level: "unbounded" }, { ...value, mode: "unspecified" }]) {
      expect(validScenario(invalid)).toBe(false);
      expect(validateJsonSchemaSubset(scenarioSchema, invalid).length).toBeGreaterThan(0);
    }
    const unsupported = (): never => { throw Error("unexpected fixture or coordinator access by vector-only adapter"); };
    const context = { scenario, repoRoot: root, workDir: root, artifactDir: root, seed: "0", role: "subject", fixture: unsupported, fixtureBytes: unsupported, copyFixture: unsupported, subjectRawBytes: unsupported, artifact: unsupported, row: unsupported, get plan(): never { return unsupported(); } } satisfies AdapterContext;
    const handler = adapter.scenarios[scenario.id]!;
    expect(typeof handler.subject).toBe("function");
    const subject = await handler.subject!(context);
    expect(subject.projection).toBeDefined();
    if ("oracle" in row) {
      expect(typeof handler.oracle).toBe("function");
      const oracle = await handler.oracle!(Object.assign(Object.create(context), { role: "oracle" }));
      expect(subject.projection).toEqual(oracle.projection);
    } else expect(handler.oracle).toBeUndefined();
  }
});

import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

interface Fixture { readonly schemaVersion: 1; readonly neutralFixture: string; readonly neutralSchema: string; readonly neutralRouter: string; readonly mountedFixture: string; readonly mountedSchema: string; readonly mountedRouter: string; readonly neutralLaws: readonly string[]; readonly mountedLaws: readonly string[]; readonly mountedCaseIds: readonly string[]; readonly neutralCaseIds: readonly string[] }
const root = resolve(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/🧱️rust-source-direction/🔐️pool-use-ownership/🔣️.json", import.meta.url), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(new URL("../../../🧬️schema/🧱️rust-source-direction/🔐️pool-use-ownership/🔣️.json", import.meta.url), "utf8"));
const source = (path: string): string => readFileSync(resolve(root, path), "utf8");
const json = (path: string): any => JSON.parse(source(path));

function nativeRosters(path: string, name: string): string[][] {
  const tree = ts.createSourceFile(path, source(path), ts.ScriptTarget.Latest, true);
  const owner = tree.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === name)!;
  const rosters: string[][] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isPropertyAssignment(node) && node.name.getText(tree) === "laws" && ts.isArrayLiteralExpression(node.initializer)) rosters.push(node.initializer.elements.map(element => ts.isStringLiteral(element) ? element.text : ""));
    ts.forEachChild(node, visit);
  };
  visit(owner);
  return rosters;
}

test("closed pool use ownership corpus retains neutral and mounted original laws", () => {
  const oracle = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(oracle(fixture)).toBe(true);
  expect(validateJsonSchemaSubset(schema, fixture)).toEqual([]);
  for (const candidate of [{ ...fixture, unknown: true }, { ...fixture, neutralLaws: fixture.neutralLaws.slice(1) }, { ...fixture, mountedCaseIds: [] }]) {
    expect(oracle(candidate)).toBe(false);
    expect(validateJsonSchemaSubset(schema, candidate).length).toBeGreaterThan(0);
  }
});

test("neutral pool and mounted database have separate closed authority and original native clients", () => {
  const mounted = json(fixture.mountedFixture), mountedSchema = json(fixture.mountedSchema), neutral = json(fixture.neutralFixture), neutralModule = json(fixture.neutralSchema);
  const mountedOracle = new Ajv({ strict: true, allErrors: true }).compile(mountedSchema);
  const neutralSchema = neutralModule.$defs.UseFixture;
  const neutralOracle = new Ajv({ strict: true, allErrors: true }).compile(neutralSchema);
  expect(mountedOracle(mounted)).toBe(true);
  expect(validateJsonSchemaSubset(mountedSchema, mounted)).toEqual([]);
  expect(neutralOracle(neutral)).toBe(true);
  expect(validateJsonSchemaSubset(neutralSchema, neutral)).toEqual([]);
  expect(mounted.cases.map((row: { id: string }) => row.id)).toEqual([...fixture.mountedCaseIds]);
  expect(neutral.cases).toHaveLength(5);
  expect(neutral.cases.map((row: { id: string }) => row.id)).toEqual([...fixture.neutralCaseIds]);
  expect(Object.hasOwn(neutral, "mountedCases")).toBe(false);
  expect(neutralOracle({ ...neutral, mountedCases: mounted.cases })).toBe(false);
  expect(validateJsonSchemaSubset(neutralSchema, { ...neutral, mountedCases: mounted.cases }).length).toBeGreaterThan(0);
  const path = fixture.neutralRouter, tree = ts.createSourceFile(path, source(path), ts.ScriptTarget.Latest, true);
  const owner = tree.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "WorkerPoolUseCheckScript")!.getText(tree);
  expect(owner).not.toContain("🛍️products");
  expect(owner).not.toContain("mountedCases");
  expect(nativeRosters(path, "WorkerPoolUseCheckScript")).toEqual([[...fixture.neutralLaws]]);
  expect(nativeRosters(fixture.mountedRouter, "DocumentMountSingleFlightCheckScript")[0]).toEqual(expect.arrayContaining([...fixture.mountedLaws]));
  const higherTree = ts.createSourceFile(fixture.mountedRouter, source(fixture.mountedRouter), ts.ScriptTarget.Latest, true);
  const higher = higherTree.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "DocumentMountSingleFlightCheckScript")!.getText(higherTree);
  expect(higher).toContain('"🧫️fixtures/🔐️pool-use/🔣️.json"');
  expect(higher).toContain("mountedPoolUse.cases");
  for (const marker of ["missing mounted pool-use marker", "missing authority pool-use marker", "missing sync-hello pool-use marker", "missing mounted pool-use law"]) expect(higher).toContain(marker);
  console.log("[DEBUG] pool-use-owned-authorities neutral=5 mounted=5 native=3+3");
});

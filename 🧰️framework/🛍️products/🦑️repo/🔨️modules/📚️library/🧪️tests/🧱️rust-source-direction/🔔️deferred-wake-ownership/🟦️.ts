import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

interface Fixture { readonly schemaVersion: 1; readonly neutralFixture: string; readonly neutralSchema: string; readonly neutralRouter: string; readonly writerFixture: string; readonly writerSchema: string; readonly writerRouter: string; readonly caseIds: readonly string[]; readonly neutralLaws: readonly string[]; readonly writerLaws: readonly string[]; readonly deniedNeutralKeys: readonly string[]; readonly writerMarkers: readonly string[]; readonly neutralMarkers: readonly string[] }
const root = resolve(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/🧱️rust-source-direction/🔔️deferred-wake-ownership/🔣️.json", import.meta.url), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(new URL("../../../🧬️schema/🧱️rust-source-direction/🔔️deferred-wake-ownership/🔣️.json", import.meta.url), "utf8"));
const source = (path: string): string => readFileSync(resolve(root, path), "utf8");
const json = (path: string): any => JSON.parse(source(path));
function owner(path: string, name: string): ts.ClassDeclaration {
  const tree = ts.createSourceFile(path, source(path), ts.ScriptTarget.Latest, true);
  return tree.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === name) as ts.ClassDeclaration;
}
function nativeGroups(node: ts.Node, executor: string): unknown[] {
  const groups: unknown[] = [];
  const literal = (value: ts.Node): unknown => {
    if (ts.isStringLiteral(value)) return value.text;
    if (ts.isArrayLiteralExpression(value)) return value.elements.map(literal);
    if (ts.isObjectLiteralExpression(value)) return Object.fromEntries(value.properties.map(property => {
      if (!ts.isPropertyAssignment(property)) throw Error("Nonliteral native group property");
      return [property.name.getText(), literal(property.initializer)];
    }));
    throw Error("Nonliteral native group authority");
  };
  const visit = (child: ts.Node): void => {
    if (ts.isCallExpression(child) && ts.isIdentifier(child.expression) && child.expression.text === executor) {
      expect(child.arguments).toHaveLength(1);
      const options = child.arguments[0]!;
      if (!ts.isObjectLiteralExpression(options)) throw Error("Nonliteral native executor options");
      const roster = options.properties.filter(property => ts.isPropertyAssignment(property) && property.name.getText() === "groups") as ts.PropertyAssignment[];
      expect(roster).toHaveLength(1);
      if (!ts.isArrayLiteralExpression(roster[0]!.initializer)) throw Error("Nonliteral native executor groups");
      groups.push(...roster[0]!.initializer.elements.map(literal));
    }
    ts.forEachChild(child, visit);
  };
  visit(node);
  return groups;
}

test("closed deferred wake ownership corpus retains all original controller and scheduler witnesses", () => {
  const oracle = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(oracle(fixture)).toBe(true);
  expect(validateJsonSchemaSubset(schema, fixture)).toEqual([]);
  for (const candidate of [{ ...fixture, unknown: true }, { ...fixture, caseIds: fixture.caseIds.slice(1) }, { ...fixture, writerLaws: fixture.writerLaws.slice(1) }, { ...fixture, neutralLaws: [...fixture.neutralLaws.slice(1), fixture.neutralLaws[1]] }, { ...fixture, neutralMarkers: [...fixture.neutralMarkers.slice(1), "changed-marker"] }]) {
    expect(oracle(candidate)).toBe(false);
    expect(validateJsonSchemaSubset(schema, candidate).length).toBeGreaterThan(0);
  }
});

test("general deferred wake and specific writer have closed separate current fixture authority", () => {
  const neutral = json(fixture.neutralFixture), neutralSchema = json(fixture.neutralSchema).$defs.DeferredWakeFixture, writer = json(fixture.writerFixture), writerSchema = json(fixture.writerSchema);
  for (const [contract, corpus] of [[neutralSchema, neutral], [writerSchema, writer]]) {
    const oracle = new Ajv({ strict: true, allErrors: true }).compile(contract);
    expect(oracle(corpus)).toBe(true);
    expect(validateJsonSchemaSubset(contract, corpus)).toEqual([]);
    expect(oracle({ ...corpus, unknown: true })).toBe(false);
    expect(validateJsonSchemaSubset(contract, { ...corpus, unknown: true }).length).toBeGreaterThan(0);
    expect(corpus.cases.map((row: { id: string }) => row.id)).toEqual([...fixture.caseIds]);
  }
  const neutralOracle = new Ajv({ strict: true, allErrors: true }).compile(neutralSchema);
  for (const key of fixture.deniedNeutralKeys) expect(source(fixture.neutralFixture)).not.toContain(JSON.stringify(key));
  const refused = [
    { ...neutral, refusal: writer.refusal },
    { ...neutral, retryEpoch: writer.retryEpoch },
    { ...neutral, runtimeMarkers: { ...neutral.runtimeMarkers, writer: writer.runtimeMarkers.writer } },
    ...fixture.deniedNeutralKeys.flatMap(key => [
      { ...neutral, capacity: { ...neutral.capacity, [key]: 1 } },
      { ...neutral, cases: [{ ...neutral.cases[0], [key]: 1 }, ...neutral.cases.slice(1)] },
      { ...neutral, cases: [{ ...neutral.cases[0], expected: { ...neutral.cases[0].expected, [key]: 1 } }, ...neutral.cases.slice(1)] },
    ]),
  ];
  for (const candidate of refused) {
    expect(neutralOracle(candidate)).toBe(false);
    expect(validateJsonSchemaSubset(neutralSchema, candidate).length).toBeGreaterThan(0);
  }
  expect(Object.hasOwn(neutral, "refusal")).toBe(false);
  expect(Object.hasOwn(neutral, "retryEpoch")).toBe(false);
  expect(neutralOracle({ ...neutral, refusal: writer.refusal })).toBe(false);
  expect(validateJsonSchemaSubset(neutralSchema, { ...neutral, refusal: writer.refusal }).length).toBeGreaterThan(0);
  expect(source(fixture.neutralRouter)).not.toContain("🛍️products");
  const lower = owner(fixture.neutralRouter, "WorkerDeferredWakeCheckScript"), higherOwner = owner(fixture.writerRouter, "WalWriterAuthorityCheckScript"), higher = higherOwner.getText();
  expect(nativeGroups(lower, "runExactCargoLaws")).toEqual([{ package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: [...fixture.neutralLaws] }]);
  expect(nativeGroups(higherOwner, "runRepositoryExactCargoLaws")).toEqual([{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws: [...fixture.writerLaws] }]);
  expect(higher).toContain("writerDeferredWake.cases");
  expect(higher).toContain("writerDeferredWake.retryEpoch");
  expect(writer.runtimeMarkers.writer).toEqual([...fixture.writerMarkers]);
  expect(neutral.runtimeMarkers.async).toEqual([...fixture.neutralMarkers]);
  expect(Object.hasOwn(neutral.runtimeMarkers, "writer")).toBe(false);
  console.log(`[DEBUG] deferred-wake-owned-authorities neutral=5 writer=5 native=${fixture.neutralLaws.length}+${fixture.writerLaws.length}`);
});

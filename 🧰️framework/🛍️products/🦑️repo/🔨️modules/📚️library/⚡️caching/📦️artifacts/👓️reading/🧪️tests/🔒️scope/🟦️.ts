import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };

test("neutral repository readers retain supplied actual storage and accepted consumers without opening a replacement store", () => {
  const path = resolve(import.meta.dir, "../../🟦️.ts"), syntax = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true), names: string[] = [], calls: string[] = []; let parameters: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isFunctionDeclaration(node) && node.name) { names.push(node.name.text); if (node.name.text === "withRepositoryArtifactV1") parameters = node.parameters.map(row => row.name.getText(syntax)); }
    if (ts.isCallExpression(node)) calls.push(node.expression.getText(syntax));
    ts.forEachChild(node, visit);
  };
  visit(syntax);
  expect(parameters).toEqual(fixture.parameters);
  const { parameters: _parameters, requiredCalls: _calls, exports: _exports, ...protocol } = fixture;
  expect(new Ajv({ strict: true }).validate({ const: protocol }, schema.definitions.Protocol.const)).toBe(true);
  expect(calls).not.toContain("readFile");
  for (const name of fixture.exports) expect(names).toContain(name);
  for (const call of fixture.requiredCalls) expect(calls).toContain(call);
  expect(calls).not.toContain("withRepositoryCacheStoreV1");
  console.log("[DEBUG] Independent TypeScript AST identifies actual caller-supplied storage, original controls and accepted reader drainage");
});



import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { basename } from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import schema from "../../../📒️ledger/🧬️schema/🔎️observation.json" with { type: "json" };
import fixture from "../../../📒️ledger/🧫️fixtures/🔎️observation.json" with { type: "json" };

test("optional ledger refusal retains exact bounded physical predicate observations", () => {
  const source = readFileSync(new URL("../../../📒️ledger/🟨️.mjs", import.meta.url), "utf8"), tree = ts.createSourceFile("ledger.mjs", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const helper = tree.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "optionalLedgerFileObservationV1");
  expect(helper).toBeDefined();
  let regular: ts.ArrowFunction | undefined;
  const visit = (node: ts.Node): void => { if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.name.text === "regular" && node.initializer && ts.isArrowFunction(node.initializer)) regular = node.initializer; ts.forEachChild(node, visit); };
  visit(tree); expect(regular).toBeDefined();
  const observe = new Function("basename", helper!.getText(tree) + ";return optionalLedgerFileObservationV1;")(basename), make = new Function("lstatSync", "parent", "optionalLedgerFileObservationV1", "return " + regular!.getText(tree)), validate = new Ajv({ strict: true }).compile(schema), db = new Database(":memory:");
  try {
    for (const row of fixture.cases) {
      const value = row.observation;
      expect(validate(value), row.id + JSON.stringify(validate.errors)).toBe(true);
      const oracle = db.query("SELECT CASE WHEN json_extract(?1,'$.regular')=0 THEN 'not-regular' WHEN json_extract(?1,'$.symbolicLink')=1 THEN 'symbolic-link' WHEN json_extract(?1,'$.links')<>1 THEN 'link-count' WHEN json_extract(?1,'$.sameDevice')=0 THEN 'device' WHEN json_extract(?1,'$.bytes')>json_extract(?1,'$.maximumBytes') THEN 'extent' ELSE NULL END reason").get(JSON.stringify(value)) as { reason: string | null };
      expect(oracle.reason, row.id).toBe(value.reason);
      const facts = { isFile: () => value.regular, isSymbolicLink: () => value.symbolicLink, nlink: value.links, dev: value.sameDevice ? 7 : 8, size: value.bytes };
      expect(observe(value.leaf, value.maximumBytes, facts, { dev: 7 }), row.id).toEqual(value);
      let reads = 0; const guard = make(() => { reads++; return facts; }, { dev: 7 }, observe), path = value.leaf;
      if (value.reason === null) expect(guard(path, value.maximumBytes), row.id).toBe(facts);
      else {
        let failure: Error | undefined; try { guard(path, value.maximumBytes); } catch (error) { failure = error as Error; }
        expect(failure, row.id).toBeInstanceOf(Error);
        const prefix = "Original bounded optional ledger file required: ";
        expect(failure!.message.startsWith(prefix), row.id).toBe(true);
        const observation = JSON.parse(failure!.message.slice(prefix.length));
        expect(validate(observation), row.id).toBe(true); expect(observation, row.id).toEqual(value);
        expect(failure!.message.length, row.id).toBeLessThan(512);
      }
      expect(reads, row.id).toBe(1);
    }
  } finally { db.close(); }
});

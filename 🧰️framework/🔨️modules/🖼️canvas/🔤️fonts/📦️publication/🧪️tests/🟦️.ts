import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { join, relative, resolve } from "node:path";
import { readFileSync } from "node:fs";
import ts from "typescript";
import fixture from "./🔣️.json" with { type: "json" };

test("neutral Infinite fonts retain managed scratch, actual Rust reader and selected publication mode", async () => {
  const repository = resolve(import.meta.dir, "../../../../../.."), path = join(repository, fixture.producer), source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true), calls: string[] = [];
  const visit = (node: ts.Node): void => { if (ts.isCallExpression(node)) calls.push(node.expression.getText(source)); ts.forEachChild(node, visit); };
  visit(source);
  for (const call of fixture.requiredCalls) expect(calls).toContain(call);
  for (const call of fixture.forbiddenCalls) expect(calls).not.toContain(call);
  const { fontAssetPublicationRootV1 } = await import("../🟦️.ts"), root = resolve("/products"), database = new Database(":memory:"), outputs = new Set<string>();
  try {
    for (const mode of fixture.modes) {
      const output = fontAssetPublicationRootV1(root, mode);
      expect(relative(root, output).replaceAll("\\", "/")).toBe(`${fixture.namespace}/${mode}/${fixture.leaf}`);
      expect((database.query("select instr(?1,?2)=1 and instr(?1,'dist')=0 confined").get(output, root) as { confined: number }).confined).toBe(1);
      outputs.add(output);
    }
    expect(outputs.size).toBe(2);
    for (const mode of fixture.invalidModes) expect(() => fontAssetPublicationRootV1(root, mode)).toThrow();
    for (const directory of fixture.invalidRoots) expect(() => fontAssetPublicationRootV1(directory, "prototype")).toThrow();
  } finally { database.close(); }
});

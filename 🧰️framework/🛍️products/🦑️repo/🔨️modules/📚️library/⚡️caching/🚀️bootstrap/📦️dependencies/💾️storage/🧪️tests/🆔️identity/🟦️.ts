import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import corpus from "./🔣️.json";

/** 🆔️ Lists every `lstatSync`/`fstatSync`/`statSync` result whose `dev` or `ino` the module reads or hands to another call, with whether it requests `bigint`. */
async function identityStats(): Promise<readonly Readonly<{ name: string; line: number; bigint: boolean }>[]> {
  const ts = await import("typescript");
  const source = readFileSync(new URL(corpus.module, import.meta.url), "utf8"), tree = ts.createSourceFile("storage.ts", source, ts.ScriptTarget.Latest, true);
  const rows: { name: string; line: number; bigint: boolean }[] = [];
  const bigint = (call: import("typescript").CallExpression): boolean => call.arguments.some(argument => ts.isObjectLiteralExpression(argument) && argument.properties.some(property => ts.isPropertyAssignment(property) && property.name.getText(tree) === "bigint" && property.initializer.kind === ts.SyntaxKind.TrueKeyword));
  const visit = (node: import("typescript").Node, scope: import("typescript").Node): void => {
    const next = ts.isFunctionLike(node) ? node : scope;
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer && ts.isCallExpression(node.initializer) && /^(?:l|f)?statSync$/.test(node.initializer.expression.getText(tree))) {
      const name = node.name.text, call = node.initializer, reads = new RegExp("\\b" + name + "\\??\\.(?:" + corpus.identityFields.join("|") + ")\\b|[(,]\\s*" + name + "\\s*[),]");
      if (reads.test(scope.getText(tree))) rows.push({ name, line: tree.getLineAndCharacterOfPosition(call.getStart(tree)).line + 1, bigint: bigint(call) });
    }
    ts.forEachChild(node, child => visit(child, next));
  };
  visit(tree, tree);
  return rows;
}

test("observed NTFS file IDs exceed double precision, so identities must be exact bigint strings", () => {
  for (const row of corpus.observed) {
    expect(BigInt(row.ino) > BigInt(Number.MAX_SAFE_INTEGER), row.id).toBe(true);
    expect(String(Number(BigInt(row.ino))), row.id).toBe(row.roundedByDouble);
    expect(String(BigInt(row.ino)), row.id).toBe(row.ino);
    expect(row.roundedByDouble === row.ino, row.id).toBe(false);
  }
});

test("every captured or compared dependency input identity comes from a bigint stat", async () => {
  const rows = await identityStats();
  expect(rows.length).toBeGreaterThan(0);
  expect(rows.filter(row => !row.bigint)).toEqual([]);
});

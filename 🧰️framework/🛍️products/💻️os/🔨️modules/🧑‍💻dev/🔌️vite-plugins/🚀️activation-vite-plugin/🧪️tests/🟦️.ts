import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import ts from "typescript";

const path = new URL("../🏭️runtime/🟦️.ts", import.meta.url);
const tree = ts.createSourceFile(path.pathname, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
const calls = (name: string): ts.CallExpression[] => {
  const found: ts.CallExpression[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.expression.getText(tree) === name) found.push(node);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  return found;
};

test("lazy builds receive the one process envelope once and issue a derived invocation and control per build", () => {
  const receives = calls("receiveScriptProcessInvocation");
  expect(receives.length).toBe(1);
  const enclosing = (node: ts.Node): string[] => { const names: string[] = []; for (let row = node.parent; row; row = row.parent) if (ts.isVariableDeclaration(row)) names.push(row.name.getText(tree)); return names; };
  expect(enclosing(receives[0]!)).toContain("serverInvocation");
  expect(calls("withHandledScriptInvocationSignalV1").length).toBe(1);
  const controls = calls("scriptCompilerAdmissionControlV1");
  expect(controls.length).toBe(1);
  expect(controls[0]!.arguments[0]!.getText(tree)).toBe("child");
  expect(readFileSync(path, "utf8")).not.toMatch(/control\s*=\s*\{\s*\.\.\./);
});

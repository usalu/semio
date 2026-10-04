/** ⏳️ Every `📜️script.ts` awaits (or returns, voids, binds) each call to an async runner the repository library exports, so
 * a runner failure reaches the script router instead of floating as an unhandled rejection past the command's exit. */
import { expect, test } from "bun:test";
import ts from "typescript";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";

const handledParent = (node: ts.Node): boolean => ts.isAwaitExpression(node) || ts.isReturnStatement(node) || ts.isVoidExpression(node) || ts.isVariableDeclaration(node) || ts.isPropertyAccessExpression(node) || ts.isCallExpression(node) || ts.isArrayLiteralExpression(node) || ts.isPropertyAssignment(node) || (ts.isArrowFunction(node) && !ts.isBlock(node.body));

/** 🔎️ Un-awaited calls of `runners` imported by name into one script source. */
function floatingRunnerCalls(path: string, source: string, runners: ReadonlySet<string>): string[] {
  const ast = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), local = new Set<string>(), found: string[] = [];
  ast.forEachChild((node) => {
    if (ts.isImportDeclaration(node) && node.importClause?.namedBindings && ts.isNamedImports(node.importClause.namedBindings))
      for (const element of node.importClause.namedBindings.elements) if (runners.has((element.propertyName ?? element.name).text)) local.add(element.name.text);
  });
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && local.has(node.expression.text)) {
      let parent = node.parent;
      while (parent && (ts.isParenthesizedExpression(parent) || ts.isAsExpression(parent))) parent = parent.parent;
      if (!parent || !handledParent(parent)) found.push(`${path}:${ast.getLineAndCharacterOfPosition(node.getStart()).line + 1} ${node.expression.text}`);
    }
    ts.forEachChild(node, visit);
  };
  visit(ast);
  return found;
}

test("the floating-runner detector flags a bare call and admits awaited, returned and voided ones", () => {
  const runners = new Set(["runVitest"]);
  const sample = `import { runVitest } from "x";\nclass A { run(): void { runVitest("a", [], "c"); } }\nclass B { async run(): Promise<void> { await runVitest("a", [], "c"); } }\nconst c = () => runVitest("a", [], "c");\nfunction d() { return runVitest("a", [], "c"); }\nvoid runVitest("a", [], "c");`;
  expect(floatingRunnerCalls("sample.ts", sample, runners)).toEqual(["sample.ts:2 runVitest"]);
});

test("no repository script floats a call to an async library runner", async () => {
  const root = findWorkspaceRoot(import.meta.dir);
  const library = await import("../../📦️packages/🟦️typescript/🟦️.ts");
  const runners = new Set(Object.entries(library).filter(([, value]) => typeof value === "function" && value.constructor?.name === "AsyncFunction").map(([name]) => name));
  expect(runners.has("runVitest")).toBe(true);
  expect(runners.has("runRepositoryCargoTests")).toBe(true);
  const scripts = execFileSync("git", ["ls-files", "-co", "--exclude-standard", "--", "📜️script.ts", "*/📜️script.ts"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).split("\n").filter((path) => path && !path.includes("/🎫️tickets/") && !path.includes("/node_modules/") && !path.includes("/dist/"));
  expect(scripts.length).toBeGreaterThan(100);
  const floating = scripts.flatMap((path) => {
    const source = readFileSync(join(root, path), "utf8");
    return [...runners].some((name) => source.includes(name)) ? floatingRunnerCalls(path, source, runners) : [];
  });
  expect(floating).toEqual([]);
}, 120_000);

/** 🚮️ Original source inventory proves the neutral decoder needs no specialization or fixture runtime input. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import * as ts from "typescript";
import contract from "./🧫️fixtures/🔣️.json";
const root = process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd();
const decoder = "🧰️framework/🔨️modules/🌱️value/🛬️decode";

test("the original neutral decoder closure excludes products, S, Hub and fixture runtime inputs", async () => {
  const { inspectRuntimeGraphV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🟦️.ts"));
  const entry = decoder + "/🟦️.ts";
  const owners = Object.entries(contract.packages), exported = (specifier: string): string | undefined => {
    const owner = owners.find(([name]) => specifier === name || specifier.startsWith(name + "/"));
    if (!owner) return undefined;
    const map = JSON.parse(readFileSync(resolve(root, owner[1], "package.json"), "utf8")).exports as Record<string, string>, subpath = "." + specifier.slice(owner[0].length), target = map[subpath];
    return target === undefined ? undefined : relative(root, resolve(root, owner[1], target));
  };
  const aliases = Object.fromEntries([...new Set(["@semio-tech/framework-async/continuation"])].map(specifier => [specifier, exported(specifier)!]));
  const graph = inspectRuntimeGraphV1([entry], { rootDirectory: root, aliases, read: (path: string) => {
    const absolute = resolve(root, path);
    return existsSync(absolute) ? readFileSync(absolute, "utf8") : undefined;
  } });
  expect(graph.findings).toEqual([]);
  expect(graph.nodes).toContain(entry);
  const independent: { from: string; to: string; kind: string }[] = [];
  for (const path of graph.nodes as string[]) {
    for (const candidate of [path, relative(root, realpathSync(resolve(root, path)))]) {
      expect(contract.absentRoots.some(owner => candidate === owner || candidate.startsWith(owner + "/")), candidate).toBe(false);
    }
    const source = readFileSync(resolve(root, path), "utf8");
    const parsed = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true), runtimeImports: { fileName: string }[] = [];
    const visit = (node: ts.Node): void => {
      if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) {
        const clause = ts.isImportDeclaration(node) ? node.importClause : undefined, bindings = clause?.namedBindings;
        const typeOnly = ts.isImportDeclaration(node) ? clause?.isTypeOnly === true || clause !== undefined && clause.name === undefined && bindings !== undefined && ts.isNamedImports(bindings) && bindings.elements.length > 0 && bindings.elements.every(element => element.isTypeOnly) : node.isTypeOnly;
        if (!typeOnly) runtimeImports.push({ fileName: node.moduleSpecifier.text });
      } else if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && ts.isStringLiteral(node.arguments[0] ?? node)) runtimeImports.push({ fileName: (node.arguments[0] as ts.StringLiteral).text });
      ts.forEachChild(node, visit);
    };
    visit(parsed);
    for (const imported of runtimeImports) {
      const declared = imported.fileName.startsWith(".") ? undefined : exported(imported.fileName);
      expect(imported.fileName.startsWith(".") || declared !== undefined, imported.fileName).toBe(true);
      const target = declared === undefined ? resolve(root, dirname(path), imported.fileName) : resolve(root, declared);
      expect(existsSync(target), target).toBe(true);
      independent.push({ from: path, to: relative(root, target), kind: "import" });
    }
  }
  expect((graph.edges as typeof independent).toSorted((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)))).toEqual(independent.toSorted((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b))));
  const fixture = decoder + "/🧪️tests/🧫️fixtures/🔣️.json";
  expect(inspectRuntimeGraphV1([fixture], { read: (path: string) => readFileSync(resolve(root, path), "utf8") }).findings.map((row: {code: string}) => row.code)).toEqual(["runtime-fixture-edge"]);
});

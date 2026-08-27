import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, posix, resolve } from "node:path";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";
import { registryCatalogInputPaths, registryStaticImports, scanRegistryCompilerImports, type RegistryCatalogInputView, type Taxonomy } from "../../🔍️discovery/🟦️component.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as {
  schemaVersion: number; selection: string; fallback: string;
  cases: { id: string; paths: string[]; language: "ts" | "tsx" | "js" | "jsx"; source: string; imports: string[] }[];
  invalid: { id: string; path: string; source: string }[]; liveRegression: string;
  execution: { target: string; command: string; launchName: string; launchCommand: string; launchGroup: string; launchOrder: number };
};

/** 🔬️ Uses TypeScript's independently parsed source tree to collect static module identities. */
function oracle(source: string, path: string): string[] {
  const kind = path.endsWith(".tsx") ? ts.ScriptKind.TSX : path.endsWith(".jsx") ? ts.ScriptKind.JSX : /\.(?:m|c)?js$/u.test(path) ? ts.ScriptKind.JS : ts.ScriptKind.TS;
  const parsed = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, kind);
  expect((parsed as ts.SourceFile & { parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics).toHaveLength(0);
  return [...new Set(parsed.statements.flatMap((node) => (ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) ? [node.moduleSpecifier.text] : []))].sort();
}

for (const row of vector.cases) test("registry imports select leaf grammar: " + row.id, () => {
  for (const path of row.paths.flatMap((path) => [path, path.replaceAll("/", "\\")])) {
    expect(oracle(row.source, path)).toEqual(row.imports);
    expect(registryStaticImports(row.source, path)).toEqual(row.imports);
  }
});

for (const row of vector.invalid) test("registry imports reject without grammar fallback: " + row.id, () => {
  let failure: unknown;
  try { registryStaticImports(row.source, row.path); } catch (error) { failure = error; }
  expect(failure).toBeInstanceOf(Error);
  expect((failure as Error).message).toContain(row.path);
});

test("the actual numeric index remains valid TypeScript without rewriting it", () => {
  const source = readFileSync(join(repoRoot, vector.liveRegression), "utf8");
  expect(registryStaticImports(source, vector.liveRegression)).toEqual(oracle(source, vector.liveRegression));
});

test("compiler capability validation constructs exactly the selected grammar", () => {
  for (const language of ["ts", "tsx", "js", "jsx"] as const) {
    const observed: unknown[] = [];
    const platform = { Transpiler: class {
      constructor(options: unknown) { observed.push(options); }
      scanImports(source: string) { expect(source).toBe("source"); return [{ path: "./value", kind: "import-statement", external: true }]; }
    } };
    expect(scanRegistryCompilerImports("source", language, platform)).toEqual([{ path: "./value", kind: "import-statement" }]);
    expect(observed).toEqual([{ loader: language }]);
  }
  expect(() => scanRegistryCompilerImports("", "ts", null)).toThrow("compiler runtime");
  expect(() => scanRegistryCompilerImports("", "ts", {})).toThrow("Bun.Transpiler");
  expect(() => scanRegistryCompilerImports("", "ts", { Transpiler: class {} })).toThrow("scanImports");
});

test("catalog closure propagates every physical leaf language", () => {
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, library, "🔣️taxonomy.json"), "utf8")) as Taxonomy;
  const authority = taxonomy.generatorContracts["plugin-registry"]!.inputDiscovery!;
  Object.assign(authority, { implementationEntryPaths: ["📜️script.ts"], workspaceImports: {} });
  const content = new Map([
    ["📜️script.ts", "import './🧩️module/🟦️.ts'; import './🧩️module/🟦️.tsx';"],
    ["🧩️module/🟦️.ts", "export const identity = <T>(value: T) => value;"],
    ["🧩️module/🟦️.tsx", "export const view = <section/>;"],
  ]);
  const directories = new Set(["", "🧩️module"]);
  const reads: string[] = [];
  const view: RegistryCatalogInputView = {
    kind: (path) => content.has(path) ? "file" : directories.has(path) ? "directory" : null,
    entries: (path) => [...directories, ...content.keys()].filter((child) => child !== path && posix.dirname(child).replace(/^\.$/u, "") === path).map((child) => ({ name: posix.basename(child), nodeKind: directories.has(child) ? "directory" as const : "file" as const })),
    readText: (path) => { reads.push(path); const source = content.get(path); if (source === undefined) throw new Error("Unexpected virtual read: " + path); return source; },
  };
  const expected = [...content.keys(), "🧩️module"].sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
  expect(registryCatalogInputPaths(repoRoot, taxonomy, view)).toEqual(expected);
  expect(reads.sort()).toEqual([...content.keys()].sort());
});

test("registers the language-neutral compiler gate through Nx and both launch catalogs", () => {
  expect(vector.schemaVersion).toBe(1);
  expect(vector.selection).toBe("physical-leaf-extension");
  expect(vector.fallback).toBe("none");
  const expected = vector.execution;
  const project = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(repoRoot, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});

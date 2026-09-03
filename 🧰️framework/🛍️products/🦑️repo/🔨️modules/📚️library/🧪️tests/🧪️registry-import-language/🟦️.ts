import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, posix, resolve } from "node:path";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";
import Ajv from "ajv";
import * as discovery from "../../🔍️discovery/🟦️.ts";
import { registryCatalogInputPaths, registryStaticImports, scanRegistryCompilerImports, type RegistryCatalogInputView, type Taxonomy } from "../../🔍️discovery/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧪️📇️registry-import-language/🔣️.json"), "utf8")) as {
  schemaVersion: number; selection: string; fallback: string;
  cases: { id: string; paths: string[]; language: "ts" | "tsx" | "js" | "jsx"; source: string; imports: string[] }[];
  invalid: { id: string; path: string; source: string }[]; liveRegression: string;
  execution: { target: string; command: string; launchName: string; launchCommand: string; launchGroup: string; launchOrder: number };
};

const dataRoot = join(import.meta.dir, "../🧪️📇️registry-import-language/🧪️imported-data");
const dataVector = JSON.parse(readFileSync(join(dataRoot, "../🧪️📇️registry-import-language/🔣️.json"), "utf8")) as {
  cases: { id: string; path: string; role: "implementation-entry" | "static-import"; source: string; expected?: { kind: "module" | "json-data"; imports: string[] }; error?: string }[];
  graph: { entries: string[]; dataPath: string; files: { path: string; content: string; mode: number }[] };
};

/** 🌳️ Supplies exact authored content and positive directory membership without filesystem access. */
function dataGraphView(content: ReadonlyMap<string, string>, reads: string[]): RegistryCatalogInputView {
  const directories = new Set([""]);
  for (const path of content.keys()) for (let parent = posix.dirname(path); parent !== "."; parent = posix.dirname(parent)) directories.add(parent);
  return {
    kind: (path) => content.has(path) ? "file" : directories.has(path) ? "directory" : null,
    entries: (path) => [...directories, ...content.keys()].filter((child) => child !== path && posix.dirname(child).replace(/^\.$/u, "") === path).map((child) => ({ name: posix.basename(child), nodeKind: directories.has(child) ? "directory" as const : "file" as const })),
    readText: (path) => { reads.push(path); const source = content.get(path); if (source === undefined) throw new Error("Missing authored compiler input: " + path); return source; },
  };
}

test("registry imported data follows the schema-first role and strict JSON grammar", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(dataRoot, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(dataVector), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...dataVector, fallback: "jsonc" })).toBe(false);
  const dependencies = Reflect.get(discovery, "registryCompilerInputDependencies");
  expect(typeof dependencies).toBe("function");
  for (const row of dataVector.cases) {
    for (const path of [row.path, row.path.replaceAll("/", "\\")]) {
      if (row.error) expect(() => dependencies(row.source, path, row.role), row.id).toThrow(row.error);
      else expect(dependencies(row.source, path, row.role), row.id).toEqual(row.expected);
    }
    if (row.path.endsWith(".json") && row.role === "static-import") {
      const errors: import("jsonc-parser").ParseError[] = [];
      const parsed = parseJsonc(row.source, errors, { disallowComments: true, allowTrailingComma: false });
      if (row.error) expect(errors.length, row.id).toBeGreaterThan(0);
      else { expect(errors, row.id).toEqual([]); expect(parsed, row.id).toEqual(JSON.parse(row.source)); }
    }
  }
  expect(() => dependencies("{}", "value.json", "unknown")).toThrow("role");
  expect(() => registryStaticImports("{}", "value.json")).toThrow("language is not supported");
});

test("registry imported data is retained in the production compiler closure with strict entry roles", () => {
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, library, "🔣️taxonomy.json"), "utf8")) as Taxonomy;
  const authority = taxonomy.generatorContracts["plugin-registry"]!.inputDiscovery!;
  Object.assign(authority, { implementationEntryPaths: dataVector.graph.entries, workspaceImports: {} });
  const content = new Map(dataVector.graph.files.map(({ path, content }) => [path, content])), reads: string[] = [];
  expect(registryCatalogInputPaths(repoRoot, taxonomy, dataGraphView(content, reads))).toEqual([...content.keys(), "🧩️module"].sort((a, b) => Buffer.from(a).compare(Buffer.from(b))));
  expect(reads.sort()).toEqual([...content.keys()].sort());
  for (const replacement of [undefined, "{", "{/* comment */}"]) {
    const changed = new Map(content);
    if (replacement === undefined) changed.delete(dataVector.graph.dataPath); else changed.set(dataVector.graph.dataPath, replacement);
    expect(() => registryCatalogInputPaths(repoRoot, taxonomy, dataGraphView(changed, []))).toThrow();
  }
  Object.assign(authority, { implementationEntryPaths: [...dataVector.graph.entries, dataVector.graph.dataPath] });
  expect(() => registryCatalogInputPaths(repoRoot, taxonomy, dataGraphView(content, []))).toThrow("language is not supported");
  Object.assign(authority, { implementationEntryPaths: dataVector.graph.entries, workspaceImports: { "@fixture/data": { manifestPath: "package.json", entryPath: dataVector.graph.dataPath } } });
  const workspace = new Map(content);
  workspace.set("package.json", JSON.stringify({ name: "@fixture/data", exports: { ".": "./" + dataVector.graph.dataPath } }));
  workspace.set("📜️script.ts", content.get("📜️script.ts") + "\nimport named from '@fixture/data'; export { named };");
  expect(() => registryCatalogInputPaths(repoRoot, taxonomy, dataGraphView(workspace, []))).toThrow("language is not supported");
});

test("registry imported data closure matches Bun's independent in-memory compiler inputs", async () => {
  const files = new Map(dataVector.graph.files.map(({ path, content }) => ["/" + path, content])), loaded: string[] = [];
  const result = await Bun.build({
    entrypoints: dataVector.graph.entries.map((path) => "/" + path), target: "bun", write: false,
    plugins: [{ name: "authored-registry-data-oracle", setup(build) {
      build.onResolve({ filter: /.*/u }, (args) => ({ path: args.kind === "entry-point" ? args.path : posix.resolve(posix.dirname(args.importer), args.path), namespace: "authored-registry-data" }));
      build.onLoad({ filter: /.*/u, namespace: "authored-registry-data" }, (args) => {
        loaded.push(args.path);
        const contents = files.get(args.path);
        if (contents === undefined) throw new Error("Compiler requested undeclared authored input: " + args.path);
        return { contents, loader: args.path.endsWith(".json") ? "json" : "ts" };
      });
    } }],
  });
  expect(result.success, JSON.stringify(result.logs)).toBe(true);
  expect(loaded.sort()).toEqual([...files.keys()].sort());
  expect(new Set(loaded).size).toBe(loaded.length);
  expect(result.outputs.length).toBeGreaterThan(0);
  const data = dataVector.graph.files.find(({ path }) => path === dataVector.graph.dataPath)!;
  expect(parseJsonc(data.content)).toEqual(JSON.parse(data.content));
});

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
    ["📜️script.ts", "import './🟦️'; import './🟦️';"],
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

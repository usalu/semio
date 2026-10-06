/** 📏️ Checks schema-owned public exports through independent compiler and runtime clients. */
import { expect, test } from "bun:test";

import ts from "typescript";
import { existsSync, readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import config from "../../🧪️tests/🎚️config/🟦️.ts";

const root = resolve(import.meta.dir, "../..");
const entries = [join(root, "🟦️.ts"), join(root, "📦️packages/🟦️typescript/🟦️.ts")];
const options: ts.CompilerOptions = { noEmit: true, strict: true, noUncheckedIndexedAccess: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, skipLibCheck: true, types: ["bun", "node"] };

test("the language-neutral export roster is closed and unambiguous", () => {
  expect(new Set(fixture.exports.map((row) => row.name)).size).toBe(fixture.exports.length);
  expect(fixture.exports.map((row) => row.name)).toEqual(fixture.exports.map((row) => row.name).sort());
  expect(fixture.retired.some((name) => fixture.exports.some((row) => row.name === name))).toBe(false);
});

test("the registered suite discovers every language-neutral test owner", () => {
  const actual = config.test.include.map((path) => resolve(config.test.root, path)).sort();
  expect(actual).toEqual(fixture.testOwners.map((path) => join(root, path)).sort());
  expect(actual.every((path) => existsSync(path))).toBe(true);
});

test("both actual entries expose exactly the named owned types and values", () => {
  const program = ts.createProgram([...entries, join(import.meta.dir, "🟦️.ts")], options);
  expect(ts.getPreEmitDiagnostics(program).map((row) => ts.flattenDiagnosticMessageText(row.messageText, "\n"))).toEqual([]);
  const checker = program.getTypeChecker();
  for (const entry of entries) {
    const source = program.getSourceFile(entry)!;
    const symbols = checker.getExportsOfModule(checker.getSymbolAtLocation(source)!);
    expect(symbols.map((symbol) => symbol.name).sort()).toEqual(fixture.exports.map((row) => row.name));
    for (const row of fixture.exports) {
      const symbol = symbols.find((candidate) => candidate.name === row.name)!;
      const target = symbol.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(symbol) : symbol;
      expect(Boolean(target.flags & ts.SymbolFlags.Value)).toBe(row.kind === "value");
      const owner = row.owner === "wire" ? "🤖️generated/⏳️async/🟦️.ts" : "🪃️continuation/🟦️.ts";
      expect(target.declarations?.every((declaration) => declaration.getSourceFile().fileName.replaceAll("\\", "/").endsWith(owner))).toBe(true);
    }
  }
}, 30_000);

test("public entries require explicit export declarations", () => {
  for (const entry of entries) {
    const source = ts.createSourceFile(entry, readFileSync(entry, "utf8"), ts.ScriptTarget.Latest, true);
    const declarations = source.statements.filter(ts.isExportDeclaration);
    expect(declarations.length).toBeGreaterThan(0);
    for (const declaration of declarations) {
      expect(declaration.exportClause !== undefined && ts.isNamedExports(declaration.exportClause)).toBe(true);
    }
  }
});

test("an external compiler client cannot import any retired name", () => {
  const virtualPath = join(root, "📏️api", "external-client.ts");
  const source = "import { " + fixture.retired.join(", ") + " } from " + JSON.stringify(entries[1]) + ";";
  const host = ts.createCompilerHost(options);
  const getSourceFile = host.getSourceFile.bind(host);
  host.getSourceFile = (path, languageVersion, onError, createNew) => path === virtualPath ? ts.createSourceFile(path, source, languageVersion, true) : getSourceFile(path, languageVersion, onError, createNew);
  const program = ts.createProgram([virtualPath], options, host);
  const diagnostics = ts.getPreEmitDiagnostics(program).filter((row) => row.file?.fileName === virtualPath);
  expect(diagnostics.every((row) => row.code === 2305 || row.code === 2724)).toBe(true);
  expect(diagnostics.map((row) => source.slice(row.start!, row.start! + row.length!)).sort()).toEqual(fixture.retired);
});

test("the actual runtime package works with all specific and test owners unavailable", async () => {
  const { build } = await import("esbuild");
  const workspace = resolve(root, "../../..");
  const refused = ["🧰️framework/🛍️products", "✏️s", "🌎️hub"].map((area) => resolve(workspace, area).replaceAll("\\", "/") + "/");
  const source = "import * as api from " + JSON.stringify(entries[1]) + ";const scheduler=api.createContinuationScheduler();let calls=0;const cancel=scheduler.schedule(()=>calls++,0);cancel();await scheduler.yieldContinuation();const pending=scheduler.pending();scheduler.dispose();api.hostContinuations.dispose();console.log(JSON.stringify({exports:Object.keys(api).sort(),calls,pending}));";
  const bundle = await build({ stdin: { contents: source, resolveDir: root }, bundle: true, platform: "node", format: "esm", write: false, plugins: [{ name: "removed-async-owner", setup(builder) { builder.onLoad({ filter: /.*/ }, (input) => { const path = input.path.replaceAll("\\", "/"); return path.includes("/🧪️tests/") || refused.some((prefix) => path.startsWith(prefix)) ? { errors: [{ text: "Public async API loads an unavailable owner: " + path }] } : undefined; }); } }] });
  const actual = JSON.parse(execFileSync("node", ["--input-type=module"], { input: bundle.outputFiles![0]!.text, encoding: "utf8", timeout: 5_000 }));
  expect(actual).toEqual({ exports: fixture.exports.filter((row) => row.kind === "value").map((row) => row.name), calls: 0, pending: 0 });
  console.log("[DEBUG] async-public-client " + JSON.stringify(actual));
}, 15_000);

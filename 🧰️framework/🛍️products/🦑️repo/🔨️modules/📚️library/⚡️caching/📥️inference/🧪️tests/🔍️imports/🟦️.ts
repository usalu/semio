import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { join, resolve } from "node:path";
import { build } from "esbuild";
import Ajv from "ajv";

const owner = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8")) as { imports: { id: string; path: string; source: string; imports: string[]; verbatimModuleSyntax?: boolean }[]; references: { id: string; path: string; source: string; imports: string[]; requires: string[]; union: string[]; compilerRequires?: string[] }[] };
type Provider = { commandSourceImports(path: string, source: string): readonly string[]; commandSourceParseStats(): { hits: number; misses: number; bytes: number; limit: number } };

test("closed import facts retain unique language-neutral vectors", () => {
  expect(new Set(fixture.imports.map(row => row.id)).size).toBe(fixture.imports.length);
});

test("command import facts equal independent esbuild while cache never grants stale source", async () => {
  const oracles: string[][] = [];
  for (const row of fixture.imports) {
    const paths = new Set<string>();
    await build({ stdin: { contents: row.source, sourcefile: row.path, loader: row.path.endsWith(".d.ts") ? "empty" : "ts" }, bundle: true, write: false, format: "esm", define: { "import.meta.vitest": "false" }, logLevel: "silent", tsconfigRaw: { compilerOptions: { verbatimModuleSyntax: row.verbatimModuleSyntax ?? true } }, plugins: [{ name: "owned-import-oracle", setup(builder) { builder.onResolve({ filter: /^\./ }, args => { paths.add(args.path); return { path: args.path, external: true }; }); } }] });
    oracles.push([...paths].sort());
    expect(oracles.at(-1), row.id).toEqual(row.imports);
  }
  const provider: Provider = await import(pathToFileURL(join(owner, "🟨️.mjs")).href);
  for (const [index, row] of fixture.imports.entries()) {
    const facts = provider.commandSourceImports(row.path, row.source);
    expect([...facts].sort(), row.id).toEqual(oracles[index]!);
    const before = provider.commandSourceParseStats();
    expect(provider.commandSourceImports(row.path, row.source)).toEqual(facts);
    expect(provider.commandSourceParseStats().hits).toBe(before.hits + 1);
  }
  expect(provider.commandSourceImports("fresh.ts", "export {value} from './first.ts';")).toEqual(["./first.ts"]);
  expect(provider.commandSourceImports("fresh.ts", "export {value} from './second.ts';")).toEqual(["./second.ts"]);
  const facts = provider.commandSourceImports("mutation.ts", "export {value} from './owned.ts';") as string[];
  facts.push("./unowned.ts");
  expect(provider.commandSourceImports("mutation.ts", "export {value} from './owned.ts';")).toEqual(["./owned.ts"]);
  const stats = provider.commandSourceParseStats();
  expect(stats.bytes).toBeLessThanOrEqual(stats.limit);
});


test("runtime module references retain import and require resolution modes", async () => {
  const provider = await import(pathToFileURL(join(owner, "🟨️.mjs")).href) as {
    runtimeModuleSourceReferences(path: string, source: string): Readonly<{ version: 1; imports: readonly string[]; requires: readonly string[] }>;
    runtimeModuleSourceImports(path: string, source: string): string[];
    commandSourceParseStats(): { hits: number; bytes: number; limit: number };
  };
  const schema = JSON.parse(readFileSync(join(owner, "🔗️references/🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(new Set(fixture.references.map(row => row.id)).size).toBe(fixture.references.length);
  for (const row of fixture.references) {
    const observed = { imports: new Set<string>(), requires: new Set<string>() };
    await build({ stdin: { contents: row.source, sourcefile: row.path, loader: "ts" }, bundle: true, write: false, format: "esm", platform: "node", define: { "import.meta.vitest": "false" }, logLevel: "silent", tsconfigRaw: { compilerOptions: { verbatimModuleSyntax: false } }, plugins: [{ name: "owned-reference-mode-oracle", setup(builder) { builder.onResolve({ filter: /.*/ }, args => { (args.kind === "require-call" ? observed.requires : observed.imports).add(args.path); return { path: args.path, external: true }; }); } }] });
    expect([...observed.imports].sort(), row.id).toEqual([...row.imports].sort());
    expect([...observed.requires].sort(), row.id).toEqual([...(row.compilerRequires ?? row.requires)].sort());
    const value = provider.runtimeModuleSourceReferences(row.path, row.source);
    expect(validate(value), JSON.stringify(validate.errors)).toBe(true);
    expect(value, row.id).toEqual({ version: 1, imports: row.imports, requires: row.requires });
    expect(provider.runtimeModuleSourceImports(row.path, row.source), row.id).toEqual(row.union);
    const hits = provider.commandSourceParseStats().hits;
    expect(provider.runtimeModuleSourceReferences(row.path, row.source)).toEqual(value);
    expect(provider.commandSourceParseStats().hits).toBe(hits + 1);
    expect(Object.isFrozen(value)).toBe(true);
    expect(Object.isFrozen(value.imports)).toBe(true);
    expect(Object.isFrozen(value.requires)).toBe(true);
  }
  expect(validate({ version: 1, imports: ["same", "same"], requires: [] })).toBe(false);
  expect(validate({ version: 1, imports: [], requires: [], union: [] })).toBe(false);
  expect(validate({ version: 1, imports: [], requires: [1] })).toBe(false);
  const stats = provider.commandSourceParseStats();
  expect(stats.bytes).toBeLessThanOrEqual(stats.limit);
});

import { testImportEdgeEquality } from "../../../🧪️tests/🔗️import-edges/🟦️.ts";

test("full registered repository import dependency equality remains intact", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  await testImportEdgeEquality(process.cwd(), output);
}, 120000);



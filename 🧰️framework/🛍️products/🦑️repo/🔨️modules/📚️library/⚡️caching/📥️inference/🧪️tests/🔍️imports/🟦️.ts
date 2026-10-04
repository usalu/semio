import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { build } from "esbuild";

const owner = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")) as { imports: { id: string; path: string; source: string; imports: string[] }[] };
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
type Provider = { commandSourceImports(path: string, source: string): readonly string[]; commandSourceParseStats(): { hits: number; misses: number; bytes: number; limit: number } };

test("closed import facts retain unique language-neutral vectors", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  expect(new Set(fixture.imports.map(row => row.id)).size).toBe(fixture.imports.length);
});

test("command import facts equal independent esbuild while cache never grants stale source", async () => {
  const oracles: string[][] = [];
  for (const row of fixture.imports) {
    const paths = new Set<string>();
    await build({ stdin: { contents: row.source, sourcefile: row.path, loader: row.path.endsWith(".d.ts") ? "empty" : "ts" }, bundle: true, write: false, format: "esm", logLevel: "silent", tsconfigRaw: { compilerOptions: { verbatimModuleSyntax: true } }, plugins: [{ name: "owned-import-oracle", setup(builder) { builder.onResolve({ filter: /^\./ }, args => { paths.add(args.path); return { path: args.path, external: true }; }); } }] });
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

import "../../../🧪️tests/🔗️import-edges/🔁️context/🟦️.ts";
import { testImportEdgeEquality } from "../../../🧪️tests/🔗️import-edges/🟦️.ts";

test("full registered repository import dependency equality remains intact", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  await testImportEdgeEquality(process.cwd(), output);
}, 120000);

import "../../../🧪️tests/🔗️import-edges/🏛️graph/🟦️.ts";

import "../../../🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts";

import { expect, test } from "bun:test";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { join, relative, resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import TOML from "@iarna/toml";
import graphlib from "graphlib";
import { readRustSourceClosureV1 } from "../../../🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

type Row = { package: string; manifest: string; entry: string; level: number; dependencies: string[]; devDependencies: string[]; peerDependencies: string[] };
type Corpus = { version: number; facade: { package: string; manifest: string; entry: string }; peers: string[]; budget: { maximumActiveLines: number; maximumFiles: number; maximumBytes: number }; excluded: string[]; unmounted: string[]; crates: Row[] };
type Manifest = { package: { name: string; workspace: string }; lib: { path: string }; dependencies?: Record<string, { package?: string }>; ["dev-dependencies"]?: Record<string, { package?: string }> };

const owner = resolve(import.meta.dir, "../.."), workspace = resolve(owner, "../../..");
const read = (path: string) => readFileSync(resolve(owner, path), "utf8");
const corpus = JSON.parse(read("🧪️tests/🧫️fixtures/🏛️crate-partition/🔣️.json")) as Corpus;
const names = new Set(corpus.crates.map(row => row.package));
const hostCfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
const configurations: [string, string[]][] = [
  ["native", [...hostCfg, "debug_assertions"]], ["native deflate", [...hostCfg, "debug_assertions", "feature=deflate"]],
  ["wasm32-unknown-unknown", ["target_arch=wasm32", "target_os=unknown", "target_env=", "target_pointer_width=32", "debug_assertions"]],
  ["wasm32-wasip2", ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "debug_assertions"]],
];
const edges = (manifest: Manifest, table: "dependencies" | "dev-dependencies") => Object.entries(manifest[table] ?? {}).map(([alias, edge]) => edge.package ?? alias);
const parse = (row: { manifest: string }) => { const source = read(row.manifest); return { source, manifest: Bun.TOML.parse(source) as unknown as Manifest, oracle: TOML.parse(source) as unknown as Manifest }; };

test("the partition corpus validates against its closed schema with the independent Ajv oracle", () => {
  const validate = new Ajv2020({ strict: true }).compile(JSON.parse(read("🧬️schema/🏛️crate-partition/🔣️.json")));
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...corpus, hidden: true })).toBe(false);
  expect(corpus.crates.length).toBe(16);
});

for (const row of corpus.crates) test(`${row.package} parses identically, lists exactly its table and keeps the exact source boundary`, () => {
  const { manifest, oracle } = parse(row);
  expect(manifest).toEqual(oracle);
  expect(manifest.package.name).toBe(row.package);
  expect(resolve(owner, row.manifest, "..", manifest.package.workspace)).toBe(resolve(workspace, "🧰️framework"));
  expect(resolve(owner, row.manifest, "..", manifest.lib.path)).toBe(resolve(owner, row.entry));
  const normal = edges(manifest, "dependencies"), dev = edges(manifest, "dev-dependencies");
  expect(normal.filter(name => names.has(name)).sort()).toEqual([...row.dependencies].sort());
  expect(dev.filter(name => names.has(name)).sort()).toEqual([...row.devDependencies].sort());
  expect([...normal, ...dev].filter(name => corpus.peers.includes(name)).sort()).toEqual([...row.peerDependencies].sort());
  expect([...normal, ...dev]).not.toContain(corpus.facade.package);
  const boundary = (manifest as unknown as { package: { metadata: { semio: { ["rust-source-boundary"]: Record<string, number> } } } }).package.metadata.semio["rust-source-boundary"];
  expect(boundary).toEqual({ maxSourceBytes: 262144, maxSourceFiles: 32, maxActiveLines: 4000 });
});

test("the facade links every carved crate and no carved crate links the facade", () => {
  const { manifest } = parse(corpus.facade);
  const linked = edges(manifest, "dependencies");
  for (const name of names) expect(linked).toContain(name);
});

test("dependencies point strictly downward and the graph is acyclic under two independent oracles", () => {
  const level = new Map(corpus.crates.map(row => [row.package, row.level]));
  const graph = new graphlib.Graph({ directed: true });
  const adjacency = new Map<string, string[]>();
  for (const row of corpus.crates) {
    graph.setNode(row.package);
    const to = [...row.dependencies, ...row.devDependencies];
    adjacency.set(row.package, to);
    for (const dependency of row.dependencies) expect(level.get(dependency)!).toBeLessThan(row.level);
    for (const dependency of to) graph.setEdge(row.package, dependency);
  }
  expect(graphlib.alg.isAcyclic(graph)).toBe(true);
  expect(graphlib.alg.findCycles(graph)).toEqual([]);
  const state = new Map<string, number>();
  const visit = (name: string) => { expect(state.get(name) ?? 0).not.toBe(1); if (state.get(name) === 2) return; state.set(name, 1); for (const next of adjacency.get(name) ?? []) visit(next); state.set(name, 2); };
  for (const name of adjacency.keys()) visit(name);
});

const rustFiles = (directory: string): string[] => readdirSync(directory).flatMap(entry => {
  if (entry === "node_modules" || entry === "dist") return [];
  const path = join(directory, entry);
  return statSync(path).isDirectory() ? rustFiles(path) : path.endsWith(".rs") ? [path] : [];
});

const moduleFiles = (entry: string): string[] => {
  const seen = new Set<string>(), stack = [entry];
  while (stack.length) {
    const file = stack.pop()!;
    if (seen.has(file)) continue;
    seen.add(file);
    const text = readFileSync(file, "utf8");
    for (const match of text.matchAll(/#\[\s*path\s*=\s*"([^"]*)"\s*\]|\binclude!\(\s*"([^"]*\.rs)"\s*\)/g)) {
      const raw = match[1] ?? match[2];
      if (raw) stack.push(resolve(file, "..", raw));
    }
  }
  return [...seen];
};

test("every Rust file below the replication module has exactly one owning crate or is declared", () => {
  const owners = new Map<string, string[]>();
  for (const row of [...corpus.crates, { ...corpus.facade, level: 7, dependencies: [], devDependencies: [], peerDependencies: [] }]) {
    for (const file of moduleFiles(resolve(owner, row.entry))) owners.set(file, [...(owners.get(file) ?? []), row.package]);
  }
  for (const [file, crates] of owners) expect(crates, relative(owner, file)).toHaveLength(1);
  const skipped = (file: string) => corpus.excluded.some(prefix => relative(owner, file).startsWith(prefix)) || corpus.unmounted.includes(relative(owner, file));
  for (const file of rustFiles(owner)) if (!skipped(file)) expect(owners.has(file), relative(owner, file)).toBe(true);
});

test("every crate stays inside the source budget for each target and the deflate feature", async () => {
  for (const row of corpus.crates) for (const [label, cfg] of configurations) for (const tests of [false, true]) {
    const entry = resolve(owner, row.entry), manifestDirectory = resolve(owner, row.manifest, "..");
    const rows = await readRustSourceClosureV1([entry], workspace, tests ? [...cfg, "test"] : cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)), { manifestDirectory, generated: () => undefined });
    const detail = `${row.package} ${label}${tests ? " tests" : ""}`;
    expect(rows.reduce((sum, input) => sum + input.activeLines, 0), detail).toBeLessThanOrEqual(corpus.budget.maximumActiveLines);
    expect(rows.reduce((sum, input) => sum + input.bytes, 0), detail).toBeLessThanOrEqual(corpus.budget.maximumBytes);
    expect(rows.length, detail).toBeLessThanOrEqual(corpus.budget.maximumFiles);
  }
}, 600000);

test("no foreign-trait impl targets a type owned by another replication crate", () => {
  const owned = new Map<string, Set<string>>();
  const bodies = new Map<string, string[]>();
  for (const row of corpus.crates) {
    const sources = moduleFiles(resolve(owner, row.entry)).filter(file => !file.includes("🧪️tests")).map(file => readFileSync(file, "utf8").replace(/\/\/[^\n]*/g, ""));
    bodies.set(row.package, sources);
    for (const text of sources) for (const match of text.matchAll(/\b(?:struct|enum|union|type)\s+([A-Z][A-Za-z0-9_]*)/g)) owned.set(match[1]!, (owned.get(match[1]!) ?? new Set<string>()).add(row.package));
  }
  for (const [crate, sources] of bodies) for (const text of sources) for (const match of text.matchAll(/\bimpl(?:<[^{>]*(?:<[^>]*>[^{>]*)*>)?\s+(?:[A-Za-z0-9_:<>, &']+\s+for\s+)?([A-Z][A-Za-z0-9_]*)/g)) {
    const target = match[1]!;
    const homes = owned.get(target);
    if (homes !== undefined && !homes.has(crate)) expect(`${crate} impls for ${target} owned by ${[...homes].join(",")}`).toBe("");
  }
});

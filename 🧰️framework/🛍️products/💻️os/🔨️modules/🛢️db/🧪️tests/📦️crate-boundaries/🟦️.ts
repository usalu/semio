import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { resolve, dirname } from "node:path";
import TOML from "@iarna/toml";
import Ajv from "ajv";
import { readRustSourceClosureV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const db = resolve(import.meta.dir, "../.."), workspace = resolve(db, "../../../../.."), read = (path: string) => readFileSync(resolve(db, path), "utf8");
type Edge = { package?: string; path?: string };
type Manifest = { package: { name: string }; lib: { path: string }; dependencies?: Record<string, Edge>; target?: Record<string, { dependencies?: Record<string, Edge> }> };
const corpus = JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json")) as { crates: { package: string; manifest: string; entry: string; dependencies: string[]; forbiddenDependencies: string[] }[] };
const cfgs = {
  lib: ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions"],
  test: ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "test", "feature=testkit", "feature=claimed-backend-laws"],
};
const closure = (entry: string, cfg: string[]) => readRustSourceClosureV1([resolve(db, entry)], workspace, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
const owners = (JSON.parse(readFileSync(resolve(workspace, "📋️build-owners.json"), "utf8")) as { owners: { name: string; root: string; manifest: string }[] }).owners;
const members = (Bun.TOML.parse(readFileSync(resolve(workspace, "Cargo.toml"), "utf8")) as unknown as { workspace: { members: string[] } }).workspace.members;
const dbRel = db.slice(workspace.length + 1).replaceAll("\\", "/"), facade = "semio-framework-os-kernel-db";

test("language-neutral ownership corpus validates with the independent schema oracle", () => {
  const validate = new Ajv().compile(JSON.parse(read("🧬️schema/📦️crate-boundaries/🔣️.json")));
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...corpus, hidden: true })).toBe(false);
});

for (const row of corpus.crates) test(row.package + " owns an acyclic, registered and gate-sized contract", async () => {
  const source = read(row.manifest), own = Bun.TOML.parse(source) as unknown as Manifest, oracle = TOML.parse(source) as unknown as Manifest;
  expect(own).toEqual(oracle);
  expect(own.package.name).toBe(row.package);
  expect(resolve(db, row.manifest, "..", own.lib.path)).toBe(resolve(db, row.entry));
  const declared = Object.assign({}, own.dependencies, ...Object.values(own.target ?? {}).map(target => target.dependencies ?? {})) as Record<string, Edge>;
  const dependencies = Object.entries(declared).map(([alias, edge]) => edge.package ?? alias).sort();
  expect(dependencies).toEqual([...row.dependencies].sort());
  for (const forbidden of row.forbiddenDependencies) expect(dependencies).not.toContain(forbidden);
  const pkgDir = dirname(row.manifest);
  if (row.package === facade) {
    expect(owners.some(owner => owner.name === facade && owner.root === dbRel + "/" + pkgDir)).toBe(true);
  } else {
    const project = JSON.parse(read(pkgDir + "/📋️project.json")) as { name: string };
    expect(owners.some(owner => owner.name === project.name && owner.root === dbRel + "/" + pkgDir)).toBe(true);
  }
  expect(members).toContain(dbRel + "/" + pkgDir);
  for (const [label, cfg] of Object.entries(cfgs)) {
    const inputs = await closure(row.entry, cfg);
    expect(inputs.reduce((sum, input) => sum + input.activeLines, 0), label).toBeLessThanOrEqual(4000);
    expect(inputs.reduce((sum, input) => sum + input.bytes, 0), label).toBeLessThanOrEqual(262144);
    expect(inputs.length, label).toBeLessThanOrEqual(32);
  }
});

test("every production source file is owned by exactly one crate", async () => {
  const owner = new Map<string, string>();
  for (const row of corpus.crates) for (const input of await closure(row.entry, cfgs.lib)) {
    if (input.kind !== "source") continue;
    expect(owner.get(input.path) ?? row.package, input.path).toBe(row.package);
    owner.set(input.path, row.package);
  }
});

test("the carved db family is acyclic with an independent SCC oracle", () => {
  const independent = require("graphlib") as { Graph: new () => { setNode(name: string): void; setEdge(from: string, to: string): void }; alg: { findCycles(graph: unknown): string[][] } };
  const graph = new independent.Graph();
  const names = new Set(corpus.crates.map(row => row.package));
  for (const row of corpus.crates) graph.setNode(row.package);
  for (const row of corpus.crates) for (const dependency of row.dependencies) if (names.has(dependency)) graph.setEdge(row.package, dependency);
  expect(independent.alg.findCycles(graph)).toEqual([]);
});

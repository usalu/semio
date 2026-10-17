import { expect, test } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { resolve, dirname } from "node:path";
import { Database } from "bun:sqlite";
import TOML from "@iarna/toml";
import Ajv from "ajv";
import { readRustSourceClosureV1 } from "@semio-tech/process/artifacts/native-build/source";
import { rustTokens } from "@semio-tech/compiler-syntax-rust-ts";

const ui = resolve(import.meta.dir, "../.."), read = (path: string) => readFileSync(resolve(ui, path), "utf8");
type Manifest = { package: { name: string }; lib: { path: string }; dependencies: Record<string, { package?: string; path?: string }>; target?: Record<string, { dependencies?: Record<string, { package?: string; path?: string }> }> };
const corpus = JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json")) as { crates: { package: string; manifest: string; entry: string; dependencies: string[]; forbiddenDependencies: string[] }[] };
test("language-neutral ownership corpus validates with the independent schema oracle", () => {
  const validate = new Ajv().compile(JSON.parse(read("🧬️schema/📦️crate-boundaries/🔣️.json")));
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...corpus, hidden: true })).toBe(false);
});
for (const row of corpus.crates) test(row.package + " owns an acyclic production contract", async () => {
  const source = read(row.manifest), own = Bun.TOML.parse(source) as unknown as Manifest, oracle = TOML.parse(source) as unknown as Manifest;
  expect(own).toEqual(oracle);
  expect(own.package.name).toBe(row.package);
  const project = JSON.parse(read(row.manifest.replace("Cargo.toml", "📋️project.json"))) as { name: string };
  const launches = JSON.parse(readFileSync(resolve(ui, "../../..", ".vscode/launch.json"), "utf8")) as { configurations: { command?: string }[] };
  for (const target of ["build", "check", "test"]) for (const mode of ["prototype", "release"])
    expect(launches.configurations.some(launch => launch.command === `bun repo run ${project.name}:${target} --configuration=${mode} --parallel=1 -- --lib --offline`)).toBe(true);
  expect(resolve(ui, row.manifest, "..", own.lib.path)).toBe(resolve(ui, row.entry));
  const declared = Object.assign({}, own.dependencies, ...Object.values(own.target ?? {}).map(target => target.dependencies ?? {})) as Manifest["dependencies"];
  const dependencies = Object.entries(declared).map(([alias, edge]) => edge.package ?? alias).sort();
  expect(dependencies).toEqual([...row.dependencies].sort());
  for (const forbidden of row.forbiddenDependencies) expect(dependencies).not.toContain(forbidden);
  const cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
  const workspace = resolve(ui, "../../..");
  const closure = await readRustSourceClosureV1([resolve(ui, row.entry)], workspace, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
  expect(closure.reduce((sum, input) => sum + input.activeLines, 0)).toBeLessThanOrEqual(4000);
  expect(closure.reduce((sum, input) => sum + input.bytes, 0)).toBeLessThanOrEqual(262144);
  expect(closure.length).toBeLessThanOrEqual(32);
  expect(closure.some(input => input.path.endsWith(".svg"))).toBe(false);
  for (const input of closure.filter(input => input.path.endsWith(".rs"))) {
    const tokens = rustTokens(await readFile(resolve(workspace, input.path), "utf8"));
    for (const forbidden of ["semio_framework_ui", "semio_framework_os_kernel", "semio_framework_plugin", "wgpu"]) expect(tokens.some(token => token.text === forbidden), input.path).toBe(false);
  }
  if (row.package === "semio-framework-ui-text") {
    const consumer = Bun.TOML.parse(read("📦️packages/🦀️rust/Cargo.toml")) as unknown as Manifest;
    const edge = consumer.dependencies[row.package];
    if (!edge?.path) throw Error("Missing authored text producer dependency");
    expect(resolve(ui, "📦️packages/🦀️rust", edge.path, "Cargo.toml")).toBe(resolve(ui, row.manifest));
    expect(read("🎯️targets/🧊️wgpu/🦀️.rs")).toContain("pub use semio_framework_ui_text as text;");
    expect(read("🎯️targets/🧊️wgpu/🦀️.rs")).not.toContain('"📝️text/🦀️.rs"');
  }
  expect(read("🎯️targets/🧊️wgpu/🦀️.rs")).toContain("pub use semio_framework_ui_declaration as component;");
  expect(read("🎯️targets/🧊️wgpu/🦀️.rs")).not.toContain('"🧩️component/🦀️.rs"');
});

for (const row of corpus.crates.filter(row => ["semio-framework-ui-draw-contract", "semio-framework-ui-prepared-staging", "semio-framework-ui-prepared", "semio-framework-ui-surface-identity"].includes(row.package))) test(row.package + " owns production and native test inputs across target cfgs", async () => {
  const workspace = resolve(ui, "../../..");
  for (const cfg of [
    ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"],
    ["target_arch=x86_64", "target_os=linux", "target_env=gnu", "target_pointer_width=64", "unix"],
    ["target_arch=x86_64", "target_os=windows", "target_env=msvc", "target_pointer_width=64", "windows"],
    ["target_arch=wasm32", "target_os=unknown", "target_env=", "target_pointer_width=32"],
    ["target_arch=wasm32", "target_os=unknown", "target_env=", "target_pointer_width=32", "feature=browser-presenter"],
    ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32"],
    ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "test", "feature=testkit"],
  ]) {
    const closure = await readRustSourceClosureV1([resolve(ui, row.entry)], workspace, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
    expect(closure.reduce((sum, input) => sum + input.activeLines, 0), cfg.join(",")).toBeLessThanOrEqual(4000);
    expect(closure.reduce((sum, input) => sum + input.bytes, 0), cfg.join(",")).toBeLessThanOrEqual(262144);
    expect(closure.length).toBeLessThanOrEqual(32);
    for (const input of closure.filter(input => input.path.endsWith(".rs"))) {
      const tokens = rustTokens(await readFile(resolve(workspace, input.path), "utf8"));
      for (const forbidden of ["semio_framework_ui", "semio_framework_os_kernel", "semio_framework_plugin", "wgpu"]) expect(tokens.some(token => token.text === forbidden), input.path).toBe(false);
    }
  }
});

test("original surface identity neutral vectors agree with the independent SQLite oracle", () => {
  const oracle = new Database(":memory:");
  try {
    const equals = oracle.query<{ equal: number }, [string, string, string, string]>("SELECT (?1 = ?3 AND ?2 = ?4) AS equal");
    for (const row of read("🪟️surface/🪪️identity/🧪️tests/🧫️fixtures/📐️identity.csv").trim().split("\n").slice(1)) {
      const [slot, generation, otherSlot, otherGeneration, expected] = row.split(",") as [string, string, string, string, string];
      expect(Boolean(equals.get(slot, generation, otherSlot, otherGeneration)?.equal)).toBe(expected === "true");
    }
  } finally { oracle.close(); }
});

test("every authored firstparty producer closure is acyclic with an independent SCC oracle", () => {
  type Edge = { path?: string; workspace?: boolean };
  type Cargo = { package?: { name: string; workspace?: string }; dependencies?: Record<string, Edge>; target?: Record<string, { dependencies?: Record<string, Edge> }>; workspace?: { dependencies?: Record<string, Edge> } };
  type Graph = { setNode(name: string): void; setEdge(from: string, to: string): void };
  const independent = require("graphlib") as { Graph: new () => Graph; alg: { findCycles(graph: Graph): string[][] } };
  const graph = new independent.Graph(), owners = new Map<string, string>(), edges = new Map<string, string[]>();
  const visit = (path: string): string => {
    const text = readFileSync(path, "utf8"), manifest = Bun.TOML.parse(text) as unknown as Cargo;
    expect(manifest).toEqual(TOML.parse(text));
    if (!manifest.package) throw Error("Firstparty owner manifest has no package: " + path);
    const name = manifest.package.name, previous = owners.get(name);
    if (previous) { expect(previous).toBe(path); return name; }
    owners.set(name, path); graph.setNode(name); edges.set(name, []);
    const authored = { ...manifest.dependencies, ...Object.assign({}, ...Object.values(manifest.target ?? {}).map(target => target.dependencies ?? {})) };
    for (const [alias, original] of Object.entries(authored) as [string, Edge][]) {
      let edge = original, base = dirname(path);
      if (edge.workspace) {
        let workspace = manifest.package.workspace ? resolve(base, manifest.package.workspace) : base;
        while (true) {
          const candidate = resolve(workspace, "Cargo.toml");
          const table = existsSync(candidate) ? Bun.TOML.parse(readFileSync(candidate, "utf8")) as unknown as Cargo : null;
          if (table?.workspace) {
            const inherited = table.workspace.dependencies?.[alias]; if (!inherited) throw Error("Missing inherited workspace edge " + alias);
            edge = inherited; base = workspace; break;
          }
          const parent = dirname(workspace); if (parent === workspace) throw Error("Missing workspace edge " + alias); workspace = parent;
        }
      }
      if (!edge.path) continue;
      const dependency = visit(resolve(base, edge.path, "Cargo.toml"));
      graph.setEdge(name, dependency); edges.get(name)!.push(dependency);
    }
    return name;
  };
  for (const row of corpus.crates) visit(resolve(ui, row.manifest));
  const active = new Set<string>(), visited = new Set<string>();
  const cycle = (name: string): boolean => {
    if (active.has(name)) return true; if (visited.has(name)) return false;
    active.add(name); if ((edges.get(name) ?? []).some(cycle)) return true;
    active.delete(name); visited.add(name); return false;
  };
  expect([...edges.keys()].some(cycle)).toBe(false);
  expect(independent.alg.findCycles(graph)).toEqual([]);
});

test("the enclosing renderer consumes declared producers without duplicating original source mounts", async () => {
  const workspace = resolve(ui, "../../.."), cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "feature=wgpu", "feature=wgpu-engine"];
  const enclosing = await readRustSourceClosureV1([resolve(ui, "📦️packages/🦀️rust/🦀️.rs")], workspace, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
  const consumer = Bun.TOML.parse(read("📦️packages/🦀️rust/Cargo.toml")) as unknown as Manifest;
  for (const row of corpus.crates.filter(row => ["semio-framework-ui-draw-contract", "semio-framework-ui-prepared", "semio-framework-ui-surface-identity"].includes(row.package))) {
    const edge = consumer.dependencies[row.package];
    expect(edge?.path).toBeDefined();
    expect(resolve(ui, "📦️packages/🦀️rust", edge.path!, "Cargo.toml")).toBe(resolve(ui, row.manifest));
    expect(enclosing.some(input => resolve(workspace, input.path) === resolve(ui, row.entry))).toBe(false);
  }
  const prepared = Bun.TOML.parse(read("🎯️targets/🧊️wgpu/🎟️prepared/📦️packages/🦀️rust/Cargo.toml")) as unknown as Manifest;
  expect(prepared.dependencies["semio-framework-ui-prepared-staging"]?.path).toBe("../../📤️staging/📦️packages/🦀️rust");
  expect(enclosing.some(input => input.path.includes("🎟️prepared/📤️staging/"))).toBe(false);
});

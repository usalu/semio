import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { normalize as oracleNormalize, join as oracleJoin, dirname as oracleDirname } from "pathe";
import { rustSourceDirectionEdges, rustSourceReferences, type RustSourceReference, type RustSourceDirectionEdge } from "../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
import type { DependencyDirectionRule } from "../../🕸️dependencies/🧭️direction/🟦️.ts";
import { inspectRustModuleGraph } from "../../🔍️discovery/🟦️.ts";

const library = resolve(import.meta.dir, "../.."), read = (path: string): any => JSON.parse(readFileSync(join(library, path), "utf8"));
const fixture = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json") as { cases: readonly { id: string; source: string; references: readonly RustSourceReference[] }[]; rules: readonly DependencyDirectionRule[]; directions: readonly { id: string; from: string; source: string; edges: readonly RustSourceDirectionEdge[] }[]; mounts: readonly { id: string; files: Readonly<Record<string, string>>; sourcePath: string; targets: readonly string[] }[]; unsupported: readonly { id: string; source: string }[] };

const dependencies = (cwd: string): readonly string[] => {
  const line = readFileSync(join(cwd, "dependencies.d"), "utf8").split("\n")[0]!, marker = line.indexOf(": ");
  if (marker < 0) throw new Error("rustc did not emit dependency provenance");
  return (line.slice(marker + 2).match(/(?:\\[ #\\]|[^\s])+/gu) ?? []).map((value) => {
    const path = value.replace(/\\([ #\\])/gu, "$1");
    return (isAbsolute(path) ? relative(cwd, path) : path).replaceAll("\\", "/");
  });
};

test("portable compile-time paths preserve exact literal identity and ignore lexical decoys", () => {
  const validate = new Ajv({ strict: true }).compile(read("🧬️schema/🧱️rust-source-direction/🔣️.json"));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const row of fixture.cases) expect(rustSourceReferences(row.source), row.id).toEqual(row.references);
});

test("rustc independently confirms literal inputs, nested module mounts and environment inputs", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust source oracle requires a ticket-owned test artifact directory");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-source-direction-")));
  try {
    for (const row of fixture.cases) {
      const cwd = join(root, row.id);
      mkdirSync(cwd);
      writeFileSync(join(cwd, "source.rs"), row.source);
      writeFileSync(join(cwd, "fixture.txt"), "neutral");
      writeFileSync(join(cwd, "leaf.rs"), "const LEAF: usize = 1;\n");
      writeFileSync(join(cwd, "module.rs"), "pub const MODULE: usize = 2;\n");
      const child = Bun.spawn(["rustc", "--crate-name", "boundary_oracle", "--crate-type", "lib", "--test", "--emit=dep-info=dependencies.d", "source.rs"], { cwd, env: { ...process.env, CARGO_MANIFEST_DIR: cwd, OUT_DIR: cwd }, stdout: "pipe", stderr: "pipe" });
      const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
      expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
      const actual = dependencies(cwd).filter((path) => path !== "source.rs").sort();
      const expected = row.id === "unused-macro-authored-input" ? [] : [...new Set(row.references.filter((ref) => !ref.directory).map((ref) => ref.base ? ref.path.slice(1) : ref.path))].sort();
      expect(actual, row.id).toEqual(expected);
    }
    for (const row of fixture.mounts) {
      const cwd = join(root, row.id);
      for (const [path, source] of Object.entries(row.files)) { mkdirSync(join(cwd, oracleDirname(path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
      const child = Bun.spawn(["rustc", "--crate-name", "boundary_oracle", "--crate-type", "lib", "--test", "--emit=dep-info=dependencies.d", "source.rs"], { cwd, env: { ...process.env, CARGO_MANIFEST_DIR: cwd }, stdout: "pipe", stderr: "pipe" });
      const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
      expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
      const actual = dependencies(cwd);
      const compileReferences = new Map(Object.entries(row.files).filter(([path]) => path.endsWith(".rs")).map(([path, source]) => [path, rustSourceReferences(source)]));
      const graph = inspectRustModuleGraph(Object.keys(row.files), (path) => row.files[path], { compileReferences });
      const refs = rustSourceReferences(row.files[row.sourcePath]!);
      const rules: DependencyDirectionRule[] = [{ name: "all-inputs", severity: "error", from: { path: [".*"] }, to: { path: [".*"] } }];
      const edges = rustSourceDirectionEdges(row.sourcePath, refs, rules, { contexts: graph.contexts.get(row.sourcePath) });
      const targets = edges.filter((edge) => row.targets.includes(edge.to)).map((edge) => edge.to);
      expect([...new Set(targets)].sort(), row.id).toEqual([...row.targets].sort());
      expect(targets.every((target) => actual.includes(target)), row.id).toBe(true);
      expect([...graph.targets.values()].filter((target) => row.targets.includes(target)).sort(), row.id).toEqual(row.targets.filter((target) => target.endsWith(".rs")).sort());
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("framework source and test paths cannot depend on implementation files", () => {
  for (const row of fixture.directions) {
    const refs = rustSourceReferences(row.source);
    expect(rustSourceDirectionEdges(row.from, refs, fixture.rules), row.id).toEqual(row.edges);
    const targets = refs.map((ref) => oracleNormalize(oracleJoin(oracleDirname(row.from), ref.path)));
    expect(row.edges.every((edge) => targets.includes(edge.to)), row.id).toBe(true);
  }
  expect(() => rustSourceDirectionEdges("general/tests/source.rs", rustSourceReferences('include!("../../../outside.rs");'), fixture.rules)).toThrow("escapes");
});

test("unsupported compile expressions fail closed", () => {
  for (const row of fixture.unsupported) expect(() => rustSourceReferences(row.source), row.id).toThrow("Unsupported Rust compile");
  const refs = rustSourceReferences('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../specific/fixture.txt"));');
  expect(() => rustSourceDirectionEdges("general/source.rs", refs, fixture.rules)).toThrow("manifest provenance");
  expect(rustSourceDirectionEdges("general/source.rs", refs, fixture.rules, { manifestPaths: ["general/Cargo.toml"] })).toEqual([{ rule: "framework-no-implementation", from: "general/source.rs", to: "specific/fixture.txt", kind: "include_str", line: 1 }]);
  expect(rustSourceDirectionEdges("general/source.rs", rustSourceReferences('include!(concat!(env!("OUT_DIR"), "/generated.rs"));'), fixture.rules)).toEqual([]);
});

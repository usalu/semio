/** ♻️ Contract: re-carving a carved artifact reads its declared mount tree back from the carved crates (tree-sitter and `@iarna/toml` are the independent oracles for the module tree and every manifest), keeps every unchanged crate byte for byte, places only the added source, never builds a crate cycle and writes nothing when nothing changed; the corpus is language-agnostic. */
import { describe, expect, setDefaultTimeout, test } from "bun:test";
import Ajv from "ajv";
import { parse } from "@iarna/toml";
import Parser from "web-tree-sitter";
import { createRequire } from "node:module";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { applyCarveWrites, carveWrites, planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { readCarvedLayout } from "../../../🏗️authoring/📦️crate-tree/🧱️carved-layout/🟦️.ts";
import { canonicalWiring, equivalentText } from "../../../🏗️authoring/📦️crate-tree/♻️recarve-delta/🟦️.ts";
import { writeTree } from "../🏗️virtual-crate/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(240000);
const wasm = join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")), "out", "tree-sitter-rust.wasm");
const parser = async (): Promise<Parser> => { await Parser.init(); const instance = new Parser(); instance.setLanguage(await Parser.Language.load(wasm)); return instance; };
const options = { ...corpus.budget, mutationFamilies: "off" as const };
const lay = (files: Record<string, string>): string => writeTree("semio-recarve-", files);
const put = (root: string, files: Record<string, string>): void => { for (const [path, text] of Object.entries(files)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); } };
const carve = async (root: string, force = false) => {
  const plan = await planArtifactCrates(root, corpus.packageDir, force ? { ...options, carved: null } : options);
  const { writes } = await carveWrites(plan);
  return { plan, writes };
};
const manifestsOf = (root: string, dirs: readonly string[]): Map<string, { dir: string; deps: string[] }> => {
  const found = new Map<string, { dir: string; deps: string[] }>();
  for (const dir of dirs) { const text = parse(readFileSync(join(root, dir, "Cargo.toml"), "utf8")) as any; found.set(text.package.name, { dir, deps: Object.entries<any>(text.dependencies ?? {}).filter(([, spec]) => typeof spec === "object" && spec.path).map(([name]) => name) }); }
  return found;
};
const acyclic = (graph: Map<string, { deps: string[] }>): boolean => {
  const state = new Map<string, number>();
  const visit = (name: string): boolean => { if (state.get(name) === 1) return false; if (state.get(name) === 2 || !graph.has(name)) return true; state.set(name, 1); for (const dep of graph.get(name)!.deps) if (!visit(dep)) return false; state.set(name, 2); return true; };
  return [...graph.keys()].every(visit);
};
const crateFiles = (root: string, plan: { crates: readonly { name: string; packageDir: string }[] }): Map<string, string> => new Map(plan.crates.flatMap((crate) => ["Cargo.toml", "🦀️.rs"].map((file): [string, string] => [`${crate.name}:${file}`, readFileSync(join(root, crate.packageDir, file), "utf8")])));

describe("re-carve of a carved artifact", () => {
  test("the corpus is closed under its schema", () => { const ajv = new Ajv({ strict: true }); expect(ajv.validate(schema, corpus), JSON.stringify(ajv.errors)).toBe(true); });

  test("the canonical wiring form ignores order and indentation and nothing else", () => {
    expect(canonicalWiring("pub mod a {\n    pub use x::a;\n    pub use y::b;\n}\n")).toBe(canonicalWiring("pub mod a {\npub use y::b;\npub use x::a;\n}"));
    expect(canonicalWiring("pub mod a {\n pub use x::a;\n}\n")).not.toBe(canonicalWiring("pub mod b {\n pub use x::a;\n}\n"));
    expect(canonicalWiring("#[cfg(test)]\npub use x::a;\n")).not.toBe(canonicalWiring("pub use x::a;\n"));
    expect(equivalentText("a/Cargo.toml", "[a]\nx = 1\ny = 2\n", "[a]\ny = 2\nx = 1\n", false)).toBe(true);
    expect(equivalentText("a/Cargo.toml", "[a]\nx = 1\n", "[a]\nx = 2\n", false)).toBe(false);
    expect(equivalentText("a/x.rs", "fn a(){}\n\n", "fn a() {}\n", false)).toBe(true);
  });

  test("reads the declared mount tree back from the carved crates (tree-sitter agrees on the module tree)", async () => {
    const root = lay(corpus.files);
    try {
      const { writes } = await carve(root, true);
      applyCarveWrites(root, writes);
      const layout = readCarvedLayout(root, corpus.packageDir)!;
      expect(layout.crates.map((crate) => crate.name).sort()).toEqual([...corpus.crates].sort());
      const instance = await parser();
      const modules = (text: string): string[] => { const tree = instance.parse(text); expect(tree.rootNode.hasError()).toBe(false); return tree.rootNode.namedChildren.filter((node) => node.type === "mod_item").map((node) => node.childForFieldName("name")!.text); };
      expect(modules(layout.rootSource).sort()).toEqual([...corpus.virtualRoot.modules].sort());
      const items = instance.parse(layout.rootSource).rootNode.descendantsOfType("const_item").map((node) => node.childForFieldName("name")!.text);
      expect(items).toEqual(corpus.virtualRoot.items);
      for (const [file, expected] of Object.entries(corpus.virtualRoot.files)) expect(modules(layout.overlay.get(resolve(root, file))!), file).toEqual(expected);
      expect(layout.uncarvedManifest).toBe(corpus.files["📦️packages/🦀️rust/Cargo.toml"]);
      expect(parse(layout.uncarvedManifest)).toEqual(parse(corpus.files["📦️packages/🦀️rust/Cargo.toml"]));
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("an unchanged carved artifact re-carves to the same crates with zero writes", async () => {
    const root = lay(corpus.files);
    try {
      const first = await carve(root, true);
      applyCarveWrites(root, first.writes);
      const again = await carve(root);
      expect(again.plan.problems).toEqual([]);
      expect(again.plan.crates.map((crate) => [crate.name, crate.packageDir]).sort()).toEqual(first.plan.crates.map((crate) => [crate.name, crate.packageDir]).sort());
      expect(again.writes.map((write) => `${write.action} ${write.path}`)).toEqual([]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("an added source is placed in a new crate, every unchanged crate stays byte for byte, the crate graph stays acyclic and the next re-carve writes nothing", async () => {
    const root = lay(corpus.files);
    try {
      const first = await carve(root, true);
      applyCarveWrites(root, first.writes);
      const before = crateFiles(root, first.plan);
      put(root, corpus.addition.files);
      const second = await carve(root);
      expect(second.plan.problems).toEqual([]);
      expect(second.plan.crates.map((crate) => crate.name).sort()).toEqual([...corpus.crates, corpus.addition.crate].sort());
      const added = second.plan.crates.find((crate) => crate.name === corpus.addition.crate)!;
      expect(added.dependsOn.map((id) => second.plan.crates.find((crate) => crate.id === id)!.name)).toEqual(corpus.addition.dependsOn);
      const touched = second.writes.map((write) => write.path);
      for (const name of corpus.addition.unchanged) { const crate = first.plan.crates.find((entry) => entry.name === name)!; expect(touched.some((path) => path.startsWith(dirname(crate.packageDir) + "/") || path === crate.packageDir || path.startsWith(crate.packageDir + "/")), name).toBe(false); }
      expect(touched).toContain(join(added.packageDir, "Cargo.toml"));
      expect(touched).toContain("🦀️.rs");
      applyCarveWrites(root, second.writes);
      const after = crateFiles(root, first.plan);
      for (const [key, text] of before) expect(after.get(key), key).toBe(text);
      const graph = manifestsOf(root, second.plan.crates.map((crate) => crate.packageDir));
      expect(acyclic(graph)).toBe(true);
      expect(graph.get(corpus.addition.crate)!.deps).toEqual(corpus.addition.dependsOn);
      expect(parse(readFileSync(join(root, "📦️packages/🦀️rust/Cargo.toml"), "utf8")).dependencies).toHaveProperty([corpus.addition.crate]);
      const third = await carve(root);
      expect(third.plan.problems).toEqual([]);
      expect(third.writes.map((write) => `${write.action} ${write.path}`)).toEqual([]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("a source that closes a dependency cycle between carved crates merges them instead of cycling", async () => {
    const root = lay(corpus.files);
    try {
      const first = await carve(root, true);
      applyCarveWrites(root, first.writes);
      put(root, corpus.cycle.files);
      const second = await carve(root);
      expect(second.plan.problems).toEqual([]);
      expect(second.plan.crates.length).toBeLessThan(first.plan.crates.length + 1);
      applyCarveWrites(root, second.writes);
      expect(acyclic(manifestsOf(root, second.plan.crates.map((crate) => crate.packageDir)))).toBe(true);
      const third = await carve(root);
      expect(third.writes.map((write) => `${write.action} ${write.path}`)).toEqual([]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
  test("a new source edge between pinned crates becomes a manifest dependency when acyclic and leaves every other crate byte for byte", async () => {
    const root = lay(corpus.files);
    try {
      const first = await carve(root, true);
      applyCarveWrites(root, first.writes);
      const before = crateFiles(root, first.plan);
      put(root, corpus.edge.files);
      const second = await carve(root);
      expect(second.plan.problems).toEqual([]);
      expect(second.plan.crates.map((crate) => crate.name).sort()).toEqual([...corpus.crates].sort());
      applyCarveWrites(root, second.writes);
      const graph = manifestsOf(root, second.plan.crates.map((crate) => crate.packageDir));
      expect(graph.get(corpus.edge.crate)!.deps).toContain(corpus.edge.gains);
      expect(acyclic(graph)).toBe(true);
      const after = crateFiles(root, first.plan);
      for (const name of corpus.edge.unchanged) for (const file of ["Cargo.toml", "🦀️.rs"]) expect(after.get(`${name}:${file}`), name).toBe(before.get(`${name}:${file}`));
      expect((await carve(root)).writes.map((write) => `${write.action} ${write.path}`)).toEqual([]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
  test("a component nested in a root wrapper is re-exported by the re-carved facade, not mounted again", async () => {
    const root = lay(corpus.nested.files);
    try {
      const first = await carve(root, true);
      applyCarveWrites(root, first.writes);
      const again = await carve(root);
      expect(again.plan.problems).toEqual([]);
      expect(again.writes.map((write) => `${write.action} ${write.path}`)).toEqual([]);
      const facade = readFileSync(join(root, "🦀️.rs"), "utf8");
      for (const needle of corpus.nested.forbidden) expect(facade, needle).not.toContain(needle);
      expect(facade.match(/pub mod standards\b/gu)?.length).toBe(1);
      expect(facade).toContain("pub use demo_weave_io::standards::io::*;");
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
});

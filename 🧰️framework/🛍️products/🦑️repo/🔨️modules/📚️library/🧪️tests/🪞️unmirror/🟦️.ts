/** 🪞️ Unmirror: tests that the path-preserving re-export layers of a carve are deleted and every consumer is re-pointed at the owning crate. */
import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { Unmirror } from "../../🏗️authoring/📦️crate-tree/🪞️unmirror/🟦️.ts";
import { CycleMemo } from "../../🏗️authoring/📦️crate-tree/🪞️unmirror/🔁️cycle-memo/🟦️.ts";
import { Universe } from "../../🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🟦️.ts";
import { addDependencyLines, declaredDependencies, removeDependencyLines } from "../../🏗️authoring/📦️crate-tree/🪞️unmirror/📝️manifest/🟦️.ts";
import { renderUseLeaves, scanSource, scanUseStatements } from "../../🏗️authoring/📦️crate-tree/🪞️unmirror/🧵️use-tree/🟦️.ts";

type Files = Readonly<Record<string, string>>;

const run = (files: Files, crates: readonly (readonly [string, string])[], targets: readonly string[], apiRoots: readonly string[], processed: readonly string[]): ReadonlyMap<string, string> => {
  const dir = mkdtempSync(join(tmpdir(), "semio-unmirror-"));
  for (const [path, text] of Object.entries(files)) { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); }
  const universe = new Universe(dir);
  for (const [name, root] of crates) universe.addCrate(name, dirname(root), root);
  const engine = new Unmirror(universe, { apiRoots: new Set(apiRoots), targets: new Set(targets), editable: () => true });
  for (const [name] of crates) universe.root(name.replaceAll("-", "_"));
  for (const path of processed) engine.processFile(path);
  return engine.report.edits;
};

const owner = "pub mod schema {\n    pub struct Item;\n    pub fn make() -> Item { Item }\n    pub mod inner { pub struct Deep; }\n}\n";
const facade = (extra = ""): string => `pub mod schema {\n    pub use fx_b::schema::*;\n}\n#[path = "consumer.rs"]\npub mod consumer;\n${extra}`;

describe("unmirror", () => {
  test("re-points a private import and an inline path to the owning crate and deletes the emptied mirror module", () => {
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": facade(), "a/consumer.rs": "use crate::schema::Item;\npub fn f() -> Item { crate::schema::make() }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::schema::Item;\npub fn f() -> Item { fx_b::schema::make() }\n");
    expect(edits.get("a/lib.rs")).not.toContain("pub use fx_b");
    expect(edits.get("a/lib.rs")).not.toContain("mod schema");
    expect(edits.get("a/lib.rs")).toContain("pub mod consumer;");
  });

  test("replaces a glob over a mirror module with the explicit imports of the names the file uses", () => {
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": facade(), "a/consumer.rs": "use crate::schema::*;\npub fn f() -> Item { make() }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    const text = edits.get("a/consumer.rs")!;
    expect(text).toContain("use fx_b::schema::{Item, make};");
    expect(text).not.toContain("*");
  });

  test("imports the names a deleted glob over a re-exported foreign module provided, straight from that module", () => {
    const own = "pub use fx_b::schema;\npub use schema::*;\n#[path = \"consumer.rs\"]\npub mod consumer;\n";
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": own, "a/consumer.rs": "use crate::*;\npub fn f() -> Item { make() }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/lib.rs")).toBe("#[path = \"consumer.rs\"]\npub mod consumer;\n");
    expect(edits.get("a/consumer.rs")).toContain("use fx_b::schema::{Item, make};");
  });

  test("keeps the glob while the module still has items of its own", () => {
    const own = "pub mod schema {\n    pub use fx_b::schema::*;\n    pub struct Local;\n}\n#[path = \"consumer.rs\"]\npub mod consumer;\n";
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": own, "a/consumer.rs": "use crate::schema::*;\npub fn f() -> (Item, Local) { (make(), Local) }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    const text = edits.get("a/consumer.rs")!;
    expect(text).toContain("use crate::schema::*;");
    expect(text).toContain("use fx_b::schema::{Item, make};");
    expect(text).not.toContain("Local;\n");
    expect(edits.get("a/lib.rs")).not.toContain("pub use fx_b");
  });

  test("keeps the explicit root re-exports of an artifact facade as its API, re-pointed to the owner", () => {
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": facade("pub use crate::schema::Item;\n") }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], ["fx_a"], ["a/lib.rs"]);
    expect(edits.get("a/lib.rs")).toContain("pub use fx_b::schema::Item;");
    expect(edits.get("a/lib.rs")).not.toContain("pub use fx_b::schema::*");
  });

  test("re-points a file of a crate that is not unmirrored only where its path starts in an unmirrored crate", () => {
    const files = { "b/lib.rs": owner, "a/lib.rs": facade(), "c/lib.rs": "use fx_a::schema::Item;\nuse fx_d::other::Thing;\npub fn f(_: Item, _: Thing) {}\n", "d/lib.rs": "pub mod other { pub use fx_b::schema::Item as Thing; }\n" };
    const edits = run(files, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"], ["fx-c", "c/lib.rs"], ["fx-d", "d/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "c/lib.rs"]);
    expect(edits.get("c/lib.rs")).toBe("use fx_b::schema::Item;\nuse fx_d::other::Thing;\npub fn f(_: Item, _: Thing) {}\n");
  });

  test("re-points a path below a mirrored module and drops the import that became unused", () => {
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": facade(), "a/consumer.rs": "use crate::schema::inner;\npub fn f(_: inner::Deep) {}\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("pub fn f(_: fx_b::schema::inner::Deep) {}\n");
  });

  test("removes a rename alias of a deleted re-export by renaming the identifiers in its consumers", () => {
    const own = "pub use fx_b::schema::Item as Renamed;\n#[path = \"consumer.rs\"]\npub mod consumer;\n";
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": own, "a/consumer.rs": "use crate::Renamed;\npub fn f(_: Renamed) -> Vec<Renamed> { Vec::new() }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::schema::Item;\npub fn f(_: Item) -> Vec<Item> { Vec::new() }\n");
    expect(edits.get("a/lib.rs")).not.toContain("Renamed");
  });

  test("re-points a glob over the variants of an enum owned by another crate", () => {
    const edits = run({ "b/lib.rs": `${owner}pub mod kinds { pub enum Kind { A, B } }\n`, "a/lib.rs": `${facade()}pub mod kinds {\n    pub use fx_b::kinds::*;\n}\n`, "a/consumer.rs": "use crate::kinds::Kind::*;\npub fn f() -> bool { matches!(A, B) }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::kinds::Kind::*;\npub fn f() -> bool { matches!(A, B) }\n");
  });

  test("resolves through a dense ring of crates that re-export each other without revisiting a node once per path", () => {
    const size = 24;
    const ring = (index: number): string => [1, 2, size - 1].map((step) => `    pub use fx_${(index + step) % size}::m::*;`).join("\n");
    const files: Record<string, string> = { "k/lib.rs": "pub mod m { pub struct Item; }\n", "c0/consumer.rs": "use crate::m::Item;\npub fn f(_: Item) {}\n" };
    const crates: [string, string][] = [["fx-k", "k/lib.rs"]];
    for (let index = 0; index < size; index += 1) {
      files[`c${index}/lib.rs`] = `pub mod m {\n${ring(index)}\n${index === 0 ? "    pub use fx_k::m::*;\n" : ""}}\n${index === 0 ? '#[path = "consumer.rs"]\npub mod consumer;\n' : ""}`;
      crates.push([`fx-${index}`, `c${index}/lib.rs`]);
    }
    const started = Date.now();
    const edits = run(files, crates, crates.map(([name]) => name.replaceAll("-", "_")), [], ["c0/lib.rs", "c0/consumer.rs"]);
    expect(Date.now() - started).toBeLessThan(10000);
    expect(edits.get("c0/consumer.rs")).toBe("use fx_k::m::Item;\npub fn f(_: Item) {}\n");
  });

  test("re-points the module prefix of a path whose last segment a macro generates in the owner crate", () => {
    const generated = "pub mod schema {\n    pub mod conf { pub mod sub { generated!(); } }\n}\n";
    const edits = run({ "b/lib.rs": generated, "a/lib.rs": facade(), "a/consumer.rs": "use crate::schema::conf;\npub fn f() { conf::sub::made(); }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("pub fn f() { fx_b::schema::conf::sub::made(); }\n");
  });

  test("keeps an imported module head inside a macro invocation and re-points only the import, but still re-points a crate-rooted path there", () => {
    const consumer = "use crate::schema::inner;\nmacro_rules! pick { ($module:ident :: $leaf:ident) => { $module::$leaf }; }\npub fn f() { let _ = pick!(inner::Deep); let _ = vec![crate::schema::make()]; }\n";
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": facade(), "a/consumer.rs": consumer }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::schema::inner;\nmacro_rules! pick { ($module:ident :: $leaf:ident) => { $module::$leaf }; }\npub fn f() { let _ = pick!(inner::Deep); let _ = vec![fx_b::schema::make()]; }\n");
  });

  test("re-points a path through a local module inside a macro invocation when only the module's own forwarding glob is deleted", () => {
    const own = "pub mod schema {\n    pub use fx_b::schema::*;\n    pub mod windows {\n        pub use fx_b::schema::inner::*;\n        pub struct Local;\n    }\n}\n#[path = \"consumer.rs\"]\npub mod consumer;\n";
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": own, "a/consumer.rs": "use crate::schema::windows;\npub fn f() -> bool { matches!(1, 1) || matches!(0u8, _ if windows::Deep::default() == 1) }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toContain("fx_b::schema::inner::Deep::default()");
    expect(edits.get("a/lib.rs")).toContain("pub struct Local;");
  });

  test("re-points a macro argument through a head that only reaches the owner via a forwarding glob of its own module", () => {
    const files = { "b/lib.rs": "pub mod inner { pub struct Deep; }\n", "a/lib.rs": "pub mod codec {\n    pub use fx_b::inner::*;\n    pub struct Own;\n}\n", "c/lib.rs": "pub use fx_a::codec;\n#[path = \"consumer.rs\"]\npub mod consumer;\n", "c/consumer.rs": "use crate::codec;\npub fn f() { assert!(matches!(codec::Deep, _)); }\n" };
    const edits = run(files, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"], ["fx-c", "c/lib.rs"]], ["fx_a", "fx_c"], [], ["a/lib.rs", "c/lib.rs", "c/consumer.rs"]);
    expect(edits.get("c/consumer.rs")).toContain("fx_b::inner::Deep");
  });

  test("resolves a name the owner declares inside the body of a macro it invokes, and never a prelude word", () => {
    const generated = "pub mod schema {\n    macro_rules! gen { () => { pub fn made() {} }; }\n    gen!();\n    pub struct Item;\n}\n";
    const edits = run({ "b/lib.rs": generated, "a/lib.rs": facade(), "a/consumer.rs": "pub fn f() { crate::schema::made(); let _ = Some(1); }\nuse crate::schema::*;\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toContain("fx_b::schema::made();");
    expect(edits.get("a/consumer.rs")).not.toContain("Some;");
  });

  test("treats a forwarding module whose only own children are cfg(test) mounts as a pure forwarder", () => {
    const shell = "pub mod schema {\n    pub use fx_b::schema::*;\n    #[cfg(test)]\n    #[path = \"tests.rs\"]\n    pub mod tests;\n}\n#[path = \"consumer.rs\"]\npub mod consumer;\n";
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": shell, "a/tests.rs": "", "a/consumer.rs": "use crate::schema;\npub fn f() { assert!(schema::make() == schema::make()); }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::schema;\npub fn f() { assert!(schema::make() == schema::make()); }\n");
    expect(edits.get("a/lib.rs")).not.toContain("pub use fx_b");
    expect(edits.get("a/lib.rs")).toContain("pub mod tests;");
  });

  test("imports a name the owner re-exports from a crate outside the repository straight from the owner", () => {
    const external = "pub mod schema {\n    pub use outside::geometry::Kind;\n    pub struct Item;\n}\n";
    const edits = run({ "b/lib.rs": external, "a/lib.rs": facade(), "a/consumer.rs": "use crate::schema::*;\npub fn f(_: Kind, _: Item) {}\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toContain("use fx_b::schema::{Kind, Item};");
  });

  test("re-points an import whose last segment only a macro in the owner declares, by its resolvable module prefix", () => {
    const generated = "pub mod schema {\n    pub mod conf { generated!(); }\n}\n";
    const edits = run({ "b/lib.rs": generated, "a/lib.rs": facade(), "a/consumer.rs": "use crate::schema::conf::Made;\npub fn f(_: Made) {}\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::schema::conf::Made;\npub fn f(_: Made) {}\n");
  });

  test("re-points an import with an unknown last segment through the only opaque module a forwarding module globs", () => {
    const generated = "pub mod schema {\n    pub mod conf { generated!(); }\n}\npub mod plain { pub struct Item; }\n";
    const shell = "pub mod bim {\n    pub use fx_b::schema::conf::*;\n    pub use fx_b::plain::*;\n    #[cfg(test)]\n    #[path = \"tests.rs\"]\n    pub mod tests;\n}\n#[path = \"consumer.rs\"]\npub mod consumer;\n";
    const edits = run({ "b/lib.rs": generated, "a/lib.rs": shell, "a/tests.rs": "", "a/consumer.rs": "use crate::bim::Made;\npub fn f(_: Made) {}\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::schema::conf::Made;\npub fn f(_: Made) {}\n");
  });

  test("keeps a public glob over a module with items of its own and imports, privately, the names that glob only reached through a deleted edge", () => {
    const own = "pub mod patches {\n    pub use fx_b::schema::*;\n    pub struct Local;\n}\npub use patches::*;\npub fn f(_: Item, _: Local) {}\n";
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": own }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs"]);
    const text = edits.get("a/lib.rs")!;
    expect(text).toContain("pub use patches::*;");
    expect(text).toContain("use fx_b::schema::Item;");
    expect(text).not.toContain("pub use fx_b");
  });

  test("imports a name that a glob provided and the file only uses as an inline format argument", () => {
    const edits = run({ "b/lib.rs": "pub mod schema {\n    pub const ROOT: &str = \"r\";\n}\n", "a/lib.rs": facade(), "a/consumer.rs": "use crate::schema::*;\npub fn f() -> String { format!(\"{ROOT}.x\") }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/lib.rs", "a/consumer.rs"]);
    expect(edits.get("a/consumer.rs")).toBe("use fx_b::schema::ROOT;\npub fn f() -> String { format!(\"{ROOT}.x\") }\n");
  });

  test("leaves a file alone when nothing crosses a deleted edge", () => {
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": facade(), "a/consumer.rs": "pub fn f() -> u8 { 1 }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/consumer.rs"]);
    expect(edits.has("a/consumer.rs")).toBe(false);
  });
});

describe("unmirror audit", () => {
  const audit = (owner: string): string[] => {
    const dir = mkdtempSync(join(tmpdir(), "semio-unmirror-"));
    for (const [path, text] of Object.entries({ "b/lib.rs": owner, "a/lib.rs": "pub fn f() { fx_b::schema::conf::made(); }\n" })) { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); }
    const universe = new Universe(dir);
    for (const [name, root] of [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]] as const) universe.addCrate(name, dirname(root), root);
    for (const ident of ["fx_b", "fx_a"]) universe.root(ident);
    return new Unmirror(universe, { apiRoots: new Set(), targets: new Set(), editable: () => false }).audit("a/lib.rs");
  };

  test("flags a path whose last segment the owner module does not declare", () => {
    expect(audit("pub mod schema { pub mod conf { pub fn other() {} } }\n")).toEqual(["path fx_b::schema::conf::made"]);
  });

  test("tolerates a path into an owner module that a macro invocation fills", () => {
    expect(audit("pub mod schema { pub mod conf { generated!(); } }\n")).toEqual([]);
  });
});

describe("unmirror use trees", () => {
  test("flattens nested use trees and renders them grouped by parent", () => {
    const statements = scanUseStatements(scanSource("pub(crate) use a::b::{c, d as e, f::{self, g}};\nuse h::*;\n"));
    expect(statements.map((statement) => statement.visibility)).toEqual(["pub(crate)", ""]);
    expect(statements[0]!.leaves.map((leaf) => leaf.path.join("::") + (leaf.alias ? ` as ${leaf.alias}` : ""))).toEqual(["a::b::c", "a::b::d as e", "a::b::f", "a::b::f::g"]);
    expect(renderUseLeaves(statements[0]!.leaves, "pub(crate)", [], "")).toBe("pub(crate) use a::b::{c, d as e, f};\npub(crate) use a::b::f::g;");
    expect(statements[1]!.leaves[0]!.glob).toBe(true);
  });

  test("scans function-local use statements and keeps their attributes", () => {
    const statements = scanUseStatements(scanSource("fn f() {\n    #[cfg(test)]\n    use a::b;\n}\n"));
    expect(statements).toHaveLength(1);
    expect(statements[0]!.attributes).toEqual(["#[cfg(test)]"]);
  });
});

describe("unmirror manifests", () => {
  const manifest = "[package]\nname = \"x\"\n\n[dependencies]\nb = { path = \"../b\" }\nd = { path = \"../d\" }\n\n[dev-dependencies]\nz = { path = \"../z\" }\n";
  test("adds a dependency in byte order and creates a missing section", () => {
    const added = addDependencyLines(manifest, [{ name: "c", path: "../c", section: "dependencies" }, { name: "q", path: "../q", section: "dev-dependencies" }]);
    expect(added.split("\n").filter((line) => line.endsWith("}"))).toEqual(['b = { path = "../b" }', 'c = { path = "../c" }', 'd = { path = "../d" }', 'q = { path = "../q" }', 'z = { path = "../z" }']);
    expect(addDependencyLines("[package]\nname = \"x\"\n", [{ name: "c", path: "../c", section: "dependencies" }])).toContain("[dependencies]\nc = { path = \"../c\" }");
  });

  test("removes only the named dependency lines and lists the declared names", () => {
    expect(removeDependencyLines(manifest, new Set(["d"]))).not.toContain("d = ");
    expect([...declaredDependencies(manifest).keys()]).toEqual(["b", "d", "z"]);
  });
});

describe("unmirror cycle memo", () => {
  test("answers a lookup that reaches its own computation with the fallback and runs each node once", () => {
    const memo = new CycleMemo();
    const runs = new Map<string, number>();
    const edges: Record<string, string[]> = { a: ["b", "c"], b: ["a", "c"], c: ["a", "b", "leaf"], leaf: [] };
    const reach = (node: string): string[] => memo.compute(node, () => [], () => {
      runs.set(node, (runs.get(node) ?? 0) + 1);
      return [...new Set([node, ...edges[node]!.flatMap(reach)])].sort();
    });
    expect(reach("a")).toEqual(["a", "b", "c", "leaf"]);
    expect(reach("c")).toEqual(["a", "b", "c", "leaf"]);
    expect(runs.get("leaf")).toBe(1);
    expect(runs.get("a")).toBe(1);
    expect([...runs.values()].every((count) => count <= 2)).toBe(true);
  });

  test("keeps a result that never met a cycle for every later query", () => {
    const memo = new CycleMemo();
    let runs = 0;
    const leaf = (): number => memo.compute("leaf", () => -1, () => { runs += 1; return 7; });
    expect(leaf()).toBe(7);
    expect(leaf()).toBe(7);
    expect(runs).toBe(1);
  });
});

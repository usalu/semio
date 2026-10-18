/** 🪞️ Unmirror: tests that the path-preserving re-export layers of a carve are deleted and every consumer is re-pointed at the owning crate. */
import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { Unmirror } from "../../🏗️authoring/📦️crate-tree/🪞️unmirror/🟦️.ts";
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

  test("leaves a file alone when nothing crosses a deleted edge", () => {
    const edits = run({ "b/lib.rs": owner, "a/lib.rs": facade(), "a/consumer.rs": "pub fn f() -> u8 { 1 }\n" }, [["fx-b", "b/lib.rs"], ["fx-a", "a/lib.rs"]], ["fx_a"], [], ["a/consumer.rs"]);
    expect(edits.has("a/consumer.rs")).toBe(false);
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

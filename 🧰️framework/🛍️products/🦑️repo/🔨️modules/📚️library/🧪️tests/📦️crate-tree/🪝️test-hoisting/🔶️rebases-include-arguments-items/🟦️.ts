/** 🔶️ The rebases include arguments items tests of the artifact crate carve suite. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { parseRustSourceTree, prunedUse } from "../../../../🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { crateRelativeChains } from "../../../../🏗️authoring/📦️crate-tree/🗺️plan/🧭️module-chains/🟦️.ts";
import { rebaseManifestDirPaths, rebaseRustIncludes } from "../../../../🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { inlineMountTreeFiles } from "../../../../🏗️authoring/📦️crate-tree/🌲️inline-mounts/🟦️.ts";
import { transformTestClosure } from "../../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

/** 🔷 Registers the rebases include arguments items tests of the artifact crate carve suite. */
export function registerRebasesIncludeArgumentsItemsTests(): void {
  test("rebases include arguments of items moved to another directory", () => {
    const moved = 'pub const A: &str = include_str!("📜️x.json");\nconst B: &[u8] = include_bytes!("../up/y.bin");\nfn f() { let _ = "include_str!(\\"no\\")"; }';
    const rebased = rebaseRustIncludes(moved, "/repo/a", "/repo/a/deep/er");
    expect(rebased).toContain('include_str!("../../📜️x.json")');
    expect(rebased).toContain('include_bytes!("../../../up/y.bin")');
    expect(rebased).toContain('"include_str!(\\"no\\")"');
    expect(rebaseRustIncludes(moved, "/repo/a", "/repo/a")).toBe(moved);
  });
  test("hoists a root-bound test only when it needs nothing from its host module, and takes the doc comment of the removed mount with it", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": "pub fn host() {}\n/// 🧪️ documents the next item, which is the test mount\n// a plain comment between does not stop the attachment\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️free/🦀️.rs\"]\nmod free;\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️bound/🦀️.rs\"]\nmod bound;\n",
        "a/🧪️tests/🔬️free/🦀️.rs": "#[test]\nfn free() { assert_eq!(crate::DOCUMENT, \"d\"); }\n",
        "a/🧪️tests/🔬️bound/🦀️.rs": "use super::*;\n#[test]\nfn bound() { host(); assert_eq!(crate::DOCUMENT, \"d\"); }\n",
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).toContain("mod hoisted_");
      expect(result.rootSource).toContain("🔬️free/🦀️.rs");
      expect(result.rootSource).not.toContain("🔬️bound/🦀️.rs");
      const host = result.edits.get(join(root, "a/🦀️.rs"))!;
      expect(host).not.toContain("mod free;");
      expect(host).not.toContain("documents the next item");
      expect(host).toContain("mod bound;");
      expect(host).toContain("pub fn host() {}");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("folds a root-level mount tree file back into the root with re-based paths and marks the file for deletion", () => {
    const root = "/repo/art";
    const files: Record<string, string> = {
      "/repo/art/🌳️tree/🦀️.rs": '//! 🌳️ The mount tree.\n\n#[path = "."]\npub mod v1 {\n    #[path = "../x/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "."]\n    pub mod deep {\n        #[path = "../x/y/🦀️.rs"]\n        pub mod leaf;\n    }\n}\n',
      "/repo/art/🧾️decl/🦀️.rs": "pub fn declaration() {}\n",
    };
    const source = '#[path = "🌳️tree/🦀️.rs"]\npub mod standards;\n#[path = "🧾️decl/🦀️.rs"]\nmod decl;\n';
    const folded = inlineMountTreeFiles(`${root}/🦀️.rs`, source, (absolute) => files[absolute] ?? null);
    expect(folded.deletes).toEqual(new Set(["/repo/art/🌳️tree/🦀️.rs"]));
    expect(folded.rootSource).toContain('#[path = "."]\npub mod standards {');
    expect(folded.rootSource).toContain('#[path = "x/🦀️.rs"]');
    expect(folded.rootSource).toContain('#[path = "x/y/🦀️.rs"]');
    expect(folded.rootSource).toContain('#[path = "🧾️decl/🦀️.rs"]\nmod decl;');
    expect(folded.rootSource).not.toContain("The mount tree");
    expect(parseRustSourceTree(folded.rootSource).modules.map((node) => [node.name, node.inline])).toEqual([["standards", true], ["decl", false]]);
  });
  test("detaches a test whose host imports an item with a relative super path by carrying the absolute import", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-detach-super-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "."]\npub mod m {\n    #[path = "m/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "m/leaf/🦀️.rs"]\n    pub mod leaf;\n}\n',
        "m/🦀️.rs": "pub struct Agg;\n",
        "m/leaf/🦀️.rs": 'use super::Agg;\nuse crate::m::{self, Agg as Alias};\npub struct Leaf;\npub fn make() -> Agg { Agg }\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "m/leaf/🧪️tests/🦀️.rs": "use super::*;\n#[test]\nfn t() { let _: Agg = make(); let _: Alias = Agg; let _ = Leaf; }\n",
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: false, hoist: false, detach: true });
      const test = result.edits.get(join(root, "m/leaf/🧪️tests/🦀️.rs"));
      expect(test).toContain("use crate::m::Agg;");
      expect(test).toContain("use crate::m::Agg as Alias;");
      expect(test).not.toMatch(/\buse super::/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("detaches a test that imports a private host function through super and widens that function to pub", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-detach-super-private-fn-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "."]\npub mod m {\n    #[path = "m/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "m/🦀️.rs": 'fn examples() -> usize { 3 }\npub fn count() -> usize { examples() }\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "m/🧪️tests/🦀️.rs": "use super::examples;\n#[test]\nfn t() { assert_eq!(examples(), 3); }\n",
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: false, hoist: false, detach: true });
      expect(result.edits.get(join(root, "m/🦀️.rs"))).toContain("pub fn examples()");
      expect(result.edits.get(join(root, "m/🧪️tests/🦀️.rs"))).toContain("use crate::m::examples;");
      expect(result.edits.get(join(root, "m/🧪️tests/🦀️.rs"))!.match(/use crate::m::(?:\{[^}]*\b)?examples\b/gu)).toHaveLength(1);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("detaches a test whose host groups several items behind a leading super run into one absolute group", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-detach-super-group-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "."]\npub mod m {\n    #[path = "m/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "m/mutations/🦀️.rs"]\n    pub mod mutations;\n}\n',
        "m/🦀️.rs": "pub struct Cfg;\npub struct CfgMutation;\npub struct CfgDelta;\n",
        "m/mutations/🦀️.rs": '#[path = "set/🦀️.rs"]\nmod set;\npub use set::S;\n',
        "m/mutations/set/🦀️.rs": 'use super::super::{Cfg, CfgDelta, CfgMutation};\npub struct S;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "m/mutations/set/🧪️tests/🦀️.rs": "use super::*;\n#[test]\nfn t() { let _ = Cfg; let _ = CfgMutation; let _ = S; }\n",
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: false, hoist: false, detach: true });
      const test = result.edits.get(join(root, "m/mutations/set/🧪️tests/🦀️.rs"));
      expect(test).toContain("use crate::m::{Cfg, CfgMutation};");
      expect(test).not.toMatch(/::\{crate::/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("detaches a test that globs its host by carrying the host's private glob imports", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-detach-glob-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "."]\npub mod k {\n    #[path = "k/🦀️.rs"]\n    pub mod base;\n}\n#[path = "."]\npub mod m {\n    #[path = "m/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "m/leaf/🦀️.rs"]\n    pub mod leaf;\n}\n',
        "k/🦀️.rs": "pub struct Id;\n",
        "m/🦀️.rs": "pub struct Agg;\n",
        "m/leaf/🦀️.rs": 'use crate::k::base::*;\npub struct Leaf;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "m/leaf/🧪️tests/🦀️.rs": "use super::*;\n#[test]\nfn t() { let _ = Id; let _ = Leaf; }\n",
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: false, hoist: false, detach: true });
      const test = result.edits.get(join(root, "m/leaf/🧪️tests/🦀️.rs"));
      expect(test).toContain("use crate::k::base::*;");
      expect(test).toContain("use crate::m::leaf::*;");
      expect(test).not.toMatch(/\buse super::/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("prunes a group with a self leaf without leaving an empty member", () => {
    expect(prunedUse("use a::b::{self, C, D};", ["b", "C"])).toBe("use a::b::{self, C};");
    expect(prunedUse("use a::b::{self as parent, C};", ["parent", "C"])).toBe("use a::b::{self as parent, C};");
    expect(prunedUse("use a::b::{self, C};", ["b"])).toBe("use a::b;");
  });
  test("a brace group names the chains of its members, not the shared prefix module", () => {
    expect(crateRelativeChains("use crate::a::b::{c::X, d::Y};")).toEqual([["a", "b", "c", "X"], ["a", "b", "d", "Y"]]);
    expect(crateRelativeChains("use crate::a::b::*;")).toEqual([["a", "b"]]);
    expect(crateRelativeChains("let _ = crate::a::b::run();")).toEqual([["a", "b", "run"]]);
  });
  test("aggregates sibling leaf tests into the file that holds the inline parent module, not into the lib root", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-aggregate-file-"));
    try {
      const leaf = (name: string): Record<string, string> => ({
        [`tree/${name}/🦀️.rs`]: `pub fn ${name}() {}\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n`,
        [`tree/${name}/🧪️tests/🦀️.rs`]: `use super::*;\n#[test]\nfn runs() { ${name}(); }\n`,
      });
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "tree/🦀️.rs"]\npub mod s;\n',
        "tree/🦀️.rs": '#[path = "."]\npub mod modules {\n    #[path = "a/🦀️.rs"]\n    pub mod a;\n    #[path = "b/🦀️.rs"]\n    pub mod b;\n    #[path = "c/🦀️.rs"]\n    pub mod c;\n}\n',
        ...leaf("a"), ...leaf("b"), ...leaf("c"),
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: true, hoist: false });
      expect(result.rootSource).toBe(layout["🦀️.rs"]);
      const tree = result.edits.get(join(root, "tree/🦀️.rs"))!;
      expect(tree).toContain("mod leaf_tests;");
      expect(tree.indexOf("mod leaf_tests;")).toBeLessThan(tree.lastIndexOf("}"));
      expect(result.notes.some((note) => note.includes("aggregated 3 leaf test files"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("rebases paths joined onto the cargo manifest directory when a file moves to another crate directory", () => {
    const source = 'let a = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../x/y.json"));\nlet b = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../z");\nlet c = env!("CARGO_MANIFEST_DIR");';
    const moved = rebaseManifestDirPaths(source, "/repo/art/📦️packages/🦀️rust", "/repo/art/sub/deeper/📦️packages/🦀️rust");
    expect(moved).toContain('"/../../../../../x/y.json"');
    expect(moved).toContain('.join("../../../../📦️packages/z")');
    expect(moved).toContain('let c = env!("CARGO_MANIFEST_DIR");');
    expect(rebaseManifestDirPaths(moved, "/repo/art/sub/deeper/📦️packages/🦀️rust", "/repo/art/📦️packages/🦀️rust")).toBe(source);
    expect(rebaseManifestDirPaths(source, "/a", "/a")).toBe(source);
  });
}

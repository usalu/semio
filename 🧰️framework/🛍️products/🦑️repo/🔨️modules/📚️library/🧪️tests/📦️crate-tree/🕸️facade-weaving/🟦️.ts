import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { applyCarveWrites, carveWrites, planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { crateIdent as crateIdentOf } from "../../../🏗️authoring/📦️crate-tree/🗺️plan/🏷️naming/🟦️.ts";
import { compareModuleTrees, resolveCrateModules } from "../../../🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
import { scaffold, tiny, weaveFiles, writeTree } from "../🏗️virtual-crate/🟦️.ts";

const weaveTree = (deep: boolean): string => {
  const root = mkdtempSync(join(tmpdir(), "semio-weave-"));
  for (const [path, content] of Object.entries(weaveFiles(deep))) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
  return root;
};

describe("artifact crate carve", () => {
  test("weaves a mounted child into the facade instead of making its parent depend on it", async () => {
    const root = weaveTree(false);
    try {
      const merged = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, weave: false });
      const holder = (plan: typeof merged, suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix));
      expect(holder(merged, "io")).toBeDefined();
      expect(holder(merged, "io::text")).toBeUndefined();
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const io = holder(plan, "io")!;
      const text = holder(plan, "io::text")!;
      expect(text).toBeDefined();
      expect(text).not.toBe(io);
      expect(io.dependsOn).toEqual([]);
      expect(text.dependsOn).toContain(io.id);
      expect(plan.weave.map((entry) => entry.eff.join("::"))).toEqual(["io::text"]);
      const { writes } = await carveWrites(plan);
      const facade = writes.find((write) => write.path === "🦀️.rs")!.content;
      expect(facade).toContain("pub mod io {");
      expect(facade).toMatch(new RegExp(`pub use ${crateIdentOf(io.name)}::io::\\*;`, "u"));
      expect(facade).toContain(`pub use ${crateIdentOf(text.name)}::io::text;`);
      const ioFile = writes.find((write) => write.path === "🚪️io/🦀️.rs")!.content;
      expect(ioFile).not.toContain("pub mod text;");
      expect(ioFile).not.toContain("pub use");
      const original = resolveCrateModules(root, join(root, "🦀️.rs"));
      applyCarveWrites(root, writes);
      const carved = [resolveCrateModules(root, join(root, "🦀️.rs")), ...plan.crates.map((crate) => resolveCrateModules(root, join(root, crate.packageDir, "🦀️.rs")))];
      expect(compareModuleTrees(original, carved).problems).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps a mounted child in the facade when a kept root file module names it", async () => {
    const root = weaveTree(false);
    try {
      const rootFile = join(root, "🦀️.rs");
      writeFileSync(rootFile, readFileSync(rootFile, "utf8") + '#[path = "🧾️declaration/🦀️.rs"]\nmod declaration;\n');
      mkdirSync(join(root, "🧾️declaration"), { recursive: true });
      writeFileSync(join(root, "🧾️declaration/🦀️.rs"), "//! declaration\npub fn assembled() -> u8 { crate::io::text::helper_text() }\n");
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems.filter((problem) => !problem.includes("mounts no file"))).toEqual([]);
      expect(plan.weave.map((entry) => entry.eff.join("::"))).not.toContain("io::text");
      const { writes } = await carveWrites(plan);
      expect(writes.find((write) => write.path === "🦀️.rs")!.content).toContain("pub use demo_weave_io::io;");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps a grandchild mounted when its ancestor reaches it through a relative path", async () => {
    const root = weaveTree(true);
    try {
      writeFileSync(join(root, "🚪️io/🦀️.rs"), "//! io\npub fn helper() -> u8 { 1 }\npub fn via() -> u8 { text::deep::run() }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n");
      writeFileSync(join(root, "🚪️io/📝️text/🦀️.rs"), "//! text\npub fn helper_text() -> u8 { 3 }\n#[path = \"🧱️deep/🦀️.rs\"]\npub mod deep;\n");
      writeFileSync(join(root, "🚪️io/📝️text/🧱️deep/🦀️.rs"), "//! deep\npub fn run() -> u8 { 4 }\n");
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems.filter((problem) => !problem.includes("mounts no file"))).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const io = holder("io");
      const deep = holder("io::text::deep");
      expect(io.dependsOn).toContain(deep.id);
      expect(plan.weave.map((entry) => entry.eff.join("::"))).not.toContain("io::text::deep");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("weaves a grandchild below a woven child with a local module that glob-exports its owner", async () => {
    const root = weaveTree(true);
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(new Set([holder("io"), holder("io::text"), holder("io::text::deep")]).size).toBe(3);
      expect(plan.weave.map((entry) => entry.eff.join("::")).sort()).toEqual(["io::text", "io::text::deep"]);
      const { writes } = await carveWrites(plan);
      const facade = writes.find((write) => write.path === "🦀️.rs")!.content;
      expect(facade).toContain("    pub mod text {");
      expect(facade).toContain(`pub use ${crateIdentOf(holder("io::text").name)}::io::text::*;`);
      expect(facade).toContain(`pub use ${crateIdentOf(holder("io::text::deep").name)}::io::text::deep;`);
      expect(facade.indexOf("pub mod text {")).toBeLessThan(facade.indexOf("io::text::deep;"));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps the mount re-export when the parent module uses its child", async () => {
    const root = weaveTree(false);
    try {
      writeFileSync(join(root, "🚪️io/🦀️.rs"), "//! io\npub fn helper() -> u8 { 1 }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\npub fn twice() -> u8 { text::helper_text() * 2 }\n");
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.weave).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps the mount of a module that an item of the artifact root names, because the crate hosting that item cannot see a woven path", async () => {
    const root = weaveTree(false);
    try {
      const rootFile = join(root, "🦀️.rs");
      writeFileSync(rootFile, readFileSync(rootFile, "utf8").replace('pub const DOCUMENT: &str = "d";', 'pub const DOCUMENT: &str = "d";\npub fn go() -> u8 { crate::io::text::helper_text() }'));
      const before = await planArtifactCrates(weaveTree(false), "📦️packages/🦀️rust", { ...tiny });
      expect(before.weave.map((entry) => entry.eff.join("::"))).toEqual(["io::text"]);
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.weave).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("gates the re-exports of a moved root item that exists only under cfg(test), in the crates and in the facade", async () => {
    const root = writeTree("semio-gated-hook-", {
      ...scaffold("demo-gated-hook", '//! root\n#[cfg(test)]\npub fn hook() -> u8 { 3 }\npub fn go() -> u8 { 4 }\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n#[path = "."]\npub mod schema {\n    #[path = "🧬️schema/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": "//! io\npub fn run() -> u8 { crate::schema::value() + crate::go() }\n#[cfg(test)]\nmod checks {\n    #[test]\n    fn uses_the_hook() { assert_eq!(crate::hook(), 3); }\n}\n",
      "🧬️schema/🦀️.rs": "//! schema\npub fn value() -> u8 { 2 }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, weave: false, tests: { fixtures: false, aggregate: false, hoist: false, detach: false } });
      const lines = plan.crates.flatMap((crate) => crate.itemImportLines).filter((line) => /\bhook\b/u.test(line));
      expect(lines.length).toBeGreaterThan(0);
      for (const line of lines) expect(line.startsWith("#[cfg(test)]\n")).toBe(true);
      expect(plan.crates.flatMap((crate) => crate.itemImportLines).some((line) => /\bgo\b/u.test(line) && !line.startsWith("#[cfg(test)]"))).toBe(true);
      const { writes } = await carveWrites(plan);
      const facade = writes.find((write) => write.path === "🦀️.rs")!.content;
      expect(facade).toMatch(/#\[cfg\(test\)\]\npub use [a-z_]+::\{hook\};/u);
      expect(facade).toMatch(/\npub use [a-z_]+::\{go\};/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("re-exports a moved root item that exists under a test-support gate with the same gate in the crates and with cfg(test) in the facade", async () => {
    const root = writeTree("semio-gated-testkit-", {
      ...scaffold("demo-gated-kit", '//! root\n#[cfg(any(test, feature = "testkit"))]\nuse std::collections::BTreeMap as Kit;\n#[cfg(any(test, feature = "testkit"))]\npub fn hook() -> usize { Kit::<u8, u8>::new().len() + 3 }\npub fn go() -> u8 { 4 }\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n#[path = "."]\npub mod schema {\n    #[path = "🧬️schema/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": "//! io\npub fn run() -> u8 { crate::schema::value() + crate::go() }\n#[cfg(test)]\nmod checks {\n    #[test]\n    fn uses_the_hook() { assert_eq!(crate::hook(), 3); }\n}\n",
      "🧬️schema/🦀️.rs": "//! schema\npub fn value() -> u8 { 2 }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, weave: false, tests: { fixtures: false, aggregate: false, hoist: false, detach: false } });
      const lines = plan.crates.flatMap((crate) => crate.itemImportLines).filter((line) => /\bhook\b/u.test(line));
      expect(lines.length).toBeGreaterThan(0);
      for (const line of lines) expect(line.startsWith('#[cfg(any(test, feature = "testkit"))]\n')).toBe(true);
      expect(plan.movedAppends.map((append) => append.text).join("")).toContain('#[cfg(any(test, feature = "testkit"))]\nuse std::collections::BTreeMap as Kit;');
      const { writes } = await carveWrites(plan);
      expect(writes.find((write) => write.path === "🦀️.rs")!.content).toMatch(/#\[cfg\(test\)\]\npub use [a-z_]+::\{hook\};/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("a sibling that reaches a module through the mount of its parent depends on the module's owner, not on the parent", async () => {
    const root = writeTree("semio-parent-path-", {
      ...scaffold("demo-parent-path", '//! root\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": '//! io\n#[path = "📝️text/🦀️.rs"]\npub mod text;\n',
      "🚪️io/📝️text/🦀️.rs": '//! text\n#[path = "🅰️a/🦀️.rs"]\npub mod a;\n#[path = "🅱️b/🦀️.rs"]\npub mod b;\n#[path = "🅲️c/🦀️.rs"]\npub mod c;\n',
      "🚪️io/📝️text/🅰️a/🦀️.rs": "//! a\npub fn value() -> u8 { 1 }\n",
      "🚪️io/📝️text/🅱️b/🦀️.rs": "//! b\nuse crate::io::text::a;\npub fn run() -> u8 { a::value() }\n",
      "🚪️io/📝️text/🅲️c/🦀️.rs": "//! c\nuse crate::io::text::b;\npub fn again() -> u8 { b::run() }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const [text, a, b, c] = [holder("io::text"), holder("io::text::a"), holder("io::text::b"), holder("io::text::c")];
      expect(new Set([text, a, b, c]).size).toBe(4);
      expect(b.dependsOn).toContain(a.id);
      expect(c.dependsOn).toContain(b.id);
      expect(b.dependsOn).not.toContain(text.id);
      expect(c.dependsOn).not.toContain(text.id);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps a slice in the crate of the cfg(test) module it reaches, because a dependency never compiles that module", async () => {
    const root = writeTree("semio-cfg-test-module-", {
      ...scaffold("demo-cfg-test-module", '//! root\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": '//! io\n#[path = "📝️text/🦀️.rs"]\npub mod text;\n',
      "🚪️io/📝️text/🦀️.rs": '//! text\n#[path = "🅰️a/🦀️.rs"]\npub mod a;\n#[path = "🅱️b/🦀️.rs"]\npub mod b;\n',
      "🚪️io/📝️text/🅰️a/🦀️.rs": '//! a\npub fn value() -> u8 { 1 }\n#[cfg(test)]\n#[path = "🧰️support/🦀️.rs"]\npub mod support;\n',
      "🚪️io/📝️text/🅰️a/🧰️support/🦀️.rs": "//! support\npub fn helper() -> u8 { 2 }\n",
      "🚪️io/📝️text/🅱️b/🦀️.rs": "//! b\npub fn run() -> u8 { 3 }\n#[cfg(test)]\nmod checks {\n    #[test]\n    fn t() { assert_eq!(crate::io::text::a::support::helper(), 2); }\n}\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, tests: { fixtures: false, aggregate: false, hoist: false, detach: false } });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(holder("io::text::a::support").id).toBe(holder("io::text::b").id);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("does not make a slice depend on the owner of a prefix when the path names an inline wrapper module that the owner does not own", async () => {
    const root = writeTree("semio-wrapper-prefix-", {
      ...scaffold("demo-wrapper-prefix", '//! root\n#[path = "."]\npub mod top {\n    #[path = "🔝top/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "."]\n    pub mod mid {\n        #[path = "🅰️a/🦀️.rs"]\n        pub mod a;\n        #[path = "🅱️b/🦀️.rs"]\n        pub mod b;\n    }\n}\n'),
      "🔝top/🦀️.rs": "//! top\npub fn here() -> u8 { 0 }\n",
      "🅰️a/🦀️.rs": "//! a\npub fn value() -> u8 { 1 }\n",
      "🅱️b/🦀️.rs": "//! b\nuse crate::top::mid;\npub fn run() -> u8 { mid::a::value() }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const [top, a, b] = [holder("top"), holder("top::mid::a"), holder("top::mid::b")];
      expect(new Set([top, a, b]).size).toBe(3);
      expect(b.dependsOn).toContain(a.id);
      expect(b.dependsOn).not.toContain(top.id);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("glob-exports the owner of a module into the wrapper that hosts its detached test in another crate, so the wrapper does not shadow the module", async () => {
    const root = writeTree("semio-detached-test-host-", {
      ...scaffold("demo-detached-test-host", '//! root\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": "//! io\npub fn helper() -> u8 { 1 }\npub fn both() -> u8 { text::helper_text() + helper() }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n",
      "🚪️io/📝️text/🦀️.rs": `//! text\n${"// padding that makes this slice too large to share a crate with the test\n".repeat(30)}pub fn helper_text() -> u8 { 2 }\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n`,
      "🚪️io/📝️text/🧪️tests/🦀️.rs": "use super::*;\nuse crate::io::helper;\n\n#[test]\nfn text_helper_is_two() { assert_eq!(helper_text() + helper(), 3); }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { target: { files: 8, bytes: 1500, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 }, tests: { fixtures: false, aggregate: false, hoist: true, detach: true } });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const [host, text] = [holder("io::text::tests"), holder("io::text")];
      expect(host.id).not.toBe(text.id);
      const { writes } = await carveWrites(plan);
      const mirror = writes.find((write) => write.path === `${host.packageDir}/🦀️.rs`)!.content;
      expect(mirror).toContain(`pub use ${crateIdentOf(text.name)}::io::text::*;`);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);
});

/** 🔸️ The leaves fixtures feed literal tests of the artifact crate carve suite. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { aliasChains } from "../../../../🏗️authoring/📦️crate-tree/🗺️plan/🧭️module-chains/🟦️.ts";
import { external } from "../../../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { fixturesToRuntime, transformTestClosure } from "../../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

/** 🔹 Registers the leaves fixtures feed literal tests of the artifact crate carve suite. */
export function registerLeavesFixturesFeedLiteralTests(): void {
  test("leaves fixtures that feed literal-only macros embedded and keeps test modules in place that sibling tests name through super", () => {
    const concatenated = 'let script = concat!(include_str!("../../🧪️tests/🧫️fixtures/a.sql"), "\\nselect 1;");\nlet plain = include_str!("../../🧪️tests/🧫️fixtures/b.json");\nfn f() {}';
    const rewritten = fixturesToRuntime(`fn t() { ${concatenated} }`, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain('concat!(include_str!("../../🧪️tests/🧫️fixtures/a.sql")');
    expect(rewritten).toContain('fixture_text!("@fixtures/');
    const constConcat = 'const A: &str = include_str!("../../🧪️tests/🧫️fixtures/a.sql");\nfn t() { let _ = concat!(A, "x"); }';
    expect(fixturesToRuntime(constConcat, "/repo/x/y/z/test.rs", "/repo")).toBeNull();
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-ref-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n#[cfg(test)]\n#[path = "🫴️receiving/🧪️tests/🦀️.rs"]\nmod receiving;\n',
        "a/🧪️tests/🦀️.rs": 'pub(super) fn fixture() -> &\'static str { crate::DOCUMENT }\n',
        "a/🫴️receiving/🧪️tests/🦀️.rs": 'fn source() -> &\'static str { super::tests::fixture() }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).not.toContain("mod hoisted_");
      expect(result.notes.some((note) => note.includes("super::tests"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("carries the private absolute imports of the host into a hoisted test that glob-imports the host", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-glob-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Private;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'use crate::a::*;\n#[test]\nfn t() { let _: Private<u8, u8> = Private::new(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).toContain("mod hoisted_");
      expect(result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))).toContain("use std::collections::BTreeMap as Private;");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("retargets the absolute self path of a hoisted test to its new mount", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-self-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'pub fn helper() {}\nmod inner {\n    use crate::a::tests::*;\n    #[test]\n    fn t() { helper(); let _ = crate::DOCUMENT; }\n}\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      const name = /mod (hoisted_[A-Za-z0-9_]+);/u.exec(result.rootSource)![1]!;
      const moved = result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))!;
      expect(moved).toContain(`use crate::${name}::*;`);
      expect(moved).not.toContain("crate::a::tests");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("sees the aliases that a use statement inside a function body binds", () => {
    const chains = aliasChains("pub fn io() {\n    use crate::standards::io::export::serializers::artifacts as export;\n    let _ = export::json::Codec;\n}\n");
    expect(chains).toContainEqual(["standards", "io", "export", "serializers", "artifacts", "json", "Codec"]);
  });
  test("carries the private imports of the host into a nested module of a detached test that globs the host", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-detach-nested-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Private;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'pub mod context {\n    use super::super::*;\n    pub fn make() -> Private<u8, u8> { Private::new() }\n}\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: false, detach: true });
      const moved = result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))!;
      expect(moved).toMatch(/use crate::a::\*;\n\s*use std::collections::BTreeMap as Private;/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("names a private import of the host by its carried import in a hoisted test instead of through the host path", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-private-import-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Private;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'use crate::a::*;\n#[test]\nfn t() { let _ = crate::a::Private::<u8, u8>::new(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      const moved = result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))!;
      expect(moved).toContain("use std::collections::BTreeMap as Private;");
      expect(moved).toContain("let _ = Private::<u8, u8>::new();");
      expect(moved).not.toContain("crate::a::Private");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("widens the private host items a hoisted test uses and carries private absolute imports into hoisted tests", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-private-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Map;\nconst SECRET: u8 = 1;\n#[cfg(test)]\n#[path = "🧪️tests/🔬️secret/🦀️.rs"]\nmod secret;\n#[cfg(test)]\n#[path = "🧪️tests/🔬️map/🦀️.rs"]\nmod map;\n',
        "a/🧪️tests/🔬️secret/🦀️.rs": '#[test]\nfn t() { let _ = crate::DOCUMENT; let _ = SECRET; }\n',
        "a/🧪️tests/🔬️map/🦀️.rs": '#[test]\nfn t() { let _: Map<u8, u8> = Map::new(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).toContain("🔬️map/🦀️.rs");
      expect(result.rootSource).toContain("🔬️secret/🦀️.rs");
      expect(result.edits.get(join(root, "a/🧪️tests/🔬️map/🦀️.rs"))).toContain("use std::collections::BTreeMap as Map;");
      expect(result.edits.get(join(root, "a/🧪️tests/🔬️secret/🦀️.rs"))).toContain("use crate::a::{SECRET};");
      expect(result.edits.get(join(root, "a/🦀️.rs"))).toContain("pub const SECRET: u8 = 1;");
      expect(result.notes.some((note) => note.includes("widened SECRET of the host to pub"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("widens a private host item once when several hoisted tests use it", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-twice-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'fn secret() {}\n#[cfg(test)]\n#[path = "🧪️tests/🔬️one/🦀️.rs"]\nmod one;\n#[cfg(test)]\n#[path = "🧪️tests/🔬️two/🦀️.rs"]\nmod two;\n',
        "a/🧪️tests/🔬️one/🦀️.rs": '#[test]\nfn t() { secret(); let _ = crate::DOCUMENT; }\n',
        "a/🧪️tests/🔬️two/🦀️.rs": '#[test]\nfn t() { secret(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      const host = result.edits.get(join(root, "a/🦀️.rs"))!;
      expect(host).toContain("pub fn secret() {}");
      expect(host).not.toContain("pub pub");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("treats the import of an unlinked first-party crate as external noise and keeps every other unresolved import", () => {
    expect(external("E0432\tunresolved import `semio_framework_artifact_playbook_playbook`")).toBe(false);
    expect(external("E0432\tunresolved import `semio_framework_artifact_playbook_playbook`\tsome/file.rs\tno `x` in the root")).toBe(false);
    expect(external("E0432\tunresolved import `crate::editor::forms::support`")).toBe(true);
    expect(external("E0220\tassociated type `Error` not found for `V`")).toBe(false);
    expect(external("E0220\tassociated type `Error` not found for `Visitor`\tx.rs\t")).toBe(false);
    expect(external("E0220\tassociated type `Error` not found for `crate::Thing`")).toBe(true);
    expect(external("E0432\tunresolved import `helper`")).toBe(true);
  });
}

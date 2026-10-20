/** 🟠️ The inlines root file modules tests of the artifact crate carve suite. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { inlineRootFileModules } from "../../../../🏗️authoring/📦️crate-tree/🫗️inline-root/🟦️.ts";
import { absolutizeBeyond } from "../../../../🏗️authoring/📦️crate-tree/🪜️lift-kids/🟦️.ts";

/** 🟡 Registers the inlines root file modules tests of the artifact crate carve suite. */
export function registerInlinesRootFileModulesTests(): void {
  test("inlines root file modules that only glob-re-export plain items and leaves every other file module mounted", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-inline-root-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub struct Keep;\n#[path = "📦️records/🦀️.rs"]\nmod records;\npub use records::*;\n#[path = "🧷️nested/🦀️.rs"]\nmod nested;\npub use nested::*;\n',
        "📦️records/🦀️.rs": '//! 📦️ Records.\n\nuse super::*;\n\npub struct Record(pub Keep);\n',
        "🧷️nested/🦀️.rs": 'use super::*;\n#[path = "x/🦀️.rs"]\nmod x;\npub fn f() {}\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = inlineRootFileModules(null, join(root, "🦀️.rs"), layout["🦀️.rs"]!);
      expect(result.rootSource).toContain("//#region 🧩️records");
      expect(result.rootSource).toContain("pub struct Record(pub Keep);");
      expect(result.rootSource).not.toContain("pub use records::*;");
      expect(result.rootSource).not.toContain("use super::*;");
      expect(result.rootSource).toContain("mod nested;");
      const own = inlineRootFileModules(null, join(root, "🦀️.rs"), 'pub struct Keep;\n#[path = "📦️records/🦀️.rs"]\nmod records;\npub use records::*;\n');
      expect(own.rootSource).toContain("//#region 🧩️records");
      writeFileSync(join(root, "📦️records/🦀️.rs"), "use super::*;\n//#region Own\npub struct Record(pub Keep);\n//#endregion Own\n");
      const kept = inlineRootFileModules(null, join(root, "🦀️.rs"), 'pub struct Keep;\n#[path = "📦️records/🦀️.rs"]\nmod records;\npub use records::*;\n');
      expect(kept.rootSource).toContain("//#region Own");
      expect(kept.rootSource).not.toContain("🧩️records");
      expect([...result.deletes]).toEqual([join(root, "📦️records/🦀️.rs")]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("keeps the private component segment when a leaf glob is made absolute for a mutation family", () => {
    const raw = ["editor", "config", "component", "mutations", "set"];
    expect(absolutizeBeyond("use super::*;\n", raw, 0, false)).toBe("use crate::editor::config::component::mutations::*;\n");
    expect(absolutizeBeyond("use super::*;\n", raw, 0)).toBe("use crate::editor::config::mutations::*;\n");
  });
}

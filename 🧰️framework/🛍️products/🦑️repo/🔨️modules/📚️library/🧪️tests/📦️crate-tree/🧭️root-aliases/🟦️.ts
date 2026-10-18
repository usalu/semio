import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { carveWrites, planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { parseRustSourceTree } from "../../../🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { qualifyModuleHeads, rootAliasModules, superChains } from "../../../🏗️authoring/📦️crate-tree/🗺️plan/🧭️module-chains/🟦️.ts";
import { scaffold, tiny, weaveFiles, writeTree } from "../🏗️virtual-crate/🟦️.ts";

describe("artifact crate carve", () => {
  test("resolves a type name that two standards define to the definition in the closest module", async () => {
    const standard = (name: string): Record<string, string> => ({
      [`🅰️${name}/🧬️schema/🦀️.rs`]: "//! schema\npub struct Doc;\n",
      [`🅰️${name}/🚪️io/🦀️.rs`]: `//! io\nuse crate::${name}::schema::Doc;\nimpl std::fmt::Display for Doc {\n    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, "${name}") }\n}\n`,
    });
    const mounts = (name: string): string => `#[path = "."]\npub mod ${name} {\n    #[path = "🅰️${name}/🧬️schema/🦀️.rs"]\n    pub mod schema;\n    #[path = "🅰️${name}/🚪️io/🦀️.rs"]\n    pub mod io;\n}\n`;
    const root = writeTree("semio-closest-type-", { ...scaffold("demo-closest-type", `//! root\n${mounts("v1")}${mounts("v2")}`), ...standard("v1"), ...standard("v2") });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems.filter((problem) => problem.includes("ambiguous coherence"))).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(holder("v1::io")).toBe(holder("v1::schema"));
      expect(holder("v2::io")).toBe(holder("v2::schema"));
      expect(holder("v1::io")).not.toBe(holder("v2::io"));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("recognizes root alias modules and resolves the super paths written in inline module bodies", () => {
    const source = [
      "pub mod schema {\n    pub use super::standards::v1::any::schema::*;\n}",
      "pub mod diff {\n    pub use crate::standards::v1::any::schema::diff::*;\n    pub use crate::host::inferences::*;\n}",
      "pub mod busy {\n    pub use super::standards::v1::x::*;\n    pub fn f() {}\n}",
      "pub mod grouped {\n    pub use super::standards::{a, b};\n}",
      "pub mod engine {\n    pub use super::subsets::any::io::register;\n}",
    ].join("\n");
    const aliases = rootAliasModules(source, parseRustSourceTree(source).modules);
    expect([...aliases.keys()].sort()).toEqual(["diff", "engine", "schema"]);
    expect(aliases.get("schema")!.targets).toEqual([["standards", "v1", "any", "schema"]]);
    expect(aliases.get("diff")!.targets).toEqual([["standards", "v1", "any", "schema", "diff"], ["host", "inferences"]]);
    expect(aliases.get("engine")!.text).toContain("pub use super::subsets::any::io::register;");
    const inline = "pub mod engine {\n    pub use super::subsets::any::io::register;\n    pub use super::super::top::{x, y::z};\n    pub mod inner { pub use super::kept::A; }\n}";
    const module = parseRustSourceTree(inline).modules[0]!;
    const nested = module.children.map((child): [number, number] => [child.start, child.end]);
    expect(superChains(inline, module.bodyStart, module.bodyEnd, nested, ["standards", "v_raw", "engine"])).toEqual([["standards", "v_raw", "subsets", "any", "io", "register"], ["standards", "top", "x"], ["standards", "top", "y", "z"]]);
  });

  test("copies a root alias module into every crate that goes through it and depends on the crate that owns the target", async () => {
    const tree = mkdtempSync(join(tmpdir(), "semio-shim-"));
    try {
      for (const [path, content] of Object.entries(weaveFiles(false))) { mkdirSync(dirname(join(tree, path)), { recursive: true }); writeFileSync(join(tree, path), content); }
      writeFileSync(join(tree, "🦀️.rs"), readFileSync(join(tree, "🦀️.rs"), "utf8") + "pub mod views {\n    pub use super::schema::*;\n}\n");
      writeFileSync(join(tree, "🚪️io/📝️text/🦀️.rs"), "//! text\npub fn helper_text() -> u8 { crate::io::helper() + crate::views::value() }\n");
      const plan = await planArtifactCrates(tree, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(holder("io::text").dependsOn).toContain(holder("schema").id);
      expect(holder("io::text").itemImportLines.some((line) => line.includes("pub mod views {"))).toBe(true);
      expect(holder("schema").itemImportLines.some((line) => line.includes("pub mod views {"))).toBe(false);
      const { writes } = await carveWrites(plan);
      expect(writes.find((write) => write.path === "🦀️.rs")!.content).toContain("pub mod views {");
    } finally {
      rmSync(tree, { recursive: true, force: true });
    }
  });

  test("qualifies root module heads of moved items and leaves every other path alone", () => {
    const text = "use schema::snapshot::Snap;\npub fn f() { let _ = standards::v1::run(); let _ = crate::schema::x(); let _ = other::y(); let _ = self::schema::z(); let _ = value.schema::<u8>(); }\n";
    const out = qualifyModuleHeads(text, new Set(["schema", "standards"]));
    expect(out).toContain("use crate::schema::snapshot::Snap;");
    expect(out).toContain("crate::standards::v1::run()");
    expect(out).toContain("crate::schema::x()");
    expect(out).not.toContain("crate::crate::");
    expect(out).toContain("other::y()");
    expect(out).toContain("self::schema::z()");
    expect(qualifyModuleHeads(text, new Set(["schema", "standards"]), new Set(["standards"]))).toContain("let _ = standards::v1::run();");
  });
});

import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { carveWrites, planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { MeasureUnion, totalsFit } from "../../../🏗️authoring/📦️crate-tree/📏️measure-union/🟦️.ts";
import { qualifyModuleHeads } from "../../../🏗️authoring/📦️crate-tree/🗺️plan/🧭️module-chains/🟦️.ts";
import { external, normalizeCombined, rewriteForCombined } from "../../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { dropUnlinkedCascades, plainModulePath, unlinkedGlobModules } from "../../../🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
import { files, fixture, tiny } from "../🏗️virtual-crate/🟦️.ts";

describe("artifact crate carve", () => {
  test("keeps the largest size per path in a measure union and never counts the virtual root", () => {
    const union = new MeasureUnion(new Set(["mirror/🦀️.rs"]), { files: 1, bytes: 0, lines: 0 });
    const left = new Map([["a.rs", { bytes: 10, lines: 2 }], ["b.rs", { bytes: 5, lines: 1 }], ["mirror/🦀️.rs", { bytes: 99, lines: 9 }]]);
    const right = new Map([["a.rs", { bytes: 12, lines: 1 }], ["c.rs", { bytes: 7, lines: 3 }], ["mirror/🦀️.rs", { bytes: 77, lines: 7 }]]);
    union.add([left]);
    expect(union.totals()).toEqual({ files: 3, bytes: 15, lines: 3 });
    expect(union.peek([right, right])).toEqual({ files: 4, bytes: 24, lines: 6 });
    expect(union.totals()).toEqual({ files: 3, bytes: 15, lines: 3 });
    union.add([right]);
    expect(union.totals()).toEqual({ files: 4, bytes: 24, lines: 6 });
    expect(totalsFit(union.totals(), { files: 4, bytes: 24, lines: 6 })).toBe(true);
    expect(totalsFit(union.totals(), { files: 3, bytes: 24, lines: 6 })).toBe(false);
  });

  test("packs without re-reading merged closures that cannot fit, and plans exactly what a full re-measure plans", async () => {
    const names = ["a", "b", "c", "d", "e", "f"];
    const tree: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-pack"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-pack"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-pack",{snapshotSqliteTests:[]});\n',
      "🦀️.rs": `//! root\n${names.map((name) => `#[path = "."]\npub mod ${name} {\n    #[path = "${name}/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n`).join("")}`,
      ...Object.fromEntries(names.map((name, index) => [`${name}/🦀️.rs`, `//! ${name}\npub fn ${name}_value() -> u8 { ${index} }\n`])),
    };
    const root = mkdtempSync(join(tmpdir(), "semio-pack-"));
    try {
      for (const [path, content] of Object.entries(tree)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const options = { target: { files: 3, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } };
      const incremental = await planArtifactCrates(root, "📦️packages/🦀️rust", options);
      const exact = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...options, incrementalMeasure: false });
      const shape = (plan: typeof incremental): string[] => plan.crates.map((crate) => crate.attach.map((decl) => decl.eff.join("::")).sort().join("+")).sort();
      expect(shape(incremental)).toEqual(shape(exact));
      expect(shape(incremental).length).toBe(3);
      expect(incremental.packing.merged).toBe(exact.packing.merged);
      expect(incremental.packing.merged).toBeGreaterThan(0);
      expect(incremental.packing.pruned).toBeGreaterThan(0);
      expect(incremental.packing.measured).toBeLessThan(exact.packing.measured);
      expect(exact.packing.pruned).toBe(0);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rewrites carved crates into modules of one synthetic crate without touching macro paths", () => {
    const source = "use crate::a::b;\npub use dep_crate::x::y;\nmacro_rules! m { () => { $crate::keep::it() }; }\nfn f() { crate::z(); let _ = std::mem::drop(1); pub(crate) fn g() {} }\n";
    const rewritten = rewriteForCombined(source, "__c_me", new Map([["dep_crate", "__c_dep"]]));
    expect(rewritten).toContain("use crate::__c_me::a::b;");
    expect(rewritten).toContain("pub use crate::__c_dep::x::y;");
    expect(rewritten).toContain("$crate::keep::it()");
    expect(rewritten).toContain("crate::__c_me::z();");
    expect(rewritten).toContain("pub(crate) fn g()");
    expect(rewritten).toContain("std::mem::drop");
  });

  test("hosts moved root regions without importing one name twice or qualifying external use groups", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-units-"));
    const layout: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-units"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-units"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-units",{snapshotSqliteTests:[]});\n',
      "🦀️.rs": [
        "//! root",
        "pub use crate::base::one;",
        "//#region Bridge",
        "pub use ext::Alpha;",
        "use ext::Gamma;",
        "use ext::schema::snapshot::{Edge, Node};",
        "pub struct Holder { pub alpha: Alpha, pub gamma: Gamma, pub edge: Edge, pub node: Node }",
        "//#endregion Bridge",
        "//#region Maker",
        "pub fn make() -> Holder { let _ = one(); Holder { alpha: Alpha, gamma: Gamma, edge: Edge, node: Node } }",
        "//#endregion Maker",
        '#[path = "."]',
        "pub mod base {",
        '    #[path = "🧱️base/🦀️.rs"]',
        "    mod component;",
        "    pub use component::*;",
        '    #[path = "🧱️base/🍃️leaf/🦀️.rs"]',
        "    pub mod leaf;",
        "}",
        '#[path = "."]',
        "pub mod schema {",
        '    #[path = "🧬️schema/🦀️.rs"]',
        "    mod component;",
        "    pub use component::*;",
        "}",
        '#[path = "."]',
        "pub mod io {",
        '    #[path = "🚪️io/🦀️.rs"]',
        "    mod component;",
        "    pub use component::*;",
        "}",
        "",
      ].join("\n"),
      "🧬️schema/🦀️.rs": "//! schema\npub fn hold() -> crate::Holder { crate::make() }\n",
      "🚪️io/🦀️.rs": "//! io\npub fn build() -> crate::Holder { crate::make() }\n",
      "🧱️base/🦀️.rs": "//! base\npub fn one() -> u8 { 1 }\n",
      "🧱️base/🍃️leaf/🦀️.rs": "//! leaf\npub fn two() -> u8 { 2 }\n",
    };
    try {
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      const { writes } = await carveWrites(plan);
      expect(plan.problems).toEqual([]);
      const units = writes.flatMap((write) => [...write.content.matchAll(/pub mod (artifact_items\w*) \{([\s\S]*?)\n\}\n\/\/#endregion/gu)].map((match) => match[2]!));
      expect(units.length).toBeGreaterThan(0);
      for (const unit of units) {
        expect(unit.match(/use ext::Gamma;/gu)?.length ?? 0).toBeLessThanOrEqual(1);
        expect(unit).not.toContain("{crate::");
        expect(unit).not.toMatch(/use ext::\{[^}]*crate::/u);
      }
      const maker = units.find((unit) => unit.includes("pub fn make"))!;
      expect(maker).not.toContain("pub use ext::Alpha;");
      expect(maker).toMatch(/use crate::\{[^}]*Alpha[^}]*\};/u);
      expect(maker).toMatch(/use ext::Gamma;/u);
      expect(maker).toMatch(/use crate::base::one;/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("compares an unreadable manifest-relative include by what it names, not by how deep the crate sits", () => {
    const base = "E0000\tcouldn't read `<sandbox>/../../../x/y.json`: No such file or directory (os error 2)";
    const combined = "E0000\tcouldn't read `<sandbox>/../../../../../../x/y.json`: No such file or directory (os error 2)";
    expect(normalizeCombined(combined)).toBe(normalizeCombined(base));
    expect(normalizeCombined(combined)).not.toBe(normalizeCombined(base.replace("y.json", "z.json")));
  });

  test("follows a path through a root use alias to the module it names instead of the alias target", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-alias-"));
    const layout: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-alias"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-alias"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-alias",{snapshotSqliteTests:[]});\n',
      "🦀️.rs": [
        "//! root",
        '#[path = "."]',
        "pub mod standards {",
        '    #[path = "."]',
        "    pub mod v1 {",
        '        #[path = "."]',
        "        pub mod schema {",
        '            #[path = "🧬️schema/🦀️.rs"]',
        "            mod component;",
        "            pub use component::*;",
        '            #[path = "🧬️schema/🔺️diff/🦀️.rs"]',
        "            pub mod diff;",
        "        }",
        '        #[path = "."]',
        "        pub mod leaf {",
        '            #[path = "🍃️leaf/🦀️.rs"]',
        "            mod component;",
        "            pub use component::*;",
        "        }",
        "    }",
        "}",
        "pub use crate::standards::v1::schema;",
        "",
      ].join("\n"),
      "🧬️schema/🦀️.rs": "//! schema\npub fn one() -> u8 { 1 }\n",
      "🧬️schema/🔺️diff/🦀️.rs": "//! diff\npub fn two() -> u8 { 2 }\n",
      "🍃️leaf/🦀️.rs": "//! leaf\npub fn three() -> u8 { crate::schema::diff::two() }\n",
    };
    try {
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::").endsWith(suffix)))!;
      expect(holder("v1::leaf").dependsOn).toContain(holder("schema::diff").id);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("drops only the re-export cascades of an unlinked external crate from the oracle diagnostics", () => {
    const entry = (code: string, message: string, label: string, sourceLine: string) => ({ code, message, label, file: "a.rs", line: 1, sourceLine });
    const kept = dropUnlinkedCascades([
      entry("E0433", "cannot find module or crate `semio_foo` in this scope", "use of unresolved module or unlinked crate `semio_foo`", "pub use semio_foo::schema::mutations::Op;"),
      entry("E0432", "unresolved import `crate::Op`", "no `Op` in `snapshot`", "pub use snapshot::Op;"),
      entry("E0432", "unresolved import `crate::Real`", "no `Real` in `snapshot`", "pub use snapshot::Real;"),
      entry("E0425", "cannot find type `Op` in this scope", "", "fn f() -> Op"),
    ]);
    expect(kept.map((item) => item.message)).toEqual(["cannot find module or crate `semio_foo` in this scope", "unresolved import `crate::Real`", "cannot find type `Op` in this scope"]);
  });

  test("counts a path through the extern prelude of an unlinked crate as external noise and keeps every other missing name", () => {
    expect(external("E0433\tcannot find `semio_framework_schema_registry` in the crate root\tx.rs\t")).toBe(false);
    expect(external("E0433\tcannot find module or crate `semio_foo` in this scope\tx.rs\tuse of unresolved module or unlinked crate `semio_foo`")).toBe(false);
    expect(external("E0433\tcannot find `schema` in `any`\tx.rs\t")).toBe(true);
    expect(external("E0432\tunresolved import `crate::Real`\tx.rs\t")).toBe(true);
  });

  test("drops the unresolved imports that go through a module which glob-re-exports an unlinked external crate", () => {
    const entry = (code: string, message: string, label: string, sourceLine: string, file = "a.rs") => ({ code, message, label, file, line: 1, sourceLine });
    const entries = [
      entry("E0432", "unresolved import `semio_ext`", "use of unresolved module or unlinked crate `semio_ext`", "pub use semio_ext::{self as graph, *};", "engine/🦀️.rs"),
      entry("E0432", "unresolved import `crate::__c_demo::editor::engine::BoardHost`", "no `BoardHost` in `editor::engine`", "use crate::editor::engine::BoardHost;"),
      entry("E0432", "unresolved imports `crate::__c_demo::editor::engine::BoardNaming`, `crate::__c_demo::editor::engine::BoardFillStage`", "", "use crate::editor::engine::{BoardNaming, BoardFillStage};"),
      entry("E0432", "unresolved imports `crate::__c_demo::editor::engine::BoardNaming`, `crate::__c_demo::editor::other::Real`", "", "use crate::editor::engine::BoardNaming;"),
      entry("E0432", "unresolved import `crate::__c_demo::editor::other::Real`", "", "use crate::editor::other::Real;"),
    ];
    expect(dropUnlinkedCascades(entries).length).toBe(4);
    const kept = dropUnlinkedCascades(entries, new Set(["editor::engine"]));
    expect(kept.map((item) => item.message)).toEqual(["unresolved imports `crate::__c_demo::editor::engine::BoardNaming`, `crate::__c_demo::editor::other::Real`", "unresolved import `crate::__c_demo::editor::other::Real`"]);
    expect(plainModulePath(["crate", "__c_demo", "editor", "engine"])).toBe("editor::engine");
    const root = mkdtempSync(join(tmpdir(), "glob-modules-"));
    try {
      mkdirSync(join(root, "editor/engine"), { recursive: true });
      writeFileSync(join(root, "root.rs"), '#[path = "editor/🦀️.rs"]\npub mod editor;\n');
      writeFileSync(join(root, "editor/🦀️.rs"), '#[path = "engine/🦀️.rs"]\npub mod engine;\n');
      writeFileSync(join(root, "editor/engine/🦀️.rs"), "pub use semio_ext::{self as graph, *};\n");
      const found = unlinkedGlobModules(root, join(root, "root.rs"), [entry("E0432", "unresolved import `semio_ext`", "use of unresolved module or unlinked crate `semio_ext`", "pub use semio_ext::{self as graph, *};", "editor/engine/🦀️.rs")]);
      expect([...found]).toEqual(["editor::engine"]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("copies a root alias of an external crate into every crate that reaches through it with a longer path", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-alias-"));
    const layout: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-alias"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-alias"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-alias",{snapshotSqliteTests:[]});\n',
      "🦀️.rs": ["//! root", "use external_kernel as kernel;", '#[path = "."]', "pub mod schema {", '    #[path = "🧬️schema/🦀️.rs"]', "    mod component;", "    pub use component::*;", "}", '#[path = "."]', "pub mod io {", '    #[path = "🚪️io/🦀️.rs"]', "    mod component;", "    pub use component::*;", "}", ""].join("\n"),
      "🧬️schema/🦀️.rs": "//! schema\npub fn value() -> u8 { crate::kernel::default_value() }\n",
      "🚪️io/🦀️.rs": "//! io\npub fn other() -> u8 { crate::kernel::other_value() }\n",
    };
    try {
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      expect(plan.crates.length).toBe(2);
      const { writes } = await carveWrites(plan);
      for (const crate of plan.crates) expect(writes.find((write) => write.path === `${crate.packageDir.replace("/📦️packages/🦀️rust", "")}/📦️packages/🦀️rust/🦀️.rs`)!.content).toContain("use external_kernel as kernel;");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("measures the rewritten facade against the manifest directory of the artifact package", async () => {
    const root = fixture();
    try {
      writeFileSync(join(root, "data.txt"), "data");
      writeFileSync(join(root, "🦀️.rs"), `${files["🦀️.rs"]}\n#[cfg(test)]\nmod probe { pub const DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data.txt")); }\n`);
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      const { writes } = await carveWrites(plan);
      expect(writes.find((write) => write.path === "🦀️.rs")!.content).toContain("mod probe");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("never qualifies the entries of a use group that continues a path prefix", () => {
    const modules = new Set(["schema", "io"]);
    const text = "use ext::{schema::snapshot::Flow, schema::mutations::Op};\nuse ext::schema::snapshot::{Edge, Node};\nuse {schema::Alone, io::Other};\nfn f() { schema::run(); }\n";
    const out = qualifyModuleHeads(text, modules);
    expect(out).toContain("use ext::{schema::snapshot::Flow, schema::mutations::Op};");
    expect(out).toContain("use ext::schema::snapshot::{Edge, Node};");
    expect(out).toContain("use {crate::schema::Alone, crate::io::Other};");
    expect(out).toContain("fn f() { crate::schema::run(); }");
  });
});

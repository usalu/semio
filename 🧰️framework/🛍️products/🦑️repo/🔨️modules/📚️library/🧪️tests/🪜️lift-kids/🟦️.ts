/** 🪜️ Lift kids: 13 tests covering slicing a host whose child defines textual macros; keeps a #[macro_use] child in the slice of its host, because a macro is textually scoped; lifting child modules out of their host; and more. */
import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { carveWrites, planArtifactCrates } from "../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { buildSlices, collectDecls } from "../../🏗️authoring/📦️crate-tree/🧱️decl-tree/🟦️.ts";
import { absolutizeBeyond, liftModuleKids, matchesLift } from "../../🏗️authoring/📦️crate-tree/🪜️lift-kids/🟦️.ts";
import { convertMutationLeaves } from "../../🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
import { placeless } from "../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { absolutizeSuperPaths } from "../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟢️harness/🟦️.ts";

const root = (body: string): string => [
  "//! 🧪️ Lift fixture root.",
  "#[path = \".\"]",
  "pub mod schema {",
  "    #[path = \".\"]",
  "    pub mod family {",
  "        #[path = \"🧬️family/🦀️.rs\"]",
  "        mod component;",
  "        pub use component::*;",
  "        #[path = \"🧬️family/🅰️alpha/🦀️.rs\"]",
  "        pub mod alpha;",
  "        #[path = \"🧬️family/🅱️beta/🦀️.rs\"]",
  "        pub mod beta;",
  "    }",
  "    #[path = \"🧬️modules/🦀️.rs\"]",
  "    pub mod modules;",
  "}",
  body,
].join("\n");

const fixture = (alpha: string, beta: string): string => {
  const dir = mkdtempSync(join(tmpdir(), "semio-lift-kids-"));
  const write = (path: string, text: string): void => { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); };
  write("🦀️.rs", root(""));
  write("🧬️family/🦀️.rs", "//! 🧬️ family\npub struct Shared;\n");
  write("🧬️modules/🦀️.rs", "//! 🧬️ modules\npub fn measure() -> u32 { 1 }\n");
  write("🧬️family/🅰️alpha/🦀️.rs", alpha);
  write("🧬️family/🅱️beta/🦀️.rs", beta);
  return dir;
};

describe("slicing a host whose child defines textual macros", () => {
  const slices = (attribute: string): number => {
    const dir = mkdtempSync(join(tmpdir(), "semio-macro-use-"));
    const write = (path: string, text: string): void => { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); };
    write("🦀️.rs", ["//! 🧪️ macro fixture", '#[path = "."]', "pub mod terminology {", '    #[path = "🗣️terminology/🦀️.rs"]', "    mod component;", "    pub use component::*;", "}"].join("\n"));
    write("🗣️terminology/🦀️.rs", ["//! 🗣️ host", ...(attribute ? [attribute] : []), '#[path = "🔠️labels/🦀️.rs"]', "mod labels;", "labels_table! { A }"].join("\n"));
    write("🗣️terminology/🔠️labels/🦀️.rs", "macro_rules! labels_table { ($x:ident) => { pub const $x: u8 = 1; }; }\n");
    const entry = join(dir, "🦀️.rs");
    const read = (path: string): string | null => (existsSync(path) ? readFileSync(path, "utf8") : null);
    return buildSlices(collectDecls(entry, read(entry)!, read), read).slices.length;
  };

  test("keeps a #[macro_use] child in the slice of its host, because a macro is textually scoped", () => {
    expect(slices("#[macro_use]")).toBe(slices("") - 1);
  });
});

describe("lifting child modules out of their host", () => {
  test("rewrites only the super chains that climb above the lifted module and leaves the chains inside it alone", () => {
    const path = ["standards", "family", "alpha", "inner"];
    const text = "use super::sibling;\nuse super::super::sibling;\nuse super::super::super::modules::measure;\nuse super::super::super::super::top;\npub(super) fn f() {}\nmod nested { use super::local; }\n";
    const lifted = absolutizeBeyond(text, path, 1);
    expect(lifted).toBe("use super::sibling;\nuse crate::standards::family::sibling;\nuse crate::standards::modules::measure;\nuse crate::top;\npub(super) fn f() {}\nmod nested { use super::local; }\n");
    expect(absolutizeBeyond("use super::super::super::super::super::x;\n", path, 0)).toBeNull();
    expect(absolutizeBeyond("pub(super) fn f() {}\n", path, 0)).toBe("pub(crate) fn f() {}\n");
  });

  test("drops the private component segment from the absolute paths of a detached test", () => {
    expect(absolutizeSuperPaths("use super::super::component::tests::helper;\n", ["standards", "family", "alpha", "tests"])).toBe("use crate::standards::family::tests::helper;\n");
  });

  test("matches a parent by suffix, or with a trailing glob the parent and everything below it", () => {
    expect(matchesLift("standards::v2_0::schema::family", "schema::family")).toBe(true);
    expect(matchesLift("standards::v2_0::schema::family::alpha", "schema::family")).toBe(false);
    expect(matchesLift("standards::v2_0::schema::family::alpha", "schema::family::*")).toBe(true);
    expect(matchesLift("standards::v2_0::schema::family", "schema::family::*")).toBe(true);
    expect(matchesLift("standards::v2_0::schema::other", "schema::family::*")).toBe(false);
  });

  test("lets the slicer place a lifted child alone instead of uniting every sibling with its host", () => {
    const dir = fixture(
      "use super::super::modules::measure;\nuse super::beta::Beta;\npub fn alpha() -> u32 { measure() }\n",
      "use super::super::modules::measure;\npub struct Beta;\npub fn beta() -> u32 { measure() }\n",
    );
    const entry = join(dir, "🦀️.rs");
    const read = (path: string): string | null => { try { return readFileSync(path, "utf8"); } catch { return null; } };
    const before = buildSlices(collectDecls(entry, read(entry)!, read), read).slices.length;
    const lifted = liftModuleKids(null, entry, read(entry)!, ["schema::family"]);
    const overlay = (path: string): string | null => lifted.edits.get(path) ?? read(path);
    const after = buildSlices(collectDecls(entry, lifted.rootSource, overlay), overlay).slices.length;
    expect(after).toBeGreaterThan(before);
    expect(lifted.edits.get(join(dir, "🧬️family/🅰️alpha/🦀️.rs"))).toContain("use crate::schema::modules::measure;");
    expect(lifted.edits.get(join(dir, "🧬️family/🅰️alpha/🦀️.rs"))).toContain("use crate::schema::family::beta::Beta;");
    expect(lifted.notes.filter((note) => note.startsWith("lifted")).length).toBe(2);
  });

  test("keeps a child in its host when a chain climbs above the crate root and restores what it had rewritten", () => {
    const original = "use super::super::super::super::super::nowhere;\nuse super::super::modules::measure;\n";
    const dir = fixture(original, "pub struct Beta;\n");
    const entry = join(dir, "🦀️.rs");
    const lifted = liftModuleKids(null, entry, readFileSync(entry, "utf8"), ["schema::family"]);
    expect(lifted.edits.get(join(dir, "🧬️family/🅰️alpha/🦀️.rs"))).toBe(original);
    expect(lifted.notes.some((note) => note.startsWith("kept") && note.includes("crate root"))).toBe(true);
  });

  test("never places a carved crate in the directory of the package it carves", async () => {
    const dir = mkdtempSync(join(tmpdir(), "semio-lift-dir-"));
    const write = (path: string, text: string): void => { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); };
    write("Cargo.toml", '[workspace]\nresolver = "2"\nmembers = [ "📦️packages/🦀️rust" ]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n');
    write("📦️packages/🦀️rust/Cargo.toml", '[package]\nworkspace = "../.."\nname = "demo-sub"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n');
    write("🦀️.rs", "//! 🧪️ Root.\n#[path = \".\"]\npub mod zone {\n    #[path = \"🧱️one.rs\"]\n    pub mod one;\n    #[path = \"🧱️two.rs\"]\n    pub mod two;\n}\n");
    write("🧱️one.rs", "//! 🧱️ one\npub fn one() -> u32 { 1 }\n");
    write("🧱️two.rs", "//! 🧱️ two\npub fn two() -> u32 { 2 }\n");
    const plan = await planArtifactCrates(dir, "📦️packages/🦀️rust", { target: { files: 2, bytes: 1 << 20, lines: 1000 }, hard: { files: 3, bytes: 1 << 20, lines: 1000 } });
    expect(plan.problems).toEqual([]);
    expect(plan.crates.length).toBeGreaterThan(1);
    for (const crate of plan.crates) expect(crate.packageDir).not.toBe("📦️packages/🦀️rust");
    expect(new Set(plan.crates.map((crate) => crate.packageDir)).size).toBe(plan.crates.length);
  });

  test("converts a mutation leaf whose payload is an enum of phases, like the glTF leaves", () => {
    const dir = mkdtempSync(join(tmpdir(), "semio-lift-enum-leaf-"));
    const write = (path: string, text: string): void => { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); };
    const rootText = [
      "//! 🧪️ Enum leaf root.",
      "#[path = \".\"]",
      "pub mod schema {",
      "    #[path = \".\"]",
      "    pub mod mutations {",
      "        #[path = \"🧬️mutations/🦀️.rs\"]",
      "        mod component;",
      "        pub use component::*;",
      "        #[path = \".\"]",
      "        pub mod sample {",
      "            #[path = \"🧬️mutations/🅰️sample/🦀️.rs\"]",
      "            mod component;",
      "            pub use component::*;",
      "        }",
      "    }",
      "}",
      "",
    ].join("\n");
    write("🦀️.rs", rootText);
    write("🧬️mutations/🦀️.rs", "//! aggregate\nuse crate::schema::{Diff, Snap};\n\n#[derive(Clone, Debug, PartialEq, dsl::Mutations)]\n#[mutations(snapshot = Snap, diff = Diff, schema = \"demo.demo\")]\npub enum DemoMutation {\n    Sample(SampleMutation),\n}\n\npub use crate::schema::mutations::sample::{sample, SampleMutation};\n");
    write("🧬️mutations/🅰️sample/🦀️.rs", [
      "//! sample leaf",
      "use crate::schema::mutations::DemoMutation;",
      "use crate::schema::{Diff, Snap};",
      "use protocol::{MutationKind, SemanticDescriptor};",
      "",
      "#[derive(Clone, Debug, PartialEq)]",
      "pub struct SamplePayload { pub id: String }",
      "",
      "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]",
      "#[mutation_leaf(contract = ::protocol, payload = Apply)]",
      "pub enum SampleMutation {",
      "    Apply(SamplePayload),",
      "}",
      "",
      "pub fn sample(payload: SamplePayload) -> DemoMutation {",
      "    DemoMutation::Sample(SampleMutation::Apply(payload))",
      "}",
      "",
      "impl MutationKind<Snap, DemoMutation> for SampleMutation {",
      "    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: \"x\", entity: \"y\", kind: \"z\", record: \"R\" };",
      "    fn diff(&self, _base: &Snap) -> protocol::MutationOutcome<Diff> { protocol::MutationOutcome::new(Diff::default()) }",
      "    fn inverse(&self, _base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> { Ok(Vec::new()) }",
      "}",
      "",
    ].join("\n"));
    const read = (path: string): string | null => (existsSync(path) ? readFileSync(path, "utf8") : null);
    const result = convertMutationLeaves(join(dir, "🦀️.rs"), rootText, read);
    expect(result.converted).toEqual(["SampleMutation"]);
    const leaf = result.edits.get(join(dir, "🧬️mutations/🅰️sample/🦀️.rs"))!;
    expect(leaf).toContain("pub fn sample<Op>(payload: SamplePayload) -> Op");
    expect(leaf).toContain("Op::from(SampleMutation::Apply(payload))");
    expect(leaf).toMatch(/impl<Op> MutationKind<Snap, Op> for SampleMutation/u);
  });

  test("detaches a test that names the aggregate and needs nothing from its host, also when the host is a file module", () => {
    const dir = mkdtempSync(join(tmpdir(), "semio-lift-detach-"));
    const write = (path: string, text: string): void => { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); };
    const rootText = "//! 🧪️ root\n#[path = \".\"]\npub mod zone {\n    #[path = \"🧬️leaf/🦀️.rs\"]\n    pub mod leaf;\n    #[path = \"🧬️aggregate/🦀️.rs\"]\n    pub mod aggregate;\n}\n";
    write("🦀️.rs", rootText);
    write("🧬️aggregate/🦀️.rs", "//! aggregate\npub enum DemoMutation { Leaf }\n");
    write("🧬️leaf/🦀️.rs", "//! leaf\npub fn build() -> u32 { 1 }\n#[cfg(test)]\n#[path = \"🧪️tests/🦀️.rs\"]\nmod tests;\n");
    write("🧬️leaf/🧪️tests/🦀️.rs", "use crate::zone::aggregate::DemoMutation;\n#[test]\nfn names_the_aggregate() { let _ = DemoMutation::Leaf; }\n");
    const read = (path: string): string | null => (existsSync(path) ? readFileSync(path, "utf8") : null);
    const slices = buildSlices(collectDecls(join(dir, "🦀️.rs"), rootText, read), read, undefined, { detachTestsMentioning: new Set(["DemoMutation"]), detachAllTests: true }).slices;
    expect(slices.some((slice) => slice.decls.some((decl) => decl.testOnly) && !slice.decls.some((decl) => decl.id.endsWith("::leaf") && !decl.testOnly))).toBe(true);
  });

  test("declares an empty feature of the carved package in every carved crate and forwards it from the package", async () => {
    const dir = mkdtempSync(join(tmpdir(), "semio-lift-features-"));
    const write = (path: string, text: string): void => { mkdirSync(dirname(join(dir, path)), { recursive: true }); writeFileSync(join(dir, path), text); };
    write("Cargo.toml", '[workspace]\nresolver = "2"\nmembers = [ "📦️packages/🦀️rust" ]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n');
    write("📦️packages/🦀️rust/Cargo.toml", '[package]\nworkspace = "../.."\nname = "demo-sub"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[features]\n# 🧪️ Test only.\ntesting = []\n\n[dependencies]\n');
    write("🦀️.rs", "//! 🧪️ Root.\n#[path = \".\"]\npub mod zone {\n    #[path = \"🧱️one.rs\"]\n    pub mod one;\n    #[path = \"🧱️two.rs\"]\n    pub mod two;\n}\n");
    write("🧱️one.rs", "//! 🧱️ one\npub fn one() -> u32 { 1 }\n");
    write("🧱️two.rs", "//! 🧱️ two\nuse crate::zone::one::one;\npub fn two() -> u32 { one() + 1 }\n");
    const plan = await planArtifactCrates(dir, "📦️packages/🦀️rust", { target: { files: 2, bytes: 1 << 20, lines: 1000 }, hard: { files: 3, bytes: 1 << 20, lines: 1000 } });
    expect(plan.crates.length).toBeGreaterThan(1);
    const { writes } = await carveWrites(plan);
    for (const crate of plan.crates) {
      const manifest = writes.find((write) => write.path === join(crate.packageDir, "Cargo.toml"))!.content;
      const forwards = crate.dependsOn.map((id) => `"${plan.crates.find((entry) => entry.id === id)!.name}/testing"`).join(", ");
      expect(manifest).toContain(`[features]\ntesting = [${forwards}]`);
      for (const id of crate.dependsOn) expect(manifest).toMatch(new RegExp(`\\[dev-dependencies\\][^\\[]*${plan.crates.find((entry) => entry.id === id)!.name} = \\{ path = "[^"]+", features = \\["testing"\\] \\}`, "u"));
    }
    const packageManifest = writes.find((write) => write.path === "📦️packages/🦀️rust/Cargo.toml")!.content;
    expect(packageManifest).toContain(`testing = [${plan.crates.map((crate) => `"${crate.name}/testing"`).join(", ")}]`);
  });

  test("compares a type that is not found in scope without its label, because the label depends on the imports around it", () => {
    expect(placeless("E0425\tcannot find type `GltfX` in this scope\ta.rs\tnot found in this scope")).toBe("E0425\tcannot find type `GltfX` in this scope\ta.rs");
    expect(placeless("E0425\tcannot find type `GltfX` in this scope\ta.rs\t")).toBe("E0425\tcannot find type `GltfX` in this scope\ta.rs");
    expect(placeless("E0432\tunresolved import `x`\ta.rs\tno `x` in the root")).toBe("E0432\tunresolved import `x`\ta.rs\tno `x` in the root");
  });
});

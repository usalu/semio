import { describe, expect, test } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { applyCarveWrites, carveWrites, planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { cfgImpliesTest, parseRustSourceTree, referencedItemNames } from "../../../🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { implHeader } from "../../../🏗️authoring/📦️crate-tree/🗺️plan/🧭️module-chains/🟦️.ts";
import { allItems } from "../../../🏗️authoring/📦️crate-tree/🗺️plan/🧰️tree-helpers/🟦️.ts";
import { FIXTURE_READER_CRATE } from "../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { checkOrphanRules, compareModuleTrees, resolveCrateModules } from "../../../🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
import { files, fixture } from "../🏗️virtual-crate/🟦️.ts";

describe("artifact crate carve", () => {
  test("treats every cfg that only holds under cfg(test) as a test-only module mount", () => {
    expect(["cfg(test)", "cfg( test )", 'cfg(all(test,not(target_arch="wasm32")))', "cfg(all(feature = \"x\", test))", "cfg(any(test, all(test, unix)))"].map(cfgImpliesTest)).toEqual([true, true, true, true, true]);
    expect(["cfg(unix)", "cfg(not(test))", "cfg(any(test, unix))", "cfg(all(unix, windows))", "path = \"x\""].map(cfgImpliesTest)).toEqual([false, false, false, false, false]);
    const tree = parseRustSourceTree('#[cfg(all(test,not(target_arch="wasm32")))]\n#[path="q.rs"]\nmod quick;\nmod live;\n');
    expect(tree.modules.map((node) => [node.name, node.conditionalTest])).toEqual([["quick", true], ["live", false]]);
  });

  test("counts only identifiers that can name a module-level item as references", () => {
    const names = referencedItemNames("fn f(artifact: &A, count: u8) -> S { let v = artifact.definition.steps(); S { definition: v, artifact: count, extra } }");
    expect([...names].sort()).toEqual(["A", "S", "artifact", "count", "extra", "f", "fn", "let", "u8", "v"]);
    expect([...referencedItemNames("let x = pack::snapshot::Pack::new(crate::forms_steps(&self::helper), super::up());")].sort()).toEqual(["crate", "forms_steps", "helper", "let", "pack", "self", "super", "up", "x"]);
  });

  test("parses the module skeleton with exact offsets", () => {
    const tree = parseRustSourceTree(files["🦀️.rs"]!);
    expect(tree.modules.map((node) => node.name)).toEqual(["schema", "io"]);
    const schema = tree.modules[0]!;
    expect(schema.children.map((node) => `${node.name}:${node.inline}:${node.pathAttribute}`)).toEqual(["component:false:🧬️schema/🦀️.rs", "snapshot:false:🧬️schema/📸️snapshot/🦀️.rs", "diff:false:🧬️schema/🔺️diff/🦀️.rs"]);
    expect(files["🦀️.rs"]!.slice(schema.start, schema.end).startsWith("#[path = \".\"]")).toBe(true);
    expect(tree.items.map((item) => item.name)).toContain("DOCUMENT");
  });

  test("keeps impls of foreign traits with the type they implement and cuts along declarations", async () => {
    const root = fixture();
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { target: { files: 3, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::").endsWith(suffix)));
      expect(holder("schema::snapshot")).toBeDefined();
      expect(holder("io::text")).toBe(holder("schema::snapshot"));
      expect(holder("schema::diff")).not.toBe(holder("schema::snapshot"));
      const { writes } = await carveWrites(plan);
      const paths = writes.map((write) => write.path);
      expect(paths.some((path) => path.endsWith("📦️packages/🦀️rust/🦀️.rs"))).toBe(true);
      const diffCrate = holder("schema::diff")!;
      const mirror = writes.find((write) => write.path === `${diffCrate.packageDir}/🦀️.rs`)!.content;
      expect(mirror).toMatch(/pub use demo_artifact_[a-z0-9_]+::schema::snapshot;/u);
      expect(writes.find((write) => write.path.endsWith("Cargo.lock"))!.content).toContain(`name = "${holder("schema::snapshot")!.name}"`);
      expect(writes.find((write) => write.path === "Cargo.toml")!.content).toContain("members = [");
      const original = resolveCrateModules(root, join(root, "🦀️.rs"));
      applyCarveWrites(root, writes);
      const carved = [resolveCrateModules(root, join(root, "🦀️.rs")), ...plan.crates.map((crate) => resolveCrateModules(root, join(root, crate.packageDir, "🦀️.rs")))];
      expect(compareModuleTrees(original, carved).problems).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("gives every crate that reads a rewritten fixture the shared reader as dev-dependency", async () => {
    const root = fixture();
    try {
      const schemaRoot = join(root, "🧬️schema/🦀️.rs");
      writeFileSync(schemaRoot, `${files["🧬️schema/🦀️.rs"]}#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n`);
      mkdirSync(join(root, "🧬️schema/🧪️tests/🧫️fixtures"), { recursive: true });
      writeFileSync(join(root, "🧬️schema/🧪️tests/🧫️fixtures/a.json"), "{}\n");
      writeFileSync(join(root, "🧬️schema/🧪️tests/🦀️.rs"), '#[test]\nfn reads_the_fixture() { assert_eq!(include_str!("🧫️fixtures/a.json"), "{}\\n"); }\n');
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { target: { files: 3, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 }, tests: { fixtures: true, aggregate: false, hoist: false } });
      const { writes } = await carveWrites(plan);
      const reader = writes.find((write) => write.path.endsWith(".rs") && write.content.includes("fixture_text!("));
      expect(reader).toBeDefined();
      expect(reader!.content).toContain("use semio_framework_test_fixtures::fixture_text;");
      expect(reader!.content).not.toContain("runtime_fixture");
      const manifests = writes.filter((write) => write.path.endsWith("Cargo.toml") && write.content.includes(FIXTURE_READER_CRATE));
      expect(manifests.length).toBeGreaterThan(0);
      for (const manifest of manifests) expect(manifest.content).toMatch(/\[dev-dependencies\][^[]*semio-framework-test-fixtures = \{ path = "[^"]*fixture-reader[^"]*" \}/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("detects orphan-rule violations that a wrong cut would create", () => {
    const sources = (text: string) => new Map([["lib.rs", text]]);
    const violations = checkOrphanRules(
      [{ name: "types", sources: sources("pub struct Snap;") }, { name: "codec", sources: sources("impl std::fmt::Display for Snap { }") }],
      implHeader,
      allItems,
    );
    expect(violations).toEqual([{ crate: "codec", file: "lib.rs", trait: "Display", selfType: "Snap", definedIn: ["types"] }]);
    expect(checkOrphanRules([{ name: "types", sources: sources("pub struct Snap;\nimpl std::fmt::Display for Snap { }") }], implHeader, allItems)).toEqual([]);
  });

  test("accepts an impl of a foreign trait on a foreign type when a trait argument is a local type", () => {
    const sources = (text: string) => new Map([["lib.rs", text]]);
    expect(implHeader("impl MutationDiff<Play> for Diff { }")).toEqual({ trait: "MutationDiff", self: "Diff", traitArgs: ["Play"] });
    expect(checkOrphanRules([{ name: "types", sources: sources("pub struct Diff;") }, { name: "play", sources: sources("pub struct Play;\nimpl MutationDiff<Play> for Diff { }") }], implHeader, allItems)).toEqual([]);
    expect(checkOrphanRules([{ name: "types", sources: sources("pub struct Diff;") }, { name: "play", sources: sources("pub struct Play;\nimpl Other<Vec<u8>> for Diff { }") }], implHeader, allItems)).toEqual([{ crate: "play", file: "lib.rs", trait: "Other", selfType: "Diff", definedIn: ["types"] }]);
  });
});

import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { carveWrites, parseCrateBudget, planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { buildTestkitCrate, cfgFeatures, externalDependencies, optionalDependencyRow, planTestkitWiring, requiredTestFeatures, scanTestGates, testGateFeatures, testkitFeatureRow, withFeatureRows, withRequiredFeatures } from "../../../🏗️authoring/📦️crate-tree/🧰️testkit/🟦️.ts";
import { declaredNames } from "../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { files } from "../🏗️virtual-crate/🟦️.ts";

describe("test-support wiring", () => {
  test("reads a test-support gate from a cfg attribute and ignores real build features", () => {
    expect(testGateFeatures('cfg(any(test, feature = "testkit"))')).toEqual(["testkit"]);
    expect(testGateFeatures('cfg(any(test, feature = "protocol-laws", feature = "testing"))')).toEqual(["protocol-laws", "testing"]);
    expect(testGateFeatures('cfg(feature = "testkit")')).toEqual(["testkit"]);
    expect(testGateFeatures('cfg(all(test, feature = "wasm"))')).toEqual([]);
    expect(testGateFeatures('cfg(feature = "wasm")')).toEqual([]);
    expect(testGateFeatures('cfg(not(any(test, feature = "testkit")))')).toEqual([]);
    expect(testGateFeatures("cfg(test)")).toEqual([]);
    expect(testGateFeatures('path = "x"')).toEqual([]);
    expect(cfgFeatures('cfg(any(test, all(feature = "a", not(feature = "b"))))')).toEqual(["a"]);
  });

  test("finds the gated items with their exported names, the leaves of a gated re-export and the type of a gated method", () => {
    const text = [
      "#[cfg(any(test, feature = \"testkit\"))]",
      "pub fn fixture() -> u8 { 1 }",
      "#[cfg(any(test, feature = \"testkit\"))]",
      "fn private_helper() {}",
      "#[cfg(any(test, feature = \"testkit\"))]",
      "pub use inner::{Alpha, Beta as Gamma};",
      "#[cfg(any(test, feature = \"testkit\"))]",
      "use inner::Hidden;",
      "#[macro_export]",
      "#[cfg(any(test, feature = \"testkit\"))]",
      "macro_rules! kit { () => {} }",
      "pub struct Holder;",
      "impl<T> Holder {",
      "    #[cfg(any(test, feature = \"testkit\"))]",
      "    pub fn with_live_value() -> Self { Holder }",
      "}",
      "#[cfg(any(test, feature = \"testkit\"))]",
      "pub mod harness;",
      "#[cfg(test)]",
      "pub fn only_test() {}",
    ].join("\n");
    const gates = scanTestGates(text);
    expect(gates.map((gate) => [gate.names, gate.method, gate.owner, gate.use])).toEqual([
      [["fixture"], false, null, false],
      [[], false, null, false],
      [["Alpha", "Gamma"], false, null, true],
      [[], false, null, true],
      [["kit"], false, null, false],
      [["with_live_value"], true, "Holder", false],
      [["harness"], false, null, false],
    ]);
    expect(gates.every((gate) => gate.features.join() === "testkit")).toBe(true);
    expect(text.slice(gates[0]!.start, gates[0]!.end)).toBe("#[cfg(any(test, feature = \"testkit\"))]\npub fn fixture() -> u8 { 1 }");
    expect([...declaredNames("pub fn a() {}\nstruct B;\nimpl B { fn inside() {} }\nmacro_rules! m { () => {} }")].sort()).toEqual(["B", "a", "m"]);
  });

  const crateOf = async (name: string, dependsOn: string[], files: Record<string, string>, gatedFiles: Record<string, string[]> = {}) => buildTestkitCrate(
    { name, ident: name.replaceAll("-", "_"), dependsOn, files: new Map(Object.entries(files)) },
    async (feature) => new Set(gatedFiles[feature] ?? []),
  );
  const manifest = '[package]\nname = "demo"\n\n[dependencies]\nplugin = { path = "../plugin" }\nkernel = { workspace = true }\n\n[dev-dependencies]\nplugin = { path = "../plugin", features = ["app-testing"] }\nkernel = { workspace = true, features = ["laws"] }\ntrace-testing = { path = "../trace" }\nunrelated = "1"\n';
  const rows = { dependencies: ['plugin = { path = "../plugin" }', "kernel = { workspace = true }"], devDependencies: ['plugin = { path = "../plugin", features = ["app-testing"] }', 'kernel = { workspace = true, features = ["laws"] }', 'trace-testing = { path = "../trace" }', 'unrelated = "1"'] };

  test("declares a gate feature on its owner, enables the dev-only crates and external features the gated code names, and lets only the consumers enable it", async () => {
    const externals = externalDependencies(manifest, rows);
    expect(externals.map((entry) => [entry.key, entry.dependency, entry.devOnly, entry.devFeatures])).toEqual([["kernel", true, false, ["laws"]], ["plugin", true, false, ["app-testing"]], ["trace-testing", false, true, []], ["unrelated", false, true, []]]);
    const owner = await crateOf("demo-owner", [], {
      "a/🦀️.rs": '#[cfg(any(test, feature = "testkit"))]\npub fn fixture() -> u8 { trace_testing::three() + plugin::artifact_app_laws::run() }\npub fn plain() {}\n#[cfg(any(test, feature = "testkit"))]\nmod harness;',
      "a/harness.rs": "pub fn inside() { kernel::used(); }",
    }, { testkit: ["a/harness.rs"] });
    const consumer = await crateOf("demo-consumer", ["demo-owner"], { "b/🦀️.rs": "pub fn go() {}\n#[cfg(test)]\nmod checks { fn t() { assert_eq!(demo_owner::fixture(), 3); } }" });
    const bystander = await crateOf("demo-bystander", ["demo-owner"], { "c/🦀️.rs": "pub fn plain() { demo_owner::plain(); }" });
    const reader = (external: { key: string }, feature: string) => (external.key === "plugin" && feature === "app-testing" ? new Set(["artifact_app_laws"]) : external.key === "kernel" ? new Set(["laws_only"]) : null);
    const wiring = planTestkitWiring([owner, consumer, bystander], externals, new Set(), reader);
    const ownerWiring = wiring.get("demo-owner")!;
    expect(ownerWiring.features.map(testkitFeatureRow)).toEqual(['testkit = ["dep:trace-testing", "kernel/laws", "plugin/app-testing"]']);
    expect(ownerWiring.optionalDependencies).toEqual(["trace-testing"]);
    expect(wiring.get("demo-consumer")!.features).toEqual([]);
    expect([...wiring.get("demo-consumer")!.consumed]).toEqual([["demo-owner", ["testkit"]]]);
    expect([...wiring.get("demo-bystander")!.consumed]).toEqual([]);
    expect(ownerWiring.gatedNames.get("testkit")).toEqual(new Set(["fixture"]));
  });

  test("does not enable the external features of a crate that the gated code only uses through its plain api", async () => {
    const externals = externalDependencies(manifest, rows);
    const owner = await crateOf("demo-owner", [], { "a/🦀️.rs": '#[cfg(any(test, feature = "testkit"))]\npub fn fixture() { plugin::PluginBuilder::new(); }' });
    const wiring = planTestkitWiring([owner], externals, new Set(), () => new Set(["artifact_app_laws"]));
    expect(wiring.get("demo-owner")!.features.map(testkitFeatureRow)).toEqual(["testkit = []"]);
    const unknown = planTestkitWiring([owner], externals, new Set(), () => null);
    expect(unknown.get("demo-owner")!.features.map(testkitFeatureRow)).toEqual(['testkit = ["plugin/app-testing"]']);
  });

  test("forwards a feature to the crates whose gated items the gated code names, declares it on a re-exporter only when someone enables it, and skips features the manifest declares", async () => {
    const hub = await crateOf("demo-hub", [], { "h/🦀️.rs": '#[cfg(any(test, feature = "testkit"))]\npub mod harness { pub fn ctx() {} }' });
    const facade = await crateOf("demo-facade", ["demo-hub"], { "f/🦀️.rs": '#[cfg(any(test, feature = "testkit"))]\npub use demo_hub::harness;' });
    const user = await crateOf("demo-user", ["demo-facade"], { "u/🦀️.rs": "#[cfg(test)]\nfn t() { demo_facade::harness::ctx(); }" });
    const idle = await crateOf("demo-idle", ["demo-hub"], { "i/🦀️.rs": '#[cfg(any(test, feature = "testkit"))]\npub use demo_hub::harness;' });
    const wiring = planTestkitWiring([hub, facade, user, idle], [], new Set());
    expect(wiring.get("demo-hub")!.features.map(testkitFeatureRow)).toEqual(["testkit = []"]);
    expect(wiring.get("demo-facade")!.features.map(testkitFeatureRow)).toEqual(['testkit = ["demo-hub/testkit"]']);
    expect(wiring.get("demo-idle")!.features).toEqual([]);
    expect([...wiring.get("demo-facade")!.consumed]).toEqual([["demo-hub", ["testkit"]]]);
    expect([...wiring.get("demo-user")!.consumed]).toEqual([["demo-facade", ["testkit"]]]);
    expect(planTestkitWiring([hub, facade], [], new Set(["testkit"])).get("demo-hub")!.features).toEqual([]);
  });

  test("gives a gated file that two crates mount to the crate that does not depend on the other", async () => {
    const shared = '#[cfg(any(test, feature = "testkit"))]\npub fn demo_cases() -> u8 { 1 }';
    const base = await crateOf("demo-base", [], { "s/🦀️.rs": shared });
    const aggregate = await crateOf("demo-aggregate", ["demo-base"], { "s/🦀️.rs": shared, "t/🦀️.rs": "fn t() { demo_cases(); }" });
    const tests = await crateOf("demo-tests", ["demo-aggregate", "demo-base"], { "u/🦀️.rs": "fn t() { demo_cases(); }" });
    const wiring = planTestkitWiring([base, aggregate, tests], [], new Set());
    expect(wiring.get("demo-base")!.features.map(testkitFeatureRow)).toEqual(["testkit = []"]);
    expect(wiring.get("demo-aggregate")!.features).toEqual([]);
    expect([...wiring.get("demo-aggregate")!.consumed]).toEqual([]);
    expect([...wiring.get("demo-tests")!.consumed]).toEqual([["demo-base", ["testkit"]]]);
  });

  test("confirms a gated method by the name of its type", async () => {
    const owner = await crateOf("demo-owner", [], { "a/🦀️.rs": 'pub struct Holder;\nimpl Holder {\n#[cfg(any(test, feature = "testkit"))]\npub fn with_live_value() {}\n}' });
    const caller = await crateOf("demo-caller", ["demo-owner"], { "b/🦀️.rs": "fn t() { demo_owner::Holder::with_live_value(); }" });
    const stranger = await crateOf("demo-stranger", ["demo-owner"], { "c/🦀️.rs": "fn t() { other.with_live_value(); }" });
    const wiring = planTestkitWiring([owner, caller, stranger], [], new Set());
    expect([...wiring.get("demo-caller")!.consumed]).toEqual([["demo-owner", ["testkit"]]]);
    expect([...wiring.get("demo-stranger")!.consumed]).toEqual([]);
  });

  test("renders optional dependency rows, feature rows and required-features without disturbing the rest of a manifest", () => {
    expect(optionalDependencyRow('trace = { path = "../trace" }')).toBe('trace = { path = "../trace", optional = true }');
    expect(optionalDependencyRow("trace.workspace = true")).toBe("trace = { workspace = true, optional = true }");
    expect(optionalDependencyRow('trace = "1"')).toBe('trace = { version = "1", optional = true }');
    expect(optionalDependencyRow('trace = { path = "x", optional = true }')).toBe('trace = { path = "x", optional = true }');
    const base = '[package]\nname = "x"\n\n[[test]]\nname = "law"\npath = "t.rs"\n\n[[test]]\nname = "other"\npath = "o.rs"\nrequired-features = ["a"]\n\n[dependencies]\nq = "1"\n';
    const withFeature = withFeatureRows(base, [{ name: "testkit", entries: ["demo/testkit"] }]);
    expect(withFeature).toContain('[features]\ntestkit = ["demo/testkit"]\n\n[dependencies]');
    expect(withFeatureRows(withFeature, [{ name: "testkit", entries: [] }])).toContain("testkit = []");
    const required = withRequiredFeatures(withFeature, new Map([["law", ["testkit"]], ["other", ["testkit"]]]));
    expect(required).toContain('name = "law"\npath = "t.rs"\nrequired-features = ["testkit"]\n');
    expect(required).toContain('required-features = ["a", "testkit"]');
    expect(withRequiredFeatures(base, new Map())).toBe(base);
    expect(requiredTestFeatures(new Set(["facade_helper", "x"]), new Map([["testkit", new Set(["facade_helper"])], ["other", new Set(["nothing"])]]))).toEqual(["testkit"]);
  });

  test("carves a gated helper into a feature of its owner crate and enables it from the dev-dependencies of the crate that names it", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-testkit-carve-"));
    const tree: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-kit"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-kit"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n\n[dev-dependencies]\ndemo-devtool = { path = "../../🧰️devtool" }\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-kit",{snapshotSqliteTests:[]});\n',
      "🧰️devtool/Cargo.toml": '[package]\nname = "demo-devtool"\nversion = "0.1.0"\n\n[lib]\npath = "🦀️.rs"\n',
      "🧰️devtool/🦀️.rs": "pub fn three() -> u8 { 3 }\n",
      "🦀️.rs": '//! root\n#[path = "."]\npub mod schema {\n    #[path = "🧬️schema/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
      "🧬️schema/🦀️.rs": '//! schema\npub fn value() -> u8 { 2 }\n#[cfg(any(test, feature = "testkit"))]\npub fn fixture() -> u8 { demo_devtool::three() }\n',
      "🚪️io/🦀️.rs": "//! io\npub fn run() -> u8 { crate::schema::value() }\n#[cfg(test)]\nmod checks {\n    #[test]\n    fn uses_the_fixture() { assert_eq!(crate::schema::fixture(), 3); }\n}\n",
    };
    for (const [path, content] of Object.entries(tree)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { target: { files: 1, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 }, tests: { fixtures: false, aggregate: false, hoist: false, detach: false } });
      expect(plan.problems).toEqual([]);
      const { writes } = await carveWrites(plan);
      const manifestOf = (crate: { packageDir: string }) => writes.find((write) => write.path === `${crate.packageDir}/Cargo.toml`)!.content;
      const owner = plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === "schema"))!;
      const consumer = plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === "io"))!;
      expect(owner).not.toBe(consumer);
      const parsedOwner = Bun.TOML.parse(manifestOf(owner)) as { features?: Record<string, string[]>; dependencies: Record<string, { optional?: boolean }> };
      expect(parsedOwner.features).toEqual({ testkit: ["dep:demo-devtool"] });
      expect(parsedOwner.dependencies["demo-devtool"]!.optional).toBe(true);
      const parsedConsumer = Bun.TOML.parse(manifestOf(consumer)) as { features?: unknown; "dev-dependencies": Record<string, { features?: string[] }> };
      expect(parsedConsumer.features).toBeUndefined();
      expect(parsedConsumer["dev-dependencies"][owner.name]!.features).toEqual(["testkit"]);
      const facadeManifest = writes.find((write) => write.path === "📦️packages/🦀️rust/Cargo.toml")!.content;
      expect(facadeManifest).not.toContain("[features]");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);
});

describe("crate budget flag", () => {
  test("parses <files>/<bytes>/<lines> and refuses everything else", () => {
    expect(parseCrateBudget("28/200000/3000")).toEqual({ files: 28, bytes: 200000, lines: 3000 });
    for (const text of ["", "28/200000", "28/200000/3000/1", "0/1/1", "a/b/c", "28/-1/3000", "28/2e5/3000", "28/ 200000/3000"]) expect(() => parseCrateBudget(text)).toThrow("positive integers");
  });
});

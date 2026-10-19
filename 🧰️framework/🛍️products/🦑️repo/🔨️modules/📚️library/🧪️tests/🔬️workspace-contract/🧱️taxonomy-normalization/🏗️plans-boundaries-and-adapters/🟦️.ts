/** 🏗️ The physical format plans, pilot destinations, hazards, reference adapters and package boundary tests. */
import { expect, test } from "bun:test";
import { rmSync } from "node:fs";
import { inventoryTaxonomy } from "../../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../../../🧹️normalization/🧠️planning/🟦️.ts";
import { taxonomyPlanDigest } from "../../../../🧹️normalization/🗃️affected-state/🟦️.ts";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { normalizationFixture, normalizationMove, normalizationPlan } from "../../🏗️shared-fixtures/🟦️.ts";

/** 📐 Registers the physical format plans, the pilot destinations, the NFC and VS16 hazards, the reference adapters and the package boundary tests. */
export function registerPlansBoundariesAndAdapterTests(): void {
  test("plans physical-format leaves, compound extensions, generic-stem drops, parent ownership, assets, generated output, and exact fixed/configurable entries", () => {
    const fixture = normalizationFixture("kinds", {
      "🧪️case/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"fixture-rust\"\nversion = \"0.0.0\"\n",
      "🧪️case/📦️packages/🟦️typescript/package.json": "{\"name\":\"fixture-typescript\",\"exports\":{\".\":\"./🟦️.ts\"}}\n",
      "🧪️case/📦️packages/🟦️typescript/🟦️.ts": "export {};\n",
      "🧪️case/🦀️component.rs": "pub struct Component;\n",
      "🧪️case/🟦️component.ts": "export const component = true;\n",
      "🧪️case/🧪️react-component/🟦️react-component.tsx": "export const ReactComponent = () => null;\n",
      "🧪️case/🧪️types/🟦️types.d.ts": "export interface FixtureType { readonly value: number }\n",
      "🧪️case/🧪️contract/🔣️contract.json": "{\"valid\":true}\n",
      "🧪️case/🧪️guide/📝️guide.md": "# Fixture guide\n",
      "🧪️case/🖼️asset.png": "not-a-decoded-image\n",
      "🧪️case/🤖️generated/🟦️component.ts": "export const generated = true;\n",
    });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(normalizationMove(plan, "🧪️case/🦀️component.rs").destinationPath).toMatch(/\/🧪️case\/🦀️\.rs$/u);
      expect(normalizationMove(plan, "🧪️case/🟦️component.ts").destinationPath).toMatch(/\/🧪️case\/🟦️\.ts$/u);
      expect(normalizationMove(plan, "🧪️react-component/🟦️react-component.tsx").destinationPath).toMatch(/\/🧪️react-component\/🟦️\.tsx$/u);
      expect(normalizationMove(plan, "🧪️types/🟦️types.d.ts").destinationPath).toMatch(/\/🧪️types\/🟦️\.d\.ts$/u);
      expect(normalizationMove(plan, "🧪️contract/🔣️contract.json").destinationPath).toMatch(/\/🧪️contract\/🔣️\.json$/u);
      expect(normalizationMove(plan, "🧪️guide/📝️guide.md").destinationPath).toMatch(/\/🧪️guide\/📝️\.md$/u);
      const asset = normalizationMove(plan, "🧪️case/🖼️asset.png").destinationPath;
      expect(asset.endsWith("/🖼️.png")).toBe(true);
      expect(asset).not.toContain("asset.png");
      expect(normalizationMove(plan, "🧪️case/🤖️generated/🟦️component.ts").destinationPath).toContain("/🤖️generated/");
      expect(plan.moves.some((move) => move.sourcePath.endsWith("/Cargo.toml"))).toBe(false);
      expect(plan.moves.some((move) => move.sourcePath.endsWith("/package.json"))).toBe(false);
      expect(plan.moves.some((move) => move.sourcePath.endsWith("/🟦️.ts"))).toBe(false);
      expect(plan.planDigest).toBe(taxonomyPlanDigest(plan));
      expect(plan.sourceTreeDigest.length).toBeGreaterThan(0);
      expect(plan.expectedPostStateDigest.length).toBeGreaterThan(0);
      expect(canonicalJson(plan)).toBe(canonicalJson(planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] })));
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("preserves the D3D12, PDF mutation, and BMP pilot destinations exactly", () => {
    const fixture = normalizationFixture("named-pilot", {
      "🧪️golden/🪟️d3d12/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"d3d12-fixture\"\nversion = \"0.0.0\"\n",
      "🧪️golden/🪟️d3d12/📦️packages/🦀️rust/🦀️backend.rs": "pub struct Backend { pub device: usize }\n",
      "🧪️golden/🧪️tests/🧪️mutate-pdf-1-7/🦀️component.rs": "pub fn mutate() {}\n",
      "🧪️golden/🧪️tests/🧪️mutate-pdf-1-7/component.feature": "Feature: mutate PDF 1.7\n",
      "🧪️golden/🖼️rathaus-ahlen-grundriss.bmp": "BMfixture\n",
    });
    try {
      const { plan } = normalizationPlan(fixture);
      const prefix = fixture.scope;
      expect(normalizationMove(plan, "🪟️d3d12/📦️packages/🦀️rust/🦀️backend.rs").destinationPath).toBe(`${prefix}/🧪️golden/🪟️d3d12/⚙️backend/🦀️.rs`);
      const rustCase = normalizationMove(plan, "🧪️tests/🧪️mutate-pdf-1-7/🦀️component.rs").destinationPath;
      const featureCase = normalizationMove(plan, "🧪️tests/🧪️mutate-pdf-1-7/component.feature").destinationPath;
      expect(rustCase).toMatch(/\/🧪️mutate-pdf-1-7\/🦀️\.rs$/u);
      expect(featureCase).toBe(rustCase.replace(/🦀️\.rs$/u, "🥒️.feature"));
      expect(normalizationMove(plan, "🖼️rathaus-ahlen-grundriss.bmp").destinationPath).toBe(`${prefix}/🧪️golden/🖼️rathaus-ahlen-grundriss/🖼️.bmp`);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("normalizes NFC and VS16 and blocks same-kind, platform, and path-budget hazards", () => {
    const longPath = Array.from({ length: 18 }, (_, index) => `🧩️segment-${String(index).padStart(2, "0")}`).join("/");
    const fixture = normalizationFixture("collisions", {
      "🟦️collision.ts": "export const component = 1;\n",
      "🟦️collisionindex.ts": "export const index = 2;\n",
      "🦀cunicodeomponent.rs": "pub struct MissingVs16;\n",
      "🧪️unicode/🧪️cafe\u0301/🟦️.ts": "export const nfc = true;\n",
      "platformCON.ts": "export const reserved = true;\n",
      [`🧪️length/${longPath}/🟦️.ts`]: "export const long = true;\n",
    });
    try {
      const { inventory, plan } = normalizationPlan(fixture);
      const rendered = canonicalJson(plan);
      expect(normalizationMove(plan, "🦀cunicodeomponent.rs").destinationPath).toContain("🦀️.rs");
      expect(inventory.entries.some((entry) => entry.sourcePath.endsWith("🧪️unicode/🧪️cafe\u0301/🟦️.ts"))).toBe(true);
      expect(normalizationMove(plan, "🧪️unicode/🧪️cafe\u0301/🟦️.ts").destinationPath.normalize("NFC")).toBe(normalizationMove(plan, "🧪️unicode/🧪️cafe\u0301/🟦️.ts").destinationPath);
      expect(plan.moves.filter((move) => move.destinationPath.endsWith("/🟦️.ts")).length).toBeGreaterThanOrEqual(2);
      expect(rendered).toMatch(/collision/i);
      expect(rendered).toMatch(/(?:reserved|platform|windows)/i);
      expect(rendered).toMatch(/(?:path.{0,20}length|length.{0,20}path|240)/i);
      expect(plan.unresolved.length + inventory.violations.length).toBeGreaterThan(0);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("structured reference adapters emit locations, old/new values, and preimage hashes", () => {
    const fixture = normalizationFixture("references", {
      "package.json": "{\"name\":\"fixture\",\"exports\":{\".\":\"./🧪️subject/🟦️component.ts\"}}\n",
      "📋️project.json": "{\"targets\":{\"build\":{\"options\":{\"entryFile\":\"🧪️subject/🟦️component.ts\"}}}}\n",
      "🧪️subject/🦀️component.rs": "pub const VALUE: i32 = 1;\n",
      "🧪️subject/🟦️component.ts": "export const value = 1;\n",
      "🧪️subject/component.cpp": "int value() { return 1; }\n",
      "fixture/subject/component.py": "value = 1\n",
      "🧪️consumer/🦀️component.rs": "#[path = \"../🧪️subject/🦀️component.rs\"]\nmod subject;\n",
      "🧪️consumer/🟦️component.ts": "export { value } from \"../🧪️subject/🟦️component.ts\";\n",
      "🧪️consumer/🐹️component.go": "package consumer\n//go:embed ../🧪️subject/🟦️component.ts\n",
      "🧪️consumer/🐍️component.py": "from fixture.subject.component import value\n",
      "🧪️consumer/fixture.csproj": "<Project><ItemGroup><Compile Include=\"../🧪️subject/🟦️component.ts\" /></ItemGroup></Project>\n",
      "🧪️consumer/🔣️paths.json": "{\"source\":\"../🧪️subject/🟦️component.ts\"}\n",
      "🧪️consumer/🔣️paths.jsonc": "{\"source\":\"../🧪️subject/🟦️component.ts\"}\n",
      "🧪️consumer/🔣️paths.toml": "source = \"../🧪️subject/🟦️component.ts\"\n",
      "🧪️consumer/🔣️paths.yaml": "source: ../🧪️subject/🟦️component.ts\n",
      "🧪️consumer/🔣️paths.xml": "<Source path=\"../🧪️subject/🟦️component.ts\" />\n",
      "🧪️consumer/📖️references.md": "[source](../🧪️subject/🟦️component.ts)\n",
      "🧪️consumer/CMakeLists.txt": "target_sources(fixture PRIVATE \"../🧪️subject/component.cpp\")\n",
    });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.edits.length).toBeGreaterThanOrEqual(5);
      for (const edit of plan.edits) {
        expect(edit.path.length).toBeGreaterThan(0);
        expect(edit.adapter.length).toBeGreaterThan(0);
        expect(edit.structuredLocation.length).toBeGreaterThan(0);
        expect(edit.oldValue.length).toBeGreaterThan(0);
        expect(edit.newValue.length).toBeGreaterThan(0);
        expect(edit.preimage.contentHash.length).toBeGreaterThan(0);
      }
      const adapters = plan.edits.map((edit) => edit.adapter.toLowerCase()).join(" ");
      expect(adapters).toMatch(/rust/);
      expect(adapters).toMatch(/(?:typescript|javascript|ts)/);
      expect(adapters).toMatch(/go/);
      expect(adapters).toMatch(/python/);
      expect(adapters).toMatch(/dotnet/);
      expect(adapters).toMatch(/json/);
      expect(adapters).toMatch(/jsonc/);
      expect(adapters).toMatch(/toml/);
      expect(adapters).toMatch(/yaml/);
      expect(adapters).toMatch(/xml/);
      expect(adapters).toMatch(/markdown/);
      expect(adapters).toMatch(/(?:native|cmake)/);
      expect(plan.moves.some((move) => move.referenceEdits.length > 0)).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("package boundaries accept thin Rust/TypeScript glue and reject domain implementation", () => {
    const fixture = normalizationFixture("package-glue", {
      "🧪️thin/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"thin-rust\"\nversion = \"0.0.0\"\n[lib]\npath = \"🦀️.rs\"\n",
      "🧪️thin/📦️packages/🦀️rust/🦀️.rs": "pub use crate::component::*;\n",
      "🧪️thin/📦️packages/🟦️typescript/package.json": "{\"name\":\"thin-ts\",\"exports\":{\".\":\"./🟦️.ts\"}}\n",
      "🧪️thin/📦️packages/🟦️typescript/🟦️.ts": "export {};\n",
      "🧪️thick/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"thick-rust\"\nversion = \"0.0.0\"\n[lib]\npath = \"🦀️.rs\"\n",
      "🧪️thick/📦️packages/🦀️rust/🦀️.rs": "pub struct Domain { pub value: i32 }\nimpl Domain { pub fn calculate(&self) -> i32 { self.value * 2 + 1 } }\n",
      "🧪️thick/📦️packages/🟦️typescript/package.json": "{\"name\":\"thick-ts\",\"exports\":{\".\":\"./🟦️.ts\"}}\n",
      "🧪️thick/📦️packages/🟦️typescript/🟦️.ts": "export function calculate(values: number[]): number { return values.reduce((sum, value) => sum + value, 0); }\n",
    });
    try {
      const inventory = inventoryTaxonomy(fixture.options);
      const glue = /\/📦️packages\/(?:🦀️rust\/🦀️\.rs|🟦️typescript\/🟦️\.ts)$/u;
      const thin = inventory.entries.filter((entry) => entry.sourcePath.includes("/🧪️thin/📦️packages/") && glue.test(entry.sourcePath));
      const thick = inventory.entries.filter((entry) => entry.sourcePath.includes("/🧪️thick/📦️packages/") && glue.test(entry.sourcePath));
      expect(thin).toHaveLength(2);
      expect(thick).toHaveLength(2);
      const packageViolation = (violation: { readonly code: string; readonly message: string }): boolean => /(?:package|glue|implementation)/i.test(`${violation.code} ${violation.message}`);
      expect(thin.flatMap((entry) => entry.violations).filter(packageViolation)).toEqual([]);
      expect(thin.map((entry) => entry.packageRole)).toEqual(["declaration", "declaration"]);
      expect(thick.map((entry) => entry.packageRole)).toEqual(["implementation", "implementation"]);
      expect(thick.every((entry) => entry.violations.some(packageViolation))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
}

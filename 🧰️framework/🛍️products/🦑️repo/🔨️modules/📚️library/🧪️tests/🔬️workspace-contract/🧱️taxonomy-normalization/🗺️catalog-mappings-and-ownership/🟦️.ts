/** 🗺️ The minimal CAD catalog, CAD and Draw authority mapping and unowned prose tests. */
import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, posix, resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { applyTaxonomyPlan } from "../../../../🧹️normalization/🚀️apply/🟦️.ts";
import { artifactProjectionTail } from "../../../../🧹️normalization/🪞️artifact-reference-projection/🟦️.ts";
import { DRAW_SOURCE_SCENARIO, NORMALIZATION_SCHEMA_REL, artifactProjectionFixturePath, artifactProjectionNormalizationFixture, cadProjectionLiveSources, normalizationFixture, normalizationPlan, normalizationWorkspaceSnapshot, normalizationWriteFiles, projectionByteSort, projectionGolden, retainArtifactProjectionFixture } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🧭 Registers the minimal CAD catalog commit, the authority mappings and the unowned prose, selector and counterfeit rejection tests. */
export function registerCatalogMappingAndOwnershipTests(): void {
  test("a minimal CAD catalog commits and re-inventories without source hierarchy authority", () => {
    const cad = projectionGolden("artifact-example-model-catalog-v1");
    const member = cad.mappings.find(({ sourcePath }) => sourcePath.includes("/🎬️actions/") && sourcePath.slice(cad.sourceRoot.length + 1).split("/").length === 3)!;
    const model = member.sourcePath.slice(cad.sourceRoot.length + 1).split("/")[0]!;
    const manifest = cad.mappings.find(({ sourcePath }) => sourcePath === `${cad.sourceRoot}/${model}/🔣️modelDefinition.json`)!;
    const live = cadProjectionLiveSources(cad);
    const fixture = normalizationFixture("minimal-cad-canonical", Object.fromEntries([manifest, member].map(({ sourcePath }) => [artifactProjectionFixturePath(sourcePath), readFileSync(join(getWorkspaceRoot(), live.get(sourcePath)!), "utf8")])));
    try {
      const retainedDirectory = join(fixture.workspace, dirname(artifactProjectionFixturePath(cad.sourceRoot)), "🔤️fonts");
      mkdirSync(retainedDirectory);
      const { plan } = normalizationPlan(fixture);
      expect(plan.moves).toHaveLength(2);
      expect(plan.unresolved).toEqual([]);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.root, ticketDir: fixture.ticketDir, explicitTicketDir: fixture.options.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: "after-edits" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const committed = applyTaxonomyPlan(plan, { repoRoot: fixture.root, ticketDir: fixture.ticketDir, explicitTicketDir: fixture.options.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect({ state: committed.state, error: JSON.parse(readFileSync(committed.journalPath, "utf8")).error ?? null }).toEqual({ state: "committed", error: null });
      expect(existsSync(join(fixture.workspace, artifactProjectionFixturePath(cad.sourceRoot)))).toBe(false);
      expect(lstatSync(retainedDirectory).isDirectory()).toBe(true);
      const second = normalizationPlan(fixture).plan;
      expect(second.moves).toEqual([]);
      expect(second.edits).toEqual([]);
      expect(second.unresolved).toEqual([]);
      const leaf = join(fixture.workspace, artifactProjectionFixturePath(member.destinationPath));
      const original = readFileSync(leaf, "utf8");
      writeFileSync(leaf, JSON.stringify({ id: "counterfeit", schema: "spatial.unknown", version: "1.0.0" }));
      expect(normalizationPlan(fixture).plan.unresolved.some(({ code }) => code === "projection-authority-invalid")).toBe(true);
      writeFileSync(leaf, original);
      const profilePath = artifactProjectionFixturePath(cad.destinationRoot).replace("🪆️1-any", "🪆️2-any");
      normalizationWriteFiles(fixture.workspace, { [`${profilePath}/${model}/🔣️.json`]: readFileSync(join(fixture.workspace, artifactProjectionFixturePath(manifest.destinationPath)), "utf8") });
      expect(normalizationPlan(fixture).plan.unresolved.some(({ code }) => code === "projection-destination-unregistered")).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 120_000);
  test("plans the exact CAD and Draw authority mappings with structured cross-profile references", () => {
    const fixture = artifactProjectionNormalizationFixture("plan", true);
    let passed = false;
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(plan.regenerations.map(({ contractId }) => contractId).sort()).toEqual<readonly string[]>(DRAW_SOURCE_SCENARIO.oracle.activatedGeneratorIds);
      expect(plan.edits.some(({ path }) => path === NORMALIZATION_SCHEMA_REL)).toBe(false);
      const cad = projectionGolden("artifact-example-model-catalog-v1");
      const draw = projectionGolden("artifact-editor-command-bundle-v1");
      expect(artifactProjectionTail(`${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}`)).toStartWith("🗿️artifacts/");
      const cadMoves = plan.moves.filter((move) => move.rationaleRule === cad.rationaleRule);
      const drawMoves = plan.moves.filter((move) => move.rationaleRule === draw.rationaleRule);
      expect(cadMoves).toHaveLength(cad.sourceFileCount);
      expect(drawMoves).toHaveLength(draw.sourceFileCount);
      expect(cadMoves.map(({ sourcePath, destinationPath }) => ({ sourcePath: sourcePath.slice(fixture.scope.length + 1), destinationPath: destinationPath.slice(fixture.scope.length + 1) }))).toEqual(cad.mappings.map(({ sourcePath, destinationPath }) => ({ sourcePath: artifactProjectionFixturePath(sourcePath), destinationPath: artifactProjectionFixturePath(destinationPath) })));
      expect(drawMoves.map(({ sourcePath, destinationPath }) => ({ sourcePath: sourcePath.slice(fixture.scope.length + 1), destinationPath: destinationPath.slice(fixture.scope.length + 1) }))).toEqual(draw.mappings.map(({ sourcePath, destinationPath }) => ({ sourcePath: artifactProjectionFixturePath(sourcePath), destinationPath: artifactProjectionFixturePath(destinationPath) })));
      expect(plan.unresolved.filter((entry) => /projection|collision/u.test(entry.code))).toEqual([]);
      const referenceEdits = plan.edits.filter((edit) => edit.path.includes(`/${artifactProjectionFixturePath(cad.destinationRoot)}/`) && (edit.oldValue === `${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}` || edit.oldValue === `${fixture.scope}/${artifactProjectionFixturePath(draw.mappings[0]!.sourcePath)}`));
      expect(referenceEdits).toHaveLength(2);
      expect(referenceEdits.map((edit) => edit.newValue).sort(projectionByteSort)).toEqual([
        `${fixture.scope}/${artifactProjectionFixturePath(cad.destinationRoot)}`,
        `${fixture.scope}/${artifactProjectionFixturePath(draw.mappings[0]!.destinationPath)}`,
      ].sort(projectionByteSort));
      const locations = plan.edits.map((edit) => edit.structuredLocation);
      expect(locations.filter((location) => location.startsWith("artifact-catalog-glob:"))).toHaveLength(10);
      expect(locations.filter((location) => location.startsWith("artifact-catalog-comment:"))).toHaveLength(3);
      expect(locations.filter((location) => location.startsWith("artifact-catalog-marker:"))).toHaveLength(1);
      expect(locations.filter((location) => location.includes("/workspace-glob@"))).toHaveLength(2);
      expect(locations.filter((location) => location.startsWith("path-collection:"))).toHaveLength(1);
      const cadArtifactRoot = cad.sourceRoot.slice(0, cad.sourceRoot.indexOf("/🏅️standards/"));
      expect(plan.edits.filter((edit) => edit.structuredLocation.startsWith("artifact-catalog-root-join:")).map(({ oldValue, newValue }) => ({ oldValue, newValue }))).toEqual([{ oldValue: `../../${posix.relative(cadArtifactRoot, cad.sourceRoot)}`, newValue: `../../${posix.relative(cadArtifactRoot, cad.destinationRoot)}` }]);
      const removedSelectors = plan.edits.filter((edit) => edit.structuredLocation.startsWith("artifact-catalog-glob:") && edit.newValue === "");
      expect(removedSelectors).toHaveLength(2);
      expect(removedSelectors.reduce((count, edit) => count + (edit.oldValue.match(/modelDefinitions/g)?.length ?? 0), 0)).toBe(3);
      const cadExact = plan.edits.filter((edit) => edit.adapter === "rust" && edit.structuredLocation.startsWith("rust-string-path:") && /modelDefinitions\/.+\.json$/u.test(edit.oldValue));
      expect(cadExact).toHaveLength(72);
      for (const edit of cadExact) {
        const matches = cad.mappings.filter((mapping) => edit.oldValue.endsWith(mapping.sourcePath.slice(cad.sourceRoot.length + 1)) && edit.newValue.endsWith(mapping.destinationPath.slice(cad.destinationRoot.length + 1)));
        expect(matches).toHaveLength(1);
      }
      const cadMount = plan.edits.filter(({ path }) => path === `${fixture.scope}/${DRAW_SOURCE_SCENARIO.cargoModuleRoot.path}`);
      expect(cadMount).toEqual([]);
      const drawExact = plan.edits.filter((edit) => (edit.oldValue.includes(`${fixture.scope}/${artifactProjectionFixturePath(draw.sourceRoot)}`) || edit.path.startsWith(`${fixture.scope}/📦️packages/🦀️rust/`) || edit.path.includes("/🧪️tests/🧪️reference/")) && edit.path !== `${fixture.scope}/${DRAW_SOURCE_SCENARIO.cargoModuleRoot.path}` && edit.oldValue !== "🦀️.rs" && !edit.oldValue.includes("workspaceRoot") && !edit.structuredLocation.startsWith("path-collection:") && !edit.path.includes(`/${artifactProjectionFixturePath(cad.destinationRoot)}/`));
      expect(drawExact).toHaveLength(20);
      expect(drawExact.some((edit) => edit.path.endsWith("📦️packages/🦀️rust/Cargo.toml") && edit.newValue === `../../${artifactProjectionFixturePath(draw.destinationRoot)}/🔄️fsm/📦️packages/🦀️rust`)).toBe(true);
      expect(drawExact.some((edit) => edit.path.endsWith("🧪️tests/🧪️reference/🦀️.rs") && edit.newValue === `../../${artifactProjectionFixturePath(draw.destinationRoot)}/🦀️.rs`)).toBe(true);
      expect(plan.edits.filter((edit) => edit.structuredLocation.startsWith("lib.path") && edit.path.includes(artifactProjectionFixturePath(draw.destinationRoot)))).toEqual([]);
      expect(plan.unresolved.filter((entry) => /projection|reference/u.test(entry.code))).toEqual([]);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 60_000);
  test("rejects unowned artifact prose, unmatched selectors, escaped placeholders, and counterfeit owners", () => {
    const fixture = artifactProjectionNormalizationFixture("negative", true);
    let passed = false;
    try {
      expect(normalizationPlan(fixture).plan.unresolved).toEqual([]);
      const cad = projectionGolden("artifact-example-model-catalog-v1");
      const draw = projectionGolden("artifact-editor-command-bundle-v1");
      const unowned = join(fixture.workspace, "✏️s", "🔨️modules", "🧪️unowned", "🟦️.ts");
      mkdirSync(resolve(unowned, ".."), { recursive: true });
      const copiedSelector = `${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}/**/🗂️typologies/**/🔣️typology.json`;
      writeFileSync(unowned, `export const unrelated = "🖼️assets/🏗️modelDefinitions/";\nexport const counterfeit = import.meta.glob(${JSON.stringify(copiedSelector)});\n`);
      const schemaPath = join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL);
      const schema = JSON.parse(readFileSync(schemaPath, "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { sourcePathPattern: string }> };
      schema.semanticPathProjectionReferenceConsumerContracts["cad-spatial-kernel-geometry"]!.sourcePathPattern = "^.*\\.ts$";
      writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
      const runtime = join(fixture.workspace, artifactProjectionFixturePath("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts"));
      writeFileSync(runtime, readFileSync(runtime, "utf8").replace("/🗂️typologies/**/🔣️typology.json", "/🧪️unregistered/**/*.json"));
      const projectMapping = draw.mappings.find(({ sourcePath }) => sourcePath.endsWith("📋️project.json"))!;
      const project = join(fixture.workspace, artifactProjectionFixturePath(projectMapping.sourcePath));
      writeFileSync(project, readFileSync(project, "utf8").replace("{workspaceRoot}", "\\u007bworkspaceRoot\\u007d"));
      const counterfeit = join(fixture.workspace, "🧪️counterfeit", cad.sourceRoot.slice(cad.sourceRoot.lastIndexOf("/📐️cad/") + 1), "🧪️model", "🔣️modelDefinition.json");
      mkdirSync(resolve(counterfeit, ".."), { recursive: true });
      writeFileSync(counterfeit, JSON.stringify({ id: "counterfeit", schema: "model-definition", version: "1.0.0" }));
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("outside an authorized owner"))).toBe(true);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("selector or reference file"))).toBe(true);
      expect(plan.edits.some((entry) => entry.path.endsWith("🟦️unowned.ts"))).toBe(false);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("Nonempty artifact selector"))).toBe(true);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("Escaped workspace projection glob"))).toBe(true);
      expect(plan.moves.some((move) => move.sourcePath.includes("🧪️counterfeit") && /artifact-(?:example|editor)/u.test(move.rationaleRule))).toBe(false);
      expect(plan.unresolved.some((entry) => entry.path.includes("🧪️counterfeit") && entry.code === "projection-authority-invalid")).toBe(false);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 60_000);
}

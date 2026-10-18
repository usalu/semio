import { testLevelAtLeast } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { describe, expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, lstatSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { applyTaxonomyPlan } from "../../../🧹️normalization/🚀️apply/🟦️.ts";
import { artifactProjectionTail } from "../../../🧹️normalization/🪞️artifact-reference-projection/🟦️.ts";
import { inventoryTaxonomy } from "../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { noFollowTreeDigest, opaqueTreeDigest } from "../../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { planTaxonomy } from "../../../🧹️normalization/🧠️planning/🟦️.ts";
import { taxonomyPlanDigest } from "../../../🧹️normalization/🗃️affected-state/🟦️.ts";
import { verifyTaxonomy } from "../../../🧹️normalization/✅️verification/🟦️.ts";
import { type TaxonomyInventory, type TaxonomyInventoryOptions } from "../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { taxonomyScopedGitPathspec } from "../../../🧹️normalization/🚪️source-admission/🟦️.ts";
import { type TaxonomyProgress } from "../../../🧹️normalization/🏃️operation/🟦️.ts";
import { ownedFilePaths, ownedPathByteSort } from "../../🔍️filesystem/🟦️.ts";
import { ARTIFACT_PROJECTION_GOLDEN, DRAW_SOURCE_SCENARIO, NORMALIZATION_SCHEMA_REL, SCOPED_INVENTORY_PATHSPEC_GOLDEN, TICKET_IMPORTANT_HISTORY_DESTINATION_REL, TICKET_IMPORTANT_HISTORY_SOURCE_REL, artifactProjectionApply, artifactProjectionFixturePath, artifactProjectionIgnoreResidue, artifactProjectionNormalizationFixture, cadProjectionLiveSources, drawScenarioTaxonomy, expectNormalizationApplyFailure, inScopeForTest, normalizationFixture, normalizationGit, normalizationInventoryCensus, normalizationMove, normalizationPhaseSequence, normalizationPlan, normalizationWorkspaceSnapshot, normalizationWriteFiles, projectionByteSort, projectionGolden, retainArtifactProjectionFixture, scopedUnrelatedProjectionStaleFixture } from "../🏗️shared-fixtures/🟦️.ts";

describe.if(testLevelAtLeast("long"))("taxonomy normalization", () => {
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

  test("rolls back and atomically applies CAD and Draw projections to an empty second plan", () => {
    const fixture = artifactProjectionNormalizationFixture("apply", true);
    let passed = false;
    try {
      const sourceRoots = ARTIFACT_PROJECTION_GOLDEN.projections.map(({ sourceRoot }) => artifactProjectionFixturePath(sourceRoot));
      const retainedRelative = `${dirname(sourceRoots[0]!)}/🔤️fonts`;
      const retainedDirectory = join(fixture.workspace, retainedRelative);
      mkdirSync(retainedDirectory);
      if (process.platform !== "win32") for (const path of [...sourceRoots.map((path) => join(fixture.workspace, path)), retainedDirectory]) chmodSync(path, 0o750);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const plan = normalizationPlan(fixture).plan;
      expect(plan.unresolved).toEqual([]);
      const projectedNodes = new Set(Object.keys(before));
      const candidateDirectories = new Set<string>();
      for (const move of plan.moves) {
        const source = move.sourcePath.slice(fixture.scope.length + 1), destination = move.destinationPath.slice(fixture.scope.length + 1);
        projectedNodes.delete(source);
        projectedNodes.add(destination);
        for (let path = dirname(source); path !== "."; path = dirname(path)) candidateDirectories.add(path);
        for (let path = dirname(destination); path !== "."; path = dirname(path)) projectedNodes.add(path);
      }
      const expectedPruned: string[] = [];
      for (const path of [...candidateDirectories].sort((left, right) => right.split("/").length - left.split("/").length)) if (projectedNodes.has(path) && ![...projectedNodes].some((candidate) => candidate.startsWith(`${path}/`))) {
        projectedNodes.delete(path);
        expectedPruned.push(path);
      }
      const rolledBack = artifactProjectionApply(fixture, plan, { injectFailureAt: "after-edits" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const cancelFile = join(fixture.ticketDir, "🛑️cancel");
      writeFileSync(cancelFile, "cancel\n");
      expectNormalizationApplyFailure(() => artifactProjectionApply(fixture, plan, { cancelFile }));
      rmSync(cancelFile);
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const applied = artifactProjectionApply(fixture, plan);
      expect({ state: applied.state, error: JSON.parse(readFileSync(applied.journalPath, "utf8")).error ?? null }).toEqual({ state: "committed", error: null });
      expect(JSON.parse(readFileSync(applied.journalPath, "utf8")).attemptOrdinal).toBe("000002");
      for (const path of sourceRoots) expect(existsSync(join(fixture.workspace, path))).toBe(false);
      const after = normalizationWorkspaceSnapshot(fixture.workspace);
      expect(Object.entries(before).filter(([path, entry]) => entry.startsWith("directory|") && !after[path]).map(([path]) => path).sort(projectionByteSort)).toEqual(expectedPruned.sort(projectionByteSort));
      expect(fastGlob.sync("**/*", { cwd: fixture.workspace, onlyDirectories: true, dot: true, followSymbolicLinks: false }).sort(projectionByteSort)).toEqual(Object.entries(after).filter(([path, entry]) => path !== "." && entry.startsWith("directory|")).map(([path]) => path).sort(projectionByteSort));
      expect(after[retainedRelative]).toBe(before[retainedRelative]);
      for (const [path, entry] of Object.entries(before).filter(([, entry]) => entry.startsWith("directory|"))) if (after[path]) expect(after[path]).toBe(entry);
      expect(lstatSync(fixture.ticketDir).isDirectory()).toBe(true);
      expect(artifactProjectionApply(fixture, plan, { resumeJournal: applied.journalPath }).state).toBe("committed");
      for (const edit of plan.edits.filter((candidate) => /artifact-catalog|workspace-glob|path-collection/u.test(candidate.structuredLocation))) expect(readFileSync(join(fixture.repoRoot, edit.path), "utf8")).not.toContain(edit.oldValue);
      const second = normalizationPlan(fixture).plan;
      expect(second.unresolved).toEqual([]);
      expect(second.moves.filter((move) => move.rationaleRule === "artifact-example-model-catalog-projection-v1" || move.rationaleRule === "artifact-editor-command-projection-v1")).toEqual([]);
      expect(second.edits.filter((edit) => /artifact-catalog|workspace-glob|path-collection/u.test(edit.structuredLocation))).toEqual([]);
      expect(second.unresolved.filter((entry) => /projection/u.test(entry.code))).toEqual([]);
      const taxonomy = JSON.parse(readFileSync(join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL), "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { projectionContractId: string; staleMarkers: string[] }> };
      const marker = (projectionContractId: string): string => Object.values(taxonomy.semanticPathProjectionReferenceConsumerContracts).find((contract) => contract.projectionContractId === projectionContractId)!.staleMarkers[0]!;
      const cadConsumer = plan.edits.find((edit) => edit.structuredLocation.startsWith("artifact-catalog-marker:"))!.path;
      writeFileSync(join(fixture.repoRoot, cadConsumer), `${readFileSync(join(fixture.repoRoot, cadConsumer), "utf8")}\nexport const staleCadRoot = ${JSON.stringify(marker("artifact-example-model-catalog-v1"))};\n`);
      const drawPackageConsumers = plan.moves.filter((move) => move.destinationPath.includes(artifactProjectionFixturePath(projectionGolden("artifact-editor-command-bundle-v1").destinationRoot)) && move.destinationPath.endsWith("/📦️packages/🦀️rust/Cargo.toml")).map((move) => move.destinationPath);
      expect(drawPackageConsumers).toHaveLength(2);
      for (const path of drawPackageConsumers) writeFileSync(join(fixture.repoRoot, path), `${readFileSync(join(fixture.repoRoot, path), "utf8")}\n# ${marker("artifact-editor-command-bundle-v1")}\n`);
      const stale = normalizationPlan(fixture).plan;
      expect(stale.moves.filter((move) => move.rationaleRule === "artifact-example-model-catalog-projection-v1" || move.rationaleRule === "artifact-editor-command-projection-v1")).toEqual([]);
      expect(stale.unresolved.filter((entry) => entry.code === "projection-old-token-stale")).toHaveLength(3);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 120_000);

  test("rejects ignored and unplanned residual children in a projected source owner without following links", () => {
    const fixture = artifactProjectionNormalizationFixture("source-residue", true);
    let passed = false;
    try {
      const sourceRoot = join(fixture.workspace, artifactProjectionFixturePath(projectionGolden("artifact-example-model-catalog-v1").sourceRoot));
      const residue = join(sourceRoot, "🧪️unexpected");
      const linkTarget = join(fixture.ticketDir, "🧪️link-target");
      mkdirSync(linkTarget);
      writeFileSync(join(linkTarget, "📝️.md"), "not followed\n");
      artifactProjectionIgnoreResidue(fixture, residue);
      normalizationGit(fixture.root, ["check-ignore", "--quiet", "--no-index", "--", relative(fixture.root, join(residue, "📝️.md")).replaceAll("\\", "/")]);
      const { plan } = normalizationPlan(fixture);
      expect(plan.moves.length).toBeGreaterThan(0);
      expect(plan.unresolved).toEqual([]);
      expect(plan.regenerations.map(({ contractId }) => contractId)).toEqual(["plugin-registry"]);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const targetBefore = normalizationWorkspaceSnapshot(linkTarget);
      const outputsBefore = plan.regenerations.flatMap(({ outputRoots }) => outputRoots.map((path) => noFollowTreeDigest(fixture.repoRoot, path)));
      const started = new Set<string>();
      let injected = false, journalSnapshots = 0;
      const applied = artifactProjectionApply(fixture, plan, { progress: (event) => {
        if (event.phase === "transaction-journal-candidate-written" && event.path) {
          const snapshot = JSON.parse(readFileSync(resolve(fixture.repoRoot, event.path), "utf8"));
          if (snapshot.planDigest === plan.planDigest) {
            journalSnapshots++;
            for (const id of snapshot.startedRegenerationIds) started.add(id);
          }
        }
        if (event.phase !== "moves" || event.current !== event.total || injected) return;
        injected = true;
        mkdirSync(join(residue, "🧪️empty"), { recursive: true });
        writeFileSync(join(residue, "📝️.md"), "ignored residual source\n");
        symlinkSync(process.platform === "win32" ? linkTarget : relative(residue, linkTarget), join(residue, "🧪️link"), process.platform === "win32" ? "junction" : "dir");
      } });
      expect(injected).toBe(true);
      normalizationGit(fixture.root, ["check-ignore", "--quiet", "--", relative(fixture.root, join(residue, "📝️.md")).replaceAll("\\", "/")]);
      const residueRelative = relative(fixture.repoRoot, residue).replaceAll("\\", "/");
      expect({ state: applied.state, error: JSON.parse(readFileSync(applied.journalPath, "utf8")).error }).toEqual({ state: "rolled-back", error: `resume-state-drift: regeneration input membership ${plan.regenerations[0]!.id}; missing(0)=[]; added(2)=${JSON.stringify([residueRelative, `${residueRelative}/🧪️empty`])}` });
      expect(journalSnapshots).toBeGreaterThan(0);
      expect([...started]).toEqual([]);
      expect(plan.regenerations.flatMap(({ outputRoots }) => outputRoots.map((path) => noFollowTreeDigest(fixture.repoRoot, path)))).toEqual(outputsBefore);
      expect(readFileSync(join(residue, "📝️.md"), "utf8")).toBe("ignored residual source\n");
      expect(lstatSync(join(residue, "🧪️empty")).isDirectory()).toBe(true);
      expect(lstatSync(join(residue, "🧪️link")).isSymbolicLink()).toBe(true);
      expect(normalizationWorkspaceSnapshot(linkTarget)).toEqual(targetBefore);
      const after = normalizationWorkspaceSnapshot(fixture.workspace);
      for (const [path, entry] of Object.entries(before)) expect(after[path]).toBe(entry);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 120_000);

  test("rederives repeatable CAD and Draw source-tree authority before apply", () => {
    const fixture = artifactProjectionNormalizationFixture("source-tree-repeatability", true);
    let passed = false;
    try {
      const first = normalizationPlan(fixture);
      const second = normalizationPlan(fixture);
      expect(first.plan.unresolved).toEqual([]);
      expect(second.plan.unresolved).toEqual([]);
      const sourceRows = (inventory: TaxonomyInventory) => inventory.entries.map(({ sourcePath, nodeKind, contentHash, mode, size, symlinkTarget }) => ({ sourcePath, nodeKind, contentHash, mode, size, symlinkTarget }));
      expect(sourceRows(second.inventory)).toEqual(sourceRows(first.inventory));
      expect(second.plan.sourceTreeDigest).toBe(first.plan.sourceTreeDigest);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 120_000);

  test("rejects a changed explicit source-admission ticket before journal mutation", () => {
    const fixture = normalizationFixture("source-authority-ticket", { "🧪️subject/🦀️.rs": "pub const VALUE: i32 = 1;\n" }, ({ repoRoot, workspace }) => writeFileSync(join(repoRoot, ".gitignore"), `${relative(repoRoot, workspace).replaceAll("\\", "/")}/\n`));
    try {
      const { inventory, plan } = normalizationPlan(fixture);
      expect(plan.unresolved.filter(({ severity }) => severity === "error")).toEqual([]);
      const transactionTicket = join(fixture.ticketDir, "🧪️transaction-evidence");
      mkdirSync(transactionTicket, { recursive: true });
      const changedAdmission = inventoryTaxonomy({ ...fixture.options, ticketDir: transactionTicket });
      expect(inventory.entries.map(({ sourcePath }) => sourcePath)).toEqual(["🧪️tests", fixture.scope, `${fixture.scope}/🧪️subject`, `${fixture.scope}/🧪️subject/🦀️.rs`]);
      expect(changedAdmission.entries).toEqual([]);
      expect(changedAdmission.sourceTreeDigest).not.toBe(plan.sourceTreeDigest);
      const before = normalizationWorkspaceSnapshot(fixture.ticketDir);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: transactionTicket, explicitTicketDir: transactionTicket, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("Plan source-tree digest cannot be rederived exactly");
      expect(normalizationWorkspaceSnapshot(fixture.ticketDir)).toEqual(before);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("rejects changed and newly admitted in-scope sources before journal mutation", () => {
    const stableRelative = "🧪️stable/📝️.md";
    const fixture = normalizationFixture("source-authority-drift", { "🧪️subject/🦀️.rs": "pub const VALUE: i32 = 1;\n", [stableRelative]: "original\n" }, ({ repoRoot, workspace }) => writeFileSync(join(repoRoot, ".gitignore"), `${relative(repoRoot, workspace).replaceAll("\\", "/")}/\n`));
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved.filter(({ severity }) => severity === "error")).toEqual([]);
      expect(plan.moves.some(({ sourcePath }) => sourcePath === `${fixture.scope}/${stableRelative}`)).toBe(false);
      writeFileSync(join(fixture.workspace, stableRelative), "changed physical source\n");
      const changedBefore = normalizationWorkspaceSnapshot(fixture.ticketDir);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, explicitTicketDir: fixture.options.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("Plan source-tree digest cannot be rederived exactly");
      expect(normalizationWorkspaceSnapshot(fixture.ticketDir)).toEqual(changedBefore);
      writeFileSync(join(fixture.workspace, stableRelative), "original\n");
      normalizationWriteFiles(fixture.workspace, { "🧪️in-scope-extra/📝️.md": "new ignored but explicitly admitted source\n" });
      const extraBefore = normalizationWorkspaceSnapshot(fixture.ticketDir);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, explicitTicketDir: fixture.options.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("Plan source-tree digest cannot be rederived exactly");
      expect(normalizationWorkspaceSnapshot(fixture.ticketDir)).toEqual(extraBefore);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("canonical JSON recursively sorts keys and contract-identified arrays", () => {
    const left = { z: 1, rows: [{ id: "b", value: 2 }, { id: "a", value: 1 }], nested: { z: false, a: true } };
    const right = { nested: { a: true, z: false }, rows: [{ value: 1, id: "a" }, { value: 2, id: "b" }], z: 1 };
    expect(canonicalJson(left)).toBe(canonicalJson(right));
    expect(canonicalJson(left)).toBe(canonicalJson(JSON.parse(canonicalJson(left))));
  });

  test("normalization rejects malformed projection consumers and descendant nodes at its own schema boundary", () => {
    const fixture = normalizationFixture("projection-schema-boundary", { "🧪️subject/🟦️.ts": "export const value = 1;\n" });
    try {
      const schemaPath = join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL);
      const original = JSON.parse(readFileSync(schemaPath, "utf8")) as { semanticDirectoryKinds: Record<string, unknown>; semanticPathProjectionReferenceConsumerContracts: Record<string, Record<string, unknown>>; semanticDescendantContracts: Record<string, { requiredNodes: Record<string, unknown>[] }> };
      const consumer = structuredClone(original);
      consumer.semanticPathProjectionReferenceConsumerContracts["cad-spatial-kernel-geometry"]!.consumerIdentity = "counterfeit";
      writeFileSync(schemaPath, `${JSON.stringify(consumer, null, 2)}\n`);
      expect(() => inventoryTaxonomy(fixture.options)).toThrow("discovery contract validation failed");
      const descendant = structuredClone(original);
      const nodes = descendant.semanticDescendantContracts["draw-editor-command-bundle-v1"]!.requiredNodes;
      const library = nodes.find((node) => node.nodeType === "file" && node.kindId === "rust-source" && (node.pathSegments as unknown[]).length === 0)!;
      writeFileSync(schemaPath, `${JSON.stringify(descendant, null, 2)}\n`);
      expect(inventoryTaxonomy(fixture.options).entries.some(({ sourcePath }) => sourcePath === `${fixture.scope}/🧪️subject/🟦️.ts`)).toBe(true);
      library.compatibilityAlias = true;
      writeFileSync(schemaPath, `${JSON.stringify(descendant, null, 2)}\n`);
      expect(() => inventoryTaxonomy(fixture.options)).toThrow("discovery contract validation failed");
      const packaged = drawScenarioTaxonomy(structuredClone(original));
      writeFileSync(schemaPath, `${JSON.stringify(packaged, null, 2)}\n`);
      expect(inventoryTaxonomy(fixture.options).entries.some(({ sourcePath }) => sourcePath === `${fixture.scope}/🧪️subject/🟦️.ts`)).toBe(true);
      const unnamed = structuredClone(packaged);
      const nested = unnamed.semanticDescendantContracts["draw-editor-command-bundle-v1"]!.requiredNodes.find((node) => node.nodeType === "file" && node.kindId === "rust-source" && (node.pathSegments as unknown[]).length === 2)!;
      delete nested.sourceFilename;
      writeFileSync(schemaPath, `${JSON.stringify(unnamed, null, 2)}\n`);
      expect(() => inventoryTaxonomy(fixture.options)).toThrow(/discovery contract validation failed|invalid source-filename descendant authority/u);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("opaque-tree digest hashes symlink identity without reading its target", () => {
    const fixture = normalizationFixture(
      "opaque",
      { "🟦️.ts": "export const value = 1;\n" },
      ({ repoRoot }) => {
        mkdirSync(join(repoRoot, "outside"), { recursive: true });
        writeFileSync(join(repoRoot, "outside", "target.txt"), "first\n");
        symlinkSync("../../📦️packages/outside/target.txt", join(repoRoot, "compose", "target-link"), "file");
      },
    );
    try {
      const before = opaqueTreeDigest(fixture.repoRoot, "compose");
      writeFileSync(join(fixture.repoRoot, "outside", "target.txt"), "second and deliberately different\n");
      const after = opaqueTreeDigest(fixture.repoRoot, "compose");
      expect(after).toEqual(before);
      expect(before.algorithm).toBe("sha256-merkle-v1");
      expect(before.relativeRoot).toBe("compose");
      expect(before.files).toBe(1);
      expect(before.symlinks).toBe(1);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("an absent opaque prefix remains lexically excluded without requiring an impossible digest", () => {
    const fixture = normalizationFixture("absent-opaque", { "🟦️.ts": "export const value = 1;\n" });
    try {
      rmSync(join(fixture.repoRoot, "compose"), { recursive: true, force: true });
      const inventory = inventoryTaxonomy(fixture.options);
      expect(inventory.pathExclusions).toContain("compose");
      expect(inventory.entries.some((entry) => entry.sourcePath === "compose" || entry.sourcePath.startsWith("compose/"))).toBe(false);
      const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(plan.unresolved.some((violation) => violation.code === "opaque-digest-missing")).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("scoped Git pathspec rendering matches the language-neutral golden", () => {
    expect(SCOPED_INVENTORY_PATHSPEC_GOLDEN.schemaVersion).toBe(1);
    for (const row of SCOPED_INVENTORY_PATHSPEC_GOLDEN.cases) {
      expect(taxonomyScopedGitPathspec(row.inputScope, SCOPED_INVENTORY_PATHSPEC_GOLDEN.opaqueExclusions)).toEqual({
        normalizedScope: row.normalizedScope,
        conservativePrefix: row.conservativePrefix,
        positivePathspec: row.positivePathspec,
        exclusionPathspecs: row.exclusionPathspecs,
      });
    }
  });

  test("scoped inventory preserves literal Unicode scope, third-party census, and closed phases", () => {
    const literalRoot = "🧪️literal [x]*?!:";
    const decomposedRoot = "🧪️cafe\u0301";
    const fixture = normalizationFixture("scoped-pathspec", {
      ".gitignore": "ignored/\n",
      "🟦️.ts": "export const tracked = true;\n",
      [`${literalRoot}/🟦️.ts`]: "export const literal = true;\n",
      [`${decomposedRoot}/🟦️.ts`]: "export const unicode = true;\n",
    });
    try {
      normalizationWriteFiles(fixture.workspace, {
        "🧪️untracked/🟦️.ts": "export const untracked = true;\n",
        "ignored/🟦️.ts": "export const ignored = true;\n",
      });
      normalizationWriteFiles(fixture.repoRoot, { "🧪️unrelated/🟦️.ts": "export const unrelated = true;\n" });
      const events: TaxonomyProgress[] = [];
      const scopedOptions: TaxonomyInventoryOptions = fixture.options;
      const inventory = inventoryTaxonomy({ ...scopedOptions, progress: (event) => events.push(event) });
      const files = inventory.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath);
      expect(files).toContain(`${fixture.scope}/🧪️untracked/🟦️.ts`);
      expect(files.some((path) => path.startsWith("🧪️unrelated/"))).toBe(false);
      const thirdParty = fastGlob.sync("**/*", { cwd: fixture.workspace, dot: true, followSymbolicLinks: false, onlyFiles: true })
        .map((path) => `${fixture.scope}/${path.replaceAll("\\", "/")}`)
        .sort(ownedPathByteSort);
      expect([...files].sort(ownedPathByteSort)).toEqual(thirdParty);
      const phases = events.map((event) => event.phase).filter((phase, index, rows) => index === 0 || phase !== rows[index - 1]);
      expect(phases).toEqual(["tracked-enumeration", "setup", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission", "source-observation", "directories", "files", "references", "complete"]);
      for (const phase of ["tracked-enumeration", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission"]) expect(events.filter((event) => event.phase === phase).map((event) => [event.current, event.total])).toEqual([[0, 1], [1, 1]]);
      for (const phase of ["directories", "files", "references"]) expect(events.find((event) => event.phase === phase)?.current).toBe(0);
      const unicode = inventoryTaxonomy({ ...scopedOptions, scope: `${fixture.scope}/🧪️café` });
      expect(unicode.entries.some((entry) => entry.sourcePath === `${fixture.scope}/${decomposedRoot}/🟦️.ts`)).toBe(true);
      const literal = inventoryTaxonomy({ ...scopedOptions, scope: `${fixture.scope}/${literalRoot}` });
      expect(literal.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath)).toEqual([`${fixture.scope}/${literalRoot}/🟦️.ts`]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("an exact symlink leaf is inventoried without following target content", () => {
    const fixture = normalizationFixture("scoped-symlink-leaf", { "🧪️target/📝️.md": "first\n" }, ({ workspace }) => symlinkSync("🧪️target/📝️.md", join(workspace, "🧪️link"), "file"));
    try {
      const scope = `${fixture.scope}/🧪️link`;
      normalizationGit(fixture.repoRoot, ["add", "-f", "--", scope]);
      expect(lstatSync(join(fixture.workspace, "🧪️link")).isSymbolicLink()).toBe(true);
      const first = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const link = first.entries.find(({ sourcePath }) => sourcePath === scope);
      expect(link?.nodeKind).toBe("symlink");
      expect(link?.symlinkTarget).toBe("🧪️target/📝️.md");
      writeFileSync(join(fixture.workspace, "🧪️target", "📝️.md"), "second and deliberately different\n");
      const second = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      expect(second.entries.find(({ sourcePath }) => sourcePath === scope)?.contentHash).toBe(link?.contentHash);
      expect(second.entries.some(({ sourcePath }) => sourcePath.startsWith(`${scope}/`))).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("a scope below a symlink ancestor falls back to full Git enumeration and preserves the ancestor-only census", () => {
    const fixture = normalizationFixture("scoped-symlink-ancestor", { "🧪️seed/📝️.md": "seed\n" }, ({ repoRoot, workspace }) => {
      const target = join(repoRoot, "🧪️outside-target");
      normalizationWriteFiles(target, { "🧪️child/📝️.md": "must not be followed\n" });
      symlinkSync(process.platform === "win32" ? target : relative(workspace, target), join(workspace, "🧪️alias"), process.platform === "win32" ? "junction" : "dir");
    });
    try {
      const scope = `${fixture.scope}/🧪️alias/🧪️child`;
      normalizationGit(fixture.repoRoot, ["add", "-f", "--", `${fixture.scope}/🧪️alias`]);
      const scoped = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const full = inventoryTaxonomy({ repoRoot: fixture.repoRoot, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.filter(({ nodeKind }) => nodeKind !== "directory").map(({ sourcePath, nodeKind }) => ({ sourcePath, nodeKind }))).toEqual([{ sourcePath: `${fixture.scope}/🧪️alias`, nodeKind: "symlink" }]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("an exact mode-160000 ancestor refuses scoped admission without reading nested-repository content", () => {
    const fixture = normalizationFixture("scoped-gitlink-ancestor", { "🧪️seed/📝️.md": "seed\n" });
    const gitlinkRelative = `${fixture.scope}/🧪️gitlink`;
    const gitlink = join(fixture.workspace, "🧪️gitlink");
    try {
      mkdirSync(gitlink, { recursive: true });
      normalizationGit(gitlink, ["init", "--quiet"]);
      normalizationWriteFiles(gitlink, { "🧪️child/📝️.md": "nested sentinel\n" });
      normalizationGit(fixture.repoRoot, ["update-index", "--add", "--cacheinfo", `160000,${fixture.baselineCommit},${gitlinkRelative}`]);
      const reads: string[] = [];
      expect(() => inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope: `${gitlinkRelative}/🧪️child`, workers: 1, progress: (event) => { if (event.path) reads.push(event.path); } })).toThrow("Source admission scope crosses an index-owned repository boundary");
      expect(reads.some((path) => path.startsWith(`${gitlinkRelative}/`) && path !== `${gitlinkRelative}/🧪️child`)).toBe(false);
      expect(() => inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope: gitlinkRelative, workers: 1 })).toThrow("explicit repository-boundary decision");
      expect(() => inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope: fixture.scope, workers: 1 })).toThrow("explicit repository-boundary decision");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("standalone ignored-generator admission preserves scoped versus full census parity", () => {
    const fixture = normalizationFixture("scoped-generator-admission", { "🧪️generator/🟦️.ts": "export const input = true;\n" }, ({ repoRoot, workspace }) => {
      const owner = relative(repoRoot, join(workspace, "🧪️generator")).replaceAll("\\", "/");
      const outputRoot = `${owner}/🤖️generated`;
      const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
      const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as { generatorContracts: Record<string, unknown> };
      taxonomy.generatorContracts = Object.fromEntries(Object.entries({ ...taxonomy.generatorContracts, "fixture-scoped-generator": { ownership: "owned", ownerPath: owner, target: "@fixture/scoped:generate", previewTarget: "@fixture/scoped:preview-generated", checkTarget: "@fixture/scoped:check", inputPatterns: [`${owner}/🟦️.ts`], outputRoots: [{ path: outputRoot, inclusion: "ignored" }], reason: "Scoped inventory admission fixture" } }).sort(([left], [right]) => left.localeCompare(right)));
      writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
      writeFileSync(join(repoRoot, ".gitignore"), `${outputRoot}\n`);
      normalizationWriteFiles(repoRoot, { [`${outputRoot}/📝️.md`]: "owned ignored output\n" });
    });
    try {
      const scope = `${fixture.scope}/🧪️generator/🤖️generated`;
      const scoped = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const full = inventoryTaxonomy({ repoRoot: fixture.repoRoot, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("standalone explicit-ticket admission preserves scoped versus full census parity", () => {
    const fixture = normalizationFixture("scoped-ticket-admission", { ".gitignore": "🧪️explicit/\n", "🟦️.ts": "export const input = true;\n" }, ({ workspace }) => normalizationWriteFiles(workspace, { "🧪️explicit/📝️.md": "explicit ignored evidence\n" }));
    try {
      const scope = `${fixture.scope}/🧪️explicit`;
      const scoped = inventoryTaxonomy({ ...fixture.options, scope });
      const full = inventoryTaxonomy({ ...fixture.options, scope: undefined });
      const withoutTicketAuthority = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(true);
      expect(withoutTicketAuthority.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("inventory cancellation is observed from every nonterminal frozen phase boundary", () => {
    const fixture = normalizationFixture("scoped-phase-cancellation", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    const cancelFile = join(fixture.root, "🛑️cancel");
    try {
      for (const phase of ["tracked-enumeration", "setup", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission", "source-observation", "directories", "files", "references"]) {
        rmSync(cancelFile, { force: true });
        expect(() => inventoryTaxonomy({ ...fixture.options, cancelFile, progress: (event) => { if (event.phase === phase && event.current === 0) writeFileSync(cancelFile, `${phase}\n`); } })).toThrow("Taxonomy operation cancelled");
      }
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("unscoped inventory retains the frozen Git pathspec and canonical inventory bytes", () => {
    const fixture = normalizationFixture("unscoped-regression", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    try {
      const options: TaxonomyInventoryOptions = { repoRoot: fixture.repoRoot, workers: 1 };
      const first = inventoryTaxonomy(options);
      const second = inventoryTaxonomy(options);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(taxonomyScopedGitPathspec(undefined, SCOPED_INVENTORY_PATHSPEC_GOLDEN.opaqueExclusions)).toEqual({ normalizedScope: null, conservativePrefix: ".", positivePathspec: ".", exclusionPathspecs: [":(exclude,top,literal)compose", ":(exclude,top,literal)temp/compose"] });
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("reversed creation order preserves scoped canonical bytes and phase sequence", () => {
    const fixture = normalizationFixture("scoped-order", { "🧪️seed/🟦️.ts": "export const seed = true;\n" });
    const orderRoot = join(fixture.workspace, "🧪️order");
    try {
      normalizationWriteFiles(orderRoot, { "🧪️a/📝️.md": "a\n", "🧪️b/📝️.md": "b\n" });
      const firstEvents: TaxonomyProgress[] = [];
      const first = inventoryTaxonomy({ ...fixture.options, scope: `${fixture.scope}/🧪️order`, progress: (event) => firstEvents.push(event) });
      rmSync(orderRoot, { recursive: true, force: true });
      normalizationWriteFiles(orderRoot, { "🧪️b/📝️.md": "b\n", "🧪️a/📝️.md": "a\n" });
      const secondEvents: TaxonomyProgress[] = [];
      const second = inventoryTaxonomy({ ...fixture.options, scope: `${fixture.scope}/🧪️order`, progress: (event) => secondEvents.push(event) });
      expect(canonicalJson(second)).toBe(canonicalJson(first));
      expect(canonicalJson(secondEvents)).toBe(canonicalJson(firstEvents));
      expect(normalizationPhaseSequence(secondEvents)).toEqual(normalizationPhaseSequence(firstEvents));
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("root taxonomy inventory CLI writes phases only to stderr and canonical JSON only to stdout", () => {
    const fixture = normalizationFixture("scoped-cli-purity", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    try {
      const scriptPath = join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts");
      const processPath = join(getWorkspaceRoot(), "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts");
      const child = 'const [scriptPath,repoRoot,scope,processPath]=process.argv.slice(1);const {pathToFileURL}=await import("node:url");const {CleanScript}=await import(pathToFileURL(scriptPath).href);const {createScriptProcessEnvelope,withScriptProcessEnvelope}=await import(pathToFileURL(processPath).href);await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"workspace-contract-test",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{await new CleanScript(repoRoot,repoRoot,invocation).run(["taxonomy","inventory","--scope",scope,"--workers","1","--format","json"]);});';
      const result = spawnSync("bun", ["-e", child, scriptPath, fixture.repoRoot, fixture.scope, processPath], { encoding: "utf8" });
      expect(result.status).toBe(0);
      const parsed = JSON.parse(result.stdout);
      expect(result.stdout).toBe(`${canonicalJson(parsed)}\n`);
      expect(result.stdout).not.toContain("[clean taxonomy progress]");
      expect(result.stderr).toContain("[clean taxonomy progress] inventory setup 0/1");
      expect(result.stderr).toContain("[clean taxonomy progress] inventory complete");
      expect(result.stderr.split(/\r?\n/u).filter(Boolean).every((line) => line.startsWith("[clean taxonomy progress]"))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("a scoped apply ignores unrelated stale projection tokens", () => {
    const { fixture, externalPath, staleToken } = scopedUnrelatedProjectionStaleFixture("unrelated-stale");
    try {
      const plan = normalizationPlan(fixture).plan;
      expect(plan.unresolved.filter((entry) => entry.severity === "error")).toEqual([]);
      writeFileSync(join(fixture.repoRoot, externalPath), `# Unrelated\n\n${staleToken}\n`);
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(result.state).toBe("committed");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("an exact ticket-important leaf scope ignores unrelated ticket evidence, rolls back, and commits its history projection", () => {
    const content = "Historical ticket evidence.\n";
    const fixture = normalizationFixture("ticket-important-history-apply", { "🟦️.ts": "export const fixture = true;\n" }, ({ repoRoot, ticketDir }) => {
      const source = join(repoRoot, TICKET_IMPORTANT_HISTORY_SOURCE_REL);
      mkdirSync(dirname(source), { recursive: true });
      writeFileSync(source, content);
      normalizationWriteFiles(ticketDir, { "📓️unrelated/📝️.md": `Retained evidence names ${TICKET_IMPORTANT_HISTORY_SOURCE_REL}.\n` });
    });
    const options: TaxonomyInventoryOptions = { repoRoot: fixture.repoRoot, scope: TICKET_IMPORTANT_HISTORY_SOURCE_REL, ticketDir: fixture.ticketDir, workers: 1 };
    try {
      const inventory = inventoryTaxonomy(options);
      const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(plan.unresolved.filter((entry) => entry.severity === "error")).toEqual([]);
      expect(plan.moves.map((move) => [move.sourcePath, move.destinationPath, move.rationaleRule])).toEqual([[TICKET_IMPORTANT_HISTORY_SOURCE_REL, TICKET_IMPORTANT_HISTORY_DESTINATION_REL, "ticket-important-history-markdown-v1"]]);
      expect(plan.evidenceRemovals).toEqual([]);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: "before-verify" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(readFileSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_SOURCE_REL), "utf8")).toBe(content);
      expect(existsSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_DESTINATION_REL))).toBe(false);
      const applied = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(applied.state).toBe("committed");
      expect(existsSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_SOURCE_REL))).toBe(false);
      expect(readFileSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_DESTINATION_REL), "utf8")).toBe(content);
      const second = planTaxonomy(inventoryTaxonomy(options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(second.moves).toEqual([]);
      expect(second.evidenceRemovals).toEqual([]);
      expect(second.unresolved.filter((entry) => entry.severity === "error")).toEqual([]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 120_000);

  test("exact CAD source scope plans declared external edits and rejects declared stale consumers before mutation", () => {
    const fixture = artifactProjectionNormalizationFixture("declared-external", true);
    let passed = false;
    try {
      expect(normalizationPlan(fixture).plan.unresolved).toEqual([]);
      const cad = projectionGolden("artifact-example-model-catalog-v1");
      const exactScope = `${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}`;
      const schemaPath = join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL);
      const schema = JSON.parse(readFileSync(schemaPath, "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { projectionContractId: string; sourcePathIdentities: string[]; sourcePathPattern: string; staleMarkers: string[] }> };
      const contract = Object.values(schema.semanticPathProjectionReferenceConsumerContracts).find((entry) => entry.projectionContractId === cad.contractId)!;
      const declaredStalePath = `${fixture.scope}/🟦️declared-external.ts`;
      contract.sourcePathIdentities.push(declaredStalePath);
      contract.sourcePathPattern = `^(?:${contract.sourcePathIdentities.map((path) => path.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|")})$`;
      normalizationWriteFiles(fixture.repoRoot, { [declaredStalePath]: `export const stale = ${JSON.stringify(contract.staleMarkers[0])};\n` });
      writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
      const inventory = inventoryTaxonomy({ ...fixture.options, scope: exactScope });
      const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(plan.regenerations.map(({ contractId }) => contractId).sort()).toEqual<readonly string[]>(DRAW_SOURCE_SCENARIO.oracle.activatedGeneratorIds);
      expect(plan.unresolved.filter((entry) => entry.code === "projection-old-token-stale" && entry.path === declaredStalePath)).toHaveLength(1);
      expect(plan.moves.filter((move) => move.rationaleRule === cad.rationaleRule)).toHaveLength(cad.sourceFileCount);
      expect(plan.edits.some((edit) => !inScopeForTest(edit.path, exactScope))).toBe(true);
      expect(plan.edits.some((edit) => edit.path === declaredStalePath)).toBe(false);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest }));
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 60_000);

  test("inventory bytes are deterministic and its language-agnostic file census agrees with owned filesystem discovery", () => {
    const fixture = normalizationFixture("inventory", {
      ".hidden/🔣️config.json": "{}\n",
      "Cargo.toml": "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\n",
      "🦀️.rs": "pub struct Fixture;\n",
      "🟦️component.ts": "export const fixture = true;\n",
      "🤖️generated/🟦️.ts": "export const generated = true;\n",
    });
    try {
      let progressEvents = 0;
      const options: TaxonomyInventoryOptions = { ...fixture.options, progress: () => { progressEvents += 1; } };
      const first = inventoryTaxonomy(options);
      const second = inventoryTaxonomy(options);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(progressEvents).toBeGreaterThan(0);
      const expected = ownedFilePaths(fixture.workspace)
        .map((path) => `${fixture.scope}/${path.replaceAll("\\", "/")}`)
        .sort();
      const actual = first.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath).sort();
      expect(actual).toEqual(expected);
      expect(first.entries.every((entry) => !entry.sourcePath.startsWith("compose/"))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

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

  test("stale preimages block apply without changing source bytes", () => {
    const fixture = normalizationFixture("stale", { "🦀️component.rs": "pub const VALUE: i32 = 1;\n" });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(plan.moves.length).toBeGreaterThan(0);
      writeFileSync(join(fixture.workspace, "🦀️component.rs"), "pub const VALUE: i32 = 2;\n");
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest }));
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  for (const failureStage of ["after-staging", "after-moves", "after-edits", "before-verify"] as const) {
    test(`rolls back byte-for-byte when failure is injected at ${failureStage}`, () => {
      const fixture = normalizationFixture(`failure-${failureStage}`, {
        "🟦️subject.ts": "export const value = 1;\n",
        "🧪️consumer/🟦️.ts": "export { value } from \"🧪️tests/🧪️fixture/🟦️subject.ts\";\n",
      });
      try {
        const { plan } = normalizationPlan(fixture);
        expect(plan.moves.length).toBeGreaterThan(0);
        const before = normalizationWorkspaceSnapshot(fixture.workspace);
        expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: failureStage }));
        expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
        expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
      } finally {
        rmSync(fixture.root, { recursive: true, force: true });
      }
    });
  }

  test("cancellation rolls back and a successful retry converges to an empty second plan", () => {
    const cancelled = normalizationFixture("cancel", { "🟦️.ts": "export const value = 1;\n" });
    try {
      const { plan } = normalizationPlan(cancelled);
      const before = normalizationWorkspaceSnapshot(cancelled.workspace);
      const cancelFile = join(cancelled.ticketDir, "cancel");
      writeFileSync(cancelFile, "cancel\n");
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, cancelFile }));
      expect(normalizationWorkspaceSnapshot(cancelled.workspace)).toEqual(before);
      expect(opaqueTreeDigest(cancelled.repoRoot, "compose")).toEqual(cancelled.opaqueDigest);
    } finally {
      rmSync(cancelled.root, { recursive: true, force: true });
    }

    const fixture = normalizationFixture("convergence", { "🦀️.rs": "pub const VALUE: i32 = 1;\n" });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(result.planDigest).toBe(plan.planDigest);
      expect(result.appliedMoves).toBe(plan.moves.length);
      expect(result.appliedEdits).toBe(plan.edits.length);
      expect(String(result.state)).toMatch(/^(?:applied|committed|complete)$/u);
      const verification = verifyTaxonomy({ ...fixture.options, baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(canonicalJson(verification)).not.toMatch(/"severity":"error"/u);
      const secondInventory = inventoryTaxonomy(fixture.options);
      const secondPlan = planTaxonomy(secondInventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(secondPlan.moves).toEqual([]);
      expect(secondPlan.edits).toEqual([]);
      expect(secondPlan.regenerations).toEqual([]);
      expect(secondPlan.unresolved).toEqual([]);
      expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 120_000);
});

/** 📋️ The creation order, inventory CLI, stale token, ticket leaf scope and external edit tests. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { applyTaxonomyPlan } from "../../../../🧹️normalization/🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../../../🧹️normalization/🧠️planning/🟦️.ts";
import { type TaxonomyInventoryOptions } from "../../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { type TaxonomyProgress } from "../../../../🧹️normalization/🏃️operation/🟦️.ts";
import { ownedFilePaths } from "../../../🔍️filesystem/🟦️.ts";
import { DRAW_SOURCE_SCENARIO, NORMALIZATION_SCHEMA_REL, TICKET_IMPORTANT_HISTORY_DESTINATION_REL, TICKET_IMPORTANT_HISTORY_SOURCE_REL, artifactProjectionFixturePath, artifactProjectionNormalizationFixture, expectNormalizationApplyFailure, inScopeForTest, normalizationFixture, normalizationPhaseSequence, normalizationPlan, normalizationWorkspaceSnapshot, normalizationWriteFiles, projectionGolden, retainArtifactProjectionFixture, scopedUnrelatedProjectionStaleFixture } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🖥 Registers the reversed creation order, the inventory CLI streams, the stale token, the ticket important leaf scope and the CAD source scope tests. */
export function registerInventoryOrderCliAndTicketScopeTests(): void {
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
}

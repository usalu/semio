/** 🎫️ The explicit source admission, canonical json, opaque tree digest and pathspec tests. */
import { expect, test } from "bun:test";
import { mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { applyTaxonomyPlan } from "../../../../🧹️normalization/🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { opaqueTreeDigest } from "../../../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { planTaxonomy } from "../../../../🧹️normalization/🧠️planning/🟦️.ts";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { taxonomyScopedGitPathspec } from "../../../../🧹️normalization/🚪️source-admission/🟦️.ts";
import { NORMALIZATION_SCHEMA_REL, SCOPED_INVENTORY_PATHSPEC_GOLDEN, drawScenarioTaxonomy, normalizationFixture, normalizationPlan, normalizationWorkspaceSnapshot, normalizationWriteFiles } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🛂 Registers the changed ticket and source admission rejections, canonical json, schema boundary, opaque tree digest and Git pathspec tests. */
export function registerSourceAdmissionAndCanonicalJsonTests(): void {
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
}

/** 🔄️ The atomic CAD and Draw projection apply and residual child tests. */
import { expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { chmodSync, existsSync, lstatSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { noFollowTreeDigest } from "../../../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { type TaxonomyInventory } from "../../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { ARTIFACT_PROJECTION_GOLDEN, NORMALIZATION_SCHEMA_REL, artifactProjectionApply, artifactProjectionFixturePath, artifactProjectionIgnoreResidue, artifactProjectionNormalizationFixture, expectNormalizationApplyFailure, normalizationGit, normalizationPlan, normalizationWorkspaceSnapshot, projectionByteSort, projectionGolden, retainArtifactProjectionFixture } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🧩 Registers the rollback and atomic apply of the CAD and Draw projections, the residual child rejection and the source tree authority rederivation tests. */
export function registerProjectionApplyAndResidualChildTests(): void {
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
}

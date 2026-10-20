/** 🟢️ The requires fresh clean terminal tests of the direct mutation ownership suite. */
import { mutationRoot, mutationFixtureRoot } from "../../../🧬️mutation-authority/🧫️fixture/🟦️.ts";
import { expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { taxonomyCliArtifactPath } from "../../../../🧹️normalization/🎮️command-contract/🟦️.ts";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { planMutationTaxonomy, runMutationTaxonomyCli } from "../../../../🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts";
import { inventoryMutationTaxonomy } from "../../../../🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔵 Registers the requires fresh clean terminal tests of the direct mutation ownership suite. */
export function registerRequiresFreshCleanTerminalTests(): void {
  test("requires a fresh clean terminal verification before mutation apply can commit", () => {
    const goldenPath = join(contractRoot, "../🧫️fixtures/📋️mutation-inventory/🔣️.json");
    const golden = JSON.parse(readFileSync(goldenPath, "utf8")) as { schemaVersion: number; baseline: string; cases: readonly string[] };
    const root = mutationFixtureRoot("semio-mutation-terminality-");
    const ticket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️08", "☀️12", "TERMINALITY");
    const planPath = join(ticket, "plan.json");
    const applyPath = taxonomyCliArtifactPath(ticket, "apply", "json");
    const options = (plan: string, cancelFile?: string) => ({ baseline: golden.baseline, cancelFile, failOnWarning: false, format: "json" as const, kind: "mutation" as const, plan });
    const writePlan = (plan: unknown) => {
      mkdirSync(dirname(planPath), { recursive: true });
      writeFileSync(planPath, `${canonicalJson(plan)}\n`);
    };
    try {
      const directViolation = {
        schemaVersion: 1 as const,
        kind: "mutation" as const,
        sourceTreeDigest: "a".repeat(64),
        roots: ["✏️s/🧪️probe/🧬️mutations"],
        records: [{ mutationRootPath: "✏️s/🧪️probe/🧬️mutations", targetMutationDirectoryName: "➕️insert-page", state: "direct" as const, violationClasses: ["mutation/behavior-ownership"], evidence: [] }],
        violations: [{ id: "behavior", kind: "mutation/behavior-ownership", scope: "✏️s/🧪️probe/🧬️mutations/➕️insert-page", summary: "behavior remains at root", priority: "high" as const, reason: "direct leaves own behavior", solution: "move behavior" }],
      };
      expect(planMutationTaxonomy(directViolation as any, golden.baseline).unresolved).toEqual(expect.arrayContaining([expect.objectContaining({ path: "✏️s/🧪️probe/🧬️mutations/➕️insert-page" })]));

      const mutations = mutationRoot(root);
      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      const sourceRoster = fastGlob.sync("**/🧬️mutations/🦀️.rs", { cwd: root, onlyFiles: true, followSymbolicLinks: false });
      expect(sourceRoster).toEqual([`${relative(root, mutations).replaceAll("\\", "/")}/🦀️.rs`]);
      const binarySource = join(mutations, "🔢source.bin");
      writeFileSync(binarySource, Buffer.from([0, 1, 0]));
      const nulDigest = inventoryMutationTaxonomy(root).sourceTreeDigest;
      writeFileSync(binarySource, Buffer.from([0, 2, 0]));
      expect(inventoryMutationTaxonomy(root).sourceTreeDigest).not.toBe(nulDigest);
      const staleInventory = inventoryMutationTaxonomy(root);
      const stalePlan = planMutationTaxonomy(staleInventory, golden.baseline);
      writePlan(stalePlan);
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n// source-byte-only change\n");
      expect(inventoryMutationTaxonomy(root).sourceTreeDigest).not.toBe(staleInventory.sourceTreeDigest);
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath), ticket)).toThrow(/inventory digest/u);
      expect(existsSync(applyPath)).toBe(false);
      let mutateDuringInventory = true;
      const stableInventory = inventoryMutationTaxonomy(root, { afterFactsCollected: () => {
        if (!mutateDuringInventory) return;
        mutateDuringInventory = false;
        writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n// source-byte-only change during facts\n");
      } });
      expect(stableInventory.sourceTreeDigest).toBe(inventoryMutationTaxonomy(root).sourceTreeDigest);

      rmSync(mutations, { recursive: true, force: true });
      const opaqueMutations = join(root, "compose", "🧬️mutations");
      mkdirSync(opaqueMutations, { recursive: true });
      writeFileSync(join(opaqueMutations, "🦀️.rs"), "pub enum OpaqueMutation {}\n");
      expect(inventoryMutationTaxonomy(root).roots).toEqual([]);
      rmSync(join(root, "compose"), { recursive: true, force: true });

      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      symlinkSync(process.platform === "win32" ? mutations : "🦀️.rs", join(mutations, process.platform === "win32" ? "🧬️alias" : "🦀️alias.rs"), process.platform === "win32" ? "junction" : "file");
      expect(() => inventoryMutationTaxonomy(root)).toThrow(/symlink/u);
      rmSync(mutations, { recursive: true, force: true });

      const cleanInventory = inventoryMutationTaxonomy(root);
      const cleanPlan = planMutationTaxonomy(cleanInventory, golden.baseline);
      writePlan(cleanPlan);
      expect(() => runMutationTaxonomyCli(root, "apply", { ...options(planPath), baseline: "b".repeat(40) }, ticket)).toThrow(/baseline/u);
      expect(existsSync(applyPath)).toBe(false);
      mkdirSync(mutations, { recursive: true });
      expect(inventoryMutationTaxonomy(root).sourceTreeDigest).not.toBe(cleanInventory.sourceTreeDigest);
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath), ticket)).toThrow(/inventory digest/u);
      expect(existsSync(applyPath)).toBe(false);
      rmSync(mutations, { recursive: true, force: true });

      const cancelFile = join(ticket, "cancel");
      writeFileSync(cancelFile, "cancel\n");
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath, cancelFile), ticket)).toThrow(/cancelled/u);
      expect(existsSync(applyPath)).toBe(false);
      rmSync(cancelFile);

      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      mkdirSync(join(mutations, "➕️insert-page", "🦠️mutation"), { recursive: true });
      writeFileSync(join(mutations, "➕️insert-page", "🦠️mutation", "🦀️.rs"), "pub struct InsertPage;\n");
      const violatingInventory = inventoryMutationTaxonomy(root);
      const unsigned = { schemaVersion: 1 as const, kind: "mutation" as const, baselineCommit: golden.baseline, inventoryDigest: violatingInventory.sourceTreeDigest, moves: [], unresolved: [] };
      writePlan({ ...unsigned, planDigest: createHash("sha256").update(canonicalJson(unsigned)).digest("hex") });
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath), ticket)).toThrow(/terminal verification/u);
      expect(existsSync(applyPath)).toBe(false);

      rmSync(mutations, { recursive: true, force: true });
      const terminalInventory = inventoryMutationTaxonomy(root);
      writePlan(planMutationTaxonomy(terminalInventory, golden.baseline));
      runMutationTaxonomyCli(root, "apply", options(planPath), ticket);
      expect(JSON.parse(readFileSync(applyPath, "utf8"))).toMatchObject({ schemaVersion: 1, kind: "mutation", state: "committed", inventoryDigest: terminalInventory.sourceTreeDigest, terminalVerification: { clean: true, violations: [] } });
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);
}

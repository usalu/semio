import { testLevelAtLeast } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { applyTaxonomyPlan } from "../../../🧹️normalization/🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { noFollowTreeDigest } from "../../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { parseGeneratorPreviewManifest } from "../../../🧹️normalization/🧾️plan-parsing/🟦️.ts";
import { parseTaxonomyPlan } from "../../../🧹️normalization/📃️plan-document/🟦️.ts";
import { planTaxonomy } from "../../../🧹️normalization/🧠️planning/🟦️.ts";
import { repositoryLocalSymlinkTargetPath } from "../../../🧹️normalization/➰️symlink-targets/🟦️.ts";
import { taxonomyPlanDigest } from "../../../🧹️normalization/🗃️affected-state/🟦️.ts";
import { taxonomyPlatformPathViolationCodes } from "../../../🧹️normalization/👥️mutation-projection/🟦️.ts";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { type TaxonomyPlan } from "../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { ownedFilePaths, ownedFilesystemEntries, ownedPathByteSort } from "../../🔍️filesystem/🟦️.ts";
import { NORMALIZATION_SCHEMA_REL, normalizationFixture, normalizationPlan, normalizationWorkspaceSnapshot, normalizationWriteFiles } from "../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

const GENERATOR_PREVIEW_GOLDEN = join(contractRoot, "../🧫️fixtures/🏭️generator-preview/🔣️.json");

describe("generator preview protocol", () => {
  test("accepts the canonical language-neutral manifest and agrees with owned filesystem inventory", () => {
    const canonical = `${canonicalJson(JSON.parse(readFileSync(GENERATOR_PREVIEW_GOLDEN, "utf8")))}\n`;
    const manifest = parseGeneratorPreviewManifest(canonical, "fixture-generator", ["🧪️workspace/🤖️generated"], ["compose"]);
    const root = mkdtempSync(join(tmpdir(), "generator-preview-parity-"));
    try {
      for (const node of manifest.nodes) {
        const target = join(root, node.path);
        if (node.nodeKind === "directory") mkdirSync(target, { recursive: true, mode: node.mode });
        else {
          mkdirSync(resolve(target, ".."), { recursive: true });
          writeFileSync(target, Buffer.from(node.bytesBase64, "base64"), { mode: node.mode });
        }
      }
      const platformBoundary = ownedFilesystemEntries(join(root, "🧪️workspace", "🤖️generated"), true).map(({ path }) => path === "." ? "🧪️workspace/🤖️generated" : `🧪️workspace/🤖️generated/${path}`);
      expect(platformBoundary).toEqual(manifest.nodes.map((node) => node.path));
      expect(manifest.staleRemovals).toEqual(["🧪️workspace/🤖️generated/old.txt"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects noncanonical, noisy, unsafe, duplicate, and malformed manifests", () => {
    const valid = JSON.parse(readFileSync(GENERATOR_PREVIEW_GOLDEN, "utf8")) as Record<string, unknown>;
    const render = (value: unknown): string => `${canonicalJson(value)}\n`;
    const cases: readonly string[] = [
      `noise\n${render(valid)}`,
      JSON.stringify(valid),
      `${JSON.stringify({ schemaVersion: 1, contractId: valid.contractId, nodes: valid.nodes, staleRemovals: valid.staleRemovals })}\n`,
      render({ ...valid, schemaVersion: 2 }),
      render({ ...valid, contractId: "different-generator" }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }, { bytesBase64: "%%%", mode: 420, nodeKind: "file", path: "🧪️workspace/🤖️generated/🔤️.txt" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493.5, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "symlink", path: "🧪️workspace/🤖️generated" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "../../📦️packages/outside" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "compose/generated" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }, { bytesBase64: "Z2VuZXJhdGVkXG4=", mode: 420, nodeKind: "file", path: "🧪️workspace/🤖️generated/cafe\u0301.txt" }] }),
      `${JSON.stringify({ ...valid, nodes: [...(valid.nodes as unknown[])].reverse() })}\n`,
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }, { bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }] }),
      render({ ...valid, staleRemovals: ["🧪️workspace/🤖️generated/old.txt", "🧪️workspace/🤖️generated/old.txt"] }),
      render({ ...valid, staleRemovals: ["🧪️workspace/🤖️generated/🔤️.txt"] }),
    ];
    for (const content of cases) expect(() => parseGeneratorPreviewManifest(content, "fixture-generator", ["🧪️workspace/🤖️generated"], ["compose"])).toThrow();
  });

  test.if(testLevelAtLeast("long"))("plans, applies, verifies, and converges an exact Nx-owned preview", () => {
    const fixture = normalizationFixture(
      "generator-preview",
      {
        "🧪️generator/🤖️generated/old.txt": "stale\n",
      },
      ({ repoRoot, workspace }) => {
        const owner = relative(repoRoot, join(workspace, "🧪️generator")).replaceAll("\\", "/");
        const outputRoot = `${owner}/🤖️generated`;
        const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
        const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
        taxonomy.generatorContracts = Object.fromEntries(Object.entries({
          ...(taxonomy.generatorContracts as Record<string, unknown>),
          "fixture-generator": {
            ownership: "owned",
            ownerPath: owner,
            target: "@fixture/generator:generate",
            previewTarget: "@fixture/generator:preview-generated",
            checkTarget: "@fixture/generator:check",
            inputPatterns: [`${owner}/🟦️.ts`],
            outputRoots: [{ path: outputRoot, inclusion: "ignored" }],
            reason: "Language-neutral generator preview fixture",
          },
        }).sort(([left], [right]) => left.localeCompare(right)));
        writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
        writeFileSync(join(repoRoot, ".gitignore"), `${outputRoot}\n`);
        writeFileSync(join(repoRoot, "nx.json"), "{\"defaultBase\":\"main\"}\n");
        writeFileSync(join(repoRoot, "package.json"), "{\"name\":\"generator-preview-fixture\",\"private\":true,\"scripts\":{\"nx\":\"repo ./node_modules/nx/dist/bin/nx.js\"}}\n");
        const projectManifest = `${JSON.stringify({
          name: "@fixture/generator",
          root: owner,
          targets: {
            generate: { executor: "repo:script", options: { cwd: owner, command: "repo ./📜️script.ts generate" } },
            "preview-generated": { executor: "repo:script", options: { cwd: owner, command: "repo ./📜️script.ts preview-generated" } },
            check: { executor: "repo:script", options: { cwd: owner, command: "repo ./📜️script.ts check" } },
          },
        }, null, 2)}\n`;
        writeFileSync(join(repoRoot, "project.json"), projectManifest);
        for (const name of ["nx", "@semio-tech/repo-lib"]) {
          mkdirSync(dirname(join(repoRoot, "node_modules", name)), { recursive: true });
          symlinkSync(realpathSync(join(getWorkspaceRoot(), "node_modules", name)), join(repoRoot, "node_modules", name), process.platform === "win32" ? "junction" : "dir");
        }
        normalizationWriteFiles(join(workspace, "🧪️generator"), {
          "📋️project.json": projectManifest,
          "🟦️.ts": [
            'import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";',
            'import { join } from "node:path";',
            `const outputRoot = ${JSON.stringify(join(repoRoot, outputRoot))};`,
            `const outputRelative = ${JSON.stringify(outputRoot)};`,
            'const outputFile = join(outputRoot, "🔤️.txt");',
            'const bytes = Buffer.from("generated\\n");',
            '/** 👓️ Prints the exact owned preview without touching the output root. */',
            'export function previewGenerated(): void {',
            '  const nodes = [{ bytesBase64: "", mode: 0o755, nodeKind: "directory", path: outputRelative }, { bytesBase64: bytes.toString("base64"), mode: 0o644, nodeKind: "file", path: `${outputRelative}/🔤️.txt` }];',
            '  const staleRemovals = (existsSync(outputRoot) ? readdirSync(outputRoot) : []).filter((name) => name !== "🔤️.txt").map((name) => `${outputRelative}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));',
            '  process.stdout.write(`${JSON.stringify({ contractId: "fixture-generator", nodes, schemaVersion: 1, staleRemovals })}\\n`);',
            '}',
            '/** 🏭️ Replaces the owned output root with its single generated leaf. */',
            'export function generate(): void {',
            '  rmSync(outputRoot, { recursive: true, force: true });',
            '  mkdirSync(outputRoot, { recursive: true, mode: 0o755 });',
            '  writeFileSync(outputFile, bytes, { mode: 0o644 });',
            '}',
            '/** ✅️ Rejects any output root that differs from the generated leaf. */',
            'export function checkGenerated(): void {',
            '  if (!existsSync(outputFile) || !readFileSync(outputFile).equals(bytes) || readdirSync(outputRoot).join("\\0") !== "🔤️.txt") throw new Error("generated output is stale");',
            '}',
            '',
          ].join("\n"),
          "📜️script.ts": [
            'import { BundleScript, ScriptRouter, runScriptMain } from "@semio-tech/repo-lib";',
            'import { checkGenerated, generate, previewGenerated } from "./🟦️.ts";',
            '',
            'class GenerateScript extends BundleScript {',
            '  async run(): Promise<void> {',
            '    generate();',
            '  }',
            '}',
            '',
            'class PreviewGeneratedScript extends BundleScript {',
            '  async run(): Promise<void> {',
            '    previewGenerated();',
            '  }',
            '}',
            '',
            'class CheckScript extends BundleScript {',
            '  async run(): Promise<void> {',
            '    checkGenerated();',
            '  }',
            '}',
            '',
            'const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckScript);',
            '',
            'await runScriptMain(router, import.meta.url);',
            '',
          ].join("\n"),
        });
      },
    );
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(plan.regenerations).toHaveLength(1);
      expect(plan.regenerations[0]?.contractId).toBe("fixture-generator");
      expect(plan.regenerations[0]?.preview.nodes.map((node) => node.path)).toEqual(plan.regenerations[0]?.outputs.map((node) => node.path));
      expect(plan.regenerations[0]?.staleRemovals).toEqual([`${fixture.scope}/🧪️generator/🤖️generated/old.txt`]);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: "after-edits" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const retryTicket = fixture.ticketDir;
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(result.state).toBe("committed");
      expect(JSON.parse(readFileSync(result.journalPath, "utf8")).attemptOrdinal).toBe("000002");
      const second = planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(second.regenerations).toEqual([]);
      expect(second.unresolved).toEqual([]);
    } finally {
      if (process.env.KEEP_GENERATOR_FIXTURE !== "1") rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);
});

describe.if(testLevelAtLeast("long"))("taxonomy transaction dispositions v2", () => {
  const sentinelCasesPath = resolve(contractRoot, "../🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json");
  const dispositionOutcomesPath = resolve(contractRoot, "../🧫️fixtures/🎲️transaction-disposition-outcomes/🔣️.json");
  const protocolPath = resolve(contractRoot, "../🧫️fixtures/🤝️transaction-protocol/🔣️.json");

  test("keeps checkout-hostile sentinels virtual and language-neutral", () => {
    const sentinelCases = JSON.parse(readFileSync(sentinelCasesPath, "utf8")) as {
      schemaVersion: 1;
      virtualPathPolicyCases: { inputPath: string; expectedViolationCode: string }[];
      symlinkFlavorCases: { repositoryRoot: string; target: string; owned: boolean }[];
    };
    const dispositionOutcomes = JSON.parse(readFileSync(dispositionOutcomesPath, "utf8")) as {
      schemaVersion: 1;
      expectedDispositionOperations: { embeddedTicketRoots: number; embeddedTicketRootRelocations: number; evidenceRemovals: number; operationFamilies: string[] };
      affectedStateCases: { id: string; pre: unknown[]; post: unknown[] }[];
      negativeDispositionCases: { id: string; expectedCode: string }[];
    };
    const protocol = JSON.parse(readFileSync(protocolPath, "utf8")) as {
      schemaVersion: 1;
      failureStages: string[];
      journalStates: string[];
      virtualPreimageNodes: { path: string; state: "absent" | "directory" | "file" | "symlink"; contentHash?: string; size?: number; target?: string }[];
    };
    expect(Object.keys(sentinelCases).sort()).toEqual(["schemaVersion", "symlinkFlavorCases", "virtualPathPolicyCases"]);
    expect(Object.keys(dispositionOutcomes).sort()).toEqual(["affectedStateCases", "expectedDispositionOperations", "negativeDispositionCases", "schemaVersion"]);
    expect(Object.keys(protocol).sort()).toEqual(["failureStages", "journalStates", "schemaVersion", "virtualPreimageNodes"]);
    for (const row of sentinelCases.virtualPathPolicyCases) expect(taxonomyPlatformPathViolationCodes(row.inputPath, loadCatalogTaxonomy().collisionPolicy)).toContain(row.expectedViolationCode);
    for (const row of sentinelCases.symlinkFlavorCases) expect(repositoryLocalSymlinkTargetPath(row.repositoryRoot, row.target) !== null).toBe(row.owned);
    expect(dispositionOutcomes.expectedDispositionOperations).toEqual({ embeddedTicketRoots: 3, embeddedTicketRootRelocations: 4, evidenceRemovals: 2, operationFamilies: ["moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "edits", "regenerations"] });
    expect(protocol.failureStages).toEqual(["after-staging", "after-embedded-root-staging", "after-moves", "after-relocations", "after-symlink-retargeting", "after-edits", "after-regenerations", "before-verify"]);
    expect(protocol.journalStates).toHaveLength(11);
    expect(dispositionOutcomes.affectedStateCases.map((row) => row.id)).toEqual(["remove-redundant-evidence", "retarget-no-follow-symlink"]);
    expect(new Set(dispositionOutcomes.negativeDispositionCases.map((row) => row.expectedCode)).size).toBe(dispositionOutcomes.negativeDispositionCases.length);
    expect(ownedFilePaths(resolve(sentinelCasesPath, ".."))).toEqual(["🔣️.json"]);
    expect(ownedFilePaths(resolve(dispositionOutcomesPath, ".."))).toEqual(["🔣️.json"]);
    expect(ownedFilePaths(resolve(protocolPath, ".."))).toEqual(["🔣️.json"]);
    const root = mkdtempSync(join(tmpdir(), "transaction-golden-nofollow-"));
    try {
      mkdirSync(join(root, "evidence", "directory"), { recursive: true });
      writeFileSync(join(root, "evidence", "file.txt"), "sentinel\n");
      symlinkSync("../file.txt", join(root, "evidence", "link"));
      const native = noFollowTreeDigest(root, "evidence");
      const platformBoundary = ownedFilesystemEntries(join(root, "evidence"), true).map(({ path }) => path === "." ? "evidence" : `evidence/${path}`);
      expect(native.files).toBe(1);
      expect(native.directories).toBe(2);
      expect(native.symlinks).toBe(1);
      expect(platformBoundary).toEqual(["evidence", "evidence/directory", "evidence/file.txt", "evidence/link"]);
      expect(protocol.virtualPreimageNodes.find((row) => row.state === "file")?.contentHash).toBe("b5f7e7d285029324d9b3acae19cc05099271454ac98bfc059a92b0581625cd51");
      expect(protocol.virtualPreimageNodes.find((row) => row.state === "symlink")?.target).toBe("../file.txt");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects v1 and incomplete v2 plans without compatibility defaults", () => {
    expect(() => parseTaxonomyPlan({ schemaVersion: 1 })).toThrow();
    expect(() => parseTaxonomyPlan({ schemaVersion: 2, taxonomySchemaVersion: 7 })).toThrow();
  });

  test("hashes symlink target text without following its target", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-transaction-no-follow-"));
    try {
      const linkRoot = join(root, "links");
      mkdirSync(linkRoot);
      writeFileSync(join(root, "target.txt"), "first");
      symlinkSync("../../📦️packages/target.txt", join(linkRoot, "link"));
      const before = noFollowTreeDigest(linkRoot, ".");
      writeFileSync(join(root, "target.txt"), "second");
      const afterTarget = noFollowTreeDigest(linkRoot, ".");
      expect(afterTarget.digest).toBe(before.digest);
      expect(afterTarget.symlinks).toBe(1);
      rmSync(join(linkRoot, "link"));
      symlinkSync("../../📦️packages/missing.txt", join(linkRoot, "link"));
      expect(noFollowTreeDigest(linkRoot, ".").digest).not.toBe(afterTarget.digest);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("retargets a repository-local broken absolute symlink atomically and converges", () => {
    const fixture = normalizationFixture("transaction-symlink-v2", {}, ({ workspace }) => symlinkSync(join(workspace, "target.ts"), join(workspace, "🟦️.ts")));
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.symlinkTargetEdits).toHaveLength(1);
      expect(plan.unresolved).toEqual([]);
      expect(parseTaxonomyPlan(JSON.parse(JSON.stringify(plan)))).toEqual(plan);
      const tamperedTarget = JSON.parse(JSON.stringify(plan)) as TaxonomyPlan;
      (tamperedTarget.symlinkTargetEdits[0] as { oldTarget: string }).oldTarget = `${tamperedTarget.symlinkTargetEdits[0].oldTarget}-drift`;
      (tamperedTarget as { planDigest: string }).planDigest = taxonomyPlanDigest(tamperedTarget);
      expect(() => parseTaxonomyPlan(tamperedTarget)).toThrow();
      expect(() => parseTaxonomyPlan({ ...plan, unexpected: true })).toThrow();
      const deadPreparation = join(fixture.ticketDir, "🧾️taxonomy-transaction", `🔖️${plan.planDigest}`, "🔂️attempts", "🚧️prepare-000001-999999-00000000-0000-4000-8000-000000000001");
      mkdirSync(deadPreparation, { recursive: true });
      const before = noFollowTreeDigest(fixture.repoRoot, fixture.scope);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, injectFailureAt: "after-symlink-retargeting" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(existsSync(deadPreparation)).toBe(false);
      expect(noFollowTreeDigest(fixture.repoRoot, fixture.scope)).toEqual(before);
      const retryTicket = fixture.ticketDir;
      const applied = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit });
      expect(applied.appliedSymlinkTargetEdits).toBe(1);
      expect(applied.journalPath).toMatch(/🔢️000002\/🔣️\.json$/u);
      const terminalAttempt = resolve(applied.journalPath, "..");
      const deadLease = join(terminalAttempt, "🔒️lease");
      mkdirSync(deadLease);
      writeFileSync(join(deadLease, "🔣️.json"), `${canonicalJson({ schemaVersion: 1, planDigest: plan.planDigest, attemptOrdinal: "000002", token: "00000000-0000-4000-8000-000000000002", pid: 999999 })}\n`);
      const resumedCommitted = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, resumeJournal: applied.journalPath });
      expect(resumedCommitted.state).toBe("committed");
      expect(readdirSync(terminalAttempt)).toEqual(["🔣️.json"]);
      const second = planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(second.symlinkTargetEdits).toEqual([]);
      expect(second.unresolved).toEqual([]);
    } finally {
      if (process.env.KEEP_TRANSACTION_FIXTURE !== "1") rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 12_000);

  test("resumes an installed symlink retarget from its durable journal marker", () => {
    const fixture = normalizationFixture("transaction-symlink-resume-v2", {}, ({ workspace }) => symlinkSync(join(workspace, "target.ts"), join(workspace, "🟦️.ts")));
    try {
      const { plan } = normalizationPlan(fixture);
      const planPath = join(fixture.ticketDir, "plan-v2.json");
      writeFileSync(planPath, `${canonicalJson(plan)}\n`);
      const modulePath = resolve(contractRoot, "../../🧹️normalization/🚀️apply/🟦️.ts");
      const child = `const [modulePath,planPath,repoRoot,ticketDir,baseline]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(modulePath);const plan=JSON.parse(await Bun.file(planPath).text());applyTaxonomyPlan(plan,{repoRoot,ticketDir,expectedBaselineCommit:baseline,progress:(row)=>{if(row.phase==="retargeting-symlinks")process.exit(73)}});`;
      const interrupted = spawnSync("bun", ["-e", child, modulePath, planPath, fixture.repoRoot, fixture.ticketDir, plan.baselineCommit], { encoding: "utf8" });
      expect(interrupted.status).toBe(73);
      const journals = ownedFilePaths(fixture.ticketDir)
        .filter((path) => (path.startsWith("🧾️taxonomy-transaction/") || path.includes("/🧾️taxonomy-transaction/")) && /\/🔂️attempts\/🔢️[^/]+\/🔣️\.json$/u.test(path))
        .map((path) => join(fixture.ticketDir, path));
      expect(journals).toHaveLength(1);
      const leasePath = join(resolve(journals[0], ".."), "🔒️lease", "🔣️.json");
      expect(existsSync(leasePath)).toBe(true);
      const leaseBytes = readFileSync(leasePath, "utf8");
      const lease = JSON.parse(leaseBytes) as { pid: number };
      writeFileSync(leasePath, `${canonicalJson({ ...lease, pid: process.pid })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/leased by active pid/u);
      writeFileSync(leasePath, leaseBytes);
      let journalBytes = readFileSync(journals[0], "utf8");
      let journal = JSON.parse(journalBytes) as { revision: number; state: string; stagingRoot: string; installedSymlinkTargetEditIds: string[] };
      expect(journal.state).toBe("retargeting");
      expect(journal.installedSymlinkTargetEditIds).toEqual([plan.symlinkTargetEdits[0].operationId]);
      const stageRoot = join(fixture.repoRoot, journal.stagingRoot);
      const walRoot = join(stageRoot, "🚧️journal");
      mkdirSync(walRoot);
      writeFileSync(join(walRoot, "🔣️.json"), `${canonicalJson({ ...journal, revision: journal.revision + 1 })}\n`);
      writeFileSync(join(stageRoot, "unexpected"), "drift");
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/(?:unexpected|unauthorized) evidence/u);
      expect(existsSync(walRoot)).toBe(true);
      expect(readFileSync(journals[0], "utf8")).toBe(journalBytes);
      rmSync(walRoot, { recursive: true });
      expect(existsSync(leasePath)).toBe(true);
      rmSync(join(stageRoot, "unexpected"));
      journalBytes = readFileSync(journals[0], "utf8");
      journal = JSON.parse(journalBytes) as typeof journal;
      mkdirSync(walRoot);
      writeFileSync(join(stageRoot, "unexpected"), "drift");
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/(?:unexpected|unauthorized) evidence/u);
      expect(existsSync(walRoot)).toBe(true);
      rmSync(join(stageRoot, "unexpected"));
      writeFileSync(journals[0], JSON.stringify(journal, null, 2));
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/canonical JSON/u);
      writeFileSync(journals[0], `${canonicalJson({ ...journal, state: "staging" })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/impossible phase/u);
      writeFileSync(journals[0], `${canonicalJson({ ...journal, stagingRoot: `${journal.stagingRoot}-forged` })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/(?:attempt|transaction) roots/u);
      writeFileSync(journals[0], journalBytes);
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] });
      expect(result.state).toBe("committed");
      expect(readdirSync(resolve(journals[0], ".."))).toEqual(["🔣️.json"]);
      expect(planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] }).symlinkTargetEdits).toEqual([]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 12_000);

  test("relocates three complete embedded ticket roots with exact many-to-one evidence", () => {
    const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL";
    const files: Record<string, string> = {};
    for (const owner of ["pkg-a", "pkg-b", "pkg-c"]) {
      files[`${owner}/${ticket}/🧪️target-os-errors/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
      files[`${owner}/${ticket}/🧪️unique-${owner.slice(-1)}/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
    }
    const fixture = normalizationFixture("transaction-embedded-v2", files, ({ repoRoot, workspace }) => {
      const canonicalManifest = join(repoRoot, ticket, "🎫️ticket.json");
      mkdirSync(resolve(canonicalManifest, ".."), { recursive: true });
      writeFileSync(canonicalManifest, '{"id":"26/08/17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL"}\n');
      const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
      const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
      const contracts = taxonomy.fixedFilenameContracts as Record<string, unknown>;
      const directoryContracts = taxonomy.fixedDirectoryContracts as Record<string, unknown>;
      const scope = relative(repoRoot, workspace).replaceAll("\\", "/");
      taxonomy.fixedFilenameContracts = Object.fromEntries(Object.entries({ ...contracts, "fixture-cache-tag": { pathPattern: `${scope}/**/CACHEDIR.TAG`, authority: "Transaction disposition golden", reason: "Exact cache marker fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
      taxonomy.fixedDirectoryContracts = Object.fromEntries(Object.entries({ ...directoryContracts, "fixture-package-prefix": { pathPattern: `${scope}/pkg-*`, authority: "Transaction disposition golden", reason: "Exact embedded-root owner fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
      writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
    });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.embeddedTicketRoots).toHaveLength(3);
      expect(new Set(plan.embeddedTicketRoots.map((root) => root.ticketId))).toEqual(new Set(["26/08/17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL"]));
      expect(plan.embeddedTicketRootRelocations).toHaveLength(4);
      expect(plan.evidenceRemovals).toHaveLength(2);
      expect(plan.unresolved).toEqual([]);
      const rootTamper = JSON.parse(JSON.stringify(plan)) as TaxonomyPlan;
      (rootTamper.embeddedTicketRoots[0].relocationOperationIds as string[]).reverse();
      (rootTamper as { planDigest: string }).planDigest = taxonomyPlanDigest(rootTamper);
      expect(() => parseTaxonomyPlan(rootTamper)).toThrow();
      const removalTamper = JSON.parse(JSON.stringify(plan)) as TaxonomyPlan;
      (removalTamper.evidenceRemovals[0].authority as { evidenceSetDigest: string }).evidenceSetDigest = "0".repeat(64);
      (removalTamper as { planDigest: string }).planDigest = taxonomyPlanDigest(removalTamper);
      expect(() => parseTaxonomyPlan(removalTamper)).toThrow();
      const before = noFollowTreeDigest(fixture.repoRoot, fixture.scope);
      const cancellationTicket = fixture.ticketDir;
      const cancelFile = join(cancellationTicket, "cancel");
      const cancelled = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: cancellationTicket, expectedBaselineCommit: plan.baselineCommit, cancelFile, progress: (row) => { if (row.phase === "staging-evidence-removals") writeFileSync(cancelFile, "cancel\n"); } });
      expect(cancelled.state).toBe("rolled-back");
      rmSync(cancelFile, { force: true });
      expect(noFollowTreeDigest(fixture.repoRoot, fixture.scope)).toEqual(before);
      const retryTicket = fixture.ticketDir;
      const planArtifact = join(retryTicket, "📊️taxonomy-plan", "🔣️.json");
      mkdirSync(resolve(planArtifact, ".."), { recursive: true });
      writeFileSync(planArtifact, `${canonicalJson(plan)}\n`);
      const copiedConsumer = join(retryTicket, "🧪️consumer", "🔣️.json");
      mkdirSync(resolve(copiedConsumer, ".."), { recursive: true });
      writeFileSync(copiedConsumer, `${JSON.stringify({ path: plan.embeddedTicketRoots[0].sourceMetadataRoot })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit })).toThrow(/structured incoming reference/u);
      const priorAttempts = ownedFilePaths(retryTicket)
        .filter((path) => (path.startsWith("🧾️taxonomy-transaction/") || path.includes("/🧾️taxonomy-transaction/")) && path.endsWith("/🔣️.json"))
        .map((path) => join(retryTicket, path));
      expect(priorAttempts).toHaveLength(1);
      expect(JSON.parse(readFileSync(priorAttempts[0], "utf8")).state).toBe("rolled-back");
      rmSync(copiedConsumer);
      expect(applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, injectFailureAt: "after-relocations" }).state).toBe("rolled-back");
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit });
      expect(result.appliedEmbeddedTicketRootRelocations).toBe(4);
      expect(result.appliedEvidenceRemovals).toBe(2);
      expect(result.journalPath).toMatch(/🔢️000003\/🔣️\.json$/u);
      const attempts = ownedFilePaths(retryTicket)
        .filter((path) => (path.startsWith("🧾️taxonomy-transaction/") || path.includes("/🧾️taxonomy-transaction/")) && /\/🔂️attempts\/🔢️[^/]+\/🔣️\.json$/u.test(path))
        .map((path) => join(retryTicket, path))
        .sort(ownedPathByteSort);
      expect(attempts).toHaveLength(3);
      expect(attempts.map((path) => JSON.parse(readFileSync(path, "utf8")).state)).toEqual(["rolled-back", "rolled-back", "committed"]);
      const second = planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(second.embeddedTicketRoots).toEqual([]);
      expect(second.embeddedTicketRootRelocations).toEqual([]);
      expect(second.evidenceRemovals).toEqual([]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);
});

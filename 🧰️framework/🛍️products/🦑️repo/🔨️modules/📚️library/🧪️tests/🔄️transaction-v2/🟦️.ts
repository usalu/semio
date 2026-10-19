//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Transaction Plan/Journal v2 crash, recovery, and concurrency proof.
//#endregion 🧲️Header
import { describe, expect, test } from "bun:test";
import { spawn, type ChildProcess } from "node:child_process";
import { chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { applyTaxonomyPlan } from "../../🧹️normalization/🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { noFollowTreeDigest } from "../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { parseTaxonomyPlan } from "../../🧹️normalization/📃️plan-document/🟦️.ts";
import { taxonomyPlanDigest } from "../../🧹️normalization/🗃️affected-state/🟦️.ts";
import { type TaxonomyPlan } from "../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { ownedFilePaths, ownedFilesystemEntries } from "../🔍️filesystem/🟦️.ts";
import { EXACT_BOUNDARY_KEYS, FAILURE_STAGES, type Fixture, KILL_PHASES, LATE_KILL_PHASES, LEASE_PHASES, NORMALIZATION_MODULE, type ProtocolGolden, RESTORE_PHASES, SCHEMA_REL, type Snapshot, TICKET_REL, TRANSACTION_LEDGER_BOUNDARIES_GOLDEN, TRANSACTION_PROTOCOL_GOLDEN, registerChild, transactionGolden } from "./📜️boundary-contracts/🟦️.ts";
import { embeddedFixture, fixtureRunRoot, generatorFixture, git, journalPaths, plan, referenceFixture, retainFixture, snapshot, symlinkFixture, transactionRoot, writePlan } from "./🏗️fixture-builders/🟦️.ts";
import { activeReferenceFixture, attemptJournal, boundedExit, expectBoundaryGolden, expectBoundaryTuple, expectCompleteSnapshot, expectEmptyPlan, expectTerminalAttempts, killTree, killedAt, killedMixedGenerator, waitFor } from "./💥️child-control/🟦️.ts";

describe("transaction plan journal v2 aggregate", () => {
  test("keeps the language-neutral golden aligned with owned no-follow enumeration", async () => {
    const harness = JSON.parse(readFileSync(join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json"), "utf8"));
    const ts = await import("typescript");
    const launcher = ts.createSourceFile(harness.launcherPath, readFileSync(join(getWorkspaceRoot(), harness.launcherPath), "utf8"), ts.ScriptTarget.Latest, true);
    const allocationSource = ts.createSourceFile(harness.allocationPath, readFileSync(join(getWorkspaceRoot(), harness.allocationPath), "utf8"), ts.ScriptTarget.Latest, true);
    const testSource = ts.createSourceFile(harness.testPath, readFileSync(join(getWorkspaceRoot(), harness.testPath), "utf8"), ts.ScriptTarget.Latest, true);
    const builderSource = ts.createSourceFile(harness.builderPath, readFileSync(join(getWorkspaceRoot(), harness.builderPath), "utf8"), ts.ScriptTarget.Latest, true);
    const runValidator = builderSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === harness.runRootValidator);
    expect(runValidator).toBeDefined();
    const filterWaves: string[][][] = [], staticTitles: string[] = [];
    const visitLauncher = (node: import("typescript").Node): void => {
      if (ts.isVariableDeclaration(node) && node.name.getText(launcher) === harness.filterWavesConstant && node.initializer && ts.isArrayLiteralExpression(node.initializer)) filterWaves.push(node.initializer.elements.map((wave) => {
        if (!ts.isArrayLiteralExpression(wave) || wave.elements.some((entry) => !ts.isStringLiteral(entry))) throw new Error("Transaction shard filters must be closed literal strings");
        return wave.elements.map((entry) => (entry as import("typescript").StringLiteral).text);
      }));
      ts.forEachChild(node, visitLauncher);
    };
    const visitTests = (node: import("typescript").Node): void => {
      if (ts.isCallExpression(node) && ["test", "test.concurrent"].includes(node.expression.getText(testSource)) && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) staticTitles.push(node.arguments[0].text);
      ts.forEachChild(node, visitTests);
    };
    visitLauncher(launcher);
    visitTests(testSource);
    expect(filterWaves).toEqual([[harness.shards.map((entry: { filter: string }) => entry.filter)]]);
    expect(staticTitles).toHaveLength(14);
    const stages = FAILURE_STAGES.map((stage) => ({ title: `rolls back ${stage} with non-empty phase authority`, boundaries: [`rolledback:${stage}`] }));
    const phases = [...KILL_PHASES, ...RESTORE_PHASES, ...LEASE_PHASES].map((phase) => ({ title: `recovers parent-killed ${phase}`, boundaries: [`killed:${phase}`, `recovered:${phase}`] }));
    const terminal = ["committed", "rolled-back"].flatMap((state) => [`killed:transaction-terminal-${state}-stage-removed`, `recovered:transaction-terminal-${state}-stage-removed`]);
    const staticBoundaries: Record<string, string[]> = {
      "recovers parent-killed committed and rolled-back backup-only terminal cleanup": terminal,
      "rolls back a process-tree-killed mixed generator and commits ordinal two": ["killed:process-tree-mixed-generator", "recovered:process-tree-mixed-generator", "committed:process-tree-mixed-generator"],
      "keeps double-plan bytes stable, cancellation exact, and committed second apply immutable": ["rolledback:cancellation"],
      "recovers caught allocation and journal previous-image callback failures": ["rolledback:caught-attempt-canonical-published", "rolledback:caught-journal-previous-exchanged"],
    };
    const cases = [...staticTitles.map((title) => ({ title, boundaries: staticBoundaries[title] ?? [] })), ...stages, ...phases];
    expect(cases).toHaveLength(62);
    expect(new Set(cases.map((entry) => entry.title)).size).toBe(62);
    expect(cases.flatMap((entry) => entry.boundaries).sort()).toEqual(EXACT_BOUNDARY_KEYS);
    const filters = filterWaves[0][0].map((filter) => new RegExp(filter, "u"));
    for (const entry of cases) {
      expect(filters.filter((filter) => filter.test(entry.title))).toHaveLength(1);
      expect(filters.filter((filter) => filter.test(`transaction plan journal v2 aggregate ${entry.title}`))).toHaveLength(1);
    }
    expect(filters.map((filter) => cases.filter((entry) => filter.test(entry.title)).length)).toEqual(harness.shards.map((entry: { tests: number }) => entry.tests));
    expect(filters.map((filter) => cases.filter((entry) => filter.test(entry.title)).flatMap((entry) => entry.boundaries).length)).toEqual(harness.shards.map((entry: { boundaries: number }) => entry.boundaries));
    const allocationConstant = allocationSource.statements.find((node) => ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => declaration.name.getText(allocationSource) === harness.runOwnerConstant));
    const allocation = allocationSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === harness.bundleRootFunction);
    expect(allocationConstant).toBeDefined();
    expect(allocation).toBeDefined();
    const retention = builderSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === harness.retentionFunction);
    expect(retention).toBeDefined();
    const propagation: string[] = [];
    const inspectPropagation = (node: import("typescript").Node): void => {
      if (ts.isPropertyAssignment(node) && node.name.getText(launcher) === harness.runRootEnvironment) propagation.push(node.initializer.getText(launcher));
      ts.forEachChild(node, inspectPropagation);
    };
    inspectPropagation(launcher);
    expect(propagation).toEqual(["options.runRoot"]);
    const functionSource = [allocationConstant!.getText(allocationSource).replace(/^export\s+/u, ""), allocation!.getText(allocationSource).replace(/^export\s+/u, ""), runValidator!.getText(builderSource).replace(/^export\s+/u, ""), retention!.getText(builderSource).replace(/^export\s+/u, "")].join("\n");
    const compiled = [new Bun.Transpiler({ loader: "ts" }).transformSync(functionSource), ts.transpileModule(functionSource, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText];
    const roots: string[] = [];
    const allocationRepo = mkdtempSync(join(fixtureRunRoot(), "🧪️allocation-"));
    for (const code of compiled) {
      let currentRoot = "";
      const helpers = new Function("mkdirSync", "lstatSync", "join", "resolve", "isAbsolute", "parse", "relative", "sep", "dirname", "basename", "fixtureRunRoot", `${code}\nreturn { allocate: ${harness.bundleRootFunction}, validate: ${harness.runRootValidator}, retain: ${harness.retentionFunction} };`)(mkdirSync, lstatSync, join, resolve, isAbsolute, parse, relative, sep, dirname, basename, () => currentRoot) as { allocate: (repoRoot: string, runId: string) => string; validate: (repoRoot: string, runId: string, runRoot: string) => string; retain: (root: string) => void };
      const ids = [0, 1].map(() => `${process.pid}-${crypto.randomUUID()}`), pair: string[] = [];
      for (const id of ids) {
        const bundle = helpers.allocate(allocationRepo, id), root = dirname(bundle);
        pair.push(root);
        roots.push(root);
        expect(dirname(root)).toBe(join(allocationRepo, harness.runOwnerDirectory));
        expect(basename(root)).toBe(harness.runRootPrefix + id.slice(id.indexOf("-") + 1));
        expect(basename(bundle)).toBe(harness.bundleDirectory);
        expect(lstatSync(root).isDirectory()).toBe(true);
        expect(lstatSync(root).isSymbolicLink()).toBe(false);
        expect(helpers.validate(allocationRepo, id, root)).toBe(root);
        writeFileSync(join(bundle, "🔤️.txt"), "retained allocation input\n");
        expect(() => helpers.allocate(allocationRepo, id)).toThrow();
      }
      currentRoot = pair[0]!;
      const namesBefore = readdirSync(dirname(currentRoot)).sort();
      for (const id of harness.invalidRunIds) {
        expect(() => helpers.allocate(allocationRepo, id)).toThrow();
        expect(() => helpers.validate(allocationRepo, id, currentRoot)).toThrow();
      }
      expect(() => helpers.allocate("relative-root", ids[0]!)).toThrow();
      expect(() => helpers.validate(allocationRepo, ids[0]!, relative(allocationRepo, currentRoot))).toThrow();
      expect(() => helpers.validate(allocationRepo, ids[1]!, currentRoot)).toThrow();
      expect(readdirSync(dirname(currentRoot)).sort()).toEqual(namesBefore);
      const proofRepo = join(currentRoot, "🧪️ownership"), proofOwner = join(proofRepo, harness.runOwnerDirectory);
      mkdirSync(proofOwner, { recursive: true });
      const proofId = `${process.pid}-${crypto.randomUUID()}`, proofRoot = join(proofOwner, harness.runRootPrefix + proofId.slice(proofId.indexOf("-") + 1));
      symlinkSync(currentRoot, proofRoot, process.platform === "win32" ? "junction" : "dir");
      expect(() => helpers.allocate(proofRepo, proofId)).toThrow();
      expect(() => helpers.validate(proofRepo, proofId, proofRoot)).toThrow();
      expect(lstatSync(proofRoot).isSymbolicLink()).toBe(true);
      const aliasRepo = join(currentRoot, "🧪️alias");
      symlinkSync(proofRepo, aliasRepo, process.platform === "win32" ? "junction" : "dir");
      expect(() => helpers.allocate(aliasRepo, `${process.pid}-${crypto.randomUUID()}`)).toThrow();
      expect(() => helpers.validate(aliasRepo, proofId, join(aliasRepo, harness.runOwnerDirectory, basename(proofRoot)))).toThrow();
      const fileId = `${process.pid}-${crypto.randomUUID()}`, fileRoot = join(proofOwner, harness.runRootPrefix + fileId.slice(fileId.indexOf("-") + 1));
      writeFileSync(fileRoot, "not a directory\n");
      expect(() => helpers.allocate(proofRepo, fileId)).toThrow();
      expect(() => helpers.validate(proofRepo, fileId, fileRoot)).toThrow();
      expect(readFileSync(fileRoot, "utf8")).toBe("not a directory\n");
      for (const row of harness.cases) {
        const parent = row.parent === "run" ? currentRoot : row.parent === "ticket" ? join(getWorkspaceRoot(), TICKET_REL) : join(currentRoot, "🧪️nested");
        const root = join(parent, row.name);
        if (row.accepted) mkdirSync(root);
        if (row.accepted) expect(() => helpers.retain(root)).not.toThrow();
        else expect(() => helpers.retain(root)).toThrow();
      }
      const foreign = join(pair[1]!, "🧪️foreign"), fixtureLink = join(currentRoot, "🧪️link");
      mkdirSync(foreign);
      symlinkSync(foreign, fixtureLink, process.platform === "win32" ? "junction" : "dir");
      expect(() => helpers.retain(foreign)).toThrow();
      expect(() => helpers.retain(fixtureLink)).toThrow();
      expect(lstatSync(fixtureLink).isSymbolicLink()).toBe(true);
      for (const root of pair) expect(readFileSync(join(root, harness.bundleDirectory, "🔤️.txt"), "utf8")).toBe("retained allocation input\n");
    }
    expect(new Set(roots).size).toBe(4);
    const protocolGolden = JSON.parse(readFileSync(TRANSACTION_PROTOCOL_GOLDEN, "utf8")) as ProtocolGolden;
    expect(Object.keys(transactionGolden).sort()).toEqual(["boundaries", "schemaVersion", "transactionLedgers", "workspaceLedgers"]);
    expect(Object.keys(protocolGolden).sort()).toEqual(["failureStages", "journalStates", "schemaVersion", "virtualPreimageNodes"]);
    expect(ownedFilePaths(resolve(TRANSACTION_LEDGER_BOUNDARIES_GOLDEN, ".."))).toEqual(["🔣️.json"]);
    expect(ownedFilePaths(resolve(TRANSACTION_PROTOCOL_GOLDEN, ".."))).toEqual(["🔣️.json"]);
    expect(protocolGolden.failureStages).toEqual(FAILURE_STAGES);
    expect(protocolGolden.journalStates).toHaveLength(11);
    expect(Object.keys(transactionGolden.boundaries).sort()).toEqual(EXACT_BOUNDARY_KEYS);
    for (const [owner, ledgers] of [["transaction", transactionGolden.transactionLedgers], ["workspace", transactionGolden.workspaceLedgers]] as const) for (const [digest, nodes] of Object.entries(ledgers ?? {})) {
      expect(new Bun.CryptoHasher("sha256").update(canonicalJson(nodes)).digest("hex")).toBe(digest);
      for (const node of nodes) {
        if (node.kind === "symlink") expect(new Bun.CryptoHasher("sha256").update(node.target).digest("hex")).toBe(node.targetHash);
        if (node.kind === "file") {
          const bytes = Buffer.from(node.bytesBase64, "base64");
          expect(bytes.length).toBe(node.size);
          expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(node.sha256);
          if (node.json !== undefined) { if (owner === "transaction") expect(node.canonicalJson).toBe(true); expect(bytes.toString("utf8")).toBe(`${canonicalJson(node.json)}\n`); }
        }
      }
    }
    const referencedTransactions = [...new Set(Object.values(transactionGolden.boundaries).map((entry) => entry.transaction))].sort(), referencedWorkspaces = [...new Set(Object.values(transactionGolden.boundaries).map((entry) => entry.workspace))].sort();
    expect(Object.keys(transactionGolden.transactionLedgers).sort()).toEqual(referencedTransactions);
    expect(Object.keys(transactionGolden.workspaceLedgers).sort()).toEqual(referencedWorkspaces);
    for (const reference of Object.values(transactionGolden.boundaries)) {
      expect(transactionGolden.transactionLedgers[reference.transaction]).toBeDefined();
      expect(transactionGolden.workspaceLedgers[reference.workspace]).toBeDefined();
    }
    const root = mkdtempSync(join(fixtureRunRoot(), "🧪️parity-"));
    try {
      mkdirSync(join(root, "evidence", "directory"), { recursive: true });
      writeFileSync(join(root, "evidence", "file.txt"), "sentinel\n");
      symlinkSync("../file.txt", join(root, "evidence", "link"));
      expect(ownedFilesystemEntries(join(root, "evidence"), true).map(({ path }) => path === "." ? "evidence" : `evidence/${path}`)).toEqual(["evidence", "evidence/directory", "evidence/file.txt", "evidence/link"]);
      expect(noFollowTreeDigest(root, "evidence")).toMatchObject({ directories: 2, files: 1, symlinks: 1 });
      expect(protocolGolden.virtualPreimageNodes.some((entry) => entry.state === "symlink")).toBe(true);
    } finally { retainFixture(root); }
  });

  test("rejects incomplete plans and plan-digest drift", async () => {
    expect(() => parseTaxonomyPlan({ schemaVersion: 1 })).toThrow();
    const row = referenceFixture("strict-plan");
    try {
      expect(git(row.root, ["ls-files", "--", SCHEMA_REL])).toBe("");
      const value = plan(row), drift = JSON.parse(JSON.stringify(value)) as TaxonomyPlan;
      (drift as { planDigest: string }).planDigest = "0".repeat(64);
      expect(() => parseTaxonomyPlan(drift)).toThrow();
      expect(taxonomyPlanDigest(value)).toBe(value.planDigest);
      const failed = registerChild(spawn(process.execPath, ["-e", 'process.stderr.write("deliberate fixture child failure\\n"); process.exit(7);'], { stdio: ["ignore", "pipe", "pipe"] }));
      await expect(waitFor(join(row.root, "missing-marker"), 1_000, failed)).rejects.toThrow("deliberate fixture child failure");
      expect(failed.exitCode).toBe(7);
    } finally { retainFixture(row.root); }
  });

  const failureCases = [
    { stage: "after-staging", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.moves.length },
    { stage: "after-embedded-root-staging", fixture: embeddedFixture, relevant: (value: TaxonomyPlan) => value.embeddedTicketRoots.length },
    { stage: "after-moves", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.moves.length },
    { stage: "after-relocations", fixture: embeddedFixture, relevant: (value: TaxonomyPlan) => value.embeddedTicketRootRelocations.length },
    { stage: "after-symlink-retargeting", fixture: symlinkFixture, relevant: (value: TaxonomyPlan) => value.symlinkTargetEdits.length },
    { stage: "after-edits", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.edits.length },
    { stage: "after-regenerations", fixture: generatorFixture, relevant: (value: TaxonomyPlan) => value.regenerations.length },
    { stage: "before-verify", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.moves.length + value.edits.length },
  ] as const;

  for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, () => {
    const row = failure.fixture(`failure-${failure.stage}`);
    try {
      const value = plan(row), before = snapshot(row.workspace);
      expect(failure.relevant(value)).toBeGreaterThan(0);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: value.planDigest, injectFailureAt: failure.stage }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `rolledback:${failure.stage}`, row.root);
      if (failure.stage === "before-verify") {
        const retry = applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit });
        expect(retry.state).toBe("committed");
        expect(retry.journalPath).toMatch(/🔢️000002\/🔣️\.json$/u);
        expectEmptyPlan(row);
      }
    } finally { retainFixture(row.root); }
  }, 15_000);

  for (const phase of KILL_PHASES) test.concurrent(`recovers parent-killed ${phase}`, async () => {
    const active = LATE_KILL_PHASES.has(phase), row = active ? await activeReferenceFixture(`kill-${phase}`) : referenceFixture(`kill-${phase}`);
    try {
      const value = plan(row), before = snapshot(row.workspace), killed = await killedAt(row, value, phase, active ? attemptJournal(row, value) : "");
      expectBoundaryTuple(killed.transaction, killed.workspace, phase, row.root);
      const journal = attemptJournal(row, value), canonicalAttempt = existsSync(dirname(journal));
      const injectFailureAt = phase.startsWith("transaction-backup") || phase.startsWith("transaction-edit") ? "after-edits" : "after-staging";
      const result = applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, ...(canonicalAttempt ? { resumeJournal: journal } : {}), injectFailureAt });
      expect(result.state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `recovered:${phase}`, row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  for (const phase of RESTORE_PHASES) test.concurrent(`recovers parent-killed ${phase}`, async () => {
    const row = await activeReferenceFixture(`kill-${phase}`);
    try {
      const value = plan(row), before = snapshot(row.workspace), killed = await killedAt(row, value, phase, attemptJournal(row, value), "after-edits");
      expectBoundaryTuple(killed.transaction, killed.workspace, phase, row.root);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value) }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `recovered:${phase}`, row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  for (const phase of LEASE_PHASES) test.concurrent(`recovers parent-killed ${phase}`, async () => {
    const row = await activeReferenceFixture(`kill-${phase}`);
    try {
      const value = plan(row), before = snapshot(row.workspace);
      const killed = await killedAt(row, value, phase, attemptJournal(row, value));
      expectBoundaryTuple(killed.transaction, killed.workspace, phase, row.root);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value), injectFailureAt: "after-staging" }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `recovered:${phase}`, row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  test.concurrent("recovers parent-killed committed and rolled-back backup-only terminal cleanup", async () => {
    const committed = referenceFixture("terminal-committed"), rolledBack = referenceFixture("terminal-rolled-back");
    try {
      const committedPlan = plan(committed), committedKill = await killedAt(committed, committedPlan, "transaction-terminal-committed-stage-removed");
      expectBoundaryTuple(committedKill.transaction, committedKill.workspace, "transaction-terminal-committed-stage-removed", committed.root);
      expect(applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit, resumeJournal: attemptJournal(committed, committedPlan) }).state).toBe("committed");
      expectTerminalAttempts(committed, "committed");
      expectBoundaryGolden(snapshot(transactionRoot(committed)), snapshot(committed.workspace), "recovered:transaction-terminal-committed-stage-removed", committed.root);
      const rolledPlan = plan(rolledBack), before = snapshot(rolledBack.workspace), rolledKill = await killedAt(rolledBack, rolledPlan, "transaction-terminal-rolled-back-stage-removed", "", "after-edits");
      expectBoundaryTuple(rolledKill.transaction, rolledKill.workspace, "transaction-terminal-rolled-back-stage-removed", rolledBack.root);
      expect(applyTaxonomyPlan(rolledPlan, { repoRoot: rolledBack.repoRoot, ticketDir: rolledBack.ticketDir, expectedBaselineCommit: rolledBack.baselineCommit, injectFailureAt: "after-staging" }).state).toBe("rolled-back");
      expect(snapshot(rolledBack.workspace)).toEqual(before);
      expect(journalPaths(rolledBack).map((path) => JSON.parse(readFileSync(path, "utf8")).state)).toEqual(["rolled-back", "rolled-back"]);
      for (const journal of journalPaths(rolledBack)) expect(readdirSync(dirname(journal))).toEqual(["🔣️.json"]);
      expectBoundaryGolden(snapshot(transactionRoot(rolledBack)), snapshot(rolledBack.workspace), "recovered:transaction-terminal-rolled-back-stage-removed", rolledBack.root);
    } finally { retainFixture(committed.root); retainFixture(rolledBack.root); }
  }, 15_000);

  test.concurrent("rolls back a process-tree-killed mixed generator and commits ordinal two", async () => {
    const row = generatorFixture("mixed-generator");
    try {
      const value = plan(row), before = snapshot(row.workspace);
      expect(value.regenerations).toHaveLength(1);
      const killed = await killedMixedGenerator(row, value);
      expectCompleteSnapshot(killed.transaction);
      expectCompleteSnapshot(killed.workspace);
      expectBoundaryGolden(killed.transaction, killed.workspace, "killed:process-tree-mixed-generator", row.root);
      expect(Object.values(killed.workspace).some((entry) => entry.includes(Buffer.from("mixed\n").toString("base64")))).toBe(true);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: killed.journal }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), "recovered:process-tree-mixed-generator", row.root);
      const committed = applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit });
      expect(committed.journalPath).toMatch(/🔢️000002\/🔣️\.json$/u);
      expect(journalPaths(row).map((path) => JSON.parse(readFileSync(path, "utf8")).state)).toEqual(["rolled-back", "committed"]);
      expectTerminalAttempts(row, "committed");
      expectEmptyPlan(row);
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), "committed:process-tree-mixed-generator", row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  test.concurrent("elects exactly one synchronized stale-lease contender and permits committed retry", async () => {
    const row = await activeReferenceFixture("contenders");
    let contenders: ChildProcess[] = [];
    try {
      const value = plan(row);
      const journal = attemptJournal(row, value), beforeTransaction = snapshot(transactionRoot(row)), beforeWorkspace = snapshot(row.workspace);
      const barrier = mkdtempSync(join(row.root, "barrier-")), acquireRelease = join(barrier, "acquire"), winnerRelease = join(barrier, "winner"), owner = join(barrier, "owner");
      const contenderSource = `const [m,p,r,t,baseline,j,b,acquire,winner,owner]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);const fs=require("node:fs"),wait=(path)=>{while(!fs.existsSync(path))Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,2)},result=b+"/result-"+process.pid;try{applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{repoRoot:r,ticketDir:t,expectedBaselineCommit:baseline,resumeJournal:j,progress:(row)=>{if(row.phase==="transaction-lease-scanned"){fs.writeFileSync(b+"/ready-"+process.pid,"ready\\n");wait(acquire)}if(row.phase==="transaction-lease-canonical-published"){fs.writeFileSync(owner,String(process.pid));wait(winner)}}});fs.writeFileSync(result,"success\\n")}catch(error){fs.writeFileSync(result,"failure:"+(error instanceof Error?error.message:String(error))+"\\n");process.exitCode=17}`;
      const planPath = writePlan(row, value);
      contenders = [0, 1].map(() => registerChild(spawn(process.execPath, ["-e", contenderSource, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, journal, barrier, acquireRelease, winnerRelease, owner], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] })));
      const deadline = Date.now() + 10_000;
      while (readdirSync(barrier).filter((name) => name.startsWith("ready-")).length < 2) { if (Date.now() >= deadline) throw new Error("Contender barrier timed out"); await Bun.sleep(2); }
      writeFileSync(acquireRelease, "go\n");
      while (!existsSync(owner) || readdirSync(barrier).filter((name) => name.startsWith("result-") && readFileSync(join(barrier, name), "utf8").startsWith("failure:")).length !== 1) { if (Date.now() >= deadline) throw new Error("Contender election timed out"); await Bun.sleep(2); }
      expect(snapshot(row.workspace)).toEqual(beforeWorkspace);
      expect(snapshot(transactionRoot(row))).not.toEqual(beforeTransaction);
      expect(Object.keys(snapshot(transactionRoot(row))).some((path) => path.includes("-preparing"))).toBe(false);
      writeFileSync(winnerRelease, "go\n");
      const exits = await Promise.all(contenders.map((childProcess) => boundedExit(childProcess)));
      expect(exits.filter((entry) => entry.code === 0)).toHaveLength(1);
      expect(exits.filter((entry) => entry.code === 17)).toHaveLength(1);
      const failure = readdirSync(barrier).filter((name) => name.startsWith("result-")).map((name) => readFileSync(join(barrier, name), "utf8")).find((value) => value.startsWith("failure:"));
      expect(failure).toMatch(/lease acquisition failed|leased by active pid|concurrent canonical lease|stale lease destination changed/u);
      expectTerminalAttempts(row, "committed");
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: journal }).state).toBe("committed");
      expectTerminalAttempts(row, "committed");
    } finally {
      for (const contender of contenders) if (contender.exitCode === null && contender.signalCode === null) killTree(contender);
      await Promise.all(contenders.map((contender) => boundedExit(contender, 5_000).catch(() => undefined)));
      for (const contender of contenders) expect(contender.exitCode !== null || contender.signalCode !== null).toBe(true);
      retainFixture(row.root);
    }
  }, 15_000);

  test.concurrent("restores a quarantined stale lease exactly when acquisition callback throws", async () => {
    const row = await activeReferenceFixture("stale-lease-restoration");
    try {
      const value = plan(row);
      const beforeTransaction = snapshot(transactionRoot(row)), beforeWorkspace = snapshot(row.workspace);
      expect(() => applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value), progress: (event) => { if (event.phase === "transaction-lease-stale-quarantined") throw new Error("forced callback failure"); } })).toThrow(/forced callback failure/u);
      expect(snapshot(transactionRoot(row))).toEqual(beforeTransaction);
      expect(snapshot(row.workspace)).toEqual(beforeWorkspace);
    } finally { retainFixture(row.root); }
  }, 15_000);

  test.concurrent("rejects stale baseline, source digest, and counterfeit transaction segment with zero mutation", () => {
    const baseline = referenceFixture("stale-baseline"), source = referenceFixture("stale-source"), counterfeit = referenceFixture("counterfeit-segment");
    try {
      const baselinePlan = plan(baseline), baselineTransaction = snapshot(transactionRoot(baseline)), baselineWorkspace = snapshot(baseline.workspace);
      expect(() => applyTaxonomyPlan(baselinePlan, { repoRoot: baseline.repoRoot, ticketDir: baseline.ticketDir, expectedBaselineCommit: "0".repeat(40) })).toThrow(/expectedBaselineCommit/u);
      expect(snapshot(transactionRoot(baseline))).toEqual(baselineTransaction);
      expect(snapshot(baseline.workspace)).toEqual(baselineWorkspace);
      const sourcePlan = plan(source);
      writeFileSync(join(source.workspace, "unrelated.txt"), "drift\n");
      const sourceTransaction = snapshot(transactionRoot(source)), sourceWorkspace = snapshot(source.workspace);
      expect(() => applyTaxonomyPlan(sourcePlan, { repoRoot: source.repoRoot, ticketDir: source.ticketDir, expectedBaselineCommit: source.baselineCommit })).toThrow(/source-tree digest/u);
      expect(snapshot(transactionRoot(source))).toEqual(sourceTransaction);
      expect(snapshot(source.workspace)).toEqual(sourceWorkspace);
      const counterfeitPlan = plan(counterfeit), counterfeitPath = join(counterfeit.workspace, "🧾️taxonomy-transaction", "plain.txt");
      mkdirSync(dirname(counterfeitPath), { recursive: true });
      writeFileSync(counterfeitPath, "counterfeit\n");
      const counterfeitTransaction = snapshot(transactionRoot(counterfeit)), counterfeitWorkspace = snapshot(counterfeit.workspace);
      expect(() => applyTaxonomyPlan(counterfeitPlan, { repoRoot: counterfeit.repoRoot, ticketDir: counterfeit.ticketDir, expectedBaselineCommit: counterfeit.baselineCommit })).toThrow();
      expect(snapshot(transactionRoot(counterfeit))).toEqual(counterfeitTransaction);
      expect(snapshot(counterfeit.workspace)).toEqual(counterfeitWorkspace);
      const inventory = inventoryTaxonomy(counterfeit.options);
      expect(inventory.entries.some((entry) => entry.sourcePath.endsWith("🧾️taxonomy-transaction/plain.txt"))).toBe(true);
    } finally { retainFixture(baseline.root); retainFixture(source.root); retainFixture(counterfeit.root); }
  }, 15_000);

  test.concurrent("rejects stale resume preimage and malformed resume evidence byte-for-byte", async () => {
    const stale = await activeReferenceFixture("stale-resume"), forged = await activeReferenceFixture("forged-resume"), missing = referenceFixture("missing-resume");
    try {
      const stalePlan = plan(stale);
      writeFileSync(join(stale.workspace, "🧪️subject", "./🟦️.ts"), "export const value = 2;\n");
      const staleTransaction = snapshot(transactionRoot(stale)), staleWorkspace = snapshot(stale.workspace);
      expect(() => applyTaxonomyPlan(stalePlan, { repoRoot: stale.repoRoot, ticketDir: stale.ticketDir, expectedBaselineCommit: stale.baselineCommit, resumeJournal: attemptJournal(stale, stalePlan) })).toThrow(/resume-state-drift/u);
      expect(snapshot(transactionRoot(stale))).toEqual(staleTransaction);
      expect(snapshot(stale.workspace)).toEqual(staleWorkspace);
      const forgedPlan = plan(forged);
      writeFileSync(attemptJournal(forged, forgedPlan), "{}\n");
      const forgedTransaction = snapshot(transactionRoot(forged)), forgedWorkspace = snapshot(forged.workspace);
      expect(() => applyTaxonomyPlan(forgedPlan, { repoRoot: forged.repoRoot, ticketDir: forged.ticketDir, expectedBaselineCommit: forged.baselineCommit, resumeJournal: attemptJournal(forged, forgedPlan) })).toThrow();
      expect(snapshot(transactionRoot(forged))).toEqual(forgedTransaction);
      expect(snapshot(forged.workspace)).toEqual(forgedWorkspace);
      const missingPlan = plan(missing), missingTransaction = snapshot(transactionRoot(missing)), missingWorkspace = snapshot(missing.workspace);
      expect(() => applyTaxonomyPlan(missingPlan, { repoRoot: missing.repoRoot, ticketDir: missing.ticketDir, expectedBaselineCommit: missing.baselineCommit, resumeJournal: attemptJournal(missing, missingPlan) })).toThrow(/exact existing canonical attempt/u);
      expect(snapshot(transactionRoot(missing))).toEqual(missingTransaction);
      expect(snapshot(missing.workspace)).toEqual(missingWorkspace);
    } finally { retainFixture(stale.root); retainFixture(forged.root); retainFixture(missing.root); }
  }, 15_000);

  test.concurrent("rejects stale generator and embedded-reference authority before mutation", async () => {
    const generator = generatorFixture("stale-generator"), embedded = embeddedFixture("stale-reference");
    try {
      const generatorPlan = plan(generator);
      writeFileSync(join(generator.workspace, "🧪️generator", "./🟦️.ts"), "export const input = false;\n");
      const generatorTransaction = snapshot(transactionRoot(generator)), generatorWorkspace = snapshot(generator.workspace);
      expect(() => applyTaxonomyPlan(generatorPlan, { repoRoot: generator.repoRoot, ticketDir: generator.ticketDir, expectedBaselineCommit: generator.baselineCommit })).toThrow(/Regeneration input preimage changed/u);
      expect(snapshot(transactionRoot(generator))).toEqual(generatorTransaction);
      expect(snapshot(generator.workspace)).toEqual(generatorWorkspace);
      const embeddedPlan = plan(embedded);
      await killedAt(embedded, embeddedPlan, "transaction-wal-prepared");
      const reference = join(embedded.workspace, "incoming.json");
      writeFileSync(reference, `${JSON.stringify({ path: embeddedPlan.embeddedTicketRoots[0].sourceMetadataRoot })}\n`);
      const embeddedTransaction = snapshot(transactionRoot(embedded)), embeddedWorkspace = snapshot(embedded.workspace);
      expect(() => applyTaxonomyPlan(embeddedPlan, { repoRoot: embedded.repoRoot, ticketDir: embedded.ticketDir, expectedBaselineCommit: embedded.baselineCommit, resumeJournal: attemptJournal(embedded, embeddedPlan) })).toThrow(/embedded incoming references/u);
      expect(snapshot(transactionRoot(embedded))).toEqual(embeddedTransaction);
      expect(snapshot(embedded.workspace)).toEqual(embeddedWorkspace);
    } finally { retainFixture(generator.root); retainFixture(embedded.root); }
  }, 15_000);

  test.concurrent("rejects ordinal collisions and malformed attempt siblings without mutation", async () => {
    const canonical = await activeReferenceFixture("canonical-collision"), duplicate = await activeReferenceFixture("duplicate-collision"), future = await activeReferenceFixture("future-collision"), malformed = await activeReferenceFixture("malformed-sibling");
    try {
      const canonicalPlan = plan(canonical), canonicalAttempts = dirname(dirname(attemptJournal(canonical, canonicalPlan)));
      mkdirSync(join(canonicalAttempts, "🚧️prepare-000001-999999-00000000-0000-4000-8000-000000000001"));
      const canonicalTransaction = snapshot(transactionRoot(canonical)), canonicalWorkspace = snapshot(canonical.workspace);
      expect(() => applyTaxonomyPlan(canonicalPlan, { repoRoot: canonical.repoRoot, ticketDir: canonical.ticketDir, expectedBaselineCommit: canonical.baselineCommit, resumeJournal: attemptJournal(canonical, canonicalPlan) })).toThrow(/collides with canonical ordinal/u);
      expect(snapshot(transactionRoot(canonical))).toEqual(canonicalTransaction);
      expect(snapshot(canonical.workspace)).toEqual(canonicalWorkspace);
      const duplicatePlan = plan(duplicate), duplicateAttempts = dirname(dirname(attemptJournal(duplicate, duplicatePlan)));
      mkdirSync(join(duplicateAttempts, "🚧️prepare-000002-999998-00000000-0000-4000-8000-000000000002"));
      mkdirSync(join(duplicateAttempts, "🚧️prepare-000002-999999-00000000-0000-4000-8000-000000000003"));
      const duplicateTransaction = snapshot(transactionRoot(duplicate)), duplicateWorkspace = snapshot(duplicate.workspace);
      expect(() => applyTaxonomyPlan(duplicatePlan, { repoRoot: duplicate.repoRoot, ticketDir: duplicate.ticketDir, expectedBaselineCommit: duplicate.baselineCommit, resumeJournal: attemptJournal(duplicate, duplicatePlan) })).toThrow(/duplicate ordinal/u);
      expect(snapshot(transactionRoot(duplicate))).toEqual(duplicateTransaction);
      expect(snapshot(duplicate.workspace)).toEqual(duplicateWorkspace);
      const futurePlan = plan(future), futureAttempts = dirname(dirname(attemptJournal(future, futurePlan)));
      mkdirSync(join(futureAttempts, "🚧️prepare-000003-999999-00000000-0000-4000-8000-000000000004"));
      const futureTransaction = snapshot(transactionRoot(future)), futureWorkspace = snapshot(future.workspace);
      expect(() => applyTaxonomyPlan(futurePlan, { repoRoot: future.repoRoot, ticketDir: future.ticketDir, expectedBaselineCommit: future.baselineCommit, resumeJournal: attemptJournal(future, futurePlan) })).toThrow(/not exact next ordinal/u);
      expect(snapshot(transactionRoot(future))).toEqual(futureTransaction);
      expect(snapshot(future.workspace)).toEqual(futureWorkspace);
      const malformedPlan = plan(malformed), malformedAttempts = dirname(dirname(attemptJournal(malformed, malformedPlan)));
      mkdirSync(join(malformedAttempts, "malformed"));
      const malformedTransaction = snapshot(transactionRoot(malformed)), malformedWorkspace = snapshot(malformed.workspace);
      expect(() => applyTaxonomyPlan(malformedPlan, { repoRoot: malformed.repoRoot, ticketDir: malformed.ticketDir, expectedBaselineCommit: malformed.baselineCommit, resumeJournal: attemptJournal(malformed, malformedPlan) })).toThrow(/Unexpected transaction attempt entry/u);
      expect(snapshot(transactionRoot(malformed))).toEqual(malformedTransaction);
      expect(snapshot(malformed.workspace)).toEqual(malformedWorkspace);
    } finally { retainFixture(canonical.root); retainFixture(duplicate.root); retainFixture(future.root); retainFixture(malformed.root); }
  }, 15_000);

  test.concurrent("rejects forged backup and restore preparations without mutation", async () => {
    const backup = referenceFixture("forged-backup"), restore = referenceFixture("forged-restore");
    try {
      const backupPlan = plan(backup);
      const backupKill = await killedAt(backup, backupPlan, "transaction-backup-retained"), backupLeaf = Object.keys(backupKill.transaction).find((path) => path.endsWith(".backup"))!;
      writeFileSync(join(transactionRoot(backup), backupLeaf), "forged\n");
      const backupTransaction = snapshot(transactionRoot(backup)), backupWorkspace = snapshot(backup.workspace);
      expect(() => applyTaxonomyPlan(backupPlan, { repoRoot: backup.repoRoot, ticketDir: backup.ticketDir, expectedBaselineCommit: backup.baselineCommit, resumeJournal: attemptJournal(backup, backupPlan) })).toThrow(/backup/u);
      expect(snapshot(transactionRoot(backup))).toEqual(backupTransaction);
      expect(snapshot(backup.workspace)).toEqual(backupWorkspace);
      const restorePlan = plan(restore);
      const restoreKill = await killedAt(restore, restorePlan, "transaction-restore-prepared", "", "after-edits"), restoreLeaf = Object.keys(restoreKill.transaction).find((path) => path.includes("restore-") && path.endsWith(".backup"))!;
      writeFileSync(join(transactionRoot(restore), restoreLeaf), "forged\n");
      const restoreTransaction = snapshot(transactionRoot(restore)), restoreWorkspace = snapshot(restore.workspace);
      expect(() => applyTaxonomyPlan(restorePlan, { repoRoot: restore.repoRoot, ticketDir: restore.ticketDir, expectedBaselineCommit: restore.baselineCommit, resumeJournal: attemptJournal(restore, restorePlan) })).toThrow(/Restore preparation bytes/u);
      expect(snapshot(transactionRoot(restore))).toEqual(restoreTransaction);
      expect(snapshot(restore.workspace)).toEqual(restoreWorkspace);
    } finally { retainFixture(backup.root); retainFixture(restore.root); }
  }, 15_000);

  test.concurrent("rejects unreachable duplicate backup and edit publication tuples exactly", async () => {
    const backup = referenceFixture("duplicate-backup-tuple"), edit = referenceFixture("duplicate-edit-tuple"), exchanged = referenceFixture("writer-edit-tuple");
    try {
      const backupPlan = plan(backup), editPlan = plan(edit), exchangedPlan = plan(exchanged);
      const [backupKilled, editKilled, exchangedKilled] = await Promise.all([
        killedAt(backup, backupPlan, "transaction-backup-inner-exchange"),
        killedAt(edit, editPlan, "transaction-edit-inner-exchange"),
        killedAt(exchanged, exchangedPlan, "transaction-edit-exchange"),
      ]);
      const duplicateNestedLeaf = (row: Fixture, killed: Readonly<{ transaction: Snapshot }>, outerSuffix: ".backup" | ".edit"): void => {
        const writeRoot = Object.keys(killed.transaction).find((path) => killed.transaction[path].startsWith("directory|") && /write-[1-9][0-9]*-[0-9a-f-]+$/u.test(path));
        const outer = Object.keys(killed.transaction).find((path) => path.endsWith(outerSuffix) && !path.startsWith(`${writeRoot}/`));
        if (!writeRoot || !outer) throw new Error(`Missing duplicate tuple authority for ${outerSuffix}`);
        const target = join(transactionRoot(row), writeRoot, outerSuffix === ".backup" ? "🚧️.backup" : "🚧️.edit");
        writeFileSync(target, readFileSync(join(transactionRoot(row), outer)));
        chmodSync(target, Number.parseInt(killed.transaction[outer].split("|")[1], 8));
      };
      duplicateNestedLeaf(backup, backupKilled, ".backup");
      duplicateNestedLeaf(edit, editKilled, ".edit");
      const editPreparation = Object.keys(exchangedKilled.transaction).find((path) => exchangedKilled.transaction[path].startsWith("directory|") && path.includes("edit-"));
      if (!editPreparation) throw new Error("Missing exchanged edit preparation");
      mkdirSync(join(transactionRoot(exchanged), editPreparation, `🚧️write-999999-${crypto.randomUUID()}`));
      for (const [row, value, pattern] of [[backup, backupPlan, /duplicate outer and nested/u], [edit, editPlan, /duplicate outer and nested/u], [exchanged, exchangedPlan, /writer coexists/u]] as const) {
        const beforeTransaction = snapshot(transactionRoot(row)), beforeWorkspace = snapshot(row.workspace);
        expect(() => applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value) })).toThrow(pattern);
        expect(snapshot(transactionRoot(row))).toEqual(beforeTransaction);
        expect(snapshot(row.workspace)).toEqual(beforeWorkspace);
      }
    } finally { retainFixture(backup.root); retainFixture(edit.root); retainFixture(exchanged.root); }
  }, 15_000);

  test.concurrent("keeps double-plan bytes stable, cancellation exact, and committed second apply immutable", () => {
    const stable = referenceFixture("double-plan"), cancelled = referenceFixture("cancelled"), committed = referenceFixture("second-apply");
    try {
      const first = plan(stable), second = plan(stable);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(first.planDigest).toBe(second.planDigest);
      const cancelledPlan = plan(cancelled), cancelledBefore = snapshot(cancelled.workspace), cancelFile = join(cancelled.ticketDir, "cancel");
      let cancellationFired = false;
      expect(cancelledPlan.moves.length).toBeGreaterThan(0);
      expect(applyTaxonomyPlan(cancelledPlan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: cancelled.baselineCommit, cancelFile, progress: (event) => { if (event.phase === "staging") { cancellationFired = true; writeFileSync(cancelFile, "cancel\n"); } } }).state).toBe("rolled-back");
      expect(cancellationFired).toBe(true);
      rmSync(cancelFile);
      expect(snapshot(cancelled.workspace)).toEqual(cancelledBefore);
      expectTerminalAttempts(cancelled, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(cancelled)), snapshot(cancelled.workspace), "rolledback:cancellation", cancelled.root);
      const cancelledTransaction = snapshot(transactionRoot(cancelled));
      expect(() => applyTaxonomyPlan(cancelledPlan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: cancelled.baselineCommit, resumeJournal: attemptJournal(cancelled, cancelledPlan) })).toThrow(/rolled-back/u);
      expect(snapshot(transactionRoot(cancelled))).toEqual(cancelledTransaction);
      expect(snapshot(cancelled.workspace)).toEqual(cancelledBefore);
      const committedPlan = plan(committed);
      expect(applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit }).state).toBe("committed");
      const committedTransaction = snapshot(transactionRoot(committed)), committedWorkspace = snapshot(committed.workspace);
      expect(() => applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit })).toThrow(/already has a committed/u);
      expect(snapshot(transactionRoot(committed))).toEqual(committedTransaction);
      expect(snapshot(committed.workspace)).toEqual(committedWorkspace);
    } finally { retainFixture(stable.root); retainFixture(cancelled.root); retainFixture(committed.root); }
  }, 15_000);

  test.concurrent("recovers caught allocation and journal previous-image callback failures", () => {
    const preparation = referenceFixture("caught-preparation"), published = referenceFixture("caught-published"), previous = referenceFixture("caught-previous");
    try {
      const preparationPlan = plan(preparation), preparationWorkspace = snapshot(preparation.workspace), preparationTransaction = snapshot(transactionRoot(preparation));
      expect(() => applyTaxonomyPlan(preparationPlan, { repoRoot: preparation.repoRoot, ticketDir: preparation.ticketDir, expectedBaselineCommit: preparation.baselineCommit, progress: (event) => { if (event.phase === "transaction-attempt-preparation-mkdir") throw new Error("caught preparation"); } })).toThrow(/caught preparation/u);
      expect(snapshot(preparation.workspace)).toEqual(preparationWorkspace);
      expect(snapshot(transactionRoot(preparation))).toEqual(preparationTransaction);
      const publishedPlan = plan(published), publishedWorkspace = snapshot(published.workspace);
      expect(applyTaxonomyPlan(publishedPlan, { repoRoot: published.repoRoot, ticketDir: published.ticketDir, expectedBaselineCommit: published.baselineCommit, progress: (event) => { if (event.phase === "transaction-attempt-canonical-published") throw new Error("caught published"); } }).state).toBe("rolled-back");
      expect(snapshot(published.workspace)).toEqual(publishedWorkspace);
      expectTerminalAttempts(published, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(published)), snapshot(published.workspace), "rolledback:caught-attempt-canonical-published", published.root);
      const previousPlan = plan(previous), previousWorkspace = snapshot(previous.workspace);
      expect(applyTaxonomyPlan(previousPlan, { repoRoot: previous.repoRoot, ticketDir: previous.ticketDir, expectedBaselineCommit: previous.baselineCommit, progress: (event) => { if (event.phase === "transaction-journal-previous-exchanged") throw new Error("caught previous"); } }).state).toBe("rolled-back");
      expect(snapshot(previous.workspace)).toEqual(previousWorkspace);
      expectTerminalAttempts(previous, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(previous)), snapshot(previous.workspace), "rolledback:caught-journal-previous-exchanged", previous.root);
    } finally { retainFixture(preparation.root); retainFixture(published.root); retainFixture(previous.root); }
  }, 15_000);
});

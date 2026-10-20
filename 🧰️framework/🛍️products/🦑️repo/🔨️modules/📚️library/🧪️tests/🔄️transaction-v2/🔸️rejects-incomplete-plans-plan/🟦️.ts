/** 🔸️ The rejects incomplete plans plan tests of the transaction plan journal v2 aggregate suite. */
import { expect, test } from "bun:test";
import { spawn, type ChildProcess } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { applyTaxonomyPlan } from "../../../🧹️normalization/🚀️apply/🟦️.ts";
import { parseTaxonomyPlan } from "../../../🧹️normalization/📃️plan-document/🟦️.ts";
import { taxonomyPlanDigest } from "../../../🧹️normalization/🗃️affected-state/🟦️.ts";
import { type TaxonomyPlan } from "../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { KILL_PHASES, LATE_KILL_PHASES, LEASE_PHASES, NORMALIZATION_MODULE, RESTORE_PHASES, SCHEMA_REL, registerChild } from "../📜️boundary-contracts/🟦️.ts";
import { embeddedFixture, generatorFixture, git, journalPaths, plan, referenceFixture, retainFixture, snapshot, symlinkFixture, transactionRoot, writePlan } from "../🏗️fixture-builders/🟦️.ts";
import { activeReferenceFixture, attemptJournal, boundedExit, expectBoundaryGolden, expectBoundaryTuple, expectCompleteSnapshot, expectEmptyPlan, expectTerminalAttempts, killTree, killedAt, killedMixedGenerator, waitFor } from "../💥️child-control/🟦️.ts";

/** 🔹 Registers the rejects incomplete plans plan tests of the transaction plan journal v2 aggregate suite. */
export function registerRejectsIncompletePlansPlanTests(): void {
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
}

import { selectCases, selectImplementations } from "../../🔍️discovery/🎛️selection/🟦️.ts";
import { ownerShipsImplementation } from "../../🖥️host/🏗️materialization/🟦️.ts";
import { executeOne } from "../../🏃️execution/🎬️scenario/🟦️.ts";
import { readImplementationCoverage, reportsDir } from "../../📊️reporting/📋️orchestration/🟦️.ts";
import { loadClassifiedBaseline } from "../../🕸️dependencies/📋️orchestration/🟦️.ts";
import {
  type ComparisonProfile,
  type DiscoveredCase,
  type Implementation,
  type TestResult,
  buildCasePlan,
  computeCoverageMetrics,
  evaluateCrossSubjectParity,
  evaluateParity,
  formatMetrics,
  loadOracleRegistry,
  markRunComplete,
  profileTable,
  renderDiff,
  renderJUnit,
  summarizeRun,
  testCacheDir,
} from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script, type TestLevel, resolveTestLevel } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🎭️ The case's oracle decision: which implementation serves the oracle role, or the recorded no-oracle decision. */
export function oracleDecision(repoRoot: string, discovered: DiscoveredCase, level: TestLevel): { implementation: Implementation | null; noOracleDecision: string | null; comparison: ComparisonProfile; problem: string | null } {
  const registry = loadOracleRegistry(repoRoot);
  const { plan } = buildCasePlan(repoRoot, discovered, level);
  if (plan.oracle === null)
    return { implementation: null, noOracleDecision: plan.noOracleDecision, comparison: plan.comparison, problem: plan.noOracleDecision === null ? `${discovered.caseDir}: feature declares neither an oracle nor a no-oracle decision` : null };
  const entry = registry.oracles.find((candidate) => candidate.id === plan.oracle);
  if (entry === undefined) return { implementation: null, noOracleDecision: null, comparison: plan.comparison, problem: `${discovered.caseDir}: unknown oracle id ${plan.oracle}` };
  const mapped = (entry.ecosystem === "javascript" ? "typescript" : entry.ecosystem) as Implementation;
  if ((discovered.adapters as Record<string, string | undefined>)[mapped] === undefined)
    return { implementation: null, noOracleDecision: null, comparison: plan.comparison, problem: `${discovered.caseDir}: oracle ${entry.id} needs a ${mapped} adapter to run in` };
  return { implementation: mapped, noOracleDecision: null, comparison: plan.comparison, problem: null };
}

/** 🎯️ The declared execution mode of one planned scenario — what a no-oracle substitute must match. */
export function planModeOf(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, scenarioId: string): string {
  return buildCasePlan(repoRoot, discovered, level).plan.scenarios.find((scenario) => scenario.id === scenarioId)?.mode ?? "differential";
}

/** 🏃️ Runs the requested phases for every selected case and writes the run report. */
export function runPhases(repoRoot: string, segments: readonly string[], phases: readonly ("oracle" | "subject")[]): number {
  const { level, rest } = resolveTestLevel([...segments]);
  const cases = selectCases(repoRoot, rest);
  const allResults: TestResult[] = [];
  const problems: string[] = [];
  const parity: { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number }[] = [];
  // ⚖️One effective profile table for the whole run: the framework's domain-neutral profiles plus
  // every profile the discovered owners contribute.
  const profiles = profileTable(loadOracleRegistry(repoRoot));
  let scenarioCount = 0;

  for (const discovered of cases) {
    const decision = oracleDecision(repoRoot, discovered, level);
    if (decision.problem !== null) problems.push(decision.problem);
    const caseResults: TestResult[] = [];
    const rawInputOracle = buildCasePlan(repoRoot, discovered, level).plan.oracleInput === "subject-raw";

    const runSubjects = (): void => {
      if (!phases.includes("subject") && !rawInputOracle) return;
      // 🔬️Only the languages this repository actually implements the owner in are dispatched as
      // subjects; see `ownerShipsImplementation`.
      const subjects = selectImplementations(discovered, rest).filter((candidate) => ownerShipsImplementation(repoRoot, discovered, candidate));
      // 🚫️A case every one of whose adapters is a reference HOST has no subject half at all. That is
      // a real gap and it must stay visible — reported the same way `not-exercised` is, rather than
      // as a per-scenario `errored` result whose null projection would enter the parity ratio and
      // read as a subject that ran and disagreed.
      if (subjects.length === 0 && selectImplementations(discovered, rest).length > 0)
        console.error(`[test] no-subject-implementation ${discovered.caseDir} (adapters ${Object.keys(discovered.adapters).join(", ")} host references only; this repository ships no implementation of the owner in any of those languages)`);
      for (const implementation of subjects) {
        const outcome = executeOne(repoRoot, discovered, level, "subject", implementation);
        caseResults.push(...outcome.results);
        problems.push(...outcome.problems);
      }
    };
    const subjectRawInputs = (): Readonly<Partial<Record<Implementation, string>>> =>
      Object.fromEntries(caseResults.filter((result) => result.role === "subject" && result.status === "passed" && result.output.rawPath !== undefined).map((result) => [result.implementation, result.output.rawPath!] as const));
    if (rawInputOracle) runSubjects();
    if (phases.includes("oracle") && decision.implementation !== null) {
      const outcome = executeOne(repoRoot, discovered, level, "oracle", decision.implementation, rawInputOracle ? subjectRawInputs() : undefined);
      caseResults.push(...outcome.results);
      problems.push(...outcome.problems);
    }
    if (!rawInputOracle) runSubjects();
    scenarioCount += new Set(caseResults.map((result) => result.scenario)).size;
    allResults.push(...caseResults);

    if (!phases.includes("oracle") || !phases.includes("subject") || caseResults.length === 0) continue;

    const diffDir = testCacheDir(repoRoot, "diffs");
    mkdirSync(diffDir, { recursive: true });
    if (decision.implementation !== null) {
      const { verdicts, unmatched } = evaluateParity(decision.comparison, caseResults, profiles);
      for (const verdict of verdicts) {
        parity.push({ testId: verdict.testId, profile: verdict.profile, equal: verdict.equal, diffs: verdict.diffs });
        if (verdict.equal) continue;
        writeFileSync(join(diffDir, `${verdict.testId.replace(/[^A-Za-z0-9]+/g, "_")}.diff.txt`), renderDiff(verdict.testId, verdict.verdict));
        problems.push(`parity failed: ${verdict.testId} (${verdict.diffs} differences)`);
      }
      for (const orphan of unmatched) problems.push(`no oracle result to compare against: ${orphan}`);
    }

    // 🧮️ Pairwise subject equivalence keeps two implementations from exploiting different oracle
    // ambiguities, and is the ONLY parity evidence a recorded no-oracle case can offer — so it must
    // then involve at least two independently written implementations to mean anything at all.
    const crossPairs = evaluateCrossSubjectParity(decision.comparison, caseResults, profiles);
    for (const pair of crossPairs) {
      parity.push({ testId: pair.pair, profile: decision.comparison, equal: pair.equal, diffs: pair.diffs });
      if (!pair.equal) problems.push(`cross-subject parity failed: ${pair.pair} (${pair.diffs} differences)`);
    }
    // 🧭️A recorded no-oracle decision names the substitutes it relies on, and only the
    // `independent-implementations` substitute is discharged by pairwise parity. A decision resting
    // on specification vectors or metamorphic laws discharges itself inside the scenarios, so
    // demanding a second implementation there would be a rule the decision never claimed to meet.
    if (decision.implementation === null && decision.noOracleDecision !== null && crossPairs.length === 0) {
      const substitutes = loadOracleRegistry(repoRoot).noOracleDecisions.find((entry) => entry.id === decision.noOracleDecision)?.substitutes ?? [];
      if (substitutes.includes("independent-implementations")) problems.push(`${discovered.caseDir}: no-oracle decision ${decision.noOracleDecision} claims the independent-implementations substitute but only one implementation ran`);
      else if (!caseResults.every((result) => ["conformance", "property", "round-trip", "error"].includes(planModeOf(repoRoot, discovered, level, result.scenario)))) {
        problems.push(
          `${discovered.caseDir}: no-oracle decision ${decision.noOracleDecision} rests on ${substitutes.join(", ")}, which only discharge a conformance, property, round-trip or error scenario — a differential scenario needs an oracle or a second implementation`,
        );
      }
    }
  }

  // 🚫️A selected case that produced no result at all is not a pass — it is an absence of evidence,
  // and the two must never look the same. The commonest cause is legitimate (asking for the oracle
  // phase of a recorded no-oracle case, which by definition has no oracle to run), so this reports
  // rather than fails; what it must not do is stay silent while the run prints a green summary.
  const exercised = new Set(allResults.map((result) => `${result.owner}::${result.case}`));
  const unexercised = cases.filter((discovered) => !exercised.has(`${discovered.owner}::${discovered.case}`));
  for (const discovered of unexercised) {
    const decision = oracleDecision(repoRoot, discovered, level);
    const why =
      decision.implementation === null && decision.noOracleDecision !== null
        ? `recorded no-oracle decision ${decision.noOracleDecision} — its evidence is discharged by the subject phase`
        : `no implementation served the requested phase(s) ${phases.join(", ")}`;
    console.error(`[test] not-exercised ${discovered.caseDir} (${why})`);
  }

  const summary = summarizeRun(level, cases.length, scenarioCount, allResults, parity, problems);
  const dir = reportsDir(repoRoot);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "📊️summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  writeFileSync(join(dir, "📤️results.jsonl"), allResults.map((result) => JSON.stringify(result)).join("\n") + (allResults.length > 0 ? "\n" : ""));
  writeFileSync(join(dir, "📋️junit.xml"), renderJUnit(allResults));
  const metrics = computeCoverageMetrics(repoRoot, cases, allResults, parity, loadClassifiedBaseline(repoRoot));
  writeFileSync(join(dir, "📈️metrics.json"), `${JSON.stringify(metrics, null, 2)}\n`);
  markRunComplete(dir);

  console.log(
    `[test] level=${level} cases=${summary.cases} executed=${summary.executed} passed=${summary.passed} failed=${summary.failed} errored=${summary.errored} parity=${parity.filter((row) => row.equal).length}/${parity.length}${unexercised.length > 0 ? ` not-exercised=${unexercised.length}` : ""}`,
  );
  if (segments.includes("--metrics")) console.log(formatMetrics(metrics, readImplementationCoverage(repoRoot)));
  for (const problem of problems) console.error(`[test] ${problem}`);
  return summary.failed + summary.errored + problems.length === 0 ? 0 : 1;
}

/** 🔮️ The oracle phase — proves the reference library actually supports the case before any local code exists, except an explicit byte-decoder oracle, which first receives its subject artifact. */
export class OracleScript extends Script {
  run(segments: string[]): void {
    process.exit(runPhases(this.repoRoot, segments, ["oracle"]));
  }
}

/** 🎯️ The subject phase — this repository's implementations, on the same inputs. */
export class SubjectScript extends Script {
  run(segments: string[]): void {
    process.exit(runPhases(this.repoRoot, segments, ["subject"]));
  }
}

/** ⚖️ Oracle + subject + semantic comparison + pairwise subject equivalence. */
export class ParityScript extends Script {
  run(segments: string[]): void {
    process.exit(runPhases(this.repoRoot, segments, ["oracle", "subject"]));
  }
}

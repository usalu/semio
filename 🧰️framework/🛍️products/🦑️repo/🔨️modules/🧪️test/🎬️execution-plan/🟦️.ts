/** 🎬️ Execution plan
 *
 * The plan of one case and the coordinator that executes and compares it.
 */

//#region 🔌️Adapters
import { parseFeature, type ParsedFeature } from "../../../../../📦️packages/🟦️typescript/🧪️test/🥒️gherkin/🟦️.ts";
import { type TestCasePlan, type ComparisonProfile, type SubjectRawInputs, type Implementation, type TestRole, currentPlatform } from "../../../../../📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { TEST_LEVELS, type TestLevel } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { type ComparisonProfileSpec } from "../🗂️taxonomy/🟦️.ts";
import { digest, setDigest } from "../🔢️digests/🟦️.ts";
import { compareProjections, type ComparisonVerdict, coreProfileTable } from "../🟰️comparison/🟦️.ts";
import { type DiscoveredCase } from "../🧭️case-discovery/🟦️.ts";
import { markOutputDir, testCacheDir } from "../⚡️output-cache/🟦️.ts";
import { mutationManifestDigest } from "../📑️mutation-manifest/🟦️.ts";
import { type TestResult } from "../📤️results/🟦️.ts";
import { loadOracleRegistry, type OracleRegistry, pipelineTable, profileTable } from "../🤝️contributions/🟦️.ts";
import { resolveTestInputs, testInputUrisIn } from "../📥️fixture-inputs/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📋️Plan
/**
 * 📥️ The raw bytes each subject host produced, keyed by SCENARIO id and then by implementation. A byte-decoding oracle
 * (`@oracle-input-subject-raw`) judges one scenario's output at a time, so the routing must never fold a case's
 * scenarios into one map: a flat implementation → path map hands every oracle scenario the LAST subject scenario's bytes.
 */


/** ⌨️ Routes every passed subject result that produced raw bytes to its own scenario id. */
export function subjectRawInputsByScenario(results: readonly TestResult[]): SubjectRawInputs {
  const routed: Record<string, Partial<Record<Implementation, string>>> = {};
  for (const result of results) if (result.role === "subject" && result.status === "passed" && result.output.rawPath !== undefined) (routed[result.scenario] ??= {})[result.implementation] = result.output.rawPath;
  return routed;
}

/** 🧲️ The owned execution plan one native host receives — hosts never re-parse the feature. */


/** 🎚️ Levels at or below `level` — running a level runs every lower level, cumulatively. */
export function levelsUpTo(level: TestLevel): readonly TestLevel[] {
  return TEST_LEVELS.slice(0, TEST_LEVELS.indexOf(level) + 1);
}

/** 📋️ Builds the plan skeleton shared by every role/implementation execution of one case. */
export function buildCasePlan(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, registry: OracleRegistry = loadOracleRegistry(repoRoot)): { plan: Omit<TestCasePlan, "role" | "implementation" | "workDir" | "resultsPath" | "outputDir" | "artifactDir">; feature: ParsedFeature; missingFixtures: string[] } {
  const source = readFileSync(join(repoRoot, discovered.featurePath), "utf8");
  const feature = parseFeature(source);
  const { inputs, missing } = resolveTestInputs(repoRoot, discovered, testInputUrisIn(feature));
  const selectable = new Set(levelsUpTo(level));
  const owning = registry.contributions
    .filter((entry) => discovered.owner === entry.owner || discovered.owner.startsWith(`${entry.owner}/`) || entry.owner.startsWith(`${discovered.owner}/`))
    .flatMap((entry) => entry.mutationManifests)
    .find((manifest) => feature.capability === null || manifest.mutations.some((mutation) => mutation.capability === feature.capability));
  const profile = profileTable(registry).get(feature.comparison ?? "");
  const pipeline = profile?.pipeline ?? null;
  const toleranceProfile = pipeline === null ? null : (pipelineTable(registry).get(pipeline)?.toleranceProfile ?? null);
  const testEvidence = registry.contributions
    .filter((entry) => discovered.owner === entry.owner || discovered.owner.startsWith(`${entry.owner}/`) || entry.owner.startsWith(`${discovered.owner}/`))
    .flatMap((entry) => entry.testEvidence);
  return {
    feature,
    missingFixtures: missing,
    plan: {
      schemaVersion: 2,
      baselineSha: process.env.SEMIO_BASELINE_SHA,
      owner: discovered.owner,
      case: discovered.case,
      featurePath: discovered.featurePath,
      featureHash: digest(source),
      featureName: feature.name,
      description: feature.description,
      capability: feature.capability ?? "",
      target: owning === undefined ? null : { artifact: owning.artifact, standard: owning.standard, subset: owning.subset },
      mutationManifestDigest: owning === undefined ? null : mutationManifestDigest(owning),
      oracle: feature.oracle,
      noOracleDecision: feature.noOracleDecision,
      comparison: feature.comparison ?? "ordered-json-v1",
      oracleInput: feature.oracleInput,
      comparisonPipeline: pipeline,
      toleranceProfile,
      background: feature.background,
      scenarios: feature.scenarios.filter((scenario) => selectable.has(scenario.level)),
      adapters: discovered.adapters,
      inputs,
      testEvidence,
      platform: currentPlatform(),
      level,
    },
  };
}

/** 🆔️ The stable identifier every executable result carries. Display names may change; this may not. */
export function testId(owner: string, caseSlug: string, scenario: string, implementation: Implementation, role: TestRole): string {
  return `${owner}::${caseSlug}::${scenario}::${implementation}::${role}`;
}
//#endregion 📋️Plan

//#region 🏃️Coordinator
/** 🏃️ Everything one execution of a case at a level needs, resolved to absolute cache paths. */
export function planExecution(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, role: TestRole, implementation: Implementation): { plan: TestCasePlan; feature: ParsedFeature; missingFixtures: string[]; planPath: string } {
  const { plan: base, feature, missingFixtures } = buildCasePlan(repoRoot, discovered, level);
  const cacheKey = digest(`${discovered.projectName}|${base.featureHash}|${setDigest(base.inputs.map((f) => [f.name, f.digest] as const))}|${level}|${role}|${implementation}`);
  const workDir = join(testCacheDir(repoRoot, "work"), `${discovered.projectName}-${role}-${implementation}`);
  const outputDir = join(testCacheDir(repoRoot, "results"), `${discovered.projectName}-${role}-${implementation}`);
  const artifactDir = join(outputDir, "📦️artifacts");
  const ownerId = `${discovered.owner}::${discovered.case}`;
  for (const dir of [workDir, outputDir]) {
    markOutputDir(repoRoot, dir, { testId: ownerId, cacheKey });
    rmSync(join(dir, "🏁️done"), { force: true });
  }
  mkdirSync(artifactDir, { recursive: true });
  const plan: TestCasePlan = { ...base, role, implementation, workDir, outputDir, artifactDir, resultsPath: join(outputDir, "📤️results.jsonl") };
  const planPath = join(workDir, "📋️plan.json");
  writeFileSync(planPath, `${JSON.stringify(plan, null, 2)}\n`);
  return { plan, feature, missingFixtures, planPath };
}

/** 🏁️ Marks a generated output directory complete, so an interrupted run stays distinguishable from a finished one. */
export function markRunComplete(absDir: string): void {
  mkdirSync(absDir, { recursive: true });
  writeFileSync(join(absDir, "🏁️done"), "");
}

/** 🧮️ Pairs every subject result with its oracle counterpart and applies the case's comparison profile. */
export function evaluateParity(profile: ComparisonProfile, results: readonly TestResult[], profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): { verdicts: { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number; verdict: ComparisonVerdict }[]; unmatched: string[] } {
  const oracles = new Map<string, TestResult>();
  for (const result of results) if (result.role === "oracle") oracles.set(`${result.owner}::${result.case}::${result.scenario}`, result);
  const verdicts: { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number; verdict: ComparisonVerdict }[] = [];
  const unmatched: string[] = [];
  for (const result of results) {
    if (result.role !== "subject") continue;
    const oracle = oracles.get(`${result.owner}::${result.case}::${result.scenario}`);
    if (oracle === undefined) {
      unmatched.push(result.testId);
      continue;
    }
    const verdict = compareProjections(profile, oracle.output.projection, result.output.projection, profiles);
    verdicts.push({ testId: result.testId, profile, equal: verdict.equal, diffs: verdict.diffs.length, verdict });
  }
  return { verdicts, unmatched };
}

/** ✖️ Pairwise subject equivalence, so two implementations cannot exploit different oracle ambiguities. */
export function evaluateCrossSubjectParity(profile: ComparisonProfile, results: readonly TestResult[], profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): { pair: string; equal: boolean; diffs: number }[] {
  const byScenario = new Map<string, TestResult[]>();
  for (const result of results) {
    if (result.role !== "subject") continue;
    const key = `${result.owner}::${result.case}::${result.scenario}`;
    if (!byScenario.has(key)) byScenario.set(key, []);
    byScenario.get(key)!.push(result);
  }
  const out: { pair: string; equal: boolean; diffs: number }[] = [];
  for (const [key, rows] of byScenario) {
    const sorted = [...rows].sort((a, b) => a.implementation.localeCompare(b.implementation));
    for (let i = 0; i < sorted.length; i += 1) {
      for (let j = i + 1; j < sorted.length; j += 1) {
        const verdict = compareProjections(profile, sorted[i]!.output.projection, sorted[j]!.output.projection, profiles);
        out.push({ pair: `${key}::${sorted[i]!.implementation}~${sorted[j]!.implementation}`, equal: verdict.equal, diffs: verdict.diffs.length });
      }
    }
  }
  return out;
}
//#endregion 🏃️Coordinator

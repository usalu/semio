/** 🔀️ Comparison pipelines
 *
 * Multi-artifact comparison pipelines: stage assertions, evaluation and execution.
 */

//#region 🔌️Adapters
import { runProbe } from "../../📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";
import { canonicalize } from "../🟰️comparison/🟦️.ts";
import { isQualifiedProbe, type ProbeEntry, type ProbeReport, probeReportProblems } from "../🔬️probes/🟦️.ts";
import { type TestResult } from "../📤️results/🟦️.ts";
//#endregion 🔌️Adapters

//#region ⚖️Pipeline
/** ⚖️ One stage: a probe, the roles it reads, the roles it produces and the assertions on its output. */
export type ComparisonStage = Readonly<{
  probe: string;
  description?: string;
  inputs: readonly string[];
  outputs?: readonly string[];
  args?: readonly string[];
  seed?: string | number;
  /** 🚧️ A stage whose probe is not yet qualified. It RUNS and REPORTS; no release gate may claim its guarantee. */
  optional?: boolean;
  assertions?: Readonly<Record<string, unknown>>;
}>;

/** ⚖️ An ordered, multi-artifact, externally-probed comparison. */
export type ComparisonPipeline = Readonly<{ id: string; description?: string; toleranceProfile?: string; referenceScale?: "bounding-box-diagonal" | "unit" | "declared"; stages: readonly ComparisonStage[] }>;

/** ⚖️ One evaluated assertion. `expected` is what the stage declared, `actual` what the probe measured. */
export type AssertionVerdict = Readonly<{ stage: number; probe: string; key: string; expected: unknown; actual: unknown; ok: boolean; optional: boolean; reason: string }>;

/** ⚖️ The verdict of one whole pipeline run. */
export type PipelineVerdict = Readonly<{
  pipeline: string;
  equal: boolean;
  verdicts: readonly AssertionVerdict[];
  missingProbes: readonly string[];
  unqualifiedStages: readonly string[];
  /** 🚫️ Stages marked `optional` that are NOT excusable — their probe is qualified, or unregistered. The mark is ignored and reported. */
  overclaimedOptional: readonly string[];
}>;

/**
 * ⚖️ Evaluates one stage's declarative assertions against one probe report. The vocabulary is small
 * and deliberately arithmetic-free beyond comparison: `<key>Max` bounds a measured number from above,
 * `<key>Min` from below, `<key>Equal` compares structurally, and a bare boolean/number/string
 * compares for equality. Anything a probe cannot measure it must report as `unsupported` rather than
 * letting the orchestrator compute it.
 */
export function evaluateStageAssertions(stageIndex: number, stage: ComparisonStage, report: ProbeReport, effectiveOptional?: boolean): AssertionVerdict[] {
  const verdicts: AssertionVerdict[] = [];
  const optional = effectiveOptional ?? stage.optional === true;
  if (report.status !== "ok") {
    verdicts.push({ stage: stageIndex, probe: stage.probe, key: "status", expected: "ok", actual: report.status, ok: false, optional, reason: `probe reported ${report.status}` });
    return verdicts;
  }
  for (const [key, expected] of Object.entries(stage.assertions ?? {})) {
    const bound = key.endsWith("Max") ? "max" : key.endsWith("Min") ? "min" : key.endsWith("Equal") ? "equal" : "value";
    const strippedKey = bound === "value" ? key : key.slice(0, -{ max: 3, min: 3, equal: 5 }[bound]!);
    const measurementKey = Object.hasOwn(report.measurements, key) ? key : strippedKey;
    const actual = report.measurements[measurementKey];
    if (actual === undefined) {
      verdicts.push({ stage: stageIndex, probe: stage.probe, key, expected, actual: undefined, ok: false, optional, reason: `probe ${stage.probe} reported no measurement ${JSON.stringify(strippedKey)}${strippedKey === key ? "" : ` (nor ${JSON.stringify(key)})`}` });
      continue;
    }
    if (bound === "max" || bound === "min") {
      const limit = Number(expected);
      const measured = Number(actual);
      const ok = Number.isFinite(measured) && Number.isFinite(limit) && (bound === "max" ? measured <= limit : measured >= limit);
      verdicts.push({ stage: stageIndex, probe: stage.probe, key, expected, actual, ok, optional, reason: ok ? "" : `${measurementKey} ${measured} violates ${bound} ${limit}` });
      continue;
    }
    const ok = JSON.stringify(canonicalize(actual)) === JSON.stringify(canonicalize(expected));
    verdicts.push({ stage: stageIndex, probe: stage.probe, key, expected, actual, ok, optional, reason: ok ? "" : `${bound === "equal" ? measurementKey : key} is not equal to the declared value` });
  }
  return verdicts;
}

/**
 * ⚖️ Evaluates a whole pipeline against the reports its stages produced. A stage with no report is a
 * failure, not a skip: an unmeasured assertion that reads as green is the exact defect this pipeline
 * replaces v1's single generic tolerance to prevent.
 */
export function evaluatePipeline(pipeline: ComparisonPipeline, reports: ReadonlyMap<number, ProbeReport>, probes: ReadonlyMap<string, ProbeEntry>): PipelineVerdict {
  const verdicts: AssertionVerdict[] = [];
  const missingProbes: string[] = [];
  const unqualifiedStages: string[] = [];
  const overclaimedOptional: string[] = [];
  for (const [index, stage] of pipeline.stages.entries()) {
    const probe = probes.get(stage.probe);
    if (probe === undefined) missingProbes.push(stage.probe);
    else if (!isQualifiedProbe(probe)) unqualifiedStages.push(stage.probe);
    const excusable = probe !== undefined && !isQualifiedProbe(probe);
    if (stage.optional === true && !excusable) overclaimedOptional.push(stage.probe);
    const effectiveOptional = stage.optional === true && excusable;
    const report = reports.get(index);
    if (report === undefined) {
      verdicts.push({ stage: index, probe: stage.probe, key: "report", expected: "a probe report", actual: undefined, ok: false, optional: effectiveOptional, reason: `stage ${index} (${stage.probe}) produced no report` });
      continue;
    }
    verdicts.push(...evaluateStageAssertions(index, stage, report, effectiveOptional));
  }
  const equal = missingProbes.length === 0 && verdicts.every((verdict) => verdict.ok || verdict.optional);
  return { pipeline: pipeline.id, equal, verdicts, missingProbes, unqualifiedStages, overclaimedOptional };
}

/** 📦️ The artifacts one scenario pair produced, keyed by role: the oracle's and the subject's, merged. A role both
 * results produce is a problem, because a stage could then read either side's file. */
export function pipelineRoleArtifacts(results: readonly Pick<TestResult, "role" | "artifacts">[]): { artifacts: ReadonlyMap<string, string>; problems: string[] } {
  const artifacts = new Map<string, string>();
  const problems: string[] = [];
  for (const result of results)
    for (const artifact of result.artifacts ?? []) {
      if (artifacts.has(artifact.role)) problems.push(`artifact role ${artifact.role} is produced by more than one result`);
      else artifacts.set(artifact.role, artifact.path);
    }
  return { artifacts, problems };
}

/**
 * ⚖️ Runs one comparison pipeline over one scenario pair's artifacts and evaluates it. Every stage input role must name
 * an artifact one side produced — a role nobody produced is a stage without a report, which `evaluatePipeline` fails,
 * never a skip. Each stage runs its probe's registered `command` with one `--input <path>` per declared input, in the
 * declared order; its whole stdout is the one report it prints, validated before a single assertion is read. Stages run in order; `signal`
 * cancels between stages and `progress` reports each finished stage.
 */
export function executePipeline(repoRoot: string, pipeline: ComparisonPipeline, probes: ReadonlyMap<string, ProbeEntry>, artifacts: ReadonlyMap<string, string>, options: Readonly<{ budgetMs: number; signal?: AbortSignal; progress?: (done: number, total: number) => void }>): { verdict: PipelineVerdict; reports: ReadonlyMap<number, ProbeReport>; problems: string[] } {
  const reports = new Map<number, ProbeReport>();
  const problems: string[] = [];
  for (const [index, stage] of pipeline.stages.entries()) {
    if (options.signal?.aborted === true) {
      problems.push(`pipeline ${pipeline.id} cancelled before stage ${index} (${stage.probe})`);
      break;
    }
    const probe = probes.get(stage.probe);
    const unresolved = stage.inputs.filter((role) => !artifacts.has(role));
    if (probe === undefined || probe.command === undefined || probe.command.length === 0) problems.push(`stage ${index} (${stage.probe}): the probe is not registered with a command`);
    else if (unresolved.length > 0) problems.push(`stage ${index} (${stage.probe}): no artifact for role(s) ${unresolved.join(", ")}`);
    else {
      const [command, ...fixed] = probe.command;
      const run = runProbe(command!, [...fixed, ...(stage.args ?? []), ...stage.inputs.flatMap((role) => ["--input", artifacts.get(role)!])], { cwd: repoRoot, budgetMs: options.budgetMs });
      let report: unknown;
      try {
        report = JSON.parse(run.stdout);
      } catch {
        report = undefined;
      }
      const invalid = probeReportProblems(report);
      if (invalid.length > 0) problems.push(`stage ${index} (${stage.probe}) exited ${run.status} without a valid report: ${invalid.join("; ")}${run.stderr.trim().length > 0 ? ` — ${run.stderr.trim().split("\n").slice(-3).join(" / ")}` : ""}`);
      else reports.set(index, report as ProbeReport);
    }
    options.progress?.(index + 1, pipeline.stages.length);
  }
  return { verdict: evaluatePipeline(pipeline, reports, probes), reports, problems };
}
//#endregion ⚖️Pipeline

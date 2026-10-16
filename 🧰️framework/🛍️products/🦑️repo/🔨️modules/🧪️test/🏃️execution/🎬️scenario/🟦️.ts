import {repositoryTestArtifactEnvironmentV1} from "../../💾️storage/🟦️.ts";
import { testLevelBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { materializeHost } from "../../🖥️host/🏗️materialization/🟦️.ts";
import { type Implementation, type SubjectRawInputs, type TestRole } from "@semio-tech/framework/test/adapter";
import { type DiscoveredCase, type TestResult, markRunComplete, planExecution, readResults } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { type TestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { scriptInvocationEnvironment,requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,withIssuedResourceOperationV1,type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution";
import { rmSync, writeFileSync } from "node:fs";

export type PhaseOutcome = Readonly<{ results: TestResult[]; problems: string[] }>;

/** 🏃️ Executes one `(case, level, role, implementation)` triple and reads back its owned result stream. */
export async function executeOne(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, role: TestRole, implementation: Implementation, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl, subjectRawInputs?: SubjectRawInputs): Promise<PhaseOutcome> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  const planned = planExecution(repoRoot, discovered, level, role, implementation);
  const plan = subjectRawInputs === undefined ? planned.plan : { ...planned.plan, subjectRawInputs };
  const { missingFixtures, planPath } = planned;
  if (subjectRawInputs !== undefined) writeFileSync(planPath, `${JSON.stringify(plan, null, 2)}\n`);
  const problems = missingFixtures.map((uri) => `${discovered.caseDir}: unresolved fixture ${uri}`);
  if (plan.scenarios.length === 0) return { results: [], problems };
  rmSync(plan.resultsPath, { force: true });
  const host = materializeHost(repoRoot, discovered, role, implementation, planPath, plan.resultsPath);
  // 🧩️A host that could not be provisioned has not run, and an unprovisioned host must never look
  // like a case with nothing to do — the declaration is reported before anything is executed.
  if (host.problems.length > 0) return { results: [], problems: [...problems, ...host.problems] };
  let command = host.command;
  const capture=(command:string,args:readonly string[],budgetMs:number)=>withRequestedIssuedResourceControlV1(phaseControl,budgetMs,control=>withIssuedResourceOperationV1(control,()=>captureOwnedCommandResultV1(command,args,host.cwd,{control,env:scriptInvocationEnvironment(invocation,repositoryTestArtifactEnvironmentV1(repoRoot,host.env)),output:{maximumBytes:16777216,maximumLines:65536}})));
  if (host.preparation !== undefined) {
    const prepared = await capture(host.preparation.command,host.preparation.args,buildBudgetMs());
    if ((prepared.status ?? 1) !== 0) {
      problems.push(`${discovered.caseDir}: ${implementation} ${role} host preparation exited ${prepared.status}`);
      if (prepared.stdout.trim() !== "") problems.push(prepared.stdout.trimEnd());
      if (prepared.stderr.trim() !== "") problems.push(prepared.stderr.trimEnd());
      return { results: [], problems };
    }
    command = host.preparation.executableFromStdout(prepared.stdout) ?? "";
    if (command === "") return { results: [], problems: [...problems, `${discovered.caseDir}: ${implementation} ${role} host preparation emitted no executable`] };
  }
  const probe = await capture(command,host.args,testLevelBudgetMs(level));
  if (probe.stdout.trim() !== "") console.log(probe.stdout.trimEnd());
  const { results, problems: readProblems } = readResults(plan.resultsPath);
  if ((probe.status ?? 1) !== 0 && results.length === 0) {
    problems.push(`${discovered.caseDir}: ${implementation} ${role} host exited ${probe.status} without emitting results`);
    if (probe.stderr.trim() !== "") problems.push(probe.stderr.trimEnd());
  }
  // 🏁️Both generated directories are marked, not just the results one. An unmarked work directory
  // reads as permanently interrupted, and `clean test --stale` would then delete the work directory
  // of a run that is still executing in a parallel session.
  markRunComplete(plan.workDir);
  markRunComplete(plan.outputDir);
  return { results, problems: [...problems, ...readProblems.map((problem) => `${discovered.caseDir}: ${problem}`)] };
}

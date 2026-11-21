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
import { launchRepositoryCargoV1, readRepositoryCargoLaunchV1 } from "@semio-tech/repo-lib/cargo";
import { readBuildMode } from "@semio-tech/process/build-mode";
import { requireNativeStoragePathV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

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
  if (host.problems.length > 0) return { results: [], problems: [...problems, ...host.problems] };
  const environment = repositoryTestArtifactEnvironmentV1(repoRoot, host.env);
  const limits = { maximumBytes: 16777216, maximumLines: 65536 };
  const capture = (command: string, parent: ResourceLeaseControl) =>
    withRequestedIssuedResourceControlV1(parent, testLevelBudgetMs(level), control =>
      withIssuedResourceOperationV1(control, () => captureOwnedCommandResultV1(command, host.args, host.cwd, {
        control,
        env: scriptInvocationEnvironment(invocation, environment),
        output: limits,
      })));
  const preparation = host.preparation;
  let probe;
  if (preparation === undefined) {
    probe = await capture(host.command, phaseControl);
  } else {
    const mode = readBuildMode(environment.SEMIO_BUILD_MODE);
    const request = readRepositoryCargoLaunchV1(["--store=shared", `--configuration=${mode}`, "--", ...preparation.args], environment);
    probe = await launchRepositoryCargoV1(invocation, phaseControl, repoRoot, request, host.cwd, environment, {
      maximumMilliseconds: buildBudgetMs(),
      output: "capture",
      limits,
      consume: async (prepared, context) => {
        if (prepared.status !== 0 || prepared.signal !== null) {
          problems.push(`${discovered.caseDir}: ${implementation} ${role} host preparation exited ${prepared.status ?? prepared.signal}`);
          if (prepared.stdout.trim() !== "") problems.push(prepared.stdout.trimEnd());
          if (prepared.stderr.trim() !== "") problems.push(prepared.stderr.trimEnd());
          return null;
        }
        const executable = preparation.executableFromStdout(prepared.stdout);
        if (executable === null) {
          problems.push(`${discovered.caseDir}: ${implementation} ${role} host preparation emitted no executable`);
          return null;
        }
        const command = requireNativeStoragePathV1(context.store.root, executable);
        return capture(command, context.control);
      },
    });
    if (probe === null) return { results: [], problems };
  }
  if (probe.stdout.trim() !== "") console.log(probe.stdout.trimEnd());
  const { results, problems: readProblems } = readResults(plan.resultsPath);
  if ((probe.status ?? 1) !== 0 && results.length === 0) {
    problems.push(`${discovered.caseDir}: ${implementation} ${role} host exited ${probe.status} without emitting results`);
    if (probe.stderr.trim() !== "") problems.push(probe.stderr.trimEnd());
  }
  markRunComplete(plan.workDir);
  markRunComplete(plan.outputDir);
  return { results, problems: [...problems, ...readProblems.map((problem) => `${discovered.caseDir}: ${problem}`)] };
}

import type { ScriptInvocation } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { runCmd } from "@semio-tech/process/execution";
import { testFlowCompositionOwnership, flowCompositionLaws } from "../🏷️ownership/🟦️.ts";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  testFlowCompositionOwnership();
  await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd: context.repoRoot, cargoArgs: segments, groups: flowCompositionLaws().map(group => ({ package: "semio-s-flow-composition", target: { kind: "test", name: group.target }, laws: group.laws })) });
}

/** 🏃️ Runs the `CanonicalScript` command under the caller's roots and invocation. */
export async function runCanonicalScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if (args.length !== 0) throw new Error("canonical-architecture takes no arguments");
  testFlowCompositionOwnership();
  runCmd(process.execPath,["test","../../🧪️tests/🔌️port-sides/🟦️.ts"],{cwd:context.root});
  await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd:context.repoRoot, cargoArgs:["--locked"], groups:flowCompositionLaws().map(group => ({ package:"semio-s-flow-composition",target:{kind:"test",name:group.target},laws:group.laws })) });
}

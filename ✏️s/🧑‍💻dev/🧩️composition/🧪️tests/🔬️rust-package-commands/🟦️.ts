import type { ScriptInvocation } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { resolveTestLevel } from "@semio-tech/process/test-budget";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { compositionLawGroups, inTargetFamily, RETAINED_ARTIFACT_UNITS, retainedLawGroups, testCompositionOwnership } from "../🔬️ownership/🟦️.ts";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const { rest } = resolveTestLevel(segments);
  const selected = rest.indexOf("--test");
  const target = selected < 0 ? undefined : rest[selected + 1];
  const filtered = rest.indexOf("--law-filter");
  const pattern = filtered < 0 ? undefined : rest[filtered + 1];
  if (filtered >= 0 && !pattern) throw new Error("law filter requires a name fragment");
  const groups = (target === RETAINED_ARTIFACT_UNITS ? retainedLawGroups() : compositionLawGroups().filter(group => inTargetFamily(group.target.name, target))).map(group => ({ ...group, laws: group.laws.filter(law => pattern === undefined || law.includes(pattern)) })).filter(group => group.laws.length > 0);
  if (!groups.length || (selected >= 0 && !target)) throw new Error("unknown composition target or empty law selection");
  testCompositionOwnership(target);
  const cargoArgs = rest.filter((_value, index) => (selected < 0 || index !== selected && index !== selected + 1) && (filtered < 0 || index !== filtered && index !== filtered + 1));
  await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd: context.repoRoot, cargoArgs, env: process.env, nativeEnv: { RUST_MIN_STACK: process.env.RUST_MIN_STACK ?? "134217728" }, groups, progress(event) { console.log(`[composition-laws] ${event.stage}: ${event.law ?? ""}`); } });
}

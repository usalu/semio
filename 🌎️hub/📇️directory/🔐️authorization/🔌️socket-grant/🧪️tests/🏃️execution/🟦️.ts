import { join } from "node:path";
import { BundleScript, checkScriptInvocation, type ScriptInvocation } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { exactCargoStageEnvironments } from "../../../../../🏗️build/🛂staging-environment/🟦️.ts";
import { proveScopedDirectorySocketRevocationFixture } from "../🧾️fixture-verification/🟦️.ts";
import { assertSocketGrantNativeLawSources, socketGrantNativeLawPlan, type SocketGrantNativeStage } from "../📋️native-law-plan/🟦️.ts";

export type SocketGrantNativeRunner = (stage: SocketGrantNativeStage, packageRoot: string, env: NodeJS.ProcessEnv, invocation: ScriptInvocation) => Promise<void>;

const runNativeStage: SocketGrantNativeRunner = (stage, packageRoot, env, invocation) => runRepositoryCommand(invocation,"cargo", [...stage.args], packageRoot, `socket-grant:${stage.id}`, {version:1,maximumElapsedMilliseconds:600000,output:{maximumBytes:134217728,maximumLines:65536}}, { env });

/** 🏃️ Verifies the portable oracle and optionally executes the exact native law sequence. */
export async function runSocketGrantCheck(invocation: ScriptInvocation, repoRoot: string, packageRoot: string, segments: readonly string[], runner: SocketGrantNativeRunner = runNativeStage): Promise<void> {
  checkScriptInvocation(invocation);
  if (!(segments.length === 0 || (segments.length === 1 && segments[0] === "oracle"))) throw new Error("socket-grant-check accepts no arguments or oracle");
  await proveScopedDirectorySocketRevocationFixture(repoRoot);
  assertSocketGrantNativeLawSources(repoRoot);
  if (segments.length === 1) return;
  const projected = exactCargoStageEnvironments();
  const env = { ...projected.env, ...projected.nativeEnv };
  for (const stage of socketGrantNativeLawPlan()) await runner(stage, packageRoot, env, invocation);
}

/** 🔌️ Routes the socket-grant gate through its domain-owned verifier and native plan. */
export class SocketGrantCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runSocketGrantCheck(this.invocation,this.repoRoot, this.root, segments);
  }
}

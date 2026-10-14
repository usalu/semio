import { join } from "node:path";
import { type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧭️ Dispatches actual Process test ownership under the original repository operation. */
export async function runProcessTestOwner(invocation: ScriptInvocation, workspace: string, output: string, suite: string): Promise<void> {
  await runRepositoryCommand(invocation, process.execPath, [join(workspace, "🧰️framework/🔨️modules/🏃️process/📜️script.ts"), "test", suite], workspace, "process-test-owner:" + suite, { version: 1, maximumElapsedMilliseconds: invocation.policy.maximumElapsedMilliseconds, output: invocation.policy.maximumElapsedMilliseconds === 0 ? null : { maximumBytes: 1048576, maximumLines: 16384 } }, { env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output } });
}

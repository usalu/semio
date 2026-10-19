import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { join } from "node:path";
import { runRepositoryPublishedCargoExecutableV1, runDeclaredRustBuildV1 } from "../../../../📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 🏃️ Runs the `RunScript` command under the caller's roots and invocation. */
export async function runRunScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments:string[]): Promise<void> {
  await runDeclaredRustBuildV1(context.repoRoot,"@semio-tech/repo-coordinator-rs",context.root,context.invocation);
  process.exitCode=await runRepositoryPublishedCargoExecutableV1(join(context.root,"Cargo.toml"),"semio-repo-coordinator"+(process.platform==="win32"?".exe":""),segments,context.repoRoot,readBuildMode(process.env.SEMIO_BUILD_MODE),context.invocation,scriptCompilerAdmissionControlV1(context.invocation,"cargo:published-execution",0));
}

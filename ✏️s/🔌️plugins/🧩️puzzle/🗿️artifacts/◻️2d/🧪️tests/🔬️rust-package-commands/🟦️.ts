import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runRepositoryCommand } from "@semio-tech/framework/process/execution";
import { resolve } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments:string[]): Promise<void> {
  if(segments.length!==1||segments[0]!=="snapshot-sqlite-source")throw new Error("Unknown Puzzle 2D verification "+segments.join(" "));
  const snapshot=resolve(context.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
  await runRepositoryCommand(context.invocation,process.execPath,[resolve(context.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],context.repoRoot,"puzzle2d-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
}

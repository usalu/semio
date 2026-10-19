import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { runRepositoryTestCommand } from "@semio-tech/framework/process/test-execution";
import { resolve } from "node:path";

/** 🏃️ Runs the `OriginalTxtEncodingScript` command under the caller's roots and invocation. */
export async function runOriginalTxtEncodingScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
if(args.length!==1||!['source','native'].includes(args[0]))throw Error('test-original-encoding requires source or native');
if(args[0]==='source')await runRepositoryTestCommand(process.execPath,['test',resolve(context.root,'../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts')],{cwd:context.repoRoot,budgetMs:context.invocation.control.remainingMilliseconds()??undefined,signal:context.invocation.control.signal});
else await runArtifactRustTests('semio-s-artifact-stdio-txt',context.repoRoot,['--lib','txt_original_encoding_','--no-fail-fast','--success-output','immediate'],context.invocation);
 }

/** 🏃️ Runs the `OriginalTxtReceivingScript` command under the caller's roots and invocation. */
export async function runOriginalTxtReceivingScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
if(args.length!==1||!["source","native"].includes(args[0]))throw Error("test-original-receiving requires source or native");
if(args[0]==="source")await runRepositoryTestCommand(process.execPath,["test",resolve(context.root,"../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts")],{cwd:context.repoRoot,budgetMs:context.invocation.control.remainingMilliseconds()??undefined,signal:context.invocation.control.signal});
else await runArtifactRustTests("semio-s-artifact-stdio-txt",context.repoRoot,["--lib","txt_original_receiving_","--no-fail-fast","--success-output","immediate"],context.invocation);
 }

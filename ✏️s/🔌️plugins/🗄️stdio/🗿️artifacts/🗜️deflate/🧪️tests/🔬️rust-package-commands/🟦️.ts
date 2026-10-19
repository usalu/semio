import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { resolve } from "node:path";
import { BundleScript, scriptInvocationBudget } from "@semio-tech/framework/process/routing";
import { runBudgetedTestCommand } from "@semio-tech/framework/process/test-execution";


/** 🗜️ Qualifies neutral complete Deflate receiving inputs under the original invocation. */
export class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("original Deflate receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs genuine complete Deflate receiving laws in its owning package. */
export class OriginalReceivingNative extends OriginalReceivingSource{async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-deflate",this.repoRoot,["--lib","deflate_original_receiving_","--no-fail-fast"],this.invocation);}}

import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { BundleScript, scriptInvocationBudget } from "@semio-tech/framework/process/routing";
import { resolveTypescriptCompilerV1 } from "@semio-tech/framework/process/execution";
import { runBudgetedTestCommand } from "@semio-tech/framework/process/test-execution";
import { resolve } from "node:path";


/** 🌦️ Qualifies complete EPW lexical receiving under the authored original caller. */
export class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original EPW receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs actual EPW physical receiving laws under the same original finite invocation. */
export class OriginalReceivingNative extends OriginalReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-epw",this.repoRoot,["--lib","sqlite_snapshot_epw_original_receiving_","--no-fail-fast"],this.invocation);}
}

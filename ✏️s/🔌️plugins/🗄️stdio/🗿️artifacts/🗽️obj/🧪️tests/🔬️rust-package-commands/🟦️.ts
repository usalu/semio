import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { resolve } from "node:path";
import { BundleScript, scriptInvocationBudget } from "@semio-tech/framework/process/routing";
import { runBudgetedTestCommand } from "@semio-tech/framework/process/test-execution";
import { resolveTypescriptCompilerV1 } from "@semio-tech/process/execution";


/** 🗽️ Qualifies authored original OBJ complete semantic receiving inputs. */
export class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original OBJ receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs genuine original OBJ receiving laws in the owning artifact package. */
export class OriginalReceivingNative extends OriginalReceivingSource{async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-obj",this.repoRoot,["--lib","obj_original_receiving_","--no-fail-fast"],this.invocation);}}

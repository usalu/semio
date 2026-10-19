import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { resolve } from "node:path";
import { BundleScript, scriptInvocationBudget } from "@semio-tech/framework/process/routing";
import { runBudgetedTestCommand } from "@semio-tech/framework/process/test-execution";
import { resolveTypescriptCompilerV1 } from "@semio-tech/process/execution";


/** 🔺️ Qualifies genuine original STL semantic input before receiving implementation. */
export class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw new Error("original STL receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");
  const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});
  await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);
 }
}
/** 🦀️ Runs only original STL receiving laws in the mounted owning Rust module. */
export class OriginalReceivingNative extends OriginalReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-stl",this.repoRoot,["--lib","stl_original_receiving_","--no-fail-fast"],this.invocation);}
}

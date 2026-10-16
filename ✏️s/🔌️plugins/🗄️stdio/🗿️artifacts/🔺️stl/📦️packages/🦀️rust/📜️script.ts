#!/usr/bin/env bun
/** 📦️ stl Rust artifact package router. */
import { runArtifactRustPackageMain,runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {resolve} from "node:path";
import {BundleScript,scriptInvocationBudget} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 🔺️ Qualifies genuine original STL semantic input before receiving implementation. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("original STL receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");
  const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});
  await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);
 }
}
/** 🦀️ Runs only original STL receiving laws in the mounted owning Rust module. */
class OriginalReceivingNative extends OriginalReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-stl",this.repoRoot,["--lib","stl_original_receiving_","--no-fail-fast"],this.invocation.control);}
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-stl", { packages: ["semio-s-artifact-stdio-stl", "semio-s-artifact-stdio-stl-examples", "semio-s-artifact-stdio-stl-inferences", "semio-s-artifact-stdio-stl-mutations", "semio-s-artifact-stdio-stl-schema", "semio-s-artifact-stdio-stl-viewer"], commands:{"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative},snapshotSqliteTests: ["../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });

#!/usr/bin/env bun
/** 🗂️ Framework Space Collection Rust artifact package router. */
import{runArtifactRustPackageMain,runArtifactRustTests}from"../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import{BundleScript,scriptInvocationBudget}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{runBudgetedTestCommand}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import{resolve}from"node:path";
/** 📁️ Qualifies complete Collection semantic receiving against independent SQLite. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("original Collection receiving accepts no arguments");const source=resolve(this.root,"../../🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs all actual Collection owning receiving laws under its supplied finite invocation. */
class OriginalReceivingNative extends OriginalReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);const source=await Bun.file(resolve(this.root,"../../🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs")).text();for(const law of["complete","refusals","cancellation","registered_full_file","cumulative","boxed_custody"])if(!source.includes("fn sqlite_snapshot_collection_original_receiving_"+law+"("))throw Error("actual Collection Native law is not mounted: "+law);await runArtifactRustTests("semio-framework-artifact-space-collection",this.repoRoot,["--lib","sqlite_snapshot_collection_original_receiving_","--no-fail-fast"],this.invocation);}
}
await runArtifactRustPackageMain(import.meta.dir,"semio-framework-artifact-space-collection",{commands:{"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative},snapshotSqliteTests:["../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});

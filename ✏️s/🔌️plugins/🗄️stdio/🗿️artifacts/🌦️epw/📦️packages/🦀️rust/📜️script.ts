#!/usr/bin/env bun
/** 📦️ EPW Rust artifact package router. */
import{runArtifactRustPackageMain,runArtifactRustTests}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import{BundleScript,scriptInvocationBudget}from"../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import{runBudgetedTestCommand}from"../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import{resolve}from"node:path";
import{runDefinitionChecks}from"../../🧪️tests/📜️definition/🟦️.ts";
/** 🌦️ Qualifies complete EPW lexical receiving under the authored original caller. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("original EPW receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs actual EPW physical receiving laws under the same original finite invocation. */
class OriginalReceivingNative extends OriginalReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-epw",this.repoRoot,["--lib","sqlite_snapshot_epw_original_receiving_","--no-fail-fast"],this.invocation);}
}
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-stdio-epw",{ packages: ["semio-s-artifact-stdio-epw", "semio-s-artifact-stdio-epw-editor", "semio-s-artifact-stdio-epw-io-registry", "semio-s-artifact-stdio-epw-mutations", "semio-s-artifact-stdio-epw-schema", "semio-s-artifact-stdio-epw-snapshot", "semio-s-artifact-stdio-epw-tests"],commands:{"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative},twins:[{name:"epw-definition",run:runDefinitionChecks}],snapshotSqliteTests:["../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});

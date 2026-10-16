#!/usr/bin/env bun
/** 📦️ las Rust artifact package router. */
import { runArtifactRustPackageMain,runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {BundleScript,scriptInvocationBudget} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {resolve} from "node:path";
/** ☁️ Qualifies complete LAS original receiving under its authored caller. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("original LAS receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs the actual LAS physical receiving laws under the original finite policy. */
class OriginalReceivingNative extends OriginalReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-las",this.repoRoot,["--lib","sqlite_snapshot_las_original_receiving_","--no-fail-fast"],this.invocation.control);}
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-las", { packages: ["semio-s-artifact-stdio-las", "semio-s-artifact-stdio-las-mutations", "semio-s-artifact-stdio-las-schema", "semio-s-artifact-stdio-las-snapshot", "semio-s-artifact-stdio-las-tests", "semio-s-artifact-stdio-las-tests-2"],commands:{"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative}, snapshotSqliteTests: ["../../🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });

#!/usr/bin/env bun
/** 📦️ jpg Rust artifact package router. */
import { runArtifactRustPackageMain,runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {resolve} from "node:path";
import {BundleScript,scriptInvocationBudget} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 📸️ Qualifies complete JPEG semantic receiving inputs under the original invocation. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("original JPEG receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs actual original JPEG receiving laws in its owning artifact package. */
class OriginalReceivingNative extends OriginalReceivingSource{async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-jpg",this.repoRoot,["--lib","jpeg_original_receiving_","--no-fail-fast"],this.invocation.control);}}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-jpg", {commands:{"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative},snapshotSqliteTests: ["../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts", "../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });

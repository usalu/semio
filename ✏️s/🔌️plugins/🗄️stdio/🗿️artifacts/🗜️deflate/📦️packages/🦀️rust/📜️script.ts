#!/usr/bin/env bun
/** 📦️ deflate Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {resolve} from "node:path";
import {BundleScript,scriptInvocationBudget} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
/** 🗜️ Qualifies neutral complete Deflate receiving inputs under the original invocation. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("original Deflate receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs genuine complete Deflate receiving laws in its owning package. */
class OriginalReceivingNative extends OriginalReceivingSource{async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-deflate",this.repoRoot,["--lib","deflate_original_receiving_","--no-fail-fast"],this.invocation);}}
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-stdio-deflate",{ packages: ["semio-s-artifact-stdio-deflate", "semio-s-artifact-stdio-deflate-examples", "semio-s-artifact-stdio-deflate-mutations", "semio-s-artifact-stdio-deflate-schema", "semio-s-artifact-stdio-deflate-snapshot", "semio-s-artifact-stdio-deflate-tests", "semio-s-artifact-stdio-deflate-zlib"],commands:{"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative},snapshotSqliteTests:["../../🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});

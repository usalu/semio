#!/usr/bin/env bun
/** 📦️ obj Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {resolve} from "node:path";
import {BundleScript,scriptInvocationBudget} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
/** 🗽️ Qualifies authored original OBJ complete semantic receiving inputs. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("original OBJ receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs genuine original OBJ receiving laws in the owning artifact package. */
class OriginalReceivingNative extends OriginalReceivingSource{async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-obj",this.repoRoot,["--lib","obj_original_receiving_","--no-fail-fast"],this.invocation);}}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-obj", { packages: ["semio-s-artifact-stdio-obj", "semio-s-artifact-stdio-obj-diff", "semio-s-artifact-stdio-obj-inferences", "semio-s-artifact-stdio-obj-io-registry", "semio-s-artifact-stdio-obj-mutations", "semio-s-artifact-stdio-obj-schema"], commands:{"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative},snapshotSqliteTests: ["../../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });

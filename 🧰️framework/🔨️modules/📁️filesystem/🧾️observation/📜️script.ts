#!/usr/bin/env bun
import {resolve} from "node:path";
import {BundleScript,ScriptRouter} from "../../🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {runOwnedCommand} from "../../🏃️process/🎛️owned-execution/🟦️.ts";
import {cmdBudgetMs} from "../../🏃️process/⏱️budget/🟦️.ts";
/** 🧾️ Runs the defining portable physical observation and strict implementation checks. */
class TestScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("physical observation test accepts no arguments");
  await runOwnedCommand(process.execPath,[resolve(this.root,"📜️script.ts"),"test-worker"],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions",resolve(this.root,"🟦️.ts")],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
 }
}
/** 🧪️ Executes the complete portable fixture roster in its defining source owner. */
class WorkerScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw Error("physical observation worker accepts no arguments");const {runFileObservationChecksV1}=await import("./🧪️tests/🟦️.ts");await runFileObservationChecksV1();}}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("test-worker",WorkerScript),{defaultCommand:"test"});

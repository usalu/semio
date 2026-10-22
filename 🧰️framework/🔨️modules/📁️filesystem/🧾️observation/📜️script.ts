#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {resolve} from "node:path";
import {BundleScript,ScriptRouter} from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runOwnedCommand} from "../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {cmdBudgetMs} from "../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧾️ Runs the defining portable physical observation and strict implementation checks. */
class TestScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length===1&&segments[0]==="journal"){
   await runOwnedCommand(process.execPath,["test",resolve(this.root,"📥️journal/🧪️tests/🟦️.ts")],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
   await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions",resolve(this.root,"📥️journal/🟦️.ts")],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
   return;
  }
  if(segments.length)throw new Error("physical observation test accepts no arguments");
  await runOwnedCommand(process.execPath,["test",resolve(this.root,"🧪️tests/📁️descriptor/🟦️.ts"),resolve(this.root,"🧪️tests/📄️publication/🟦️.ts")],this.repoRoot,"filesystem:rooted-descriptors",cmdBudgetMs(),{env:process.env});
  await runOwnedCommand(process.execPath,[resolve(this.root,"📜️script.ts"),"test-worker"],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions",resolve(this.root,"🟦️.ts"),resolve(this.root,"📁️current/🟦️.ts"),resolve(this.root,"🧪️tests/🟦️.ts")],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
 }
}
/** 🧪️ Executes the complete portable fixture roster in its defining source owner. */
class WorkerScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("physical observation worker accepts no arguments");const {runFileObservationChecksV1,runCurrentFileRecheckChecksV1}=await import("./🧪️tests/🟦️.ts");await runCurrentFileRecheckChecksV1();await runFileObservationChecksV1();}}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test",TestScript).register("test-worker",WorkerScript), { invocation: original, ...({defaultCommand:"test"}) }));

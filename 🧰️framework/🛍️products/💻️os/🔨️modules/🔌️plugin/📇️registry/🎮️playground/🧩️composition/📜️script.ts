#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {resolve} from "node:path";
import {mkdir} from "node:fs/promises";
import {BundleScript,ScriptRouter} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {cmdBudgetMs} from "../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import {readProcessOwnerContextV1,processCacheDirectoryV1} from "../../../../../../../🔨️modules/🏃️process/📋️context/🟦️.ts";

/** 🧩️ Runs both implementations and their actual caller projections under owned process control. */
class TestScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("playground composition test accepts no arguments");
  await runOwnedCommand(process.execPath,[resolve(this.root,"📜️script.ts"),"test-worker"],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions",resolve(this.root,"🟦️.ts")],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
 }
}
/** 🧪️ Executes every portable case and the actual discovery, catalog and Nx receiving ports. */
class WorkerScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("playground composition worker accepts no arguments");
  const context=readProcessOwnerContextV1(process.env,process.cwd()),output=process.env.SEMIO_TEST_ARTIFACT_DIR??processCacheDirectoryV1(context,"playground-composition");await mkdir(output,{recursive:true});
  const {provePlaygroundCompositionV1}=await import("./🧪️tests/🟦️.ts");await provePlaygroundCompositionV1(this.repoRoot,output,true);
 }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("test-worker",WorkerScript), { invocation: original, ...({defaultCommand:"test"}) }));

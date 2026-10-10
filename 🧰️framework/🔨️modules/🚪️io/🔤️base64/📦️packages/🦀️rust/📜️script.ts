#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {resolve} from "node:path";
import {BundleScript,ScriptRouter} from "../../../../🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import {runCargoTestsV1,readCargoTestPolicyV1} from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {resolveTestLevel,testLevelBudgetMs} from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import {cmdBudgetMs} from "../../../../🏃️process/⏱️budget/🟦️.ts";

/** 🔤️ Runs every standard, URL and controlled allocation law in the defining native owner. */
class NativeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const{rest}=resolveTestLevel(segments);
  if(rest.length)throw Error("test-native accepts only an execution level");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-io-base64"],cwd:this.root,extraArgs:["--all-targets","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
 }
}
/** 🧫️ Runs the complete shared corpus against the TypeScript codec and independent system bytes. */
class SourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-source accepts no arguments");
  await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🔬️unit/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}
/** 🛂️ Enforces strict typing for the production codec and its complete shared corpus. */
class StrictScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-strict accepts no arguments");
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",resolve(this.root,"../../🧪️tests/🔬️unit/🟦️.ts")],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});
 }
}
const router=new ScriptRouter(import.meta.dir).register("test-native",NativeScript).register("test-source",SourceScript).register("test-strict",StrictScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({defaultCommand:"test-native"}) }));

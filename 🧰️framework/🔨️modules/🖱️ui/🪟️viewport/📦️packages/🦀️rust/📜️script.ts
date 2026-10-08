#!/usr/bin/env bun
import {resolve} from "node:path";
import {BundleScript,ScriptRouter} from "../../../../🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import {runCargoTestsV1,readCargoTestPolicyV1} from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {resolveTestLevel,testLevelBudgetMs} from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import {cmdBudgetMs} from "../../../../🏃️process/⏱️budget/🟦️.ts";

/** 🚪️ Runs every native vocabulary law without selecting a test name. */
class NativeScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    const{rest}=resolveTestLevel(segments);
    if(rest.length)throw Error("test-native accepts only an execution level");
    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-ui-viewport"],cwd:this.root,extraArgs:["--all-targets","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
  }
}
/** 🪆️ Checks canonical owner bindings against independent schema and SQLite fields. */
class BindingScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-binding accepts no arguments");
  const source=resolve(this.root,"../../🧪️tests/🪆️binding/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

const router=new ScriptRouter(import.meta.dir).register("test-native",NativeScript).register("test-binding",BindingScript);
await runScriptMain(router,{defaultCommand:"test-binding"});

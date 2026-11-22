#!/usr/bin/env repo
import {join} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 🚦️ Qualifies optional graph maintenance against actual reader and exclusive image custody, one process per file because the files belong to different suites with different declared births. */
class TestScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Graph maintenance tests accept no arguments");for(const file of[join(this.root,"🧪️tests/🟦️.ts"),join(this.root,"🧪️tests/📒️ledger/🟦️.ts"),join(this.root,"../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts"),join(this.root,"../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts")])await runBudgetedTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,600000),signal:this.invocation.control.signal,throwOnFailure:true});}}
const router=new ScriptRouter(import.meta.dirname).register("test",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test"}));

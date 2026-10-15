#!/usr/bin/env bun
import {join} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 🚦️ Qualifies optional graph maintenance against actual reader and exclusive image custody. */
class TestScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw Error("Graph maintenance tests accept no arguments");await runBudgetedTestCommand(process.execPath,["test",join(this.root,"🧪️tests/🟦️.ts"),join(this.root,"🧪️tests/📒️ledger.ts"),join(this.root,"../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,60000),signal:this.invocation.control.signal,throwOnFailure:true});}}
const router=new ScriptRouter(import.meta.dir).register("test",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test"}));

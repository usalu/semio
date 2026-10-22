#!/usr/bin/env repo
import {join} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 🧪️ Runs actual finite Storybook owner layout and writer lifetime laws. */
class TestScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Storybook storage tests accept no arguments");await runBudgetedTestCommand(process.execPath,["test",join(this.root,"🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,30000),signal:this.invocation.control.signal,throwOnFailure:true});}}
const router=new ScriptRouter(import.meta.dirname).register("test",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test"}));

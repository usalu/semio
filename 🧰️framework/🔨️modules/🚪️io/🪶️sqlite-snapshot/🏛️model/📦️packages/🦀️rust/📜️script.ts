#!/usr/bin/env bun
import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { scriptInvocationBudget, BundleScript, ScriptRouter } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { cmdBudgetMs } from "../../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

/** 🏛️ Checks real model ownership against independent TOML, Ajv and SQLite. */
class SourceScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw Error("test-source accepts no arguments");await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🏛️ownership.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,15000),signal:this.invocation.control.signal,throwOnFailure:true});}}
const router=new ScriptRouter(import.meta.dir).register("test-source",SourceScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-source"}));

#!/usr/bin/env bun
import {resolve} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runRepositoryCargoTestsV1} from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import {repositoryCargoTestPolicyV1} from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 🧬️ Checks neutral projection ownership and independent identity admission. */
class ContractScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw Error("Borrowed reference contract accepts no arguments");await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,60000),signal:this.invocation.control.signal,throwOnFailure:true});}}
/** 🧒️ Executes original bounded borrowed reference laws. */
class TestScript extends Script{async run(args:string[]):Promise<void>{await runRepositoryCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-artifact-reference-borrow"],cwd:this.root,extraArgs:args},repositoryCargoTestPolicyV1(resolve(this.root,"Cargo.toml"),this.root,process.env),this.repoRoot,this.invocation);}}
const router=new ScriptRouter(import.meta.dir).register("test-native",TestScript).register("test-contract",ContractScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-contract"}));

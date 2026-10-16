#!/usr/bin/env bun
/** 🔗️ Durable link package script: native codec tests and the schema contract check. */
import {resolve} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget,createIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runRepositoryCargoTestsV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import {repositoryCargoTestPolicyV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 🧬️ Checks canonical durable link schema and independent schema ownership. */
class ContractScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw Error("Link schema contract accepts no arguments");await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,60000),signal:this.invocation.control.signal,throwOnFailure:true});}}
/** 🧒️ Executes original bounded link schema laws. */
class TestScript extends Script{async run(args:string[]):Promise<void>{const phaseControl=createIssuedResourceControlV1(this.invocation,"cargo:package-tests",0);await runRepositoryCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-store-link-schema"],cwd:this.root,extraArgs:args},repositoryCargoTestPolicyV1(resolve(this.root,"Cargo.toml"),this.root,process.env),this.repoRoot,this.invocation,phaseControl);}}
const router=new ScriptRouter(import.meta.dir).register("test-native",TestScript).register("test-contract",ContractScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-contract"}));

#!/usr/bin/env repo
/** 🪪️ Blob identity package script: native identity law tests and the schema contract check. */
import {resolve} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget,createIssuedResourceControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runAdmittedCargoTestsV1,admittedCargoTestPolicyV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
/** 🧬️ Checks canonical durable blob identity and independent schema ownership. */
class ContractScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Blob identity contract accepts no arguments");await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,60000),signal:this.invocation.control.signal,throwOnFailure:true});}}
/** 🧒️ Executes original bounded blob identity laws. */
class TestScript extends Script{async run(args:string[]):Promise<void>{const phaseControl=createIssuedResourceControlV1(this.invocation,"cargo:package-tests",0);await runAdmittedCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-store-blob-identity"],cwd:this.root,extraArgs:args},await admittedCargoTestPolicyV1(resolve(this.root,"Cargo.toml"),this.root,process.env),this.repoRoot,this.invocation,phaseControl);}}
const router=new ScriptRouter(import.meta.dirname).register("test-native",TestScript).register("test-contract",ContractScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-contract"}));

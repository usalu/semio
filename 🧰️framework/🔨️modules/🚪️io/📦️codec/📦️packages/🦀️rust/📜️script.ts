#!/usr/bin/env bun
import {resolve} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget,createIssuedResourceControlV1} from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
/** 🧬️ Checks the complete owned contract with independent schema and capacity oracles. */
class ContractScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw new Error("IO codec contract accepts no arguments");await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,60000),signal:this.invocation.control.signal,throwOnFailure:true});}}
/** 🧪️ Runs every original native codec law through genuine bounded Cargo admission. */
class NativeScript extends Script{async run(args:string[]):Promise<void>{const phaseControl=createIssuedResourceControlV1(this.invocation,"cargo:package-tests",0);if(args.length)throw new Error("IO codec native laws accept no arguments");await runAdmittedCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-io-codec"],cwd:this.root,extraArgs:["--lib","--no-fail-fast"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation,phaseControl);}}
const router=new ScriptRouter(import.meta.dir).register("test-contract",ContractScript).register("test-native",NativeScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-contract"}));

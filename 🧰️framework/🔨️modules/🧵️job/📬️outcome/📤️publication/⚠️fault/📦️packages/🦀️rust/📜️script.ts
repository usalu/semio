#!/usr/bin/env bun
import{resolve}from"node:path";
import{Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,createIssuedResourceControlV1}from"../../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runAdmittedCargoTestsV1, admittedCargoTestPolicyV1 } from "../../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
/** ⚠️ Runs typed fault publication laws through their actual bounded Original package invocation. */
class TestScript extends Script{async run(args:string[]):Promise<void>{const phaseControl=createIssuedResourceControlV1(this.invocation,"cargo:package-tests",0);if(args.length)throw Error("Job typed-fault publication native suite accepts no filters");const manifest=resolve(this.root,"Cargo.toml");await runAdmittedCargoTestsV1({manifestPath:manifest,packages:["semio-framework-job-fault-publication"],cwd:this.root},await admittedCargoTestPolicyV1(manifest,this.root,process.env),this.repoRoot,this.invocation,phaseControl);}}
const router=new ScriptRouter(import.meta.dir).register("test-native",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-native"}));

#!/usr/bin/env repo
import {resolve} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,createIssuedResourceControlV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runAdmittedCargoTestsV1, admittedCargoTestPolicyV1 } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
/** 🧱️ Qualifies this complete native UI foundation through its genuine compiler owner. */
class TestScript extends Script{async run(args:string[]):Promise<void>{const phaseControl=createIssuedResourceControlV1(this.invocation,"cargo:package-tests",0);await runAdmittedCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-ui-contract-identity"],cwd:this.root,extraArgs:args},await admittedCargoTestPolicyV1(resolve(this.root,"Cargo.toml"),this.root,process.env),this.repoRoot,this.invocation,phaseControl);}}
const router=new ScriptRouter(import.meta.dirname).register("test-native",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-native"}));

#!/usr/bin/env bun
import {join} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationEnvironment} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandV1} from "@semio-tech/process/execution";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
/** 🧪️ Runs schema and genuine finite artifact reader lifetime laws. */
class TestScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Artifact reader tests accept no arguments");const control=scriptCompilerAdmissionControlV1(this.invocation,"artifact:reader-tests",60000),env=scriptInvocationEnvironment(this.invocation,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"});for(const selection of[["test",join(this.root,"🧪️tests/🟦️.ts"),join(this.root,"🧪️tests/🔒️scope/🟦️.ts")],["test",join(this.root,"../💾️storage/🧪️tests/📥️physical/🟦️.ts"),"--test-name-pattern=^physical scratch publication retains"]])process.stdout.write(await captureOwnedCommandV1(process.execPath,selection,this.repoRoot,{control,env,output:{maximumBytes:1048576,maximumLines:65536}}));}}
const router=new ScriptRouter(import.meta.dir).register("test",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test"}));

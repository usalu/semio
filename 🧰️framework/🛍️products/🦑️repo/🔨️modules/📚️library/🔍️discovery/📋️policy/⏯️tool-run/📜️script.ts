#!/usr/bin/env bun
import {join,resolve} from "node:path";
import {BundleScript,ScriptRouter,advanceScriptInvocation,scriptInvocationBudget,receiveScriptProcessInvocation,runScriptMain} from "@semio-tech/framework/process/routing";
import {runOwnedCommand} from "@semio-tech/framework/process/execution";

/** 🧭️ Runs the complete Repository ToolRun policy contract under the original caller control. */
export class ToolRunPolicySourceScript extends BundleScript {
  async run(args:string[]):Promise<void> {
    if(args.length)throw Error("Expected test-tool-run-policy-source with no arguments");
    const source=resolve(import.meta.dir,"../../🧪️tests/🧪️interactivity-tool-run-policy/🟦️.ts");
    const options={signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"test-tool-run-policy-source","running")};
    await advanceScriptInvocation(this.invocation,"test-tool-run-policy-source","running");
    await runOwnedCommand(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,import.meta.path],this.repoRoot,"repository:tool-run-policy:types",scriptInvocationBudget(this.invocation,30000),options);
    await advanceScriptInvocation(this.invocation,"test-tool-run-policy-source","running");
    await runOwnedCommand(process.execPath,[source],this.repoRoot,"repository:tool-run-policy:laws",scriptInvocationBudget(this.invocation,30000),options);
    await advanceScriptInvocation(this.invocation,"test-tool-run-policy-source","complete");
  }
}

if(import.meta.main){
  const router=new ScriptRouter(import.meta.dir).register("test",ToolRunPolicySourceScript);
  await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
}

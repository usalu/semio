#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget,advanceScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCargoTestsV1 } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
/** 🧪️ Runs the owned neutral corpus against its independent implementation. */
class SourceScript extends BundleScript {async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-source");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts")],this.repoRoot,"math-rank:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"test-source","running")});}}
/** 🦀️ Runs original native math ownership laws through repository Cargo admission. */
class TestScript extends BundleScript {async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test");await runRepositoryCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-math-rank"],cwd:this.root,extraArgs:["--lib"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation);}}
const router=new ScriptRouter(import.meta.dir).register("test-source",SourceScript).register("test",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

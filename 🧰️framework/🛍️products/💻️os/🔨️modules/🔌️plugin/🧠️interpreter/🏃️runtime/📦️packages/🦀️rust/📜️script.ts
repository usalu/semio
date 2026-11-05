#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCargo,runOwnedCommand,resolveTypescriptCompilerV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter,scriptInvocationBudget } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

import { resolve } from "node:path";

/** 🧠️ Type-checks the owned interpreter runtime crate. */
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

/** 🧪️ Runs the owned interpreter runtime crate's library laws. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

/** 🧮️ Checks the canonical Original byte-call and poll framing fixtures against independent relational oracles. */
class OriginalSourceScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("Original source checks accept no arguments");const tests=["🎟️call/🧪️tests/🟦️.ts","🎟️call/🧮️unit/🧪️tests/🟦️.ts","🎟️call/🔄️poll/🧪️tests/🟦️.ts","🏗️startup/🧪️tests/🟦️.ts"].map(path=>resolve(this.root,"../..",path));for(const command of [[resolveTypescriptCompilerV1(this.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",...tests],["test",...tests]])await runOwnedCommand(process.execPath,command,this.repoRoot,"plugin-interpreter-runtime:original:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});}
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test-original-source",OriginalSourceScript).register("check", CheckScript).register("test", TestScript), { invocation: original }));

#!/usr/bin/env bun
import{resolve}from"node:path";
import{BundleScript,ScriptRouter,runScriptMain,receiveScriptProcessInvocation}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{runOwnedCommand}from"../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
/** 🔺️ Executes the neutral sparse algebra against the independent JSON Patch oracle. */
class TestScript extends BundleScript{async run():Promise<void>{const file=resolve(this.root,"🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath,["test",file],this.root,"mutation-diff:test",120000);await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"mutation-diff:types",120000);}}
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript),{invocation:original,defaultCommand:"test"}));

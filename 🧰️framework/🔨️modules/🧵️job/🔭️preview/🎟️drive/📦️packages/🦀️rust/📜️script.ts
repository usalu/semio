#!/usr/bin/env bun
import{resolve}from"node:path";
import{Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain}from"../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{runRepositoryCargoTestsV1}from"../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import{repositoryCargoTestPolicyV1}from"../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🟦️.ts";
/** 🔭️ Runs every preview owner law through its actual original bounded package invocation. */
class TestScript extends Script{async run(args:string[]):Promise<void>{if(args.length)throw Error("Job preview native suite accepts no filters");const manifest=resolve(this.root,"Cargo.toml");await runRepositoryCargoTestsV1({manifestPath:manifest,packages:["semio-framework-job-preview"],cwd:this.root},repositoryCargoTestPolicyV1(manifest,this.root,process.env),this.repoRoot,this.invocation);}}
const router=new ScriptRouter(import.meta.dir).register("test-native",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-native"}));

#!/usr/bin/env bun
import {resolve} from "node:path";
import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runRepositoryCargoTestsV1} from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import {repositoryCargoTestPolicyV1} from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🟦️.ts";
/** 🧱️ Qualifies this complete native UI foundation through its genuine compiler owner. */
class TestScript extends Script{async run(args:string[]):Promise<void>{await runRepositoryCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-ui-contract-text"],cwd:this.root,extraArgs:args},repositoryCargoTestPolicyV1(resolve(this.root,"Cargo.toml"),this.root,process.env),this.repoRoot,this.invocation);}}
const router=new ScriptRouter(import.meta.dir).register("test-native",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test-native"}));

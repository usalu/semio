#!/usr/bin/env bun
import {join} from "node:path";
import {BundleScript,ScriptRouter,runScriptMain,receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {verifyCapturedCargoArtifactsV1} from "../../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import {buildRepositoryCargoArtifacts} from "../../../../../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import {repositoryCargoNetworkV1} from "../../../../../../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
/** 🦀️ Builds the authored Native reader through canonical compiler input and capture ownership. */
class BuildScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Native reader build accepts no arguments");const control=createIssuedResourceControlV1(this.invocation,"native:reader-build",0);
  await buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"),["-p","semio-framework-os-native-runtime-reader","--bin","semio-native-runtime-reader"],this.repoRoot,{invocation:this.invocation,producer:"artifact",network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),control,consume:verifyCapturedCargoArtifactsV1,environment:process.env});
 }
}
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(new ScriptRouter(import.meta.dir).register("build",BuildScript),{invocation:original}));

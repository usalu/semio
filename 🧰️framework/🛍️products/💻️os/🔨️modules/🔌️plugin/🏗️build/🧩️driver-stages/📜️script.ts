#!/usr/bin/env repo
import { repositoryCargoNetworkV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildComponentClosureV1 } from "./🟦️.ts";
import { descriptorEmitterPinArgumentsV1 } from "../../🖨️describe/🛠️emitter/📋️contract/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../🔨️modules/⚙️runtime/📄️parse/🟦️.ts";

/** 🧩️ Compiles and publishes precisely one authored plugin component through its original caller. */
class ComponentScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args[0]!=="--manifest"||!(args.length===2||args.length===4&&args[2]==="--published-emitter"))throw new Error("component --manifest <Cargo.toml> [--published-emitter <binarySha256:receiptSha256:indexSha256>]");
  const emitterPin=args.length===4?descriptorEmitterPinArgumentsV1(args[3]!,readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode,process.platform):undefined;
  const manifest=args[1],metadata=(runtimeParseTomlV1(readFileSync(resolve(this.repoRoot,manifest),"utf8")) as {package?:{metadata?:{component?:{package?:string}}}}).package?.metadata?.component?.package;
  if(!metadata?.startsWith("semio:"))throw new Error(`Not a plugin component manifest: ${manifest}`);
  const outcome=await buildComponentClosureV1(this.repoRoot,new Set([metadata.slice("semio:".length)]),{facet:"browser",invocation:this.invocation,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),control:scriptCompilerAdmissionControlV1(this.invocation,"component:driver",0),...(emitterPin?{emitterPin}:{})});
  console.log(`[component] ${metadata}: built, described and materialized; report ${outcome.report}`);
 }
}
const router=new ScriptRouter(import.meta.dirname).register("component",ComponentScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

#!/usr/bin/env bun
import {repositoryCargoNetworkV1} from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildComponentClosureV1 } from "./🟦️.ts";

/** 🧩️ Compiles and publishes precisely one authored plugin component through its original caller. */
class ComponentScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==2||args[0]!=="--manifest")throw Error("component --manifest <Cargo.toml>");
  const manifest=args[1],metadata=(Bun.TOML.parse(readFileSync(resolve(this.repoRoot,manifest),"utf8")) as {package?:{metadata?:{component?:{package?:string}}}}).package?.metadata?.component?.package;
  if(!metadata?.startsWith("semio:"))throw Error(`Not a plugin component manifest: ${manifest}`);
  const outcome=await buildComponentClosureV1(this.repoRoot,new Set([metadata.slice("semio:".length)]),{facet:"browser",invocation:this.invocation,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),control:scriptCompilerAdmissionControlV1(this.invocation,"component:driver",0)});
  console.log(`[component] ${metadata}: built, described and materialized; report ${outcome.report}`);
 }
}
const router=new ScriptRouter(import.meta.dir).register("component",ComponentScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

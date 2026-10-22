#!/usr/bin/env repo
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {BundleScript,ScriptRouter,receiveScriptProcessInvocation,runScriptMain} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {GenerateScript,CheckGeneratedScript,PreviewGeneratedScript,CheckScript} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import {specificCatalogPublicationV1} from "./🟦️.ts";

const router=new ScriptRouter(resolve(import.meta.dir,"../.."),resolve(import.meta.dir,"../../../.."));
for(const [name,Selected] of Object.entries({generate:GenerateScript,check:CheckGeneratedScript,preview:PreviewGeneratedScript,verify:CheckScript})){
 router.register(name,class extends BundleScript{
  async run(args:string[]):Promise<void>{
   if(args.length)throw Error("Catalog publication commands accept no arguments");
   await new Selected(this.root,this.repoRoot,this.invocation,specificCatalogPublicationV1(path=>readFileSync(resolve(this.repoRoot,path),"utf8"))).run([]);
  }
 });
}
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

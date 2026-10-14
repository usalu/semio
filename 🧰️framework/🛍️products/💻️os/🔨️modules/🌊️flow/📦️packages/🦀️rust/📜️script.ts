#!/usr/bin/env bun
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCargoTests } from "../../../../../🦑️repo/🔨️modules/📚️library/🟦️.ts";

/** 🧪️ Runs the original extension contribution laws with their defining Flow owner. */
class ExtensionRegistryScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-extension-registry accepts no arguments");
  await runRepositoryCargoTests(["semio-framework-os-flow"],this.root,this.invocation,["--lib","registry::contribution_tests"]);
 }
}
const router=new ScriptRouter(import.meta.dir).register("test-extension-registry",ExtensionRegistryScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));

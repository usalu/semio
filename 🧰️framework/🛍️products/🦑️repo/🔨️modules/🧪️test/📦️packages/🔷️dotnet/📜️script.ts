#!/usr/bin/env repo
import { receiveScriptProcessInvocation, BundleScript, ScriptRouter, runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runDepsScript, runBuildScript } from "../../🧪️tests/🔬️dotnet-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

/** 📥️ Restores the locked support dependencies inside their actual finite shared backing. */
class DepsScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runDepsScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
/** 📦️ Publishes the support library's unchanged Release ABI into the current intent's fixed product tree. */
class BuildScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dirname).register("deps",DepsScript).register("build",BuildScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

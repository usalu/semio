#!/usr/bin/env repo
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnerCommandScript } from "./🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";

/** 📥️ Executes the literal discovered command within its genuine original process owner authority. */
class OwnerCommandScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOwnerCommandScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

const router=new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("owner-command",OwnerCommandScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

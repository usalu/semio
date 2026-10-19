#!/usr/bin/env bun
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnerCommandScript } from "./🟦️.ts";

/** 📥️ Executes the literal discovered command within its genuine original process owner authority. */
class OwnerCommandScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOwnerCommandScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

const router=new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("owner-command",OwnerCommandScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

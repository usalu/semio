#!/usr/bin/env bun
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, scriptInvocationBudget, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { consumeOwnerArgumentsV1 } from "../../../🏗️build/📤️arguments/🟨️.mjs";
import { devToolingEnv } from "../../🌿️environment/🟦️.ts";
import { repositoryProcessOwnerContextV1, repositoryVitestPolicyV1 } from "../../📋️context/🔣️policy/🟦️.ts";

/** 📥️ Executes the literal discovered command within its genuine original process owner authority. */
class OwnerCommandScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args[0]!=="--cwd"||!args[1]||args[2]!=="--"||!args[3])throw Error("owner-command --cwd <directory> -- <command> <args>");
  const transported=consumeOwnerArgumentsV1(process.env),cwd=resolve(this.repoRoot,args[1]),environment=devToolingEnv({SEMIO_VITEST_POLICY:JSON.stringify(repositoryVitestPolicyV1(cwd)),SEMIO_PROCESS_OWNER_CONTEXT:JSON.stringify(repositoryProcessOwnerContextV1(cwd))});
  delete environment.SEMIO_OWNER_ARGUMENTS;
  await runOwnedCommand(args[3],[...args.slice(4),...transported.arguments],cwd,"process:owner-command",scriptInvocationBudget(this.invocation,0),{env:environment,signal:this.invocation.control.signal});
 }
}

const router=new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("owner-command",OwnerCommandScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

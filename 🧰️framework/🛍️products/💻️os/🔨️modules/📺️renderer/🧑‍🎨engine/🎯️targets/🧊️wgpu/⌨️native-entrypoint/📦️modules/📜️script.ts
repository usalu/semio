#!/usr/bin/env repo
import { readNativeCallerAuthorityV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import {receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
import { nativeRuntimeDirectory, publishNativePlaygroundRuntimeV1 } from "./🟦️.ts";

/** 📥️ Publishes the native runtime from the session and the modules its `component` targets already staged. */
class PublishScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, selectedProfile] = args, profile = selectedProfile as "prototype" | "release", repo = this.repoRoot, controller = new AbortController(), cancel = () => controller.abort();
    if (args.length !== 2||!variant) throw new Error("publish <variant> <prototype|release>");
    nativeRuntimeDirectory(repo,variant!,profile);
    if(profile!==readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode)throw new Error("Native publication must match actual Original mode");
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await publishNativePlaygroundRuntimeV1(repo,this.invocation,variant,controller.signal);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("publish", PublishScript)).run(process.argv.slice(2), original));

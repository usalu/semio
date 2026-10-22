#!/usr/bin/env repo
import { readNativeCallerAuthorityV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import {withPlaygroundSessionV1} from "../../../../../../🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {createRequestedIssuedResourceControlV1, receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import {runtimeCatalogBuildInventoryV1} from "../../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { isAbsolute, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { builtPluginModulesRootIn } from "../../../../../../🧑‍💻dev/📦️packages/🟦️typescript/♻️activation/🟦️.ts";
import { moduleDirectoryName } from "../../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE, TRUSTED_PLUGIN_MODULE_RECEIPT_FILE } from "../../../../../../🔌️plugin/📇️registry/📦️deployment/🧬️schema/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, SHIPPED_COMPONENT_PROFILE } from "../../../../../../🔌️plugin/📦️packages/🦀️rust/🖨️describe/🏷️component-receipt/🟦️.ts";
import { nativeRuntimeDirectory, publishNativeRuntime } from "./🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const registryPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";

/** 📥️ Publishes the native runtime from the session and the modules its `component` targets already staged. */
class PublishScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, selectedProfile] = args, profile = selectedProfile as "prototype" | "release", repo = this.repoRoot, controller = new AbortController(), cancel = () => controller.abort();
    if (args.length !== 2||!variant) throw new Error("publish <variant> <prototype|release>");
    nativeRuntimeDirectory(repo,variant!,profile);
    if(profile!==readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode)throw new Error("Native publication must match actual Original mode");
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await withPlaygroundSessionV1(repo,this.invocation,variant,async(session,projection,control)=>{
      const catalog=projection.entries,phase=createRequestedIssuedResourceControlV1(control,0,controller.signal);
      if (session.variant !== variant || !Array.isArray(session.plugins)) throw new Error("Native session identity mismatch");
      const modules=session.plugins.map(plugin=>{const entry=catalog.find(row=>row.pluginId===plugin.pluginId);if(!entry)throw new Error("Missing Native component declaration: "+plugin.pluginId);return{pluginId:entry.pluginId,cratePath:entry.cratePath};});
      await publishNativeRuntime(repo,this.invocation,variant,modules,phase);
      console.log(`Published native ${variant} ${profile}: ${modules.length} completed components`);
      });
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("publish", PublishScript)).run(process.argv.slice(2), original));

#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import {runtimeCatalogBuildInventoryV1} from "../../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { isAbsolute, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { pluginModulesRootIn } from "../../../../../../🧑‍💻dev/♻️activation/🟦️.ts";
import { moduleDirectoryName } from "../../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE, TRUSTED_PLUGIN_MODULE_RECEIPT_FILE } from "../../../../../../🔌️plugin/📇️registry/📦️deployment/🧬️schema/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, SHIPPED_COMPONENT_PROFILE } from "../../../../../../🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { nativeRuntimeDirectory, publishNativeRuntime } from "./🟦️.ts";
import { buildComponentClosureV1 } from "../../../../../../🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts";

const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const registryPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";

/** 📥️ Builds exactly the session's components through the build driver, then publishes the native runtime from their modules and the session. */
class PublishScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, selectedProfile] = args, profile = selectedProfile as "prototype" | "release", repo = this.repoRoot, controller = new AbortController(), cancel = () => controller.abort();
    if (args.length !== 2) throw new Error("publish <variant> <prototype|release>");
    nativeRuntimeDirectory(".", variant, profile);
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const catalog = readGeneratedCatalogProjection(join(repo,registryPath,"dist/sessions",variant)).entries;
      const session = (await import(pathToFileURL(join(repo, registryPath, "dist/sessions", variant, "🎮️playground-session/🟦️.ts")).href)).PLAYGROUND_SESSION;
      if (session.variant !== variant || !Array.isArray(session.plugins)) throw new Error("Native session identity mismatch");
      await buildComponentClosureV1(repo, new Set(session.plugins.map((plugin: { pluginId: string }) => plugin.pluginId)), { control: scriptCompilerAdmissionControlV1(this.invocation, "native:driver", 0) });
      const modules = session.plugins.map((plugin: { pluginId: string }) => {
        const entry = catalog.find((row: { pluginId: string }) => row.pluginId === plugin.pluginId);
        if (!entry) throw new Error(`Missing native catalog component: ${plugin.pluginId}`);
        const crate = resolve(repo, entry.cratePath), path = relative(repo, crate);
        if (isAbsolute(path) || path.split(/[\\/]/).includes("..") || !/^[a-z0-9_]+\.wasm$/.test(entry.wasmOut)) throw new Error("Invalid native component input path");
        const page = join(pluginModulesRootIn(repo), moduleDirectoryName(plugin.pluginId, runtimeCatalogBuildInventoryV1().moduleDirectories));
        return { pluginId: plugin.pluginId, wasm: join(crate, COMPONENT_PROFILE_POLICIES[SHIPPED_COMPONENT_PROFILE].directory, entry.wasmOut), descriptor: join(page, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE), descriptorPack: join(page, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE), receipt: join(page, TRUSTED_PLUGIN_MODULE_RECEIPT_FILE) };
      });
      await publishNativeRuntime(join(repo, ownerPath), variant, profile, modules, controller.signal);
      console.log(`Published native ${variant} ${profile}: ${modules.length} completed components`);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("publish", PublishScript)).run(process.argv.slice(2), original));

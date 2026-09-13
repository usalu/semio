#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { BundleScript, ScriptRouter } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { pluginModulesRootIn } from "../../../../../../🧑‍💻dev/♻️activation/🟦️.ts";
import { moduleDirectoryName } from "../../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { nativeRuntimeDirectory, publishNativeRuntime } from "./🟦️.ts";

const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const registryPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";

/** 📥️ Consumes only the outer graph's completed component, descriptor and session producers. */
class PublishScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [variant, selectedProfile] = args, profile = selectedProfile as "dev" | "release", repo = this.repoRoot, controller = new AbortController(), cancel = () => controller.abort();
    if (args.length !== 2) throw new Error("publish <variant> <dev|release>");
    nativeRuntimeDirectory(".", variant, profile);
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const catalog = JSON.parse(readFileSync(join(repo, registryPath, "🤖️generated/🔌️plugins.json"), "utf8"));
      const session = (await import(pathToFileURL(join(repo, registryPath, "dist/sessions", variant, "🎮️playground-session/🟦️.ts")).href)).PLAYGROUND_SESSION;
      if (session.variant !== variant || !Array.isArray(session.plugins)) throw new Error("Native session identity mismatch");
      const modules = session.plugins.map((plugin: { pluginId: string }) => {
        const entry = catalog.find((row: { pluginId: string }) => row.pluginId === plugin.pluginId);
        if (!entry) throw new Error(`Missing native catalog component: ${plugin.pluginId}`);
        const crate = resolve(repo, entry.cratePath), path = relative(repo, crate);
        if (isAbsolute(path) || path.split(/[\\/]/).includes("..") || !/^[a-z0-9_]+\.wasm$/.test(entry.wasmOut)) throw new Error("Invalid native component input path");
        return { pluginId: plugin.pluginId, wasm: join(crate, "dist", `component-${profile}`, entry.wasmOut), descriptor: join(pluginModulesRootIn(repo, profile), moduleDirectoryName(plugin.pluginId), "🔣️.json") };
      });
      await publishNativeRuntime(join(repo, ownerPath), variant, profile, modules, controller.signal);
      console.log(`Published native ${variant} ${profile}: ${modules.length} completed components`);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("publish", PublishScript).run(process.argv.slice(2));

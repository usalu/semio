import {existsSync,readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
import type {PluginBuildTargetV1} from "../../📇️registry/🔎️discovery/🟦️.ts";
import { hostShimSource } from "../../🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../../🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import {moduleDirectoryName,parseModuleDirectories} from "../../📇️registry/📦️deployment/🟦️.ts";
/** 🔎️Reports stale extension artifacts read-only, retaining bytes for the owning producer. `rebuilding`
 * names the extension crates the caller is about to produce and republish in this same run: their install
 * directory is replaced by its actual activation publisher, so reporting it as retained output would
 * refuse the one build that repairs it — the gate would then have no reachable exit, because its own
 * recovery instruction routes back through this check. */
function assertExtensionOutputsFresh(root: string, rebuilding: readonly PluginBuildTargetV1[] = []): void {
  if (!existsSync(root)) return;
  const currentHostShim = hostShimSource();
  const republished = new Set<string>(rebuilding.filter((target) => target.role === "extension").map((target) => moduleDirectoryName(target.pluginId, parseModuleDirectories({version: 1, modules: [{pluginId: target.pluginId, directoryName: target.directoryName}]}))));
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    if (republished.has(entry.name)) continue;
    const dir = join(root, entry.name);
    const retainedWorkerPath = join(dir, "🧵️plugin-worker.js");
    if (existsSync(retainedWorkerPath)) throw new Error(`Extension retained worker preserved: ${retainedWorkerPath}. Review this input before running @semio-tech/framework-os-dev:plugin for its owner.`);
    const shimPath = join(dir, PLUGIN_HOST_SHIM_FILE);
    if (existsSync(shimPath)) {
      if (readFileSync(shimPath, "utf8") !== currentHostShim) throw new Error(`Extension stale host shim preserved: ${shimPath}. Rebuild its owner through @semio-tech/framework-os-dev:plugin after reviewing the retained output.`);
    }
  }
}


export {assertExtensionOutputsFresh};

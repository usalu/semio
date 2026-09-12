import { createRequire } from "node:module";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { BundleScript, getWorkspaceRoot, runExactCargoLawProcess } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { stageArtifacts } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "../../../📇️registry/📦️deployment/🟦️.ts";
import { pluginModulesRoot } from "../../../../🧑‍💻dev/♻️activation/🟦️.ts";
import { ACTOR_COMPONENT_EXPORTS, PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../🛂️descriptor/🟦️.ts";
import { artifactFiles, fileDigest } from "../../📦️distribution/📋️inventory/🟦️.ts";
import { ensurePreview2ShimVendorAt, hostShimSource, PLUGIN_HOST_SHIM_FILE, PREVIEW2_VENDOR_RELATIVE, pluginComponentBridgeSource, SHARD_WORKER_FILE, shardWorkerSource, transpilePluginComponentAsync } from "../🟦️.ts";

type Profile = "dev" | "release";

/** 🎚️ Selects immutable browser output ownership before any producer runs. */
function componentProfile(value: string | undefined): Profile {
  if (value !== "dev" && value !== "release") throw new Error("Select a component profile: dev or release");
  return value;
}


/** 📦️ Enumerates regular staged files without admitting symlinks or unrelated mutable state. */
export class SupportScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1) throw new Error("Usage: support <dev|release>");
    const profile = componentProfile(args[0]), root = pluginModulesRoot(profile);
    mkdirSync(root, { recursive: true });
    const temporary = mkdtempSync(join(root, ".support-"));
    try {
      const vendor = join(temporary, "vendor"), shard = join(temporary, "shard");
      ensurePreview2ShimVendorAt(vendor, getWorkspaceRoot());
      mkdirSync(shard);
      writeFileSync(join(shard, SHARD_WORKER_FILE), shardWorkerSource());
      await stageArtifacts(join(root, PREVIEW2_VENDOR_RELATIVE), `browser-support:${profile}:preview2`, artifactFiles(vendor));
      await stageArtifacts(join(root, MODULE_SHARD_DIRECTORY), `browser-support:${profile}:shard`, artifactFiles(shard));
      console.log(`Browser support ${profile}: vendor shims and shard worker staged`);
    } finally { rmSync(temporary, { recursive: true, force: true }); }
  }
}

export class MaterializeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 3 || args[1] !== "--manifest") throw new Error("Usage: materialize <dev|release> --manifest <Cargo.toml>");
    const profile = componentProfile(args[0]), repo = getWorkspaceRoot(), manifestPath = resolve(repo, args[2]!);
    if (!manifestPath.startsWith(repo + sep) || lstatSync(manifestPath).isSymbolicLink()) throw new Error("Component manifest must belong to the workspace");
    const manifest = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(manifestPath, "utf8"));
    const metadata = manifest.package?.metadata, identity = metadata?.component?.package;
    if (typeof identity !== "string" || !/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/.test(identity) || !["plugin", "extension"].includes(metadata.semio?.role)) throw new Error("Expected a Cargo plugin or extension component");
    const pluginId = identity.slice(6), crate = manifest.package.name.replaceAll("-", "_"), componentBase = crate + "_component";
    const artifact = join(dirname(manifestPath), "dist", `component-${profile}`, crate + ".wasm");
    const root = pluginModulesRoot(profile), output = join(root, moduleDirectoryName(pluginId)), vendor = join(root, PREVIEW2_VENDOR_RELATIVE);
    if (!existsSync(artifact) || !existsSync(join(vendor, ".nx-artifact.json"))) throw new Error("Missing component or browser support prerequisite; run the materialize target through Nx");
    const controller = new AbortController(), cancel = (): void => controller.abort();
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    mkdirSync(root, { recursive: true });
    const temporary = mkdtempSync(output + ".materialize-");
    try {
      writeFileSync(join(temporary, PLUGIN_HOST_SHIM_FILE), hostShimSource());
      console.log(`Materializing ${pluginId} ${profile}: transpile`);
      await transpilePluginComponentAsync(artifact, temporary, componentBase, { repoRoot: repo, preview2VendorDir: vendor, signal: controller.signal, optimize: profile === "release" });
      controller.signal.throwIfAborted();
      console.log(`Materializing ${pluginId} ${profile}: descriptor`);
      const stdoutPath = join(temporary, ".descriptor.stdout"), stderrPath = join(temporary, ".descriptor.stderr");
      const probe = await runExactCargoLawProcess("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(temporary, componentBase + ".js")], { cwd: repo, env: process.env, budgetMs: 60_000, maxOutputBytes: 12 * 1024 * 1024, stdoutPath, stderrPath, cancelled: () => controller.signal.aborted });
      if (probe.status !== 0) throw new Error(`Plugin descriptor failed for ${pluginId}: ${probe.stderr}`);
      const base64 = probe.stdout.trim();
      if (!/^[A-Za-z0-9+/]+={0,2}$/.test(base64)) throw new Error(`Invalid descriptor response for ${pluginId}`);
      const descriptor = finalizePluginDescriptor(Buffer.from(base64, "base64"), pluginId, await fileDigest(artifact), await fileDigest(join(temporary, componentBase + ".core.wasm")));
      rmSync(stdoutPath); rmSync(stderrPath);
      writeFileSync(join(temporary, "🛂️.descriptor.semio"), descriptor.pack);
      writeFileSync(join(temporary, "🔣️.json"), descriptor.json);
      writeFileSync(join(temporary, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(componentBase, crate + ".wasm"));
      controller.signal.throwIfAborted();
      await stageArtifacts(output, relative(repo, manifestPath).split(sep).join("/") + `:browser:${profile}`, artifactFiles(temporary), { signal: controller.signal });
      console.log(`Materialized ${pluginId} ${profile}: browser bridge and descriptor staged -> ${output}`);
      // 📣️ A served dev session reads THIS directory directly, so a plugin is live the moment it is
      // staged; an extension is not — the runtime install root (`/🧩️extension-modules`) is written by
      // `activate-<variant>-<renderer>-<profile>`. Saying so is the difference between "the rebuild
      // did not take" and one more command (26/09/09 `📓️extension-invoke-door-2026-09-12.md` §6.3).
      if (metadata.semio.role === "extension") console.log(`Extension ${pluginId} is staged but NOT published to any runtime install root — run: bun nx run @semio-tech/framework-os-dev:activate-<variant>-<react|wgpu>-${profile}`);
    } finally {
      process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel);
      rmSync(temporary, { recursive: true, force: true });
    }
  }
}

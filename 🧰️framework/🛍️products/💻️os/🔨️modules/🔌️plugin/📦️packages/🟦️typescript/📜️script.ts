#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { createReadStream, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { decodePackValue, encodePackValue, packValueToExactJson, type PackValue } from "@semio-tech/framework-os";
import { BundleScript, ScriptRouter, getWorkspaceRoot, runBundleScriptMain, runExactCargoLawProcess } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { stageArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { ensurePreview2ShimVendorAt, hostShimSource, PLUGIN_HOST_SHIM_FILE, PREVIEW2_VENDOR_RELATIVE, pluginComponentBridgeSource, SHARD_WORKER_FILE, shardWorkerSource, transpilePluginComponentAsync } from "./🟦️.ts";
import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "../../📇️registry/📦️deployment/🟦️.ts";

const SCRIPT_ROOT = dirname(fileURLToPath(import.meta.url));
export const ACTOR_COMPONENT_EXPORTS = JSON.parse(readFileSync(join(SCRIPT_ROOT, "../../🧫️fixtures/🛂️actor-exports/🔣️.json"), "utf8")) as Record<string, string[]>;

/** 🛂️ Both package roles must expose the complete actor world before publication. */
export function assertActorComponentExports(component: Record<string, unknown>, required: Record<string, string[]>): void {
  for (const [name, methods] of Object.entries(required)) {
    const api = component[name] as Record<string, unknown> | undefined;
    for (const method of methods) {
      if (typeof api?.[method] !== "function") throw new Error(`Missing actor export ${name}.${method}`);
    }
  }
}

export const PLUGIN_DESCRIPTOR_PROBE_SOURCE = `
import { pathToFileURL } from "node:url";
${assertActorComponentExports.toString()}
const component = await import(pathToFileURL(process.argv[1]).href);
assertActorComponentExports(component, ${JSON.stringify(ACTOR_COMPONENT_EXPORTS)});
const bytes = await component.describe.describe();
if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > 8 * 1024 * 1024) throw new Error("Invalid descriptor byte extent");
process.stdout.write(Buffer.from(bytes).toString("base64"));
`;

/** 🔏️ Uses the same native pack self-hash convention for the genuine guest descriptor. */
export function finalizePluginDescriptor(bytes: Uint8Array, pluginId: string, wasmSha256: string, coreWasmSha256: string): { pack: Uint8Array; json: string } {
  const descriptor = decodePackValue(bytes) as unknown as { manifest?: { pluginId?: string }; hashes?: Record<string, string> };
  if (descriptor?.manifest?.pluginId === "assembly-failed") throw new Error("Plugin descriptor assembly failed");
  if (descriptor?.manifest?.pluginId !== pluginId || !descriptor.hashes) throw new Error("Plugin descriptor identity mismatch");
  if (![wasmSha256, coreWasmSha256].every((hash) => /^[a-f0-9]{64}$/.test(hash))) throw new Error("Invalid plugin artifact digest");
  descriptor.hashes.wasmSha256 = wasmSha256;
  descriptor.hashes.coreWasmSha256 = coreWasmSha256;
  descriptor.hashes.descriptorSha256 = "";
  descriptor.hashes.descriptorSha256 = createHash("sha256").update(encodePackValue(descriptor as PackValue)).digest("hex");
  return { pack: encodePackValue(descriptor as PackValue), json: JSON.stringify(packValueToExactJson(descriptor as PackValue), null, 2) + "\n" };
}

type Profile = "dev" | "release";

/** 🎚️ Selects immutable browser output ownership before any producer runs. */
function componentProfile(value: string | undefined): Profile {
  if (value !== "dev" && value !== "release") throw new Error("Select a component profile: dev or release");
  return value;
}

export function browserModuleRoot(profile: Profile): string { return join(SCRIPT_ROOT, "dist", profile, "🔌️plugin-modules"); }

/** 📦️ Enumerates regular staged files without admitting symlinks or unrelated mutable state. */
export function artifactFiles(root: string): Map<string, string> {
  const files = new Map<string, string>();
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isSymbolicLink()) throw new Error(`Symlink in browser artifact: ${path}`);
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) files.set(relative(root, path), path);
      else throw new Error(`Unsupported browser artifact: ${path}`);
    }
  };
  walk(root);
  return files;
}

async function fileDigest(path: string): Promise<string> {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest("hex");
}

class SupportScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1) throw new Error("Usage: support <dev|release>");
    const profile = componentProfile(args[0]), root = browserModuleRoot(profile);
    mkdirSync(root, { recursive: true });
    const temporary = mkdtempSync(join(root, ".support-"));
    try {
      const vendor = join(temporary, "vendor"), shard = join(temporary, "shard");
      ensurePreview2ShimVendorAt(vendor, getWorkspaceRoot());
      mkdirSync(shard);
      writeFileSync(join(shard, SHARD_WORKER_FILE), shardWorkerSource());
      stageArtifacts(join(root, PREVIEW2_VENDOR_RELATIVE), `browser-support:${profile}:preview2`, artifactFiles(vendor));
      stageArtifacts(join(root, MODULE_SHARD_DIRECTORY), `browser-support:${profile}:shard`, artifactFiles(shard));
      console.log(`Browser support ${profile}: vendor shims and shard worker staged`);
    } finally { rmSync(temporary, { recursive: true, force: true }); }
  }
}

class MaterializeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 3 || args[1] !== "--manifest") throw new Error("Usage: materialize <dev|release> --manifest <Cargo.toml>");
    const profile = componentProfile(args[0]), repo = getWorkspaceRoot(), manifestPath = resolve(repo, args[2]!);
    if (!manifestPath.startsWith(repo + sep) || lstatSync(manifestPath).isSymbolicLink()) throw new Error("Component manifest must belong to the workspace");
    const manifest = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(manifestPath, "utf8"));
    const metadata = manifest.package?.metadata, identity = metadata?.component?.package;
    if (typeof identity !== "string" || !/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/.test(identity) || !["plugin", "extension"].includes(metadata.semio?.role)) throw new Error("Expected a Cargo plugin or extension component");
    const pluginId = identity.slice(6), crate = manifest.package.name.replaceAll("-", "_"), componentBase = crate + "_component";
    const artifact = join(dirname(manifestPath), "dist", `component-${profile}`, crate + ".wasm");
    const root = browserModuleRoot(profile), output = join(root, moduleDirectoryName(pluginId)), vendor = join(root, PREVIEW2_VENDOR_RELATIVE);
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
      stageArtifacts(output, relative(repo, manifestPath).split(sep).join("/") + `:browser:${profile}`, artifactFiles(temporary));
      console.log(`Materialized ${pluginId} ${profile}: browser bridge and descriptor staged`);
    } finally {
      process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel);
      rmSync(temporary, { recursive: true, force: true });
    }
  }
}

const router = new ScriptRouter(SCRIPT_ROOT).register("support", SupportScript).register("materialize", MaterializeScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);

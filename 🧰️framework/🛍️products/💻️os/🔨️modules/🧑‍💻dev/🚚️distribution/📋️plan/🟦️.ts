/** 🧩️ Semantic distribution plan owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { fileURLToPath, pathToFileURL } from "node:url";

import { DISTRIBUTION_LAYOUT, distributionOutputOwner, parseDistributionManifest, parseDistributionStaticInputs, type DistributionInput, type DistributionLayout, type DistributionManifest } from "../🟦️.ts";

import { distributionFileWitness, distributionNode, distributionPathOrder, distributionStaticSourcePaths } from "../📥️source/🟦️.ts";

import type { OwnedBuildConfig } from "../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";

import type { InlineConfig } from "vite";



//#endregion 🔖️Sweep
//#endregion 🔬️ParityScript

//#region 📤️DistributionBundle
type DistributionBundlePlan = { readonly manifest: DistributionManifest; readonly files: ReadonlyMap<string, Uint8Array> };

/** 📣️ Reports local progress while keeping the machine-readable preview channel exact. */
function distributionProgress(message: string): void { if (process.env.SEMIO_GENERATOR_PREVIEW !== "1") console.error(`[DEBUG] ${message}`); }

/** 🗺️ Enumerates only the bounded bundle root and refuses unknown children before publication. */
function distributionExistingPaths(layout: DistributionLayout, destination: string, previous: DistributionManifest | null): string[] {
  const expected = new Set(previous?.outputs.map(row => row.path) ?? []), directories = new Set<string>([layout.bundles]), found: string[] = [];
  for (const path of expected) for (let parent = dirname(path); parent !== "."; parent = dirname(parent)) directories.add(parent);
  const root = distributionNode(destination);
  if (root && (!root.isDirectory() || root.isSymbolicLink())) throw new Error("Distribution destination must be a real directory");
  const inspect = (path: string) => {
    const node = distributionNode(join(destination, path));
    if (!node) return;
    if (node.isSymbolicLink()) throw new Error(`Distribution symlink is forbidden: ${path}`);
    if (node.isDirectory()) {
      if (!directories.has(path)) throw new Error(`Unknown distribution child: ${path}`);
      for (const name of readdirSync(join(destination, path))) inspect(`${path}/${name}`);
    } else if (node.isFile() && expected.has(path)) found.push(path);
    else throw new Error(`Unknown distribution child: ${path}`);
  };
  inspect(layout.bundles);
  inspect(layout.entry.output);
  return found.sort(distributionPathOrder);
}

/** 📋️ Validates plan bytes and checks every retained prior-manifest witness before any write. */
async function distributionPreflight(plan: DistributionBundlePlan, layout: DistributionLayout, destination: string): Promise<DistributionManifest | null> {
  const manifest = parseDistributionManifest(plan.manifest, layout);
  if (plan.files.size !== manifest.outputs.length) throw new Error("Distribution plan file inventory differs from its manifest");
  for (const row of manifest.outputs) {
    const bytes = plan.files.get(row.path);
    if (!bytes || bytes.length !== row.bytes || createHash("sha256").update(bytes).digest("hex") !== row.sha256) throw new Error(`Distribution plan byte mismatch: ${row.path}`);
  }
  const manifestPath = join(destination, layout.manifest), manifestNode = distributionNode(manifestPath);
  if (manifestNode && (!manifestNode.isFile() || manifestNode.isSymbolicLink())) throw new Error("Distribution manifest must be a regular file");
  const previous = manifestNode ? parseDistributionManifest(JSON.parse(readFileSync(manifestPath, "utf8")), layout) : null;
  const found = distributionExistingPaths(layout, destination, previous);
  if (previous) {
    if (found.join("\0") !== previous.outputs.map(row => row.path).sort(distributionPathOrder).join("\0")) throw new Error("Prior distribution output inventory is incomplete");
    for (const row of previous.outputs) {
      const actual = await distributionFileWitness(join(destination, row.path), row.path);
      if (actual.bytes !== row.bytes || actual.sha256 !== row.sha256) throw new Error(`Divergent distribution bytes: ${row.path}`);
    }
  }
  return previous;
}

/** 🏗️ Runs the real compiler without copy hooks, capturing main and worker input byte witnesses. */
/** @emoji 🔁 The one crossing between the owned build contract and the build tool that consumes it.
 * `🏗️builder/🌐️vite` states the production configuration in the tool-independent owned shapes of
 * `🛠️build-tooling` — readonly arrays, structural server and plugin hooks — which is exactly why the
 * tool's own mutable declarations cannot be reached by assignment. The two descriptions cover the
 * same runtime objects, and this is the single place a distribution build hands one to the other. */
function buildToolConfig(config: OwnedBuildConfig): InlineConfig {
  return config as unknown as InlineConfig;
}

async function renderDistributionBundle(workspace: string, artifactRoot: string): Promise<DistributionBundlePlan> {
  const scriptDirectory = dirname(fileURLToPath(import.meta.url)), playDirectory = resolve(scriptDirectory, "../.."), authorityDirectory = join(playDirectory, "🚚️distribution");
  const authority = parseDistributionStaticInputs(JSON.parse(readFileSync(join(authorityDirectory, "🔗️inputs.json"), "utf8")));
  const witnesses = new Map<string, DistributionInput>(), pending = new Map<string, Promise<void>>();
  const observe = (id: string): Promise<void> => {
    if (id.includes("\0")) return Promise.resolve();
    const absolute = resolve(playDirectory, id.split(/[?#]/u)[0]!);
    if (!existsSync(absolute) || !statSync(absolute).isFile()) return Promise.resolve();
    const canonical = realpathSync(absolute), path = relative(realpathSync(workspace), canonical).replaceAll("\\", "/").normalize("NFC");
    if (path === ".." || path.startsWith("../")) throw new Error(`Compiler input escapes workspace: ${id}`);
    if (path.startsWith(relative(workspace, join(playDirectory, DISTRIBUTION_LAYOUT.directory)).replaceAll("\\", "/") + "/")) throw new Error(`Compiler input consumes its own output: ${path}`);
    const current = pending.get(path);
    if (current) return current;
    const operation = distributionFileWitness(canonical, path).then(row => { witnesses.set(path, row); if (witnesses.size % 500 === 0) distributionProgress(`recorded ${witnesses.size} compiler input witnesses`); });
    pending.set(path, operation);
    return operation;
  };
  for (const path of [...new Set([...authority.paths, ...await distributionStaticSourcePaths(workspace, authority.moduleEntries)])]) {
    if (path.startsWith("/") || path.includes("\\") || path.split("/").some(part => !part || part === "." || part === "..") || !existsSync(join(workspace, path))) throw new Error(`Missing or unsafe distribution static input: ${path}`);
    await observe(join(workspace, path));
  }
  for (const row of [...DISTRIBUTION_LAYOUT.chunks, ...DISTRIBUTION_LAYOUT.assets]) if (row.kind !== "name") await observe(join(workspace, row.source));
  await observe(join(workspace, DISTRIBUTION_LAYOUT.entry.source));
  const collector = () => ({
    name: "semio-distribution-input-witness", enforce: "pre" as const,
    async load(id: string) { await observe(id); return null; },
    async buildEnd(this: { getWatchFiles(): string[] }) {
      const files = this.getWatchFiles();
      distributionProgress(`collecting ${files.length} compiler watch inputs`);
      for (const path of files) await observe(path);
      distributionProgress("compiler watch inputs collected");
    },
  });
  const original = { SEMIO_PLUGIN: process.env.SEMIO_PLUGIN, SEMIO_RENDERER: process.env.SEMIO_RENDERER, SEMIO_BRAND: process.env.SEMIO_BRAND };
  process.env.SEMIO_PLUGIN = "s";
  process.env.SEMIO_RENDERER = "react";
  process.env.SEMIO_BRAND = "";
  let result: Awaited<ReturnType<typeof import("vite")["build"]>>;
  try {
    const { build } = await import("vite"), { default: createConfig } = await import("../../🏗️builder/🌐️vite/🟦️.ts");
    const config = buildToolConfig(await createConfig({ command: "build", mode: "production", isSsrBuild: false, isPreview: false }));
    distributionProgress("actual production configuration loaded");
    const workerPlugins = config.worker?.plugins;
    result = await build({ ...config, configFile: false, publicDir: false, logLevel: "silent", cacheDir: join(artifactRoot, "vite-distribution-cache"), plugins: [collector(), ...(config.plugins ?? [])], worker: { ...config.worker, plugins: () => [collector(), ...(workerPlugins?.() ?? [])] }, build: { ...config.build, write: false, emptyOutDir: false, outDir: join(artifactRoot, "distribution-unwritten") } });
  } finally {
    for (const [name, value] of Object.entries(original)) if (value === undefined) delete process.env[name]; else process.env[name] = value;
  }
  const files = new Map<string, Uint8Array>();
  distributionProgress("compiler returned output bytes");
  for (const bundle of Array.isArray(result) ? result : [result]) {
    if (!("output" in bundle)) throw new Error("Distribution generation cannot use a watch-mode compiler");
    for (const item of bundle.output) {
      if (files.has(item.fileName)) throw new Error(`Duplicate compiler output: ${item.fileName}`);
      distributionOutputOwner(DISTRIBUTION_LAYOUT, item.fileName);
      files.set(item.fileName, Buffer.from(item.type === "chunk" ? item.code : item.source));
    }
  }
  distributionProgress(`rechecking ${witnesses.size} compiler input witnesses`);
  for (const row of witnesses.values()) {
    const current = await distributionFileWitness(join(workspace, row.path), row.path);
    if (current.bytes !== row.bytes || current.sha256 !== row.sha256) throw new Error(`Compiler input changed during production build: ${row.path}`);
  }
  const manifest = parseDistributionManifest({
    version: 1, contractId: "dev-distribution-bundle", layoutSha256: createHash("sha256").update(readFileSync(join(authorityDirectory, "📇️layout.json"))).digest("hex"),
    inputs: [...witnesses.values()].sort((left, right) => distributionPathOrder(left.path, right.path)),
    outputs: [...files].map(([path, bytes]) => ({ ownerId: distributionOutputOwner(DISTRIBUTION_LAYOUT, path), path, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") })).sort((left, right) => distributionPathOrder(left.path, right.path)),
  }, DISTRIBUTION_LAYOUT);
  return { manifest, files };
}

/** 🧾️ Emits the bounded generator protocol; copied static namespaces are separate owners. */
function distributionPreviewProtocol(plan: DistributionBundlePlan, workspace: string, destination: string) {
  const rootPath = relative(workspace, destination).replaceAll("\\", "/"), nodes: { path: string; nodeKind: "directory" | "file"; mode: number; bytesBase64: string }[] = [];
  const files = new Map(plan.files), directories = new Set<string>([`${rootPath}/${DISTRIBUTION_LAYOUT.bundles}`]);
  files.set(DISTRIBUTION_LAYOUT.manifest, Buffer.from(JSON.stringify(plan.manifest, null, 2) + "\n"));
  for (const [path, bytes] of files) {
    const absolute = `${rootPath}/${path}`;
    nodes.push({ path: absolute, nodeKind: "file", mode: 0o644, bytesBase64: Buffer.from(bytes).toString("base64") });
    for (let parent = dirname(absolute); parent !== rootPath; parent = dirname(parent)) directories.add(parent);
  }
  for (const path of directories) nodes.push({ path, nodeKind: "directory", mode: 0o755, bytesBase64: "" });
  nodes.sort((left, right) => distributionPathOrder(left.path, right.path));
  return { contractId: "dev-distribution-bundle", schemaVersion: 1, nodes, staleRemovals: [] as string[] };
}

export { type DistributionBundlePlan, distributionExistingPaths, distributionPreflight, distributionPreviewProtocol, distributionProgress, renderDistributionBundle };

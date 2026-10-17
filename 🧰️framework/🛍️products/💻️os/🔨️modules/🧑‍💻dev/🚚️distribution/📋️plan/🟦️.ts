import {runtimeCatalogBuildInventoryV1} from "../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { runtimeFixturePathV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🟦️.ts";
import { requirePlaygroundVariant } from "../../../🔌️plugin/📇️registry/🎮️playground/⭐️default/🟦️.ts";

import { loadDevContribution, resolveDevContributionFile } from "../../🧩️contribution/📥️loading/🟦️.ts";
/** 🧩️ Semantic distribution plan owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { fileURLToPath, pathToFileURL } from "node:url";

import { DISTRIBUTION_LAYOUT, distributionOutputOwner, parseDistributionManifest, parseDistributionStaticInputs, type DistributionInput, type DistributionLayout, type DistributionManifest } from "../🟦️.ts";

import { distributionFileWitness, distributionNode, distributionPathOrder, distributionStaticSourcePaths } from "../📥️source/🟦️.ts";

import type { OwnedBuildConfig } from "@semio-tech/ui-react/build-tooling";

import type { InlineConfig } from "vite";



//#endregion 🔖️Sweep
//#endregion 🔬️ParityScript

//#region 📤️DistributionBundle
type DistributionBundlePlan = { readonly manifest: DistributionManifest; readonly files: ReadonlyMap<string, Uint8Array> };

/** 📣️ Reports local progress while keeping the machine-readable preview channel exact. */
function distributionProgress(message: string): void { if (process.env.SEMIO_GENERATOR_PREVIEW !== "1") console.error(`[TRACE] ${message}`); }

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

export { type DistributionBundlePlan, distributionExistingPaths, distributionPreflight, distributionPreviewProtocol, distributionProgress };

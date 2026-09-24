/** 🧩️ Semantic distribution publication owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { DISTRIBUTION_LAYOUT, distributionOutputOwner, parseDistributionManifest, parseDistributionStaticInputs, type DistributionInput, type DistributionLayout, type DistributionManifest } from "../🟦️.ts";

import { DistributionBundlePlan, distributionPreflight } from "../📋️plan/🟦️.ts";

import { distributionFileWitness, distributionNode, distributionPathOrder, distributionRealAncestors } from "../📥️source/🟦️.ts";



/** 📤️ Stages verified bytes and retains exact previous bytes before narrowly retiring owned leaves. */
async function publishDistributionBundle(plan: DistributionBundlePlan, layout: DistributionLayout, destination: string, artifactRoot: string): Promise<{ retired: string[]; recovery: string }> {
  const previous = await distributionPreflight(plan, layout, destination);
  const stage = mkdtempSync(join(artifactRoot, "distribution-publication-")), nextRoot = join(stage, "📤️next"), previousRoot = join(stage, "📥️previous");
  const previousManifestBytes = previous ? readFileSync(join(destination, layout.manifest)) : null;
  const nextFiles = new Map(plan.files);
  nextFiles.set(layout.manifest, Buffer.from(JSON.stringify(plan.manifest, null, 2) + "\n"));
  for (const [path, bytes] of nextFiles) {
    mkdirSync(dirname(join(nextRoot, path)), { recursive: true });
    writeFileSync(join(nextRoot, path), bytes, { flag: "wx", mode: 0o644 });
    const staged = await distributionFileWitness(join(nextRoot, path), path);
    if (staged.bytes !== bytes.length || staged.sha256 !== createHash("sha256").update(bytes).digest("hex")) throw new Error(`Staged distribution bytes differ: ${path}`);
  }
  for (const row of previous?.outputs ?? []) {
    mkdirSync(dirname(join(previousRoot, row.path)), { recursive: true });
    copyFileSync(join(destination, row.path), join(previousRoot, row.path), fsConstants.COPYFILE_EXCL);
    const retained = await distributionFileWitness(join(previousRoot, row.path), row.path);
    if (retained.bytes !== row.bytes || retained.sha256 !== row.sha256) throw new Error(`Retained distribution bytes differ: ${row.path}`);
  }
  if (previousManifestBytes) {
    mkdirSync(previousRoot, { recursive: true });
    writeFileSync(join(previousRoot, layout.manifest), previousManifestBytes, { flag: "wx" });
  }
  await distributionPreflight(plan, layout, destination);
  if (previousManifestBytes && !readFileSync(join(destination, layout.manifest)).equals(previousManifestBytes)) throw new Error("Distribution manifest changed during staging");
  const publicationOrder = [...nextFiles.keys()].sort((left, right) => Number(left === layout.manifest) - Number(right === layout.manifest) || Number(left === layout.entry.output) - Number(right === layout.entry.output) || distributionPathOrder(left, right));
  mkdirSync(destination, { recursive: true });
  for (const path of publicationOrder) {
    distributionRealAncestors(destination, path);
    const target = join(destination, path), node = distributionNode(target);
    if (node) {
      if (!node.isFile() || node.isSymbolicLink()) throw new Error(`Distribution publication target changed kind: ${path}`);
      const bytes = readFileSync(target), old = previous?.outputs.find(row => row.path === path);
      const unchanged = path === layout.manifest ? previousManifestBytes !== null && bytes.equals(previousManifestBytes) : old !== undefined && bytes.length === old.bytes && createHash("sha256").update(bytes).digest("hex") === old.sha256;
      if (!unchanged) throw new Error(`Distribution publication target changed during staging: ${path}`);
      if (bytes.equals(Buffer.from(nextFiles.get(path)!))) continue;
    }
    mkdirSync(dirname(target), { recursive: true });
    if (!node) copyFileSync(join(nextRoot, path), target, fsConstants.COPYFILE_EXCL);
    else renameSync(join(nextRoot, path), target);
  }
  const retired = (previous?.outputs ?? []).map(row => row.path).filter(path => !plan.files.has(path)).sort(distributionPathOrder);
  for (const path of retired) {
    distributionRealAncestors(destination, path);
    const row = previous!.outputs.find(item => item.path === path)!;
    const witness = await distributionFileWitness(join(destination, path), path);
    if (witness.bytes !== row.bytes || witness.sha256 !== row.sha256 || distributionNode(join(destination, path))?.isSymbolicLink()) throw new Error(`Distribution retirement witness changed: ${path}`);
    unlinkSync(join(destination, path));
  }
  const oldDirectories = new Set<string>();
  for (const path of retired) for (let parent = dirname(path); parent !== "." && parent !== layout.bundles; parent = dirname(parent)) oldDirectories.add(parent);
  for (const path of [...oldDirectories].sort((left, right) => right.split("/").length - left.split("/").length)) if (readdirSync(join(destination, path)).length === 0) rmdirSync(join(destination, path));
  writeFileSync(join(stage, "🧾️publication.json"), JSON.stringify({ retired, destination, recovery: previousRoot }, null, 2) + "\n", { flag: "wx" });
  return { retired, recovery: stage };
}

export { publishDistributionBundle };

/** 🧩️ Semantic distribution freshness owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { DISTRIBUTION_LAYOUT, distributionOutputOwner, parseDistributionManifest, parseDistributionStaticInputs, type DistributionInput, type DistributionLayout, type DistributionManifest } from "../🟦️.ts";

import { DistributionBundlePlan, distributionPreflight } from "../📋️plan/🟦️.ts";

import { distributionPathOrder } from "../📥️source/🟦️.ts";



/** ✅️ Checks exactly owned outputs while leaving separately owned static roots untouched. */
async function checkDistributionBundle(plan: DistributionBundlePlan, layout: DistributionLayout, destination: string): Promise<string[]> {
  const previous = await distributionPreflight(plan, layout, destination);
  if (!previous) return [layout.manifest, ...plan.manifest.outputs.map(row => row.path)].sort(distributionPathOrder);
  const expected = new Map(plan.manifest.outputs.map(row => [row.path, row]));
  const stale = new Set<string>();
  if (readFileSync(join(destination, layout.manifest), "utf8") !== JSON.stringify(plan.manifest, null, 2) + "\n") stale.add(layout.manifest);
  for (const row of previous.outputs) if (expected.get(row.path)?.sha256 !== row.sha256 || expected.get(row.path)?.bytes !== row.bytes) stale.add(row.path);
  for (const row of plan.manifest.outputs) if (!previous.outputs.some(old => old.path === row.path && old.sha256 === row.sha256 && old.bytes === row.bytes)) stale.add(row.path);
  return [...stale].sort(distributionPathOrder);
}

export { checkDistributionBundle };

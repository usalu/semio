/** 🧩️ Semantic distribution compiler owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { DISTRIBUTION_LAYOUT, distributionOutputOwner, parseDistributionManifest, parseDistributionStaticInputs, type DistributionInput, type DistributionLayout, type DistributionManifest } from "../🟦️.ts";

import { renderDistributionBundle } from "../📋️plan/🟦️.ts";



/** 🧵️ Compiles after this router module has finished loading, outside its configuration import cycle. */
export async function materializeDistributionBundle(workspace: string, artifactDirectory: string): Promise<void> {
  const plan = await renderDistributionBundle(workspace, artifactDirectory);
  for (const [path, bytes] of plan.files) {
    mkdirSync(dirname(join(artifactDirectory, path)), { recursive: true });
    writeFileSync(join(artifactDirectory, path), bytes, { flag: "wx", mode: 0o644 });
  }
  writeFileSync(join(artifactDirectory, DISTRIBUTION_LAYOUT.manifest), JSON.stringify(plan.manifest, null, 2) + "\n", { flag: "wx", mode: 0o644 });
}

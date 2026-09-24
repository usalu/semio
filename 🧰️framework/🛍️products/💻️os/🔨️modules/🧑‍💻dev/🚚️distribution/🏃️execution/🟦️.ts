/** 🧩️ Semantic distribution execution owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { tmpdir } from "node:os";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { DISTRIBUTION_LAYOUT, distributionOutputOwner, parseDistributionManifest, parseDistributionStaticInputs, type DistributionInput, type DistributionLayout, type DistributionManifest } from "../🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { distributionPreflight, distributionPreviewProtocol, distributionProgress } from "../📋️plan/🟦️.ts";

import { publishDistributionBundle } from "../📤️publication/🟦️.ts";

import { distributionPathOrder } from "../📥️source/🟦️.ts";

import { checkDistributionBundle } from "../🔍️freshness/🟦️.ts";



/** 🚚️ Publishes, previews or byte-checks only this distribution's compiler-owned partition. */
class DistributionBundleScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const command = segments[0];
    if (segments.length !== 1 || !["generate", "preview", "check"].includes(command ?? "")) throw new Error("distribution expects exactly generate, preview or check");
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(process.env.SEMIO_TEST_ARTIFACT_DIR) : mkdtempSync(join(tmpdir(), "semio-distribution-"));
    mkdirSync(artifactRoot, { recursive: true });
    const destination = resolve(import.meta.dir, "../..", DISTRIBUTION_LAYOUT.directory);
    distributionProgress("compiling bounded distribution without static copies");
    const compiled = mkdtempSync(join(artifactRoot, "distribution-compiled-"));
    runCmd("bun", ["--eval", "const api = await import(process.argv[1]); await api.materializeDistributionBundle(process.argv[2], process.argv[3]);", new URL("../🏗️compiler/🟦️.ts", import.meta.url).href, this.repoRoot, compiled], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
    const manifest = parseDistributionManifest(JSON.parse(readFileSync(join(compiled, DISTRIBUTION_LAYOUT.manifest), "utf8")), DISTRIBUTION_LAYOUT);
    const plan = { manifest, files: new Map(manifest.outputs.map(row => [row.path, readFileSync(join(compiled, row.path))])) };
    distributionProgress(`witnessed ${plan.manifest.inputs.length} compiler inputs and ${plan.manifest.outputs.length} outputs`);
    if (command === "check") {
      const findings = await checkDistributionBundle(plan, DISTRIBUTION_LAYOUT, destination);
      if (findings.length) throw new Error(`Distribution outputs are stale: ${findings.join(", ")}`);
      console.log("distribution check: fresh");
    } else if (command === "preview") {
      const previous = await distributionPreflight(plan, DISTRIBUTION_LAYOUT, destination), protocol = distributionPreviewProtocol(plan, this.repoRoot, destination);
      protocol.staleRemovals = (previous?.outputs ?? []).filter(row => !plan.files.has(row.path)).map(row => relative(this.repoRoot, join(destination, row.path)).replaceAll("\\", "/")).sort(distributionPathOrder);
      process.stdout.write(JSON.stringify(protocol) + "\n");
    } else {
      const result = await publishDistributionBundle(plan, DISTRIBUTION_LAYOUT, destination, artifactRoot);
      console.log(JSON.stringify({ outputs: plan.manifest.outputs.length, retired: result.retired, recovery: result.recovery }));
    }
  }
}

export { DistributionBundleScript };

/** 🧩️ Semantic scale fixture publication owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

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
} from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { checkScaleFixtureArtifacts, renderScaleFixtureArtifacts, scaleFixtureFlag, scaleFixtureGeneratedDir, writeScaleFixtureArtifacts } from "../📽️projection/🟦️.ts";



class ScaleFixtureGenerateScript extends BundleScript {
  run(segments: string[]): void {
    const pluginCount = scaleFixtureFlag(segments, "plugins", 50);
    const extensionsPerPlugin = scaleFixtureFlag(segments, "extensions", 50);
    const seed = scaleFixtureFlag(segments, "seed", 1);
    writeScaleFixtureArtifacts(this.repoRoot, pluginCount, extensionsPerPlugin, seed);
  }
}

/** 🧾️ Emits the canonical scale-fixture output protocol from the seeded in-memory renderer. */
class ScaleFixturePreviewGeneratedScript extends BundleScript {
  run(segments: string[]): void {
    const pluginCount = scaleFixtureFlag(segments, "plugins", 50);
    const extensionsPerPlugin = scaleFixtureFlag(segments, "extensions", 50);
    const seed = scaleFixtureFlag(segments, "seed", 1);
    const dir = scaleFixtureGeneratedDir(this.repoRoot);
    const rootPath = relative(this.repoRoot, dir).replaceAll("\\", "/").normalize("NFC");
    const { registryJson, catalogJson } = renderScaleFixtureArtifacts(pluginCount, extensionsPerPlugin, seed);
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: `${rootPath}/📇️registry` },
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: `${rootPath}/🗂️catalog` },
      { bytesBase64: Buffer.from(catalogJson).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/🗂️catalog/🔣️.json` },
      { bytesBase64: Buffer.from(registryJson).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/📇️registry/🔣️.json` },
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const expected = new Set(["📇️registry", "🗂️catalog"]);
    const staleRemovals = (existsSync(dir) ? readdirSync(dir) : []).filter((name) => !expected.has(name)).map((name) => `${rootPath}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
    process.stdout.write(`${JSON.stringify({ contractId: "scale-fixture", nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}

class ScaleFixtureCheckScript extends BundleScript {
  run(): void {
    if (!checkScaleFixtureArtifacts(this.repoRoot)) {
      throw new Error("scale-fixture check: 🤖️generated/{📇️registry,🗂️catalog}/🔣️.json are stale — run `bun ./📜️script.ts generate scale-fixture`");
    }
    console.log("scale-fixture check: fresh");
  }
}

export { ScaleFixtureCheckScript, ScaleFixtureGenerateScript, ScaleFixturePreviewGeneratedScript };

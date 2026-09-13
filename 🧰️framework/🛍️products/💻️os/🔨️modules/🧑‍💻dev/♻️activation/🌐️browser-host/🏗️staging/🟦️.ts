/** 🧩️ Semantic browser host staging owner. */

import { ACTOR_COMPONENT_EXPORTS, assertActorComponentExports, finalizePluginDescriptor, PLUGIN_DESCRIPTOR_PROBE_SOURCE } from "../../../../🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";

import { artifactFiles } from "../../../../🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";

import { closeTestBrowserHostStagingV1, parseTestBrowserGisMaterializationReceiptV1, parseTestBrowserHostStagingReceiptV1, prepareTestBrowserHostRootsV1, resolveTestBrowserHostRootsV1, TEST_BROWSER_ACTIVATION_ROOT_ENV, TEST_BROWSER_HOST_RECEIPT_ENV, TEST_BROWSER_MODULE_ROOT_ENV, type TestBrowserHostRootsV1, writeTestBrowserGisMaterializationReceiptV1 } from "../🟦️.ts";

import { stageArtifacts } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";

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
} from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { decodeDocumentPackBytes, decodePackValue, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, encodeDocumentArchiveBytes, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

import { PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS } from "../../../../🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";

import {
  ensurePreview2ShimVendorAt,
  hostShimSource,
  PLUGIN_HOST_SHIM_FILE,
  PREVIEW2_VENDOR_RELATIVE,
  GUESTSLIM_FONT_RELATIVE,
  pluginComponentBridgeSource,
  rewriteJcoComponentAssetUrls,
  SHARD_WORKER_FILE,
  shardWorkerSource,
  rewriteJcoAsyncResultLifting,
  rewritePreview2ShimImportSource,
  rewritePreview2ShimImports,
  transpilePluginComponentAsync,
  type PluginWebMaterializeContext,
} from "../../../../🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { ensureWasmTarget, pluginWasmProfile } from "../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { buildPluginCargo } from "../../../../🔌️plugin/🏗️build/📦️materialization/🟦️.ts";

import { pluginFileDigest } from "../../../../🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts";

import { ensureAppleDeveloperDir } from "../../../⚙️engine/📤️publication/🟦️.ts";



export type TestBrowserHostStageInputV1 = Readonly<{
  artifactRoot: string;
  selectedGis: Readonly<{
    generationId: string;
    currentSha256: string;
    componentPath: string;
    componentSha256: string;
    descriptorPath: string;
    descriptorSha256: string;
  }>;
}>;

function ownedTestBrowserHostInput(artifactRoot: string, path: string, maximum: number, label: string): Readonly<{ path: string; size: number }> {
  if (!isAbsolute(path)) throw new Error(`${label} must be absolute`);
  const sourceInfo = lstatSync(path), canonical = realpathSync(path), child = relative(artifactRoot, canonical), info = lstatSync(canonical);
  if (!child || child.startsWith("..") || isAbsolute(child) || sourceInfo.isSymbolicLink() || info.isSymbolicLink() || !info.isFile() || info.size < 1 || info.size > maximum) throw new Error(`${label} is not an owned bounded regular file`);
  return Object.freeze({ path: canonical, size: info.size });
}

function exactSpaceCreateArtifactArgs(value: unknown): boolean {
  const pending: Array<Readonly<{ value: unknown; depth: number }>> = [{ value, depth: 0 }];
  const matches: string[][] = [];
  let visited = 0;
  while (pending.length) {
    const current = pending.pop()!;
    if (++visited > 32_768 || current.depth > 64) throw new Error("Space descriptor structure exceeds its inspection bound");
    if (Array.isArray(current.value)) {
      for (const child of current.value) pending.push({ value: child, depth: current.depth + 1 });
      continue;
    }
    if (!current.value || typeof current.value !== "object") continue;
    const row = current.value as Record<string, unknown>;
    if (row.id === "createArtifact" && Array.isArray(row.args)) {
      const ids = row.args.map((arg) => (arg && typeof arg === "object" && !Array.isArray(arg) ? (arg as Record<string, unknown>).id : undefined));
      if (ids.some((id) => typeof id !== "string")) return false;
      matches.push(ids as string[]);
    }
    for (const child of Object.values(row)) pending.push({ value: child, depth: current.depth + 1 });
  }
  return matches.length === 1 && JSON.stringify(matches[0]) === JSON.stringify(["name", "kindChoice"]);
}

async function materializeTestBrowserPluginV1(input: Readonly<{
  target: PluginRegistryEntry;
  artifact: string;
  moduleRoot: string;
  descriptorPath?: string;
  selectedSource?: Readonly<{ componentSha256: string; descriptorSha256: string }>;
}>): Promise<void> {
  if ((input.descriptorPath === undefined) !== (input.selectedSource === undefined)) throw new Error("Selected browser plugin materialization identity is incomplete");
  const outDir = join(input.moduleRoot, moduleDirectoryName(input.target.pluginId));
  mkdirSync(outDir, { recursive: true, mode: 0o700 });
  const componentBase = `${input.target.wasmOut.replace(/\.wasm$/u, "")}_component`;
  writeFileSync(join(outDir, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  await transpilePluginComponentAsync(input.artifact, outDir, componentBase, { repoRoot, preview2VendorDir: join(input.moduleRoot, PREVIEW2_VENDOR_RELATIVE), optimize: pluginWasmProfile() === "wasm-release", wasmOptBin: join(repoRoot, "node_modules/binaryen/bin/wasm-opt") });
  if (input.descriptorPath) {
    copyFileSync(input.descriptorPath, join(outDir, "🛂️.descriptor.semio"));
  } else {
    const probe = runProbe("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(outDir, `${componentBase}.js`)], { cwd: repoRoot, budgetMs: 60_000 });
    if (probe.status !== 0) throw new Error(`Test browser host descriptor failed for ${input.target.pluginId}: ${probe.stderr}`);
    const base64 = probe.stdout.trim();
    if (!/^[A-Za-z0-9+/]+={0,2}$/u.test(base64)) throw new Error(`Invalid test browser host descriptor for ${input.target.pluginId}`);
    const descriptor = finalizePluginDescriptor(Buffer.from(base64, "base64"), input.target.pluginId, await pluginFileDigest(input.artifact), await pluginFileDigest(join(outDir, `${componentBase}.core.wasm`)));
    writeFileSync(join(outDir, "🛂️.descriptor.semio"), descriptor.pack);
    writeFileSync(join(outDir, "🔣️.json"), descriptor.json);
    if (input.target.pluginId === "space" && !exactSpaceCreateArtifactArgs(JSON.parse(descriptor.json))) throw new Error("Fresh Space host descriptor does not expose exact name/kindChoice creation args");
  }
  writeFileSync(join(outDir, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(componentBase, input.target.wasmOut));
  if (input.selectedSource) writeTestBrowserGisMaterializationReceiptV1(outDir, input.selectedSource);
}

/** 🧊️ Builds and atomically closes the exact test-only Space host over one retained selected GIS current. */
export async function stageTestBrowserHostV1(input: TestBrowserHostStageInputV1): Promise<TestBrowserHostRootsV1> {
  if (!isAbsolute(input.artifactRoot)) throw new Error("Test browser host requires an absolute ticket artifact root");
  const artifactRoot = realpathSync(input.artifactRoot);
  if (!artifactRoot.split(/[\\/]/u).includes("🗑️generated")) throw new Error("Test browser host requires a ticket-generated owner");
  const component = ownedTestBrowserHostInput(artifactRoot, input.selectedGis.componentPath, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "Selected GIS component");
  const descriptor = ownedTestBrowserHostInput(artifactRoot, input.selectedGis.descriptorPath, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "Selected GIS descriptor");
  if (![input.selectedGis.generationId, input.selectedGis.currentSha256, input.selectedGis.componentSha256, input.selectedGis.descriptorSha256].every((value) => /^[0-9a-f]{64}$/u.test(value))) throw new Error("Selected GIS identity is invalid");
  if (await pluginFileDigest(component.path) !== input.selectedGis.componentSha256 || await pluginFileDigest(descriptor.path) !== input.selectedGis.descriptorSha256) throw new Error("Selected GIS bytes differ from their retained current");
  const entries = readGeneratedCatalogProjection().entries;
  const space = entries.find((entry) => entry.pluginId === "space" && entry.role === "plugin");
  const gis = entries.find((entry) => entry.pluginId === "gis" && entry.role === "plugin");
  if (!space || !gis || PLUGIN_BUILD_TARGETS.filter((entry) => ["space", "gis"].includes(entry.pluginId)).length !== 2) throw new Error("Test browser host registry selection is not exact");
  ensureAppleDeveloperDir();
  ensureWasmTarget();
  const builtSpace = await buildPluginCargo(space);
  // 🔒️ `buildPluginCargo` now always writes into the ONE shared `cargoTargetDirectory` (fine-grain-locked,
  // no private target dirs); this ticket-owned copy is what makes the artifact eligible for
  // `ownedTestBrowserHostInput`'s bounded-regular-file-inside-`artifactRoot` check below.
  const stagedSpacePath = join(artifactRoot, "browser-host-wasi-target", basename(builtSpace.artifact));
  mkdirSync(dirname(stagedSpacePath), { recursive: true, mode: 0o700 });
  writeFileSync(stagedSpacePath, readFileSync(builtSpace.artifact), { mode: 0o600 });
  const spaceComponent = ownedTestBrowserHostInput(artifactRoot, stagedSpacePath, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "Fresh Space component");
  const spaceComponentSha256 = await pluginFileDigest(spaceComponent.path);
  const stagingOwner = mkdtempSync(join(artifactRoot, ".browser-host-build-"));
  try {
    const stagingArtifactRoot = join(stagingOwner, "🗑️generated");
    mkdirSync(stagingArtifactRoot, { mode: 0o700 });
    const roots = prepareTestBrowserHostRootsV1(stagingArtifactRoot);
    ensurePreview2ShimVendorAt(join(roots.moduleRoot, PREVIEW2_VENDOR_RELATIVE), repoRoot);
    const shardRoot = join(roots.moduleRoot, MODULE_SHARD_DIRECTORY);
    mkdirSync(shardRoot, { recursive: true, mode: 0o700 });
    writeFileSync(join(shardRoot, SHARD_WORKER_FILE), shardWorkerSource());
    await materializeTestBrowserPluginV1({ target: space, artifact: spaceComponent.path, moduleRoot: roots.moduleRoot });
    await materializeTestBrowserPluginV1({ target: gis, artifact: component.path, descriptorPath: descriptor.path, moduleRoot: roots.moduleRoot, selectedSource: { componentSha256: input.selectedGis.componentSha256, descriptorSha256: input.selectedGis.descriptorSha256 } });
    if (lstatSync(spaceComponent.path).size !== spaceComponent.size || await pluginFileDigest(spaceComponent.path) !== spaceComponentSha256) throw new Error("Fresh Space component changed during browser staging");
    if (await pluginFileDigest(component.path) !== input.selectedGis.componentSha256 || await pluginFileDigest(descriptor.path) !== input.selectedGis.descriptorSha256) throw new Error("Selected GIS bytes changed during browser staging");
    writeFileSync(join(roots.browserHostRoot, "extensions", ".nx-artifact.json"), `${JSON.stringify({ owner: "test-browser-host:extensions", version: 1 })}\n`);
    const closed = closeTestBrowserHostStagingV1(stagingArtifactRoot, { generationId: input.selectedGis.generationId, currentSha256: input.selectedGis.currentSha256, componentSha256: input.selectedGis.componentSha256, descriptorSha256: input.selectedGis.descriptorSha256 }, { byteLength: spaceComponent.size, sha256: spaceComponentSha256 });
    await stageArtifacts(join(artifactRoot, "browser-host"), "test-browser-host:s:dev", artifactFiles(closed.browserHostRoot));
    const finalRoot = join(artifactRoot, "browser-host");
    return resolveTestBrowserHostRootsV1({
      SEMIO_TEST_ARTIFACT_DIR: artifactRoot,
      [TEST_BROWSER_MODULE_ROOT_ENV]: join(finalRoot, "modules"),
      [TEST_BROWSER_ACTIVATION_ROOT_ENV]: join(finalRoot, "activation"),
      [TEST_BROWSER_HOST_RECEIPT_ENV]: join(finalRoot, "🔣️receipt.json"),
    })!;
  } finally {
    rmSync(stagingOwner, { recursive: true, force: true });
  }
}

export { exactSpaceCreateArtifactArgs, materializeTestBrowserPluginV1, ownedTestBrowserHostInput };

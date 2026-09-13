/** 🧩️ Semantic plugin size owner. */

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

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../📇️registry/📦️deployment/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { pluginOutRoot } from "../🏗️build/📋️plan/🟦️.ts";



//#region 🪶️PluginSizeMeasurement
/** @emoji 📏️ One built plugin's jco-extracted core wasm module, section-walked byte-for-byte — no
 * external tooling (`wasm-tools`/`twiggy`) required, so this runs anywhere `bun` runs. */
type PluginWasmSizeBreakdown = {
  readonly totalBytes: number;
  readonly codeBytes: number;
  readonly dataBytes: number;
  readonly nameBytes: number;
  readonly otherCustomBytes: number;
  readonly functionCount: number;
  readonly memoryInitialPages: number | null;
  readonly memoryMaxPages: number | null;
};

type PluginWasmSizeRow = PluginWasmSizeBreakdown & { readonly pluginId: string; readonly file: string };

/** @emoji 🔢️ Reads one unsigned LEB128 varint starting at `offset`; returns the decoded value and the
 * offset just past it. Values here (section sizes, function/page counts) never approach 2^53, so a
 * bigint accumulator collapsed to `Number` is safe and simpler than juggling two code paths. */
function readULEB128(buf: Buffer, offset: number): { readonly value: number; readonly next: number } {
  let result = 0n;
  let shift = 0n;
  let pos = offset;
  for (;;) {
    const byte = buf[pos]!;
    pos += 1;
    result |= BigInt(byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) break;
    shift += 7n;
  }
  return { value: Number(result), next: pos };
}

/** @emoji 📏️ Byte-level breakdown of one core wasm module's sections. Section ids per the wasm binary
 * format: 0=custom (name-prefixed — the "name" custom section is pure debug/dev-tooling weight, see
 * `[profile.wasm-release]`'s `strip = "symbols"`), 5=memory, 10=code, 11=data. Reports only the first
 * declared memory's limits — every plugin here declares exactly one. */
function analyzePluginWasmModule(filePath: string): PluginWasmSizeBreakdown {
  const buf = readFileSync(filePath);
  const totalBytes = buf.byteLength;
  if (buf.length < 8 || buf.readUInt32LE(0) !== 0x6d736100) {
    throw new Error(`not a wasm module: ${filePath}`);
  }
  let offset = 8;
  let codeBytes = 0;
  let dataBytes = 0;
  let nameBytes = 0;
  let otherCustomBytes = 0;
  let functionCount = 0;
  let memoryInitialPages: number | null = null;
  let memoryMaxPages: number | null = null;
  while (offset < buf.length) {
    const sectionId = buf[offset]!;
    offset += 1;
    const sectionSizeRead = readULEB128(buf, offset);
    const sectionSize = sectionSizeRead.value;
    const sectionStart = sectionSizeRead.next;
    const sectionEnd = sectionStart + sectionSize;
    if (sectionId === 10) {
      codeBytes += sectionSize;
      functionCount += readULEB128(buf, sectionStart).value;
    } else if (sectionId === 11) {
      dataBytes += sectionSize;
    } else if (sectionId === 5) {
      const countRead = readULEB128(buf, sectionStart);
      if (countRead.value > 0) {
        const flagsRead = readULEB128(buf, countRead.next);
        const hasMax = (flagsRead.value & 0x01) !== 0;
        const minRead = readULEB128(buf, flagsRead.next);
        memoryInitialPages = minRead.value;
        memoryMaxPages = hasMax ? readULEB128(buf, minRead.next).value : null;
      }
    } else if (sectionId === 0) {
      const nameLenRead = readULEB128(buf, sectionStart);
      const customName = buf.toString("utf8", nameLenRead.next, nameLenRead.next + nameLenRead.value);
      if (customName === "name") nameBytes += sectionSize;
      else otherCustomBytes += sectionSize;
    }
    offset = sectionEnd;
  }
  return { totalBytes, codeBytes, dataBytes, nameBytes, otherCustomBytes, functionCount, memoryInitialPages, memoryMaxPages };
}

const PLUGIN_SIZE_REPORT_PATH = join(pluginOutRoot, "📊️size-report.json");

/** @emoji 📏️ Every jco-extracted core wasm module currently on disk under `plugin-modules/`, largest
 * first. `🪞️vendor` and other non-plugin dirs are skipped the same way `rewriteExistingPluginShimImports`
 * skips them. */
function collectPluginWasmSizeRows(): PluginWasmSizeRow[] {
  if (!existsSync(pluginOutRoot)) return [];
  const rows: PluginWasmSizeRow[] = [];
  for (const entry of readdirSync(pluginOutRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || !moduleIdForDirectoryName(entry.name)) continue;
    const pluginDir = join(pluginOutRoot, entry.name);
    for (const file of readdirSync(pluginDir)) {
      if (!/\.core\d*\.wasm$/.test(file)) continue;
      const pluginId = moduleIdForDirectoryName(entry.name);
      if (pluginId) rows.push({ pluginId, file, ...analyzePluginWasmModule(join(pluginDir, file)) });
    }
  }
  return rows.sort((a, b) => b.totalBytes - a.totalBytes);
}

function formatPluginSizeBytes(n: number): string {
  return `${(n / (1024 * 1024)).toFixed(2)}MB`;
}

type EngineWasmSizeRow = PluginWasmSizeBreakdown & { readonly engineId: string; readonly file: string };

const ENGINE_SIZE_REPORT_PATH = join(pluginOutRoot, "📈️engine-size-report.json");

/** @emoji 📏️ Every wasm-bindgen engine's `*_bg.wasm` currently built under `node_modules/@semio-tech/*`
 * (flow-core, node-graph, editor, tiled-map, paint, terrain, board-2d — see `runWasmPackWebBuild`'s
 * `profile` option). These packages are workspace-symlinked (bun links `node_modules/@semio-tech/<pkg>`
 * to the crate dir), so each entry's realpath is resolved before scanning its `pkg/` dir. Reuses
 * `analyzePluginWasmModule` verbatim — it's generic wasm section accounting, not plugin-specific. */
function collectEngineWasmSizeRows(): EngineWasmSizeRow[] {
  const scopeDir = join(repoRoot, "node_modules/@semio-tech");
  if (!existsSync(scopeDir)) return [];
  const rows: EngineWasmSizeRow[] = [];
  for (const entry of readdirSync(scopeDir, { withFileTypes: true })) {
    let pkgDir: string;
    try {
      pkgDir = join(realpathSync(join(scopeDir, entry.name)), "pkg");
    } catch {
      continue;
    }
    if (!existsSync(pkgDir)) continue;
    for (const file of readdirSync(pkgDir)) {
      if (!file.endsWith("_bg.wasm")) continue;
      rows.push({ engineId: entry.name, file, ...analyzePluginWasmModule(join(pkgDir, file)) });
    }
  }
  return rows.sort((a, b) => b.totalBytes - a.totalBytes);
}

/** @emoji 📏️`plugin size` — measures every built plugin's core wasm (total/code/data/name bytes,
 * function count, memory initial/max pages), prints a per-plugin + total report, and persists
 * `📊️size-report.json` so the next run prints deltas — makes wasm-release/wasm-opt/dedup regressions
 * visible without re-deriving byte counts by hand. */
class PluginSizeScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const rows = collectPluginWasmSizeRows();
    if (rows.length === 0) {
      console.log("no built plugin core wasm modules found under plugin-modules/ — run `plugin` (build) first");
      return;
    }
    const previousRows: readonly PluginWasmSizeRow[] = existsSync(PLUGIN_SIZE_REPORT_PATH) ? (JSON.parse(readFileSync(PLUGIN_SIZE_REPORT_PATH, "utf8")) as PluginWasmSizeRow[]) : [];
    const previousByKey = new Map(previousRows.map((row) => [`${row.pluginId}/${row.file}`, row]));
    let totalBytes = 0;
    let totalCode = 0;
    let totalData = 0;
    let totalName = 0;
    let totalFunctions = 0;
    console.log(`plugin wasm size report (${rows.length} modules)`);
    for (const row of rows) {
      totalBytes += row.totalBytes;
      totalCode += row.codeBytes;
      totalData += row.dataBytes;
      totalName += row.nameBytes;
      totalFunctions += row.functionCount;
      const previousRow = previousByKey.get(`${row.pluginId}/${row.file}`);
      const delta = previousRow ? row.totalBytes - previousRow.totalBytes : null;
      const deltaLabel = delta === null ? "(new)" : delta === 0 ? "(=)" : `(${delta > 0 ? "+" : ""}${formatPluginSizeBytes(delta)})`;
      const maxLabel = row.memoryMaxPages === null ? "unbounded" : `${row.memoryMaxPages}pg`;
      console.log(
        `  ${row.pluginId.padEnd(16)} total=${formatPluginSizeBytes(row.totalBytes)} code=${formatPluginSizeBytes(row.codeBytes)} data=${formatPluginSizeBytes(row.dataBytes)} name=${formatPluginSizeBytes(row.nameBytes)} fns=${row.functionCount} mem=${row.memoryInitialPages ?? "?"}/${maxLabel} ${deltaLabel}`,
      );
    }
    console.log(`total: ${formatPluginSizeBytes(totalBytes)} (code ${formatPluginSizeBytes(totalCode)}, data ${formatPluginSizeBytes(totalData)}, name ${formatPluginSizeBytes(totalName)}, ${totalFunctions} functions across ${rows.length} modules)`);
    writeFileSync(PLUGIN_SIZE_REPORT_PATH, `${JSON.stringify(rows, null, 2)}\n`);

    const engineRows = collectEngineWasmSizeRows();
    if (engineRows.length === 0) return;
    const previousEngineRows: readonly EngineWasmSizeRow[] = existsSync(ENGINE_SIZE_REPORT_PATH) ? (JSON.parse(readFileSync(ENGINE_SIZE_REPORT_PATH, "utf8")) as EngineWasmSizeRow[]) : [];
    const previousEngineByKey = new Map(previousEngineRows.map((row) => [`${row.engineId}/${row.file}`, row]));
    let engineTotalBytes = 0;
    let engineTotalCode = 0;
    let engineTotalData = 0;
    let engineTotalName = 0;
    console.log(`engine wasm size report (${engineRows.length} modules)`);
    for (const row of engineRows) {
      engineTotalBytes += row.totalBytes;
      engineTotalCode += row.codeBytes;
      engineTotalData += row.dataBytes;
      engineTotalName += row.nameBytes;
      const previousRow = previousEngineByKey.get(`${row.engineId}/${row.file}`);
      const delta = previousRow ? row.totalBytes - previousRow.totalBytes : null;
      const deltaLabel = delta === null ? "(new)" : delta === 0 ? "(=)" : `(${delta > 0 ? "+" : ""}${formatPluginSizeBytes(delta)})`;
      console.log(`  ${row.engineId.padEnd(40)} total=${formatPluginSizeBytes(row.totalBytes)} code=${formatPluginSizeBytes(row.codeBytes)} data=${formatPluginSizeBytes(row.dataBytes)} name=${formatPluginSizeBytes(row.nameBytes)} ${deltaLabel}`);
    }
    console.log(
      `engine total: ${formatPluginSizeBytes(engineTotalBytes)} (code ${formatPluginSizeBytes(engineTotalCode)}, data ${formatPluginSizeBytes(engineTotalData)}, name ${formatPluginSizeBytes(engineTotalName)} across ${engineRows.length} modules)`,
    );
    writeFileSync(ENGINE_SIZE_REPORT_PATH, `${JSON.stringify(engineRows, null, 2)}\n`);
  }
}

export { ENGINE_SIZE_REPORT_PATH, EngineWasmSizeRow, PLUGIN_SIZE_REPORT_PATH, PluginSizeScript, PluginWasmSizeBreakdown, PluginWasmSizeRow, analyzePluginWasmModule, collectEngineWasmSizeRows, collectPluginWasmSizeRows, formatPluginSizeBytes, readULEB128 };

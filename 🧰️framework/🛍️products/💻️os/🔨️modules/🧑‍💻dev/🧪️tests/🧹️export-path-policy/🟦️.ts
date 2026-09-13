/** 🧩️ Semantic export path policy owner. */

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
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();



//#endregion 🔖️CapabilityLayeringLint

//#region 🔖️PluginIndexExportPathLint
/** 🕳️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s detector for a finding surfaced by
 * `26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY`: every `🟦️.ts` barrel under
 * `✏️s/🔌️plugins/<plugin>/📦️packages/🟦️typescript/` re-exports its `_snapshot`/`_diff`/`_mutations`
 * families as `export * as ... from "<relative path>"`, and the vast majority of those paths were
 * written against the pre-migration `🗿️artifacts/<a>/🧬️schema/…` tree — since migrated to
 * `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/…` — so they now resolve to nothing on disk. Reproduced live:
 * 517/567 relative specifiers dead across 33 plugins (worst: `📕️norm` 180/180, `🧱️block` 36/36,
 * `🧩️puzzle` 36/36).
 *
 * **Deliberately report-only, never wired into `verify`/`plugin lint`.** Unlike
 * `KNOWN_CAPABILITY_VIOLATIONS`/`KNOWN_LAYERING_VIOLATIONS` above (a *hard* gate with a hand-picked,
 * evidence-backed allowlist of pre-existing exceptions), 517 dead specifiers have no sane per-entry
 * grandfather list, and the actual fix — repointing every path at the migrated
 * `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/` shape — is explicitly out of this ticket's boundary (it would
 * mean editing `🟦️.ts`, forbidden here) and remains unowned. So `run()` below never throws: it
 * is only reachable via its own standalone `index-lint` router command / nx target, not folded into
 * any gate the way `layer-lint` was. */
const PLUGIN_BARREL_RELATIVE_EXPORT_PATTERN = /from\s+"(\.[^"]+)"/g;

/** 🧭️ Resolution order this lint checks a barrel's relative specifier against — literal path, then
 * `.ts`/`.tsx`, then a directory's `🟦️.ts`/`index.ts`. Matches how the rest of this toolchain
 * (bundler + `tsc`) would actually resolve the same specifier. */
function resolvesPluginBarrelExport(baseDir: string, spec: string): boolean {
  return [spec, `${spec}.ts`, `${spec}.tsx`, `${spec}/🟦️.ts`, `${spec}/index.ts`].some((candidate) => existsSync(join(baseDir, candidate)));
}

class PluginIndexExportPathLintScript extends BundleScript {
  async run(): Promise<void> {
    const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
    let totalDead = 0;
    let totalAll = 0;
    let pluginsWithDeadPaths = 0;
    for (const pluginId of readdirSync(pluginsRoot).sort()) {
      const indexPath = join(pluginsRoot, pluginId, "📦️packages/🟦️typescript/🟦️.ts");
      if (!existsSync(indexPath)) continue;
      const source = await Bun.file(indexPath).text();
      const baseDir = dirname(indexPath);
      const deadSpecs: string[] = [];
      let total = 0;
      for (const [, spec] of source.matchAll(PLUGIN_BARREL_RELATIVE_EXPORT_PATTERN)) {
        total++;
        if (!resolvesPluginBarrelExport(baseDir, spec)) deadSpecs.push(spec);
      }
      totalAll += total;
      totalDead += deadSpecs.length;
      if (deadSpecs.length === 0) continue;
      pluginsWithDeadPaths++;
      const cause = deadSpecs.some((s) => s.includes("🗿️artifacts/")) ? "likely pre-standards path (🗿️artifacts/<a>/🧬️schema/…) against the migrated 🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/ tree" : "target does not exist on disk";
      console.warn(`[plugin-index-export-path-lint] WARN ${relative(repoRoot, indexPath)}: ${deadSpecs.length}/${total} relative export path(s) resolve to nothing (${cause})`);
    }
    console.log(`plugin index export path lint: ${totalDead}/${totalAll} dead relative export path(s) across ${pluginsWithDeadPaths} plugin(s) — REPORT ONLY, does not gate (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE)`);
  }
}

export { PLUGIN_BARREL_RELATIVE_EXPORT_PATTERN, PluginIndexExportPathLintScript, resolvesPluginBarrelExport };

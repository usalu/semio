/** 🧩️ Semantic capability policy owner. */

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

import { generatePluginRegistry, type PluginRegistryEntry } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();



const PLUGIN_HOST_MODE_SYMBOLS = ["SEMIO_PLUGIN", "PLAYGROUND_APP_KIND", "hostMode", "pluginFilter"] as const;

function walkRustSources(dir: string, out: string[]): void {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name === "target" || ent.name === "node_modules") continue;
    const abs = join(dir, ent.name);
    if (ent.isDirectory()) {
      walkRustSources(abs, out);
      continue;
    }
    if (ent.name.endsWith(".rs")) out.push(abs);
  }
}

/** 🕵️ Statically scans this playground's own `⚙️vite.config.ts` for its hardcoded
 * `{ find: "...", replacement: path.resolve(repoRoot, "...") }` `resolve.alias` entries and
 * asserts every `replacement` target exists on disk — a plain text/regex scan rather than an
 * `import()` of the config module itself, since that module's default export executes a full
 * `defineConfig({...})` (brand/plugin/renderer resolution, plugin-factory calls with real I/O)
 * as an unconditional side effect of module evaluation, which would be unsafe and slow to
 * trigger merely to read one array. Scanning the source text keeps `⚙️vite.config.ts` itself as
 * the single source of truth (no second, independently-stale-able alias list) while still
 * catching a stale alias before it ships silently. Dynamic mount points (`/🔌️plugin-modules`,
 * `/renderer-modules`) aren't in this pattern (they resolve local `const` dir variables, not an
 * inline `path.resolve(repoRoot, "...")` literal) and are intentionally not checked here —
 * `renderer-modules` in particular is a build-output directory that legitimately doesn't exist
 * until a wgpu build populates it. */
async function checkPlaygroundAliasFreshness(): Promise<string[]> {
  const viteConfigPath = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts");
  const source = await Bun.file(viteConfigPath).text();
  const aliasPattern = /\{\s*find:\s*"([^"]+)",\s*replacement:\s*path\.resolve\(repoRoot,\s*"([^"]+)"\)\s*\}/g;
  const failures: string[] = [];
  for (const [, find, relativeTarget] of source.matchAll(aliasPattern)) {
    if (!existsSync(join(repoRoot, relativeTarget))) {
      failures.push(`⚙️vite.config.ts: alias "${find}" -> "${relativeTarget}" does not exist on disk`);
    }
  }
  return failures;
}

/** 🚧️ Pre-existing capability-rule violations, real but predating this lint's revival (ticket
 * 26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL): the filter this rule ran against used
 * to match zero packages, so these went undetected for as long as they've existed. Grandfathered
 * as WARN (not a `verify gate` failure) so reviving the rule with real teeth doesn't redden the
 * gate for unrelated pre-existing plugin work — mirrors the master ticket's general
 * warn-until-finalization pattern for revived W0 checks. Any violation NOT already listed here
 * still hard-fails the gate; remove an entry once its underlying violation is fixed.
 *
 * The `semio-framework-os` (OS HOST crate) block below is `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s
 * addition: exactly the 17 plugin crates census'd live in 📓️w0-d-sdk-surface.md §3.2 as depending on the
 * host crate, each verified with `grep -rl '^semio-framework-os[[:space:]]*=' ✏️s/🔌️plugins/**​/📦️packages/🦀️rust/Cargo.toml`
 * at seed time. This backlog is APA's own to shrink (its W3/W4 waves move the `semio_framework_os::*`
 * symbols these 17 crates use into the SDK's curated re-export surface and drop the host dep) — it must
 * only ever shrink from here; never add a NEW plugin to this list to silence a fresh violation. */
const KNOWN_CAPABILITY_VIOLATIONS = new Set<string>([
  "semio-s-plugin-writer: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-procedural: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-gis: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-demonstrator: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-process: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-layout: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-cad: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-shooting: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-animate: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-lowpoly: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-remodel: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-note: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-trinity: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-draw: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-raster: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-puzzle: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-space: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  // 🚪️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: predates the std::env/std::process addition — puzzle's
  // build.rs already used std::fs (read_dir/copy/write) under the OLD std::fs/std::net-only check, undeclared
  // (no localBackboneStorage capability), so this was already a live gate failure before this wave touched
  // anything. Seeded here rather than left as an unexplained new-looking regression once std::env joined the
  // same check (build.rs also reads CARGO_MANIFEST_DIR/OUT_DIR via std::env::var).
  "semio-s-plugin-puzzle: uses std::fs/std::net/std::env/std::process without localBackboneStorage capability (✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs)",
]);

class PluginCapabilityLintScript extends BundleScript {
  async run(): Promise<void> {
    const metadataResult = runProbe("cargo", ["metadata", "--format-version", "1", "--no-deps"], { cwd: repoRoot, budgetMs: buildBudgetMs() });
    if (metadataResult.status !== 0) {
      throw new Error(metadataResult.stderr || "cargo metadata failed");
    }
    const metadata = JSON.parse(metadataResult.stdout || "{}") as {
      packages: Array<{
        name: string;
        manifest_path: string;
        dependencies: Array<{ name: string }>;
      }>;
    };
    const registryEntries = generatePluginRegistry(repoRoot);
    const pluginPackageNames = new Map(registryEntries.map((entry) => [entry.packageName, entry.pluginId]));
    const depRules: Record<string, string> = {
      rusqlite: "localBackboneStorage",
      libloading: "forbidden",
      reqwest: "forbidden",
      "web-sys": "forbidden",
      "js-sys": "forbidden",
      egui: "forbidden",
      eframe: "forbidden",
      wgpu: "forbidden",
      "wgpu-core": "forbidden",
      winit: "forbidden",
      // 🚪️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: the OS HOST crate. A plugin/extension may
      // depend on the plugin SDK, never on the host that hosts it — see 📓️w0-d-sdk-surface.md §3.2 for
      // the current 17-crate backlog, seeded below in KNOWN_CAPABILITY_VIOLATIONS.
      "semio-framework-os": "forbidden",
    };
    // 🕵️ Real registry membership, not a path substring: the pre-emoji-rename
    // `/plugin/rs/Cargo.toml` substring this used to filter on matches zero packages under
    // today's `✏️s/🔌️plugins/**/📦️packages/🦀️rust/Cargo.toml` layout, so every plugin
    // silently skipped this lint entirely — revived in ticket
    // 26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL.
    let checkedPackageCount = 0;
    const failures: string[] = [];
    for (const pkg of metadata.packages) {
      if (!pluginPackageNames.has(pkg.name)) continue;
      checkedPackageCount++;
      const manifestText = await Bun.file(pkg.manifest_path).text();
      const declared = new Set<string>();
      const metaMatch = manifestText.match(/\[package\.metadata\.semio\][\s\S]*?capabilities\s*=\s*\[([^\]]*)\]/);
      if (metaMatch?.[1]) {
        for (const entry of metaMatch[1].match(/"([^"]+)"/g) ?? []) {
          declared.add(entry.slice(1, -1));
        }
      }
      if (manifestText.includes("local_backbone_storage()") || manifestText.includes("ArtifactKind::Backbone")) {
        declared.add("localBackboneStorage");
      }
      const depNames = new Set(pkg.dependencies.map((dep) => dep.name));
      for (const dep of pkg.dependencies) {
        const otherPluginId = pluginPackageNames.get(dep.name);
        if (otherPluginId && dep.name !== pkg.name) {
          failures.push(`${pkg.name}: cross-plugin dependency on ${dep.name} (${otherPluginId})`);
        }
      }
      for (const [dep, rule] of Object.entries(depRules)) {
        if (!depNames.has(dep)) continue;
        if (rule === "forbidden") {
          failures.push(`${pkg.name}: forbidden dependency ${dep}`);
          continue;
        }
        if (!declared.has(rule)) {
          failures.push(`${pkg.name}: dependency ${dep} requires capability ${rule}`);
        }
      }
      const rustSources: string[] = [];
      walkRustSources(dirname(pkg.manifest_path), rustSources);
      for (const sourcePath of rustSources) {
        const source = await Bun.file(sourcePath).text();
        // 🚪️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: std::env/std::process joined std::fs/std::net
        // on the same footing — all four are raw-ambient-authority escape hatches a sandboxed plugin
        // must not reach for directly; all four are gated by the same localBackboneStorage capability.
        if (/std::fs::|std::net::|std::env::|std::process::/.test(source) && !declared.has("localBackboneStorage")) {
          failures.push(`${pkg.name}: uses std::fs/std::net/std::env/std::process without localBackboneStorage capability (${relative(repoRoot, sourcePath)})`);
        }
        for (const symbol of PLUGIN_HOST_MODE_SYMBOLS) {
          if (!source.includes(symbol)) continue;
          failures.push(`${pkg.name}: program source references host-mode symbol ${symbol} (${relative(repoRoot, sourcePath)})`);
        }
      }
    }
    const grandfathered = failures.filter((f) => KNOWN_CAPABILITY_VIOLATIONS.has(f));
    const blocking = [...failures.filter((f) => !KNOWN_CAPABILITY_VIOLATIONS.has(f)), ...(await checkPlaygroundAliasFreshness())];
    for (const warning of grandfathered) console.warn(`[plugin-capability-lint] WARN (grandfathered, see spawned fix-it task): ${warning}`);
    if (blocking.length > 0) {
      for (const failure of blocking) console.error(`[plugin-capability-lint] ${failure}`);
      throw new Error(`plugin capability lint failed (${blocking.length} issue(s), ${checkedPackageCount} plugin package(s) evaluated)`);
    }
    console.log(`program capability lint passed (${checkedPackageCount} plugin package(s) evaluated, ${grandfathered.length} grandfathered warning(s))`);
  }
}

export { KNOWN_CAPABILITY_VIOLATIONS, PLUGIN_HOST_MODE_SYMBOLS, PluginCapabilityLintScript, checkPlaygroundAliasFreshness, walkRustSources };

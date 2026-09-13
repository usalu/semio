/** 🧩️ Semantic benchmark plan owner. */

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

const repoRoot = getWorkspaceRoot();



//#endregion 🔖️ScaleFixture

//#region 🔖️Bench
/** ⚖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b-bench): `bun ./📜️script.ts bench plugins
 * [--renderer native|react|wgpu] [--count 50] [--extensions 50] [--shards 8] [--out <path>]` — the
 * dev-side harness the root `bench` verb (`/📜️script.ts` `//#region 🔖️BenchScript`) routes into.
 * Budget 1 (registry parse) is measured HERE, directly, in JS — no wasm/kernel involved, so it never
 * needs the native/web split. Budgets 2-8 are measured by the renderer-specific harness: `native`
 * drives `semio-wgpu-native --scale/--scale-wasm/--shards/--report`
 * (`📺️renderer/…/🧊️wgpu/🦀️.rs`'s `scale_bench` module — real `Kernel`/`ShardLoop`/
 * `WasmtimeRuntime`, real scale-fixture wasm component, see that module's own doc for its honest
 * single-physical-ShardLoop scope note); `react`/`wgpu` (web) drive `//#region 🧪️BenchWebRows` below —
 * NOT `🔬️ParityScript`'s `🔖️ServerPool` (that machinery boots the FULL app against one real plugin
 * variant, a different app than the scale fixture, and needs real fleet wasm this session doesn't have
 * either) — instead the real `ShardClient` runs inside a real headless-Chromium page
 * (`📊️bench-web-harness/🟦️.ts`, bundled with `Bun.build`) against real browser `Worker`s running a protocol
 * STUB in place of the not-yet-compiled guest SDK's real worker. Budgets 3/4/6/7/8 are genuine passes of
 * `ShardClient`'s own sharding/heartbeat-trap/checkpoint logic at 100-actor scale; budgets 2/5 are
 * stub-worker timings reported as `pass-stub-worker`/`fail-stub-worker`, never plain `pass`/`fail`,
 * because they exclude real wasm instantiation and guest compute. Any harness failure (no Chromium, bundle
 * error, page timeout) falls every row back to `"skipped"` with the real error — never a fabricated pass.
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-workforce.md §4
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-bench-web-rows-report.md
 */
function benchOutDir(renderer: string): string {
  const configured = process.env.BENCH_OUT_DIR ?? join(getRepoMetaDir(repoRoot), "⚡️cache", "bench", "plugins", renderer);
  const dir = resolve(repoRoot, configured);
  mkdirSync(dir, { recursive: true });
  return dir;
}

type BenchBudgetDefinition = Readonly<{ id: number; description: string; nativeThreshold?: string; webThreshold?: string }>;

/** 📓️ `design-workforce.md` §4, verbatim, ONE const — descriptions + threshold numbers as data, never
 * scattered literals. The pass/fail MATH for budgets 2-8 lives on whichever side measures them (this
 * table is not re-evaluated here); budget 1's math lives in `benchRegistryRow` right below it. */
const BENCH_BUDGETS: readonly BenchBudgetDefinition[] = [
  { id: 1, description: "Registry: 2550 records parsed, instantiations == 0, < 150ms", nativeThreshold: "150ms", webThreshold: "150ms" },
  { id: 2, description: "Cold boot to first interactive frame, only on-startup-finished actors live", nativeThreshold: "1500ms", webThreshold: "2500ms" },
  { id: 3, description: "Activate 50 plugins + 50 extensions of one plugin: active_actors==100, shards==K, no shard > ceil(100/K)+1" },
  { id: 4, description: "Memory <= K x 512MiB + 256MiB headroom (web Worker count==K); native RSS <= 1.5GiB", nativeThreshold: "1.5GiB RSS" },
  { id: 5, description: "Interactive p95 command->patch, 40 cpu actors saturating background", nativeThreshold: "8ms", webThreshold: "16ms" },
  { id: 6, description: "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", nativeThreshold: "pause <= 250ms" },
  { id: 7, description: "stateful actor LRU-suspended and resumed -> identical state hash" },
  { id: 8, description: "Capability revoked at runtime -> denied completion, actor stays alive, quota counters zero" },
] as const;

function benchFlag(segments: readonly string[], name: string, fallback: string): string {
  for (let i = 0; i < segments.length; i++) {
    const segment = segments[i]!;
    if (segment === `--${name}` && segments[i + 1] !== undefined) return segments[i + 1]!;
    if (segment.startsWith(`--${name}=`)) return segment.slice(name.length + 3);
  }
  return fallback;
}

/** ⏱️ Budget 1 — measured directly: reads+parses the on-disk registry.json this run generated,
 * timing ONLY that (never touches wasm/kernel, so `instantiations == 0` is true by construction, not
 * merely asserted). */
function benchRegistryRow(registryPath: string, expectedRecordCount: number): Record<string, unknown> {
  const t0 = performance.now();
  const raw = readFileSync(registryPath, "utf8");
  const parsed = JSON.parse(raw) as { recordCount: number };
  const elapsedMs = performance.now() - t0;
  const pass = parsed.recordCount === expectedRecordCount && elapsedMs < 150;
  return {
    id: 1,
    description: BENCH_BUDGETS[0]!.description,
    status: pass ? "pass" : "fail",
    measured: { elapsedMs, recordCount: parsed.recordCount, instantiations: 0 },
    threshold: { maxMs: 150, instantiations: 0 },
    note: "measured by this dev script directly (bun readFileSync + JSON.parse) — no wasm/kernel touched",
  };
}

function benchWebSkippedRow(budget: BenchBudgetDefinition, renderer: string, reason: string): Record<string, unknown> {
  return {
    id: budget.id,
    description: budget.description,
    status: "skipped",
    measured: null,
    threshold: budget.webThreshold ?? null,
    note: `${renderer} web-renderer bench row ${budget.id} could not be measured this run: ${reason}`,
  };
}

export { BENCH_BUDGETS, BenchBudgetDefinition, benchFlag, benchOutDir, benchRegistryRow, benchWebSkippedRow };

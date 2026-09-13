/** 🧩️ Semantic benchmark browser owner. */

import { repoCacheDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { BenchBudgetDefinition, benchWebSkippedRow } from "../📋️plan/🟦️.ts";

import { benchWebMeasuredRow } from "../🧪️stub/🟦️.ts";

import { ScaleFixtureRegistry } from "../../../../../🧫️fixtures/⚖️scale/📽️projection/🟦️.ts";



//#region 🧪️BenchWebRows
/** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (bench-web-rows): bundles `📊️bench-web-harness/🟦️.ts` for
 * the BROWSER with Bun's own bundler (no external bundler dependency), runs it inside a real headless
 * Chromium page via `playwright` (already a repo dependency — the SAME dynamic-import pattern this file's
 * own collab/studio-e2e scripts already use), and merges the raw per-budget measurements back onto
 * `BENCH_BUDGETS`'s id/description/threshold. See that file's own header doc for exactly what is REAL
 * (driven through the genuine `ShardClient`, real browser `Worker`s, real postMessage round trips) versus
 * STUB (no real fleet wasm exists yet — `semio-framework-plugin` does not compile this session — so each
 * worker runs a tiny protocol stub instead of the real generated `shardWorkerSource()`). `renderer` is
 * accepted for parity with the native row's `--renderer` flag and threaded into the report's metadata,
 * but the harness itself is renderer-agnostic: it measures the `ShardClient` transport layer, which react
 * and wgpu(web) share — it does NOT exercise either renderer's own paint/patch path. That gap is stated
 * here rather than silently implied by a `react`/`wgpu`-labelled row. */
async function buildBenchWebHarnessBundle(): Promise<string> {
  const entry = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️bench-web-harness/🟦️.ts");
  const result = await Bun.build({ entrypoints: [entry], target: "browser", format: "esm" });
  if (!result.success) throw new Error(`bench-web harness bundle failed: ${result.logs.map((log) => log.message).join("; ")}`);
  const output = result.outputs[0];
  if (!output) throw new Error("bench-web harness bundle produced no output file");
  return await output.text();
}

async function runWebBenchViaHeadlessChromium(pluginIds: readonly string[], firstPluginExtensionIds: readonly string[], shardCount: number): Promise<Record<string, unknown>[]> {
  const bundleJs = await buildBenchWebHarnessBundle();
  const html = `<!doctype html><html><head><meta charset="utf-8"><title>bench-web</title></head><body><script type="module">
${bundleJs}
window.__BENCH_WEB__ = { done: false, rows: null, error: null };
runBenchWebBudgets(${JSON.stringify({ pluginIds, firstPluginExtensionIds, shardCount })})
  .then((rows) => { window.__BENCH_WEB__.rows = rows; window.__BENCH_WEB__.done = true; })
  .catch((error) => { window.__BENCH_WEB__.error = String((error && error.stack) || error); window.__BENCH_WEB__.done = true; });
</script></body></html>`;
  // 🎭️ Matches `StudioE2eScript`'s own install location note above — same repo-scoped Playwright cache.
  process.env.PLAYWRIGHT_BROWSERS_PATH = process.env.PLAYWRIGHT_BROWSERS_PATH ?? repoCacheDirectory(repoRoot, "tools", "ms-playwright");
  const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.setContent(html, { waitUntil: "load" });
    await page.waitForFunction(() => (window as unknown as { __BENCH_WEB__: { done: boolean } }).__BENCH_WEB__.done === true, { timeout: 60_000 });
    const state = await page.evaluate(() => (window as unknown as { __BENCH_WEB__: { rows: Record<string, unknown>[] | null; error: string | null } }).__BENCH_WEB__);
    if (state.error) throw new Error(`bench-web harness page error: ${state.error}`);
    if (!state.rows) throw new Error("bench-web harness produced no rows");
    return state.rows;
  } finally {
    await browser.close();
  }
}

/** ▶️ Runs budgets 2-8 for `react`/`wgpu` through the real `ShardClient` + headless-Chromium harness.
 * `registry` supplies the actor id vocabulary (`buildScaleFixtureRegistry`'s own deterministic ids —
 * never invented ones) budget 3 needs: 50 plugin ids + the 50 extension ids belonging to plugin[0]. On
 * ANY harness failure (no Chromium installed, bundle error, page timeout, …) every row falls back to
 * `benchWebSkippedRow` with the real error message — never a silently fabricated pass. */
async function benchWebRows(budgets: readonly BenchBudgetDefinition[], renderer: string, registry: ScaleFixtureRegistry, shardCount: number): Promise<Record<string, unknown>[]> {
  const plugins = registry.records.filter((record) => record.kind === "plugin").map((record) => record.id);
  const firstPluginId = plugins[0];
  if (!firstPluginId) return budgets.map((budget) => benchWebSkippedRow(budget, renderer, "scale-fixture registry has no plugin records"));
  const firstPluginExtensions = registry.records.filter((record) => record.kind === "extension" && record.parentId === firstPluginId).map((record) => record.id);
  try {
    const raw = await runWebBenchViaHeadlessChromium(plugins, firstPluginExtensions, shardCount);
    const byId = new Map(raw.map((row) => [row.id, row]));
    return budgets.map((budget) => {
      const row = byId.get(budget.id);
      return row ? benchWebMeasuredRow(budget, renderer, row) : benchWebSkippedRow(budget, renderer, "harness returned no row for this budget id");
    });
  } catch (error) {
    const reason = error instanceof Error ? (error.stack ?? error.message) : String(error);
    return budgets.map((budget) => benchWebSkippedRow(budget, renderer, reason));
  }
}

export { benchWebRows, buildBenchWebHarnessBundle, runWebBenchViaHeadlessChromium };

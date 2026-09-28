/** 🥾️ Boot budget of the served `s` React shell: what the user waits for when the shell starts — the serve answering its
 * first request (when the harness had to start it), one COLD boot in a fresh browser context and one WARM reload in the
 * same context.
 *
 * Per load, from the page's own Navigation / Paint / Long Task / Resource Timing entries (the browser is the third-party
 * oracle): time to first byte, first contentful paint, the shell's `data-semio-os-ready` beacon, every catalog plugin
 * `loaded` (`__semioOsCatalogProbe`), long tasks, and per resource kind (`bootResourceKindV1`) the count, transferred and
 * decoded bytes. Law (`🧑‍💻dev/🧫️fixtures/🥾️boot-budget.json`): the payload of the cold boot (counts and decoded MB per kind)
 * and the warm reload's transfer are judged on every run — they do not depend on the machine; timings are judged only
 * while the 1-minute load stays ≤ `loadCeilingPerCore` × cores, else the check is `blocked` with the measured values.
 *
 * Promoted from the ticket harnesses `wp-f2/f2-boot.mjs` + `wp-f3/f3-boot.mjs` (ticket 26/09/23 F2/F3: serve start 65 s →
 * 3.8 s, 453 → 465 modules / 40 MB after the drei/three-stdlib split).
 */

import { cpus, loadavg, tmpdir } from "node:os";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { BrowserContext, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
import { devServePortV1, ensureDevServe, type DevServeFixtureV1 } from "../../🚀️local-hub/🏃️execution/🟦️.ts";
import { awaitBeacon } from "../🧮️program-matrix/🟦️.ts";

//#region 🧾️Budget
/** 🗂️ What one boot resource is, by its path: the shell's JS modules, the plugin descriptors (JSON), the guest font pack,
 * style sheets, wasm, and everything else. */
export type BootResourceKindV1 = "module" | "json" | "font" | "style" | "wasm" | "other";

/** 📏️ One kind's payload bound. */
export type BootPayloadBoundV1 = Readonly<{ maxCount: number; maxDecodedMB: number }>;

/** 📏️ The budget the fixture declares. */
export type BootBudgetV1 = Readonly<{
  loadCeilingPerCore: number;
  serve: Readonly<{ maxReadyMs: number }>;
  cold: Readonly<{ maxFcpMs: number; maxReadyMs: number; maxPluginsLoadedMs: number; maxLongTaskMs: number }>;
  warm: Readonly<{ maxReadyMs: number; maxTransferMB: number }>;
  payload: Readonly<Record<BootResourceKindV1, BootPayloadBoundV1>>;
}>;

/** 🧾️ One resource timing entry as the reducer reads it. */
export type BootResourceV1 = Readonly<{ url: string; initiatorType: string; transferSize: number; decodedBodySize: number }>;

/** 📦️ Count and megabytes of one kind. */
export type BootPayloadV1 = Readonly<{ count: number; transferMB: number; decodedMB: number }>;

/** ⏱️ One measured load. */
export type BootLoadV1 = Readonly<{
  ttfbMs: number | null;
  fcpMs: number | null;
  readyMs: number | null;
  pluginsLoadedMs: number | null;
  plugins: Readonly<{ total: number; loaded: number }>;
  longTaskMs: number;
  longTasks: number;
  payload: Readonly<Record<BootResourceKindV1, BootPayloadV1>>;
}>;

/** 🧾️ Everything one run measured. */
export type BootMeasurementV1 = Readonly<{ load: number; cores: number; serveReadyMs: number | null; cold: BootLoadV1; warm: BootLoadV1 }>;

/** ⚖️ The verdict: `blocked` only when every always-judged bound held and the timings could not be judged. */
export type BootVerdictV1 = Readonly<{ status: "pass" | "fail" | "blocked"; timingJudged: boolean; violations: readonly string[] }>;

/** 🧪️ One law vector of the fixture — expected values computed by an independent implementation. */
export type BootBudgetVectorV1 = Readonly<{ name: string; resources: readonly BootResourceV1[]; expectedKinds: readonly BootResourceKindV1[]; expectedPayload: Readonly<Record<BootResourceKindV1, BootPayloadV1>>; measurement: BootMeasurementV1; expectedVerdict: BootVerdictV1 }>;

/** 📖️ The fixture: the budget and its law vectors. */
export type BootBudgetFixtureV1 = Readonly<{ schema: "semio.os-dev.boot-budget/v1"; description: string; budget: BootBudgetV1; vectors: readonly BootBudgetVectorV1[] }>;

export const BOOT_RESOURCE_KINDS_V1: readonly BootResourceKindV1[] = ["module", "json", "font", "style", "wasm", "other"];

/** 📖️ Reads `🧑‍💻dev/🧫️fixtures/🥾️boot-budget.json`. */
export function readBootBudgetFixtureV1(): BootBudgetFixtureV1 {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🥾️boot-budget.json", import.meta.url), "utf8")) as BootBudgetFixtureV1;
  if (fixture.schema !== "semio.os-dev.boot-budget/v1") throw new Error(`boot budget: unexpected schema ${String(fixture.schema)}`);
  return fixture;
}

/** 🗂️ The kind of one resource, by its decoded path: a JSON file the code IMPORTS (initiator `script`) is a module — a
 * bundle inlines it — while a fetched one is a descriptor; anything a style sheet pulls in (cursors, images) is `other`. */
export function bootResourceKindV1(url: string, initiatorType: string): BootResourceKindV1 {
  const path = decodeURIComponent(new URL(url).pathname);
  if (path.endsWith(".wasm")) return "wasm";
  if (/\.(woff2?|ttf|otf)$/u.test(path) || /fonts?\.bin$/u.test(path)) return "font";
  if (path.endsWith(".css")) return "style";
  if (path.endsWith(".json")) return initiatorType === "script" ? "module" : "json";
  if (/\.(m?js|jsx|ts|tsx)$/u.test(path) || initiatorType === "script") return "module";
  return "other";
}

const MB = 2 ** 20;
const round2 = (value: number): number => Math.round(value * 100) / 100;

/** 📦️ Count and megabytes per kind (every kind present, rounded to 0.01 MB after summing). */
export function bootPayloadV1(resources: readonly BootResourceV1[]): Record<BootResourceKindV1, BootPayloadV1> {
  const totals = Object.fromEntries(BOOT_RESOURCE_KINDS_V1.map((kind) => [kind, { count: 0, transfer: 0, decoded: 0 }])) as Record<BootResourceKindV1, { count: number; transfer: number; decoded: number }>;
  for (const resource of resources) {
    const row = totals[bootResourceKindV1(resource.url, resource.initiatorType)];
    row.count += 1;
    row.transfer += resource.transferSize;
    row.decoded += resource.decodedBodySize;
  }
  return Object.fromEntries(BOOT_RESOURCE_KINDS_V1.map((kind) => [kind, { count: totals[kind].count, transferMB: round2(totals[kind].transfer / MB), decodedMB: round2(totals[kind].decoded / MB) }])) as Record<BootResourceKindV1, BootPayloadV1>;
}

/** ⚖️ Judges one run against the budget: payload bounds and the warm transfer always, timings only under the load ceiling;
 * a missing timing (the beacon or the plugins never came) is always a violation. */
export function judgeBootBudgetV1(budget: BootBudgetV1, measured: BootMeasurementV1): BootVerdictV1 {
  const timingJudged = measured.load <= budget.loadCeilingPerCore * measured.cores;
  const violations: string[] = [];
  for (const kind of BOOT_RESOURCE_KINDS_V1) {
    const bound = budget.payload[kind];
    const row = measured.cold.payload[kind];
    if (row.count > bound.maxCount) violations.push(`${kind}: ${row.count} files (bound ${bound.maxCount})`);
    if (row.decodedMB > bound.maxDecodedMB) violations.push(`${kind}: ${row.decodedMB} MB (bound ${bound.maxDecodedMB} MB)`);
  }
  const warmTransferMB = round2(BOOT_RESOURCE_KINDS_V1.reduce((sum, kind) => sum + measured.warm.payload[kind].transferMB, 0));
  if (warmTransferMB > budget.warm.maxTransferMB) violations.push(`warm reload transferred ${warmTransferMB} MB (bound ${budget.warm.maxTransferMB} MB)`);
  for (const [label, load] of [["cold", measured.cold], ["warm", measured.warm]] as const) {
    if (load.readyMs === null) violations.push(`${label}: the shell never became ready`);
    if (load.plugins.total === 0 || load.plugins.loaded < load.plugins.total) violations.push(`${label}: ${load.plugins.loaded}/${load.plugins.total} plugins loaded`);
  }
  if (timingJudged) {
    const over = (label: string, value: number | null, bound: number): void => {
      if (value !== null && value > bound) violations.push(`${label} ${value} ms (bound ${bound} ms)`);
    };
    over("serve first answer", measured.serveReadyMs, budget.serve.maxReadyMs);
    over("cold first contentful paint", measured.cold.fcpMs, budget.cold.maxFcpMs);
    over("cold ready", measured.cold.readyMs, budget.cold.maxReadyMs);
    over("cold all plugins loaded", measured.cold.pluginsLoadedMs, budget.cold.maxPluginsLoadedMs);
    over("cold long tasks", measured.cold.longTaskMs, budget.cold.maxLongTaskMs);
    over("warm ready", measured.warm.readyMs, budget.warm.maxReadyMs);
  }
  return { status: violations.length > 0 ? "fail" : timingJudged ? "pass" : "blocked", timingJudged, violations };
}
//#endregion 🧾️Budget

//#region ⏱️Measure
const installBootProbe = (): void => {
  performance.setResourceTimingBufferSize(100_000);
  const marks = { fcp: null as number | null, ready: null as number | null, longTasks: [] as number[] };
  Object.defineProperty(window, "__semioBootBudget", { value: marks });
  new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) if (entry.name === "first-contentful-paint") marks.fcp = entry.startTime;
  }).observe({ type: "paint", buffered: true });
  new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) marks.longTasks.push(entry.duration);
  }).observe({ type: "longtask", buffered: true });
  new MutationObserver(() => {
    if (marks.ready === null && document.documentElement?.hasAttribute("data-semio-os-ready")) marks.ready = performance.now();
  }).observe(document, { attributes: true, subtree: true, attributeFilter: ["data-semio-os-ready"] });
};

async function measureLoad(page: Page, navigate: () => Promise<unknown>, deadlineMs: number, signal: AbortSignal): Promise<BootLoadV1> {
  await navigate();
  const deadline = Date.now() + deadlineMs;
  const beacon = await awaitBeacon(page, deadline);
  const readyFallback = beacon?.startsWith("ready:") ? await page.evaluate(() => performance.now()) : null;
  let plugins = { total: 0, loaded: 0 };
  let pluginsLoadedMs: number | null = null;
  while (Date.now() < deadline && !signal.aborted && beacon?.startsWith("ready:")) {
    const state = await page.evaluate(() => {
      const rows = (window as unknown as { __semioOsCatalogProbe?: { plugins?: { status: string }[] } }).__semioOsCatalogProbe?.plugins ?? [];
      return { total: rows.length, loaded: rows.filter((row) => row.status === "loaded").length, now: performance.now() };
    });
    plugins = { total: state.total, loaded: state.loaded };
    if (state.total > 0 && state.loaded === state.total) {
      pluginsLoadedMs = Math.round(state.now);
      break;
    }
    await page.waitForTimeout(250);
  }
  await page.waitForTimeout(2_000);
  const facts = await page.evaluate(() => {
    const marks = (window as unknown as { __semioBootBudget: { fcp: number | null; ready: number | null; longTasks: number[] } }).__semioBootBudget;
    const navigation = performance.getEntriesByType("navigation")[0] as PerformanceNavigationTiming | undefined;
    const resources = (performance.getEntriesByType("resource") as PerformanceResourceTiming[]).map((entry) => ({ url: entry.name, initiatorType: entry.initiatorType, transferSize: entry.transferSize, decodedBodySize: entry.decodedBodySize }));
    return { ttfb: navigation ? navigation.responseStart : null, fcp: marks.fcp, ready: marks.ready, longTasks: marks.longTasks, resources };
  });
  const ready = facts.ready ?? readyFallback;
  return {
    ttfbMs: facts.ttfb === null ? null : Math.round(facts.ttfb),
    fcpMs: facts.fcp === null ? null : Math.round(facts.fcp),
    readyMs: ready === null ? null : Math.round(ready),
    pluginsLoadedMs,
    plugins,
    longTaskMs: Math.round(facts.longTasks.reduce((sum, duration) => sum + duration, 0)),
    longTasks: facts.longTasks.length,
    payload: bootPayloadV1(facts.resources),
  };
}

/** 🥾️ One cold boot in a fresh persistent browser profile (a real HTTP disk cache, like the user's browser), then one warm
 * reload in it; the profile is deleted afterwards. */
export async function runBootBudget(options: Readonly<{ baseUrl: string; serveReadyMs: number | null; headed: boolean; signal: AbortSignal }>): Promise<BootMeasurementV1> {
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const profile = mkdtempSync(join(tmpdir(), "semio-boot-budget-"));
  const context: BrowserContext = await chromium.launchPersistentContext(profile, { headless: !options.headed, viewport: { width: 1440, height: 900 }, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
  const abort = (): void => void context.close().catch(() => undefined);
  options.signal.addEventListener("abort", abort, { once: true });
  try {
    const page = context.pages()[0] ?? (await context.newPage());
    page.setDefaultNavigationTimeout(300_000);
    await page.addInitScript(installBootProbe);
    const load = +loadavg()[0]!.toFixed(1);
    const cold = await measureLoad(page, () => page.goto(options.baseUrl, { waitUntil: "commit" }), 300_000, options.signal);
    const warm = await measureLoad(page, () => page.reload({ waitUntil: "commit" }), 120_000, options.signal);
    return { load: Math.max(load, +loadavg()[0]!.toFixed(1)), cores: cpus().length, serveReadyMs: options.serveReadyMs, cold, warm };
  } finally {
    options.signal.removeEventListener("abort", abort);
    await context.close().catch(() => undefined);
    rmSync(profile, { recursive: true, force: true });
  }
}
//#endregion ⏱️Measure

//#region 🚪️Cli
function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

const payloadLine = (payload: Readonly<Record<BootResourceKindV1, BootPayloadV1>>): string => BOOT_RESOURCE_KINDS_V1.filter((kind) => payload[kind].count > 0).map((kind) => `${kind} ${payload[kind].count}/${payload[kind].decodedMB} MB`).join(", ");

/** 🚪️ `verify boot --serve <url> [--hub <url>] [--locale en|de] [--tag <t>] [--out <dir>] [--headed]` — boots the shell the
 * serve `--serve` names (reused, or started by the shared `ensureDevServe` fixture, joined to `--hub` when given, and
 * stopped again), writes `<out>/<tag>/boot.json`, publishes the acceptance check `boot-budget` (en + de; `blocked` when a
 * precondition is missing or the timings could not be judged on a saturated machine) and exits non-zero on a failure.
 * SIGINT/SIGTERM cancel. */
export async function runBootBudgetCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const serveUrl = flagValue(segments, "--serve");
  if (!serveUrl) throw new Error("usage: verify boot --serve <url> [--hub <url>] [--locale en|de] [--tag <t>] [--out <dir>] [--headed]");
  const hubUrl = flagValue(segments, "--hub");
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  try {
    await withAcceptanceRecord(repoRoot, "boot-budget", async () => {
      const tag = flagValue(segments, "--tag") ?? "boot-budget";
      const outDir = join(resolve(flagValue(segments, "--out") ?? defaultOutDir), tag);
      mkdirSync(outDir, { recursive: true });
      const fixture = readBootBudgetFixtureV1();
      let serveReadyMs: number | null = null;
      const serve: DevServeFixtureV1 | Error = await ensureDevServe({
        repoRoot,
        port: devServePortV1(serveUrl),
        hubUrl,
        locale,
        signal: controller.signal,
        onProgress: (status, line) => {
          if (status.kind === "ready") serveReadyMs = status.waitedMs;
          console.log(line);
        },
      }).catch((error: unknown) => (error instanceof Error ? error : new Error(String(error))));
      if (serve instanceof Error) {
        const reason = serve.message.split("\n")[0]!.slice(0, 200);
        publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({ check: "boot-budget", status: "blocked", startedAt, measured: { serve: serveUrl, cancelled: controller.signal.aborted }, summary: { en: `no serve at ${serveUrl}: ${reason}`, de: `keine Shell unter ${serveUrl}: ${reason}` } }));
        process.exitCode = 1;
        return;
      }
      try {
        const measured = await runBootBudget({ baseUrl: serve.url, serveReadyMs, headed: segments.includes("--headed"), signal: controller.signal });
        const verdict = controller.signal.aborted ? { status: "fail" as const, timingJudged: false, violations: ["cancelled"] } : judgeBootBudgetV1(fixture.budget, measured);
        const reportPath = join(outDir, "boot.json");
        writeFileSync(reportPath, `${JSON.stringify({ baseUrl: serve.url, reused: serve.reused, measured, verdict }, null, 1)}\n`);
        const timings = `serve ${measured.serveReadyMs === null ? "reused" : `${measured.serveReadyMs} ms`}, cold FCP ${measured.cold.fcpMs} / ready ${measured.cold.readyMs} / plugins ${measured.cold.pluginsLoadedMs} ms, warm ready ${measured.warm.readyMs} ms, load ${measured.load}`;
        const verdictLine = verdict.violations.length > 0 ? `; ${verdict.violations.join("; ")}` : "";
        console.log(`[boot-budget] ${verdict.status.toUpperCase()} ${timings}; cold payload ${payloadLine(measured.cold.payload)}${verdictLine}`);
        publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({
          check: "boot-budget",
          status: verdict.status,
          startedAt,
          measured: {
            load: measured.load,
            timingJudged: verdict.timingJudged,
            serveReadyMs: measured.serveReadyMs ?? -1,
            coldFcpMs: measured.cold.fcpMs ?? -1,
            coldReadyMs: measured.cold.readyMs ?? -1,
            coldPluginsLoadedMs: measured.cold.pluginsLoadedMs ?? -1,
            warmReadyMs: measured.warm.readyMs ?? -1,
            ...Object.fromEntries(BOOT_RESOURCE_KINDS_V1.flatMap((kind) => [[`cold.${kind}.count`, measured.cold.payload[kind].count], [`cold.${kind}.decodedMB`, measured.cold.payload[kind].decodedMB]])),
          },
          summary: {
            en: `${verdict.status === "blocked" ? "payload within budget, timings not judged (load)" : verdict.status}: ${timings}${verdictLine}`,
            de: `${verdict.status === "blocked" ? "Nutzlast im Budget, Zeiten nicht bewertet (Last)" : verdict.status === "pass" ? "bestanden" : "fehlgeschlagen"}: Shell ${measured.serveReadyMs === null ? "wiederverwendet" : `${measured.serveReadyMs} ms`}, kalt FCP ${measured.cold.fcpMs} / bereit ${measured.cold.readyMs} / Plugins ${measured.cold.pluginsLoadedMs} ms, warm bereit ${measured.warm.readyMs} ms, Last ${measured.load}${verdict.violations.length > 0 ? `; ${verdict.violations.length} Verstöße` : ""}`,
          },
          evidence: [reportPath],
        }));
        if (verdict.status === "fail") process.exitCode = 1;
      } finally {
        await serve.stop();
      }
    });
  } finally {
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
}
//#endregion 🚪️Cli

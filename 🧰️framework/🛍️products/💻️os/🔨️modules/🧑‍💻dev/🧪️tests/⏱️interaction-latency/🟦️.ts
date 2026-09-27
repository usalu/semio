/** ⏱️ Interaction latency of the served `s` React shell: typing in the text editors and dragging on the canvases, measured
 * the way the user perceives it.
 *
 * Per scenario of `🧑‍💻dev/🧫️fixtures/⏱️interaction-latency.json`: the program opened from Home, its target focused, then N
 * trusted inputs (Playwright keyboard/mouse through CDP) at the scenario's interval. Latency of one input = from the event's own
 * timestamp to the end of the first frame after it (a capture-phase listener arms `requestAnimationFrame` → `MessageChannel`,
 * which runs after that frame's rAF work and paint). Paints = `renderFrame` calls of every canvas session class the page
 * loaded — the wasm bindings' exported classes and the host elements' own (the 2D canvas host paints in JS) — whose prototypes
 * are wrapped; a scenario that hooks no class fails, since an unmeasured paint count is not a pass. The browser's own Event Timing entries (≥ 16 ms) are the third-party
 * oracle for the input's processing cost.
 * Law: paints per input ≤ the scenario's bound (demand-driven painting, always judged); p95 input → frame ≤ the scenario's
 * bound, judged only while the machine's 1-minute load stays ≤ `loadCeilingPerCore` × cores (else the check is `blocked` with
 * the measured values — a timing measured on a saturated machine is not a verdict).
 *
 * Promoted from the ticket harness `wp-f1/f1-latency.mjs` (ticket 26/09/23 F1: jack keydown → frame p50 217 → 11 ms) by F2.
 */

import { cpus, loadavg } from "node:os";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
import { awaitBeacon, dismissIntroduction, kindOf, windowIds } from "../🧮️program-matrix/🟦️.ts";

//#region 🧾️Scenarios
/** 🎬️ One scenario of the fixture. */
export type LatencyScenario = Readonly<{ id: string; pluginId: string; appId: string; window: string; target: string; action: "type" | "drag"; inputs: number; intervalMs: number; maxP95Ms: number; maxPaintsPerInput: number }>;

/** 🎬️ The fixture: scenarios and the load ceiling above which timings are not judged. */
export type LatencyScenarios = Readonly<{ schema: "semio.os-dev.interaction-latency-scenarios/v1"; loadCeilingPerCore: number; scenarios: readonly LatencyScenario[] }>;

/** 📖️ Reads `🧑‍💻dev/🧫️fixtures/⏱️interaction-latency.json`. */
export function readLatencyScenarios(): LatencyScenarios {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "🧫️fixtures", "⏱️interaction-latency.json"), "utf8")) as LatencyScenarios;
  if (fixture.schema !== "semio.os-dev.interaction-latency-scenarios/v1") throw new Error(`interaction latency: unexpected schema ${String(fixture.schema)}`);
  return fixture;
}

/** 📊️ The quantile of `values` (nearest rank). */
export function quantileV1(values: readonly number[], q: number): number | null {
  if (values.length === 0) return null;
  const sorted = [...values].sort((left, right) => left - right);
  return +sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))]!.toFixed(1);
}
//#endregion 🧾️Scenarios

//#region ⏱️Measure
/** 🧾️ One scenario's measurement and verdict. */
export type LatencyRow = Readonly<{
  id: string;
  opened: boolean;
  inputs: number;
  frames: Readonly<{ n: number; p50: number | null; p95: number | null; max: number | null }>;
  paints: Readonly<{ n: number; perInput: number; p50Ms: number | null; p95Ms: number | null; classes: readonly string[] }>;
  eventTiming: Readonly<Record<string, { n: number; p95: number | null }>>;
  load: number;
  timingJudged: boolean;
  status: "pass" | "fail" | "blocked";
  violations: readonly string[];
}>;

/** 🗃️ Resource Timing entries the page keeps. The browser default (250) is full long before a program opens — the s boot alone
 * loads ~500 scripts — so the paint hook, which finds the wasm canvas session modules through those entries, found none and
 * every scenario reported 0 paints per input (ticket 26/09/23 F2). */
const RESOURCE_TIMING_ENTRIES = 100_000;

const installProbe = (resourceTimingEntries: number): void => {
  const state = { rows: [] as { t0: number; ms: number }[], events: [] as { name: string; duration: number }[], paints: [] as number[], classes: new Set<string>(), armed: false };
  Object.defineProperty(window, "__semioLatency", { value: state });
  performance.setResourceTimingBufferSize(resourceTimingEntries);
  for (const type of ["keydown", "pointermove", "pointerdown", "pointerup"]) {
    addEventListener(
      type,
      (event) => {
        if (!event.isTrusted || !state.armed) return;
        const t0 = event.timeStamp;
        requestAnimationFrame(() => {
          const channel = new MessageChannel();
          channel.port1.onmessage = () => state.rows.push({ t0, ms: performance.now() - t0 });
          channel.port2.postMessage(0);
        });
      },
      { capture: true, passive: true },
    );
  }
  new PerformanceObserver((list) => {
    if (!state.armed) return;
    for (const entry of list.getEntries()) state.events.push({ name: entry.name, duration: entry.duration });
  }).observe({ type: "event", durationThreshold: 16, buffered: false } as PerformanceObserverInit);
};

async function hookPaints(page: Page): Promise<readonly string[]> {
  return page.evaluate(async () => {
    const state = (window as unknown as { __semioLatency: { paints: number[]; classes: Set<string>; armed: boolean } }).__semioLatency;
    const sessionModule = (url: string): boolean => {
      const path = decodeURIComponent(url);
      if (/worker/iu.test(path)) return false;
      return (/\.js(\?|$)/u.test(path) && /\/pkg\/|bindings\//u.test(path)) || /🧱️elements\/[^?]*\.tsx?(\?|$)/u.test(path);
    };
    const urls = [...new Set(performance.getEntriesByType("resource").map((entry) => entry.name).filter(sessionModule))];
    const hooked: string[] = [];
    for (const url of urls) {
      let module: Record<string, unknown>;
      try {
        module = (await import(url)) as Record<string, unknown>;
      } catch {
        continue;
      }
      for (const [name, value] of Object.entries(module)) {
        const prototype = (value as { prototype?: { renderFrame?: (...args: unknown[]) => unknown; __semioLatencyHooked?: boolean } } | null)?.prototype;
        if (typeof value !== "function" || typeof prototype?.renderFrame !== "function" || prototype.__semioLatencyHooked) continue;
        const original = prototype.renderFrame;
        prototype.renderFrame = function (this: unknown, ...args: unknown[]) {
          const t0 = performance.now();
          try {
            return original.apply(this, args);
          } finally {
            if (state.armed) {
              state.paints.push(performance.now() - t0);
              state.classes.add(name);
            }
          }
        };
        prototype.__semioLatencyHooked = true;
        hooked.push(name);
      }
    }
    return hooked;
  });
}

async function openProgram(page: Page, scenario: LatencyScenario): Promise<string[]> {
  const before = await windowIds(page);
  await dismissIntroduction(page);
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    document.body.focus();
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  if ((await input.count()) === 0) return [];
  await input.fill(kindOf(scenario.appId));
  await page.waitForTimeout(1_200);
  for (const id of [`spawn.${scenario.pluginId}.${scenario.appId}`, `spawn.${scenario.pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) === 0) continue;
    await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
    const deadline = Date.now() + 90_000;
    while (Date.now() < deadline) {
      const fresh = (await windowIds(page)).filter((windowId) => !before.includes(windowId));
      if (fresh.length > 0) {
        await page.waitForTimeout(5_000);
        return fresh;
      }
      await page.waitForTimeout(250);
    }
  }
  await page.keyboard.press("Escape");
  return [];
}

async function measureScenario(page: Page, scenario: LatencyScenario, loadCeiling: number): Promise<LatencyRow> {
  const load = +loadavg()[0]!.toFixed(1);
  const empty: LatencyRow = { id: scenario.id, opened: false, inputs: 0, frames: { n: 0, p50: null, p95: null, max: null }, paints: { n: 0, perInput: 0, p50Ms: null, p95Ms: null, classes: [] }, eventTiming: {}, load, timingJudged: false, status: "fail", violations: ["the program did not open"] };
  const opened = await openProgram(page, scenario);
  if (opened.length === 0) return empty;
  const windowSelector = scenario.window ? `[id$="${scenario.window}"]` : `[id="${opened[0]}"]`;
  const target = page.locator(`${windowSelector} [data-slot="window-body"] ${scenario.target}`).first();
  if (!(await target.waitFor({ state: "visible", timeout: 30_000 }).then(() => true, () => false))) return { ...empty, opened: true, violations: [`no visible ${scenario.target} in ${windowSelector}`] };
  const box = (await target.boundingBox())!;
  const x = box.x + box.width * 0.5;
  const y = box.y + box.height * 0.4;
  await page.mouse.click(x, y);
  await page.waitForTimeout(800);
  if (scenario.action === "type") await page.keyboard.press("End");
  const hooked = await hookPaints(page);
  await page.waitForTimeout(1_500);
  await page.evaluate(() => {
    const state = (window as unknown as { __semioLatency: { rows: unknown[]; events: unknown[]; paints: unknown[]; classes: Set<string>; armed: boolean } }).__semioLatency;
    state.rows.length = 0;
    state.events.length = 0;
    state.paints.length = 0;
    state.classes.clear();
    state.armed = true;
  });
  if (scenario.action === "type") {
    for (let index = 0; index < scenario.inputs; index++) {
      await page.keyboard.press(index % 2 === 0 ? "a" : "Backspace");
      await page.waitForTimeout(scenario.intervalMs);
    }
  } else {
    await page.mouse.move(x, y);
    await page.mouse.down();
    for (let index = 1; index <= scenario.inputs; index++) {
      await page.mouse.move(x + 3 * index, y + 2 * index);
      await page.waitForTimeout(scenario.intervalMs);
    }
    await page.mouse.up();
  }
  await page.waitForTimeout(1_000);
  const state = await page.evaluate(() => {
    const probe = (window as unknown as { __semioLatency: { rows: { ms: number }[]; events: { name: string; duration: number }[]; paints: number[]; classes: Set<string>; armed: boolean } }).__semioLatency;
    probe.armed = false;
    return { frames: probe.rows.map((row) => row.ms), events: probe.events, paints: probe.paints, classes: [...probe.classes] };
  });
  const inputs = scenario.action === "type" ? scenario.inputs : scenario.inputs + 2;
  const byEvent: Record<string, number[]> = {};
  for (const entry of state.events) (byEvent[entry.name] ??= []).push(entry.duration);
  const perInput = +(state.paints.length / inputs).toFixed(2);
  const frames = { n: state.frames.length, p50: quantileV1(state.frames, 0.5), p95: quantileV1(state.frames, 0.95), max: quantileV1(state.frames, 1) };
  const timingJudged = load <= loadCeiling;
  const violations = [
    ...(hooked.length > 0 ? [] : ["no canvas session class was hooked, so paints went unmeasured"]),
    ...(state.frames.length >= Math.floor(inputs * 0.8) ? [] : [`only ${state.frames.length} of ${inputs} inputs reached a frame`]),
    ...(perInput <= scenario.maxPaintsPerInput ? [] : [`${perInput} paints per input (bound ${scenario.maxPaintsPerInput})`]),
    ...(timingJudged && frames.p95 !== null && frames.p95 > scenario.maxP95Ms ? [`input → frame p95 ${frames.p95} ms (bound ${scenario.maxP95Ms} ms)`] : []),
  ];
  return {
    id: scenario.id,
    opened: true,
    inputs,
    frames,
    paints: { n: state.paints.length, perInput, p50Ms: quantileV1(state.paints, 0.5), p95Ms: quantileV1(state.paints, 0.95), classes: state.classes },
    eventTiming: Object.fromEntries(Object.entries(byEvent).map(([name, values]) => [name, { n: values.length, p95: quantileV1(values, 0.95) }])),
    load,
    timingJudged,
    status: violations.length > 0 ? "fail" : timingJudged ? "pass" : "blocked",
    violations,
  };
}

/** ⏱️ Runs every scenario (optionally `only` some ids), one fresh page per scenario. */
export async function runInteractionLatency(options: Readonly<{ baseUrl: string; only: readonly string[]; headed: boolean; signal: AbortSignal; onRow: (row: LatencyRow) => void }>): Promise<readonly LatencyRow[]> {
  const fixture = readLatencyScenarios();
  const loadCeiling = fixture.loadCeilingPerCore * cpus().length;
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser: Browser = await chromium.launch({ headless: !options.headed, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
  const abort = (): void => void browser.close().catch(() => undefined);
  options.signal.addEventListener("abort", abort, { once: true });
  const rows: LatencyRow[] = [];
  try {
    for (const scenario of fixture.scenarios.filter((row) => options.only.length === 0 || options.only.includes(row.id))) {
      if (options.signal.aborted) break;
      const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
      page.setDefaultNavigationTimeout(300_000);
      await page.addInitScript(installProbe, RESOURCE_TIMING_ENTRIES);
      try {
        await page.goto(options.baseUrl, { waitUntil: "commit" });
        const beacon = await awaitBeacon(page, Date.now() + 300_000);
        if (!beacon?.startsWith("ready:")) throw new Error(`the shell never became ready (${beacon ?? "no beacon"})`);
        await dismissIntroduction(page);
        await page.keyboard.press("Escape").catch(() => undefined);
        await page.waitForTimeout(3_000);
        const row = await measureScenario(page, scenario, loadCeiling);
        rows.push(row);
        options.onRow(row);
      } catch (error) {
        const row: LatencyRow = { id: scenario.id, opened: false, inputs: 0, frames: { n: 0, p50: null, p95: null, max: null }, paints: { n: 0, perInput: 0, p50Ms: null, p95Ms: null, classes: [] }, eventTiming: {}, load: +loadavg()[0]!.toFixed(1), timingJudged: false, status: "fail", violations: [`measurement failed: ${error instanceof Error ? error.message : String(error)}`.slice(0, 300)] };
        rows.push(row);
        options.onRow(row);
      } finally {
        await page.close().catch(() => undefined);
      }
    }
  } finally {
    options.signal.removeEventListener("abort", abort);
    await browser.close().catch(() => undefined);
  }
  return rows;
}
//#endregion ⏱️Measure

//#region 🚪️Cli
function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `verify latency <serveUrl> [--tag <t>] [--only <scenario>,…] [--out <dir>] [--headed]` — writes
 * `<out>/<tag>/latency.json`, publishes the acceptance check `interaction-latency` (`blocked` when every failure-free row could
 * not be timed on a saturated machine), exits non-zero on a failure. SIGINT/SIGTERM cancel. */
export async function runInteractionLatencyCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const valued = new Set(["--tag", "--only", "--out"]);
  const baseUrl = segments.find((segment, index) => !segment.startsWith("--") && !valued.has(segments[index - 1] ?? ""));
  if (!baseUrl) throw new Error("usage: verify latency <serveUrl> [--tag <t>] [--only <scenario>,…] [--out <dir>] [--headed]");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  try {
    const tag = flagValue(segments, "--tag") ?? "interaction-latency";
    const outDir = join(resolve(flagValue(segments, "--out") ?? defaultOutDir), tag);
    mkdirSync(outDir, { recursive: true });
    const reportPath = join(outDir, "latency.json");
    const rows: LatencyRow[] = [];
    await runInteractionLatency({ baseUrl, only: (flagValue(segments, "--only") ?? "").split(",").filter(Boolean), headed: segments.includes("--headed"), signal: controller.signal, onRow: (row) => {
      rows.push(row);
      writeFileSync(reportPath, `${JSON.stringify({ baseUrl, rows }, null, 1)}\n`);
      console.log(`[interaction-latency] ${row.status.toUpperCase()} ${row.id}: input → frame p50 ${row.frames.p50} / p95 ${row.frames.p95} ms (n ${row.frames.n}), ${row.paints.perInput} paints/input (paint p50 ${row.paints.p50Ms} ms), load ${row.load}${row.violations.length ? ` — ${row.violations.join("; ")}` : ""}`);
    } });
    const failed = rows.filter((row) => row.status === "fail");
    const blocked = rows.filter((row) => row.status === "blocked");
    const status = rows.length === 0 || failed.length > 0 || controller.signal.aborted ? "fail" : blocked.length > 0 ? "blocked" : "pass";
    const line = (row: LatencyRow): string => `${row.id} p95 ${row.frames.p95} ms ${row.paints.perInput}/input`;
    publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({
      check: "interaction-latency",
      status,
      startedAt,
      measured: Object.fromEntries(rows.flatMap((row) => [[`${row.id}.p95Ms`, row.frames.p95 ?? -1], [`${row.id}.paintsPerInput`, row.paints.perInput]])),
      summary: {
        en: `${rows.length - failed.length - blocked.length} pass, ${blocked.length} not timed (load), ${failed.length} fail: ${rows.map(line).join(", ")}`,
        de: `${rows.length - failed.length - blocked.length} bestanden, ${blocked.length} ohne Zeitmessung (Last), ${failed.length} fehlgeschlagen: ${rows.map(line).join(", ")}`,
      },
      evidence: [reportPath],
    }));
    if (status === "fail") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
}
//#endregion 🚪️Cli

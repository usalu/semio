/** 💤️ The idle budget of the served `s` React shell: every spawnable program, left alone, paints nothing, and a long session
 * with many windows does not grow.
 *
 * Idle census (`verify idle <serveUrl>`): per program of the shell's catalog probe (editor and/or viewer role), opened from the
 * Home landing by the palette chord, its bodies rendered, the main thread settled — then `--seconds` with NO input are traced
 * through Chromium's own tracing (CDP `Tracing`, the third-party oracle): main-thread frames (`BeginMainThreadFrame`),
 * animation-frame callbacks (`FireAnimationFrame`) and compositor draws (`DrawFrame`) per second, main-thread busy % (reported;
 * load-sensitive, not judged), then closed and the frames of the 4 s after closing counted (a loop that survives its window).
 * Law: < 1 main frame/s, < 1 animation frame/s, < 1 compositor draw/s while open and after closing.
 *
 * Memory soak (`--soak-minutes <m> --windows <n>`): opens the first `n` editors, then every minute forces a GC (CDP
 * `HeapProfiler.collectGarbage`) and samples the JS heap, DOM nodes, event listeners, the dedicated workers' heaps and the
 * idle frames. Law: from the end of the warm-up (a fifth of the soak) on, the post-GC JS heap grows ≤ 0.5 MB/min
 * (least-squares slope) and DOM nodes and listeners grow ≤ 1 %, frames stay < 1/s.
 *
 * Promoted from the ticket harnesses `wp-f1/f1-idle-census.mjs` + `f1-lib.mjs` (ticket 26/09/23 F1: 146 programs, 4 continuous
 * loops found and fixed) and F2's soak (ticket 26/09/23 F2).
 */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, CDPSession, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
import { awaitBeacon, dismissIntroduction, keyOf, kindOf, roleOf, windowIds, type MatrixProgram } from "../🧮️program-matrix/🟦️.ts";

//#region 🎞️Trace
/** 📏️ The idle law's bounds (per second) and the soak's growth bounds. */
export const IDLE_BUDGET_V1 = Object.freeze({ framesPerSec: 1, rafPerSec: 1, drawsPerSec: 1, heapSlopeMBPerMin: 0.5, domGrowthRatio: 0.01, listenerGrowthRatio: 0.01 });

const TRACE_CATEGORIES = ["toplevel", "devtools.timeline", "disabled-by-default-devtools.timeline.frame"].join(",");

/** 🎞️ One traced window reduced to what the idle law judges. */
export type IdleReading = Readonly<{ seconds: number; framesPerSec: number; rafPerSec: number; drawsPerSec: number; mainBusyPct: number; workersBusyPct: number }>;

type TraceEvent = Readonly<{ ph: string; name: string; pid: number; tid: number; ts?: number; dur?: number; args?: { readonly name?: string } }>;

/** 🧮️ Reduces trace events to the renderer's main thread (the busiest `CrRendererMain`), its compositor and its workers. */
export function reduceIdleTrace(events: readonly TraceEvent[], seconds: number): IdleReading {
  const threadNames = new Map<string, string>();
  for (const event of events) if (event.ph === "M" && event.name === "thread_name") threadNames.set(`${event.pid}:${event.tid}`, event.args?.name ?? "?");
  const threads = new Map<string, { counts: Map<string, number>; intervals: [number, number][] }>();
  for (const event of events) {
    if (event.ph === "M") continue;
    const key = `${event.pid}:${event.tid}`;
    let row = threads.get(key);
    if (row === undefined) {
      row = { counts: new Map(), intervals: [] };
      threads.set(key, row);
    }
    row.counts.set(event.name, (row.counts.get(event.name) ?? 0) + 1);
    if (event.ph === "X" && (event.name === "ThreadControllerImpl::RunTask" || event.name === "RunTask") && typeof event.ts === "number") row.intervals.push([event.ts, event.ts + (event.dur ?? 0)]);
  }
  const busy = (intervals: [number, number][]): number => {
    let total = 0;
    let end = -Infinity;
    for (const [start, stop] of [...intervals].sort((left, right) => left[0] - right[0])) {
      if (start >= end) total += stop - start;
      else if (stop > end) total += stop - end;
      end = Math.max(end, stop);
    }
    return total;
  };
  const named = (pattern: RegExp) => [...threads.entries()].filter(([key]) => pattern.test(threadNames.get(key) ?? ""));
  const main = named(/^CrRendererMain$/u).sort((left, right) => right[1].intervals.length - left[1].intervals.length)[0];
  const pid = main?.[0].split(":")[0];
  const sameProcess = (entries: ReturnType<typeof named>) => entries.filter(([key]) => key.split(":")[0] === pid);
  const compositor = sameProcess(named(/Compositor/u)).filter(([key]) => !/Tile|Raster/u.test(threadNames.get(key) ?? ""))[0];
  const viz = named(/VizCompositor/u);
  const workers = sameProcess(named(/DedicatedWorker/iu));
  const per = (value: number): number => +(value / seconds).toFixed(2);
  const count = (entry: (typeof threads extends Map<string, infer V> ? V : never) | undefined, name: string): number => entry?.counts.get(name) ?? 0;
  return {
    seconds,
    framesPerSec: per(count(main?.[1], "BeginMainThreadFrame")),
    rafPerSec: per(count(main?.[1], "FireAnimationFrame")),
    drawsPerSec: per(count(compositor?.[1], "DrawFrame") + viz.reduce((sum, [, row]) => sum + count(row, "DrawFrame"), 0)),
    mainBusyPct: main ? +((100 * busy(main[1].intervals)) / (seconds * 1_000_000)).toFixed(2) : 0,
    workersBusyPct: +((100 * workers.reduce((sum, [, row]) => sum + busy(row.intervals), 0)) / (seconds * 1_000_000)).toFixed(2),
  };
}

async function traceIdle(cdp: CDPSession, seconds: number): Promise<IdleReading> {
  const chunks: string[] = [];
  const complete = new Promise<{ stream?: string }>((resolveTrace) => cdp.once("Tracing.tracingComplete", (event) => resolveTrace(event as { stream?: string })));
  await cdp.send("Tracing.start", { categories: TRACE_CATEGORIES, transferMode: "ReturnAsStream", streamFormat: "json" });
  await new Promise((wait) => setTimeout(wait, seconds * 1000));
  await cdp.send("Tracing.end");
  const { stream } = await complete;
  if (!stream) throw new Error("idle trace: no stream");
  for (;;) {
    const read = (await cdp.send("IO.read", { handle: stream, size: 4 << 20 })) as { data: string; eof: boolean; base64Encoded?: boolean };
    chunks.push(read.base64Encoded ? Buffer.from(read.data, "base64").toString("utf8") : read.data);
    if (read.eof) break;
  }
  await cdp.send("IO.close", { handle: stream });
  const parsed = JSON.parse(chunks.join("")) as TraceEvent[] | { traceEvents: TraceEvent[] };
  return reduceIdleTrace(Array.isArray(parsed) ? parsed : parsed.traceEvents, seconds);
}

async function settleMain(cdp: CDPSession, budgetMs: number): Promise<boolean> {
  const taskSeconds = async (): Promise<number> => ((await cdp.send("Performance.getMetrics")) as { metrics: { name: string; value: number }[] }).metrics.find((row) => row.name === "TaskDuration")?.value ?? 0;
  const started = Date.now();
  let quiet = 0;
  let last = await taskSeconds();
  while (Date.now() - started < budgetMs) {
    await new Promise((wait) => setTimeout(wait, 1_000));
    const now = await taskSeconds();
    quiet = now - last < 0.05 ? quiet + 1 : 0;
    last = now;
    if (quiet >= 3) return true;
  }
  return false;
}
//#endregion 🎞️Trace

//#region 🧭️Shell
type Shell = { readonly browser: Browser; readonly page: Page; readonly cdp: CDPSession; readonly workerHeaps: () => Promise<number>; programs: readonly MatrixProgram[] };

/** ⏱️ Settles with `work` or rejects after `ms` — a hung browser (its process gone, its protocol promise never settling) must
 * never stall a census for hours (measured: 3 h on one dead Chromium, ticket 26/09/23 F2). */
function withDeadline<T>(work: Promise<T>, ms: number, label: string): Promise<T> {
  work.catch(() => undefined);
  let timer: ReturnType<typeof setTimeout> | undefined;
  return Promise.race([work, new Promise<T>((_resolve, reject) => { timer = setTimeout(() => reject(new Error(`${label}: no answer within ${ms / 1000} s`)), ms); })]).finally(() => clearTimeout(timer));
}

/** 🚀️ Boots a fresh shell, up to three attempts, each bounded. */
async function bootShellBounded(baseUrl: string, headed: boolean): Promise<Shell> {
  let failure: unknown = null;
  for (let attempt = 1; attempt <= 3; attempt++) {
    const booting = bootShell(baseUrl, headed);
    try {
      return await withDeadline(booting, IDLE_BOOT_DEADLINE_MS, `shell boot attempt ${attempt}`);
    } catch (error) {
      failure = error;
      void booting.then((shell) => shell.browser.close(), () => undefined);
    }
  }
  throw failure instanceof Error ? failure : new Error(String(failure));
}

async function bootShell(baseUrl: string, headed: boolean): Promise<Shell> {
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: !headed, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  page.setDefaultNavigationTimeout(300_000);
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Performance.enable", { timeDomain: "timeTicks" });
  const sessions = new Set<string>();
  cdp.on("Target.attachedToTarget", (event) => {
    const attached = event as { sessionId: string; targetInfo: { type: string } };
    if (attached.targetInfo.type === "worker") sessions.add(attached.sessionId);
  });
  cdp.on("Target.detachedFromTarget", (event) => sessions.delete((event as { sessionId: string }).sessionId));
  const pending = new Map<number, (value: number) => void>();
  cdp.on("Target.receivedMessageFromTarget", (event) => {
    const message = JSON.parse((event as { message: string }).message) as { id: number; result?: { usedSize: number } };
    pending.get(message.id)?.(message.result?.usedSize ?? 0);
    pending.delete(message.id);
  });
  await cdp.send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: false, flatten: false });
  let nextId = 1;
  const workerHeaps = async (): Promise<number> => {
    let total = 0;
    for (const sessionId of sessions) {
      const id = nextId++;
      total += await new Promise<number>((resolveHeap) => {
        pending.set(id, resolveHeap);
        cdp.send("Target.sendMessageToTarget", { sessionId, message: JSON.stringify({ id, method: "Runtime.getHeapUsage" }) }).catch(() => resolveHeap(0));
        setTimeout(() => {
          if (pending.delete(id)) resolveHeap(0);
        }, 5_000);
      });
    }
    return total;
  };
  await page.goto(baseUrl, { waitUntil: "commit" });
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  if (!beacon?.startsWith("ready:")) throw new Error(`the shell never became ready (${beacon ?? "no beacon"})`);
  await dismissIntroduction(page);
  await page.keyboard.press("Escape").catch(() => undefined);
  await settleMain(cdp, 20_000);
  const probe = (await page.evaluate(() => (window as unknown as { __semioOsCatalogProbe?: { programs?: MatrixProgram[] } }).__semioOsCatalogProbe?.programs ?? [])) as MatrixProgram[];
  return { browser, page, cdp, workerHeaps, programs: probe };
}

async function openProgram(shell: Shell, program: MatrixProgram): Promise<string[]> {
  const { page } = shell;
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
  await input.fill(kindOf(program.appId));
  await page.waitForTimeout(1_200);
  const ids = [`spawn.${program.pluginId}.${program.appId}`];
  if (shell.programs.find((entry) => entry.pluginId === program.pluginId)?.appId === program.appId) ids.push(`spawn.${program.pluginId}`);
  for (const id of ids) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) === 0) continue;
    await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
    const deadline = Date.now() + 90_000;
    while (Date.now() < deadline) {
      const fresh = (await windowIds(page)).filter((windowId) => !before.includes(windowId));
      if (fresh.length > 0) {
        const rendered = Date.now() + 60_000;
        while (Date.now() < rendered && (await page.evaluate((wanted) => wanted.some((windowId) => document.getElementById(windowId)?.querySelector('[data-slot="window-body"] [aria-busy="true"]') !== null), fresh))) await page.waitForTimeout(500);
        return fresh;
      }
      await page.waitForTimeout(250);
    }
    return [];
  }
  await page.keyboard.press("Escape");
  return [];
}

async function closeWindows(page: Page, ids: readonly string[]): Promise<void> {
  for (const id of ids) {
    await page.evaluate((windowId) => {
      const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId);
      const button = tab?.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab?.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]');
      if (button instanceof HTMLElement) button.click();
    }, id);
    await page.waitForTimeout(400);
  }
  await page.waitForTimeout(1_500);
}
//#endregion 🧭️Shell

//#region 💤️Census
/** 🧾️ One program's idle row. */
export type IdleRow = Readonly<{ key: string; role: string; appId: string; windows: number; settled: boolean; open: IdleReading | null; afterClose: IdleReading | null; pass: boolean; detail: string | null }>;

/** ⏱️ A row that yields no verdict within this bound is recorded as such and the browser is restarted. */
const IDLE_ROW_WATCHDOG_MS = 240_000;

/** ⏱️ One shell boot (launch, load, beacon, settle) must finish within this bound. */
const IDLE_BOOT_DEADLINE_MS = 360_000;

const within = (reading: IdleReading | null): boolean => reading !== null && reading.framesPerSec < IDLE_BUDGET_V1.framesPerSec && reading.rafPerSec < IDLE_BUDGET_V1.rafPerSec && reading.drawsPerSec < IDLE_BUDGET_V1.drawsPerSec;

/** 💤️ Runs the idle census over every program of `roles` (optionally narrowed by `only` = plugin ids or plugin/kind keys). */
export async function runIdleCensus(options: Readonly<{ baseUrl: string; roles: readonly string[]; only: readonly string[]; done: ReadonlySet<string>; seconds: number; headed: boolean; signal: AbortSignal; onRow: (row: IdleRow) => void }>): Promise<readonly IdleRow[]> {
  const rows: IdleRow[] = [];
  let shell = await bootShellBounded(options.baseUrl, options.headed);
  const abort = (): void => void shell.browser.close().catch(() => undefined);
  options.signal.addEventListener("abort", abort, { once: true });
  try {
    const wanted = shell.programs.filter((program) => options.roles.includes(roleOf(program.appId)) && (options.only.length === 0 || options.only.includes(program.pluginId) || options.only.includes(keyOf(program))) && !options.done.has(`${keyOf(program)}#${roleOf(program.appId)}`));
    for (const program of wanted) {
      if (options.signal.aborted) break;
      const measure = async (): Promise<IdleRow> => {
        const opened = await openProgram(shell, program);
        if (opened.length === 0) return { key: keyOf(program), role: roleOf(program.appId), appId: program.appId, windows: 0, settled: false, open: null, afterClose: null, pass: false, detail: "did not open" };
        const settled = await settleMain(shell.cdp, 30_000);
        const open = await traceIdle(shell.cdp, options.seconds);
        await closeWindows(shell.page, opened);
        const afterClose = await traceIdle(shell.cdp, 4);
        const pass = within(open) && within(afterClose);
        return { key: keyOf(program), role: roleOf(program.appId), appId: program.appId, windows: opened.length, settled, open, afterClose, pass, detail: pass ? null : `open ${JSON.stringify(open)} after close ${JSON.stringify(afterClose)}` };
      };
      let watchdog: ReturnType<typeof setTimeout> | undefined;
      const measuring = measure();
      measuring.catch(() => undefined);
      const row = await Promise.race([
        measuring,
        new Promise<IdleRow>((resolveRow) => {
          watchdog = setTimeout(() => resolveRow({ key: keyOf(program), role: roleOf(program.appId), appId: program.appId, windows: 0, settled: false, open: null, afterClose: null, pass: false, detail: `no verdict within ${IDLE_ROW_WATCHDOG_MS / 1000} s` }), IDLE_ROW_WATCHDOG_MS);
        }),
      ]).catch((error: unknown): IdleRow => ({ key: keyOf(program), role: roleOf(program.appId), appId: program.appId, windows: 0, settled: false, open: null, afterClose: null, pass: false, detail: `measurement failed: ${error instanceof Error ? error.message : String(error)}`.slice(0, 400) }));
      clearTimeout(watchdog);
      rows.push(row);
      options.onRow(row);
      const healthy = row.open !== null && (await withDeadline(windowIds(shell.page), 30_000, "health probe").then(() => true, () => false));
      if (!healthy) {
        await withDeadline(shell.browser.close(), 30_000, "browser close").catch(() => undefined);
        shell = await bootShellBounded(options.baseUrl, options.headed);
        continue;
      }
      const remaining = (await withDeadline(windowIds(shell.page), 30_000, "window census").catch(() => [] as string[])).filter((windowId) => windowId !== "s-home-main");
      if (remaining.length > 0) await withDeadline(closeWindows(shell.page, remaining), 60_000, "close leftovers").catch(() => undefined);
    }
  } finally {
    options.signal.removeEventListener("abort", abort);
    await shell.browser.close().catch(() => undefined);
  }
  return rows;
}
//#endregion 💤️Census

//#region 🫧️Soak
/** 🧾️ One soak sample (post-GC). */
export type SoakSample = Readonly<{ minute: number; jsHeapMB: number; workersHeapMB: number; nodes: number; listeners: number; idle: IdleReading }>;

/** 📈️ Least-squares slope of `values` over `minutes`. */
export function slopePerMinute(minutes: readonly number[], values: readonly number[]): number {
  const count = minutes.length;
  if (count < 2) return 0;
  const meanX = minutes.reduce((sum, value) => sum + value, 0) / count;
  const meanY = values.reduce((sum, value) => sum + value, 0) / count;
  let numerator = 0;
  let denominator = 0;
  for (let index = 0; index < count; index++) {
    numerator += (minutes[index]! - meanX) * (values[index]! - meanY);
    denominator += (minutes[index]! - meanX) ** 2;
  }
  return denominator === 0 ? 0 : numerator / denominator;
}

/** 🫧️ Opens the first `windows` editors and samples memory and idle frames every minute for `minutes`. */
export async function runMemorySoak(options: Readonly<{ baseUrl: string; minutes: number; windows: number; only: readonly string[]; headed: boolean; signal: AbortSignal; onSample: (sample: SoakSample) => void }>): Promise<{ readonly opened: readonly string[]; readonly samples: readonly SoakSample[] }> {
  const shell = await bootShellBounded(options.baseUrl, options.headed);
  const abort = (): void => void shell.browser.close().catch(() => undefined);
  options.signal.addEventListener("abort", abort, { once: true });
  try {
    const seen = new Set<string>();
    const editors = shell.programs.filter((program) => roleOf(program.appId) === "editor" && (options.only.length === 0 || options.only.includes(program.pluginId)) && !seen.has(program.pluginId) && seen.add(program.pluginId));
    const opened: string[] = [];
    for (const program of editors) {
      if (opened.length >= options.windows || options.signal.aborted) break;
      if ((await openProgram(shell, program)).length > 0) opened.push(keyOf(program));
    }
    await settleMain(shell.cdp, 60_000);
    const samples: SoakSample[] = [];
    const started = Date.now();
    for (let minute = 0; minute <= options.minutes && !options.signal.aborted; minute++) {
      const due = started + minute * 60_000;
      if (Date.now() < due) await new Promise((wait) => setTimeout(wait, due - Date.now()));
      await shell.cdp.send("HeapProfiler.collectGarbage").catch(() => undefined);
      const metrics = Object.fromEntries(((await shell.cdp.send("Performance.getMetrics")) as { metrics: { name: string; value: number }[] }).metrics.map((row) => [row.name, row.value]));
      const idle = await traceIdle(shell.cdp, 5);
      const sample: SoakSample = { minute, jsHeapMB: +((metrics.JSHeapUsedSize ?? 0) / 2 ** 20).toFixed(2), workersHeapMB: +((await shell.workerHeaps()) / 2 ** 20).toFixed(2), nodes: metrics.Nodes ?? 0, listeners: metrics.JSEventListeners ?? 0, idle };
      samples.push(sample);
      options.onSample(sample);
    }
    return { opened, samples };
  } finally {
    options.signal.removeEventListener("abort", abort);
    await shell.browser.close().catch(() => undefined);
  }
}

/** ⚖️ The soak law over the samples after the warm-up (a fifth of the soak). */
export function judgeSoak(samples: readonly SoakSample[]): Readonly<{ heapSlopeMBPerMin: number; workersSlopeMBPerMin: number; domGrowthRatio: number; listenerGrowthRatio: number; maxFramesPerSec: number; violations: readonly string[] }> {
  const warm = samples.filter((sample) => sample.minute >= Math.ceil((samples.at(-1)?.minute ?? 0) / 5));
  const minutes = warm.map((sample) => sample.minute);
  const heapSlopeMBPerMin = +slopePerMinute(minutes, warm.map((sample) => sample.jsHeapMB)).toFixed(3);
  const workersSlopeMBPerMin = +slopePerMinute(minutes, warm.map((sample) => sample.workersHeapMB)).toFixed(3);
  const growth = (first: number, last: number): number => (first <= 0 ? 0 : +((last - first) / first).toFixed(4));
  const domGrowthRatio = warm.length < 2 ? 0 : growth(warm[0]!.nodes, warm.at(-1)!.nodes);
  const listenerGrowthRatio = warm.length < 2 ? 0 : growth(warm[0]!.listeners, warm.at(-1)!.listeners);
  const maxFramesPerSec = Math.max(0, ...samples.map((sample) => Math.max(sample.idle.framesPerSec, sample.idle.rafPerSec, sample.idle.drawsPerSec)));
  const violations = [
    ...(warm.length >= 2 ? [] : ["fewer than two samples after the warm-up"]),
    ...(heapSlopeMBPerMin <= IDLE_BUDGET_V1.heapSlopeMBPerMin ? [] : [`JS heap grows ${heapSlopeMBPerMin} MB/min`]),
    ...(workersSlopeMBPerMin <= IDLE_BUDGET_V1.heapSlopeMBPerMin ? [] : [`worker heaps grow ${workersSlopeMBPerMin} MB/min`]),
    ...(domGrowthRatio <= IDLE_BUDGET_V1.domGrowthRatio ? [] : [`DOM nodes grew ${(domGrowthRatio * 100).toFixed(1)} %`]),
    ...(listenerGrowthRatio <= IDLE_BUDGET_V1.listenerGrowthRatio ? [] : [`event listeners grew ${(listenerGrowthRatio * 100).toFixed(1)} %`]),
    ...(maxFramesPerSec < IDLE_BUDGET_V1.framesPerSec ? [] : [`${maxFramesPerSec} frames/s while idle`]),
  ];
  return { heapSlopeMBPerMin, workersSlopeMBPerMin, domGrowthRatio, listenerGrowthRatio, maxFramesPerSec, violations };
}
//#endregion 🫧️Soak

//#region 🚪️Cli
function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `verify idle <serveUrl> [--tag] [--roles editor,viewer] [--only …] [--seconds 10] [--out] [--headed]` (acceptance check
 * `idle-budget`) or, with `--soak-minutes <m> [--windows 10]`, the memory soak (acceptance check `memory-soak`). Writes
 * `<out>/<tag>/idle.json` or `soak.json`, rows as they are measured; `--resume` keeps the measured rows of an earlier run of the
 * same tag and measures only the rest (rows without a reading are measured again); SIGINT/SIGTERM cancel. */
export async function runIdleBudgetCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const valued = new Set(["--tag", "--roles", "--only", "--seconds", "--soak-minutes", "--windows", "--out"]);
  const baseUrl = segments.find((segment, index) => !segment.startsWith("--") && !valued.has(segments[index - 1] ?? ""));
  if (!baseUrl) throw new Error("usage: verify idle <serveUrl> [--tag <t>] [--roles editor,viewer] [--only <plugin|plugin/kind>,…] [--seconds <n>] [--resume] [--soak-minutes <m> --windows <n>] [--out <dir>] [--headed]");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  const soakMinutes = Number(flagValue(segments, "--soak-minutes") ?? 0);
  const tag = flagValue(segments, "--tag") ?? (soakMinutes > 0 ? "memory-soak" : "idle-budget");
  const outDir = join(resolve(flagValue(segments, "--out") ?? defaultOutDir), tag);
  mkdirSync(outDir, { recursive: true });
  const only = (flagValue(segments, "--only") ?? "").split(",").filter(Boolean);
  const headed = segments.includes("--headed");
  try {
    if (soakMinutes > 0) {
      const windows = Number(flagValue(segments, "--windows") ?? 10);
      const samples: SoakSample[] = [];
      const reportPath = join(outDir, "soak.json");
      const result = await runMemorySoak({ baseUrl, minutes: soakMinutes, windows, only, headed, signal: controller.signal, onSample: (sample) => {
        samples.push(sample);
        writeFileSync(reportPath, `${JSON.stringify({ baseUrl, samples }, null, 1)}\n`);
        console.log(`[memory-soak] minute ${sample.minute}: heap ${sample.jsHeapMB} MB, workers ${sample.workersHeapMB} MB, nodes ${sample.nodes}, listeners ${sample.listeners}, frames/s ${sample.idle.framesPerSec}`);
      } });
      const verdict = judgeSoak(result.samples);
      writeFileSync(reportPath, `${JSON.stringify({ baseUrl, opened: result.opened, samples: result.samples, verdict }, null, 1)}\n`);
      const status = verdict.violations.length === 0 && result.opened.length === windows ? "pass" : "fail";
      publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({
        check: "memory-soak",
        status,
        startedAt,
        measured: { windows: result.opened.length, minutes: result.samples.at(-1)?.minute ?? 0, heapSlopeMBPerMin: verdict.heapSlopeMBPerMin, workersSlopeMBPerMin: verdict.workersSlopeMBPerMin, domGrowthRatio: verdict.domGrowthRatio, listenerGrowthRatio: verdict.listenerGrowthRatio, maxFramesPerSec: verdict.maxFramesPerSec },
        summary: {
          en: `${result.opened.length}/${windows} windows for ${result.samples.at(-1)?.minute ?? 0} min: heap ${verdict.heapSlopeMBPerMin} MB/min, workers ${verdict.workersSlopeMBPerMin} MB/min, DOM ${(verdict.domGrowthRatio * 100).toFixed(1)} %, listeners ${(verdict.listenerGrowthRatio * 100).toFixed(1)} %, max ${verdict.maxFramesPerSec} frames/s${verdict.violations.length ? `; ${verdict.violations.join("; ")}` : ""}`,
          de: `${result.opened.length}/${windows} Fenster über ${result.samples.at(-1)?.minute ?? 0} min: Heap ${verdict.heapSlopeMBPerMin} MB/min, Worker ${verdict.workersSlopeMBPerMin} MB/min, DOM ${(verdict.domGrowthRatio * 100).toFixed(1)} %, Listener ${(verdict.listenerGrowthRatio * 100).toFixed(1)} %, höchstens ${verdict.maxFramesPerSec} Bilder/s${verdict.violations.length ? `; ${verdict.violations.length} Verstöße` : ""}`,
        },
        evidence: [reportPath],
      }));
      if (status !== "pass") process.exitCode = 1;
      return;
    }
    const roles = (flagValue(segments, "--roles") ?? "editor,viewer").split(",").filter(Boolean);
    const reportPath = join(outDir, "idle.json");
    const rows: IdleRow[] = segments.includes("--resume") && existsSync(reportPath) ? ((JSON.parse(readFileSync(reportPath, "utf8")) as { rows: IdleRow[] }).rows.filter((row) => row.open !== null)) : [];
    await runIdleCensus({ baseUrl, roles, only, done: new Set(rows.map((row) => `${row.key}#${row.role}`)), seconds: Number(flagValue(segments, "--seconds") ?? 10), headed, signal: controller.signal, onRow: (row) => {
      rows.push(row);
      writeFileSync(reportPath, `${JSON.stringify({ baseUrl, roles, rows }, null, 1)}\n`);
      console.log(`[idle-budget] ${row.pass ? "PASS" : "FAIL"} ${row.key}#${row.role} frames/s ${row.open?.framesPerSec ?? "-"} rAF/s ${row.open?.rafPerSec ?? "-"} draws/s ${row.open?.drawsPerSec ?? "-"} busy ${row.open?.mainBusyPct ?? "-"} %${row.detail ? ` — ${row.detail.slice(0, 200)}` : ""}`);
    } });
    const passed = rows.filter((row) => row.pass).length;
    const failed = rows.filter((row) => !row.pass).map((row) => `${row.key}#${row.role}`);
    const status = rows.length > 0 && passed === rows.length && !controller.signal.aborted ? "pass" : "fail";
    publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({
      check: "idle-budget",
      status,
      startedAt,
      measured: { rows: rows.length, passed, failed: rows.length - passed, maxFramesPerSec: Math.max(0, ...rows.map((row) => row.open?.framesPerSec ?? 0)), maxRafPerSec: Math.max(0, ...rows.map((row) => row.open?.rafPerSec ?? 0)) },
      summary: {
        en: `${passed}/${rows.length} programs idle within the budget${failed.length ? `; over it: ${failed.slice(0, 8).join(", ")}` : ""}`,
        de: `${passed}/${rows.length} Programme bleiben im Leerlauf im Budget${failed.length ? `; darüber: ${failed.slice(0, 8).join(", ")}` : ""}`,
      },
      evidence: [reportPath],
    }));
    console.log(`[idle-budget] === ${tag}: ${status.toUpperCase()} ${passed}/${rows.length} → ${outDir} ===`);
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
}
//#endregion 🚪️Cli

/** 🔀️ The per-origin connection budget of the served `s` React shell, measured in Chromium's own NetLog.
 *
 * A browser allows six HTTP/1.1 connections per origin; a request that holds one open while idle (an `EventSource`, a long
 * poll, a blocking job request) takes it from every later module fetch, command and install. The shell carries every
 * long-lived stream of the page and its workers on ONE WebSocket per origin (`semio.io.stream-mux/v1`,
 * `🧰️framework/🔨️modules/🚪️io/🔀️stream-mux`). This harness proves it the way a user's browser experiences it: boots `s`, then
 * stresses the page's own channel with `--streams` folder change streams and a burst of `--fetches` same-origin requests,
 * touches every watched folder from outside, and reduces the NetLog (Chromium's record, the third-party oracle) to the law:
 * no request idles on a serve-origin connection for ≥ 10 s, exactly one stream-mux WebSocket per page, every fetch of the
 * burst answered within 20 s, every folder notice delivered exactly, and the channel back at its baseline after closing.
 *
 * Promoted from the ticket harnesses `wp-f2/f2-conn-probe.mjs`, `f2-stress.mjs`, `f2-netlog-inventory.py` (ticket 26/09/23 F2:
 * before 4–5 of 6 connections held permanently; per-stream SSE contrast 0/300 fetches in 20 s).
 */

import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import type { Browser } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

//#region 🧾️NetLog
/** 🧾️ One URL request of a Chromium NetLog: when its job started, when it went out on a socket, when it ended, bytes read. */
type NetLogRequest = { url: string | null; start: number | null; sent: number | null; end: number | null; bytes: number; webSocket: boolean };

/** 📊️ The per-origin reduction of a NetLog. */
export type ConnectionInventory = Readonly<{
  origin: string;
  requests: number;
  queueMs: Readonly<{ p50: number | null; p95: number | null; max: number | null }>;
  idleHolds: readonly string[];
  transfersOver10s: readonly string[];
  streamMuxSockets: number;
  otherWebSockets: number;
  stalls: number;
}>;

/** 🔍️ Reduces a Chromium NetLog (`--log-net-log`, capture mode Default) to the connection inventory of `origin`. A request on the
 * wire ≥ 10 s is an IDLE HOLD when it read under 64 KiB (a stream or long poll) and a transfer otherwise (a big file). */
export function connectionInventory(netLogText: string, origin: string): ConnectionInventory {
  const trimmed = netLogText.trimEnd();
  const log = JSON.parse(trimmed.endsWith("}") ? trimmed : `${trimmed.replace(/[,\s]+$/u, "")}]}`) as {
    readonly constants: { readonly logEventTypes: Record<string, number>; readonly logEventPhase: Record<string, number>; readonly logSourceType: Record<string, number> };
    readonly events: readonly { readonly time: string; readonly type: number; readonly phase?: number; readonly source: { readonly id: number; readonly type: number }; readonly params?: Record<string, unknown> }[];
  };
  const name = (table: Record<string, number>) => new Map(Object.entries(table).map(([key, value]) => [value, key] as const));
  const types = name(log.constants.logEventTypes);
  const phases = name(log.constants.logEventPhase);
  const sources = name(log.constants.logSourceType);
  const requests = new Map<number, NetLogRequest>();
  let last = 0;
  let stalls = 0;
  for (const event of log.events) {
    const time = Number(event.time);
    last = Math.max(last, time);
    const type = types.get(event.type) ?? "";
    if (type === "SOCKET_POOL_STALLED_MAX_SOCKETS_PER_GROUP" || type === "SOCKET_POOL_STALLED_MAX_SOCKETS") stalls += 1;
    if (sources.get(event.source.type) !== "URL_REQUEST") continue;
    const phase = phases.get(event.phase ?? -1) ?? "";
    let request = requests.get(event.source.id);
    if (request === undefined) {
      request = { url: null, start: null, sent: null, end: null, bytes: 0, webSocket: false };
      requests.set(event.source.id, request);
    }
    const params = event.params ?? {};
    if (type === "REQUEST_ALIVE" && phase === "PHASE_BEGIN") request.start = time;
    if (type === "URL_REQUEST_START_JOB" && typeof params.url === "string" && request.url === null) {
      request.url = params.url;
      request.start ??= time;
    }
    if (type === "HTTP_TRANSACTION_SEND_REQUEST" && phase === "PHASE_BEGIN" && request.sent === null) request.sent = time;
    if (type === "REQUEST_ALIVE" && phase === "PHASE_END") request.end = time;
    if (type === "URL_REQUEST_JOB_FILTERED_BYTES_READ" && typeof params.byte_count === "number") request.bytes += params.byte_count;
    if (type.startsWith("WEBSOCKET")) request.webSocket = true;
  }
  const webOrigin = origin.replace(/^http/u, "ws");
  const mine = [...requests.values()].filter((request) => request.url !== null && (request.url.startsWith(`${origin}/`) || request.url.startsWith(`${webOrigin}/`)));
  const http = mine.filter((request) => !request.webSocket && !request.url!.startsWith("ws"));
  const sockets = mine.filter((request) => request.webSocket || request.url!.startsWith("ws"));
  const queues = http.filter((request) => request.sent !== null && request.start !== null).map((request) => request.sent! - request.start!).sort((left, right) => left - right);
  const pick = (q: number): number | null => (queues.length === 0 ? null : queues[Math.min(queues.length - 1, Math.floor(q * queues.length))]!);
  const held = http.filter((request) => request.sent !== null && (request.end ?? last) - request.sent >= 10_000);
  const label = (request: NetLogRequest): string => `${Math.round(((request.end ?? last) - request.sent!) / 1000)}s ${request.bytes}B ${decodeURIComponent(new URL(request.url!).pathname).slice(0, 96)}`;
  return {
    origin,
    requests: http.length,
    queueMs: { p50: pick(0.5), p95: pick(0.95), max: queues.at(-1) ?? null },
    idleHolds: held.filter((request) => request.bytes < 65_536).map(label),
    transfersOver10s: held.filter((request) => request.bytes >= 65_536).map(label),
    streamMuxSockets: sockets.filter((request) => new URL(request.url!).pathname === "/semio-stream-mux").length,
    otherWebSockets: sockets.filter((request) => new URL(request.url!).pathname !== "/semio-stream-mux").length,
    stalls,
  };
}
//#endregion 🧾️NetLog

//#region 🏋️Stress
/** 🎛️ One connection-budget run. */
export type ConnectionBudgetOptions = Readonly<{ baseUrl: string; tag: string; streams: number; fetches: number; outDir: string; headed: boolean; signal: AbortSignal }>;

/** 🧾️ What one run measured. */
export type ConnectionBudgetReport = Readonly<{
  baseUrl: string;
  tag: string;
  beacon: string | null;
  channel: Readonly<{ before: unknown; opened: unknown; after: unknown }>;
  fetches: Readonly<{ count: number; ok: number; within20s: number; p50Ms: number | null; maxMs: number | null }>;
  notices: Readonly<{ folders: number; freshOpens: number; firstTouch: number; secondTouch: number; expectedSecond: number }>;
  inventory: ConnectionInventory;
  violations: readonly string[];
}>;

/** 🏋️ Boots `s`, stresses the page's stream channel and judges the budget law. */
export async function runConnectionBudget(repoRoot: string, options: ConnectionBudgetOptions): Promise<ConnectionBudgetReport> {
  const origin = new URL(options.baseUrl).origin;
  const runDir = join(options.outDir, options.tag);
  mkdirSync(runDir, { recursive: true });
  const netLog = join(runDir, "netlog.json");
  const scratch = mkdtempSync(join(tmpdir(), "semio-connection-budget-"));
  const folders = Array.from({ length: options.streams }, (_, index) => join(scratch, `folder-${String(index).padStart(4, "0")}`));
  for (const folder of folders) mkdirSync(join(folder, ".semio"), { recursive: true });
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser: Browser = await chromium.launch({ headless: !options.headed, args: ["--use-angle=metal", `--log-net-log=${netLog}`, "--net-log-capture-mode=Default"] });
  const abort = (): void => void browser.close().catch(() => undefined);
  options.signal.addEventListener("abort", abort, { once: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
    page.setDefaultNavigationTimeout(300_000);
    await page.goto(options.baseUrl, { waitUntil: "commit" });
    const beacon = await page
      .waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") ?? document.documentElement.getAttribute("data-semio-os-error"), undefined, { timeout: 300_000, polling: 1_000 })
      .then((handle) => handle.jsonValue() as Promise<string>)
      .catch(() => null);
    console.log(`[connection-budget] ${options.tag}: shell ${beacon ?? "never ready"}`);
    await page.waitForTimeout(5_000);
    const muxModule = `/@fs${resolve(repoRoot, "🧰️framework/🔨️modules/🚪️io/🔀️stream-mux/🟦️.ts")}`;
    const opened = await page.evaluate(
      async ({ muxModule, folders, fetches }) => {
        const mux = (await import(muxModule)) as typeof import("../../../../../../🔨️modules/🚪️io/🔀️stream-mux/🟦️.ts");
        const channel = mux.pageStreamMuxChannelV1("/semio-stream-mux") as import("../../../../../../🔨️modules/🚪️io/🔀️stream-mux/🟦️.ts").StreamMuxChannelV1;
        const before = channel.census();
        const rows = folders.map(() => ({ opened: [] as string[], notices: 0 }));
        const subscriptions = folders.map((folder, index) => channel.open("backbone.folder", `folder://${folder}`, { opened: (mode) => void rows[index]!.opened.push(mode), data: () => void (rows[index]!.notices += 1) }));
        Reflect.set(globalThis, "__semioConnectionBudget", { channel, rows, subscriptions });
        const durations = await Promise.all(
          Array.from({ length: fetches }, async (_, index) => {
            const controller = new AbortController();
            const timer = setTimeout(() => controller.abort(), 20_000);
            const begin = performance.now();
            try {
              const response = await fetch(`/favicon.ico?connection-budget=${index}`, { cache: "no-store", signal: controller.signal });
              await response.arrayBuffer();
              return response.ok ? performance.now() - begin : -1;
            } catch {
              return null;
            } finally {
              clearTimeout(timer);
            }
          }),
        );
        await new Promise((resolveWait) => setTimeout(resolveWait, 3_000));
        return { before, opened: channel.census(), durations };
      },
      { muxModule, folders, fetches: options.fetches },
    );
    for (const folder of folders) writeFileSync(join(folder, ".semio", "touch.json"), JSON.stringify({ at: Date.now() }));
    await page.waitForTimeout(3_000);
    const firstTouch = await page.evaluate(() => (Reflect.get(globalThis, "__semioConnectionBudget") as { rows: { notices: number }[] }).rows.map((row) => row.notices));
    const touchedAgain = folders.filter((_, index) => index % 2 === 0);
    for (const folder of touchedAgain) writeFileSync(join(folder, ".semio", "touch.json"), JSON.stringify({ at: Date.now() }));
    await page.waitForTimeout(3_000);
    const final = await page.evaluate(async () => {
      const state = Reflect.get(globalThis, "__semioConnectionBudget") as { channel: { census(): unknown }; rows: { opened: string[]; notices: number }[]; subscriptions: { close(): void }[] };
      const notices = state.rows.map((row) => row.notices);
      const freshOpens = state.rows.filter((row) => row.opened.includes("fresh")).length;
      for (const subscription of state.subscriptions) subscription.close();
      await new Promise((resolveWait) => setTimeout(resolveWait, 1_000));
      return { notices, freshOpens, after: state.channel.census() };
    });
    await browser.close();
    const completed = opened.durations.filter((value): value is number => value !== null && value >= 0).sort((left, right) => left - right);
    const inventory = connectionInventory(readFileSync(netLog, "utf8"), origin);
    const firstTotal = firstTouch.filter((value) => value >= 1).length;
    const secondTotal = final.notices.filter((value, index) => value - firstTouch[index]! >= 1).length;
    const violations = [
      ...(beacon?.startsWith("ready") ? [] : [`the shell never became ready (${beacon ?? "no beacon"})`]),
      ...inventory.idleHolds.map((hold) => `a request idled on a ${origin} connection: ${hold}`),
      ...(inventory.streamMuxSockets === 1 ? [] : [`expected exactly one stream-mux WebSocket, saw ${inventory.streamMuxSockets}`]),
      ...(completed.length === options.fetches ? [] : [`${options.fetches - completed.length} of ${options.fetches} fetches did not finish within 20 s`]),
      ...(final.freshOpens === folders.length ? [] : [`${folders.length - final.freshOpens} folder streams never opened`]),
      ...(firstTotal === folders.length ? [] : [`${folders.length - firstTotal} folders missed their first change notice`]),
      ...(secondTotal === touchedAgain.length ? [] : [`${secondTotal} folders got a second notice, ${touchedAgain.length} were touched again`]),
      ...(JSON.stringify(final.after) === JSON.stringify(opened.before) ? [] : [`the channel did not return to its baseline: ${JSON.stringify(opened.before)} → ${JSON.stringify(final.after)}`]),
    ];
    return {
      baseUrl: options.baseUrl,
      tag: options.tag,
      beacon,
      channel: { before: opened.before, opened: opened.opened, after: final.after },
      fetches: { count: options.fetches, ok: completed.length, within20s: completed.length, p50Ms: completed.length === 0 ? null : Math.round(completed[Math.floor(completed.length / 2)]!), maxMs: completed.length === 0 ? null : Math.round(completed.at(-1)!) },
      notices: { folders: folders.length, freshOpens: final.freshOpens, firstTouch: firstTotal, secondTouch: secondTotal, expectedSecond: touchedAgain.length },
      inventory,
      violations,
    };
  } finally {
    options.signal.removeEventListener("abort", abort);
    await browser.close().catch(() => undefined);
    rmSync(scratch, { recursive: true, force: true });
  }
}
//#endregion 🏋️Stress

//#region 🚪️Cli
function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `verify connections <baseUrl> [--tag <t>] [--streams <n>] [--fetches <n>] [--out <dir>] [--headed]` — runs
 * {@link runConnectionBudget}, writes `<out>/<tag>/connection-budget.json` + the NetLog, publishes the acceptance result
 * (`connection-budget`) and exits non-zero on any violation. SIGINT/SIGTERM cancel the run. */
export async function runConnectionBudgetCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const baseUrl = segments[0];
  if (!baseUrl || baseUrl.startsWith("--")) throw new Error("usage: verify connections <baseUrl> [--tag <t>] [--streams <n>] [--fetches <n>] [--out <dir>] [--headed]");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  try {
    const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
    const tag = flagValue(segments, "--tag") ?? "connection-budget";
    const report = await runConnectionBudget(repoRoot, {
      baseUrl,
      tag,
      streams: Number(flagValue(segments, "--streams") ?? 64),
      fetches: Number(flagValue(segments, "--fetches") ?? 300),
      outDir,
      headed: segments.includes("--headed"),
      signal: controller.signal,
    });
    const reportPath = join(outDir, tag, "connection-budget.json");
    writeFileSync(reportPath, `${JSON.stringify(report, null, 1)}\n`);
    const status = report.violations.length === 0 ? "pass" : "fail";
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check: "connection-budget",
        status,
        startedAt,
        measured: {
          idleHolds: report.inventory.idleHolds.length,
          streamMuxSockets: report.inventory.streamMuxSockets,
          streams: report.notices.folders,
          fetches: report.fetches.count,
          fetchesWithin20s: report.fetches.within20s,
          noticesFirstTouch: report.notices.firstTouch,
          noticesSecondTouch: report.notices.secondTouch,
          socketPoolStalls: report.inventory.stalls,
        },
        summary: {
          en: `${report.inventory.idleHolds.length} idle HTTP holds on ${new URL(baseUrl).origin}, ${report.inventory.streamMuxSockets} stream channel, ${report.notices.folders} streams, ${report.fetches.within20s}/${report.fetches.count} fetches within 20 s${report.violations.length ? `; ${report.violations.slice(0, 3).join("; ")}` : ""}`,
          de: `${report.inventory.idleHolds.length} ruhende HTTP-Verbindungen auf ${new URL(baseUrl).origin}, ${report.inventory.streamMuxSockets} Stromkanal, ${report.notices.folders} Ströme, ${report.fetches.within20s}/${report.fetches.count} Abrufe innerhalb von 20 s${report.violations.length ? `; ${report.violations.slice(0, 3).join("; ")}` : ""}`,
        },
        evidence: [reportPath, join(outDir, tag, "netlog.json")],
      }),
    );
    for (const violation of report.violations) console.log(`[connection-budget] VIOLATION ${violation}`);
    console.log(`[connection-budget] === ${tag}: ${status.toUpperCase()} → ${join(outDir, tag)} ===`);
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
}
//#endregion 🚪️Cli

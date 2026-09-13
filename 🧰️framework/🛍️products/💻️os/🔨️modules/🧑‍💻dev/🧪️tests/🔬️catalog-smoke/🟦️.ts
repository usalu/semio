/** 🧩️ Semantic catalog smoke owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { ensureParityPlaywrightBrowsersPath } from "../⚖️parity/🏃️execution/🟦️.ts";

import { openStudioE2e, openStudioE2eCommandPalette, waitForStudioE2eCondition } from "../🎬️studio/🟦️.ts";



//#endregion 🔖️SpaceE2eVerify

//#region 🔖️CatalogSmokeVerify
/** 🔬️ Catalog-wide render smoke: one live `s` session, every program the shell itself offers spawned in
 * turn, each proven to mount a window with a non-empty rendered body. The program list is never hardcoded
 * — it comes from the shell's own dev probe (`window.__semioOsCatalogProbe`, `#region 🔖️CatalogSmokeProbe`
 * in `🏛️ShellHost/🟦️.tsx`), which also reports every plugin left in `failed`/`crashed` install status.
 * Report shape: `🧑‍💻dev/🧬️schema/🔣️.json#/$defs/CatalogSmokeReportV1`. */
type CatalogSmokeStatus = "pass" | "fail" | "skipped";

type CatalogSmokeRow = {
  readonly pluginId: string;
  readonly appId: string;
  readonly label: string;
  readonly status: CatalogSmokeStatus;
  readonly durationMs: number;
  readonly windowId: string | null;
  readonly descendants: number;
  readonly width: number;
  readonly height: number;
  readonly firstError: string | null;
};

type CatalogSmokePluginStatus = { readonly pluginId: string; readonly status: string };

/** 🔬️ What the shell itself reported before any program could be spawned. `beacon` is the raw
 * `semioOsReady`/`semioOsError`/`semioOsNotFound` dataset value (`null` when none was ever set);
 * `failure` is the boot step that gave up. Recorded rather than thrown so a shell that never boots still
 * produces the same machine-readable artifact as one that boots and fails a program. */
type CatalogSmokeBoot = {
  readonly beacon: string | null;
  readonly failure: string | null;
  /** ⏱️Milliseconds from navigation start to each boot phase: the document commit, the first module the
   * page actually requested, and the readiness beacon. `null` for a phase never reached — the unbundled
   * dev module graph never fires `load` on a busy machine, so these, not `goto`, are the boot evidence. */
  readonly phaseMs: { readonly commit: number | null; readonly firstModule: number | null; readonly beacon: number | null };
  readonly bodyExcerpt: string;
  readonly consoleErrors: readonly string[];
};

type CatalogSmokeReport = {
  readonly baseUrl: string;
  readonly shellPluginId: string;
  readonly startedAt: string;
  readonly durationMs: number;
  readonly boot: CatalogSmokeBoot;
  readonly programs: readonly CatalogSmokeRow[];
  readonly failedPlugins: readonly CatalogSmokePluginStatus[];
  readonly totals: { readonly pass: number; readonly fail: number; readonly skipped: number };
};

/** 🔬️ Bound on how much shell console noise and body text the report carries — enough to name the first
 * cause, never a full session log. */
const CATALOG_SMOKE_BOOT_ERROR_CAPACITY = 20;

const CATALOG_SMOKE_BODY_EXCERPT_CAPACITY = 1000;

const CATALOG_SMOKE_FAILED_PLUGIN_STATUSES = ["failed", "crashed"] as const;

/** 🔬️ Default `--out` directory, beside the dev module's other generated artifacts. */
const CATALOG_SMOKE_DEFAULT_OUT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🔬️catalog-smoke";

/** 🔬️ Folds per-program outcomes and the shell's install-status table into the report the command writes
 * and exits on — pure, so the aggregation is unit-testable without a browser. */
function summarizeCatalogSmoke(input: {
  readonly baseUrl: string;
  readonly shellPluginId: string;
  readonly startedAt: string;
  readonly durationMs: number;
  readonly boot: CatalogSmokeBoot;
  readonly programs: readonly CatalogSmokeRow[];
  readonly plugins: readonly CatalogSmokePluginStatus[];
}): CatalogSmokeReport {
  const totals = { pass: 0, fail: 0, skipped: 0 };
  for (const row of input.programs) totals[row.status] += 1;
  return {
    baseUrl: input.baseUrl,
    shellPluginId: input.shellPluginId,
    startedAt: input.startedAt,
    durationMs: input.durationMs,
    boot: { ...input.boot, bodyExcerpt: input.boot.bodyExcerpt.slice(0, CATALOG_SMOKE_BODY_EXCERPT_CAPACITY), consoleErrors: input.boot.consoleErrors.slice(0, CATALOG_SMOKE_BOOT_ERROR_CAPACITY) },
    programs: input.programs,
    failedPlugins: input.plugins.filter((plugin) => (CATALOG_SMOKE_FAILED_PLUGIN_STATUSES as readonly string[]).includes(plugin.status)),
    totals,
  };
}

/** 🔬️ Non-zero exit whenever the shell never booted, a program failed to render, a plugin never left
 * `failed`/`crashed`, or nothing at all rendered — a vacuous green is itself a failure. */
function catalogSmokeExitCode(report: CatalogSmokeReport): number {
  return report.boot.failure !== null || report.totals.fail > 0 || report.failedPlugins.length > 0 || report.totals.pass === 0 ? 1 : 0;
}

function catalogSmokeMarkdownCell(value: string): string {
  return value.replaceAll("|", "\\|").replaceAll("\n", " ").slice(0, 200);
}

/** 🔬️ Markdown mirror of {@link summarizeCatalogSmoke}'s report — one row per program, ordered as spawned. */
function catalogSmokeMarkdown(report: CatalogSmokeReport): string {
  const lines = [
    `# Catalog smoke — \`${report.shellPluginId}\` @ ${report.baseUrl}`,
    "",
    `Started ${report.startedAt} · ${report.durationMs} ms · ${report.totals.pass} pass / ${report.totals.fail} fail / ${report.totals.skipped} skipped`,
    "",
    `Shell boot: beacon \`${report.boot.beacon ?? "none"}\` — commit ${report.boot.phaseMs.commit ?? "never"} ms, first module ${report.boot.phaseMs.firstModule ?? "never"} ms, beacon ${report.boot.phaseMs.beacon ?? "never"} ms${report.boot.failure === null ? "" : ` — FAILED: ${report.boot.failure}`}`,
    "",
    "| pluginId | appId | status | duration ms | window | nodes | first error |",
    "|---|---|---|---|---|---|---|",
  ];
  for (const row of report.programs) {
    lines.push(`| ${catalogSmokeMarkdownCell(row.pluginId)} | ${catalogSmokeMarkdownCell(row.appId)} | ${row.status} | ${row.durationMs} | ${catalogSmokeMarkdownCell(row.windowId ?? "-")} | ${row.descendants} | ${catalogSmokeMarkdownCell(row.firstError ?? "-")} |`);
  }
  lines.push("", `## Plugins in a failed install status (${report.failedPlugins.length})`, "");
  if (report.failedPlugins.length === 0) lines.push("none");
  else {
    lines.push("| pluginId | status |", "|---|---|");
    for (const plugin of report.failedPlugins) lines.push(`| ${catalogSmokeMarkdownCell(plugin.pluginId)} | ${catalogSmokeMarkdownCell(plugin.status)} |`);
  }
  if (report.boot.failure !== null) {
    lines.push("", `## Shell boot diagnostics (${report.boot.consoleErrors.length} console error(s))`, "", "```", report.boot.bodyExcerpt || "(empty body)", "```", "");
    for (const error of report.boot.consoleErrors) lines.push(`- ${catalogSmokeMarkdownCell(error)}`);
  }
  return `${lines.join("\n")}\n`;
}

/** 🔬️ A dev server reloads the page whenever a peer touches a watched file, which destroys the execution
 * context under any in-flight `evaluate`. Retries such a read on the next document instead of failing the
 * whole smoke — a transient reload is not a catalog defect. */
async function catalogSmokeEvaluate<Result>(page: import("playwright").Page, read: () => Promise<Result>, attempts = 4): Promise<Result> {
  let last: unknown;
  for (let attempt = 0; attempt < attempts; attempt++) {
    try {
      return await read();
    } catch (error) {
      last = error;
      if (!String(error).includes("Execution context was destroyed")) throw error;
      await page.waitForLoadState("domcontentloaded").catch(() => undefined);
      await page.waitForTimeout(1000);
    }
  }
  throw last;
}

async function readCatalogSmokeProbe(page: import("playwright").Page): Promise<{ shellPluginId: string; ready: boolean; plugins: CatalogSmokePluginStatus[]; programs: { pluginId: string; appId: string; label: string }[] } | null> {
  return (await catalogSmokeEvaluate(page, () => page.evaluate(() => (window as unknown as { __semioOsCatalogProbe?: unknown }).__semioOsCatalogProbe ?? null))) as never;
}

async function catalogSmokeWindowIds(page: import("playwright").Page): Promise<string[]> {
  return await catalogSmokeEvaluate(page, () => page.evaluate(() => [...document.querySelectorAll<HTMLElement>("[id^='framework.window.']")].filter((element) => !element.id.includes(".windowControls")).map((element) => element.id)));
}

/** 🔬️ Spawns one program through the shell's own command palette (`spawn.<pluginId>` item) and reports the
 * window element it mounted, or `null` when no new window appeared within the deadline. */
async function spawnCatalogSmokeProgram(page: import("playwright").Page, pluginId: string, before: readonly string[], timeoutMs: number): Promise<string | null> {
  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill(pluginId);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first();
  await item.waitFor({ state: "visible", timeout: timeoutMs }).catch(() => undefined);
  if ((await item.count()) === 0) {
    await page.keyboard.press("Escape");
    return null;
  }
  await item.click();
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const after = await catalogSmokeWindowIds(page);
    const opened = after.find((id) => !before.includes(id));
    if (opened) return opened;
    await page.waitForTimeout(250);
  }
  return null;
}

async function closeCatalogSmokeWindow(page: import("playwright").Page, windowId: string): Promise<void> {
  const close = page.locator(`[id="${windowId}.windowControls.close"]`).first();
  if ((await close.count()) === 0) return;
  await close.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(250);
}

async function runCatalogSmokeVerify(baseUrl: string, opts: { readonly outDir: string; readonly timeoutMs: number; readonly perProgramMs: number }): Promise<CatalogSmokeReport> {
  ensureParityPlaywrightBrowsersPath();
  const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });

  const startedAt = new Date().toISOString();
  const started = Date.now();
  const rows: CatalogSmokeRow[] = [];
  let probe: Awaited<ReturnType<typeof readCatalogSmokeProbe>> = null;
  let bootFailure: string | null = null;
  let beacon: string | null = null;
  let bodyExcerpt = "";
  const phaseMs: { commit: number | null; firstModule: number | null; beacon: number | null } = { commit: null, firstModule: null, beacon: null };
  try {
    console.log(`[catalog-smoke] navigating to ${baseUrl}`);
    const navigationStarted = Date.now();
    page.on("request", (request) => {
      if (phaseMs.firstModule === null && request.resourceType() === "script") phaseMs.firstModule = Date.now() - navigationStarted;
    });
    // 🚦️`commit` and not `load`: the unbundled dev module graph legitimately never fires `load` on a busy
    // machine (observed: `goto` timing out at 300 s against a shell that was booting fine), so the beacon
    // poll below owns the deadline and a slow boot is reported, never thrown.
    await page.goto(baseUrl, { waitUntil: "commit", timeout: opts.timeoutMs });
    phaseMs.commit = Date.now() - navigationStarted;
    console.log(`[catalog-smoke] document committed after ${phaseMs.commit} ms`);
    const beaconDeadline = Date.now() + opts.timeoutMs;
    while (Date.now() < beaconDeadline) {
      beacon = await catalogSmokeEvaluate(page, () =>
        page.evaluate(() => {
          const data = document.documentElement.dataset;
          if (data.semioOsReady !== undefined) return `ready:${data.semioOsReady}`;
          if (data.semioOsError !== undefined) return `error:${data.semioOsError}`;
          if (data.semioOsNotFound !== undefined) return `not-found:${data.semioOsNotFound}`;
          return null;
        }),
      );
      if (beacon !== null) break;
      await page.waitForTimeout(1000);
    }
    if (beacon !== null) phaseMs.beacon = Date.now() - navigationStarted;
    console.log(`[catalog-smoke] readiness beacon: ${beacon ?? "none"} (commit ${phaseMs.commit} ms, first module ${phaseMs.firstModule ?? "never"} ms, beacon ${phaseMs.beacon ?? "never"} ms)`);
    if (beacon === null || !beacon.startsWith("ready:")) bootFailure = `shell never reached a ready beacon (${beacon ?? "no beacon within " + opts.timeoutMs + "ms"})`;

    const deadline = Date.now() + opts.timeoutMs;
    if (bootFailure === null) {
      await waitForStudioE2eCondition(page, ({ text }) => /Home/i.test(text) && /Studios|Search/i.test(text), "home shell", deadline).catch((error: unknown) => {
        bootFailure = `home shell never listed studios: ${String(error)}`;
      });
    }
    if (bootFailure === null) {
      await openStudioE2e(page, deadline).catch((error: unknown) => {
        bootFailure = `studio never opened: ${String(error)}`;
      });
    }
    if (bootFailure === null) {
      await page.waitForFunction(() => document.querySelector(".semio-node-graph-host") != null, undefined, { timeout: opts.timeoutMs }).catch((error: unknown) => {
        bootFailure = `studio workflow window never rendered: ${String(error)}`;
      });
    }
    if (bootFailure === null) {
      probe = await readCatalogSmokeProbe(page);
      if (probe === null) bootFailure = "shell exposed no `window.__semioOsCatalogProbe` — is the dev server serving a development build?";
    }
    if (bootFailure !== null) console.error(`[catalog-smoke] boot failed: ${bootFailure}`);
    else console.log(`[catalog-smoke] ${probe!.programs.length} spawnable programs, ${probe!.plugins.length} registry rows`);

    for (const program of probe?.programs ?? []) {
      const programStarted = Date.now();
      const errorCursor = errors.length;
      const before = await catalogSmokeWindowIds(page);
      const windowId = await spawnCatalogSmokeProgram(page, program.pluginId, before, opts.perProgramMs);
      if (windowId === null) {
        rows.push({ pluginId: program.pluginId, appId: program.appId, label: program.label, status: "fail", durationMs: Date.now() - programStarted, windowId: null, descendants: 0, width: 0, height: 0, firstError: errors[errorCursor] ?? "no window element appeared after spawn" });
        continue;
      }
      const measured = await catalogSmokeEvaluate(page, () => page.evaluate((id) => {
        const element = document.getElementById(id);
        if (!element) return { descendants: 0, width: 0, height: 0 };
        const rect = element.getBoundingClientRect();
        return { descendants: element.querySelectorAll("*").length, width: Math.round(rect.width), height: Math.round(rect.height) };
      }, windowId));
      const rendered = measured.width > 0 && measured.height > 0 && measured.descendants > 0;
      const firstError = errors[errorCursor] ?? null;
      rows.push({ pluginId: program.pluginId, appId: program.appId, label: program.label, status: rendered && firstError === null ? "pass" : "fail", durationMs: Date.now() - programStarted, windowId, descendants: measured.descendants, width: measured.width, height: measured.height, firstError: rendered ? firstError : (firstError ?? `window ${windowId} rendered ${measured.width}x${measured.height} with ${measured.descendants} descendants`) });
      await closeCatalogSmokeWindow(page, windowId);
    }
  } finally {
    bodyExcerpt = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    await browser.close();
  }

  const report = summarizeCatalogSmoke({ baseUrl, shellPluginId: probe?.shellPluginId ?? "unknown", startedAt, durationMs: Date.now() - started, boot: { beacon, failure: bootFailure, phaseMs, bodyExcerpt, consoleErrors: errors }, programs: rows, plugins: probe?.plugins ?? [] });
  mkdirSync(opts.outDir, { recursive: true });
  writeFileSync(join(opts.outDir, "🔬️catalog-smoke.json"), `${JSON.stringify(report, null, 2)}\n`);
  writeFileSync(join(opts.outDir, "🔬️catalog-smoke.md"), catalogSmokeMarkdown(report));
  console.log(catalogSmokeMarkdown(report));
  console.log(`[catalog-smoke] report written to ${opts.outDir}`);
  return report;
}

export { CATALOG_SMOKE_BODY_EXCERPT_CAPACITY, CATALOG_SMOKE_BOOT_ERROR_CAPACITY, CATALOG_SMOKE_DEFAULT_OUT_REL, CATALOG_SMOKE_FAILED_PLUGIN_STATUSES, CatalogSmokeBoot, CatalogSmokePluginStatus, CatalogSmokeReport, CatalogSmokeRow, CatalogSmokeStatus, catalogSmokeEvaluate, catalogSmokeExitCode, catalogSmokeMarkdown, catalogSmokeMarkdownCell, catalogSmokeWindowIds, closeCatalogSmokeWindow, readCatalogSmokeProbe, runCatalogSmokeVerify, spawnCatalogSmokeProgram, summarizeCatalogSmoke };

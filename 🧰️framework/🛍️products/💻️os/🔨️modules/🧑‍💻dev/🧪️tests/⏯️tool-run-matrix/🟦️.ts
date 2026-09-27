/** ⏯️ The tool-run column of the program matrix: every program that declares a tool, inside ONE served `s`, as a user drives it.
 *
 * Per program (`🧑‍💻dev/🧫️fixtures/⏯️tool-run-matrix.json`, each with its tool, its declared `mutating` flag and the example a run
 * needs): a fresh browser context spawns it from the palette, seats the example, arms the tool through the shell's Tool
 * category, raises History, the Tasks window and the Tool runs panel, then drives two real runs:
 *
 *   run A: Start → progress → Pause (the progress must hold still) → Resume → terminal (a `ready to finalize` run is finalized)
 *          → a mutating tool's result is committed (document changed or `Check in` +1, and a new `↶` History row) and the rail
 *          Undo restores the document; a read-only tool finalizes and writes nothing;
 *   run B: Start → Abort at once → terminal Aborted → nothing committed.
 *
 * Every change of what the person sees (panel status + progress text, Tasks row state, check-in) is kept as the row's timeline.
 * A row passes when run A meets its tool's declared contract and run B aborts cleanly.
 *
 * Promoted from the session-12/13 ticket harness `wp-s15/s15-toolrun-matrix.mjs` = `wp-s16/s16-toolrun-matrix.mjs` (selectors
 * from U5's tool-run probe), unchanged in judgement.
 * @see ../🧮️program-matrix/🟦️.ts
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { awaitBeacon, click, clickUncovered, dismissIntroduction, readShell, seatLocale, withDevServe, witness } from "../🧮️program-matrix/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

/** 🧰️ One program that declares a tool, and the example a run of it needs (`first` = the first non-empty one). */
type ToolProgram = Readonly<{ pluginId: string; appId: string; toolId: string; mutating: boolean; example?: string }>;

/** 📌️ `semio.os-dev.tool-run-matrix/v1` — every program that declares a tool. */
function readToolPrograms(): readonly ToolProgram[] {
  const doc = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "🧫️fixtures", "⏯️tool-run-matrix.json"), "utf8")) as { schema?: string; programs?: ToolProgram[] };
  if (doc.schema !== "semio.os-dev.tool-run-matrix/v1" || !Array.isArray(doc.programs)) throw new Error("tool-run matrix fixture: unexpected schema");
  return doc.programs;
}

const READY_TO_FINALIZE = /^(Complete, ready to finalize|Fertig, bereit zum Abschließen)/u;
const TERMINAL = /^(Finalized|Abgeschlossen|Aborted|Abgebrochen|Failed|Fehlgeschlagen)/u;
const ABORTED = /^(Aborted|Abgebrochen)/u;
const COMMITTED = /^(Finalized|Abgeschlossen)/u;

type RunView = { scope: string; status: string; value: string | null; text: string | null };
type TaskView = { id: string | null; state: string | null; progress: string | null; buttons: string[] };
type Sample = { runs: RunView[]; tasks: TaskView[]; checkin: string; history: string[]; stall: string[] };

/** 🔭️ What the person sees of tool runs: the Tool runs panel rows, the Tasks window's tool-run rows, check-in, History, stalls. */
const sample = (page: Page): Promise<Sample> =>
  page.evaluate(() => {
    const runs = [...document.querySelectorAll('[data-ui-node-key^="framework.toolRun."][data-ui-node-key$=".status"]')]
      .filter((status) => !(status.getAttribute("data-ui-node-key") ?? "").startsWith("framework.toolRun.ready"))
      .map((status) => {
        const scope = (status.getAttribute("data-ui-node-key") ?? "").slice(0, -".status".length);
        const bar = document.querySelector(`[data-ui-node-key="${scope}.progress"]`);
        return { scope, status: (status.textContent ?? "").trim(), value: bar?.getAttribute("aria-valuenow") ?? null, text: bar?.getAttribute("aria-valuetext") ?? null };
      });
    const tasks = [...document.querySelectorAll("[data-semio-task-manager-task]")]
      .filter((row) => row.getAttribute("data-semio-task-manager-lane") === "toolRun")
      .map((row) => ({ id: row.getAttribute("data-semio-task-manager-task"), state: row.getAttribute("data-semio-task-manager-state"), progress: row.querySelector("[role='progressbar']")?.getAttribute("aria-valuetext") ?? null, buttons: [...row.querySelectorAll("button")].map((button) => `${button.id}${button.disabled ? "(disabled)" : ""}`) }));
    const checkin = (document.querySelector("#s-checkin")?.textContent ?? "").trim();
    const history = [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => ((element as HTMLElement).innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 60));
    const stall = [...document.querySelectorAll("[data-semio-command-stall-id]")].map((element) => (element.textContent ?? "").trim().slice(0, 120));
    return { runs, tasks, checkin, history, stall };
  });

const editsOf = (state: Sample): number => Number(/\((\d+)\)\s*$/u.exec(state.checkin)?.[1] ?? 0);
const latestRun = (state: Sample): RunView | null => state.runs.at(-1) ?? null;

async function historyOpen(page: Page): Promise<boolean> {
  return page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
}

/** 🧰️ Spawns the program, seats its example, arms its tool, raises History, the Tasks window and the Tool runs panel; answers
 * `null` when armed, else what was missing, plus the example it picked and the ones offered. */
async function openTools(page: Page, program: ToolProgram): Promise<{ missing: string | null; examplePicked: string | null; exampleOptions: string[] | null }> {
  await dismissIntroduction(page);
  await page.evaluate(() => document.body.focus());
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(program.appId)?.[1] ?? program.pluginId);
  await page.waitForTimeout(1_200);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${program.pluginId}.${program.appId}"], [data-slot="command-item"][data-command-item-id="spawn.${program.pluginId}"]`).first();
  if ((await item.count()) === 0) return { missing: "no palette row", examplePicked: null, exampleOptions: null };
  await item.click({ force: true });
  await page.waitForTimeout(12_000);
  let examplePicked: string | null = null;
  let exampleOptions: string[] | null = null;
  if (program.example) {
    const picker = page.locator('[id="playground.navbar.fixture"]').first();
    if (await picker.count()) {
      await picker.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(800);
      const options = await page.locator('[role="option"]').evaluateAll((rows) => rows.map((row) => ({ value: row.getAttribute("data-value") ?? "", text: (row.textContent ?? "").trim(), selected: row.getAttribute("aria-selected") === "true" })));
      const wanted = program.example === "first" ? options.find((option) => !option.selected && option.value !== "" && !/^(empty|leer|none|keine)$/iu.test(option.text)) : options.find((option) => new RegExp(program.example!, "u").test(`${option.value} ${option.text}`));
      exampleOptions = options.map((option) => `${option.value}=${option.text}${option.selected ? "*" : ""}`).slice(0, 12);
      examplePicked = wanted ? `${wanted.value}=${wanted.text}` : null;
      if (wanted) await page.locator('[role="option"]').nth(options.indexOf(wanted)).click({ force: true }).catch(() => undefined);
      else await page.keyboard.press("Escape");
      await page.waitForTimeout(10_000);
    } else examplePicked = "no fixture picker";
  }
  const category = page.locator('[id="framework.category.tool"]').first();
  if (await category.count()) {
    await category.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_000);
  }
  const tool = page.locator(`[id="tool.${program.toolId}"]`).first();
  if ((await tool.count()) === 0) return { missing: "no tool row", examplePicked, exampleOptions };
  if ((await tool.getAttribute("aria-pressed").catch(() => null)) !== "true") await tool.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(800);
  if (!(await historyOpen(page))) await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(800);
  await page.locator('[id="os.task-manager"]').first().click({ timeout: 10_000 }).catch(() => undefined);
  await page.waitForTimeout(800);
  await page.locator('[data-tab-id="framework.panel.toolRun"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_200);
  return { missing: null, examplePicked, exampleOptions };
}

/** 🧾️ The document's pending edits and History rows, read from the History panel (raised for the reading, then the Tool runs
 * panel is raised again so the run's own controls stay in reach). */
async function readHistory(page: Page): Promise<{ edits: number; history: string[]; undoableRows: number; checkin: string; render: string }> {
  if (!(await historyOpen(page))) await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_000);
  const state = await sample(page);
  const render = witness(await readShell(page)).render;
  await page.locator('[data-tab-id="framework.panel.toolRun"]').first().click({ force: true }).catch(() => undefined);
  await page.evaluate(() => document.getElementById("framework.panelTab.framework.panel.toolRun")?.click());
  await page.waitForTimeout(800);
  return { edits: editsOf(state), history: state.history.slice(-4), undoableRows: state.history.filter((row) => row.includes("↶")).length, checkin: state.checkin, render };
}

async function start(page: Page): Promise<string> {
  const button = page.locator('[data-ui-node-key="framework.toolRun.ready.toolRunStart"]').first();
  if (await button.count()) {
    await button.click({ force: true }).catch(() => undefined);
    return "ready Start button";
  }
  await page.keyboard.press(process.platform === "darwin" ? "Meta+Enter" : "Control+Enter");
  return "chord";
}

async function control(page: Page, action: "pause" | "resume" | "abort", taskId: string | null | undefined): Promise<string> {
  const tasks = page.locator(`[id="os.task-manager.${action === "pause" ? "suspend" : action === "abort" ? "cancel" : action}.${taskId}"]`).first();
  if (taskId && (await tasks.count())) {
    await tasks.click({ timeout: 5_000 }).catch(() => undefined);
    return "tasks";
  }
  const panel = page.locator(`[data-ui-node-key$=".${{ pause: "toolRunPause", resume: "toolRunResume", abort: "toolRunAbort" }[action]}"]`).first();
  if (await panel.count()) {
    await panel.click({ force: true }).catch(() => undefined);
    return "panel";
  }
  return "absent";
}

/** 🎛️ One tool-run matrix run. */
export type ToolRunMatrixOptions = Readonly<{ baseUrl: string; tag: string; locale: string; only: readonly string[]; pause: boolean; runMs: number; outDir: string; signal: AbortSignal }>;

type RunRecord = Record<string, unknown> & { timeline: { label: string; summary: string; stall: string[] }[] };

/** 🧾️ One program's row. */
export type ToolRunRow = Record<string, unknown> & { key: string; toolId: string; mutating: boolean; a: RunRecord; b: RunRecord; faults: string[]; pass: boolean; verdict: string };

async function runRow(browser: Browser, program: ToolProgram, options: ToolRunMatrixOptions): Promise<ToolRunRow> {
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  const faults: string[] = [];
  page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 240)));
  page.on("console", (message) => {
    if (/refused|dispatch-failed|panicked|unreachable|trap|command stalled/iu.test(message.text())) faults.push(`${message.type()}: ${message.text()}`.slice(0, 240));
  });
  const row: ToolRunRow = { key: `${program.pluginId}/${program.appId}`, toolId: program.toolId, mutating: program.mutating, a: { timeline: [] }, b: { timeline: [] }, faults: [], pass: false, verdict: "not run" };
  const note = (run: RunRecord, label: string, state: Sample): void => {
    const last = run.timeline.at(-1);
    const summary = `${latestRun(state)?.status ?? "-"}|${latestRun(state)?.text ?? "-"}|${state.tasks.map((task) => `${task.state}:${task.progress}`).join(",") || "-"}|${state.checkin}`;
    if (last?.summary !== summary || label !== "sample") run.timeline.push({ label, summary, stall: state.stall });
  };
  try {
    await page.goto(options.baseUrl, { waitUntil: "commit", timeout: 300_000 });
    await awaitBeacon(page, Date.now() + 300_000);
    await dismissIntroduction(page);
    await page.waitForTimeout(2_000);
    if (options.locale !== "en") row.locale = await seatLocale(page, options.locale);
    await page.keyboard.press("Escape").catch(() => undefined);
    const opened = await openTools(page, program);
    Object.assign(row, { open: opened.missing, example: opened.examplePicked, exampleOptions: opened.exampleOptions });
    if (opened.missing) {
      row.verdict = opened.missing;
      return row;
    }
    const before = await readHistory(page);
    Object.assign(row.a, { editsBefore: before.edits, checkinBefore: before.checkin, via: await start(page) });
    const deadline = Date.now() + options.runMs;
    let paused = false;
    const progressSeen = new Set<string>();
    while (Date.now() < deadline && !options.signal.aborted) {
      await page.waitForTimeout(400);
      const state = await sample(page);
      note(row.a, "sample", state);
      const run = latestRun(state);
      if (run) row.a.started = true;
      if (run?.text && /\d/u.test(run.text)) progressSeen.add(run.text);
      const task = state.tasks[0];
      if (!paused && options.pause && run && !TERMINAL.test(run.status)) {
        paused = true;
        row.a.pauseVia = await control(page, "pause", task?.id);
        await page.waitForTimeout(2_000);
        const heldA = await sample(page);
        await page.waitForTimeout(2_000);
        const heldB = await sample(page);
        note(row.a, "paused", heldB);
        row.a.pausedFrozen = JSON.stringify(latestRun(heldA)?.text) === JSON.stringify(latestRun(heldB)?.text) && !TERMINAL.test(latestRun(heldB)?.status ?? "");
        row.a.pausedState = `${latestRun(heldB)?.status ?? "-"} / ${heldB.tasks.map((entry) => entry.state).join(",")}`;
        row.a.resumeVia = await control(page, "resume", heldB.tasks[0]?.id ?? task?.id);
        await page.waitForTimeout(1_500);
        note(row.a, "resumed", await sample(page));
      }
      if (run && READY_TO_FINALIZE.test(run.status) && !row.a.finalizeVia) {
        row.a.readyToFinalize = run.status;
        const finalize = page.locator('[data-ui-node-key$=".toolRunFinalize"]').first();
        row.a.finalizeVia = (await finalize.count()) ? await finalize.click({ force: true }).then(() => "panel", () => "panel-unclickable") : "absent";
        await page.waitForTimeout(1_500);
        continue;
      }
      if (run && TERMINAL.test(run.status)) {
        row.a.terminal = run.status;
        break;
      }
    }
    if (!paused) row.a.pauseVia = options.pause ? "finished before a pause" : "not paused (--no-pause)";
    row.a.progressSeen = [...progressSeen].slice(0, 6);
    await page.waitForTimeout(2_500);
    const after = await readHistory(page);
    const terminalA = String(row.a.terminal ?? "");
    Object.assign(row.a, { editsAfter: after.edits, checkinAfter: after.checkin, historyTail: after.history, documentChanged: after.render !== before.render, newUndoableRows: after.undoableRows - before.undoableRows });
    row.a.committed = COMMITTED.test(terminalA) && (row.a.documentChanged === true || after.edits > before.edits) && after.undoableRows - before.undoableRows > 0;
    if (COMMITTED.test(terminalA)) {
      row.a.undo = await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]');
      if (row.a.undo !== "ok") {
        await page.keyboard.press(process.platform === "darwin" ? "Meta+KeyZ" : "Control+KeyZ");
        row.a.undo = "chord";
      }
      await page.waitForTimeout(3_000);
      const undone = await readHistory(page);
      Object.assign(row.a, { editsAfterUndo: undone.edits, undoRestored: undone.render === before.render });
      row.a.undone = row.a.committed === true && undone.render === before.render && undone.edits === before.edits;
    }
    const beforeB = await sample(page);
    row.b.editsBefore = (await readHistory(page)).edits;
    const knownScopes = new Set(beforeB.runs.map((run) => run.scope));
    row.b.via = await start(page);
    const deadlineB = Date.now() + 60_000;
    let aborted = false;
    while (Date.now() < deadlineB && !options.signal.aborted) {
      await page.waitForTimeout(250);
      const state = await sample(page);
      note(row.b, "sample", state);
      const run = state.runs.filter((entry) => !knownScopes.has(entry.scope)).at(-1) ?? null;
      if (run && !aborted && !TERMINAL.test(run.status)) {
        aborted = true;
        row.b.abortVia = await control(page, "abort", state.tasks[0]?.id);
      }
      if (run && TERMINAL.test(run.status)) {
        row.b.terminal = run.status;
        break;
      }
    }
    if (!aborted) row.b.abortVia = "finished before an abort";
    await page.waitForTimeout(2_000);
    row.b.editsAfter = (await readHistory(page)).edits;
    row.b.aborted = ABORTED.test(String(row.b.terminal ?? "")) && row.b.editsAfter === row.b.editsBefore;
    const contractA = program.mutating ? row.a.committed === true && row.a.undone === true : COMMITTED.test(terminalA) && after.edits === before.edits;
    row.verdict = program.mutating
      ? (contractA ? "commit+undo PASS" : `commit ${row.a.committed ? "ok" : "MISSING"} / undo ${row.a.undone ? "ok" : "MISSING"}`)
      : (contractA ? "read-only PASS (finalized, nothing written)" : `read-only ${terminalA || "no terminal"}`);
    row.pass = contractA && row.b.aborted === true;
    await page.screenshot({ path: join(options.outDir, `${row.key.replace(/[^A-Za-z0-9]+/gu, "-")}.png`) }).catch(() => undefined);
  } catch (error) {
    row.error = String(error).split("\n")[0]!.slice(0, 300);
    row.verdict = `probe error: ${String(row.error)}`;
  } finally {
    row.faults = faults.slice(0, 8);
    await context.close();
  }
  return row;
}

/** ⏯️ Runs every selected program's two runs, one fresh browser context per program; the report is rewritten after every row
 * and the signal ends the run after the current program. */
export async function runToolRunMatrix(options: ToolRunMatrixOptions): Promise<{ rows: ToolRunRow[]; cancelled: boolean; fatal?: string }> {
  mkdirSync(options.outDir, { recursive: true });
  const programs = readToolPrograms().filter((program) => options.only.length === 0 || options.only.some((entry) => `${program.pluginId}/${program.appId}`.includes(entry)));
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  const report = { baseUrl: options.baseUrl, tag: options.tag, locale: options.locale, started: new Date().toISOString(), finished: null as string | null, rows: [] as ToolRunRow[], cancelled: false };
  const flush = (): void => writeFileSync(join(options.outDir, "tool-run.json"), JSON.stringify(report, null, 1));
  try {
    for (const [index, program] of programs.entries()) {
      if (options.signal.aborted) {
        report.cancelled = true;
        break;
      }
      const row = await runRow(browser, program, options);
      report.rows.push(row);
      flush();
      console.log(`[tool-run] ${index + 1}/${programs.length} ${row.pass ? "PASS" : "FAIL"} ${row.key} tool=${row.toolId} A: start=${String(row.a.started ?? false)} progress=${(row.a.progressSeen as string[] | undefined)?.length ?? 0} pause=${String(row.a.pauseVia ?? "-")}${row.a.pausedFrozen === undefined ? "" : row.a.pausedFrozen ? "(frozen)" : "(NOT frozen)"} terminal=${String(row.a.terminal ?? "-")} committed=${String(row.a.committed ?? false)} undone=${String(row.a.undone ?? "-")} | B: abort=${String(row.b.abortVia ?? "-")} terminal=${String(row.b.terminal ?? "-")} aborted=${String(row.b.aborted ?? false)} | faults=${row.faults.length} | ${row.verdict}`);
    }
  } finally {
    report.finished = new Date().toISOString();
    flush();
    await browser.close();
  }
  return { rows: report.rows, cancelled: report.cancelled };
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `verify tool-run --serve <url> [--tag <t>] [--locale en|de] [--only <plugin/appId substring>,…] [--no-pause]
 * [--run-ms <n>] [--out <dir>]` — runs against the local-only serve `--serve` names (reused, or started and stopped by
 * `withDevServe`), writes `tool-run.json` + screenshots under `<out>/<tag>/`, publishes the acceptance record, exits non-zero
 * unless every selected program passes. */
export async function runToolRunMatrixCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const serveUrl = flagValue(segments, "--serve");
  if (!serveUrl) throw new Error("usage: verify tool-run --serve <url> [--tag <t>] [--locale en|de] [--only …] [--no-pause] [--run-ms <n>] [--out <dir>]");
  const locale = flagValue(segments, "--locale") ?? "en";
  const tag = flagValue(segments, "--tag") ?? `tool-run-${locale}`;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  await withAcceptanceRecord(repoRoot, "tool-run-matrix", () => withDevServe(repoRoot, "tool-run-matrix", { serveUrl, locale, signal: controller.signal, startedAt }, async (baseUrl) => {
    const outDir = join(resolve(flagValue(segments, "--out") ?? defaultOutDir), tag);
    const report = await runToolRunMatrix({ baseUrl, tag, locale, only: (flagValue(segments, "--only") ?? "").split(",").filter(Boolean), pause: !segments.includes("--no-pause"), runMs: Number(flagValue(segments, "--run-ms") ?? 120_000), outDir, signal: controller.signal });
    const passed = report.rows.filter((row) => row.pass).length;
    const total = report.rows.length;
    const failing = report.rows.filter((row) => !row.pass).map((row) => `${row.key}: ${row.verdict}${row.b.aborted === false ? " / abort" : ""}`);
    const unreachable = total > 0 && report.rows.every((row) => /ERR_CONNECTION_REFUSED/u.test(String(row.error ?? "")));
    const status = unreachable ? "blocked" : report.cancelled || total === 0 ? "fail" : passed === total ? "pass" : "fail";
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check: "tool-run-matrix",
        status,
        startedAt,
        measured: { locale, programs: total, passed, failed: total - passed, cancelled: report.cancelled, faultRows: report.rows.filter((row) => row.faults.length > 0).length },
        summary: {
          en: `${passed}/${total} tool programs run, pause, commit as declared, undo and abort in ${locale}${failing.length ? `; failing: ${failing.slice(0, 4).join("; ")}` : ""}`,
          de: `${passed}/${total} Werkzeugprogramme laufen, pausieren, schreiben wie deklariert, lassen sich rückgängig machen und abbrechen in ${locale}${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 4).join("; ")}` : ""}`,
        },
        evidence: [join(outDir, "tool-run.json")],
      }),
    );
    console.log(`[tool-run] === ${tag}: PASS ${passed}/${total} → ${outDir} ===`);
    if (status !== "pass") process.exitCode = 1;
  }));
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}

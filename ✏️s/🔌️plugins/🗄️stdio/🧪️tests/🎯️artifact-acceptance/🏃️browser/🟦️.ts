/** 🏃️ Browser journey per well-known stdio artifact through the real OS shell, driven by the fixture matrix.
 *
 * Per row: OPEN the editor program from the command palette (the shell's own open plan) → RENDERED body → one EDIT through the program's own rail verb or rendered control (`mutateUndoRedo` pins) → SAVE acknowledgment (the `Check in (n)` count moves, `#s-checkin` pressed, count and notices read back) → CLOSE by its dock tab → REOPEN → RELOAD the page and REOPEN → EXPORT pressed in the rail, the download judged by the row's third-party oracle → INVALID input handed to the Import picker must raise a refusal notice and no page fault. Every step writes `pass | fail | skipped` with its evidence; rows with no shipped surface fail at `open`.
 *
 * `run --serve <url> [--only <artifact,…>] [--batch A,B] [--locale en|de] [--tag <t>] [--out <dir>] [--headed]`
 * @see ../🧫️fixtures/🔣️.json
 * @see ../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, Download, FileChooser, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "@semio-tech/framework-plugin/build/plan";
import { ensureParityPlaywrightBrowsersPath } from "@semio-tech/framework-os-dev/parity/execution";
import { FAULT, NOISE, awaitBeacon, clickUncovered, dismissIntroduction, mutateUndoRedo, readShell, seatLocale, unfoldActionsRail, windowIds, withDevServe, witness } from "@semio-tech/framework-os-dev/testing/program-matrix";
import { ACCEPTANCE_DIRECTORY, REPO_ROOT, hexBytes, readMatrix, type AcceptanceRow } from "../📇️matrix/🟦️.ts";
import { judgeBytes } from "../🔬️oracles/🟦️.ts";

//#region 🔖️Report
export type StepVerdict = Readonly<{ status: "pass" | "fail" | "skipped"; detail: string }>;
export const STEPS = ["open", "render", "edit", "save", "close", "reopen", "reload-reopen", "export", "invalid"] as const;
export type StepName = (typeof STEPS)[number];
export type RowReport = { artifact: string; batch: string; kind: string; steps: Partial<Record<StepName, StepVerdict>>; faults: string[]; pass: boolean };
export type JourneyReport = { baseUrl: string; tag: string; locale: string; started: string; finished?: string; rows: RowReport[]; fatal?: string };

/** 📋️ Markdown matrix: one line per artifact, one column per step. */
export function journeyTable(report: JourneyReport): string {
  const mark = { pass: "ok", fail: "FAIL", skipped: "-" } as const;
  const lines = [`| batch | artifact | ${STEPS.join(" | ")} | verdict |`, `| --- | --- | ${STEPS.map(() => "---").join(" | ")} | --- |`];
  for (const row of report.rows) lines.push(`| ${row.batch} | ${row.artifact} | ${STEPS.map((step) => (row.steps[step] ? mark[row.steps[step]!.status] : "-")).join(" | ")} | ${row.pass ? "PASS" : "FAIL"} |`);
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Report

//#region 🔖️Journey
type Program = Readonly<{ pluginId: string; appId: string }>;
const ok = (detail: string): StepVerdict => ({ status: "pass", detail });
const no = (detail: string): StepVerdict => ({ status: "fail", detail });
const skipped = (detail: string): StepVerdict => ({ status: "skipped", detail });
const kindOf = (appId: string): string => /^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId;

async function drive(page: Page, row: AcceptanceRow, locale: string, outDir: string, faults: string[], refusals: string[], downloads: Download[], choosers: { next: Uint8Array | null; name: string }): Promise<Partial<Record<StepName, StepVerdict>>> {
  const steps: Partial<Record<StepName, StepVerdict>> = {};
  const notices = (): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll("[data-semio-transient-notice]")].map((element) => `${element.getAttribute("data-notice-code") ?? ""}|${(element.textContent ?? "").trim()}`));
  const programs = await page.evaluate(() => ((window as unknown as { __semioOsCatalogProbe?: { programs: Program[] } }).__semioOsCatalogProbe?.programs ?? []) as Program[]);
  const program = programs.find((entry) => kindOf(entry.appId) === row.artifact && entry.appId.endsWith("#editor") && entry.pluginId === row.plugin) ?? programs.find((entry) => kindOf(entry.appId) === row.artifact && entry.appId.endsWith("#editor"));
  const openProgram = async (): Promise<string | null> => {
    const before = await windowIds(page);
    await dismissIntroduction(page);
    await page.evaluate(() => (document.activeElement instanceof HTMLElement ? document.activeElement.blur() : undefined));
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
    if ((await input.count()) === 0) return "command palette never opened";
    await input.fill(row.artifact);
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${program!.pluginId}.${program!.appId}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) === 0) { await page.keyboard.press("Escape"); return `no palette row spawn.${program!.pluginId}.${program!.appId}`; }
    await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }));
    const deadline = Date.now() + 120_000;
    while (Date.now() < deadline) {
      if ((await windowIds(page)).some((id) => !before.includes(id))) { await page.waitForTimeout(3_500); return null; }
      await page.waitForTimeout(250);
    }
    return `no new window; notices ${JSON.stringify((await notices()).slice(-2))}`;
  };
  const freshWindows = async (before: readonly string[]): Promise<string[]> => (await windowIds(page)).filter((id) => !before.includes(id));
  const bodyRendered = async (): Promise<string | null> => {
    const shell = await readShell(page);
    return shell.panes.some((pane) => pane.chars > 0 || pane.svg > 0 || pane.canvases > 0) ? null : `no pane painted (${shell.panes.length} panes)`;
  };
  const closeNewest = async (ids: readonly string[]): Promise<string[]> => {
    for (const id of ids) await page.evaluate((windowId) => { const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId); const button = tab?.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab?.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]'); if (button instanceof HTMLElement) button.click(); }, id);
    await page.waitForTimeout(1_500);
    return (await windowIds(page)).filter((id) => ids.includes(id));
  };
  const railIds = async (): Promise<string[]> => (await unfoldActionsRail(page), (await readShell(page)).actions.map((id) => id.replace(/^action\./u, "")));

  if (program === undefined) { steps.open = no(`no #editor program of ${row.kind} in the shell catalog probe (surface ${row.surface})`); return steps; }
  const before = await windowIds(page);
  const failure = await openProgram();
  steps.open = failure === null ? ok(`${program.pluginId} ${program.appId}`) : no(failure);
  if (failure !== null) return steps;
  const opened = await freshWindows(before);
  const unpainted = await bodyRendered();
  steps.render = unpainted === null && faults.length === 0 ? ok("body painted, no fault") : no(unpainted ?? faults.slice(-2).join(" | "));

  if (row.edit.mode === "none") steps.edit = skipped("no shipped surface");
  else {
    const key = `${program.pluginId}/${row.artifact}`;
    const verbs = row.edit.verb ? { [key]: row.edit.verb } : row.edit.path === "details" ? { [key]: "setSnapshotValue" } : {};
    const args = row.edit.verb && row.edit.args ? { [`${key}.${row.edit.verb}`]: row.edit.args } : {};
    const detailsVerb = row.edit.verb ?? "setSnapshotValue";
    const detailsControl = row.edit.path === "details" && row.edit.field ? `[data-ui-node-key="edit"][aria-label^="${row.edit.field}:"]` : null;
    const edits = detailsControl !== null ? { [`${key}.${detailsVerb}`]: { control: detailsControl, value: row.edit.value ?? "" } } : row.edit.mode === "rendered" && row.edit.verb && row.edit.control ? { [`${key}.${row.edit.verb}`]: { control: row.edit.control, value: row.edit.value ?? "" } } : {};
    const result = await mutateUndoRedo(page, refusals, key, verbs, args, edits, "@liveId", 6);
    steps.edit = result.mutation !== null && "mutated" in result && result.mutated ? ok(`${result.mutation}: ${JSON.stringify(result.mutationDetail ?? "applied and undo/redo differ")}`) : no(`${result.mutationDetail ?? "no verb moved the document"} (rail ${result.railRows} rows)`);
  }
  const dirty = witness(await readShell(page)).edits;
  if (steps.edit?.status !== "pass") steps.save = skipped("nothing to save");
  else {
    const checkin = page.locator("#s-checkin").first();
    if ((await checkin.count()) === 0) steps.save = no("no #s-checkin control");
    else {
      const seen = (await notices()).length;
      await clickUncovered(page, "#s-checkin");
      await page.waitForTimeout(3_000);
      const after = witness(await readShell(page)).edits;
      steps.save = dirty > 0 && after < dirty ? ok(`check-in count ${dirty} -> ${after}`) : no(`check-in count ${dirty} -> ${after}; notices ${JSON.stringify((await notices()).slice(seen))}`);
    }
  }

  const stuck = await closeNewest(opened);
  steps.close = stuck.length === 0 ? ok("dock tab closed") : no(`still open: ${stuck.join(",")}`);
  const beforeReopen = await windowIds(page);
  const reopenFailure = await openProgram();
  const reopenPainted = reopenFailure === null ? await bodyRendered() : reopenFailure;
  steps.reopen = reopenPainted === null ? ok("reopened and painted") : no(reopenPainted);
  const reopened = await freshWindows(beforeReopen);

  await page.reload({ waitUntil: "commit" });
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await seatLocale(page, locale);
  const afterReload = await windowIds(page);
  const reloadFailure = await openProgram();
  const reloadPainted = reloadFailure === null ? await bodyRendered() : reloadFailure;
  steps["reload-reopen"] = beacon?.startsWith("ready") && reloadPainted === null ? ok(`${beacon}; reopened and painted (${afterReload.length} windows survived the reload)`) : no(reloadPainted ?? String(beacon));

  if (row.surface === "missing") { steps.export = skipped("no shipped surface"); steps.invalid = skipped("no shipped surface"); return steps; }
  const rail = await railIds();
  const exportVerb = rail.find((id) => /export/iu.test(id) && !/^exportDocument$/u.test(id)) ?? rail.find((id) => /export/iu.test(id));
  if (exportVerb === undefined) steps.export = no(`no export row in the rail (${rail.length} rows)`);
  else {
    const cursor = downloads.length;
    await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${exportVerb}"]`);
    const deadline = Date.now() + 60_000;
    while (downloads.length === cursor && Date.now() < deadline) await page.waitForTimeout(250);
    const download = downloads[cursor];
    if (download === undefined) steps.export = no(`${exportVerb} produced no download`);
    else {
      mkdirSync(join(outDir, "exports"), { recursive: true });
      const path = join(outDir, "exports", `${row.artifact}-${download.suggestedFilename().replace(/[^A-Za-z0-9._-]+/gu, "_")}`);
      await download.saveAs(path);
      const bytes = new Uint8Array(readFileSync(path));
      const verdict = judgeBytes(row.export.format, bytes);
      steps.export = verdict.ok ? ok(`${exportVerb}: ${download.suggestedFilename()} ${bytes.length} bytes, ${verdict.detail}`) : no(`${exportVerb}: ${download.suggestedFilename()} refused by ${row.export.format} oracle: ${verdict.detail}`);
    }
  }
  const importVerb = rail.find((id) => /import/iu.test(id));
  if (importVerb === undefined) steps.invalid = no(`no import row in the rail (${rail.length} rows)`);
  else {
    choosers.next = hexBytes(row.invalid.bytes);
    choosers.name = `invalid${row.extension}`;
    const seen = (await notices()).length;
    const refused = refusals.length;
    const faulted = faults.length;
    await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${importVerb}"]`);
    await page.waitForTimeout(4_000);
    const raised = (await notices()).slice(seen);
    const refusal = refusals.slice(refused);
    const pageErrors = faults.slice(faulted).filter((line) => line.startsWith("pageerror"));
    steps.invalid = choosers.next !== null ? no(`${importVerb} never opened the file picker`) : raised.length + refusal.length > 0 && pageErrors.length === 0 ? ok(`refused: ${JSON.stringify([...raised, ...refusal].slice(0, 2))}`) : no(`no refusal notice for invalid input (${pageErrors.length} page errors)`);
  }
  await closeNewest(reopened).catch(() => undefined);
  return steps;
}

/** 🧭️ Runs the selected rows against the shell at `baseUrl`. */
export async function runJourney(options: Readonly<{ baseUrl: string; tag: string; locale: string; only: readonly string[]; batches: readonly string[]; outDir: string; headed: boolean; signal: AbortSignal }>): Promise<JourneyReport> {
  const outDir = join(options.outDir, options.tag);
  mkdirSync(outDir, { recursive: true });
  const report: JourneyReport = { baseUrl: options.baseUrl, tag: options.tag, locale: options.locale, started: new Date().toISOString(), rows: [] };
  const flush = (): void => writeFileSync(join(outDir, "journey.json"), JSON.stringify(report, null, 1));
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser: Browser = await chromium.launch({ headless: !options.headed, args: ["--use-angle=swiftshader"] });
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
    page.setDefaultNavigationTimeout(180_000);
    const faults: string[] = [];
    const refusals: string[] = [];
    const downloads: Download[] = [];
    const choosers: { next: Uint8Array | null; name: string } = { next: null, name: "" };
    page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 300)));
    page.on("download", (download) => downloads.push(download));
    page.on("filechooser", (chooser: FileChooser) => {
      const payload = choosers.next;
      choosers.next = null;
      void chooser.setFiles(payload === null ? [] : [{ name: choosers.name, mimeType: "application/octet-stream", buffer: Buffer.from(payload) }]);
    });
    page.on("console", (message) => {
      const text = message.text();
      if (/WebSocket connection to 'ws:\/\/127\.0\.0\.1:\d+\/bridge' failed/u.test(text)) return;
      if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
      if (FAULT.test(text) && !NOISE.test(text)) faults.push(`${message.type()}: ${text}`.slice(0, 300));
    });
    await page.goto(options.baseUrl, { waitUntil: "commit", timeout: 300_000 });
    const beacon = await awaitBeacon(page, Date.now() + 300_000);
    if (!beacon?.startsWith("ready")) throw new Error(`shell never became ready: ${beacon}`);
    await dismissIntroduction(page);
    await seatLocale(page, options.locale);
    const rows = readMatrix().artifacts.filter((row) => (options.only.length === 0 || options.only.includes(row.artifact)) && (options.batches.length === 0 || options.batches.includes(row.batch)));
    for (const row of rows) {
      if (options.signal.aborted) { report.fatal = "cancelled"; break; }
      const faultMark = faults.length;
      const entry: RowReport = { artifact: row.artifact, batch: row.batch, kind: row.kind, steps: {}, faults: [], pass: false };
      try {
        entry.steps = await drive(page, row, options.locale, outDir, faults, refusals, downloads, choosers);
      } catch (error) {
        entry.steps.open ??= no(`journey threw: ${String(error).split("\n")[0]!.slice(0, 200)}`);
      }
      entry.faults = faults.slice(faultMark).slice(0, 5);
      entry.pass = STEPS.every((step) => entry.steps[step]?.status === "pass");
      report.rows.push(entry);
      flush();
      console.log(`[artifact-acceptance] ${row.batch}/${row.artifact}: ${entry.pass ? "PASS" : "FAIL"} ${STEPS.map((step) => `${step}=${entry.steps[step]?.status ?? "-"}`).join(" ")}`);
    }
  } catch (error) {
    report.fatal = String(error).split("\n")[0]!.slice(0, 300);
  } finally {
    await browser.close();
  }
  report.finished = new Date().toISOString();
  flush();
  writeFileSync(join(outDir, "table.md"), journeyTable(report));
  return report;
}

const flag = (segments: readonly string[], name: string): string | undefined => { const value = segments[segments.indexOf(name) + 1]; return segments.includes(name) && value !== undefined && !value.startsWith("--") ? value : undefined; };

/** 🚪️ CLI entry: `run --serve <url> …`; exits non-zero unless every selected row passes every step. */
export async function runJourneyCli(segments: readonly string[]): Promise<void> {
  const serveUrl = flag(segments, "--serve");
  if (!serveUrl) throw new Error("usage: run --serve <url> [--only <artifact,…>] [--batch A,B] [--locale en|de] [--tag <t>] [--out <dir>] [--headed]");
  const locale = flag(segments, "--locale") ?? "en";
  const controller = new AbortController();
  process.once("SIGINT", () => controller.abort());
  process.once("SIGTERM", () => controller.abort());
  await withDevServe(REPO_ROOT, "stdio-artifact-acceptance", { serveUrl, locale, signal: controller.signal, startedAt: new Date() }, async (baseUrl) => {
    const report = await runJourney({ baseUrl, tag: flag(segments, "--tag") ?? locale, locale, only: (flag(segments, "--only") ?? "").split(",").filter(Boolean), batches: (flag(segments, "--batch") ?? "").split(",").filter(Boolean), outDir: resolve(flag(segments, "--out") ?? join(ACCEPTANCE_DIRECTORY, "🤖️generated")), headed: segments.includes("--headed"), signal: controller.signal });
    const failed = report.rows.filter((row) => !row.pass).map((row) => row.artifact);
    console.log(`[artifact-acceptance] ${report.rows.length - failed.length}/${report.rows.length} artifacts pass${report.fatal ? `; fatal: ${report.fatal}` : ""}${failed.length ? `; failing: ${failed.join(", ")}` : ""}`);
    if (report.fatal || failed.length > 0 || report.rows.length === 0) process.exitCode = 1;
  });
}

if (import.meta.main) await runJourneyCli(process.argv.slice(2).filter((segment, index) => !(index === 0 && segment === "run")));
//#endregion 🔖️Journey

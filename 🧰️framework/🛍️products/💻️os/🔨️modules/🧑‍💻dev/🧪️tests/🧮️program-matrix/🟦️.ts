/** 🧮️ The editor × viewer × locale matrix of the served `s` React shell, as a user drives it.
 *
 * One row per spawnable program the shell's own catalog probe lists (`window.__semioOsCatalogProbe.programs`), opened
 * from the Home landing through the command palette by the shell chord, judged on: open · body RENDERED (not the
 * skeleton, no window fault, own content beside the chrome) · every Actions/Utilities chip HIT-TESTABLE by a pointer ·
 * one real verb from the Actions rail · undo · redo (the History ledger, `Check in (n)` and a structural render digest;
 * the edit count must read a clean round trip `[e, e+1, e, e+1]`) · the verb's own History row in the run's locale ·
 * 0 fault lines · closed by its dock tab. Viewers are opened, render-checked, chip-checked and closed.
 *
 * The oracles are third-party: Chromium's own hit-testing (`elementFromPoint`) for every chip, and the framework's own
 * ledger and check-in count for every edit. Verb pins, staged arguments and pre-verbs are data
 * (`🧑‍💻dev/🧫️fixtures/🧮️program-matrix.json`); each 📕️norm standard stages its own first committed mutation fixture's `➡️after` snapshot.
 *
 * Promoted from the session-12/13 ticket harness `wp-s15/s15-matrix.mjs` + the S6 witness `🐍️s6-all-kinds-sweep.mjs`
 * (ticket 26/09/23, measured there 75/75 editors en + de and 70/70 viewers on restage4).
 * @see ../🔬️catalog-smoke/🟦️.ts — the open-and-render-only sibling
 */

import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

//#region 🔖️Pins
/** 📌️ `semio.os-dev.program-matrix-pins/v1` — the verb each kind is driven with and what it needs staged. */
export type MatrixPins = Readonly<{
  schema: "semio.os-dev.program-matrix-pins/v1";
  liveId: string;
  excludedKinds: readonly string[];
  pluginVerbs: Readonly<Record<string, string>>;
  pluginArgs: Readonly<Record<string, Readonly<Record<string, string>>>>;
  kindVerbs: Readonly<Record<string, string>>;
  kindPre: Readonly<Record<string, string>>;
  kindArgs: Readonly<Record<string, Readonly<Record<string, string>>>>;
  kindOrigin: Readonly<Record<string, string>>;
}>;

/** 📌️ Reads the pins (`🧑‍💻dev/🧫️fixtures/🧮️program-matrix.json`). */
export function readMatrixPins(): MatrixPins {
  const pins = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "🧫️fixtures", "🧮️program-matrix.json"), "utf8")) as MatrixPins;
  if (pins.schema !== "semio.os-dev.program-matrix-pins/v1") throw new Error(`program matrix pins: unexpected schema ${String(pins.schema)}`);
  return pins;
}

/** 🗂️ One spawnable program of the shell's catalog probe. */
type MatrixProgram = Readonly<{ pluginId: string; appId: string }>;

const kindOf = (appId: string): string => /^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId;
const roleOf = (appId: string): string => appId.split("#").at(-1) ?? "editor";
const subsetOf = (appId: string): string => /@[^/]+\/([^#]+)#/u.exec(appId)?.[1] ?? "*";
const baseKeyOf = (program: MatrixProgram): string => `${program.pluginId}/${kindOf(program.appId)}`;
const keyOf = (program: MatrixProgram): string => `${baseKeyOf(program)}${subsetOf(program.appId) === "*" ? "" : `/${subsetOf(program.appId)}`}`;

/** 📕️ Each norm standard's `setSnapshot` stages ITS OWN codec-canonical document: the `➡️after` snapshot of the
 * standard's first committed mutation fixture, folded to one line by deleting line breaks only — never re-serialized, so
 * f64 carriers keep their exact JSON form. */
function normSnapshot(repoRoot: string, kind: string): string | null {
  const root = join(repoRoot, "✏️s", "🔌️plugins", "📕️norm", "🗿️artifacts");
  const dir = readdirSync(root).find((name) => name.endsWith(kind));
  if (!dir) return null;
  const mutations = join(root, dir, "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures", "🧬️mutations");
  if (!existsSync(mutations)) return null;
  for (const mutation of readdirSync(mutations).sort()) {
    for (const example of readdirSync(join(mutations, mutation)).sort()) {
      const path = join(mutations, mutation, example, "📸️snapshot", "➡️after", "🔣️.json");
      if (existsSync(path)) return readFileSync(path, "utf8").replace(/\s*\n\s*/gu, "");
    }
  }
  return null;
}

/** 🗺️ Resolves every selected program's verb and staged arguments, keyed by the row key `<plugin>/<kind>[/<subset>]`:
 * a kind pin beats the plugin pin of the kind's ORIGIN plugin (📽️demonstrator re-hosts other plugins' kinds). */
function resolvePins(repoRoot: string, pins: MatrixPins, programs: readonly MatrixProgram[]): { verbs: Record<string, string>; args: Record<string, Readonly<Record<string, string>>> } {
  const verbs: Record<string, string> = { ...pins.pluginVerbs };
  const args: Record<string, Readonly<Record<string, string>>> = { ...pins.pluginArgs };
  for (const program of programs) {
    const key = keyOf(program);
    const base = baseKeyOf(program);
    const origin = pins.kindOrigin[base] ?? program.pluginId;
    const verb = pins.kindVerbs[key] ?? pins.kindVerbs[base] ?? pins.pluginVerbs[origin];
    if (verb) verbs[key] = verb;
    for (const [argKey, value] of Object.entries(pins.pluginArgs)) if (argKey.startsWith(`${origin}.`)) args[`${key}.${argKey.slice(origin.length + 1)}`] = value;
    for (const [argKey, value] of Object.entries(pins.kindArgs)) if (argKey.startsWith(`${base}.`)) args[`${key}.${argKey.slice(base.length + 1)}`] = value;
    if (key !== base) for (const [argKey, value] of Object.entries(pins.kindArgs)) if (argKey.startsWith(`${key}.`)) args[argKey] = value;
    if (program.pluginId === "norm") {
      const snapshot = normSnapshot(repoRoot, kindOf(program.appId));
      if (snapshot !== null) args[`${key}.setSnapshot`] = { snapshot };
    }
  }
  return { verbs, args };
}
//#endregion 🔖️Pins

//#region 🔖️Witness
/** 🚨️ Console lines that count as a fault, and the known noise that never does. */
export const FAULT = /unreachable|trapped|\btrap\b|panicked|fault|refused|dropped action|not-ui-safe|missing-owned|invalid-args|unsupported|pageerror|Uncaught|dispatch-failed/i;
export const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|Failed to load resource: the server responded with a status of 404|Download the (React|Vue) DevTools|typed-operation slots/;

/** 🧾️ The whole shell surface in one evaluation: ledger rows, check-in count, panes, bodies, rail rows. */
type ShellReading = Readonly<{
  windowIds: readonly string[];
  checkin: string | null;
  ledger: readonly Readonly<{ id: string; label: string; dimmed: boolean }>[];
  panes: readonly Readonly<{ id: string | null; canvases: number; svg: number; chars: number }>[];
  actions: readonly string[];
  bodies: readonly string[];
  measures: readonly string[];
}>;

export const readShell = (page: Page): Promise<ShellReading> =>
  page.evaluate(() => {
    const text = (element: Element | null): string => ((element as HTMLElement | null)?.innerText ?? "").replace(/\s+/gu, " ").trim();
    const entries = [...document.querySelectorAll('[id^="framework.history.entry."]')]
      .filter((element) => !element.id.endsWith(".revert"))
      .map((element) => ({ id: element.id, label: text(element).slice(0, 60), dimmed: element.className.includes("opacity") || element.getAttribute("data-dimmed") === "true" }));
    const checkin = document.querySelector("#s-checkin");
    return {
      windowIds: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id): id is string => typeof id === "string"),
      checkin: checkin === null ? null : text(checkin),
      ledger: entries,
      panes: [...document.querySelectorAll("[data-surface-id]")].map((element) => ({ id: element.getAttribute("data-surface-id"), canvases: element.querySelectorAll("canvas").length, svg: element.querySelectorAll("svg *").length, chars: text(element).length })),
      actions: [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((element) => element.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./u.test(id)),
      bodies: [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => {
        const pane = body.querySelector('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"]');
        const paneElements = pane === null ? 0 : pane.querySelectorAll("*").length + 1;
        const paneChars = pane === null ? 0 : text(pane).length;
        return `${body.querySelectorAll("*").length - paneElements}/${text(body).length - paneChars}/${body.querySelectorAll("svg *").length}/${body.querySelectorAll("canvas").length}`;
      }),
      measures: [...document.querySelectorAll('[data-slot="window-measures-body"], [data-slot="window-measure-tree-row"]')].map((row) => text(row).slice(0, 60)),
    };
  });

/** 🔢️ The uncommitted-applied-edit count the check-in button prints as `Check in (N)`; `-1` when it is absent. */
function editCount(shell: ShellReading): number {
  const match = /\((\d+)\)\s*$/u.exec(shell.checkin ?? "");
  return match === null ? (shell.checkin === null ? -1 : 0) : Number(match[1]);
}

/** 🔬️ What an interaction has to move: applied ledger rows, the edit count and a structural render digest (element
 * counts, svg nodes, canvases; the Actions pane subtracted, because unfolding it injects ~50 static labels). */
function witness(shell: ShellReading): { applied: number; ledger: string[]; edits: number; render: string } {
  return {
    applied: shell.ledger.filter((entry) => !entry.dimmed).length,
    ledger: shell.ledger.map((entry) => `${entry.id}${entry.dimmed ? "~" : ""}:${entry.label}`),
    edits: editCount(shell),
    render: JSON.stringify([...shell.panes.map((pane) => `${pane.id}:${pane.chars}:${pane.svg}:${pane.canvases}`), ...shell.bodies, ...shell.measures]),
  };
}

async function awaitBeacon(page: Page, deadline: number): Promise<string | null> {
  while (Date.now() < deadline) {
    const beacon = await page.evaluate(() => {
      const data = document.documentElement.dataset;
      if (data.semioOsReady !== undefined) return `ready:${data.semioOsReady}`;
      if (data.semioOsError !== undefined) return `error:${data.semioOsError}`;
      if (data.semioOsNotFound !== undefined) return `not-found:${data.semioOsNotFound}`;
      return null;
    });
    if (beacon !== null) return beacon;
    await page.waitForTimeout(1000);
  }
  return null;
}

const windowIds = (page: Page): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id): id is string => typeof id === "string"));

type CatalogProbe = Readonly<{ plugins: readonly Readonly<{ pluginId: string; status: string }>[]; programs: readonly MatrixProgram[] }>;
const readProbe = (page: Page): Promise<CatalogProbe | null> => page.evaluate(() => ((window as unknown as { __semioOsCatalogProbe?: unknown }).__semioOsCatalogProbe ?? null) as never);

async function dismissIntroduction(page: Page): Promise<void> {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) return;
    const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip|überspringen/iu }).first();
    if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape").catch(() => undefined);
    await page.waitForTimeout(500);
  }
}

async function click(page: Page, selector: string): Promise<string> {
  const locator = page.locator(selector).first();
  if ((await locator.count()) === 0) return "absent";
  return locator
    .click({ force: true, timeout: 8_000 })
    .then(() => "ok")
    .catch((error: unknown) => String(error).split("\n")[0]!.slice(0, 80));
}

/** 🫥️ Presses a row only after retiring the docked panels that geometrically cover its centre: a forced click on a
 * covered row answers the panel and still reports `ok`. */
export async function clickUncovered(page: Page, selector: string): Promise<string> {
  if ((await page.locator(selector).count()) === 0) return "absent";
  const covering = await page
    .locator(selector)
    .first()
    .evaluate((row) => {
      const rect = row.getBoundingClientRect();
      if (rect.width === 0 || rect.height === 0) return [];
      const x = rect.left + rect.width / 2;
      const y = rect.top + rect.height / 2;
      const top = document.elementFromPoint(x, y);
      if (top === null || row === top || row.contains(top) || top.contains(row)) return [];
      const own = row.closest('[data-slot="panel"]');
      const hits = new Set<string>();
      for (const panel of document.querySelectorAll('[data-slot="panel"]')) {
        if (!(panel instanceof HTMLElement) || panel.offsetParent === null || panel === own) continue;
        const box = panel.getBoundingClientRect();
        if (box.left <= x && box.right >= x && box.top <= y && box.bottom >= y) hits.add(panel.id.replace(/^framework\.panelTab\./u, ""));
      }
      return [...hits];
    })
    .catch(() => [] as string[]);
  if (covering.length > 0) {
    for (const panel of covering) await click(page, `[data-slot="panel-tab-button"][id="${panel}"], [id="${panel}"]`);
    await page.waitForTimeout(1_200);
  }
  return click(page, selector);
}

/** 🎛️ Unfolds the Actions rail of every folded engagement chip until the pane lists rows (polled, never slept through). */
export async function unfoldActionsRail(page: Page, budgetMs = 25_000): Promise<number> {
  const railRowCount = (): Promise<number> =>
    page.evaluate(() => [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((element) => element.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./u.test(id)).length);
  const foldedToggleIds = (): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].filter((toggle) => toggle.closest('[data-folded="true"]') !== null).map((toggle) => toggle.id));
  const clicked = new Set<string>();
  const deadline = Date.now() + budgetMs;
  const chipDeadline = Math.min(deadline, Date.now() + 15_000);
  while (Date.now() < chipDeadline && (await page.locator('[id$=".engagement.toggle"]').count()) === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(2_500);
  let rows = await railRowCount();
  while (rows === 0 && Date.now() < deadline) {
    for (const id of await foldedToggleIds()) {
      clicked.add(id);
      await clickUncovered(page, `[id="${id}"]`);
    }
    await page.waitForTimeout(700);
    rows = await railRowCount();
  }
  await page.waitForTimeout(800);
  return clicked.size;
}

/** 🕰️ Raises the History panel only when it is not already open (its tab toggles). */
async function raiseHistory(page: Page): Promise<string> {
  const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
  if (open) return "already-open";
  const clicked = await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_500);
  return clicked;
}

/** 🔎️ Every identifier the focused program currently prints, newest surfaces first (a pure read). */
const liveDocumentIds = (page: Page): Promise<string[]> =>
  page.evaluate(() => {
    const ids: string[] = [];
    const push = (value: string): void => {
      const trimmed = value.trim();
      if (trimmed.length > 0 && trimmed.length <= 64 && !ids.includes(trimmed)) ids.push(trimmed);
    };
    for (const element of document.querySelectorAll('[data-slot="window-body"] [data-row-id], [data-slot="window-body"] [data-node-id], [data-slot="window-body"] [data-feature-id], [data-slot="window-body"] [data-item-id], [data-slot="window-body"] [data-zone-id]')) {
      for (const name of ["data-row-id", "data-node-id", "data-feature-id", "data-item-id", "data-zone-id"]) push(element.getAttribute(name) ?? "");
    }
    for (const row of document.querySelectorAll('[data-slot="window-measure-tree-row"], [data-slot="window-measures-body"] [id]')) {
      push(row.getAttribute("data-measure-id") ?? "");
      const head = /^([A-Za-z0-9][A-Za-z0-9._-]{0,63})\b/u.exec((row.textContent ?? "").trim());
      if (head) push(head[1]!);
    }
    for (const entry of document.querySelectorAll('[id^="framework.history.entry."]')) for (const match of (entry.textContent ?? "").matchAll(/\bid=([A-Za-z0-9._-]{1,64})/gu)) push(match[1]!);
    return ids;
  });

const errorLine = (key: string, error: unknown): string => `${key}:${String(error).split("\n")[0]!.slice(0, 60)}`;

/** 🧾️ Fills one staged argument of an unfolded action form: native `<select>`, shadcn combobox, range input, Radix
 * slider thumb or plain input. The live-id token resolves a document id from the program's own surfaces. */
export async function fillStagedArgument(page: Page, key: string, value: string, liveId: string): Promise<string> {
  const live = value === liveId;
  for (const scope of [`[data-slot="window-action-pane"] `, ""]) {
    const select = page.locator(`${scope}select[id$=".arg.${key}"], ${scope}select[id$="${key}"], ${scope}select[name="${key}"]`).first();
    if ((await select.count()) > 0) {
      const options = await select.evaluate((element) => [...(element as HTMLSelectElement).options].map((option) => option.value).filter((option) => option.length > 0));
      const chosen = live ? options[0] : options.includes(value) ? value : (options[0] ?? value);
      if (chosen === undefined) return `${key}:no-live-option`;
      return select.selectOption(chosen).then(() => `${key}=${chosen}${chosen === value ? "" : "(live)"}`).catch((error: unknown) => errorLine(key, error));
    }
    const combobox = page.locator(`${scope}[role="combobox"][id$=".arg.${key}"], ${scope}[role="combobox"][id="${key}"], ${scope}[role="combobox"][id$=".${key}"]`).first();
    if ((await combobox.count()) > 0) {
      if (!(await combobox.click({ timeout: 8_000, force: true }).then(() => true).catch(() => false))) return `${key}:combobox-unclickable`;
      await page.waitForTimeout(500);
      const exact = live ? null : page.locator(`[role="option"][data-value="${value}"], [role="option"]:has-text("${value}")`).first();
      if (exact !== null && (await exact.count()) > 0) return exact.click({ timeout: 8_000, force: true }).then(() => `${key}=${value}`).catch((error: unknown) => errorLine(key, error));
      const option = page.locator('[role="option"]').first();
      if ((await option.count()) === 0) return `${key}:no-live-option`;
      const label = ((await option.textContent()) ?? "").trim().slice(0, 40);
      return option.click({ timeout: 8_000, force: true }).then(() => `${key}=${label}(live)`).catch((error: unknown) => errorLine(key, error));
    }
    const input = page.locator(`${scope}[id$=".arg.${key}"]:is(input,textarea), ${scope}[id$="${key}"]:is(input,textarea), ${scope}[name="${key}"]`).first();
    if ((await input.count()) === 0) continue;
    if (live) {
      const candidates = await liveDocumentIds(page);
      if (candidates.length === 0) return `${key}:no-live-id`;
      return input.fill(candidates[0]!).then(() => `${key}=${candidates[0]}(live)`).catch((error: unknown) => errorLine(key, error));
    }
    if ((await input.getAttribute("type")) === "range") {
      return input
        .evaluate((element, next) => {
          Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value")?.set?.call(element, String(next));
          element.dispatchEvent(new Event("input", { bubbles: true }));
          element.dispatchEvent(new Event("change", { bubbles: true }));
          return (element as HTMLInputElement).value;
        }, value)
        .then((seated) => `${key}=${seated}`)
        .catch((error: unknown) => errorLine(key, error));
    }
    return input.fill(value).then(() => `${key}=${value}`).catch((error: unknown) => errorLine(key, error));
  }
  const thumb = live ? null : page.locator('[data-slot="window-action-pane"] [role="slider"], [role="slider"]').first();
  if (thumb !== null && (await thumb.count()) > 0) {
    const now = (): Promise<number> => thumb.getAttribute("aria-valuenow").then(Number);
    await thumb.focus().catch(() => undefined);
    let current = await now();
    for (let step = 0; step < 80 && Math.abs(current - Number(value)) > 1e-9; step += 1) {
      await page.keyboard.press(current < Number(value) ? "ArrowRight" : "ArrowLeft");
      await page.waitForTimeout(60);
      const next = await now();
      if (next === current) break;
      current = next;
    }
    return `${key}=${current}${current === Number(value) ? "" : `(asked ${value})`}`;
  }
  return `${key}:absent`;
}

/** 🚀️ Presses a staged verb's execute control, falling back from the exact pane-scoped id to any execute control. */
export async function submitStagedVerb(page: Page, verbId: string): Promise<string> {
  for (const selector of [`[data-slot="window-action-pane"] [id$=".action.${verbId}.execute"]`, `[id$=".action.${verbId}.execute"]`, `[id$="${verbId}.execute"]`, `[id*="${verbId}"][id$=".execute"]`, `[data-slot="window-action-pane"] [id$=".execute"]:visible`]) {
    const outcome = await click(page, selector);
    if (outcome !== "absent") return outcome;
  }
  return "absent";
}

/** 🫧️ A harmless dispatch that makes the host re-read a spawned program (the host reads it two dispatches behind). */
async function neutralDispatch(page: Page): Promise<string | null> {
  for (const verb of ["clearSelection", "selectAll"]) {
    if ((await click(page, `[data-slot="window-action-pane"] [id="action.${verb}"]`)) === "ok") {
      await page.waitForTimeout(2_000);
      return verb;
    }
  }
  await page.waitForTimeout(2_000);
  return null;
}

type VerbAttempt = Readonly<{ verbId: string; filled: string[]; mutated: boolean; undone: boolean; redone: boolean; redoDiffersFromUndo: boolean; undoLane: string | null; redoLane: string | null; edits: number[]; applied: number[]; refusal: string | null }>;

/** ✏️ One verb → undo → redo, each read two neutral dispatches later; undo/redo through the rail row first, the chord
 * second, and the lane that answered is recorded. */
async function runVerb(page: Page, verbId: string, args: Readonly<Record<string, string>> | undefined, refusals: readonly string[], liveId: string): Promise<VerbAttempt> {
  const cursor = refusals.length;
  await raiseHistory(page);
  let before = witness(await readShell(page));
  await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${verbId}"]`);
  await page.waitForTimeout(1_200);
  const filled: string[] = [];
  for (const [key, value] of Object.entries(args ?? {})) filled.push(await fillStagedArgument(page, key, value, liveId));
  if (filled.length > 0) {
    await page.waitForTimeout(400);
    before = witness(await readShell(page));
  }
  await submitStagedVerb(page, verbId);
  await page.waitForTimeout(2_000);
  await neutralDispatch(page);
  const readVerb = witness(await readShell(page));
  await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]');
  await neutralDispatch(page);
  let readUndo = witness(await readShell(page));
  let undoLane = readUndo.edits < readVerb.edits || readUndo.render !== readVerb.render ? "rail" : null;
  if (undoLane === null) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+KeyZ" : "Control+KeyZ");
    await neutralDispatch(page);
    readUndo = witness(await readShell(page));
    if (readUndo.edits < readVerb.edits || readUndo.render !== readVerb.render) undoLane = "chord";
  }
  await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.redo"]');
  await neutralDispatch(page);
  let readRedo = witness(await readShell(page));
  let redoLane = readRedo.edits > readUndo.edits || readRedo.render !== readUndo.render ? "rail" : null;
  if (redoLane === null) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+Shift+KeyZ" : "Control+Shift+KeyZ");
    await neutralDispatch(page);
    readRedo = witness(await readShell(page));
    if (readRedo.edits > readUndo.edits || readRedo.render !== readUndo.render) redoLane = "chord";
  }
  return {
    verbId,
    filled,
    mutated: readVerb.edits > before.edits || readVerb.applied > before.applied || readVerb.render !== before.render,
    undone: readUndo.edits < readVerb.edits || readUndo.render !== readVerb.render,
    redone: readRedo.edits > readUndo.edits || readRedo.render !== readUndo.render,
    redoDiffersFromUndo: readRedo.edits !== readUndo.edits || readRedo.render !== readUndo.render,
    undoLane,
    redoLane,
    edits: [before.edits, readVerb.edits, readUndo.edits, readRedo.edits],
    applied: [before.applied, readVerb.applied, readUndo.applied, readRedo.applied],
    refusal: refusals[cursor] ?? null,
  };
}

/** 🎯️ Drives the kind's pinned verb first, then scanned document verbs, until one round-trips. */
async function mutateUndoRedo(page: Page, refusals: readonly string[], key: string, verbs: Readonly<Record<string, string>>, args: Readonly<Record<string, Readonly<Record<string, string>>>>, liveId: string, maxRows: number) {
  const railToggles = await unfoldActionsRail(page);
  const ids = (await readShell(page)).actions.map((id) => id.replace(/^action\./u, ""));
  if (ids.length === 0) return { railToggles, railRows: 0, railRowIds: [] as string[], mutation: null, mutationDetail: "no Actions rail row after unfolding", attempts: [] as VerbAttempt[], known: verbs[key] ?? null };
  const known = verbs[key];
  const scanned = ids.filter((id) => {
    if (id === "set-cell" || id === "replace-text" || id === "set-node") return false;
    if (/close|quit|delete|remove|reset|export|checkpoint|alternative|^undo$|^redo$|^commit$|^checkout/iu.test(id)) return false;
    return !/^set[A-Z]|selection|selectall|reorganize|viewport|zoom|^pan|^fit|^focus|^hover|granularity|^open|^toggle|^copy|^cut/iu.test(id);
  });
  const order = known && ids.includes(known) ? [known, ...scanned.filter((id) => id !== known)] : scanned;
  const attempts: VerbAttempt[] = [];
  let best: VerbAttempt | null = null;
  for (const verbId of order.slice(0, maxRows)) {
    const attempt = await runVerb(page, verbId, args[`${key}.${verbId}`] ?? args[verbId], refusals, liveId);
    attempts.push(attempt);
    if (best === null && attempt.mutated) best = attempt;
    if (attempt.mutated && attempt.redoDiffersFromUndo) return { railToggles, railRows: ids.length, railRowIds: ids, mutation: verbId, mutationDetail: null, attempts, known: known ?? null, ...attempt };
  }
  return { railToggles, railRows: ids.length, railRowIds: ids, mutation: best?.verbId ?? null, mutationDetail: best ? "verb moved the document; undo and redo did not read two different documents" : `none of ${scanned.length} rail rows moved the ledger`, attempts, known: known ?? null, ...(best ?? {}) };
}

/** 🇩🇪️ Seats the shell locale through the Settings surface's own language control and reports what it reached. */
async function seatLocale(page: Page, wanted: string): Promise<string> {
  if (wanted === "en") return "en(boot)";
  if ((await click(page, '[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], button:has-text("Settings")')) === "absent") return `${wanted}:no-settings-control`;
  await page.waitForTimeout(3_000);
  const language = page.locator('[role="tab"], [role="button"], button').filter({ hasText: /language|sprache/iu }).first();
  if ((await language.count()) > 0) {
    await language.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(2_000);
  }
  const control = page.locator('select, [role="combobox"]').filter({ hasText: /english|deutsch|german/iu }).first();
  if ((await control.count()) === 0) return `${wanted}:no-language-control`;
  if ((await control.evaluate((element) => element.tagName.toLowerCase())) === "select") await control.selectOption(wanted).catch(() => undefined);
  else {
    await control.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(800);
    await page.locator('[role="option"]').filter({ hasText: /deutsch|german/iu }).first().click({ force: true }).catch(() => undefined);
  }
  await page.waitForTimeout(6_000);
  await page.keyboard.press("Escape").catch(() => undefined);
  await page.waitForTimeout(2_000);
  return `${wanted}=${(await page.evaluate(() => document.documentElement.lang || null)) ?? "unreported"}`;
}
//#endregion 🔖️Witness

//#region 🔖️Matrix
/** 🎛️ One matrix run. */
export type ProgramMatrixOptions = Readonly<{
  baseUrl: string;
  tag: string;
  locale: string;
  roles: readonly string[];
  reloadEvery: number;
  only: readonly string[];
  skip: readonly string[];
  resume: boolean;
  outDir: string;
  headed: boolean;
  maxVerbRows: number;
  installBudgetMs: number;
  signal: AbortSignal;
}>;

/** 🧾️ One program's row. */
type MatrixRow = Record<string, unknown> & { key: string; role: string; appId: string; pass: boolean; faultCount: number };

/** 📊️ The whole run, written after every row. */
export type ProgramMatrixReport = { baseUrl: string; tag: string; locale: string; roles: readonly string[]; started: string; finished?: string; boots: unknown[]; census: unknown; rows: MatrixRow[]; fatal?: string; cancelled?: boolean; refusals?: string[]; faultsTail?: string[]; agentBridgeConnectionErrors?: number };

const NEUTRAL_ROWS = /^(Clear Selection|Select All|Auswahl aufheben|Alles auswählen|Undo|Redo|Rückgängig|Wiederholen|Toggle Panel|Panel umschalten)\b/u;
const CLOSE_DROP = /refused: instance-retired \(gesture window=/u;
const BRIDGE_NOISE = /WebSocket connection to 'ws:\/\/127\.0\.0\.1:\d+\/bridge' failed/u;

type RenderFacts = { id: string; present: boolean; elements?: number; svg?: number; canvases?: string[]; uiNodeKeys?: number; text?: string; textChars?: number; skeleton?: boolean; fault?: string | null; chips?: { id: string; hit: string }[]; stacked?: boolean };

/** 🖼️ What each opened window paints (the chrome subtracted) and whether each chip is the element under its own centre. */
const renderFacts = (page: Page, ids: readonly string[]): Promise<RenderFacts[]> =>
  page.evaluate((wanted) => {
    const rows: RenderFacts[] = [];
    for (const id of wanted) {
      const host = document.getElementById(id);
      const body = host?.querySelector('[data-slot="window-body"]') ?? null;
      if (!host || body === null) {
        rows.push({ id, present: false });
        continue;
      }
      const chrome = new Set([...body.querySelectorAll('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"], [data-slot="window-engagement-overlay"], [data-slot="utility-bar-overlay"], [data-slot="window-measures-overlay"], [data-slot="window-search-overlay"], [data-slot="pane-host"]')].flatMap((pane) => [pane, ...pane.querySelectorAll("*")]));
      const own = [...body.querySelectorAll("*")].filter((element) => !chrome.has(element));
      const text = own.filter((element) => element.children.length === 0).map((element) => (element.textContent ?? "").trim()).filter((value) => value.length > 0).join(" ").replace(/\s+/gu, " ");
      const chips = [...host.querySelectorAll('[id$=".engagement.toggle"], [id$=".utilityBar.unfold"], [id$=".utilityBar.fold"]')].map((chip) => {
        const rect = chip.getBoundingClientRect();
        if (rect.width === 0 || rect.height === 0) return { id: chip.id, hit: "zero-size" };
        if (chip instanceof HTMLButtonElement && chip.disabled) return { id: chip.id.replace(/^framework\.window\./u, ""), hit: "self" };
        const top = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
        const hit = top !== null && (top === chip || chip.contains(top));
        return { id: chip.id.replace(/^framework\.window\./u, ""), hit: hit ? "self" : `${top?.tagName ?? "none"}[${top?.getAttribute("data-slot") ?? ""}]#${top?.closest("[id]")?.id ?? ""}` };
      });
      rows.push({
        id,
        present: true,
        elements: own.length,
        svg: own.filter((element) => element instanceof SVGElement).length,
        canvases: own.filter((element): element is HTMLCanvasElement => element.tagName === "CANVAS").map((canvas) => `${canvas.width}x${canvas.height}`),
        uiNodeKeys: own.filter((element) => element.hasAttribute("data-ui-node-key")).length,
        text: text.slice(0, 160),
        textChars: text.length,
        skeleton: body.querySelector('[data-slot="pane-host-root"] [aria-busy="true"]') !== null && own.length < 12,
        fault: host.querySelector("[data-semio-window-fault]")?.getAttribute("data-semio-window-fault") ?? null,
        chips,
      });
    }
    return rows;
  }, ids);

const rendered = (facts: readonly RenderFacts[]): boolean => facts.length > 0 && facts.every((row) => row.present && !row.skeleton && !row.fault && ((row.canvases?.length ?? 0) > 0 || (row.svg ?? 0) > 0 || (row.textChars ?? 0) > 0 || (row.uiNodeKeys ?? 0) > 0));
const chipsHit = (facts: readonly RenderFacts[]): boolean => facts.every((row) => (row.chips ?? []).every((chip) => chip.hit === "self"));

const historyRows = (page: Page): Promise<{ all: number; verbRows: string[] }> =>
  page
    .evaluate(() => [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => ((element as HTMLElement).innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 70)))
    .then((rows) => ({ all: rows.length, verbRows: [...new Set(rows.filter((row) => !NEUTRAL_ROWS.test(row)))].slice(-3) }));

/** 🧮️ Runs the matrix and returns its report; the report (and one screenshot per row) is rewritten after every row, so
 * a cancelled or crashed run keeps every finished row and `resume` continues from them. Cancellation (the signal) ends
 * the run after the current row. */
export async function runProgramMatrix(repoRoot: string, options: ProgramMatrixOptions): Promise<ProgramMatrixReport> {
  const pins = readMatrixPins();
  const outDir = join(options.outDir, options.tag);
  mkdirSync(outDir, { recursive: true });
  const out = join(outDir, "matrix.json");
  const log = (line: string): void => console.log(`[matrix] ${line}`);
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser: Browser = await chromium.launch({ headless: !options.headed, args: ["--use-angle=metal"] });
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  page.setDefaultNavigationTimeout(180_000);
  await page.addInitScript(() => {
    const seen: { key: string; code: string | null; text: string; lang: string }[] = [];
    Object.defineProperty(window, "__matrixNotices", { value: seen });
    new MutationObserver(() => {
      for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
        const text = (element.firstChild?.textContent ?? element.textContent ?? "").trim();
        const key = `${element.getAttribute("data-notice-code") ?? ""}|${text}`;
        if (seen.at(-1)?.key !== key) seen.push({ key, code: element.getAttribute("data-notice-code"), text, lang: document.documentElement.lang });
      }
    }).observe(document, { subtree: true, childList: true, characterData: true });
  });
  const notices = (): Promise<{ code: string | null; text: string; lang: string }[]> => page.evaluate(() => (window as unknown as { __matrixNotices: { code: string | null; text: string; lang: string }[] }).__matrixNotices);
  const faults: string[] = [];
  const refusals: string[] = [];
  let bridgeNoise = 0;
  page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 300)));
  page.on("console", (message) => {
    const text = message.text();
    if (BRIDGE_NOISE.test(text)) return void (bridgeNoise += 1);
    if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
    if (FAULT.test(text) && !NOISE.test(text)) faults.push(`${message.type()}: ${text}`.slice(0, 300));
  });

  const boot = async (): Promise<Record<string, unknown>> => {
    await page.goto(options.baseUrl, { waitUntil: "commit", timeout: 300_000 });
    const beacon = await awaitBeacon(page, Date.now() + 300_000);
    await dismissIntroduction(page);
    await page.waitForTimeout(3_000);
    const seated = await seatLocale(page, options.locale);
    await page.keyboard.press("Escape").catch(() => undefined);
    return { beacon, seated, lang: await page.evaluate(() => document.documentElement.lang) };
  };
  const onHome = async (): Promise<boolean> => {
    const ids = await windowIds(page).catch(() => null);
    const beacon = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")).catch(() => null);
    return beacon !== null && ids !== null && ids.length === 1 && ids[0] === "s-home-main";
  };
  const openPalette = async () => {
    await dismissIntroduction(page);
    await page.evaluate(() => {
      if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
      document.body.focus();
    });
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
    return input;
  };

  const previous: ProgramMatrixReport | null = options.resume && existsSync(out) ? (JSON.parse(readFileSync(out, "utf8")) as ProgramMatrixReport) : null;
  const report: ProgramMatrixReport = { baseUrl: options.baseUrl, tag: options.tag, locale: options.locale, roles: options.roles, started: previous?.started ?? new Date().toISOString(), boots: previous?.boots ?? [], census: null, rows: (previous?.rows ?? []).filter((row) => row.pass) };
  const done = new Set(report.rows.map((row) => `${row.key}#${row.role}`));
  const flush = (): void => writeFileSync(out, JSON.stringify(report, null, 1));

  let programs: MatrixProgram[] = [];
  let verbs: Record<string, string> = {};
  let args: Record<string, Readonly<Record<string, string>>> = {};

  const openProgram = async (program: MatrixProgram): Promise<{ windowIds: string[]; item: string | null; detail: string | null }> => {
    const before = await windowIds(page);
    const input = await openPalette();
    if ((await input.count()) === 0) return { windowIds: [], item: null, detail: "command palette never opened" };
    await input.fill(kindOf(program.appId));
    await page.waitForTimeout(1_200);
    const ids = [`spawn.${program.pluginId}.${program.appId}`];
    if (programs.find((entry) => entry.pluginId === program.pluginId)?.appId === program.appId) ids.push(`spawn.${program.pluginId}`);
    let chosen: string | null = null;
    for (const id of ids) {
      const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
      await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
      if ((await item.count()) > 0) {
        chosen = id;
        await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
        break;
      }
    }
    if (chosen === null) {
      await page.keyboard.press("Escape");
      return { windowIds: [], item: null, detail: `no ${ids.join(" | ")} palette row` };
    }
    const deadline = Date.now() + 90_000;
    while (Date.now() < deadline) {
      const fresh = (await windowIds(page)).filter((id) => !before.includes(id));
      if (fresh.length > 0) {
        await page.waitForTimeout(3_500);
        return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), item: chosen, detail: null };
      }
      const seen = await notices();
      const last = seen.at(-1);
      if (last && Date.now() > deadline - 80_000 && /spawnProgram|open/iu.test(last.code ?? "")) return { windowIds: [], item: chosen, detail: `refused: ${last.code} ${last.text}` };
      await page.waitForTimeout(250);
    }
    return { windowIds: [], item: chosen, detail: "palette row pressed, no new window" };
  };

  const closeProgram = async (ids: readonly string[]): Promise<{ outcomes: string[]; remaining: string[] }> => {
    const outcomes: string[] = [];
    for (const id of ids) {
      outcomes.push(
        await page.evaluate((windowId) => {
          const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId);
          if (!tab) return "no-tab";
          const button = tab.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]');
          if (!(button instanceof HTMLElement)) return "no-close";
          button.click();
          return "clicked";
        }, id),
      );
      await page.waitForTimeout(400);
    }
    await page.waitForTimeout(1_200);
    return { outcomes, remaining: (await windowIds(page)).filter((id) => ids.includes(id)) };
  };

  const runRow = async (program: MatrixProgram): Promise<MatrixRow> => {
    const started = Date.now();
    const faultCursor = faults.length;
    const noticeCursor = (await notices()).length;
    const role = roleOf(program.appId);
    const row: MatrixRow = { key: keyOf(program), role, appId: program.appId, pass: false, faultCount: 0 };
    const opened = await openProgram(program);
    Object.assign(row, { item: opened.item, windowIds: opened.windowIds, openDetail: opened.detail, openMs: Date.now() - started });
    if (opened.windowIds.length > 0) {
      const render = await renderFacts(page, opened.windowIds);
      for (const facts of render.filter((entry) => !entry.present)) {
        const rect = await page.evaluate((windowId) => {
          const box = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId)?.getBoundingClientRect();
          return box ? [box.left, box.top, box.width, box.height] : null;
        }, facts.id);
        if (rect) await page.mouse.click(rect[0]! + Math.min(20, rect[2]! / 2), rect[1]! + rect[3]! / 2);
        await page.waitForTimeout(2_500);
        Object.assign(facts, (await renderFacts(page, [facts.id]))[0], { stacked: true });
      }
      Object.assign(row, { render, bodiesRendered: rendered(render), chipsHit: chipsHit(render) });
      if (role === "editor") {
        const pre = pins.kindPre[row.key];
        if (pre) {
          await unfoldActionsRail(page);
          const preVerb = pre.replace(/!$/u, "");
          let preOutcome = `${preVerb}:${await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${preVerb}"]`)}`;
          if (pre.endsWith("!")) {
            await page.waitForTimeout(800);
            for (const [argKey, value] of Object.entries(args[`${row.key}.${preVerb}`] ?? {})) preOutcome += `:${await fillStagedArgument(page, argKey, value, pins.liveId)}`;
            preOutcome += `:submit=${await submitStagedVerb(page, preVerb)}`;
          }
          row.pre = preOutcome;
          await page.waitForTimeout(1_500);
        }
        const verb = await mutateUndoRedo(page, refusals, row.key, verbs, args, pins.liveId, options.maxVerbRows);
        Object.assign(row, {
          railRowIds: verb.railRowIds,
          railRows: verb.railRows,
          knownVerb: verb.known,
          knownVerbOffered: verb.known !== null && verb.railRowIds.includes(verb.known),
          verb: verb.mutation,
          verbDetail: verb.mutationDetail,
          filled: "filled" in verb ? verb.filled : [],
          edits: "edits" in verb ? verb.edits : [],
          applied: "applied" in verb ? verb.applied : [],
          undoLane: "undoLane" in verb ? verb.undoLane : null,
          redoLane: "redoLane" in verb ? verb.redoLane : null,
          mutated: "mutated" in verb ? verb.mutated : false,
          redoDiffersFromUndo: "redoDiffersFromUndo" in verb ? verb.redoDiffersFromUndo : false,
          attempts: verb.attempts.map((attempt) => `${attempt.verbId}:${attempt.edits.join(",")}:${attempt.undoLane ?? "-"}/${attempt.redoLane ?? "-"}${attempt.refusal ? `:${attempt.refusal.slice(0, 80)}` : ""}`),
          historyRows: await historyRows(page),
          renderAfterRedo: (await renderFacts(page, opened.windowIds)).map((facts) => ({ id: facts.id, present: facts.present, skeleton: facts.skeleton, fault: facts.fault, canvases: facts.canvases, textChars: facts.textChars, text: facts.text?.slice(0, 100) })),
        });
      }
      await page.screenshot({ path: join(outDir, `${row.key.replace(/[^A-Za-z0-9]+/gu, "-")}-${role}.png`) }).catch(() => undefined);
      const closeCursor = faults.length;
      row.close = await closeProgram(opened.windowIds);
      row.closeDrops = faults.slice(closeCursor).filter((line) => CLOSE_DROP.test(line)).length;
      faults.splice(closeCursor, faults.length - closeCursor, ...faults.slice(closeCursor).filter((line) => !CLOSE_DROP.test(line)));
    }
    row.notices = (await notices()).slice(noticeCursor).map((notice) => `${notice.code}|${notice.lang}|${notice.text}`.slice(0, 160));
    row.faultLines = faults.slice(faultCursor).slice(0, 4);
    row.faultCount = faults.length - faultCursor;
    const edits = (row.edits as number[] | undefined) ?? [];
    row.cleanRoundTrip = edits.length === 4 && edits[1]! >= edits[0]! && edits[1]! >= 1 && edits[2] === edits[1]! - 1 && edits[3] === edits[1];
    const opens = opened.windowIds.length > 0;
    row.pass =
      role === "editor"
        ? opens && row.bodiesRendered === true && row.chipsHit === true && ((row.railRows as number | undefined) ?? 0) > 0 && row.mutated === true && row.redoDiffersFromUndo === true && row.cleanRoundTrip === true && row.faultCount === 0
        : opens && row.bodiesRendered === true && row.chipsHit === true && row.faultCount === 0;
    row.totalMs = Date.now() - started;
    return row;
  };

  /** ⏳️ Plugins install lazily after the beacon and the probe's programs grow with them: waits until every plugin reached
   * a terminal status (`loaded`, `failed`, `crashed`) or nothing changed for `quietMs`, bounded by `budgetMs`. */
  const settledProbe = async (budgetMs: number, quietMs = 20_000): Promise<{ probe: CatalogProbe | null; settled: boolean; waitedMs: number }> => {
    const began = Date.now();
    let last = "";
    let changedAt = Date.now();
    let reportedAt = 0;
    for (;;) {
      const probe = await readProbe(page);
      const terminal = probe !== null && probe.plugins.length > 0 && probe.plugins.every((row) => ["loaded", "failed", "crashed"].includes(row.status));
      const fingerprint = probe === null ? "none" : `${probe.plugins.filter((row) => row.status === "loaded").length}/${probe.plugins.length}:${probe.programs.length}`;
      if (fingerprint !== last) {
        last = fingerprint;
        changedAt = Date.now();
      }
      if (terminal || Date.now() - changedAt >= quietMs) return { probe, settled: terminal, waitedMs: Date.now() - began };
      if (Date.now() - began >= budgetMs || options.signal.aborted) return { probe, settled: false, waitedMs: Date.now() - began };
      if (Date.now() - reportedAt >= 10_000) {
        reportedAt = Date.now();
        log(`waiting for plugin installs: loaded/registry:programs ${fingerprint} (${Math.round((Date.now() - began) / 1000)} s)`);
      }
      await page.waitForTimeout(2_000);
    }
  };

  try {
    report.boots.push(await boot());
    const { probe, settled, waitedMs } = await settledProbe(options.installBudgetMs);
    if (probe === null) throw new Error("the shell exposed no window.__semioOsCatalogProbe");
    programs = probe.programs.map((entry) => ({ pluginId: entry.pluginId, appId: entry.appId })).filter((program) => options.roles.includes(roleOf(program.appId)) && !pins.excludedKinds.includes(baseKeyOf(program)));
    ({ verbs, args } = resolvePins(repoRoot, pins, programs));
    const selected = programs.filter((program) => {
      const key = keyOf(program);
      if (options.only.length > 0 && !options.only.some((entry) => entry === program.pluginId || entry === key)) return false;
      return !options.skip.some((entry) => entry === program.pluginId || entry === key);
    });
    report.census = { registryRows: probe.plugins.length, loaded: probe.plugins.filter((row) => row.status === "loaded").length, notLoaded: probe.plugins.filter((row) => row.status !== "loaded").map((row) => `${row.pluginId}:${row.status}`), programs: probe.programs.length, selected: selected.length, installsSettled: settled, installWaitMs: waitedMs };
    log(`boot ${JSON.stringify(report.boots.at(-1))} census ${JSON.stringify(report.census)}`);
    let sinceBoot = 0;
    for (const [index, program] of selected.entries()) {
      if (options.signal.aborted) {
        report.cancelled = true;
        log("cancelled; the report keeps every finished row");
        break;
      }
      if (done.has(`${keyOf(program)}#${roleOf(program.appId)}`)) continue;
      if (sinceBoot >= options.reloadEvery || !(await onHome())) {
        report.boots.push(await boot());
        sinceBoot = 0;
      }
      sinceBoot += 1;
      let row: MatrixRow;
      try {
        row = await runRow(program);
      } catch (error) {
        log(`row ${keyOf(program)} interrupted (${String(error).split("\n")[0]!.slice(0, 120)}); re-booting and retrying once`);
        report.boots.push(await boot());
        sinceBoot = 1;
        row = await runRow(program).catch((retryError: unknown) => ({ key: keyOf(program), role: roleOf(program.appId), appId: program.appId, pass: false, openDetail: `probe error: ${String(retryError).split("\n")[0]!.slice(0, 160)}`, faultCount: 0 }));
        row.retried = true;
      }
      report.rows.push(row);
      flush();
      log(`${index + 1}/${selected.length} ${row.pass ? "PASS" : "FAIL"} ${row.key}#${row.role} rendered=${String(row.bodiesRendered)} chips=${String(row.chipsHit)} verb=${String(row.verb ?? "-")} edits=${JSON.stringify(row.edits ?? null)} history=${JSON.stringify((row.historyRows as { verbRows?: string[] } | undefined)?.verbRows ?? [])} faults=${row.faultCount} ${String(row.openDetail ?? "")} ${String(row.verbDetail ?? "")} ${Math.round(((row.totalMs as number | undefined) ?? 0) / 1000)}s`);
      if (!row.pass) {
        report.boots.push(await boot());
        sinceBoot = 0;
      }
    }
  } catch (error) {
    report.fatal = String(error);
    log(`FATAL ${report.fatal}`);
  } finally {
    report.finished = new Date().toISOString();
    report.refusals = [...new Set(refusals)].slice(0, 40);
    report.faultsTail = faults.slice(-20);
    report.agentBridgeConnectionErrors = bridgeNoise;
    flush();
    await browser.close();
  }
  return report;
}

/** 📝️ One markdown table row per program. */
export function programMatrixTable(report: ProgramMatrixReport): string {
  const lines = [`| program | role | pass | verb | edits | history | faults | detail |`, `|---|---|---|---|---|---|---|---|`];
  for (const row of report.rows) lines.push(`| ${row.key} | ${row.role} | ${row.pass ? "PASS" : "FAIL"} | ${String(row.verb ?? "-")} | ${JSON.stringify(row.edits ?? null)} | ${((row.historyRows as { verbRows?: string[] } | undefined)?.verbRows ?? []).join(" · ").replaceAll("|", "\\|")} | ${row.faultCount} | ${String(row.openDetail ?? row.verbDetail ?? "").replaceAll("|", "\\|").slice(0, 120)} |`);
  return `${lines.join("\n")}\n`;
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `verify matrix <baseUrl> [--tag <t>] [--locale en|de] [--roles editor,viewer] [--only <plugin|plugin/kind>,…]
 * [--skip …] [--resume] [--reload-every <n>] [--install-budget-ms <n>] [--out <dir>] [--headed]` — runs the matrix, writes `matrix.json`,
 * `table.md` and one screenshot per row under `<out>/<tag>/`, publishes the acceptance record and exits non-zero unless
 * every selected row passes. */
export async function runProgramMatrixCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const baseUrl = segments[0];
  if (!baseUrl || baseUrl.startsWith("--")) throw new Error("usage: verify matrix <baseUrl> [--tag <t>] [--locale en|de] [--roles editor,viewer] [--only …] [--skip …] [--resume] [--reload-every <n>] [--out <dir>] [--headed]");
  const locale = flagValue(segments, "--locale") ?? "en";
  const roles = (flagValue(segments, "--roles") ?? "editor").split(",").filter(Boolean);
  const tag = flagValue(segments, "--tag") ?? `${locale}-${roles.join("-")}`;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  try {
    const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
    const report = await runProgramMatrix(repoRoot, {
      baseUrl,
      tag,
      locale,
      roles,
      reloadEvery: Number(flagValue(segments, "--reload-every") ?? 10),
      only: (flagValue(segments, "--only") ?? "").split(",").filter(Boolean),
      skip: (flagValue(segments, "--skip") ?? "").split(",").filter(Boolean),
      resume: segments.includes("--resume"),
      outDir,
      headed: segments.includes("--headed"),
      maxVerbRows: 6,
      installBudgetMs: Number(flagValue(segments, "--install-budget-ms") ?? 600_000),
      signal: controller.signal,
    });
    writeFileSync(join(outDir, tag, "table.md"), programMatrixTable(report));
    const passed = report.rows.filter((row) => row.pass).length;
    const total = report.rows.length;
    const failed = report.rows.filter((row) => !row.pass).map((row) => `${row.key}#${row.role}`);
    const status = report.fatal || report.cancelled || total === 0 ? "fail" : passed === total ? "pass" : "fail";
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check: "program-matrix",
        status,
        startedAt,
        measured: { locale, roles: roles.join(","), rows: total, passed, failed: total - passed, fatal: Boolean(report.fatal), cancelled: Boolean(report.cancelled) },
        summary: {
          en: `${passed}/${total} ${roles.join("+")} programs pass in ${locale}${failed.length ? `; failing: ${failed.slice(0, 8).join(", ")}` : ""}${report.fatal ? `; fatal: ${report.fatal.slice(0, 160)}` : ""}`,
          de: `${passed}/${total} ${roles.join("+")}-Programme bestehen in ${locale}${failed.length ? `; fehlgeschlagen: ${failed.slice(0, 8).join(", ")}` : ""}${report.fatal ? `; Abbruch: ${report.fatal.slice(0, 160)}` : ""}`,
        },
        evidence: [join(outDir, tag, "matrix.json"), join(outDir, tag, "table.md")],
      }),
    );
    console.log(`[matrix] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===`);
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
}
//#endregion 🔖️Matrix

/** ⏪️ `verify time-travel` — the end-to-end gate of non-destructive history editing, driven the way a person drives it in the
 * puzzle 2d dev serve (the reference app of the history-editing design, React `serve puzzle2d react dev` on :6012 or the wgpu
 * browser shell on :6112). One headless Chromium, one tab at a time, one fresh browser context per UI locale (`en`, then
 * `de`, chosen through `navigator.language`), numbered steps per locale (promoted from the ticket probe of 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING, work package W3-E2E):
 *
 * 1. boot: `data-board-fixture-parsed` first, then a board with nodes and the locale of the history panel; then the
 *    document is bound to a fresh local folder through the sync card (`--folder-at=1`, the default), so every edit persists;
 * 2. select two nodes, drag them by (+80,+40) world units → exactly one new document row labelled from its
 *    `drag-selection` mutation, `data-board-positions-json` moved by the offset; then one downstream drag of a third
 *    node, so "downstream is not applied" is observable;
 * 3. Edit the drag mutation (`historyEditBegin`) → the React band (`role=status`) in `editing`, the preview = state
 *    before the drag + the draft (design §4), the downstream drag not applied, the editor's dx/dy steppers (snap step)
 *    and the targets reference list;
 * 4. dx → 120 through the stepper (keyboard: fill, Enter, ArrowUp, ArrowDown) → preview; Accept → replay → review
 *    `ready` → head at +120 with the downstream drag re-applied;
 * 5. Finalize → dialog (destructive Overwrite, New alternative + name) → Overwrite → band gone, an
 *    "History edited — overwrite" row; after step 8 a page reload plus a re-attach of the same folder must restore the head
 *    and every document row;
 * 6. Undo (chord, then button) → +80; Redo → +120; the undone/redone rows;
 * 7. a second session on dy → New alternative with a name → the alternative row, the head and the Alternatives section
 *    (`framework.history.alternative.<id>`); switching to the trunk and back when the trunk is listed, else a second
 *    alternative and switching between the two;
 * 8. fatal path: duplicate a node, drag the clone, withdraw the clone's `create-node` → review `blocked`, the drag row
 *    reads Error "Target missing", Finalize disabled; Next problem → Withdraw the drag → `ready`; Exit → zero trace;
 * 9. console: uncaught page errors, hard faults and `[DEBUG] ` lines fail the run; errors and warnings are digested;
 * 10. input controls (G6, design §18): a rotate command → Edit → the angle dial (ticks at 0, ±90, 180°, degrees, Arrow = one
 *     step, PageUp = next detent); a scale command → Edit → the factor slider (log ticks at its snaps, a typed value beyond
 *     the hard bound refused naming the bound, the draft kept); Exit leaves zero trace;
 * 11. keep editing after a clean review: `ready` → Begin another mutation → Accept → `ready` with two accepted drafts →
 *     Finalize overwrite of both;
 * 12. fatal loop resolved by EDITING targets (G3, design §16.4): withdraw an upstream `create-node` → the downstream drag is
 *     `mutation.target-missing`, review `blocked` → Next problem → select two remaining nodes → Use selection → chips read
 *     labels, the board highlights them → Accept → `ready` → Finalize overwrite → head equals the expectation;
 * 13. a warning introduced by an upstream edit (G4, design §16.5, S2-W2D scenario `warning-from-an-upstream-edit`): lock and
 *     unlock a node, drag it with another, withdraw the unlock → the drag reads Warning "Partially applied" (new since this
 *     edit), review `ready` → Finalize overwrite → still visible; the reload check (after step 13) re-reads it;
 * 14. phone width (G13): a fresh 375 px touch context — band, editor and finalize prompt inside the viewport, touch-sized,
 *     reachable by touch and keyboard, no horizontal page scroll;
 * 15. tablet width (G13): the same journey in a fresh 768 × 1024 touch context (the tablet breakpoint of `📱️device`);
 * 16. long history (G9): a fresh document whose example load is one transaction of several hundred mutations — edit its first
 *     mutation, Accept, replay progress over at least `--long-history` mutations, Cancel (review "Replay needed"), Replay
 *     again with an Edit pressed while it replays (refused, the session keeps its target), Exit with zero trace; the history
 *     grows through the palette's Set Active Example until the replay is observable;
 * 17. two peers on one local folder (G10): two fresh contexts attached to the same folder — the second reads the first's
 *     document, the first begins a history edit, the second's drag arrives as a base move (the session survives, the remote
 *     edit stays downstream and unapplied), Accept replays it too, Finalize reaches the second peer. Presence (the ⏪ roster
 *     badge, "is editing") travels only through a hub and is recorded as such.
 *
 * Step 3 also asserts the G13 transitions (`history-panel-reveals-on-session-start`, `focus-moves-to-the-editor`), step 4
 * focus on the band, step 5 focus in the prompt; steps 3, 5, 14 and 15 run the structural ARIA oracle (`aria-query`'s WAI-ARIA
 * role model + `dom-accessibility-api`'s accessible names, bundled into the page) over the band, the history panel and the
 * prompt (wgpu: over its ARIA mirror). The reload check (G5) runs after step 13 and is attributed to step 5: every pre-reload
 * row with its localized label, the alternatives (main line + alternatives) and the current alternative.
 *
 * Every control is the real one, reached the way a user reaches it: the windowed history body is scrolled until the
 * section holds the row, steppers take keyboard input, arg-carrying commands run from the palette (`mod+p`) through the
 * command panel's staged form.
 *
 * `--renderer wgpu` drives the puzzle 2d wgpu shell (default serve :6112) through the same steps: rows, buttons, inputs and
 * the dialog through its ARIA mirror (`#semio-wgpu-accessibility`), chrome through `dumpChrome` hit rects, the board with the
 * pointer and keyboard on the canvas at the positions `semioWgpuIntrospection.dumpBoard2d` publishes (region `🔖️Wgpu`).
 *
 * Run: `verify time-travel [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,3]
 * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--explore]` — the serve is reused when it answers, else
 * started for the run and stopped after it ({@link ensureDevServe}). `--only` selects steps (each selection boots its own
 * document, so a later step needs the earlier ones it builds on: 3–7 and 11 build on 2, 6 and 7 on 4–5); `reload` selects the
 * reload check (it needs step 5's overwrite or step 13's warning), which runs on its own only when named. `--chords <locales>` drives Accept/Discard/Exit through
 * the band chords (`alt+enter`, `alt+backspace`, `alt+shift+backspace`) in those locales and through the band buttons
 * elsewhere; step 11 always takes the other way once, so chords and buttons are both driven. `--explore` boots each locale,
 * opens the history panel, dumps the DOM inventory and exits. Outputs under `--out` (default `🤖️generated/🧪️time-travel`):
 * `probe-<renderer>-<stamp>.md` (step lines, verdicts, notes, timeline, console digest), `.ndjson` (one record per
 * verdict/note/dump) and `probe-<renderer>-<stamp>-<locale>-s<step>-<tag>.png|json`; the acceptance record `time-travel`.
 * @see ../../../../../../🔨️modules/⏪️time-travel/🦀️.rs
 * @see ../../../🔌️plugin/⏪️time-travel/🦀️.rs
 * @see ../../../📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx
 * @see ../👥️two-human/🟦️.ts */
import { appendFileSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, BrowserContext, ConsoleMessage, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { devServePortV1, ensureDevServe } from "../../🚀️local-hub/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

//#region 🔖️Arguments
type Locale = "en" | "de";
type Positions = Record<string, [number, number]>;
type Point = { x: number; y: number };
type Renderer = "react" | "wgpu";

const OVERVIEW = "2d-overview";
const HISTORY_TAB = "framework.panel.history";
const INSPECTION_TAB = "framework.panel.inspection";
const PLUGIN_VARIANT = "puzzle2d";
const CHECK_ID = "time-travel";
const ORDER = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 9] as const;
const DEFAULT_SERVES: Readonly<Record<Renderer, string>> = { react: "http://127.0.0.1:6012/", wgpu: "http://127.0.0.1:6112/" };

let renderer: Renderer = "react";
let serveOrigin = "http://127.0.0.1:6012";
let only: Set<number> | null = null;
let reloadSelected = false;
let locales: Locale[] = ["en", "de"];
let chordLocales = new Set<string>(["en", "de"]);
let explore = false;
let folderAt = 1;
let longHistory = 200;
let OUT = "";
let stamp = "";
let base = "";
let ndjsonPath = "";
let t0 = Date.now();
let cancelled: AbortSignal = new AbortController().signal;

/** 🏷️ The value after `flag` in `segments` (`--flag value`), or undefined when absent or followed by another flag. */
const flagValue = (segments: readonly string[], flag: string): string | undefined => {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
};

/** 🧭️ Seats one run's configuration from its CLI segments, opens its output directory and its ndjson stream. */
const configure = (segments: readonly string[], defaultOutDir: string, signal: AbortSignal) => {
  renderer = flagValue(segments, "--renderer") === "wgpu" ? "wgpu" : "react";
  serveOrigin = new URL(flagValue(segments, "--serve") ?? DEFAULT_SERVES[renderer]).origin;
  const steps = flagValue(segments, "--only");
  only = steps ? new Set(steps.split(",").map((entry) => Number(entry.trim())).filter(Number.isFinite)) : null;
  reloadSelected = !steps || steps.split(",").some((entry) => entry.trim() === "reload");
  locales = (flagValue(segments, "--locales") ?? "en,de").split(",").map((entry) => entry.trim()).filter((entry): entry is Locale => entry === "en" || entry === "de");
  chordLocales = new Set((flagValue(segments, "--chords") ?? "en,de").split(",").map((entry) => entry.trim()).filter(Boolean));
  explore = segments.includes("--explore");
  folderAt = Number(flagValue(segments, "--folder-at") ?? "1");
  longHistory = Math.max(1, Number(flagValue(segments, "--long-history") ?? "200") || 200);
  OUT = resolve(flagValue(segments, "--out") ?? defaultOutDir);
  mkdirSync(OUT, { recursive: true });
  stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
  base = `probe-${renderer}-${stamp}`;
  ndjsonPath = join(OUT, `${base}.ndjson`);
  writeFileSync(ndjsonPath, "");
  t0 = Date.now();
  cancelled = signal;
  for (const rows of [verdicts, notes, timeline, consoleRows, pageErrors, hardFaults, harvestedNotices] as unknown[][]) rows.length = 0;
};

/** 🌐️ The puzzle 2d playground route of the serve under test. */
const routeUrl = () => `${serveOrigin}/?plugin=${PLUGIN_VARIANT}`;
//#endregion 🔖️Arguments

//#region 🔖️Report
type VerdictRow = { locale: Locale; step: number; name: string; ok: boolean; detail: Record<string, unknown> };
const verdicts: VerdictRow[] = [];
const notes: string[] = [];
const timeline: string[] = [];
let currentLocale: Locale = "en";
let currentStep = 0;

/** 🧾️ One ndjson record, stamped with the run clock, the locale and the step. */
const emit = (record: Record<string, unknown>) => appendFileSync(ndjsonPath, `${JSON.stringify({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, ...record })}\n`);

/** 🕰️ One timeline line, printed and kept for the markdown report. */
const log = (message: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${currentLocale}/${currentStep} ${message}`;
  timeline.push(row);
  console.log(row);
};

/** ⚖️ One PASS/FAIL verdict of the current locale and step. */
const verdict = (name: string, ok: boolean, detail: Record<string, unknown> = {}) => {
  verdicts.push({ locale: currentLocale, step: currentStep, name, ok, detail });
  emit({ kind: "verdict", name, verdict: ok ? "PASS" : "FAIL", ...detail });
  log(`${ok ? "PASS" : "FAIL"} ${name} ${JSON.stringify(detail).slice(0, 600)}`);
  return ok;
};

/** 🔖️ Attributes the verdicts `record` makes to `step` — a check taken later in the run than the step whose gap it proves. */
const atStep = <T,>(step: number, record: () => T) => {
  const previous = currentStep;
  currentStep = step;
  try {
    return record();
  } finally {
    currentStep = previous;
  }
};

/** 📝️ An observation that is neither PASS nor FAIL (timing, a design reading, an inventory). */
const note = (name: string, detail: Record<string, unknown> = {}) => {
  const row = `${currentLocale}/${currentStep} ${name} ${JSON.stringify(detail).slice(0, 900)}`;
  notes.push(row);
  emit({ kind: "note", name, ...detail });
  log(`NOTE ${name} ${JSON.stringify(detail).slice(0, 600)}`);
};
//#endregion 🔖️Report

//#region 🔖️Copy
/** 🌍️ The localized texts every verdict compares against, byte-equal with their Rust/TS producers:
 * drag/create labels (`✋️drag-selection`, `🌱create-node`), history-edit rows (`SupersedeLedger::label`), the finalize
 * dialog (`history_edit_finalize_dialog`), the panel copy (`HistoryPanelText`) and the review lines (`TimeTravelLabel`). */
const COPY = {
  en: {
    commands: "Commands",
    items: (count: number) => (count === 1 ? "1 item" : `${count} items`),
    drag: (count: number, dx: string, dy: string) => `Drag ${COPY.en.items(count)} by (${dx}, ${dy})`,
    createNode: (id: string) => `Create node "${id}"`,
    row: (role: "edit" | "undo" | "redo", scope: string | null, count: number) =>
      `${{ edit: "History edited", undo: "History edit undone", redo: "History edit redone" }[role]} — ${scope === null ? "overwrite" : `alternative ${scope}`}: ${count === 1 ? "1 mutation" : `${count} mutations`}`,
    edit: /^(Edit)$/,
    withdraw: /^(Withdraw)$/,
    pending: "Not applied while editing",
    targetMissing: "Error: Target missing",
    severityError: /\b(Error|Fatal)\b/,
    stageEditing: "Editing a mutation",
    reviewReady: "Ready to finalize",
    reviewBlocked: "Errors must be fixed or withdrawn before finalizing",
    dialogTitle: "Finish editing history",
    overwrite: "Overwrite",
    overwriteDescription: "Replaces the edited mutations in every alternative that contains them.",
    newAlternative: "New alternative",
    defaultName: "Edited history",
    tourButtons: [/^\s*(x\s*)?skip\s*$/i, /^\s*close\s*$/i],
    warningPartial: "Warning: Partially applied",
    introduced: "New since this edit",
    useSelection: /^(use selection)$/i,
    changeLocked: (id: string) => `Change node "${id}" locked`,
    rotateCommand: /^Rotate\b/,
    scaleCommand: /^Scale\b/,
    rotate: (count: number, degrees: string) => `Rotate ${COPY.en.items(count)} by ${degrees}°`,
    scale: (count: number, factor: string) => `Scale ${COPY.en.items(count)} by a factor of ${factor}`,
    accepted: (count: number) => `Accepted changes: ${count}`,
    greaterThanZero: /Must be greater than 0\b/,
    atLeastZero: /Must be at least 0\b/,
    trunk: "Main line",
    current: "Current",
    example: /^Set Active Example\b/,
    exampleQuery: "Set Active Example",
    manifest: /^Change manifest id\b/,
    createNodeRow: /^Create node "/,
    replayCancelled: "Replay cancelled",
  },
  de: {
    commands: "Befehle",
    items: (count: number) => (count === 1 ? "1 Element" : `${count} Elemente`),
    drag: (count: number, dx: string, dy: string) => `${COPY.de.items(count)} um (${dx}; ${dy}) ziehen`,
    createNode: (id: string) => `Knoten "${id}" erstellen`,
    row: (role: "edit" | "undo" | "redo", scope: string | null, count: number) =>
      `${{ edit: "Verlauf bearbeitet", undo: "Verlaufsbearbeitung rückgängig", redo: "Verlaufsbearbeitung wiederhergestellt" }[role]} — ${scope === null ? "überschrieben" : `Alternative ${scope}`}: ${count === 1 ? "1 Mutation" : `${count} Mutationen`}`,
    edit: /^(Bearbeiten)$/,
    withdraw: /^(Zurückziehen)$/,
    pending: "Beim Bearbeiten nicht angewendet",
    targetMissing: "Fehler: Ziel fehlt",
    severityError: /\b(Fehler|Kritisch)\b/,
    stageEditing: "Mutation wird bearbeitet",
    reviewReady: "Bereit zum Abschließen",
    reviewBlocked: "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden",
    dialogTitle: "Verlaufsbearbeitung abschließen",
    overwrite: "Überschreiben",
    overwriteDescription: "Ersetzt die bearbeiteten Mutationen in jeder Alternative, die sie enthält.",
    newAlternative: "Neue Alternative",
    defaultName: "Bearbeiteter Verlauf",
    tourButtons: [/^\s*(x\s*)?(skip|überspringen)\s*$/i, /^\s*(close|schließen)\s*$/i],
    warningPartial: "Warnung: Teilweise angewendet",
    introduced: "Neu durch diese Bearbeitung",
    useSelection: /^(auswahl verwenden)$/i,
    changeLocked: (id: string) => `Sperre von Knoten "${id}" ändern`,
    rotateCommand: /^Drehen\b/,
    scaleCommand: /^Skalieren\b/,
    rotate: (count: number, degrees: string) => `${COPY.de.items(count)} um ${degrees}° drehen`,
    scale: (count: number, factor: string) => `${COPY.de.items(count)} um den Faktor ${factor} skalieren`,
    accepted: (count: number) => `Übernommene Änderungen: ${count}`,
    greaterThanZero: /Muss größer als 0 sein/,
    atLeastZero: /Muss mindestens 0 sein/,
    trunk: "Hauptlinie",
    current: "Aktuell",
    example: /^Aktives Beispiel festlegen\b/,
    exampleQuery: "Aktives Beispiel",
    manifest: /^Manifest-ID ändern\b/,
    createNodeRow: /^Knoten "/,
    replayCancelled: "Neu anwenden abgebrochen",
  },
} as const;

/** 🔢️ `puzzle2d_selection_number`: two decimals, trailing zeros and a negative zero dropped, a decimal comma in German. */
const labelNumber = (value: number, locale: Locale) => {
  const rounded = Math.round(value * 100) / 100;
  const text = (rounded === 0 ? 0 : rounded).toFixed(2).replace(/0+$/, "").replace(/\.$/, "");
  return locale === "de" ? text.replace(".", ",") : text;
};
const dragLabel = (count: number, dx: number, dy: number, locale = currentLocale) => COPY[locale].drag(count, labelNumber(dx, locale), labelNumber(dy, locale));
//#endregion 🔖️Copy

//#region 🔖️Console
type ConsoleRow = { t: number; locale: Locale; step: number; source: "page" | "worker"; type: string; text: string };
const consoleRows: ConsoleRow[] = [];
const pageErrors: { locale: Locale; step: number; text: string }[] = [];
const hardFaults: { locale: Locale; step: number; text: string }[] = [];
const BENIGN_RE = /contributions document sources|staged plugin module|\[stale\]|activate-puzzle2d-react-dev|transform freshness|Download the React DevTools/i;
const HARD_FAULT_RE =
  /worker fault|\bunreachable\b|\[semio-plugin panic\]|panicked at|SemioFaultError|terminal-fault|admission failed|shard .* (lost|terminated)|native-owner-required|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked/i;

/** 📥️ Keeps one console line (page or worker) in the ring and classifies it as a hard fault. */
const keepConsole = (source: ConsoleRow["source"], message: ConsoleMessage) => {
  const row: ConsoleRow = { t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source, type: message.type(), text: message.text().slice(0, 2000) };
  consoleRows.push(row);
  if (consoleRows.length > 20000) consoleRows.shift();
  if (HARD_FAULT_RE.test(row.text) && !BENIGN_RE.test(row.text)) hardFaults.push({ locale: row.locale, step: row.step, text: row.text.slice(0, 400) });
};

/** 🎧️ Subscribes one page and every worker it spawns: page lines that belong to a worker are left to that worker's
 * own listener, so no line is counted twice. */
const listen = (target: Page) => {
  target.on("console", (message) => {
    if (message.worker?.()) return;
    keepConsole("page", message);
  });
  target.on("worker", (worker) => worker.on("console", (message) => keepConsole("worker", message)));
  target.on("response", (response) => {
    if (response.url().includes("/semio-backbone")) consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "backbone", text: `HTTP ${response.status()} ${response.request().method()} ${decodeURIComponent(response.url()).slice(0, 400)}` });
    if (response.status() < 400) return;
    consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "http", text: `HTTP ${response.status()} ${response.request().method()} ${response.url().slice(0, 300)}` });
  });
  target.on("pageerror", (error) => {
    const text = String(error?.stack ?? error).slice(0, 1200);
    pageErrors.push({ locale: currentLocale, step: currentStep, text });
    consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "pageerror", text });
  });
};
//#endregion 🔖️Console

//#region 🔖️Page
let browser: Browser;
let context: BrowserContext;
let page: Page;
let mod = "Meta";
let navigations = 0;
let expectedReloads = 0;

/** 🛟️ One `page.evaluate` that never takes the run down (a reload race answers `fallback`). */
const evalSafe = async <T, A = undefined>(fn: (arg: A) => T | Promise<T>, fallback: T, arg?: A): Promise<T> => {
  try {
    return (await page.evaluate(fn as (value: unknown) => T | Promise<T>, arg as unknown)) as T;
  } catch {
    return fallback;
  }
};
const sleep = (ms: number) => page.waitForTimeout(ms);

/** ⏳️ Polls `read` until `settled` holds or `timeoutMs` passes; answers the last value and the wait. */
const waitUntil = async <T,>(read: () => Promise<T>, settled: (value: T) => boolean, timeoutMs = 20000, everyMs = 250) => {
  const start = Date.now();
  let value = await read();
  while (!settled(value) && Date.now() - start < timeoutMs) {
    await sleep(everyMs);
    value = await read();
  }
  return { value, waitedMs: Date.now() - start, ok: settled(value) };
};

/** 📸️ One screenshot of the current step. */
const shot = async (tag: string) => {
  const path = join(OUT, `${base}-${currentLocale}-s${currentStep}-${tag}.png`);
  await page.screenshot({ path }).catch(() => {});
  return path;
};

/** 🧰️ Writes one diagnostic JSON beside the screenshots and returns its path. */
const dumpJson = (tag: string, value: unknown) => {
  const path = join(OUT, `${base}-${currentLocale}-s${currentStep}-${tag}.json`);
  writeFileSync(path, JSON.stringify(value, null, 1));
  return path;
};

type Device = { key: string; viewport: { width: number; height: number }; isMobile: boolean; hasTouch: boolean; windows: number };
type Peer = { context: BrowserContext; page: Page };

/** 🖥️ The desktop device every locale pass starts on: three board windows side by side. */
const DESKTOP: Device = { key: "desktop", viewport: { width: 1600, height: 1000 }, isMobile: false, hasTouch: false, windows: 3 };
/** 📱️ A phone below `UI_MOBILE_MAX_WIDTH_PX` (767): one window, the merged mobile panel. */
const PHONE: Device = { key: "mobile", viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true, windows: 1 };
/** 📒️ A tablet in the `📱️device` band 768…1023 px: dock anchors, touch. */
const TABLET: Device = { key: "tablet", viewport: { width: 768, height: 1024 }, isMobile: true, hasTouch: true, windows: 1 };

/** 🌱️ Opens a fresh browser context and its one tab for `device` in the current locale — the run's console listeners on, the
 * wgpu diagnostics armed, the playground route loaded — and makes it the page every helper drives; answers whether it booted. */
const openPage = async (device: Device, label: string, polls = 60): Promise<boolean> => {
  context = await browser.newContext({ viewport: device.viewport, isMobile: device.isMobile, hasTouch: device.hasTouch, deviceScaleFactor: device.isMobile ? 2 : 1, locale: currentLocale === "de" ? "de-DE" : "en-US", acceptDownloads: false });
  page = await context.newPage();
  if (renderer === "wgpu") await page.addInitScript(() => {
    try {
      globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
    } catch {
      return;
    }
  });
  listen(page);
  await page.goto(routeUrl(), { waitUntil: "domcontentloaded", timeout: 90000 }).catch((error) => log(`${label} goto failed ${String(error).split("\n")[0]}`));
  const booted = await waitForBoot(label, polls, device.windows);
  if (booted) {
    await installBandTrace();
    await installNoticeTrace();
  }
  return booted;
};

/** 🔀️ Makes `peer` the page every helper drives. */
const use = (peer: Peer) => {
  context = peer.context;
  page = peer.page;
};

/** 🪟️ Runs `body` on fresh pages it opens through {@link openPage}, then closes every one of them and restores the locale's main
 * page — a step in its own document never disturbs the main flow. */
const onFreshPages = async (body: (open: (device: Device, label: string) => Promise<Peer | null>) => Promise<void>) => {
  const main: Peer = { context, page };
  const opened: Peer[] = [];
  try {
    await body(async (device, label) => {
      const booted = await openPage(device, label);
      opened.push({ context, page });
      return booted ? { context, page } : null;
    });
  } finally {
    for (const peer of opened) {
      use(peer);
      await harvestNotices();
      await peer.context.close().catch(() => {});
    }
    use(main);
  }
};
//#endregion 🔖️Page

//#region 🔖️Wgpu
/** 🧊️ The wgpu shell paints on a canvas and publishes no DOM of its own. What a user (or an assistive technology) can
 * reach is: the ARIA mirror `#semio-wgpu-accessibility` (one element per projected node, `data-node-key` = the node key or
 * chrome control id, `data-window` = its window; a synthetic `click` on an actionable element is an `accessibility-activate`,
 * typing into a mirrored input an `accessibility-value`), the chrome hit registry (`semioWgpuIntrospection.dumpChrome()`:
 * every pointer target with its page rect, and the dispatched-action ledger when `SEMIO_RUNTIME_DIAGNOSTICS` is armed), and
 * the canvas itself for pointer and keyboard input. The board's positions, camera and selection are read from
 * `semioWgpuIntrospection.dumpBoard2d(windowId)`, the wgpu twin of React's `data-board-*` vitals.
 * @see ../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts
 * @see ../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs */
type MirrorNode = { key: string; window: string; role: string; label: string; description: string; disabled: boolean; expanded: string | null; pressed: string | null; selected: string | null; value: string | null; valueNow: string | null; valueMin: string | null; valueMax: string | null; valueText: string | null; actionable: boolean; focused: boolean; live: string | null; busy: boolean; shortcut: string | null; tag: string };
type Board2dSurface = { surfaceId: string; windowId: string; rect: [number, number, number, number]; camera: { x: number; y: number; zoom: number } | null; positions: Positions; selection: string[]; highlighted?: string[]; nodes: number; edges: number; handles: number; parsed: boolean };

/** 🪞️ Every mirrored node in reading order, with the ARIA state the mirror stamps. */
const mirror = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).map((el) => {
        const describedBy = el.getAttribute("aria-describedby");
        return {
          key: el.dataset.nodeKey ?? "",
          window: el.dataset.window ?? "",
          role: el.getAttribute("role") ?? el.tagName.toLowerCase(),
          label: el.getAttribute("aria-label") ?? el.textContent ?? "",
          description: describedBy ? (document.getElementById(describedBy)?.textContent ?? "") : "",
          disabled: el.getAttribute("aria-disabled") === "true",
          expanded: el.getAttribute("aria-expanded"),
          pressed: el.getAttribute("aria-pressed"),
          selected: el.getAttribute("aria-selected"),
          value: el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement ? el.value : null,
          valueNow: el.getAttribute("aria-valuenow"),
          valueMin: el.getAttribute("aria-valuemin"),
          valueMax: el.getAttribute("aria-valuemax"),
          valueText: el.getAttribute("aria-valuetext"),
          actionable: el.dataset.actionable === "true",
          focused: el.dataset.focused === "true" || document.activeElement === el,
          live: el.getAttribute("aria-live"),
          busy: el.getAttribute("aria-busy") === "true",
          shortcut: el.getAttribute("aria-keyshortcuts"),
          tag: el.tagName.toLowerCase(),
        };
      }),
    [] as MirrorNode[],
  );

/** 🔎️ The mirrored node an authored key names: exact, then a `/`- or `.`-delimited suffix (a chrome control id prefixes
 * the tab id: `shell.panel.tab.<anchor>.<tabId>`), never a bare substring. */
const mirrorFind = (nodes: MirrorNode[], authored: string) =>
  nodes.find((node) => node.key === authored) ?? nodes.find((node) => !node.key.includes("::") && (node.key.endsWith(`/${authored}`) || node.key.endsWith(`.${authored}`)));

/** 🧷️ Stamps the mirrored element of `key` (in `window` when given) with a probe token and answers its locator. */
const mirrorLocator = async (key: string, windowId?: string) => {
  const token = markToken();
  const found = await evalSafe(
    (arg) => {
      const el = Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).find((node) => node.dataset.nodeKey === arg.key && (!arg.windowId || node.dataset.window === arg.windowId));
      el?.setAttribute("data-probe-target", arg.token);
      return Boolean(el);
    },
    false,
    { key, windowId: windowId ?? null, token },
  );
  return found ? page.locator(`[data-probe-target="${token}"]`).first() : null;
};

/** 👆️ Activates one mirrored node the way an assistive technology does: a `click` on its mirror element, which the mirror
 * forwards as `accessibility-activate` to the renderer. */
const mirrorActivate = async (key: string, windowId?: string) => {
  const target = await mirrorLocator(key, windowId);
  if (!target) return false;
  await target.dispatchEvent("click").catch(() => {});
  return true;
};

/** 🎯️ Focuses one mirrored node (the mirror forwards `accessibility-focus`, so keyboard input then reaches that node). */
const mirrorFocus = async (key: string) => {
  const target = await mirrorLocator(key);
  if (!target) return false;
  await target.focus().catch(() => {});
  return true;
};

/** 🧲️ The chrome hit registry: every pointer target with its page rect (CSS px). */
const chromeHits = async () => {
  const json = await evalSafe(async () => (await (window as unknown as { semioWgpuIntrospection?: { dumpChrome: () => Promise<string> } }).semioWgpuIntrospection?.dumpChrome()) ?? "", "");
  try {
    return (JSON.parse(json || "{}") as { hits?: { controlId: string; kind: string; rect: [number, number, number, number]; windowId?: string; action?: string }[]; actions?: { seq: number; controllerId: string; action: string; origin: string; args?: string }[] });
  } catch {
    return {};
  }
};

/** 🖱️ Presses a chrome control with the real pointer at the centre of its registered hit rect. */
const pointerPress = async (authored: string) => {
  const hits = (await chromeHits()).hits ?? [];
  const hit = hits.find((row) => row.controlId === authored) ?? hits.find((row) => row.controlId.endsWith(`.${authored}`) || row.controlId.endsWith(`/${authored}`));
  if (!hit) return null;
  await page.mouse.click(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
  return hit.controlId;
};

/** ☝️ Presses `authored` through the mirror (activation), else with the pointer on its chrome hit rect. */
const wgpuPress = async (authored: string) => {
  const node = mirrorFind(await mirror(), authored);
  if (node && (await mirrorActivate(node.key, node.window))) return `mirror:${node.key}`;
  const hit = await pointerPress(authored);
  return hit ? `pointer:${hit}` : null;
};

/** 🎲️ The overview board as `dumpBoard2d` publishes it, or null while the renderer lacks the export (a named prerequisite). */
const wgpuBoard = async () => {
  const json = await evalSafe(
    async (windowId) => {
      const introspection = (window as unknown as { semioWgpuIntrospection?: { dumpBoard2d?: (windowId?: string) => Promise<string> } }).semioWgpuIntrospection;
      return typeof introspection?.dumpBoard2d === "function" ? await introspection.dumpBoard2d(windowId) : null;
    },
    null as string | null,
    OVERVIEW,
  );
  if (!json) return null;
  try {
    const dump = JSON.parse(json) as { surfaces?: Board2dSurface[] };
    return dump.surfaces?.find((surface) => surface.windowId === OVERVIEW) ?? dump.surfaces?.[0] ?? null;
  } catch {
    return null;
  }
};

/** 📜️ The contract `dumpBoard2d` is asked to answer — the wgpu twin of React's Board2dHost `data-board-*` vitals. */
const WGPU_BOARD_CONTRACT = "semioWgpuIntrospection.dumpBoard2d(windowId?) → {surfaces:[{surfaceId, windowId, rect:[x,y,w,h] (page CSS px of the board canvas), camera:{x,y,zoom}, positions:{nodeId:[x,y]} (the published fixture), selection:[id], highlighted:[id] (`highlighted_ids_json`, the ids an open time-travel draft references — React `data-board-highlighted-ids-json`), nodes, edges, handles, parsed}]} — read-only, from the Board2d scene the frame worker already holds (fixture_json, camera_json, selection_json, highlighted_ids_json)";

/** 🎮️ Band controls as the wgpu shell names them (`⏪️time-travel` `TimeTravelControl::control_id`). */
const WGPU_BAND_CONTROLS: readonly (readonly [string, string])[] = [
  ["accept", "ui.timeTravel.accept"],
  ["discard", "ui.timeTravel.discard"],
  ["cancelReplay", "shell.time-travel.cancel-replay"],
  ["rerun", "shell.time-travel.rerun"],
  ["finalize", "shell.time-travel.finalize"],
  ["back", "shell.time-travel.back"],
  ["exit", "ui.timeTravel.exit"],
];

/** 🪧️ Stage and review words of the band's announced message (`TimeTravelLabel`, en and de), in match order. */
const BAND_WORDS: readonly (readonly [string, RegExp])[] = [
  ["replaying", /Replaying later mutations|Spätere Mutationen werden neu angewendet/],
  ["reviewing", /Reviewing the edited history|Bearbeiteter Verlauf wird geprüft/],
  ["choosing", /Choose how to finalize|Art des Abschlusses wählen/],
  ["finalizing", /Finalizing the history edit|Verlaufsbearbeitung wird abgeschlossen/],
  ["editing", /Editing a mutation|Mutation wird bearbeitet/],
];
const REVIEW_WORDS: readonly (readonly [string, RegExp])[] = [
  ["ready", /Ready to finalize|Bereit zum Abschließen/],
  ["blocked", /Errors must be fixed or withdrawn before finalizing|Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden/],
  ["needsReplay", /Replay needed|Neu anwenden nötig/],
  ["noChanges", /No changes: showing the current history|Keine Änderungen: aktueller Verlauf wird angezeigt/],
];

/** 📣️ The band as the wgpu shell announces it: the live `shell.time-travel.status` node (its message joins stage · target ·
 * progress · review · worst · fault · accepted) and the band controls the mirror carries. */
const wgpuBand = async (): Promise<Band | null> => {
  const nodes = await mirror();
  const status = nodes.find((node) => node.key === "shell.time-travel.status");
  if (!status) return null;
  const text = status.label;
  const controls = WGPU_BAND_CONTROLS.map(([control, key]) => [control, nodes.find((node) => node.key === key)] as const)
    .filter((entry): entry is readonly [string, MirrorNode] => Boolean(entry[1]))
    .map(([control, node]) => ({ control, disabled: node.disabled, title: node.description || null, keys: node.shortcut, text: node.label }));
  return {
    stage: BAND_WORDS.find(([, pattern]) => pattern.test(text))?.[0] ?? "",
    role: status.role,
    live: status.live,
    generation: null,
    text,
    review: REVIEW_WORDS.find(([, pattern]) => pattern.test(text))?.[0] ?? null,
    outcome: /Worst outcome: (\w+)|Schwerstes Ergebnis: (\w+)/.exec(text)?.slice(1).find(Boolean)?.toLowerCase() ?? null,
    fault: null,
    target: text,
    progress: status.valueNow !== null && status.valueMax !== null ? `${status.valueNow}/${status.valueMax}` : null,
    controls,
  };
};
//#endregion 🔖️Wgpu

//#region 🔖️Board
type Vitals = { surface: string; nodes: number; edges: number; handles: number; parsed: string; selection: string; camera: string; transform: string; utility: string; highlighted: string | null };

/** 🩺️ The overview pane's board vitals, read without a guest round trip: React's `data-board-*` attributes, wgpu's
 * `dumpBoard2d`. */
const vitals = async (): Promise<Vitals | null> => {
  if (renderer === "wgpu") {
    const board = await wgpuBoard();
    return board ? { surface: board.surfaceId, nodes: board.nodes, edges: board.edges, handles: board.handles, parsed: String(board.parsed), selection: JSON.stringify(board.selection), camera: JSON.stringify(board.camera), transform: "", utility: "", highlighted: board.highlighted ? JSON.stringify(board.highlighted) : null } : null;
  }
  return reactVitals();
};

/** 💓️ React's Board2dHost `data-board-*` vitals of the overview pane. */
const reactVitals = () =>
  evalSafe(
    (surface) => {
      const el = document.querySelector(`[data-surface-id="${surface}"]`);
      if (!el) return null;
      return {
        surface,
        nodes: Number(el.getAttribute("data-board-nodes") ?? "-1"),
        edges: Number(el.getAttribute("data-board-edges") ?? "-1"),
        handles: Number(el.getAttribute("data-board-handles") ?? "-1"),
        parsed: el.getAttribute("data-board-fixture-parsed") ?? "",
        selection: el.getAttribute("data-board-selection-json") ?? "",
        camera: el.getAttribute("data-board-camera-json") ?? "",
        transform: el.getAttribute("data-board-transform-json") ?? "",
        utility: el.getAttribute("data-board-active-utility") ?? "",
        highlighted: el.getAttribute("data-board-highlighted-ids-json"),
      };
    },
    null as Vitals | null,
    `window:${OVERVIEW}`,
  );

/** 📍️ Every node position the overview pane publishes (React `data-board-positions-json`, wgpu `dumpBoard2d`). */
const positions = async (): Promise<Positions> => (renderer === "wgpu" ? ((await wgpuBoard())?.positions ?? {}) : reactPositions());
const reactPositions = async () => JSON.parse(await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-positions-json") ?? "{}", "{}", `window:${OVERVIEW}`)) as Positions;
const selectionIds = (v: Vitals | null) => {
  try {
    return JSON.parse(v?.selection || "[]") as string[];
  } catch {
    return [] as string[];
  }
};
/** 🔦️ The ids the board highlights for an open draft's references, or null while the renderer publishes none. */
const highlightedIds = (v: Vitals | null) => {
  if (v?.highlighted === null || v?.highlighted === undefined) return null;
  try {
    return JSON.parse(v.highlighted || "[]") as string[];
  } catch {
    return null;
  }
};
const cameraOf = (v: Vitals | null) => {
  try {
    const camera = (JSON.parse(v?.camera || "{}") ?? {}) as { x?: number; y?: number; zoom?: number };
    return { x: camera.x ?? 0, y: camera.y ?? 0, zoom: camera.zoom && camera.zoom > 0 ? camera.zoom : 1 };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
};
const paneBox = async () => {
  if (renderer === "wgpu") {
    const rect = (await wgpuBoard())?.rect;
    return rect ? { x: rect[0], y: rect[1], width: rect[2], height: rect[3] } : { x: 0, y: 0, width: 1, height: 1 };
  }
  return (await page.locator(`[data-surface-id="window:${OVERVIEW}"] canvas`).first().boundingBox().catch(() => null)) ?? { x: 0, y: 0, width: 1, height: 1 };
};

/** 📌️ World → screen through the published camera: the pane centre is the camera position. */
const toScreen = (world: [number, number], camera: { x: number; y: number; zoom: number }, box: { x: number; y: number; width: number; height: number }): Point => ({
  x: box.x + box.width / 2 + (world[0] - camera.x) * camera.zoom,
  y: box.y + box.height / 2 + (world[1] - camera.y) * camera.zoom,
});
const inside = (point: Point, box: { x: number; y: number; width: number; height: number }, margin: number) => point.x > box.x + margin && point.x < box.x + box.width - margin && point.y > box.y + margin && point.y < box.y + box.height - margin;

/** 🗺️ The screen layout of every node: its point, whether it is inside the pane and its nearest neighbour in pixels. */
const layout = async () => {
  const [v, p, box] = [await vitals(), await positions(), await paneBox()];
  const camera = cameraOf(v);
  const points = Object.entries(p).map(([id, world]) => ({ id, world, at: toScreen(world, camera, box) }));
  const rows = points.map((row) => ({ ...row, clearance: Math.min(...points.filter((other) => other.id !== row.id).map((other) => Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y)), 1e9) }));
  return { camera, box, rows };
};

/** 🔭️ Zooms out at the pane centre until at least `minimum` nodes are inside the pane with some clearance. */
const frameBoard = async (minimum: number) => {
  for (let attempt = 0; attempt < 6; attempt++) {
    const { box, rows } = await layout();
    const usable = rows.filter((row) => inside(row.at, box, 90) && row.clearance >= 14);
    if (usable.length >= minimum) return usable.length;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(1200);
  }
  return 0;
};

/** 🍒️ Picks `count` distinct nodes that are inside the pane, clickable (≥ 14 px from any other node), whose
 * `offsets` (world units) keep them inside the pane, and that are ≥ `spread` px apart from each other. */
const pickNodes = async (count: number, offsets: [number, number][], exclude: string[], spread = 40) => {
  const { camera, box, rows } = await layout();
  const picked: typeof rows = [];
  const candidates = rows
    .filter((row) => !exclude.includes(row.id) && row.clearance >= 14 && inside(row.at, box, 90))
    .filter((row) => offsets.every(([dx, dy]) => inside({ x: row.at.x + dx * camera.zoom, y: row.at.y + dy * camera.zoom }, box, 60)))
    .sort((a, b) => b.clearance - a.clearance);
  for (const row of candidates) {
    if (picked.length === count) break;
    if (picked.every((other) => Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y) >= spread && Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y) <= 420)) picked.push(row);
  }
  return { camera, box, picked };
};

/** 🧬️ A node to duplicate whose clone (`duplicate_selection_in_fixture` offsets it by +24,+24 world units) and a drag
 * destination for that clone are both ≥ 55 world units from every other node — the press on the clone lands on the
 * clone, never on a neighbour's free handle. Zooms out until one is inside the pane. */
const pickCloneSource = async (exclude: string[]) => {
  const moves: [number, number][] = [[70, 70], [90, 0], [0, 90], [-90, 0], [0, -90], [70, -70], [-70, 70], [-70, -70]];
  for (let attempt = 0; attempt < 6; attempt++) {
    const { camera, box, rows } = await layout();
    const clear = (world: [number, number], skip: string[]) => rows.every((other) => skip.includes(other.id) || Math.hypot(other.world[0] - world[0], other.world[1] - world[1]) >= 55);
    for (const row of rows) {
      if (exclude.includes(row.id) || row.clearance < 14 || !inside(row.at, box, 90)) continue;
      const clone: [number, number] = [row.world[0] + 24, row.world[1] + 24];
      if (!clear(clone, [row.id])) continue;
      const move = moves.find(([dx, dy]) => {
        const destination: [number, number] = [clone[0] + dx, clone[1] + dy];
        return clear(destination, []) && Math.hypot(destination[0] - row.world[0], destination[1] - row.world[1]) >= 55 && inside(toScreen(destination, camera, box), box, 60);
      });
      if (move) return { row, move, zoom: camera.zoom };
    }
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(1200);
  }
  return null;
};

/** 🖐️ A press–move–release of `steps` pointer moves from `from` by `(dx, dy)` screen pixels. */
const dragBy = async (from: Point, dx: number, dy: number, steps = 12) => {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  for (let index = 1; index <= steps; index++) await page.mouse.move(from.x + (dx * index) / steps, from.y + (dy * index) / steps);
  await page.mouse.up();
};

/** 🧮️ `after[id] - before[id]`, or null when either is missing. */
const offsetOf = (before: Positions, after: Positions, id: string): [number, number] | null => (before[id] && after[id] ? [after[id][0] - before[id][0], after[id][1] - before[id][1]] : null);
const near = (a: number, b: number, eps: number) => Math.abs(a - b) <= eps;
/** 📐️ Whether `id` sits at `origin + (dx, dy)` within `eps`. */
const placed = (p: Positions, id: string, origin: [number, number] | undefined, dx: number, dy: number, eps = 0.02) => Boolean(origin && p[id] && near(p[id][0], origin[0] + dx, eps) && near(p[id][1], origin[1] + dy, eps));
/** 🚩️ The ids whose positions differ between two readings (beyond `eps`), plus ids present in only one. */
const movedIds = (a: Positions, b: Positions, eps = 1e-6) => {
  const ids = new Set([...Object.keys(a), ...Object.keys(b)]);
  return [...ids].filter((id) => !a[id] || !b[id] || !near(a[id][0], b[id][0], eps) || !near(a[id][1], b[id][1], eps));
};
//#endregion 🔖️Board

//#region 🔖️Chrome
/** 🪪️ The live DOM id of an authored id: exact, or namespaced `…/<authored>` (panel bodies, window measures). */
const resolveDomId = (authored: string) =>
  evalSafe(
    (target) => {
      for (const el of Array.from(document.querySelectorAll("[id]"))) if (el.id === target || el.id.endsWith(`/${target}`)) return el.id;
      return null;
    },
    null as string | null,
    authored,
  );
const byId = (id: string) => page.locator(`[id="${id.replace(/"/g, '\\"')}"]`).first();

/** 🗝️ The live handle of an authored key whichever renderer answers: React's DOM id, wgpu's mirror key. */
const presentKey = async (authored: string) => (renderer === "wgpu" ? (mirrorFind(await mirror(), authored)?.key ?? null) : resolveDomId(authored));

/** 🚥️ The per-window time-travel indicators as `stage-or-role:accessible name` (React `[data-semio-time-travel-indicator]`,
 * wgpu `framework.window.<id>.timeTravel.indicator`, a `note`). */
const indicators = async () =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /framework\.window\..+\.timeTravel\.indicator$/.test(node.key)).map((node) => `${node.role}:${node.label}`)
    : evalSafe(() => Array.from(document.querySelectorAll("[data-semio-time-travel-indicator]")).map((el) => `${el.getAttribute("data-semio-time-travel-indicator")}:${el.getAttribute("aria-label")}`), [] as string[]);

/** 👥️ The presence roster's accessible text (React `#s-presence-peers`, wgpu the `s-presence-peers` status node). */
const presenceText = async () =>
  renderer === "wgpu"
    ? (mirrorFind(await mirror(), "s-presence-peers")?.label ?? null)
    : evalSafe(() => {
        const el = document.getElementById("s-presence-peers");
        return el ? `${el.getAttribute("aria-label") ?? ""} ${(el as HTMLElement).innerText}`.trim() : null;
      }, null as string | null);

/** 🔡️ The visible text of an authored key: React's `innerText`, wgpu's accessible name and description. */
const textOfKey = async (authored: string) => {
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), authored);
    return node ? `${node.label} ${node.description}`.trim() : "";
  }
  const id = await resolveDomId(authored);
  return id ? await byId(id).innerText().catch(() => "") : "";
};

/** 🗃️ The dock panels that are open, by tab id (wgpu: the chrome tab `shell.panel.tab.<anchor>.<tabId>` reads pressed). */
const openPanelTabIds = async () =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /^shell\.panel\.tab\./.test(node.key) && (node.pressed === "true" || node.selected === "true")).map((node) => node.key.replace(/^shell\.panel\.tab\.[^.]+\./, ""))
    : reactOpenPanelTabIds();
const reactOpenPanelTabIds = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).filter((el) => (el as HTMLElement).offsetParent !== null).map((el) => el.id.replace(/^framework\.panelTab\./, "")), [] as string[]);

/** 🍔️ The phone layout's one toggle for the merged mobile panel (`ui.mobilePanel.toggle`, a navbar Toggle). */
const MOBILE_PANEL_TOGGLE = "ui.mobilePanel.toggle";

/** 🔓️ Opens (never toggles) the panel tab `id`; at phone width the tab lives in the merged mobile panel, so its toggle opens
 * first. */
const openTab = async (id: string) => {
  if ((await openPanelTabIds()).includes(id)) return true;
  if (renderer === "wgpu") {
    let via = await wgpuPress(id);
    if (via === null && (await wgpuPress(MOBILE_PANEL_TOGGLE)) !== null) {
      await sleep(700);
      via = await wgpuPress(id);
    }
    return via !== null && (await waitUntil(openPanelTabIds, (ids) => ids.includes(id), 8000)).ok;
  }
  const tab = page.locator(`[data-slot="panel-tab-button"][id="${id}"], [id="${id}"]`).first();
  const toggle = page.locator(`[id="${MOBILE_PANEL_TOGGLE}"]`).first();
  if (!(await tab.count().catch(() => 0)) && (await toggle.count().catch(() => 0))) {
    await toggle.click({ timeout: 3000 }).catch(() => {});
    await sleep(700);
  }
  if (!(await tab.count().catch(() => 0))) return false;
  await tab.click({ timeout: 4000 }).catch(() => {});
  return (await waitUntil(openPanelTabIds, (ids) => ids.includes(id), 8000)).ok;
};

/** 🔒️ Closes every open dock panel (they overlay the board, and a pointer gesture there lands on the panel), and the merged
 * mobile panel when it is open. */
const closePanels = async () => {
  for (const id of await openPanelTabIds()) {
    if (renderer === "wgpu") await wgpuPress(id);
    else await page.locator(`[data-slot="panel-tab-button"][id="${id}"]`).first().click({ timeout: 3000 }).catch(() => {});
    await sleep(300);
  }
  if (renderer === "wgpu") {
    if (mirrorFind(await mirror(), MOBILE_PANEL_TOGGLE)?.pressed === "true") await wgpuPress(MOBILE_PANEL_TOGGLE);
  } else {
    const toggle = page.locator(`[id="${MOBILE_PANEL_TOGGLE}"][aria-pressed="true"], [id="${MOBILE_PANEL_TOGGLE}"][data-state="on"]`).first();
    if (await toggle.count().catch(() => 0)) await toggle.click({ timeout: 3000 }).catch(() => {});
  }
  return openPanelTabIds();
};

/** 🙈️ Dismisses a boot tour dialog by its Skip/Close button (en or de), scoped to dialogs only. */
const dismissTour = async () => {
  if (renderer === "wgpu") {
    if (mirrorFind(await mirror(), "ui.introduction.skip") && (await wgpuPress("ui.introduction.skip"))) log("dismissed tour ui.introduction.skip");
    return;
  }
  for (const pattern of COPY[currentLocale].tourButtons) {
    const button = page.locator('[role="dialog"] button', { hasText: pattern }).first();
    if (await button.count().catch(() => 0)) {
      await button.click({ timeout: 2000 }).catch(() => {});
      await sleep(600);
      log(`dismissed tour ${pattern}`);
    }
  }
};

/** 🥾️ Waits for `windows` board windows (three on a desktop, one at phone width) and a canvas, dismissing a tour on the way. */
const waitForBoot = async (label: string, polls = 100, windows = 3) => {
  for (let index = 0; index < polls; index++) {
    await sleep(3000);
    const shape = renderer === "wgpu" ? await wgpuBootShape() : await evalSafe(() => ({ windows: document.querySelectorAll('[data-slot="window"]').length, canvases: document.querySelectorAll("canvas").length, dialogs: document.querySelectorAll('[role="dialog"]').length, body: document.body?.innerText.slice(0, 160) ?? "" }), { windows: 0, canvases: 0, dialogs: 0, body: "" });
    if (shape.dialogs) await dismissTour();
    if (shape.windows >= windows && shape.canvases >= 1) {
      log(`${label} booted after ${((index + 1) * 3).toFixed(0)} s: ${JSON.stringify(shape).slice(0, 200)}`);
      await dismissTour();
      return true;
    }
    if (index % 5 === 4) log(`${label} waiting… ${JSON.stringify(shape).slice(0, 200)}`);
  }
  return false;
};

/** 🌅️ The wgpu boot shape: the introspection shim is attached only once the frame worker booted, its structure dump
 * names the live windows, and the ARIA mirror holds nodes once the first frame was projected. */
const wgpuBootShape = async () => {
  const shape = await evalSafe(
    async () => {
      const introspection = (window as unknown as { semioWgpuIntrospection?: { dumpStructure: () => Promise<string> } }).semioWgpuIntrospection;
      let windowIds: string[] = [];
      try {
        windowIds = introspection ? ((JSON.parse((await introspection.dumpStructure()) || "{}") as { windowIds?: string[] }).windowIds ?? []) : [];
      } catch {
        windowIds = [];
      }
      const mirrorRoot = document.getElementById("semio-wgpu-accessibility");
      return { booted: Boolean(introspection), windowIds, mirrorNodes: Number(mirrorRoot?.dataset.nodeCount ?? "0"), canvases: document.querySelectorAll("canvas").length };
    },
    { booted: false, windowIds: [] as string[], mirrorNodes: 0, canvases: 0 },
  );
  const boards = shape.windowIds.filter((id) => id.startsWith("2d-")).length;
  return { windows: boards, canvases: shape.canvases, dialogs: mirrorFind(await mirror(), "ui.introduction.skip") ? 1 : 0, body: JSON.stringify(shape).slice(0, 160) };
};

/** 🎟️ Marks the first element `find` answers with `data-probe-target=<token>` in page, so Playwright can click the
 * exact element a structural search found. */
const markToken = (() => {
  let next = 0;
  return () => `probe-${++next}`;
})();
//#endregion 🔖️Chrome

//#region 🔖️History
type HistoryRow = { id: string; kind: "entry" | "mutation"; key: string; label: string; text: string; expandable: boolean; expanded: boolean; parent: string | null; index: number };

/** 📃️ Every materialised history row (`framework.history.entry.<seq>` and `framework.history.mutation.<id>`) in reading
 * order; a mutation row belongs to the entry row above it. wgpu answers from the ARIA mirror (label plus description). */
const readHistory = async (): Promise<HistoryRow[]> => {
  if (renderer !== "wgpu") return reactReadHistory();
  let parent: string | null = null;
  return (await mirror())
    .filter((node) => !node.key.includes("::") && /(^|\/)framework\.history\.(entry\.\d+$|mutation\.)/.test(node.key))
    .map((node, index) => {
      const match = node.key.match(/framework\.history\.(entry|mutation)\.(.+)$/)!;
      const kind = match[1] as "entry" | "mutation";
      if (kind === "entry") parent = node.key;
      return { id: node.key, kind, key: match[2], label: node.label.replace(/\s+/g, " ").trim(), text: `${node.label} ${node.description}`.replace(/\s+/g, " ").trim().slice(0, 400), expandable: node.expanded !== null, expanded: node.expanded === "true", parent: kind === "mutation" ? parent : null, index };
    });
};
const reactReadHistory = () =>
  evalSafe(
    () => {
      const rows = Array.from(document.querySelectorAll("[id]")).filter((el) => /(^|\/)framework\.history\.(entry\.\d+$|mutation\.)/.test(el.id));
      let parent: string | null = null;
      return rows.map((el, index) => {
        const match = el.id.match(/framework\.history\.(entry|mutation)\.(.+)$/)!;
        const labelEl = el.querySelector('[data-slot="tree-label"]') as HTMLElement | null;
        const first = labelEl?.querySelector("span") as HTMLElement | null;
        const kind = match[1] as "entry" | "mutation";
        if (kind === "entry") parent = el.id;
        return {
          id: el.id,
          kind,
          key: match[2],
          label: (first?.innerText ?? labelEl?.innerText ?? "").replace(/\s+/g, " ").trim(),
          text: [(el as HTMLElement).innerText, el.getAttribute("title") ?? "", ...Array.from(el.querySelectorAll("[title]")).map((node) => node.getAttribute("title") ?? "")].join(" ").replace(/\s+/g, " ").trim().slice(0, 400),
          expandable: el.hasAttribute("aria-expanded"),
          expanded: el.getAttribute("aria-expanded") === "true",
          parent: kind === "mutation" ? parent : null,
          index,
        };
      });
    },
    [] as HistoryRow[],
  );

/** ⏫️ Scrolls the history panel's scroll container to its start or end (the Commands section is windowed). wgpu: a wheel
 * over the panel's registered scroll region. */
const scrollHistory = async (where: "start" | "end") => {
  if (renderer !== "wgpu") return reactScrollHistory(where);
  const region = ((await chromeHits()).hits ?? []).find((hit) => /scroll/i.test(hit.kind) && /history/i.test(`${hit.controlId} ${hit.windowId ?? ""}`));
  if (!region) return false;
  await page.mouse.move(region.rect[0] + region.rect[2] / 2, region.rect[1] + region.rect[3] / 2);
  await page.mouse.wheel(0, where === "end" ? 20000 : -20000);
  return true;
};
const reactScrollHistory = (where: "start" | "end") =>
  evalSafe(
    (edge) => {
      const row = Array.from(document.querySelectorAll("[id]")).find((el) => /framework\.history\.(entry\.\d+|commands)$/.test(el.id));
      let node = row?.parentElement ?? null;
      while (node) {
        const style = getComputedStyle(node);
        if ((style.overflowY === "auto" || style.overflowY === "scroll") && node.scrollHeight > node.clientHeight + 4) {
          node.scrollTop = edge === "end" ? node.scrollHeight : 0;
          return true;
        }
        node = node.parentElement;
      }
      return false;
    },
    false,
    where,
  );

/** 🗄️ Opens the History panel and waits for its body (`framework.history.actions`). */
const openHistory = async () => {
  const opened = await openTab(HISTORY_TAB);
  const body = await waitUntil(() => presentKey("framework.history.actions"), (id) => id !== null, 15000);
  return opened && body.ok;
};

/** 🛞️ Scrolls the History body by `fraction` of its viewport (negative = up); answers false once it cannot move that way.
 * wgpu: a wheel of that share of the panel's registered scroll region. */
const scrollHistoryBy = async (fraction: number) => {
  if (renderer === "wgpu") {
    const region = ((await chromeHits()).hits ?? []).find((hit) => /scroll/i.test(hit.kind) && /history/i.test(`${hit.controlId} ${hit.windowId ?? ""}`));
    if (!region) return false;
    await page.mouse.move(region.rect[0] + region.rect[2] / 2, region.rect[1] + region.rect[3] / 2);
    await page.mouse.wheel(0, region.rect[3] * fraction);
    return true;
  }
  return evalSafe(
    (share) => {
      const row = Array.from(document.querySelectorAll("[id]")).find((el) => /framework\.history\.(entry\.\d+|commands)$/.test(el.id));
      let node = row?.parentElement ?? null;
      while (node) {
        const style = getComputedStyle(node);
        if ((style.overflowY === "auto" || style.overflowY === "scroll") && node.scrollHeight > node.clientHeight + 4) {
          const before = node.scrollTop;
          node.scrollTop = before + node.clientHeight * share;
          return Math.abs(node.scrollTop - before) > 1;
        }
        node = node.parentElement;
      }
      return false;
    },
    false,
    fraction,
  );
};

/** 📖️ Pages through the windowed History body from its start (`down`) or its end (`up`), calling `visit` with the rows each
 * page materialises until it answers true (found) or the body stops moving / shows nothing new for two pages. */
const pageHistory = async (direction: "down" | "up", visit: (rows: HistoryRow[]) => Promise<boolean>) => {
  await openHistory();
  const scrolled = await scrollHistory(direction === "down" ? "start" : "end");
  if (scrolled) await sleep(600);
  const seen = new Set<string>();
  let idle = 0;
  for (let index = 0; index < 40; index++) {
    const rows = await readHistory();
    const fresh = rows.filter((row) => !seen.has(row.id)).length;
    for (const row of rows) seen.add(row.id);
    if (await visit(rows)) return true;
    idle = fresh ? 0 : idle + 1;
    if (!scrolled || idle >= 2 || !(await scrollHistoryBy(direction === "down" ? 0.8 : -0.8))) break;
    await sleep(350);
  }
  return false;
};

/** 🧵️ Every history row of the windowed body, paged from its start to its end (it ends scrolled to the newest rows), merged by id. */
const allHistoryRows = async () => {
  const merged = new Map<string, HistoryRow>();
  await pageHistory("down", async (rows) => {
    for (const row of rows) merged.set(row.id, row);
    return false;
  });
  return [...merged.values()];
};

/** 📑️ The entry rows that are document or history-edit rows: expandable rows (they carry mutation children) plus
 * rows labelled as a history edit. Chrome rows (panel toggles, tool arming) are neither. */
const documentEntries = (rows: HistoryRow[]) => rows.filter((row) => row.kind === "entry" && (row.expandable || /History edit|Verlauf bearbeitet|Verlaufsbearbeitung/.test(row.label)));

/** 🆔️ The edit ids the document rows carry: each expandable row is expanded on the page that shows it and its mutation
 * children's keys (`<editId>#<op>`) read — the label-independent identity of what history holds. */
const documentEditIds = async (_rows: HistoryRow[]) => {
  const ids = new Set<string>();
  const expanded = new Set<string>();
  await pageHistory("down", async (rows) => {
    for (const row of rows) if (row.kind === "mutation") ids.add(row.key.replace(/#\d+$/, ""));
    for (const entry of documentEntries(rows).filter((row) => row.expandable && !expanded.has(row.id)).slice(0, 12)) {
      expanded.add(entry.id);
      for (const mutation of (await expandEntry(entry.id)).mutations) ids.add(mutation.key.replace(/#\d+$/, ""));
    }
    return expanded.size >= 48;
  });
  return [...ids].sort();
};

/** 🔤️ Whether a row label is the locale-invariant op text a row falls back to without a label (`create-node node { id=… }`). */
const readsAsOpText = (label: string) => /^[a-z]+(?:-[a-z]+)+\s/.test(label) || /\{\s*\w+=/.test(label);

/** 🕵️ The mutation row `key` names (`framework.history.mutation.<key>`): paged from the newest rows upwards, expanding each
 * collapsed document row a page shows until the row is materialised (a row lists its mutations only while expanded). */
const findMutationRow = async (key: string) => {
  let found: HistoryRow | null = null;
  const expanded = new Set<string>();
  await pageHistory("up", async (rows) => {
    found = rows.find((row) => row.kind === "mutation" && row.key === key) ?? null;
    for (const entry of [...documentEntries(rows)].reverse().filter((row) => row.expandable && !row.expanded && !expanded.has(row.id))) {
      if (found) break;
      expanded.add(entry.id);
      found = (await expandEntry(entry.id)).mutations.find((row) => row.key === key) ?? null;
    }
    return found !== null;
  });
  return found as HistoryRow | null;
};

/** 🆕️ The first mutation row of an expandable document row newer than `newestBefore` that `matches`, with its entry row. */
const newMutation = async (newestBefore: number, matches: (row: HistoryRow) => boolean, timeoutMs = 30000) => {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    for (const entry of documentEntries(await allHistoryRows()).filter((row) => row.expandable && Number(row.key) > newestBefore)) {
      const mutation = (await expandEntry(entry.id)).mutations.find(matches);
      if (mutation) return { entry: entry as HistoryRow | null, mutation: mutation as HistoryRow | null, waitedMs: Date.now() - start };
    }
    await sleep(1000);
  }
  return { entry: null as HistoryRow | null, mutation: null as HistoryRow | null, waitedMs: Date.now() - start };
};

/** 🔝️ The newest entry sequence among `rows` (entry rows are keyed by their command-log seq). The Commands list is
 * windowed, so "new rows" are the ones NEWER than this, never the ones missing from an earlier read. */
const newestEntrySeq = (rows: HistoryRow[]) => Math.max(-1, ...rows.filter((row) => row.kind === "entry").map((row) => Number(row.key)).filter(Number.isFinite));

/** 🌳️ Expands one entry row through its disclosure button (wgpu: its activation, else focus + ArrowRight) and waits for
 * its mutation children. */
const expandEntry = async (entryId: string) => {
  if (renderer === "wgpu") {
    const row = (await mirror()).find((node) => node.key === entryId);
    let state = !row ? "absent" : row.expanded === "true" ? "open" : "activate";
    if (state === "activate") {
      await mirrorActivate(entryId);
      if (!(await waitUntil(readHistory, (rows) => rows.some((entry) => entry.parent === entryId), 4000)).ok) {
        state = "arrow";
        await mirrorFocus(entryId);
        await page.keyboard.press("ArrowRight").catch(() => {});
      }
    }
    const children = await waitUntil(readHistory, (rows) => rows.some((entry) => entry.parent === entryId), 10000);
    return { state, mutations: children.value.filter((entry) => entry.parent === entryId) };
  }
  const state = await evalSafe(
    (id) => {
      const row = document.getElementById(id);
      if (!row) return "absent";
      if (row.getAttribute("aria-expanded") === "true") return "open";
      const chevron = row.querySelector("button[aria-expanded]") as HTMLButtonElement | null;
      if (chevron) chevron.click();
      else (row as HTMLElement).click();
      return chevron ? "chevron" : "row";
    },
    "absent",
    entryId,
  );
  const children = await waitUntil(readHistory, (rows) => rows.some((row) => row.parent === entryId), 10000);
  return { state, mutations: children.value.filter((row) => row.parent === entryId) };
};

/** 🕹️ Presses the row action of `rowId` whose accessible name matches `name` (hovering first, the action strip is a
 * reveal region); falls back to the row's own label, which is the row's activation. */
const pressRowAction = async (rowId: string, name: RegExp) => {
  if (renderer === "wgpu") {
    const action = (await mirror()).find((node) => node.key.startsWith(`${rowId}::row-action::`) && name.test(node.label.trim()));
    if (action) return (await mirrorActivate(action.key, action.window)) ? "action" : "absent";
    return (await mirrorActivate(rowId)) ? "label" : "absent";
  }
  const token = markToken();
  await byId(rowId).hover({ timeout: 3000 }).catch(() => {});
  const via = await evalSafe(
    (arg) => {
      const row = document.getElementById(arg.rowId);
      if (!row) return "absent";
      const pattern = new RegExp(arg.source, arg.flags);
      const candidates = Array.from(row.querySelectorAll('button, [role="button"], [data-slot="action"]'));
      const hit = candidates.find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? (el as HTMLElement).innerText ?? "").trim()));
      if (hit) {
        hit.setAttribute("data-probe-target", arg.token);
        return "action";
      }
      const label = row.querySelector('[data-slot="tree-label"]');
      (label ?? row).setAttribute("data-probe-target", arg.token);
      return "label";
    },
    "absent",
    { rowId, source: name.source, flags: name.flags, token },
  );
  if (via !== "absent") await page.locator(`[data-probe-target="${token}"]`).first().click({ force: true, timeout: 4000 }).catch((error) => log(`pressRowAction ${rowId} click failed ${String(error).split("\n")[0]}`));
  return via;
};

/** 🔘️ Clicks a panel-body button by its authored id (`framework.history.editor.withdraw`, …). */
const pressAuthored = async (authored: string) => {
  const section = authored.match(/framework\.history\.(editor|timeTravel)/)?.[0];
  if (section) {
    await revealHistory(section);
    await revealHistory(`${authored}.row`);
  }
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const node = mirrorFind(nodes, authored) ?? mirrorFind(nodes, `${authored}.row`);
    if (!node) return { present: false, disabled: null as boolean | null, id: null as string | null };
    await mirrorActivate(node.key, node.window);
    return { present: true, disabled: node.disabled as boolean | null, id: node.key as string | null };
  }
  const id = (await resolveDomId(authored)) ?? (await resolveDomId(`${authored}.row`));
  if (!id) return { present: false, disabled: null as boolean | null, id: null as string | null };
  const disabled = await evalSafe((target) => {
    const el = document.getElementById(target);
    const button = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
    if (button) return button.disabled;
    const flag = el?.getAttribute("aria-disabled") ?? el?.getAttribute("data-disabled");
    return flag === null || flag === undefined ? null : flag === "true" || flag === "";
  }, null as boolean | null, id);
  const target = page.locator(`[id="${id}"]`).first();
  const tag = await target.evaluate((el) => el.tagName.toLowerCase()).catch(() => "");
  const inner = tag === "button" ? 1 : await target.locator("button").count().catch(() => 0);
  await (tag === "button" ? target : inner ? target.locator("button").first() : target.locator('[data-slot="tree-label"]').first()).click({ force: true, timeout: 4000 }).catch(() => target.click({ force: true, timeout: 4000 }).catch(() => {}));
  return { present: true, disabled, id };
};

/** 📇️ The history-panel inventory for diagnosis: every id under `framework.history` (wgpu: every mirrored history, band,
 * dialog and indicator node), with role and state. */
const historyInventory = async (): Promise<Record<string, unknown>[]> =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /framework\.history|shell\.time-travel|shell\.dialog|ui\.timeTravel|timeTravel\.indicator|framework\.sync|s-presence-peers|shell\.panel\.tab/.test(node.key)).slice(0, 400).map((node) => ({ id: node.key, window: node.window, role: node.role, expanded: node.expanded, disabled: node.disabled, aria: node.label, title: node.description, value: node.value ?? node.valueNow, text: node.label }))
    : reactHistoryInventory();
const reactHistoryInventory = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id]"))
        .filter((el) => /framework\.history|framework\.panel\.history|ui\.dialog/.test(el.id))
        .slice(0, 400)
        .map((el) => ({
          id: el.id,
          tag: el.tagName.toLowerCase(),
          slot: el.getAttribute("data-slot"),
          role: el.getAttribute("role"),
          expanded: el.getAttribute("aria-expanded"),
          disabled: (el as HTMLButtonElement).disabled ?? null,
          aria: el.getAttribute("aria-label"),
          title: el.getAttribute("title"),
          value: (el as HTMLInputElement).value ?? null,
          text: (el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 140) ?? "",
        })),
    [] as Record<string, unknown>[],
  );
//#endregion 🔖️History

//#region 🔖️Band
type BandControl = { control: string; disabled: boolean; title: string | null; keys: string | null; text: string };
type Band = { stage: string; role: string | null; live: string | null; generation: string | null; text: string; review: string | null; outcome: string | null; fault: string | null; target: string | null; progress: string | null; controls: BandControl[] };

/** 🎗️ The time-travel band, or null when no session is live: React's `[data-semio-time-travel]`, wgpu's announced
 * `shell.time-travel.status` with its mirrored controls. */
const band = (): Promise<Band | null> => (renderer === "wgpu" ? wgpuBand() : reactBand());
const reactBand = () =>
  evalSafe(
    () => {
      const el = document.querySelector("[data-semio-time-travel]");
      if (!el) return null;
      const attr = (selector: string, name: string) => el.querySelector(selector)?.getAttribute(name) ?? null;
      const progress = el.querySelector("progress");
      return {
        stage: el.getAttribute("data-semio-time-travel") ?? "",
        role: el.getAttribute("role"),
        live: el.getAttribute("aria-live"),
        generation: el.getAttribute("data-time-travel-generation"),
        text: (el as HTMLElement).innerText.replace(/\s+/g, " ").trim(),
        review: attr("[data-semio-time-travel-review]", "data-semio-time-travel-review"),
        outcome: attr("[data-semio-time-travel-outcome]", "data-semio-time-travel-outcome"),
        fault: attr("[data-semio-time-travel-fault]", "data-semio-time-travel-fault"),
        target: (el.querySelector("[data-semio-time-travel-target]") as HTMLElement | null)?.innerText ?? null,
        progress: progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : null,
        controls: Array.from(el.querySelectorAll("[data-semio-time-travel-control]")).map((button) => ({
          control: button.getAttribute("data-semio-time-travel-control") ?? "",
          disabled: (button as HTMLButtonElement).disabled,
          title: button.getAttribute("title"),
          keys: button.getAttribute("aria-keyshortcuts"),
          text: (button as HTMLElement).innerText.trim(),
        })),
      };
    },
    null as Band | null,
  );

/** 🎞️ Installs an in-page MutationObserver that records every distinct band state (stage | review | progress) with
 * its time, so a stage React rendered for one frame is still seen. Re-install after every reload. */
const installBandTrace = () =>
  renderer === "wgpu"
    ? evalSafe(
        (words) => {
          const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver };
          host.__probeBandTrace = [];
          host.__probeBandObserver?.disconnect();
          let last = "";
          const read = () => {
            const el = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
            const text = el?.getAttribute("aria-label") ?? "";
            const stage = words.stages.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const review = words.reviews.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const progress = el?.getAttribute("aria-valuenow") !== null && el?.getAttribute("aria-valuenow") !== undefined ? `${el?.getAttribute("aria-valuenow")}/${el?.getAttribute("aria-valuemax")}` : "";
            const key = el ? `${stage}|${review}|${progress}` : "none";
            if (key === last) return;
            last = key;
            host.__probeBandTrace!.push({ t: Math.round(performance.now()), key });
            if (host.__probeBandTrace!.length > 400) host.__probeBandTrace!.shift();
          };
          const root = document.getElementById("semio-wgpu-accessibility") ?? document.body;
          host.__probeBandObserver = new MutationObserver(read);
          host.__probeBandObserver.observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-label", "aria-valuenow", "aria-valuemax"] });
          read();
          return true;
        },
        false,
        { stages: BAND_WORDS.map(([stage, pattern]) => [stage, pattern.source] as [string, string]), reviews: REVIEW_WORDS.map(([review, pattern]) => [review, pattern.source] as [string, string]) },
      )
    : reactInstallBandTrace();
const reactInstallBandTrace = () =>
  evalSafe(() => {
    const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver };
    host.__probeBandTrace = [];
    host.__probeBandObserver?.disconnect();
    let last = "";
    const read = () => {
      const el = document.querySelector("[data-semio-time-travel]");
      const progress = el?.querySelector("progress");
      const key = el ? `${el.getAttribute("data-semio-time-travel")}|${el.querySelector("[data-semio-time-travel-review]")?.getAttribute("data-semio-time-travel-review") ?? ""}|${progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : ""}` : "none";
      if (key === last) return;
      last = key;
      host.__probeBandTrace!.push({ t: Math.round(performance.now()), key });
      if (host.__probeBandTrace!.length > 400) host.__probeBandTrace!.shift();
    };
    host.__probeBandObserver = new MutationObserver(read);
    host.__probeBandObserver.observe(document.body, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-semio-time-travel", "data-semio-time-travel-review", "value", "max"] });
    read();
    return true;
  }, false);
/** 🔔️ Records every transient notice code the shell shows for the page's life — React's `[data-notice-code]`, wgpu's polite
 * `shell.notice` mirror node described by the code (`🧯️wgpu-transient-notice` corpus); re-install after a reload. */
const installNoticeTrace = () =>
  evalSafe((wgpu) => {
    const host = window as unknown as { __probeNoticeCodes?: string[]; __probeNoticeObserver?: MutationObserver };
    host.__probeNoticeCodes ??= [];
    host.__probeNoticeObserver?.disconnect();
    const keep = (code: string | null | undefined) => {
      if (code && !host.__probeNoticeCodes!.includes(code)) host.__probeNoticeCodes!.push(code);
    };
    const read = () => {
      if (wgpu) {
        const node = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.notice"]');
        const describedBy = node?.getAttribute("aria-describedby");
        keep(describedBy ? document.getElementById(describedBy)?.textContent?.trim() : null);
        return;
      }
      for (const el of Array.from(document.querySelectorAll("[data-notice-code]"))) keep(el.getAttribute("data-notice-code"));
    };
    host.__probeNoticeObserver = new MutationObserver(read);
    host.__probeNoticeObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true, attributeFilter: wgpu ? undefined : ["data-notice-code"] });
    read();
    return true;
  }, false, renderer === "wgpu");
const noticeCodes = () => evalSafe(() => ((window as unknown as { __probeNoticeCodes?: string[] }).__probeNoticeCodes ?? []).slice(), [] as string[]);
const harvestedNotices: { locale: Locale; code: string }[] = [];

/** 🧺️ Keeps the current page's notice codes for the locale before its page goes away (a reload, a closed fresh context). */
const harvestNotices = async () => {
  for (const code of await noticeCodes()) if (!harvestedNotices.some((row) => row.locale === currentLocale && row.code === code)) harvestedNotices.push({ locale: currentLocale, code });
};

/** 🗣️ Every notice code the locale's pages showed so far: the harvested ones and the current page's. */
const shownNoticeCodes = async () => [...new Set([...harvestedNotices.filter((row) => row.locale === currentLocale).map((row) => row.code), ...(await noticeCodes())])];
const bandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: { t: number; key: string }[] }).__probeBandTrace ?? []).slice(), [] as { t: number; key: string }[]);
const clearBandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: unknown[] }).__probeBandTrace = []).length, 0);

/** ⌨️ Puts keyboard focus where the shell admits a chord: React listens on the document, so the active field is blurred;
 * the wgpu wire admits keys only from its canvas or a mirror element, so the largest focusable canvas takes focus. */
const prepareChord = async () => {
  if (renderer !== "wgpu") {
    await evalSafe(() => (document.activeElement as HTMLElement | null)?.blur?.(), undefined);
    return "blurred";
  }
  return evalSafe(() => {
    const canvases = Array.from(document.querySelectorAll("canvas")).sort((left, right) => right.width * right.height - left.width * left.height);
    for (const canvas of canvases) {
      canvas.focus();
      if (document.activeElement === canvas) return "canvas";
    }
    const status = document.querySelector<HTMLElement>('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
    status?.focus();
    return document.activeElement === status ? "status" : "none";
  }, "none");
};

type BandWay = "auto" | "chord" | "button";

/** ↔️ The way a chord locale does NOT take by default — the one press per locale that keeps both ways driven. */
const otherWay = (): BandWay => (chordLocales.has(currentLocale) ? "button" : "chord");

/** 🎹️ Presses one band control by its button, or by its chord (`accept`, `discard`, `exit`): in a chord locale by default,
 * always when `way` is `chord`, never when it is `button`. */
const pressBand = async (control: string, way: BandWay = "auto") => {
  const chords: Record<string, string> = { accept: "Alt+Enter", discard: "Alt+Backspace", exit: "Alt+Shift+Backspace" };
  if (chords[control] && (way === "chord" || (way === "auto" && chordLocales.has(currentLocale)))) {
    await prepareChord();
    await page.keyboard.press(chords[control]).catch(() => {});
    return `chord ${chords[control]}`;
  }
  if (renderer === "wgpu") {
    const id = WGPU_BAND_CONTROLS.find(([name]) => name === control)?.[1];
    return (id && (await wgpuPress(id))) ?? "absent";
  }
  await page.locator(`[data-semio-time-travel-control="${control}"]`).first().click({ timeout: 4000 }).catch((error) => log(`band ${control} click failed ${String(error).split("\n")[0]}`));
  return "button";
};

type DialogButton = { text: string; destructive: string | null; tone: string | null; disabled: boolean | null; description: string | null };
type FinalizeDialog = { title: string; text: string; overwrite: DialogButton | null; submit: DialogButton | null; cancel: { text: string } | null; name: { id: string; value: string } | null };

/** 🗨️ The wgpu finalize dialog through the mirror: `shell.dialog.<id>` (title) with `.choice.overwrite`, `.confirm` (New
 * alternative), `.cancel` and `.field.name`; the chrome projects only the dialog's nodes while it is open. */
const wgpuFinalizeDialog = async (): Promise<FinalizeDialog | null> => {
  const nodes = await mirror();
  const root = nodes.find((node) => /^shell\.dialog\.[^.]+$/.test(node.key) && nodes.some((other) => other.key === `${node.key}.confirm`));
  if (!root) return null;
  const button = (suffix: string): DialogButton | null => {
    const node = nodes.find((other) => other.key === `${root.key}.${suffix}`);
    return node ? { text: node.label, destructive: null, tone: null, disabled: node.disabled, description: node.description || null } : null;
  };
  const name = nodes.find((node) => node.key === `${root.key}.field.name`);
  return { title: root.label, text: nodes.filter((node) => node.key.startsWith(root.key)).map((node) => node.label).join(" ").slice(0, 500), overwrite: button("choice.overwrite"), submit: button("confirm"), cancel: button("cancel"), name: name ? { id: name.key, value: name.value ?? name.label } : null };
};

/** ✍️ Types the alternative name into the open finalize dialog; answers what the field then holds. */
const dialogFillName = async (value: string) => {
  if (renderer === "wgpu") {
    const dialog = await wgpuFinalizeDialog();
    const field = dialog?.name ? await mirrorLocator(dialog.name.id) : null;
    if (!field) return null;
    await field.fill(value, { force: true, timeout: 4000 }).catch(() => {});
    return field.inputValue({ timeout: 2000 }).catch(() => null);
  }
  const field = page.locator('[role="dialog"] input[id="name"], [role="dialog"] input[type="text"]').first();
  await field.fill(value, { timeout: 4000 }).catch(() => {});
  return field.inputValue({ timeout: 2000 }).catch(() => null);
};

/** 🗳️ Presses the finalize dialog's destructive Overwrite choice or its New alternative submit. */
const dialogChoose = async (choice: "overwrite" | "submit") => {
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const root = nodes.find((node) => /^shell\.dialog\.[^.]+$/.test(node.key) && nodes.some((other) => other.key === `${node.key}.confirm`));
    return root ? mirrorActivate(`${root.key}.${choice === "overwrite" ? "choice.overwrite" : "confirm"}`) : false;
  }
  return page.locator(`[id="${choice === "overwrite" ? "ui.dialog.choice.overwrite" : "ui.dialog.submit"}"]`).first().click({ timeout: 4000 }).then(() => true).catch((error) => {
    log(`dialog ${choice} click failed ${String(error).split("\n")[0]}`);
    return false;
  });
};

/** 💬️ The finalize dialog (`finalizeHistoryEdit`): title, the destructive Overwrite choice, the name field, submit. */
const finalizeDialog = (): Promise<FinalizeDialog | null> => (renderer === "wgpu" ? wgpuFinalizeDialog() : reactFinalizeDialog());
const reactFinalizeDialog = () =>
  evalSafe(
    () => {
      const dialog = Array.from(document.querySelectorAll('[role="dialog"]')).find((el) => el.querySelector('[id="ui.dialog.submit"]')) as HTMLElement | undefined;
      if (!dialog) return null;
      const button = (id: string) => {
        const el = dialog.querySelector(`[id="${id}"]`);
        const control = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
        const describedBy = (control ?? el)?.getAttribute("aria-describedby");
        return el ? { text: (el as HTMLElement).innerText.trim(), destructive: el.getAttribute("data-destructive") ?? control?.getAttribute("data-destructive") ?? null, tone: el.getAttribute("data-tone") ?? control?.getAttribute("data-tone") ?? null, disabled: control?.disabled ?? null, description: describedBy ? (document.getElementById(describedBy)?.textContent ?? null) : null } : null;
      };
      const name = (dialog.querySelector('input[id="name"]') ?? dialog.querySelector('input[type="text"]')) as HTMLInputElement | null;
      return {
        title: (dialog.querySelector("h2") as HTMLElement | null)?.innerText.trim() ?? dialog.innerText.split("\n")[0],
        text: dialog.innerText.replace(/\s+/g, " ").slice(0, 500),
        overwrite: button("ui.dialog.choice.overwrite"),
        submit: button("ui.dialog.submit"),
        cancel: button("ui.dialog.cancel"),
        name: name ? { id: name.id, value: name.value } : null,
      };
    },
    null as FinalizeDialog | null,
  );

type Editor = { heading: string | null; status: string | null; dx: StepperRead | null; dy: StepperRead | null; targets: { id: string; chips: string[]; useSelection: boolean; text: string } | null; inputs: string[] };
type StepperRead = { id: string; tag: string; stepper: boolean; value: string | null; step: string | null; min: string | null; max: string | null; plus: boolean; minus: boolean; role: string | null };

/** ✏️ The draft editor section: its heading, the Rust band status line, the dx/dy controls and the targets list. */
const editor = (): Promise<Editor | null> => (renderer === "wgpu" ? wgpuEditor() : reactEditor());

/** 🖊️ The draft editor through the wgpu mirror; a number input is a `spinbutton` (or `slider`) whose value the mirror
 * carries as the input value / `aria-valuenow`. */
const wgpuEditor = async (): Promise<Editor | null> => {
  const nodes = await mirror();
  const number = (pointer: string): StepperRead | null => {
    const node = mirrorFind(nodes, `framework.history.editor.input.${pointer}`);
    if (!node) return null;
    const steppers = nodes.filter((other) => other.key.startsWith(`${node.key}::`) || other.key.startsWith(`${node.key}.`));
    return { id: node.key, tag: node.tag, stepper: node.role === "spinbutton", value: node.value ?? node.valueNow, step: null, min: null, max: null, plus: steppers.some((other) => /increase|erhöhen|\+/i.test(other.label)), minus: steppers.some((other) => /decrease|verringern|−|-/i.test(other.label)), role: node.role };
  };
  const targets = mirrorFind(nodes, "framework.history.editor.input.targets");
  const chips = nodes.filter((node) => /framework\.history\.editor\.input\.targets\.chip\.\d+$/.test(node.key)).map((node) => node.label);
  return {
    heading: mirrorFind(nodes, "framework.history.editor.target")?.label ?? null,
    status: mirrorFind(nodes, "framework.history.timeTravel.status")?.label ?? null,
    dx: number("dx"),
    dy: number("dy"),
    targets: targets ? { id: targets.key, chips, useSelection: nodes.some((node) => /use selection|auswahl verwenden/i.test(node.label) && node.key.includes("framework.history.editor")), text: targets.label } : null,
    inputs: nodes.filter((node) => /framework\.history\.editor\.input\.[^/]*\.row$/.test(node.key)).map((node) => `${node.key.replace(/^.*framework\.history\.editor\.input/, "")}=${node.label.slice(0, 60)}`),
  };
};
const reactEditor = () =>
  evalSafe(
    () => {
      const find = (authored: string) => Array.from(document.querySelectorAll("[id]")).find((el) => el.id === authored || el.id.endsWith(`/${authored}`)) as HTMLElement | undefined;
      const number = (pointer: string) => {
        const el = find(`framework.history.editor.input.${pointer}`);
        if (!el) return null;
        const input = (el.tagName === "INPUT" ? el : el.querySelector("input")) as HTMLInputElement | null;
        const group = el.closest('[data-slot="stepper-group"]') ?? el.querySelector('[data-slot="stepper-group"]');
        return {
          id: el.id,
          tag: el.tagName.toLowerCase(),
          stepper: input?.getAttribute("data-stepper-input") === "true",
          value: input?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? null,
          step: input?.getAttribute("step") ?? null,
          min: input?.getAttribute("min") ?? null,
          max: input?.getAttribute("max") ?? null,
          plus: Boolean(group?.querySelector('[data-slot="stepper-plus"]')),
          minus: Boolean(group?.querySelector('[data-slot="stepper-minus"]')),
          role: el.getAttribute("role") ?? el.querySelector('[role="slider"]')?.getAttribute("role") ?? null,
        };
      };
      const targetsEl = find("framework.history.editor.input.targets");
      const chips = Array.from(document.querySelectorAll("[id]")).filter((el) => /framework\.history\.editor\.input\.targets\.chip\.\d+$/.test(el.id)).map((el) => (el as HTMLElement).innerText.replace(/\s+/g, " ").trim());
      return {
        heading: find("framework.history.editor.target")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        status: find("framework.history.timeTravel.status")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        dx: number("dx"),
        dy: number("dy"),
        targets: targetsEl ? { id: targetsEl.id, chips, useSelection: Array.from(targetsEl.querySelectorAll("button")).some((b) => /use selection|auswahl verwenden/i.test(`${b.innerText} ${b.getAttribute("aria-label") ?? ""}`)), text: targetsEl.innerText.replace(/\s+/g, " ").trim().slice(0, 200) } : null,
        inputs: Array.from(document.querySelectorAll("[id]")).filter((el) => /framework\.history\.editor\.input\.[^/]*\.row$/.test(el.id)).map((el) => `${el.id.replace(/^.*framework\.history\.editor\.input/, "")}=${(el as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 60)}`),
      };
    },
    null as Editor | null,
  );

/** 🪜️ Scrolls the history body so the section (or row) `authored` names sits at the top of the panel — what a user does to
 * reach it; the body is windowed, so rows below the viewport are not in the document until they scroll in. */
const revealHistory = async (authored: string) => {
  if (renderer === "wgpu") {
    let node = mirrorFind(await mirror(), authored);
    if (!node && (await scrollHistory("end"))) {
      await sleep(700);
      node = mirrorFind(await mirror(), authored);
    }
    if (node) await mirrorFocus(node.key);
    await sleep(node ? 500 : 0);
    return Boolean(node);
  }
  const found = await evalSafe(
    (target) => {
      const el = Array.from(document.querySelectorAll("[id]")).find((node) => node.id === target || node.id.endsWith(`/${target}`)) as HTMLElement | undefined;
      el?.scrollIntoView({ block: "start" });
      return Boolean(el);
    },
    false,
    authored,
  );
  await sleep(found ? 500 : 0);
  return found;
};

/** 📗️ {@link editor} after revealing the editor's inputs section (`framework.history.editor.inputs`). */
const readEditor = async () => {
  await openHistory();
  if (!(await revealHistory("framework.history.editor.inputs"))) await revealHistory("framework.history.editor");
  return editor();
};

/** 💯️ Types `value` into the editor's number control at `pointer` (one `fill` = one input event = one draft), then
 * Enter to commit and blur; answers the value the field held before Enter. */
const typeEditorNumber = async (pointer: string, value: number) => {
  await revealHistory("framework.history.editor.inputs");
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null };
    await field.fill(String(value), { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await mirrorFocus(node.key);
    await page.keyboard.press("Enter").catch(() => {});
    return { present: true, typed };
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) return { present: false, typed: null as string | null };
  const host = page.locator(`[id="${id}"]`).first();
  const input = (await host.evaluate((el) => el.tagName === "INPUT").catch(() => false)) ? host : host.locator("input").first();
  await input.click({ timeout: 4000 }).catch(() => {});
  await input.fill(String(value), { timeout: 4000 }).catch(() => {});
  const typed = await input.inputValue({ timeout: 2000 }).catch(() => null);
  await input.press("Enter").catch(() => {});
  return { present: true, typed };
};

/** ⬆️ Presses one key in the editor's number control at `pointer` (ArrowUp/ArrowDown step by the control's step). */
const keyEditorNumber = async (pointer: string, key: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    if (!node || !(await mirrorFocus(node.key))) return false;
    await page.keyboard.press(key).catch(() => {});
    return true;
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) return false;
  const host = page.locator(`[id="${id}"]`).first();
  const thumb = host.locator('[role="slider"]').first();
  const input = (await host.evaluate((el) => el.tagName === "INPUT").catch(() => false)) ? host : (await thumb.count().catch(() => 0)) ? thumb : host.locator("input").first();
  await input.focus().catch(() => {});
  await input.press(key).catch(() => {});
  return true;
};

type NumberControlRead = { id: string; dial: boolean; role: string | null; valueNow: number | null; valueText: string | null; min: number | null; max: number | null; ticks: { snap: number; left: number | null }[]; refusal: string | null; invalid: boolean; rowText: string };

/** 🎚️ One editor number control as a person perceives it: whether it is a dial, the thumb's announced value (display units)
 * and value text, the travel, every detent tick (its stored snap and, on a track, its axis position in %), the visible
 * refusal and the row's text. wgpu answers from the mirror (`role=slider|spinbutton`, `aria-value*`); its ticks are painted
 * only, so `ticks` is empty there. */
const readNumberControl = async (pointer: string): Promise<NumberControlRead | null> => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  const authored = `framework.history.editor.input.${pointer}`;
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const node = mirrorFind(nodes, authored);
    const row = mirrorFind(nodes, `${authored}.row`);
    if (!node) return null;
    const number = (text: string | null) => (text === null || text === "" || !Number.isFinite(Number(text)) ? null : Number(text));
    return { id: node.key, dial: /dial/i.test(`${node.description} ${node.role}`), role: node.role, valueNow: number(node.valueNow), valueText: node.valueText, min: number(node.valueMin), max: number(node.valueMax), ticks: [], refusal: row?.description || null, invalid: false, rowText: `${row?.label ?? ""} ${row?.description ?? ""}`.trim() };
  }
  return evalSafe(
    (target) => {
      const find = (suffix: string) => Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === suffix || el.id.endsWith(`/${suffix}`));
      const host = find(target);
      if (!host) return null;
      const row = find(`${target}.row`) ?? host.parentElement ?? host;
      const thumb = host.querySelector<HTMLElement>('[role="slider"]') ?? (host.getAttribute("role") === "slider" ? host : null);
      const field = (host.tagName === "INPUT" ? host : host.querySelector("input")) as HTMLInputElement | null;
      const number = (text: string | null | undefined) => (text === null || text === undefined || text === "" || !Number.isFinite(Number(text)) ? null : Number(text));
      const alert = row.querySelector<HTMLElement>('[role="alert"]');
      return {
        id: host.id,
        dial: Boolean(host.querySelector('[data-slot="slider-dial"]')),
        role: thumb?.getAttribute("role") ?? field?.getAttribute("role") ?? field?.type ?? null,
        valueNow: number(thumb?.getAttribute("aria-valuenow") ?? field?.value),
        valueText: thumb?.getAttribute("aria-valuetext") ?? field?.getAttribute("aria-valuetext") ?? null,
        min: number(thumb?.getAttribute("aria-valuemin") ?? field?.min),
        max: number(thumb?.getAttribute("aria-valuemax") ?? field?.max),
        ticks: Array.from(host.querySelectorAll<HTMLElement>('[data-slot="slider-tick"]')).map((tick) => ({ snap: Number(tick.getAttribute("data-snap")), left: tick.style?.left ? parseFloat(tick.style.left) : null })),
        refusal: alert?.textContent?.trim() || null,
        invalid: Boolean(row.querySelector('[aria-invalid="true"]') ?? host.querySelector('[data-invalid="true"]')),
        rowText: row.innerText.replace(/\s+/g, " ").trim().slice(0, 300),
      };
    },
    null as NumberControlRead | null,
    authored,
  );
};

/** 🪡️ Types `text` into a slider's typed readout the way a person does (double-click the readout, type, Enter); wgpu types
 * into the mirrored control. Answers what the field held before Enter. */
const typeSliderText = async (pointer: string, text: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  const authored = `framework.history.editor.input.${pointer}`;
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), authored);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null, via: "absent" };
    await field.fill(text, { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await page.keyboard.press("Enter").catch(() => {});
    return { present: true, typed, via: `mirror ${node.role}` };
  }
  const token = markToken();
  const readout = await evalSafe(
    (arg) => {
      const find = (suffix: string) => Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === suffix || el.id.endsWith(`/${suffix}`));
      const row = find(`${arg.target}.row`) ?? find(arg.target)?.parentElement ?? null;
      const value = row?.querySelector<HTMLElement>('[data-slot="slider-value"]');
      value?.setAttribute("data-probe-target", arg.token);
      return Boolean(value);
    },
    false,
    { target: authored, token },
  );
  if (!readout) return { present: false, typed: null as string | null, via: "no-readout" };
  await page.locator(`[data-probe-target="${token}"]`).first().dblclick({ timeout: 4000 }).catch(() => {});
  const field = page.locator('[data-slot="slider-content"] input[type="number"]').first();
  await waitUntil(() => field.count().catch(() => 0), (count) => count > 0, 4000);
  await field.fill(text, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  await field.press("Enter").catch(() => {});
  return { present: true, typed, via: "readout" };
};

/** 🫳️ Presses "Use selection" of the editor's reference input at `pointer` (React: the button inside the reference list;
 * wgpu: the mirrored button so named). */
const pressUseSelection = async (pointer: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  const pattern = COPY[currentLocale].useSelection;
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const node = nodes.find((other) => other.key.includes(`framework.history.editor.input.${pointer}`) && pattern.test(other.label.trim())) ?? nodes.find((other) => /(^|[./])useSelection$/.test(other.key) && pattern.test(other.label.trim())) ?? nodes.find((other) => pattern.test(other.label.trim()));
    if (!node) return null;
    await mirrorActivate(node.key, node.window);
    return `mirror:${node.key}`;
  }
  const token = markToken();
  const found = await evalSafe(
    (arg) => {
      const host = Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === arg.target || el.id.endsWith(`/${arg.target}`));
      const pattern = new RegExp(arg.source, arg.flags);
      const button = Array.from(host?.querySelectorAll<HTMLElement>("button") ?? []).find((el) => pattern.test(`${el.innerText}`.trim()) || pattern.test((el.getAttribute("aria-label") ?? "").trim()));
      button?.setAttribute("data-probe-target", arg.token);
      return button ? (button.id || "button") : null;
    },
    null as string | null,
    { target: `framework.history.editor.input.${pointer}`, source: pattern.source, flags: pattern.flags, token },
  );
  if (found) await page.locator(`[data-probe-target="${token}"]`).first().click({ timeout: 4000 }).catch((error) => log(`use selection click failed ${String(error).split("\n")[0]}`));
  return found;
};

type FocusRead = { key: string | null; tag: string; isBand: boolean; inBand: boolean; inDialog: boolean; inHistory: boolean };

/** 🧿️ Where keyboard focus is: React's active element (its own id, else the nearest ancestor id that names a framework or
 * dialog control), whether it IS the band region and whether it sits in the band, the prompt or the History panel; wgpu's
 * focused mirror node (the band is its live `shell.time-travel.status` node). */
const focusRead = async (): Promise<FocusRead> => {
  if (renderer === "wgpu") {
    const node = (await mirror()).find((other) => other.focused);
    const band = node?.key === "shell.time-travel.status";
    return { key: node?.key ?? null, tag: node?.tag ?? "", isBand: band, inBand: band || /^(ui\.timeTravel|shell\.time-travel)\./.test(node?.key ?? ""), inDialog: /^shell\.dialog\./.test(node?.key ?? ""), inHistory: /framework\.history/.test(node?.key ?? "") };
  }
  return evalSafe(
    () => {
      const active = document.activeElement as HTMLElement | null;
      let node: HTMLElement | null = active;
      while (node && !/framework\.|ui\.dialog/.test(node.id)) node = node.parentElement;
      return { key: node?.id ?? (active?.id || null), tag: active?.tagName.toLowerCase() ?? "", isBand: Boolean(active?.hasAttribute("data-semio-time-travel")), inBand: Boolean(active?.closest("[data-semio-time-travel]")), inDialog: Boolean(active?.closest('[role="dialog"]')), inHistory: Boolean(active?.closest('[id*="framework.panel.history"], [id*="framework.history"]')) };
    },
    { key: null, tag: "", isBand: false, inBand: false, inDialog: false, inHistory: false } as FocusRead,
  );
};
const focusIsOnAnEditorInput = (focus: FocusRead) => Boolean(focus.key && /framework\.history\.editor\.input\./.test(focus.key) && !focus.key.endsWith(".row"));

/** 👀️ Whether the editor's first input is revealed WITHOUT the probe scrolling: React — inside the window viewport and the
 * top element at its centre (nothing covers it); wgpu — mirrored (the windowed body projects only what is on screen). */
const editorRevealed = async () => {
  if (renderer === "wgpu") {
    const node = (await mirror()).find((other) => /framework\.history\.editor\.input\.[^.]+$/.test(other.key));
    return { revealed: Boolean(node), first: node?.key ?? null };
  }
  return evalSafe(
    () => {
      const input = Array.from(document.querySelectorAll<HTMLElement>('[id*="/framework.history.editor.input."]')).find((el) => !el.id.endsWith(".row"));
      if (!input) return { revealed: false, first: null as string | null };
      const rect = input.getBoundingClientRect();
      const x = rect.left + Math.min(rect.width / 2, 20);
      const y = rect.top + rect.height / 2;
      const inside = rect.width > 0 && rect.height > 0 && rect.top >= 0 && rect.bottom <= innerHeight && rect.left >= 0 && rect.right <= innerWidth;
      const top = inside ? document.elementFromPoint(x, y) : null;
      return { revealed: inside && Boolean(top && (input.contains(top) || top.contains(input))), first: input.id as string | null };
    },
    { revealed: false, first: null as string | null },
  );
};

/** 🖋️ Types `text` into the editor's text input at `pointer` (`input(Text)` commits on blur): fill, then Tab away — wgpu fills
 * the mirrored field and hands focus back to the canvas. Answers what the field held before it lost focus. */
const typeEditorText = async (pointer: string, text: string) => {
  await revealHistory("framework.history.editor.inputs");
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null };
    await field.fill(text, { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await prepareChord();
    return { present: true, typed };
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) return { present: false, typed: null as string | null };
  const host = page.locator(`[id="${id}"]`).first();
  const field = (await host.evaluate((el) => el.tagName === "INPUT" || el.tagName === "TEXTAREA").catch(() => false)) ? host : host.locator("input, textarea").first();
  await field.click({ timeout: 4000 }).catch(() => {});
  await field.fill(text, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  await field.press("Tab").catch(() => {});
  return { present: true, typed };
};

//#region 🔖️AriaOracle
/** ♿️ The structural ARIA oracle as one browser module: `aria-query` (the WAI-ARIA role model: every role's supported
 * attributes, every `aria-*` attribute that exists) and `dom-accessibility-api` (accessible names), the third-party pair the
 * React time-travel laws run in jsdom, here bundled once per run and evaluated into the live page (CDP evaluation, so no page
 * CSP applies). */
const ARIA_ORACLE_SOURCE = 'import { computeAccessibleName, getRole } from "dom-accessibility-api"; import { aria, roles } from "aria-query"; (globalThis as unknown as { __probeAria?: unknown }).__probeAria = { computeAccessibleName, getRole, aria, roles };';
let ariaOracleBundle: string | null = null;

/** 📦️ Bundles {@link ARIA_ORACLE_SOURCE} into one IIFE (resolved from this module's directory, so the repository's installed
 * copies answer); cached for the run. */
const ariaOracle = async () => {
  if (ariaOracleBundle !== null) return ariaOracleBundle;
  const entry = join(import.meta.dir, "aria-oracle.ts");
  const built = await Bun.build({
    entrypoints: [entry],
    target: "browser",
    format: "iife",
    minify: true,
    plugins: [{ name: "time-travel-aria-oracle", setup: (build) => {
      build.onResolve({ filter: /aria-oracle\.ts$/ }, () => ({ path: entry, namespace: "time-travel-aria-oracle" }));
      build.onLoad({ filter: /.*/, namespace: "time-travel-aria-oracle" }, () => ({ contents: ARIA_ORACLE_SOURCE, loader: "ts" }));
    } }],
  });
  if (!built.success || !built.outputs[0]) throw new Error(`aria oracle bundle failed: ${built.logs.map(String).join("; ").slice(0, 400)}`);
  ariaOracleBundle = await built.outputs[0].text();
  return ariaOracleBundle;
};

/** 🧩️ Evaluates the oracle bundle into the current page unless it is already there (a reload or a new page clears it). */
const ensureAriaOracle = async () => {
  if (await evalSafe(() => Boolean((window as unknown as { __probeAria?: unknown }).__probeAria), false)) return true;
  const bundle = await ariaOracle();
  await page.evaluate(bundle).catch((error) => log(`aria oracle evaluate failed ${String(error).split("\n")[0]}`));
  return evalSafe(() => Boolean((window as unknown as { __probeAria?: unknown }).__probeAria), false);
};

type AriaReport = { oracle: boolean; roots: number; checked: number; findings: string[] };

/** 🦮️ The axe-level structural findings under the elements `selectors` match (the W2-B law's rules, in the live page): an
 * unknown `aria-*` attribute, one the element's role does not support, a `labelledby`/`describedby` naming no element, a
 * control (button, field, slider, spinbutton, combobox, tree item, tab, menu item, option, checkbox, switch, link) without an
 * accessible name, and an id repeated under the roots. Hidden subtrees (`aria-hidden`, `hidden`, no layout box) are skipped. */
const ariaFindings = async (selectors: string[]): Promise<AriaReport> => {
  const oracle = await ensureAriaOracle();
  if (!oracle) return { oracle, roots: 0, checked: 0, findings: ["the aria oracle is not in the page"] };
  return evalSafe(
    (roots) => {
      type Oracle = { computeAccessibleName: (element: Element) => string; getRole: (element: Element) => string | null; aria: ReadonlyMap<string, unknown>; roles: ReadonlyMap<string, { readonly props: Readonly<Record<string, unknown>> }> };
      const oracleApi = (window as unknown as { __probeAria: Oracle }).__probeAria;
      const findings: string[] = [];
      const ids = new Map<string, number>();
      const elements = roots.flatMap((selector) => Array.from(document.querySelectorAll<HTMLElement>(selector)));
      const seen = new Set<Element>();
      let checked = 0;
      for (const root of elements) {
        for (const element of [root, ...Array.from(root.querySelectorAll<HTMLElement>("*"))]) {
          if (seen.has(element)) continue;
          seen.add(element);
          if (element.closest('[aria-hidden="true"], [hidden]')) continue;
          checked += 1;
          if (element.id) ids.set(element.id, (ids.get(element.id) ?? 0) + 1);
          const role = oracleApi.getRole(element);
          const where = `${role ?? element.tagName.toLowerCase()}#${element.id || element.getAttribute("data-node-key") || ""}`;
          for (const { name, value } of Array.from(element.attributes)) {
            if (!name.startsWith("aria-")) continue;
            if (!oracleApi.aria.has(name)) findings.push(`${where}: unknown ${name}`);
            else if (role !== null && oracleApi.roles.get(role) !== undefined && !(name in oracleApi.roles.get(role)!.props) && name !== "aria-hidden") findings.push(`${where}: ${name} not supported`);
            if ((name === "aria-describedby" || name === "aria-labelledby") && value.split(/\s+/u).some((id) => id !== "" && document.getElementById(id) === null)) findings.push(`${where}: ${name} → missing ${value}`);
          }
          const control = element.matches('button, input:not([type="hidden"]), select, textarea, [role="slider"], [role="combobox"], [role="spinbutton"], [role="treeitem"], [role="tab"], [role="menuitem"], [role="option"], [role="checkbox"], [role="switch"], [role="link"]');
          if (control && element.getClientRects().length > 0 && oracleApi.computeAccessibleName(element).trim() === "") findings.push(`${where}: no accessible name`);
        }
      }
      for (const [id, count] of ids) if (count > 1) findings.push(`duplicate id ${id}`);
      return { oracle: true, roots: elements.length, checked, findings };
    },
    { oracle: true, roots: 0, checked: 0, findings: ["the aria oracle threw in the page"] } as AriaReport,
    selectors,
  );
};

/** 🪴️ The roots an ARIA verdict checks: React's band, History panel and open prompt; wgpu's whole ARIA mirror (which projects
 * only the prompt while one is open). */
const ariaRoots = (kind: "session" | "prompt") =>
  renderer === "wgpu" ? ["#semio-wgpu-accessibility"] : kind === "prompt" ? ['[role="dialog"]'] : ["[data-semio-time-travel]", '[id="framework.panelTab.framework.panel.history"]'];

/** 🏛️ One structural-ARIA verdict over `kind`'s roots: no finding, and the roots exist. */
const ariaVerdict = async (name: string, kind: "session" | "prompt") => {
  const report = await ariaFindings(ariaRoots(kind));
  return verdict(name, report.oracle && report.roots > 0 && report.findings.length === 0, { ...report, findings: report.findings.slice(0, 24), selectors: ariaRoots(kind), oracle: "aria-query (role model) + dom-accessibility-api (accessible names), bundled into the page" });
};
//#endregion 🔖️AriaOracle

//#region 🔖️ReplayArm
type ArmSample = { t: number; stage: string; review: string; done: number | null; total: number | null };
type ArmLog = { samples: ArmSample[]; acted: { t: number; via: string; disabled: boolean | null; stage: string } | null };
type ArmAction = { kind: "cancel" } | { kind: "edit"; rowId: string; source: string; flags: string };

/** 🪤️ Arms the page to act in the first frame the band shows `replaying` — a person pressing the moment the progress bar
 * appears — and samples every band state (stage, review, progress) with its in-page time until {@link disarmReplay}. `cancel`
 * presses the band's Cancel replay (React `[data-semio-time-travel-control=cancelReplay]`, wgpu `shell.time-travel.cancel-replay`);
 * `edit` presses Edit on the row `rowId` (its Edit row action, else the row's activation, which is the same verb). */
const armReplay = (action: ArmAction) =>
  evalSafe(
    (arg) => {
      const host = window as unknown as { __probeArm?: ArmLog; __probeArmObserver?: MutationObserver };
      host.__probeArmObserver?.disconnect();
      const armedAt = performance.now();
      const armLog: ArmLog = { samples: [], acted: null };
      host.__probeArm = armLog;
      const read = (): Omit<ArmSample, "t"> => {
        if (arg.wgpu) {
          const status = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
          const text = status?.getAttribute("aria-label") ?? "";
          const stage = status ? (arg.stages.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "") : "none";
          const review = arg.reviews.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
          const now = status?.getAttribute("aria-valuenow");
          const max = status?.getAttribute("aria-valuemax");
          return { stage, review, done: now ? Number(now) : null, total: max ? Number(max) : null };
        }
        const band = document.querySelector("[data-semio-time-travel]");
        const progress = band?.querySelector("progress");
        return { stage: band?.getAttribute("data-semio-time-travel") ?? "none", review: band?.querySelector("[data-semio-time-travel-review]")?.getAttribute("data-semio-time-travel-review") ?? "", done: progress ? Number(progress.getAttribute("value")) : null, total: progress ? Number(progress.getAttribute("max")) : null };
      };
      const press = (): { via: string; disabled: boolean | null } => {
        const disabledOf = (el: Element) => (el as HTMLButtonElement).disabled === true || el.getAttribute("aria-disabled") === "true";
        if (arg.action.kind === "cancel") {
          const control = document.querySelector<HTMLElement>(arg.wgpu ? '#semio-wgpu-accessibility [data-node-key="shell.time-travel.cancel-replay"]' : '[data-semio-time-travel-control="cancelReplay"]');
          if (!control) return { via: "absent", disabled: null };
          const disabled = disabledOf(control);
          control.click();
          return { via: "cancelReplay", disabled };
        }
        const pattern = new RegExp(arg.action.source, arg.action.flags);
        const rowId = arg.action.rowId;
        if (arg.wgpu) {
          const nodes = Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]"));
          const button = nodes.find((node) => (node.dataset.nodeKey ?? "").startsWith(`${rowId}::row-action::`) && pattern.test((node.getAttribute("aria-label") ?? "").trim()));
          const target = button ?? nodes.find((node) => node.dataset.nodeKey === rowId);
          if (!target) return { via: "absent", disabled: null };
          const disabled = disabledOf(target);
          target.click();
          return { via: button ? "action" : "row", disabled };
        }
        const row = document.getElementById(rowId);
        if (!row) return { via: "absent", disabled: null };
        const button = Array.from(row.querySelectorAll<HTMLElement>('button, [role="button"]')).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
        const target = button ?? row.querySelector<HTMLElement>('[data-slot="tree-label"]') ?? row;
        const disabled = disabledOf(target) || row.getAttribute("aria-disabled") === "true";
        target.click();
        return { via: button ? "action" : "row", disabled };
      };
      const tick = () => {
        const state = read();
        const last = armLog.samples[armLog.samples.length - 1];
        if (!last || last.stage !== state.stage || last.review !== state.review || last.done !== state.done || last.total !== state.total) {
          armLog.samples.push({ t: Math.round(performance.now() - armedAt), ...state });
          if (armLog.samples.length > 800) armLog.samples.splice(1, 1);
        }
        if (armLog.acted === null && state.stage === "replaying") armLog.acted = { t: Math.round(performance.now() - armedAt), stage: state.stage, ...press() };
      };
      host.__probeArmObserver = new MutationObserver(tick);
      host.__probeArmObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true });
      tick();
      return true;
    },
    false,
    { action, wgpu: renderer === "wgpu", stages: BAND_WORDS.map(([stage, pattern]) => [stage, pattern.source] as [string, string]), reviews: REVIEW_WORDS.map(([review, pattern]) => [review, pattern.source] as [string, string]) },
  );

/** 📼️ The armed page's log: every sampled band state and the press it made (null before any). */
const replayArmLog = () => evalSafe(() => (window as unknown as { __probeArm?: ArmLog }).__probeArm ?? null, null as ArmLog | null);

/** 🧯️ Stops the arm's observer and answers its final log. */
const disarmReplay = async () => {
  const final = await replayArmLog();
  await evalSafe(() => (window as unknown as { __probeArmObserver?: MutationObserver }).__probeArmObserver?.disconnect(), undefined);
  return final;
};
//#endregion 🔖️ReplayArm
//#endregion 🔖️Band

//#region 🔖️Steps
type Ctx = {
  a?: string;
  b?: string;
  c?: string;
  p0?: Positions;
  p1?: Positions;
  pc?: Positions;
  dx?: number;
  dy?: number;
  cOffset?: [number, number];
  dragEntry?: string;
  dragMutation?: string;
  cMutation?: string;
  afterOverwrite?: Positions;
  dyEdited?: number;
  alternative?: string;
  folder?: string;
  noFolder?: boolean;
  g4?: { drag: string; p: string; q: string };
};

/** 🏹️ Finds the drag's `drag-selection` mutation row by its stable mutation id (paging the windowed body, expanding its
 * entry) and presses Edit on it; answers the band once `editing`. */
const beginDragEdit = async (ctx: Ctx) => {
  await openHistory();
  const mutation = ctx.dragMutation ? await findMutationRow(ctx.dragMutation) : null;
  if (!mutation) return { entry: null as string | null, via: "absent", band: null as Band | null, waitedMs: 0 };
  if (mutation.parent !== ctx.dragEntry) log(`drag row moved ${ctx.dragEntry} → ${mutation.parent}`);
  const via = await pressRowAction(mutation.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  return { entry: mutation.parent, via, band: settled.value, waitedMs: settled.waitedMs };
};

/** 🖍️ Presses Edit on the mutation row `key` names (also from a review: Begin on another target) and answers the band once
 * it reads `editing`. */
const beginEditOf = async (key: string) => {
  await openHistory();
  const row = await findMutationRow(key);
  if (!row) return { row: null as HistoryRow | null, via: "absent", band: null as Band | null, waitedMs: 0 };
  const via = await pressRowAction(row.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing" && (b.target ?? b.text).includes(row.label.slice(0, 12)), 20000);
  return { row: row as HistoryRow | null, via, band: settled.value, waitedMs: settled.waitedMs };
};

/** 🌄️ Step 1 — the board is parsed and populated, and the history panel speaks the locale under test. */
const step1 = async (ctx: Ctx) => {
  const parsed = await waitUntil(vitals, (v) => v?.parsed === "true", 60000);
  verdict("fixture-parsed", parsed.ok, { parsed: parsed.value?.parsed, waitedMs: parsed.waitedMs });
  const populated = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
  const p = await positions();
  verdict("board-has-nodes", (populated.value?.nodes ?? 0) > 0 && Object.keys(p).length > 0, { nodes: populated.value?.nodes, edges: populated.value?.edges, positions: Object.keys(p).length, camera: populated.value?.camera });
  const opened = await openHistory();
  const commands = await presentKey("framework.history.commands");
  const commandsText = await textOfKey("framework.history.commands");
  verdict("history-panel-speaks-the-locale", opened && commandsText.toLowerCase().includes(COPY[currentLocale].commands.toLowerCase()), { opened, commands, commandsText: commandsText.slice(0, 80), expected: COPY[currentLocale].commands, lang: await evalSafe(() => navigator.language, "") });
  emit({ kind: "inventory", tag: "boot-history", rows: await historyInventory() });
  await shot("boot");
  await closePanels();
  if (folderAt === 1) {
    ctx.folder = join(OUT, `folder-${stamp}-${currentLocale}`);
    mkdirSync(ctx.folder, { recursive: true });
    const before = await positions();
    const attached = await attachFolder(ctx.folder);
    const after = await positions();
    verdict("local-folder-attach-keeps-the-document", attached.typed === ctx.folder && attached.attachButton && movedIds(before, after, 1e-6).length === 0, { attached, drift: movedIds(before, after, 1e-6).length, reading: "the document is bound to a local folder (sync card → Folder → Attach, `persistedLocalOnly`) before any edit, so every later edit is persisted; step 5's reload check re-attaches it" });
    if (renderer === "wgpu" && !attached.card) {
      ctx.noFolder = true;
      note("wgpu-browser-build-offers-no-folder", { attached, reading: "S2-W2C decision (`📓️w2-c-report.md` S2.1): the browser wgpu build serves no folder transport, so it never attaches, remembers or offers a folder; the reload check then measures a plain reload" });
    }
    await closePanels();
  }
};

/** ✋️ Step 2 — select two nodes, drag them by (+80,+40) world units, one new row; then a downstream drag of a third node. */
const step2 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const { camera, picked } = await pickNodes(3, [[80, 40], [-60, 30]], [], 50);
  if (picked.length < 3) {
    verdict("three-clickable-nodes", false, { picked: picked.map((row) => row.id), camera });
    return;
  }
  const [a, b, c] = picked;
  ctx.a = a.id;
  ctx.b = b.id;
  ctx.c = c.id;
  log(`picked a=${a.id}@${JSON.stringify(a.at)} b=${b.id}@${JSON.stringify(b.at)} c=${c.id}@${JSON.stringify(c.at)} zoom=${camera.zoom}`);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await page.mouse.click(a.at.x, a.at.y);
  const first = await waitUntil(vitals, (v) => selectionIds(v).includes(a.id), 15000);
  await page.keyboard.down("Shift");
  await page.mouse.click(b.at.x, b.at.y);
  await page.keyboard.up("Shift");
  const both = await waitUntil(vitals, (v) => selectionIds(v).includes(a.id) && selectionIds(v).includes(b.id), 15000);
  verdict("two-nodes-selected", first.ok && both.ok && selectionIds(both.value).length === 2, { selection: selectionIds(both.value), transform: both.value?.transform?.slice(0, 200) });
  ctx.p0 = await positions();
  const zoom = cameraOf(await vitals()).zoom;
  await dragBy(a.at, 80 * zoom, 40 * zoom);
  const moved = await waitUntil(positions, (p) => offsetOf(ctx.p0!, p, a.id)?.some((v) => Math.abs(v) > 0.5) === true && offsetOf(ctx.p0!, p, b.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  ctx.p1 = moved.value;
  const offA = offsetOf(ctx.p0, ctx.p1, a.id);
  const offB = offsetOf(ctx.p0, ctx.p1, b.id);
  ctx.dx = offA?.[0] ?? 0;
  ctx.dy = offA?.[1] ?? 0;
  const others = movedIds(ctx.p0, ctx.p1).filter((id) => id !== a.id && id !== b.id);
  verdict("both-nodes-moved-by-one-common-offset", moved.ok && Boolean(offA && offB && near(offA[0], offB[0], 1e-6) && near(offA[1], offB[1], 1e-6)), { offA, offB, zoom, waitedMs: moved.waitedMs });
  verdict("offset-is-80-40-world-units", Boolean(offA && near(offA[0], 80, 0.005) && near(offA[1], 40, 0.005)), { offA, screenDelta: [80 * zoom, 40 * zoom], note: "screen delta = world offset × zoom; a pixel-quantised pointer makes this inexact" });
  verdict("no-other-node-moved", others.length === 0, { others: others.slice(0, 8) });
  const expected = dragLabel(2, ctx.dx, ctx.dy);
  const grew = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 1, 30000, 1000);
  const added = grew.value;
  verdict("exactly-one-new-history-row", added.length === 1, { added: added.map((row) => `${row.id}=${row.label}`), waitedMs: grew.waitedMs });
  const row = added.find((entry) => entry.label.startsWith(expected) || entry.text.includes(expected)) ?? added[0];
  verdict("row-labelled-from-the-drag-mutation", Boolean(row && (row.label === expected || row.label.startsWith(expected))), { expected, label: row?.label, text: row?.text?.slice(0, 160) });
  if (row) {
    ctx.dragEntry = row.id;
    const expanded = await expandEntry(row.id);
    const mutation = expanded.mutations.find((entry) => entry.label.startsWith(expected)) ?? expanded.mutations[0];
    ctx.dragMutation = mutation?.key;
    verdict("row-expands-to-its-drag-selection-mutation", Boolean(mutation && mutation.label.startsWith(expected)), { state: expanded.state, mutations: expanded.mutations.map((entry) => `${entry.key}=${entry.label}`).slice(0, 8) });
  }
  await shot("dragged");
  await closePanels();
  const beforeC = await positions();
  const cAt = toScreen(beforeC[c.id], cameraOf(await vitals()), await paneBox());
  await dragBy(cAt, -60 * zoom, 30 * zoom);
  const cMoved = await waitUntil(positions, (p) => offsetOf(beforeC, p, c.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  ctx.pc = cMoved.value;
  ctx.cOffset = offsetOf(beforeC, ctx.pc, c.id) ?? [0, 0];
  const cOthers = movedIds(beforeC, ctx.pc).filter((id) => id !== c.id);
  verdict("downstream-drag-moves-only-c", cMoved.ok && cOthers.length === 0, { cOffset: ctx.cOffset, others: cOthers.slice(0, 6) });
  const cRows = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((entry) => Number(entry.key) > newestBefore && entry.id !== ctx.dragEntry), (rows) => rows.length >= 1, 30000, 1000);
  const cRow = cRows.value[0];
  if (cRow) {
    const expanded = await expandEntry(cRow.id);
    ctx.cMutation = expanded.mutations[0]?.key;
    verdict("downstream-drag-is-its-own-row", cRows.value.length === 1 && cRow.label.startsWith(dragLabel(1, ctx.cOffset[0], ctx.cOffset[1])), { label: cRow.label, expected: dragLabel(1, ctx.cOffset[0], ctx.cOffset[1]), mutation: ctx.cMutation });
  } else verdict("downstream-drag-is-its-own-row", false, { rows: cRows.value.length });
  await closePanels();
};

/** 🎬️ Step 3 — Edit the drag: band `editing`, preview = state before + draft, downstream pending, dx/dy steppers and targets. */
const step3 = async (ctx: Ctx) => {
  if (!ctx.dragEntry || !ctx.dragMutation || !ctx.p0 || !ctx.a || !ctx.b || !ctx.c || !ctx.pc) {
    verdict("precondition-step-2", false, { ctx: Object.keys(ctx) });
    return;
  }
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  const b = begun.band;
  verdict("edit-opens-the-band-in-editing", b?.stage === "editing", { via: begun.via, stage: b?.stage, waitedMs: begun.waitedMs, text: b?.text?.slice(0, 200) });
  verdict("band-is-a-polite-status-region", b?.role === "status" && b?.live === "polite", { role: b?.role, live: b?.live });
  verdict("band-names-stage-and-target", Boolean(b && b.text.includes(COPY[currentLocale].stageEditing) && (b.target ?? "").includes(dragLabel(2, ctx.dx!, ctx.dy!))), { text: b?.text?.slice(0, 200), target: b?.target });
  verdict("band-offers-accept-discard-exit", JSON.stringify(b?.controls.map((c) => c.control)) === JSON.stringify(["accept", "discard", "exit"]), { controls: b?.controls });
  const focus = await waitUntil(focusRead, focusIsOnAnEditorInput, 6000, 150);
  verdict("focus-moves-to-the-editor", focus.ok, { focus: focus.value, waitedMs: focus.waitedMs, reading: "design §16 / G13: a draft that starts puts keyboard focus on its first input (`timeTravelTransitionV1` → `editor`; React `timeTravelFocusElementV1`, wgpu `resolve_time_travel_focus`), never on its row" });
  const revealed = await waitUntil(editorRevealed, (state) => state.revealed, 6000, 200);
  const tabs = await openPanelTabIds();
  verdict("history-panel-reveals-on-session-start", tabs.includes(HISTORY_TAB) && revealed.ok, { tabs, revealed: revealed.value, waitedMs: revealed.waitedMs, reading: "the edge into a session opens the History panel with the band and the editor's first input in view (no probe scroll; React: inside the viewport and uncovered, wgpu: projected). In one tab a session can only begin from that panel's own Edit row action — a closed-panel begin needs the agent/MCP gateway, which this serve does not run" });
  const indicator = await indicators();
  verdict("windows-wear-the-time-travel-indicator", indicator.length > 0, { indicator: indicator.slice(0, 4) });
  const roster = await presenceText();
  verdict("presence-roster-shows-no-editing-peer", roster === null || !/⏪|is editing|bearbeitet .* in der Zeitreise/.test(roster), { roster, reading: "one tab, no peer: the ⏪ badge and the \"is editing\" notes (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers`) must stay absent; a live second peer needs a presence transport (hub), which this serve does not run" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], ctx.dx!, ctx.dy!) && placed(p, ctx.c!, ctx.p0![ctx.c!], 0, 0), 15000);
  verdict("preview-is-state-before-target-plus-draft", placed(preview.value, ctx.a, ctx.p0[ctx.a], ctx.dx!, ctx.dy!) && placed(preview.value, ctx.b, ctx.p0[ctx.b], ctx.dx!, ctx.dy!), {
    a: preview.value[ctx.a],
    aBefore: ctx.p0[ctx.a],
    draft: [ctx.dx, ctx.dy],
    design: "§4: the preview is the document as of the edited mutation with the DRAFT applied (Begin drafts the original input), so the dragged nodes read pre-drag + draft",
  });
  verdict("downstream-not-applied-while-editing", placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { c: preview.value[ctx.c], cBeforeItsDrag: ctx.p0[ctx.c], cHead: ctx.pc[ctx.c], waitedMs: preview.waitedMs });
  const pendingRows = await waitUntil(readHistory, (rows) => rows.some((row) => row.kind === "mutation" && row.key === ctx.cMutation && row.text.includes(COPY[currentLocale].pending)), 10000);
  const cRow = pendingRows.value.find((row) => row.kind === "mutation" && row.key === ctx.cMutation);
  verdict("downstream-row-reads-not-applied", Boolean(cRow?.text.includes(COPY[currentLocale].pending)), { cRow: cRow?.text?.slice(0, 160), expected: COPY[currentLocale].pending });
  const e = await waitUntil(readEditor, (value) => Boolean(value?.dx && value?.dy && value?.targets), 15000);
  const ed = e.value;
  const steppers = renderer === "wgpu" ? Boolean(ed?.dx?.stepper && ed?.dy?.stepper) : Boolean(ed?.dx?.stepper && ed?.dy?.stepper && ed.dx.step === "1" && ed.dy.step === "1" && ed.dx.plus && ed.dx.minus);
  verdict("editor-shows-dx-dy-steppers-with-grid-snap-step", steppers, { dx: ed?.dx, dy: ed?.dy, note: renderer === "wgpu" ? "the mirror announces a spinbutton; its snap step is proven by ArrowUp/ArrowDown in step 4" : "snapSource {config: gridFactor} resolves to the grid factor (default 1) as the stepper step" });
  verdict("editor-dx-dy-read-the-original-input", Boolean(ed?.dx && ed?.dy && near(Number(ed.dx.value), ctx.dx!, 0.005) && near(Number(ed.dy.value), ctx.dy!, 0.005)), { dx: ed?.dx?.value, dy: ed?.dy?.value, expected: [ctx.dx, ctx.dy] });
  verdict("editor-shows-the-targets-reference-list", Boolean(ed?.targets && ed.targets.chips.length === 2 && ed.targets.chips.some((chip) => chip.includes(ctx.a!)) && ed.targets.chips.some((chip) => chip.includes(ctx.b!)) && ed.targets.useSelection), { targets: ed?.targets, inputs: ed?.inputs });
  await ariaVerdict("aria-band-editor-and-history-have-no-structural-findings", "session");
  emit({ kind: "inventory", tag: "editing", rows: await historyInventory(), editor: ed, band: b });
  await shot("editing");
};

/** 🎼️ Step 4 — dx → 120 through the stepper (keyboard), Accept, replay, review `ready`, head at +120 with downstream. */
const step4 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || (await band())?.stage !== "editing") {
    verdict("precondition-editing-session", false, { band: (await band())?.stage ?? null });
    return;
  }
  const typed = await typeEditorNumber("dx", 120);
  verdict("dx-stepper-control-reachable", typed.present && typed.typed === "120", { typed, authored: "framework.history.editor.input.dx" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!) && placed(p, ctx.b!, ctx.p0![ctx.b!], 120, ctx.dy!), 20000);
  verdict("dx-120-updates-the-preview", preview.ok && placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { typed, a: preview.value[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dy!], c: preview.value[ctx.c], waitedMs: preview.waitedMs });
  if (typed.present) {
    await keyEditorNumber("dx", "ArrowUp");
    const up = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 121, ctx.dy!), 15000);
    await keyEditorNumber("dx", "ArrowDown");
    const down = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!), 15000);
    verdict("arrow-keys-step-dx-by-the-snap-step", up.ok && down.ok, { up: up.value[ctx.a], down: down.value[ctx.a], dxField: (await editor())?.dx?.value });
  }
  const accepted = (await editor())?.dx?.value ?? null;
  await clearBandTrace();
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  const trace = await bandTrace();
  const stages = [...new Set(trace.map((row) => row.key.split("|")[0]))];
  const progress = trace.filter((row) => row.key.split("|")[2]);
  verdict("accept-replays-then-reviews-ready", review.value?.stage === "reviewing" && review.value.review === "ready", { via, dxField: accepted, band: review.value?.text?.slice(0, 200), review: review.value?.review, waitedMs: review.waitedMs });
  const bandFocus = await waitUntil(focusRead, (focus) => focus.isBand, 6000, 150);
  verdict("focus-moves-to-the-band-on-review", bandFocus.ok, { focus: bandFocus.value, waitedMs: bandFocus.waitedMs, reading: "G13: a replay or a review puts focus on the band (React `[data-semio-time-travel]` tabIndex −1, wgpu `shell.time-travel.status`)" });
  if (stages.includes("replaying") || progress.length) verdict("replay-stage-rendered", true, { trace: trace.slice(0, 20) });
  else note("replay-stage-not-rendered", { trace: trace.slice(0, 20), reading: "the two-mutation replay finished inside one reactor turn, so the band went editing → reviewing without a replaying frame (progress is throttled ≥ 100 ms / ≥ 5 %)" });
  verdict("review-line-reads-ready", Boolean(review.value?.text.includes(COPY[currentLocale].reviewReady)), { text: review.value?.text?.slice(0, 200) });
  const finalize = review.value?.controls.find((c) => c.control === "finalize");
  verdict("finalize-enabled-when-ready", finalize?.disabled === false, { controls: review.value?.controls });
  const head = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!) && placed(p, ctx.c!, ctx.pc![ctx.c!], 0, 0), 15000);
  verdict("reviewed-head-at-plus-120-with-downstream-reapplied", head.ok && placed(head.value, ctx.b, ctx.p0[ctx.b], 120, ctx.dy!), { a: head.value[ctx.a], b: head.value[ctx.b], c: head.value[ctx.c], cHead: ctx.pc?.[ctx.c] });
  await shot("reviewing");
};

/** 🏁️ Step 5 — Finalize → dialog → Overwrite → band gone, overwrite row, head persists across a reload. */
const step5 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || (await band())?.review !== "ready") {
    verdict("precondition-ready-review", false, { band: await band() });
    return;
  }
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  const d = dialog.value;
  const copy = COPY[currentLocale];
  verdict("finalize-opens-the-dialog", Boolean(d && d.title.includes(copy.dialogTitle)), { title: d?.title, waitedMs: dialog.waitedMs });
  const promptFocus = await waitUntil(focusRead, (focus) => focus.inDialog, 6000, 150);
  verdict("focus-moves-into-the-finalize-prompt", promptFocus.ok, { focus: promptFocus.value, waitedMs: promptFocus.waitedMs, reading: "G13: choosing puts focus on the prompt's first control (its Alternative name field)" });
  verdict("dialog-offers-destructive-overwrite", Boolean(d?.overwrite && d.overwrite.text.includes(copy.overwrite) && (renderer === "wgpu" || d.overwrite.destructive === "true") && (d.overwrite.description ?? "").includes(copy.overwriteDescription)), { overwrite: d?.overwrite, expectedDescription: copy.overwriteDescription, reading: renderer === "wgpu" ? "the mirror carries no tone; the destructive choice is told by its description" : "data-destructive + the choice description" });
  verdict("dialog-offers-new-alternative-with-a-name-field", Boolean(d?.submit && d.submit.text.includes(copy.newAlternative) && d.name && d.name.value === copy.defaultName), { submit: d?.submit, name: d?.name });
  if (renderer === "wgpu") note("band-while-the-dialog-is-open", { band: (await band())?.stage ?? null, reading: "the wgpu chrome projects only the modal dialog's nodes while it is open (`chrome_accessibility_nodes`), so the band's status is not announced then" });
  else verdict("band-reads-choosing-while-the-dialog-is-open", (await band())?.stage === "choosing", { band: (await band())?.stage });
  await ariaVerdict("aria-finalize-prompt-has-no-structural-findings", "prompt");
  await shot("dialog");
  await dialogChoose("overwrite");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict("overwrite-closes-the-session", gone.ok && (await finalizeDialog()) === null, { waitedMs: gone.waitedMs, band: gone.value?.stage ?? null });
  const expected = copy.row("edit", null, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict("overwrite-row-appears", rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  ctx.afterOverwrite = await positions();
  verdict("head-keeps-plus-120-after-overwrite", placed(ctx.afterOverwrite, ctx.a, ctx.p0[ctx.a], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.b, ctx.p0[ctx.b], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.c, ctx.pc![ctx.c], 0, 0), { a: ctx.afterOverwrite[ctx.a], c: ctx.afterOverwrite[ctx.c] });
  await shot("overwritten");
  note("reload-check-deferred", { reason: "the reload is taken after step 8 and before step 9 (attributed to step 5), so steps 6–8 keep the document it would otherwise replace and step 9 digests its console" });
};

/** 📂️ Picks the sync card's Folder kind: the sync utilities render as one grouped toggle (`ui.utilities.group.sync`)
 * whose menu lists File / Folder / Remote. */
const pickSyncFolder = async () => {
  if (renderer === "wgpu") {
    if (!mirrorFind(await mirror(), "framework.sync.folder")) await wgpuPress("ui.utilities.group.sync");
    await sleep(700);
    return (await wgpuPress("framework.sync.folder")) ?? "absent";
  }
  const direct = page.locator('[id="framework.sync.folder"]').first();
  if (!(await direct.count().catch(() => 0))) {
    await page.locator('[id="ui.utilities.group.sync"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(700);
  }
  if (await direct.count().catch(() => 0)) {
    await direct.click({ timeout: 4000 }).catch(() => {});
    return "id";
  }
  const item = page.locator('[role="menuitem"], [role="menuitemradio"], [role="option"], button').filter({ hasText: /^\s*(folder|ordner)\s*$/i }).first();
  if (await item.count().catch(() => 0)) {
    await item.click({ timeout: 4000 }).catch(() => {});
    return "text";
  }
  return "absent";
};

/** 🗂️ Attaches the session document to the local folder `path` through the sync chip (`s-sync-status` →
 * `framework.sync.folder` → `framework.sync.folder.path` → Attach): `openSyncTarget` opens it with a
 * `persistedLocalOnly` folder binding the backbone worker reads and writes through the dev serve's `/semio-backbone`. */
const attachFolder = async (path: string) => {
  if (renderer === "wgpu") {
    if (!(await openPanelTabIds()).includes("s-sync-status")) await wgpuPress("s-sync-status");
    await sleep(600);
    await pickSyncFolder();
    const shown = await waitUntil(async () => mirrorFind(await mirror(), "framework.sync.folder.path") ?? null, (node) => node !== null, 8000);
    const field = shown.value ? await mirrorLocator(shown.value.key, shown.value.window) : null;
    await field?.fill(path, { force: true, timeout: 4000 }).catch(() => {});
    const typed = field ? await field.inputValue({ timeout: 2000 }).catch(() => null) : null;
    await sleep(400);
    const attached = await wgpuPress("framework.sync.attach");
    await sleep(1500);
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    return { card: shown.ok, typed, attachButton: attached !== null };
  }
  const chip = page.locator('[id="s-sync-status"]').first();
  if (!(await openPanelTabIds()).includes("s-sync-status")) await chip.click({ timeout: 4000 }).catch(() => {});
  await sleep(600);
  await pickSyncFolder();
  const field = page.locator('[id="framework.sync.folder.path"]').first();
  const shown = await waitUntil(() => field.count().catch(() => 0), (count) => count > 0, 8000);
  await field.fill(path, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  const attach = page.locator("button").filter({ hasText: /^\s*(attach|verbinden)\s*$/i }).first();
  const attachCount = await attach.count().catch(() => 0);
  if (attachCount) await attach.click({ timeout: 4000 }).catch(() => {});
  else await field.press("Enter").catch(() => {});
  await sleep(1500);
  await page.keyboard.press("Escape").catch(() => {});
  return { card: shown.ok, typed, attachButton: attachCount > 0 };
};

/** 🔁️ Step 5's reload half, taken after step 8. A `?plugin=` playground opens no space, so
 * `resolveDocumentOpeningBindings` (`🏛️ShellHost/🧭️opening/🟦️.ts`) answers no binding and a bare reload re-runs Set
 * Active Example; the one local-only route is the sync card's folder attach, remembered per device in
 * `os.config.local-folders` by the document's store id. The folder is attached at boot (`--folder-at=1`) or here with one
 * edit to trigger the write (`--folder-at=5`); then: see the archive on disk, reload, take the reconnect band's offer
 * (`🏛️ShellHost/📁️local-folders`, fallback: attach again by hand), compare head, rows and edit ids with the ones before the
 * reload, and finally detach and reload once more — a forgotten folder must not be offered. */
const reloadCheck = async (ctx: Ctx) => {
  if (!ctx.afterOverwrite && !ctx.g4) return;
  if (ctx.noFolder) return plainReloadCheck(ctx);
  const early = ctx.folder !== undefined;
  const folder = ctx.folder ?? join(OUT, `folder-${stamp}-${currentLocale}`);
  mkdirSync(folder, { recursive: true });
  const beforeAttach = await positions();
  await closePanels();
  const attached = early ? { card: true, typed: folder, attachButton: true } : await attachFolder(folder);
  const afterAttach = await positions();
  if (!early) verdict("folder-attach-keeps-the-head", movedIds(beforeAttach, afterAttach, 1e-6).length === 0, { attached, drift: movedIds(beforeAttach, afterAttach, 1e-6).slice(0, 6) });
  const files = () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => `${entry.parentPath ?? ""}/${entry.name}`.replace(folder, ""));
  if (!early && ctx.c && afterAttach[ctx.c]) {
    await closePanels();
    const zoom = cameraOf(await vitals()).zoom;
    await dragBy(toScreen(afterAttach[ctx.c], cameraOf(await vitals()), await paneBox()), 10 * zoom, 10 * zoom);
    await waitUntil(positions, (p) => offsetOf(afterAttach, p, ctx.c!)?.some((v) => Math.abs(v) > 0.5) === true, 15000);
  }
  const written = await waitUntil(async () => files(), (list) => list.length > 0, 30000, 1000);
  verdict("folder-attach-writes-the-document-archive", attached.typed === folder && written.ok, { files: written.value.slice(0, 8), folder, waitedMs: written.waitedMs, reading: "the archive is written on the bound document's outbound mutations (`archivePersistence` in `bindDocumentBackbone.send`), so one edit follows the attach" });
  const before = await positions();
  const historyBefore = await allHistoryRows();
  const rowsBefore = documentEntries(historyBefore).map((row) => row.label.slice(0, 60));
  const editsBefore = await documentEditIds(historyBefore);
  emit({ kind: "rows", tag: "before-reload", editIds: editsBefore, rows: historyBefore.filter((row) => row.kind === "entry").map((row) => ({ key: row.key, label: row.label, text: row.text.slice(0, 160), expandable: row.expandable })) });
  const warned = historyBefore.find((row) => row.kind === "entry" && /Set Active Example|Beispiel/i.test(row.label) && /Warning|Warnung/.test(row.text));
  if (warned) {
    const expanded = await expandEntry(warned.id);
    note("example-row-warning-after-attach", { label: warned.text.slice(0, 120), flagged: expanded.mutations.filter((row) => /Warning|Warnung/.test(row.text)).slice(0, 8).map((row) => row.text.slice(0, 200)), shown: expanded.mutations.length });
  }
  const alternativesBefore = await readAlternatives();
  const currentBefore = alternativesBefore.find(isCurrentAlternative);
  const g4Before = ctx.g4 ? (await findMutationRow(ctx.g4.drag))?.text ?? null : null;
  emit({ kind: "alternatives", tag: "before-reload", rows: alternativesBefore, current: currentBefore?.key ?? null, g4: g4Before });
  await closePanels();
  await harvestNotices();
  expectedReloads += 1;
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebooted = await waitForBoot("reload", 60);
  await installBandTrace();
  await installNoticeTrace();
  const bare = await positions();
  note("bare-reload-reruns-the-example", { driftCount: movedIds(before, bare, 1e-6).length, route: await evalSafe(() => location.search, "") });
  const example = (await allHistoryRows()).filter((row) => row.kind === "entry" && /Set Active Example|Beispiel/i.test(row.label));
  if (example[0]) {
    const expanded = await expandEntry(example[0].id);
    note("post-reload-example-row", { label: example[0].label.slice(0, 120), text: example[0].text.slice(0, 200), flagged: expanded.mutations.filter((row) => /Warning|Warnung|Error|Fehler/.test(row.text)).slice(0, 6).map((row) => row.text.slice(0, 160)) });
  }
  await closePanels();
  const folderName = folder.split("/").pop() ?? folder;
  const offer = await waitUntil(reconnectBand, (state) => state?.state === "offered", 20000, 500);
  const offered = offer.value;
  verdict("folder-reconnect-offered", Boolean(offered && offered.state === "offered" && offered.role === "status" && offered.live === "polite" && offered.message.includes(folderName) && offered.reconnect && offered.forget), { band: offered, folderName, waitedMs: offer.waitedMs });
  let reattached: Record<string, unknown>;
  if (offered?.reconnect) {
    const via = renderer === "wgpu" ? await wgpuPress("s-folder-reconnect") : await page.locator("#s-folder-reconnect").click({ timeout: 4000 }).then(() => "button").catch(() => null);
    const busy = await waitUntil(reconnectBand, (state) => state === null || state.state === "reconnecting", 10000, 100);
    const settled = await waitUntil(reconnectBand, (state) => state === null, 60000, 500);
    reattached = { via, sawBusy: busy.value?.state === "reconnecting", bandGone: settled.ok, waitedMs: settled.waitedMs };
    verdict("folder-reconnect-attaches-and-closes-the-band", Boolean(via) && settled.ok, reattached);
  } else {
    reattached = { fallback: await attachFolder(folder) };
    note("folder-reconnect-fallback-manual-attach", reattached);
  }
  const restored = await waitUntil(positions, (p) => movedIds(before, p, 1e-6).length === 0, 60000, 1000);
  const drift = movedIds(before, restored.value, 1e-6);
  verdict("positions-persist-after-reload", rebooted && restored.ok, { rebooted, reattached, a: ctx.a ? restored.value[ctx.a] : null, expected: ctx.a ? before[ctx.a] : null, driftCount: drift.length, drift: drift.slice(0, 6), folder });
  note("folder-backbone-requests", { requests: consoleRows.filter((row) => row.locale === currentLocale && row.type === "backbone").slice(-30).map((row) => `${row.t}s ${row.text}`), sessions: await evalSafe(() => Array.from(document.querySelectorAll("[data-surface-id]")).map((el) => el.getAttribute("data-surface-id")).slice(0, 3), [] as (string | null)[]) });
  const hydrated = await waitUntil(allHistoryRows, (all) => documentEntries(all).length >= rowsBefore.length, 20000, 1000);
  const historyAfter = hydrated.value;
  const editsAfter = await documentEditIds(historyAfter);
  emit({ kind: "rows", tag: "after-reload-reattach", editIds: editsAfter, rows: historyAfter.filter((row) => row.kind === "entry").map((row) => ({ key: row.key, label: row.label, text: row.text.slice(0, 160), expandable: row.expandable })) });
  verdict("edit-ids-survive-the-reload", editsBefore.length > 0 && editsBefore.every((id) => editsAfter.includes(id)), { before: editsBefore.length, after: editsAfter.length, lost: editsBefore.filter((id) => !editsAfter.includes(id)).slice(0, 12) });
  const labels = documentEntries(historyAfter).map((row) => row.label.slice(0, 60));
  const lost = rowsBefore.filter((label) => !labels.includes(label));
  verdict("document-rows-survive-the-reload", rowsBefore.length > 0 && lost.length === 0 && labels.length >= rowsBefore.length, { before: rowsBefore.slice(0, 24), after: labels.slice(0, 24), lost: lost.slice(0, 16), counts: [rowsBefore.length, labels.length], waitedMs: hydrated.waitedMs, reading: "G5: every pre-reload row comes back with its localized label (rows whose command emits no description included: `Edit.verb`)" });
  const opText = labels.filter(readsAsOpText);
  verdict("no-row-reads-op-text-after-reload", opText.length === 0, { opText: opText.slice(0, 8), reading: "G5: no row falls back to the locale-invariant op text (`create-node node {…}`)" });
  const overwrite = COPY[currentLocale].row("edit", null, 1);
  if (rowsBefore.some((label) => label.startsWith(overwrite))) verdict("overwrite-row-survives-the-reload", labels.some((label) => label.startsWith(overwrite)), { labels: labels.slice(0, 10) });
  else note("overwrite-row-absent-before-the-reload", { before: rowsBefore.slice(0, 12), reading: "the overwrite row was no longer in the document when the reload check ran (a dev reload reset it earlier)" });
  const alternativesAfter = await readAlternatives();
  const currentAfter = alternativesAfter.find(isCurrentAlternative);
  emit({ kind: "alternatives", tag: "after-reload-reattach", rows: alternativesAfter, current: currentAfter?.key ?? null });
  const missingAlternatives = alternativesBefore.filter((row) => !alternativesAfter.some((other) => other.key === row.key));
  verdict("alternatives-survive-the-reload", alternativesBefore.length > 0 && missingAlternatives.length === 0, { before: alternativesBefore.map((row) => `${row.key}=${row.text.slice(0, 50)}`), after: alternativesAfter.map((row) => `${row.key}=${row.text.slice(0, 50)}`), missing: missingAlternatives.map((row) => row.key) });
  verdict("main-line-listed-after-the-reload", alternativesAfter.some((row) => row.text.includes(COPY[currentLocale].trunk)), { after: alternativesAfter.map((row) => row.text.slice(0, 50)), expected: COPY[currentLocale].trunk });
  verdict("current-alternative-restored-after-the-reload", Boolean(currentBefore && currentAfter && currentAfter.key === currentBefore.key), { before: currentBefore?.key ?? null, after: currentAfter?.key ?? null });
  if (ctx.g4) {
    const g4After = (await findMutationRow(ctx.g4.drag))?.text ?? null;
    atStep(13, () => verdict("warning-row-survives-the-reload", Boolean(g4After?.includes(COPY[currentLocale].warningPartial)), { before: g4Before, after: g4After, expected: COPY[currentLocale].warningPartial, reading: "G4: the warning an edit introduced persists in history through finalize and a folder reload + re-attach" }));
  }
  await shot("reloaded");
  await closePanels();
  const detached = await detachFolder();
  await harvestNotices();
  expectedReloads += 1;
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebootedAgain = await waitForBoot("reload-after-detach", 60);
  await installBandTrace();
  await installNoticeTrace();
  const stray = await waitUntil(reconnectBand, (state) => state !== null, 15000, 500);
  verdict("a-forgotten-folder-is-not-offered-after-reload", rebootedAgain && !stray.ok, { detached, band: stray.value, waitedMs: stray.waitedMs, reading: "Detach on the sync card forgets `os.config.local-folders`' binding, so the next load offers nothing" });
};

/** 🔂️ The reload half on a build that offers no folder route (the browser wgpu shell, note
 * `wgpu-browser-build-offers-no-folder`): one plain reload, then whether the head and the rows persisted. */
const plainReloadCheck = async (_ctx: Ctx) => {
  const before = await positions();
  const rowsBefore = documentEntries(await allHistoryRows()).map((row) => row.label.slice(0, 60));
  await closePanels();
  await harvestNotices();
  expectedReloads += 1;
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebooted = await waitForBoot("reload", 60);
  await installBandTrace();
  await installNoticeTrace();
  const after = await waitUntil(positions, (p) => Object.keys(p).length > 0 && movedIds(before, p, 1e-6).length === 0, 30000, 1000);
  const drift = movedIds(before, after.value, 1e-6);
  const rowsAfter = documentEntries(await allHistoryRows()).map((row) => row.label.slice(0, 60));
  verdict("reload-restores-the-edited-document", rebooted && drift.length === 0 && rowsBefore.every((label) => rowsAfter.includes(label)), { rebooted, driftCount: drift.length, drift: drift.slice(0, 6), rowsBefore: rowsBefore.slice(0, 16), rowsAfter: rowsAfter.slice(0, 16), reading: "no folder route on this build: G5 (reload persistence) needs a persisted-local route for the browser wgpu shell (owner S2-W2C)" });
  await shot("reloaded");
  await closePanels();
};

type ReconnectBand = { state: string; role: string | null; live: string | null; label: string | null; message: string; reconnect: boolean; forget: boolean };

/** 📁️ The folder-reconnect band a reload offers for a document bound to a local folder (`🏛️ShellHost/📁️local-folders`):
 * `role=status`, polite, `data-semio-folder-reconnect=offered|reconnecting`, its message and the Reconnect / Forget buttons
 * (wgpu: the same ids through the mirror). */
const reconnectBand = async (): Promise<ReconnectBand | null> => {
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const reconnect = nodes.find((node) => node.key === "s-folder-reconnect");
    if (!reconnect) return null;
    const message = nodes.find((node) => node.key === "s-folder-reconnect-message");
    return { state: reconnect.disabled ? "reconnecting" : "offered", role: "status", live: "polite", label: null, message: message?.label ?? "", reconnect: true, forget: nodes.some((node) => node.key === "s-folder-forget") };
  }
  return evalSafe(() => {
    const el = document.querySelector("[data-semio-folder-reconnect]");
    if (!el) return null;
    return {
      state: el.getAttribute("data-semio-folder-reconnect") ?? "",
      role: el.getAttribute("role"),
      live: el.getAttribute("aria-live"),
      label: el.getAttribute("aria-label"),
      message: document.getElementById("s-folder-reconnect-message")?.textContent ?? "",
      reconnect: Boolean(document.getElementById("s-folder-reconnect")),
      forget: Boolean(document.getElementById("s-folder-forget")),
    };
  }, null as ReconnectBand | null);
};

/** ✂️ Detaches the document from its folder through the sync card (Detach / Trennen; wgpu `framework.sync.detach`). */
const detachFolder = async () => {
  if (renderer === "wgpu") {
    if (!(await openPanelTabIds()).includes("s-sync-status")) await wgpuPress("s-sync-status");
    await sleep(600);
    await pickSyncFolder();
    await sleep(600);
    const via = await wgpuPress("framework.sync.detach");
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    return via;
  }
  if (!(await openPanelTabIds()).includes("s-sync-status")) await page.locator('[id="s-sync-status"]').first().click({ timeout: 4000 }).catch(() => {});
  await sleep(600);
  await pickSyncFolder();
  const detach = page.locator("button").filter({ hasText: /^\s*(detach|trennen)\s*$/i }).first();
  const present = await detach.count().catch(() => 0);
  if (present) await detach.click({ timeout: 4000 }).catch(() => {});
  await sleep(1000);
  await page.keyboard.press("Escape").catch(() => {});
  return present ? "button" : null;
};

/** ↩️ Step 6 — Undo takes the finalize back (+80), Redo re-authors it (+120); chord first, then the panel button. */
const step6 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.afterOverwrite) {
    verdict("precondition-step-5", false, {});
    return;
  }
  const isAt = (dx: number) => async () => placed(await positions(), ctx.a!, ctx.p0![ctx.a!], dx, ctx.dy!);
  const attempt = async (verb: "undo" | "redo", target: number) => {
    const chord = verb === "undo" ? `${mod}+z` : `${mod}+Shift+z`;
    await prepareChord();
    await page.keyboard.press(chord).catch(() => {});
    const byChord = await waitUntil(isAt(target), (ok) => ok, 12000, 400);
    verdict(`${verb}-chord-${verb === "undo" ? "takes-back" : "re-applies"}-the-finalize`, byChord.ok, { chord, a: (await positions())[ctx.a!], expected: [ctx.p0![ctx.a!][0] + target, ctx.p0![ctx.a!][1] + ctx.dy!], waitedMs: byChord.waitedMs });
    if (byChord.ok) return 0;
    const presses: Record<string, unknown>[] = [];
    await openHistory();
    for (let press = 1; press <= 4; press++) {
      const before = await positions();
      const panelsBefore = await openPanelTabIds();
      if (!panelsBefore.includes(HISTORY_TAB)) await openHistory();
      const pressed = await pressAuthored(`framework.history.${verb}`);
      const settled = await waitUntil(isAt(target), (ok) => ok, 8000, 400);
      presses.push({ press, pressed, reached: settled.ok, moved: movedIds(before, await positions()).length, panels: await openPanelTabIds() });
      if (settled.ok) {
        verdict(`${verb}-button-reaches-the-finalize`, press === 1, { presses });
        return press;
      }
    }
    verdict(`${verb}-button-reaches-the-finalize`, false, { presses });
    return -1;
  };
  await closePanels();
  await attempt("undo", ctx.dx!);
  const undone = await positions();
  verdict("undo-restores-plus-80", placed(undone, ctx.a, ctx.p0[ctx.a], ctx.dx!, ctx.dy!) && placed(undone, ctx.b!, ctx.p0[ctx.b!], ctx.dx!, ctx.dy!), { a: undone[ctx.a], b: undone[ctx.b!] });
  await closePanels();
  await attempt("redo", 120);
  const redone = await positions();
  verdict("redo-restores-plus-120", placed(redone, ctx.a, ctx.p0[ctx.a], 120, ctx.dy!) && placed(redone, ctx.b!, ctx.p0[ctx.b!], 120, ctx.dy!), { a: redone[ctx.a] });
  const copy = COPY[currentLocale];
  const rows = await allHistoryRows();
  const labels = documentEntries(rows).map((row) => row.label);
  verdict("undone-and-redone-rows-appear", labels.some((label) => label.startsWith(copy.row("undo", null, 1))) && labels.some((label) => label.startsWith(copy.row("redo", null, 1))), { expected: [copy.row("undo", null, 1), copy.row("redo", null, 1)], labels: labels.slice(-8) });
  await shot("undo-redo");
  await closePanels();
};

/** 🌿️ Step 7 — a second session on dy → New alternative with a name → its row and head; then switch alternatives. */
const step7 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.dragEntry) {
    verdict("precondition-step-2", false, {});
    return;
  }
  const dy0 = ctx.dy!;
  ctx.dyEdited = Math.round(dy0 + 30);
  ctx.alternative = `probe ${currentLocale} ${stamp.slice(11, 19)}`;
  const first = await editDragAsAlternative(ctx, ctx.dyEdited, ctx.alternative, "");
  if (!first) return;
  await shot("alternative");
  const listed = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(ctx.alternative!)), 15000, 1000);
  verdict("alternatives-section-lists-the-new-alternative", listed.ok, { rows: listed.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${row.active ? " [active]" : ""}`), expectedSelector: "framework.history.alternatives / framework.history.alternative.<id>" });
  if (!listed.value.length) {
    verdict("alternative-switcher-present", false, { inventory: (await historyInventory()).filter((row) => /alternative/i.test(String(row.id))).slice(0, 12), note: "no `framework.history.alternative.<id>` rows in this build" });
    await closePanels();
    return;
  }
  const trunk = listed.value.find((row) => !row.text.includes(ctx.alternative!));
  const expectAt = (dy: number) => async () => placed(await positions(), ctx.a!, ctx.p0![ctx.a!], 120, dy) && placed(await positions(), ctx.b!, ctx.p0![ctx.b!], 120, dy);
  if (trunk) {
    const via = await switchAlternative(trunk);
    const back = await waitUntil(expectAt(dy0), (ok) => ok, 20000, 500);
    verdict("switching-to-the-trunk-shows-original-positions", back.ok, { via, trunk: trunk.text.slice(0, 60), a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + dy0] });
    const mine = (await readAlternatives()).find((row) => row.text.includes(ctx.alternative!));
    const forth = mine ? await switchAlternative(mine) : "absent";
    const edited = await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    verdict("switching-back-shows-the-edited-positions", edited.ok, { via: forth, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
  } else {
    verdict("trunk-listed-after-new-alternative", false, { rows: listed.value.map((row) => row.text.slice(0, 60)), pending: "W1-G: after finalize-as-new-alternative the original trunk is not yet listed — expected-pending" });
    const second = `${ctx.alternative} b`;
    const dy2 = Math.round(dy0 + 60);
    const made = await editDragAsAlternative(ctx, dy2, second, "-2");
    if (!made) return;
    const both = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(second)) && rows.some((row) => row.text.includes(ctx.alternative!) && !row.text.includes(second)), 15000, 1000);
    verdict("alternatives-section-lists-both-alternatives", both.ok, { rows: both.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${row.active ? " [active]" : ""}`) });
    const firstRow = both.value.find((row) => row.text.includes(ctx.alternative!) && !row.text.includes(second));
    const viaFirst = firstRow ? await switchAlternative(firstRow) : "absent";
    const atFirst = await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    verdict("switching-to-the-first-alternative-shows-its-positions", atFirst.ok, { via: viaFirst, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
    const secondRow = (await readAlternatives()).find((row) => row.text.includes(second));
    const viaSecond = secondRow ? await switchAlternative(secondRow) : "absent";
    const atSecond = await waitUntil(expectAt(dy2), (ok) => ok, 20000, 500);
    verdict("switching-to-the-second-alternative-shows-its-positions", atSecond.ok, { via: viaSecond, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + dy2] });
    const back = firstRow ? await switchAlternative((await readAlternatives()).find((row) => row.key === firstRow.key) ?? firstRow) : "absent";
    await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    note("left-on-the-first-alternative", { via: back });
  }
  await shot("switched");
  await closePanels();
};

type AlternativeRow = { id: string; key: string; text: string; active: boolean };

/** 🌲️ The Alternatives section's rows (`framework.history.alternative.<id>`), with whether each reads active. */
const readAlternatives = async (): Promise<AlternativeRow[]> => {
  await openHistory();
  for (const edge of ["start", "end"] as const) await scrollHistory(edge);
  if (renderer === "wgpu") return (await mirror()).filter((node) => !node.key.includes("::") && /(^|\/)framework\.history\.alternative\.[^/]+$/.test(node.key)).map((node) => ({ id: node.key, key: node.key.replace(/^.*framework\.history\.alternative\./, ""), text: `${node.label} ${node.description}`.trim(), active: node.selected === "true" || node.pressed === "true" }));
  return evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id]"))
        .filter((el) => /(^|\/)framework\.history\.alternative\.[^/]+$/.test(el.id))
        .map((el) => ({
          id: el.id,
          key: el.id.replace(/^.*framework\.history\.alternative\./, ""),
          text: (el as HTMLElement).innerText.replace(/\s+/g, " ").trim(),
          active: el.getAttribute("aria-selected") === "true" || el.getAttribute("aria-current") !== null || el.getAttribute("data-selected") === "true",
        })),
    [] as AlternativeRow[],
  );
};

/** 💡️ Whether an alternatives row is the current one: its selected state, or the "Current" word its description carries. */
const isCurrentAlternative = (row: AlternativeRow) => row.active || new RegExp(`\\b${COPY[currentLocale].current}\\b`).test(row.text);

/** 🔃️ Switches to one alternative through its Switch row action, else the row's own activation. */
const switchAlternative = async (row: AlternativeRow) => pressRowAction(row.id, /^(Switch|Switch to|Switch alternative|Wechseln|Umschalten|Alternative wechseln)$/i);

/** 🍀️ One history-edit session on the drag's dy through the real stepper → Accept → Finalize → New alternative `name`;
 * verdicts carry `suffix` so a second alternative reads apart from the first. Answers whether the session committed. */
const editDragAsAlternative = async (ctx: Ctx, dyValue: number, name: string, suffix: string) => {
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  verdict(`session-opens${suffix}`, begun.band?.stage === "editing", { via: begun.via, stage: begun.band?.stage });
  if (begun.band?.stage !== "editing") return false;
  const ed = (await waitUntil(readEditor, (value) => Boolean(value?.dx && value?.dy), 15000)).value;
  verdict(`editor-reads-the-effective-input${suffix}`, Boolean(ed?.dx && near(Number(ed.dx.value), 120, 0.005)), { dx: ed?.dx?.value, dy: ed?.dy?.value, note: "the drag's effective input in the current alternative carries the overwritten dx=120" });
  if (!ed?.dx) {
    dumpJson(`no-editor${suffix}`, { band: await band(), panels: await openPanelTabIds(), inventory: (await historyInventory()).filter((row) => !/entry\.|mutation\./.test(String(row.id))), rows: (await readHistory()).slice(0, 40) });
    await shot(`no-editor${suffix}`);
  }
  const typed = await typeEditorNumber("dy", dyValue);
  verdict(`dy-stepper-control-reachable${suffix}`, typed.present && typed.typed === String(dyValue), { typed, authored: "framework.history.editor.input.dy" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, dyValue), 20000);
  verdict(`dy-edit-updates-the-preview${suffix}`, preview.ok, { a: preview.value[ctx.a!], expected: [ctx.p0![ctx.a!][0] + 120, ctx.p0![ctx.a!][1] + dyValue] });
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict(`review-ready${suffix}`, review.value?.review === "ready", { via, review: review.value?.review, text: review.value?.text?.slice(0, 160) });
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  const nameValue = await dialogFillName(name);
  verdict(`name-field-takes-the-alternative-name${suffix}`, dialog.ok && nameValue === name, { nameValue, submit: dialog.value?.submit });
  await dialogChoose("submit");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict(`new-alternative-closes-the-session${suffix}`, gone.ok, { band: gone.value?.stage ?? null, fault: gone.value?.fault ?? null });
  const expected = COPY[currentLocale].row("edit", name, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict(`alternative-row-appears${suffix}`, rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(0, 6) });
  const head = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, dyValue), 15000);
  verdict(`head-is-on-the-edited-alternative${suffix}`, head.ok, { a: head.value[ctx.a!], expected: [ctx.p0![ctx.a!][0] + 120, ctx.p0![ctx.a!][1] + dyValue] });
  return gone.ok;
};

/** 🚧️ The hard-minimum law of a stepper (design §18, `time_travel_number_limits`) in the open `create-node` session: the
 * `/index` stepper (schema `minimum: 0`, an inclusive hard bound) refuses a typed `-1` visibly, naming the bound ("Must be at
 * least 0"), dispatches nothing (the preview keeps the clone where it was), and Home reaches the bound itself. */
const hardMinimumRefusal = async (clone: string, beforeDrag: Positions) => {
  const copy = COPY[currentLocale];
  const before = await readNumberControl("index");
  if (!before) {
    verdict("stepper-below-its-hard-minimum-is-refused-naming-the-bound", false, { reason: "no `framework.history.editor.input.index` stepper in the create-node editor", inputs: (await editor())?.inputs });
    return;
  }
  const typed = await typeEditorNumber("index", -1);
  await sleep(900);
  const refused = await readNumberControl("index");
  const refusal = refused?.refusal ?? (copy.atLeastZero.test(refused?.rowText ?? "") ? (refused?.rowText ?? null) : null);
  verdict("stepper-below-its-hard-minimum-is-refused-naming-the-bound", Boolean(refusal && copy.atLeastZero.test(refusal)), { typed, before: { valueNow: before.valueNow, min: before.min }, refusal, invalid: refused?.invalid, rowText: refused?.rowText, expected: String(copy.atLeastZero), reading: "create-node `/index` (x-semio-ui stepper, schema `minimum: 0`): an inclusive hard bound → `UiNumberLimits` refusal \"Must be at least 0\" / \"Muss mindestens 0 sein\", nothing dispatched" });
  const kept = await positions();
  verdict("stepper-refusal-keeps-the-draft", placed(kept, clone, beforeDrag[clone], 0, 0) && (renderer === "wgpu" || refused?.invalid === true), { clone: kept[clone], beforeDrag: beforeDrag[clone], invalid: refused?.invalid, valueAfter: refused?.valueNow });
  await keyEditorNumber("index", "Home");
  const home = await waitUntil(() => readNumberControl("index"), (control) => control?.valueNow === 0 && !control.invalid, 8000, 300);
  verdict("stepper-home-reaches-the-hard-minimum", home.ok, { valueNow: home.value?.valueNow, invalid: home.value?.invalid, refusal: home.value?.refusal, reading: "design §18: Home = the hard minimum" });
};

/** 💥️ Step 8 — duplicate a node, drag the clone, withdraw the clone's `create-node` → blocked review with an Error row,
 * Finalize disabled; Next problem → withdraw the drag → ready; Exit → zero trace. */
const step8 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const spot = await pickCloneSource(exclude);
  const d = spot?.row;
  if (!spot || !d) {
    verdict("clone-source-with-free-space", false, { reason: "no node whose clone (+24,+24) and drag destination are ≥ 55 world units from every other node" });
    return;
  }
  log(`clone source ${d.id} at ${JSON.stringify(d.world)} drag ${JSON.stringify(spot.move)} zoom=${spot.zoom}`);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(d.at.x, d.at.y);
  const selected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === d.id, 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  const grew = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  verdict("duplicate-adds-one-clone", selected.ok && grew.ok && Boolean(clone), { d: d.id, clone, nodes: [nodesBefore, grew.value?.nodes] });
  if (!clone) return;
  const reselected = await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  const cleared = await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  note("clone-deselected-before-its-drag", { cleared: cleared.ok, selection: selectionIds(cleared.value), reading: "board `sole_indirect_handle_hit_idle_selected_node`: while a node with a sole free handle is the only selection, a press inside it resolves to that handle and starts a link drag" });
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  const grab = { x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 };
  await dragBy(grab, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const cloneOffset = offsetOf(beforeDrag, dragged.value, clone);
  verdict("clone-drag-moves-only-the-clone", dragged.ok && movedIds(beforeDrag, dragged.value).every((id) => id === clone), { reselected: reselected.ok, cloneOffset, moved: movedIds(beforeDrag, dragged.value).slice(0, 4), selection: selectionIds(await vitals()), grab, cloneAt });
  const head = dragged.value;
  const added = (await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 2, 30000, 1000)).value;
  const copy = COPY[currentLocale];
  let createRow: HistoryRow | undefined;
  let dragRow: HistoryRow | undefined;
  for (const entry of added) {
    const expanded = await expandEntry(entry.id);
    createRow ??= expanded.mutations.find((row) => row.label.startsWith(copy.createNode(clone)));
    if (cloneOffset) dragRow ??= expanded.mutations.find((row) => row.label.startsWith(dragLabel(1, cloneOffset[0], cloneOffset[1])));
  }
  verdict("duplicate-and-drag-rows-carry-their-mutations", Boolean(createRow && dragRow), { added: added.map((row) => row.label), create: createRow?.label, drag: dragRow?.label, expectedCreate: copy.createNode(clone) });
  if (!createRow || !dragRow) return;
  const rowsPre = documentEntries(await allHistoryRows()).map((row) => `${row.id}=${row.label}`);
  await installBandTrace();
  const via = await pressRowAction(createRow.id, copy.edit);
  const editing = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  const preview = await waitUntil(positions, (p) => placed(p, clone, beforeDrag[clone], 0, 0), 15000);
  verdict("editing-the-create-node-previews-the-clone-before-its-drag", editing.ok && preview.ok, { via, clone: preview.value[clone], beforeDrag: beforeDrag[clone], head: head[clone], waitedMs: preview.waitedMs });
  await hardMinimumRefusal(clone, beforeDrag);
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  verdict("withdraw-control-reachable", withdraw.present && withdraw.disabled !== true, { withdraw, authored: "framework.history.editor.withdraw(.row)" });
  const withdrawn = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("withdraw-previews-the-document-without-the-clone", withdrawn.ok, { withdraw, hasClone: Boolean(withdrawn.value[clone]) });
  await pressBand("accept");
  const blocked = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("replay-review-is-blocked", blocked.value?.review === "blocked", { review: blocked.value?.review, text: blocked.value?.text?.slice(0, 200), outcome: blocked.value?.outcome });
  verdict("review-line-names-the-blocking-errors", Boolean(blocked.value?.text.includes(copy.reviewBlocked)), { text: blocked.value?.text?.slice(0, 200) });
  const finalize = blocked.value?.controls.find((c) => c.control === "finalize");
  verdict("finalize-disabled-while-blocked", finalize?.disabled === true, { finalize });
  await openHistory();
  const failing = (await waitUntil(readHistory, (rows) => rows.some((row) => row.key === dragRow!.key && row.text.includes(copy.targetMissing)), 15000)).value.find((row) => row.key === dragRow!.key);
  verdict("failing-row-reads-error-target-missing", Boolean(failing?.text.includes(copy.targetMissing) && copy.severityError.test(failing.text)), { row: failing?.text?.slice(0, 200), expected: copy.targetMissing });
  await shot("blocked");
  emit({ kind: "inventory", tag: "blocked", rows: await historyInventory(), band: blocked.value });
  const rounds: Record<string, unknown>[] = [];
  for (let round = 0; round < 4 && (await band())?.review === "blocked"; round++) {
    const next: Record<string, unknown> = await pressAuthored("framework.history.timeTravel.nextProblem");
    if (!next.present) {
      await openHistory();
      const problem = (await readHistory()).find((row) => row.kind === "mutation" && copy.severityError.test(row.text));
      next.fallback = problem ? `edit on failing row ${problem.key} via ${await pressRowAction(problem.id, copy.edit)}` : "no failing row";
    }
    const generationBefore = (await band())?.generation;
    const reopened = await waitUntil(band, (b) => b?.stage === "editing", 15000);
    await sleep(800);
    const generationEditing = (await band())?.generation;
    const heading = (await editor())?.heading ?? (await band())?.target;
    const pulled: Record<string, unknown> = await pressAuthored("framework.history.editor.withdraw");
    const marked = await waitUntil(readHistory, (rows) => rows.some((row) => row.key === dragRow!.key && /Withdrawn|Zurückgezogen/.test(row.text)), 8000);
    const dragText = marked.value.find((row) => row.key === dragRow!.key)?.text?.slice(0, 160);
    await pressBand("accept");
    const settled = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
    rounds.push({ round, next, reopened: reopened.ok, generationBefore, generationEditing, heading, pulled, dragMarkedWithdrawn: marked.ok, dragText, review: settled.value?.review, accepted: settled.value?.text?.match(/Accepted changes: \d+|Übernommene Änderungen: \d+/)?.[0] });
  }
  verdict("next-problem-control-reachable", rounds.length > 0 && (rounds[0].next as { present?: boolean } | undefined)?.present === true, { first: rounds[0]?.next, authored: "framework.history.timeTravel.nextProblem(.row)" });
  const ready = await band();
  verdict("withdrawing-the-failing-drag-makes-the-review-ready", ready?.review === "ready", { rounds, review: ready?.review, text: ready?.text?.slice(0, 160) });
  await shot("resolved");
  await pressBand("exit");
  const gone = await waitUntil(band, (b) => b === null, 20000);
  verdict("exit-closes-the-session", gone.ok, { band: gone.value?.stage ?? null });
  const after = await waitUntil(positions, (p) => movedIds(head, p).length === 0, 15000);
  verdict("exit-leaves-head-positions-unchanged", after.ok, { drift: movedIds(head, after.value).slice(0, 8) });
  const seqOf = (row: string) => Number(row.match(/framework\.history\.entry\.(\d+)=/)?.[1] ?? -1);
  const newestPre = Math.max(...rowsPre.map(seqOf));
  const rowsPost = documentEntries(await allHistoryRows()).map((row) => `${row.id}=${row.label}`);
  const addedRows = rowsPost.filter((row) => seqOf(row) > newestPre);
  const relabelled = rowsPost.filter((row) => seqOf(row) <= newestPre && rowsPre.some((pre) => seqOf(pre) === seqOf(row)) && !rowsPre.includes(row));
  verdict("exit-leaves-no-new-rows", addedRows.length === 0 && relabelled.length === 0, { newestPre, added: addedRows.map((row) => row.slice(0, 120)), relabelled: relabelled.map((row) => row.slice(0, 120)) });
  await shot("exited");
  await closePanels();
};

/** 📟️ Step 9 — the locale's console: uncaught page errors and hard guest faults fail; errors, warnings and `[DEBUG] `
 * lines are digested with their most frequent texts. */
const step9 = async (_ctx: Ctx) => {
  const mine = consoleRows.filter((row) => row.locale === currentLocale);
  const errors = mine.filter((row) => row.type === "error" && !BENIGN_RE.test(row.text));
  const warnings = mine.filter((row) => row.type === "warning" && !BENIGN_RE.test(row.text));
  const debug = mine.filter((row) => row.text.includes("[DEBUG] "));
  const http = mine.filter((row) => row.type === "http");
  const top = (rows: ConsoleRow[]) => [...rows.reduce((map, row) => map.set(row.text.slice(0, 160), (map.get(row.text.slice(0, 160)) ?? 0) + 1), new Map<string, number>())].sort((x, y) => y[1] - x[1]).slice(0, 12);
  const refusals = [...new Set(mine.map((row) => /refused a local batch \S+ ((?:local|sync)\.[\w.-]+)/.exec(row.text)?.[1]).filter((code): code is string => Boolean(code)))];
  const shownCodes = await shownNoticeCodes();
  if (!refusals.length) note("no-command-rejection-in-this-run", { shownCodes });
  else verdict("rejection-notices-carry-their-code", refusals.every((code) => shownCodes.includes(code)), { refusals, shownCodes, reading: renderer === "wgpu" ? "wgpu projects the transient notice as the polite `shell.notice` mirror node described by its code (`🧯️wgpu-transient-notice`); the main page's trace saw these codes" : "React `[data-notice-code]` over the main page's life" });
  const uncaught = pageErrors.filter((row) => row.locale === currentLocale);
  verdict("no-uncaught-page-errors", uncaught.length === 0, { count: uncaught.length, first: uncaught.slice(0, 3) });
  const hard = hardFaults.filter((row) => row.locale === currentLocale);
  verdict("no-hard-guest-faults", hard.length === 0, { count: hard.length, first: hard.slice(0, 3) });
  verdict("console-is-debug-free", debug.length === 0, { count: debug.length, top: top(debug).slice(0, 6), reading: "AGENTS.md: temporary `[DEBUG] ` logs are removed before a run counts" });
  note("console-digest", { lines: mine.length, errors: errors.length, warnings: warnings.length, debug: debug.length, http: top(http), topErrors: top(errors), topWarnings: top(warnings), topDebug: top(debug) });
};

//#region 🔖️Gaps
type Box = { x: number; y: number; width: number; height: number };

/** 🪐️ The board's pivot math of `rotate-selection` / `scale-selection`: the centroid of `ids`, a point turned by `degrees`
 * about a pivot (counter-clockwise in the board's world frame, the corpus `rotate-and-scale-replay` convention) and a point
 * spread by `factor` from it. */
const centroidOf = (p: Positions, ids: string[]): [number, number] => [ids.reduce((sum, id) => sum + p[id][0], 0) / ids.length, ids.reduce((sum, id) => sum + p[id][1], 0) / ids.length];
const rotatedAbout = (point: [number, number], pivot: [number, number], degrees: number): [number, number] => {
  const radians = (degrees * Math.PI) / 180;
  const [x, y] = [point[0] - pivot[0], point[1] - pivot[1]];
  return [pivot[0] + x * Math.cos(radians) - y * Math.sin(radians), pivot[1] + x * Math.sin(radians) + y * Math.cos(radians)];
};
const scaledAbout = (point: [number, number], pivot: [number, number], factor: number): [number, number] => [pivot[0] + (point[0] - pivot[0]) * factor, pivot[1] + (point[1] - pivot[1]) * factor];
const sitsAt = (p: Positions, id: string, expected: [number, number], eps = 0.05) => Boolean(p[id] && near(p[id][0], expected[0], eps) && near(p[id][1], expected[1], eps));

/** 🖇️ Clears the selection (Escape) and selects `rows` on the board: a click, then Shift-clicks; answers the selection. */
const selectNodes = async (rows: { id: string; at: Point }[]) => {
  await closePanels();
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 4000);
  for (const [index, row] of rows.entries()) {
    if (index > 0) await page.keyboard.down("Shift");
    await page.mouse.click(row.at.x, row.at.y);
    if (index > 0) await page.keyboard.up("Shift");
    await waitUntil(vitals, (v) => selectionIds(v).includes(row.id), 8000);
  }
  return selectionIds(await vitals());
};

/** 🗾️ The current screen rows of `ids` (camera and pane as published now). */
const rowsOf = async (ids: string[]) => (await layout()).rows.filter((row) => ids.includes(row.id));

/** 🎛️ Runs one arg-carrying app command the way a person reaches it: the palette (`mod+p`), the query, the item — which opens
 * the bottom-middle command panel on its category with the command's staged form (React `buildCommandCategoryTree`, wgpu
 * `build_command_category_ui`) — then the form's Execute (`command-<key>-execute`) with the staged defaults. */
const runPaletteCommand = async (query: string, label: RegExp, commandId: string) => {
  await prepareChord();
  await page.keyboard.press(`${mod}+p`).catch(() => {});
  if (renderer === "wgpu") {
    const input = await waitUntil(async () => mirrorFind(await mirror(), "ui.search.input") ?? null, (node) => node !== null, 8000);
    const field = input.value ? await mirrorLocator(input.value.key, input.value.window) : null;
    await field?.fill(query, { force: true, timeout: 4000 }).catch(() => {});
    const item = await waitUntil(async () => (await mirror()).find((node) => /(^|[./])ui\.search\.item\.\d+$/.test(node.key) && label.test(node.label.trim())) ?? null, (node) => node !== null, 8000);
    if (item.value) await mirrorActivate(item.value.key, item.value.window);
    const execute = await waitUntil(async () => (await mirror()).find((node) => node.key.endsWith(`${commandId}-execute`)) ?? null, (node) => node !== null, 10000);
    const staged = (await mirror()).filter((node) => node.key.includes(`${commandId}.arg.`)).map((node) => `${node.key.replace(/^.*\.arg\./, "")}=${node.value ?? node.valueNow ?? node.label}`);
    if (execute.value && !execute.value.disabled) await mirrorActivate(execute.value.key, execute.value.window);
    return { palette: input.ok, item: item.value?.label ?? null, execute: execute.value?.key ?? null, disabled: execute.value?.disabled ?? null, staged };
  }
  const input = page.locator('[id$="ui.search.input"]').first();
  const opened = await waitUntil(() => input.count().catch(() => 0), (count) => count > 0, 8000);
  await input.fill(query, { timeout: 4000 }).catch(() => {});
  const byCommandId = page.locator(`[data-command-item-id$=".${commandId}"]`).first();
  const item = (await waitUntil(() => byCommandId.count().catch(() => 0), (count) => count > 0, 6000)).ok ? byCommandId : page.locator("[data-command-item-id]").filter({ hasText: label }).first();
  const itemText = await item.innerText({ timeout: 2000 }).catch(() => null);
  await item.click({ timeout: 4000 }).catch((error) => log(`palette item click failed ${String(error).split("\n")[0]}`));
  const execute = page.locator(`[id$="${commandId}-execute"]`).first();
  const shown = await waitUntil(() => execute.count().catch(() => 0), (count) => count > 0, 10000);
  const executeId = shown.ok ? await execute.getAttribute("id").catch(() => null) : null;
  const staged = await evalSafe((id) => Array.from(document.querySelectorAll<HTMLElement>(`[id*="${id}.arg."]`)).map((el) => `${el.id.replace(/^.*\.arg\./, "")}=${(el.querySelector("input") as HTMLInputElement | null)?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? el.innerText.replace(/\s+/g, " ").trim().slice(0, 40)}`), [] as string[], commandId);
  const disabled = shown.ok ? await execute.isDisabled({ timeout: 2000 }).catch(() => null) : null;
  if (shown.ok) {
    await execute.hover({ timeout: 2000 }).catch(() => {});
    await execute.click({ force: true, timeout: 4000 }).catch((error) => log(`execute click failed ${String(error).split("\n")[0]}`));
  }
  return { palette: opened.ok, item: itemText, execute: executeId, disabled, staged, inventory: shown.ok ? null : await evalSafe(() => Array.from(document.querySelectorAll("[id]")).filter((el) => /^command[-.]|command\.category/.test(el.id.split("/").pop() ?? "")).map((el) => el.id).slice(0, 40), [] as string[]) };
};

/** 🕛️ The angle detents a dial's ticks name in degrees: radians when they fit one turn (the stored unit), else as given. */
const tickDegrees = (ticks: { snap: number }[]) => ticks.map((tick) => Math.round(Math.abs(tick.snap) <= Math.PI + 1e-6 ? (tick.snap * 180) / Math.PI : tick.snap));

/** 🚪️ Exits the live session through the band and waits for it to close. */
const exitSession = async () => {
  await pressBand("exit");
  return waitUntil(band, (b) => b === null, 20000);
};

/** 🏆️ Accepted drafts → Finalize → the prompt's destructive Overwrite; answers whether the session closed. */
const finalizeOverwrite = async () => {
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  await dialogChoose("overwrite");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  return { dialog: dialog.ok, closed: gone.ok, band: gone.value?.stage ?? null };
};

/** 🎡️ Step 10 — G6 input controls and their edits: a rotate command → Edit → the angle dial (ticks 0, ±90, 180°, degrees,
 * Arrow = 1°, PageUp = next detent) → Exit with zero trace; Edit again → PageUp to 180° → Accept → Finalize overwrite → the head
 * and the row read 180°. A scale command → Edit → the factor slider (ticks at its snaps on a log axis; a typed factor beyond the
 * hard bound is refused naming it and the draft is kept) → a typed factor 2 → Accept → Finalize overwrite → head and row read 2. */
const step10 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const copy = COPY[currentLocale];
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { picked } = await pickNodes(2, [[0, 0]], exclude, 60);
  if (picked.length < 2) {
    verdict("g6-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const ids = picked.map((row) => row.id);
  const selected = await selectNodes(picked);
  verdict("g6-two-nodes-selected", selected.length === 2 && ids.every((id) => selected.includes(id)), { selected, ids });
  const before = await positions();
  const pivot = centroidOf(before, ids);
  const newestRotate = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const rotateRun = await runPaletteCommand(currentLocale === "de" ? "Drehen" : "Rotate", copy.rotateCommand, "rotateSelection");
  const rotated = await waitUntil(positions, (p) => ids.every((id) => sitsAt(p, id, rotatedAbout(before[id], pivot, 90))), 20000);
  verdict("g6-rotate-command-turns-the-selection-by-90-degrees", rotated.ok, { run: rotateRun, a: rotated.value[ids[0]], expected: rotatedAbout(before[ids[0]], pivot, 90), pivot, reading: "palette `mod+p` → Rotate… → command panel form (angle default 90) → Execute; one `rotateSelection` ToolTransaction" });
  await closePanels();
  const rotateRow = await newMutation(newestRotate, (row) => row.label.startsWith(copy.rotate(2, labelNumber(90, currentLocale))));
  verdict("g6-rotate-is-one-row-labelled-from-its-leaf", Boolean(rotateRow.mutation), { entry: rotateRow.entry?.label, mutation: rotateRow.mutation?.label, expected: copy.rotate(2, labelNumber(90, currentLocale)) });
  await shot("rotated");
  if (rotateRow.mutation) {
    await installBandTrace();
    const begun = await beginEditOf(rotateRow.mutation.key);
    verdict("g6-edit-opens-the-rotate-session", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
    if (begun.band?.stage === "editing") {
      const dial = (await waitUntil(() => readNumberControl("angle"), (control) => control !== null, 15000)).value;
      const degrees = tickDegrees(dial?.ticks ?? []);
      verdict("g6-angle-is-a-dial", Boolean(dial?.dial), { dial, reading: "x-semio-ui `widget: dial` → `SliderAppearance::Dial` (the editor builder's Dial arm, S2-W1E S2.4)" });
      if (renderer === "wgpu") note("g6-angle-ticks-are-painted-only", { dial, screenshot: await shot("angle-dial"), reading: "the wgpu mirror projects a slider as `<input type=range>` with `aria-value*`; detent ticks are painted only, so the tick verdict is React's and the detent law is proven by PageUp below" });
      else verdict("g6-angle-dial-shows-ticks-at-0-90-180-degrees", [-180, -90, 0, 90, 180].every((value) => degrees.includes(value)), { ticks: dial?.ticks, degrees, expected: [-180, -90, 0, 90, 180] });
      verdict("g6-angle-reads-degrees", Boolean(dial && dial.valueNow !== null && near(dial.valueNow, 90, 0.01) && /°|deg/.test(dial.valueText ?? "")), { valueNow: dial?.valueNow, valueText: dial?.valueText, min: dial?.min, max: dial?.max, expected: "aria-valuenow 90, aria-valuetext \"90 °\" (displayFactor 180/π, displayUnit deg)" });
      const turnedBy = (value: number) => async () => {
        const p = await positions();
        return ids.every((id) => sitsAt(p, id, rotatedAbout(before[id], pivot, value)));
      };
      await keyEditorNumber("angle", "ArrowUp");
      const up = await waitUntil(turnedBy(91), (ok) => ok, 10000, 300);
      const afterUp = await readNumberControl("angle");
      await keyEditorNumber("angle", "ArrowDown");
      const down = await waitUntil(turnedBy(90), (ok) => ok, 10000, 300);
      verdict("g6-arrow-keys-step-the-angle-by-one-degree", up.ok && down.ok, { up: up.ok, down: down.ok, valueAfterUp: afterUp?.valueNow, a: (await positions())[ids[0]], expectedAtUp: rotatedAbout(before[ids[0]], pivot, 91) });
      await keyEditorNumber("angle", "PageUp");
      const detent = await waitUntil(turnedBy(180), (ok) => ok, 10000, 300);
      const afterPage = await readNumberControl("angle");
      verdict("g6-page-up-goes-to-the-next-detent", detent.ok, { valueAfterPageUp: afterPage?.valueNow, expected: 180, a: (await positions())[ids[0]], expectedA: rotatedAbout(before[ids[0]], pivot, 180), reading: "design §18: PageUp = next detent when snaps exist (90° → 180°), else ±10 steps" });
      await shot("angle-dial");
      const exited = await exitSession();
      const kept = await waitUntil(turnedBy(90), (ok) => ok, 15000, 400);
      verdict("g6-exit-leaves-the-rotation-as-it-was", exited.ok && kept.ok, { band: exited.value?.stage ?? null, a: (await positions())[ids[0]] });
      const again = await beginEditOf(rotateRow.mutation.key);
      if (again.band?.stage === "editing") {
        await keyEditorNumber("angle", "PageUp");
        const half = await waitUntil(turnedBy(180), (ok) => ok, 10000, 300);
        await pressBand("accept");
        const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
        const finalized = review.value?.review === "ready" ? await finalizeOverwrite() : null;
        const head = await waitUntil(turnedBy(180), (ok) => ok, 15000, 400);
        verdict("g6-rotate-edit-finalizes-the-new-angle", half.ok && review.value?.review === "ready" && Boolean(finalized?.closed) && head.ok, { preview180: half.ok, review: review.value?.review, finalized, a: head.value ? (await positions())[ids[0]] : null, expected: rotatedAbout(before[ids[0]], pivot, 180), reading: "PageUp to the next detent (180°) → Accept → Finalize → Overwrite: the rotation is replayed about its recorded pivot" });
        const relabelled = await findMutationRow(rotateRow.mutation.key);
        verdict("g6-rotate-row-reads-the-edited-angle", Boolean(relabelled?.label.startsWith(copy.rotate(2, labelNumber(180, currentLocale)))), { label: relabelled?.label, expected: copy.rotate(2, labelNumber(180, currentLocale)) });
        if (!finalized?.closed) await exitSession();
      } else verdict("g6-rotate-edit-finalizes-the-new-angle", false, { reason: "the second rotate session did not open", via: again.via, band: again.band?.text?.slice(0, 160) });
    }
  }
  const beforeScale = await positions();
  const pivotScale = centroidOf(beforeScale, ids);
  const newestScale = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const stillSelected = selectionIds(await vitals());
  if (!ids.every((id) => stillSelected.includes(id))) await selectNodes(await rowsOf(ids));
  const scaleRun = await runPaletteCommand(currentLocale === "de" ? "Skalieren" : "Scale", copy.scaleCommand, "scaleSelection");
  const scaled = await waitUntil(positions, (p) => ids.every((id) => sitsAt(p, id, scaledAbout(beforeScale[id], pivotScale, 1.5))), 20000);
  verdict("g6-scale-command-spreads-the-selection-by-1.5", scaled.ok, { run: scaleRun, a: scaled.value[ids[0]], expected: scaledAbout(beforeScale[ids[0]], pivotScale, 1.5), pivot: pivotScale });
  await closePanels();
  const scaleRow = await newMutation(newestScale, (row) => row.label.startsWith(copy.scale(2, labelNumber(1.5, currentLocale))));
  verdict("g6-scale-is-one-row-labelled-from-its-leaf", Boolean(scaleRow.mutation), { entry: scaleRow.entry?.label, mutation: scaleRow.mutation?.label, expected: copy.scale(2, labelNumber(1.5, currentLocale)) });
  if (!scaleRow.mutation) return;
  await installBandTrace();
  const begun = await beginEditOf(scaleRow.mutation.key);
  verdict("g6-edit-opens-the-scale-session", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const factor = (await waitUntil(() => readNumberControl("factor"), (control) => control !== null, 15000)).value;
  const snaps = (factor?.ticks ?? []).map((tick) => tick.snap);
  const travel = { min: factor?.min ?? 0.1, max: factor?.max ?? 10 };
  const logAt = (value: number) => (100 * Math.log(value / travel.min)) / Math.log(travel.max / travel.min);
  if (renderer === "wgpu") note("g6-factor-ticks-are-painted-only", { factor, screenshot: await shot("factor-slider"), reading: "ticks and their axis positions are painted only on wgpu" });
  else {
    verdict("g6-factor-slider-has-ticks-at-its-snaps", [0.25, 0.5, 1, 2, 4].every((value) => snaps.some((snap) => near(snap, value, 1e-6))), { ticks: factor?.ticks, expected: [0.25, 0.5, 1, 2, 4] });
    verdict("g6-factor-ticks-sit-on-a-log-axis", (factor?.ticks.length ?? 0) >= 5 && (factor?.ticks ?? []).every((tick) => tick.left !== null && near(tick.left, logAt(tick.snap), 1.5)), { ticks: factor?.ticks, expectedLeft: snaps.map((snap) => Number(logAt(snap).toFixed(1))), travel, reading: "`scale: log`: 1 sits at the travel's geometric middle (50 % for 0.1 … 10), never at 9 % as on a linear axis" });
  }
  const draftBefore = await positions();
  const typed = await typeSliderText("factor", "-5");
  await sleep(1200);
  const refused = await readNumberControl("factor");
  const refusal = refused?.refusal ?? (copy.greaterThanZero.test(refused?.rowText ?? "") ? refused?.rowText ?? null : null);
  verdict("g6-out-of-bounds-factor-is-refused-naming-the-bound", Boolean(refusal && copy.greaterThanZero.test(refusal)), { typed, refusal, invalid: refused?.invalid, rowText: refused?.rowText, expected: String(copy.greaterThanZero), reading: "schema `exclusiveMinimum: 0` → `UiNumberBound` refusal in display units" });
  const drift = movedIds(draftBefore, await positions(), 1e-6);
  verdict("g6-refusal-keeps-the-draft", drift.length === 0 && Boolean(refused && factor && refused.valueNow !== null && factor.valueNow !== null && near(refused.valueNow, factor.valueNow, 1e-6)), { valueBefore: factor?.valueNow, valueAfter: refused?.valueNow, drift: drift.slice(0, 6) });
  await shot("factor-refused");
  if (renderer !== "wgpu") await page.keyboard.press("Escape").catch(() => {});
  const doubled = await typeSliderText("factor", "2");
  const spreadBy = (factor: number) => async () => {
    const p = await positions();
    return ids.every((id) => sitsAt(p, id, scaledAbout(beforeScale[id], pivotScale, factor)));
  };
  const spread = await waitUntil(spreadBy(2), (ok) => ok, 15000, 300);
  verdict("g6-typed-factor-previews-the-new-spread", spread.ok, { typed: doubled, a: (await positions())[ids[0]], expected: scaledAbout(beforeScale[ids[0]], pivotScale, 2) });
  await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  const finalized = review.value?.review === "ready" ? await finalizeOverwrite() : null;
  const head = await waitUntil(spreadBy(2), (ok) => ok, 15000, 400);
  verdict("g6-scale-edit-finalizes-the-new-factor", review.value?.review === "ready" && Boolean(finalized?.closed) && head.ok, { review: review.value?.review, finalized, a: (await positions())[ids[0]], expected: scaledAbout(beforeScale[ids[0]], pivotScale, 2) });
  const relabelled = await findMutationRow(scaleRow.mutation.key);
  verdict("g6-scale-row-reads-the-edited-factor", Boolean(relabelled?.label.startsWith(copy.scale(2, labelNumber(2, currentLocale)))), { label: relabelled?.label, expected: copy.scale(2, labelNumber(2, currentLocale)) });
  if (!finalized?.closed) await exitSession();
  await closePanels();
};

/** ♻️ Step 11 — keep editing after a clean review: the drag's dy → Accept → `ready` → Begin the downstream drag's dx (from
 * the review) → Discard (back to the review, one accepted draft) → Begin it again → Accept the other way (chord ↔ button) →
 * `ready` with two accepted drafts → Finalize overwrite of both → one row, head carries both. */
const step11 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || !ctx.cMutation || !ctx.dragMutation) {
    verdict("precondition-step-2", false, { ctx: Object.keys(ctx) });
    return;
  }
  const copy = COPY[currentLocale];
  const head = await positions();
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  verdict("keep-editing-first-session-opens", begun.band?.stage === "editing", { via: begun.via, stage: begun.band?.stage });
  if (begun.band?.stage !== "editing") return;
  const ed = (await waitUntil(readEditor, (value) => Boolean(value?.dx && value?.dy), 15000)).value;
  const [dx, dy] = [Number(ed?.dx?.value ?? Number.NaN), Number(ed?.dy?.value ?? Number.NaN)];
  const dyNew = Math.round(dy + 10);
  const typedDy = await typeEditorNumber("dy", dyNew);
  await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], dx, dyNew), 15000);
  await pressBand("accept");
  const first = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("keep-editing-first-review-is-ready", first.value?.review === "ready", { typedDy, dx, dy, dyNew, review: first.value?.review, text: first.value?.text?.slice(0, 160) });
  if (first.value?.review !== "ready") return;
  const second = await beginEditOf(ctx.cMutation);
  verdict("keep-editing-begins-another-mutation-from-a-ready-review", second.band?.stage === "editing", { via: second.via, row: second.row?.label, band: second.band?.text?.slice(0, 160), reading: "design §4: Begin is legal from Reviewing; the session keeps its accepted draft" });
  if (second.band?.stage !== "editing") return;
  const cEd = (await waitUntil(readEditor, (value) => Boolean(value?.dx), 15000)).value;
  const cdx = Number(cEd?.dx?.value ?? Number.NaN);
  const cdxNew = Math.round(cdx + 15);
  const discardedDraft = await typeEditorNumber("dx", cdxNew);
  const reviewedHead = await positions();
  const discardVia = await pressBand("discard");
  const discarded = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 20000, 100);
  const cBack = await waitUntil(positions, (p) => placed(p, ctx.c!, head[ctx.c!], 0, 0), 10000);
  verdict("keep-editing-discard-returns-to-the-review-keeping-the-accepted-draft", discarded.value?.review === "ready" && discarded.value.text.includes(copy.accepted(1)) && cBack.ok, { via: discardVia, typed: discardedDraft, review: discarded.value?.review, text: discarded.value?.text?.slice(0, 200), c: cBack.value[ctx.c], cHead: head[ctx.c], a: reviewedHead[ctx.a], reading: "design §4: Discard returns to the stage the draft began from (Reviewing), the accepted draft stays, the discarded one leaves no trace" });
  const third = await beginEditOf(ctx.cMutation);
  if (third.band?.stage !== "editing") {
    verdict("keep-editing-second-review-is-ready-with-two-accepted-drafts", false, { reason: "the session did not reopen on the downstream drag after Discard", via: third.via, band: third.band?.text?.slice(0, 160) });
    return;
  }
  const typedDx = await typeEditorNumber("dx", cdxNew);
  const acceptVia = await pressBand("accept", otherWay());
  const both = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("keep-editing-second-review-is-ready-with-two-accepted-drafts", both.value?.review === "ready" && both.value.text.includes(copy.accepted(2)), { typedDx, cdx, cdxNew, via: acceptVia, review: both.value?.review, text: both.value?.text?.slice(0, 200), expected: copy.accepted(2), reading: "the second Accept takes the way this locale does not take by default (chord ↔ button), so both are driven" });
  const reviewed = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], dx, dyNew) && placed(p, ctx.c!, head[ctx.c!], cdxNew - cdx, 0), 15000);
  verdict("keep-editing-review-shows-both-drafts", reviewed.ok && placed(reviewed.value, ctx.b, ctx.p0[ctx.b], dx, dyNew), { a: reviewed.value[ctx.a], c: reviewed.value[ctx.c], expectedC: head[ctx.c] ? [head[ctx.c][0] + cdxNew - cdx, head[ctx.c][1]] : null });
  await shot("two-drafts");
  const finalized = await finalizeOverwrite();
  const expected = copy.row("edit", null, 2);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict("keep-editing-finalize-overwrites-both-in-one-row", finalized.closed && rows.ok, { finalized, expected, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  const after = await positions();
  const others = movedIds(head, after).filter((id) => id !== ctx.a && id !== ctx.b && id !== ctx.c);
  verdict("keep-editing-head-carries-both-edits", placed(after, ctx.a, ctx.p0[ctx.a], dx, dyNew) && placed(after, ctx.b, ctx.p0[ctx.b], dx, dyNew) && placed(after, ctx.c, head[ctx.c], cdxNew - cdx, 0) && others.length === 0, { a: after[ctx.a], c: after[ctx.c], others: others.slice(0, 6) });
  await closePanels();
};

/** 🩹️ Step 12 — G3, the fatal loop resolved by EDITING targets: duplicate a node and drag the clone; withdraw the upstream
 * `create-node` → the drag is `mutation.target-missing`, review `blocked` → Next problem opens the drag → select two remaining
 * nodes → Use selection → chips read labels, the board highlights them, the preview moves them → Accept → `ready` → Finalize
 * overwrite → head = the pre-session head without the clone, the two nodes moved by the drag's offset. */
const step12 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const copy = COPY[currentLocale];
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const spot = await pickCloneSource(exclude);
  if (!spot) {
    verdict("g3-clone-source-with-free-space", false, { reason: "no node whose clone (+24,+24) and drag destination are ≥ 55 world units from every other node" });
    return;
  }
  const d = spot.row;
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const idsBefore = Object.keys(await positions());
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const [source] = await rowsOf([d.id]);
  await selectNodes(source ? [source] : [d]);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  const grew = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  verdict("g3-duplicate-adds-one-clone", grew.ok && Boolean(clone), { d: d.id, clone, nodes: [nodesBefore, grew.value?.nodes] });
  if (!clone) return;
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  await dragBy({ x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 }, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const move = offsetOf(beforeDrag, dragged.value, clone);
  verdict("g3-clone-drag-moves-only-the-clone", dragged.ok && movedIds(beforeDrag, dragged.value).every((id) => id === clone), { move, moved: movedIds(beforeDrag, dragged.value).slice(0, 4) });
  if (!move) return;
  const createRow = await newMutation(newestBefore, (row) => row.label.startsWith(copy.createNode(clone)));
  const dragRow = await newMutation(newestBefore, (row) => row.label.startsWith(dragLabel(1, move[0], move[1])));
  verdict("g3-create-and-drag-rows-carry-their-mutations", Boolean(createRow.mutation && dragRow.mutation), { create: createRow.mutation?.label, drag: dragRow.mutation?.label, expected: [copy.createNode(clone), dragLabel(1, move[0], move[1])] });
  if (!createRow.mutation || !dragRow.mutation) return;
  const dragKey = dragRow.mutation.key;
  const head = await positions();
  await closePanels();
  const { picked } = await pickNodes(2, [[move[0], move[1]]], [...exclude, d.id, clone], 60);
  if (picked.length < 2) {
    verdict("g3-two-remaining-nodes-to-pick", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const targets = picked.map((row) => row.id);
  await installBandTrace();
  const begun = await beginEditOf(createRow.mutation.key);
  verdict("g3-edit-opens-the-upstream-create-node", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  const deleted = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("g3-withdrawing-the-create-node-deletes-the-upstream-node", withdraw.present && deleted.ok, { withdraw });
  await pressBand("accept");
  const blocked = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("g3-the-review-is-blocked", blocked.value?.review === "blocked", { review: blocked.value?.review, text: blocked.value?.text?.slice(0, 200) });
  const failing = (await waitUntil(() => findMutationRow(dragKey), (row) => Boolean(row?.text.includes(copy.targetMissing)), 15000)).value;
  verdict("g3-the-downstream-drag-reads-error-target-missing", Boolean(failing?.text.includes(copy.targetMissing)), { row: failing?.text?.slice(0, 200), expected: copy.targetMissing });
  await shot("g3-blocked");
  const next = await pressAuthored("framework.history.timeTravel.nextProblem");
  const reopened = await waitUntil(band, (b) => b?.stage === "editing", 15000);
  await sleep(800);
  const heading = (await editor())?.heading ?? reopened.value?.target ?? null;
  verdict("g3-next-problem-opens-the-failing-drag", next.present && reopened.ok && Boolean((heading ?? "").includes(dragLabel(1, move[0], move[1]))), { next, heading, band: reopened.value?.text?.slice(0, 160) });
  if (!reopened.ok) return;
  const selected = await selectNodes(await rowsOf(targets));
  verdict("g3-board-selection-works-while-editing", selected.length === 2 && targets.every((id) => selected.includes(id)), { selected, targets, reading: "design §7: interaction (selection) verbs keep working while the document is frozen" });
  await openHistory();
  const used = await pressUseSelection("targets");
  const chips = await waitUntil(readEditor, (value) => (value?.targets?.chips.length ?? 0) === 2, 10000);
  const chipTexts = chips.value?.targets?.chips ?? [];
  verdict("g3-use-selection-replaces-the-targets", Boolean(used) && chips.ok, { used, chips: chipTexts, targets });
  verdict("g3-chips-show-labels-not-raw-ids", chipTexts.length === 2 && chipTexts.every((chip) => chip.trim().length > 0 && !targets.some((id) => chip.includes(id))), { chips: chipTexts, ids: targets, reading: "design §16.4: a chip reads the entity's label (`ArtifactApp::entity_label`), not its raw id" });
  const lit = await waitUntil(vitals, (v) => {
    const shown = highlightedIds(v);
    return shown !== null && shown.length === 2 && targets.every((id) => shown.includes(id));
  }, 10000);
  verdict("g3-board-highlights-the-referenced-nodes", lit.ok, { highlighted: highlightedIds(lit.value), raw: lit.value?.highlighted ?? null, targets, reading: renderer === "wgpu" ? "wgpu: `dumpBoard2d` must publish `highlighted` (the board scene's `highlighted_ids_json`); absent = not observable" : "React `data-board-highlighted-ids-json` (`Board2dScene.highlighted_ids_json` ← `InteractionView::draft_references`)" });
  const preview = await waitUntil(positions, (p) => targets.every((id) => placed(p, id, head[id], move[0], move[1], 0.05)), 15000);
  verdict("g3-preview-moves-the-new-targets", preview.ok, { targets: targets.map((id) => ({ id, at: preview.value[id], expected: head[id] ? [head[id][0] + move[0], head[id][1] + move[1]] : null })) });
  await shot("g3-use-selection");
  await pressBand("accept");
  const ready = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("g3-editing-the-targets-makes-the-review-ready", ready.value?.review === "ready", { review: ready.value?.review, text: ready.value?.text?.slice(0, 200) });
  if (ready.value?.review !== "ready") {
    await exitSession();
    await closePanels();
    return;
  }
  const finalized = await finalizeOverwrite();
  const expectedRow = copy.row("edit", null, 2);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expectedRow)), 20000, 1000);
  verdict("g3-finalize-overwrite-closes-the-session", finalized.closed && rows.ok, { finalized, expectedRow, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  const final = await positions();
  const expected: Positions = Object.fromEntries(Object.entries(head).filter(([id]) => id !== clone).map(([id, xy]): [string, [number, number]] => [id, targets.includes(id) ? [xy[0] + move[0], xy[1] + move[1]] : xy]));
  const drift = movedIds(expected, final, 0.05);
  verdict("g3-head-equals-the-expectation", drift.length === 0, { drift: drift.slice(0, 8), clonePresent: Boolean(final[clone]), targets: targets.map((id) => ({ id, at: final[id], expected: expected[id] })) });
  await shot("g3-finalized");
  await closePanels();
};

/** ⚠️ Step 13 — G4, a warning introduced by an upstream edit (S2-W2D corpus `warning-from-an-upstream-edit`, on the live
 * example): lock a node in the Inspection panel, unlock it, drag it together with a second node; withdraw the unlock → the
 * drag skips the locked member, Warning "Partially applied" (new since this edit), review `ready` → Finalize overwrite → the
 * warning stays on the row; the reload check re-reads it after the folder reload. */
const step13 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const copy = COPY[currentLocale];
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { picked } = await pickNodes(2, [[40, 40]], exclude, 60);
  if (picked.length < 2) {
    verdict("g4-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const [p, q] = picked;
  const lockKey = "puzzle2d-play-inspector.node.locked";
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await selectNodes([p]);
  await openTab(INSPECTION_TAB);
  const shown = await waitUntil(() => presentKey(lockKey), (id) => id !== null, 10000);
  const lockText = await textOfKey(lockKey);
  const lockPress = await pressAuthored(lockKey);
  const locked = await waitUntil(() => textOfKey(lockKey), (text) => /\btrue\b/.test(text), 10000);
  const lockMutation = await newMutation(newestBefore, (row) => row.label.startsWith(copy.changeLocked(p.id)));
  verdict("g4-lock-in-the-inspector-is-a-history-row", shown.ok && locked.ok && Boolean(lockMutation.mutation), { node: p.id, before: lockText, after: locked.value, press: lockPress, mutation: lockMutation.mutation?.label, expected: copy.changeLocked(p.id), via: `${INSPECTION_TAB} → ${lockKey}` });
  const newestAfterLock = newestEntrySeq(await allHistoryRows());
  await openTab(INSPECTION_TAB);
  await pressAuthored(lockKey);
  const unlocked = await waitUntil(() => textOfKey(lockKey), (text) => /\bfalse\b/.test(text), 10000);
  const unlockMutation = await newMutation(newestAfterLock, (row) => row.label.startsWith(copy.changeLocked(p.id)) && row.key !== lockMutation.mutation?.key);
  verdict("g4-unlock-in-the-inspector-is-a-history-row", unlocked.ok && Boolean(unlockMutation.mutation), { after: unlocked.value, mutation: unlockMutation.mutation?.key });
  if (!unlockMutation.mutation) return;
  const selected = await selectNodes(await rowsOf([p.id, q.id]));
  const beforeDrag = await positions();
  const newestBeforeDrag = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const zoom = cameraOf(await vitals()).zoom;
  await dragBy(toScreen(beforeDrag[p.id], cameraOf(await vitals()), await paneBox()), 40 * zoom, 40 * zoom);
  const moved = await waitUntil(positions, (now) => [p.id, q.id].every((id) => offsetOf(beforeDrag, now, id)?.some((v) => Math.abs(v) > 0.5) === true), 20000);
  const offset = offsetOf(beforeDrag, moved.value, p.id);
  verdict("g4-both-nodes-dragged-together", moved.ok && Boolean(offset), { selected, offset, q: offsetOf(beforeDrag, moved.value, q.id) });
  if (!offset) return;
  const dragRow = await newMutation(newestBeforeDrag, (row) => row.label.startsWith(dragLabel(2, offset[0], offset[1])));
  verdict("g4-the-drag-is-one-row", Boolean(dragRow.mutation), { mutation: dragRow.mutation?.label, expected: dragLabel(2, offset[0], offset[1]) });
  if (!dragRow.mutation) return;
  const dragKey = dragRow.mutation.key;
  await installBandTrace();
  const begun = await beginEditOf(unlockMutation.mutation.key);
  verdict("g4-edit-opens-the-upstream-unlock", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  await sleep(800);
  await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("g4-the-review-is-ready-not-blocked", review.value?.review === "ready", { withdraw, review: review.value?.review, text: review.value?.text?.slice(0, 200), reading: "design §16.1: warnings never block finalizing" });
  verdict("g4-the-band-names-the-warning", review.value?.outcome === "warning" || /Worst outcome: Warning|Schwerstes Ergebnis: Warnung/.test(review.value?.text ?? ""), { outcome: review.value?.outcome, text: review.value?.text?.slice(0, 200) });
  const warned = (await waitUntil(() => findMutationRow(dragKey), (row) => Boolean(row?.text.includes(copy.warningPartial)), 15000)).value;
  verdict("g4-the-drag-row-reads-warning-partially-applied", Boolean(warned?.text.includes(copy.warningPartial)), { row: warned?.text?.slice(0, 200), expected: copy.warningPartial });
  verdict("g4-the-warning-is-marked-new-since-this-edit", Boolean(warned?.text.includes(copy.introduced)), { row: warned?.text?.slice(0, 200), expected: copy.introduced, reading: "design §16.5: `HistoryMutationEntry.introduced`" });
  const replayed = await positions();
  verdict("g4-the-replay-skips-the-locked-member", placed(replayed, p.id, beforeDrag[p.id], 0, 0, 0.05) && placed(replayed, q.id, beforeDrag[q.id], offset[0], offset[1], 0.05), { p: replayed[p.id], pBefore: beforeDrag[p.id], q: replayed[q.id] });
  await shot("g4-ready-with-warning");
  const finalized = await finalizeOverwrite();
  verdict("g4-finalize-overwrite-closes-the-session", finalized.closed, finalized);
  const after = (await waitUntil(() => findMutationRow(dragKey), (row) => Boolean(row?.text.includes(copy.warningPartial)), 15000)).value;
  verdict("g4-the-warning-stays-visible-after-finalize", Boolean(after?.text.includes(copy.warningPartial)), { row: after?.text?.slice(0, 200) });
  const final = await positions();
  verdict("g4-the-head-keeps-the-locked-member-in-place", placed(final, p.id, beforeDrag[p.id], 0, 0, 0.05) && placed(final, q.id, beforeDrag[q.id], offset[0], offset[1], 0.05), { p: final[p.id], q: final[q.id] });
  ctx.g4 = { drag: dragKey, p: p.id, q: q.id };
  await shot("g4-finalized");
  await closePanels();
};

/** 🔲️ The page box of the band (`band`) or the open prompt (`dialog`): React's element, wgpu's union of the chrome hit rects
 * that carry its control ids. */
const chromeBox = async (kind: "band" | "dialog"): Promise<Box | null> => {
  if (renderer === "wgpu") {
    const hits = ((await chromeHits()).hits ?? []).filter((hit) => (kind === "band" ? /^(ui\.timeTravel|shell\.time-travel)\./ : /^shell\.dialog\./).test(hit.controlId));
    if (!hits.length) return null;
    const [x0, y0] = [Math.min(...hits.map((hit) => hit.rect[0])), Math.min(...hits.map((hit) => hit.rect[1]))];
    const [x1, y1] = [Math.max(...hits.map((hit) => hit.rect[0] + hit.rect[2])), Math.max(...hits.map((hit) => hit.rect[1] + hit.rect[3]))];
    return { x: x0, y: y0, width: x1 - x0, height: y1 - y0 };
  }
  return page.locator(kind === "band" ? "[data-semio-time-travel]" : '[role="dialog"]').first().boundingBox({ timeout: 3000 }).catch(() => null);
};

/** 📏️ Every visible band control with its page box (React `[data-semio-time-travel-control]`, wgpu the band's hit rects). */
const bandControlBoxes = async (): Promise<{ control: string; box: Box | null }[]> => {
  if (renderer === "wgpu") {
    const hits = (await chromeHits()).hits ?? [];
    return WGPU_BAND_CONTROLS.map(([control, key]) => ({ control, hit: hits.find((hit) => hit.controlId === key) })).filter((entry) => entry.hit).map((entry) => ({ control: entry.control, box: { x: entry.hit!.rect[0], y: entry.hit!.rect[1], width: entry.hit!.rect[2], height: entry.hit!.rect[3] } }));
  }
  const controls = page.locator("[data-semio-time-travel-control]");
  const rows: { control: string; box: Box | null }[] = [];
  for (let index = 0; index < (await controls.count().catch(() => 0)); index++) {
    const control = controls.nth(index);
    rows.push({ control: (await control.getAttribute("data-semio-time-travel-control").catch(() => null)) ?? "", box: await control.boundingBox({ timeout: 2000 }).catch(() => null) });
  }
  return rows;
};

/** 🖼️ Whether `box` lies inside the page viewport. */
const insideViewport = (box: Box | null) => {
  const size = page.viewportSize() ?? { width: 0, height: 0 };
  return Boolean(box && box.width > 0 && box.height > 0 && box.x >= -0.5 && box.y >= -0.5 && box.x + box.width <= size.width + 0.5 && box.y + box.height <= size.height + 0.5);
};

/** 🤏️ Taps a band control with a touch (React: the control element; wgpu: the touchscreen at its chrome hit rect). */
const tapBand = async (control: string) => {
  if (renderer === "wgpu") {
    const key = WGPU_BAND_CONTROLS.find(([name]) => name === control)?.[1];
    const hit = ((await chromeHits()).hits ?? []).find((row) => row.controlId === key);
    if (!hit) return (await wgpuPress(key ?? control)) ?? "absent";
    await page.touchscreen.tap(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
    return `tap:${hit.controlId}`;
  }
  return page.locator(`[data-semio-time-travel-control="${control}"]`).first().tap({ timeout: 4000 }).then(() => "tap").catch((error) => `tap failed ${String(error).split("\n")[0]}`);
};

/** 🫵️ Taps the open prompt's destructive Overwrite with a touch. */
const tapOverwrite = async () => {
  if (renderer === "wgpu") {
    const hit = ((await chromeHits()).hits ?? []).find((row) => /^shell\.dialog\.[^.]+\.choice\.overwrite$/.test(row.controlId));
    if (!hit) return (await dialogChoose("overwrite")) ? "mirror" : "absent";
    await page.touchscreen.tap(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
    return `tap:${hit.controlId}`;
  }
  return page.locator('[id="ui.dialog.choice.overwrite"]').first().tap({ timeout: 4000 }).then(() => "tap").catch((error) => `tap failed ${String(error).split("\n")[0]}`);
};

/** 🌀️ Tab and Shift+Tab inside the open prompt: focus must stay in it (a modal traps focus) and reach at least two stops. */
const tabsStayInThePrompt = async () => {
  const trail: string[] = [];
  let inside = true;
  for (const key of ["Tab", "Tab", "Tab", "Shift+Tab", "Shift+Tab"]) {
    await page.keyboard.press(key).catch(() => {});
    await sleep(250);
    const focus = await focusRead();
    trail.push(`${key}→${focus.key ?? focus.tag}`);
    inside &&= focus.inDialog;
  }
  return { ok: inside && new Set(trail.map((row) => row.split("→")[1])).size >= 2, trail };
};

/** 🤳️ Steps 14 and 15 — G13 phone and tablet: a fresh touch context (its own document) at the device's viewport; one tapped and
 * dragged node → Edit → the band inside the viewport with touch-sized controls and no structural ARIA finding, the editor's dx
 * typed by keyboard, Accept by touch, the prompt inside the viewport, keyboard-trapped and ARIA-clean, Overwrite by touch; no
 * horizontal page scroll. Verdicts carry the device key (`mobile-…`, `tablet-…`). */
const deviceJourney = (device: Device) => async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const key = device.key;
    const opened = await open(device, key);
    verdict(`${key}-boots-at-${device.viewport.width}-px`, opened !== null, { viewport: page.viewportSize() });
    if (!opened) {
      dumpJson(`${key}-boot-failure`, { inventory: await historyInventory(), body: await evalSafe(() => document.body?.innerText.slice(0, 400) ?? "", "") });
      await shot(`${key}-boot-failure`);
      return;
    }
    const width = await evalSafe(() => ({ scroll: document.documentElement.scrollWidth, inner: innerWidth }), { scroll: -1, inner: -1 });
    verdict(`${key}-no-horizontal-page-scroll`, width.scroll >= 0 && width.scroll <= width.inner + 1, width);
    const board = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
    await closePanels();
    await frameBoard(2);
    const { camera, picked } = await pickNodes(1, [[40, 0]], [], 40);
    if (!board.ok || !picked.length) {
      verdict(`${key}-one-clickable-node`, false, { vitals: board.value, picked: picked.length });
      return;
    }
    const m = picked[0];
    const newestBefore = newestEntrySeq(await allHistoryRows());
    await closePanels();
    await page.touchscreen.tap(m.at.x, m.at.y);
    const tapped = await waitUntil(vitals, (v) => selectionIds(v).includes(m.id), 10000);
    const p0 = await positions();
    await dragBy(m.at, 40 * camera.zoom, 0);
    const moved = await waitUntil(positions, (p) => offsetOf(p0, p, m.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    const offset = offsetOf(p0, moved.value, m.id);
    verdict(`${key}-tap-selects-and-a-drag-moves-the-node`, tapped.ok && moved.ok, { node: m.id, offset, selection: selectionIds(tapped.value) });
    if (!offset) return;
    const row = await newMutation(newestBefore, (entry) => entry.label.startsWith(dragLabel(1, offset[0], offset[1])));
    if (!row.mutation) {
      verdict(`${key}-the-drag-is-a-row`, false, { expected: dragLabel(1, offset[0], offset[1]) });
      return;
    }
    const begun = await beginEditOf(row.mutation.key);
    verdict(`${key}-edit-opens-the-band`, begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
    if (begun.band?.stage !== "editing") return;
    await shot(`${key}-editing`);
    const bandBox = await chromeBox("band");
    verdict(`${key}-band-inside-the-viewport`, insideViewport(bandBox), { bandBox, viewport: page.viewportSize() });
    const controls = await bandControlBoxes();
    verdict(`${key}-band-controls-are-touch-sized`, controls.length > 0 && controls.every((entry) => entry.box !== null && entry.box.width >= 24 && entry.box.height >= 24 && insideViewport(entry.box)), { controls, reading: "WCAG 2.5.8: a target is at least 24 × 24 CSS px" });
    await ariaVerdict(`${key}-aria-band-and-history-have-no-structural-findings`, "session");
    const dxNew = Math.round(offset[0] + 20);
    const typed = await typeEditorNumber("dx", dxNew);
    const previewed = await waitUntil(positions, (p) => placed(p, m.id, p0[m.id], dxNew, offset[1], 0.05), 15000);
    const dxId = renderer === "wgpu" ? null : await resolveDomId("framework.history.editor.input.dx");
    const dxBox = dxId ? await byId(dxId).boundingBox({ timeout: 2000 }).catch(() => null) : null;
    verdict(`${key}-editor-input-reachable-by-keyboard`, typed.present && previewed.ok && (renderer === "wgpu" || insideViewport(dxBox)), { typed, dxBox, a: previewed.value[m.id] });
    const accept = await tapBand("accept");
    const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
    verdict(`${key}-accept-by-touch-reviews-ready`, review.value?.review === "ready", { accept, review: review.value?.review, text: review.value?.text?.slice(0, 160) });
    const finalize = await tapBand("finalize");
    const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
    const dialogBox = await chromeBox("dialog");
    verdict(`${key}-finalize-prompt-inside-the-viewport`, dialog.ok && insideViewport(dialogBox) && Boolean(dialog.value?.title.includes(copy.dialogTitle)), { finalize, dialogBox, title: dialog.value?.title });
    await shot(`${key}-prompt`);
    await ariaVerdict(`${key}-aria-finalize-prompt-has-no-structural-findings`, "prompt");
    const tabs = await tabsStayInThePrompt();
    verdict(`${key}-finalize-prompt-is-keyboard-reachable`, tabs.ok, tabs);
    const overwrite = await tapOverwrite();
    const gone = await waitUntil(band, (b) => b === null, 30000);
    const head = await positions();
    verdict(`${key}-overwrite-by-touch-closes-the-session`, gone.ok && placed(head, m.id, p0[m.id], dxNew, offset[1], 0.05), { overwrite, band: gone.value?.stage ?? null, a: head[m.id] });
    await shot(`${key}-done`);
  });

const step14 = deviceJourney(PHONE);
const step15 = deviceJourney(TABLET);

//#region 🔖️LongHistory
/** 📈️ Growth rounds a long-history step may add (one palette Set Active Example each, several hundred mutations) when the
 * replay of the current history finishes before a frame shows it. */
const LONG_HISTORY_GROWTH_ROUNDS = 2;

/** 📚️ The example load's transaction of a fresh document — the OLDEST entry row reading Set Active Example, paged from the end
 * of the windowed body, expanded — with the mutation rows the panel lists for it. */
const exampleTransaction = async () => {
  const copy = COPY[currentLocale];
  let entry: HistoryRow | null = null;
  await pageHistory("up", async (rows) => {
    const hits = documentEntries(rows).filter((row) => copy.example.test(row.label)).sort((left, right) => Number(left.key) - Number(right.key));
    entry = hits[0] ?? entry;
    return entry !== null;
  });
  const found = entry as HistoryRow | null;
  if (!found) return { entry: null as HistoryRow | null, mutations: [] as HistoryRow[], more: false };
  const expanded = await expandEntry(found.id);
  const moreText = /^\s*\+?\d+\s+(more|weitere)\b/i;
  const more = renderer === "wgpu"
    ? (await mirror()).some((node) => /framework\.history\./.test(node.key) && !/framework\.history\.mutation\./.test(node.key) && (/(^|[./])more\b/i.test(node.key) || moreText.test(node.label)))
    : await evalSafe((source) => Array.from(document.querySelectorAll<HTMLElement>("[id]")).some((el) => /framework\.history\./.test(el.id) && !/framework\.history\.mutation\./.test(el.id) && (/(^|[./])more\b/i.test(el.id.split("/").pop() ?? "") || new RegExp(source, "i").test(el.innerText ?? ""))), false, moreText.source);
  return { entry: found as HistoryRow | null, mutations: expanded.mutations, more };
};

/** 🖌️ Begins a session on the first editable of `candidates` and drafts a real change: the manifest id (a text input), else
 * a created node's x (+10, a grid-snapped stepper), else Withdraw. */
const draftLongHistoryTarget = async (candidates: HistoryRow[]) => {
  const copy = COPY[currentLocale];
  const ordered = [...candidates.filter((row) => copy.manifest.test(row.label)), ...candidates.filter((row) => copy.createNodeRow.test(row.label)), ...candidates].filter((row, index, all) => all.findIndex((other) => other.key === row.key) === index);
  for (const target of ordered.slice(0, 3)) {
    const begun = await beginEditOf(target.key);
    if (begun.band?.stage !== "editing") {
      note("g9-candidate-did-not-open", { target: target.label, via: begun.via });
      continue;
    }
    if (copy.manifest.test(target.label)) return { target, kind: "manifest", draft: await typeEditorText("newManifestId", `probe-long-history-${stamp.slice(11, 19)}`) as Record<string, unknown> };
    if (copy.createNodeRow.test(target.label)) {
      const x = await readNumberControl("node.x");
      if (x?.valueNow !== null && x?.valueNow !== undefined) return { target, kind: "node-x", draft: { from: x.valueNow, ...(await typeEditorNumber("node.x", Math.round(x.valueNow + 10))) } as Record<string, unknown> };
    }
    return { target, kind: "withdraw", draft: (await pressAuthored("framework.history.editor.withdraw")) as Record<string, unknown> };
  }
  return null;
};

/** ⏱️ The band states an armed replay sampled, summarised: whether a `replaying` frame showed, the largest progress total, the
 * time from the arm to the review and the press the arm made. */
const replaySummary = (armLog: ArmLog | null) => {
  const samples = armLog?.samples ?? [];
  const replaying = samples.filter((sample) => sample.stage === "replaying");
  const totals = replaying.map((sample) => sample.total ?? 0);
  const reviewedAt = samples.find((sample, index) => index > 0 && sample.stage === "reviewing")?.t ?? null;
  return { sawReplaying: replaying.length > 0, frames: replaying.length, total: totals.length ? Math.max(...totals) : null, firstReplayingAt: replaying[0]?.t ?? null, reviewedAt, acted: armLog?.acted ?? null, samples: samples.slice(0, 40) };
};

/** 🗻️ Step 16 — G9 long history in a fresh document: the example load is one transaction of several hundred mutations. Edit its
 * first mutation (a real change), Accept — the replay shows progress over at least `--long-history` mutations and Cancel stops
 * it (review "Replay needed", Replay again offered); Replay again with Edit pressed on another mutation while it replays — the
 * press is refused (disabled, or the `timeTravel.illegal` notice) and the session keeps its target; the review completes; Exit
 * leaves zero trace. Presses land in the first frame that shows the replay (a person reacting to the progress bar). While no
 * frame shows a replay the history grows by one Set Active Example (palette) and the session is retried. */
const step16 = async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const opened = await open(DESKTOP, "long-history");
    verdict("g9-fresh-document-boots", opened !== null, {});
    if (!opened) return;
    const board = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
    const created = (board.value?.nodes ?? 0) + Math.max(0, board.value?.edges ?? 0);
    const tx = await exampleTransaction();
    verdict("g9-every-mutation-of-the-long-transaction-is-reachable", Boolean(tx.entry) && (tx.mutations.length >= created || tx.more), { entry: tx.entry?.label ?? null, listed: tx.mutations.length, createdAtLeast: created, more: tx.more, rows: tx.mutations.map((row) => row.label).slice(0, 10), reading: "acceptance item 1: every mutation is editable — the example's transaction holds one create-node per node and one connect-handles per edge, so the panel must list them all or offer a way to the rest (N1: `HISTORY_PANEL_MUTATION_ROWS` = 8, no \"more\" row)" });
    if (!tx.entry || !tx.mutations.length) return;
    const headBefore = await positions();
    let rowsBefore = documentEntries(await allHistoryRows()).map((row) => row.label);
    let session: Awaited<ReturnType<typeof draftLongHistoryTarget>> = null;
    let first: ReturnType<typeof replaySummary> | null = null;
    const rounds: Record<string, unknown>[] = [];
    for (let round = 0; round <= LONG_HISTORY_GROWTH_ROUNDS; round++) {
      const current = await exampleTransaction();
      session = await draftLongHistoryTarget(current.mutations);
      if (!session) break;
      await sleep(900);
      await armReplay({ kind: "cancel" });
      const via = await pressBand("accept");
      const settled = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 240000, 100);
      first = replaySummary(await disarmReplay());
      rounds.push({ round, target: session.target.label, kind: session.kind, via, review: settled.value?.review ?? null, ...first, samples: first.samples.slice(0, 12) });
      if (first.sawReplaying || round === LONG_HISTORY_GROWTH_ROUNDS) break;
      await exitSession();
      const examplesBefore = documentEntries(await allHistoryRows()).filter((row) => copy.example.test(row.label)).length;
      const grown = await runPaletteCommand(copy.exampleQuery, copy.example, "setActiveExample");
      const landed = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => copy.example.test(row.label)).length, (count) => count > examplesBefore, 120000, 2000);
      note("g9-history-grew-by-one-example-load", { round, run: grown, examples: [examplesBefore, landed.value], waitedMs: landed.waitedMs, nodes: (await vitals())?.nodes });
      rowsBefore = documentEntries(await allHistoryRows()).map((row) => row.label);
    }
    emit({ kind: "long-history", rounds });
    if (!session || !first) {
      verdict("g9-a-long-history-mutation-opens-for-editing", false, { rounds, reading: "no listed mutation of the example transaction opened a session" });
      return;
    }
    verdict("g9-the-replay-shows-progress-over-the-long-history", first.sawReplaying && (first.total ?? 0) >= longHistory, { total: first.total, frames: first.frames, firstReplayingAt: first.firstReplayingAt, reviewedAt: first.reviewedAt, minimum: longHistory, rounds: rounds.length, reading: "the band shows `replaying` with progress done/total (React `<progress>`, wgpu `shell.time-travel.status` aria-valuenow/max) over at least --long-history mutations" });
    const afterCancel = await band();
    const cancelLanded = first.acted?.via === "cancelReplay" && afterCancel?.review === "needsReplay";
    verdict("g9-cancel-stops-the-replay", cancelLanded && Boolean(afterCancel?.text.includes(copy.replayCancelled)), { acted: first.acted, review: afterCancel?.review, text: afterCancel?.text?.slice(0, 200), expected: copy.replayCancelled, reading: "design §4: ReplayCancelled → Reviewing with the cancelled fault, review needsReplay (\"Replay needed: later mutations are not checked yet\")" });
    const rerunControl = afterCancel?.controls.find((control) => control.control === "rerun");
    if (cancelLanded) verdict("g9-replay-again-is-offered-after-cancel", rerunControl?.disabled === false, { controls: afterCancel?.controls });
    await shot("g9-cancelled");
    const tx2 = await exampleTransaction();
    const other = tx2.mutations.find((row) => row.key !== session!.target.key) ?? null;
    let prepared = "";
    if (other && !cancelLanded) {
      const again = await beginEditOf(session.target.key);
      const redraft = session.kind === "manifest" ? await typeEditorText("newManifestId", `probe-long-history-${stamp.slice(11, 19)}-b`) : session.kind === "node-x" ? await typeEditorNumber("node.x", Math.round(Number((session.draft as { from?: number }).from ?? 0) + 20)) : { present: false, typed: null };
      await sleep(900);
      prepared = `begin ${again.band?.stage ?? "?"} → redraft ${JSON.stringify(redraft).slice(0, 80)} → `;
    }
    const otherRow = other ? await findMutationRow(other.key) : null;
    if (!otherRow) {
      verdict("g9-edit-during-replay-is-refused", false, { reason: "no second mutation row of the example transaction to press Edit on" });
    } else {
      await armReplay({ kind: "edit", rowId: otherRow.id, source: copy.edit.source, flags: copy.edit.flags });
      const started = cancelLanded ? `rerun via ${await pressBand("rerun")}` : `${prepared}accept via ${await pressBand("accept")}`;
      const completed = await waitUntil(band, (b) => b?.stage === "reviewing" && b.review !== null && b.review !== "needsReplay", 240000, 100);
      const second = replaySummary(await disarmReplay());
      const codes = await shownNoticeCodes();
      const editedAfter = (second.samples ?? []).some((sample) => sample.stage === "editing" && sample.t > (second.acted?.t ?? Number.POSITIVE_INFINITY));
      verdict("g9-edit-during-replay-is-refused", second.acted !== null && second.acted.via !== "absent" && (second.acted.disabled === true || codes.includes("timeTravel.illegal")) && !editedAfter, { started, acted: second.acted, codes, editedAfter, frames: second.frames, row: otherRow.label, reading: "design §4: Begin is legal only from Inactive and Reviewing — while Replaying the press is refused (`timeTravel.illegal`, a warning notice) or the Edit action is disabled (N15)" });
      verdict("g9-the-replay-completes-the-review", completed.ok && (completed.value?.review === "ready" || completed.value?.review === "blocked"), { started, review: completed.value?.review, text: completed.value?.text?.slice(0, 200), total: second.total, reviewedAt: second.reviewedAt });
    }
    await shot("g9-reviewed");
    const exited = await exitSession();
    const zero = await waitUntil(positions, (p) => movedIds(headBefore, p, 1e-6).length === 0, 20000, 500);
    const rowsAfter = documentEntries(await allHistoryRows()).map((row) => row.label);
    verdict("g9-exit-leaves-zero-trace", exited.ok && rowsAfter.length === rowsBefore.length && (rounds.length > 1 || zero.ok), { band: exited.value?.stage ?? null, drift: movedIds(headBefore, zero.value, 1e-6).length, rows: [rowsBefore.length, rowsAfter.length], grown: rounds.length > 1, reading: "Exit discards every accepted draft: no new row; the head is unchanged unless the history grew (then only the rows are compared)" });
  });
//#endregion 🔖️LongHistory

//#region 🔖️Peers
/** 🎫️ The mutation row of the newest document row whose label starts with `prefix` (paged, expanded), or null. */
const mutationLabelled = async (prefix: string) => {
  const entry = documentEntries(await allHistoryRows()).filter((row) => row.label.startsWith(prefix)).sort((left, right) => Number(right.key) - Number(left.key))[0];
  if (!entry) return null;
  return (await expandEntry(entry.id)).mutations.find((row) => row.label.startsWith(prefix)) ?? null;
};

/** 🤝️ Step 17 — G10 two peers on one local folder: peer A (a fresh context) attaches a fresh folder and drags one node — the
 * archive lands in the folder; peer B (another fresh context) attaches the same folder and reads A's document; A begins a
 * history edit of its drag; B drags another node, which reaches A over the folder's change stream (`backbone.folder`) as a base
 * move — A's session keeps editing, the remote drag is listed downstream and not applied; A changes dx, Accept replays B's drag
 * too, Finalize overwrite reaches B. Presence (the roster's ⏪ badge, "is editing") travels only through a hub's presence
 * frames, never through a folder binding, so it is recorded as a note. */
const step17 = async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const folder = join(OUT, `folder-${stamp}-${currentLocale}-peers`);
    mkdirSync(folder, { recursive: true });
    const a = await open(DESKTOP, "peer-a");
    verdict("g10-first-peer-boots", a !== null, {});
    if (!a) return;
    await closePanels();
    const attachedA = await attachFolder(folder);
    if (!attachedA.card) {
      note("g10-no-folder-route-on-this-build", { attachedA, reading: "two peers need a shared persisted route: this build offers no folder card" });
      return;
    }
    verdict("g10-first-peer-attaches-the-shared-folder", attachedA.typed === folder && attachedA.attachButton, { attachedA, folder });
    await closePanels();
    await frameBoard(6);
    const { camera, picked } = await pickNodes(2, [[60, 30], [-50, 40]], [], 90);
    if (picked.length < 2) {
      verdict("g10-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
      return;
    }
    const [n1, n2] = [picked[0].id, picked[1].id];
    await selectNodes([picked[0]]);
    const p0 = await positions();
    await dragBy(picked[0].at, 60 * camera.zoom, 30 * camera.zoom);
    const movedA = await waitUntil(positions, (p) => offsetOf(p0, p, n1)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    const aOff = offsetOf(p0, movedA.value, n1);
    if (!aOff) {
      verdict("g10-first-peer-drag-moves-the-node", false, { n1 });
      return;
    }
    const aRow = await mutationLabelled(dragLabel(1, aOff[0], aOff[1]));
    const files = () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => entry.name);
    const written = await waitUntil(async () => files(), (list) => list.length > 0, 30000, 1000);
    verdict("g10-first-peer-edit-is-written-to-the-folder", Boolean(aRow) && written.ok, { aOff, row: aRow?.label ?? null, files: written.value.slice(0, 6) });
    if (!aRow) return;
    const headA = await positions();
    const b = await open(DESKTOP, "peer-b");
    verdict("g10-second-peer-boots", b !== null, {});
    if (!b) return;
    await closePanels();
    const attachedB = await attachFolder(folder);
    const shared = await waitUntil(positions, (p) => Object.keys(p).length > 0 && movedIds(headA, p, 1e-6).length === 0, 45000, 1000);
    const bSeesA = await mutationLabelled(dragLabel(1, aOff[0], aOff[1]));
    verdict("g10-second-peer-opens-the-shared-document", attachedB.typed === folder && shared.ok && Boolean(bSeesA), { attachedB, drift: movedIds(headA, shared.value, 1e-6).slice(0, 6), driftCount: movedIds(headA, shared.value, 1e-6).length, row: bSeesA?.label ?? null, reading: "attaching a folder that holds an archive reads it (`documentArchiveReplaced`): B now shows A's document and A's drag row" });
    use(a);
    await installBandTrace();
    const begun = await beginEditOf(aRow.key);
    verdict("g10-first-peer-begins-a-history-edit", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
    if (begun.band?.stage !== "editing") return;
    note("g10-presence-travels-only-through-a-hub", { a: await presenceText(), reading: "the roster's ⏪ badge and the \"is editing\" notes come from presence frames (presence bit 13), which only a hub socket carries (`🏪️store/👷️worker` `presenceAuthority` is set from hub frames); a folder binding carries none — the live two-peer presence case needs a hub-backed space serve (`🚀️local-hub` + `?space=`)" });
    use(b);
    await closePanels();
    await frameBoard(4);
    const n2At = (await rowsOf([n2]))[0];
    if (!n2At) {
      verdict("g10-second-peer-drags-another-node", false, { n2 });
      return;
    }
    await selectNodes([n2At]);
    const beforeB = await positions();
    const zoomB = cameraOf(await vitals()).zoom;
    await dragBy(n2At.at, -50 * zoomB, 40 * zoomB);
    const movedB = await waitUntil(positions, (p) => offsetOf(beforeB, p, n2)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    const bOff = offsetOf(beforeB, movedB.value, n2);
    verdict("g10-second-peer-drags-another-node", Boolean(bOff), { n2, bOff });
    if (!bOff) return;
    use(a);
    const bLabel = dragLabel(1, bOff[0], bOff[1]);
    const arrived = await waitUntil(() => mutationLabelled(bLabel), (row) => row !== null, 45000, 1500);
    verdict("g10-a-remote-edit-arrives-while-editing", arrived.ok, { expected: bLabel, row: arrived.value?.text?.slice(0, 160) ?? null, waitedMs: arrived.waitedMs, reading: "B's write reaches A over the folder's change stream; A's store takes the new history while the session edits" });
    const during = await band();
    verdict("g10-the-session-survives-the-base-move", during?.stage === "editing" && Boolean((during.target ?? during.text).includes(aRow.label)), { stage: during?.stage, target: during?.target, text: during?.text?.slice(0, 160), reading: "design §7: remote ingests keep arriving while frozen → BaseMoved; in Editing the preview is re-shown, the draft stays" });
    const previewA = await positions();
    verdict("g10-the-remote-edit-stays-downstream-and-unapplied-while-editing", placed(previewA, n2, p0[n2], 0, 0, 0.05) && Boolean(arrived.value?.text.includes(copy.pending)), { n2: previewA[n2], n2Before: p0[n2], row: arrived.value?.text?.slice(0, 160) ?? null, expected: copy.pending });
    const dxNew = Math.round(aOff[0] + 20);
    const typed = await typeEditorNumber("dx", dxNew);
    await pressBand("accept");
    const review = await waitUntil(band, (b2) => b2?.stage === "reviewing" && Boolean(b2.review), 60000, 100);
    const replayed = await waitUntil(positions, (p) => placed(p, n1, p0[n1], dxNew, aOff[1], 0.05) && placed(p, n2, p0[n2], bOff[0], bOff[1], 0.05), 20000);
    verdict("g10-accept-replays-the-remote-edit-too", review.value?.review === "ready" && replayed.ok, { typed, review: review.value?.review, n1: replayed.value[n1], n2: replayed.value[n2], expected: { n1: [p0[n1][0] + dxNew, p0[n1][1] + aOff[1]], n2: [p0[n2][0] + bOff[0], p0[n2][1] + bOff[1]] } });
    if (review.value?.review !== "ready") {
      await exitSession();
      return;
    }
    const finalized = await finalizeOverwrite();
    verdict("g10-finalize-closes-the-session", finalized.closed, finalized);
    use(b);
    const converged = await waitUntil(positions, (p) => placed(p, n1, p0[n1], dxNew, aOff[1], 0.05) && placed(p, n2, p0[n2], bOff[0], bOff[1], 0.05), 45000, 1000);
    const overwriteRow = documentEntries(await allHistoryRows()).find((row) => row.label.startsWith(copy.row("edit", null, 1)));
    verdict("g10-the-second-peer-sees-the-finalized-edit", converged.ok && Boolean(overwriteRow), { n1: converged.value[n1], n2: converged.value[n2], row: overwriteRow?.label ?? null, waitedMs: converged.waitedMs });
    await shot("g10-peers");
  });
//#endregion 🔖️Peers
//#endregion 🔖️Gaps

const STEPS: Record<number, (ctx: Ctx) => Promise<void>> = { 1: step1, 2: step2, 3: step3, 4: step4, 5: step5, 6: step6, 7: step7, 8: step8, 9: step9, 10: step10, 11: step11, 12: step12, 13: step13, 14: step14, 15: step15, 16: step16, 17: step17 };
//#endregion 🔖️Steps

//#region 🔖️Main
/** 🌎️ One locale pass: a fresh desktop context whose `navigator.language` selects the UI locale, one tab, every step of
 * {@link ORDER} (the reload check before the fresh-context steps), or the `--explore` inventory. Ctrl-C stops after the running
 * step. */
const runLocale = async (locale: Locale) => {
  currentLocale = locale;
  currentStep = 0;
  navigations = 0;
  expectedReloads = 0;
  log(`navigating to ${serveOrigin} renderer=${renderer} locale=${locale}`);
  const booted = await openPage(DESKTOP, "boot", 100);
  page.on("framenavigated", (frame) => {
    if (frame !== page.mainFrame()) return;
    navigations += 1;
    log(`main frame navigated (#${navigations}) → ${frame.url().slice(0, 120)}`);
  });
  mod = (await page.evaluate(() => navigator.platform).catch(() => "MacIntel")).includes("Mac") ? "Meta" : "Control";
  currentStep = 1;
  if (!verdict("boot", booted, { mod })) {
    dumpJson("boot-failure", { inventory: await historyInventory(), console: consoleRows.filter((row) => row.locale === locale).slice(-80) });
    await shot("boot-failure");
    await context.close().catch(() => {});
    return;
  }
  if (explore && renderer === "wgpu") {
    const structure = await evalSafe(async () => (await (window as unknown as { semioWgpuIntrospection?: { dumpStructure: () => Promise<string> } }).semioWgpuIntrospection?.dumpStructure()) ?? "", "");
    const before = await mirror();
    await openHistory();
    const history = await mirror();
    await attachFolder(join(OUT, `folder-explore-${stamp}-${locale}`));
    const afterSync = await mirror();
    await closePanels();
    await openTab(INSPECTION_TAB);
    const inspection = await mirror();
    await closePanels();
    await prepareChord();
    await page.keyboard.press(`${mod}+p`).catch(() => {});
    await sleep(1500);
    const searchInput = mirrorFind(await mirror(), "ui.search.input");
    if (searchInput) await (await mirrorLocator(searchInput.key, searchInput.window))?.fill(locale === "de" ? "Drehen" : "Rotate", { force: true, timeout: 4000 }).catch(() => {});
    await sleep(1200);
    const palette = await mirror();
    await shot("explore-palette");
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    const line = (node: MirrorNode) => `${node.window}|${node.key}|${node.role}|${node.label.slice(0, 60)}${node.description ? ` — ${node.description.slice(0, 40)}` : ""}`;
    const windowIds = (() => {
      try {
        return (JSON.parse(structure || "{}") as { windowIds?: string[] }).windowIds;
      } catch {
        return null;
      }
    })();
    dumpJson("explore", { windowIds, board: await wgpuBoard(), boardContract: WGPU_BOARD_CONTRACT, chrome: await chromeHits(), mirrorBefore: before.map(line), mirrorHistory: history.map(line), mirrorAfterSync: afterSync.map(line), mirrorInspection: inspection.map(line), mirrorPalette: palette.map(line), band: await band(), presence: await presenceText(), aria: await ariaFindings(ariaRoots("session")) });
    await shot("explore");
    await context.close().catch(() => {});
    return;
  }
  if (explore) {
    await openHistory();
    const syncIds = () => evalSafe(() => Array.from(document.querySelectorAll("[id]")).filter((el) => /sync/i.test(el.id)).map((el) => `${el.tagName.toLowerCase()}#${el.id}[${el.getAttribute("data-slot") ?? el.getAttribute("role") ?? ""}] vis=${(el as HTMLElement).offsetParent !== null} ${(el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 60) ?? ""}`), [] as string[]);
    const syncBefore = await syncIds();
    await page.locator('[id="s-sync-status"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(1000);
    const syncOpen = await syncIds();
    const picked = await pickSyncFolder();
    log(`sync folder pick: ${picked}`);
    await sleep(1000);
    const syncFolder = await syncIds();
    const popovers = await evalSafe(() => Array.from(document.querySelectorAll("[data-radix-popper-content-wrapper], [role=dialog]")).map((el) => (el as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 200)), [] as string[]);
    await shot("explore-sync");
    await page.keyboard.press("Escape").catch(() => {});
    dumpJson("explore", { syncBefore, syncOpen, syncFolder, popovers, inventory: await historyInventory(), rows: await readHistory(), band: await band(), vitals: await vitals(), aria: await ariaFindings(ariaRoots("session")), tabs: await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((el) => `${el.id}:${(el as HTMLElement).innerText.trim()}`), [] as string[]) });
    await shot("explore");
    await context.close().catch(() => {});
    return;
  }
  const ctx: Ctx = {};
  const boardBlocked = renderer === "wgpu" && (await wgpuBoard()) === null;
  if (boardBlocked) verdict("wgpu-board-introspection-present", false, { prerequisite: WGPU_BOARD_CONTRACT, reading: "the wgpu Board2d surface projects no accessibility nodes and no dump carries its positions, camera or selection, so no pointer gesture can be aimed at a node; every board step is blocked until the export exists" });
  let reloaded = false;
  for (const step of ORDER) {
    if (cancelled.aborted) {
      note("cancelled-before-the-step", { step });
      break;
    }
    if ((step === 14 || step === 9) && !reloaded && reloadSelected && !boardBlocked) {
      reloaded = true;
      currentStep = 5;
      try {
        await reloadCheck(ctx);
      } catch (error) {
        verdict("reload-check-threw", false, { error: String(error).slice(0, 400) });
      }
    }
    if (only && !only.has(step)) continue;
    if (boardBlocked && step !== 1 && step !== 9) {
      currentStep = step;
      verdict("blocked-by-missing-board-introspection", false, { prerequisite: "semioWgpuIntrospection.dumpBoard2d" });
      continue;
    }
    currentStep = step;
    const navigationsBefore = navigations;
    const reloadsBefore = expectedReloads;
    log(`step ${step} begins`);
    try {
      await STEPS[step](ctx);
    } catch (error) {
      verdict("step-threw", false, { error: String(error).slice(0, 400) });
    }
    if (navigations - navigationsBefore > expectedReloads - reloadsBefore) note("page-reloaded-under-the-step", { navigations: navigations - navigationsBefore, reading: "the dev serve reloaded the page (Vite reconnect / supervisor recycle); verdicts after that point measure a fresh document" });
    const results = verdicts.filter((row) => row.locale === locale && row.step === step);
    if (results.some((row) => !row.ok)) dumpJson("failure", { inventory: await historyInventory(), band: await band(), editor: await editor(), dialog: await finalizeDialog(), vitals: await vitals(), console: consoleRows.filter((row) => row.locale === locale && row.step === step).slice(-60) });
    await shot("end");
  }
  if (reloadSelected && !reloaded && !boardBlocked && !cancelled.aborted) {
    currentStep = 5;
    await reloadCheck(ctx);
  }
  emit({ kind: "context", ctx });
  await context.close().catch(() => {});
};

type RunSummary = { verdicts: number; passed: number; failed: number; uncaught: number; hard: number; failing: string[]; mdPath: string };

/** 🎪️ The whole run: one headless Chromium (the repository's pinned browser build), every locale, the markdown report;
 * answers the counts the acceptance record carries. */
const runProbe = async (): Promise<RunSummary> => {
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--enable-features=Vulkan,UseSkiaRenderer"] });
  try {
    for (const locale of locales) {
      if (cancelled.aborted) break;
      await runLocale(locale);
    }
  } finally {
    await browser.close().catch(() => {});
  }
  const stepLines: string[] = [];
  for (const locale of locales) {
    for (const step of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]) {
      const rows = verdicts.filter((row) => row.locale === locale && row.step === step);
      if (!rows.length) continue;
      const failed = rows.filter((row) => !row.ok);
      stepLines.push(`STEP ${locale}/${step} ${failed.length ? "FAIL" : "PASS"} (${rows.length - failed.length}/${rows.length})${failed.length ? ` — ${failed.map((row) => row.name).join(", ")}` : ""}`);
    }
  }
  const passed = verdicts.filter((row) => row.ok).length;
  const summary = `time-travel probe renderer=${renderer} PASS=${passed} FAIL=${verdicts.length - passed} uncaught=${pageErrors.length} hard=${hardFaults.length} locales=${locales.join(",")} chords=${[...chordLocales].join(",")} only=${only ? `${[...only].join(",")}${reloadSelected ? ",reload" : ""}` : "all"}${cancelled.aborted ? " cancelled" : ""}`;
  console.log(summary);
  for (const line of stepLines) console.log(line);
  emit({ kind: "summary", summary, steps: stepLines });
  const mdPath = join(OUT, `${base}.md`);
  writeFileSync(
    mdPath,
    [
      `# ${base} (renderer ${renderer}, serve ${serveOrigin}, locales ${locales.join(",")}, chords ${[...chordLocales].join(",")}, only ${only ? `${[...only].join(",")}${reloadSelected ? ",reload" : ""}` : "all"}, long history ≥ ${longHistory})`,
      "",
      summary,
      "",
      "## Steps",
      ...stepLines,
      "",
      "## Verdicts",
      ...verdicts.map((row) => `- ${row.ok ? "PASS" : "FAIL"} ${row.locale}/${row.step} ${row.name} ${JSON.stringify(row.detail).slice(0, 700)}`),
      "",
      "## Notes",
      ...(notes.length ? notes.map((row) => `- ${row}`) : ["(none)"]),
      "",
      "## Uncaught page errors",
      ...(pageErrors.length ? pageErrors.map((row) => `- ${row.locale}/${row.step} ${row.text.slice(0, 600)}`) : ["(none)"]),
      "",
      "## Hard faults",
      ...(hardFaults.length ? hardFaults.slice(0, 40).map((row) => `- ${row.locale}/${row.step} ${row.text}`) : ["(none)"]),
      "",
      "## Timeline",
      ...timeline,
      "",
      "## Console (errors and warnings, last 200)",
      ...consoleRows.filter((row) => row.type === "error" || row.type === "warning" || row.type === "pageerror" || row.type === "http").slice(-200).map((row) => `${row.t}s ${row.locale}/${row.step} ${row.source} ${row.type}: ${row.text.slice(0, 400)}`),
    ].join("\n"),
  );
  return { verdicts: verdicts.length, passed, failed: verdicts.length - passed, uncaught: pageErrors.length, hard: hardFaults.length, failing: [...new Set(verdicts.filter((row) => !row.ok).map((row) => `${row.locale}/${row.step} ${row.name}`))], mdPath };
};

/** 🚦️ `verify time-travel [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,…]
 * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--explore]` — reuses the serve answering at `--serve` (default
 * :6012 React, :6112 wgpu) or starts the puzzle 2d serve of `--renderer` there for the run ({@link ensureDevServe}; stopped
 * after it), drives every locale, writes the report under `--out`, publishes the `time-travel` acceptance record (pass only
 * when every verdict passes with no uncaught page error and no hard guest fault) and exits non-zero otherwise. Ctrl-C stops
 * after the running step and still writes the report. */
export async function runTimeTravelCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  configure(segments, defaultOutDir, controller.signal);
  const startedAt = new Date();
  const serveUrl = `${serveOrigin}/`;
  await withAcceptanceRecord(
    repoRoot,
    CHECK_ID,
    async () => {
      const serve = await ensureDevServe({ repoRoot, port: devServePortV1(serveUrl), variant: PLUGIN_VARIANT, renderer, signal: controller.signal, onProgress: (_status, line) => console.log(line) }).catch((error: unknown) => (error instanceof Error ? error : new Error(String(error))));
      if (serve instanceof Error) {
        const reason = serve.message.split("\n")[0]!.slice(0, 200);
        publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({ check: CHECK_ID, status: "blocked", startedAt, measured: { serve: serveUrl, renderer, cancelled: controller.signal.aborted }, summary: { en: `no ${renderer} puzzle 2d serve at ${serveUrl}: ${reason}`, de: `kein ${renderer}-Puzzle-2D-Server unter ${serveUrl}: ${reason}` } }));
        process.exitCode = 1;
        return;
      }
      try {
        const run = await runProbe();
        const status = run.verdicts > 0 && run.failed === 0 && run.uncaught === 0 && run.hard === 0 && !controller.signal.aborted ? "pass" : "fail";
        const failing = run.failing.slice(0, 8).join(", ");
        publishAcceptanceCheckResult(
          repoRoot,
          acceptanceCheckResult({
            check: CHECK_ID,
            status,
            startedAt,
            measured: { renderer, serve: serveUrl, locales: locales.join(","), chords: [...chordLocales].join(","), verdicts: run.verdicts, passed: run.passed, failed: run.failed, uncaught: run.uncaught, hard: run.hard, cancelled: controller.signal.aborted },
            summary: {
              en: `${run.passed}/${run.verdicts} time-travel verdicts pass on ${renderer} (${locales.join(", ")}); ${run.uncaught} uncaught page errors, ${run.hard} hard faults${failing ? `; failing: ${failing}` : ""}`.slice(0, 1900),
              de: `${run.passed}/${run.verdicts} Zeitreise-Prüfungen bestehen auf ${renderer} (${locales.join(", ")}); ${run.uncaught} unbehandelte Seitenfehler, ${run.hard} harte Fehler${failing ? `; fehlgeschlagen: ${failing}` : ""}`.slice(0, 1900),
            },
            evidence: [run.mdPath, ndjsonPath],
          }),
        );
        if (status !== "pass") process.exitCode = 1;
      } finally {
        await serve.stop();
      }
    },
    (error) => /ECONNREFUSED|ERR_CONNECTION_REFUSED/u.test(String(error)),
  );
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}
//#endregion 🔖️Main

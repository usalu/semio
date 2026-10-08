/** ⏪️ `verify time-travel` — the end-to-end gate of non-destructive history editing, driven the way a person drives it in the
 * puzzle 2d dev serve (the reference app of the history-editing design, React `serve puzzle2d react dev` on :6012 or the wgpu
 * browser shell on :6112). One headless Chromium, one tab at a time, one fresh browser context per UI locale (`en`, then
 * `de`, chosen through `navigator.language`), numbered steps per locale (promoted from the ticket probe of 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING, work package W3-E2E):
 *
 * 1. boot: `data-board-snapshot-parsed` first, then a board with nodes and the locale of the history panel; then the
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
 * 9. console: uncaught page errors, hard faults and lines with AGENTS.md's temporary-log tag fail the run; errors and warnings are digested;
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
 * 16. long history (G9, N1, N15): a fresh document whose example load is one transaction of several hundred mutations — its
 *     row is a tree window whose `total` (React `data-tree-window-total`) counts every mutation and whose LAST one opens for
 *     editing; edit its first mutation, Accept, replay progress over at least `--long-history` mutations, Cancel (review
 *     "Replay needed"), Replay again with an Edit pressed while it replays (disabled, naming why; the session keeps its
 *     target), Exit with zero trace; the history grows through the palette's Set Active Example until the replay is observable;
 * 17. two peers on one local folder (G10): two fresh contexts attached to the same folder — the second reads the first's
 *     document, the first begins a history edit, the second's drag arrives as a base move (the session survives, the remote
 *     edit stays downstream and unapplied), Accept replays it too, Finalize reaches the second peer, whose history body shows
 *     what it replays (`HistoryPatch.reprojection`). Presence (the ⏪ roster badge, "is editing") travels only through a hub
 *     and is recorded as such;
 * 18. history steps and the stepped document load (N17, §20.8): a fresh document, bound to a fresh folder, grown by a second
 *     example load, its first mutation finalized as a new alternative; switching back to the main line is a deferred history
 *     step — the body's `framework.history.reprojection` section ("History step", "Replaying history: d of t mutations") —
 *     during which Undo is refused (`history.replaying`) and Cancel replay drops the step with zero trace; switching again
 *     completes it. A second fresh page attaching that folder loads the archive stepwise ("Document load", "Loading
 *     document: d of t"), refuses a command meanwhile (`document.loading`) and Cancel replay keeps its previous document;
 * 19. configuration is not history (L4, §20.13), windows count rows (N1), list inputs (N2), transaction labels (§19.1): camera
 *     zoom and pan add no history row; a drag adds exactly one (the commands window `total` grows by one) labelled by its
 *     intent leaf; Undo after a camera move takes back the drag and leaves the camera; Edit of a duplicated node's
 *     `create-node` → Add item / Remove item on its handles list (the row names its item count) → Exit with zero trace;
 * 20. row actions (design §22.1, §22.20): every mutation row offers Edit and Withdraw; Withdraw pressed on a row opens (or
 *     stacks) a session with a withdrawn draft — a duplicated node's `create-node` withdrawn from its row leaves its drag
 *     blocking, the blocking drag is withdrawn from ITS row (no editor needed) → review `ready`; a row holding an accepted
 *     draft offers Restore in place of Withdraw, and Restore drops that draft; Exit leaves zero trace;
 * 21. the goal's own sentence as one verdict set (puzzle 2d: dragging the selection yields ONE drag mutation whose selection AND
 *     offset stay editable): one drag of two nodes = one row; Edit → the selection as a reference list (Use selection, a chip
 *     with Remove per target) and dx / dy as steppers stepping by the grid snap; the offset is stepped, a target removed, the
 *     targets replaced by the board selection — the preview follows each — and Accept re-applies the downstream drag; Exit.
 *
 * Step 3 also asserts the G13 transitions (`history-panel-reveals-on-session-start`, `focus-moves-to-the-editor`), step 4
 * focus on the band and the N15 refusal of Edit on another mutation while a changed draft is open (disabled, its reason named
 * and shown), step 5 focus in the prompt and Edit refused while choosing; steps 3, 5, 14 and 15 run the structural ARIA oracle (`aria-query`'s WAI-ARIA
 * role model + `dom-accessibility-api`'s accessible names, bundled into the page) over the band, the history panel and the
 * prompt (wgpu: over its ARIA mirror). The reload check (G5) runs after step 13 and is attributed to step 5: every pre-reload
 * row with its localized label, the alternatives (main line + alternatives) and the current alternative.
 *
 * Every control is the real one, reached the way a user reaches it: the windowed history body is scrolled until the
 * section holds the row, steppers take keyboard input, arg-carrying commands run from the palette (`mod+p`) through the
 * command panel's staged form. Counts come from the windows' own `total` (the rows a window holds, materialised or not),
 * never from the rows the DOM happens to hold (W1E-6); the wgpu mirror publishes no window size, so wgpu pages until the
 * expected rows were all seen.
 *
 * `--renderer wgpu` drives the puzzle 2d wgpu shell (default serve :6112) through the same steps: rows, buttons, inputs and
 * the dialog through its ARIA mirror (`#semio-wgpu-accessibility`), chrome through `dumpChrome` hit rects, the board with the
 * pointer and keyboard on the canvas at the positions `semioWgpuIntrospection.dumpBoard2d` publishes (region `🔖️Wgpu`).
 *
 * Run: `verify time-travel [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,3]
 * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--universal] [--variant <declared variant>] [--explore]` — the serve is reused when it answers, else
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
import { appendFileSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, BrowserContext, ConsoleMessage, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER, playgroundCatalog } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { devServePortV1, ensureDevServe } from "../../🚀️local-hub/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

import { timeTravelServeVariantV1 } from "./🚀️boot/🟦️.ts";
import { universalMirrorControlsV1, universalMirrorVerbsV1, type UniversalControlV1, type UniversalVerbV1 } from "./🪞️inventory/🟦️.ts";

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
const ORDER = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 19, 20, 21, 22, 23, 14, 15, 16, 17, 18, 24, 9] as const;
const STEP_NUMBERS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24] as const;
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
  wgpuSeedTerminology = segments.includes("--wgpu-seed-terminology");
  universalRoute = segments.includes("--universal") ? (flagValue(segments, "--serve") ?? DEFAULT_SERVES[renderer]) : null;
  if (universalRoute !== null && !steps) only = new Set([24, 9]);
  universalEditor = flagValue(segments, "--editor") ?? null;
  lastBootFault = "";
  folderAt = Number(flagValue(segments, "--folder-at") ?? "1");
  longHistory = Math.max(1, Number(flagValue(segments, "--long-history") ?? "200") || 200);
  OUT = resolve(flagValue(segments, "--out") ?? defaultOutDir);
  mkdirSync(OUT, { recursive: true });
  stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
  base = `probe-${renderer}-${stamp}`;
  ndjsonPath = join(OUT, `${base}.ndjson`);
  writeFileSync(ndjsonPath, "");
  t0 = Date.now();
  invalidatedBy = null;
  cancelled = signal;
  for (const rows of [verdicts, notes, timeline, consoleRows, pageErrors, hardFaults, harvestedNotices, harvestedBandFaults] as unknown[][]) rows.length = 0;
};

/** 🌐️ The puzzle 2d playground route of the serve under test. */
const routeUrl = () => universalRoute ?? `${serveOrigin}/?plugin=${PLUGIN_VARIANT}`;
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
    edit: /^(Edit)(:|$)/,
    withdraw: /^(Withdraw)(:|$)/,
    restore: /^(Restore)(:|$)/,
    refusalNotEditable: "The inputs of this mutation cannot be edited",
    refusalReadOnly: "History cannot be edited in a read-only view",
    switchAction: /^(Switch)(:|$)/i,
    refusalIllegal: "Not possible right now",
    refusalBlocked: "Blocked: resolve the pending change or the errors first",
    stepTitle: "History step",
    stepProgress: /Replaying history: (\d+) of (\d+) mutations/,
    loadTitle: "Document load",
    loadProgress: /Loading document: (\d+) of (\d+)/,
    remoteTitle: "Remote history change",
    remoteProgress: /Replaying a remote history change: (\d+) of (\d+) mutations/,
    remotePaused: "Remote history change paused",
    itemCount: (count: number) => `Items: ${count}`,
    noticeReplaying: "History is still replaying — wait for it or cancel it first.",
    noticeLoading: "The document is still loading — wait for it or cancel it first.",
    historyFull: /^This document's history is full \((\d+) edits\)\.$/,
    replayFaulted: "Replay failed: later mutations could not be checked",
    channelMismatch: /^This plugin was built for app channel \d+, but this app speaks app channel \d+ — rebuild the plugin\.$/,
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
    drifted: "Precondition drifted",
    overwriteTwo: "History edited — overwrite: 2 mutations",
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
    edit: /^(Bearbeiten)(:|$)/,
    withdraw: /^(Zurückziehen)(:|$)/,
    restore: /^(Wiederherstellen)(:|$)/,
    refusalNotEditable: "Die Eingaben dieser Mutation können nicht bearbeitet werden",
    refusalReadOnly: "Der Verlauf kann in einer schreibgeschützten Ansicht nicht bearbeitet werden",
    switchAction: /^(Wechseln)(:|$)/i,
    refusalIllegal: "Derzeit nicht möglich",
    refusalBlocked: "Blockiert: zuerst die offene Änderung oder die Fehler auflösen",
    stepTitle: "Verlaufsschritt",
    stepProgress: /Verlauf wird neu angewendet: (\d+) von (\d+) Mutationen/,
    loadTitle: "Dokument laden",
    loadProgress: /Dokument wird geladen: (\d+) von (\d+)/,
    remoteTitle: "Entfernte Verlaufsänderung",
    remoteProgress: /Entfernte Verlaufsänderung wird angewendet: (\d+) von (\d+) Mutationen/,
    remotePaused: "Entfernte Verlaufsänderung pausiert",
    itemCount: (count: number) => `Elemente: ${count}`,
    noticeReplaying: "Der Verlauf wird noch neu angewendet — abwarten oder zuerst abbrechen.",
    noticeLoading: "Das Dokument wird noch geladen — abwarten oder zuerst abbrechen.",
    historyFull: /^Der Verlauf dieses Dokuments ist voll \((\d+) Bearbeitungen\)\.$/,
    replayFaulted: "Erneutes Anwenden fehlgeschlagen: Spätere Mutationen konnten nicht geprüft werden",
    channelMismatch: /^Dieses Plugin wurde für App-Kanal \d+ gebaut, diese App spricht aber App-Kanal \d+ — Plugin neu bauen\.$/,
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
    drifted: "Vorbedingung nicht mehr erfüllt",
    overwriteTwo: "Verlauf bearbeitet — überschrieben: 2 Mutationen",
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
 * own listener, so no line is counted twice. The dev serve's HMR socket is read too: its `full-reload` and `error` frames are kept
 * (type `vite`), so a navigation under a step can be attributed ({@link navigationCause}). */
const listen = (target: Page) => {
  target.on("console", (message) => {
    if (message.worker?.()) return;
    keepConsole("page", message);
  });
  target.on("worker", (worker) => worker.on("console", (message) => keepConsole("worker", message)));
  target.on("websocket", (socket) =>
    socket.on("framereceived", (frame) => {
      const text = typeof frame.payload === "string" ? frame.payload : "";
      if (!/"type":"(full-reload|error)"/u.test(text)) return;
      consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "vite", text: text.slice(0, 600) });
    }),
  );
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
/** 🔓️ `text` with its percent-escapes decoded (stack frames carry `@fs` URLs of emoji paths); unchanged when it is not decodable. */
const readableUrlText = (text: string) => {
  try {
    return decodeURIComponent(text);
  } catch {
    return text;
  }
};

/** 🛰️ Why the page just navigated: the dev serve told it to (a `full-reload` frame on the HMR socket within the last 3 s — a served
 * file was saved; its `triggeredBy` / `path` names the file) or nothing did, in which case the PRODUCT reloaded its own page. */
const navigationCause = () => {
  const now = (Date.now() - t0) / 1000;
  const frame = [...consoleRows].reverse().find((row) => row.type === "vite" && row.text.includes("full-reload") && now - row.t <= 3);
  return frame ? `dev serve full-reload ${frame.text.slice(0, 300)}` : "no dev-serve reload frame in the 3 s before it: the page reloaded itself";
};
//#endregion 🔖️Console

//#region 🔖️Page
let browser: Browser;
let context: BrowserContext;
let page: Page;
let mod = "Meta";
let navigations = 0;
let expectedReloads = 0;
let probeReloadAt = 0;
let invalidatedBy: string | null = null;
let wgpuSeedTerminology = false;
let universalRoute: string | null = null;
let universalEditor: string | null = null;
let lastBootFault = "";

/** 🛟️ One `page.evaluate` that never takes the run down (a reload race answers `fallback`). */
const evalSafe = async <T, A = undefined>(fn: (arg: A) => T | Promise<T>, fallback: T, arg?: A): Promise<T> => {
  const began = Date.now();
  try {
    return (await page.evaluate(fn as (value: unknown) => T | Promise<T>, arg as unknown)) as T;
  } catch {
    return fallback;
  } finally {
    if (Date.now() - began > 5000) log(`slow page read: ${Date.now() - began} ms in ${String(fn).replace(/\s+/g, " ").slice(0, 140)}`);
  }
};
const sleep = (ms: number) => page.waitForTimeout(ms);

/** ⏳️ Polls `read` until `settled` holds or `timeoutMs` passes; answers the last value and the wait. */
const waitUntil = async <T,>(read: () => Promise<T>, settled: (value: T) => boolean, timeoutMs = 20000, everyMs = 250) => {
  const start = Date.now();
  let value = await read();
  while (!settled(value) && Date.now() - start < timeoutMs && invalidatedBy === null) {
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

/** 🩺️ One diagnosis of the live page under `tag`: a screenshot plus the band, the band trace, the editor, the open panels, the
 * history inventory and rows, the shown notices and the step's last console lines — taken where a verdict fails for a reason the
 * verdict's own detail cannot tell. */
const diagnose = async (tag: string) => {
  await shot(tag);
  return dumpJson(tag, { band: await band(), trace: (await bandTrace()).slice(-30), editor: await editor(), panels: await openPanelTabIds(), commands: await treeWindow("framework.history.commands"), inventory: (await historyInventory()).filter((row) => !/entry\.|mutation\./.test(String(row.id))), rows: (await readHistory()).slice(0, 60), notices: await pageNotices(), vitals: await vitals(), console: consoleRows.filter((row) => row.locale === currentLocale && row.step === currentStep).slice(-80) });
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
  if (renderer === "wgpu") await page.addInitScript((seed) => {
    try {
      globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
      if (seed && !globalThis.localStorage?.getItem("semio.os.config")) globalThis.localStorage?.setItem("semio.os.config", JSON.stringify({ preferences: { "os.config.ui-preferences": JSON.stringify({ version: 1, events: [{ mutation: "setTerminology", terminology: "native" }] }) } }));
    } catch {
      return;
    }
  }, wgpuSeedTerminology);
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
      const fresh = page;
      fresh.on("framenavigated", (frame) => {
        if (frame !== fresh.mainFrame()) return;
        navigations += 1;
        const cause = Date.now() - probeReloadAt < 8000 ? "expected (the probe reloaded it)" : navigationCause();
        if (!cause.startsWith("expected")) invalidatedBy ??= cause;
        log(`${label} main frame navigated (#${navigations}) → ${frame.url().slice(0, 120)} — ${cause}`);
      });
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
type MirrorNode = { key: string; window: string; role: string; inputType: string | null; label: string; description: string; disabled: boolean; expanded: string | null; pressed: string | null; selected: string | null; value: string | null; valueNow: string | null; valueMin: string | null; valueMax: string | null; valueText: string | null; actionable: boolean; focused: boolean; live: string | null; busy: boolean; shortcut: string | null; tag: string; invalid: boolean; setSize: number | null; posInSet: number | null; step: string | null; min: string | null; max: string | null; tone: string | null; checked: string | null; readonly: boolean };
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
          inputType: el instanceof HTMLInputElement ? el.type : null,
          invalid: el.getAttribute("aria-invalid") === "true",
          setSize: el.hasAttribute("aria-setsize") ? Number(el.getAttribute("aria-setsize")) : null,
          posInSet: el.hasAttribute("aria-posinset") ? Number(el.getAttribute("aria-posinset")) : null,
          step: el.getAttribute("step"),
          min: el.getAttribute("min"),
          max: el.getAttribute("max"),
          tone: el.getAttribute("data-tone"),
          checked: el.getAttribute("aria-checked"),
          readonly: el.getAttribute("aria-readonly") === "true" || (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) && el.readOnly,
        };
      }),
    [] as MirrorNode[],
  );

/** 🔎️ The mirrored node an authored key names: exact, then a `/`- or `.`-delimited suffix (a chrome control id prefixes
 * the tab id: `shell.panel.tab.<anchor>.<tabId>`), never a bare substring. */
const mirrorFind = (nodes: MirrorNode[], authored: string) =>
  nodes.find((node) => node.key === authored) ?? nodes.find((node) => !node.key.includes("::") && (node.key.endsWith(`/${authored}`) || node.key.endsWith(`␟${authored}`) || node.key.endsWith(`\u001f${authored}`) || node.key.endsWith(`.${authored}`)));

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
/** 🪂️ Waits until the mirror holds the node `authored` names (and, with `enabled`, until it is not `aria-disabled`): a panel's
 * nodes reach the mirror up to ~3 s after the press that opens it, and a control enabled by typing follows its field by a refresh
 * or two (measured on the first live wgpu run). Answers the node, or null after `timeoutMs`. */
const mirrorAwait = async (authored: string, timeoutMs = 6000, enabled = false) => {
  const found = await waitUntil(async () => mirrorFind(await mirror(), authored) ?? null, (node) => node !== null && (!enabled || !node.disabled), timeoutMs, 300);
  return found.ok ? found.value : null;
};

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
const WGPU_BOARD_CONTRACT = "semioWgpuIntrospection.dumpBoard2d(windowId?) → {surfaces:[{surfaceId, windowId, rect:[x,y,w,h] (page CSS px of the board canvas), camera:{x,y,zoom}, positions:{nodeId:[x,y]} (the published fixture), selection:[id], highlighted:[id] (`highlighted_ids_json`, the ids an open time-travel draft references — React `data-board-highlighted-ids-json`), nodes, edges, handles, parsed}]} — read-only, from the Board2d scene the frame worker already holds (snapshot_json, camera_json, selection_json, highlighted_ids_json)";

/** 🎮️ Band controls as the wgpu shell names them (`⏪️time-travel` `TimeTravelControl::control_id`). */
const WGPU_BAND_CONTROLS: readonly (readonly [string, string])[] = [
  ["nextProblem", "shell.time-travel.next-problem"],
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
  const meter = nodes.find((node) => node.key === "shell.time-travel.progress") ?? status;
  const text = status.label;
  const controls = WGPU_BAND_CONTROLS.map(([control, key]) => [control, nodes.find((node) => node.key === key)] as const)
    .filter((entry): entry is readonly [string, MirrorNode] => Boolean(entry[1]))
    .map(([control, node]) => ({ control, disabled: node.disabled, title: node.description || null, keys: node.shortcut, text: node.label, native: false, id: node.key }));
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
    progress: meter.valueNow !== null && meter.valueMax !== null ? `${meter.valueNow}/${meter.valueMax}` : null,
    controls,
  };
};
//#endregion 🔖️Wgpu

//#region 🔖️Board
type Vitals = { surface: string; nodes: number; edges: number; handles: number; parsed: string; selection: string; camera: string; transform: string; utility: string; highlighted: string | null };

/** 🩻️ The overview pane's board vitals, read without a guest round trip: React's `data-board-*` attributes, wgpu's
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
        parsed: el.getAttribute("data-board-snapshot-parsed") ?? "",
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
/** 🪞 The overview board's rect in page CSS px. React: the pane's canvas. wgpu: the chrome hit registry's `ScrollRegion` of the
 * window (measured on B2w: [6.4, 60.8, 787.2, 907.2]) — `dumpBoard2d().rect` is window-local ([0, 0, w, h]), so aiming with
 * it lands 6 px left of and 61 px above the node. */
const paneBox = async () => {
  if (renderer === "wgpu") {
    const region = ((await chromeHits()).hits ?? []).find((hit) => hit.kind === "ScrollRegion" && hit.controlId === OVERVIEW)?.rect;
    const rect = region ?? (await wgpuBoard())?.rect;
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
      for (const el of Array.from(document.querySelectorAll("[id]"))) if (el.id === target || el.id.endsWith(`/${target}`) || el.id.endsWith(`␟${target}`)) return el.id;
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

/** 🗃️ The dock panels that are open, by tab id. wgpu: a panel is open when the mirror carries its body — nodes whose
 * `data-window` IS the tab id (`framework.panel.history`, `s-sync-status`), the tab being the chrome button of that key.
 * `aria-pressed` is not the witness: on B2 five chrome buttons read `pressed=true` at boot while no panel is shown
 * (`wgpuPressedTabsWithoutPanel` judges that on its own). */
const openPanelTabIds = async () => {
  if (renderer !== "wgpu") return reactOpenPanelTabIds();
  const nodes = await mirror();
  const windows = new Set(nodes.map((node) => node.window));
  return nodes.filter((node) => node.window === "shell.chrome" && node.role === "button" && !node.key.includes("::") && windows.has(node.key)).map((node) => node.key);
};

/** 🫥️ The wgpu chrome's panel tabs that read `aria-pressed=true` while the mirror carries no body for them. */
const wgpuPressedTabsWithoutPanel = async () => {
  const nodes = await mirror();
  const windows = new Set(nodes.map((node) => node.window));
  return nodes.filter((node) => node.window === "shell.chrome" && node.role === "button" && node.pressed === "true" && /(^|\.)panel\./.test(node.key) && !windows.has(node.key)).map((node) => `${node.key}=${node.label}`);
};
const reactOpenPanelTabIds = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).filter((el) => (el as HTMLElement).offsetParent !== null).map((el) => el.id.replace(/^framework\.panelTab\./, "")), [] as string[]);

/** 🍔️ The phone layout's one toggle for the merged mobile panel (`ui.mobilePanel.toggle`, a navbar Toggle). */
const MOBILE_PANEL_TOGGLE = "ui.mobilePanel.toggle";

/** 🪧 Opens (never toggles) the panel tab `id`; at phone width the tab lives in the merged mobile panel, so its toggle opens
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
    const shape = renderer === "wgpu" ? await wgpuBootShape() : await evalSafe(() => ({ windows: document.querySelectorAll('[data-slot="window"]').length, canvases: document.querySelectorAll("canvas").length, dialogs: document.querySelectorAll('[role="dialog"]').length, chrome: true, body: document.body?.innerText.slice(0, 160) ?? "" }), { windows: 0, canvases: 0, dialogs: 0, chrome: false, body: "" });
    if (renderer === "wgpu" && shape.canvases === 0) {
      const fault = await evalSafe(() => (document.body?.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 300), "");
      if (/boot-failed|renderer fault/u.test(fault)) {
        lastBootFault = fault;
        log(`${label} wgpu shell refused to boot: ${fault}`);
        return false;
      }
    }
    if (shape.dialogs) await dismissTour();
    if ((universalRoute !== null ? shape.windows >= 1 : shape.windows >= windows && shape.canvases >= 1) && shape.chrome) {
      log(`${label} booted after ${((index + 1) * 3).toFixed(0)} s: ${JSON.stringify(shape).slice(0, 200)}`);
      await dismissTour();
      return true;
    }
    if (index % 5 === 4) log(`${label} waiting… ${JSON.stringify(shape).slice(0, 200)}`);
  }
  return false;
};

/** 🌅️ The wgpu boot shape: the introspection shim is attached only once the frame worker booted, its structure dump
 * names the live windows, and the ARIA mirror holds nodes once the first frame was projected — the shell counts as booted
 * for a person only once the mirror carries the chrome (`shell.chrome`, measured on B2 a few seconds after the boards). */
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
      return { booted: Boolean(introspection), windowIds, mirrorNodes: mirrorRoot?.querySelectorAll("[data-node-key]").length ?? 0, chromeNodes: mirrorRoot?.querySelectorAll('[data-window="shell.chrome"]').length ?? 0, canvases: document.querySelectorAll("canvas").length };
    },
    { booted: false, windowIds: [] as string[], mirrorNodes: 0, chromeNodes: 0, canvases: 0 },
  );
  const boards = shape.windowIds.filter((id) => id.startsWith("2d-")).length;
  return { windows: boards, canvases: shape.canvases, dialogs: mirrorFind(await mirror(), "ui.introduction.skip") ? 1 : 0, chrome: shape.chromeNodes > 0, body: JSON.stringify(shape).slice(0, 160) };
};

/** 🎟️ Marks the first element `find` answers with `data-probe-target=<token>` in page, so Playwright can click the
 * exact element a structural search found. */
const markToken = (() => {
  let next = 0;
  return () => `probe-${++next}`;
})();
//#endregion 🔖️Chrome

//#region 🔖️History
type HistoryRow = { id: string; kind: "entry" | "mutation"; key: string; label: string; text: string; expandable: boolean; expanded: boolean; parent: string | null; index: number; icon: string };

/** 📃️ Every materialised history row (`framework.history.entry.<seq>` and `framework.history.mutation.<id>`) in reading
 * order; a mutation row belongs to the entry row above it. wgpu answers from the ARIA mirror (label plus description). */
const readHistory = async (): Promise<HistoryRow[]> => {
  if (renderer !== "wgpu") return reactReadHistory();
  let parent: string | null = null;
  return (await mirror())
    .filter((node) => !node.key.includes("::") && /(^|[\/␟\u001f])framework\.history\.(entry\.\d+|mutation\.[^\/␟\u001f]+)$/.test(node.key))
    .map((node, index) => {
      const match = node.key.match(/(?:^|[\/␟\u001f])framework\.history\.(entry|mutation)\.([^\/␟\u001f]+)$/)!;
      const kind = match[1] as "entry" | "mutation";
      if (kind === "entry") parent = node.key;
      return { id: node.key, kind, key: match[2], label: node.label.replace(/\s+/g, " ").trim(), text: `${node.label} ${node.description}`.replace(/\s+/g, " ").trim().slice(0, 400), expandable: node.expanded !== null, expanded: node.expanded === "true", parent: kind === "mutation" ? parent : null, index, icon: "" };
    });
};
const reactReadHistory = () =>
  evalSafe(
    () => {
      const rows = Array.from(document.querySelectorAll("[id]")).filter((el) => /(^|[\/␟])framework\.history\.(entry\.\d+|mutation\.[^\/␟]+)$/.test(el.id));
      let parent: string | null = null;
      return rows.map((el, index) => {
        const match = el.id.match(/(?:^|[\/␟])framework\.history\.(entry|mutation)\.([^\/␟]+)$/)!;
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
          icon: el.querySelector('[data-slot="tree-icon"] [data-icon]')?.getAttribute("data-icon") ?? "",
        };
      });
    },
    [] as HistoryRow[],
  );

type TreeWindowRead = { key: string; path: string | null; total: number; offset: number; length: number };

/** 🏢️ The tree window whose authored key is `key` (exact, or `/`-namespaced) as React stamps it on its branch container:
 * `data-tree-window-total` (every row the window holds, materialised or not), `-offset` and `-length` (the slice it
 * materialises now). Null while the container is not mounted, and always on wgpu, whose ARIA mirror publishes no window size.
 * @see ../../../../../../🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx */
const treeWindow = (key: string): Promise<TreeWindowRead | null> =>
  renderer === "wgpu"
    ? Promise.resolve(null)
    : evalSafe(
        (target) => {
          const el = Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).find((node) => {
            const authored = node.getAttribute("data-tree-window-key") ?? "";
            return authored === target || authored.endsWith(`/${target}`);
          });
          if (!el) return null;
          const number = (name: string) => Number(el.getAttribute(name) ?? "NaN");
          return { key: el.getAttribute("data-tree-window-key") ?? target, path: el.getAttribute("data-tree-window-path"), total: number("data-tree-window-total"), offset: number("data-tree-window-offset"), length: number("data-tree-window-length") };
        },
        null as TreeWindowRead | null,
        key,
      );

/** 📶️ How many rows the history's Commands section holds — its window's `total`, never the rows the DOM holds (W1E-6). */
const historyCommandsTotal = async () => {
  if (renderer === "wgpu") {
    const sizes = (await mirror()).filter((node) => !node.key.includes("::") && /(^|[\/␟\u001f])framework\.history\.entry\.\d+$/.test(node.key)).map((node) => node.setSize).filter((size): size is number => size !== null && Number.isFinite(size));
    return sizes.length ? Math.max(...sizes) : null;
  }
  const read = await treeWindow("framework.history.commands");
  return read && Number.isFinite(read.total) ? read.total : null;
};

/** ⏬️ Scrolls the tree window `key` until it materialises its last row (`offset + length = total`), the way a person scrolls a
 * long list to its end; answers the final window and how many scrolls it took (React only; null on wgpu). */
const reachWindowEnd = async (key: string) => {
  if (renderer === "wgpu") return null;
  let read = await treeWindow(key);
  let scrolls = 0;
  for (; read && read.offset + read.length < read.total && scrolls < 60; scrolls++) {
    await evalSafe(
      (target) => {
        const el = Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).find((node) => {
          const authored = node.getAttribute("data-tree-window-key") ?? "";
          return authored === target || authored.endsWith(`/${target}`);
        });
        (el?.lastElementChild as HTMLElement | null)?.scrollIntoView({ block: "end" });
        return Boolean(el);
      },
      false,
      key,
    );
    await sleep(500);
    read = await treeWindow(key);
  }
  return read ? { ...read, scrolls, atEnd: read.offset + read.length >= read.total } : null;
};

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
  if (renderer !== "wgpu" && body.ok) await waitUntil(() => treeWindow("framework.history.commands"), (read) => read !== null && (read.total === 0 || read.length > 0), 8000, 150);
  if (renderer === "wgpu" && body.ok) {
    let last = -1;
    await waitUntil(async () => (await readHistory()).length, (count) => {
      const settled = count > 0 && count === last;
      last = count;
      return settled;
    }, 12000, 800);
  }
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
 * page materialises until it answers true (found), every row the Commands window counts (`total`) was seen (unless `deep`: a
 * search for a mutation row goes on through an expanded row's own window, which holds hundreds of rows under one entry), or the
 * body stops moving / shows nothing new (two pages without a window total — wgpu —, four while the total says rows remain). */
const pageHistory = async (direction: "down" | "up", visit: (rows: HistoryRow[]) => Promise<boolean>, deep = false) => {
  await openHistory();
  const whole = renderer === "wgpu" && !deep ? await readHistory() : [];
  const wholeTotal = whole.length ? await historyCommandsTotal() : null;
  const scrolled = wholeTotal !== null && whole.filter((row) => row.kind === "entry").length >= wholeTotal ? false : await scrollHistory(direction === "down" ? "start" : "end");
  if (scrolled) await sleep(600);
  await waitUntil(readHistory, (rows) => rows.filter((row) => row.kind === "entry" && row.expanded).every((entry) => rows.some((child) => child.parent === entry.id)), 2500, 150);
  const total = await historyCommandsTotal();
  const seen = new Set<string>();
  let idle = 0;
  for (let index = 0; index < 80; index++) {
    const rows = await readHistory();
    const fresh = rows.filter((row) => !seen.has(row.id)).length;
    for (const row of rows) seen.add(row.id);
    if (await visit(rows)) return true;
    idle = fresh ? 0 : idle + 1;
    if (!deep && total !== null && [...seen].filter((id) => /framework\.history\.entry\.\d+$/.test(id)).length >= total) break;
    if (!scrolled || idle >= (total === null ? 2 : 4) || !(await scrollHistoryBy(direction === "down" ? 0.8 : -0.8))) break;
    await sleep(350);
  }
  return false;
};

/** 🧵️ Every history row of the windowed body, paged from its start to its end (it ends scrolled to the newest rows), merged by id. */
const allHistoryRows = async () => {
  const merged = new Map<string, HistoryRow>();
  for (let attempt = 0; attempt < 3 && ![...merged.values()].some((row) => row.kind === "entry"); attempt++) {
    if (attempt) await sleep(700);
    await pageHistory("down", async (rows) => {
      for (const row of rows) merged.set(row.id, row);
      return false;
    });
  }
  return [...merged.values()];
};

/** 🪬️ Whether a Commands row is a shell chrome command — a panel toggle or tab switch, which the probe itself causes every time it
 * opens or closes a panel to read the body (React: the row's `monitor` icon; wgpu: its label, the mirror carries no icon). */
const isShellChromeRow = (row: HistoryRow) => row.kind === "entry" && !row.expandable && (row.icon === "monitor" || /^(Toggle Panel|Switch Panel Tab|Panel umschalten|Panel-Tab wechseln)$/.test(row.label));

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
    for (const entry of [...documentEntries(rows)].reverse().filter((row) => row.expandable && !expanded.has(row.id) && (!row.expanded || !rows.some((child) => child.parent === row.id)))) {
      if (found) break;
      expanded.add(entry.id);
      found = (await expandEntry(entry.id)).mutations.find((row) => row.key === key) ?? null;
    }
    return found !== null;
  }, true);
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
  const rows = await allHistoryRows();
  note("history-rows-at-the-missed-mutation", { newestBefore, entries: rows.filter((row) => row.kind === "entry" && Number(row.key) > newestBefore).map((row) => `${row.key}=${row.label}${row.expandable ? (row.expanded ? " (open)" : " (expandable)") : ""}`).slice(0, 14), mutations: rows.filter((row) => row.kind === "mutation").map((row) => `${row.parent ?? "?"} › ${row.label}`).slice(0, 8), reading: "what the History body showed when no new mutation row matched within the wait" });
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
    let state = !row ? "absent" : row.expanded === "true" ? "open" : "pointer on the chevron";
    if (state === "pointer on the chevron") {
      const hits = (await chromeHits()).hits ?? [];
      const chevron = hits.find((hit) => hit.controlId === `tree.chevron.${entryId}`);
      const label = hits.find((hit) => hit.controlId === `tree.label.${entryId}`);
      if (chevron) await page.mouse.click(chevron.rect[0] + chevron.rect[2] / 2, chevron.rect[1] + chevron.rect[3] / 2);
      else if (label) {
        state = "pointer on the row's right end";
        await page.mouse.click(label.rect[0] + label.rect[2] - 7, label.rect[1] + label.rect[3] / 2);
      } else state = "no hit for the row";
    }
    const children = await waitUntil(readHistory, (rows) => rows.some((entry) => entry.parent === entryId), 14000, 400);
    if (!children.ok) {
      const hits = ((await chromeHits()).hits ?? []).filter((hit) => hit.controlId.includes(entryId)).map((hit) => `${hit.controlId} [${hit.rect.map((value) => Math.round(value)).join(",")}]`);
      log(`expand ${entryId}: ${state}, no child after ${children.waitedMs} ms; row now expanded=${(await mirror()).find((node) => node.key === entryId)?.expanded ?? "absent"}; hits ${JSON.stringify(hits)}; mutation rows in the mirror ${JSON.stringify(children.value.filter((entry) => entry.kind === "mutation").map((entry) => `${entry.parent} › ${entry.label}`).slice(0, 4))}`);
    }
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

type ReasonReveal = { hover: boolean; leave: boolean; focus: boolean; escape: boolean; click: boolean; blur: boolean; describedBy: boolean };
type RowActionState = { present: boolean; disabled: boolean | null; reason: string; label: string; shown: string | null; reveal: ReasonReveal | null; via: string; native?: boolean; busy?: boolean };

/** 🚫️ The row action of `rowId` named by `name` as a person perceives it (gap N15, W1E-1): whether it is disabled, the reason
 * it names to assistive technology (always the button's `aria-describedby`; wgpu the mirror button's description) and, on
 * React unless `reveal` is false, the reason a sighted person sees — `[data-slot=row-action-reason][data-revealed]` carrying
 * the reason under the same id: revealed after a hover (≈ 400 ms), on keyboard focus and on a press, concealed again on
 * leave, Escape and blur. wgpu's painted tooltip is outside the DOM, so its mirror description stands for it. */
const rowActionState = async (rowId: string, name: RegExp, reveal = true): Promise<RowActionState> => {
  if (renderer === "wgpu") {
    const action = (await mirror()).find((node) => node.key.startsWith(`${rowId}::row-action::`) && name.test(node.label.trim()));
    return action ? { present: true, disabled: action.disabled, reason: action.description, label: action.label, shown: action.description || null, reveal: null, via: action.key, native: false, busy: action.busy } : { present: false, disabled: null, reason: "", label: "", shown: null, reveal: null, via: "absent" };
  }
  const token = markToken();
  const read = await evalSafe(
    (arg) => {
      const row = document.getElementById(arg.rowId);
      const pattern = new RegExp(arg.source, arg.flags);
      const button = Array.from(row?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
      if (!button) return null;
      button.setAttribute("data-probe-target", arg.token);
      const reason = (button.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim();
      return { disabled: button.getAttribute("aria-disabled") === "true" || (button as HTMLButtonElement).disabled, reason, label: (button.getAttribute("aria-label") ?? button.innerText ?? "").trim(), native: (button as HTMLButtonElement).disabled === true, busy: button.getAttribute("aria-busy") === "true" };
    },
    null as { disabled: boolean; reason: string; label: string; native: boolean; busy: boolean } | null,
    { rowId, source: name.source, flags: name.flags, token },
  );
  if (!read) return { present: false, disabled: null, reason: "", label: "", shown: null, reveal: null, via: "absent" };
  if (!reveal || !read.reason) return { present: true, ...read, shown: null, reveal: null, via: "action" };
  const target = page.locator(`[data-probe-target="${token}"]`).first();
  const revealed = () =>
    evalSafe(
      (arg) => {
        const button = document.querySelector(`[data-probe-target="${arg.token}"]`);
        const ids = (button?.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean);
        const hint = Array.from(document.querySelectorAll<HTMLElement>('[data-slot="row-action-reason"][data-revealed]')).find((el) => (el.textContent ?? "").includes(arg.reason) && el.getBoundingClientRect().width > 2 && getComputedStyle(el).visibility !== "hidden");
        return { shown: hint ? (hint.textContent ?? "").trim().slice(0, 160) : null, describedBy: Boolean(hint && ids.includes(hint.id)) };
      },
      { shown: null as string | null, describedBy: false },
      { token, reason: read.reason },
    );
  await target.hover({ timeout: 3000, force: true }).catch(() => {});
  await sleep(900);
  const hover = await revealed();
  await page.mouse.move(1, 1).catch(() => {});
  await sleep(400);
  const leave = await revealed();
  await target.focus().catch(() => {});
  await sleep(300);
  const focus = await revealed();
  await page.keyboard.press("Escape").catch(() => {});
  await sleep(300);
  const escape = await revealed();
  await target.click({ force: true, timeout: 3000 }).catch(() => {});
  await sleep(300);
  const click = await revealed();
  await evalSafe(() => (document.activeElement as HTMLElement | null)?.blur?.(), undefined);
  await page.mouse.move(1, 1).catch(() => {});
  await sleep(300);
  const blur = await revealed();
  const sequence: ReasonReveal = { hover: hover.shown !== null, leave: leave.shown === null, focus: focus.shown !== null, escape: escape.shown === null, click: click.shown !== null, blur: blur.shown === null, describedBy: hover.describedBy || focus.describedBy || click.describedBy };
  const shown = hover.shown ?? focus.shown ?? click.shown;
  return { present: true, ...read, shown: Object.values(sequence).every(Boolean) ? shown : null, reveal: sequence, via: "action" };
};

/** 👇️ Taps the row action of `rowId` named by `name` with a touch (a phone or tablet person) and answers whether the reason of
 * a disabled one then shows as `[data-slot=row-action-reason][data-revealed]` (React; null on wgpu, whose tooltip is painted). */
const tapRevealsReason = async (rowId: string, name: RegExp) => {
  if (renderer === "wgpu") return null;
  const target = await evalSafe(
    (arg) => {
      const row = document.getElementById(arg.rowId);
      const pattern = new RegExp(arg.source, arg.flags);
      const button = Array.from(row?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
      if (!button) return null;
      button.scrollIntoView({ block: "center" });
      const rect = button.getBoundingClientRect();
      const reason = (button.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim();
      return { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2, disabled: button.getAttribute("aria-disabled") === "true", reason };
    },
    null as { x: number; y: number; disabled: boolean; reason: string } | null,
    { rowId, source: name.source, flags: name.flags },
  );
  if (!target) return { tapped: false, disabled: null as boolean | null, reason: "", shown: null as string | null };
  await page.touchscreen.tap(target.x, target.y);
  await sleep(500);
  const shown = await evalSafe((reason) => (reason ? (Array.from(document.querySelectorAll<HTMLElement>('[data-slot="row-action-reason"][data-revealed]')).find((el) => (el.textContent ?? "").includes(reason) && el.getBoundingClientRect().width > 2)?.textContent?.trim() ?? null) : null), null as string | null, target.reason);
  return { tapped: true, disabled: target.disabled as boolean | null, reason: target.reason, shown };
};

/** 🔘️ Clicks a panel-body button by its authored id (`framework.history.editor.withdraw`, …), first scrolling its section and
 * its row into view unless `reveal` is false (a deep row of a windowed section the caller already scrolled to). */
const pressAuthored = async (authored: string, reveal = true) => {
  const section = authored.match(/framework\.history\.(editor|timeTravel)/)?.[0];
  if (section && reveal) {
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
type BandControl = { control: string; disabled: boolean; title: string | null; keys: string | null; text: string; native?: boolean; id?: string };
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
        controls: Array.from(el.querySelectorAll("[data-semio-time-travel-control]")).map((button) => {
          const reason = (button.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)).filter((hint): hint is HTMLElement => hint !== null && hint.getAttribute("data-slot") === "row-action-reason").map((hint) => (hint.textContent ?? "").trim()).join(" ");
          return {
            control: button.getAttribute("data-semio-time-travel-control") ?? "",
            disabled: button.getAttribute("aria-disabled") === "true" || (button as HTMLButtonElement).disabled === true,
            title: reason || button.getAttribute("title"),
            keys: button.getAttribute("aria-keyshortcuts"),
            text: (button as HTMLElement).innerText.trim(),
            native: (button as HTMLButtonElement).disabled === true,
            id: button.id,
          };
        }),
      };
    },
    null as Band | null,
  );

/** 🎞️ Installs an in-page MutationObserver that records every distinct band state (stage | review | progress) with
 * its time, so a stage React rendered for one frame is still seen, and — for the page's life — every fault line the band shows
 * (React: `data-semio-time-travel-fault` code + its words; wgpu: a status line carrying a code-like token). Re-install after
 * every reload. */
const installBandTrace = () =>
  renderer === "wgpu"
    ? evalSafe(
        (words) => {
          const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver; __probeBandFaults?: { code: string | null; text: string }[] };
          host.__probeBandTrace = [];
          host.__probeBandFaults ??= [];
          host.__probeBandObserver?.disconnect();
          let last = "";
          const read = () => {
            const el = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
            const text = el?.getAttribute("aria-label") ?? "";
            const code = /\b(?:timeTravel|vcs|history|plugin|document|module)\.[\w-]+/u.exec(text)?.[0] ?? null;
            if (code && !host.__probeBandFaults!.some((row) => row.text === text)) host.__probeBandFaults!.push({ code, text: text.slice(0, 300) });
            const stage = words.stages.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const review = words.reviews.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const meter = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.progress"]') ?? el;
            const progress = meter?.getAttribute("aria-valuenow") !== null && meter?.getAttribute("aria-valuenow") !== undefined ? `${meter?.getAttribute("aria-valuenow")}/${meter?.getAttribute("aria-valuemax")}` : "";
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
    const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver; __probeBandFaults?: { code: string | null; text: string }[] };
    host.__probeBandTrace = [];
    host.__probeBandFaults ??= [];
    host.__probeBandObserver?.disconnect();
    let last = "";
    const read = () => {
      const el = document.querySelector("[data-semio-time-travel]");
      const fault = el?.querySelector<HTMLElement>("[data-semio-time-travel-fault]");
      const code = fault?.getAttribute("data-semio-time-travel-fault") ?? null;
      const words = (fault?.textContent ?? "").replace(/\s+/g, " ").trim();
      if (fault && !host.__probeBandFaults!.some((row) => row.code === code && row.text === words)) host.__probeBandFaults!.push({ code, text: words.slice(0, 300) });
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
type ShownNotice = { code: string; text: string; t?: number };

/** 🔔️ Records every transient notice the shell shows for the page's life, by code with its words — React's
 * `[data-notice-code]`, wgpu's polite `shell.notice` mirror node named by the message and described by the code
 * (`🧯️wgpu-transient-notice` corpus) — and, under the probe's own code `shell.document-restore`, React's document-restore
 * alert (`[data-semio-bootstrap-status][role=alert]`: "Document restore failed: …", `🪪️host-bootstrap`); re-install after a
 * reload. */
const installNoticeTrace = () =>
  evalSafe((wgpu) => {
    const host = window as unknown as { __probeNotices?: { code: string; text: string; at: number }[]; __probeNoticeObserver?: MutationObserver };
    host.__probeNotices ??= [];
    host.__probeNoticeObserver?.disconnect();
    const keep = (code: string | null | undefined, text: string) => {
      const words = text.replace(/\s+/g, " ").trim().slice(0, 300);
      if (code && !host.__probeNotices!.some((row) => row.code === code && row.text === words)) host.__probeNotices!.push({ code, text: words, at: performance.now() });
    };
    const read = () => {
      if (wgpu) {
        const node = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.notice"]');
        const describedBy = node?.getAttribute("aria-describedby");
        keep(describedBy ? document.getElementById(describedBy)?.textContent?.trim() : null, node?.getAttribute("aria-label") ?? "");
        return;
      }
      for (const el of Array.from(document.querySelectorAll<HTMLElement>("[data-notice-code]"))) keep(el.getAttribute("data-notice-code"), el.textContent ?? "");
      for (const el of Array.from(document.querySelectorAll<HTMLElement>('[data-semio-bootstrap-status][role="alert"]'))) keep("shell.document-restore", el.textContent ?? "");
    };
    host.__probeNoticeObserver = new MutationObserver(read);
    host.__probeNoticeObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true, attributeFilter: wgpu ? undefined : ["data-notice-code"] });
    read();
    return true;
  }, false, renderer === "wgpu");
const pageNotices = async (): Promise<ShownNotice[]> => {
  const rows = await evalSafe(() => ((window as unknown as { __probeNotices?: { code: string; text: string; at: number }[] }).__probeNotices ?? []).map((row) => ({ code: row.code, text: row.text, agoMs: performance.now() - row.at })), [] as { code: string; text: string; agoMs: number }[]);
  const clock = (Date.now() - t0) / 1000;
  return rows.map((row) => ({ code: row.code, text: row.text, t: Number((clock - row.agoMs / 1000).toFixed(1)) }));
};
const harvestedNotices: ({ locale: Locale } & ShownNotice)[] = [];

/** 🧺️ Keeps the current page's notices for the locale before its page goes away (a reload, a closed fresh context). */
const harvestNotices = async () => {
  for (const row of await pageNotices()) if (!harvestedNotices.some((kept) => kept.locale === currentLocale && kept.code === row.code && kept.text === row.text)) harvestedNotices.push({ locale: currentLocale, ...row });
  for (const row of await pageBandFaults()) if (!harvestedBandFaults.some((kept) => kept.locale === currentLocale && kept.code === row.code && kept.text === row.text)) harvestedBandFaults.push({ locale: currentLocale, ...row });
};

type BandFault = { code: string | null; text: string };
const pageBandFaults = () => evalSafe(() => ((window as unknown as { __probeBandFaults?: BandFault[] }).__probeBandFaults ?? []).slice(), [] as BandFault[]);
const harvestedBandFaults: ({ locale: Locale } & BandFault)[] = [];

/** 🧨️ Every fault line the locale's bands showed so far (harvested pages and the current one). */
const shownBandFaults = async (): Promise<BandFault[]> => {
  const rows = [...harvestedBandFaults.filter((row) => row.locale === currentLocale).map(({ code, text }) => ({ code, text })), ...(await pageBandFaults())];
  return rows.filter((row, index) => rows.findIndex((other) => other.code === row.code && other.text === row.text) === index);
};

/** 🗣️ Every notice the locale's pages showed so far (code and words): the harvested ones and the current page's. */
const shownNotices = async (): Promise<ShownNotice[]> => {
  const rows: ShownNotice[] = [...harvestedNotices.filter((row) => row.locale === currentLocale).map(({ code, text, t }) => ({ code, text, t })), ...(await pageNotices())];
  return rows.filter((row, index) => rows.findIndex((other) => other.code === row.code && other.text === row.text) === index);
};

/** 📻️ Every notice code the locale's pages showed so far. */
const shownNoticeCodes = async () => [...new Set((await shownNotices()).map((row) => row.code))];
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

/** 🧱️ Every text of another element the band's own text lies on (React; null on wgpu, whose band is painted in the canvas):
 * sampled at three points of each text box of the band, a hit is an element outside the band — neither its ancestor nor its
 * descendant — one of whose own text boxes holds that point. An in-flow band with an opaque surface has none. Also answers
 * where the band sits (`[data-slot=layout-subfooter] > [data-semio-bottom-bands]`, the bands row under the footer). */
const bandOverlaps = () =>
  renderer === "wgpu"
    ? Promise.resolve(null)
    : evalSafe(
        () => {
          const strip = document.querySelector<HTMLElement>("[data-semio-time-travel]");
          if (!strip) return null;
          const textBoxes = (el: Element) =>
            Array.from(el.childNodes)
              .filter((node) => node.nodeType === Node.TEXT_NODE && (node.textContent ?? "").trim().length > 0)
              .flatMap((node) => {
                const range = document.createRange();
                range.selectNodeContents(node);
                return Array.from(range.getClientRects()).filter((rect) => rect.width >= 2 && rect.height >= 2);
              });
          const points: { x: number; y: number }[] = [];
          for (const el of [strip, ...Array.from(strip.querySelectorAll<HTMLElement>("*"))]) for (const rect of textBoxes(el)) for (const share of [0.15, 0.5, 0.85]) points.push({ x: rect.left + rect.width * share, y: rect.top + rect.height / 2 });
          const hits = new Map<Element, string>();
          const covers = new Map<Element, string>();
          const name = (el: Element) => `${el.tagName.toLowerCase()}#${(el.closest("[id]") as HTMLElement | null)?.id.slice(-60) ?? ""}[${el.getAttribute("data-slot") ?? ""}] "${(el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 40)}"`;
          const targets = Array.from(strip.querySelectorAll<HTMLElement>("[data-semio-time-travel-control]")).map((control) => control.getBoundingClientRect()).flatMap((rect) => [0.2, 0.5, 0.8].map((share) => ({ x: rect.left + rect.width * share, y: rect.top + rect.height / 2 })));
          for (const point of [...points, ...targets]) {
            if (point.x < 0 || point.y < 0 || point.x > innerWidth || point.y > innerHeight) continue;
            const top = document.elementFromPoint(point.x, point.y);
            if (top && !strip.contains(top) && !covers.has(top)) covers.set(top, name(top));
          }
          for (const point of points) {
            for (const el of document.elementsFromPoint(point.x, point.y)) {
              if (hits.has(el) || strip.contains(el) || el.contains(strip)) continue;
              const style = getComputedStyle(el);
              if (style.visibility === "hidden" || style.display === "none" || Number(style.opacity) === 0) continue;
              if (textBoxes(el).some((rect) => point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom)) hits.set(el, `${el.tagName.toLowerCase()}#${(el.closest("[id]") as HTMLElement | null)?.id.slice(-60) ?? ""}[${el.getAttribute("data-slot") ?? ""}] "${(el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 40)}"`);
            }
          }
          const bands = strip.closest("[data-semio-bottom-bands]");
          const rect = strip.getBoundingClientRect();
          return { points: points.length + targets.length, overlaps: [...hits.values()].slice(0, 12), coveredBy: [...covers.values()].slice(0, 12), inBottomBands: bands !== null, inSubfooter: bands?.parentElement?.getAttribute("data-slot") === "layout-subfooter", slot: bands?.parentElement?.getAttribute("data-slot") ?? null, rect: [Math.round(rect.left), Math.round(rect.top), Math.round(rect.width), Math.round(rect.height)] };
        },
        null as { points: number; overlaps: string[]; coveredBy: string[]; inBottomBands: boolean; inSubfooter: boolean; slot: string | null; rect: number[] } | null,
      );

/** 🪩️ Judges {@link bandOverlaps} under `name` (a note on wgpu). */
const bandOverlapVerdict = async (name: string) => {
  const read = await bandOverlaps();
  if (read === null) return note(`${name}-not-observable`, { reading: renderer === "wgpu" ? "the wgpu band is painted in the canvas; its layout is the wgpu shell law's" : "no band is open" });
  verdict(name, read.points > 0 && read.overlaps.length === 0 && read.coveredBy.length === 0, { ...read, reading: "the band's text lies on no other element's text and nothing is painted over its text or its controls (band rect ∩ footer / panel tab bars / panel rows = ∅; every control is the top element at its own points): an in-flow strip with its own surface, never an overlay and never under one" });
};

/** 🫧️ Whether the mutation row `key` is revealed WITHOUT the probe scrolling (React: History panel open, the row inside the
 * viewport and the top element at its start; wgpu: projected by the mirror, which holds only what is on screen). */
const mutationRowRevealed = async (key: string) => {
  const open = (await openPanelTabIds()).includes(HISTORY_TAB);
  if (renderer === "wgpu") {
    const node = (await mirror()).find((other) => !other.key.includes("::") && other.key.endsWith(`framework.history.mutation.${key}`));
    return { open, revealed: open && Boolean(node), row: node?.key ?? null };
  }
  return evalSafe(
    (arg) => {
      const row = Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id.endsWith(`framework.history.mutation.${arg.key}`));
      if (!row) return { open: arg.open, revealed: false, row: null as string | null };
      const rect = row.getBoundingClientRect();
      const inside = rect.width > 0 && rect.height > 0 && rect.top >= 0 && rect.bottom <= innerHeight && rect.left >= 0 && rect.right <= innerWidth;
      const top = inside ? document.elementFromPoint(rect.left + Math.min(rect.width / 2, 60), rect.top + rect.height / 2) : null;
      return { open: arg.open, revealed: arg.open && inside && Boolean(top && (row.contains(top) || top.contains(row))), row: row.id as string | null };
    },
    { open, revealed: false, row: null as string | null },
    { key, open },
  );
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
    return { id: node.key, tag: node.tag, stepper: node.role === "spinbutton" || (node.tag === "input" && node.inputType === "number"), value: node.value ?? node.valueNow, step: node.step, min: node.min, max: node.max, plus: steppers.some((other) => /increase|erhöhen|\+/i.test(other.label)), minus: steppers.some((other) => /decrease|verringern|−|-/i.test(other.label)), role: node.role };
  };
  const targets = mirrorFind(nodes, "framework.history.editor.input.targets.row");
  const chips = nodes.filter((node) => /framework\.history\.editor\.input\.targets\.chip\.\d+\.row$/.test(node.key)).map((node) => node.label);
  return {
    heading: mirrorFind(nodes, "framework.history.editor.target")?.label ?? null,
    status: mirrorFind(nodes, "framework.history.timeTravel.status")?.label ?? null,
    dx: number("dx"),
    dy: number("dy"),
    targets: targets ? { id: targets.key, chips, useSelection: Boolean(mirrorFind(nodes, "framework.history.editor.input.targets.useSelection") ?? mirrorFind(nodes, "framework.history.editor.input.targets.useSelection.row")), text: `${targets.label} ${targets.description}`.trim() } : null,
    inputs: nodes.filter((node) => /framework\.history\.editor\.input\.[^/]*\.row$/.test(node.key)).map((node) => `${node.key.replace(/^.*framework\.history\.editor\.input/, "")}=${node.label.slice(0, 60)}`),
  };
};
const reactEditor = () =>
  evalSafe(
    () => {
      const find = (authored: string) => Array.from(document.querySelectorAll("[id]")).find((el) => el.id === authored || el.id.endsWith(`/${authored}`) || el.id.endsWith(`␟${authored}`)) as HTMLElement | undefined;
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
      const targetsEl = find("framework.history.editor.input.targets.row");
      const chipLabel = (el: Element) => ((el.querySelector('[data-slot="tree-label"] span') ?? el.querySelector('[data-slot="tree-label"]') ?? el) as HTMLElement).innerText.replace(/\s+/g, " ").trim();
      const chips = Array.from(document.querySelectorAll("[id]")).filter((el) => /framework\.history\.editor\.input\.targets\.chip\.\d+\.row$/.test(el.id)).map(chipLabel);
      return {
        heading: find("framework.history.editor.target")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        status: find("framework.history.timeTravel.status")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        dx: number("dx"),
        dy: number("dy"),
        targets: targetsEl ? { id: targetsEl.id, chips, useSelection: Boolean(find("framework.history.editor.input.targets.useSelection") ?? find("framework.history.editor.input.targets.useSelection.row")), text: targetsEl.innerText.replace(/\s+/g, " ").trim().slice(0, 200) } : null,
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
      const el = Array.from(document.querySelectorAll("[id]")).find((node) => node.id === target || node.id.endsWith(`/${target}`) || node.id.endsWith(`␟${target}`)) as HTMLElement | undefined;
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

/** 🪤 What the editor's number control at `pointer` is the moment before it is typed into: the band stage, the field's own state
 * and whether another element covers its centre. */
const diagnoseTyping = async (pointer: string) => ({
  stage: (await band())?.stage ?? null,
  field: await evalSafe(
    (target) => {
      const host = Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === target || el.id.endsWith(`/${target}`) || el.id.endsWith(`␟${target}`));
      const field = (host?.tagName === "INPUT" ? host : host?.querySelector("input")) as HTMLInputElement | null;
      if (!field) return null;
      const rect = field.getBoundingClientRect();
      const top = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
      return { id: host?.id ?? "", disabled: field.disabled, readOnly: field.readOnly, value: field.value, rect: [Math.round(rect.left), Math.round(rect.top), Math.round(rect.width), Math.round(rect.height)], covered: top && top !== field && !field.contains(top) ? `${top.tagName.toLowerCase()}#${top.id}[${top.getAttribute("data-slot") ?? ""}]` : null };
    },
    null as { id: string; disabled: boolean; readOnly: boolean; value: string; rect: number[]; covered: string | null } | null,
    `framework.history.editor.input.${pointer}`,
  ),
});

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
  const before = await diagnoseTyping(pointer);
  const refused: string[] = [];
  const reason = (action: string) => (error: unknown) => void refused.push(`${action}: ${String(error).split("\n").slice(0, 6).join(" | ").slice(0, 500)}`);
  await input.click({ timeout: 4000 }).catch(reason("click"));
  await input.fill(String(value), { timeout: 4000 }).catch(reason("fill"));
  const typed = await input.inputValue({ timeout: 2000 }).catch(() => null);
  await input.press("Enter", { timeout: 4000 }).catch(reason("enter"));
  if (!refused.length) return { present: true, typed };
  const state = await input.evaluate((el) => {
    const field = el as HTMLInputElement;
    const rect = field.getBoundingClientRect();
    const top = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
    return { disabled: field.disabled, readOnly: field.readOnly, rect: [Math.round(rect.left), Math.round(rect.top), Math.round(rect.width), Math.round(rect.height)], covered: top && top !== field && !field.contains(top) ? `${top.tagName.toLowerCase()}#${top.id}[${top.getAttribute("data-slot") ?? ""}]` : null };
  }).catch(() => null);
  log(`typeEditorNumber ${pointer} refused ${JSON.stringify({ refused, state, before }).slice(0, 1800)}`);
  await shot(`type-${pointer}-refused`);
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
  if (!id) {
    log(`keyEditorNumber ${pointer} ${key}: no such control in the document (band ${(await band())?.stage ?? "none"})`);
    return false;
  }
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
    return { id: node.key, dial: /dial/i.test(`${node.description} ${node.role}`), role: node.role, valueNow: number(node.valueNow), valueText: node.valueText, min: number(node.valueMin ?? node.min), max: number(node.valueMax ?? node.max), ticks: [], refusal: node.invalid ? node.description || null : null, invalid: node.invalid, rowText: `${row?.label ?? ""} ${row?.description ?? ""} ${node.invalid ? node.description : ""}`.trim() };
  }
  return evalSafe(
    (target) => {
      const find = (suffix: string) => Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === suffix || el.id.endsWith(`/${suffix}`) || el.id.endsWith(`␟${suffix}`));
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
    const nodes = await mirror();
    const node = nodes.find((other) => other.key.endsWith(`${authored}::editor`)) ?? mirrorFind(nodes, authored);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null, via: "absent" };
    await field.fill(text, { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await field.press("Enter", { timeout: 4000 }).catch(() => page.keyboard.press("Enter").catch(() => {}));
    return { present: true, typed: typed as string | null, via: `mirror ${node.role}` };
  }
  const token = markToken();
  const field = page.locator(`[data-probe-row="${token}"] [data-slot="slider-content"] input[type="number"]`).first();
  let attempts = 0;
  for (; attempts < 3; attempts++) {
    const readout = await evalSafe(
      (arg) => {
        const find = (suffix: string) => Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === suffix || el.id.endsWith(`/${suffix}`) || el.id.endsWith(`␟${suffix}`));
        const row = find(`${arg.target}.row`) ?? find(arg.target)?.parentElement ?? null;
        for (const stale of Array.from(document.querySelectorAll("[data-probe-row], [data-probe-target]"))) if (stale.getAttribute("data-probe-row") === arg.token || stale.getAttribute("data-probe-target") === arg.token) {
          stale.removeAttribute("data-probe-row");
          stale.removeAttribute("data-probe-target");
        }
        const value = row?.querySelector<HTMLElement>('[data-slot="slider-value"]');
        value?.setAttribute("data-probe-target", arg.token);
        row?.setAttribute("data-probe-row", arg.token);
        return Boolean(value) || Boolean(row?.querySelector('[data-slot="slider-content"] input[type="number"]'));
      },
      false,
      { target: authored, token },
    );
    if (!readout) return { present: false, typed: null as string | null, via: "no-readout" };
    if (await field.count().catch(() => 0)) break;
    await evalSafe(
      (mark) => {
        const host = window as unknown as { __probeReadout?: { t: number; editing: boolean; focus: string }[] };
        const states: { t: number; editing: boolean; focus: string }[] = (host.__probeReadout = []);
        const start = performance.now();
        const sample = () => {
          const row = document.querySelector(`[data-probe-row="${mark}"]`);
          const active = document.activeElement as HTMLElement | null;
          let named: HTMLElement | null = active;
          while (named && !named.id) named = named.parentElement;
          const state = { t: Math.round(performance.now() - start), editing: Boolean(row?.querySelector('[data-slot="slider-content"] input[type="number"]')), focus: active ? `${active.tagName.toLowerCase()}[${active.getAttribute("data-slot") ?? active.getAttribute("role") ?? ""}]@${named?.id.slice(-70) ?? ""}` : "none" };
          const last = states[states.length - 1];
          if (!last || last.editing !== state.editing || last.focus !== state.focus) states.push(state);
          if (performance.now() - start < 2500) setTimeout(sample, 20);
        };
        sample();
        return true;
      },
      false,
      token,
    );
    await page.locator(`[data-probe-target="${token}"]`).first().dblclick({ timeout: 4000 }).catch(() => {});
    await sleep(700);
    if (await field.count().catch(() => 0)) break;
  }
  const opening = await evalSafe(() => ((window as unknown as { __probeReadout?: { t: number; editing: boolean; focus: string }[] }).__probeReadout ?? []).slice(0, 24), [] as { t: number; editing: boolean; focus: string }[]);
  if (!(await field.count().catch(() => 0))) return { present: true, typed: null as string | null, via: `readout did not stay open for typing after ${attempts} double-clicks`, opening };
  await field.fill(text, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  const bounds = await field.evaluate((el) => ({ min: el.getAttribute("min"), max: el.getAttribute("max") }), undefined, { timeout: 3000 }).catch(() => null);
  await field.press("Enter", { timeout: 4000 }).catch(() => {});
  const life = await evalSafe(
    async (mark) => {
      const states: { t: number; editing: boolean; invalid: boolean; alert: string | null }[] = [];
      const start = performance.now();
      while (performance.now() - start < 1000) {
        const row = document.querySelector(`[data-probe-row="${mark}"]`);
        const editor = row?.querySelector<HTMLInputElement>('[data-slot="slider-content"] input[type="number"]') ?? null;
        const state = { t: Math.round(performance.now() - start), editing: Boolean(editor), invalid: editor?.getAttribute("aria-invalid") === "true" || Boolean(row?.querySelector('[data-invalid="true"]')), alert: row ? (row.querySelector('[role="alert"]')?.textContent?.trim() ?? null) : "row-unmounted" };
        const last = states[states.length - 1];
        if (!last || last.editing !== state.editing || last.invalid !== state.invalid || last.alert !== state.alert) states.push(state);
        await new Promise((resolveTick) => setTimeout(resolveTick, 40));
      }
      return states;
    },
    [] as { t: number; editing: boolean; invalid: boolean; alert: string | null }[],
    token,
  );
  return { present: true, typed, via: attempts ? `readout (double-click ${attempts + 1})` : "readout", bounds, life, opening: opening.slice(0, 8) };
};

/** 🫳️ Presses "Use selection" of the editor's reference input at `pointer`: the first row of its windowed reference row,
 * the button `framework.history.editor.input.<pointer>.useSelection` (its row `….useSelection.row`), on both renderers. */
const pressUseSelection = async (pointer: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") await mirrorAwait(`framework.history.editor.input.${pointer}.useSelection`, 40000);
  const pressed = await pressAuthored(`framework.history.editor.input.${pointer}.useSelection`);
  return pressed.present && pressed.disabled !== true ? (pressed.id ?? "useSelection") : null;
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
type ArmTarget = { t: number; stage: string; present: boolean; disabled: boolean | null; reason: string; how: string };
type ArmLog = { samples: ArmSample[]; acted: { t: number; via: string; disabled: boolean | null; reason: string; stage: string; settledMs: number } | null; target: ArmTarget[] };
type ArmAction = { kind: "cancel" } | { kind: "edit"; rowId: string; source: string; flags: string };

/** 🧘️ How long after the band turned `replaying` the armed Edit press waits for the rows to follow it (one History body
 * refresh is ≥ 100 ms). */
const EDIT_SETTLE_MS = 160;

/** 🪤️ Arms the page to act in the first frame the band shows `replaying` — a person pressing the moment the progress bar
 * appears — and samples every band state (stage, review, progress) with its in-page time until {@link disarmReplay}. `cancel`
 * presses the band's Cancel replay (React `[data-semio-time-travel-control=cancelReplay]`, wgpu `shell.time-travel.cancel-replay`);
 * `edit` presses Edit on the row `rowId` (its Edit row action, else the row's activation, which is the same verb) — as a person
 * does, on a SETTLED read: once the action reads disabled, else `EDIT_SETTLE_MS` after the band turned `replaying` (one body
 * refresh is ≥ 100 ms, so by then the rows have followed the band), with a timer beside the observer so a quiet DOM still ticks. */
const armReplay = (action: ArmAction) =>
  evalSafe(
    (arg) => {
      const host = window as unknown as { __probeArm?: ArmLog; __probeArmObserver?: MutationObserver; __probeArmTimer?: number };
      host.__probeArmObserver?.disconnect();
      if (host.__probeArmTimer !== undefined) clearInterval(host.__probeArmTimer);
      const armedAt = performance.now();
      let replayingSince: number | null = null;
      const armLog: ArmLog = { samples: [], acted: null, target: [] };
      host.__probeArm = armLog;
      const read = (): Omit<ArmSample, "t"> => {
        if (arg.wgpu) {
          const status = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
          const text = status?.getAttribute("aria-label") ?? "";
          const stage = status ? (arg.stages.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "") : "none";
          const review = arg.reviews.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
          const meter = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.progress"]') ?? status;
          const now = meter?.getAttribute("aria-valuenow");
          const max = meter?.getAttribute("aria-valuemax");
          return { stage, review, done: now ? Number(now) : null, total: max ? Number(max) : null };
        }
        const band = document.querySelector("[data-semio-time-travel]");
        const progress = band?.querySelector("progress");
        return { stage: band?.getAttribute("data-semio-time-travel") ?? "none", review: band?.querySelector("[data-semio-time-travel-review]")?.getAttribute("data-semio-time-travel-review") ?? "", done: progress ? Number(progress.getAttribute("value")) : null, total: progress ? Number(progress.getAttribute("max")) : null };
      };
      const press = (): { via: string; disabled: boolean | null; reason: string } => {
        const disabledOf = (el: Element) => (el as HTMLButtonElement).disabled === true || el.getAttribute("aria-disabled") === "true";
        const reasonOf = (el: Element) => (el.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim();
        if (arg.action.kind === "cancel") {
          const control = document.querySelector<HTMLElement>(arg.wgpu ? '#semio-wgpu-accessibility [data-node-key="shell.time-travel.cancel-replay"]' : '[data-semio-time-travel-control="cancelReplay"]');
          if (!control) return { via: "absent", disabled: null, reason: "" };
          const disabled = disabledOf(control);
          control.click();
          return { via: "cancelReplay", disabled, reason: reasonOf(control) };
        }
        const pattern = new RegExp(arg.action.source, arg.action.flags);
        const rowId = arg.action.rowId;
        if (arg.wgpu) {
          const nodes = Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]"));
          const button = nodes.find((node) => (node.dataset.nodeKey ?? "").startsWith(`${rowId}::row-action::`) && pattern.test((node.getAttribute("aria-label") ?? "").trim()));
          const target = button ?? nodes.find((node) => node.dataset.nodeKey === rowId);
          if (!target) return { via: "absent", disabled: null, reason: "" };
          const disabled = disabledOf(target);
          target.click();
          return { via: button ? "action" : "row", disabled, reason: reasonOf(target) };
        }
        const row = document.getElementById(rowId);
        if (!row) return { via: "absent", disabled: null, reason: "" };
        const button = Array.from(row.querySelectorAll<HTMLElement>('button, [role="button"]')).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
        const target = button ?? row.querySelector<HTMLElement>('[data-slot="tree-label"]') ?? row;
        const disabled = disabledOf(target) || row.getAttribute("aria-disabled") === "true";
        target.click();
        return { via: button ? "action" : "row", disabled, reason: reasonOf(target) };
      };
      const watch = (stage: string) => {
        if (arg.action.kind !== "edit") return;
        const pattern = new RegExp(arg.action.source, arg.action.flags);
        const rowId = arg.action.rowId;
        const button = arg.wgpu
          ? Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).find((node) => (node.dataset.nodeKey ?? "").startsWith(`${rowId}::row-action::`) && pattern.test((node.getAttribute("aria-label") ?? "").trim()))
          : Array.from(document.getElementById(rowId)?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
        const describedBy = (button?.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean);
        const how = button ? [(button as HTMLButtonElement).disabled === true ? "disabled attribute" : "", button.getAttribute("aria-disabled") === "true" ? "aria-disabled" : "", button.getAttribute("aria-busy") === "true" ? "aria-busy" : "", describedBy.length ? `describedby ${describedBy.length} id(s), ${describedBy.filter((id) => document.getElementById(id)).length} in the document` : "no aria-describedby", button.getAttribute("title") ? `title ${button.getAttribute("title")}` : ""].filter(Boolean).join("; ") : "";
        const state = { stage, present: Boolean(button), disabled: button ? (button as HTMLButtonElement).disabled === true || button.getAttribute("aria-disabled") === "true" : null, reason: button ? describedBy.map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim() : "", how };
        const last = armLog.target[armLog.target.length - 1];
        if (!last || last.stage !== state.stage || last.present !== state.present || last.disabled !== state.disabled || last.reason !== state.reason || last.how !== state.how) armLog.target.push({ t: Math.round(performance.now() - armedAt), ...state });
        if (armLog.target.length > 200) armLog.target.splice(1, 1);
      };
      const tick = () => {
        const state = read();
        watch(state.stage);
        const last = armLog.samples[armLog.samples.length - 1];
        if (!last || last.stage !== state.stage || last.review !== state.review || last.done !== state.done || last.total !== state.total) {
          armLog.samples.push({ t: Math.round(performance.now() - armedAt), ...state });
          if (armLog.samples.length > 800) armLog.samples.splice(1, 1);
        }
        if (state.stage === "replaying" && replayingSince === null) replayingSince = performance.now();
        const settledMs = replayingSince === null ? 0 : Math.round(performance.now() - replayingSince);
        const editReadsDisabled = armLog.target[armLog.target.length - 1]?.disabled === true;
        if (armLog.acted === null && state.stage === "replaying" && (arg.action.kind === "cancel" || editReadsDisabled || settledMs >= arg.settleMs)) armLog.acted = { t: Math.round(performance.now() - armedAt), stage: state.stage, settledMs, ...press() };
      };
      host.__probeArmObserver = new MutationObserver(tick);
      host.__probeArmObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true });
      host.__probeArmTimer = window.setInterval(tick, 40);
      tick();
      return true;
    },
    false,
    { action, settleMs: EDIT_SETTLE_MS, wgpu: renderer === "wgpu", stages: BAND_WORDS.map(([stage, pattern]) => [stage, pattern.source] as [string, string]), reviews: REVIEW_WORDS.map(([review, pattern]) => [review, pattern.source] as [string, string]) },
  );

/** 📼️ The armed page's log: every sampled band state and the press it made (null before any). */
const replayArmLog = () => evalSafe(() => (window as unknown as { __probeArm?: ArmLog }).__probeArm ?? null, null as ArmLog | null);

/** 🧯️ Stops the arm's observer and answers its final log. */
const disarmReplay = async () => {
  const final = await replayArmLog();
  await evalSafe(() => {
    const host = window as unknown as { __probeArmObserver?: MutationObserver; __probeArmTimer?: number };
    host.__probeArmObserver?.disconnect();
    if (host.__probeArmTimer !== undefined) clearInterval(host.__probeArmTimer);
  }, undefined);
  return final;
};
//#endregion 🔖️ReplayArm

//#region 🔖️Reprojection
type ReprojectionKind = "step" | "load" | "remote";
type ReprojectionRead = { title: string; status: string; kind: ReprojectionKind | null; done: number | null; total: number | null; paused: boolean; cancel: boolean; rerun: boolean };

/** 🎴️ Which history change a reprojection status line names, with its progress (`HistoryPatch.reprojection {kind, done,
 * total, paused}`, rendered by the runtime's `time_travel_reprojection_section` in the current locale). */
const reprojectionKindOf = (status: string): { kind: ReprojectionKind | null; done: number | null; total: number | null; paused: boolean } => {
  const copy = COPY[currentLocale];
  for (const [kind, pattern] of [["step", copy.stepProgress], ["load", copy.loadProgress], ["remote", copy.remoteProgress]] as const) {
    const match = pattern.exec(status);
    if (match) return { kind, done: Number(match[1]), total: Number(match[2]), paused: false };
  }
  return { kind: status.includes(copy.remotePaused) ? "remote" : null, done: null, total: null, paused: status.includes(copy.remotePaused) };
};

/** 📡️ The history body's `framework.history.reprojection` section (design §16.6, gap N17, §20.8): what replays — this
 * replica's own history step, a whole-document load or a remote change — its progress in words, paused, and its Cancel
 * replay / Replay again controls; null while nothing replays (or the History panel is closed). */
const reprojection = async (): Promise<ReprojectionRead | null> => {
  const section = await presentKey("framework.history.reprojection");
  if (section === null) return null;
  const status = (await textOfKey("framework.history.reprojection.status")).replace(/\s+/g, " ").trim();
  const sectionText = renderer === "wgpu" ? (mirrorFind(await mirror(), "framework.history.reprojection")?.label ?? "") : await textOfKey("framework.history.reprojection");
  const title = (sectionText.split("\n").map((line) => line.trim()).find(Boolean) ?? "").slice(0, 80);
  const row = async (authored: string) => (await presentKey(authored)) !== null || (await presentKey(`${authored}.row`)) !== null;
  return { title, status, ...reprojectionKindOf(status), cancel: await row("framework.history.reprojection.cancelReplay"), rerun: await row("framework.history.reprojection.rerun") };
};

type ReprojectionSample = { t: number; text: string; title: string; shell: string; kind: string | null; phase: string | null; notice: string | null; role: string | null; live: string | null; control: string | null; session: boolean };
type ReprojectionPress = { t: number; via: string; disabled: boolean | null; notices: string[] };
type ReprojectionArmLog = { samples: ReprojectionSample[]; undo: ReprojectionPress | null; cancel: ReprojectionPress | null; ids: string[] | null };
type ReprojectionArmAction = "observe" | "cancel" | "undo-then-cancel";

/** 🕸️ Arms the page to react to a replaying history change the frame it is announced — a person hearing "History step: Replaying
 * history…" or "Document load: Loading document…" — and samples, with its in-page time, both of its surfaces until
 * {@link disarmReprojection}: the shell's polite status outside the History panel (React `[data-semio-history-reprojection]`
 * with its `-phase`, `data-notice-code`, `role=status` announcement and `-control`; wgpu the mirror node
 * `shell.history.reprojection`) and the History body's `framework.history.reprojection` section, plus whether a session band
 * is open. `undo-then-cancel` first presses the body's Undo (`framework.history.undo.run`, a history step that must be refused
 * while one replays or a document loads), then — once the refusal's notice showed or 1.5 s passed with the change still up —
 * Cancel replay (the shell's `cancelReplay` control, else the body's); `cancel` presses Cancel replay at once; `observe` only
 * samples. */
const armReprojection = (action: ReprojectionArmAction) =>
  evalSafe(
    (arg) => {
      const host = window as unknown as { __probeReprojection?: ReprojectionArmLog; __probeReprojectionObserver?: MutationObserver; __probeNotices?: { code: string }[] };
      host.__probeReprojectionObserver?.disconnect();
      const armedAt = performance.now();
      const armLog: ReprojectionArmLog = { samples: [], undo: null, cancel: null, ids: null };
      host.__probeReprojection = armLog;
      const find = (authored: string) => document.querySelector<HTMLElement>(arg.wgpu ? `#semio-wgpu-accessibility [data-node-key="${authored}"], #semio-wgpu-accessibility [data-node-key$="/${authored}"]` : `[id="${authored}"], [id$="/${authored}"]`);
      const textOf = (el: HTMLElement | null) => (el === null ? "" : arg.wgpu ? `${el.getAttribute("aria-label") ?? ""}` : el.innerText).replace(/\s+/g, " ").trim();
      const titleOf = (el: HTMLElement | null) => (el === null ? "" : arg.wgpu ? (el.getAttribute("aria-label") ?? "") : (el.innerText.split("\n").map((line) => line.trim()).find(Boolean) ?? "")).slice(0, 80);
      const notices = () => (host.__probeNotices ?? []).map((row) => row.code);
      const click = (el: HTMLElement | null, via: string): ReprojectionPress => {
        const button = el === null ? null : arg.wgpu || el.matches("button") ? el : (el.querySelector<HTMLElement>("button") ?? el);
        const disabled = button === null ? null : (button as HTMLButtonElement).disabled === true || button.getAttribute("aria-disabled") === "true";
        button?.click();
        return { t: Math.round(performance.now() - armedAt), via: button === null ? "absent" : via, disabled, notices: notices() };
      };
      const shell = () => {
        if (arg.wgpu) {
          const node = document.querySelector<HTMLElement>('#semio-wgpu-accessibility [data-node-key="shell.history.reprojection"]');
          return { shell: (node?.getAttribute("aria-label") ?? "").trim(), kind: null, phase: null, notice: null, role: node?.getAttribute("role") ?? null, live: node?.getAttribute("aria-live") ?? null, control: null };
        }
        const el = document.querySelector<HTMLElement>("[data-semio-history-reprojection]");
        const announcement = el?.querySelector<HTMLElement>("[data-semio-history-reprojection-announcement]") ?? null;
        return { shell: (announcement?.textContent ?? "").replace(/\s+/g, " ").trim(), kind: el?.getAttribute("data-semio-history-reprojection") ?? null, phase: el?.getAttribute("data-semio-history-reprojection-phase") ?? null, notice: el?.getAttribute("data-notice-code") ?? null, role: announcement?.getAttribute("role") ?? null, live: announcement?.getAttribute("aria-live") ?? null, control: el?.querySelector("[data-semio-history-reprojection-control]")?.getAttribute("data-semio-history-reprojection-control") ?? null };
      };
      const tick = () => {
        const status = textOf(find("framework.history.reprojection.status"));
        const sample = { text: status, title: status === "" ? "" : titleOf(find("framework.history.reprojection")), ...shell(), session: document.querySelector(arg.wgpu ? '#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]' : "[data-semio-time-travel]") !== null };
        const last = armLog.samples[armLog.samples.length - 1];
        if (!last || last.text !== sample.text || last.shell !== sample.shell || last.phase !== sample.phase || last.control !== sample.control || last.session !== sample.session) {
          armLog.samples.push({ t: Math.round(performance.now() - armedAt), ...sample });
          if (armLog.samples.length > 600) armLog.samples.splice(1, 1);
        }
        const live = sample.text !== "" || sample.shell !== "";
        if (live && armLog.ids === null) {
          const section = find("framework.history.reprojection");
          const scope = arg.wgpu ? Array.from(document.querySelectorAll<HTMLElement>('#semio-wgpu-accessibility [data-node-key*="history.reprojection"]')).map((node) => `${node.dataset.nodeKey ?? ""}[${node.getAttribute("role") ?? ""}]`) : Array.from((section?.closest('[data-slot="tree-section-wrapper"]') ?? section)?.querySelectorAll<HTMLElement>("[id]") ?? []).filter((node) => /framework\.history/.test(node.id)).map((node) => `${node.id.replace(/^.*framework\.history\./, "")}[${node.getAttribute("data-slot") ?? node.getAttribute("role") ?? ""}]`);
          armLog.ids = scope.slice(0, 16);
        }
        if (!live || arg.action === "observe") return;
        const cancel = () => (arg.wgpu ? null : document.querySelector<HTMLElement>('[data-semio-history-reprojection-control="cancelReplay"]')) ?? find("framework.history.reprojection.cancelReplay") ?? find("framework.history.reprojection.cancelReplay.row");
        if (arg.action === "cancel" && armLog.cancel === null) {
          armLog.cancel = click(cancel(), "cancelReplay");
          return;
        }
        if (armLog.undo === null) {
          armLog.undo = click(find("framework.history.undo.run") ?? find("framework.history.undo"), "framework.history.undo");
          setTimeout(tick, 1600);
          return;
        }
        if (armLog.cancel === null && (notices().some((code) => code === "history.replaying" || code === "document.loading") || performance.now() - armedAt - armLog.undo.t >= 1500)) armLog.cancel = click(cancel(), "cancelReplay");
      };
      host.__probeReprojectionObserver = new MutationObserver(tick);
      host.__probeReprojectionObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true });
      tick();
      return true;
    },
    false,
    { action, wgpu: renderer === "wgpu" },
  );

/** 🛑️ Stops the reprojection arm and answers its log: every status it sampled and the presses it made. */
const disarmReprojection = async () => {
  const final = await evalSafe(() => (window as unknown as { __probeReprojection?: ReprojectionArmLog }).__probeReprojection ?? null, null as ReprojectionArmLog | null);
  await evalSafe(() => (window as unknown as { __probeReprojectionObserver?: MutationObserver }).__probeReprojectionObserver?.disconnect(), undefined);
  return final;
};

/** 📉️ An armed reprojection log summarised: the statuses it saw on either surface (de-duplicated), their kinds and largest
 * progress total, when the change first showed and when it went away, the shell samples, and the presses. */
const reprojectionSummary = (armLog: ReprojectionArmLog | null) => {
  const samples = armLog?.samples ?? [];
  const shown = samples.filter((sample) => sample.text !== "" || sample.shell !== "");
  const lines = shown.flatMap((sample) => [sample.text, sample.shell]).filter(Boolean);
  const kinds = [...new Set([...lines.map((line) => reprojectionKindOf(line).kind), ...shown.map((sample) => sample.kind)].filter((kind): kind is ReprojectionKind => kind === "step" || kind === "load" || kind === "remote"))];
  const totals = lines.map((line) => reprojectionKindOf(line).total ?? 0);
  const firstAt = shown[0]?.t ?? null;
  const goneAt = firstAt === null ? null : (samples.find((sample) => sample.t > firstAt && sample.text === "" && sample.shell === "")?.t ?? null);
  return { seen: shown.length > 0, kinds, total: totals.length ? Math.max(...totals) : null, titles: [...new Set(shown.map((sample) => sample.title).filter(Boolean))].slice(0, 4), texts: [...new Set(lines)].slice(0, 6), shell: shown.filter((sample) => sample.shell !== "").map(({ t, shell, kind, phase, notice, role, live, control, session }) => ({ t, shell, kind, phase, notice, role, live, control, session })).slice(0, 12), firstAt, goneAt, undo: armLog?.undo ?? null, cancel: armLog?.cancel ?? null, ids: armLog?.ids ?? null };
};

/** 📢️ The shell-status verdicts of one observed history change (audit W1E-3, contract of S4-UI): it is announced outside the
 * History panel — React `[data-semio-history-reprojection=<kind>]` with its `-phase`, a `role=status` `aria-live=polite`
 * announcement reading the kernel copy "<title>: <text>"; wgpu the polite (or progressbar) mirror node
 * `shell.history.reprojection` named by the kernel line —, its Cancel replay / Replay again controls show only while no
 * session is open, and a refused change names its reason in words (its code only as `data-notice-code`). `suffix` tells two
 * observations of one step apart; nothing is judged when the shell never showed the change. */
const reprojectionShellVerdicts = (summary: ReturnType<typeof reprojectionSummary>, suffix = "") => {
  const copy = COPY[currentLocale];
  const titles: Record<ReprojectionKind, string> = { step: copy.stepTitle, load: copy.loadTitle, remote: copy.remoteTitle };
  const shell = summary.shell;
  if (!shell.length) {
    const bodyShownMs = summary.firstAt === null ? 0 : summary.goneAt === null ? Number.POSITIVE_INFINITY : summary.goneAt - summary.firstAt;
    if (bodyShownMs >= 1000) verdict(`reprojection-status-is-announced-outside-the-history-panel${suffix}`, false, { bodyShownMs: Number.isFinite(bodyShownMs) ? bodyShownMs : "until the observation ended", titles: summary.titles, texts: summary.texts, shell: [], reading: renderer === "wgpu" ? "the History body showed the change for longer than a second while the mirror never projected `shell.history.reprojection`" : "the History body showed the change for longer than a second while no shell status `[data-semio-history-reprojection]` was ever rendered — a person with the History panel closed is told nothing (W1E-3)" });
    else note(`reprojection-shell-status-not-seen${suffix}`, { summary, reading: "the change came and went without a shell frame showing it (progress refreshes ≥ 100 ms)" });
    return;
  }
  const announced = (sample: (typeof shell)[number]) => {
    const kind = (sample.kind as ReprojectionKind | null) ?? reprojectionKindOf(sample.shell).kind;
    const line = reprojectionKindOf(sample.shell);
    const words = line.kind !== null || line.paused || /:\s*\S/.test(sample.shell);
    if (renderer === "wgpu") return words && (sample.live === "polite" || sample.role === "progressbar");
    return kind !== null && sample.shell.startsWith(`${titles[kind]}: `) && words && sample.role === "status" && sample.live === "polite" && ["progress", "paused", "refused"].includes(sample.phase ?? "");
  };
  verdict(`reprojection-status-is-announced-outside-the-history-panel${suffix}`, shell.every(announced), { shell, reading: renderer === "wgpu" ? "W1E-3 on wgpu: the mirror node `shell.history.reprojection` is a polite status (a progressbar while it replays) named by the kernel's status line" : "W1E-3: `[data-semio-history-reprojection=<kind>]` + `-phase=progress|paused|refused`, a `role=status` `aria-live=polite` announcement reading the kernel copy \"<title>: <text>\" (`historyReprojectionStatus`)" });
  if (renderer === "wgpu") note(`reprojection-controls-live-in-the-history-body${suffix}`, { reading: "the wgpu shell status dispatches nothing; Cancel replay / Replay again are the History body's `framework.history.reprojection` rows" });
  else verdict(`reprojection-controls-show-only-while-no-session-is-open${suffix}`, shell.every((sample) => (sample.session ? sample.control === null : sample.phase === "refused" || sample.control !== null)), { shell, reading: "`historyReprojectionControlV1`: Cancel replay (Replay again while a remote change is paused) only while no history-edit session is open — the session's band owns both while it is" });
  const refused = shell.filter((sample) => sample.phase === "refused");
  if (refused.length) verdict(`a-refused-history-change-names-its-reason-in-words${suffix}`, refused.every((sample) => sample.notice !== null && !sample.shell.includes(sample.notice)), { refused, reading: "a refused adoption reads `<kind>.refused` with the notice's words; its code is only `data-notice-code`" });
};

/** ⌛️ Waits until neither the shell status nor the history body shows a replaying change for `quietMs` in a row (the step
 * adopted, the load finished or a cancel dropped it), at most `timeoutMs`. */
const reprojectionSettled = async (timeoutMs = 120000, quietMs = 1500) => {
  const start = Date.now();
  let quietSince: number | null = null;
  const shellLive = () => (renderer === "wgpu" ? mirror().then((nodes) => nodes.some((node) => node.key === "shell.history.reprojection")) : evalSafe(() => document.querySelector("[data-semio-history-reprojection]") !== null, false));
  while (Date.now() - start < timeoutMs) {
    const live = (await reprojection()) !== null || (await shellLive());
    quietSince = !live ? (quietSince ?? Date.now()) : null;
    if (quietSince !== null && Date.now() - quietSince >= quietMs) return { ok: true, waitedMs: Date.now() - start };
    await sleep(250);
  }
  return { ok: false, waitedMs: Date.now() - start };
};
//#endregion 🔖️Reprojection
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
  if (!mutation) return { entry: null as string | null, via: "absent", band: null as Band | null, waitedMs: 0, editorWaitedMs: 0 };
  if (mutation.parent !== ctx.dragEntry) log(`drag row moved ${ctx.dragEntry} → ${mutation.parent}`);
  const via = await pressRowAction(mutation.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  const editorWaitedMs = settled.ok ? await editorArrived(mutation.label) : 0;
  return { entry: mutation.parent, via, band: settled.value, waitedMs: settled.waitedMs, editorWaitedMs };
};

/** 🪄️ Waits for the draft editor of the session that just began on the mutation labelled `label`: the band (host chrome) reads
 * `editing` first, the editor rows are part of the History body and arrive with its next refresh (measured 0.3–1.5 s under
 * load). Answers the wait; a person sees the editor before acting on it, so every step acts after this. */
const editorArrived = async (label: string) => {
  const prefix = label.slice(0, 12);
  const arrived = await waitUntil(editor, (value) => Boolean(value?.heading && value.heading.includes(prefix)), 15000, 150);
  if (!arrived.ok) log(`editor of "${prefix}…" did not arrive within ${arrived.waitedMs} ms (heading ${JSON.stringify(arrived.value?.heading ?? null)})`);
  return arrived.waitedMs;
};

/** 🖍️ Presses Edit on the mutation row `key` names (also from a review: Begin on another target) and answers the band once
 * it reads `editing`. */
const beginEditOf = async (key: string) => {
  await openHistory();
  const row = await findMutationRow(key);
  if (!row) return { row: null as HistoryRow | null, via: "absent", band: null as Band | null, waitedMs: 0, editorWaitedMs: 0 };
  const via = await pressRowAction(row.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing" && (b.target ?? b.text).includes(row.label.slice(0, 12)), 20000);
  const editorWaitedMs = settled.ok ? await editorArrived(row.label) : 0;
  return { row: row as HistoryRow | null, via, band: settled.value, waitedMs: settled.waitedMs, editorWaitedMs };
};

/** 🌄️ Step 1 — the board is parsed and populated, and the history panel speaks the locale under test. */
const step1 = async (ctx: Ctx) => {
  const parsed = await waitUntil(vitals, (v) => v?.parsed === "true", 60000);
  verdict("fixture-parsed", parsed.ok, { parsed: parsed.value?.parsed, waitedMs: parsed.waitedMs });
  const populated = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
  const p = await positions();
  verdict("board-has-nodes", (populated.value?.nodes ?? 0) > 0 && Object.keys(p).length > 0, { nodes: populated.value?.nodes, edges: populated.value?.edges, positions: Object.keys(p).length, camera: populated.value?.camera });
  if (renderer === "wgpu") {
    const pressed = await wgpuPressedTabsWithoutPanel();
    const windows = Array.from(new Set((await mirror()).map((node) => node.window)));
    await shot("wgpu-boot-chrome");
    note("wgpu-pressed-panel-tabs-without-a-mirrored-panel", { pressedWithoutPanel: pressed, mirroredWindows: windows, reading: "coordinator decision (2026-10-05, F15): `aria-pressed=true` on a dock tab names the active tab of its dock group, as on React and in the shared fixture — not a shown panel; recorded, not judged. The probe reads open panels from the mirrored body windows" });
  }
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
  const totalBefore = await historyCommandsTotal();
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
  const totalAfter = await historyCommandsTotal();
  if (totalBefore === null) note("n1-commands-window-total-not-published", { reading: "no window size is published (React `data-tree-window-total`, wgpu `aria-setsize` of the entry rows); the new row is proven by its sequence above" });
  else {
    const newer = (await allHistoryRows()).filter((entry) => entry.kind === "entry" && Number(entry.key) > newestBefore);
    const totalNow = await historyCommandsTotal();
    verdict("n1-commands-window-total-grows-by-one-per-edit", totalBefore !== null && totalNow !== null && totalNow - totalBefore === newer.length && added.length === 1, { totalBefore, totalAfter, totalNow, newRows: newer.map((entry) => `${entry.key}=${entry.label}${entry.expandable ? " (document)" : ""}`), documentRows: added.length, reading: "N1 / W1E-6: the Commands section is a tree window whose `total` counts every row, materialised or not — the total grows by exactly the rows that are newer than the read before the drag (the drag's one document row plus the chrome commands of the gesture and of the probe's own panel toggles, which are command rows without mutations)" });
  }
  const row = added.find((entry) => entry.label.startsWith(expected) || entry.text.includes(expected)) ?? added[0];
  verdict("row-labelled-from-the-drag-mutation", Boolean(row && (row.label === expected || row.label.startsWith(expected))), { expected, label: row?.label, text: row?.text?.slice(0, 160) });
  if (row) {
    ctx.dragEntry = row.id;
    const expanded = await expandEntry(row.id);
    const mutation = expanded.mutations.find((entry) => entry.label.startsWith(expected)) ?? expanded.mutations[0];
    ctx.dragMutation = mutation?.key;
    verdict("row-expands-to-its-drag-selection-mutation", Boolean(mutation && mutation.label.startsWith(expected)), { state: expanded.state, mutations: expanded.mutations.map((entry) => `${entry.key}=${entry.label}`).slice(0, 8) });
    verdict("s19-the-transaction-row-reads-its-intent-leaf", Boolean(mutation && row.label === mutation.label && !expanded.mutations.some((entry) => entry.key !== mutation.key && row.label === entry.label)), { row: row.label, intent: mutation?.label, children: expanded.mutations.map((entry) => entry.label).slice(0, 8), reading: "design §19.1: a tool transaction's row is labelled by its declared intent leaf (the drag), never by a structural support leaf that rides with it (proximity `connect-handles`); a one-leaf transaction holds it trivially" });
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
  const controlIds = (b?.controls ?? []).map((c) => [c.control, c.id ?? "", WGPU_BAND_CONTROLS.find(([name]) => name === c.control)?.[1] ?? ""] as const);
  verdict("band-controls-carry-their-control-ids", controlIds.length > 0 && controlIds.every(([, id, expected]) => id !== "" && id === expected), { controls: controlIds.map(([control, id, expected]) => ({ control, id, expected })), reading: "every band control carries the corpus `controlId` as its id on both renderers (React DOM id = wgpu mirror key, `TimeTravelControl::control_id`)" });
  await bandOverlapVerdict("band-overlaps-no-other-text");
  if (renderer !== "wgpu") {
    const seat = await bandOverlaps();
    verdict("band-sits-in-the-subfooter-bands", Boolean(seat?.inBottomBands && seat.inSubfooter), { seat, reading: "bands live in `[data-slot=layout-subfooter] > [data-semio-bottom-bands]` — the bands row under the footer (in flow, own surface; S5-UI r3)" });
  }
  const focus = await waitUntil(focusRead, focusIsOnAnEditorInput, 6000, 150);
  verdict("focus-moves-to-the-editor", focus.ok, { focus: focus.value, waitedMs: focus.waitedMs, reading: "design §16 / G13: a draft that starts puts keyboard focus on its first input (`timeTravelTransitionV1` → `editor`; React `timeTravelFocusElementV1`, wgpu `resolve_time_travel_focus`), never on its row" });
  const revealed = await waitUntil(editorRevealed, (state) => state.revealed, 6000, 200);
  const tabs = await openPanelTabIds();
  verdict("history-panel-reveals-on-session-start", tabs.includes(HISTORY_TAB) && revealed.ok, { tabs, revealed: revealed.value, waitedMs: revealed.waitedMs, reading: "the edge into a session opens the History panel with the band and the editor's first input in view (no probe scroll; React: inside the viewport and uncovered, wgpu: projected). In one tab a session can only begin from that panel's own Edit row action — a closed-panel begin needs the agent/MCP gateway, which this serve does not run" });
  const indicator = await indicators();
  verdict("windows-wear-the-time-travel-indicator", indicator.length > 0, { indicator: indicator.slice(0, 4) });
  const roster = await presenceText();
  verdict("presence-roster-shows-no-editing-peer", roster === null || !/⏪|is editing|bearbeitet .* im Verlauf|bearbeitet den Verlauf/.test(roster), { roster, reading: "one tab, no peer: the ⏪ badge and the \"is editing\" notes (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers`) must stay absent; a live second peer needs a presence transport (hub), which this serve does not run" });
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
  verdict("editor-shows-the-targets-reference-list", Boolean(ed?.targets && ed.targets.chips.length === 2 && ed.targets.chips.every((chip) => chip.trim().length > 0) && ed.targets.useSelection), { targets: ed?.targets, ids: [ctx.a, ctx.b], inputs: ed?.inputs, reading: "the reference row is a tree window: \"Use selection\" (`….targets.useSelection`), then one chip row per referenced id (`….targets.chip.<i>.row`) named by the entity's label (`ArtifactApp::entity_label`)" });
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
  if (typed.present && ctx.cMutation) {
    const copy = COPY[currentLocale];
    const cRow = await findMutationRow(ctx.cMutation);
    const state = cRow ? await rowActionState(cRow.id, copy.edit) : null;
    verdict("n15-edit-is-refused-while-a-changed-draft-is-open-naming-why", Boolean(state?.present && state.disabled === true && state.native !== true && state.reason.includes(copy.refusalBlocked)), { row: cRow?.label ?? null, state, expected: copy.refusalBlocked, reading: "gap N15: `TimeTravelSession::begin_refusal` is Blocked while the open draft differs from its original, so Edit on every other mutation row is disabled and names why (`RowAction::disabled_because` → React `aria-describedby`, wgpu the mirror button's description)" });
    verdict("n15-the-refusal-reason-is-visible", Boolean(state?.shown?.includes(copy.refusalBlocked)), { shown: state?.shown ?? null, reveal: state?.reveal ?? null, reason: state?.reason ?? null, reading: renderer === "wgpu" ? "wgpu: the mirror button's description (its painted tooltip is outside the DOM)" : "W1E-1: `[data-slot=row-action-reason][data-revealed]` shows the reason after a hover (≈ 400 ms), on keyboard focus and on a press, hides it on leave, Escape and blur, and is always the button's `aria-describedby`" });
  }
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
  const behind = renderer === "wgpu" ? null : (await readHistory()).find((row) => row.kind === "mutation" && row.key === ctx.cMutation);
  if (behind) {
    const refused = await waitUntil(() => rowActionState(behind.id, copy.edit, false), (read) => read.present && read.disabled === true, 5000, 200);
    const state = refused.value;
    verdict("n15-edit-is-refused-while-choosing-naming-why", state.present && state.disabled === true && state.native !== true && state.reason.includes(copy.refusalIllegal), { row: behind.label, state, waitedMs: refused.waitedMs, expected: copy.refusalIllegal, reading: "gap N15: Begin is illegal while the finalize prompt is open (Choosing), so every Edit behind it is disabled and names why" });
  } else note("n15-edit-while-choosing-not-observable", { reading: renderer === "wgpu" ? "the wgpu chrome projects only the modal prompt's nodes while it is open" : "the downstream mutation row is not materialised behind the prompt" });
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
    if (!(await mirrorAwait("framework.sync.folder", 6000))) {
      await wgpuPress("ui.utilities.group.sync");
      await mirrorAwait("framework.sync.folder", 6000);
    }
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
 * `persistedLocalOnly` folder binding the backbone worker reads and writes through the dev serve's `/semio-backbone`.
 * `afterPress` runs the moment Attach is pressed — a person turning to the History panel while the folder's archive loads —
 * instead of the settle and the Escape that close the card. */
const attachFolder = async (path: string, afterPress?: () => Promise<unknown>) => {
  if (renderer === "wgpu") {
    if (!(await openPanelTabIds()).includes("s-sync-status")) await wgpuPress("s-sync-status");
    await pickSyncFolder();
    const shown = await waitUntil(async () => mirrorFind(await mirror(), "framework.sync.folder.path") ?? null, (node) => node !== null, 10000, 300);
    const field = shown.value ? await mirrorLocator(shown.value.key, shown.value.window) : null;
    await field?.fill(path, { force: true, timeout: 4000 }).catch(() => {});
    const typed = field ? await field.inputValue({ timeout: 2000 }).catch(() => null) : null;
    await field?.press("Tab", { timeout: 2000 }).catch(() => {});
    const ready = await mirrorAwait("framework.sync.attach", 8000, true);
    const attached = ready ? await wgpuPress("framework.sync.attach") : null;
    if (afterPress) {
      await afterPress();
      return { card: shown.ok, typed, attachButton: attached !== null };
    }
    const active = await mirrorAwait("framework.sync.active", 10000);
    const after = { cardText: active ? active.label.slice(0, 200) : null, chip: mirrorFind(await mirror(), "s-sync-status")?.label ?? null, alerts: (await mirror()).filter((node) => node.key === "shell.notice").map((node) => `${node.description}: ${node.label}`.slice(0, 200)) };
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    return { card: shown.ok, typed, attachButton: attached !== null, after };
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
  if (afterPress) {
    await afterPress();
    return { card: shown.ok, typed, attachButton: attachCount > 0 };
  }
  await sleep(1500);
  const after = await evalSafe(() => {
    const field = document.querySelector<HTMLElement>('[id="framework.sync.folder.path"]');
    const card = field?.closest('[data-slot="panel"], [role="dialog"], [data-radix-popper-content-wrapper]') ?? field?.parentElement?.parentElement?.parentElement ?? null;
    const chip = document.querySelector<HTMLElement>('[id="s-sync-status"]');
    return {
      cardText: (card as HTMLElement | null)?.innerText.replace(/\s+/g, " ").trim().slice(0, 400) ?? null,
      chip: chip ? `${chip.getAttribute("aria-label") ?? ""} ${chip.innerText}`.replace(/\s+/g, " ").trim().slice(0, 120) : null,
      alerts: Array.from(document.querySelectorAll<HTMLElement>('[role="alert"], [data-notice-code]')).map((el) => `${el.getAttribute("data-notice-code") ?? el.getAttribute("role")}: ${(el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 200)}`).slice(0, 6),
    };
  }, { cardText: null as string | null, chip: null as string | null, alerts: [] as string[] });
  await shot(`sync-card-after-attach-${markToken()}`);
  await page.keyboard.press("Escape").catch(() => {});
  return { card: shown.ok, typed, attachButton: attachCount > 0, after };
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
  const listedBefore = await waitUntil(readAlternatives, (rows) => rows.length > 0, 40000, 2000);
  const alternativesBefore = listedBefore.value;
  if (!listedBefore.ok) note("no-alternatives-listed-before-the-reload", { waitedMs: listedBefore.waitedMs, commits: historyBefore.filter((row) => row.kind === "entry" && /Commit Checkpoint|Kontrollpunkt/i.test(row.label)).length, reading: "the History body shows its Alternatives section (the main line) only once the document holds a checkpoint; a folder-bound document commits one by itself about 20 s after an edit, and none came within the wait — the before/after comparison of the alternatives cannot be judged for this document" });
  else if (listedBefore.waitedMs > 3000) note("alternatives-listed-after-the-first-checkpoint", { waitedMs: listedBefore.waitedMs, rows: alternativesBefore.map((row) => row.text.slice(0, 50)) });
  const currentBefore = alternativesBefore.find(isCurrentAlternative);
  const g4Before = ctx.g4 ? (await findMutationRow(ctx.g4.drag))?.text ?? null : null;
  emit({ kind: "alternatives", tag: "before-reload", rows: alternativesBefore, current: currentBefore?.key ?? null, g4: g4Before });
  await closePanels();
  await harvestNotices();
  expectedReloads += 1;
  probeReloadAt = Date.now();
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
    await openHistory();
    await armReprojection("observe");
    const via = renderer === "wgpu" ? await wgpuPress("s-folder-reconnect") : await page.locator("#s-folder-reconnect").click({ timeout: 4000 }).then(() => "button").catch(() => null);
    const busy = await waitUntil(reconnectBand, (state) => state === null || state.state === "reconnecting", 10000, 100);
    const settled = await waitUntil(reconnectBand, (state) => state === null, 60000, 500);
    await reprojectionSettled(60000);
    const load = reprojectionSummary(await disarmReprojection());
    reattached = { via, sawBusy: busy.value?.state === "reconnecting", bandGone: settled.ok, waitedMs: settled.waitedMs, load };
    verdict("folder-reconnect-attaches-and-closes-the-band", Boolean(via) && settled.ok, reattached);
    if (load.seen) {
      verdict("stepped-load-shows-its-progress", load.kinds.includes("load") && (load.total ?? 0) > 0, { load, reading: "§20.8: a whole-document load rides the history wire (`HistoryPatch.reprojection {kind: load}`) — \"Document load\" / \"Loading document: d of t\" in the shell status and the body's `framework.history.reprojection` section" });
      reprojectionShellVerdicts(load, "-load");
    } else note("stepped-load-finished-within-one-refresh", { load, reading: "the reconnect's archive load finished before anything re-rendered (progress refreshes ≥ 100 ms); step 18 loads a grown archive" });
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
  if (alternativesBefore.length > 0) verdict("alternatives-survive-the-reload", missingAlternatives.length === 0, { before: alternativesBefore.map((row) => `${row.key}=${row.text.slice(0, 50)}`), after: alternativesAfter.map((row) => `${row.key}=${row.text.slice(0, 50)}`), missing: missingAlternatives.map((row) => row.key) });
  verdict("main-line-listed-after-the-reload", alternativesAfter.some((row) => row.text.includes(COPY[currentLocale].trunk)), { after: alternativesAfter.map((row) => row.text.slice(0, 50)), expected: COPY[currentLocale].trunk });
  if (alternativesBefore.length > 0) verdict("current-alternative-restored-after-the-reload", Boolean(currentBefore && currentAfter && currentAfter.key === currentBefore.key), { before: currentBefore?.key ?? null, after: currentAfter?.key ?? null });
  if (ctx.g4) {
    const g4After = (await findMutationRow(ctx.g4.drag))?.text ?? null;
    atStep(13, () => verdict("warning-row-survives-the-reload", Boolean(g4After?.includes(COPY[currentLocale].warningPartial)), { before: g4Before, after: g4After, expected: COPY[currentLocale].warningPartial, reading: "G4: the warning an edit introduced persists in history through finalize and a folder reload + re-attach" }));
  }
  await shot("reloaded");
  await closePanels();
  const detached = await detachFolder();
  await harvestNotices();
  expectedReloads += 1;
  probeReloadAt = Date.now();
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
  probeReloadAt = Date.now();
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
  if (renderer === "wgpu") {
    const mirrored = async () => (await mirror()).filter((node) => !node.key.includes("::") && /(^|[\/␟\u001f])framework\.history\.alternative\.[^\/␟\u001f]+$/.test(node.key)).map((node) => ({ id: node.key, key: node.key.replace(/^.*framework\.history\.alternative\./, ""), text: `${node.label} ${node.description}`.trim(), active: node.selected === "true" || node.pressed === "true" }));
    for (const edge of ["start", "end"] as const) {
      await scrollHistory(edge);
      await sleep(500);
      const rows = await mirrored();
      if (rows.length) return rows;
    }
    return [];
  }
  await scrollHistory("start");
  await sleep(250);
  await revealHistory("framework.history.alternatives");
  await waitUntil(() => treeWindow("framework.history.alternatives"), (read) => read === null || read.total === 0 || read.length > 0, 3000, 150);
  return evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id]"))
        .filter((el) => /(^|[\/␟])framework\.history\.alternative\.[^\/␟]+$/.test(el.id))
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
const switchAlternative = async (row: AlternativeRow) => pressRowAction(row.id, COPY[currentLocale].switchAction);

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
  const reachable = verdict(`dy-stepper-control-reachable${suffix}`, typed.present && typed.typed === String(dyValue), { typed, authored: "framework.history.editor.input.dy" });
  if (!reachable) await diagnose(`dy-unreachable${suffix}`);
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
  await revealWindowRow("framework.history.editor.inputs", "framework.history.editor.input.index.row");
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
  const blockedControls = blocked.value?.controls ?? [];
  const refusedControls = blockedControls.filter((c) => c.disabled);
  verdict("band-offers-next-problem-first-while-blocked", blockedControls[0]?.control === "nextProblem" && blockedControls[0].disabled === false, { controls: blockedControls.map((c) => `${c.control}${c.disabled ? " (refused)" : ""}`), reading: "design §22.2: while the review blocks, the band offers Next problem before every other control (React `data-semio-time-travel-control=nextProblem`, wgpu `shell.time-travel.next-problem`)" });
  verdict("refused-band-controls-stay-reachable-and-name-why", refusedControls.length > 0 && refusedControls.every((c) => c.native !== true && Boolean(c.title)), { refused: refusedControls, reading: "a refused band control is `aria-disabled` with its reason as `aria-describedby` → `[data-slot=row-action-reason]` (wgpu: the mirror node's description), never the native `disabled` attribute, so keyboard and assistive technology reach the refusal" });
  const blockingRead = await waitUntil(() => mutationRowRevealed(dragRow!.key), (read) => read.revealed, 4000, 200);
  const blockingRow = blockingRead.value;
  const blockingWindow = renderer === "wgpu" ? null : await evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).filter((el) => /framework\.history\.(commands|entry\.\d+)$/.test(el.getAttribute("data-tree-window-key") ?? "")).map((el) => `${el.getAttribute("data-tree-window-key")}: total ${el.getAttribute("data-tree-window-total")}, offset ${el.getAttribute("data-tree-window-offset")}, length ${el.getAttribute("data-tree-window-length")}`).slice(0, 8), [] as string[]);
  verdict("the-first-blocking-row-is-revealed-on-a-blocked-review", blockingRow.revealed, { ...blockingRow, waitedMs: blockingRead.waitedMs, windows: blockingWindow, expected: `framework.history.mutation.${dragRow.key}`, reading: "on the edge into a blocked review the host opens the History panel and scrolls to the first blocking row — read before the probe scrolls anything" });
  await bandOverlapVerdict("band-overlaps-no-other-text-while-blocked");
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

/** 📟️ Step 9 — the locale's console: uncaught page errors and hard guest faults fail; errors, warnings and lines with
 * AGENTS.md's temporary-log tag are digested with their most frequent texts. */
const step9 = async (_ctx: Ctx) => {
  const mine = consoleRows.filter((row) => row.locale === currentLocale);
  const errors = mine.filter((row) => row.type === "error" && !BENIGN_RE.test(row.text));
  const warnings = mine.filter((row) => row.type === "warning" && !BENIGN_RE.test(row.text));
  const debug = mine.filter((row) => /\[DEBUG\] /u.test(row.text));
  const http = mine.filter((row) => row.type === "http");
  const top = (rows: ConsoleRow[]) => [...rows.reduce((map, row) => map.set(row.text.slice(0, 160), (map.get(row.text.slice(0, 160)) ?? 0) + 1), new Map<string, number>())].sort((x, y) => y[1] - x[1]).slice(0, 12);
  const refusals = [...new Set(mine.map((row) => /refused a local batch \S+ ((?:local|sync)\.[\w.-]+)/.exec(row.text)?.[1]).filter((code): code is string => Boolean(code)))];
  const shownCodes = await shownNoticeCodes();
  const notices = await shownNotices();
  const copy = COPY[currentLocale];
  const full = notices.filter((row) => row.code === "history.full");
  if (full.length) verdict("history-full-notice-names-the-edit-count", full.every((row) => Number(copy.historyFull.exec(row.text)?.[1] ?? "0") > 0), { full, reading: "`HistoryPatch.editCount` is the `{n}` this notice names, carried as data (never digits read from a fault message)" });
  else note("no-history-full-notice", { reading: "the paged edit ledger admits far more edits than a run makes, so `HistoryPatch.editCount` shows only as this notice's `{n}`; its folding from every patch is the React law `folds the history's edit count from every patch` (W2-B suite)" });
  const laneWords: Record<string, string> = { "history.replaying": copy.noticeReplaying, "document.loading": copy.noticeLoading };
  const lane = notices.filter((row) => row.code in laneWords);
  if (lane.length) verdict("history-lane-notices-speak-the-locale", lane.every((row) => row.text.includes(laneWords[row.code]!)), { lane, expected: laneWords, reading: "kernel `HISTORY_NOTICE_LABELS` (`🧫️history-notices`): a refused history-lane command reaches the person in the UI locale, by code" });
  note("notices-shown", { notices: notices.slice(0, 24) });
  const faults = await shownBandFaults();
  if (faults.length) verdict("band-never-shows-a-raw-fault-code", faults.every((row) => row.text !== "" && (row.code === null ? false : !row.text.includes(row.code)) && (row.code === null || row.code.startsWith("timeTravel.") ? true : row.text.includes(copy.replayFaulted))), { faults, expected: copy.replayFaulted, reading: renderer === "wgpu" ? "the wgpu band's status line must never carry a fault code (any `<scope>.<code>` token here is one)" : "the band reads a known fault in its own words and any other as \"Replay failed: later mutations could not be checked\"; the code is only `data-semio-time-travel-fault`" });
  else note("no-band-fault-in-this-run", { reading: "no replay fault or cancel showed on a band (step 16's Cancel shows `timeTravel.cancelled` as \"Replay cancelled\")" });
  const restores = notices.filter((row) => row.code === "shell.document-restore");
  verdict("no-document-restore-fails", restores.length === 0, { alerts: restores, reading: renderer === "wgpu" ? "React-only observation (the wgpu shell's restore alert is not projected to the probe); none recorded" : "the shell's restore alert (`[data-semio-bootstrap-status][role=alert]`, \"Document restore failed: <fault>\") means an archive the document read back was rejected — the shell then retires the document's port, so nothing the page publishes afterwards is persisted or reaches a peer" });
  const mismatch = notices.filter((row) => row.code === "plugin.channel-mismatch");
  const mismatchLines = mine.filter((row) => /plugin\.channel-mismatch|speaks app channel \d+, the host app channel \d+/u.test(row.text)).map((row) => row.text.slice(0, 300));
  verdict("no-plugin-is-refused-for-a-channel-mismatch", mismatch.length === 0 && mismatchLines.length === 0, { mismatch, lines: mismatchLines.slice(0, 6), reading: "S4-BUMP's guest↔host channel handshake (`admit_guest_channel_version`): a component built for another app channel is refused before its first frame — on a fresh activation none is" });
  if (mismatch.length) verdict("channel-mismatch-notice-is-localized", mismatch.every((row) => copy.channelMismatch.test(row.text)), { mismatch, reading: "kernel `🧫️framework-notices`: the refusal names both channels in the UI locale, never the raw code" });
  if (!refusals.length) note("no-command-rejection-in-this-run", { shownCodes });
  else verdict("rejection-notices-carry-their-code", refusals.every((code) => shownCodes.includes(code)), { refusals, shownCodes, reading: renderer === "wgpu" ? "wgpu projects the transient notice as the polite `shell.notice` mirror node described by its code (`🧯️wgpu-transient-notice`); the main page's trace saw these codes" : "React `[data-notice-code]` over the main page's life" });
  const uncaught = pageErrors.filter((row) => row.locale === currentLocale);
  verdict("no-uncaught-page-errors", uncaught.length === 0, { count: uncaught.length, first: uncaught.slice(0, 3) });
  const hard = hardFaults.filter((row) => row.locale === currentLocale);
  verdict("no-hard-guest-faults", hard.length === 0, { count: hard.length, first: hard.slice(0, 3) });
  verdict("console-is-debug-free", debug.length === 0, { count: debug.length, top: top(debug).slice(0, 6), reading: "AGENTS.md: temporary-log tagged lines are removed before a run counts" });
  const folderRequests = mine.filter((row) => row.type === "backbone").map((row) => `${row.t}s step ${row.step} ${/HTTP \d+ \w+/u.exec(row.text)?.[0] ?? row.text.slice(0, 24)}`);
  if (folderRequests.length) note("folder-requests-of-the-run", { requests: folderRequests.slice(0, 60), puts: folderRequests.filter((row) => row.includes("PUT")).length, reading: "every folder request of this locale's pages with its run-clock time and step: a PUT is the page writing the archive, a GET 204 an empty folder, a GET 200 an archive read" });
  if (errors.length || uncaught.length) note("console-errors-in-full", { errors: errors.slice(0, 8).map((row) => `${row.t}s step ${row.step} ${row.source}: ${readableUrlText(row.text).slice(0, 1400)}`), uncaught: uncaught.slice(0, 4).map((row) => `step ${row.step}: ${readableUrlText(row.text).slice(0, 900)}`) });
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

/** 🎛️ Runs one arg-carrying app action the way a person reaches it. React: the Overview window's Actions rail (its header toggle
 * `framework.window.<id>.engagement.toggle` → the row `action.<id>` → the staged form → Execute
 * `framework.window.<id>.action.<id>.execute` with the staged defaults); the palette (`mod+p`) lists shell, plugin, app and mode
 * COMMANDS, panels and windows only — a window action (`resolveWindowActions`, flag `inPalette`) lives in its window's rail on
 * both renderers. wgpu: the same rail through the mirror (`framework.window.2dOverview.engagement.toggle` → the treeitem
 * `<window>/framework.section.engagements/action.<id>` → the form's Execute); the rail's keys are calibrated on the first live
 * wgpu run, the form's Execute key is matched by suffix until a board with nodes exists to open one. */
const runPaletteCommand = async (query: string, label: RegExp, commandId: string) => {
  if (renderer === "wgpu") {
    await closePanels();
    const railRow = async () => (await mirror()).find((node) => node.key.endsWith(`/action.${commandId}`) || node.key === `action.${commandId}`) ?? null;
    if (!(await railRow())) await wgpuPress("framework.window.2dOverview.engagement.toggle");
    const row = await waitUntil(railRow, (node) => node !== null, 10000, 300);
    if (row.value) await mirrorActivate(row.value.key, row.value.window);
    const execute = await waitUntil(async () => (await mirror()).find((node) => node.role === "button" && new RegExp(`(^|[./])action\\.${commandId}\\.execute$|${commandId}[-.]execute$`, "u").test(node.key)) ?? null, (node) => node !== null, 10000, 300);
    const staged = (await mirror()).filter((node) => node.key.includes(`action.${commandId}.arg.`) || node.key.includes(`${commandId}.arg.`)).map((node) => `${node.key.replace(/^.*\.arg\./, "")}=${node.value ?? node.valueNow ?? node.label}`);
    if (execute.value && !execute.value.disabled) await mirrorActivate(execute.value.key, execute.value.window);
    await sleep(600);
    if (await railRow()) await wgpuPress("framework.window.2dOverview.engagement.toggle");
    return { palette: row.ok, item: row.value?.label ?? null, execute: execute.value?.key ?? null, disabled: execute.value?.disabled ?? null, staged, inventory: execute.ok ? null : (await mirror()).filter((node) => /engagement|action\./u.test(node.key)).map((node) => `${node.key}|${node.role}`).slice(0, 40) };
  }
  await closePanels();
  const pane = page.locator(`[data-slot="window"][id="${OVERVIEW}"]`).first();
  const rail = pane.locator('[data-slot="window-action-pane"]').first();
  const toggle = pane.locator('[id$=".engagement.toggle"]').first();
  if (!(await rail.isVisible().catch(() => false))) await toggle.click({ timeout: 4000 }).catch((error) => log(`actions rail toggle failed ${String(error).split("\n")[0]}`));
  const row = pane.locator(`[id="action.${commandId}"]`).first();
  const opened = await waitUntil(() => row.count().catch(() => 0), (count) => count > 0, 30000);
  if (opened.waitedMs > 3000) note("actions-rail-opened-late", { commandId, waitedMs: opened.waitedMs, opened: opened.ok, reading: "the window's Actions rail showed its rows this long after its header toggle was pressed" });
  const itemText = opened.ok ? ((await row.innerText({ timeout: 2000 }).catch(() => null))?.replace(/\s+/g, " ").trim() ?? null) : null;
  if (opened.ok) {
    await row.scrollIntoViewIfNeeded({ timeout: 3000 }).catch(() => {});
    const rowLabel = row.locator('[data-slot="tree-label"]').first();
    await ((await rowLabel.count().catch(() => 0)) ? rowLabel : row).click({ timeout: 4000 }).catch((error) => log(`actions rail item click failed ${String(error).split("\n")[0]}`));
  }
  const execute = pane.locator(`[id$=".action.${commandId}.execute"]`).first();
  const shown = await waitUntil(() => execute.count().catch(() => 0), (count) => count > 0, 10000);
  const executeId = shown.ok ? await execute.getAttribute("id").catch(() => null) : null;
  const staged = await evalSafe((id) => Array.from(document.querySelectorAll<HTMLElement>(`[id^="action.${id}.arg."]`)).filter((el) => !el.id.endsWith(".disclosureLabel")).map((el) => `${el.id.replace(/^.*\.arg\./, "")}=${(el.querySelector("input") as HTMLInputElement | null)?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? el.querySelector('[role="combobox"]')?.textContent?.trim() ?? el.innerText.replace(/\s+/g, " ").trim().slice(0, 40)}`), [] as string[], commandId);
  const disabled = shown.ok ? await execute.isDisabled({ timeout: 2000 }).catch(() => null) : null;
  if (shown.ok) {
    await execute.scrollIntoViewIfNeeded({ timeout: 3000 }).catch(() => {});
    await execute.click({ timeout: 4000 }).catch((error) => log(`execute click failed ${String(error).split("\n")[0]}`));
    await sleep(400);
  }
  const inventory = shown.ok ? null : await evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>('[data-slot="window-action-pane"] [id]')).map((el) => el.id).slice(0, 60), [] as string[]);
  if (await rail.isVisible().catch(() => false)) await toggle.click({ timeout: 4000 }).catch(() => {});
  return { palette: opened.ok, item: itemText && label.test(itemText) ? itemText : itemText === null ? null : `${itemText} (label mismatch)`, execute: executeId, disabled, staged, inventory };
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
        const dialAgain = await waitUntil(() => readNumberControl("angle"), (control) => control !== null && control.valueNow !== null, 15000, 300);
        if (!dialAgain.ok) await diagnose("rotate-second-session-no-editor");
        const keyed = await keyEditorNumber("angle", "PageUp");
        const half = await waitUntil(turnedBy(180), (ok) => ok, 10000, 300);
        if (!half.ok) await diagnose("rotate-second-session-no-preview");
        const acceptVia = await pressBand("accept");
        const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
        if (!review.ok) await diagnose("rotate-second-session-no-review");
        const finalized = review.value?.review === "ready" ? await finalizeOverwrite() : null;
        const head = await waitUntil(turnedBy(180), (ok) => ok, 15000, 400);
        verdict("g6-rotate-edit-finalizes-the-new-angle", half.ok && review.value?.review === "ready" && Boolean(finalized?.closed) && head.ok, { secondSession: { via: again.via, waitedMs: again.waitedMs, dial: dialAgain.value?.valueNow ?? null, dialWaitedMs: dialAgain.waitedMs, keyed, acceptVia, stageAfterAccept: review.value?.stage ?? null }, preview180: half.ok, review: review.value?.review, finalized, a: head.value ? (await positions())[ids[0]] : null, expected: rotatedAbout(before[ids[0]], pivot, 180), reading: "PageUp to the next detent (180°) → Accept → Finalize → Overwrite: the rotation is replayed about its recorded pivot" });
        const relabelledRow = await waitUntil(() => findMutationRow(rotateRow.mutation!.key), (row) => Boolean(row?.label.startsWith(copy.rotate(2, labelNumber(180, currentLocale)))), 12000, 500);
        const relabelled = relabelledRow.value;
        verdict("g6-rotate-row-reads-the-edited-angle", Boolean(relabelled?.label.startsWith(copy.rotate(2, labelNumber(180, currentLocale)))), { label: relabelled?.label, text: relabelled?.text?.slice(0, 120), waitedMs: relabelledRow.waitedMs, expected: copy.rotate(2, labelNumber(180, currentLocale)) });
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
  const relabelledRow = await waitUntil(() => findMutationRow(scaleRow.mutation!.key), (row) => Boolean(row?.label.startsWith(copy.scale(2, labelNumber(2, currentLocale)))), 12000, 500);
  const relabelled = relabelledRow.value;
  verdict("g6-scale-row-reads-the-edited-factor", Boolean(relabelled?.label.startsWith(copy.scale(2, labelNumber(2, currentLocale)))), { label: relabelled?.label, text: relabelled?.text?.slice(0, 120), waitedMs: relabelledRow.waitedMs, expected: copy.scale(2, labelNumber(2, currentLocale)) });
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
  const replayedHead = await waitUntil(positions, (now) => placed(now, p.id, beforeDrag[p.id], 0, 0, 0.05) && placed(now, q.id, beforeDrag[q.id], offset[0], offset[1], 0.05), 10000, 200);
  const replayed = replayedHead.value;
  verdict("g4-the-replay-skips-the-locked-member", replayedHead.ok, { p: replayed[p.id], pBefore: beforeDrag[p.id], q: replayed[q.id], qExpected: [beforeDrag[q.id][0] + offset[0], beforeDrag[q.id][1] + offset[1]], waitedMs: replayedHead.waitedMs });
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
    const under = renderer === "wgpu" ? null : await evalSafe((at) => {
      const el = document.elementFromPoint(at.x, at.y);
      return el ? `${el.tagName.toLowerCase()} in ${el.closest("[data-slot]")?.getAttribute("data-slot") ?? "no slot"} / ${el.closest('[data-slot="window"], [data-slot="panel"]')?.id ?? "no window"}` : null;
    }, null as string | null, m.at);
    await page.touchscreen.tap(m.at.x, m.at.y);
    const tapped = await waitUntil(vitals, (v) => selectionIds(v).includes(m.id), 10000);
    const p0 = await positions();
    await dragBy(m.at, 40 * camera.zoom, 0);
    const moved = await waitUntil(positions, (p) => offsetOf(p0, p, m.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    const offset = offsetOf(p0, moved.value, m.id);
    verdict(`${key}-tap-selects-and-a-drag-moves-the-node`, tapped.ok && moved.ok, { node: m.id, offset, selection: selectionIds(tapped.value), tapWaitedMs: tapped.waitedMs, underTheTap: under, reading: "`underTheTap` is the element at the tap point just before the tap: a tap that selects nothing while a panel still covers the node is the probe's timing, one on the board canvas a dropped tap" });
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
    await bandOverlapVerdict(`${key}-band-overlaps-no-other-text`);
    await ariaVerdict(`${key}-aria-band-and-history-have-no-structural-findings`, "session");
    const dxNew = Math.round(offset[0] + 20);
    const typed = await typeEditorNumber("dx", dxNew);
    const previewed = await waitUntil(positions, (p) => placed(p, m.id, p0[m.id], dxNew, offset[1], 0.05), 15000);
    const dxId = renderer === "wgpu" ? null : await resolveDomId("framework.history.editor.input.dx");
    const dxBox = dxId ? await byId(dxId).boundingBox({ timeout: 2000 }).catch(() => null) : null;
    verdict(`${key}-editor-input-reachable-by-keyboard`, typed.present && previewed.ok && (renderer === "wgpu" || insideViewport(dxBox)), { typed, dxBox, a: previewed.value[m.id] });
    if (renderer === "wgpu") note(`${key}-a-tap-on-a-refused-edit-tells-its-reason`, { reading: "wgpu paints its tooltip; the mirror description carries the reason (step 4)" });
    else {
      const edited = typed.present ? await findMutationRow(row.mutation.key) : null;
      const tapped = edited ? await tapRevealsReason(edited.id, copy.edit) : null;
      verdict(`${key}-a-tap-on-a-refused-edit-tells-its-reason`, Boolean(tapped?.tapped && tapped.disabled && tapped.shown?.includes(copy.refusalBlocked)), { row: edited?.label ?? null, tapped, expected: copy.refusalBlocked, reading: "W1E-1 on touch: with a changed draft open every Edit is refused (Blocked); a tap on one reveals its reason (`[data-slot=row-action-reason][data-revealed]`)" });
    }
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
  if (!found) return { entry: null as HistoryRow | null, mutations: [] as HistoryRow[], window: null as TreeWindowRead | null };
  const expanded = await expandEntry(found.id);
  return { entry: found as HistoryRow | null, mutations: expanded.mutations, window: await treeWindow(`framework.history.entry.${found.key}`) };
};

/** 🔚️ The LAST mutation row of the history row `entry` (gap N1). React scrolls the row's tree window until it materialises its
 * last row (`offset + length = total`) and answers the window. wgpu, whose mirror publishes no window size, pages the body down
 * and keeps, in reading order, every mutation row between `entry` and the next history row — across pages, so a page that
 * starts inside the row (its entry scrolled off) still counts — until `expected` were seen or nothing new shows. */
const lastMutationOf = async (entry: HistoryRow, expected: number) => {
  if (renderer !== "wgpu") {
    const end = await reachWindowEnd(`framework.history.entry.${entry.key}`);
    const rows = (await readHistory()).filter((row) => row.kind === "mutation" && row.parent === entry.id);
    const last = rows[rows.length - 1] ?? null;
    const lastIndex = last ? await evalSafe((id) => {
      const el = document.getElementById(id);
      const index = el?.getAttribute("data-tree-window-row") ?? el?.closest("[data-tree-window-row]")?.getAttribute("data-tree-window-row") ?? el?.querySelector("[data-tree-window-row]")?.getAttribute("data-tree-window-row");
      return index === null || index === undefined ? null : Number(index);
    }, null as number | null, last.id) : null;
    return { total: end?.total ?? null, atEnd: end?.atEnd ?? false, scrolls: end?.scrolls ?? 0, seen: rows.length, last, lastIndex, createNodes: rows.filter((row) => COPY[currentLocale].createNodeRow.test(row.label)).length };
  }
  const seen = new Map<string, HistoryRow>();
  let inside = false;
  let idle = 0;
  for (let page = 0; page < 80 && seen.size < expected && idle < 4; page++) {
    const before = seen.size;
    const rows = await readHistory();
    for (const row of rows) {
      if (row.kind === "entry") inside = row.id === entry.id;
      else if (inside) seen.set(row.key, row);
    }
    idle = seen.size > before ? 0 : idle + 1;
    if (!(await scrollHistoryBy(0.8))) break;
    await sleep(400);
  }
  const rows = [...seen.values()];
  return { total: null as number | null, atEnd: seen.size >= expected, scrolls: 0, seen: rows.length, last: rows[rows.length - 1] ?? null, lastIndex: rows.length ? rows.length - 1 : null, createNodes: rows.filter((row) => COPY[currentLocale].createNodeRow.test(row.label)).length };
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
  const target = armLog?.target ?? [];
  const refusedAt = target.find((row) => row.stage === "replaying" && row.disabled === true) ?? null;
  return { sawReplaying: replaying.length > 0, frames: replaying.length, total: totals.length ? Math.max(...totals) : null, firstReplayingAt: replaying[0]?.t ?? null, reviewedAt, acted: armLog?.acted ?? null, samples: samples.slice(0, 40), target: target.slice(0, 24), disabledWhileReplaying: refusedAt ? { afterMs: refusedAt.t - (replaying[0]?.t ?? refusedAt.t), reason: refusedAt.reason } : null };
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
    const nodes = board.value?.nodes ?? 0;
    const end = tx.entry ? await lastMutationOf(tx.entry, created) : null;
    const reachable = renderer === "wgpu" ? Boolean(end?.atEnd && end.last && end.createNodes >= nodes) : Boolean(tx.window && tx.window.total >= created && end?.atEnd && end.last && end.total !== null && end.lastIndex === end.total - 1);
    verdict("g9-every-mutation-of-the-long-transaction-is-reachable", reachable, { entry: tx.entry?.label ?? null, window: tx.window, end: end ? { ...end, last: end.last?.label ?? null } : null, createdAtLeast: created, firstRows: tx.mutations.map((row) => row.label).slice(0, 6), reading: renderer === "wgpu" ? "acceptance item 1 / N1: the wgpu mirror publishes no window size, so the body is paged until the row's mutations seen reach one create-node per node (counted by label) and nodes + edges in all" : "acceptance item 1 / N1: the example's row is a tree window whose `total` counts every mutation (≥ one create-node per node + one connect-handles per edge); scrolled to its end it materialises row `total − 1` (`data-tree-window-row`) — W1E-6: counts come from `window.total`, never from DOM rows" });
    if (!tx.entry || !tx.mutations.length) return;
    if (end?.last) {
      const lastRow = end.last;
      const via = await pressRowAction(lastRow.id, copy.edit);
      const opened = await waitUntil(band, (b) => b?.stage === "editing" && (b.target ?? b.text).includes(lastRow.label.slice(0, 12)), 20000);
      verdict("n1-the-last-mutation-of-the-long-transaction-opens-for-editing", opened.ok, { via, row: lastRow.label, key: lastRow.key, band: opened.value?.text?.slice(0, 160) });
      if (opened.value) await exitSession();
    }
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
      rounds.push({ round, kind: session.kind, via, review: settled.value?.review ?? null, ...first, target: session.target.label, samples: first.samples.slice(0, 12) });
      const cancelLandedInTime = first.acted?.via === "cancelReplay" && settled.value?.review === "needsReplay";
      if ((first.sawReplaying && cancelLandedInTime) || round === LONG_HISTORY_GROWTH_ROUNDS) break;
      note("g9-replay-too-short-to-cancel", { round, sawReplaying: first.sawReplaying, total: first.total, replayMs: first.firstReplayingAt !== null && first.reviewedAt !== null ? first.reviewedAt - first.firstReplayingAt : null, acted: first.acted, review: settled.value?.review ?? null, reading: "the replay finished before a Cancel pressed in its first rendered frame reached the guest (or before any frame showed it); the history grows by one example load and the step retries" });
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
      const noticed = second.acted?.disabled === true ? null : await waitUntil(shownNoticeCodes, (shown) => shown.includes("timeTravel.illegal"), 2500, 250);
      const codes = await shownNoticeCodes();
      const editedAfter = (second.samples ?? []).some((sample) => sample.stage === "editing" && sample.t > (second.acted?.t ?? Number.POSITIVE_INFINITY));
      const replayMs = second.firstReplayingAt !== null && second.reviewedAt !== null ? second.reviewedAt - second.firstReplayingAt : null;
      verdict("g9-edit-during-replay-is-refused", second.acted !== null && second.acted.via !== "absent" && (second.acted.disabled === true || codes.includes("timeTravel.illegal")) && !editedAfter, { started, acted: second.acted, codes, noticeWaitedMs: noticed?.waitedMs ?? null, editedAfter, frames: second.frames, replayMs, disabledWhileReplaying: second.disabledWhileReplaying, target: second.target, row: otherRow.label, reading: "design §4: Begin is legal only from Inactive and Reviewing — while Replaying the press is refused (`timeTravel.illegal`, a warning notice) or the Edit action is disabled (N15); a press that neither opens a session nor is refused in words is a silently dropped command" });
      const refusedFrames = second.target.filter((row) => row.stage === "replaying" && row.disabled === true && !row.how.includes("aria-busy"));
      const pendingFrames = second.target.filter((row) => row.disabled === true && row.how.includes("aria-busy"));
      if (second.acted && second.acted.via !== "absent") verdict("n15-edit-is-disabled-while-replaying-naming-why", second.acted.via === "action" && refusedFrames.length > 0 && refusedFrames.every((row) => row.reason.includes(copy.refusalIllegal) && !row.how.includes("disabled attribute")), { judgedOn: `every sampled frame of the replay in which the Edit action reads disabled by the guest (aria-disabled, never the native attribute, not aria-busy) with the reason as its description. The press is a settled one: once the action reads disabled, else ${EDIT_SETTLE_MS} ms after the band turned replaying. Frames that read aria-busy are the host's pending state of the probe's own press and are listed, not judged; no guest-disabled frame at all means Edit stayed enabled for the settle time of a running replay`, acted: second.acted, replayMs, refusedFrames: refusedFrames.length, pendingFrames: pendingFrames.length, disabledWhileReplaying: second.disabledWhileReplaying, target: second.target, expected: copy.refusalIllegal, reading: "gap N15: while a replay runs every Edit row action is disabled (`RowAction::disabled_because`) and names why — an aria-disabled button with its reason as the accessible description, never an enabled control whose press is refused afterwards" });
      const firstReplaying = second.target.find((row) => row.stage === "replaying") ?? null;
      if (second.acted && second.acted.via !== "absent") verdict("n15-edit-reads-disabled-with-its-reason-in-the-first-replaying-frame", firstReplaying !== null && firstReplaying.disabled === true && firstReplaying.reason.includes(copy.refusalIllegal) && !firstReplaying.how.includes("disabled attribute"), { firstReplayingFrame: firstReplaying, pressSettledMs: second.acted.settledMs, pressedDisabled: second.acted.disabled, reasonAtThePress: second.acted.reason, target: second.target.slice(0, 8), expected: copy.refusalIllegal, reading: "the unsettled read: the page's MutationObserver samples the Edit row action in the very DOM commit in which the band first reads `replaying` — the action must read aria-disabled with the reason as its description in that same frame (S5-UI `f3`: row actions are refused from the session stage)" });
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
    const aRowRead = await waitUntil(() => mutationLabelled(dragLabel(1, aOff[0], aOff[1])), (row) => row !== null, 15000, 750);
    const aRow = aRowRead.value;
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
    const bSeesARead = await waitUntil(() => mutationLabelled(dragLabel(1, aOff[0], aOff[1])), (row) => row !== null, 20000, 1000);
    const bSeesA = bSeesARead.value;
    verdict("g10-second-peer-opens-the-shared-document", attachedB.typed === folder && shared.ok && Boolean(bSeesA), { rowWaitedMs: bSeesARead.waitedMs, attachedB, drift: movedIds(headA, shared.value, 1e-6).slice(0, 6), driftCount: movedIds(headA, shared.value, 1e-6).length, row: bSeesA?.label ?? null, reading: "attaching a folder that holds an archive reads it (`documentArchiveReplaced`): B now shows A's document and A's drag row" });
    use(a);
    await installBandTrace();
    const begun = await beginEditOf(aRow.key);
    verdict("g10-first-peer-begins-a-history-edit", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
    if (begun.band?.stage !== "editing") return;
    await armReprojection("observe");
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
    await openHistory();
    await armReprojection("observe");
    use(a);
    const bLabel = dragLabel(1, bOff[0], bOff[1]);
    const arrived = await waitUntil(() => mutationLabelled(bLabel), (row) => row !== null, 45000, 1500);
    verdict("g10-a-remote-edit-arrives-while-editing", arrived.ok, { expected: bLabel, row: arrived.value?.text?.slice(0, 160) ?? null, waitedMs: arrived.waitedMs, reading: "B's write reaches A over the folder's change stream; A's store takes the new history while the session edits" });
    const during = await band();
    verdict("g10-the-session-survives-the-base-move", during?.stage === "editing" && Boolean((during.target ?? during.text).includes(aRow.label)), { stage: during?.stage, target: during?.target, text: during?.text?.slice(0, 160), reading: "design §7: remote ingests keep arriving while frozen → BaseMoved; in Editing the preview is re-shown, the draft stays" });
    const whileEditing = reprojectionSummary(await disarmReprojection());
    if (whileEditing.seen) reprojectionShellVerdicts(whileEditing, "-during-a-session");
    else note("g10-the-base-move-replayed-within-one-refresh", { whileEditing });
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
    const converged = await waitUntil(positions, (p) => placed(p, n1, p0[n1], dxNew, aOff[1], 0.05), 45000, 1000);
    const adopted = reprojectionSummary(await disarmReprojection());
    const overwriteRead = await waitUntil(async () => documentEntries(await allHistoryRows()).find((row) => row.label.startsWith(copy.row("edit", null, 1))) ?? null, (row) => row !== null, 20000, 1000);
    const overwriteRow = overwriteRead.value;
    verdict("g10-the-second-peer-sees-the-finalized-edit", converged.ok && Boolean(overwriteRow), { n1: converged.value[n1], n2: converged.value[n2], row: overwriteRow?.label ?? null, waitedMs: converged.waitedMs });
    if (adopted.seen) {
      verdict("g10-the-second-peer-shows-what-it-replays", adopted.kinds.length > 0, { adopted, reading: "N17 / §20.8: the peer names the change it replays (`HistoryPatch.reprojection {kind: remote|load}`) with its progress in words while it adopts the finalized edit" });
      reprojectionShellVerdicts(adopted, "-peer");
    } else note("g10-the-second-peer-adopted-within-one-refresh", { adopted, reading: "the folder change stream replaced the archive and the peer adopted it before its body re-rendered (progress refreshes ≥ 100 ms)" });
    await shot("g10-peers");
    const stateOf = async () => {
      const now = await positions();
      return { n1: now[n1] ?? null, n2: now[n2] ?? null, rows: documentEntries(await allHistoryRows()).map((row) => row.label.slice(0, 60)), band: (await band())?.stage ?? null };
    };
    const finalB = await stateOf();
    use(a);
    const finalA = await stateOf();
    await shot("g10-peer-a-final");
    const agree = JSON.stringify([finalA.n1, finalA.n2]) === JSON.stringify([finalB.n1, finalB.n2]) && JSON.stringify(finalA.rows) === JSON.stringify(finalB.rows);
    verdict("g10-both-peers-converge-after-the-finalize", agree, { a: finalA, b: finalB, reading: "after the first peer finalized, both peers hold the same document (the two nodes) and list the same document rows" });
    const survived = Boolean(finalA.n2 && finalB.n2 && near(finalA.n2[0], p0[n2][0] + bOff[0], 0.05) && near(finalA.n2[1], p0[n2][1] + bOff[1], 0.05) && near(finalB.n2[0], p0[n2][0] + bOff[0], 0.05) && near(finalB.n2[1], p0[n2][1] + bOff[1], 0.05));
    note("g10-the-second-peers-drag-after-the-finalize", { survived, n2: { a: finalA.n2, b: finalB.n2, withTheDrag: [p0[n2][0] + bOff[0], p0[n2][1] + bOff[1]], withoutIt: p0[n2] }, reading: "design §22.25: a remote tail edit is adopted at ingest and listed \"Not applied while editing\"; until the folder route merges a fetched pair (S5-LOAD / S5-STORE) a read-back is last-writer-wins, so whether the second peer's drag survives the first peer's finalize is recorded, not judged" });
    note("g10-final-states-of-both-peers", {
      a: finalA,
      b: finalB,
      agree,
      expected: { n1: [p0[n1][0] + dxNew, p0[n1][1] + aOff[1]], n2: [p0[n2][0] + bOff[0], p0[n2][1] + bOff[1]] },
      folderRequests: consoleRows.filter((row) => row.locale === currentLocale && row.step === currentStep && row.type === "backbone").map((row) => `${row.t}s ${/HTTP \d+ \w+/.exec(row.text)?.[0] ?? row.text.slice(0, 20)}`),
      reading: "what each peer holds at the end (the first peer edited history while the second peer dragged), and every folder request of the step with its run-clock time: a PUT is a peer writing the archive, the GETs after it are the peers reading it",
    });
  });
//#endregion 🔖️Peers

//#region 🔖️HistorySteps
/** 🧗️ Step 18 — N17 history steps and the §20.8 stepped document load, in fresh documents. Page A, bound to a fresh folder (its
 * archive lands there), grows by a second example load and finalizes an edit of its first example's first mutation as a new
 * alternative; switching back to the main line is a deferred local step the body shows as "History step" / "Replaying
 * history: d of t mutations" — Undo meanwhile is refused (`history.replaying`), Cancel replay drops the step with zero trace
 * (the alternative stays current) — and switching again completes it; one drag then marks A's head. Page B attaches A's folder:
 * the archive loads stepwise ("Document load" / "Loading document: d of t"), Undo meanwhile is refused (`document.loading`) and
 * Cancel replay keeps B's own document; uncancelled, B shows A's head. Presses land the frame the status shows (a person
 * reacting to it); a step or a load that finishes before any frame shows it is recorded as such. */
const step18 = async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const folder = join(OUT, `folder-${stamp}-${currentLocale}-steps`);
    mkdirSync(folder, { recursive: true });
    const a = await open(DESKTOP, "history-step");
    verdict("n17-fresh-document-boots", a !== null, {});
    if (!a) return;
    await closePanels();
    const attached = await attachFolder(folder);
    await closePanels();
    const examples = async () => documentEntries(await allHistoryRows()).filter((row) => copy.example.test(row.label)).length;
    const examplesBefore = await examples();
    const grown = await runPaletteCommand(copy.exampleQuery, copy.example, "setActiveExample");
    const landed = await waitUntil(examples, (count) => count > examplesBefore, 120000, 2000);
    verdict("n17-history-grows-by-a-second-example-load", landed.ok, { run: grown, examples: [examplesBefore, landed.value], waitedMs: landed.waitedMs });
    await closePanels();
    const tx = await exampleTransaction();
    const session = tx.mutations.length ? await draftLongHistoryTarget(tx.mutations) : null;
    verdict("n17-a-mutation-of-the-first-example-opens-with-a-draft", session !== null, { entry: tx.entry?.label ?? null, kind: session?.kind ?? null, target: session?.target.label ?? null });
    if (!session) return;
    await sleep(900);
    await pressBand("accept");
    const review = await waitUntil(band, (b) => b?.stage === "reviewing" && b.review !== null && b.review !== "needsReplay", 240000, 200);
    if (review.value?.review !== "ready") {
      verdict("n17-the-edit-reviews-ready", false, { review: review.value?.review ?? null, text: review.value?.text?.slice(0, 200) });
      await exitSession();
      return;
    }
    const name = `probe steps ${currentLocale} ${stamp.slice(11, 19)}`;
    await pressBand("finalize");
    await waitUntil(finalizeDialog, (d) => d !== null, 15000);
    const typedName = await dialogFillName(name);
    await dialogChoose("submit");
    const closed = await waitUntil(band, (b) => b === null, 60000);
    const listed = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(name)) && rows.some((row) => row.text.includes(copy.trunk)), 20000, 1000);
    verdict("n17-the-edit-is-kept-as-a-new-alternative", closed.ok && listed.ok, { typedName, alternatives: listed.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${isCurrentAlternative(row) ? " [current]" : ""}`) });
    if (!listed.ok) return;
    const onAlternative = await positions();
    const rowsOnAlternative = documentEntries(await allHistoryRows()).map((row) => row.label);
    const trunkRow = async () => (await readAlternatives()).find((row) => row.text.includes(copy.trunk)) ?? null;
    const currentIsTrunk = async () => Boolean((await readAlternatives()).find(isCurrentAlternative)?.text.includes(copy.trunk));
    const trunk = await trunkRow();
    await armReprojection("undo-then-cancel");
    const via = trunk ? await switchAlternative(trunk) : "absent";
    await sleep(800);
    await reprojectionSettled(180000);
    const step = reprojectionSummary(await disarmReprojection());
    emit({ kind: "history-step", via, step });
    if (!step.seen) note("n17-the-history-step-adopted-within-one-refresh", { via, step, reading: "the switch's replay finished before the body re-rendered (progress refreshes ≥ 100 ms), so neither the refusal nor Cancel could be pressed while it ran" });
    else {
      verdict("n17-a-history-step-shows-its-progress", step.kinds.includes("step") && (step.total ?? 0) > 0, { step, reading: "N17: an alternative switch away from the applied tail is a deferred local step — \"History step\" / \"Replaying history: d of t mutations\" in the shell status and the body's `framework.history.reprojection` section (`HistoryPatch.reprojection {kind: step}`)" });
      reprojectionShellVerdicts(step, "-step");
      const refused = (await shownNotices()).find((row) => row.code === "history.replaying");
      if (step.undo) verdict("n17-a-second-history-step-is-refused-while-one-replays", Boolean(refused?.text.includes(copy.noticeReplaying)), { undo: step.undo, notice: refused ?? null, expected: copy.noticeReplaying, reading: "`VcsError::HistoryReplaying`: every further history step (here Undo) is refused while one replays, with the localized notice" });
      if (step.cancel) {
        const back = await waitUntil(positions, (p) => movedIds(onAlternative, p, 1e-6).length === 0, 15000, 500);
        const rowsAfter = documentEntries(await allHistoryRows()).map((row) => row.label);
        verdict("n17-cancel-replay-drops-the-history-step-with-zero-trace", step.cancel.via !== "absent" && !(await currentIsTrunk()) && back.ok && rowsAfter.length === rowsOnAlternative.length, { cancel: step.cancel, drift: movedIds(onAlternative, back.value, 1e-6).slice(0, 6), rows: [rowsOnAlternative.length, rowsAfter.length], reading: "`discard_local_step`: Cancel replay drops a local step with zero trace — the alternative stays current and nothing is recorded" });
      }
    }
    if (!(await currentIsTrunk())) {
      const again = await trunkRow();
      if (again) await switchAlternative(again);
      await reprojectionSettled(180000);
    }
    const done = await waitUntil(currentIsTrunk, (ok) => ok, 60000, 1000);
    verdict("n17-the-history-step-completes-on-the-main-line", done.ok, { alternatives: (await readAlternatives()).map((row) => `${row.text.slice(0, 50)}${isCurrentAlternative(row) ? " [current]" : ""}`) });
    await closePanels();
    await frameBoard(4);
    const { camera, picked } = await pickNodes(1, [[60, 30]], [], 40);
    if (picked[0]) {
      await selectNodes(picked);
      const before = await positions();
      await dragBy(picked[0].at, 60 * camera.zoom, 30 * camera.zoom);
      await waitUntil(positions, (p) => offsetOf(before, p, picked[0]!.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    }
    const headA = await positions();
    await closePanels();
    if (!attached.card) {
      note("n17-no-folder-route-for-the-stepped-load", { attached, reading: "this build offers no folder card (browser wgpu), so the stepped archive load is reached only through a hub or a file import" });
      return;
    }
    const written = await waitUntil(async () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).length, (count) => count > 0, 30000, 1000);
    const b = await open(DESKTOP, "document-load");
    verdict("n17-second-page-boots", b !== null, { archiveFiles: written.value });
    if (!b) return;
    const own = await positions();
    await closePanels();
    await armReprojection("undo-then-cancel");
    const attachedB = await attachFolder(folder, () => openHistory());
    await sleep(800);
    const settledLoad = await reprojectionSettled(90000);
    const load = reprojectionSummary(await disarmReprojection());
    emit({ kind: "document-load", load, attachedB, settled: settledLoad });
    const shown = await positions();
    const isOwn = movedIds(own, shown, 1e-6).length === 0;
    const isA = movedIds(headA, shown, 1e-6).length === 0;
    if (!load.seen) {
      note("n17-the-document-load-finished-within-one-refresh", { load, reading: "the archive load finished before the body re-rendered (progress refreshes ≥ 100 ms)" });
      verdict("n17-attaching-the-folder-loads-the-shared-document", isA, { drift: movedIds(headA, shown, 1e-6).slice(0, 6), attachedB });
      return;
    }
    verdict("n17-the-stepped-load-shows-its-progress", load.kinds.includes("load") && (load.total ?? 0) > 0, { load, reading: "§20.8: a whole-document load is stepped on the history wire (`HistoryPatch.reprojection {kind: load}`), never one `LoadDocument` — \"Document load\" / \"Loading document: d of t\" in the shell status and the body" });
    reprojectionShellVerdicts(load, "-load");
    const refused = (await shownNotices()).find((row) => row.code === "document.loading");
    if (load.undo) verdict("n17-a-command-during-the-load-is-refused-naming-the-load", Boolean(refused?.text.includes(copy.noticeLoading)), { undo: load.undo, notice: refused ?? null, expected: copy.noticeLoading, reading: "N17: any command while a whole-document load is live is refused with the localized `document.loading` notice" });
    if (load.cancel) verdict("n17-cancel-replay-keeps-the-previous-document", isOwn, { cancel: load.cancel, drift: movedIds(own, shown, 1e-6).slice(0, 6), reading: "`historyEditCancelReplay` without a session cancels a live load: the previous document stays, zero trace" });
    else verdict("n17-attaching-the-folder-loads-the-shared-document", isA, { drift: movedIds(headA, shown, 1e-6).slice(0, 6), attachedB });
    if (load.cancel && load.cancel.via !== "absent") {
      await openHistory();
      const stale = await reprojection();
      verdict("n17-a-cancelled-load-clears-its-status", settledLoad.ok && stale === null, { settled: settledLoad, status: stale, reading: "after Cancel replay nothing loads any more, so the History body's `framework.history.reprojection` section (and the shell status) must go away; a status that stays reads as a load that never ends" });
      await shot("load-cancelled");
      const noticesBefore = (await shownNotices()).length;
      await closePanels();
      await frameBoard(6);
      const { camera, picked } = await pickNodes(1, [[30, 20]], [], 90);
      if (picked.length) {
        await selectNodes([picked[0]]);
        const beforeDrag = await positions();
        await dragBy(picked[0].at, 30 * camera.zoom, 20 * camera.zoom);
        const dragged = await waitUntil(positions, (now) => offsetOf(beforeDrag, now, picked[0].id)?.some((value) => Math.abs(value) > 0.5) === true, 10000);
        verdict("n17-the-document-is-editable-after-a-cancelled-load", dragged.ok, { node: picked[0].id, offset: offsetOf(beforeDrag, dragged.value, picked[0].id), notices: (await shownNotices()).slice(noticesBefore), reading: "a cancelled load leaves the previous document as it was — a person goes on working in it; a drag that is refused (`document.loading`) means the cancelled load still holds the document" });
      } else note("n17-no-node-to-drag-after-the-cancelled-load", { picked: 0 });
    }
  });
//#endregion 🔖️HistorySteps

//#region 🔖️ConfigAndLists
/** 🛤️ Scrolls the tree window `windowKey` slice by slice from its start until the row `authored` is materialised and in view —
 * how a person scrolls a long editor to one of its rows (wgpu: the body pages until the mirror projects it). */
const revealWindowRow = async (windowKey: string, authored: string) => {
  for (let attempt = 0; attempt < 40; attempt++) {
    if (renderer === "wgpu" ? Boolean(mirrorFind(await mirror(), authored)) : await revealHistory(authored)) return true;
    const moved = renderer === "wgpu"
      ? await scrollHistoryBy(0.5)
      : await evalSafe(
          (arg) => {
            const el = Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).find((node) => {
              const key = node.getAttribute("data-tree-window-key") ?? "";
              return key === arg.key || key.endsWith(`/${arg.key}`);
            });
            const rows = Array.from(el?.children ?? []).filter((child) => !child.hasAttribute("data-tree-window-spacer"));
            const anchor = (arg.first ? el?.firstElementChild : rows[rows.length - 1]) as HTMLElement | null | undefined;
            anchor?.scrollIntoView({ block: "start" });
            return Boolean(anchor);
          },
          false,
          { key: windowKey, first: attempt === 0 },
        );
    if (!moved) return false;
    await sleep(500);
  }
  return false;
};

/** 📊️ The item count a list input's row names (`Items: n` / `Elemente: n`), scrolling its row into view only when it is not
 * materialised; null while it cannot be reached. */
const listItemCount = async (pointerKey: string) => {
  const pattern = new RegExp(COPY[currentLocale].itemCount(0).replace("0", "(\\d+)"));
  const read = async () => {
    const match = pattern.exec(await textOfKey(`${pointerKey}.row`));
    return match ? Number(match[1]) : null;
  };
  return (await read()) ?? ((await revealWindowRow("framework.history.editor.inputs", `${pointerKey}.row`)) ? read() : null);
};

/** ⚙️ Step 19 — on the main document: L4 (§20.13) — camera zoom and pan add no history row at all; N1 — one drag adds exactly
 * one row, the Commands window `total` grows by one, and §19.1 — the row reads its intent leaf; L4 again — Undo after a camera
 * move takes back the drag and leaves the camera, Redo re-applies it; N2 — Edit of a duplicated node's `create-node`: its
 * handles list names its item count, Add item drafts one more item, Remove item drops it, Exit leaves zero trace. */
const step19 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  const cameraNow = async () => cameraOf(await vitals());
  const sameCamera = (left: { x: number; y: number; zoom: number }, right: { x: number; y: number; zoom: number }) => near(left.x, right.x, 1e-3) && near(left.y, right.y, 1e-3) && near(left.zoom, right.zoom, 1e-4);
  await closePanels();
  await frameBoard(6);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  const totalBefore = await historyCommandsTotal();
  await closePanels();
  const cameraBefore = await cameraNow();
  const box = await paneBox();
  const centre = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
  const byGesture: Record<string, string[]> = {};
  let newestSeen = newestBefore;
  const rowsOfGesture = async (gesture: string) => {
    const rows = (await allHistoryRows()).filter((row) => row.kind === "entry" && Number(row.key) > newestSeen);
    byGesture[gesture] = rows.filter((row) => !isShellChromeRow(row)).map((row) => `${row.key}=${row.label}`);
    newestSeen = Math.max(newestSeen, newestEntrySeq(rows));
    await closePanels();
    await page.mouse.move(centre.x, centre.y);
  };
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.wheel(0, -240);
  await sleep(900);
  await rowsOfGesture("wheel zoom in");
  await page.mouse.down({ button: "middle" });
  for (let index = 1; index <= 10; index++) await page.mouse.move(centre.x + 6 * index, centre.y + 4 * index);
  await page.mouse.up({ button: "middle" });
  await sleep(900);
  await rowsOfGesture("middle-button pan");
  await page.mouse.wheel(0, 120);
  await sleep(1500);
  const cameraAfter = await cameraNow();
  const rowsAfter = await allHistoryRows();
  const totalAfter = await historyCommandsTotal();
  byGesture["wheel zoom out"] = rowsAfter.filter((row) => row.kind === "entry" && Number(row.key) > newestSeen && !isShellChromeRow(row)).map((row) => `${row.key}=${row.label}`);
  const added = rowsAfter.filter((row) => row.kind === "entry" && Number(row.key) > newestBefore);
  const foreign = added.filter((row) => !isShellChromeRow(row));
  verdict("l4-camera-moves-are-never-history-rows", !sameCamera(cameraBefore, cameraAfter) && foreign.length === 0 && (totalBefore === null || (totalAfter !== null && totalAfter - totalBefore === added.length)), { cameraBefore, cameraAfter, rowsFromTheCameraMoves: foreign.map((row) => `${row.key}=${row.label}${row.expandable ? " (document)" : ""}`), byGesture, probePanelRows: added.length - foreign.length, totals: [totalBefore, totalAfter], reading: "L4 / §20.13: camera zoom and pan stream through the window transient and commit config edits, which the history projection never lists — not as a row, not as a mutation (the law observes ALL rows; the panel toggles the probe itself makes to read the body are shell chrome rows and are told apart by their icon)" });
  await closePanels();
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { picked } = await pickNodes(1, [[50, 30]], exclude, 40);
  if (!picked[0]) {
    verdict("l4-one-clickable-node", false, {});
    return;
  }
  const node = picked[0].id;
  const newestBeforeDrag = newestEntrySeq(await allHistoryRows());
  const totalBeforeDrag = await historyCommandsTotal();
  const [target] = await rowsOf([node]);
  await selectNodes(target ? [target] : picked);
  const p0 = await positions();
  const zoom = (await cameraNow()).zoom;
  const [grab] = await rowsOf([node]);
  await dragBy((grab ?? picked[0]).at, 50 * zoom, 30 * zoom);
  const moved = await waitUntil(positions, (p) => offsetOf(p0, p, node)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const offset = offsetOf(p0, moved.value, node);
  if (!offset) {
    verdict("l4-the-drag-moves-the-node", false, { node });
    return;
  }
  const row = await newMutation(newestBeforeDrag, (entry) => entry.label.startsWith(dragLabel(1, offset[0], offset[1])));
  const totalAfterDrag = await historyCommandsTotal();
  if (totalBeforeDrag === null) note("n1-commands-window-total-not-published", { reading: "no window size is published (React `data-tree-window-total`, wgpu `aria-setsize` of the entry rows)" });
  else {
    const newer = (await allHistoryRows()).filter((entry) => entry.kind === "entry" && Number(entry.key) > newestBeforeDrag);
    const totalNow = await historyCommandsTotal();
    const documents = newer.filter((entry) => entry.expandable);
    verdict("n1-one-drag-adds-exactly-one-row-to-the-window-total", Boolean(row.entry) && totalBeforeDrag !== null && totalNow !== null && totalNow - totalBeforeDrag === newer.length && documents.length === 1, { totals: [totalBeforeDrag, totalAfterDrag, totalNow], row: row.entry?.label ?? null, newRows: newer.map((entry) => `${entry.key}=${entry.label}${entry.expandable ? " (document)" : ""}`), documentRows: documents.length, reading: "N1: the Commands window `total` grows by exactly the rows newer than the read before the drag — the drag's ONE document row plus command rows without mutations (the selection click's board events, the probe's own panel toggles)" });
  }
  verdict("s19-the-drag-row-reads-its-intent-leaf", Boolean(row.entry && row.mutation && row.entry.label === row.mutation.label), { row: row.entry?.label ?? null, intent: row.mutation?.label ?? null, expected: dragLabel(1, offset[0], offset[1]) });
  await closePanels();
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.wheel(0, -180);
  await sleep(1500);
  const zoomed = await cameraNow();
  await prepareChord();
  await page.keyboard.press(`${mod}+z`).catch(() => {});
  const undone = await waitUntil(async () => placed(await positions(), node, p0[node], 0, 0), (ok) => ok, 12000, 400);
  await sleep(600);
  const cameraUndone = await cameraNow();
  verdict("l4-undo-takes-back-the-drag-not-the-camera", undone.ok && sameCamera(zoomed, cameraUndone), { node, at: (await positions())[node], before: p0[node], zoomed, cameraUndone, reading: "L4: undo of the artifact never steps over config edits — one Undo after a camera move takes back the drag, the camera stays" });
  await prepareChord();
  await page.keyboard.press(`${mod}+Shift+z`).catch(() => {});
  const redone = await waitUntil(async () => placed(await positions(), node, p0[node], offset[0], offset[1]), (ok) => ok, 12000, 400);
  verdict("l4-redo-re-applies-the-drag", redone.ok, { node, at: (await positions())[node], expected: [p0[node]![0] + offset[0], p0[node]![1] + offset[1]] });
  await closePanels();
  await frameBoard(4);
  const spot = await pickCloneSource([...exclude, node]);
  if (!spot) {
    verdict("n2-clone-source-with-free-space", false, {});
    return;
  }
  const idsBefore = Object.keys(await positions());
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const newestBeforeClone = newestEntrySeq(await allHistoryRows());
  const [source] = await rowsOf([spot.row.id]);
  await selectNodes(source ? [source] : [spot.row]);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  const createRow = clone ? await newMutation(newestBeforeClone, (entry) => entry.label.startsWith(copy.createNode(clone))) : { entry: null, mutation: null, waitedMs: 0 };
  verdict("n2-duplicate-adds-a-create-node-row", Boolean(createRow.mutation), { clone, mutation: createRow.mutation?.label ?? null });
  if (!createRow.mutation) return;
  const head = await positions();
  const rowsPre = documentEntries(await allHistoryRows()).map((entry) => entry.label);
  const begun = await beginEditOf(createRow.mutation.key);
  verdict("n2-edit-opens-the-create-node-session", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const list = "framework.history.editor.input.node.handles";
  await readEditor();
  const count = await listItemCount(list);
  const handlesBefore = (await vitals())?.handles ?? null;
  await revealWindowRow("framework.history.editor.inputs", `${list}.row`);
  const add = await pressAuthored(`${list}.add`, false);
  const grew = await waitUntil(() => listItemCount(list), (now) => count !== null && now === count + 1, 10000, 400);
  verdict("n2-add-item-drafts-one-more-list-item", count !== null && add.present && add.disabled !== true && grew.ok, { add, count: [count, grew.value], handles: [handlesBefore, (await vitals())?.handles ?? null], reading: "N2: a list input's row names its item count and holds Add item (`historyEditInput{path: …/handles/-, edit: insert}`), which drafts the item's default" });
  if (count !== null && grew.ok) {
    await revealWindowRow("framework.history.editor.inputs", `${list}.${count}.row`);
    const remove = await pressAuthored(`${list}.${count}.remove`, false);
    const shrank = await waitUntil(() => listItemCount(list), (now) => now === count, 10000, 400);
    verdict("n2-remove-item-drops-the-drafted-item", remove.present && remove.disabled !== true && shrank.ok, { remove, count: [grew.value, shrank.value], reading: "N2: each list item's first row holds Remove item (`edit: remove` at its pointer)" });
  }
  note("n2-long-option-rows-not-offered-by-puzzle-2d", { reading: "a select or segmented input with MORE than `UI_FIXED_LIST_ITEMS` (32) options becomes windowed option rows (`….option.<i>`, `role=treeitem` with `aria-selected=true|false`, the chosen row `data-selected`); no puzzle 2d leaf offers that many, so the contract is proven by the React `🧺️ListInputs` and wgpu `the_guest_editor_offers_list_and_chip_edits_within_their_bounds` laws (S4-UI), not live" });
  const exited = await exitSession();
  const restored = await waitUntil(positions, (p) => movedIds(head, p, 1e-6).length === 0, 15000, 500);
  const rowsSettled = await waitUntil(async () => documentEntries(await allHistoryRows()).map((entry) => entry.label), (rows) => rows.length === rowsPre.length, 8000, 800);
  const rowsPost = rowsSettled.value;
  verdict("n2-exit-leaves-zero-trace", exited.ok && restored.ok && rowsPost.length === rowsPre.length, { band: exited.value?.stage ?? null, drift: movedIds(head, restored.value, 1e-6).slice(0, 6), rows: [rowsPre.length, rowsPost.length], rowsWaitedMs: rowsSettled.waitedMs });
  await shot("config-and-lists");
  await closePanels();
};
//#endregion 🔖️ConfigAndLists
//#endregion 🔖️Gaps

/** 🧿 Step 20 — row actions (design §22.1, §22.20), on the main document: a node is duplicated and its clone dragged (two rows:
 * `create-node`, then its drag). Every mutation row offers Edit and Withdraw. Withdraw on the create-node's ROW opens a session
 * with a withdrawn draft (no editor) → the replay leaves the drag blocking ("Target missing"); the row now holds an accepted
 * draft and offers Restore in place of Withdraw. Withdraw on the BLOCKING drag's row — a row action needs no input schema, so it
 * works for a mutation whose inputs cannot be edited too; the row's Edit state is recorded — makes the review `ready` with two
 * accepted changes. Restore on the drag's row drops that draft (blocked again, one accepted change). Exit leaves zero trace. */
const step20 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const spot = await pickCloneSource(exclude);
  if (!spot) {
    verdict("r22-clone-source-with-free-space", false, { reason: "no node whose clone and drag destination are free" });
    return;
  }
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(spot.row.at.x, spot.row.at.y);
  await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === spot.row.id, 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  if (!clone) {
    verdict("r22-duplicate-adds-one-clone", false, { nodes: [nodesBefore, (await vitals())?.nodes ?? null] });
    return;
  }
  await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  await dragBy({ x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 }, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const cloneOffset = offsetOf(beforeDrag, dragged.value, clone);
  const head = dragged.value;
  const added = (await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 2, 30000, 1000)).value;
  let createRow: HistoryRow | undefined;
  let dragRow: HistoryRow | undefined;
  for (const entry of added) {
    const expanded = await expandEntry(entry.id);
    createRow ??= expanded.mutations.find((row) => row.label.startsWith(copy.createNode(clone)));
    if (cloneOffset) dragRow ??= expanded.mutations.find((row) => row.label.startsWith(dragLabel(1, cloneOffset[0], cloneOffset[1])));
  }
  verdict("r22-duplicate-and-drag-rows-carry-their-mutations", Boolean(createRow && dragRow), { added: added.map((row) => row.label), create: createRow?.label, drag: dragRow?.label });
  if (!createRow || !dragRow) return;
  const rowsPre = documentEntries(await allHistoryRows()).map((row) => row.label);
  const offered = { createEdit: await rowActionState(createRow.id, copy.edit, false), createWithdraw: await rowActionState(createRow.id, copy.withdraw, false), dragEdit: await rowActionState(dragRow.id, copy.edit, false), dragWithdraw: await rowActionState(dragRow.id, copy.withdraw, false) };
  verdict("r22-every-mutation-row-offers-edit-and-withdraw", Object.values(offered).every((state) => state.present) && offered.createWithdraw.disabled !== true && offered.dragWithdraw.disabled !== true, { offered, reading: "design §22.1: a mutation row always carries `[Edit, Withdraw]` (a withdrawn row `[Edit]`, a row with an accepted draft `[Edit, Restore]`); an action that cannot run is disabled with its reason, never absent" });
  await installBandTrace();
  const viaWithdraw = await pressRowAction(createRow.id, copy.withdraw);
  let afterWithdraw = await waitUntil(band, (b) => b !== null && (b.stage === "editing" || (b.stage === "reviewing" && Boolean(b.review))), 20000, 100);
  const stageAfterPress = afterWithdraw.value?.stage ?? null;
  if (afterWithdraw.value?.stage === "editing") {
    await pressBand("accept");
    afterWithdraw = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  }
  verdict("r22-withdraw-from-the-row-opens-a-session-with-a-withdrawn-draft", viaWithdraw === "action" && afterWithdraw.value?.stage === "reviewing" && afterWithdraw.value.review === "blocked", { via: viaWithdraw, stageAfterPress, review: afterWithdraw.value?.review ?? null, text: afterWithdraw.value?.text?.slice(0, 200), reading: "Withdraw pressed on a mutation row (`historyEditWithdraw {mutationId}`) opens or stacks a session whose draft withdraws that mutation — no editor; here the clone's drag then fails downstream, so the review blocks" });
  const gone = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("r22-the-review-shows-the-document-without-the-withdrawn-node", gone.ok, { hasClone: Boolean(gone.value[clone]) });
  const createNow = await findMutationRow(createRow.key);
  const restoreOffer = createNow ? await rowActionState(createNow.id, copy.restore, false) : null;
  const withdrawOffer = createNow ? await rowActionState(createNow.id, copy.withdraw, false) : null;
  verdict("r22-a-row-with-an-accepted-draft-offers-restore-in-place-of-withdraw", Boolean(restoreOffer?.present && restoreOffer.disabled !== true && withdrawOffer && !withdrawOffer.present), { row: createNow?.text?.slice(0, 160) ?? null, restore: restoreOffer, withdraw: withdrawOffer });
  const failing = await findMutationRow(dragRow.key);
  const failingEdit = failing ? await rowActionState(failing.id, copy.edit, false) : null;
  const viaBlocking = failing ? await pressRowAction(failing.id, copy.withdraw) : "absent";
  let ready = await waitUntil(band, (b) => b !== null && (b.stage === "editing" || (b.stage === "reviewing" && b.review === "ready")), 30000, 100);
  if (ready.value?.stage === "editing") {
    await pressBand("accept");
    ready = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  }
  verdict("r22-withdrawing-the-blocking-mutation-from-its-row-makes-the-review-ready", viaBlocking === "action" && ready.value?.review === "ready" && Boolean(ready.value.text.includes(copy.accepted(2))), { via: viaBlocking, blockingRow: failing?.text?.slice(0, 160) ?? null, editOfTheBlockingRow: failingEdit ? { disabled: failingEdit.disabled, reason: failingEdit.reason } : null, review: ready.value?.review ?? null, text: ready.value?.text?.slice(0, 200), expected: copy.accepted(2), reading: "design §22.1: Withdraw needs no input schema, so a blocking mutation is withdrawn from its row whether or not its inputs can be edited (the row's Edit state is recorded: disabled with \"" + copy.refusalNotEditable + "\" for a non-editable one)" });
  const dragNow = await findMutationRow(dragRow.key);
  const viaRestore = dragNow ? await pressRowAction(dragNow.id, copy.restore) : "absent";
  const restored = await waitUntil(band, (b) => b?.stage === "reviewing" && b.review === "blocked" && b.text.includes(copy.accepted(1)), 30000, 100);
  verdict("r22-restore-from-the-row-drops-the-accepted-draft", viaRestore === "action" && restored.ok, { via: viaRestore, review: restored.value?.review ?? null, text: restored.value?.text?.slice(0, 200), expected: `${copy.reviewBlocked} … ${copy.accepted(1)}`, reading: "Restore (`historyEditRestore {mutationId}`) drops that mutation's accepted draft: the drag is applied again, fails again, and one accepted change (the withdrawn create-node) remains" });
  await shot("row-actions");
  const exited = await exitSession();
  const back = await waitUntil(positions, (p) => movedIds(head, p, 1e-6).length === 0, 20000, 500);
  const rowsSettled = await waitUntil(async () => documentEntries(await allHistoryRows()).map((row) => row.label), (rows) => rows.length === rowsPre.length, 8000, 800);
  verdict("r22-exit-leaves-zero-trace", exited.ok && back.ok && rowsSettled.ok, { band: exited.value?.stage ?? null, drift: movedIds(head, back.value, 1e-6).slice(0, 6), rows: [rowsPre.length, rowsSettled.value.length] });
  await closePanels();
};

/** 🥅️ Step 21 — the goal's own sentence, one verdict set on the main document: two selected nodes are dragged (ONE history row
 * holding the `drag-selection` leaf), a third node is dragged afterwards (downstream). Edit on the drag → the editor offers the
 * SELECTION as a reference list — Use selection plus one chip per target, each with its Remove — and the OFFSET as dx / dy
 * steppers that step by the grid snap. Each is changed through its own control and the preview follows: ArrowUp on dx, Remove on
 * one target, the board selection through Use selection. Accept re-applies the downstream drag (review ready); Exit, zero trace. */
const step21 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const { camera, picked } = await pickNodes(4, [[60, 40], [-50, 30], [61, 40]], exclude, 60);
  if (picked.length < 4) {
    verdict("goal-four-clickable-nodes", false, { picked: picked.map((row) => row.id), reading: "the set needs two nodes to drag, one for the downstream drag and one to take over as the target" });
    return;
  }
  const [a, b, c, d] = picked.map((row) => row.id);
  await selectNodes([picked[0], picked[1]]);
  const p0 = await positions();
  await dragBy(picked[0].at, 60 * camera.zoom, 40 * camera.zoom);
  const moved = await waitUntil(positions, (now) => offsetOf(p0, now, a)?.some((value) => Math.abs(value) > 0.5) === true && offsetOf(p0, now, b)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const offset = offsetOf(p0, moved.value, a) ?? [0, 0];
  const [dx, dy] = offset;
  const drag = await newMutation(newestBefore, (row) => row.label.startsWith(dragLabel(2, dx, dy)));
  const dragEntries = documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore);
  const leaves = drag.entry ? (await expandEntry(drag.entry.id)).mutations : [];
  verdict("goal-one-drag-of-the-selection-is-one-history-row", moved.ok && dragEntries.length === 1 && Boolean(drag.mutation) && leaves.filter((row) => row.label.startsWith(dragLabel(2, dx, dy))).length === 1, { offset, rows: dragEntries.map((row) => row.label), leaves: leaves.map((row) => row.label), expected: dragLabel(2, dx, dy), reading: "the select tool is a state machine: one gesture = one transaction = one History row, holding one `drag-selection` mutation for the whole selection" });
  if (!drag.mutation) return;
  await closePanels();
  const [cRow] = await rowsOf([c]);
  const beforeC = await positions();
  await selectNodes(cRow ? [cRow] : [picked[2]]);
  await dragBy((cRow ?? picked[2]).at, -50 * camera.zoom, 30 * camera.zoom);
  const cMoved = await waitUntil(positions, (now) => offsetOf(beforeC, now, c)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const cOffset = offsetOf(beforeC, cMoved.value, c) ?? [0, 0];
  const head = cMoved.value;
  const rowsPre = documentEntries(await allHistoryRows()).map((row) => row.label);
  await installBandTrace();
  const begun = await beginEditOf(drag.mutation.key);
  verdict("goal-edit-opens-the-drag-mutation", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const slow = renderer === "wgpu" ? 45000 : 15000;
  const editorRead = await waitUntil(readEditor, (value) => Boolean(value?.dx && value?.dy && value?.targets), slow);
  const ed = editorRead.value;
  if (editorRead.waitedMs > 5000) note("the-editor-arrived-late", { waitedMs: editorRead.waitedMs, complete: editorRead.ok, reading: "time from the band reading `editing` to the editor's inputs (targets, dx, dy) being readable" });
  if (renderer === "wgpu") note("wgpu-editor-mirror-nodes", { nodes: (await mirror()).filter((node) => /framework\.history\.(editor|timeTravel)|shell\.time-travel|ui\.timeTravel/u.test(node.key)).map((node) => `${node.key}|${node.tag}${node.inputType ? `[${node.inputType}]` : ""}:${node.role}|${node.label.slice(0, 50)}|value=${node.value ?? node.valueNow ?? ""} step=${node.step ?? ""} min=${node.min ?? ""} max=${node.max ?? ""}${node.disabled ? " disabled" : ""}${node.actionable ? " actionable" : ""}${node.description ? ` described=${node.description.slice(0, 50)}` : ""}`), reading: "what the ARIA mirror carries for the open editor and the band — the DOM an assistive technology reads" });
  const removes = await Promise.all([0, 1].map((index) => presentKey(`framework.history.editor.input.targets.chip.${index}`)));
  verdict("goal-the-editor-offers-the-selection-as-a-reference-list", Boolean(ed?.targets?.useSelection && ed.targets.chips.length === 2 && removes.every((key) => key !== null)), { chips: ed?.targets?.chips ?? null, useSelection: ed?.targets?.useSelection ?? null, removes, reading: "the drag's `targets` input is a reference list: Use selection takes the board selection, each referenced node is a chip named by its label with its own Remove" });
  const snapStep = renderer === "wgpu" ? (mirrorFind(await mirror(), "framework.history.editor.input.dx")?.step ?? null) : (ed?.dx?.step ?? null);
  verdict("goal-the-editor-offers-the-offset-as-steppers-with-the-grid-snap", Boolean(ed?.dx?.stepper && ed.dy?.stepper && near(Number(ed.dx.value), dx, 0.005) && near(Number(ed.dy.value), dy, 0.005) && (renderer === "wgpu" ? true : ed.dx.step === "1" && ed.dy.step === "1" && ed.dx.plus && ed.dx.minus)), { dx: ed?.dx, dy: ed?.dy, snapStep, reading: "dx / dy are number steppers reading the recorded offset; their step is the board's grid snap (`snapSource {config: gridFactor}`, default 1) — on wgpu the step itself is proven by the ArrowUp below" });
  const onGrid = Math.abs(dx - Math.round(dx)) < 1e-6 && Math.abs(dy - Math.round(dy)) < 1e-6;
  verdict("goal-the-recorded-drag-offset-lies-on-the-grid-snap", onGrid, { offset, editor: [ed?.dx?.value ?? null, ed?.dy?.value ?? null], row: drag.mutation.label, reading: "the board snaps a drag to its grid (snap 1 world unit): the recorded `dx` / `dy` are whole snaps, which is what the row label shows and what the steppers step from" });
  const dx1 = onGrid ? dx + 1 : Math.ceil(dx);
  const exact = onGrid ? 0.02 : 1e-5;
  await keyEditorNumber("dx", "ArrowUp");
  const stepped = await waitUntil(positions, (now) => placed(now, a, p0[a], dx1, dy, exact) && placed(now, b, p0[b], dx1, dy, exact), slow);
  verdict("goal-stepping-the-offset-previews-it-and-keeps-downstream-unapplied", stepped.ok && placed(stepped.value, c, beforeC[c], 0, 0), { a: stepped.value[a], expected: [p0[a][0] + dx1, p0[a][1] + dy], steppedTo: dx1, waitedMs: stepped.waitedMs, c: stepped.value[c], cBeforeItsDrag: beforeC[c], reading: "one ArrowUp = one grid snap (from an offset that is not on the grid: up to the next snap, as a number stepper steps); the preview is the document before the drag plus the draft, the later drag is not applied" });
  await revealWindowRow("framework.history.editor.inputs", "framework.history.editor.input.targets.chip.1.row");
  if (renderer === "wgpu") await mirrorAwait("framework.history.editor.input.targets.chip.1", slow);
  const removed = await pressAuthored("framework.history.editor.input.targets.chip.1", false);
  const oneChip = await waitUntil(readEditor, (value) => (value?.targets?.chips.length ?? 0) === 1, slow);
  const without = await waitUntil(positions, (now) => [a, b].filter((id) => placed(now, id, p0[id], dx1, dy, exact)).length === 1 && [a, b].filter((id) => placed(now, id, p0[id], 0, 0)).length === 1, slow);
  verdict("goal-removing-a-target-previews-the-drag-without-it", removed.present && oneChip.ok && without.ok, { removed, chips: oneChip.value?.targets?.chips ?? null, waitedMs: [oneChip.waitedMs, without.waitedMs], a: without.value[a], b: without.value[b], before: { a: p0[a], b: p0[b] }, reading: "Remove on one chip drops that node from the drag's selection: the preview moves only the remaining target" });
  const [dRow] = await rowsOf([d]);
  const selected = dRow ? await selectNodes([dRow]) : [];
  await openHistory();
  const used = await pressUseSelection("targets");
  const replaced = await waitUntil(readEditor, (value) => (value?.targets?.chips.length ?? 0) === 1, slow);
  const taken = await waitUntil(positions, (now) => placed(now, d, head[d], dx1, dy, 0.05) && placed(now, a, p0[a], 0, 0) && placed(now, b, p0[b], 0, 0), slow);
  verdict("goal-use-selection-replaces-the-targets", selected.includes(d) && Boolean(used) && replaced.ok && taken.ok, { selected, used, chips: replaced.value?.targets?.chips ?? null, waitedMs: [replaced.waitedMs, taken.waitedMs], d: taken.value[d], expected: [head[d][0] + dx1, head[d][1] + dy], a: taken.value[a], b: taken.value[b], reading: "the board selection replaces the drag's targets: the preview drags the newly selected node by the drafted offset and leaves the former targets where they were before the drag" });
  const via = await pressBand("accept");
  const review = await waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 60000, 100);
  const reapplied = await waitUntil(positions, (now) => placed(now, c, beforeC[c], cOffset[0], cOffset[1], 0.05) && placed(now, d, head[d], dx1, dy, 0.05), slow);
  verdict("goal-accept-re-applies-the-downstream-drag", review.value?.review === "ready" && reapplied.ok, { via, review: review.value?.review ?? null, text: review.value?.text?.slice(0, 160), c: reapplied.value[c], expectedC: [beforeC[c][0] + cOffset[0], beforeC[c][1] + cOffset[1]], d: reapplied.value[d], reading: "Accept replays every later mutation on the edited history: the downstream drag lands where it was recorded, the review is ready to finalize" });
  await shot("goal-reviewed");
  let exited = await exitSession();
  if (!exited.ok && renderer === "wgpu") exited = await waitUntil(band, (state) => state === null, slow);
  const back = await waitUntil(positions, (now) => movedIds(head, now, 1e-6).length === 0, renderer === "wgpu" ? slow : 20000, 500);
  const rowsSettled = await waitUntil(async () => documentEntries(await allHistoryRows()).map((row) => row.label), (rows) => rows.length === rowsPre.length, 8000, 800);
  verdict("goal-exit-leaves-zero-trace", exited.ok && back.ok && rowsSettled.ok, { band: exited.value?.stage ?? null, exitWaitedMs: exited.waitedMs, backWaitedMs: back.waitedMs, drift: movedIds(head, back.value, 1e-6).slice(0, 6), rows: [rowsPre.length, rowsSettled.value.length] });
  await closePanels();
};

type HandleRow = { id: string; at: [number, number]; node: string; kind: string; flag: boolean };

/** 🪝️ The handles the overview board publishes (React `data-board-handle-positions-json`: viewport-bounded and capped, rows
 * `[id, x, y, nodeId, kind, flag]`), or null where the renderer publishes none. */
const handleRows = async (): Promise<{ total: number; published: number; capped: boolean; rows: HandleRow[] } | null> => {
  if (renderer === "wgpu") return null;
  const raw = await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-handle-positions-json") ?? "", "", `window:${OVERVIEW}`);
  try {
    const parsed = JSON.parse(raw || "{}") as { total?: number; published?: number; capped?: boolean; rows?: [string, number, number, string, string, boolean][] };
    if (!parsed.rows) return null;
    return { total: parsed.total ?? 0, published: parsed.published ?? parsed.rows.length, capped: parsed.capped === true, rows: parsed.rows.map(([id, x, y, node, kind, flag]) => ({ id, at: [x, y] as [number, number], node, kind, flag })) };
  } catch {
    return null;
  }
};

/** 🪢️ Step 22 — design §22.13, `mutation.precondition-drifted`. The live example has no free handle, so the step makes a
 * compatible free pair: a one-handle node is duplicated (the clone's handle is free), the original is deleted (the handle it
 * was connected to is free again — read as the published handle row of another node whose flag flipped), and the clone is
 * dropped so its handle lies 8 world units beside that freed handle: the drop records the drag AND its proximity
 * `connect-handles` (with the tolerance it was recorded under) in ONE row. Edit the drag → dx far away → Accept: the connect
 * is kept and reads "Warning: Precondition drifted", new since this edit, review ready (a warning never blocks). Withdraw on
 * the connect's row → ready with two accepted changes and no warning → Finalize overwrite "2 mutations". A precondition the
 * probe cannot establish (no one-handle clone source, the freed handle not identifiable, no connect recorded) is a NOTE with
 * its evidence — the drifted outcome is then not judged. */
const step22 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  if (renderer === "wgpu") {
    note("d13-not-probed-on-this-renderer", { reading: "the step aims at handles through React's `data-board-handle-positions-json`; `dumpBoard2d` publishes no handle positions" });
    return;
  }
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const tried: string[] = [];
  let spot: Awaited<ReturnType<typeof pickCloneSource>> = null;
  let published: Awaited<ReturnType<typeof handleRows>> = null;
  for (let attempt = 0; attempt < 8 && !spot; attempt++) {
    const candidate = await pickCloneSource([...exclude, ...tried]);
    if (!candidate) break;
    published = await handleRows();
    if ((published?.rows ?? []).filter((row) => row.node === candidate.row.id).length === 1) spot = candidate;
    else tried.push(candidate.row.id);
  }
  if (!spot || !published) {
    note("d13-not-provokable-no-one-handle-clone-source", { tried, published: published ? { total: published.total, published: published.published, capped: published.capped } : null });
    return;
  }
  let x = spot.row;
  let zoomedIn = 0;
  while (zoomedIn < 8 && (published.capped || !published.rows.some((row) => row.node === x.id))) {
    await page.mouse.move(x.at.x, x.at.y);
    await page.mouse.wheel(0, -300);
    await sleep(900);
    const [again] = await rowsOf([x.id]);
    if (again) x = again;
    published = (await waitUntil(handleRows, (rows) => rows !== null && rows.published > 0, 4000, 400)).value ?? published;
    zoomedIn += 1;
  }
  const ownBefore = published.rows.find((row) => row.node === x.id) ?? null;
  if (published.capped || !ownBefore) {
    note("d13-not-provokable-the-board-caps-its-published-handles", { source: x.id, zoomedIn, published: { total: published.total, published: published.published, capped: published.capped }, zoom: cameraOf(await vitals()).zoom, reading: "`data-board-handle-positions-json` is viewport-bounded and capped at 128 rows in id order; the step needs every handle around the source published, so it zooms in on it until the cap no longer cuts" });
    return;
  }
  const before = published;
  const cameras: Record<string, unknown> = { zoomedIn: cameraOf(await vitals()) };
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const edgesBefore = (await vitals())?.edges ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(x.at.x, x.at.y);
  await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === x.id, 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  if (!clone) {
    note("d13-not-provokable-no-clone", { nodes: [nodesBefore, (await vitals())?.nodes ?? null] });
    return;
  }
  await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  cameras.afterTheDuplicate = cameraOf(await vitals());
  const zoom = cameraOf(await vitals()).zoom;
  await page.mouse.click(x.at.x - 8 * zoom * Math.SQRT1_2, x.at.y - 8 * zoom * Math.SQRT1_2);
  const reselected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === x.id, 15000);
  cameras.afterTheReselect = cameraOf(await vitals());
  const deleted = reselected.ok ? await runPaletteCommand("delete", /./u, "deleteSelection") : null;
  const removed = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore && (v?.edges ?? -1) === edgesBefore - 1, 30000);
  cameras.afterTheDelete = cameraOf(await vitals());
  let republished = await waitUntil(handleRows, (rows) => (rows?.rows ?? []).some((row) => row.node === clone), 3000, 400);
  for (let attempt = 0; attempt < 8 && !republished.ok; attempt++) {
    const at = toScreen((await positions())[clone], cameraOf(await vitals()), await paneBox());
    await page.mouse.move(at.x, at.y);
    await page.mouse.wheel(0, -300);
    await sleep(900);
    republished = await waitUntil(handleRows, (rows) => rows !== null && !rows.capped && rows.rows.some((row) => row.node === clone), 2500, 400);
  }
  cameras.atTheRead = cameraOf(await vitals());
  const after = republished.value;
  const cloneHandle = (after?.rows ?? []).find((row) => row.node === clone) ?? null;
  const flipped = (after?.rows ?? []).filter((row) => row.node !== x.id && row.node !== clone && before.rows.some((old) => old.id === row.id && old.flag !== row.flag));
  const freed = flipped.length === 1 ? flipped[0] : null;
  const evidence = { source: x.id, sourceHandle: ownBefore, zoomedIn, cameras, clone, cloneHandle, flipped: flipped.slice(0, 6), reselected: reselected.ok, deleted, removed: removed.ok, counts: { nodes: [nodesBefore, removed.value?.nodes ?? null], edges: [edgesBefore, removed.value?.edges ?? null] }, published: after ? { total: after.total, published: after.published, capped: after.capped, waitedMs: republished.waitedMs } : null };
  if (!removed.ok || !cloneHandle || !freed) {
    note("d13-not-provokable-the-freed-handle-is-not-identifiable", { ...evidence, reading: "the handle the deleted node was connected to is read as the one published handle row of another node whose flag flipped with the delete" });
    return;
  }
  const vector: [number, number] = [freed.at[0] - cloneHandle.at[0], freed.at[1] - cloneHandle.at[1]];
  const length = Math.hypot(vector[0], vector[1]);
  const aim: [number, number] = length > 8 ? [Math.round(vector[0] - (8 * vector[0]) / length), Math.round(vector[1] - (8 * vector[1]) / length)] : [Math.round(vector[0]), Math.round(vector[1])];
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const box = await paneBox();
  const cloneAt = toScreen(beforeDrag[clone], camera, box);
  const grab = { x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 };
  const release = { x: grab.x + aim[0] * camera.zoom, y: grab.y + aim[1] * camera.zoom };
  if (!inside(grab, box, 20) || !inside(release, box, 20) || (aim[0] === 0 && aim[1] === 0)) {
    note("d13-not-provokable-the-drop-point-is-outside-the-pane", { ...evidence, freed, vector, aim, grab, release, zoom: camera.zoom });
    return;
  }
  await dragBy(grab, aim[0] * camera.zoom, aim[1] * camera.zoom);
  const dragged = await waitUntil(positions, (now) => offsetOf(beforeDrag, now, clone)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const offset = offsetOf(beforeDrag, dragged.value, clone) ?? [0, 0];
  const connected = await waitUntil(vitals, (v) => (v?.edges ?? -1) === edgesBefore, 10000);
  const dropped = (await waitUntil(handleRows, (rows) => (rows?.rows ?? []).some((row) => row.node === clone), 6000, 400)).value;
  const cloneHandleNow = (dropped?.rows ?? []).find((row) => row.node === clone) ?? null;
  const freedNow = (dropped?.rows ?? []).find((row) => row.id === freed.id) ?? null;
  const gap = cloneHandleNow && freedNow ? Math.hypot(cloneHandleNow.at[0] - freedNow.at[0], cloneHandleNow.at[1] - freedNow.at[1]) : null;
  const drag = dragged.ok ? await newMutation(newestBefore, (row) => row.label.startsWith(dragLabel(1, offset[0], offset[1]))) : { entry: null, mutation: null, waitedMs: 0 };
  const leaves = drag.entry ? (await expandEntry(drag.entry.id)).mutations : [];
  const connect = leaves.find((row) => row.id !== drag.mutation?.id) ?? null;
  const dropEvidence = { ...evidence, freed, vector, aim, offset, handleGapAfterTheDrop: gap, kinds: [cloneHandle.kind, freed.kind], edges: [edgesBefore - 1, connected.value?.edges ?? null], row: drag.entry?.label ?? null, leaves: leaves.map((row) => row.label) };
  if (!drag.mutation || !connect || !connected.ok) {
    note("d13-not-provokable-the-drop-recorded-no-connect", { ...dropEvidence, reading: "`puzzle2d_selection_yields`: a drop pairs every free handle of the moved node with the nearest compatible free handle within the proximity radius (12 world units) and records `connect-handles` beside the drag; with the gap above no connect was recorded — the freed handle may be misread, or the kinds are not compatible" });
    return;
  }
  verdict("d13-a-drop-beside-a-free-handle-records-the-drag-and-its-connect-in-one-row", leaves.length === 2, { ...dropEvidence, reading: "one gesture = one transaction = one History row: the `drag-selection` leaf and the proximity `connect-handles` it landed, which states the tolerance it was recorded under" });
  const head = await positions();
  await installBandTrace();
  const begun = await beginEditOf(drag.mutation.key);
  verdict("d13-edit-opens-the-drag", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const far = Math.round(offset[0]) + 200;
  const typed = await typeEditorNumber("dx", far);
  const previewed = await waitUntil(positions, (now) => placed(now, clone, beforeDrag[clone], far, offset[1], 0.05), 15000);
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("d13-a-drifted-connect-does-not-block-the-review", previewed.ok && review.value?.review === "ready", { typed, via, review: review.value?.review ?? null, text: review.value?.text?.slice(0, 200), clone: previewed.value[clone], expected: [beforeDrag[clone][0] + far, beforeDrag[clone][1] + offset[1]], reading: "design §16.1 / §22.13: the connect is kept and warns; a warning never blocks finalizing" });
  verdict("d13-the-band-names-the-warning", review.value?.outcome === "warning" || /Worst outcome: Warning|Schwerstes Ergebnis: Warnung/u.test(review.value?.text ?? ""), { outcome: review.value?.outcome ?? null, text: review.value?.text?.slice(0, 200) });
  const warned = (await waitUntil(() => findMutationRow(connect.key), (row) => Boolean(row?.text.includes(copy.drifted)), 15000)).value;
  verdict("d13-the-connect-row-reads-precondition-drifted", Boolean(warned?.text.includes(copy.drifted)), { row: warned?.text?.slice(0, 220) ?? null, expected: copy.drifted, reading: "`mutation.precondition-drifted`: the two handles now lie farther apart than the tolerance the connect was recorded under" });
  verdict("d13-the-drifted-warning-is-marked-new-since-this-edit", Boolean(warned?.text.includes(copy.introduced)), { row: warned?.text?.slice(0, 220) ?? null, expected: copy.introduced });
  const keptEdges = (await vitals())?.edges ?? null;
  const viaWithdraw = warned ? await pressRowAction(warned.id, copy.withdraw) : "absent";
  let ready = await waitUntil(band, (b) => b !== null && (b.stage === "editing" || (b.stage === "reviewing" && Boolean(b.review) && b.text.includes(copy.accepted(2)))), 30000, 100);
  if (ready.value?.stage === "editing") {
    await pressBand("accept");
    ready = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  }
  const apart = await waitUntil(vitals, (v) => (v?.edges ?? -1) === edgesBefore - 1, 10000);
  verdict("d13-withdrawing-the-drifted-connect-leaves-a-ready-review-without-a-warning", viaWithdraw === "action" && ready.value?.review === "ready" && Boolean(ready.value.text.includes(copy.accepted(2))) && ready.value.outcome !== "warning" && !/Worst outcome: Warning|Schwerstes Ergebnis: Warnung/u.test(ready.value.text) && apart.ok, { via: viaWithdraw, review: ready.value?.review ?? null, outcome: ready.value?.outcome ?? null, text: ready.value?.text?.slice(0, 220), edges: { kept: keptEdges, withdrawn: apart.value?.edges ?? null, expected: [edgesBefore, edgesBefore - 1] }, expected: copy.accepted(2) });
  await shot("d13-reviewed");
  const finalized = await finalizeOverwrite();
  const overwrite = await waitUntil(async () => documentEntries(await allHistoryRows()).find((row) => row.label.startsWith(copy.overwriteTwo)) ?? null, (row) => row !== null, 20000, 800);
  const settled = await waitUntil(positions, (now) => placed(now, clone, beforeDrag[clone], far, offset[1], 0.05) && movedIds(head, now, 1e-6).every((id) => id === clone), 15000);
  verdict("d13-finalize-overwrite-records-both-edits", finalized.closed && overwrite.ok && settled.ok && ((await vitals())?.edges ?? -1) === edgesBefore - 1, { finalized, row: overwrite.value?.label ?? null, expected: copy.overwriteTwo, clone: settled.value[clone], edges: (await vitals())?.edges ?? null });
  await closePanels();
};

/** 🟦️ The engine's live gesture state of the overview board (React `data-board-interaction-json`: `mode`, `utility`,
 * `hoveredId`), or null where the renderer publishes none. */
const boardInteraction = async (): Promise<{ mode: string; utility: string; hoveredId: string | null } | null> => {
  if (renderer === "wgpu") return null;
  const raw = await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-interaction-json") ?? "", "", `window:${OVERVIEW}`);
  try {
    const parsed = JSON.parse(raw || "null") as { mode?: string; utility?: string; hoveredId?: string | null } | null;
    return parsed ? { mode: parsed.mode ?? "", utility: parsed.utility ?? "", hoveredId: parsed.hoveredId ?? null } : null;
  } catch {
    return null;
  }
};

type TargetRegion = { id: string; x: number; y: number; width: number; height: number; locked: boolean };

/** 🔬️ The target regions the overview board publishes (React `data-board-target-regions-json`, world units). */
const targetRegions = async (): Promise<TargetRegion[]> => {
  if (renderer === "wgpu") return [];
  const raw = await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-target-regions-json") ?? "", "", `window:${OVERVIEW}`);
  try {
    return JSON.parse(raw || "[]") as TargetRegion[];
  } catch {
    return [];
  }
};

/** 🛠️ Arms the overview window's utility `id` through its utility bar (`ui.utilities.<window>` holds one button per utility,
 * its DOM id the utility id; a press on the armed one falls back to select, so an armed utility is left alone). */
const setUtility = async (id: "select" | "brush" | "areaBrush") => {
  if ((await vitals())?.utility === id) return true;
  const button = page.locator(`[id="ui.utilities.${OVERVIEW}"] button[id="${id}"]`).first();
  if (!(await button.count().catch(() => 0))) {
    await page.locator('[id$="2dOverview.utilityBar.unfold"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(700);
  }
  await button.click({ timeout: 4000 }).catch((error) => log(`utility ${id} press failed ${String(error).split("\n")[0]}`));
  return (await waitUntil(vitals, (v) => v?.utility === id, 6000)).ok;
};

/** 🗜️ Zooms in on `world` until the board publishes every on-screen handle (its attribute is capped at 128 rows) and answers
 * the rows. A structural change resets the camera, so every handle aim starts here. */
const revealHandles = async (world: [number, number]) => {
  let read = (await waitUntil(handleRows, (rows) => rows !== null && rows.rows.length > 0, 3000, 400)).value;
  for (let attempt = 0; attempt < 8 && (!read || read.capped || read.rows.length === 0); attempt++) {
    const at = toScreen(world, cameraOf(await vitals()), await paneBox());
    await page.mouse.move(at.x, at.y);
    await page.mouse.wheel(0, -300);
    await sleep(900);
    read = await handleRows();
  }
  return read;
};

/** 🪚️ The History rows newer than `seq`, waited for until `want` document rows are listed: the document rows with their
 * mutation rows, and the command rows that are neither document rows nor the probe's own panel toggles. */
const documentRowsSince = async (seq: number, want: number, timeoutMs: number) => {
  const read = async () => (await allHistoryRows()).filter((row) => row.kind === "entry" && Number(row.key) > seq);
  const settled = await waitUntil(read, (rows) => documentEntries(rows).length >= want, timeoutMs, 1000);
  const documents: { id: string; label: string; mutations: string[]; keys: string[]; count: number }[] = [];
  for (const entry of documentEntries(settled.value)) {
    const expected = 1 + Number(/\(\+(\d+)\)\s*$/u.exec(entry.label)?.[1] ?? "0");
    let mutations = entry.expandable ? (await expandEntry(entry.id)).mutations : [];
    for (let attempt = 0; attempt < 4 && entry.expandable && mutations.length < expected; attempt++) {
      await sleep(800);
      mutations = (await expandEntry(entry.id)).mutations;
    }
    const total = renderer === "wgpu" ? null : ((await treeWindow(`framework.history.entry.${entry.key}`))?.total ?? null);
    documents.push({ id: entry.id, label: entry.label, mutations: mutations.map((row) => row.label), keys: mutations.map((row) => row.key), count: total !== null && Number.isFinite(total) ? total : mutations.length });
  }
  const commands = settled.value.filter((row) => documentEntries([row]).length === 0 && !isShellChromeRow(row)).map((row) => row.label);
  return { documents, commands, waitedMs: settled.waitedMs, newest: Math.max(seq, ...settled.value.map((row) => Number(row.key)).filter(Number.isFinite)) };
};

/** 🧯 The pixels of one board window's canvas (PNG, base64; the header strip and the footer left out) — equality of two
 * shots is the evidence that a window painted nothing new between them. */
const paneShot = async (windowId: string) => {
  const box = await page.locator(`[data-surface-id="window:${windowId}"] canvas`).first().boundingBox().catch(() => null);
  if (!box || box.width < 60 || box.height < 160) return null;
  const shot = await page.screenshot({ clip: { x: Math.round(box.x) + 4, y: Math.round(box.y) + 40, width: Math.round(box.width) - 8, height: Math.round(box.height) - 90 } }).catch(() => null);
  return shot ? shot.toString("base64") : null;
};

/** 🧰 Step 23 — design §22.32 (a), "tools are state machines that yield mutations within a transaction", on the board tools
 * a person reaches: one release = ONE History row with its mutations, a release that changes nothing or an aborted gesture =
 * no row. In order: a node drag's streamed preview paints in the other windows (pixel evidence) and Escape aborts it; a click
 * on empty board; a drag that becomes the upstream row; a history edit begun in the middle of a drag; Select All from the
 * Actions rail and Use selection in an open edit; Delete of a one-handle capsule (node + its edge) and its replay on Accept of
 * the upstream edit; a wire dragged from a clone's free handle onto the freed handle; a second delete and the brush (Alt +
 * sweep over the freed handle) placing a node that is selected at once, edited, withdrawn and restored; the area brush
 * painting a region and a grip resizing it. A precondition the probe cannot establish is a NOTE, never a FAIL. */
const step23 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  if (renderer === "wgpu") {
    note("m32-not-probed-on-this-renderer", { reading: "the step aims at handles, regions and the utility bar through React's `data-board-*` attributes and DOM ids; the wgpu arm has no reader for them yet" });
    return;
  }
  const settledNewestSeq = async () => {
    let last = -2;
    const read = await waitUntil(async () => newestEntrySeq(await allHistoryRows()), (newest) => {
      const same = newest > 0 && newest === last;
      last = newest;
      return same;
    }, 12000, 600);
    return read.value;
  };
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { camera, picked } = await pickNodes(2, [[50, 40]], exclude, 60);
  if (picked.length < 2) {
    verdict("m32-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const [vNode, uNode] = picked;
  const zoom = camera.zoom;
  const windows = ["2d-detail", "2d-selection"];
  const shots = async () => {
    const out: Record<string, string | null> = {};
    for (const id of windows) out[id] = await paneShot(id);
    return out;
  };
  let seq = await settledNewestSeq();
  const p0 = await positions();
  await selectNodes([vNode]);
  await page.mouse.move(vNode.at.x, vNode.at.y);
  await sleep(1500);
  const still1 = await shots();
  const still2 = await shots();
  await page.mouse.down();
  await sleep(700);
  const pressed1 = await shots();
  const pressed2 = await shots();
  await page.mouse.move(vNode.at.x + 50 * zoom, vNode.at.y + 40 * zoom, { steps: 8 });
  await sleep(1000);
  const moved = await shots();
  const dragMode = (await boardInteraction())?.mode ?? null;
  const perWindow = windows.map((id) => ({ window: id, shot: moved[id] !== null, stableBeforeThePress: still1[id] !== null && still1[id] === still2[id], stableWhilePressed: pressed1[id] !== null && pressed1[id] === pressed2[id], paintedWhileDragged: moved[id] !== pressed2[id] }));
  const judged = perWindow.filter((row) => row.stableBeforeThePress && row.stableWhilePressed);
  verdict("m32-a-streamed-drag-preview-paints-in-the-other-windows", dragMode === "dragNodes" && judged.length > 0 && judged.every((row) => row.paintedWhileDragged), { perWindow, judgedOn: judged.map((row) => row.window), mode: dragMode, reading: "S5-PUZZLE B3: an open stream's preview paints in every window (framework overlay). Evidence is pixels: a window whose canvas is pixel-stable before the press and while the node is held must paint something new once the node is dragged; a window that is not pixel-stable on its own is listed, not judged" });
  await page.keyboard.press("Escape").catch(() => {});
  await sleep(600);
  await page.mouse.up();
  await sleep(2500);
  const afterEscape = await positions();
  const escapeRows = await documentRowsSince(seq, 0, 0);
  verdict("m32-an-aborted-drag-leaves-no-row", dragMode === "dragNodes" && movedIds(p0, afterEscape, 1e-6).length === 0 && escapeRows.documents.length === 0, { mode: dragMode, moved: movedIds(p0, afterEscape, 1e-6).slice(0, 4), documentRows: escapeRows.documents.map((row) => row.label), commandRows: escapeRows.commands, reading: "Escape in the middle of a node drag cancels the gesture: the node is back, the tool's transaction yields nothing, History lists no document row" });
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 4000);
  await sleep(1500);
  seq = Math.max(seq, await settledNewestSeq());
  await closePanels();
  const first = await layout();
  let empty: Point | null = null;
  for (let gy = 120; gy < first.box.height - 120 && !empty; gy += 40)
    for (let gx = 40; gx < first.box.width - 40 && !empty; gx += 40) {
      const candidate = { x: first.box.x + gx, y: first.box.y + gy };
      if (first.rows.every((row) => Math.hypot(row.at.x - candidate.x, row.at.y - candidate.y) > 90)) empty = candidate;
    }
  const beforeIdle = await positions();
  if (empty) await page.mouse.click(empty.x, empty.y);
  await sleep(3000);
  const idle = await documentRowsSince(seq, 0, 0);
  verdict("m32-a-press-and-release-that-changes-nothing-leaves-no-row", empty !== null && idle.documents.length === 0 && idle.commands.length === 0 && movedIds(beforeIdle, await positions(), 1e-6).length === 0 && selectionIds(await vitals()).length === 0, { at: empty, documentRows: idle.documents.map((row) => row.label), commandRows: idle.commands, reading: "a click on empty board with nothing selected changes neither the document nor the selection: zero trace, not even a command row" });
  seq = idle.newest;
  await selectNodes([uNode]);
  const pU = await positions();
  await dragBy(uNode.at, 40 * zoom, 30 * zoom);
  const uMoved = await waitUntil(positions, (now) => offsetOf(pU, now, uNode.id)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const uOffset = offsetOf(pU, uMoved.value, uNode.id) ?? [0, 0];
  const uDrag = await newMutation(seq, (row) => row.label.startsWith(dragLabel(1, uOffset[0], uOffset[1])));
  if (!uDrag.mutation) {
    verdict("m32-the-upstream-drag-is-recorded", false, { offset: uOffset, expected: dragLabel(1, uOffset[0], uOffset[1]) });
    return;
  }
  const uKey = uDrag.mutation.key;
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 4000);
  await sleep(1200);
  const uRow = await findMutationRow(uKey);
  seq = newestEntrySeq(await readHistory());
  const pG = await positions();
  const [vLive] = await rowsOf([vNode.id]);
  const vAt = vLive?.at ?? vNode.at;
  await page.mouse.move(vAt.x, vAt.y);
  await page.mouse.down();
  await page.mouse.move(vAt.x + 50 * zoom, vAt.y + 40 * zoom, { steps: 8 });
  await sleep(700);
  const modeMid = (await boardInteraction())?.mode ?? null;
  const begunInPage = uRow
    ? await evalSafe(
        (arg) => {
          const row = document.getElementById(arg.rowId);
          const button = Array.from(row?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => new RegExp(arg.source, arg.flags).test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
          button?.click();
          return Boolean(button);
        },
        false,
        { rowId: uRow.id, source: copy.edit.source, flags: copy.edit.flags },
      )
    : false;
  const frozen = await waitUntil(band, (state) => state?.stage === "editing", 10000, 100);
  const modeFrozen = (await boardInteraction())?.mode ?? null;
  const codesFrozen = await shownNoticeCodes();
  await page.mouse.up();
  await sleep(1500);
  if (await band()) await exitSession();
  await sleep(2000);
  const afterG = await positions();
  const gRows = await documentRowsSince(seq, 0, 0);
  verdict("m32-a-history-edit-begun-mid-drag-leaves-no-row", modeMid === "dragNodes" && begunInPage && frozen.ok && movedIds(pG, afterG, 1e-6).length === 0 && gRows.documents.length === 0, { modeWhileDragging: modeMid, editPressedInPage: begunInPage, bandAfterThePress: frozen.value?.stage ?? null, waitedMs: frozen.waitedMs, modeAfterTheBegin: modeFrozen, notices: codesFrozen.slice(-4), moved: movedIds(pG, afterG, 1e-6).slice(0, 4), documentRows: gRows.documents.map((row) => `${row.label} [${row.mutations.join(" | ")}]`), commandRows: gRows.commands, reading: "a history edit that begins while a board gesture is live freezes the document: the gesture is retired, its release commits nothing, the node is where it was once the session is left. Edit is pressed in page (a DOM click on the row action) because the pointer is held on the board" });
  seq = gRows.newest;
  await closePanels();
  const headU = await positions();
  const pane = page.locator(`[data-slot="window"][id="${OVERVIEW}"]`).first();
  const rail = pane.locator('[data-slot="window-action-pane"]').first();
  const railToggle = pane.locator('[id$=".engagement.toggle"]').first();
  const selectAllFromTheRail = async () => {
    if (!(await rail.isVisible().catch(() => false))) await railToggle.click({ timeout: 4000 }).catch(() => {});
    const item = pane.locator('[id="action.selectAll"]').first();
    const listed = await waitUntil(() => item.count().catch(() => 0), (count) => count > 0, 20000);
    const label = item.locator('[data-slot="tree-label"]').first();
    if (listed.ok) await ((await label.count().catch(() => 0)) ? label : item).click({ timeout: 4000 }).catch((error) => log(`select all press failed ${String(error).split("\n")[0]}`));
    const all = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0 && selectionIds(v).length === (v?.nodes ?? -1), 15000);
    if (await rail.isVisible().catch(() => false)) await railToggle.click({ timeout: 4000 }).catch(() => {});
    return { listed: listed.ok, ok: all.ok, selected: selectionIds(all.value).length, nodes: all.value?.nodes ?? null, waitedMs: all.waitedMs };
  };
  const everything = await selectAllFromTheRail();
  verdict("m32-select-all-from-the-actions-rail-selects-every-node", everything.listed && everything.ok, { ...everything, reading: "F16: the Actions rail's Select All selects every node of the board (the domain declares its topology)" });
  const begunAll = await beginEditOf(uKey);
  if (begunAll.band?.stage === "editing") {
    const kept = selectionIds(await vitals()).length;
    const again = kept === everything.nodes ? null : await selectAllFromTheRail();
    await openHistory();
    const used = await pressUseSelection("targets");
    const sample = Object.keys(headU).filter((id) => id !== uNode.id).slice(0, 5);
    const taken = await waitUntil(positions, (now) => placed(now, uNode.id, pU[uNode.id], uOffset[0], uOffset[1], 0.05) && sample.every((id) => placed(now, id, headU[id], uOffset[0], uOffset[1], 0.05)), 30000, 500);
    const editor = await readEditor();
    verdict("m32-use-selection-takes-the-select-all-selection", used !== null && taken.ok, { used, selectionWhenTheEditOpened: kept, selectedAgainInTheSession: again, chipsMaterialised: editor?.targets?.chips.length ?? null, sample: sample.slice(0, 3).map((id) => ({ id, at: taken.value[id] ?? null, expected: [headU[id][0] + uOffset[0], headU[id][1] + uOffset[1]] })), waitedMs: taken.waitedMs, reading: "Use selection in the open edit of the drag takes the Select All selection as its targets: the preview drags every node by the recorded offset" });
    await pressBand("discard");
    await sleep(700);
    await exitSession();
    await waitUntil(positions, (now) => movedIds(headU, now, 1e-6).length === 0, 20000, 500);
  } else verdict("m32-use-selection-takes-the-select-all-selection", false, { via: begunAll.via, band: begunAll.band?.stage ?? null });
  await closePanels();
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 6000);
  const cameraHome = cameraOf(await vitals());
  const focus: [number, number] = [cameraHome.x, cameraHome.y];
  const firstRows = await revealHandles(focus);
  const posNow = await positions();
  const camNow = cameraOf(await vitals());
  const boxNow = await paneBox();
  const perNode = new Map<string, number>();
  for (const row of firstRows?.rows ?? []) perNode.set(row.node, (perNode.get(row.node) ?? 0) + 1);
  const middle = { x: boxNow.x + boxNow.width / 2, y: boxNow.y + boxNow.height / 2 };
  const capsules = (firstRows?.rows ?? [])
    .filter((row) => /capsule/u.test(row.kind) && row.flag === false && perNode.get(row.node) === 1 && row.node !== uNode.id && row.node !== vNode.id && Boolean(posNow[row.node]) && inside(toScreen(posNow[row.node], camNow, boxNow), boxNow, 120))
    .map((row) => ({ id: row.node, kind: row.kind, far: Math.hypot(toScreen(posNow[row.node], camNow, boxNow).x - middle.x, toScreen(posNow[row.node], camNow, boxNow).y - middle.y) }))
    .sort((left, right) => left.far - right.far);
  const twin = capsules.find((row, index) => index > 0 && row.kind === capsules[0]?.kind) ?? null;
  const third = capsules.find((row, index) => index > 0 && row.id !== twin?.id) ?? null;
  if (!twin || !third) {
    note("m32-not-provokable-fewer-than-three-connected-one-handle-nodes-in-view", { capsules: capsules.slice(0, 8), published: firstRows ? { total: firstRows.total, published: firstRows.published, capped: firstRows.capped } : null, camera: camNow, reading: "the step deletes one such node, duplicates a second one of the SAME handle kind (its clone's free handle is then compatible with the handle the delete freed) and deletes a third" });
    return;
  }
  const [x1, c2, x2] = [capsules[0].id, twin.id, third.id];
  const cameras: { when: string; x: number; y: number; zoom: number }[] = [];
  const mark = async (when: string) => {
    const now = cameraOf(await vitals());
    cameras.push({ when, x: Number(now.x.toFixed(2)), y: Number(now.y.toFixed(2)), zoom: Number(now.zoom.toFixed(3)) });
  };
  const stableDocumentRows = async () => {
    let last = -1;
    const read = await waitUntil(async () => documentEntries(await allHistoryRows()).length, (count) => {
      const same = count > 0 && count === last;
      last = count;
      return same;
    }, 12000, 700);
    return read.value;
  };
  const deleteNode = async (id: string) => {
    await mark(`before the delete of ${id.slice(0, 8)}`);
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    await waitUntil(vitals, (v) => selectionIds(v).length === 0, 5000);
    const before = await revealHandles(focus);
    const counts = await vitals();
    const at = toScreen((await positions())[id], cameraOf(await vitals()), await paneBox());
    await page.mouse.click(at.x - 6, at.y + 6);
    const selected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === id, 8000);
    const selection = selectionIds(selected.value).slice(0, 4);
    const cameraBefore = cameraOf(await vitals());
    await prepareChord();
    await page.keyboard.press("Delete").catch(() => {});
    const removed = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === (counts?.nodes ?? 0) - 1, 15000);
    await sleep(2500);
    const cameraAfter = cameraOf(await vitals());
    await mark(`2.5 s after the delete of ${id.slice(0, 8)}`);
    const after = await revealHandles(focus);
    const freed = (after?.rows ?? []).filter((row) => row.node !== id && (before?.rows ?? []).some((old) => old.id === row.id && old.flag !== row.flag));
    return { selected: selected.ok, selection, removed: removed.ok, nodes: [counts?.nodes ?? null, removed.value?.nodes ?? null], edges: [counts?.edges ?? null, removed.value?.edges ?? null], freed, cameraBefore, cameraAfter };
  };
  seq = Math.max(seq, await settledNewestSeq());
  await closePanels();
  const d1 = await deleteNode(x1);
  const d1Rows = await documentRowsSince(seq, 1, 20000);
  verdict("m32-deleting-a-node-with-its-edge-is-one-history-row", d1.selected && d1.removed && d1.edges[1] === (d1.edges[0] ?? 0) - 1 && d1Rows.documents.length === 1 && d1Rows.documents[0].mutations.some((label) => label.includes(x1)), { node: x1, nodes: d1.nodes, edges: d1.edges, rows: d1Rows.documents.map((row) => ({ row: row.label, mutationCount: row.count, mutations: row.mutations })), commandRows: d1Rows.commands, freedHandles: d1.freed.map((row) => row.id), reading: "`nodeDelete`: Delete on a selected node removes the node and its edge in ONE tool transaction — one History row" });
  note("m32-camera-after-a-node-delete", { before: d1.cameraBefore, after: d1.cameraAfter, reset: Math.abs(d1.cameraBefore.zoom - d1.cameraAfter.zoom) > 1e-6 || Math.abs(d1.cameraBefore.x - d1.cameraAfter.x) > 1e-6, reading: "O6: whether a structural change takes the person's camera back to its origin (read 2.5 s after the delete)" });
  seq = d1Rows.newest;
  const headD = await positions();
  const begunUp = await beginEditOf(uKey);
  if (begunUp.band?.stage === "editing") {
    const farther = Math.round(uOffset[0]) + 20;
    const typed = await typeEditorNumber("dx", farther);
    const preview = await waitUntil(positions, (now) => placed(now, uNode.id, pU[uNode.id], farther, uOffset[1], 0.05), 20000);
    verdict("m32-an-upstream-edit-previews-without-the-later-delete", preview.ok && Boolean(preview.value[x1]), { typed, u: preview.value[uNode.id] ?? null, expected: [pU[uNode.id][0] + farther, pU[uNode.id][1] + uOffset[1]], deletedNodeInThePreview: preview.value[x1] ?? null, reading: "the preview is the document as of the edited drag plus the draft: the later delete is not applied, the deleted node is shown" });
    const via = await pressBand("accept");
    const review = await waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 60000, 100);
    const replayed = await waitUntil(positions, (now) => !now[x1] && placed(now, uNode.id, pU[uNode.id], farther, uOffset[1], 0.05), 20000);
    verdict("m32-accept-of-the-upstream-edit-replays-the-delete", review.value?.review === "ready" && replayed.ok, { via, review: review.value?.review ?? null, text: review.value?.text?.slice(0, 160), deletedNodeAfterAccept: replayed.value[x1] ?? null, u: replayed.value[uNode.id] ?? null, reading: "Accept replays every later mutation: the delete row's mutations are applied again on the edited history, the review is ready" });
    await exitSession();
    await waitUntil(positions, (now) => movedIds(headD, now, 1e-6).length === 0 && !now[x1], 20000, 500);
  } else verdict("m32-an-upstream-edit-previews-without-the-later-delete", false, { via: begunUp.via, band: begunUp.band?.stage ?? null });
  seq = Math.max(seq, await settledNewestSeq());
  await closePanels();
  await mark("after the upstream edit session, before the duplicate");
  await revealHandles(focus);
  const idsBeforeClone = Object.keys(await positions());
  const c2At = toScreen((await positions())[c2], cameraOf(await vitals()), await paneBox());
  await page.mouse.click(c2At.x - 6, c2At.y + 6);
  await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === c2, 8000);
  const beforeClone = await vitals();
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === (beforeClone?.nodes ?? 0) + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBeforeClone.includes(id)) ?? null;
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 6000);
  const cloneRows = await documentRowsSince(seq, 1, 15000);
  seq = cloneRows.newest;
  await closePanels();
  await mark("after the duplicate, before the wire");
  const wireRead = await revealHandles(focus);
  const cloneHandle = clone ? (wireRead?.rows.find((row) => row.node === clone) ?? null) : null;
  const wireTarget = d1.freed.length === 1 ? (wireRead?.rows.find((row) => row.id === d1.freed[0].id) ?? null) : null;
  if (!cloneHandle || !wireTarget) note("m32-not-provokable-no-free-handle-pair-for-a-wire", { clone, cloneHandle, freed: d1.freed.map((row) => row.id), wireTarget, cloneRow: cloneRows.documents.map((row) => row.label) });
  else {
    const cam = cameraOf(await vitals());
    const box = await paneBox();
    const from = toScreen(cloneHandle.at, cam, box);
    const to = toScreen(wireTarget.at, cam, box);
    const edgesBefore = (await vitals())?.edges ?? -1;
    await page.mouse.move(from.x, from.y);
    await sleep(600);
    await page.mouse.down();
    await page.mouse.move((from.x + to.x) / 2, (from.y + to.y) / 2, { steps: 6 });
    await sleep(500);
    const wireMode = (await boardInteraction())?.mode ?? null;
    await page.mouse.move(to.x, to.y, { steps: 6 });
    await sleep(600);
    await page.mouse.up();
    const wired = await waitUntil(vitals, (v) => (v?.edges ?? -1) === edgesBefore + 1, 15000);
    const wireRows = await documentRowsSince(seq, 1, 20000);
    verdict("m32-a-wire-dragged-handle-to-handle-is-one-history-row", wired.ok && wireRows.documents.length === 1 && wireRows.documents[0].count === 1 && wireRows.documents[0].mutations[0].includes(cloneHandle.id) && wireRows.documents[0].mutations[0].includes(wireTarget.id), { from: cloneHandle.id, to: wireTarget.id, kinds: [cloneHandle.kind, wireTarget.kind], mode: wireMode, edges: [edgesBefore, wired.value?.edges ?? null], rows: wireRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })), commandRows: wireRows.commands, reading: "`edgeCreate`: a wire dragged from a free handle and dropped on a compatible free handle is ONE tool transaction — one History row holding the connect" });
    seq = wireRows.newest;
  }
  await closePanels();
  const d2 = await deleteNode(x2);
  const d2Rows = await documentRowsSince(seq, 1, 20000);
  seq = d2Rows.newest;
  await closePanels();
  if (!d2.removed || d2.freed.length !== 1) note("m32-not-provokable-no-freed-handle-for-the-brush", { node: x2, selected: d2.selected, selection: d2.selection, removed: d2.removed, nodes: d2.nodes, edges: d2.edges, freed: d2.freed.map((row) => row.id), rows: d2Rows.documents.map((row) => ({ row: row.label, mutations: row.mutations.slice(0, 6) })), commandRows: d2Rows.commands });
  else {
    const armed = await setUtility("brush");
    await mark("before the brush");
    const brushRead = await revealHandles(focus);
    const slot = brushRead?.rows.find((row) => row.id === d2.freed[0].id) ?? d2.freed[0];
    const at = toScreen(slot.at, cameraOf(await vitals()), await paneBox());
    const idsBeforeBrush = Object.keys(await positions());
    const beforeBrush = await vitals();
    await page.mouse.move(at.x - 90, at.y - 60);
    await page.keyboard.down("Alt");
    await page.mouse.move(at.x, at.y, { steps: 8 });
    await sleep(1500);
    const hovered = (await boardInteraction())?.hoveredId ?? null;
    await page.mouse.move(at.x - 110, at.y - 80, { steps: 8 });
    await sleep(600);
    await page.keyboard.up("Alt");
    const stamped = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === (beforeBrush?.nodes ?? 0) + 1, 15000);
    const stampedAt = Date.now();
    const stampedId = Object.keys(await positions()).find((id) => !idsBeforeBrush.includes(id)) ?? null;
    await mark("right after the brush placed its node");
    const selectArmed = await setUtility("select");
    const stays = { clickedAfterMs: null as number | null, attempts: 0, selected: false, selection: [] as string[], stillSelectedAfterMs: null as number | null, still: false };
    const hovers: string[] = [];
    for (let attempt = 0; attempt < 2 && stampedId && !stays.selected; attempt++) {
      const where = (await positions())[stampedId];
      if (!where) break;
      const centre = toScreen(where, cameraOf(await vitals()), await paneBox());
      let aim: Point | null = null;
      for (const [dx, dy] of [[0, 0], [-6, 6], [6, 6], [8, 0], [-8, 0], [0, 9], [0, -9], [12, 12], [-12, 12], [14, 0], [-14, 0]] as const) {
        await page.mouse.move(centre.x + dx, centre.y + dy);
        await sleep(260);
        const over = (await boardInteraction())?.hoveredId ?? null;
        hovers.push(`${dx},${dy}=${over ?? "nothing"}`);
        if (over === stampedId) {
          aim = { x: centre.x + dx, y: centre.y + dy };
          break;
        }
      }
      if (!aim) continue;
      stays.attempts += 1;
      await page.mouse.click(aim.x, aim.y);
      stays.clickedAfterMs ??= Date.now() - stampedAt;
      const selected = await waitUntil(vitals, (v) => selectionIds(v).includes(stampedId), 5000);
      stays.selected = selected.ok;
      stays.selection = selectionIds(selected.value).slice(0, 4);
    }
    if (stays.selected && stampedId) {
      await sleep(3500);
      stays.stillSelectedAfterMs = 3500;
      stays.still = selectionIds(await vitals()).includes(stampedId);
    }
    const brushRows = await documentRowsSince(seq, 1, 20000);
    const brushRow = brushRows.documents[0] ?? null;
    verdict("m32-a-brush-placed-node-is-one-history-row", armed && stamped.ok && stampedId !== null && brushRows.documents.length === 1 && brushRow?.count === 2 && Boolean(brushRow.mutations.some((label) => label.includes(stampedId) && !label.includes(slot.id))) && (stamped.value?.edges ?? -1) === (beforeBrush?.edges ?? 0) + 1, { slot: slot.id, hoveredWhileArmed: hovered, placed: stampedId, nodes: [beforeBrush?.nodes ?? null, stamped.value?.nodes ?? null], edges: [beforeBrush?.edges ?? null, stamped.value?.edges ?? null], rows: brushRows.documents.map((row) => ({ row: row.label, mutationCount: row.count, materialised: row.mutations })), commandRows: brushRows.commands, reading: "`brushPlace`: with the brush armed, Alt held over an open handle previews a compatible node and leaving the slot stamps it — the node and its connect in ONE tool transaction, one History row" });
    if (stays.attempts === 0) note("m32-the-placed-node-could-not-be-aimed-at", { placed: stampedId, at: stampedId ? ((await positions())[stampedId] ?? null) : null, hovers: hovers.slice(0, 22), reading: "the board never reported the placed node under the pointer (`hoveredId`) at or around its published position, so the click that should select it was not made and the verdict is not judged" });
    else verdict("m32-a-placed-node-selected-at-once-stays-selected", selectArmed && stays.selected && stays.still, { ...stays, hovers: hovers.slice(0, 12), placed: stampedId, reading: "a node that was just placed is clicked as soon as the select utility is armed: it is selected and still selected 3.5 s later, after the guest's publication of the placement came back" });
    seq = brushRows.newest;
    const createIndex = brushRow && stampedId ? brushRow.mutations.findIndex((label) => label.includes(stampedId) && !label.includes(slot.id)) : -1;
    const createKey = brushRow && createIndex >= 0 ? brushRow.keys[createIndex] : null;
    if (!createKey || !stampedId) note("m32-the-create-mutation-of-the-placed-node-is-not-listed", { rows: brushRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })) });
    else {
      const createRow = await findMutationRow(createKey);
      const editOffer = createRow ? await rowActionState(createRow.id, copy.edit, false) : null;
      const begunC = editOffer?.present && editOffer.disabled !== true ? await beginEditOf(createKey) : null;
      const editorC = begunC?.band?.stage === "editing" ? (await waitUntil(readEditor, (value) => (value?.inputs.length ?? 0) > 0, 10000)).value : null;
      verdict("m32-edit-of-the-placed-node-offers-its-inputs", begunC?.band?.stage === "editing" && (editorC?.inputs.length ?? 0) > 0, { row: createRow?.label ?? null, edit: editOffer ? { present: editOffer.present, disabled: editOffer.disabled, reason: editOffer.reason } : null, band: begunC?.band?.text?.slice(0, 140) ?? null, inputs: editorC?.inputs.slice(0, 14) ?? null, reading: "Edit on the placed node's create mutation opens the editor with the leaf's inputs; a mutation whose inputs cannot be edited offers Edit disabled with its reason" });
      if (await band()) await exitSession();
      const headC = await positions();
      const nodesHeadC = (await vitals())?.nodes ?? null;
      const rowsHeadC = await stableDocumentRows();
      const forWithdraw = await findMutationRow(createKey);
      const withdrawOffer = forWithdraw ? await rowActionState(forWithdraw.id, copy.withdraw, false) : null;
      const bandBeforeWithdraw = (await band())?.stage ?? null;
      const viaWithdraw = forWithdraw ? await pressRowAction(forWithdraw.id, copy.withdraw) : "absent";
      let withdrawn = await waitUntil(band, (state) => state !== null && (state.stage === "editing" || (state.stage === "reviewing" && Boolean(state.review))), 20000, 100);
      if (withdrawn.value?.stage === "editing") {
        await pressBand("accept");
        withdrawn = await waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 60000, 100);
      }
      const gone = await waitUntil(positions, (now) => !now[stampedId], 15000);
      const forRestore = await findMutationRow(createKey);
      const restoreOffer = forRestore ? await rowActionState(forRestore.id, copy.restore, false) : null;
      const viaRestore = forRestore ? await pressRowAction(forRestore.id, copy.restore) : "absent";
      const back = await waitUntil(positions, (now) => Boolean(now[stampedId]), 20000);
      const afterRestore = await band();
      verdict("m32-withdraw-then-restore-of-the-placed-node-works", viaWithdraw === "action" && gone.ok && viaRestore === "action" && back.ok, { withdraw: { offered: withdrawOffer ? { present: withdrawOffer.present, disabled: withdrawOffer.disabled, reason: withdrawOffer.reason, busy: withdrawOffer.busy } : null, bandBeforeThePress: bandBeforeWithdraw, bandAfterThePress: withdrawn.value?.stage ?? null, waitedMs: withdrawn.waitedMs, notices: (await shownNoticeCodes()).slice(-4), via: viaWithdraw, review: withdrawn.value?.review ?? null, text: withdrawn.value?.text?.slice(0, 160), nodeGoneFromThePreview: gone.ok }, restore: { offered: restoreOffer ? { present: restoreOffer.present, disabled: restoreOffer.disabled } : null, via: viaRestore, nodeBackInThePreview: back.ok, band: afterRestore?.stage ?? null, review: afterRestore?.review ?? null }, reading: "Withdraw on the create mutation's row opens a session whose preview lacks the node; Restore on the same row drops that draft and the node is back" });
      if (await band()) await exitSession();
      const settledC = await waitUntil(positions, (now) => movedIds(headC, now, 1e-6).length === 0 && Boolean(now[stampedId]) && Object.keys(now).length === Object.keys(headC).length, 20000, 500);
      const rowsAfterC = await stableDocumentRows();
      verdict("m32-withdraw-then-restore-leaves-the-document-as-it-was", settledC.ok && rowsAfterC === rowsHeadC, { band: (await band())?.stage ?? null, placedNodeInTheDocument: Boolean(settledC.value[stampedId]), nodes: [nodesHeadC, (await vitals())?.nodes ?? null], positions: [Object.keys(headC).length, Object.keys(settledC.value).length], drift: movedIds(headC, settledC.value, 1e-6).slice(0, 4), rows: [rowsHeadC, rowsAfterC], waitedMs: settledC.waitedMs, reading: "Restore dropped the only draft, so the session is left with nothing to change: the document and its History rows are those from before the Withdraw" });
    }
  }
  await closePanels();
  await setUtility("select");
  await mark("before the regions");
  const home = cameras.filter((row, index) => index > 0 && row.zoom === 1 && row.x === 0 && row.y === 0 && cameras[index - 1].zoom !== 1);
  note("m32-camera-trace", { cameras, cameBackToItsOrigin: home.map((row) => row.when), reading: "O6: the probe zooms in to aim at handles; a camera that is back at (0, 0) zoom 1 at a later mark without the probe zooming out was reset by the shell after a structural change (delete, duplicate, wire, brush)" });
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  for (let attempt = 0; attempt < 8 && cameraOf(await vitals()).zoom > 1.01; attempt++) {
    const box = await paneBox();
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(900);
  }
  seq = Math.max(seq, await settledNewestSeq());
  const wide = await layout();
  let spot: Point | null = null;
  for (let gy = 140; gy < wide.box.height - 240 && !spot; gy += 40)
    for (let gx = 30; gx < wide.box.width - 200 && !spot; gx += 40) {
      const candidate = { x: wide.box.x + gx, y: wide.box.y + gy };
      const probes = [candidate, { x: candidate.x + 130, y: candidate.y + 100 }, { x: candidate.x + 65, y: candidate.y + 50 }, { x: candidate.x + 130, y: candidate.y }, { x: candidate.x, y: candidate.y + 100 }];
      if (wide.rows.every((row) => probes.every((point) => Math.hypot(row.at.x - point.x, row.at.y - point.y) > 80))) spot = candidate;
    }
  if (!spot) {
    note("m32-not-provokable-no-empty-board-area-for-a-region", { camera: wide.camera, nodes: wide.rows.length });
    return;
  }
  const areaArmed = await setUtility("areaBrush");
  await openHistory();
  await scrollHistory("start");
  await sleep(600);
  const regionsBefore = await targetRegions();
  await page.mouse.move(spot.x, spot.y);
  await page.mouse.down();
  await page.mouse.move(spot.x + 90, spot.y + 70, { steps: 8 });
  await sleep(400);
  const paintMode = (await boardInteraction())?.mode ?? null;
  await page.mouse.up();
  const painted = await waitUntil(targetRegions, (list) => list.length === regionsBefore.length + 1, 8000);
  const region = painted.value.find((row) => !regionsBefore.some((old) => old.id === row.id)) ?? null;
  const seqAtThePaint = seq;
  const shown = await waitUntil(readHistory, (rows) => rows.some((row) => row.kind === "entry" && Number(row.key) > seqAtThePaint && documentEntries([row]).length > 0), 8000, 400);
  const idleCreate = await documentRowsSince(seq, 1, 15000);
  let createRows = idleCreate;
  let flushedBy = idleCreate.documents.length ? "nothing — it arrived while the pointer rested" : "";
  if (!createRows.documents.length) {
    await page.mouse.move(spot.x + 200, spot.y + 150, { steps: 4 });
    createRows = await documentRowsSince(seq, 1, 6000);
    if (createRows.documents.length) flushedBy = "the next pointer move over the board";
  }
  if (!createRows.documents.length) {
    await setUtility("select");
    createRows = await documentRowsSince(seq, 1, 8000);
    if (createRows.documents.length) flushedBy = "arming another utility";
  }
  verdict("m32-creating-a-region-is-one-history-row", areaArmed && painted.ok && region !== null && shown.ok && idleCreate.documents.length === 1 && idleCreate.documents[0].count === 1 && Boolean(idleCreate.documents[0].mutations[0]?.includes(region.id)), { mode: paintMode, region, rowsCountedAfterSequence: seqAtThePaint, listedInTheOpenHistoryPanelUntouched: { ok: shown.ok, waitedMs: shown.waitedMs, reading: "the History panel is open and scrolled to its newest rows before the paint; for 8 s after the release nothing is pressed, moved or scrolled — the new row must show on its own, as it does for a drag" }, rowsWhileThePointerRested: idleCreate.documents.map((row) => ({ row: row.label, mutations: row.mutations })), restedMs: idleCreate.waitedMs, rowsInTheEnd: createRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })), broughtBy: flushedBy || "nothing brought it within 29 s", commandRows: createRows.commands, reading: "`regionCreate`: the area brush's release paints a target region — ONE tool transaction, one History row, committed by the release itself. A region that is on the board while History lists no row until the next input is a release that was not committed" });
  seq = createRows.newest;
  if (!region) return;
  await setUtility("select");
  const createArrived = createRows.documents.length > 0;
  const camR = cameraOf(await vitals());
  const boxR = await paneBox();
  const grip = toScreen([region.x + region.width, region.y + region.height], camR, boxR);
  await page.mouse.move(grip.x - 1, grip.y - 1);
  await sleep(600);
  await page.mouse.down();
  await page.mouse.move(grip.x + 40 * camR.zoom, grip.y + 30 * camR.zoom, { steps: 8 });
  await sleep(400);
  const gripMode = (await boardInteraction())?.mode ?? null;
  await page.mouse.up();
  const resized = await waitUntil(targetRegions, (list) => Math.abs((list.find((row) => row.id === region.id)?.width ?? region.width) - region.width) > 1, 8000);
  const resizeRows = await documentRowsSince(seq, createArrived ? 1 : 2, 15000);
  const now = resized.value.find((row) => row.id === region.id) ?? null;
  verdict("m32-resizing-a-region-is-one-history-row", resized.ok && resizeRows.documents.length === (createArrived ? 1 : 2) && resizeRows.documents.every((row) => row.count === 1 && Boolean(row.mutations[0]?.includes(region.id))), { mode: gripMode, size: { before: [region.width, region.height], after: now ? [now.width, now.height] : null }, rows: resizeRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })), createRowWasListedBefore: createArrived, commandRows: resizeRows.commands, reading: "`regionResize`: a grip drag commits one absolute pose on release — ONE tool transaction, one History row (when the create row had not been listed yet, it arrives with it and both are counted)" });
  await closePanels();
};

type UniversalControl = UniversalControlV1;
type UniversalVerb = UniversalVerbV1;

/** 🧭 The input controls of the open history editor, read without knowing the editor: every `framework.history.editor.input.<pointer>.row`
 * with the contract role of the control it holds (slider, spinbutton, radiogroup, listbox, combobox, switch, textbox, or a
 * reference list when the row has Use selection / chips), its accessible name, value and min / max / step where the DOM
 * carries them. Each control is stamped `data-probe-u=<index>` so it can be operated. */
const universalControls = async (): Promise<UniversalControl[]> => renderer === "wgpu" ? universalMirrorControlsV1(await mirror()) :
  evalSafe(
    () => {
      const all = Array.from(document.querySelectorAll<HTMLElement>("[id]"));
      const rows = all.filter((el) => /framework\.history\.editor\.input\.[^/␟]+\.row$/u.test(el.id));
      const pointers = rows.map((row) => row.id.replace(/^.*framework\.history\.editor\.input\./u, "").replace(/\.row$/u, ""));
      const out: UniversalControl[] = [];
      document.querySelectorAll("[data-probe-u]").forEach((el) => el.removeAttribute("data-probe-u"));
      rows.forEach((row, at) => {
        const pointer = pointers[at];
        if (/\.(useSelection|chip\.\d+)$/u.test(pointer)) return;
        const name = ((row.querySelector('[data-slot="tree-label"]') as HTMLElement | null)?.innerText ?? row.getAttribute("aria-label") ?? "").replace(/\s+/gu, " ").trim().slice(0, 80);
        const own = all.find((el) => el.id.startsWith(`${row.id}/`)) ?? row;
        const within = (selector: string) => (own.matches(selector) ? own : own.querySelector<HTMLElement>(selector)) ?? row.querySelector<HTMLElement>(selector);
        const chips = pointers.filter((other) => other.startsWith(`${pointer}.chip.`)).length;
        const reference = pointers.includes(`${pointer}.useSelection`) || chips > 0;
        const slider = within('[role="slider"]');
        const number = within('input[type="number"], input[data-stepper-input="true"], [role="spinbutton"]');
        const radiogroup = within('[role="radiogroup"]');
        const listbox = within('[role="listbox"]');
        const combobox = within('[role="combobox"], select');
        const toggle = within('[role="switch"], [role="checkbox"], input[type="checkbox"]');
        const text = within('textarea, input[type="text"], input:not([type]), [role="textbox"]');
        const pick = reference ? row : (slider ?? number ?? radiogroup ?? listbox ?? combobox ?? toggle ?? text);
        if (!pick) return;
        const role = reference ? "reference list" : slider ? "slider" : number ? "spinbutton" : radiogroup ? "radiogroup" : listbox ? "listbox" : combobox ? "combobox" : toggle ? "switch" : "textbox";
        const input = pick as HTMLInputElement;
        const value = reference
          ? String(chips)
          : slider
            ? pick.getAttribute("aria-valuenow")
            : number
              ? (input.value ?? pick.getAttribute("aria-valuenow"))
              : radiogroup
                ? (pick.querySelector('[role="radio"][aria-checked="true"]')?.textContent?.trim() ?? null)
                : listbox
                  ? (pick.querySelector('[role="option"][aria-selected="true"]')?.textContent?.trim() ?? null)
                  : combobox
                    ? (pick instanceof HTMLSelectElement ? pick.value : (pick.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60))
                    : toggle
                      ? (pick.getAttribute("aria-checked") ?? String(input.checked))
                      : (input.value ?? pick.textContent ?? null);
        pick.setAttribute("data-probe-u", String(out.length));
        out.push({ index: out.length, pointer, role, name, value, min: pick.getAttribute("min") ?? pick.getAttribute("aria-valuemin"), max: pick.getAttribute("max") ?? pick.getAttribute("aria-valuemax"), step: pick.getAttribute("step") ?? pick.getAttribute("data-step"), options: radiogroup ? pick.querySelectorAll('[role="radio"]').length : listbox ? pick.querySelectorAll('[role="option"]').length : null, disabled: input.disabled === true || pick.getAttribute("aria-disabled") === "true" || pick.getAttribute("aria-readonly") === "true" || input.readOnly === true });
      });
      return out;
    },
    [] as UniversalControl[],
  );

/** 🧑‍🚀 The verbs the open Actions rail lists (`action.<verb>` rows under their `action.category.<name>` groups), in order. */
const universalVerbs = async (): Promise<UniversalVerb[]> => renderer === "wgpu" ? universalMirrorVerbsV1(await mirror()) :
  evalSafe(
    () => {
      const pane = Array.from(document.querySelectorAll<HTMLElement>('[data-slot="window-action-pane"]')).find((el) => el.offsetParent !== null);
      if (!pane) return [];
      let category = "";
      const out: UniversalVerb[] = [];
      for (const el of Array.from(pane.querySelectorAll<HTMLElement>('[id^="action."]'))) {
        if (el.id.startsWith("action.category.")) {
          category = el.id.slice("action.category.".length);
          continue;
        }
        if (!/^action\.[A-Za-z0-9_-]+$/u.test(el.id)) continue;
        const label = ((el.querySelector('[data-slot="tree-label"]') as HTMLElement | null)?.innerText ?? el.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 60);
        out.push({ id: el.id, verb: el.id.slice("action.".length), category, label, disabled: el.getAttribute("aria-disabled") === "true" });
      }
      return out;
    },
    [] as UniversalVerb[],
  );

/** 🛰 Changes one editor control by the smallest step its role offers: ArrowRight on a slider, ArrowUp on a spinbutton, the
 * next radio / option, a click on a switch, one appended character (committed by Tab) in a textbox. Answers what it did. */
const universalChange = async (control: UniversalControl) => {
  const target = renderer === "wgpu" && control.key ? await mirrorLocator(control.key) : page.locator(`[data-probe-u="${control.index}"]`).first();
  if (!target) return "the control left the retained mirror";
  await target.scrollIntoViewIfNeeded({ timeout: 3000 }).catch(() => {});
  if (control.role === "reference list") {
    const chips = Number(control.value ?? "0");
    if (chips > 0) {
      const removed = await pressAuthored(`framework.history.editor.input.${control.pointer}.chip.${chips - 1}`, false);
      return `Remove on the last of ${chips} chip(s): ${removed.present ? "pressed" : "absent"}`;
    }
    return `Use selection (no chip to remove): ${(await pressUseSelection(control.pointer)) ?? "not pressed"}`;
  }
  if (control.role === "slider" || control.role === "spinbutton") {
    const atMax = control.max !== null && control.value !== null && Number(control.value) >= Number(control.max);
    const key = control.role === "slider" ? (atMax ? "ArrowLeft" : "ArrowRight") : atMax ? "ArrowDown" : "ArrowUp";
    await target.focus({ timeout: 3000 }).catch(() => {});
    await page.keyboard.press(key).catch(() => {});
    return key;
  }
  if (control.role === "textbox") {
    await target.focus({ timeout: 3000 }).catch(() => {});
    await page.keyboard.press("End").catch(() => {});
    await page.keyboard.type("x").catch(() => {});
    await page.keyboard.press("Tab").catch(() => {});
    return 'typed "x", committed by Tab';
  }
  if (control.role === "switch") {
    if (renderer === "wgpu" && control.key) return await mirrorActivate(control.key) ? "accessibility activation" : "activation absent";
    await target.click({ timeout: 3000 }).catch(() => {});
    return "click";
  }
  if (renderer === "wgpu" && control.key && ["radiogroup", "listbox", "combobox"].includes(control.role)) {
    if (control.role === "combobox") { await mirrorActivate(control.key); await sleep(500); }
    const option = (await mirror()).find(node => (node.key.startsWith(`${control.key}::`) || node.key.startsWith(`${control.key}.`)) && ["radio", "option"].includes(node.role) && node.checked !== "true" && node.selected !== "true" && !node.disabled);
    if (option) return await mirrorActivate(option.key, option.window) ? "the first other accessible option" : "option activation absent";
    await page.keyboard.press("Escape");
    return "no other accessible option offered";
  }
  if (control.role === "radiogroup") {
    await target.locator('[role="radio"][aria-checked="false"]').first().click({ timeout: 3000 }).catch(() => {});
    return "the first unchecked radio";
  }
  if (control.role === "listbox") {
    await target.locator('[role="option"]:not([aria-selected="true"])').first().click({ timeout: 3000 }).catch(() => {});
    return "the first unselected option";
  }
  if (control.role === "combobox") {
    if ((await target.evaluate((el) => el.tagName).catch(() => "")) === "SELECT") {
      await target.evaluate((el) => {
        const select = el as HTMLSelectElement;
        select.selectedIndex = (select.selectedIndex + 1) % Math.max(1, select.options.length);
        select.dispatchEvent(new Event("change", { bubbles: true }));
      }).catch(() => {});
      return "the next option of the select";
    }
    await target.click({ timeout: 3000 }).catch(() => {});
    const option = page.locator('[role="option"]:not([aria-selected="true"]):not([aria-disabled="true"])').first();
    if ((await waitUntil(() => option.count().catch(() => 0), (count) => count > 0, 3000, 200)).ok) {
      await option.click({ timeout: 3000 }).catch(() => {});
      return "the first other option";
    }
    await page.keyboard.press("Escape").catch(() => {});
    return "no other option offered";
  }
  return "not operated";
};

/** 🌱 Changes one clean history input through its published control and confirms the settled draft. */
const universalChangeOneInput = async () => {
  const attempts: Record<string, unknown>[] = [];
  let changed: string | null = null;
  const clean = (await waitUntil(universalControls, (list) => list.length > 0, 6000, 400)).value;
  for (const control of clean.filter((row) => !row.disabled && row.role !== "reference list").slice(0, 6)) {
    await dismissNotices();
    const how = await universalChange(control);
    const after = await waitUntil(universalControls, (list) => {
      const now = list.find((row) => row.pointer === control.pointer);
      return now !== undefined && now.value !== control.value;
    }, 5000, 300);
    const settled = await settledControl(control.pointer);
    const refusedWith = (await visibleNoticeCodes()).filter((code) => /^(timeTravel|app\.command)\./u.test(code));
    attempts.push({ control: `${control.role} "${control.name}"`, how, before: control.value, after: settled?.value ?? null, ...(refusedWith.length ? { refusedWith } : {}) });
    if (after.ok && settled !== null && settled.value !== control.value && refusedWith.length === 0) {
      changed = `${control.role} "${control.name}": ${control.value} → ${settled.value} (${how})`;
      break;
    }
  }
  return { changed, attempts };
};

/** 🌐 Step 24 — design §23.3, the UNIVERSAL LIVE JOURNEY (batch U): the history-edit journey on whatever editor the serve
 * boots, with framework selectors only (Actions rail, History panel, time-travel band, history editor, finalize prompt).
 * u1 boot + rail verbs + History; u2 TWO mutation rows without editor knowledge (rail actions run with their declared
 * defaults, the second preferably one that acts on what the first left selected; else rows of the booted example); u3 Edit
 * on the OLDER row → band `editing` + the editor's controls by contract role (the inventory, also written to
 * `<out>/<editor>-controls.json`); `u-control-<role>` one control of every inventoried role operated, each on a clean draft;
 * u4 one control changed → the draft is acknowledged, the later row reads "not applied"; u5 Accept → replay (stage + progress
 * when a frame shows it) → review, blockers resolved through Next problem → Withdraw; `u-conflict-…` the blocked → resolved
 * path (taken in u5, else by withdrawing the older row); u6 Finalize → Overwrite, then a second input edit → Finalize → named New alternative;
 * u7 Withdraw → Restore on a row, Exit, zero trace; u8 no faults, labels in the locale, none a raw key. What an editor does not offer is a
 * NOTE named `u<n>-NOT-OFFERED-…` with its evidence — never a PASS. */
const stepU = async (_ctx: Ctx) => {
  const copy = COPY[currentLocale];
  const started = Date.now();
  const shell = await evalSafe((wgpu) => ({ url: location.href, title: document.title, windows: wgpu ? [...new Set(Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-window]")).map(el => el.dataset.window ?? "").filter(Boolean))].slice(0, 8) : Array.from(document.querySelectorAll<HTMLElement>('[data-slot="window"]')).map((el) => el.id).slice(0, 8), canvases: document.querySelectorAll("canvas").length }), { url: "", title: "", windows: [] as string[], canvases: 0 }, renderer === "wgpu");
  const editor = universalEditor ?? (shell.title.toLowerCase().replace(/semio/gu, "").replace(/[^a-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "") || "editor");
  await closePanels();
  const pane = page.locator('[data-slot="window"]').filter({ has: page.locator('[id$=".engagement.toggle"]') }).first();
  const retainedToggle = renderer === "wgpu" ? (await mirror()).find(node => node.key.endsWith(".engagement.toggle"))?.key ?? null : null;
  const hasRail = renderer === "wgpu" ? retainedToggle !== null : (await pane.count().catch(() => 0)) > 0;
  const toggle = pane.locator('[id$=".engagement.toggle"]').first();
  const rail = pane.locator('[data-slot="window-action-pane"]').first();
  const retainedRailVisible = async () => (await mirror()).some(node => /(?:^|[\/␟\u001f])action\.(?:category\.)?[A-Za-z0-9_-]+$/u.test(node.key));
  const openRail = async () => {
    if (renderer === "wgpu") {
      if (retainedToggle && !await retainedRailVisible()) { await wgpuPress(retainedToggle); await waitUntil(retainedRailVisible, Boolean, 8000, 200); }
      return;
    }
    if (!hasRail || (await rail.isVisible().catch(() => false))) return;
    await toggle.click({ timeout: 4000 }).catch(() => {});
    await waitUntil(() => rail.isVisible().catch(() => false), Boolean, 6000, 200);
    await sleep(500);
  };
  const closeRail = async () => {
    if (renderer === "wgpu") { if (retainedToggle && await retainedRailVisible()) await wgpuPress(retainedToggle); return; }
    if (hasRail && (await rail.isVisible().catch(() => false))) await toggle.click({ timeout: 4000 }).catch(() => {});
  };
  const stableDocumentLabels = async () => {
    let last = "";
    const read = await waitUntil(async () => documentEntries(await allHistoryRows()).map((row) => row.label), (labels) => {
      const key = JSON.stringify(labels);
      const same = key === last;
      last = key;
      return same;
    }, 12000, 700);
    return read.value;
  };
  const isHistoryEdit = (row: HistoryRow) => /History edit|Verlauf bearbeitet|Verlaufsbearbeitung/u.test(row.label);
  const settledControl = async (pointer: string) => {
    await sleep(1200);
    let last: string | null | undefined;
    const read = await waitUntil(universalControls, (list) => {
      const now = list.find((row) => row.pointer === pointer);
      const same = now !== undefined && now.value === last;
      last = now?.value;
      return same;
    }, 6000, 350);
    return read.value.find((row) => row.pointer === pointer) ?? null;
  };
  const visibleNoticeCodes = async () => renderer === "wgpu" ? (await mirror()).filter(node => node.key === "shell.notice").map(node => node.description) : evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>("[data-notice-code]")).filter((el) => el.offsetParent !== null).map((el) => el.getAttribute("data-notice-code") ?? ""), [] as string[]);
  const visibleNoticeTexts = async () => renderer === "wgpu" ? (await mirror()).filter(node => node.key === "shell.notice").map(node => node.label) : evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>("[data-notice-code]")).filter((el) => el.offsetParent !== null).map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120)), [] as string[]);
  const dismissNotices = async () => {
    if (renderer === "wgpu") { const dismiss = (await mirror()).find(node => /^shell\.notice\.(?:dismiss|close)$/u.test(node.key)); if (dismiss) await mirrorActivate(dismiss.key); return; }
    await evalSafe(() => document.querySelectorAll<HTMLElement>("[data-notice-code] button").forEach((button) => button.click()), undefined);
    await sleep(250);
  };
  const reviewed = () => waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 90000, 100);
  const resolveBlockers = async (first: Band | null, path: string[]) => {
    let now = first;
    for (let round = 0; round < 4 && now?.review === "blocked"; round++) {
      const next = await pressBand("nextProblem");
      const reopened = await waitUntil(band, (state) => state?.stage === "editing", 8000, 100);
      const blockerText = reopened.value?.target ?? reopened.value?.text ?? "";
      const blocker = (await allHistoryRows()).find((row) => row.kind === "mutation" && row.label.length > 3 && blockerText.includes(row.label)) ?? null;
      const pulled = await pressAuthored("framework.history.editor.withdraw");
      let how = pulled.present ? "the editor's Withdraw" : "";
      if (!pulled.present) how = `the blocker's row action Withdraw: ${blocker ? await pressRowAction(blocker.id, copy.withdraw) : "absent"}`;
      await sleep(900);
      if ((await band())?.stage === "editing") await pressBand("accept");
      now = (await reviewed()).value;
      path.push(`${copy === COPY.de ? "Nächstes Problem" : "Next problem"} (${next}) → ${reopened.ok ? `the editor opens on "${(blocker?.label ?? blockerText).slice(0, 60)}"` : "no editor"} → ${how} → ${now?.review ?? "no review"}`);
    }
    return now;
  };
  await openRail();
  const verbs = await universalVerbs();
  await closeRail();
  const historyOpened = await openHistory();
  const commandsLabel = await textOfKey("framework.history.commands");
  const bootRows = await stableDocumentLabels();
  verdict("u1-boots-and-lists-its-actions-and-its-history", historyOpened && shell.windows.length > 0, { ...shell, editor, railOffered: hasRail, railVerbs: verbs.length, verbs: verbs.slice(0, 48).map((row) => `${row.category || "-"}/${row.verb}=${row.label}${row.disabled ? " [disabled]" : ""}`), historyRowsAtBoot: bootRows.slice(0, 8), commandsSection: commandsLabel.slice(0, 40), reading: "the editor boots into a shell with at least one window; its Actions rail lists the verbs the manifest declares and the History panel opens" });
  let seq = Math.max(-1, newestEntrySeq(await allHistoryRows()));
  const seqAtBoot = seq;
  const skipCategory = /^(transfer|file|view|selection|navigation|clipboard|help|window|history)$/iu;
  const skipVerb = /^(select|deselect|clear|copy|cut|paste|delete|remove|import|export|open|close|load|save|undo|redo|focus|zoom|fit|toggle|show|hide|reset|commit|checkout|switch|createAlternative|set(Active)?(Example|Document|Fixture|Camera))/iu;
  const rank = (category: string) => (/^create$/iu.test(category) ? 0 : /^transform$/iu.test(category) ? 1 : 2);
  const candidates = verbs
    .map((row, index) => ({ ...row, index }))
    .filter((row) => !row.disabled && !skipCategory.test(row.category) && !skipVerb.test(row.verb))
    .sort((left, right) => rank(left.category) - rank(right.category) || left.index - right.index)
    .slice(0, 14);
  type Candidate = (typeof candidates)[number];
  const tried: Record<string, unknown>[] = [];
  const runVerb = async (candidate: Candidate, phase: string) => {
    await openRail();
    if (renderer === "wgpu") {
      const item = mirrorFind(await mirror(), candidate.id);
      if (!item) { tried.push({ phase, verb: candidate.verb, outcome: "its row left the rail" }); return null; }
      await mirrorActivate(item.key, item.window);
      const execute = await waitUntil(async () => (await mirror()).find(node => node.key.endsWith(`.action.${candidate.verb}.execute`)) ?? null, node => node !== null, 4000, 200);
      const executeDisabled = execute.value?.disabled ?? false;
      if (execute.ok && execute.value && !executeDisabled) await mirrorActivate(execute.value.key, execute.value.window);
      const made = executeDisabled ? null : await documentRowsSince(seq, 1, execute.ok ? 8000 : 5000);
      if ((await mirror()).some(node => node.role === "dialog")) { await page.keyboard.press("Escape"); await sleep(400); }
      const row = made?.documents[0] ?? null;
      tried.push({ phase, verb: candidate.verb, label: candidate.label, category: candidate.category, form: execute.ok, executeDisabled, row: row ? `${row.label} [${row.count} mutation(s)]` : null });
      if (made) seq = made.newest;
      if (!row && execute.ok) await mirrorActivate(item.key, item.window);
      return row ? { id: row.id, label: row.label, form: execute.ok } : null;
    }
    const item = pane.locator(`[id="${candidate.id}"]`).first();
    if (!(await item.count().catch(() => 0))) {
      tried.push({ phase, verb: candidate.verb, outcome: "its row left the rail" });
      return null;
    }
    const label = item.locator('[data-slot="tree-label"]').first();
    const press = (await label.count().catch(() => 0)) ? label : item;
    await item.scrollIntoViewIfNeeded({ timeout: 2000 }).catch(() => {});
    const execute = pane.locator(`[id$=".action.${candidate.verb}.execute"]`).first();
    if (!(await execute.count().catch(() => 0))) await press.click({ timeout: 3000 }).catch(() => {});
    const form = (await waitUntil(() => execute.count().catch(() => 0), (count) => count > 0, 1800, 200)).ok;
    let executeDisabled: boolean | null = null;
    let staged: string[] = [];
    if (form) {
      staged = await evalSafe((verb) => Array.from(document.querySelectorAll<HTMLElement>(`[id^="action.${verb}.arg."]`)).filter((el) => !el.id.endsWith(".disclosureLabel")).map((el) => `${el.id.replace(/^.*\.arg\./u, "")}=${(el.querySelector("input") as HTMLInputElement | null)?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? (el.querySelector('[role="combobox"]')?.textContent ?? "").trim()}`.slice(0, 48)).slice(0, 8), [] as string[], candidate.verb);
      executeDisabled = (await execute.isDisabled({ timeout: 1500 }).catch(() => false)) || (await execute.getAttribute("aria-disabled").catch(() => null)) === "true";
      if (!executeDisabled) {
        await execute.scrollIntoViewIfNeeded({ timeout: 2000 }).catch(() => {});
        await execute.click({ timeout: 3000 }).catch(() => {});
      }
    }
    const made = executeDisabled ? null : await documentRowsSince(seq, 1, form ? 5000 : 3500);
    if (await page.locator('[role="dialog"]').first().isVisible().catch(() => false)) {
      await page.keyboard.press("Escape").catch(() => {});
      await sleep(400);
    }
    const row = made?.documents[0] ?? null;
    tried.push({ phase, verb: candidate.verb, label: candidate.label, category: candidate.category, form, staged, executeDisabled, row: row ? `${row.label} [${row.count} mutation(s)]` : null });
    if (made) seq = made.newest;
    if (!row && form) {
      await openRail();
      await press.click({ timeout: 2000 }).catch(() => {});
    }
    return row ? { id: row.id, label: row.label, form } : null;
  };
  let producer: Candidate | null = null;
  let first: { id: string; label: string; form: boolean } | null = null;
  for (const candidate of candidates) {
    if (first !== null || Date.now() - started > 150000 || invalidatedBy !== null) break;
    first = await runVerb(candidate, "first row");
    if (first) producer = candidate;
  }
  let second: { id: string; label: string; form: boolean } | null = null;
  let secondHow = "";
  if (first && producer) {
    const others = candidates.filter((row) => row.id !== producer!.id).sort((left, right) => (rank(left.category) === 1 ? 0 : 1) - (rank(right.category) === 1 ? 0 : 1) || left.index - right.index);
    for (const candidate of others.slice(0, 7)) {
      if (second !== null || Date.now() - started > 230000 || invalidatedBy !== null) break;
      second = await runVerb(candidate, "second row, acting on what the first left selected");
      if (second) secondHow = `the rail action "${candidate.label}" (${candidate.verb}) run after the first row — it acts on what the first action left selected`;
    }
    const selectAll = verbs.find((row) => row.verb === "selectAll" && !row.disabled) ?? null;
    if (!second && selectAll && invalidatedBy === null) {
      await openRail();
      if (renderer === "wgpu") await wgpuPress(selectAll.id);
      else {
        const item = pane.locator(`[id="${selectAll.id}"]`).first();
        const label = item.locator('[data-slot="tree-label"]').first();
        await ((await label.count().catch(() => 0)) ? label : item).click({ timeout: 3000 }).catch(() => {});
      }
      await sleep(1200);
      for (const candidate of others.filter((row) => rank(row.category) === 1).slice(0, 3)) {
        if (second !== null || Date.now() - started > 260000) break;
        second = await runVerb(candidate, "second row, after Select All");
        if (second) secondHow = `the rail action "${candidate.label}" (${candidate.verb}) run after Select All`;
      }
    }
    if (!second && invalidatedBy === null) {
      second = await runVerb(producer, "second row, the same action again");
      if (second) secondHow = `the rail action "${producer.label}" once more (a row that does not depend on the first)`;
    }
  }
  await closeRail();
  await openHistory();
  const entriesNow = documentEntries(await allHistoryRows()).sort((left, right) => Number(right.key) - Number(left.key));
  const route = first && producer ? `the rail action "${producer.label}" (${producer.verb}) ${first.form ? "executed with its staged defaults" : "run without a form"}` : entriesNow.length > 0 ? "History rows the editor booted with (no rail action produced a row with its defaults)" : "";
  if (first !== null || entriesNow.length > 0) verdict("u2-two-mutation-rows-without-editor-knowledge", (first !== null && second !== null) || (first === null && entriesNow.length >= 2), { route, firstRow: first?.label ?? entriesNow[1]?.label ?? entriesNow[0]?.label ?? null, secondRow: second?.label ?? (first === null ? (entriesNow[0]?.label ?? null) : null), secondRowBy: secondHow || null, railActionsTried: tried, documentRows: entriesNow.length, reading: "two document rows are made without knowing the editor — rail actions executed with the defaults their forms stage (the second preferably one that acts on what the first left selected), else rows of the booted example; the journey edits the OLDER one so that a later row exists" });
  else note("u2-NOT-OFFERED-no-mutation-row", { railActionsTried: tried, verbs: verbs.length, candidates: candidates.map((row) => row.verb), reading: "no rail action produced a document row with its declared defaults and the editor booted with an empty history: the journey has no row to edit" });
  const older = first ? entriesNow.filter((row) => row.id === first!.id) : entriesNow.filter((row) => !isHistoryEdit(row)).slice(1, 2);
  const inspect = [...older, ...entriesNow.filter((row) => !older.includes(row) && !isHistoryEdit(row))].slice(0, 4);
  const offers: string[] = [];
  let target: { key: string; label: string; entry: number } | null = null;
  for (const entry of inspect) {
    if (target) break;
    const mutations = entry.expandable ? (await expandEntry(entry.id)).mutations.slice(0, 6) : [];
    for (const mutation of mutations) {
      let edit = await rowActionState(mutation.id, copy.edit, false);
      let withdraw = await rowActionState(mutation.id, copy.withdraw, false);
      for (let attempt = 0; attempt < 5 && !edit.present && !withdraw.present; attempt++) {
        await sleep(800);
        const again = (await expandEntry(entry.id)).mutations.find((row) => row.key === mutation.key);
        if (!again) continue;
        edit = await rowActionState(again.id, copy.edit, false);
        withdraw = await rowActionState(again.id, copy.withdraw, false);
      }
      offers.push(`${entry.label.slice(0, 48)} › ${mutation.label.slice(0, 60)}: edit ${!edit.present ? "absent" : edit.disabled ? `disabled: ${edit.reason}` : "offered"}, withdraw ${!withdraw.present ? "absent" : withdraw.disabled ? `disabled: ${withdraw.reason}` : "offered"}`);
      if (edit.present && edit.disabled !== true) {
        target = { key: mutation.key, label: mutation.label, entry: Number(entry.key) };
        break;
      }
    }
  }
  let editingWords = false;
  let inventory: UniversalControl[] = [];
  const operated: Record<string, unknown> = {};
  let conflict: { by: string; path: string[] } | null = null;
  let reviewReady = false;
  if (!target) note("u3-NOT-OFFERED-no-mutation-offers-edit", { offers: offers.slice(0, 12), reading: "no mutation of the inspected rows offers Edit enabled (each is withdraw-only or its inputs are not published): the editor, the draft, the replay and the finalize of an EDIT cannot be driven; u4–u6 are not reached" });
  else {
    await installBandTrace();
    const begun = await beginEditOf(target.key);
    const controlsRead = begun.band?.stage === "editing" ? await waitUntil(universalControls, (list) => list.length > 0, 8000, 400) : null;
    inventory = controlsRead?.value ?? [];
    editingWords = BAND_WORDS.find(([stage]) => stage === "editing")?.[1].test(begun.band?.text ?? "") ?? false;
    const describe = (row: UniversalControl) => `${row.role} "${row.name}" [${row.pointer}] value=${row.value ?? ""}${row.min !== null ? ` min=${row.min}` : ""}${row.max !== null ? ` max=${row.max}` : ""}${row.step !== null ? ` step=${row.step}` : ""}${row.options !== null ? ` options=${row.options}` : ""}${row.disabled ? " [read-only]" : ""}`;
    verdict("u3-edit-opens-an-editor-with-contract-controls", begun.band?.stage === "editing" && inventory.length > 0, { mutation: target.label, via: begun.via, band: begun.band?.text?.slice(0, 140) ?? null, controls: inventory.length, inventory: inventory.map(describe), offers: offers.slice(0, 8), reading: "Edit on a mutation row opens the session (band `editing`) and the history editor with at least one input control of a contract role; the inventory is the per-editor evidence that the leaf's inputs carry UI metadata" });
    let open = begun.band?.stage === "editing" && inventory.length > 0;
    const roles = [...new Set(inventory.filter((row) => !row.disabled).map((row) => row.role))];
    for (const role of open ? roles : []) {
      const name = `u-control-${role.replace(/\s+/gu, "-")}`;
      if (Date.now() - started > 330000 || invalidatedBy !== null) {
        note(`${name}-NOT-REACHED`, { reason: invalidatedBy !== null ? "the run was invalidated" : "the journey's time budget for the control roles was spent" });
        continue;
      }
      const attemptsOfRole: Record<string, unknown>[] = [];
      let acknowledged = false;
      for (let nth = 0; nth < 3 && !acknowledged && open; nth++) {
        const control = (await waitUntil(universalControls, (list) => list.some((row) => row.role === role && !row.disabled), 6000, 400)).value.filter((row) => row.role === role && !row.disabled)[nth];
        if (!control) break;
        await dismissNotices();
        const how = await universalChange(control);
        const after = await waitUntil(universalControls, (list) => {
          const now = list.find((row) => row.pointer === control.pointer);
          return now !== undefined && now.value !== control.value;
        }, 5000, 300);
        const settled = await settledControl(control.pointer);
        const refusedWith = (await visibleNoticeCodes()).filter((code) => /^(timeTravel|app\.command)\./u.test(code));
        const stage = (await band())?.stage ?? null;
        acknowledged = after.ok && settled !== null && settled.value !== control.value && stage === "editing" && refusedWith.length === 0;
        let shownLater: string | null = null;
        if (refusedWith.length) {
          await sleep(1500);
          shownLater = (await universalControls()).find((row) => row.pointer === control.pointer)?.value ?? null;
        }
        attemptsOfRole.push({ control: describe(control), how, before: control.value, shownAfter: settled?.value ?? null, ...(refusedWith.length ? { refusedWith, notice: await visibleNoticeTexts(), shownTwoSecondsAfterTheRefusal: shownLater } : {}), band: stage });
        if (await band()) {
          await pressBand("discard");
          await sleep(900);
        }
        if (await band()) await exitSession();
        open = (await beginEditOf(target.key)).band?.stage === "editing";
      }
      operated[role] = { acknowledged, attempts: attemptsOfRole };
      const refused = attemptsOfRole.some((row) => Array.isArray(row.refusedWith));
      if (attemptsOfRole.length === 0) note(`${name}-NOT-OPERABLE`, { reason: "the role is not in the editor after it was reopened" });
      else if (role === "reference list" && !acknowledged && !refused) note(`${name}-NOT-OPERABLE`, { attempts: attemptsOfRole, reading: "the reference list holds no chip to remove and Use selection took nothing (the editor has no selection of the referenced kind at this point): the role is inventoried, its operation is not judged" });
      else verdict(name, acknowledged, { attempts: attemptsOfRole, reading: "a control of the role is operated by the smallest step the role offers (up to three controls of the role are tried); it counts when the editor republishes it with the new value, the band stays `editing` and the session raises no refusal notice — a value the session refuses (`timeTravel.invalid-input`) is not a draft, whatever the field shows. The draft is discarded afterwards so the next role starts from the recorded inputs" });
      if (!open) {
        note("u-control-loop-stopped-the-session-did-not-reopen", { after: role });
        break;
      }
    }
    if (open) {
      const { changed, attempts } = await universalChangeOneInput();
      const bandDrafted = await band();
      let whileEditing = await allHistoryRows();
      const downstream = whileEditing.filter((row) => row.kind === "entry" && Number(row.key) > target!.entry && documentEntries([row]).length > 0);
      let pending = whileEditing.filter((row) => row.text.includes(copy.pending));
      if (pending.length === 0 && downstream.length > 0 && downstream[downstream.length - 1].expandable) {
        await expandEntry(downstream[downstream.length - 1].id);
        whileEditing = await allHistoryRows();
        pending = whileEditing.filter((row) => row.text.includes(copy.pending));
      }
      verdict("u4-changing-one-input-is-acknowledged-as-a-draft", changed !== null && bandDrafted?.stage === "editing" && downstream.length > 0 && pending.length > 0, { changed, attempts, band: bandDrafted?.text?.slice(0, 140) ?? null, laterRows: downstream.map((row) => row.label.slice(0, 60)), rowsReadingNotApplied: pending.map((row) => row.text.slice(0, 90)).slice(0, 4), expected: copy.pending, reading: "one control is changed by one step on a clean draft; the editor republishes it, the band stays `editing`, and the later document row reads \"not applied\" while the draft is open" });
      const viaAccept = await pressBand("accept");
      const firstReviewed = await reviewed();
      const firstReview = firstReviewed.value?.review ?? null;
      const firstOutcome = firstReviewed.value?.outcome ?? null;
      const path: string[] = [`Accept (${viaAccept}) → ${firstReview ?? `no review (band ${firstReviewed.value?.stage ?? "gone"})`}${firstOutcome ? `, worst outcome ${firstOutcome}` : ""}`];
      const resolved = await resolveBlockers(firstReviewed.value, path);
      const trace = (await bandTrace()).map((row) => row.key.split("|"));
      const stages = trace.map((parts) => parts[0]).filter((stage, index, list) => stage && stage !== list[index - 1]);
      const progress = trace.filter((parts) => parts[0] === "replaying").map((parts) => parts[2] ?? "").filter(Boolean);
      reviewReady = resolved?.review === "ready";
      if (firstReview === "blocked" && reviewReady) conflict = { by: "the edit itself (u5)", path };
      verdict("u5-accept-replays-to-a-review-and-blockers-resolve", reviewReady, { path, firstReview, firstOutcome, stagesSeen: stages.slice(0, 14), text: resolved?.text?.slice(0, 180) ?? null, reading: "Accept replays the later mutations and ends in a review; a blocked review is resolved through Next problem → Withdraw of the blocker → Accept until it reads ready; the path taken is recorded" });
      if (stages.includes("replaying")) verdict("u5-the-replay-is-shown-as-a-stage-with-its-progress", progress.length > 0, { stagesSeen: stages.slice(0, 14), progressFrames: [...new Set(progress)].slice(0, 8), reading: "the band reads `replaying` while the later rows are applied again and its progress names done / total" });
      else note("u5-the-replay-finished-within-one-refresh", { stagesSeen: stages.slice(0, 14), laterRows: downstream.length, reading: "no DOM commit showed the band in `replaying`: the replay of the later rows ended before the band rendered a frame for it, so the progress is not judged" });
      if (reviewReady) {
        const seqBefore = newestEntrySeq(await allHistoryRows());
        const viaFinalize = await pressBand("finalize");
        const dialog = await waitUntil(finalizeDialog, (value) => value !== null, 15000);
        const offered = { overwrite: dialog.value?.overwrite?.text ?? null, newAlternative: dialog.value?.submit?.text ?? null, nameField: Boolean(dialog.value?.name), title: dialog.value?.title ?? null };
        if (dialog.ok) await dialogChoose("overwrite");
        const gone = await waitUntil(band, (state) => state === null, 30000);
        const edited = await waitUntil(async () => documentEntries(await allHistoryRows()).find((row) => Number(row.key) > seqBefore && isHistoryEdit(row)) ?? null, (row) => row !== null, 20000, 800);
        verdict("u6-finalize-offers-overwrite-and-new-alternative-and-overwrites", dialog.ok && offered.overwrite !== null && offered.newAlternative !== null && gone.ok && edited.ok, { via: viaFinalize, offered, sessionClosed: gone.ok, row: edited.value?.label ?? null, reading: "Finalize opens the prompt with the destructive Overwrite AND New alternative (with its name field); Overwrite closes the session and History lists the history-edit row" });
      } else note("u6-NOT-REACHED-the-review-is-not-ready", { path, review: resolved?.review ?? null, text: resolved?.text?.slice(0, 180) ?? null });
    } else note("u4-NOT-REACHED-the-editor-did-not-open", { band: begun.band?.stage ?? null, via: begun.via });
  }
  if (await band()) await exitSession();
  await sleep(800);
  if (target && reviewReady) {
    const labelsBeforeAlternative = await stableDocumentLabels();
    const alternativesBefore = await readAlternatives();
    const seqBeforeAlternative = newestEntrySeq(await allHistoryRows());
    const begun = await beginEditOf(target.key);
    const draft = begun.band?.stage === "editing" ? await universalChangeOneInput() : { changed: null, attempts: [] };
    const name = `probe ${editor} ${currentLocale} ${stamp.slice(11, 19)}`;
    const path: string[] = [];
    let ready: Awaited<ReturnType<typeof band>> = null;
    if (draft.changed) {
      const via = await pressBand("accept");
      const review = await reviewed();
      path.push(`Accept (${via}) → ${review.value?.review ?? "no review"}`);
      ready = await resolveBlockers(review.value, path);
    }
    if (ready?.review === "ready") {
      const viaFinalize = await pressBand("finalize");
      const dialog = await waitUntil(finalizeDialog, value => value !== null, 15000);
      const nameValue = dialog.ok ? await dialogFillName(name) : null;
      const submitted = nameValue === name && await dialogChoose("submit");
      const gone = await waitUntil(band, state => state === null, 30000);
      const edited = await waitUntil(async () => documentEntries(await allHistoryRows()).find(row => Number(row.key) > seqBeforeAlternative && isHistoryEdit(row) && row.label.includes(name)) ?? null, row => row !== null, 20000, 800);
      const listed = await waitUntil(readAlternatives, rows => rows.some(row => row.text.includes(name) && isCurrentAlternative(row)) && rows.some(row => !row.text.includes(name)), 15000, 800);
      verdict("u6-second-edit-finalizes-as-a-named-new-alternative", dialog.ok && nameValue === name && submitted && gone.ok && edited.ok && listed.ok, { changed: draft.changed, attempts: draft.attempts, path, via: viaFinalize, name, nameValue, submitted, sessionClosed: gone.ok, row: edited.value?.label ?? null, alternatives: listed.value.map(row => ({ key: row.key, text: row.text, current: isCurrentAlternative(row) })), reading: "a second clean edit is accepted and finalized through New alternative; History records its name, the named alternative is current, and the previous head remains offered" });
      const previous = listed.value.find(row => !row.text.includes(name) && (alternativesBefore.some(before => before.key === row.key && isCurrentAlternative(before)) || alternativesBefore.length === 0));
      const mine = listed.value.find(row => row.text.includes(name));
      if (previous && mine) {
        const viaPrevious = await switchAlternative(previous);
        const restored = await waitUntil(stableDocumentLabels, labels => JSON.stringify(labels) === JSON.stringify(labelsBeforeAlternative), 15000, 800);
        const viaMine = await switchAlternative(mine);
        const current = await waitUntil(readAlternatives, rows => rows.some(row => row.key === mine.key && isCurrentAlternative(row)), 15000, 800);
        verdict("u6-new-alternative-preserves-and-switches-the-previous-head", restored.ok && current.ok, { viaPrevious, viaMine, restoredRows: restored.value.length, expectedRows: labelsBeforeAlternative.length, current: current.value.filter(isCurrentAlternative).map(row => row.text), reading: "switching to the prior head restores its exact history, then switching back selects the new named alternative" });
      }
    } else verdict("u6-second-edit-reaches-review-for-new-alternative", false, { begun: begun.band?.stage ?? null, changed: draft.changed, attempts: draft.attempts, path, review: ready?.review ?? null, reading: "after Overwrite the same mutation must remain editable so its second clean change can finalize as a named alternative" });
    if (await band()) await exitSession();
    await sleep(800);
  }
  if (conflict === null) {
    const journeyRows = documentEntries(await allHistoryRows()).filter((row) => !isHistoryEdit(row) && Number(row.key) > seqAtBoot).sort((left, right) => Number(left.key) - Number(right.key));
    const upstream = journeyRows[0] ?? null;
    const upstreamMutation = upstream?.expandable ? ((await expandEntry(upstream.id)).mutations[0] ?? null) : null;
    const offer = upstreamMutation ? await rowActionState(upstreamMutation.id, copy.withdraw, false) : null;
    if (journeyRows.length < 2 || !upstreamMutation || !offer?.present || offer.disabled === true) note("u-conflict-NOT-OFFERED-no-upstream-row-to-withdraw", { journeyRows: journeyRows.map((row) => row.label.slice(0, 60)), withdraw: offer ? { present: offer.present, disabled: offer.disabled, reason: offer.reason } : null });
    else {
      const labelsBeforeConflict = await stableDocumentLabels();
      const forWithdraw = await findMutationRow(upstreamMutation.key);
      const via = forWithdraw ? await pressRowAction(forWithdraw.id, copy.withdraw) : "absent";
      let after = await waitUntil(band, (state) => state !== null && (state.stage === "editing" || (state.stage === "reviewing" && Boolean(state.review))), 20000, 100);
      if (after.value?.stage === "editing") {
        await pressBand("accept");
        after = await reviewed();
      }
      const path: string[] = [`Withdraw on "${upstreamMutation.label.slice(0, 60)}" (${via}) → ${after.value?.review ?? "no review"}${after.value?.outcome ? `, worst outcome ${after.value.outcome}` : ""}`];
      const resolved = after.value?.review === "blocked" ? await resolveBlockers(after.value, path) : after.value;
      if (after.value?.review === "blocked") {
        if (resolved?.review === "ready") conflict = { by: "withdrawing the upstream row", path };
        verdict("u-conflict-a-blocked-review-is-resolved-through-next-problem", resolved?.review === "ready", { by: "withdrawing the upstream row", path, text: resolved?.text?.slice(0, 180) ?? null, reading: "withdrawing the older of the two rows leaves the later row without what it depends on: the review blocks, Next problem names the blocker, withdrawing it reaches ready" });
      } else note("u-conflict-NOT-OFFERED-the-later-row-survives-the-withdraw", { path, review: after.value?.review ?? null, outcome: after.value?.outcome ?? null, text: after.value?.text?.slice(0, 180) ?? null, reading: "the later row does not depend on the withdrawn one strongly enough to fail: no blocked review to resolve on this editor's two rows" });
      if (await band()) await exitSession();
      await waitUntil(stableDocumentLabels, (labels) => JSON.stringify(labels) === JSON.stringify(labelsBeforeConflict), 10000, 1000);
    }
  } else verdict("u-conflict-a-blocked-review-is-resolved-through-next-problem", true, { by: conflict.by, path: conflict.path, reading: "the conflict clause was taken in u5: the edit made a row Fatal, the review blocked, Next problem named the blocker, withdrawing it reached ready" });
  const withdrawOffers: string[] = [];
  const withdrawable = async () => {
    const entries = documentEntries(await allHistoryRows()).filter((row) => !isHistoryEdit(row)).sort((left, right) => Number(right.key) - Number(left.key)).slice(0, 4);
    for (const entry of entries) {
      const mutations = entry.expandable ? (await expandEntry(entry.id)).mutations.slice(0, 6) : [];
      for (const mutation of mutations) {
        const offer = await rowActionState(mutation.id, copy.withdraw, false);
        withdrawOffers.push(`${entry.label.slice(0, 40)} › ${mutation.label.slice(0, 50)}: ${!offer.present ? "absent" : offer.disabled ? `disabled: ${offer.reason}` : "offered"}`);
        if (offer.present && offer.disabled !== true) return { key: mutation.key, label: mutation.label };
      }
    }
    return null;
  };
  let pull = await withdrawable();
  let rowMadeForIt: string | null = null;
  if (!pull && producer && invalidatedBy === null) {
    seq = Math.max(seq, newestEntrySeq(await allHistoryRows()));
    const again = await runVerb(producer, "a row for Withdraw → Restore");
    await closeRail();
    await openHistory();
    rowMadeForIt = again?.label ?? null;
    if (again) pull = await withdrawable();
  }
  const labelsBefore = await stableDocumentLabels();
  if (!pull) note("u7-NOT-OFFERED-no-row-offers-withdraw", { withdrawOffers: withdrawOffers.slice(0, 12), rowMadeForIt, documentRows: labelsBefore.length });
  else {
    const forWithdraw = await findMutationRow(pull.key);
    const viaWithdraw = forWithdraw ? await pressRowAction(forWithdraw.id, copy.withdraw) : "absent";
    let withdrawn = await waitUntil(band, (state) => state !== null && (state.stage === "editing" || (state.stage === "reviewing" && Boolean(state.review))), 20000, 100);
    if (withdrawn.value?.stage === "editing") {
      await pressBand("accept");
      withdrawn = await reviewed();
    }
    let forRestore = await findMutationRow(pull.key);
    let restoreOffer = forRestore ? await rowActionState(forRestore.id, copy.restore, false) : null;
    let restoreWaitedMs = 0;
    for (let attempt = 0; attempt < 7 && !restoreOffer?.present; attempt++) {
      await sleep(800);
      restoreWaitedMs += 800;
      forRestore = await findMutationRow(pull.key);
      restoreOffer = forRestore ? await rowActionState(forRestore.id, copy.restore, false) : null;
    }
    const viaRestore = forRestore && restoreOffer?.present ? await pressRowAction(forRestore.id, copy.restore) : "absent";
    await sleep(1500);
    const afterRestore = await band();
    if (await band()) await exitSession();
    await sleep(800);
    const labelsAfter = await stableDocumentLabels();
    verdict("u7-withdraw-then-restore-then-exit-leaves-zero-trace", viaWithdraw === "action" && withdrawn.ok && viaRestore === "action" && (await band()) === null && JSON.stringify(labelsAfter) === JSON.stringify(labelsBefore), { mutation: pull.label, rowMadeForIt, withdraw: { via: viaWithdraw, review: withdrawn.value?.review ?? null }, restore: { offered: restoreOffer ? { present: restoreOffer.present, disabled: restoreOffer.disabled, reason: restoreOffer.reason } : null, offeredAfterMs: restoreWaitedMs, via: viaRestore, bandAfter: afterRestore?.stage ?? null }, rows: [labelsBefore.length, labelsAfter.length], reading: "Withdraw on a mutation row opens a session with that mutation withdrawn; Restore on the same row drops the draft; leaving the session leaves the document rows exactly as they were" });
  }
  const faults = { uncaught: pageErrors.filter((row) => row.locale === currentLocale).length, hard: hardFaults.filter((row) => row.locale === currentLocale).length };
  const rawLabels = [...verbs.filter((row) => row.label === "" || row.label === row.verb).map((row) => `rail verb ${row.verb}: "${row.label}"`), ...inventory.filter((row) => row.name === "" || row.name === row.pointer || row.name === row.pointer.split(".").pop()).map((row) => `control [${row.pointer}]: "${row.name}"`)];
  verdict("u8-no-faults-and-labels-in-the-locale", faults.uncaught === 0 && faults.hard === 0 && rawLabels.length === 0 && commandsLabel.toLowerCase().includes(copy.commands.toLowerCase()) && (target === null || editingWords), { ...faults, commandsSection: commandsLabel.slice(0, 40), expectedCommands: copy.commands, bandSpokeTheLocale: target === null ? "no session was opened" : editingWords, labelsThatAreRawKeys: rawLabels.slice(0, 10), firstErrors: pageErrors.slice(0, 3).map((row) => row.text.slice(0, 160)), reading: "no uncaught page error and no hard guest fault during the journey; the History section and the band speak the locale under test; no rail verb and no editor control is labelled by its raw key or left without a name" });
  const file = join(OUT, `${editor}-controls.json`);
  let record: { locales?: Record<string, unknown> } & Record<string, unknown> = {};
  try {
    record = JSON.parse(readFileSync(file, "utf8")) as typeof record;
  } catch {
    record = {};
  }
  writeFileSync(file, JSON.stringify({ ...record, editor, url: shell.url, title: shell.title, windows: shell.windows, locales: { ...(record.locales ?? {}), [currentLocale]: { at: new Date().toISOString(), route, secondRowBy: secondHow || null, editedMutation: target?.label ?? null, railVerbs: verbs.map((row) => ({ verb: row.verb, category: row.category, label: row.label })), controls: inventory.map((row) => ({ pointer: row.pointer, role: row.role, name: row.name, value: row.value, min: row.min, max: row.max, step: row.step, options: row.options, readOnly: row.disabled })), operated, conflict } } }, null, 1));
  note("u-journey-summary", { editor, url: shell.url, route, secondRowBy: secondHow || null, controls: inventory.length, rolesOperated: Object.keys(operated), reviewReady, conflictBy: conflict?.by ?? null, controlsFile: file, tookMs: Date.now() - started, verbs: verbs.length });
  await closePanels();
};

const STEPS: Record<number, (ctx: Ctx) => Promise<void>> = { 1: step1, 2: step2, 3: step3, 4: step4, 5: step5, 6: step6, 7: step7, 8: step8, 9: step9, 10: step10, 11: step11, 12: step12, 13: step13, 14: step14, 15: step15, 16: step16, 17: step17, 18: step18, 19: step19, 20: step20, 21: step21, 22: step22, 23: step23, 24: stepU };
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
  let booted = await openPage(DESKTOP, "boot", universalRoute !== null ? 30 : 100);
  if (renderer === "wgpu") {
    currentStep = 1;
    if (!wgpuSeedTerminology) {
      const refused = !booted && /terminology authority/u.test(lastBootFault);
      verdict("wgpu-boots-in-a-fresh-browser-profile", booted, { fault: lastBootFault || null, reading: "a fresh browser profile must boot with its explicitly admitted host locale and the native terminology seed; no stored UI preference is required" });
      if (refused) {
        await context.close().catch(() => {});
        wgpuSeedTerminology = true;
        booted = await openPage(DESKTOP, "boot-with-a-stored-terminology", 100);
      }
    }
    if (wgpuSeedTerminology) note("wgpu-terminology-preference-seeded", { stored: 'localStorage["semio.os.config"].preferences["os.config.ui-preferences"] = {version: 1, events: [{mutation: "setTerminology", terminology: "native"}]}', reading: "diagnostic continuation uses an explicitly stored native terminology after the recorded fresh-profile boot failure; it does not grant fresh-profile success" });
    currentStep = 0;
  }
  page.on("framenavigated", (frame) => {
    if (frame !== page.mainFrame()) return;
    navigations += 1;
    const cause = Date.now() - probeReloadAt < 8000 ? "expected (the probe reloaded it)" : navigationCause();
    if (!cause.startsWith("expected")) invalidatedBy ??= cause;
    log(`main frame navigated (#${navigations}) → ${frame.url().slice(0, 120)} — ${cause}`);
  });
  mod = (await page.evaluate(() => navigator.platform).catch(() => "MacIntel")).includes("Mac") ? "Meta" : "Control";
  currentStep = 1;
  if (!verdict("boot", booted, { mod })) {
    const mismatch = consoleRows.filter((row) => row.locale === locale && /plugin\.channel-mismatch|speaks app channel \d+, the host app channel \d+/u.test(row.text)).map((row) => row.text.slice(0, 300));
    if (mismatch.length) verdict("no-plugin-is-refused-for-a-channel-mismatch", false, { lines: mismatch.slice(0, 6), reading: "the serve's plugin component speaks another app channel than its host (S4-BUMP handshake): a stale build — coordinator action: re-describe / re-activate" });
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
  let boardEmpty = false;
  if (renderer === "wgpu" && (await wgpuBoard()) !== null) {
    currentStep = 1;
    const atBoot = await waitUntil(wgpuBoard, (surface) => (surface?.nodes ?? 0) > 0, 15000, 1000);
    verdict("wgpu-loads-the-example-at-boot", atBoot.ok, { nodes: atBoot.value?.nodes ?? null, edges: atBoot.value?.edges ?? null, waitedMs: atBoot.waitedMs, fixture: mirrorFind(await mirror(), "playground.navbar.fixture")?.label ?? null, reading: "the playground route names its example in the navbar and the React shell boots with it loaded (one \"Set Active Example\" row); the wgpu board must hold its nodes too" });
    if (!atBoot.ok) {
      const opened = await wgpuPress("playground.navbar.fixture");
      const option = await waitUntil(async () => (await mirror()).find((node) => /(^|[./])shell\.example\.[\w-]+$/u.test(node.key)) ?? null, (node) => node !== null, 6000, 300);
      const chosen = option.value ? await mirrorActivate(option.value.key, option.value.window) : false;
      const loaded = await waitUntil(wgpuBoard, (surface) => (surface?.nodes ?? 0) > 0, 30000, 1000);
      verdict("wgpu-navbar-example-loads-the-board", loaded.ok, { opened, option: option.value ? `${option.value.key} = ${option.value.label}` : null, chosen, nodes: loaded.value?.nodes ?? null, waitedMs: loaded.waitedMs, reading: "choosing the example in the navbar's example control (`playground.navbar.fixture` → `shell.example.<id>`, the `setActiveExample` action) must fill the board" });
      boardEmpty = !loaded.ok;
      if (boardEmpty) await shot("wgpu-empty-board");
    }
    currentStep = 0;
  }
  const boardBlocked = renderer === "wgpu" && (boardEmpty || (await wgpuBoard()) === null);
  if (boardBlocked && !boardEmpty) verdict("wgpu-board-introspection-present", false, { prerequisite: WGPU_BOARD_CONTRACT, reading: "the wgpu Board2d surface projects no accessibility nodes and no dump carries its positions, camera or selection, so no pointer gesture can be aimed at a node; every board step is blocked until the export exists" });
  let reloaded = false;
  for (const step of ORDER) {
    if (cancelled.aborted) {
      note("cancelled-before-the-step", { step });
      break;
    }
    if (invalidatedBy !== null) {
      note("run-invalidated-before-the-step", { step, by: invalidatedBy, reading: "the page was reloaded under the run by something other than the probe (a served file saved without the serve lock, or the page reloading itself); every verdict from the step that was running on is void and the batch must be run again" });
      break;
    }
    if ((step === 14 || step === 9) && !reloaded && reloadSelected && !boardBlocked && invalidatedBy === null) {
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
      verdict(boardEmpty ? "blocked-by-the-empty-board" : "blocked-by-missing-board-introspection", false, { prerequisite: boardEmpty ? "a board with nodes (wgpu-loads-the-example-at-boot / wgpu-navbar-example-loads-the-board)" : "semioWgpuIntrospection.dumpBoard2d" });
      continue;
    }
    currentStep = step;
    const navigationsBefore = navigations;
    const reloadsBefore = expectedReloads;
    const noticesBefore = (await shownNotices()).length;
    log(`step ${step} begins`);
    try {
      await STEPS[step](ctx);
    } catch (error) {
      verdict("step-threw", false, { error: String(error).slice(0, 400) });
    }
    const stepNotices = (await shownNotices()).slice(noticesBefore);
    if (stepNotices.length) note("notices-first-shown-in-this-step", { notices: stepNotices });
    if (navigations - navigationsBefore > expectedReloads - reloadsBefore) note("page-reloaded-under-the-step", { navigations: navigations - navigationsBefore, reading: "the dev serve reloaded the page (Vite reconnect / supervisor recycle); verdicts after that point measure a fresh document" });
    const results = verdicts.filter((row) => row.locale === locale && row.step === step);
    if (results.some((row) => !row.ok)) {
      const dialog = await finalizeDialog();
      if (!dialog) await openHistory();
      dumpJson("failure", { inventory: await historyInventory(), rows: await readHistory(), commands: await treeWindow("framework.history.commands"), band: await band(), editor: await editor(), dialog, vitals: await vitals(), console: consoleRows.filter((row) => row.locale === locale && row.step === step).slice(-60) });
      await shot("failure");
    }
    await shot("end");
  }
  if (reloadSelected && !reloaded && !boardBlocked && !cancelled.aborted && invalidatedBy === null) {
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
    for (const step of STEP_NUMBERS) {
      const rows = verdicts.filter((row) => row.locale === locale && row.step === step);
      if (!rows.length) continue;
      const failed = rows.filter((row) => !row.ok);
      stepLines.push(`STEP ${locale}/${step} ${failed.length ? "FAIL" : "PASS"} (${rows.length - failed.length}/${rows.length})${failed.length ? ` — ${failed.map((row) => row.name).join(", ")}` : ""}`);
    }
  }
  const passed = verdicts.filter((row) => row.ok).length;
  const summary = `time-travel probe renderer=${renderer} PASS=${passed} FAIL=${verdicts.length - passed} uncaught=${pageErrors.length} hard=${hardFaults.length} locales=${locales.join(",")} chords=${[...chordLocales].join(",")} only=${only ? `${[...only].join(",")}${reloadSelected ? ",reload" : ""}` : "all"}${cancelled.aborted ? " cancelled" : ""}${invalidatedBy !== null ? ` INVALIDATED by ${invalidatedBy.slice(0, 260)}` : ""}`;
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
 * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--universal] [--variant <declared variant>] [--explore]` — reuses the serve answering at `--serve` (default
 * :6012 React, :6112 wgpu) or starts the catalog-admitted universal variant (puzzle 2d for specialized steps) there for the run ({@link ensureDevServe}; stopped
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
      let variant: string | null = null;
      const startServe = async () => {
        const selectedVariant = timeTravelServeVariantV1({ universal: universalRoute !== null, url: routeUrl(), renderer, explicit: flagValue(segments, "--variant") }, playgroundCatalog);
        variant = selectedVariant;
        if (universalRoute !== null) {
          const route = new URL(universalRoute);
          route.searchParams.set("plugin", variant);
          universalRoute = route.href;
        }
        return ensureDevServe({ repoRoot, port: devServePortV1(serveUrl), variant, renderer, signal: controller.signal, logPath: join(OUT, `${base}-serve.log`), beforeSpawn: async () => (await import("../../♻️activation/🏃️execution/🟦️.ts")).activatePlaygroundRuntime(selectedVariant, "dev", renderer, { signal: controller.signal }), onProgress: (_status, line) => console.log(line) });
      };
      const serve = await startServe().catch((error: unknown) => (error instanceof Error ? error : new Error(String(error))));
      if (serve instanceof Error) {
        const reason = serve.message.split("\n")[0]!.slice(0, 200);
        publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({ check: CHECK_ID, status: "blocked", startedAt, measured: { serve: serveUrl, renderer, variant, cancelled: controller.signal.aborted }, summary: { en: `no ${renderer} ${variant ?? "declared playground"} serve at ${serveUrl}: ${reason}`, de: `kein ${renderer}-Server für ${variant ?? "den deklarierten Playground"} unter ${serveUrl}: ${reason}` } }));
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
            measured: { renderer, variant, serve: routeUrl(), locales: locales.join(","), chords: [...chordLocales].join(","), verdicts: run.verdicts, passed: run.passed, failed: run.failed, uncaught: run.uncaught, hard: run.hard, cancelled: controller.signal.aborted },
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

// #region 🧲️Header
/** @emoji 🎡️ semio-tech play brands — one English, native-terminology shell brand per app pane plus the landing introduction. */
// #endregion 🧲️Header

import type { IntroductionDefinition, ShellBrand, ShellLocale, ShellTerminology } from "@semio-tech/framework";
import type { IconName } from "@semio-tech/ui-react";
import { PLAY_GROUPS, PLAY_HOST, PLAY_RUNTIME_PANES } from "./🔨️modules/🧩️runtime/🟦️.ts";
export { PLAY_HOST };

//#region 🏷️PlayShared
/** @emoji 🇬🇧️ Play is English-locked — the single source the landing page's boot-time `initUiLocaleSync` and every pane brand read. */
export const PLAY_LOCALE: ShellLocale = "en";

/** @emoji 🗣️ Play shows every app in its own native vocabulary, never a partner re-skin. */
export const PLAY_TERMINOLOGY: ShellTerminology = "native";

/** @emoji ✒️ The semio emblem. */
export const SEMIO_TECH_PLAY_LOGO_SVG = `<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="semio"><path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117"/><path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"/><g fill="#ff344f" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z"/></g><g fill="#34d1bf" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z"/></g></svg>`;

/** @emoji 🎓️ Landing introduction, shown on the overview only (never inside a pane shell). */
export const SEMIO_TECH_PLAY_INTRODUCTION: IntroductionDefinition = {
  title: "Welcome to semio Play",
  steps: [
    {
      id: "welcome",
      title: "Welcome to semio Play",
      body: `Play puts every semio app into one live grid — ${PLAY_RUNTIME_PANES.length} apps across ${PLAY_GROUPS.length} groups, from CAD and procedural modelling to structural analysis, building standards and notes.\n\nEvery pane runs the real app on its own document, entirely in your browser.`,
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [],
      demonstrations: [],
    },
    {
      id: "navigate",
      title: "Find an app",
      body: "Move the mouse to glide across the grid. Hover a card to reveal its app, click it to open the app full screen, and press Escape or Overview to come back.",
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [],
      demonstrations: [],
    },
    {
      id: "prototype",
      title: "Early prototype",
      body: "Play is under active development. Some apps are still incomplete and show the direction rather than the final state. Reload the page if something stops responding.",
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [],
      demonstrations: [],
    },
  ],
};
//#endregion 🏷️PlayShared

//#region 🎡️PlayPanes
/** @emoji 🎡️ One live pane of the play grid — order here IS grid order (row-major). */
export type PlayPaneSpec = {
  readonly id: string;
  readonly variant: string;
  readonly group: string;
  readonly brand: ShellBrand;
  readonly label: string;
  readonly tagline: string;
  readonly description: string;
  readonly icon: IconName;
  /** @emoji 📚️ The curated example the pane boots on, or `undefined` when its app publishes none. */
  readonly example?: string;
};

/** @emoji 🏷️ The shell brand one pane boots with — English, native terminology, ephemeral so a refresh
 * always starts clean, and the catalog's curated example as the boot default so the grid opens every app
 * on real content instead of `exampleOptions[0]`. Exactly how the demonstrator's brands carry
 * `defaults.exampleId`: a DEFAULT, never a lock, so the pane's own example picker stays usable. */
export function playPaneBrand(variant: string, label: string, exampleId?: string): ShellBrand {
  return { id: `semio-tech-play-${variant}`, windowTitle: `semio · ${label}`, logoSvg: SEMIO_TECH_PLAY_LOGO_SVG, locks: { locale: PLAY_LOCALE, terminology: PLAY_TERMINOLOGY, themeId: "semio" }, ...(exampleId ? { defaults: { exampleId } } : {}), ephemeral: true };
}

export const PLAY_PANES: readonly PlayPaneSpec[] = PLAY_RUNTIME_PANES.map(pane => ({
  id: pane.variant,
  variant: pane.variant,
  group: pane.group,
  brand: playPaneBrand(pane.variant, pane.label, pane.example),
  label: pane.label,
  tagline: pane.tagline,
  description: pane.description,
  icon: pane.icon as IconName,
  example: pane.example,
}));

/** @emoji 📐️ How much wider than tall the grid may get. A landscape strip pans naturally, but past this
 * the overview's card columns get too narrow to read, so squareness wins over one saved cell. */
const PLAY_GRID_MAX_COLUMN_SURPLUS = 2;

/** @emoji 🔢️ The grid `count` panes are laid out in — of every near-square shape that holds them, the one
 * leaving the FEWEST empty cells, ties going to the squarest. Purely a function of the live pane count,
 * so the grid follows the catalog as apps are added; eight panes give the demonstrator's own gapless 4×2. */
export function playGridDimensions(count: number): { readonly columns: number; readonly rows: number } {
  if (!Number.isInteger(count) || count < 1) throw new Error(`Play grid needs at least one pane: ${count}`);
  let best: { readonly columns: number; readonly rows: number } | undefined;
  for (let columns = 1; columns <= count; columns += 1) {
    const rows = Math.ceil(count / columns);
    if (columns < rows || columns - rows > PLAY_GRID_MAX_COLUMN_SURPLUS) continue;
    if (best === undefined) { best = { columns, rows }; continue; }
    const empty = columns * rows - count, bestEmpty = best.columns * best.rows - count;
    if (empty < bestEmpty || (empty === bestEmpty && columns - rows < best.columns - best.rows)) best = { columns, rows };
  }
  if (best === undefined) throw new Error(`Play grid has no shape for ${count} panes`);
  return best;
}

/** @emoji 📍️ The columns row `row` actually holds: every row but the last is full, and a short last row is
 * centred, so the grid never trails off into a ragged block of empty viewports on one side. */
export function playGridRowSpan(row: number, count: number): { readonly first: number; readonly last: number } {
  const { columns, rows } = playGridDimensions(count);
  if (!Number.isInteger(row) || row < 0 || row >= rows) throw new Error(`Play grid row outside the grid: ${row}`);
  const inRow = Math.min(columns, count - row * columns);
  const first = Math.floor((columns - inRow) / 2);
  return { first, last: first + inRow - 1 };
}

/** @emoji 📍️ Zero-based cell of a pane in row-major order over {@link playGridRowSpan}. */
export function playPaneGridCell(paneIndex: number, count: number): { readonly column: number; readonly row: number } {
  const { columns } = playGridDimensions(count);
  if (!Number.isInteger(paneIndex) || paneIndex < 0 || paneIndex >= count) throw new Error(`Play pane index outside the grid: ${paneIndex}`);
  const row = Math.floor(paneIndex / columns);
  return { column: playGridRowSpan(row, count).first + (paneIndex % columns), row };
}

/** @emoji 🧭️ Columns a viewport whose top edge sits `rowOffset` rows into the grid may rest on: the span of
 * the row filling MOST of it. Panning is clamped to this, so most of the screen is always an app — the
 * empty flanks beside a short trailing row are never a viewport of their own. Taking the union of both
 * overlapped rows instead would open the whole width for a one-percent sliver of the row above. */
export function playOccupiedColumnRange(rowOffset: number, count: number): { readonly first: number; readonly last: number } {
  const { rows } = playGridDimensions(count);
  return playGridRowSpan(Math.min(rows - 1, Math.max(0, Math.round(rowOffset))), count);
}
//#endregion 🎡️PlayPanes

//#region ⏱️PlayIdle
/** @emoji ⏱️ Browser timing surface used by the paced pane-boot queue. */
export type PlayIdleScheduler = {
  readonly setTimeout: (callback: () => void, delayMs: number) => number;
  readonly clearTimeout: (handle: number) => void;
  readonly requestIdleCallback?: (callback: () => void, options?: { readonly timeout: number }) => number;
  readonly cancelIdleCallback?: (handle: number) => void;
};

/** @emoji 🐢️ Enforces a minimum delay before yielding the next warm boot to the browser's idle queue. */
export function schedulePlayIdle(callback: () => void, delayMs: number, scheduler: PlayIdleScheduler): () => void {
  let idleHandle: number | null = null;
  const timeoutHandle = scheduler.setTimeout(() => {
    if (scheduler.requestIdleCallback) idleHandle = scheduler.requestIdleCallback(callback, { timeout: 1_000 });
    else callback();
  }, delayMs);
  return () => {
    scheduler.clearTimeout(timeoutHandle);
    if (idleHandle != null) scheduler.cancelIdleCallback?.(idleHandle);
  };
}

/** @emoji 🧮️ Picks the least recently used pristine live panes to suspend so at most `budget` stay live; the focused pane is never chosen. */
export function playPanesOverBudget(liveIdsByRecency: readonly string[], dirtyIds: ReadonlySet<string>, focusedId: string | null, budget: number): readonly string[] {
  const candidates = liveIdsByRecency.filter(id => id !== focusedId && !dirtyIds.has(id));
  const excess = liveIdsByRecency.length - budget;
  return excess > 0 ? candidates.slice(0, excess) : [];
}

/** @emoji 🌡️ One step of the background warm-boot queue: the pane it may start next, or why it stays idle. */
export type PlayWarmBootStep = { readonly kind: "boot"; readonly id: string } | { readonly kind: "hold"; readonly reason: "focused" | "budget" | "complete" };

/** @emoji 🐢️ Warm-boot policy — a pane nobody has touched is started in the background ONLY when
 * (1) no pane is focused full screen (a focused app owns the machine), and (2) the page is still under
 * {@link playPanesOverBudget}'s live budget, so warming never pushes a live pane over the edge and
 * therefore never evicts a pane the user touched. Panes are warmed in grid order and the queue stops for
 * good once the budget is full of warm panes; it resumes only when the user's own navigation frees a slot. */
export function playNextWarmBootPane(paneIds: readonly string[], bootedIds: ReadonlySet<string>, liveCount: number, focusedId: string | null, budget: number): PlayWarmBootStep {
  if (focusedId !== null) return { kind: "hold", reason: "focused" };
  if (liveCount >= budget) return { kind: "hold", reason: "budget" };
  const next = paneIds.find(id => !bootedIds.has(id));
  return next === undefined ? { kind: "hold", reason: "complete" } : { kind: "boot", id: next };
}
//#endregion ⏱️PlayIdle

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️playgrid/🟦️.ts");
  await registerTests1(import.meta.vitest, { playGridDimensions, playGridRowSpan, playOccupiedColumnRange, playPaneGridCell, playNextWarmBootPane, playPanesOverBudget, schedulePlayIdle, PLAY_PANES, PLAY_LOCALE, PLAY_TERMINOLOGY });
}

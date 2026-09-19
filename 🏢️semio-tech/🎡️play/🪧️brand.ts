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
};

/** @emoji 🏷️ The shell brand one pane boots with — English, native terminology, ephemeral so a refresh always starts clean. */
export function playPaneBrand(variant: string, label: string): ShellBrand {
  return { id: `semio-tech-play-${variant}`, windowTitle: `semio · ${label}`, logoSvg: SEMIO_TECH_PLAY_LOGO_SVG, locks: { locale: PLAY_LOCALE, terminology: PLAY_TERMINOLOGY, themeId: "semio" }, ephemeral: true };
}

export const PLAY_PANES: readonly PlayPaneSpec[] = PLAY_RUNTIME_PANES.map(pane => ({
  id: pane.variant,
  variant: pane.variant,
  group: pane.group,
  brand: playPaneBrand(pane.variant, pane.label),
  label: pane.label,
  tagline: pane.tagline,
  description: pane.description,
  icon: pane.icon as IconName,
}));

/** @emoji 🔢️ The most square grid holding `count` panes: columns first, rows as many as needed. */
export function playGridDimensions(count: number): { readonly columns: number; readonly rows: number } {
  if (!Number.isInteger(count) || count < 1) throw new Error(`Play grid needs at least one pane: ${count}`);
  const columns = Math.ceil(Math.sqrt(count));
  return { columns, rows: Math.ceil(count / columns) };
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
//#endregion ⏱️PlayIdle

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️playgrid/🟦️.ts");
  await registerTests1(import.meta.vitest, { playGridDimensions, playPanesOverBudget, schedulePlayIdle, PLAY_PANES, PLAY_LOCALE, PLAY_TERMINOLOGY });
}

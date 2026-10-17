/** 🏷️ Tool run actions and labels: the framework-reserved action ids, chords, descriptors, legality and the EN/DE panel text. TypeScript twin of `🏷️actions-labels/🦀️.rs`. */

import { TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_PROVISIONAL_CAP, TOOL_RUN_REASON_REBASING, TOOL_RUN_REASON_TRACE_TRUNCATED } from "../🟦️.ts";
import { isToolRunTerminal, type ToolRunState } from "../🚦️lifecycle/🟦️.ts";

//#region 🔖️Actions
export const TOOL_RUN_START_ACTION_ID = "toolRunStart";
export const TOOL_RUN_PAUSE_ACTION_ID = "toolRunPause";
export const TOOL_RUN_RESUME_ACTION_ID = "toolRunResume";
export const TOOL_RUN_STEP_ACTION_ID = "toolRunStep";
export const TOOL_RUN_ABORT_ACTION_ID = "toolRunAbort";
export const TOOL_RUN_FINALIZE_ACTION_ID = "toolRunFinalize";
export const TOOL_RUN_DISMISS_ACTION_ID = "toolRunDismiss";
export const TOOL_RUN_ACTION_IDS = [TOOL_RUN_START_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_STEP_ACTION_ID, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_DISMISS_ACTION_ID] as const;
export type ToolRunActionId = (typeof TOOL_RUN_ACTION_IDS)[number];

export const TOOL_RUN_START_CHORD = "mod+enter";
export const TOOL_RUN_PAUSE_RESUME_CHORD = "mod+alt+enter";
export const TOOL_RUN_STEP_CHORD = "mod+alt+arrowright";
export const TOOL_RUN_ABORT_CHORD = "mod+.";
export const TOOL_RUN_FINALIZE_CHORD = "mod+shift+enter";
export const TOOL_RUN_DISMISS_CHORD = "escape";

export const TOOL_RUN_ARG_TOOL_ID = "toolId";
export const TOOL_RUN_ARG_WINDOW_ID = "windowId";
export const TOOL_RUN_ARG_RUN_ID = "runId";
export const TOOL_RUN_ARG_GENERATION = "generation";

export type ToolRunActionDescriptor = { readonly id: ToolRunActionId; readonly chord: string; readonly label: ToolRunLabelKey; readonly args: readonly { readonly name: string; readonly required: boolean }[] };

const TARGETED_ARGS = [
  { name: TOOL_RUN_ARG_RUN_ID, required: true },
  { name: TOOL_RUN_ARG_GENERATION, required: true },
] as const;

/** 🎛️ The seven generic tool run actions (§2.5). */
export const TOOL_RUN_ACTIONS: readonly ToolRunActionDescriptor[] = [
  {
    id: TOOL_RUN_START_ACTION_ID,
    chord: TOOL_RUN_START_CHORD,
    label: "actionStart",
    args: [
      { name: TOOL_RUN_ARG_TOOL_ID, required: true },
      { name: TOOL_RUN_ARG_WINDOW_ID, required: false },
    ],
  },
  { id: TOOL_RUN_PAUSE_ACTION_ID, chord: TOOL_RUN_PAUSE_RESUME_CHORD, label: "actionPause", args: TARGETED_ARGS },
  { id: TOOL_RUN_RESUME_ACTION_ID, chord: TOOL_RUN_PAUSE_RESUME_CHORD, label: "actionResume", args: TARGETED_ARGS },
  { id: TOOL_RUN_STEP_ACTION_ID, chord: TOOL_RUN_STEP_CHORD, label: "actionStep", args: TARGETED_ARGS },
  { id: TOOL_RUN_ABORT_ACTION_ID, chord: TOOL_RUN_ABORT_CHORD, label: "actionAbort", args: TARGETED_ARGS },
  { id: TOOL_RUN_FINALIZE_ACTION_ID, chord: TOOL_RUN_FINALIZE_CHORD, label: "actionFinalize", args: TARGETED_ARGS },
  { id: TOOL_RUN_DISMISS_ACTION_ID, chord: TOOL_RUN_DISMISS_CHORD, label: "actionDismiss", args: [{ name: TOOL_RUN_ARG_RUN_ID, required: true }] },
];

/** 🚦️ Whether an action is enabled for a slot in `state` (`null` = no run). */
export function isToolRunActionLegal(id: ToolRunActionId, state: ToolRunState | null): boolean {
  switch (id) {
    case TOOL_RUN_START_ACTION_ID:
      return state === null || isToolRunTerminal(state);
    case TOOL_RUN_DISMISS_ACTION_ID:
      return state !== null && isToolRunTerminal(state);
    case TOOL_RUN_PAUSE_ACTION_ID:
      return state === "running";
    case TOOL_RUN_RESUME_ACTION_ID:
    case TOOL_RUN_STEP_ACTION_ID:
      return state === "paused";
    case TOOL_RUN_FINALIZE_ACTION_ID:
      return state === "running" || state === "paused" || state === "complete";
    case TOOL_RUN_ABORT_ACTION_ID:
      return state === "starting" || state === "running" || state === "paused" || state === "complete" || state === "finalizing";
  }
}

//#endregion 🔖️Actions

//#region 🔖️Labels
/** 🗣️ Framework-owned EN/DE text of the tool run panel (§2.5), no default locale. */
export const TOOL_RUN_LABELS = {
  stateStarting: { en: "Starting", de: "Wird gestartet" },
  stateRunning: { en: "Running", de: "Läuft" },
  statePaused: { en: "Paused", de: "Pausiert" },
  stateComplete: { en: "Complete, ready to finalize", de: "Fertig, bereit zum Abschließen" },
  stateFinalizing: { en: "Finalizing", de: "Wird abgeschlossen" },
  stateFinalized: { en: "Finalized", de: "Abgeschlossen" },
  stateAborting: { en: "Aborting", de: "Wird abgebrochen" },
  stateAborted: { en: "Aborted, nothing was changed", de: "Abgebrochen, nichts wurde geändert" },
  stateFaulted: { en: "Failed, nothing was changed", de: "Fehlgeschlagen, nichts wurde geändert" },
  actionStart: { en: "Start", de: "Starten" },
  actionPause: { en: "Pause", de: "Pausieren" },
  actionResume: { en: "Resume", de: "Fortsetzen" },
  actionStep: { en: "Step", de: "Einzelschritt" },
  actionAbort: { en: "Abort", de: "Abbrechen" },
  actionFinalize: { en: "Finalize", de: "Abschließen" },
  actionDismiss: { en: "Dismiss", de: "Schließen" },
  finalizeDisabled: { en: "Available once the run has started", de: "Verfügbar, sobald der Lauf gestartet ist" },
  readyToStart: { en: "Ready to start", de: "Bereit zum Starten" },
  rebasingStep: { en: "Artifact changed, re-applying provisional result", de: "Artefakt geändert, vorläufiges Ergebnis wird neu angewendet" },
  conflictStep: { en: "{0} provisional changes conflict with the current artifact", de: "{0} vorläufige Änderungen stehen im Konflikt mit dem aktuellen Artefakt" },
  traceTruncatedStep: { en: "Oldest {0} rejected attempts are no longer shown", de: "Die ältesten {0} verworfenen Versuche werden nicht mehr angezeigt" },
  provisionalCapStep: { en: "Provisional change limit of {0} reached, run completed", de: "Grenze von {0} vorläufigen Änderungen erreicht, Lauf abgeschlossen" },
  progressValueText: { en: "{stage} ({i}/{n}): {completed} of {total} {unit} ({pct} %)", de: "{stage} ({i}/{n}): {completed} von {total} {unit} ({pct} %)" },
} as const satisfies Record<string, { en: string; de: string }>;
export type ToolRunLabelKey = keyof typeof TOOL_RUN_LABELS;

/** 💬️ Status label key of a state. */
export function toolRunStateLabel(state: ToolRunState): ToolRunLabelKey {
  return `state${state[0]!.toUpperCase()}${state.slice(1)}` as ToolRunLabelKey;
}

/** 🚧️ Label key of a framework-reserved reason code. */
export function toolRunReasonLabel(code: number): ToolRunLabelKey | undefined {
  return ({ [TOOL_RUN_REASON_REBASING]: "rebasingStep", [TOOL_RUN_REASON_CONFLICT]: "conflictStep", [TOOL_RUN_REASON_TRACE_TRUNCATED]: "traceTruncatedStep", [TOOL_RUN_REASON_PROVISIONAL_CAP]: "provisionalCapStep" } as Record<number, ToolRunLabelKey>)[code];
}

/** 🔣️ Single-pass `{name}` substitution; unknown or unterminated placeholders stay literal. */
export function toolRunFormat(template: string, value: (name: string) => string | undefined): string {
  let out = "";
  let rest = template;
  for (let open = rest.indexOf("{"); open !== -1; open = rest.indexOf("{")) {
    out += rest.slice(0, open);
    const after = rest.slice(open + 1);
    const close = after.search(/[{}]/);
    const text = close !== -1 && after[close] === "}" ? value(after.slice(0, close)) : undefined;
    if (text === undefined) {
      out += "{";
      rest = after;
    } else {
      out += text;
      rest = after.slice(close + 1);
    }
  }
  return out + rest;
}
//#endregion 🔖️Labels

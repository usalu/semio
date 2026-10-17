/** 🚦️ Tool run lifecycle: the pure transition table, events, effects, admission and the `ToolRunMachine` reducer. TypeScript twin of `🚦️lifecycle/🦀️.rs`. */

//#region 🔖️Lifecycle
export const TOOL_RUN_STATES = ["starting", "running", "paused", "complete", "finalizing", "finalized", "aborting", "aborted", "faulted"] as const;
export type ToolRunState = (typeof TOOL_RUN_STATES)[number];

/** 🏁️ `finalized`, `aborted` and `faulted`. */
export function isToolRunTerminal(state: ToolRunState): boolean {
  return state === "finalized" || state === "aborted" || state === "faulted";
}

export type ToolRunSlot = { readonly run: bigint; readonly generation: number; readonly state: ToolRunState };

export type ToolRunEvent =
  | { readonly type: "start"; readonly run: bigint }
  | { readonly type: "jobAdmitted" | "pause" | "resume" | "step" | "jobComplete" | "jobFault" | "finalize" | "publicationComplete" | "revalidationConflicts" | "storeRejected" | "abortComplete"; readonly run: bigint; readonly generation: number }
  | { readonly type: "abort"; readonly run: bigint; readonly generation: number; readonly publishing: boolean }
  | { readonly type: "settingsChanged" | "baseChanged" | "dismiss"; readonly run: bigint }
  | { readonly type: "closed" };

export const TOOL_RUN_EVENT_KEYS = ["start", "jobAdmitted", "pause", "resume", "step", "jobComplete", "jobFault", "settingsChanged", "baseChanged", "finalize", "publicationComplete", "revalidationConflicts", "storeRejected", "abort", "abortWhilePublishing", "abortComplete", "dismiss", "closed"] as const;
export type ToolRunEventKey = (typeof TOOL_RUN_EVENT_KEYS)[number];

/** 🗝️ Lifecycle-law matrix column of an event. */
export function toolRunEventKey(event: ToolRunEvent): ToolRunEventKey {
  return event.type === "abort" && event.publishing ? "abortWhilePublishing" : event.type;
}

export const TOOL_RUN_EFFECTS = ["spawnJob", "schedule", "stopScheduling", "driveOneUnit", "holdResult", "reconfigure", "refold", "beginFinalize", "releaseProvisional", "retractConflicts", "keepProvisional", "closeJob", "cancelBatch", "retireProvisional", "discardProvisional", "clearTrace", "retireAll"] as const;
export type ToolRunEffect = (typeof TOOL_RUN_EFFECTS)[number];

/** 💾️ True only for the one transition that changes the store generation. */
export function toolRunEffectCommits(effect: ToolRunEffect): boolean {
  return effect === "releaseProvisional";
}

export type ToolRunRejection = "toolRun.stale" | "toolRun.busy" | "toolRun.illegal";
export type ToolRunTransition = { readonly slot: ToolRunSlot | null; readonly effect: ToolRunEffect };
export type ToolRunApplyResult = { readonly ok: true; readonly transition: ToolRunTransition } | { readonly ok: false; readonly rejection: ToolRunRejection };

type ToolRunRow = readonly [ToolRunState | null, ToolRunEffect, boolean];

const TOOL_RUN_TABLE: Readonly<Partial<Record<ToolRunState, Partial<Record<ToolRunEventKey, ToolRunRow | ToolRunRejection>>>>> = {
  starting: { jobAdmitted: ["running", "schedule", false], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false], jobFault: ["faulted", "discardProvisional", false] },
  running: { pause: ["paused", "stopScheduling", false], finalize: ["finalizing", "beginFinalize", false], jobComplete: ["complete", "holdResult", false], settingsChanged: ["running", "reconfigure", true], baseChanged: ["running", "refold", true], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false], jobFault: ["faulted", "discardProvisional", false] },
  paused: { resume: ["running", "schedule", false], step: ["paused", "driveOneUnit", false], finalize: ["finalizing", "beginFinalize", false], jobComplete: ["complete", "holdResult", false], settingsChanged: ["paused", "reconfigure", true], baseChanged: ["paused", "refold", true], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false], jobFault: ["faulted", "discardProvisional", false] },
  complete: { settingsChanged: ["running", "reconfigure", true], baseChanged: ["complete", "refold", true], finalize: ["finalizing", "beginFinalize", false], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false] },
  finalizing: { publicationComplete: ["finalized", "releaseProvisional", false], revalidationConflicts: ["complete", "retractConflicts", true], storeRejected: ["complete", "keepProvisional", true], abort: ["aborting", "cancelBatch", false], abortWhilePublishing: "toolRun.stale" },
  aborting: { abortComplete: ["aborted", "retireProvisional", false] },
  finalized: { dismiss: [null, "clearTrace", false] },
  aborted: { dismiss: [null, "clearTrace", false] },
  faulted: { dismiss: [null, "clearTrace", false] },
};

/** 🛣️ What a run occupies on its document instance for its local actor (§2.2 invariant 5). */
export type ToolRunLane = { readonly mutating: boolean; readonly toolId: string; readonly windowId?: string };

/** 🤝️ Two mutating runs always share a lane; two read-only runs share it on the same tool and window; a mutating and a read-only run never do. */
export function toolRunLanesShare(left: ToolRunLane, right: ToolRunLane): boolean {
  if (left.mutating !== right.mutating) return false;
  return left.mutating || (left.toolId === right.toolId && left.windowId === right.windowId);
}

/** 🚦️ `toolRun.busy` while a run on the same lane is non-terminal, else the indices of the terminal runs the start replaces. */
export function toolRunAdmit(lane: ToolRunLane, live: readonly (readonly [ToolRunLane, ToolRunState])[]): { readonly ok: true; readonly replaces: readonly number[] } | { readonly ok: false; readonly rejection: ToolRunRejection } {
  const shared = live.flatMap(([other], index) => (toolRunLanesShare(other, lane) ? [index] : []));
  if (shared.some((index) => !isToolRunTerminal(live[index]![1]))) return { ok: false, rejection: "toolRun.busy" };
  return { ok: true, replaces: shared };
}

/** ⚙️ The pure §2.2 reducer of one run slot; {@link toolRunAdmit} decides which slot a start may take. */
export const ToolRunMachine = {
  /** ⚖️ Staleness guard first, then the transition table. */
  apply(slot: ToolRunSlot | null, event: ToolRunEvent): ToolRunApplyResult {
    if (event.type === "start") {
      if (slot !== null && !isToolRunTerminal(slot.state)) return { ok: false, rejection: "toolRun.busy" };
      if (slot !== null && event.run <= slot.run) return { ok: false, rejection: "toolRun.illegal" };
      return { ok: true, transition: { slot: { run: event.run, generation: 0, state: "starting" }, effect: "spawnJob" } };
    }
    if (event.type === "closed") return { ok: true, transition: { slot: null, effect: "retireAll" } };
    if (slot === null || event.run !== slot.run || ("generation" in event && event.generation !== slot.generation)) return { ok: false, rejection: "toolRun.stale" };
    const row = TOOL_RUN_TABLE[slot.state]?.[toolRunEventKey(event)];
    if (row === undefined) return { ok: false, rejection: "toolRun.illegal" };
    if (typeof row === "string") return { ok: false, rejection: row };
    const [to, effect, increment] = row;
    return { ok: true, transition: { slot: to === null ? null : { run: slot.run, generation: increment ? (slot.generation + 1) >>> 0 : slot.generation, state: to }, effect } };
  },
};
//#endregion 🔖️Lifecycle

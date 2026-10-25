/** 🪧️ Tool run panel: the key scheme of the framework ToolRun panel and the diff of the runs a rendered panel holds. */

//#region 🔖️Panel
/** ⌨️ Key of the framework ToolRun panel root; each run is one child group {@link toolRunPanelGroupId}. */
export const TOOL_RUN_PANEL_ID = "framework.toolRun";

/** 👥️ The key of run `run`'s group in the ToolRun panel. */
export function toolRunPanelGroupId(run: bigint): string {
  return `${TOOL_RUN_PANEL_ID}.${run}`;
}

/** 🔎️ The run a ToolRun panel group key names; `null` for any other key. */
export function toolRunPanelGroupRun(key: string): bigint | null {
  const digits = key.startsWith(`${TOOL_RUN_PANEL_ID}.`) ? key.slice(TOOL_RUN_PANEL_ID.length + 1) : "";
  return /^(0|[1-9][0-9]*)$/.test(digits) ? BigInt(digits) : null;
}

/** 📣️ The runs a rendered ToolRun panel holds and those of them not in `previous`. */
export function toolRunPanelNewRuns(previous: ReadonlySet<bigint>, keys: Iterable<string>): { readonly current: ReadonlySet<bigint>; readonly added: readonly bigint[] } {
  const current = new Set<bigint>();
  for (const key of keys) {
    const run = toolRunPanelGroupRun(key);
    if (run !== null) current.add(run);
  }
  return { current, added: [...current].filter((run) => !previous.has(run)).sort((left, right) => (left < right ? -1 : left > right ? 1 : 0)) };
}
//#endregion 🔖️Panel

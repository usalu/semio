// #region 🧲️Header
/** @emoji ⏯️ The shell side of the framework ToolRun panel (`framework.panel.toolRun`, body `framework.body.toolRun`):
 * which runs a rendered panel body holds, whether a run the shell has not seen yet started — the moment the shell
 * reveals the panel — and each live run as the Tasks window lists it. The group-key law is `⏯️tool-run`'s own
 * (`toolRunPanelNewRuns`), pinned by its lifecycle fixture; the wgpu Shell reads the same law in Rust. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { ActionBinding, BuiltNode } from "@semio-tech/framework";
import { TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, toolRunPanelGroupRun, toolRunPanelNewRuns } from "../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";
// #endregion 🔌️Adapters

//#region ⏯️ToolRunPanel
/** 📣️ The runs the rendered ToolRun panel `body` holds, and the ones of them not in `seen` — a non-empty `added` asks the
 * shell to reveal the panel. A panel that has not rendered yet holds no runs. */
export function toolRunPanelReveal(seen: ReadonlySet<bigint>, body: BuiltNode | undefined): { readonly runs: ReadonlySet<bigint>; readonly added: readonly bigint[] } {
  const { current, added } = toolRunPanelNewRuns(seen, (body?.children ?? []).map((group) => group.key));
  return { runs: current, added };
}

/** ⏯️ One of a run's own panel controls, re-offered elsewhere: the binding its button fires and whether the run's state
 * allows it right now. */
export interface ToolRunPanelControlV1 {
  readonly binding: ActionBinding;
  readonly disabled: boolean;
}

/** ⏯️ One LIVE run of a ToolRun panel body — running or paused, never a terminal one — as the Tasks window lists it: the
 * tool's label, the run's status line, its progress and the run's own Pause / Resume / Abort controls (`null` when the
 * panel does not offer that verb in the run's state). */
export interface ToolRunPanelTaskV1 {
  readonly run: bigint;
  readonly label: string;
  readonly status: string;
  readonly completed: number;
  readonly total: number | null;
  readonly valueText: string;
  readonly pause: ToolRunPanelControlV1 | null;
  readonly resume: ToolRunPanelControlV1 | null;
  readonly abort: ToolRunPanelControlV1 | null;
}

function toolRunPanelNode(group: BuiltNode, key: string): BuiltNode | undefined {
  const pending = [...group.children];
  for (let node = pending.shift(); node; node = pending.shift()) {
    if (node.key === key) return node;
    pending.push(...node.children);
  }
  return undefined;
}

function toolRunPanelControl(group: BuiltNode, action: string): ToolRunPanelControlV1 | null {
  const button = toolRunPanelNode(group, `${group.key}.${action}`);
  const binding = button?.bindings.find((candidate) => candidate.trigger === "activate");
  return button && binding ? { binding, disabled: button.disabled } : null;
}

/** 🏃️ The live runs a rendered ToolRun panel `body` holds, oldest first, read from the SAME body the ToolRun panel
 * renders — the run's own status, progress and controls — so the Tasks window can only ever offer what the run's owner
 * offers. A run whose panel group offers neither Pause nor Resume is terminal (Start/Dismiss only) and is not a task. */
export function toolRunPanelTasksV1(body: BuiltNode | undefined): readonly ToolRunPanelTaskV1[] {
  return (body?.children ?? []).flatMap((group): ToolRunPanelTaskV1[] => {
    const run = toolRunPanelGroupRun(group.key);
    if (run === null) return [];
    const pause = toolRunPanelControl(group, TOOL_RUN_PAUSE_ACTION_ID);
    const resume = toolRunPanelControl(group, TOOL_RUN_RESUME_ACTION_ID);
    if (pause === null && resume === null) return [];
    const status = toolRunPanelNode(group, `${group.key}.status`);
    const progress = toolRunPanelNode(group, `${group.key}.progress`);
    const label = group.accessibility.label ?? "";
    return [{
      run,
      label,
      status: status?.component.type === "text" ? status.component.value : "",
      completed: progress?.component.type === "progress" ? progress.component.completed : 0,
      total: progress?.component.type === "progress" ? (progress.component.total ?? null) : null,
      valueText: progress?.component.type === "progress" ? progress.component.valueText : "",
      pause,
      resume,
      abort: toolRunPanelControl(group, TOOL_RUN_ABORT_ACTION_ID),
    }];
  });
}
//#endregion ⏯️ToolRunPanel

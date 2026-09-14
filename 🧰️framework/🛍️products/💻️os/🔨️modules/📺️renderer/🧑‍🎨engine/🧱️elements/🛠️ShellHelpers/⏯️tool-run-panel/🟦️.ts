// #region 🧲️Header
/** @emoji ⏯️ The shell side of the framework ToolRun panel (`framework.panel.toolRun`, body `framework.body.toolRun`):
 * which runs a rendered panel body holds, and whether a run the shell has not seen yet started — the moment the shell
 * reveals the panel. The group-key law is `⏯️tool-run`'s own (`toolRunPanelNewRuns`), pinned by its lifecycle fixture;
 * the wgpu Shell reads the same law in Rust. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { BuiltNode } from "@semio-tech/framework";
import { toolRunPanelNewRuns } from "../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";
// #endregion 🔌️Adapters

//#region ⏯️ToolRunPanel
/** 📣️ The runs the rendered ToolRun panel `body` holds, and the ones of them not in `seen` — a non-empty `added` asks the
 * shell to reveal the panel. A panel that has not rendered yet holds no runs. */
export function toolRunPanelReveal(seen: ReadonlySet<bigint>, body: BuiltNode | undefined): { readonly runs: ReadonlySet<bigint>; readonly added: readonly bigint[] } {
  const { current, added } = toolRunPanelNewRuns(seen, (body?.children ?? []).map((group) => group.key));
  return { runs: current, added };
}
//#endregion ⏯️ToolRunPanel

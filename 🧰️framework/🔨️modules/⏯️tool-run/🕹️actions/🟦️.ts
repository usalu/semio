/** ⏯️ TypeScript twin of `semio_framework_tool_run_actions` (`🦀️.rs` beside this file): the run vocabulary a tool or utility
 * declares and the predicate that reserves the run actions. Exported through `@semio-tech/framework`. */

export { TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ABORT_CHORD, TOOL_RUN_ACTION_IDS, TOOL_RUN_ACTIONS, TOOL_RUN_DISMISS_ACTION_ID, TOOL_RUN_DISMISS_CHORD, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_FINALIZE_CHORD, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_PAUSE_RESUME_CHORD, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_START_ACTION_ID, TOOL_RUN_START_CHORD, TOOL_RUN_STEP_ACTION_ID, TOOL_RUN_STEP_CHORD, type ToolRunActionId } from "../🏷️actions-labels/🟦️.ts";

/** 🛠️ Mirrors Rust `app_declares_tool_run`: the seven reserved tool run actions are injected exactly when a
 * tool or utility declares `run`. */
export function appDeclaresToolRun(app: { readonly tools: readonly { readonly run?: unknown }[]; readonly utilities: readonly { readonly run?: unknown }[] }): boolean {
  return app.tools.some((tool) => tool.run !== undefined) || app.utilities.some((utility) => utility.run !== undefined);
}

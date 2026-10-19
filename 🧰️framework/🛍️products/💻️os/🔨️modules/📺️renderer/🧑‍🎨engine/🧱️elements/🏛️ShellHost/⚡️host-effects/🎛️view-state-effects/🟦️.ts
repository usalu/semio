/** 🎛️ The host effects that change the shell view state: active utility, active tool and dialog opening.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useApplyHostEffects}: each function is one effect arm of the `applyHostEffects` pass, moved verbatim. */
import { shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type Effect } from "@semio-tech/framework";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

/** 🧰️ Mirrors a program's utility switch into the host registers and the follow-up view state. */
export function applySetActiveUtilityEffect({ writeUtilityRegister, activeToolIdRef, writeToolRegister, activeWindowIdRef, effect, nextViewState }: Pick<ShellCoreAfterSpawnedSession, "writeUtilityRegister" | "activeToolIdRef" | "writeToolRegister" | "activeWindowIdRef"> & {
  readonly effect: Extract<Effect, Readonly<Record<"setActiveUtility", unknown>>>;
  readonly nextViewState: ActiveSession["viewState"];
}) {
  const { windowId, utilityId } = effect.setActiveUtility;
  writeUtilityRegister(windowId, utilityId || null, null);
  if (utilityId && activeToolIdRef.current) writeToolRegister(null, null);
  if (windowId === activeWindowIdRef.current) nextViewState = { ...nextViewState, activeUtilityId: utilityId || undefined, activeToolId: utilityId ? undefined : nextViewState.activeToolId };
  return nextViewState;
}

/** 🛠️ Mirrors a program's tool switch into the host registers and the follow-up view state. */
export function applySetActiveToolEffect({ writeToolRegister, clearAllWindowUtilities, effect, nextViewState }: Pick<ShellCoreAfterSpawnedSession, "writeToolRegister" | "clearAllWindowUtilities"> & {
  readonly effect: Extract<Effect, Readonly<Record<"setActiveTool", unknown>>>;
  readonly nextViewState: ActiveSession["viewState"];
}) {
  const { toolId } = effect.setActiveTool;
  writeToolRegister(toolId || null, null);
  if (toolId) clearAllWindowUtilities();
  nextViewState = { ...nextViewState, activeToolId: toolId || undefined, activeUtilityId: toolId ? undefined : nextViewState.activeUtilityId };
  return nextViewState;
}

/** 💬️ Opens an owned dialog when the effect's session is still the shown one. */
export function applyOpenDialogEffect({ shellStateRef, makeOwnedDialog, liveDialogRef, dispatch, effect, baseSession, effectOrigin }: Pick<ShellCoreAfterSpawnedSession, "shellStateRef" | "makeOwnedDialog" | "liveDialogRef" | "dispatch"> & {
  readonly effect: Extract<Effect, Readonly<Record<"openDialog", unknown>>>;
  readonly baseSession: ActiveSession;
  readonly effectOrigin: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>["presentation"];
}) {
  const { dialogId, args } = effect.openDialog;
  if (shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session)) {
    const value = makeOwnedDialog(dialogId, effectOrigin, args as Record<string, unknown> | undefined);
    if (value !== null) {
      liveDialogRef.current = value;
      dispatch({ type: "SET_DIALOG", value });
    }
  }
}

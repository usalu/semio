/** 🚀️ Shell plumbing, effect dispatch: the host effect dispatcher that encodes action and command invocations. */
import { type Effect, type HostEffectDispatchScope, hostEffectInvocationV1, resolveUiDirtyScope, type UiDirtyScope } from "@semio-tech/framework";
import { causalOrderKeyV1, type InputProvenanceV1 } from "../../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import { type ActiveSession, type LoadedProgramState } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";

//#region ShellHelpers
/** 🔁️ The one-action-at-a-time callback shared by the `requestFileOpen`/`dispatchAction`/
 * `requestMediaFrames` `applyHostEffects` branches: dispatches `action` against the emitting program
 * instance and feeds its own `requestedEffects` back through `applyHostEffects` recursively. */
export type EffectDispatchOne = (action: string, args?: Record<string, unknown>) => Promise<void>;

/** 🎯️ Encodes a host-effect callback as the same fully scoped JSON `ActionInvocation` every other
 * renderer action uses; pack-base64 belongs inside the worker channel, never at `handleAction`'s edge. */
export function encodeEffectActionInvocation(baseSession: ActiveSession, action: string, args?: Record<string, unknown>): string {
  return JSON.stringify(hostEffectInvocationV1(effectDispatchScope(baseSession), [], action, args).invocation);
}

/** 🎯️ Encodes a recursively requested app command with its manifest owner. */
export function encodeEffectCommandInvocation(baseSession: ActiveSession, commandId: string, args?: Record<string, unknown>): string {
  return JSON.stringify(hostEffectInvocationV1(effectDispatchScope(baseSession), [commandId], commandId, args).invocation);
}

/** 🎯️ The `(plugin, app, mode, window kind, window instance)` a host effect re-dispatches in, read off
 * one live session — the shell's half of {@link hostEffectInvocationV1}. */
function effectDispatchScope(baseSession: ActiveSession): HostEffectDispatchScope {
  const windowKindId = baseSession.viewState.activeWindowKindId ?? baseSession.app.windowKinds[0]?.id ?? "";
  return {
    pluginId: baseSession.pluginId,
    appId: baseSession.app.id,
    modeId: baseSession.viewState.activeModeId ?? baseSession.app.defaultModeId ?? baseSession.app.modes[0]?.id ?? baseSession.app.id,
    windowKindId,
    windowInstanceId: baseSession.viewState.windowId ?? windowKindId,
  };
}

/** 🔁️ Builds an {@link EffectDispatchOne} bound to one plugin instance + `applyHostEffects` closure;
 * declared app commands re-enter the typed command channel and framework/window actions retain their
 * scoped action channel.
 *
 * 🗣️ `resolveViewState` is REQUIRED, and is the reason this takes a resolver instead of reading
 * `baseSession.viewState`: an `ActiveSession`'s stored view state is the shell's own partial
 * projection (`{ activeModeId }` at session establishment) and carries no `locale`/`terminology`,
 * while the guest's `ViewModel` declares both NON-optional. A recursively requested effect
 * (`dispatchAction`, `requestFileOpen`'s import, `requestMediaFrames`) dispatched with that raw
 * projection faulted in the guest with the anonymous `missing field \`locale\`` — and a
 * `scheduleDispatchAction` one is `void`-dispatched, so the fault surfaced only as an unhandled
 * `SemioFaultError` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, the deferred `flowEvalTick` re-arm).
 * Host owners pass `resolvedTargetViewState`, the single admission that stamps both preferences.
 *
 * 🔗️ `provenance` is the ledger provenance of the input whose guest answer armed this follow-up
 * (`🎯️input-ledger` L2): its `causedBy` becomes the actor queue's causal `order` key, so a `dispatchAction`
 * re-arm dequeues BEFORE any input the user issued after its cause. Absent (`null` cause) the follow-up
 * is a root in its own right and carries no order. */
export function makeEffectDispatchOne(
  pluginEntry: LoadedProgramState,
  baseSession: ActiveSession,
  applyEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope?: UiDirtyScope) => Promise<void>,
  isCurrent: () => boolean,
  resolveViewState: (session: ActiveSession) => ViewModel,
  provenance?: Pick<InputProvenanceV1, "causedBy" | "windowId">,
): EffectDispatchOne {
  // 🔗️ A follow-up inherits its cause's key: `causalOrderKeyV1` over `{ inputSeq: cause, causedBy: cause }` IS the cause.
  const dispatch = provenance !== undefined && provenance.causedBy !== null ? { order: causalOrderKeyV1({ inputSeq: provenance.causedBy, causedBy: provenance.causedBy }) } : undefined;
  return async (action, args) => {
    if (!isCurrent()) return;
    const viewState = resolveViewState(baseSession);
    const isAppCommand = (baseSession.app.commands ?? []).some((command) => command.id === action);
    const response = isAppCommand && pluginEntry.handle.handleCommand
      ? await pluginEntry.handle.handleCommand(baseSession.instanceId, encodeEffectCommandInvocation(baseSession, action, args), viewState, dispatch)
      : await pluginEntry.handle.handleAction(baseSession.instanceId, encodeEffectActionInvocation(baseSession, action, args), viewState, dispatch);
    if (isCurrent()) await applyEffects(response.requestedEffects ?? [], { ...baseSession, viewState }, resolveUiDirtyScope(response.uiScope));
  };
}
//#endregion ShellHelpers

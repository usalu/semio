import { parseResolvedPluginViewState, type PluginViewState, type ResolvedPluginViewState } from "@semio-tech/framework";
import { guestWindowIdV1, spawnedIdOfWindowInstanceV1 } from "../🪟️spawned-program/🟦️.ts";
/** 📨️ Selects the declared typed input channel while preserving the shell's operation lifecycle. */
import type { ShellInputActionV1, InputOutcomeV1 } from "../🎯️input-ledger/🟦️.ts";

type CommandV1 = Readonly<{ id: string; kind?: string }>;
type OwnerV1 = Readonly<{ pluginId: string; app: Readonly<{ id: string; commands?: readonly CommandV1[]; modes?: readonly Readonly<{ id: string; commands?: readonly CommandV1[] }>[] }>; pluginCommands?: readonly CommandV1[] }>;

/** 📦️ Preserves actual guest output only after the input has settled successfully. */
export function programInputResultV1(outcome: InputOutcomeV1, output: unknown): InputOutcomeV1 {
  return outcome.kind === "applied" && output !== undefined ? { ...outcome, output } : outcome;
}

/** 🏁️ Uses actual terminal job output instead of its initial admission token. */
export function settledProgramOutputV1(immediate: unknown, pending: boolean, completion: Readonly<{ terminalOutput?: unknown }> | null): unknown {
  return pending ? completion?.terminalOutput : immediate;
}

/** 🪪️ Resolves a command only against the addressed program's declarations. */
export function declaredProgramCommandV1(action: ShellInputActionV1, target: OwnerV1): CommandV1 | null {
  const address = action.commandAddress;
  if (!address) return null;
  const owner = address.owner;
  if ("plugin" in owner) return owner.plugin.pluginId === target.pluginId ? target.pluginCommands?.find(command => command.id === address.commandId) ?? null : null;
  const scoped = "mode" in owner ? owner.mode : owner.app;
  if (scoped.pluginId !== target.pluginId || scoped.appId !== target.app.id) return null;
  const commands = "mode" in owner ? target.app.modes?.find(mode => mode.id === owner.mode.modeId)?.commands : target.app.commands;
  return commands?.find(command => command.id === address.commandId) ?? null;
}

/** 🚀️ Calls the actual action or command port with the same view and causal dispatch authority. */
export function invokeProgramInputV1<T, V, D>(plugin: Readonly<{ handleAction: (instanceId: number, invocation: string, view: V, dispatch: D) => Promise<T>; handleCommand?: (instanceId: number, invocation: string, view: V, dispatch: D) => Promise<T> }>, instanceId: number, action: ShellInputActionV1, actionInvocation: () => string, view: V, dispatch: D): Promise<T> {
  if (!action.commandAddress) return plugin.handleAction(instanceId, actionInvocation(), view, dispatch);
  if (!plugin.handleCommand) return Promise.reject(new Error("program exposes no typed command port"));
  return plugin.handleCommand(instanceId, JSON.stringify({ address: action.commandAddress, arguments: action.args ?? {} }), view, dispatch);
}

/** 🖼️ Projects shell window coordinates into their owned guest context and preserves actual surface authority. */
export function ownedCallerViewContextV1(caller: ResolvedPluginViewState, retained: PluginViewState, spawnedId: string | null, spawnedIds: readonly string[]): ResolvedPluginViewState | null {
  let actual: ResolvedPluginViewState;
  try { actual = parseResolvedPluginViewState(retained); } catch { return null; }
  const ids = [caller.windowId, caller.focusedWindowId, ...(caller.windowInstances ?? []).map(window => window.id), ...Object.keys(caller.activeUtilityByWindowId ?? {}), ...Object.keys(caller.toolRunTraceCursorByWindowId ?? {})].filter((id): id is string => id !== undefined);
  if (ids.some(id => { const owner = spawnedIdOfWindowInstanceV1(id, spawnedIds); return owner !== null && owner !== spawnedId; })) return null;
  const guestId = (id: string): string => guestWindowIdV1(id, spawnedIds) ?? id;
  const normalized: ResolvedPluginViewState = {
    ...caller,
    ...(caller.windowId === undefined ? {} : { windowId: guestId(caller.windowId) }),
    ...(caller.focusedWindowId === undefined ? {} : { focusedWindowId: guestId(caller.focusedWindowId) }),
    ...(caller.windowInstances === undefined ? {} : { windowInstances: caller.windowInstances.map(window => ({ ...window, id: guestId(window.id) })) }),
    ...(caller.activeUtilityByWindowId === undefined ? {} : { activeUtilityByWindowId: Object.fromEntries(Object.entries(caller.activeUtilityByWindowId).map(([id, utility]) => [guestId(id), utility])) }),
    ...(caller.toolRunTraceCursorByWindowId === undefined ? {} : { toolRunTraceCursorByWindowId: Object.fromEntries(Object.entries(caller.toolRunTraceCursorByWindowId).map(([id, cursor]) => [guestId(id), cursor])) }),
  };
  const ownWindow = (id: string): boolean => (actual.windowInstances ?? []).some(window => window.id === id);
  if (normalized.focusedWindowId !== undefined && !ownWindow(normalized.focusedWindowId)
    || Object.keys(normalized.activeUtilityByWindowId ?? {}).some(id => !ownWindow(id))
    || Object.keys(normalized.toolRunTraceCursorByWindowId ?? {}).some(id => !ownWindow(id))
    || normalized.windowId !== undefined && normalized.windowId !== actual.windowId
    || normalized.activeModeId !== undefined && normalized.activeModeId !== actual.activeModeId
    || normalized.activeWindowKindId !== undefined && normalized.activeWindowKindId !== actual.activeWindowKindId
    || normalized.sessionIdentity !== undefined && JSON.stringify(normalized.sessionIdentity) !== JSON.stringify(actual.sessionIdentity)
    || (normalized.windowInstances ?? []).some(window => !(actual.windowInstances ?? []).some(owned => owned.id === window.id && owned.windowKindId === window.windowKindId))) return null;
  return { ...actual, ...normalized, sessionIdentity: actual.sessionIdentity };
}

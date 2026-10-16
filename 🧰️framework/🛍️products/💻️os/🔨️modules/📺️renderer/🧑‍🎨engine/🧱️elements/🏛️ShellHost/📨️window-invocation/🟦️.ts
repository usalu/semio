/** 📨️ Plugin and window action invocation encoding, released-pointer bookkeeping, the window host event handlers and app command ownership checks.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import React from "react";
// #region 🔌️Adapters
import {
  HOST_EVENT_ACTION_ID,
  type HostEventKind,
  type ActionDescriptor,
  type ActionInvocation,
  type AppDefinition,
  type CommandInvocation
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { type ActiveSession, type ExtraWindowInstance } from "../../🐚️Shell/🟦️.tsx";
import { sessionWindowInstances } from "../../🛠️ShellHelpers/🟦️.tsx";
import { parseHubProgramIdV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";

//#region FrameworkOsShell
/** 🪪️ The plugin an invocation addresses for a program: its descriptor's own plugin id. A hub document's
 * catalog-resolved program id (`🌎️hub-source/🔍️resolution`) is a host identity only — the guest and the hub's verified
 * execution target know the plugin by its descriptor id, and refuse any other owner. */
export function invocationPluginIdV1(programId: string): string {
  return parseHubProgramIdV1(programId)?.pluginId ?? programId;
}

/** 🪟️ Builds the sole action wire shape from the exact target window instance and its owner chain. */
export function windowActionInvocation(
  session: ActiveSession,
  action: ActionDescriptor,
  extraInstances: readonly Pick<ExtraWindowInstance, "id" | "windowKindId">[] = [],
  requestedWindowId?: string,
): ActionInvocation {
  const instances = sessionWindowInstances(session.app, extraInstances);
  const windowInstanceId = requestedWindowId ?? session.viewState.windowId ?? session.viewState.activeWindowKindId ?? instances[0]?.id ?? session.app.windowKinds[0]?.id ?? "";
  const windowKindId = instances.find((instance) => instance.id === windowInstanceId)?.windowKindId ?? session.app.windowKinds.find((kind) => kind.id === windowInstanceId)?.id ?? session.app.windowKinds[0]?.id ?? "";
  return {
    address: {
      pluginId: invocationPluginIdV1(session.pluginId),
      appId: session.app.id,
      modeId: session.viewState.activeModeId ?? session.app.defaultModeId ?? session.app.modes[0]?.id ?? session.app.id,
      windowKindId,
      windowInstanceId,
      actionId: action.action,
    },
    arguments: {
      ...(typeof action.args === "object" && action.args != null ? (action.args as Record<string, unknown>) : {}),
      windowId: windowInstanceId,
    },
  };
}

/** ✋️ Pointers whose capture a pane saw released by their own `pointerup`, by window: the implicit release that
 * follows is no lost capture. */
const RELEASED_POINTERS_BY_WINDOW = new Map<string, Set<number>>();

/** 📨️ Forwards a window pane's focus loss and its lost pointer capture to the program as `hostEvent{windowId, kind}`, so
 * an open gesture there ends without a trace: focus moving within the pane is no blur, and the implicit capture release
 * after the pane's own `pointerup` is no loss — a `pointercancel` or a capture taken away is. */
export function windowHostEventHandlersV1(controllerId: string, windowId: string, onAction: (action: ActionDescriptor) => unknown): Pick<React.ComponentPropsWithoutRef<"div">, "onBlur" | "onPointerUpCapture" | "onPointerCancelCapture" | "onLostPointerCapture"> {
  const forward = (kind: HostEventKind) => void onAction({ controllerId, action: HOST_EVENT_ACTION_ID, args: { windowId, kind } });
  const released = (): Set<number> => RELEASED_POINTERS_BY_WINDOW.get(windowId) ?? RELEASED_POINTERS_BY_WINDOW.set(windowId, new Set()).get(windowId)!;
  return {
    onBlur: (event) => {
      if (!(event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget))) forward("blur");
    },
    onPointerUpCapture: (event) => void released().add(event.pointerId),
    onPointerCancelCapture: (event) => void released().delete(event.pointerId),
    onLostPointerCapture: (event) => {
      if (!released().delete(event.pointerId)) forward("captureLost");
    },
  };
}

/** 🧩️ Serializes the exact action invocation used by the retained actor and ordinary plugin routes. */
export function encodeWindowActionInvocation(session: ActiveSession, action: ActionDescriptor, extraInstances: readonly Pick<ExtraWindowInstance, "id" | "windowKindId">[] = [], requestedWindowId?: string): string {
  return JSON.stringify(windowActionInvocation(session, action, extraInstances, requestedWindowId));
}

/** 🎛️ Builds an app-owned command wire without pretending host catalogue state is a window action. */
export function encodeAppCommandInvocation(pluginId: string, app: AppDefinition, commandId: string, args: Readonly<Record<string, unknown>>): string {
  const invocation: CommandInvocation = {
    address: { owner: { app: { pluginId: invocationPluginIdV1(pluginId), appId: app.id } }, commandId },
    arguments: { ...args },
  };
  return JSON.stringify(invocation);
}

/** 📋️ Tests whether an app explicitly opts into a host-pushed command. */
export function appOwnsCommand(app: AppDefinition, commandId: string): boolean {
  return (app.commands ?? []).some((command) => command.id === commandId);
}

/** 📄️ Tests whether the app's OWN declaration of a host-pushed command includes `pageCount`.
 * The shell still sends exactly those arguments (`page: 0`, `pageCount: 1`) as ONE pack-encoded
 * `handleCommand`. It does not cut the payload into 4 KiB JSON pages
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function appCommandTakesPageRun(app: AppDefinition, commandId: string): boolean {
  const command = (app.commands ?? []).find((entry) => entry.id === commandId);
  return (command?.args ?? []).some((arg) => arg.id === "pageCount");
}
//#endregion FrameworkOsShell

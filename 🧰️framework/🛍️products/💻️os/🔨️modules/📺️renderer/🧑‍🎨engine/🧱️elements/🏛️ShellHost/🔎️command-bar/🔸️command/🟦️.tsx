/** 🔸️ The command phase of useCommandBar. */
import { useCallback, useEffect, useMemo } from "react";
import { SemioFaultError, SURFACE_FAULT_CODES, type CommandAddress, type CommandInvocation, type LocalizedLabel, resolveModeTools, resolveUiDirtyScope } from "@semio-tech/framework";
import { SHELL_HUB_ROUTE } from "../../🔀️surface-switch/🟦️.ts";
import { programHistoryKeyV1, programHistoryProjectionV1 } from "../../🪟️spawned-program/🟦️.ts";
import { toggleDocumentFullscreen, useShellKeydown } from "@semio-tech/ui-react";
import { FRAMEWORK_CATEGORY_COMMAND_ID, EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { buildCommandCategoryTabs, buildToolTabs, commandAddressKey, commandKeybindingChords, commandOwnerPluginId, detectCommandPlatform, dispatchOsCommand, isOsCommandAddress } from "../../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { OPEN_HUB_COMMAND_ID, OPEN_TASK_MANAGER_COMMAND_ID, resolveManifestLabel, shellLabel, viewerReadOnlyNoticeText } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { controlOwnsKeyboardEvent, keyboardEventMatchesChord, resolveKeybindingIntent } from "../../../🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";
import { manifestLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { appFaultNoticeV1, historyFaultNoticeV1, historySilentRefusalOfFaultV1 } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { FRAMEWORK_TASK_MANAGER_PANEL_ID } from "../../../📌️ChromePanels/🆔️panel-ids/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.tsx";

/** 🔹 Builds onCommand, handleCommandKeydown, commandCategoryTabs, resolvedModeTools of useCommandBar. */
export function useCommandState({ resolvedCommands, expandedCommandIdRef, dispatch, commandStagedArgsByCommandIdRef, startTutorialRef, toggleTutorialRecordingRef, scope, navigateHistory, commitUiPreference, dockLayoutStore, dockUiStateStore, locks, noteShellCommand, uiTerminology, session, tutorialRecordingRef, tutorialDrivenRef, tutorialRecorderRef, loadedPlugins, showTransientNotice, uiLocale, injectActiveUtility, captureDialogOrigin, isCurrentDialogOrigin, directBrowserActorForSession, dispatchDirectBrowserActorCommand, captureEffectOwner, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, applyHostEffects, isViewerReadOnlyFault, isMutationRejectedFault, showMutationRejectedNotice, historyStore, uiTerminologyRef, replayShellOwnedCommandRef, mobile, commandCategoryList, focusedApp, focusedModeId, toolMeasuresByToolIdRef, onActionStable }: Pick<ReturnType<typeof useScopeState>, "resolvedCommands" | "expandedCommandIdRef" | "dispatch" | "commandStagedArgsByCommandIdRef" | "startTutorialRef" | "toggleTutorialRecordingRef" | "scope" | "navigateHistory" | "commitUiPreference" | "dockLayoutStore" | "dockUiStateStore" | "locks" | "noteShellCommand" | "uiTerminology" | "session" | "tutorialRecordingRef" | "tutorialDrivenRef" | "tutorialRecorderRef" | "loadedPlugins" | "showTransientNotice" | "uiLocale" | "injectActiveUtility" | "captureDialogOrigin" | "isCurrentDialogOrigin" | "directBrowserActorForSession" | "dispatchDirectBrowserActorCommand" | "captureEffectOwner" | "isCurrentEffectOwner" | "applyHistoryPatch" | "applyLeftoverInteractionView" | "applyHostEffects" | "isViewerReadOnlyFault" | "isMutationRejectedFault" | "showMutationRejectedNotice" | "historyStore" | "uiTerminologyRef" | "replayShellOwnedCommandRef" | "mobile" | "commandCategoryList" | "focusedApp" | "focusedModeId" | "toolMeasuresByToolIdRef" | "onActionStable">) {
  useEffect(() => {
    const valid = new Set(resolvedCommands.map((entry) => commandAddressKey(entry.address)));
    if (expandedCommandIdRef.current && !valid.has(expandedCommandIdRef.current)) dispatch({ type: "SET_COMMAND_EXPANDED", value: null });
    for (const commandKey of Object.keys(commandStagedArgsByCommandIdRef.current)) {
      if (!valid.has(commandKey)) dispatch({ type: "RESET_COMMAND_ARGS", commandId: commandKey });
    }
  }, [resolvedCommands]);

  /**
   * 🎛️ Dispatches a resolved command: os-scope commands are handled locally (no program round trip);
   * plugin/app/mode-scope commands route through the active session's program `handleCommand`, mirroring
   * `onAction`'s tail. Plugin commands are only resolvable/dispatchable for the active session's program
   * instance (no headless-instance routing for non-focused plugins yet).
   */
  const onCommand = useCallback(
    (address: CommandAddress, args?: Record<string, unknown>) => {
      const commandId = address.commandId;
      if (isOsCommandAddress(address) && commandId === "os.playTutorial") {
        const tutorialId = typeof args?.tutorialId === "string" ? args.tutorialId : "";
        if (tutorialId) startTutorialRef.current(tutorialId);
        return;
      }
      if (isOsCommandAddress(address) && commandId === "os.recordTutorial") {
        toggleTutorialRecordingRef.current();
        return;
      }
      if (isOsCommandAddress(address) && commandId === "os.toggleFullscreen") {
        void toggleDocumentFullscreen(scope.rootRef.current ?? document.documentElement).catch((error) => console.error("Fullscreen request was rejected", error));
      }
      if (isOsCommandAddress(address) && commandId === OPEN_HUB_COMMAND_ID) {
        navigateHistory(SHELL_HUB_ROUTE);
      }
      if (isOsCommandAddress(address) && commandId === OPEN_TASK_MANAGER_COMMAND_ID) {
        dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-right", value: [FRAMEWORK_TASK_MANAGER_PANEL_ID] });
        dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-right", value: true });
      }
      if (isOsCommandAddress(address)) {
        dispatchOsCommand(commandId, args, commitUiPreference, dispatch, dockLayoutStore, dockUiStateStore, locks);
        const rawCommandLabel = resolvedCommands.find((entry) => commandAddressKey(entry.address) === commandAddressKey(address))?.definition.label as LocalizedLabel | string | undefined;
        noteShellCommand(commandId, manifestLabelTextV1(rawCommandLabel ?? commandId, uiTerminology), args);
        return;
      }
      if (!session) return;
      if (tutorialRecordingRef.current && !tutorialDrivenRef.current.active) {
        tutorialRecorderRef.current?.recordEvent({ kind: "command", command: commandId, args });
      }
      const ownerPluginId = commandOwnerPluginId(address.owner);
      if (!ownerPluginId || ownerPluginId !== session.pluginId) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === ownerPluginId)?.handle;
      if (!plugin) return;
      if (session.app.role === "viewer" && resolvedCommands.find((entry) => commandAddressKey(entry.address) === commandAddressKey(address))?.definition.kind === "mutation") {
        showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
        return;
      }
      const dispatchViewState = injectActiveUtility(session.viewState);
      const invocation: CommandInvocation = { address, arguments: args ?? {} };
      const commandOrigin = captureDialogOrigin(session);
      if (!isCurrentDialogOrigin(commandOrigin)) return;
      let directBrowserActor: ReturnType<typeof directBrowserActorForSession>;
      try {
        directBrowserActor = directBrowserActorForSession(session);
      } catch (error) {
        return;
      }
      if (directBrowserActor !== null) {
        void dispatchDirectBrowserActorCommand(directBrowserActor, invocation, dispatchViewState).catch((error) => {
          showTransientNotice(shellLabel("ui.common.renderError"), "error");
        });
        return;
      }
      if (!plugin.handleCommand) return;
      const commandOwner = captureEffectOwner(session, commandOrigin);
      if (!isCurrentEffectOwner(commandOwner)) return;
      void plugin
        .handleCommand(session.instanceId, JSON.stringify(invocation), dispatchViewState)
        .then((response) => {
          if (!isCurrentEffectOwner(commandOwner)) return;
          applyHistoryPatch(response.historyPatch);
            applyLeftoverInteractionView(response.output);
          return applyHostEffects(response.requestedEffects ?? [], { ...session, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope), commandOwner);
        })
        .catch((error) => {
          if (isViewerReadOnlyFault(error)) {
            showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
            return;
          }
          if (isMutationRejectedFault(error)) {
            showMutationRejectedNotice((error as SemioFaultError).fault);
            return;
          }
          const silentRefusal = error instanceof SemioFaultError ? historySilentRefusalOfFaultV1(error.fault) : null;
          if (silentRefusal !== null) {
            console.debug(`[os-shell] a command was ignored: ${silentRefusal}`);
            return;
          }
          const historyNotice = error instanceof SemioFaultError ? historyFaultNoticeV1(error.fault, programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(session), EMPTY_SHELL_HISTORY_PROJECTION_V1).editCount) : null;
          if (historyNotice !== null) {
            showTransientNotice(historyNotice.text, historyNotice.kind, historyNotice.code);
            return;
          }
          const appNotice = error instanceof SemioFaultError ? appFaultNoticeV1(error.fault, session.app, uiTerminologyRef.current) : null;
          if (appNotice !== null) {
            showTransientNotice(appNotice.text, appNotice.kind, appNotice.code);
            return;
          }
          console.error("Command execution failed", error);
        });
    },
    [applyHostEffects, applyHistoryPatch, applyLeftoverInteractionView, captureDialogOrigin, isCurrentDialogOrigin, captureEffectOwner, directBrowserActorForSession, dispatchDirectBrowserActorCommand, isCurrentEffectOwner, commitUiPreference, dockLayoutStore, dockUiStateStore, injectActiveUtility, loadedPlugins, session, locks, resolvedCommands, noteShellCommand, showTransientNotice, isViewerReadOnlyFault, uiLocale],
  );

  replayShellOwnedCommandRef.current = (commandId, args) => dispatchOsCommand(commandId, args, commitUiPreference, dispatch, dockLayoutStore, dockUiStateStore, locks);

  const handleCommandKeydown = useCallback(
    (event: globalThis.KeyboardEvent) => {
      if (event.defaultPrevented || controlOwnsKeyboardEvent(event)) return;
      for (const entry of [...resolvedCommands].reverse()) {
        if (!entry.definition.inPalette) continue;
        const entryKey = commandAddressKey(entry.address);
        const platform = detectCommandPlatform(typeof navigator !== "undefined" ? `${navigator.platform} ${navigator.userAgent}` : "");
        const keys = commandKeybindingChords(entry.definition, platform).join(",");
        if (!keys?.split(",").some((chord) => keyboardEventMatchesChord(event, chord.trim().toLowerCase()))) continue;
        event.preventDefault();
        const staged = commandStagedArgsByCommandIdRef.current[entryKey] ?? {};
        const intent = resolveKeybindingIntent(entry.definition, expandedCommandIdRef.current === entryKey ? entry.definition.id : null, staged);
        if (intent.kind === "fire") onCommand(entry.address);
        else if (intent.kind === "execute") onCommand(entry.address, intent.args);
        else {
          const commandPath = [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${entry.definition.category}`];
          if (mobile) {
            dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
            dispatch({ type: "SET_MOBILE_PANEL_PATH", value: commandPath });
          } else {
            dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
            dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: commandPath });
          }
          dispatch({ type: "SET_COMMAND_EXPANDED", value: entryKey });
        }
        return;
      }
    },
    [mobile, onCommand, resolvedCommands],
  );
  useShellKeydown(scope.rootRef, handleCommandKeydown, [handleCommandKeydown]);

  const commandCategoryTabs = useMemo(() => buildCommandCategoryTabs(resolvedCommands, commandCategoryList, expandedCommandIdRef, commandStagedArgsByCommandIdRef, onCommand, dispatch), [resolvedCommands, commandCategoryList, onCommand]);

  const resolvedModeTools = useMemo(
    () => resolveModeTools(focusedApp ?? undefined, focusedModeId).map((tool) => ({ ...tool, label: resolveManifestLabel(tool.label as LocalizedLabel | string, uiTerminology, uiLocale) })),
    [focusedApp, focusedModeId, uiTerminology, uiLocale],
  );

  const hasToolSession = session !== null && session !== undefined;
  const toolTabs = useMemo(() => (hasToolSession ? buildToolTabs(resolvedModeTools, toolMeasuresByToolIdRef, onActionStable) : []), [hasToolSession, resolvedModeTools, onActionStable]);
  return { onCommand, commandCategoryTabs, toolTabs };
}

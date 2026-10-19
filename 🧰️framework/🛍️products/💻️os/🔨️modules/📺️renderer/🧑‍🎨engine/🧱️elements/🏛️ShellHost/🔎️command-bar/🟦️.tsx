/** 🔎️ Mode, example and role switchers, resolved commands, `onCommand`, command keydown handling and the command and tool tabs.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useCommandBar` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { useCallback, useEffect, useLayoutEffect, useMemo } from "react";
// #region 🔌️Adapters
import {
  type AppDefinition,
  type AppRole,
  SemioFaultError,
  SURFACE_FAULT_CODES,
  type CommandAddress,
  type CommandInvocation,
  examplesForApp,
  type LocalizedLabel,
  resolveModeTools,
  resolveUiDirtyScope,
  resolveWindowActions
} from "@semio-tech/framework";
import {
  MODE_STEP_CONTROL_IDS,
  SURFACE_ROLE_CONTROL_IDS,
  SURFACE_ROLE_ORDER,
  SHELL_HUB_ROUTE,
  surfaceRoleAppsV1
} from "../🔀️surface-switch/🟦️.ts";
import { programHistoryKeyV1, programHistoryProjectionV1, spawnedProgramWindowInstancesV1 } from "../🪟️spawned-program/🟦️.ts";
import {
  ButtonGroup,
  ButtonGroupItem,
  cn,
  ariaKeyshortcutsText,
  type IconName,
  interactiveActiveFillClass,
  NavbarExampleSelect,
  toggleDocumentFullscreen,
  useShellKeydown
} from "@semio-tech/ui-react";
import { sharedDocumentRoleSwitchRefusedV1 } from "../🧭️opening/🟦️.ts";
import { actionStageKey } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { FRAMEWORK_CATEGORY_COMMAND_ID, EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { buildActiveExampleAction, appOffersRegisteredExamples } from "../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import {
  buildCommandCategoryTabs,
  buildToolTabs,
  commandAddressKey,
  commandCategories,
  commandKeybindingChords,
  commandOwnerPluginId,
  detectCommandPlatform,
  dispatchOsCommand,
  isOsCommandAddress,
  resolveCommands
} from "../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import {
  OPEN_HUB_COMMAND_ID,
  OPEN_TASK_MANAGER_COMMAND_ID,
  resolveAppLabel,
  resolveManifestLabel,
  shellLabel,
  surfaceRoleGroupText,
  appModeGroupText,
  viewerReadOnlyNoticeText
} from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import {
  controlOwnsKeyboardEvent,
  keyboardEventMatchesChord,
  resolveKeybindingIntent
} from "../../🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";
import { isMutationKindDefinition } from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { manifestLabelTextV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import {
  appFaultNoticeV1,
  historyFaultNoticeV1,
  historySilentRefusalOfFaultV1,
  frameworkLayoutDeclaredInstances
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { sessionWindowInstances } from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
// #endregion 🔌️Adapters
import { FRAMEWORK_TASK_MANAGER_PANEL_ID } from "../../📌️ChromePanels/🆔️panel-ids/🟦️.ts";
import { replayRefusalNoticeTextV1 } from "../📣️replay-refusal/🟦️.ts";
import { type ArtifactKindChoiceDraftDefinitionV1, artifactKindChoiceDraftRetirementsV1, SURFACE_ROLE_ICON_IDS } from "../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterCheckInAndSync } from "../✅️check-in-sync/🟦️.tsx";

export type ShellCoreAfterCommandBar = ShellCoreAfterCheckInAndSync & ReturnType<typeof useCommandBar>;

//#region FrameworkOsShell
export function useCommandBar(core: ShellCoreAfterCheckInAndSync) {
  const {
    scope, hostMode, mobile, locks, dispatch, historyStore, exampleOptionsRef, lastDispatchedExampleIdRef, loadedPlugins,
    session, localPrograms, uiLocale, uiTerminology, applyHistoryPatch, applyLeftoverInteractionView, landingAppId,
    appLabelsOverlay, actionPaneStagedArgsByKey, commandStagedArgsByCommandId, activeExampleId, extraWindowInstances,
    surfaceSwitchBusy, uiTerminologyRef, replayShellOwnedCommandRef, openDocumentSessionsRef, captureDialogOrigin,
    isCurrentDialogOrigin, captureEffectOwner, isCurrentEffectOwner, sharedSpaceAccessRef, sharedSpaceAccessVersion,
    navigateHistory, dockLayoutStore, dockUiStateStore, panel, focusedProgram, focusedApp, focusedModeId,
    toolMeasuresByToolIdRef, startTutorialRef, toggleTutorialRecordingRef, tutorialDrivenRef, tutorialRecordingRef,
    tutorialRecorderRef, expandedCommandIdRef, commandStagedArgsByCommandIdRef, injectActiveUtility,
    directBrowserActorForSession, dispatchDirectBrowserActorCommand, applyHostEffects, onAction, noteShellCommand,
    onActionStable, applyModeChange, switchToSessionRole, showTransientNotice, isViewerReadOnlyFault,
    isMutationRejectedFault, showMutationRejectedNotice, controlKeybindings, osCommands, commitUiPreference,
    selectedSpaceArtifactCreationCatalog, selectedSpaceArtifactKinds, artifactCreationChoiceRevisionRef
  } = core;
  const activePluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest, [loadedPlugins, session?.pluginId]);
  const activeModeId = session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "";

  const focusedPluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === focusedProgram?.pluginId)?.manifest, [loadedPlugins, focusedProgram?.pluginId]);
  const exampleOptions = useMemo(() => {
    const app = focusedApp;
    const examples = app ? examplesForApp(focusedPluginManifest?.examples ?? [], app) : [];
    if (!app || !appOffersRegisteredExamples(app, examples.length)) return [];
    return examples.map((example) => ({
      id: example.id,
      label: resolveAppLabel(appLabelsOverlay, "example", example.id, resolveManifestLabel(example.label, uiTerminology, uiLocale)),
      icon: "file" as IconName,
    }));
  }, [focusedPluginManifest, focusedApp, appLabelsOverlay, uiTerminology, uiLocale]);
  exampleOptionsRef.current = exampleOptions;

  const dispatchActiveExample = useCallback(
    (exampleId: string) => {
      if (!session || !focusedApp || !focusedProgram) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === focusedProgram.pluginId)?.handle;
      if (!plugin) return;
      return onAction(buildActiveExampleAction(focusedApp.controllerId, exampleId));
    },
    [loadedPlugins, onAction, session, focusedApp, focusedProgram],
  );

  /** 🎛️ Shared by the desktop navbar center cluster and the mobile panel's synthetic "App" tab (see `mobilePanelTabs`). */
  const exampleSelectElement = useMemo(() => {
    if (!session || !focusedApp || exampleOptions.length === 0 || locks.exampleId || (hostMode && focusedApp.id === landingAppId)) return null;
    return (
      <NavbarExampleSelect
        key="fixture"
        id="playground.navbar.fixture"
        value={activeExampleId}
        options={exampleOptions}
        onValueChange={(exampleId) => {
          if (exampleId) lastDispatchedExampleIdRef.current = exampleId;
          dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
          dispatchActiveExample(exampleId || "");
        }}
      />
    );
  }, [session, focusedApp, exampleOptions, locks.exampleId, hostMode, landingAppId, activeExampleId, dispatchActiveExample]);

  /** 🎛️ Shared by the desktop navbar center cluster and the mobile panel's synthetic "App" tab (see `mobilePanelTabs`).
   * `aria-keyshortcuts` republishes the two framework mode-cycling chords on the GROUP, which is where
   * they belong: the chords step through the group rather than addressing any one button. */
  const modeSwitcherElement = useMemo(() => {
    if (!session || session.app.modes.length <= 1) return null;
    const stepShortcuts = ariaKeyshortcutsText([controlKeybindings.get(MODE_STEP_CONTROL_IDS.next), controlKeybindings.get(MODE_STEP_CONTROL_IDS.previous)].filter(Boolean).join(","));
    return (
      <ButtonGroup key="modes" id="playground.navbar.modes" role="group" aria-label={appModeGroupText(uiLocale)} aria-keyshortcuts={stepShortcuts}>
        {session.app.modes.map((mode) => {
          const isActive = activeModeId === mode.id;
          return (
            <ButtonGroupItem
              key={mode.id}
              id={`playground.navbar.modes.${mode.id}`}
              className={cn(isActive && interactiveActiveFillClass)}
              data-state={isActive ? "on" : undefined}
              aria-pressed={isActive}
              onClick={() => applyModeChange(mode.id)}
              icon={mode.iconId}
              text={resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label as LocalizedLabel | string, uiTerminology, uiLocale))}
            />
          );
        })}
      </ButtonGroup>
    );
  }, [session, activeModeId, applyModeChange, appLabelsOverlay, controlKeybindings, uiTerminology, uiLocale]);

  /** 👁️✏️ The plugin's editor/viewer pair for the OPEN document's dialect, or `null` when it declares
   * fewer than two surfaces for it — which is exactly when the role group must not render. */
  const sessionRoleApps = useMemo((): Readonly<Record<AppRole, AppDefinition>> | null => {
    if (!session) return null;
    const apps = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.manifest.apps ?? [];
    return surfaceRoleAppsV1(apps, session.app.dialect);
  }, [loadedPlugins, session]);

  /** 👁️✏️ Navbar role group — the in-shell control that makes `…#viewer` reachable without a
   * reload (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 2). Labels come from each target
   * `AppDefinition`'s OWN localized label, so en/de follow the plugin rather than a shell dictionary.
   * Shares the mode switcher's placement in both the desktop navbar cluster and the mobile "App" tab. */
  const roleSwitcherElement = useMemo(() => {
    if (!session || sessionRoleApps === null) return null;
    const scope = [...openDocumentSessionsRef.current.values()].find((entry) => entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId)?.scope;
    const access = scope === undefined ? null : (sharedSpaceAccessRef.current.get(scope.spaceId) ?? null);
    const refused = (role: AppRole, isActive: boolean): boolean => !isActive && sharedDocumentRoleSwitchRefusedV1(access, role);
    return (
      <ButtonGroup key="roles" id="playground.navbar.roles" role="group" aria-label={surfaceRoleGroupText(uiLocale)} aria-busy={surfaceSwitchBusy || undefined}>
        {SURFACE_ROLE_ORDER.map((role) => {
          const app = sessionRoleApps[role];
          const isActive = session.app.role === role;
          return (
            <ButtonGroupItem
              key={role}
              id={SURFACE_ROLE_CONTROL_IDS[role]}
              className={cn(isActive && interactiveActiveFillClass)}
              data-state={isActive ? "on" : undefined}
              data-role={role}
              aria-pressed={isActive}
              disabled={(surfaceSwitchBusy && !isActive) || refused(role, isActive)}
              aria-description={refused(role, isActive) ? replayRefusalNoticeTextV1("view-only-access", uiLocale) : undefined}
              aria-keyshortcuts={ariaKeyshortcutsText(controlKeybindings.get(SURFACE_ROLE_CONTROL_IDS[role]))}
              onClick={() => void switchToSessionRole(role).catch((switchError) => console.error(`role switch to ${role} failed`, switchError))}
              icon={SURFACE_ROLE_ICON_IDS[role]}
              text={resolveManifestLabel(app.label as LocalizedLabel | string, uiTerminology, uiLocale)}
            />
          );
        })}
      </ButtonGroup>
    );
  }, [session, sessionRoleApps, surfaceSwitchBusy, switchToSessionRole, controlKeybindings, uiTerminology, uiLocale, sharedSpaceAccessVersion]);

  const resolvedCommands = useMemo(() => {
    const resolved = resolveCommands(osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale, localPrograms.map((entry) => entry.manifest), selectedSpaceArtifactKinds);
    return session?.app.role === "viewer" ? resolved.filter((entry) => !isMutationKindDefinition(entry.definition)) : resolved;
  }, [osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale, loadedPlugins, selectedSpaceArtifactKinds]);

  useLayoutEffect(() => {
    const revision = selectedSpaceArtifactCreationCatalog?.choiceRevision;
    if (artifactCreationChoiceRevisionRef.current === revision) return;
    artifactCreationChoiceRevisionRef.current = revision;
    const choiceArgs = (args: readonly { readonly id: string; readonly schema: { readonly kind: string; readonly format?: { readonly kind: string } } }[]) =>
      args.map((arg) => ({ id: arg.id, artifactKind: arg.schema.kind === "string" && arg.schema.format?.kind === "artifactKind" }));
    const actionDefinitions: ArtifactKindChoiceDraftDefinitionV1[] = [];
    if (session !== null) {
      for (const instance of sessionWindowInstances(session.app, extraWindowInstances)) {
        const kind = session.app.windowKinds.find((candidate) => candidate.id === instance.windowKindId);
        if (kind === undefined) continue;
        for (const definition of resolveWindowActions(session.app, kind)) actionDefinitions.push({ ownerId: actionStageKey(instance.id, definition.id), args: choiceArgs(definition.args) });
      }
    }
    for (const spawned of panel?.spawnedApps ?? []) {
      const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (app === undefined) continue;
      for (const instance of spawnedProgramWindowInstancesV1(spawned.id, app.windowKinds, frameworkLayoutDeclaredInstances(app.defaultLayout, app.windowKinds))) {
        const kind = app.windowKinds.find((candidate) => candidate.id === instance.windowKindId);
        if (kind === undefined) continue;
        for (const definition of resolveWindowActions(app, kind)) actionDefinitions.push({ ownerId: actionStageKey(instance.id, definition.id), args: choiceArgs(definition.args) });
      }
    }
    for (const retirement of artifactKindChoiceDraftRetirementsV1(actionPaneStagedArgsByKey, actionDefinitions)) {
      const split = retirement.ownerId.lastIndexOf(":");
      if (split < 1) continue;
      dispatch({ type: "STAGE_ACTION_ARG", windowId: retirement.ownerId.slice(0, split), actionId: retirement.ownerId.slice(split + 1), argId: retirement.argId, value: undefined });
    }
    const commandDefinitions = resolvedCommands.map((entry) => ({ ownerId: commandAddressKey(entry.address), args: choiceArgs(entry.definition.args) }));
    for (const retirement of artifactKindChoiceDraftRetirementsV1(commandStagedArgsByCommandId, commandDefinitions)) {
      dispatch({ type: "STAGE_COMMAND_ARG", commandId: retirement.ownerId, argId: retirement.argId, value: undefined });
    }
  }, [actionPaneStagedArgsByKey, commandStagedArgsByCommandId, extraWindowInstances, loadedPlugins, panel?.spawnedApps, resolvedCommands, selectedSpaceArtifactCreationCatalog?.choiceRevision, session]);

  const commandCategoryList = useMemo(() => commandCategories(resolvedCommands), [resolvedCommands, uiLocale]);

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

  return {
    focusedPluginManifest, exampleOptions, dispatchActiveExample, exampleSelectElement, modeSwitcherElement,
    roleSwitcherElement, resolvedCommands, onCommand, commandCategoryTabs, toolTabs
  };
}
//#endregion FrameworkOsShell

/** 🔶️ The scope phase of useCommandBar. */
import { useCallback, useLayoutEffect, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { type AppDefinition, type AppRole, examplesForApp, type LocalizedLabel, resolveWindowActions } from "@semio-tech/framework";
import { MODE_STEP_CONTROL_IDS, SURFACE_ROLE_CONTROL_IDS, SURFACE_ROLE_ORDER, surfaceRoleAppsV1 } from "../../🔀️surface-switch/🟦️.ts";
import { spawnedProgramWindowInstancesV1 } from "../../🪟️spawned-program/🟦️.ts";
import { ButtonGroup, ButtonGroupItem, cn, ariaKeyshortcutsText, type IconName, interactiveActiveFillClass, NavbarExampleSelect } from "@semio-tech/ui-react";
import { sharedDocumentRoleSwitchRefusedV1 } from "../../🧭️opening/🟦️.ts";
import { actionStageKey } from "../../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { buildActiveExampleAction, appOffersRegisteredExamples } from "../../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import { commandAddressKey, commandCategories, resolveCommands } from "../../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { resolveAppLabel, resolveManifestLabel, surfaceRoleGroupText, appModeGroupText } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { isMutationKindDefinition } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { frameworkLayoutDeclaredInstances } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { sessionWindowInstances } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { replayRefusalNoticeTextV1 } from "../../📣️replay-refusal/🟦️.ts";
import { type ArtifactKindChoiceDraftDefinitionV1, artifactKindChoiceDraftRetirementsV1, SURFACE_ROLE_ICON_IDS } from "../../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterCheckInAndSync } from "../../✅️check-in-sync/🟦️.tsx";

/** 🔷 Builds scope, hostMode, mobile, locks of useCommandBar. */
export function useScopeState({ core }: { readonly core: ShellCoreAfterCheckInAndSync }) {
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

  /** 🧮️ Shared by the desktop navbar center cluster and the mobile panel's synthetic "App" tab (see `mobilePanelTabs`).
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

  /** 🔸️ Navbar role group — the in-shell control that makes `…#viewer` reachable without a
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
  return { resolvedCommands, expandedCommandIdRef, dispatch, commandStagedArgsByCommandIdRef, startTutorialRef, toggleTutorialRecordingRef, scope, navigateHistory, commitUiPreference, dockLayoutStore, dockUiStateStore, locks, noteShellCommand, uiTerminology, session, tutorialRecordingRef, tutorialDrivenRef, tutorialRecorderRef, loadedPlugins, showTransientNotice, uiLocale, injectActiveUtility, captureDialogOrigin, isCurrentDialogOrigin, directBrowserActorForSession, dispatchDirectBrowserActorCommand, captureEffectOwner, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, applyHostEffects, isViewerReadOnlyFault, isMutationRejectedFault, showMutationRejectedNotice, historyStore, uiTerminologyRef, replayShellOwnedCommandRef, mobile, commandCategoryList, focusedApp, focusedModeId, toolMeasuresByToolIdRef, onActionStable, focusedPluginManifest, exampleOptions, dispatchActiveExample, exampleSelectElement, modeSwitcherElement, roleSwitcherElement };
}

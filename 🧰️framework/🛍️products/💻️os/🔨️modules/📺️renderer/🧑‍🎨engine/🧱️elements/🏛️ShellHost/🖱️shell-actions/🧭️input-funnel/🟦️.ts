/** 🧭️ The staged body of the shell input funnel: recovery, shell-intercepted actions, dispatch target resolution and program dispatch.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useShellActions}: the body of the shell input funnel (`onAction`) after its ledger entry is issued, moved verbatim. */
import { type ShellDialogOriginV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type ActionDispatchLifecycleV1, SURFACE_FAULT_CODES, RECORD_TUTORIAL_ACTION_ID, resolveWindowActions, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, hostArmedViewContext } from "@semio-tech/framework";
import { FRAMEWORK_SYNC_CONTROLLER_ID } from "@semio-tech/framework/backbone";
import { spawnedProgramViewStateV1, guestWindowIdV1, spawnedIdOfWindowInstanceV1 } from "../../🪟️spawned-program/🟦️.ts";
import { INPUT_IDENTITY_SETTLE_BOUND_MS_V1, inputAwaitsHubIdentityV1, inputIdentitySettleNoticeTextV1, type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../🎯️input-ledger/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { NOTE_WORLD_NAVIGATION_ACTION_ID, TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS, undeclaredActionDiagnostic } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { rememberedExampleIdFromDispatchV1, frameworkOwnsExampleSwitch } from "../../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import { frameworkLayoutDeclaredInstances } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { checkinSubmitMessageV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { FRAMEWORK_CHECKIN_CONTROLLER_ID, viewerReadOnlyNoticeText } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { requestFileOpen } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { pasteActionWithRetainedFragment } from "../../../🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts";
import { abortExtensionRequestsForActor, isDeclaredSurfaceCancelAction } from "../../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { typedOperationCancellationOwnerIsCurrentV1 } from "../../🏁️operation-settle/🟦️.ts";
import type { ShellCoreAfterShellUriAndDocuments } from "../../🗄️shell-uri-documents/🟦️.ts";
import { handleSetActiveToolAction, handleSetActiveUtilityAction } from "./🧰️utility-and-tool-activation/🟦️.ts";
import { handleRecoveryAction, handleSetActivePanelTabAction, handleSyncCardAction, handleWorldNavigationAction } from "./🔄️recovery-navigation-sync-and-panel-actions/🟦️.ts";
import { buildDispatchViewState, resolveDispatchTarget, routeRemoteUndo } from "./🎯️dispatch-target-and-view-state/🟦️.ts";
import { dispatchDirectBrowserActorAction, dispatchProgramAction } from "./🚀️program-dispatch/🟦️.ts";

/** 🚦️ Runs one shell input through the funnel stages and answers its ledger outcome; a throw is mapped by the caller. */
export async function runShellInput({ clipboardFragmentRef, primaryPluginId, dispatch, reloadPlugin, uninstallPlugin, session, hubIdentityGateRef, showTransientNoticeRef, uiLocaleRef, hubIdentitySettleRef, spawnedIdsRef, panel, loadedPlugins, extraWindowInstancesRef, captureDialogOrigin, isCurrentDialogOrigin, captureEffectOwner, startTutorialRef, toggleTutorialRecordingRef, tutorialPlayingRef, tutorialDrivenRef, tutorialRecordingRef, tutorialRecorderRef, completeIntroductionInteraction, activeWindowIdRef, activeUtilityByWindowIdRef, writeUtilityRegister, activeToolIdRef, writeToolRegister, uiTerminologyRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, clearAllWindowUtilities, dispatchCheckpointRef, browseSyncBackbonePath, syncCardKind, syncDraftPath, attachSyncTarget, detachSyncBackbone, hostMode, hostControllerId, spacePrograms, spawnProgram, hostApp, updateSpacePanel, mobile, inferenceHistoryByRuntimeKeyRef, openDocumentSessionsRef, historyCanUndo, localHistoryOrderRef, verifiedSessionAuthorityRef, ensureBackboneWorker, directBrowserActorForSession, dropForSealedInstance, uiLocale, uiTerminology, identityRef, treeWindowSchedulerRef, lastDispatchedExampleIdRef, dispatchDirectBrowserActorCommand, sessionWorkRef, historyStore, exampleOptionsRef, defaults, refreshHistorySnapshot, observeLocalInteraction, awaitOperationSettle, dropForRetiredInstance, requested, refuse, applied, entry, submittedOrigin, core, requiredOperationOwner, onAction, propagateFailure, refusalReasonForError, lifecycle }: Pick<ShellCoreAfterShellUriAndDocuments, "clipboardFragmentRef" | "primaryPluginId" | "dispatch" | "reloadPlugin" | "uninstallPlugin" | "session" | "hubIdentityGateRef" | "showTransientNoticeRef" | "uiLocaleRef" | "hubIdentitySettleRef" | "spawnedIdsRef" | "panel" | "loadedPlugins" | "extraWindowInstancesRef" | "captureDialogOrigin" | "isCurrentDialogOrigin" | "captureEffectOwner" | "startTutorialRef" | "toggleTutorialRecordingRef" | "tutorialPlayingRef" | "tutorialDrivenRef" | "tutorialRecordingRef" | "tutorialRecorderRef" | "completeIntroductionInteraction" | "activeWindowIdRef" | "activeUtilityByWindowIdRef" | "writeUtilityRegister" | "activeToolIdRef" | "writeToolRegister" | "uiTerminologyRef" | "applyHistoryPatch" | "applyLeftoverInteractionView" | "isCurrentEffectOwner" | "applyHostEffects" | "clearAllWindowUtilities" | "dispatchCheckpointRef" | "browseSyncBackbonePath" | "syncCardKind" | "syncDraftPath" | "attachSyncTarget" | "detachSyncBackbone" | "hostMode" | "hostControllerId" | "spacePrograms" | "spawnProgram" | "hostApp" | "updateSpacePanel" | "mobile" | "inferenceHistoryByRuntimeKeyRef" | "openDocumentSessionsRef" | "historyCanUndo" | "localHistoryOrderRef" | "verifiedSessionAuthorityRef" | "ensureBackboneWorker" | "directBrowserActorForSession" | "dropForSealedInstance" | "uiLocale" | "uiTerminology" | "identityRef" | "treeWindowSchedulerRef" | "lastDispatchedExampleIdRef" | "dispatchDirectBrowserActorCommand" | "sessionWorkRef" | "historyStore" | "exampleOptionsRef" | "defaults" | "refreshHistorySnapshot" | "observeLocalInteraction" | "awaitOperationSettle" | "dropForRetiredInstance"> & {
  readonly requested: ShellInputActionV1;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
  readonly entry: ReturnType<ShellCoreAfterShellUriAndDocuments["inputLedgerRef"]["current"]["issue"]>;
  readonly submittedOrigin: ShellDialogOriginV1 | undefined;
  readonly core: ShellCoreAfterShellUriAndDocuments;
  readonly requiredOperationOwner: ReturnType<ShellCoreAfterShellUriAndDocuments["captureEffectOwner"]> | undefined;
  readonly onAction: (requested: ShellInputActionV1, submittedOrigin?: ShellDialogOriginV1, propagateFailure?: boolean, lifecycle?: ActionDispatchLifecycleV1, requiredOperationOwner?: ReturnType<ShellCoreAfterShellUriAndDocuments["captureEffectOwner"]>) => Promise<InputOutcomeV1>;
  readonly propagateFailure: boolean;
  readonly refusalReasonForError: (error: unknown) => { readonly reason: InputRefusalReasonV1; readonly detail?: string; readonly diagnostic?: InputDiagnosticV1; readonly draftDisposition?: InputDraftDispositionV1; };
  readonly lifecycle: ActionDispatchLifecycleV1 | undefined;
}) {
  const action = pasteActionWithRetainedFragment(requested, clipboardFragmentRef.current);
  const recoveryOutcome = handleRecoveryAction({ primaryPluginId, dispatch, reloadPlugin, uninstallPlugin, action, refuse, applied });
  if (recoveryOutcome !== undefined) return recoveryOutcome;

  if (!session) return refuse("no-session");
  if (entry.provenance.origin === "user" && inputAwaitsHubIdentityV1(hubIdentityGateRef.current)) {
    showTransientNoticeRef.current(inputIdentitySettleNoticeTextV1(uiLocaleRef.current), "info");
    await hubIdentitySettleRef.current.wait(INPUT_IDENTITY_SETTLE_BOUND_MS_V1);
  }
  const programOfWindow = (windowId: string): { readonly spawnedId: string | null; readonly session: ActiveSession; readonly extraInstances: readonly { readonly id: string; readonly windowKindId: string }[]; readonly guestWindowId: string } => {
    const spawnedId = spawnedIdOfWindowInstanceV1(windowId, spawnedIdsRef.current);
    const spawned = spawnedId === null ? undefined : panel?.spawnedApps.find((candidate) => candidate.id === spawnedId);
    const app = spawned ? loadedPlugins.find((candidate) => candidate.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId) : undefined;
    if (!spawned || !app) return { spawnedId: null, session, extraInstances: extraWindowInstancesRef.current, guestWindowId: windowId };
    return {
      spawnedId: spawned.id,
      session: { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) },
      extraInstances: frameworkLayoutDeclaredInstances(app.defaultLayout, app.windowKinds),
      guestWindowId: guestWindowIdV1(windowId, spawnedIdsRef.current) ?? windowId,
    };
  };
  if (isDeclaredSurfaceCancelAction(action.action)) {
    const aborted = abortExtensionRequestsForActor(`${session.pluginId}:${session.instanceId}`, `cancelled by ${action.action}`);
  }
  const actionOrigin = submittedOrigin ?? captureDialogOrigin(session);
  if (!isCurrentDialogOrigin(actionOrigin)) return refuse("owner-mismatch", "dialog origin is not current");
  const primaryActionOwner = captureEffectOwner(session, actionOrigin, entry.provenance.inputSeq);

  if (action.action === EXPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
    void core.documentTransferRef.current.exportDocumentArchive();
    return applied();
  }
  if (action.action === IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
    void core.documentTransferRef.current.importDocumentArchive();
    return applied();
  }
  if (action.action === SAVE_ARTIFACT_FILE_ACTION_ID) {
    void core.documentTransferRef.current.saveNaturalFile();
    return applied();
  }
  if (action.action === OPEN_ARTIFACT_FILE_ACTION_ID) {
    void core.documentTransferRef.current.openNaturalFile();
    return applied();
  }
  if (action.action === START_INTRODUCTION_ACTION_ID) {
    dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
    return applied();
  }

  if (action.action === START_TUTORIAL_ACTION_ID) {
    const args = typeof action.args === "object" && action.args != null ? (action.args as { tutorialId?: unknown }) : {};
    if (typeof args.tutorialId === "string") startTutorialRef.current(args.tutorialId);
    return applied();
  }
  if (action.action === RECORD_TUTORIAL_ACTION_ID) {
    toggleTutorialRecordingRef.current();
    return applied();
  }

  if (tutorialPlayingRef.current && !tutorialDrivenRef.current.active) {
    dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
    dispatch({ type: "SET_TUTORIAL_DEVIATED", value: true });
  }

  if (tutorialRecordingRef.current && !tutorialDrivenRef.current.active) {
    if (!TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS.has(action.action)) {
      tutorialRecorderRef.current?.recordEvent({ kind: "action", action: action.action, args: action.args as Record<string, unknown> | undefined });
    }
  }

  if (action.action === NOTE_WORLD_NAVIGATION_ACTION_ID) {
    return handleWorldNavigationAction({ extraWindowInstancesRef, completeIntroductionInteraction, session, action, applied });
  }

  if (action.action === SET_ACTIVE_UTILITY_ACTION_ID) {
    return await handleSetActiveUtilityAction({ activeWindowIdRef, activeUtilityByWindowIdRef, writeUtilityRegister, activeToolIdRef, writeToolRegister, completeIntroductionInteraction, loadedPlugins, uiLocaleRef, uiTerminologyRef, spawnedIdsRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, action, refuse, programOfWindow, entry, primaryActionOwner, applied });
  }

  if (action.action === SET_ACTIVE_TOOL_ACTION_ID) {
    return await handleSetActiveToolAction({ activeToolIdRef, writeToolRegister, clearAllWindowUtilities, completeIntroductionInteraction, activeWindowIdRef, loadedPlugins, uiLocaleRef, uiTerminologyRef, activeUtilityByWindowIdRef, spawnedIdsRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, action, refuse, programOfWindow, entry, primaryActionOwner, applied });
  }

  completeIntroductionInteraction((interaction) => interaction.on.kind === "action" && interaction.on.id === action.action);

  if (action.controllerId === FRAMEWORK_CHECKIN_CONTROLLER_ID) {
    if (action.action !== "submit") return refuse("undeclared-action", `check-in: ${action.action}`);
    dispatchCheckpointRef.current(checkinSubmitMessageV1(action.args));
    return applied();
  }

  if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
    return handleSyncCardAction({ dispatch, browseSyncBackbonePath, syncCardKind, syncDraftPath, attachSyncTarget, detachSyncBackbone, action, applied, refuse });
  }

  if (hostMode && action.action === "spawnApp" && action.controllerId !== hostControllerId) {
    const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String((action.args as { pluginId?: string }).pluginId ?? "") : "";
    const program = spacePrograms.find((entry) => entry.pluginId === pluginId);
    if (!program) return refuse("undeclared-action", `spawnApp: no program "${pluginId}"`);
    void spawnProgram(program);
    return applied();
  }

  if (hostMode && action.action === "setActivePanelTab" && (action.controllerId === hostControllerId || action.controllerId === session.app.controllerId)) {
    return handleSetActivePanelTabAction({ hostApp, updateSpacePanel, mobile, dispatch, session, action, refuse, applied });
  }

  let { targetSession, hostWindowId, targetSpawnedId } = resolveDispatchTarget({ hostMode, panel, loadedPlugins, activeWindowIdRef, spawnedIdsRef, session, action });
  const undoOutcome = routeRemoteUndo({ inferenceHistoryByRuntimeKeyRef, openDocumentSessionsRef, historyCanUndo, localHistoryOrderRef, verifiedSessionAuthorityRef, ensureBackboneWorker, action, targetSession, refuse, applied });
  if (undoOutcome !== undefined) return undoOutcome;
  if (action.action === "undo" || action.action === "redo") {
    const directNow = (() => { try { return directBrowserActorForSession(targetSession) !== null ? "yes" : "null"; } catch (error) { return `throw:${String(error)}`; } })();
    const owners = [...openDocumentSessionsRef.current.values()].map((entry) => ({ pluginId: entry.session.pluginId, instanceId: entry.session.instanceId }));
    if (directNow === "null") {
      const documentOwners = [...openDocumentSessionsRef.current.values()].filter((entry) => entry.session.pluginId === targetSession.pluginId);
      if (documentOwners.length === 1) {
        targetSession = documentOwners[0]!.session;
      }
    }
  }
  if (requiredOperationOwner !== undefined && !typedOperationCancellationOwnerIsCurrentV1(requiredOperationOwner.session, targetSession, isCurrentEffectOwner(requiredOperationOwner))) {
    return refuse("owner-mismatch", "typed operation owner is no longer current");
  }
  const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
  if (!plugin) return refuse("dispatch-failed", `no loaded program for "${targetSession.pluginId}"`);
  if (dropForSealedInstance(targetSession, "action", action.action)) return refuse("instance-sealed", `${targetSession.pluginId}#${targetSession.instanceId}`);
  const actionOwner = captureEffectOwner(targetSession, actionOrigin, entry.provenance.inputSeq);
  if (!isCurrentEffectOwner(actionOwner)) {
    return refuse("owner-mismatch", "effect owner is not current");
  }

  const { baseDispatchViewState, dispatchWindowId, dispatchExtraInstances } = buildDispatchViewState({ spawnedIdsRef, extraWindowInstancesRef, applyLeftoverInteractionView, uiLocale, uiTerminology, identityRef, activeUtilityByWindowIdRef, activeWindowIdRef, treeWindowSchedulerRef, hostWindowId, targetSpawnedId, targetSession, action });
  const dispatchViewState = hostArmedViewContext(baseDispatchViewState, activeToolIdRef.current, dispatchWindowId);
  if (!dispatchViewState) {
    return refuse("view-state-unresolved", `window=${dispatchWindowId ?? ""}`);
  }
  const undeclared = undeclaredActionDiagnostic(targetSession.app.id, action.action, targetSession.app.windowKinds, (baseDispatchViewState.windowInstances ?? []).find((instance) => instance.id === dispatchWindowId)?.windowKindId ?? null, targetSession.app.actions ?? []);
  if (undeclared && !frameworkOwnsExampleSwitch(targetSession.app.role, action.action)) {
    console.error(undeclared.message, undeclared);
    return refuse("undeclared-action", undeclared.message);
  }
  if (targetSession.app.role === "viewer" && targetSession.app.windowKinds.some((kind) => resolveWindowActions(targetSession.app, kind).some((entry) => entry.id === action.action && entry.kind === "mutation"))) {
    core.showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
    return refuse("viewer-read-only", undefined, true);
  }

  lastDispatchedExampleIdRef.current = rememberedExampleIdFromDispatchV1(action, lastDispatchedExampleIdRef.current);

  if (action.action === "openImportSnapshot") {
    void requestFileOpen("application/json,.json", "text", false)
      .then(async (opened) => {
        if (!opened[0]) return;
        onAction({ controllerId: action.controllerId, action: "importSnapshot", args: { payload: opened[0].contents, name: opened[0].name }, provenance: { ...entry.provenance, origin: "user", causedBy: entry.provenance.inputSeq } });
      })
      .catch((error) => undefined);
    return applied();
  }
  let directBrowserActor: ReturnType<typeof directBrowserActorForSession>;
  try {
    directBrowserActor = directBrowserActorForSession(targetSession);
  } catch (error) {
    const outcome = refuse("owner-mismatch", `browser actor owner: ${String(error instanceof Error ? error.message : error)}`);
    if (propagateFailure) throw error;
    return outcome;
  }
  const directOutcome = await dispatchDirectBrowserActorAction({ dispatchDirectBrowserActorCommand, uiTerminologyRef, directBrowserActor, targetSession, dispatchViewState, action, dispatchExtraInstances, dispatchWindowId, applied, refusalReasonForError, core, refuse, propagateFailure });
  if (directOutcome !== undefined) return directOutcome;
  return await dispatchProgramAction({ sessionWorkRef, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, historyStore, lastDispatchedExampleIdRef, exampleOptionsRef, defaults, dispatch, refreshHistorySnapshot, applyHostEffects, observeLocalInteraction, awaitOperationSettle, uiTerminologyRef, uiLocale, dropForSealedInstance, dropForRetiredInstance, targetSession, plugin, dispatchViewState, action, dispatchExtraInstances, dispatchWindowId, entry, actionOwner, lifecycle, onAction, actionOrigin, hostWindowId, core, applied, refuse, refusalReasonForError, propagateFailure });
}

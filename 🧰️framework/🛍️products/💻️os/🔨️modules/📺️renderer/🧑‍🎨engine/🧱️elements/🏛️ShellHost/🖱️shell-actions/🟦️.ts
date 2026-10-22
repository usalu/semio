/** 🖱️ `onAction`, shell command notes, the stable action and intent callbacks and the browser actor intent.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useShellActions` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellDialogSessionIsCurrentV1, type ShellDialogOriginV1 } from "../🗨️dialog-origin/🟦️.ts";
import { useCallback, useEffect, useRef } from "../../⚛️react-port/🟦️.ts";
// #region 🔌️Adapters
import {
  type ActionDispatchLifecycleV1,
  type ActionDescriptor,
  SemioFaultError,
  SURFACE_FAULT_CODES,
  type HistoryPatch,
  CLEAR_SELECTION_ACTION_ID,
  panelTabKindId,
  RECORD_TUTORIAL_ACTION_ID,
  resolveUiDirtyScope,
  resolveWindowActions,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
  EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  OPEN_ARTIFACT_FILE_ACTION_ID,
  SAVE_ARTIFACT_FILE_ACTION_ID,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  windowElementId,
  hostArmedViewContext,
  panelViewContext,
  windowViewContext
} from "@semio-tech/framework";
import { decodePackWire } from "@semio-tech/framework/packs";
import {
  programHistoryKeyV1,
  programHistoryProjectionV1,
  spawnedProgramViewStateV1,
  guestActiveUtilityByWindowIdV1,
  guestWindowIdV1,
  spawnedIdOfWindowInstanceV1
} from "../🪟️spawned-program/🟦️.ts";
import {
  causalOrderKeyV1,
  expectedGenerationFromArgsV1,
  INPUT_IDENTITY_SETTLE_BOUND_MS_V1,
  inputActionWindowV1,
  inputActionWithWindowV1,
  inputAppliedV1,
  inputAwaitsHubIdentityV1,
  inputCommitReceiptV1,
  inputDiagnosticV1,
  inputIdentitySettleNoticeTextV1,
  inputRefusalNoticeTextV1,
  inputRefusalNotifiesV1,
  inputRefusalTextV1,
  inputRefusedV1,
  resolveUtilityActivationV1,
  type InputCommitReceiptV1,
  type InputDiagnosticV1,
  type InputDraftDispositionV1,
  type InputOutcomeV1,
  type InputRefusalReasonV1,
  type ShellInputActionV1
} from "../🎯️input-ledger/🟦️.ts";
import {
  NOTE_WORLD_NAVIGATION_ACTION_ID,
  TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS,
  buildNoteShellCommandAction,
  type ShellLabelTextV1,
  undeclaredActionDiagnostic
} from "../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import {
  navbarExampleIdFromHistoryUpserts,
  rememberedExampleIdFromDispatchV1,
  frameworkOwnsExampleSwitch,
  historyRefreshNeededV1
} from "../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import {
  flattenPanelTabLeaves,
  panelAnchorForGroup,
  appFaultNoticeV1,
  historyFaultNoticeV1,
  historyInputDraftDispositionV1,
  historyOutputNoticeV1,
  historySilentRefusalOfFaultV1,
  historySilentRefusalOfOutputV1,
  frameworkLayoutDeclaredInstances
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { uiIntentToActionDescriptor } from "../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
// #endregion 🔌️Adapters
import { browserActorActionRefusalReasonV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } from "@semio-tech/framework/browser-bundle/action-handoff";
import { publishBrowserActorHostEffectsV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import type { ShellCoreAfterShellUriAndDocuments } from "../🗄️shell-uri-documents/🟦️.ts";
import type { RetainedBrowserActorUiV1 } from "../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";
import { runShellInput } from "./🧭️input-funnel/🟦️.ts";

export type ShellCoreAfterShellActions = ShellCoreAfterShellUriAndDocuments & ReturnType<typeof useShellActions>;

//#region FrameworkOsShell
export function useShellActions(core: ShellCoreAfterShellUriAndDocuments) {
  const {
    hostMode, mobile, defaults, dispatch, historyStore, exampleOptionsRef, lastDispatchedExampleIdRef,
    localHistoryOrderRef, loadedPlugins, pluginSupervisorById, session, uiLocale, uiTerminology, applyHistoryPatch,
    applyLeftoverInteractionView, refreshHistorySnapshot, hostApp, hostControllerId, hostCatalogueTabId, spacePrograms,
    syncCardKind, syncDraftPath, clipboardFragmentRef, extraWindowInstancesRef, awaitOperationSettle, sessionWorkRef,
    dropForSealedInstance, dropForRetiredInstance, treeWindowSchedulerRef, identityRef, verifiedSessionAuthorityRef,
    uiLocaleRef, uiTerminologyRef, showTransientNoticeRef, dispatchCheckpointRef, inferenceHistoryByRuntimeKeyRef,
    openDocumentSessionsRef, captureDialogOrigin, isCurrentDialogOrigin, captureEffectOwner, isCurrentEffectOwner,
    browserActorUiByRuntimeKeyRef, ensureBackboneWorker, primaryPluginId, reloadPlugin, uninstallPlugin, panel,
    spawnedIdsRef, historyCanUndo, activeUtilityByWindowIdRef, activeToolIdRef, inputLedgerRef, refusalNoticeThrottleRef,
    hubIdentityGateRef, hubIdentitySettleRef, writeUtilityRegister, clearAllWindowUtilities, writeToolRegister,
    activeWindowIdRef, startTutorialRef, toggleTutorialRecordingRef, tutorialDrivenRef, tutorialPlayingRef,
    tutorialRecordingRef, tutorialRecorderRef, shellStateRef, observeLocalInteraction, completeIntroductionInteraction,
    injectActiveUtility, updateSpacePanel, requestServiceOperation, directBrowserActorForSession,
    dispatchDirectBrowserActorCommand, applyHostEffects, attachSyncTarget, detachSyncBackbone, browseSyncBackbonePath,
    spawnProgram
  } = core;
  /** 🎯️ The shell's ONE input funnel. Every call mints a `🎯️input-ledger` entry first and settles it with a
   * typed outcome — `applied` for every branch that ran (shell-intercepted or guest-dispatched and settled),
   * `refused(reason)` for every gate that used to be a silent `return`. A refusal prints one plain
   * `console.warn` line and, for a user-origin input whose reason the notice table admits, one throttled
   * notice (§G). `propagateFailure` keeps rethrowing AFTER the ledger is settled (the tutorial director). */
  const onAction = useCallback(
    async (requested: ShellInputActionV1, submittedOrigin?: ShellDialogOriginV1, propagateFailure = false, lifecycle?: ActionDispatchLifecycleV1, requiredOperationOwner?: ReturnType<typeof captureEffectOwner>): Promise<InputOutcomeV1> => {
      const requestedWindowId = inputActionWindowV1(requested, activeWindowIdRef.current);
      const ledger = inputLedgerRef.current;
      const entry = ledger.issue(requested, { windowId: requestedWindowId ?? activeWindowIdRef.current ?? null, origin: requested.provenance?.origin ?? "user" });
      const refuse = (reason: InputRefusalReasonV1, detail?: string, alreadyNotified = false, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1): InputOutcomeV1 => {
        const outcome = inputRefusedV1(entry.provenance.inputSeq, reason, detail, diagnostic, draftDisposition);
        if (ledger.settle(outcome) && outcome.kind === "refused") {
          console.warn(inputRefusalTextV1(entry.action, outcome, entry.provenance));
          if (!alreadyNotified && inputRefusalNotifiesV1(outcome, entry.provenance) && refusalNoticeThrottleRef.current.admit(reason, performance.now())) {
            showTransientNoticeRef.current(inputRefusalNoticeTextV1(reason, uiLocaleRef.current), "info");
          }
        }
        return outcome;
      };
      const applied = (commit?: InputCommitReceiptV1): InputOutcomeV1 => {
        const outcome = inputAppliedV1(entry.provenance.inputSeq, commit);
        ledger.settle(outcome);
        return outcome;
      };
      /** 🚦️ The one place a thrown dispatch error becomes a reason — shared by the direct-actor route and
       * the `handleAction` route so both speak the ledger's vocabulary. */
      const refusalReasonForError = (error: unknown): { readonly reason: InputRefusalReasonV1; readonly detail?: string; readonly diagnostic?: InputDiagnosticV1; readonly draftDisposition?: InputDraftDispositionV1 } => {
        const diagnostic = error instanceof SemioFaultError ? inputDiagnosticV1(error.fault) : undefined;
        const draftDisposition = error instanceof SemioFaultError ? historyInputDraftDispositionV1(error.fault) : undefined;
        if (core.isViewerReadOnlyFault(error)) return { reason: "viewer-read-only", diagnostic, draftDisposition };
        if (core.isMutationRejectedFault(error)) return { reason: "mutation-rejected", diagnostic, draftDisposition };
        const text = String(error instanceof Error ? error.message : error);
        if (/queue is full|queue full|another action pending|action-busy/u.test(text)) return { reason: "queue-full", detail: text, diagnostic, draftDisposition };
        if (/action-catching-up/u.test(text)) return { reason: "catching-up", detail: text, diagnostic, draftDisposition };
        if (/action-owner-mismatch|owner retired|ambiguous document owner/u.test(text)) return { reason: "owner-mismatch", detail: text, diagnostic, draftDisposition };
        return { reason: "dispatch-failed", detail: text, diagnostic, draftDisposition };
      };
      try {
        return await runShellInput({ clipboardFragmentRef, primaryPluginId, dispatch, reloadPlugin, uninstallPlugin, session, hubIdentityGateRef, showTransientNoticeRef, uiLocaleRef, hubIdentitySettleRef, spawnedIdsRef, panel, loadedPlugins, extraWindowInstancesRef, captureDialogOrigin, isCurrentDialogOrigin, captureEffectOwner, startTutorialRef, toggleTutorialRecordingRef, tutorialPlayingRef, tutorialDrivenRef, tutorialRecordingRef, tutorialRecorderRef, completeIntroductionInteraction, activeWindowIdRef, activeUtilityByWindowIdRef, writeUtilityRegister, activeToolIdRef, writeToolRegister, uiTerminologyRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, clearAllWindowUtilities, dispatchCheckpointRef, browseSyncBackbonePath, syncCardKind, syncDraftPath, attachSyncTarget, detachSyncBackbone, hostMode, hostControllerId, spacePrograms, spawnProgram, hostApp, updateSpacePanel, mobile, inferenceHistoryByRuntimeKeyRef, openDocumentSessionsRef, historyCanUndo, localHistoryOrderRef, verifiedSessionAuthorityRef, ensureBackboneWorker, directBrowserActorForSession, dropForSealedInstance, uiLocale, uiTerminology, identityRef, treeWindowSchedulerRef, lastDispatchedExampleIdRef, dispatchDirectBrowserActorCommand, sessionWorkRef, historyStore, exampleOptionsRef, defaults, refreshHistorySnapshot, observeLocalInteraction, awaitOperationSettle, dropForRetiredInstance, requested, refuse, applied, entry, submittedOrigin, core, requiredOperationOwner, onAction, propagateFailure, refusalReasonForError, lifecycle });
      } catch (unexpected) {
        const outcome = refuse("dispatch-failed", `unexpected: ${String(unexpected instanceof Error ? unexpected.message : unexpected)}`, false, unexpected instanceof SemioFaultError ? inputDiagnosticV1(unexpected.fault) : undefined, unexpected instanceof SemioFaultError ? historyInputDraftDispositionV1(unexpected.fault) : undefined);
        if (propagateFailure) throw unexpected;
        return ledger.outcome(entry.provenance.inputSeq) ?? outcome;
      }
    },
    [
      applyHostEffects,
      dropForRetiredInstance,
      dropForSealedInstance,
      refreshHistorySnapshot,
      awaitOperationSettle,
      captureDialogOrigin,
      isCurrentDialogOrigin,
      captureEffectOwner,
      directBrowserActorForSession,
      dispatchDirectBrowserActorCommand,
      isCurrentEffectOwner,
      applyHistoryPatch,
      applyLeftoverInteractionView,
      attachSyncTarget,
      browseSyncBackbonePath,
      clearAllWindowUtilities,
      detachSyncBackbone,
      injectActiveUtility,
      loadedPlugins,
      observeLocalInteraction,
      panel,
      session,
      writeUtilityRegister,
      writeToolRegister,
      spawnProgram,
      hostMode,
      syncCardKind,
      syncDraftPath,
      updateSpacePanel,
      hostControllerId,
      hostApp,
      hostCatalogueTabId,
      historyCanUndo,
      ensureBackboneWorker,
      completeIntroductionInteraction,
      primaryPluginId,
      reloadPlugin,
      uninstallPlugin,
      pluginSupervisorById,
      spacePrograms,
      mobile,
      uiLocale,
    ],
  );

  /** 🧭️ Logs a shell-chrome command (theme change, dock drag, window resize, panel toggle, …) into the
   * plugin's session-only command-history panel — routed through the exact same `onAction` funnel as every
   * other action (see `NOTE_SHELL_COMMAND_ACTION_ID`) so it lands on `targetSession.instanceId` via the
   * standard `handleAction` call, just tagged with an id the plugin intercepts before the app sees it.
   * Answers the ledger outcome, so a caller that retires the target instance right after (closing a spawned
   * program's last window) can wait for the note to land first. No-ops when there's no active app session. */
  const noteShellCommand = useCallback(
    (commandId: string, label: ShellLabelTextV1, detail?: Record<string, unknown>): Promise<InputOutcomeV1 | null> => {
      if (!session) return Promise.resolve(null);
      return onAction(buildNoteShellCommandAction(session.app.controllerId, commandId, label, detail));
    },
    [session, onAction],
  );

  const onActionRef = useRef(onAction);
  core.onActionRef = onActionRef;
  useEffect(() => {
    onActionRef.current = onAction;
  }, [onAction]);

  const onActionStable = useCallback((action: Parameters<typeof onAction>[0], lifecycle?: ActionDispatchLifecycleV1): Promise<InputOutcomeV1> => onActionRef.current(action, undefined, false, lifecycle), []);
  const onIntentStable = useCallback((intent: Parameters<typeof uiIntentToActionDescriptor>[0], windowId?: string): Promise<InputOutcomeV1> => {
    const action = uiIntentToActionDescriptor(intent);
    return onActionStable(windowId === undefined ? action : inputActionWithWindowV1(action, windowId));
  }, [onActionStable]);
  const onBrowserActorIntent = useCallback(async (runtimeKey: string, captured: RetainedBrowserActorUiV1, surfaceKey: string, intent: Parameters<typeof uiIntentToActionDescriptor>[0]): Promise<InputOutcomeV1> => {
    const action = uiIntentToActionDescriptor(intent);
    const ledger = inputLedgerRef.current;
    const issued = ledger.issue(action, { windowId: surfaceKey, origin: "user" });
    const applied = (commit?: InputCommitReceiptV1): InputOutcomeV1 => {
      const outcome = inputAppliedV1(issued.provenance.inputSeq, commit);
      ledger.settle(outcome);
      return outcome;
    };
    const refused = (reason: InputRefusalReasonV1, detail?: string, diagnostic?: InputDiagnosticV1): InputOutcomeV1 => {
      const outcome = inputRefusedV1(issued.provenance.inputSeq, reason, detail, diagnostic);
      if (ledger.settle(outcome)) console.warn(inputRefusalTextV1(issued.action, outcome as Extract<InputOutcomeV1, { kind: "refused" }>, issued.provenance));
      return outcome;
    };
    const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    const store = retained?.windows.get(surfaceKey) ?? retained?.panels.get(surfaceKey);
    if (store === undefined || retained?.actions !== captured.actions || retained.identity === null || entry?.clientInstanceId !== captured.clientInstanceId || entry.session.instanceId !== captured.sessionInstanceId || !shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session)) return refused("owner-mismatch", "browser actor owner retired");
    const current = (): boolean => {
      const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      const opening = openDocumentSessionsRef.current.get(runtimeKey);
      return live?.actions === captured.actions && live.identity !== null && opening?.clientInstanceId === captured.clientInstanceId && opening.session.instanceId === captured.sessionInstanceId && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, opening.session);
    };
    try {
      const result = await retained.actions.dispatchIntent({
        scope: retained.scope,
        verifiedSurfaceId: retained.verifiedSurfaceId,
        activationGeneration: retained.activationGeneration,
        appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
        instanceId: retained.identity.instanceId,
        surfaceRevision: store.getRevisionSnapshot(),
      }, surfaceKey, intent);
      if (!current()) return refused("owner-mismatch", "browser actor owner retired after completion");
      for (const patch of result.historyPatches) if (current()) applyHistoryPatch(decodePackWire(Uint8Array.from(patch), "$.historyPatch") as HistoryPatch, false, { pluginId: entry.session.pluginId, instanceId: entry.session.instanceId });
      await publishBrowserActorHostEffectsV1(result.hostEffects, current, (effect) => {
        if ("requestServiceOperation" in effect) return requestServiceOperation(entry.session, effect.requestServiceOperation, current);
        else window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
      });
      if (!current()) return refused("owner-mismatch", "browser actor owner retired during publication");
      return applied(result.commit ?? undefined);
    } catch (error) {
      const reason = browserActorActionRefusalReasonV1(error) ?? "dispatch-failed";
      const detail = String(error instanceof Error ? error.message : error);
      if (browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.actions !== captured.actions) return refused("owner-mismatch", detail);
      core.showTransientNotice(shellLabel("ui.common.renderError"), "error");
      return refused(reason, detail, error instanceof SemioFaultError ? inputDiagnosticV1(error.fault) : undefined);
    }
  }, [applyHistoryPatch, requestServiceOperation]);

  return { onAction, noteShellCommand, onActionStable, onIntentStable, onBrowserActorIntent };
}
//#endregion FrameworkOsShell

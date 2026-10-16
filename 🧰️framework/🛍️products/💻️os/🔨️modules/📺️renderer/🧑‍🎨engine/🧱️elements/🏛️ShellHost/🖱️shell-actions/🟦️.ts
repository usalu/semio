/** 🖱️ `onAction`, shell command notes, the stable action and intent callbacks and the browser actor intent.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useShellActions` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellDialogSessionIsCurrentV1, type ShellDialogOriginV1 } from "../🗨️dialog-origin/🟦️.ts";
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { useCallback, useEffect, useRef } from "react";
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
import { encodeBackboneWorkerRequest, FRAMEWORK_SYNC_CONTROLLER_ID } from "@semio-tech/framework/backbone";
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
import { type ActiveSession, resolveBootExampleId, type SyncCardKind } from "../../🐚️Shell/🟦️.tsx";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { leftoverWorldSelectionOverlayV1, publishLeftoverWorldSelectionV1 } from "../../🌐️World3dHost/🟦️.tsx";
import {
  NOTE_WORLD_NAVIGATION_ACTION_ID,
  TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS,
  navbarExampleIdFromHistoryUpserts,
  rememberedExampleIdFromDispatchV1,
  buildNoteShellCommandAction,
  buildSpacePanelState,
  flattenPanelTabLeaves,
  introductionTargetsWindow,
  panelAnchorForGroup,
  EMPTY_SHELL_HISTORY_PROJECTION_V1,
  type ShellLabelTextV1,
  appFaultNoticeV1,
  historyFaultNoticeV1,
  historyInputDraftDispositionV1,
  historyOutputNoticeV1,
  historySilentRefusalOfFaultV1,
  historySilentRefusalOfOutputV1,
  parsePanelState,
  checkinSubmitMessageV1,
  FRAMEWORK_CHECKIN_CONTROLLER_ID,
  requestFileOpen,
  frameworkLayoutDeclaredInstances,
  pasteActionWithRetainedFragment,
  undeclaredActionDiagnostic,
  frameworkOwnsExampleSwitch,
  historyRefreshNeededV1,
  sessionWindowInstances,
  shellLabel,
  uiIntentToActionDescriptor,
  viewerReadOnlyNoticeText,
  abortExtensionRequestsForActor,
  isDeclaredSurfaceCancelAction
} from "../../🛠️ShellHelpers/🟦️.tsx";
// #endregion 🔌️Adapters
import { browserActorActionRefusalReasonV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } from "@semio-tech/framework/browser-bundle/action-handoff";
import { publishBrowserActorHostEffectsV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import { shellHistoryUndoRouteV1 } from "../🪪️host-bootstrap/🟦️.tsx";
import { syncAttachTargetV1 } from "../../🔄️ShellSync/🟦️.tsx";
import { startedTypedOperationV1, typedOperationCancellationActionV1, typedOperationCancellationOwnerIsCurrentV1 } from "../🏁️operation-settle/🟦️.ts";
import { encodeWindowActionInvocation, windowActionInvocation } from "../📨️window-invocation/🟦️.ts";
import { OBSERVED_INTERACTION_ACTION_IDS } from "../🎬️tutorial-recorder/🟦️.tsx";
import { panelDefinitionPath, requiredHostPanelLeafId } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterShellUriAndDocuments } from "../🗄️shell-uri-documents/🟦️.ts";
import type { RetainedBrowserActorUiV1 } from "../🔒️identity-document-ports/🟦️.ts";

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
        const action = pasteActionWithRetainedFragment(requested, clipboardFragmentRef.current);
        if (action.controllerId === "recovery") {
          const args = typeof action.args === "object" && action.args != null ? (action.args as { pluginId?: string }) : {};
          const pluginId = args.pluginId ?? primaryPluginId;
          if (!pluginId) return refuse("dispatch-failed", "recovery: no plugin id");
          if (action.action === "recovery.restartApp") {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
            void reloadPlugin(pluginId);
            return applied();
          }
          if (action.action === "recovery.disablePlugin") {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "quarantined" });
            if (pluginId !== primaryPluginId) void uninstallPlugin(pluginId);
            return applied();
          }
          if (action.action === "recovery.showDiagnostics") {
            return applied();
          }
        }

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
        // 🛑️ A cancel gesture retires the requesting instance's IN-FLIGHT extension work before the
        // gesture itself is forwarded, and never instead of it: the guest still owns its own
        // bookkeeping (its pending table, its progress ledger, its arming latch) and the extension
        // actor still has to be told through its own capability. This hop is only the third one —
        // the host-side door, which nothing inside the per-actor request queue can reach. The action
        // id is not known to this shell: a mounted surface DECLARES it off its own status contract
        // (`declareSurfaceCancelAction`), so the shell stays domain-neutral
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
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
        // 🎓️ First-run walkthrough (mirrors setActiveUtility below): fully shell-intercepted, resets
        // playback to the first step, never forwarded to the program.
        if (action.action === START_INTRODUCTION_ACTION_ID) {
          dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
          return applied();
        }

        // 🎥️ Fully shell-intercepted, mirroring `START_INTRODUCTION_ACTION_ID` above: sandboxes the
        // document and starts tutorial playback from t=0 (real work happens in `startTutorialRef`, wired up
        // by the TutorialOrchestration block further down this component).
        if (action.action === START_TUTORIAL_ACTION_ID) {
          const args = typeof action.args === "object" && action.args != null ? (action.args as { tutorialId?: unknown }) : {};
          if (typeof args.tutorialId === "string") startTutorialRef.current(args.tutorialId);
          return applied();
        }
        if (action.action === RECORD_TUTORIAL_ACTION_ID) {
          toggleTutorialRecordingRef.current();
          return applied();
        }

        // 🎥️ Deviation detection: any action NOT stamped by the tutorial director/seek/converge path while
        // a tutorial is actively playing means the user diverged from the recording — auto-pause and flag
        // `deviated` so pressing Play again converges instead of resuming blindly mid-drift.
        if (tutorialPlayingRef.current && !tutorialDrivenRef.current.active) {
          dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
          dispatch({ type: "SET_TUTORIAL_DEVIATED", value: true });
        }

        // ⏺️ Recorder tap: annotational-only capture (see `TutorialTracks.events` doc comment) — never
        // re-dispatched on playback. Skips navigation/introduction/tutorial-control actions (noise, or
        // meaningless to replay) and anything the director itself just dispatched.
        if (tutorialRecordingRef.current && !tutorialDrivenRef.current.active) {
          if (!TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS.has(action.action)) {
            tutorialRecorderRef.current?.recordEvent({ kind: "action", action: action.action, args: action.args as Record<string, unknown> | undefined });
          }
        }

        // 🧭️ Camera-navigation gesture report from a 3D window's `WorldOrbitGated` (shell-only, never
        // forwarded to the program) — completes any pan/zoom/orbit interaction of the active step that
        // targets the window the gesture happened on. Celebrates only `windowId`'s own pane (via
        // `windowElementId`, its unique per-instance element id) — never the whole window-kind alias
        // selector, which would celebrate every other open pane of that same kind too (e.g. a split view).
        if (action.action === NOTE_WORLD_NAVIGATION_ACTION_ID) {
          const args = typeof action.args === "object" && action.args != null ? (action.args as { windowId?: unknown; gestures?: unknown }) : {};
          const windowId = typeof args.windowId === "string" ? args.windowId : "";
          const gestures = Array.isArray(args.gestures) ? (args.gestures as readonly string[]) : [];
          if (windowId) {
            const windowKindId = sessionWindowInstances(session.app, extraWindowInstancesRef.current).find((instance) => instance.id === windowId)?.windowKindId ?? windowId;
            for (const gesture of gestures) {
              completeIntroductionInteraction(
                (interaction) => interaction.on.kind === gesture && introductionTargetsWindow(windowId, windowKindId, interaction.on.id),
                windowElementId(windowId),
              );
            }
          }
          return applied();
        }

        // 🧰️ Utility activation (P5): host-owned session state, never a document operation. Re-clicking the active
        // utility (or an empty utilityId) deactivates. We resolve the target window from the descriptor's tagged
        // `windowId` (see `tagSetActiveUtilityWindow`), falling back to the active window, compare-and-set the
        // window's register (`expectedGeneration` is the generation the pressing widget rendered — a press
        // against a register that moved since is refused `stale-generation`, the L3 replacement for the
        // deleted 8 s `lastUtilityArmAtRef` echo-off window), then forward the resolved utility to the plugin
        // so it can clear/prepare scratch.
        if (action.action === SET_ACTIVE_UTILITY_ACTION_ID) {
          const args = typeof action.args === "object" && action.args != null ? (action.args as { utilityId?: unknown; windowId?: unknown }) : {};
          const windowId = typeof args.windowId === "string" && args.windowId ? args.windowId : (activeWindowIdRef.current ?? "");
          if (!windowId) return refuse("view-state-unresolved", "no target window");
          const requestedUtility = typeof args.utilityId === "string" ? args.utilityId : "";
          const next = resolveUtilityActivationV1(activeUtilityByWindowIdRef.current[windowId], requestedUtility);
          const expected = expectedGenerationFromArgsV1(action.args);
          const write = writeUtilityRegister(windowId, next, expected);
          if (write.kind === "refused") return refuse("stale-generation", `window=${windowId} expected=${write.expected} current=${write.register.generation}`);
          // 🛠️ A tool and a window utility are mutually exclusive interaction owners — activating a real
          // utility clears any active mode-level tool.
          if (next && activeToolIdRef.current) writeToolRegister(null, null);
          if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "utility" && interaction.on.id === next);
          // 🧰️🪟️ "Active utility is per-window" means per-window of whichever PROGRAM owns that window.
          // A spawned foreign editor's utility bar forwarded to the landing app's instance, so arming a
          // drawing tool inside a spawned `draw` reached `s.space.home` and did nothing.
          const { spawnedId: utilitySpawnedId, session: utilitySession, extraInstances: utilityExtraInstances, guestWindowId: utilityGuestWindowId } = programOfWindow(windowId);
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === utilitySession.pluginId);
          const program = pluginEntry?.handle;
          if (program) {
            const viewState = windowViewContext(
              {
                ...utilitySession.viewState,
                locale: uiLocaleRef.current,
                terminology: uiTerminologyRef.current,
                activeToolId: next ? undefined : activeToolIdRef.current ?? undefined,
                activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, utilitySpawnedId, spawnedIdsRef.current),
                windowInstances: sessionWindowInstances(utilitySession.app, utilityExtraInstances).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
              },
              utilityGuestWindowId,
            );
            if (!viewState) return refuse("view-state-unresolved", `window=${windowId}`);
            const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { utilityId: next } };
            await program
              .handleAction(utilitySession.instanceId, encodeWindowActionInvocation({ ...utilitySession, viewState }, forwarded, utilityExtraInstances, utilityGuestWindowId), viewState, { order: causalOrderKeyV1(entry.provenance) })
              .then((response) => {
                applyHistoryPatch(response.historyPatch, false, { pluginId: utilitySession.pluginId, instanceId: utilitySession.instanceId });
                applyLeftoverInteractionView(response.output, windowId);
                if (!isCurrentEffectOwner(primaryActionOwner)) return;
                return applyHostEffects(response.requestedEffects ?? [], { ...utilitySession, viewState }, resolveUiDirtyScope(response.uiScope), primaryActionOwner);
              })
              .catch((utilityError) => undefined);
          }
          return applied();
        }

        // 🛠️ Tool activation: host-owned session state (mode-scoped, windowless), never a document operation.
        // Re-clicking the active tool (or an empty toolId) deactivates. Mutually exclusive with every
        // window's active utility — activating a tool clears them all, mirroring `SET_ACTIVE_UTILITY_ACTION_ID`.
        // 🔢️ Same compare-and-set over the one tool register.
        if (action.action === SET_ACTIVE_TOOL_ACTION_ID) {
          const args = typeof action.args === "object" && action.args != null ? (action.args as { toolId?: unknown }) : {};
          const requestedTool = typeof args.toolId === "string" ? args.toolId : "";
          const next = resolveUtilityActivationV1(activeToolIdRef.current, requestedTool);
          const expected = expectedGenerationFromArgsV1(action.args);
          const write = writeToolRegister(next, expected);
          if (write.kind === "refused") return refuse("stale-generation", `tool expected=${write.expected} current=${write.register.generation}`);
          if (next) clearAllWindowUtilities();
          const priorToolLeftover = leftoverWorldSelectionOverlayV1();
          // 🛠️ A mode-level tool is the one leftover authority that legitimately speaks for EVERY pane —
          // it just cleared every window's utility above, so the per-pane overlays go with it.
          publishLeftoverWorldSelectionV1(
            {
              ids: priorToolLeftover?.ids ?? [],
              hoveredId: priorToolLeftover?.hoveredId ?? null,
              hoveredDomain: priorToolLeftover?.hoveredDomain,
              gumballActive: priorToolLeftover?.gumballActive ?? false,
              gumballAnchorId: priorToolLeftover?.gumballAnchorId ?? null,
              activeUtility: next === "fill" ? "fill" : "select",
              activeToolId: next,
            },
            { kind: "allWindows" },
          );
          if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "tool" && interaction.on.id === next);
          const { spawnedId: toolSpawnedId, session: toolSession, extraInstances: toolExtraInstances, guestWindowId: toolGuestWindowId } = programOfWindow(activeWindowIdRef.current ?? "");
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === toolSession.pluginId);
          const program = pluginEntry?.handle;
          if (program) {
            const baseToolViewState: ViewModel = {
              ...toolSession.viewState,
              locale: uiLocaleRef.current,
              terminology: uiTerminologyRef.current,
              activeToolId: next ?? undefined,
              activeUtilityId: next ? undefined : toolSession.viewState.activeUtilityId,
              windowInstances: sessionWindowInstances(toolSession.app, toolExtraInstances).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
              activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, toolSpawnedId, spawnedIdsRef.current),
            };
            const viewState = toolGuestWindowId ? windowViewContext(baseToolViewState, toolGuestWindowId) : panelViewContext(baseToolViewState);
            if (!viewState) return refuse("view-state-unresolved", `tool window=${toolGuestWindowId}`);
            const forwarded: ActionDescriptor = { controllerId: toolSession.app.controllerId, action: action.action, args: { toolId: next } };
            await program
              .handleAction(toolSession.instanceId, encodeWindowActionInvocation({ ...toolSession, viewState }, forwarded, toolExtraInstances, toolGuestWindowId || undefined), viewState, { order: causalOrderKeyV1(entry.provenance) })
              .then((response) => {
                applyHistoryPatch(response.historyPatch, false, { pluginId: toolSession.pluginId, instanceId: toolSession.instanceId });
                applyLeftoverInteractionView(response.output);
                if (!isCurrentEffectOwner(primaryActionOwner)) return;
                return applyHostEffects(response.requestedEffects ?? [], { ...toolSession, viewState }, { kind: "full" }, primaryActionOwner);
              })
              .catch((toolError) => undefined);
          }
          return applied();
        }

        completeIntroductionInteraction((interaction) => interaction.on.kind === "action" && interaction.on.id === action.action);

        if (action.controllerId === FRAMEWORK_CHECKIN_CONTROLLER_ID) {
          if (action.action !== "submit") return refuse("undeclared-action", `check-in: ${action.action}`);
          dispatchCheckpointRef.current(checkinSubmitMessageV1(action.args));
          return applied();
        }

        if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
          if (action.action === "selectFile") {
            dispatch({ type: "SET_SYNC_CARD_KIND", value: "file" });
            dispatch({ type: "SET_SYNC_DRAFT_PATH", value: "" });
            void browseSyncBackbonePath("file");
            return applied();
          }
          if (action.action === "selectFolder") {
            dispatch({ type: "SET_SYNC_CARD_KIND", value: "folder" });
            dispatch({ type: "SET_SYNC_DRAFT_PATH", value: "" });
            void browseSyncBackbonePath("folder");
            return applied();
          }
          if (action.action === "browse") {
            if (syncCardKind === "folder") void browseSyncBackbonePath("folder");
            else if (syncCardKind === "file") void browseSyncBackbonePath("file");
            return applied();
          }
          if (action.action === "selectRemote") {
            dispatch({ type: "SET_SYNC_CARD_KIND", value: "remote" });
            dispatch({ type: "SET_SYNC_DRAFT_PATH", value: "" });
            return applied();
          }
          if (action.action === "attach") {
            const args = typeof action.args === "object" && action.args != null ? (action.args as { readonly path?: unknown; readonly kind?: unknown }) : {};
            const kind: SyncCardKind = args.kind === "remote" || args.kind === "folder" ? args.kind : "file";
            const target = syncAttachTargetV1(kind, "path" in args ? String(args.path ?? "") : syncDraftPath);
            if (target === null) return refuse("dispatch-failed", "sync attach: empty path");
            void attachSyncTarget(target);
            return applied();
          }
          if (action.action === "detach") {
            void detachSyncBackbone();
            return applied();
          }
          return refuse("undeclared-action", `sync card: ${action.action}`);
        }

        if (hostMode && action.action === "spawnApp" && action.controllerId !== hostControllerId) {
          const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String((action.args as { pluginId?: string }).pluginId ?? "") : "";
          const program = spacePrograms.find((entry) => entry.pluginId === pluginId);
          if (!program) return refuse("undeclared-action", `spawnApp: no program "${pluginId}"`);
          void spawnProgram(program);
          return applied();
        }

        if (hostMode && action.action === "setActivePanelTab" && (action.controllerId === hostControllerId || action.controllerId === session.app.controllerId)) {
          const tabId = typeof action.args === "object" && action.args != null && typeof (action.args as { tabId?: unknown }).tabId === "string" ? (action.args as { tabId: string }).tabId : "";
          const targetApp = action.controllerId === session.app.controllerId ? session.app : hostApp;
          if (!targetApp || tabId.length === 0 || Array.from(tabId).length > 256 || /[\0-]/u.test(tabId)) return refuse("dispatch-failed", "setActivePanelTab: invalid tab id");
          const leaf = flattenPanelTabLeaves(targetApp.panelTabs).find((tab) => panelTabKindId(tab.kind) === tabId);
          const path = panelDefinitionPath(targetApp.panelTabs, tabId);
          if (!leaf || !path) return refuse("undeclared-action", `setActivePanelTab: unknown tab "${tabId}"`);
          const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
          updateSpacePanel(buildSpacePanelState(currentPanel.spawnedApps, tabId, currentPanel.activeSpawnedId));
          if (mobile) {
            dispatch({ type: "SET_MOBILE_PANEL_PATH", value: path });
            dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
          } else {
            const anchor = panelAnchorForGroup(leaf.group);
            dispatch({ type: "SET_PANEL_PATH", anchor, value: path });
            dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: true });
          }
          return applied();
        }

        /** 🪟️ Which spawned instance this dispatch belongs to — `null` for the session's own app. It
         * decides BOTH the target instance and the host↔guest window-id translation below. */
        let targetSpawnedId: string | null = null;
        let targetSession =
          hostMode && action.controllerId !== session.app.controllerId
            ? (() => {
                const spawned = panel?.spawnedApps.find((entry) => {
                  const app = loadedPlugins.find((p) => p.handle.pluginId === entry.pluginId)?.manifest.apps.find((a) => a.id === entry.appId);
                  return app?.controllerId === action.controllerId;
                });
                if (!spawned) return session;
                const app = loadedPlugins.find((p) => p.handle.pluginId === spawned.pluginId)?.manifest.apps.find((a) => a.id === spawned.appId);
                if (!app) return session;
                targetSpawnedId = spawned.id;
                return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) };
              })()
            : session;
        /** 🪟️ The window id in the SHELL's namespace — what the canvas, the utility map and the
         * leftover-overlay lane are keyed by. */
        const hostWindowId = inputActionWindowV1(action, activeWindowIdRef.current) ?? undefined;
        /**
         * 🪟️ A WINDOW-scoped dispatch belongs to the program that owns that window, whatever
         * controller raised it. Shell chrome (the navbar's history verbs, a keybinding, a panel row)
         * raises actions with the SESSION's controller id while the active window belongs to a
         * spawned program — measured on the live `s` host as
         * `noteShellCommand refused: view-state-unresolved (user window=space-2::s-workflow)`, because
         * the host app has no window by that name. This is the "panel actions dispatch in the ACTIVE
         * window" rule, stated once for every route below instead of per call site.
         */
        if (hostMode && hostWindowId !== undefined) {
          const windowOwnerId = spawnedIdOfWindowInstanceV1(hostWindowId, spawnedIdsRef.current);
          if (windowOwnerId !== null && windowOwnerId !== targetSpawnedId) {
            const owner = panel?.spawnedApps.find((entry) => entry.id === windowOwnerId);
            const ownerApp = owner ? loadedPlugins.find((entry) => entry.handle.pluginId === owner.pluginId)?.manifest.apps.find((candidate) => candidate.id === owner.appId) : undefined;
            if (owner && ownerApp) {
              targetSpawnedId = owner.id;
              targetSession = { pluginId: owner.pluginId, instanceId: owner.instanceId, app: ownerApp, viewState: spawnedProgramViewStateV1(session.viewState, ownerApp) };
            }
          }
        }
        if (action.action === "undo") {
          const mounted = Object.entries(inferenceHistoryByRuntimeKeyRef.current).filter(([runtimeKey, history]) => {
            const document = openDocumentSessionsRef.current.get(runtimeKey);
            return document?.session.pluginId === targetSession.pluginId && document.session.instanceId === targetSession.instanceId && document.clientInstanceId === history.clientInstanceId && history.sessionInstanceId === targetSession.instanceId;
          });
          const remote = mounted.length === 1 ? mounted[0] : undefined;
          const route = shellHistoryUndoRouteV1(remote?.[1] === undefined ? null : { ...remote[1].status, order: remote[1].order }, { canUndo: historyCanUndo, order: localHistoryOrderRef.current });
          if (route === "blocked") return refuse("dispatch-failed", "undo route blocked: nothing to undo");
          if (route === "remote") {
            const history = remote![1];
            if (!directorySessionAuthorityIsCurrentV1(history.authority, verifiedSessionAuthorityRef.current)) return refuse("owner-mismatch", "undo: directory session authority is not current");
            ensureBackboneWorker().postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: history.owner, serviceId: history.serviceId, action: "undo", operationEpoch: history.historyEpoch, payload: { historyEpoch: history.historyEpoch, clientInstanceId: history.clientInstanceId, scope: history.scope } }) });
            return applied();
          }
        }
        // ⏪️ Reserved history verbs act on the DOCUMENT owner, not the app-chrome session the navbar/
        // keybinding dispatch carries — the retained browser actor is keyed to the open document session,
        // so without this remap `directBrowserActorForSession` misses and the dispatch dies on the retired
        // `plugin.handleAction` path (ticket 26/09/02/PUZZLE-3D-END-TO-END).
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
        // 🪦️ A control that outlived its session (a long-lived canvas host's own callback, a queued
        // pointer gesture) still addresses the instance a switch sealed. Dropping here is what turns the
        // twenty `no actor for instance N` stacks the 6018 journey logged after one role chord into one
        // typed line per late dispatch.
        if (dropForSealedInstance(targetSession, "action", action.action)) return refuse("instance-sealed", `${targetSession.pluginId}#${targetSession.instanceId}`);
        const actionOwner = captureEffectOwner(targetSession, actionOrigin, entry.provenance.inputSeq);
        if (!isCurrentEffectOwner(actionOwner)) {
          return refuse("owner-mismatch", "effect owner is not current");
        }

        // 🚫️ The old `setDocument` → `patchAppSource` mirror (spawned-instance content write-back on the
        // os document) is deleted — app content no longer embeds on the os document at all
        // (`OsAppInstance.document` is now just an `OsDocumentRef` handle). A spawned instance's content
        // sync now goes through its own `openDocument`-opened `DocumentHost` channel, same as any other
        // document; there is no host-side JS mirroring step anymore.
        // 🪟️ `windowId` is read back off the tagged `action.args` (see `windowMeasuresChrome`/`tagSetActiveUtilityWindow`),
        // falling back to the active window — stamped into the dispatched view state so the plugin can key any
        // per-window option mutation off `view_state.windowId` instead of ever guessing at the active window.
        /** 🪟️ The window id the GUEST knows. A spawned instance owns exactly one set of windows, so
         * its instance ids are its window KIND ids; the `${spawnedId}::` namespace is the shell's.
         * Without this strip the guest is addressed at a window it never declared and
         * `undeclaredActionDiagnostic` drops the action — the shape that made every foreign-kind verb
         * unreachable inside `s` (`🪟️spawned-program`). */
        const dispatchWindowId = guestWindowIdV1(hostWindowId, spawnedIdsRef.current);
        const dispatchExtraInstances = targetSpawnedId === null ? extraWindowInstancesRef.current : frameworkLayoutDeclaredInstances(targetSession.app.defaultLayout, targetSession.app.windowKinds);
        // 🧹️ `clearSelection` retires the host's own leftover overlay on the SAME turn it is dispatched.
        // The guest's half of the clear is exact — measured on 6018, the clearing turn's
        // `turn-result.presence` carries the cleared mark for the row that lost the selection — but the
        // leftover overlay is a HOST lane that `treeItemToTreeData` ORs into `isSelected`
        // (`leftoverTreeItemSelectedV1`), and it is only ever rewritten by a leftover `interactionView` a
        // later gesture publishes. So the row the guest had just retired kept `aria-selected="true"` until
        // the next pick, which is exactly the one-keystroke-wide window
        // `📓️window-gaps-followup-2026-09-14.md` G1 measured.
        if (action.action === CLEAR_SELECTION_ACTION_ID) applyLeftoverInteractionView({ interactionView: { selectedIds: [], selectionCleared: true } }, hostWindowId);
        const baseDispatchViewState: ViewModel = {
          ...targetSession.viewState,
          locale: uiLocale,
          terminology: uiTerminology,
          // 🪪️ The same stamp as the refresh pass (`runUiRefreshPass`'s `viewState`) and
          // `resolvedTargetViewState`: a dispatch carries the signed-in human, because a verb that
          // names the owner cannot be resolved without one. Measured live on serve 6190 → hub 7611
          // (ticket 26/09/18, S9 §2): Home's own Actions rail answered `createStudio refused:
          // dispatch-failed (user window=s-home-main) — retained command reducer rejected operation:
          // app.message s.home.session-identity-required` on a shell that had been signed in for 90 s,
          // because this projection is rebuilt per call from refs and never carried the identity.
          sessionIdentity: identityRef.current ? { userId: identityRef.current.userId, displayName: identityRef.current.displayName } : undefined,
          windowInstances: sessionWindowInstances(targetSession.app, dispatchExtraInstances).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
          activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, targetSpawnedId, spawnedIdsRef.current),
          focusedWindowId: guestWindowIdV1(activeWindowIdRef.current, spawnedIdsRef.current),
          // 🪟️ See the identical spread in `runUiRefreshPass` — an action's own render must materialise
          // the same tree windows the last refresh did, or a pick dispatched on a streamed row lands on
          // a body the guest rebuilt at a different offset.
          ...treeWindowSchedulerRef.current!.viewStateFields(),
        };
        const dispatchViewState = hostArmedViewContext(baseDispatchViewState, activeToolIdRef.current, dispatchWindowId);
        if (!dispatchViewState) {
          return refuse("view-state-unresolved", `window=${dispatchWindowId ?? ""}`);
        }
        // 🚨️ Undeclared-action drop — ALWAYS visible, never `[TRACE]`/diagnostics-gated: this is the one
        // place a fully wired binding dies without a fault reaching anyone, so it names the app, the action
        // and the dispatching window kind (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        const undeclared = undeclaredActionDiagnostic(targetSession.app.id, action.action, targetSession.app.windowKinds, (baseDispatchViewState.windowInstances ?? []).find((instance) => instance.id === dispatchWindowId)?.windowKindId ?? null, targetSession.app.actions ?? []);
        if (undeclared && !frameworkOwnsExampleSwitch(targetSession.app.role, action.action)) {
          console.error(undeclared.message, undeclared);
          return refuse("undeclared-action", undeclared.message);
        }
        // 👁️✏️ Client-side half of the read-only guarantee (contract freeze §2.3/§5) — the SDK-side
        // `VcsArtifactApp` guard is the source of truth (a `ArtifactViewer`-declared session can never
        // even construct a `Mutation`-kind action), this just avoids a pointless round trip and shows the
        // same notice a `"viewer.read-only"` fault reply gets in the `.catch` below. `showTransientNotice`/
        // `isViewerReadOnlyFault` are deliberately NOT in this callback's dep list below — both are stable
        // across renders (refs + `dispatch` only), and are declared later in this component, so adding
        // them would read a not-yet-initialized `const` on the render that first creates this callback.
        if (targetSession.app.role === "viewer" && targetSession.app.windowKinds.some((kind) => resolveWindowActions(targetSession.app, kind).some((entry) => entry.id === action.action && entry.kind === "mutation"))) {
          core.showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
          return refuse("viewer-read-only", undefined, true);
        }

        // 🎨️ EVERY route that loads an example records which one, not only `NavbarExampleSelect`'s own
        // `onValueChange`. `navbarExampleIdFromHistoryUpserts` restores this id when a `Set Active Example`
        // row comes back live, i.e. on REDO — and a row can be redone that this shell never dispatched
        // through the picker (a palette entry, a context-menu row, a replayed shell command, the boot
        // load). Wave B26 measured `navbar example from history {"navbarExample":"concrete-forest",
        // "remembered":""}` on :6013: undo relabelled the picker (a popped row needs no memory, it falls
        // back to the boot example) while redo silently could not, because nothing outside the picker had
        // ever written the id down.
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
        if (directBrowserActor !== null) {
          // 📮️ The hub-mounted browser-actor route: a rejection (`another action pending`, `action-busy`,
          // `action-owner-mismatch`, a closed mailbox) is a typed ledger refusal now, not a `[TRACE]` error
          // plus a "render error" toast (design §0 row 2). The mailbox names its own reasons
          // (`browserActorActionRefusalReasonV1`); a fault it does not know falls back to the shared mapper.
          try {
            const result = await dispatchDirectBrowserActorCommand(
              directBrowserActor,
              windowActionInvocation({ ...targetSession, viewState: dispatchViewState }, action, dispatchExtraInstances, dispatchWindowId),
              dispatchViewState,
            );
            return applied(result.commit ?? undefined);
          } catch (actionError) {
            const mailboxReason = browserActorActionRefusalReasonV1(actionError);
            const mapped = refusalReasonForError(actionError);
            const { reason, detail, diagnostic, draftDisposition } = mailboxReason === null ? mapped : { ...mapped, reason: mailboxReason, detail: String(actionError instanceof Error ? actionError.message : actionError) };
            if (reason === "mutation-rejected") core.showMutationRejectedNotice((actionError as SemioFaultError).fault);
            const appNotice = reason === "dispatch-failed" && actionError instanceof SemioFaultError ? appFaultNoticeV1(actionError.fault, targetSession.app, uiTerminologyRef.current) : null;
            if (appNotice !== null) core.showTransientNotice(appNotice.text, appNotice.kind, appNotice.code);
            const outcome = refuse(reason, appNotice?.code ?? detail, reason === "mutation-rejected" || appNotice !== null, diagnostic, draftDisposition);
            if (propagateFailure) throw actionError;
            return outcome;
          }
        }
        // ⏳️ The whole round trip — admitting turn, host-effect pass and the `OperationCompleted` frame
        // `awaitOperationSettle` waits for — is what a switch has to outlive, so the ledger entry spans
        // the entire chain, not just `handleAction`'s own promise.
        const releaseActionWork = sessionWorkRef.current.begin(targetSession.pluginId, targetSession.instanceId, "typed-operation");
        try {
          const response = await plugin.handleAction(targetSession.instanceId, encodeWindowActionInvocation({ ...targetSession, viewState: dispatchViewState }, action, dispatchExtraInstances, dispatchWindowId), dispatchViewState, { order: causalOrderKeyV1(entry.provenance) });
          const startedOperation = startedTypedOperationV1(response.output);
          if (startedOperation !== undefined && isCurrentEffectOwner(actionOwner)) lifecycle?.started({
            operationId: startedOperation.operationId,
            generation: startedOperation.generation,
            cancel: () => onAction(typedOperationCancellationActionV1(action, startedOperation), actionOrigin!, false, undefined, actionOwner),
          });
          applyHistoryPatch(response.historyPatch, false, { pluginId: targetSession.pluginId, instanceId: targetSession.instanceId });
          applyLeftoverInteractionView(response.output, hostWindowId);
          const rejectedNotice = historyOutputNoticeV1(response.output, programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(targetSession), EMPTY_SHELL_HISTORY_PROJECTION_V1).editCount);
          if (rejectedNotice !== null) core.showTransientNotice(rejectedNotice.text, rejectedNotice.kind, rejectedNotice.code);
          const ignoredRefusal = historySilentRefusalOfOutputV1(response.output);
          if (ignoredRefusal !== null) console.debug(`[os-shell] ${action.action} was ignored: ${ignoredRefusal}`);
          const navbarExample = navbarExampleIdFromHistoryUpserts(response.historyPatch?.upserts, lastDispatchedExampleIdRef.current, resolveBootExampleId("", exampleOptionsRef.current, defaults.exampleId));
          if (navbarExample !== undefined) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: navbarExample });
          const needsHistoryRefresh = historyRefreshNeededV1(action.action, response.historyPatch);
          if (!isCurrentEffectOwner(actionOwner)) {
            if (needsHistoryRefresh) refreshHistorySnapshot({ pluginId: targetSession.pluginId, instanceId: targetSession.instanceId });
            return applied();
          }
          await applyHostEffects(response.requestedEffects ?? [], { ...targetSession, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope), actionOwner);
          if (OBSERVED_INTERACTION_ACTION_IDS.has(action.action)) observeLocalInteraction({ plugin, instanceId: targetSession.instanceId });
          // 🏁️ `handleAction` answers on the guest's FIRST reactor turn — a typed command is only ADMITTED
          // there, its work runs on later turns and lands as an `OperationCompleted` frame. Awaiting that
          // frame here is what makes this promise mean "the action finished", which every self-gating
          // background tick loop depends on (`ComponentSceneHostProps.onAction`). The ledger's own
          // `settled(inputSeq)` is the input-level twin (design §H).
          const completion = await awaitOperationSettle(targetSession.instanceId, startedOperation?.operation);
          if (needsHistoryRefresh) refreshHistorySnapshot({ pluginId: targetSession.pluginId, instanceId: targetSession.instanceId });
          return applied(completion === null ? undefined : inputCommitReceiptV1(completion.operation, completion.revision));
        } catch (actionError) {
          let outcome: InputOutcomeV1;
          const diagnostic = actionError instanceof SemioFaultError ? inputDiagnosticV1(actionError.fault) : undefined;
          const historyNotice = actionError instanceof SemioFaultError ? historyFaultNoticeV1(actionError.fault, programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(targetSession), EMPTY_SHELL_HISTORY_PROJECTION_V1).editCount) : null;
          const silentRefusal = actionError instanceof SemioFaultError ? historySilentRefusalOfFaultV1(actionError.fault) : null;
          const draftDisposition = actionError instanceof SemioFaultError ? historyInputDraftDispositionV1(actionError.fault) : undefined;
          const appNotice = actionError instanceof SemioFaultError ? appFaultNoticeV1(actionError.fault, targetSession.app, uiTerminologyRef.current) : null;
          if (core.isViewerReadOnlyFault(actionError)) {
            core.showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
            outcome = refuse("viewer-read-only", undefined, true, diagnostic, draftDisposition);
          } else if (core.isMutationRejectedFault(actionError)) {
            core.showMutationRejectedNotice((actionError as SemioFaultError).fault);
            outcome = refuse("mutation-rejected", undefined, true, diagnostic, draftDisposition);
          } else if (silentRefusal !== null) {
            outcome = refuse("dispatch-failed", silentRefusal, true, diagnostic, draftDisposition);
          } else if (historyNotice !== null) {
            core.showTransientNotice(historyNotice.text, historyNotice.kind, historyNotice.code);
            outcome = refuse("dispatch-failed", historyNotice.code, true, diagnostic, draftDisposition);
          } else if (appNotice !== null) {
            core.showTransientNotice(appNotice.text, appNotice.kind, appNotice.code);
            outcome = refuse("dispatch-failed", appNotice.code, true, diagnostic);
          } else if (dropForSealedInstance(targetSession, "action failure", action.action)) {
            outcome = refuse("instance-sealed", `${targetSession.pluginId}#${targetSession.instanceId}`);
          } else if (dropForRetiredInstance(targetSession, `action ${action.action}`, actionError)) {
            outcome = refuse("instance-retired", `${targetSession.pluginId}#${targetSession.instanceId}`);
          } else {
            const { reason, detail, diagnostic, draftDisposition } = refusalReasonForError(actionError);
            outcome = refuse(reason, detail, false, diagnostic, draftDisposition);
          }
          if (propagateFailure) throw actionError;
          return outcome;
        } finally {
          releaseActionWork();
        }
      } catch (unexpected) {
        // 🧯️ L1: an input never dangles — a throw anywhere above that no branch mapped still settles the
        // entry (`settle` is idempotent, so an already-settled entry keeps its first outcome).
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

  // 🐢️ `onAction`'s own identity churns every action (its deps include `session`, `panel`, …). Render
  // trees built from `UiNode`s only need a *callable* action dispatcher, not a fresh one each time —
  // route them through this permanently-stable ref indirection so `interpretUiNode`'s `React.memo`
  // (and any `useMemo` keyed on the dispatcher passed to it) can actually bail.
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

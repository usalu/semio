/** 🎯️ The program that owns a dispatch, the remote undo route and the dispatched view state of the input funnel.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | useShellActions}: each function is one stage of the shell input funnel (`onAction`), moved verbatim. */
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { CLEAR_SELECTION_ACTION_ID } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { spawnedProgramViewStateV1, guestActiveUtilityByWindowIdV1, guestWindowIdV1, spawnedIdOfWindowInstanceV1 } from "../../../🪟️spawned-program/🟦️.ts";
import { inputActionWindowV1, type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../../🎯️input-ledger/🟦️.ts";
import { type ActiveSession } from "../../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { frameworkLayoutDeclaredInstances } from "../../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { sessionWindowInstances } from "../../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { shellHistoryUndoRouteV1 } from "../../../🪪️host-bootstrap/🟦️.tsx";
import type { ShellCoreAfterShellUriAndDocuments } from "../../../🗄️shell-uri-documents/🟦️.ts";

/** 🧲️ Resolves the spawned program that owns a dispatch from its controller and its window. */
export function resolveDispatchTarget({ hostMode, panel, loadedPlugins, activeWindowIdRef, spawnedIdsRef, session, action }: Pick<ShellCoreAfterShellUriAndDocuments, "hostMode" | "panel" | "loadedPlugins" | "activeWindowIdRef" | "spawnedIdsRef"> & {
  readonly session: ActiveSession;
  readonly action: ShellInputActionV1;
}) {
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
  /** 🧷️ The window id in the SHELL's namespace — what the canvas, the utility map and the
   * leftover-overlay lane are keyed by. */
  const hostWindowId = inputActionWindowV1(action, activeWindowIdRef.current) ?? undefined;
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
  return { targetSession, hostWindowId, targetSpawnedId };
}

/** ⏪️ Sends an undo to the mounted remote history owner when the undo route is remote. */
export function routeRemoteUndo({ inferenceHistoryByRuntimeKeyRef, openDocumentSessionsRef, historyCanUndo, localHistoryOrderRef, verifiedSessionAuthorityRef, ensureBackboneWorker, action, targetSession, refuse, applied }: Pick<ShellCoreAfterShellUriAndDocuments, "inferenceHistoryByRuntimeKeyRef" | "openDocumentSessionsRef" | "historyCanUndo" | "localHistoryOrderRef" | "verifiedSessionAuthorityRef" | "ensureBackboneWorker"> & {
  readonly action: ShellInputActionV1;
  readonly targetSession: ActiveSession;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}) {
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
}

/** 🧮️ Builds the guest window id, the declared instances and the view state projection a dispatch carries. */
export function buildDispatchViewState({ spawnedIdsRef, extraWindowInstancesRef, applyLeftoverInteractionView, uiLocale, uiTerminology, identityRef, activeUtilityByWindowIdRef, activeWindowIdRef, treeWindowSchedulerRef, hostWindowId, targetSpawnedId, targetSession, action }: Pick<ShellCoreAfterShellUriAndDocuments, "spawnedIdsRef" | "extraWindowInstancesRef" | "applyLeftoverInteractionView" | "uiLocale" | "uiTerminology" | "identityRef" | "activeUtilityByWindowIdRef" | "activeWindowIdRef" | "treeWindowSchedulerRef"> & {
  readonly hostWindowId: string | undefined;
  readonly targetSpawnedId: string | null;
  readonly targetSession: ActiveSession;
  readonly action: ShellInputActionV1;
}) {
  /** 🔗️ The window id the GUEST knows. A spawned instance owns exactly one set of windows, so
   * its instance ids are its window KIND ids; the `${spawnedId}::` namespace is the shell's.
   * Without this strip the guest is addressed at a window it never declared and
   * `undeclaredActionDiagnostic` drops the action — the shape that made every foreign-kind verb
   * unreachable inside `s` (`🪟️spawned-program`). */
  const dispatchWindowId = guestWindowIdV1(hostWindowId, spawnedIdsRef.current);
  const dispatchExtraInstances = targetSpawnedId === null ? extraWindowInstancesRef.current : frameworkLayoutDeclaredInstances(targetSession.app.defaultLayout, targetSession.app.windowKinds);
  if (action.action === CLEAR_SELECTION_ACTION_ID) applyLeftoverInteractionView({ interactionView: { selectedIds: [], selectionCleared: true } }, hostWindowId);
  const baseDispatchViewState: ViewModel = {
    ...targetSession.viewState,
    locale: uiLocale,
    terminology: uiTerminology,
    sessionIdentity: identityRef.current ? { userId: identityRef.current.userId, displayName: identityRef.current.displayName } : undefined,
    windowInstances: sessionWindowInstances(targetSession.app, dispatchExtraInstances).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
    activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, targetSpawnedId, spawnedIdsRef.current),
    focusedWindowId: guestWindowIdV1(activeWindowIdRef.current, spawnedIdsRef.current),
    ...treeWindowSchedulerRef.current!.viewStateFields(),
  };
  return { baseDispatchViewState, dispatchWindowId, dispatchExtraInstances };
}

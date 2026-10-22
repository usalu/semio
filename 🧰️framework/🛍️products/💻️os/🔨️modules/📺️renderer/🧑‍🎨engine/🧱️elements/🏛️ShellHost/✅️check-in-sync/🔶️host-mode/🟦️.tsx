/** 🔶️ The host mode phase of useCheckInAndSync. */
import { createAdmittedShellInstanceV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { isTerminalDocumentCheckInPhaseV1, type DocumentCheckInStatusV1 } from "@semio-tech/framework/document/check-in";
import { useCallback, useEffect, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { type DocumentScope } from "@semio-tech/framework/document/scope";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { type ScopedPresencePeerV1 } from "../../👥️presence-scope/🟦️.ts";
import { SHELL_HUB_ROUTE, shellRouteAdmissionTextV1, shellRouteAdmissionV1 } from "../../🔀️surface-switch/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { historyRefusalNoticeV1 } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { automaticCheckinWaitsV1, canCheckIn, checkpointGateV1, computeSyncPillState, type SyncPillState } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { timeTravelPeerPresenceV1, timeTravelRowActionRefusalsV1 } from "../../../🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { type HubLinkV1, type HubSessionPresenceV1 } from "../../../🔄️ShellSync/🟦️.tsx";
import { encodeAppCommandInvocation } from "../../📨️window-invocation/🟦️.ts";
import { findSpaceIndexEditor, mintDirectoryCommandRequestId, selectedSpaceArtifactCreationCatalogV1, SPACE_INDEX_DOCUMENT_ID } from "../../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterDocumentTransferAndTabs } from "../../📤️document-transfer-tabs/🟦️.tsx";
import type { BackgroundSpaceIndexSession } from "../../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";

/** 🔷 Builds hostMode, shellState, dispatch, historyStore of useCheckInAndSync. */
export function useHostModeState({ core }: { readonly core: ShellCoreAfterDocumentTransferAndTabs }) {
  const {
    hostMode, shellState, dispatch, historyStore, loadedPlugins, session, uiLocale, uiTerminology, appLabelsOverlay,
    syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId, openSpaceIdRef, panelTreeConfigCacheRef,
    treeWindowGeneration, treeWindowHost, backboneWorkerRef, identity, identityRef, identityOffline, hubSessionRefused,
    verifiedSessionAuthority, hubEnv, hubSessionCapability, identityResolution, showTransientNoticeRef,
    dispatchCheckpointRef, backgroundSpaceIndexSessionsRef, openDocumentRef, closeDocumentRef, retireDocumentAttachment,
    spaceArtifactCreationCatalog, spaceArtifactCreationCatalogUi, setHubWorkspaceOpen, extensionFetchAbortRef,
    openDocumentSessionsRef, captureDialogOrigin, presencePeersByRuntimeKey, loadedPluginsRef, navigateHistory,
    sessionRoute, focusedApp, focusedPanelBodyStores, focusedHistoryV1, historyCheckpointId, focusedTimeTravel,
    focusedHistoryEntries, attachSyncTarget, detachSyncBackbone, browseSyncBackbonePath, onAction,
    currentDocumentRuntimeKey, focusedBrowserActorPanels
  } = core;
  //#region FrameworkOsShell
  //#region 🔖️CheckIn — ticket §C5 "when the user edits an artifact, the mutations are saved and
  const currentDocumentId = currentDocumentRuntimeKey === null ? null : (openDocumentSessionsRef.current.get(currentDocumentRuntimeKey)?.documentId ?? null);
  const selectedSpaceArtifactCreationCatalog = useMemo(() => selectedSpaceArtifactCreationCatalogV1(
    spaceArtifactCreationCatalog,
    spaceArtifactCreationCatalogUi,
    session === null ? null : captureDialogOrigin(session),
  ), [captureDialogOrigin, currentDocumentId, currentDocumentRuntimeKey, session, spaceArtifactCreationCatalog, spaceArtifactCreationCatalogUi]);
  const selectedSpaceArtifactKinds = selectedSpaceArtifactCreationCatalog?.kinds;
  const artifactCreationChoiceRevisionRef = useRef<string | undefined>(undefined);

  const presencePeers = useMemo((): readonly ScopedPresencePeerV1[] => currentDocumentRuntimeKey === null ? [] : (presencePeersByRuntimeKey[currentDocumentRuntimeKey] ?? []), [currentDocumentRuntimeKey, presencePeersByRuntimeKey]);
  /** ⏪️👥️ Peers' open history edits, labelled from this program's own history rows: the roster chip says who edits
   * which mutation, and the history body's affected rows carry a note through the interpreter's presence overlay. */
  const focusedTimeTravelStage = focusedTimeTravel?.stage ?? null;
  const rowActionRefusals = useMemo(() => timeTravelRowActionRefusalsV1(focusedTimeTravelStage === null ? null : { stage: focusedTimeTravelStage }), [focusedTimeTravelStage, uiLocale]);
  const timeTravelPeers = useMemo(() => timeTravelPeerPresenceV1(presencePeers, Object.values(focusedHistoryEntries), { terminology: uiTerminology, locale: uiLocale }), [presencePeers, focusedHistoryEntries, uiTerminology, uiLocale]);

  const currentSyncStatus = currentDocumentRuntimeKey ? (syncStatusByDocumentId[currentDocumentRuntimeKey] ?? null) : null;
  const syncPillState: SyncPillState = useMemo(() => computeSyncPillState(currentSyncStatus), [currentSyncStatus]);

  /** 📶️ Every attached document's sync status, for the footer's one hub badge — the whole point of
   * that badge is that it is NOT the active document's, so this reads the full map rather than
   * `currentSyncStatus`. */
  const hubConnectionStatuses = useMemo(() => Object.values(syncStatusByDocumentId), [syncStatusByDocumentId]);
  /** 🔐️ Derived from the SHELL's own hub session, not from the workspace overlay's mirror of it: a
   * remembered capability is a session until the hub pronounces it over (`hubSessionRefused`), which
   * survives the overlay being closed and a reload re-verifying it. `"none"` only when this shell was
   * served without any hub (`VITE_S_HUB_URL` unset) — purely local-first, nothing to sign in to. */
  const hubSessionPresence: HubSessionPresenceV1 = hubEnv === null ? "none" : hubSessionCapability === null || hubSessionRefused ? "signedOut" : "signedIn";
  /** 🔗️ What the session revalidation loop last learned about the link: an unreachable hub while the
   * verified authority is kept is a short shortage the loop rides out on its bounded backoff — the badge
   * says `reconnecting` and nothing else in the shell stops. */
  const hubLink: HubLinkV1 = identityOffline ? "unreachable" : verifiedSessionAuthority !== null ? "reachable" : "verifying";
  /** 🔸️ Opens the hub workspace through its own address, so the badge, the palette verb and a pasted
   * `/hub` link all take one path — but the overlay's own state is opened directly, because the URI
   * lane is host-mode-only: `applyShellUri` is guarded by `hostMode`, so in a plugin playground a
   * `/hub` navigation never reaches the route branch and the entry point would be a dead button
   * everywhere except the hub host. Signing in to a hub is shell chrome, not a host-mode privilege. */
  const openHubWorkspace = useCallback(() => {
    setHubWorkspaceOpen(true);
    if (hostMode) navigateHistory(SHELL_HUB_ROUTE);
  }, [hostMode, navigateHistory]);
  /** 🚪️ What the human is told while the session route waits for the identity it is opened for, or opens without its
   * hub ({@link shellRouteAdmissionV1}) — a held route is never silent. */
  const routeAdmission = hostMode && sessionRoute !== null ? shellRouteAdmissionV1(sessionRoute, identityResolution) : "apply";
  const routeAdmissionNotice = routeAdmission === "apply" ? null : shellRouteAdmissionTextV1(routeAdmission, uiLocale);
  /** 👥️ The hub identities this shell's presence lane currently sees. A peer with no hub identity
   * (a folder-only or anonymous participant) contributes nothing, so the roster can never show a
   * member the directory does not know. */
  const hubOnlineUserIds = useMemo(() => [...new Set(presencePeers.flatMap((peer) => (typeof peer.userId === "string" && peer.userId.length > 0 ? [peer.userId] : [])))], [presencePeers]);

  const isEditorSession = canCheckIn(session?.app.role);

  useEffect(() => {
    void backgroundSpaceIndexSessionsRef.current.retain(owned => identity !== null && owned.hubBaseUrl === identity.hubBaseUrl && owned.userId === identity.userId && loadedPlugins.some(entry => entry.handle === owned.plugin) && openDocumentSessionsRef.current.get(owned.runtimeKey)?.clientInstanceId === owned.clientInstanceId)
      .catch(error => undefined);
  }, [identity?.hubBaseUrl, identity?.userId, loadedPlugins]);

  /** 📌️ §C5 item 6 — after a successful checkpoint, `TouchArtifact` the space's `index` document so
   * every connected user's home/space table `updated`/`updated-by` columns move. The index document
   * is almost never the one mounted in this shell's single visible session while an artifact editor
   * is open, so this opens (once per space, cached) a background, non-visible instance of the `s.space`
   * editor bound to `index` and dispatches its `touchArtifact` command directly — never touches
   * `dispatch({type:"SET_SESSION"...})`, so the user's own editor stays exactly where it is. */
  const touchSpaceIndexArtifact = useCallback(
    async (spaceId: string, artifactId: string) => {
      try {
        const identity = identityRef.current;
        if (identity === null || extensionFetchAbortRef.current.signal.aborted) return;
        const identityIsCurrent = () => !extensionFetchAbortRef.current.signal.aborted && identityRef.current?.hubBaseUrl === identity.hubBaseUrl && identityRef.current?.userId === identity.userId;
        const touch = async (plugin: PluginWasmHandle, targetSession: ActiveSession) => {
          if (!identityIsCurrent() || !plugin.handleCommand) return;
          const wire = encodeAppCommandInvocation(plugin.pluginId, targetSession.app, "touchArtifact", { id: artifactId, nowMs: Date.now(), actor: identity.userId });
          await plugin.handleCommand(targetSession.instanceId, wire, targetSession.viewState);
        };
        const liveEntry = currentDocumentId === SPACE_INDEX_DOCUMENT_ID && currentDocumentRuntimeKey !== null ? openDocumentSessionsRef.current.get(currentDocumentRuntimeKey) : undefined;
        if (liveEntry?.scope?.spaceId === spaceId && liveEntry.session.app.role === "editor") {
          await touch(liveEntry.plugin, liveEntry.session);
          return;
        }
        const current = (owned: BackgroundSpaceIndexSession) => identityIsCurrent() && owned.hubBaseUrl === identity.hubBaseUrl && owned.userId === identity.userId && loadedPluginsRef.current.some(entry => entry.handle === owned.plugin) && openDocumentSessionsRef.current.get(owned.runtimeKey)?.clientInstanceId === owned.clientInstanceId;
        await backgroundSpaceIndexSessionsRef.current.run(spaceId, {
          current,
          create: async () => {
            const located = findSpaceIndexEditor(loadedPluginsRef.current);
            if (!located || !identityIsCurrent()) return null;
            const { plugin: pluginEntry, app } = located;
            const plugin = pluginEntry.handle;
            const instanceId = await createAdmittedShellInstanceV1(() => identityIsCurrent() && loadedPluginsRef.current.some(entry => entry.handle === plugin), () => plugin.createApp(app.id), id => plugin.destroyApp(id));
            if (instanceId === null) return null;
            const targetSession: ActiveSession = { pluginId: plugin.pluginId, instanceId, app, viewState: { activeModeId: app.defaultModeId ?? app.modes[0]?.id } };
            let retained = false;
            try {
              if (!identityIsCurrent() || !loadedPluginsRef.current.some(entry => entry.handle === plugin)) return null;
              const receipt = await openDocumentRef.current({ documentId: SPACE_INDEX_DOCUMENT_ID, schema: located.index.documentSchema, spaceId }, undefined, { session: targetSession, plugin, background: true });
              if (receipt === null) return null;
              retained = true;
              return { ...receipt, plugin, session: targetSession, hubBaseUrl: identity.hubBaseUrl, userId: identity.userId };
            } finally { if (!retained) await plugin.destroyApp(instanceId); }
          },
          release: async owned => {
            try {
              try { closeDocumentRef.current(owned.runtimeKey, owned.clientInstanceId); }
              finally { await retireDocumentAttachment(owned.plugin, owned.session.instanceId, owned.clientInstanceId); }
            } finally { await owned.plugin.destroyApp(owned.session.instanceId); }
          },
          visit: async owned => { if (current(owned)) await touch(owned.plugin, owned.session); },
        });
      } catch (touchError) {
      }
    },
    [currentDocumentId, currentDocumentRuntimeKey, retireDocumentAttachment],
  );

  /** 🪟️ Fires `commitCheckpoint` through the SAME action funnel the History panel's own quick
   * "Checkpoint" button uses (`history_command` in `🔌️plugin/component.rs`) — `message` is optional
   * (auto check-ins pass `"auto"`), `authors` rides along for when the framework threads it (today it
   * doesn't — `history_command` hardcodes `authors: Vec::new()`, see `📓️w3-a-report.md`'s
   * sharedFileRequest). `checkpointDispatchedRef` lets the effect below tell "a checkpoint we asked
   * for landed" apart from "the session just mounted with a pre-existing checkpoint". While the focused program's history
   * is being edited the guest freezes it (`timeTravel.frozen`), so no check-in fires: an automatic one waits for the next
   * idle window after the session closes, an explicit one is told why (e2e R2-6). */
  const checkpointDispatchedRef = useRef(false);
  /** ✅️ Hub Check Ins this shell asked for, by request id, and the latest status per document — the
   * worker names the acknowledged head and drives the hub job; the shell only shows and cancels it. */
  const documentCheckInOwnersRef = useRef<Map<string, { readonly runtimeKey: string }>>(new Map());
  core.documentCheckInOwnersRef = documentCheckInOwnersRef;
  const [documentCheckInUi, setDocumentCheckInUi] = useState<Readonly<Record<string, { readonly requestId: string; readonly scope: DocumentScope; readonly status: DocumentCheckInStatusV1 }>>>({});
  core.setDocumentCheckInUi = setDocumentCheckInUi;
  const requestHubCheckIn = useCallback((runtimeKey: string) => {
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    const worker = backboneWorkerRef.current;
    if (entry?.scope === undefined || worker === null) return;
    const requestId = mintDirectoryCommandRequestId();
    documentCheckInOwnersRef.current.set(requestId, { runtimeKey });
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "document-check-in", requestId, clientInstanceId: entry.clientInstanceId, scope: entry.scope }) });
  }, []);
  const cancelHubCheckIn = useCallback((runtimeKey: string) => {
    const current = documentCheckInUi[runtimeKey];
    if (current === undefined || isTerminalDocumentCheckInPhaseV1(current.status.phase)) return;
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "document-check-in-cancel", requestId: current.requestId, scope: current.scope }) });
  }, [documentCheckInUi]);
  /** 🔒️ Closes a finished Check In's band; a running one is cancelled, never dismissed. */
  const dismissHubCheckIn = useCallback((runtimeKey: string) => {
    setDocumentCheckInUi((current) => {
      const entry = current[runtimeKey];
      if (entry === undefined || !isTerminalDocumentCheckInPhaseV1(entry.status.phase)) return current;
      const { [runtimeKey]: _dismissed, ...rest } = current;
      return rest;
    });
  }, []);
  const currentCheckIn = currentDocumentRuntimeKey === null ? undefined : documentCheckInUi[currentDocumentRuntimeKey];
  const dispatchCheckpoint = useCallback(
    (message: string): boolean => {
      if (!session) return false;
      const gate = checkpointGateV1(focusedHistoryV1().timeTravel, message);
      if (gate === "frozen") {
        const notice = historyRefusalNoticeV1("timeTravel.frozen");
        showTransientNoticeRef.current(notice.text, notice.kind, notice.code);
      }
      if (gate !== "dispatch") return false;
      if (message === "auto") {
        const attached = [...openDocumentSessionsRef.current.values()].find((entry) => entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId);
        const loading = focusedHistoryV1().reprojection?.kind === "load" || attached?.replacements.pending === true;
        if (automaticCheckinWaitsV1({ loading, attached: attached !== undefined, bound: attached !== undefined && attached.port !== null && !attached.port.closing })) return false;
      }
      checkpointDispatchedRef.current = true;
      const authors = identityRef.current ? [{ id: identityRef.current.userId, name: identityRef.current.displayName }] : [];
      onAction({ controllerId: session.app.controllerId, action: "commitCheckpoint", args: { message, authors } });
      return true;
    },
    [focusedHistoryV1, session, onAction],
  );

  const previousCheckpointIdRef = useRef<string | undefined>(undefined);
  return { previousCheckpointIdRef, historyCheckpointId, checkpointDispatchedRef, currentDocumentRuntimeKey, requestHubCheckIn, openSpaceIdRef, currentDocumentId, touchSpaceIndexArtifact, isEditorSession, dispatchCheckpoint, focusedHistoryV1, historyStore, session, dispatchCheckpointRef, uiTerminology, uiLocale, focusedApp, focusedPanelBodyStores, onAction, appLabelsOverlay, treeWindowHost, panelTreeConfigCacheRef, focusedBrowserActorPanels, treeWindowGeneration, shellState, syncBackboneUri, syncStatusByDocumentId, syncPillState, syncCardKind, syncDraftPath, dispatch, attachSyncTarget, detachSyncBackbone, browseSyncBackbonePath, selectedSpaceArtifactCreationCatalog, selectedSpaceArtifactKinds, artifactCreationChoiceRevisionRef, presencePeers, rowActionRefusals, timeTravelPeers, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace, routeAdmission, routeAdmissionNotice, hubOnlineUserIds, cancelHubCheckIn, dismissHubCheckIn, currentCheckIn };
}

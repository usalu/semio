/** ✅️ Presence and hub link state, document check-in, auto check-in, the history tab and the sync tab.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useCheckInAndSync` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { createAdmittedShellInstanceV1 } from "../🗨️dialog-origin/🟦️.ts";
import { isTerminalDocumentCheckInPhaseV1, type DocumentCheckInStatusV1 } from "../../../../../📇️directory/🧬️schema/📌️document-check-in-v1/🟦️.ts";
// #region 🔌️Adapters
import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState
} from "react";
import { buildFrameworkSyncUtilities, encodeBackboneWorkerRequest, type DocumentScope } from "@semio-tech/framework-os";
import { type ScopedPresencePeerV1 } from "../👥️presence-scope/🟦️.ts";
import { SHELL_HUB_ROUTE, shellRouteAdmissionTextV1, shellRouteAdmissionV1 } from "../🔀️surface-switch/🟦️.ts";
import { type PanelTabNode, singleTreeLeaf } from "@semio-tech/ui-react";
import { type ActiveSession, selectQuarantinedConflicts } from "../../🐚️Shell/🟦️.tsx";
import {
  panelTabDefinitionToNode,
  historyRefusalNoticeV1,
  AutoCheckinScheduler,
  automaticCheckinWaitsV1,
  canCheckIn,
  checkpointGateV1,
  checkpointOnCloseKeyV1,
  useCheckpointOnCloseV1,
  computeSyncPillState,
  shellHistoryCursorDomV1,
  partitionFrameworkHistoryPanelTab,
  shellTabIcon,
  syncPillText,
  type SyncPillState
} from "../../🛠️ShellHelpers/🟦️.tsx";
// #endregion 🔌️Adapters
import { timeTravelPeerPresenceV1, timeTravelRowActionRefusalsV1 } from "../../🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import { type PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import { SyncAttachCard, type HubLinkV1, type HubSessionPresenceV1 } from "../../🔄️ShellSync/🟦️.tsx";
import { UTILITY_CATEGORY_ICON_ID } from "../../🎛️UtilityTree/🟦️.tsx";
import { shellUncommittedEditCountV1 } from "../🏁️operation-settle/🟦️.ts";
import { encodeAppCommandInvocation } from "../📨️window-invocation/🟦️.ts";
import { findSpaceIndexEditor, mintDirectoryCommandRequestId, selectedSpaceArtifactCreationCatalogV1, SPACE_INDEX_DOCUMENT_ID } from "../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterDocumentTransferAndTabs } from "../📤️document-transfer-tabs/🟦️.tsx";
import type { BackgroundSpaceIndexSession } from "../🔒️identity-document-ports/🟦️.ts";

export type ShellCoreAfterCheckInAndSync = ShellCoreAfterDocumentTransferAndTabs & ReturnType<typeof useCheckInAndSync>;

export function useCheckInAndSync(core: ShellCoreAfterDocumentTransferAndTabs) {
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
  // checked into vcs": status pill (`#s-sync-status`), auto check-in (idle ≥ 20s or ≥ 200 uncommitted
  // edits), explicit check-in (`#s-checkin`), checkpoint-on-close, and the post-checkpoint
  // `TouchArtifact` relay to the space index. Placed ahead of `🧰️FooterUtilityLeaves`/`🔄️SyncLeaf`
  // (their `useMemo`s below close over these) — everything here is additive, no existing behaviour
  // changed for a session outside a hub-bound space.
  const currentDocumentId = currentDocumentRuntimeKey === null ? null : (openDocumentSessionsRef.current.get(currentDocumentRuntimeKey)?.documentId ?? null);
  const selectedSpaceArtifactCreationCatalog = useMemo(() => selectedSpaceArtifactCreationCatalogV1(
    spaceArtifactCreationCatalog,
    spaceArtifactCreationCatalogUi,
    session === null ? null : captureDialogOrigin(session),
  ), [captureDialogOrigin, currentDocumentId, currentDocumentRuntimeKey, session, spaceArtifactCreationCatalog, spaceArtifactCreationCatalogUi]);
  const selectedSpaceArtifactKinds = selectedSpaceArtifactCreationCatalog?.kinds;
  const artifactCreationChoiceRevisionRef = useRef<string | undefined>(undefined);

  // 👥️ Host-only normalized roster, keyed by the exact verified document runtime. It never enters a
  // plugin view-state payload, so an app cannot forge or persist Shell presence chrome.
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
  /** 🔗️ Opens the hub workspace through its own address, so the badge, the palette verb and a pasted
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

  /** 📌️ Fires `commitCheckpoint` through the SAME action funnel the History panel's own quick
   * "Checkpoint" button uses (`history_command` in `🔌️plugin/component.rs`) — `message` is optional
   * (auto check-ins pass `"auto"`), `authors` rides along for when the framework threads it (today it
   * doesn't — `history_command` hardcodes `authors: Vec::new()`, see `📓️w3-a-report.md`'s
   * sharedFileRequest). `checkpointDispatchedRef` lets the effect below tell "a checkpoint we asked
   * for landed" apart from "the session just mounted with a pre-existing checkpoint". While the focused program's history
   * is being edited the guest freezes it (`timeTravel.frozen`), so no check-in fires: an automatic one waits for the next
   * idle window after the session closes, an explicit one is told why (e2e R2-6). */
  const checkpointDispatchedRef = useRef(false);
  /** 📌️ Hub Check Ins this shell asked for, by request id, and the latest status per document — the
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
  /** 📌️ Closes a finished Check In's band; a running one is cancelled, never dismissed. */
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

  // 📌️ §C5 item 6 continued — `TouchArtifact` fires once per checkpoint THIS shell asked for,
  // detected as the focused program's `currentCheckpointId` changing away from whatever it was the last
  // time this effect ran (never on the initial mount/session-snapshot, which isn't a checkpoint WE
  // just made).
  const previousCheckpointIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    const previous = previousCheckpointIdRef.current;
    const next = historyCheckpointId;
    previousCheckpointIdRef.current = next;
    if (!checkpointDispatchedRef.current || !next || next === previous) return;
    checkpointDispatchedRef.current = false;
    if (currentDocumentRuntimeKey !== null) requestHubCheckIn(currentDocumentRuntimeKey);
    const spaceId = openSpaceIdRef.current;
    if (spaceId && currentDocumentId && currentDocumentId !== SPACE_INDEX_DOCUMENT_ID) {
      void touchSpaceIndexArtifact(spaceId, currentDocumentId);
    }
  }, [historyCheckpointId, currentDocumentId, currentDocumentRuntimeKey, requestHubCheckIn, touchSpaceIndexArtifact]);

  // 📌️ §C5 item 2 — auto check-in, delegated to the framework-free `AutoCheckinScheduler`
  // (`ShellHelpers`) so the debounce/storm-guard logic is unit-testable with fake timers without
  // mounting this component. One scheduler instance per open editor session (rebuilt whenever
  // `session`/`currentDocumentId` changes — a document switch is a fresh idle clock), `cancel`ed on
  // unmount/switch (the effect's own cleanup).
  const autoCheckinSchedulerRef = useRef<AutoCheckinScheduler | null>(null);
  useEffect(() => {
    if (!isEditorSession || !currentDocumentId) {
      autoCheckinSchedulerRef.current = null;
      return;
    }
    const scheduler: AutoCheckinScheduler = new AutoCheckinScheduler(() => {
      if (!dispatchCheckpoint("auto")) scheduler.defer();
    });
    autoCheckinSchedulerRef.current = scheduler;
    return () => scheduler.cancel();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isEditorSession, currentDocumentId, dispatchCheckpoint]);
  const uncommittedEditCountRef = useRef(0);
  useEffect(() => {
    let notified: number | null = null;
    const observe = (): void => {
      const count = shellUncommittedEditCountV1(focusedHistoryV1().entries);
      uncommittedEditCountRef.current = count;
      if (count === notified) return;
      notified = count;
      autoCheckinSchedulerRef.current?.notify(count);
    };
    observe();
    return historyStore.subscribe(observe);
  }, [historyStore, focusedHistoryV1]);

  // 📌️ §C5 item 4 — checkpoint on close: fires from the cleanup of an effect keyed on the program's
  // identity and `currentDocumentId`, so it runs the instant either changes (switching document/app —
  // this shell keeps exactly one session mounted, so "switch away" IS "close" here) as well as on true
  // unmount — never on a new session object of the same program (every view-state rewrite mints one, and the
  // New-alternative submit fired a close check-in into the frozen history, e2e R2-6). Best-effort (fire-and-forget, not gated on the success-detection effect above — by the
  // time the response arrives `historyProjection` may already belong to the NEW session).
  const closingSpaceId = openSpaceIdRef.current;
  useCheckpointOnCloseV1(checkpointOnCloseKeyV1(isEditorSession, session, currentDocumentId), () => {
    if (uncommittedEditCountRef.current === 0) return;
    dispatchCheckpoint("auto");
    if (closingSpaceId && currentDocumentId && currentDocumentId !== SPACE_INDEX_DOCUMENT_ID) void touchSpaceIndexArtifact(closingSpaceId, currentDocumentId);
  });

  // 📌️ §C5 item 3 — explicit check-in: the history body's `#s-checkin` dispatches `framework.checkin` `submit`, which
  // `onAction` answers through this ref (the same fallback message the wgpu shell uses).
  dispatchCheckpointRef.current = dispatchCheckpoint;
  //#endregion 🔖️CheckIn

  /** 🧾️ `data-history-json` on the shell root follows the focused program's history straight from the store — written on
   * the element, never through a shell render. */
  const historyDomElementRef = useRef<HTMLDivElement | null>(null);
  const publishHistoryDomRef = useRef<() => void>(() => undefined);
  publishHistoryDomRef.current = () => historyDomElementRef.current?.setAttribute("data-history-json", JSON.stringify(shellHistoryCursorDomV1(focusedHistoryV1(), { terminology: uiTerminology, locale: uiLocale })));
  const historyDomRef = useCallback((element: HTMLDivElement | null) => {
    historyDomElementRef.current = element;
    publishHistoryDomRef.current();
  }, []);
  useLayoutEffect(() => {
    publishHistoryDomRef.current();
    return historyStore.subscribe(() => publishHistoryDomRef.current());
  }, [historyStore, focusedHistoryV1, uiTerminology, uiLocale]);

  //#region 🧰️FooterUtilityLeaves — bottom-right's History leaf: the framework-injected `framework.panel.history` tab
  // (every app gets one — see `AppBuilder::build_definition`) rendered from the focused program's own
  // `framework.body.history` through the interpreter, like every other panel body.
  const frameworkHistoryTab = useMemo((): PanelTabNode | null => {
    const tab = session && focusedApp ? partitionFrameworkHistoryPanelTab(focusedApp.panelTabs).history : undefined;
    return tab === undefined ? null : panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, 1, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels);
  }, [appLabelsOverlay, focusedBrowserActorPanels, onAction, focusedPanelBodyStores, session, focusedApp, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration]);
  //#endregion 🧰️FooterUtilityLeaves

  //#region 🔄️SyncLeaf — bottom-left's sync tab, replacing the old floating footer SyncAttachCard.
  const quarantinedConflicts = useMemo(() => selectQuarantinedConflicts(shellState), [shellState]);
  const frameworkSyncTab = useMemo((): PanelTabNode | null => {
    // 🩹️ `buildFrameworkSyncUtilities` already returns `readonly FrameworkSyncUtilityLeaf[]` — the
    // `as readonly UtilityNode[]` cast this line used to carry was simply the wrong target type for
    // `SyncAttachCard`'s own prop (a stale workaround, not a real narrowing need).
    const syncUtilities = buildFrameworkSyncUtilities(syncBackboneUri);
    if (!syncUtilities.length) return null;
    const syncStatus = syncBackboneUri ? (syncStatusByDocumentId[syncBackboneUri.replace(/^actor:\/\//, "")] ?? null) : null;
    // 📌️ §C5 item 1 — the status pill lives on THIS tab's own folded chrome button (`id`/`name`,
    // always visible in the footer's tab strip, no click needed) rather than the manual sync-card's
    // OWN status line further below (`SyncAttachCard`'s `syncStatusLabel`, peer-owned, unchanged) —
    // the pill reflects the CURRENT session's document (`syncPillState`, computed in `🔖️CheckIn`
    // above from `currentDocumentId`), not just a manually-attached `remote://` override, so it
    // updates for the common case (identity auto-bound to a hub space) too.
    return singleTreeLeaf({
      id: "s-sync-status",
      icon: shellTabIcon(UTILITY_CATEGORY_ICON_ID.sync),
      name: syncPillText(syncPillState, uiLocale),
      order: 0,
      tree: {
        sections: [
          {
            id: "framework.sync.root",
            label: "",
            items: [
              {
                id: "framework.sync.card",
                label: "",
                control: (
                  <SyncAttachCard
                    activeUri={syncBackboneUri}
                    cardKind={syncCardKind}
                    draftPath={syncDraftPath}
                    syncUtilities={syncUtilities}
                    status={syncStatus}
                    quarantinedConflicts={quarantinedConflicts}
                    onAction={onAction}
                    onDraftPathChange={(value) => dispatch({ type: "SET_SYNC_DRAFT_PATH", value })}
                    onClose={() => dispatch({ type: "SET_SYNC_CARD_KIND", value: null })}
                    onAttach={attachSyncTarget}
                    onDetach={detachSyncBackbone}
                    onBrowsePath={() => (syncCardKind === "folder" ? browseSyncBackbonePath("folder") : syncCardKind === "file" ? browseSyncBackbonePath("file") : undefined)}
                  />
                ),
              },
            ],
          },
        ],
      },
    });
  }, [attachSyncTarget, browseSyncBackbonePath, detachSyncBackbone, onAction, syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId, syncPillState, quarantinedConflicts, uiLocale]);
  //#endregion 🔄️SyncLeaf

  return {
    selectedSpaceArtifactCreationCatalog, selectedSpaceArtifactKinds, artifactCreationChoiceRevisionRef, presencePeers,
    rowActionRefusals, timeTravelPeers, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace,
    routeAdmission, routeAdmissionNotice, hubOnlineUserIds, cancelHubCheckIn, dismissHubCheckIn, currentCheckIn,
    historyDomRef, frameworkHistoryTab, frameworkSyncTab
  };
  //#endregion FrameworkOsShell
}

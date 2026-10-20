/** 🔸️ The auto checkin scheduler phase of useCheckInAndSync. */
import { useCallback, useEffect, useLayoutEffect, useMemo, useRef } from "react";
import { buildFrameworkSyncUtilities } from "@semio-tech/framework/backbone";
import { type PanelTabNode, singleTreeLeaf } from "@semio-tech/ui-react";
import { selectQuarantinedConflicts } from "../../../🐚️Shell/🕹️shell-store/🟦️.ts";
import { panelTabDefinitionToNode, partitionFrameworkHistoryPanelTab } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { AutoCheckinScheduler, checkpointOnCloseKeyV1, useCheckpointOnCloseV1, syncPillText } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { shellHistoryCursorDomV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { shellTabIcon } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { SyncAttachCard } from "../../../🔄️ShellSync/🟦️.tsx";
import { UTILITY_CATEGORY_ICON_ID } from "../../../🎛️UtilityTree/🟦️.tsx";
import { shellUncommittedEditCountV1 } from "../../🏁️operation-settle/🟦️.ts";
import { SPACE_INDEX_DOCUMENT_ID } from "../../🧾️space-artifact-creation/🟦️.ts";
import type { useHostModeState } from "../🔶️host-mode/🟦️.tsx";

/** 🔹 Builds autoCheckinSchedulerRef, uncommittedEditCountRef, closingSpaceId, historyDomElementRef of useCheckInAndSync. */
export function useAutoCheckinSchedulerState({ previousCheckpointIdRef, historyCheckpointId, checkpointDispatchedRef, currentDocumentRuntimeKey, requestHubCheckIn, openSpaceIdRef, currentDocumentId, touchSpaceIndexArtifact, isEditorSession, dispatchCheckpoint, focusedHistoryV1, historyStore, session, dispatchCheckpointRef, uiTerminology, uiLocale, focusedApp, focusedPanelBodyStores, onAction, appLabelsOverlay, treeWindowHost, panelTreeConfigCacheRef, focusedBrowserActorPanels, treeWindowGeneration, shellState, syncBackboneUri, syncStatusByDocumentId, syncPillState, syncCardKind, syncDraftPath, dispatch, attachSyncTarget, detachSyncBackbone, browseSyncBackbonePath }: Pick<ReturnType<typeof useHostModeState>, "previousCheckpointIdRef" | "historyCheckpointId" | "checkpointDispatchedRef" | "currentDocumentRuntimeKey" | "requestHubCheckIn" | "openSpaceIdRef" | "currentDocumentId" | "touchSpaceIndexArtifact" | "isEditorSession" | "dispatchCheckpoint" | "focusedHistoryV1" | "historyStore" | "session" | "dispatchCheckpointRef" | "uiTerminology" | "uiLocale" | "focusedApp" | "focusedPanelBodyStores" | "onAction" | "appLabelsOverlay" | "treeWindowHost" | "panelTreeConfigCacheRef" | "focusedBrowserActorPanels" | "treeWindowGeneration" | "shellState" | "syncBackboneUri" | "syncStatusByDocumentId" | "syncPillState" | "syncCardKind" | "syncDraftPath" | "dispatch" | "attachSyncTarget" | "detachSyncBackbone" | "browseSyncBackbonePath">) {
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

  const closingSpaceId = openSpaceIdRef.current;
  useCheckpointOnCloseV1(checkpointOnCloseKeyV1(isEditorSession, session, currentDocumentId), () => {
    if (uncommittedEditCountRef.current === 0) return;
    dispatchCheckpoint("auto");
    if (closingSpaceId && currentDocumentId && currentDocumentId !== SPACE_INDEX_DOCUMENT_ID) void touchSpaceIndexArtifact(closingSpaceId, currentDocumentId);
  });

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
  const frameworkHistoryTab = useMemo((): PanelTabNode | null => {
    const tab = session && focusedApp ? partitionFrameworkHistoryPanelTab(focusedApp.panelTabs).history : undefined;
    return tab === undefined ? null : panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, 1, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels);
  }, [appLabelsOverlay, focusedBrowserActorPanels, onAction, focusedPanelBodyStores, session, focusedApp, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration]);
  //#endregion 🧰️FooterUtilityLeaves

  //#region 🔄️SyncLeaf — bottom-left's sync tab, replacing the old floating footer SyncAttachCard.
  const quarantinedConflicts = useMemo(() => selectQuarantinedConflicts(shellState), [shellState]);
  const frameworkSyncTab = useMemo((): PanelTabNode | null => {
    const syncUtilities = buildFrameworkSyncUtilities(syncBackboneUri);
    if (!syncUtilities.length) return null;
    const syncStatus = syncBackboneUri ? (syncStatusByDocumentId[syncBackboneUri.replace(/^actor:\/\//, "")] ?? null) : null;
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
  return { historyDomRef, frameworkHistoryTab, frameworkSyncTab };
}

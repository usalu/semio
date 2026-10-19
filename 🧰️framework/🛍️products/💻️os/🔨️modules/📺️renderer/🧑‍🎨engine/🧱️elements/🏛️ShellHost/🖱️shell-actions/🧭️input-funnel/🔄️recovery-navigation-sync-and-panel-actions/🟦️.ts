/** 🔄️ The shell-intercepted recovery, camera navigation, sync card and panel tab actions of the input funnel.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | useShellActions}: each function is one stage of the shell input funnel (`onAction`), moved verbatim. */
import { panelTabKindId, windowElementId } from "@semio-tech/framework";
import { type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../../🎯️input-ledger/🟦️.ts";
import { type ActiveSession } from "../../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { type SyncCardKind } from "../../../../🐚️Shell/👥️peer-interaction/🟦️.ts";
import { buildSpacePanelState, parsePanelState } from "../../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { flattenPanelTabLeaves, panelAnchorForGroup } from "../../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { introductionTargetsWindow, sessionWindowInstances } from "../../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { syncAttachTargetV1 } from "../../../../🔄️ShellSync/🟦️.tsx";
import { panelDefinitionPath, requiredHostPanelLeafId } from "../../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterShellUriAndDocuments } from "../../../🗄️shell-uri-documents/🟦️.ts";

/** 🚑️ Restarts or quarantines a program for the recovery controller actions. */
export function handleRecoveryAction({ primaryPluginId, dispatch, reloadPlugin, uninstallPlugin, action, refuse, applied }: Pick<ShellCoreAfterShellUriAndDocuments, "primaryPluginId" | "dispatch" | "reloadPlugin" | "uninstallPlugin"> & {
  readonly action: ShellInputActionV1;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}) {
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
}

/** 🧭️ Completes the introduction interactions a camera navigation gesture report satisfies. */
export function handleWorldNavigationAction({ extraWindowInstancesRef, completeIntroductionInteraction, session, action, applied }: Pick<ShellCoreAfterShellUriAndDocuments, "extraWindowInstancesRef" | "completeIntroductionInteraction"> & {
  readonly session: ActiveSession;
  readonly action: ShellInputActionV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}) {
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

/** 📡️ Routes the sync card actions to the attach, detach, browse and selection callbacks. */
export function handleSyncCardAction({ dispatch, browseSyncBackbonePath, syncCardKind, syncDraftPath, attachSyncTarget, detachSyncBackbone, action, applied, refuse }: Pick<ShellCoreAfterShellUriAndDocuments, "dispatch" | "browseSyncBackbonePath" | "syncCardKind" | "syncDraftPath" | "attachSyncTarget" | "detachSyncBackbone"> & {
  readonly action: ShellInputActionV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
}) {
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

/** 🗂️ Validates and applies a panel tab activation on the host or the session program. */
export function handleSetActivePanelTabAction({ hostApp, updateSpacePanel, mobile, dispatch, session, action, refuse, applied }: Pick<ShellCoreAfterShellUriAndDocuments, "hostApp" | "updateSpacePanel" | "mobile" | "dispatch"> & {
  readonly session: ActiveSession;
  readonly action: ShellInputActionV1;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}) {
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

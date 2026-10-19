/** 📤️ Document transfer tasks, agent approvals, the app keydown handler, the current document and the workbench, details and bottom-anchor tab lists.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useDocumentTransferAndTabs` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { useCallback, useEffect, useMemo, useRef } from "react";
// #region 🔌️Adapters
import {
  type AppRole,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  type LocalizedLabel,
  panelTabKindId,
  resolveWindowActions,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
  type Severity
} from "@semio-tech/framework";
import { decodePackValue } from "@semio-tech/framework/packs";
import { decodeDocumentArchiveBytes, encodeDocumentArchiveBytes, type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import {
  KEYBINDING_UNOWNED_CODE,
  keybindingUnownedTextV1,
  modeLayoutStacksV1,
  reservedShellChordsV1,
  resolveKeybindingTargetWindowV1,
  type WindowScopeInstanceV1,
  type WindowScopeKindV1,
  type WindowScopeLayoutNodeV1
} from "../⌨️window-scope/🟦️.ts";
import {
  SHELL_KEYBINDINGS,
  formatKeybindingShortcut,
  type PanelTabNode,
  singleTreeLeaf,
  staticTreePanelDefinition,
  useShellKeydown
} from "@semio-tech/ui-react";
import { localDocumentOwnerIdentityForSessionV1 } from "../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { type ActiveSession } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { actionStageKey } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { actionRequiresStagedForm } from "../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx";
import {
  DOCUMENT_ARCHIVE_FILE_EXTENSION,
  DOCUMENT_ARCHIVE_MEDIA_TYPE,
  documentArchiveFileNameV1,
  naturalFileNameV1,
  naturalFileFormatV1,
  naturalMediaDescriptorMatchesV1,
  openNaturalFileOwnerV1,
  bytesOfDataUrlV1,
  documentTransferNoticeTextV1,
  documentTransferNoticeCodeV1,
  type DocumentTransferNoticeV1,
  type OpenWithEntry
} from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import {
  NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1,
  NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1,
  readBlobBytesBoundedV1,
  requestFileSelectionV1,
  downloadMediaExportBytes,
  requestFileOpen
} from "../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { buildSpacePanelState, parsePanelState, studioPanelFocusingSpawned } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import {
  controlOwnsKeyboardEvent,
  keyboardEventMatchesChord,
  resolveKeybindingIntent
} from "../../🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";
import {
  openArtifactWithText,
  resolveAppLabel,
  resolveManifestLabel,
  setAsDefaultText,
  shellLabel,
  shellTabIcon,
  surfaceRoleChipText
} from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import {
  panelAnchorForGroup,
  panelTabDefinitionToNode,
  integrateAppSettingsPanelTabsIntoFrameworkBranch,
  partitionFrameworkHistoryPanelTab
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { type BrowserActorPanelHostV1, uiIntentToActionDescriptor } from "../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import {
  pasteActionWithRetainedFragment,
  type ClipboardActionDescriptor,
  pasteArgsFragment
} from "../../🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts";
// #endregion 🔌️Adapters
import { FRAMEWORK_CHAT_PANEL_ID } from "../../📌️ChromePanels/🆔️panel-ids/🟦️.ts";
import { type PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import { focusAgentApprovalV1 } from "../../🤖️AgentApprovals/🟦️.tsx";
import { documentTransferTasksV1, type TaskManagerDocumentTransferV1 } from "../../🧵️TaskManager/🟦️.tsx";
import { flattenPanelTabNodes, requiredHostPanelLeafId } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterThemeDriverSettings } from "../🎨️theme-driver-settings/🟦️.tsx";

export type ShellCoreAfterDocumentTransferAndTabs = ShellCoreAfterThemeDriverSettings & ReturnType<typeof useDocumentTransferAndTabs>;

export function useDocumentTransferAndTabs(core: ShellCoreAfterThemeDriverSettings) {
  const {
    scope, localDocumentOwnerRegistry, hostMode, dispatch, loadedPlugins, session, uiLocale, uiTerminology,
    uiKeybindingOverrides, hostApp, landingAppId, hostAppId, spacePrograms, appLabelsOverlay, openWithFocusRole,
    clipboardFragmentRef, panelTreeConfigCacheRef, treeWindowGeneration, treeWindowHost, agentBridge, uiLocaleRef,
    showTransientNoticeRef, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, browserActorUiVersion,
    foldSpaceDirectoryRef, panel, focusedApp, focusedSpawnedId, focusedPanelBodyStores, focusedWindowInstancesRef,
    activeUtilityByWindowIdRef, activeToolIdRef, utilityGenerationFor, toolGeneration, activeWindowIdRef,
    effectiveModeLayoutRef, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, updateSpacePanel,
    resolveSyncTargetSession, onAction, onActionStable, onBrowserActorIntent, dispatchSetDefaultApp,
    dispatchClearDefaultApp, openArtifactWithAppRef, openWithEntries, showTransientNotice, frameworkSettingsTab,
    taskManagerListenersRef, documentTransfersRef, documentTransferTasksRef
  } = core;
  //#region FrameworkOsShell
  //#region 📤️DocumentTransfer
  const publishDocumentTransfers = (): void => {
    documentTransferTasksRef.current = documentTransferTasksV1([...documentTransfersRef.current.values()].map((entry) => entry.transfer));
    for (const listener of [...taskManagerListenersRef.current]) listener();
  };
  const notifyDocumentTransfer = (notice: DocumentTransferNoticeV1, file: string, kind: Severity): void => {
    showTransientNoticeRef.current(documentTransferNoticeTextV1(notice, file, uiLocaleRef.current), kind, documentTransferNoticeCodeV1(notice));
  };
  /** 🧵️ One file moving into a program as a cancellable Tasks-window task (`documentTransfer` lane): `progress` states
   * `completed` of `total` units (`null` = open-ended), Cancel aborts `signal`, `finish` removes the row. */
  const trackDocumentTransfer = (file: string, pluginId: string): { readonly signal: AbortSignal; readonly progress: (completed: number, total: number | null) => void; readonly finish: () => void } => {
    const id = crypto.randomUUID();
    const abort = new AbortController();
    const update = (patch: Partial<TaskManagerDocumentTransferV1>): void => {
      const entry = documentTransfersRef.current.get(id);
      if (entry === undefined) return;
      documentTransfersRef.current.set(id, { ...entry, transfer: { ...entry.transfer, ...patch } });
      publishDocumentTransfers();
    };
    documentTransfersRef.current.set(id, { transfer: { id, file, pluginId, startedAtMs: Date.now(), progress: null, cancelling: false }, abort });
    abort.signal.addEventListener("abort", () => update({ cancelling: true }), { once: true });
    publishDocumentTransfers();
    return {
      signal: abort.signal,
      progress: (completed, total) => update({ progress: { completed, total, text: `${completed}/${total ?? "?"}` } }),
      finish: () => {
        documentTransfersRef.current.delete(id);
        publishDocumentTransfers();
      },
    };
  };
  /** 📤️ The focused document program, or `null` when the Home or the space host has the canvas. */
  const documentTransferTarget = (): { readonly target: ActiveSession; readonly handle: PluginWasmHandle } | null => {
    const target = resolveSyncTargetSession();
    if (target === null || target.app.id === hostAppId || target.app.id === landingAppId) return null;
    const handle = loadedPlugins.find((entry) => entry.handle.pluginId === target.pluginId)?.handle;
    return handle === undefined ? null : { target, handle };
  };
  /** 📤️ Export Document: the focused program's archive, written as one `.semio-archive` file. */
  const exportDocumentArchive = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null) return notifyDocumentTransfer("no-document", "", "info");
    const file = documentArchiveFileNameV1(owned.target.app.id, new Date());
    try {
      if (!owned.handle.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
      downloadMediaExportBytes(file, DOCUMENT_ARCHIVE_MEDIA_TYPE, encodeDocumentArchiveBytes(await owned.handle.readAppDocumentArchive(owned.target.instanceId)));
      notifyDocumentTransfer("exported", file, "info");
    } catch (error) {
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("export-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer("export-failed", file, "error");
    }
  };
  /** 📥️ Import Document: a picked archive opens as a NEW document of the focused program, loaded with progress and
   * cancellation from the Tasks window; the focused document stays untouched. */
  const importDocumentArchive = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null || session === null) return notifyDocumentTransfer("no-document", "", "info");
    const load = owned.handle.loadAppDocumentArchive;
    const [picked] = await requestFileOpen(`${DOCUMENT_ARCHIVE_FILE_EXTENSION},${DOCUMENT_ARCHIVE_MEDIA_TYPE}`, "dataUrl", false);
    if (picked === undefined) return;
    let archive: DocumentArchivePack;
    try {
      archive = decodeDocumentArchiveBytes(bytesOfDataUrlV1(picked.contents));
    } catch (error) {
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1("import-unreadable")} ${picked.name}`, error);
      return notifyDocumentTransfer("import-unreadable", picked.name, "warning");
    }
    const task = trackDocumentTransfer(picked.name, owned.target.pluginId);
    let instanceId: number | null = null;
    try {
      if (!load) throw new Error("document-backbone.archive-loader-unavailable");
      instanceId = await owned.handle.createApp(owned.target.app.id);
      await load(instanceId, archive, task.signal, (status) => task.progress(status.completed, status.total > 0 ? status.total : null));
      const program = spacePrograms.find((entry) => entry.pluginId === owned.target.pluginId && entry.appId === owned.target.app.id);
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      updateSpacePanel(studioPanelFocusingSpawned(currentPanel, { id: `${owned.target.pluginId}-${instanceId}`, pluginId: owned.target.pluginId, instanceId, appId: owned.target.app.id, label: program?.label ?? owned.target.app.id, breadcrumb: program?.breadcrumb ?? [] }));
      notifyDocumentTransfer("imported", picked.name, "info");
    } catch (error) {
      if (instanceId !== null) await owned.handle.destroyApp(instanceId).catch((destroyError: unknown) => console.error(`[os-shell] imported instance ${instanceId} could not be closed`, destroyError));
      const notice: DocumentTransferNoticeV1 = task.signal.aborted ? "import-cancelled" : "import-failed";
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(notice)} ${picked.name}`, error);
      notifyDocumentTransfer(notice, picked.name, task.signal.aborted ? "info" : "error");
    } finally {
      task.finish();
    }
  };
  const saveNaturalFile = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null) return notifyDocumentTransfer("no-document", "", "info");
    const format = naturalFileFormatV1(owned.target.app);
    const exportMedia = owned.handle.exportAppMedia;
    const file = format === null ? "" : naturalFileNameV1(owned.target.app.id, format.extension, new Date());
    const task = trackDocumentTransfer(file, owned.target.pluginId);
    try {
      if (format === null || exportMedia === undefined) throw new Error("natural-file.exporter-unavailable");
      if (task.signal.aborted) throw task.signal.reason;
      task.progress(0, 1);
      const artifact = await exportMedia(owned.target.instanceId, "artifact:native");
      if (artifact === null || task.signal.aborted) throw task.signal.reason ?? new Error("natural-file.export-empty");
      if (!naturalMediaDescriptorMatchesV1(decodePackValue(artifact.descriptor), format)) throw new Error("natural-file.export-descriptor-mismatch");
      downloadMediaExportBytes(file, format.mediaType, artifact.data);
      task.progress(1, 1);
      notifyDocumentTransfer("file-saved", file, "info");
    } catch (error) {
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("file-save-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer("file-save-failed", file, "error");
    } finally {
      task.finish();
    }
  };
  const openNaturalFile = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null || session === null) return notifyDocumentTransfer("no-document", "", "info");
    const format = naturalFileFormatV1(owned.target.app);
    const importMedia = owned.handle.importAppMedia;
    if (format === null || importMedia === undefined) return notifyDocumentTransfer("file-open-failed", "", "error");
    const [picked] = await requestFileSelectionV1(`${format.extension},${format.mediaType}`);
    if (picked === undefined) return;
    const task = trackDocumentTransfer(picked.name, owned.target.pluginId);
    let instanceId: number | null = null;
    try {
      if (task.signal.aborted) throw task.signal.reason;
      const bytes = await readBlobBytesBoundedV1(picked, {
        signal: task.signal,
        maximumBytes: NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1,
        chunkBytes: NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1,
        progress: task.progress,
      });
      instanceId = await openNaturalFileOwnerV1({
        signal: task.signal,
        create: () => owned.handle.createApp(owned.target.app.id),
        importBytes: (freshInstanceId) => importMedia(freshInstanceId, "artifact:native", { portId: "artifact:native", kindId: format.formatKind, wire: { kind: "binary", format_kind: format.formatKind } }, bytes),
        retire: (freshInstanceId) => owned.handle.destroyApp(freshInstanceId),
      });
      const program = spacePrograms.find((entry) => entry.pluginId === owned.target.pluginId && entry.appId === owned.target.app.id);
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      updateSpacePanel(studioPanelFocusingSpawned(currentPanel, { id: `${owned.target.pluginId}-${instanceId}`, pluginId: owned.target.pluginId, instanceId, appId: owned.target.app.id, label: program?.label ?? owned.target.app.id, breadcrumb: program?.breadcrumb ?? [] }));
      notifyDocumentTransfer("file-opened", picked.name, "info");
    } catch (error) {
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1("file-open-failed")} ${picked.name}`, error);
      notifyDocumentTransfer("file-open-failed", picked.name, task.signal.aborted ? "info" : "error");
    } finally {
      task.finish();
    }
  };
  const documentTransferRef = useRef({ exportDocumentArchive, importDocumentArchive, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer });
  core.documentTransferRef = documentTransferRef;
  documentTransferRef.current = { exportDocumentArchive, importDocumentArchive, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer };
  //#endregion 📤️DocumentTransfer
  /** ⛩️ An approval request has ONE affordance: its row in the agent conversation. Revealing the chat panel
   * is what makes it visible; `focus` additionally moves keyboard focus onto it (the footer notice's
   * Review), retried across a few frames while the panel mounts. */
  const revealAgentApproval = useCallback((approvalId: string, focus = true) => {
    dispatch({ type: "SET_PANEL_PATH", anchor: "top-right", value: [FRAMEWORK_CHAT_PANEL_ID] });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-right", value: true });
    if (!focus) return;
    let frames = 0;
    const attempt = () => {
      if (focusAgentApprovalV1(approvalId) || frames++ >= 30) return;
      requestAnimationFrame(attempt);
    };
    requestAnimationFrame(attempt);
  }, []);
  /** 🔔️ Every NEWLY arrived approval reveals the conversation it lives in — without taking focus from what
   * the human is typing; the footer notice and its live region carry the announcement. */
  const revealedApprovalIdsRef = useRef(new Set<string>());
  useEffect(() => {
    for (const approval of agentBridge.pendingApprovals) {
      if (revealedApprovalIdsRef.current.has(approval.approvalId)) continue;
      revealedApprovalIdsRef.current.add(approval.approvalId);
      revealAgentApproval(approval.approvalId, false);
    }
  }, [agentBridge.pendingApprovals, revealAgentApproval]);

  const handleAppKeydown = useCallback(
    (event: globalThis.KeyboardEvent) => {
      if (event.defaultPrevented || controlOwnsKeyboardEvent(event)) return;
      if (!session) return;
      const keyApp = focusedApp ?? session.app;
      const parseKeys = (keys: string) =>
        keys
          .split(",")
          .map((key) => key.trim().toLowerCase())
          .filter(Boolean);
      const instances = focusedWindowInstancesRef.current;
      const focusedWindowId = activeWindowIdRef.current ?? (focusedSpawnedId !== null ? (instances[0]?.id ?? null) : (session.viewState.windowId ?? session.viewState.activeWindowKindId ?? null));
      const mounted: readonly WindowScopeInstanceV1[] = modeLayoutStacksV1(effectiveModeLayoutRef.current as WindowScopeLayoutNodeV1 | null)
        .flatMap((stack) => [...stack.windowIds])
        .map((id) => ({ id, windowKindId: instances.find((instance) => instance.id === id)?.windowKindId ?? id }));
      const scopeKinds: readonly WindowScopeKindV1[] = keyApp.windowKinds.map((kind) => ({ id: kind.id, actionIds: resolveWindowActions(keyApp, kind).map((action) => action.id) }));
      const reservedChords = reservedShellChordsV1(SHELL_KEYBINDINGS, uiKeybindingOverrides);
      const actionDefinition = (windowKindId: string, actionId: string) => {
        const kind = keyApp.windowKinds.find((entry) => entry.id === windowKindId);
        return kind ? resolveWindowActions(keyApp, kind).find((action) => action.id === actionId) : undefined;
      };
      if (event.key === "Escape") {
        const windowId = activeWindowIdRef.current;
        if (windowId && activeUtilityByWindowIdRef.current[windowId]) {
          event.preventDefault();
          void onAction({ controllerId: keyApp.controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { windowId, utilityId: "", expectedGeneration: utilityGenerationFor(windowId) } });
          return;
        }
        if (activeToolIdRef.current) {
          event.preventDefault();
          void onAction({ controllerId: keyApp.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "", expectedGeneration: toolGeneration() } });
          return;
        }
      }
      for (const binding of keyApp.keybindings) {
        for (const chord of parseKeys(binding.keys)) {
          if (!keyboardEventMatchesChord(event, chord)) continue;
          if (reservedChords.has(chord)) continue;
          const target = resolveKeybindingTargetWindowV1({ kinds: scopeKinds, mounted, focusedWindowId, actionId: binding.action.action });
          event.preventDefault();
          if (target.windowId === null) {
            const anyDefinition = keyApp.windowKinds.flatMap((kind) => resolveWindowActions(keyApp, kind)).find((action) => action.id === binding.action.action);
            const label = anyDefinition ? resolveAppLabel(appLabelsOverlay, "action", anyDefinition.id, resolveManifestLabel(anyDefinition.label as LocalizedLabel | string, uiTerminology, uiLocale)) : binding.action.action;
            showTransientNotice(keybindingUnownedTextV1(uiLocale, formatKeybindingShortcut(chord), label), "info", KEYBINDING_UNOWNED_CODE);
            return;
          }
          const targetWindowId = target.windowId;
          const targetKindId = mounted.find((instance) => instance.id === targetWindowId)?.windowKindId ?? targetWindowId;
          const definition = actionDefinition(targetKindId, binding.action.action);
          if (!definition) return;
          if (target.kind === "owner") dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: targetWindowId });
          if (actionRequiresStagedForm(definition)) {
            const retainedPaste = pasteActionWithRetainedFragment<ClipboardActionDescriptor>({ action: definition.id }, clipboardFragmentRef.current);
            const retainedArgs = retainedPaste.args;
            if (pasteArgsFragment(retainedPaste) !== undefined) {
              onAction({ controllerId: keyApp.controllerId, action: retainedPaste.action, args: { ...(typeof retainedArgs === "object" && retainedArgs !== null ? retainedArgs : {}), windowId: targetWindowId } });
              return;
            }
            const expanded = actionPaneExpandedByWindowIdRef.current[targetWindowId] ?? null;
            const staged = actionPaneStagedArgsByKeyRef.current[actionStageKey(targetWindowId, definition.id)] ?? {};
            const intent = resolveKeybindingIntent(definition, expanded, staged);
            if (intent.kind === "execute") {
              onAction({ controllerId: keyApp.controllerId, action: intent.actionId, args: { ...(typeof intent.args === "object" && intent.args !== null ? intent.args : {}), windowId: targetWindowId } });
            } else if (intent.kind === "open") {
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId: targetWindowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId: targetWindowId, value: intent.actionId });
            }
            return;
          }
          onAction({ ...binding.action, args: { ...(typeof binding.action.args === "object" && binding.action.args !== null ? (binding.action.args as Record<string, unknown>) : {}), windowId: targetWindowId } });
          return;
        }
      }
      if ((event.ctrlKey || event.metaKey) && !event.altKey && event.key.toLowerCase() === "z") {
        event.preventDefault();
        onAction({ controllerId: keyApp.controllerId, action: event.shiftKey ? "redo" : "undo" });
        return;
      }
      if ((event.ctrlKey || event.metaKey) && !event.altKey && !event.shiftKey && event.key.toLowerCase() === "y") {
        event.preventDefault();
        onAction({ controllerId: keyApp.controllerId, action: "redo" });
        return;
      }
    },
    [onAction, session, focusedApp, focusedSpawnedId, appLabelsOverlay, showTransientNotice, uiKeybindingOverrides, uiLocale, uiTerminology, utilityGenerationFor, toolGeneration],
  );
  useShellKeydown(scope.rootRef, handleAppKeydown, [handleAppKeydown]);

  /** 📌️ Reverse-lookup: `openDocumentSessionsRef` is keyed by exact runtime identity, never
   * the other way around (no `ActiveSession.documentId` field exists — see `📓️w3-a-report.md`'s
   * "Design decisions"). This tiny scan intentionally runs on every render because `openDocument`
   * fills the ref while retaining the same visible session identity. */
  const currentDocumentRuntimeKey = (() => {
    if (!session) return null;
    for (const [runtimeKey, entry] of openDocumentSessionsRef.current) {
      if (entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId) return runtimeKey;
    }
    return null;
  })();
  const currentDocumentEntry = currentDocumentRuntimeKey === null ? undefined : openDocumentSessionsRef.current.get(currentDocumentRuntimeKey);
  const currentLocalDocumentOwnerIdentity = !session
    ? null
    : localDocumentOwnerIdentityForSessionV1(
      { pluginId: session.pluginId, appId: session.app.id, sessionInstanceId: session.instanceId },
      currentDocumentRuntimeKey === null || currentDocumentEntry === undefined ? undefined : { runtimeKey: currentDocumentRuntimeKey, clientInstanceId: currentDocumentEntry.clientInstanceId },
    );
  const currentLocalDocumentOwner = currentLocalDocumentOwnerIdentity === null ? null : localDocumentOwnerRegistry.acquire(currentLocalDocumentOwnerIdentity);
  useEffect(() => {
    localDocumentOwnerRegistry.retain([
      ...[...openDocumentSessionsRef.current.entries()].map(([runtimeKey, entry]) => ({ pluginId: entry.session.pluginId, appId: entry.session.app.id, sessionInstanceId: entry.session.instanceId, runtimeKey, clientInstanceId: entry.clientInstanceId })),
      ...(panel?.spawnedApps ?? []).map(spawned => ({ pluginId: spawned.pluginId, appId: spawned.appId, sessionInstanceId: spawned.instanceId, runtimeKey: `spawned:${spawned.id}`, clientInstanceId: spawned.id })),
      ...(currentDocumentEntry === undefined && currentLocalDocumentOwnerIdentity !== null ? [currentLocalDocumentOwnerIdentity] : []),
    ]);
  });
  useEffect(() => {
    foldSpaceDirectoryRef.current();
  }, [currentDocumentRuntimeKey, session?.pluginId, session?.instanceId]);
  const currentBrowserActorUi = useMemo(
    () => currentDocumentRuntimeKey === null ? undefined : browserActorUiByRuntimeKeyRef.current.get(currentDocumentRuntimeKey),
    [browserActorUiVersion, currentDocumentRuntimeKey],
  );
  /** 🎭️ The panel route of an actor-bound document ({@link BrowserActorPanelHostV1}): its intents go to the verified
   * actor under the panel's own surface; `browserActorUiVersion` moves when a panel store first arrives. */
  const browserActorPanelIntent = useMemo(
    () => currentDocumentRuntimeKey === null || currentBrowserActorUi === undefined ? null : (tabId: string, intent: Parameters<typeof uiIntentToActionDescriptor>[0]) => onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, tabId, intent),
    [currentBrowserActorUi, currentDocumentRuntimeKey, onBrowserActorIntent],
  );
  const browserActorPanels = useMemo(
    (): BrowserActorPanelHostV1 | null => !session || browserActorPanelIntent === null || currentBrowserActorUi?.sessionInstanceId !== session.instanceId ? null : { stores: new Map(currentBrowserActorUi.panels), onIntent: browserActorPanelIntent, onAction: onActionStable },
    [browserActorPanelIntent, browserActorUiVersion, currentBrowserActorUi, onActionStable, session],
  );

  const focusedBrowserActorPanels = focusedSpawnedId === null ? browserActorPanels : null;
  const activeRightPanelTab = focusedApp?.panelTabs.find((tab) => panelAnchorForGroup(tab.group) === "top-right");
  const activePanelTabId = panel?.activePanelTab ?? (activeRightPanelTab ? panelTabKindId(activeRightPanelTab.kind) : undefined) ?? (focusedApp?.panelTabs[0] ? panelTabKindId(focusedApp.panelTabs[0].kind) : undefined);

  const workbenchLeftTabs = useMemo((): PanelTabNode[] => {
    if (!session || !focusedApp) return [];
    const pluginLeftTabs = focusedApp.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-left").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels));
    if (hostMode && focusedApp.id === hostAppId && pluginLeftTabs.length > 0) return pluginLeftTabs;
    const hasPluginArtifactTab = flattenPanelTabNodes(pluginLeftTabs).some((tab) => tab.id === FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    if (hasPluginArtifactTab) return pluginLeftTabs;
    const openWithSection = (role: AppRole, entries: readonly OpenWithEntry[]) => ({
      id: `artifact.openWith.${role}`,
      label: `${openArtifactWithText(uiLocale)} — ${surfaceRoleChipText(role, uiLocale)}`,
      defaultOpen: openWithFocusRole == null || openWithFocusRole === role,
      items: entries.map((entry) => ({
        id: `artifact.openWith.${role}.${entry.app.pluginId}.${entry.app.appId}`,
        label: entry.current ? `${entry.pluginLabel} ✓` : entry.pluginLabel,
        onClick: entry.current ? undefined : () => void openArtifactWithAppRef(entry.app, session.app.dialect, role),
        control: (
          <button
            type="button"
            aria-pressed={entry.isDefault}
            onClick={(event) => {
              event.stopPropagation();
              if (entry.isDefault) dispatchClearDefaultApp(session.app.dialect, role);
              else dispatchSetDefaultApp(session.app.dialect, role, entry.app);
            }}
          >
            {entry.isDefault ? "★" : "☆"} {setAsDefaultText(uiLocale)}
          </button>
        ),
      })),
    });
    const artifactTab = singleTreeLeaf({
      id: FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
      icon: shellTabIcon(FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID),
      name: shellLabel("ui.panel.artifact"),
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: "artifact.root",
            label: shellLabel("ui.panel.artifact"),
            items: [{ id: "artifact.empty", label: hostMode ? `${panel?.spawnedApps.length ?? 0} ${shellLabel("ui.panel.spawnedAppsSuffix")}` : shellLabel("ui.panel.artifactEmpty") }],
          },
          ...(openWithEntries && openWithEntries.viewer.length > 0 ? [openWithSection("viewer", openWithEntries.viewer)] : []),
          ...(openWithEntries && openWithEntries.editor.length > 0 ? [openWithSection("editor", openWithEntries.editor)] : []),
        ],
      }),
    });
    return [artifactTab, ...pluginLeftTabs];
  }, [appLabelsOverlay, focusedBrowserActorPanels, onAction, panel?.spawnedApps.length, focusedPanelBodyStores, session, focusedApp, hostMode, uiLocale, uiTerminology, hostAppId, openWithEntries, openWithFocusRole, openArtifactWithAppRef, dispatchSetDefaultApp, dispatchClearDefaultApp, treeWindowHost, treeWindowGeneration]);

  const detailsRightTabs = useMemo((): PanelTabNode[] => {
    if (!session || !focusedApp) return [];
    return focusedApp.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-right").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels));
  }, [appLabelsOverlay, focusedBrowserActorPanels, onAction, focusedPanelBodyStores, session, focusedApp, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration]);

  /**
   * 🧭️ The two bottom anchors' app-declared tabs — `PanelGroup::Display` → `bottom-left`,
   * `PanelGroup::Settings` → `bottom-right`. Without these an app could declare either group and the
   * dock assembly dropped the tab silently: the guest still rendered the body (every declared tab is
   * flattened into `buildUiRefreshRequest`) and the result was cached as a panel body and then never
   * mounted, which is exactly how puzzle3d's own Settings section — grid spacing, chunk size,
   * proximity radius, overlap budget — was unreachable while the framework's own `framework.settings`
   * branch beside it opened fine.
   *
   * 🕰️ {@link partitionFrameworkHistoryPanelTab} keeps the framework-injected `framework.panel.history` out: the
   * dock mounts it as its own bottom-right leaf ({@link frameworkHistoryTab}) rather than inside the Settings branch.
   */
  const appTabsForBottomAnchor = useCallback(
    (anchor: ReturnType<typeof panelAnchorForGroup>): PanelTabNode[] =>
      session && focusedApp
        ? partitionFrameworkHistoryPanelTab(focusedApp.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === anchor)).rest
            .map((tab, order) => panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels))
        : [],
    [appLabelsOverlay, focusedBrowserActorPanels, onAction, focusedPanelBodyStores, session, focusedApp, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration],
  );
  const displayBottomLeftTabs = useMemo(() => appTabsForBottomAnchor("bottom-left"), [appTabsForBottomAnchor]);
  const settingsBottomRightTabs = useMemo(() => appTabsForBottomAnchor("bottom-right"), [appTabsForBottomAnchor]);
  const settingsBottomRightDockTab = useMemo(
    () => integrateAppSettingsPanelTabsIntoFrameworkBranch(frameworkSettingsTab, settingsBottomRightTabs),
    [frameworkSettingsTab, settingsBottomRightTabs],
  );

  return {
    revealAgentApproval, currentDocumentRuntimeKey, currentLocalDocumentOwner, currentBrowserActorUi,
    focusedBrowserActorPanels, workbenchLeftTabs, detailsRightTabs, displayBottomLeftTabs, settingsBottomRightDockTab
  };
  //#endregion FrameworkOsShell
}

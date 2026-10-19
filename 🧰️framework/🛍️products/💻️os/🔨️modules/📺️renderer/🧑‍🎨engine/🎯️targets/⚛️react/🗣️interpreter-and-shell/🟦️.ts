/** 🗣️ Interpreter and OS shell exports of the renderer entry: the UI interpreter and document store, the shell store, helpers, boot, error boundary, shell component, plugin runtime, host ports, wasm session loader, search, find, sync attach card, utility tree and chrome panels. */
import { nodeGraphActions, textEditorActions, type PluginRegistryEntry, IMPORT_CHUNK_BYTES, importPayloadChunks } from "@semio-tech/framework";
import { type UiInterpreterContext, usePluginSurfaceActions, PluginSurfaceActionsContext } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/interpreter-context";
import { uiNodeDomId } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/stable-dom-ids";
import { interpretUiNode, InterpretedUiNode } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter";
import { UiNodeView } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/node-tree";
import { UiPresenceOverlayContext, usePresenceOverlayEntry, type UiPresenceOverlayEntry, type UiPresenceOverlayValue } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/presence-overlay";
import { registerElementHostsV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/ElementHosts";
import { type FrameworkOsBootOptions, type FrameworkOsBootExecution } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell";
import { uiInspectorMixedPlaceholder, type PluginPanelStatus, type PluginSupervisorState } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/program-sessions";
import { type FrameworkOsLocks, type ResolvedShellLocks, resolveShellLocks, mergeShellLockSources, type FrameworkOsDefaults, resolveShellDefaults, resolveBootExampleId, shouldReplayIntroductionOnLoad, shouldPersistIntroductionSeen, isEphemeralShellBrand, clearDurableShellStorage } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/boot-and-locks";
import { type ShellPresencePeer, type PeerInteractionDomain, type PeerInteractionRoster, derivePeerInteractionByDomain, peerIdsSelecting, peerIdsHovering } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/peer-interaction";
import type { ViewModel, PluginManifest } from "@semio-tech/framework-os/modules/renderer/engine/elements/PluginRuntime/handle-contract";
import { actionStageKey, type CommandPanelState, type ShellState } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/state-shapes";
import { type ShellAction } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/shell-actions";
import { shellReducer, shellStateUnchanged, selectUiDevice, selectOpenConflicts, selectQuarantinedConflicts, initialShellState } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/shell-store";
import { NOTE_WORLD_NAVIGATION_ACTION_ID, buildNoteShellCommandAction, isShellOwnedCommandId, TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/plumbing/action-ids";
import { encodeEffectActionInvocation, encodeEffectCommandInvocation } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/plumbing/effect-dispatch";
import { dispatchOpenedFiles, scheduleDispatchAction, sampleMediaFrameTimestampsMs, runTier2VideoFrames, type RequestMediaFramesArgs, requestMediaFramesSourceV1, runMediaFramesV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/media-import";
import { type ShellRoute, parseShellRoute, pluginAvailabilityRouteV1, type PluginAvailabilityRouteV1, pluginShouldReceiveContributions, pluginShouldEstablishSession, appBreadcrumb, resolveAppBreadcrumb, resolveArtifactByAppId, appWindowLabel, retitleWindowLayoutNode, resolveFrameworkLayoutSeed, classifyWindowLayoutChange, flattenPanelTabLeaves, panelTabDefinitionToNode, panelAnchorForGroup, integrateAppSettingsPanelTabsIntoFrameworkBranch, partitionFrameworkHistoryPanelTab, spawnedWindowChromeForKind } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/route-and-chrome";
import { studioPanelFocusingSpawned, viewStateWithSpacePanel, type SpawnedAppEntry, type SpacePanelState } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/panel";
import { uiIntentToActionDescriptor, resolveUtilities, resolveUtilityNodes, uiNodeToTreePanelConfig } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/tree-windows";
import { type SelectionUtilityOptions } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/window-panes/utility-bar-options";
import { synthesizeLocalizedLabel, resolveManifestLabel, shellLabel, shellTabIcon, syncShellLabelLocale, shellTerminologyLabel } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/labels";
import { driverDisplayLabel, type ResolvedCommand, commandAddressKey, resolveCommands, commandCategories, buildOsCommands, dispatchOsCommand, buildCommandCategoryTree, buildCommandCategoryTabs, buildToolTabs, toolCategoryOpenPath, toolLeafInactiveRepress, toolIdFromPanelTabId, reconcileToolTabSelection, toolPanelTreeContentRevision, type ToolTabSelection, type ToolTabSelectionEffect } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/command-registry";
import { DEFAULT_PANEL_WIDTH_PX } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/plumbing";
import { createLatestAsyncDispatcher, createDirectionalAsyncDispatcher, createInFlightSkippingInterval, createCoalescingActionDispatcher, PUZZLE3D_MESH_COMMAND_RAW_BYTES, PUZZLE3D_MESH_PAGE_VALUES, type Puzzle3dBrushMeshPage, puzzle3dBrushMeshDigest, puzzle3dBrushMeshPages, type Puzzle3dBrushMeshQueueStep, puzzle3dBrushMeshQueueStep, drainPuzzle3dBrushMeshQueue, Puzzle3dBrushMeshRegistry, puzzle3dBrushMeshRegistry } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/extension-io";
import { windowMeasureTreeContainsId, windowMeasureDomId, qualifyWindowMeasureIds, renderWindowMeasuresTree } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/window-panes/window-measures";
import { renderStagedArgControl, actionRequiresStagedForm } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/window-panes/staged-args";
import { isEditableEventTarget, keyboardEventMatchesChord, type KeybindingIntent, resolveKeybindingIntent } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/window-panes/keyboard-chords";
import { resolveUtilityActivation } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/window-panes/clipboard-activation";
import { actionCategoryId, actionCategories, buildActionCategoryTree, type WindowActionPaneProps, WindowActionPane } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/window-panes";
import { preserveJsonIdentity, mergeRecordPreservingIdentity, type UiRefreshCache, introductionTargetsWindow, buildActiveUtilityByWindowId, buildUiRefreshRequest, applyUiRefreshResponseToCache } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/ui-refresh";
import { AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD, AutoCheckinScheduler, canCheckIn, type SyncPillState, computeSyncPillState, syncPillText } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/transfer-checkin-tutorial";
import { bootFrameworkOs } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell";
import { ShellFaultBoundary } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/fault-boundary";
import { SetWindowTitleContext, SetWindowIconContext, TutorialRecorder, type FrameworkOsShellProps, FrameworkOsShell, shellActorId, canonicalSurfaceId, reloadRetainsActiveApp, directoryCommandFromAction, mintDirectoryCommandRequestId, retainDirectoryCommandResult, DIRECTORY_COMMAND_RESULT_SLOTS, type DirectoryCommandResultSlotV1, type DirectoryCommandReceiptV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHost";
import { type PluginDispatchHintV1, type PluginOperationCompletion, type PluginWasmHandle, loadPluginModule, adaptPluginHandle, fetchDescriptorManifest, resolveDescriptorBeforeRuntime, applyUiPatchToRetained, decodeWirePatchOps, serializeCommandIngressForActor, serializePerActor } from "@semio-tech/framework-os/modules/renderer/engine/elements/PluginRuntime";
import { createDemandFrameScheduler } from "@semio-tech/framework/canvas/react-renderer";
import { createGraphSession, type FlowWasmSession, createFlowSession, type EditorWasmSession, createEditorSession, type RasterWasmSession, createRasterSession, type MapWasmSession, createMapSession, type TerrainWasmSession, createTerrainSession, BoardSessionFactoryContext, registerWasmBindingsV1, type WasmBindingsV1, resolveAppSurfaceSessionFactory, isFlowGraphScene } from "@semio-tech/framework-os/modules/renderer/engine/elements/WasmSessionLoader";
import { type UISearchItem, UISearch, type UIFindItem, type UIFindContextValue, UIFindProvider, useUIFind, useUIFindSafe, UIFind } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellSearch";
import { SyncAttachCard } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellSync";
import { sortUtilityNodes, UTILITY_CATEGORIES, UTILITY_CATEGORY_ICON_ID, groupUtilityNodesByCategory, dedupeUtilityNodesById, type UtilityRibbonSegment, buildUtilityRibbonSegments, reconcileUtilityPath, findPressedUtilityLeafId, findUtilityGroupPath, UtilityTree } from "@semio-tech/framework-os/modules/renderer/engine/elements/UtilityTree";
import { createFrameworkSettingsPanelTab, useNamedLayoutHost, ShellRouteNotFoundPage, PluginRecoveryPanel } from "@semio-tech/framework-os/modules/renderer/engine/elements/ChromePanels";
import { FRAMEWORK_SETTINGS_PANEL_ID, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, FRAMEWORK_SETTINGS_THEME_TAB_ID, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID } from "@semio-tech/framework-os/modules/renderer/engine/elements/ChromePanels/panel-ids";
import { type DisplayHostApi, createFrameworkDisplayPanelTabs } from "@semio-tech/framework-os/modules/renderer/engine/elements/ChromePanels/display-panel";
import { type SettingsHostApi } from "@semio-tech/framework-os/modules/renderer/engine/elements/ChromePanels/settings-general";
import { type MarketplacePluginEntry, type MarketplaceExtensionEntry, type MarketplaceHostApi, FRAMEWORK_MARKETPLACE_TAB_ID, createFrameworkMarketplacePanelTab } from "@semio-tech/framework-os/modules/renderer/engine/elements/ChromePanels/marketplace-panel";
import { useShellContextMenuFallback, openSurfaceContextMenu, surfaceContextMenuTitleKey, type SurfaceContextMenuResult, ShellContextMenuFallbackContext, useAppKeybindingsByActionId, useMapContextMenuSpecs } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { wireLabel, parseSceneJsonField } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/scene-host-contract";

//#region 🔖️UiInterpreter
// 🧭️ MIGRATION (semantic UI contract, ticket 26/08/20): `renderUiControl`/`uiTreeNodeToTreePanelConfig`/
// `declarativeTreeDragController`/`declarativeSurfaceStatus` are gone — they existed only for the old
// `UiControlNode`/`UiTreeNode` union, which no longer exists (every control is its own `Component`
// variant now; tree row/section conversion is internal to `🟦️Interpreter`'s `TreeView`; a node's status
// is just `UiNodeRecord.activity`). `UiNodeView`/`UiPresenceOverlay*` are new: `UiNodeView` is the
// atomic per-node subscribing render unit, `UiPresenceOverlay*` is the hover/selection channel that
// deliberately does NOT live on the document store (see `🟦️Interpreter`'s own header doc).
export {
  type UiInterpreterContext,
  usePluginSurfaceActions,
  useShellContextMenuFallback,
  openSurfaceContextMenu,
  surfaceContextMenuTitleKey,
  type SurfaceContextMenuResult,
  uiNodeDomId,
  interpretUiNode,
  InterpretedUiNode,
  UiNodeView,
  wireLabel,
  parseSceneJsonField,
  PluginSurfaceActionsContext,
  ShellContextMenuFallbackContext,
  UiPresenceOverlayContext,
  usePresenceOverlayEntry,
  type UiPresenceOverlayEntry,
  type UiPresenceOverlayValue,
};
export { builtNodeToSnapshot, UiDocumentStore, emitIntent, useUiNode, useUiDocumentRoot, useUiDocumentRevision, type UiDocumentState } from "@semio-tech/framework-os/modules/renderer/engine/elements/UiDocumentStore";

/** 🪢️ The renderer package is a composition root of the element hosts: the Interpreter resolves every scene and extension host through its registry. */
registerElementHostsV1();
//#endregion 🔖️UiInterpreter

//#region 🔖️OsShell
/** 🔁️ Re-exported so `node-graph-host.tsx`/`text-editor-host.tsx` can import action-name maps from this shell module rather than reaching into `@semio-tech/framework` directly. */
export { nodeGraphActions, textEditorActions };

//#region 🔖️types
export {
  uiInspectorMixedPlaceholder,
  type ViewModel,
  type PluginManifest,
  type PluginPanelStatus,
  type PluginSupervisorState,
  type SpawnedAppEntry,
  type SpacePanelState,
  type FrameworkOsBootOptions,
  type FrameworkOsBootExecution,
  type FrameworkOsLocks,
  type ResolvedShellLocks,
  resolveShellLocks,
  mergeShellLockSources,
  type FrameworkOsDefaults,
  resolveShellDefaults,
  resolveBootExampleId,
  shouldReplayIntroductionOnLoad,
  shouldPersistIntroductionSeen,
  isEphemeralShellBrand,
  clearDurableShellStorage,
  type ShellPresencePeer,
  type PeerInteractionDomain,
  type PeerInteractionRoster,
  derivePeerInteractionByDomain,
  peerIdsSelecting,
  peerIdsHovering,
};
//#endregion 🔖️types

//#region 🧮️ShellStore
export { actionStageKey, type CommandPanelState, type ShellState, type ShellAction, shellReducer, shellStateUnchanged, selectUiDevice, selectOpenConflicts, selectQuarantinedConflicts, initialShellState };
//#endregion 🧮️ShellStore

//#region ShellHelpers
export type {
  ResolvedActionArgDef,
  ResolvedActionDefinition,
  ResolvedToolDefinition
} from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/labels";
export type { ShellCommandInverse } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHelpers/plumbing/action-ids";
export { NOTE_WORLD_NAVIGATION_ACTION_ID, buildNoteShellCommandAction, isShellOwnedCommandId, encodeEffectActionInvocation, encodeEffectCommandInvocation, TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS, dispatchOpenedFiles, IMPORT_CHUNK_BYTES, importPayloadChunks, scheduleDispatchAction, sampleMediaFrameTimestampsMs, runTier2VideoFrames, type RequestMediaFramesArgs, requestMediaFramesSourceV1, runMediaFramesV1, type ShellRoute, parseShellRoute, appBreadcrumb, resolveAppBreadcrumb, resolveArtifactByAppId, appWindowLabel, studioPanelFocusingSpawned, viewStateWithSpacePanel, retitleWindowLayoutNode, resolveFrameworkLayoutSeed, classifyWindowLayoutChange, flattenPanelTabLeaves, panelTabDefinitionToNode, panelAnchorForGroup, integrateAppSettingsPanelTabsIntoFrameworkBranch, partitionFrameworkHistoryPanelTab, uiIntentToActionDescriptor, resolveUtilities, resolveUtilityNodes, type SelectionUtilityOptions, spawnedWindowChromeForKind, uiNodeToTreePanelConfig, synthesizeLocalizedLabel, resolveManifestLabel, shellLabel, shellTabIcon, syncShellLabelLocale, shellTerminologyLabel, driverDisplayLabel, DEFAULT_PANEL_WIDTH_PX, createLatestAsyncDispatcher, createDirectionalAsyncDispatcher, createInFlightSkippingInterval, createCoalescingActionDispatcher, PUZZLE3D_MESH_COMMAND_RAW_BYTES, PUZZLE3D_MESH_PAGE_VALUES, type Puzzle3dBrushMeshPage, puzzle3dBrushMeshDigest, puzzle3dBrushMeshPages, type Puzzle3dBrushMeshQueueStep, puzzle3dBrushMeshQueueStep, drainPuzzle3dBrushMeshQueue, Puzzle3dBrushMeshRegistry, puzzle3dBrushMeshRegistry, windowMeasureTreeContainsId, windowMeasureDomId, qualifyWindowMeasureIds, renderWindowMeasuresTree, renderStagedArgControl, actionRequiresStagedForm, isEditableEventTarget, keyboardEventMatchesChord, type KeybindingIntent, resolveKeybindingIntent, resolveUtilityActivation, actionCategoryId, actionCategories, buildActionCategoryTree, type WindowActionPaneProps, WindowActionPane, type ResolvedCommand, commandAddressKey, resolveCommands, commandCategories, buildOsCommands, dispatchOsCommand, buildCommandCategoryTree, buildCommandCategoryTabs, buildToolTabs, toolCategoryOpenPath, toolLeafInactiveRepress, toolIdFromPanelTabId, reconcileToolTabSelection, toolPanelTreeContentRevision, type ToolTabSelection, type ToolTabSelectionEffect, preserveJsonIdentity, mergeRecordPreservingIdentity, type UiRefreshCache, introductionTargetsWindow, buildActiveUtilityByWindowId, buildUiRefreshRequest, applyUiRefreshResponseToCache, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD, AutoCheckinScheduler, canCheckIn, type SyncPillState, computeSyncPillState, syncPillText };
export { pluginAvailabilityRouteV1, type PluginAvailabilityRouteV1 };
export { pluginShouldEstablishSession };
export { pluginShouldReceiveContributions };
//#endregion ShellHelpers

//#region Boot
export { bootFrameworkOs };
//#endregion Boot

//#region ErrorBoundary
export { ShellFaultBoundary };
//#endregion ErrorBoundary

//#region FrameworkOsShell
export {
  SetWindowTitleContext,
  SetWindowIconContext,
  useAppKeybindingsByActionId,
  useMapContextMenuSpecs,
  TutorialRecorder,
  type FrameworkOsShellProps,
  FrameworkOsShell,
  shellActorId,
  canonicalSurfaceId,
  reloadRetainsActiveApp,
  directoryCommandFromAction,
  mintDirectoryCommandRequestId,
  retainDirectoryCommandResult,
  DIRECTORY_COMMAND_RESULT_SLOTS,
  type DirectoryCommandResultSlotV1,
  type DirectoryCommandReceiptV1,
};
//#endregion FrameworkOsShell

//#region 🔖️plugin-runtime
export { type PluginDispatchHintV1, type PluginOperationCompletion, type PluginWasmHandle, loadPluginModule, adaptPluginHandle, fetchDescriptorManifest, resolveDescriptorBeforeRuntime, applyUiPatchToRetained, decodeWirePatchOps, serializeCommandIngressForActor, serializePerActor };
export type { PluginRegistryEntry };
//#endregion 🔖️plugin-runtime

//#region 🔖️host-ports
export type { CatalogAdmissionPortV1, DocumentAccessPortV1, PluginDistributionPortV1, ShellHostPortsV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellHost/host-ports";
export type { UiPreferenceVocabularyV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/UiPreferences";
//#endregion 🔖️host-ports

//#region 🔖️wasm-session-loader
export {
  createDemandFrameScheduler,
  createGraphSession,
  type FlowWasmSession,
  createFlowSession,
  type EditorWasmSession,
  createEditorSession,
  type RasterWasmSession,
  createRasterSession,
  type MapWasmSession,
  createMapSession,
  type TerrainWasmSession,
  createTerrainSession,
  BoardSessionFactoryContext,
  registerWasmBindingsV1,
  type WasmBindingsV1,
  resolveAppSurfaceSessionFactory,
  isFlowGraphScene,
};
//#endregion 🔖️wasm-session-loader

//#region 🔖️ui-search-find
export { type UISearchItem, UISearch, type UIFindItem, type UIFindContextValue, UIFindProvider, useUIFind, useUIFindSafe, UIFind };
//#endregion 🔖️ui-search-find

//#region 🔖️sync-attach-card
export { SyncAttachCard };
//#endregion 🔖️sync-attach-card

//#region 🔖️utility-tree
export {
  sortUtilityNodes,
  UTILITY_CATEGORIES,
  UTILITY_CATEGORY_ICON_ID,
  groupUtilityNodesByCategory,
  dedupeUtilityNodesById,
  type UtilityRibbonSegment,
  buildUtilityRibbonSegments,
  reconcileUtilityPath,
  findPressedUtilityLeafId,
  findUtilityGroupPath,
  UtilityTree,
};
//#endregion 🔖️utility-tree

//#region 🔖️os-chrome-panels
export {
  type DisplayHostApi,
  createFrameworkDisplayPanelTabs,
  type SettingsHostApi,
  createFrameworkSettingsPanelTab,
  FRAMEWORK_SETTINGS_PANEL_ID,
  FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
  FRAMEWORK_SETTINGS_THEME_TAB_ID,
  FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID,
  useNamedLayoutHost,
  ShellRouteNotFoundPage,
  PluginRecoveryPanel,
  type MarketplacePluginEntry,
  type MarketplaceExtensionEntry,
  type MarketplaceHostApi,
  FRAMEWORK_MARKETPLACE_TAB_ID,
  createFrameworkMarketplacePanelTab,
};
//#endregion 🔖️os-chrome-panels
//#endregion 🔖️OsShell

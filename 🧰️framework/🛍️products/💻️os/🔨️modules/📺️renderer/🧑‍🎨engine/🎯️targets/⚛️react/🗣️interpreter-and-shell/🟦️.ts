/** 🗣️ Interpreter and OS shell exports of the renderer entry: the UI interpreter and document store, the shell store, helpers, boot, error boundary, shell component, plugin runtime, host ports, wasm session loader, search, find, sync attach card, utility tree and chrome panels. */
import { nodeGraphActions, textEditorActions, type PluginRegistryEntry, IMPORT_CHUNK_BYTES, importPayloadChunks } from "@semio-tech/framework";
import { type UiInterpreterContext, usePluginSurfaceActions, PluginSurfaceActionsContext } from "../../../🧱️elements/🗣️Interpreter/🧷️interpreter-context/🟦️.ts";
import { uiNodeDomId } from "../../../🧱️elements/🗣️Interpreter/🪪️stable-dom-ids/🟦️.ts";
import { interpretUiNode, InterpretedUiNode } from "../../../🧱️elements/🗣️Interpreter/🟦️.tsx";
import { UiNodeView } from "../../../🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx";
import { UiPresenceOverlayContext, usePresenceOverlayEntry, type UiPresenceOverlayEntry, type UiPresenceOverlayValue } from "../../../🧱️elements/🗣️Interpreter/👥️presence-overlay/🟦️.ts";
import { registerElementHostsV1 } from "../../../🧱️elements/🪢️ElementHosts/🟦️.ts";
import { type SpawnedAppEntry, type SpacePanelState, type FrameworkOsBootOptions, type FrameworkOsBootExecution } from "../../../🧱️elements/🐚️Shell/🟦️.tsx";
import { uiInspectorMixedPlaceholder, type PluginPanelStatus, type PluginSupervisorState } from "../../../🧱️elements/🐚️Shell/📦️program-sessions/🟦️.ts";
import { type FrameworkOsLocks, type ResolvedShellLocks, resolveShellLocks, mergeShellLockSources, type FrameworkOsDefaults, resolveShellDefaults, resolveBootExampleId, shouldReplayIntroductionOnLoad, shouldPersistIntroductionSeen, isEphemeralShellBrand, clearDurableShellStorage } from "../../../🧱️elements/🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { type ShellPresencePeer, type PeerInteractionDomain, type PeerInteractionRoster, derivePeerInteractionByDomain, peerIdsSelecting, peerIdsHovering } from "../../../🧱️elements/🐚️Shell/👥️peer-interaction/🟦️.ts";
import type { ViewModel, PluginManifest } from "../../../🧱️elements/🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { actionStageKey, type CommandPanelState, type ShellState } from "../../../🧱️elements/🐚️Shell/🗃️state-shapes/🟦️.ts";
import { type ShellAction } from "../../../🧱️elements/🐚️Shell/🎛️shell-actions/🟦️.ts";
import { shellReducer, shellStateUnchanged, selectUiDevice, selectOpenConflicts, selectQuarantinedConflicts, initialShellState } from "../../../🧱️elements/🐚️Shell/🕹️shell-store/🟦️.ts";
import { NOTE_WORLD_NAVIGATION_ACTION_ID, buildNoteShellCommandAction, isShellOwnedCommandId, TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { encodeEffectActionInvocation, encodeEffectCommandInvocation } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🚀️effect-dispatch/🟦️.ts";
import { dispatchOpenedFiles, scheduleDispatchAction, sampleMediaFrameTimestampsMs, runTier2VideoFrames, type RequestMediaFramesArgs, requestMediaFramesSourceV1, runMediaFramesV1 } from "../../../🧱️elements/🛠️ShellHelpers/🎞️media-import/🟦️.ts";
import { type ShellRoute, parseShellRoute, pluginAvailabilityRouteV1, type PluginAvailabilityRouteV1, pluginShouldReceiveContributions, pluginShouldEstablishSession, appBreadcrumb, resolveAppBreadcrumb, resolveArtifactByAppId, appWindowLabel, retitleWindowLayoutNode, resolveFrameworkLayoutSeed, classifyWindowLayoutChange, flattenPanelTabLeaves, panelTabDefinitionToNode, panelAnchorForGroup, integrateAppSettingsPanelTabsIntoFrameworkBranch, partitionFrameworkHistoryPanelTab, spawnedWindowChromeForKind } from "../../../🧱️elements/🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { studioPanelFocusingSpawned, viewStateWithSpacePanel } from "../../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts";
import { uiIntentToActionDescriptor, resolveUtilities, resolveUtilityNodes, uiNodeToTreePanelConfig } from "../../../🧱️elements/🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import { type SelectionUtilityOptions } from "../../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/🪛️utility-bar-options/🟦️.tsx";
import { synthesizeLocalizedLabel, resolveManifestLabel, shellLabel, shellTabIcon, syncShellLabelLocale, shellTerminologyLabel } from "../../../🧱️elements/🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { driverDisplayLabel, type ResolvedCommand, commandAddressKey, resolveCommands, commandCategories, buildOsCommands, dispatchOsCommand, buildCommandCategoryTree, buildCommandCategoryTabs, buildToolTabs, toolCategoryOpenPath, toolLeafInactiveRepress, toolIdFromPanelTabId, reconcileToolTabSelection, toolPanelTreeContentRevision, type ToolTabSelection, type ToolTabSelectionEffect } from "../../../🧱️elements/🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { DEFAULT_PANEL_WIDTH_PX } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { createLatestAsyncDispatcher, createDirectionalAsyncDispatcher, createInFlightSkippingInterval, createCoalescingActionDispatcher, PUZZLE3D_MESH_COMMAND_RAW_BYTES, PUZZLE3D_MESH_PAGE_VALUES, type Puzzle3dBrushMeshPage, puzzle3dBrushMeshDigest, puzzle3dBrushMeshPages, type Puzzle3dBrushMeshQueueStep, puzzle3dBrushMeshQueueStep, drainPuzzle3dBrushMeshQueue, Puzzle3dBrushMeshRegistry, puzzle3dBrushMeshRegistry } from "../../../🧱️elements/🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { windowMeasureTreeContainsId, windowMeasureDomId, qualifyWindowMeasureIds, renderWindowMeasuresTree } from "../../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/🎚️window-measures/🟦️.tsx";
import { renderStagedArgControl, actionRequiresStagedForm } from "../../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx";
import { isEditableEventTarget, keyboardEventMatchesChord, type KeybindingIntent, resolveKeybindingIntent } from "../../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";
import { resolveUtilityActivation } from "../../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts";
import { actionCategoryId, actionCategories, buildActionCategoryTree, type WindowActionPaneProps, WindowActionPane } from "../../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/🟦️.tsx";
import { preserveJsonIdentity, mergeRecordPreservingIdentity, type UiRefreshCache, introductionTargetsWindow, buildActiveUtilityByWindowId, buildUiRefreshRequest, applyUiRefreshResponseToCache } from "../../../🧱️elements/🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD, AutoCheckinScheduler, canCheckIn, type SyncPillState, computeSyncPillState, syncPillText } from "../../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { bootFrameworkOs } from "../../../🧱️elements/🐚️Shell/🟦️.tsx";
import { ShellFaultBoundary } from "../../../🧱️elements/🐚️Shell/🟦️.tsx";
import { SetWindowTitleContext, SetWindowIconContext, TutorialRecorder, type FrameworkOsShellProps, FrameworkOsShell, shellActorId, canonicalSurfaceId, reloadRetainsActiveApp, directoryCommandFromAction, mintDirectoryCommandRequestId, retainDirectoryCommandResult, DIRECTORY_COMMAND_RESULT_SLOTS, type DirectoryCommandResultSlotV1, type DirectoryCommandReceiptV1 } from "../../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import { type PluginDispatchHintV1, type PluginOperationCompletion, type PluginWasmHandle, loadPluginModule, adaptPluginHandle, fetchDescriptorManifest, resolveDescriptorBeforeRuntime, applyUiPatchToRetained, decodeWirePatchOps, serializeCommandIngressForActor, serializePerActor } from "../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { createDemandFrameScheduler } from "@semio-tech/framework/canvas/react-renderer";
import { createGraphSession, type FlowWasmSession, createFlowSession, type EditorWasmSession, createEditorSession, type RasterWasmSession, createRasterSession, type MapWasmSession, createMapSession, type TerrainWasmSession, createTerrainSession, type Board2dWasmSession, type AppSurfaceSessionFactory, BoardSessionFactoryContext, createWasmModuleLoader, registerWasmBindingsV1, type WasmBindingsV1, resolveAppSurfaceSessionFactory, isFlowGraphScene } from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { type UISearchItem, UISearch, type UIFindItem, type UIFindContextValue, UIFindProvider, useUIFind, useUIFindSafe, UIFind } from "../../../🧱️elements/🔎️ShellSearch/🟦️.tsx";
import { SyncAttachCard } from "../../../🧱️elements/🔄️ShellSync/🟦️.tsx";
import { sortUtilityNodes, UTILITY_CATEGORIES, UTILITY_CATEGORY_ICON_ID, groupUtilityNodesByCategory, dedupeUtilityNodesById, type UtilityRibbonSegment, buildUtilityRibbonSegments, reconcileUtilityPath, findPressedUtilityLeafId, findUtilityGroupPath, UtilityTree } from "../../../🧱️elements/🎛️UtilityTree/🟦️.tsx";
import { createFrameworkSettingsPanelTab, useNamedLayoutHost, ShellRouteNotFoundPage, PluginRecoveryPanel } from "../../../🧱️elements/📌️ChromePanels/🟦️.tsx";
import { FRAMEWORK_SETTINGS_PANEL_ID, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, FRAMEWORK_SETTINGS_THEME_TAB_ID, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID } from "../../../🧱️elements/📌️ChromePanels/🆔️panel-ids/🟦️.ts";
import { type DisplayHostApi, createFrameworkDisplayPanelTabs } from "../../../🧱️elements/📌️ChromePanels/🖥️display-panel/🟦️.tsx";
import { type SettingsHostApi } from "../../../🧱️elements/📌️ChromePanels/⚙️settings-general/🟦️.tsx";
import { type MarketplacePluginEntry, type MarketplaceExtensionEntry, type MarketplaceHostApi, FRAMEWORK_MARKETPLACE_TAB_ID, createFrameworkMarketplacePanelTab } from "../../../🧱️elements/📌️ChromePanels/🏪️marketplace-panel/🟦️.tsx";
import { useShellContextMenuFallback, openSurfaceContextMenu, surfaceContextMenuTitleKey, type SurfaceContextMenuResult, ShellContextMenuFallbackContext, useAppKeybindingsByActionId, useMapContextMenuSpecs } from "../../../🧱️elements/🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { wireLabel, parseSceneJsonField } from "../../../🧱️elements/🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";

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
export { builtNodeToSnapshot, UiDocumentStore, emitIntent, useUiNode, useUiDocumentRoot, useUiDocumentRevision, type UiDocumentState } from "../../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";

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
} from "../../../🧱️elements/🛠️ShellHelpers/🏷️labels/🟦️.tsx";
export type { ShellCommandInverse } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
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
export type { CatalogAdmissionPortV1, DocumentAccessPortV1, PluginDistributionPortV1, ShellHostPortsV1 } from "../../../🧱️elements/🏛️ShellHost/🪝️host-ports/🟦️.ts";
export type { UiPreferenceVocabularyV1 } from "../../../🧱️elements/🎚️UiPreferences/🟦️.ts";
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
  type Board2dWasmSession,
  type AppSurfaceSessionFactory,
  BoardSessionFactoryContext,
  createWasmModuleLoader,
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

// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/component.tsx
/** @emoji 🏗️ `ShellHost` — the `FrameworkOsShell` orchestrator: boots/hot-swaps plugin wasm modules,
 * owns the window/dock/panel layout, wires the tutorial recorder/player, presence, backbone sync,
 * command/tool/utility ribbons, context menus, and mounts every per-app window via `🟦️Interpreter`.
 * The single largest component in the renderer-react package. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createAdmittedShellInstanceV1, shellDialogOriginIsCurrentV1, shellDialogOriginV1, shellDialogSessionIsCurrentV1, shellEffectSourceIsCurrentV1, type ShellDialogOriginV1, type ShellDialogV1 } from "./🗨️dialog-origin/🟦️.ts";
import { OwnedShellDialog } from "./🗨️dialog-origin/🌐️browser/🟦️.tsx";
import { admitDocumentOpeningV1, BackgroundDocumentSessionsV1, browserDocumentMountIsCurrentV1, DocumentAttachmentLaneV1, LatestDocumentReplacementV1, parkDocumentOpeningReplacementV1, runDocumentOpeningAttemptV1, settleDocumentOpeningReplacementV1, type DocumentOpeningReceiptV1 } from "./🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { createArtifactCreationCatalogMountV1, runArtifactCreationReadyOpeningV1, type ArtifactCreationCatalogMountV1 } from "./🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { directorySessionAuthorityIsCurrentV1, startDirectorySessionRefreshV1, type DirectorySessionRefreshV1 } from "../../../../📇️directory/🪪️session-refresh/🟦️.ts";
import { directoryAdministrationCommandAllowedV1 } from "../../../../📇️directory/🧬️schema/🟦️.ts";
import { BrowserBrokerPortClientV1 } from "../../../../📇️directory/🪪️session-refresh/🌐️broker-port/🟦️.ts";
import { SessionAuthorityNotice } from "../../../../📇️directory/🪪️session-refresh/🪪️notice/🟦️.tsx";
import { OwnedTutorialRunV1, TutorialDriveV1, runPausedTutorialSeekV1 } from "./🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import React, {
  createContext,
  type CSSProperties,
  type KeyboardEvent,
  type MouseEvent,
  type ReactElement,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useLayoutEffect,
  useMemo,
  useReducer,
  useRef,
  useState,
} from "react";
import {
  type ActionDescriptor,
  type ActionInvocation,
  type AppDefinition,
  type AppRef,
  type AppRole,
  AppRouter,
  type AppRouterManifest,
  type ArtifactDialect,
  type ArtifactKindChoice,
  dialectCoordinate,
  decodeArtifactKindChoice,
  EMPTY_OPENING_PREFERENCES,
  foldOpeningPreferences,
  type OpeningConfigMutation,
  type OpeningPreferences,
  SemioFaultError,
  SURFACE_FAULT_CODES,
  type CommandAddress,
  type CommandInvocation,
  exampleArtifactSources,
  examplesForApp,
  resolveDocumentOperatorKinds,
  scopeContributionsJson,
  type ContextMenuItemSpec,
  createBrowserStoragePort,
  createDevPluginSource,
  createExtensionSource,
  multiplexPluginSources,
  createMemoryStoragePort,
  createScopedStoragePort,
  DockLayoutStore,
  type DockUiPanelState,
  DockUiStateStore,
  expandPluginRegistry,
  encodeArtifactKindChoice,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  type Effect,
  type HistoryEntry,
  type HistoryPatch,
  type IntroductionInteraction,
  CLEAR_SELECTION_ACTION_ID,
  INTERACTION_SELECT_ACTION_ID,
  latestWins,
  type LocalizedLabel,
  NamedLayoutStore,
  normalizeAppLabelsOverlay,
  organizeContextMenu,
  panelTabKindId,
  pendingPanelUiNode,
  pendingWindowUiNode,
  parseResolvedPluginViewState,
  type AppCatalogue,
  type PluginAppLabelsOverlay,
  type PluginContextMenuRequest,
  type PluginSource,
  type PluginSourceEvent,
  type PluginUiRefreshSectionResponse,
  type ProgramHotSwapEvent,
  RECORD_TUTORIAL_ACTION_ID,
  resolveExternalSlots,
  resolveLayoutForMode,
  resolveModeTools,
  resolvePlaygroundDefaultAppId,
  resolvePluginHostConfig,
  resolvePluginRegistryId,
  resolveUiDirtyScope,
  resolveWindowActions,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
  SET_SELECTION_MODE_ACTION_ID,
  SELECT_ALL_ACTION_ID,
  type ShellBrand,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  type StoragePort,
  TUTORIAL_CONVERGE_MS,
  type TutorialAssetSrc,
  type TutorialCameraState,
  type TutorialChapter,
  type TutorialDefinition,
  type TutorialArtifactEventKind,
  type TutorialEvent,
  type TutorialGestureCue,
  type TutorialUiChange,
  type TutorialUiSnapshot,
  type TutorialVideoCue,
  type BuiltNode,
  type UiDirtyScope,
  type UtilityNode,
  waitForEvent,
  windowElementId,
  hostArmedViewContext,
  panelViewContext,
  windowViewContext,
  type WindowEngagement,
  type WindowLayout,
  type WindowMeasure,
  type Conflict,
  type ConflictResolution,
  type Fault,
  type InvocationResponse,
  type MergePolicy,
  type MergeReport,
  type Severity,
} from "@semio-tech/framework";
import {
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type BrowserBrokerPortResponseV1,
  type BrowserActorUiMountedV1,
  artifactFrontierIsEditedForV1,
  artifactFrontierIsGenesisForV1,
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncUtilities,
  buildRemoteBackboneUri,
  decodeBackboneMessage,
  decodeBackboneWorkerResponse,
  decodeDocumentArchiveBytes,
  decodePackValue,
  BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES,
  DOCUMENT_ARCHIVE_MAXIMUM_BYTES,
  encodeBackboneWorkerRequest,
  encodeDocumentArchiveBytes,
  encodeMutationEnvelopesPack,
  encodePackValue,
  FRAMEWORK_SYNC_CONTROLLER_ID,
  type DirectorySessionAuthorityV1,
  type PersistenceBinding,
  DirectoryHttpError,
  type DirectoryAdministrationPhaseV1,
  type DirectoryAdministrationInviteCapabilityStatusV1,
  type DirectoryCommand,
  type DirectoryCommandErrorCodeV1,
  type DirectoryCommandReceiptV1,
  type DirectoryEvent,
  type DirectorySpaceAdministrationPageV1,
  parseDirectorySpaceAdministrationPageV1,
  type DocumentScope,
  type DocumentArchivePack,
  type ArtifactFrontier,
  type DirectoryStreamMessage,
  documentRuntimeKeyV1,
  type GisMapApprovalHistoryStatusV1,
  resolveArtifactOpeningRelay,
  type ResolvedArtifactOpeningRelay,
} from "@semio-tech/framework-os";
/** 🔗️ `mutationEnvelopeFromWire`/`mutationEnvelopeToWire`/`MutationEnvelope` live in the wire-contract
 * package itself, not re-exported by `@semio-tech/framework-os` — same source
 * `🧰️framework/🛍️products/💻️os/🟦️.ts` (that package's own root) imports them from for its
 * own `encode`/`decodeMutationEnvelopesPack` helpers above. */
import { DOCUMENT_BACKBONE_RETENTION_LIMITS, type LocalInteractionState, type MutationEnvelope } from "@semio-tech/framework-replication";
import { scopedPresencePeersV1 } from "./👥️presence-scope/🟦️.ts";
import { MODE_STEP_CONTROL_IDS, SURFACE_ROLE_CONTROL_IDS, SURFACE_ROLE_ORDER, createSealedInstanceLedgerV1, createSessionAppSwitchGateV1, createSessionWorkLedgerV1, quiesceSessionWorkV1, resolveBootPrimaryAppV1, roleSwitchTargetV1, sealedInstanceDropTextV1, sealedInstanceDropV1, stepModeIdV1, surfaceRoleAppsV1, surfaceSwitchBusyTextV1 } from "./🔀️surface-switch/🟦️.ts";


function scopeRuntimeKey(message: { readonly documentId: string; readonly scope?: DocumentScope }): string | null {
  const scope = message.scope;
  if (scope === undefined || scope.documentId !== message.documentId) return null;
  return documentRuntimeKeyV1({ kind: "hub", ...scope });
}

let localBrowserBrokerProof = (() => {
  if (typeof window === "undefined") return undefined;
  const match = /^#semio-broker=([0-9a-f]{64})$/u.exec(window.location.hash);
  if (!match) return undefined;
  window.history.replaceState(window.history.state, "", `${window.location.pathname}${window.location.search}`);
  return match[1];
})();

/** 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C3 — the config-lane
 * identity facet's documentId/schema + fold. `@semio-tech/framework-os` never had a live consumer of
 * its former `./backbone-worker` subpath export (the taxonomy-purity sweep removed the unused
 * `🟦️glue.backbone-worker.ts` shim and the export entry), so this import goes straight to the
 * owner-root file by the same relative path the `new Worker(new URL(...))` call below already uses.
 * Never redefined here. */
import { IDENTITY_CONFIG_SCHEMA, identityActorConfig, foldIdentityEvent } from "../../../../🏪️store/👷️worker/🟦️.ts";
/** 🪪️ Self-contained identity facet (see that file's header doc for why it isn't re-exported through
 * `🎚️config/🧬️schema/**`) — `Identity`/mutation vocabulary, never redeclared here. */
import {
  type Identity,
  type IdentityConfigMutation,
  type UiPreferencesConfigMutation,
  applyIdentityConfigMutation,
  setAppearance,
  setCustomDriver,
  setCustomTheme,
  setDriver,
  setKeybindingOverride as setKeybindingOverrideMutation,
  setLayout,
  setLocale,
  setTerminology,
  setTheme,
  signIn,
} from "../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import {
  decodeWorldProjectionTemplateId,
  worldProjectionSpecIconId,
  worldProjectionSpecLabel,
} from "@semio-tech/infinite-world-r3f";
import {
  type Anchor,
  ANCHORS,
  App,
  applyDockSkeleton,
  applyUiThemeToRoot,
  borderNormalBottomClass,
  buildKeysByActionId,
  builtinUiDrivers,
  builtinUiThemes,
  ButtonGroup,
  ButtonGroupItem,
  CanvasSkeleton,
  CELEBRATE_STAMP_DURATION_MS,
  celebrateAllElements,
  celebrateElements,
  childElementId,
  ChromeAwareWindowScrollSurface,
  clearUiThemeFromRoot,
  cn,
  ariaKeyshortcutsText,
  composeControlKeybindings,
  composeTutorialUi,
  ContextMenuController,
  type ContextMenuItem,
  createShellScope,
  createTutorialClock,
  DEFAULT_UI_DRIVER,
  detectShellLocale,
  disposeShellI18nInstance,
  dockSkeletonOf,
  dockSkeletonsEqual,
  elementIdSelector,
  type ElementsSurfaceAppearance,
  type ElementsSurfaceDevice,
  findPanelTabInDock,
  findPanelTabNode,
  findPanelTabPath,
  Footer,
  getTutorialCameraDriver,
  Icon,
  type IconName,
  iconRenderPort,
  insertWindowAtDropZone,
  resolveStackPathForWindowId,
  splitWithWindow,
  interactiveActiveFillClass,
  interpolateTutorialCamera,
  isContextMenuPointerTarget,
  Layout,
  LevelProvider,
  loadingBorderClass,
  Mode,
  type ModeCanvasDropTarget,
  type ModeWindowDescriptor,
  moveTabInDock,
  moveTreeUnitInDock,
  Navbar,
  NavbarExampleSelect,
  navbarFillItem,
  type NavbarItem,
  PanelChromeTabBar,
  type PanelDock,
  PanelDockProvider,
  panelTabChildren,
  type PanelTabDockMove,
  type PanelTabNode,
  type PanelTabSelectionOptions,
  type PanelTreeUnitDockMove,
  parseUiTheme,
  PresenceBar,
  type PresencePeer,
  readStoredIntroductionSeen,
  reconcileActivePath,
  resolveUiDriver,
  SemioLogo,
  semioTheme,
  serializeUiTheme,
  setActiveUiTheme,
  ShellBrandLogo,
  shellChromeTitleClassName,
  type ShellScope,
  ShellScopeProvider,
  singleTreeLeaf,
  staticTreePanelDefinition,
  TextSelectionContextMenuHost,
  type ThemeAppearanceName,
  type ThemePaletteGroup,
  Toggle,
  toggleDocumentFullscreen,
  TutorialBar,
  tutorialCameraAt,
  TutorialCaptions,
  type TutorialChapterMarker,
  type TutorialClock,
  type TutorialClockPort,
  tutorialCuesBetween,
  TutorialGhostPointer,
  tutorialSlice,
  type TutorialSlice,
  TutorialVideoOverlay,
  UI_MOBILE_MEDIA_QUERY,
  UI_TERMINOLOGY_NATIVE,
  type UiChromeLayout,
  type UiDriver,
  UIIntroduction,
  UiKeybindingsProvider,
  type UiLocale,
  type UiTranslationKey,
  uiDataLabel,
  type UiStatus,
  type UiTheme,
  useActionHotkey,
  useElementsSurfaceChrome,
  useLabel,
  useMediaQuery,
  usePanelChromeHotkeys,
  useShellKeydown,
  useShellScope,
  useTutorialClock,
  validateTutorial,
  WindowBodySkeleton,
  type WindowLayoutNode,
  type WindowTemplateDropPayload,
  writeStoredIntroductionSeen,
} from "@semio-tech/ui-react";
import { canonicalUiDriver, canonicalUiTheme, commitUiPreferencesConfigMutation, readUiPreferences, resolveUiPreferences, subscribeUiPreferences } from "../../🎚️UiPreferences/🟦️.ts";
import {
  AppCatalogueContext,
  InterpretedUiNode,
  PluginSurfaceActionsContext,
  ShellContextMenuFallbackContext,
  wireLabel,
} from "../🗣️Interpreter/🟦️.tsx";
import { builtNodeToSnapshot, UiDocumentStore } from "../📃️UiDocumentStore/🟦️.tsx";
import { SpaceAdministrationPane, spaceAdministrationCapabilities, spaceAdministrationInviteRevocable, spaceAdministrationMemberRemovable, spaceAdministrationNameValid, type SpaceAdministrationIntentV1 } from "../🛂️SpaceAdministration/🟦️.tsx";
import { BoardSessionFactoryContext, resolveAppSurfaceSessionFactory, type AppSurfaceSessionFactory } from "../🪪️WasmSessionLoader/🟦️.tsx";
import { resolveDocumentOpeningBindings, resolveDocumentOpeningTarget, type DocumentOpeningReference, type DocumentOpeningTarget } from "./🧭️opening/🟦️.ts";
import {
  actionStageKey,
  type ActiveSession,
  EMPTY_SHELL_DEFAULTS,
  EMPTY_SHELL_LOCKS,
  type ExtraWindowInstance,
  type FrameworkOsDefaults,
  initialShellState,
  isEphemeralShellBrand,
  type LoadedProgramState,
  type PluginSupervisorState,
  resolveBootExampleId,
  resolvePluginCanvasStatus,
  type ResolvedShellLocks,
  selectOpenConflicts,
  selectQuarantinedConflicts,
  ShellFaultBoundary,
  shellReducer,
  shouldAutoStartIntroduction,
  shouldPersistIntroductionSeen,
  shouldReplayIntroductionOnLoad,
  type SpacePanelState,
  type SpaceProgramEntry,
  type SpawnedAppEntry,
  type ViewModel,
} from "../🐚️Shell/🟦️.tsx";
import {
  beginInteractivePluginAction,
  clearPendingWorldProjection,
  endInteractivePluginAction,
  mapContextMenuSpecs,
  registerPendingWorldProjection,
  leftoverOverlayCarryingSelectionV1,
  leftoverOverlayCarryingUtilityV1,
  leftoverWorldArmedWindowOverlayV1,
  leftoverWorldSelectionOverlayV1,
  leftoverWorldWindowOverlayV1,
  publishLeftoverWorldSelectionV1,
  subscribeLeftoverWorldSelectionV1,
  WindowInstanceIdContext,
} from "../🌐️World3dHost/🟦️.tsx";
import {
  DEFAULT_PANEL_WIDTH_PX,
  EMPTY_APP_CATALOGUE,
  EMPTY_APP_LABELS_OVERLAY,
  FRAMEWORK_CATEGORY_COMMAND_ID,
  FRAMEWORK_CATEGORY_DISPLAY_ID,
  FRAMEWORK_CATEGORY_TOOL_ID,
  LAYOUT_CHANGE_SETTLE_MS,
  NOTE_WORLD_NAVIGATION_ACTION_ID,
  PANEL_TAB_BAR_HOSTS,
  PRESENCE_HEARTBEAT_INTERVAL_MS,
  TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS,
  actionCategoryId,
  actionRequiresStagedForm,
  appBreadcrumb,
  appWindowLabel,
  applyFrameworkLayoutSeed,
  applyTutorialUiChangeToShell,
  applyTutorialUiSnapshotToShell,
  applyUiRefreshResponseToCache,
  buildActiveExampleAction,
  SET_ACTIVE_EXAMPLE_ACTION_ID,
  navbarExampleIdFromHistoryUpserts,
  rememberedExampleIdFromDispatchV1,
  interactionViewFromLeftoverOutput,
  leftoverInteractionStateV1,
  buildActiveUtilityByWindowId,
  buildCommandCategoryTabs,
  buildNoteShellCommandAction,
  buildOsCommands,
  buildSpacePanelState,
  buildToolTabs,
  toolCategoryOpenPath,
  toolLeafInactiveRepress,
  toolPanelTreeContentRevision,
  browserActorDispatchUiScopeV1,
  browserActorWindowConfigDispatchUiScopeV1,
  typedOperationCompletionRefreshV1,
  buildUiRefreshRequest,
  createUiRefreshCoalescerV1,
  type UiRefreshCoalescerV1,
  hostEffectRefreshScopeV1,
  captureCurrentFrameworkLayout,
  captureTutorialUiSnapshot,
  categoryTabIcon,
  classifyWindowLayoutChange,
  commandAddressKey,
  commandCategories,
  commandCategoryLabel,
  commandKeybindingChords,
  commandOwnerPluginId,
  detectCommandPlatform,
  dispatchOpenedFiles,
  dispatchOsCommand,
  drainSegmentedMediaExport,
  downloadDataUrl,
  downloadMediaExport,
  mediaExportEncodingText,
  shellSegmentedDownloadSinkFactory,
  filterDefinitionsForRole,
  flattenPanelTabLeaves,
  groupOpenWithEntries,
  introductionTargetsWindow,
  isEditableEventTarget,
  isMutationKindDefinition,
  isOsCommandAddress,
  keyboardEventMatchesChord,
  loadPluginModuleResilient,
  makeEffectDispatchOne,
  mergeRecordPreservingIdentity,
  mergeUiDirtyScopeV1,
  openArtifactWithText,
  OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID,
  OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID,
  type OpenWithEntry,
  panelAnchorForGroup,
  panelJsonFromState,
  panelTabDefinitionToNode,
  parsePanelState,
  parseShellRoute,
  pluginAvailabilityRouteV1,
  pluginShouldReceiveContributions,
  pluginShouldEstablishSession,
  AutoCheckinScheduler,
  canCheckIn,
  checkinActionText,
  checkinCancelText,
  checkinMessagePlaceholderText,
  checkinSubmitText,
  computeSyncPillState,
  createLatestAsyncDispatcher,
  presenceClientIdentity,
  preserveJsonIdentity,
  programArmedToolRevealV1,
  reconcileToolTabSelection,
  renderStagedArgControl,
  requestFileOpen,
  resolveAppLabel,
  resolveAppBreadcrumb,
  resolveCanvasBodyKey,
  resolveCommands,
  resolveDialogDefinition,
  resolveArtifactByAppId,
  resolveFrameworkLayoutSeed,
  resolveIntroductionDefinition,
  resolveKeybindingIntent,
  clipboardWriteFragmentFromEffect,
  pasteActionWithRetainedFragment,
  pasteArgsFragment,
  resolveManifestLabel,
  resolvePanelTabLabel,
  resolveUtilityActivation,
  undeclaredActionDiagnostic,
  historyPatchShouldApplyV1,
  historyRefreshNeededV1,
  resolveUtilityNodes,
  resolveWindowEngagement,
  retitleWindowLayoutNode,
  runRequestMediaFrames,
  scheduleDispatchAction,
  SEGMENTED_DOWNLOAD_MARKER_PREFIX,
  sessionWindowInstances,
  setAsDefaultText,
  shellLabel,
  shellRendersPanelTabItself,
  shellTabIcon,
  spawnedWindowChromeForKind,
  studioPanelFocusingSpawned,
  surfaceRoleChipText,
  surfaceRoleGroupText,
  appModeGroupText,
  syncDocumentId,
  syncPillText,
  syncShellLabelLocale,
  synthesizeLocalizedLabel,
  toolIdFromPanelTabId,
  uiIntentToActionDescriptor,
  type SyncPillState,
  viewerReadOnlyNoticeText,
  useUIHistory,
  utilityBarNode,
  utilityNodeTreeContainsId,
  viewStateWithSpacePanel,
  windowActionPaneNode,
  windowEngagementToSearchSpec,
  windowEngagementToSpec,
  windowMeasureTreeContainsId,
  windowMeasuresChrome,
  beginCancellableExtensionRequest,
  abortExtensionRequestsForActor,
  isDeclaredSurfaceCancelAction,
  type ActionPaneSlice,
  type PluginInstallOutcome,
  type ResolvedCommand,
  type ToolTabSelection,
  type TutorialUiBridgeContext,
  type UiRefreshCache,
} from "../🛠️ShellHelpers/🟦️.tsx";
import { createContributionsPublisher, type ContributionsOperatorScope, type ContributionsPublishOutcome, type ContributionsSessionKey } from "../🛠️ShellHelpers/🧩️contributions/🟦️.ts";

import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "../../../../../../../../♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx";
import { ENTWERFEN_MIT_BESTAND_BRAND_IDS } from "../../../../../../../../♻️mit-bestand/🧺️demonstrator/🪧️brand.ts";
import {
  createFrameworkDisplayPanelTabs,
  createFrameworkMarketplacePanelTab,
  createFrameworkSettingsPanelTab,
  DEFAULT_APP_NONE_VALUE,
  type ConflictsHostApi,
  type DefaultAppRow,
  type DefaultAppsHostApi,
  encodeDefaultAppValue,
  FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID,
  FRAMEWORK_SETTINGS_PANEL_ID,
  type DisplayHostApi,
  type MarketplaceExtensionEntry,
  type MarketplaceHostApi,
  type MarketplacePluginEntry,
  PluginRecoveryPanel,
  type SettingsHostApi,
  ShellRouteNotFoundPage,
  useNamedLayoutHost,
} from "../📌️ChromePanels/🟦️.tsx";
import { onPluginInstancesLost, PluginBootShardLostError, leftoverInspectionRefreshScope, leftoverInspectionPanelHash, leftoverBrushPreviewWindowHash, leftoverBrushPreviewRefreshScope, leftoverBrushPreviewRefreshReady, type PluginWasmHandle, type PluginExtensionCompletion, serializePerActor, setPluginRuntimeActor } from "../🔌️PluginRuntime/🟦️.tsx";
import { documentBackboneEffectV1, type ActorDocumentMessagePortV1 } from "../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";
import { BrowserActorActionMailboxV1 } from "../../../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } from "../../../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts";
import { publishBrowserActorHostEffectsV1 } from "../../../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts";
import { InferencePortOpeningMailboxV1 } from "../../../../💡️inference/🚪️opening/🟦️.ts";
import { isShardLostError } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { type WindowFault, type WindowFaultClass, windowFaultFromError } from "./🩺️fault/🟦️.ts";
import { EXTENSION_TARGETS } from "../../../../🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { PLUGIN_CATALOG } from "../../../../🔌️plugin/📇️registry/🟦️.ts";
import { MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE } from "../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { BootstrapStatusNotice, ExecutionTargetStatusNotice, GisMapInferenceRequestControl, InferencePortPanel, inferencePortStatusRuntimeKeyV1, reduceBootstrapUiState, reduceExecutionTargetUiState, resolveRequiredHostApps, retainInferencePortOwnerAfterCloseV1, shellHistoryUndoRouteV1, type BootstrapUiState, type ExecutionTargetUiState, type InferencePortOwnerV1, type InferencePortUiAction } from "./🪪️host-bootstrap/🟦️.tsx";
import { ArtifactCreationCatalogNotice, ArtifactCreationProgressNotice, reduceArtifactCreationProgressUiV1, type ArtifactCreationProgressOwnerV1, type ArtifactCreationProgressUiStateV1 } from "./🌱️artifact-creation/🟦️.tsx";
import {
  DirectoryBootstrapStatusNotice,
  applyDirectoryEventPageBootstrapV1,
  closeDirectoryHomeOwnerV1,
  openDirectoryHomeOwnerV1,
  type DirectoryBootstrapUiState,
  type DirectoryHomeOwnerV1,
} from "./📇️directory-bootstrap/🟦️.tsx";


import { SyncAttachCard } from "../🔄️ShellSync/🟦️.tsx";
import { useAgentBridge } from "../🔗️AgentBridge/🟦️.tsx";
import { AgentPresence } from "../🚦️AgentPresence/🟦️.tsx";
import { AgentApprovals } from "../🤖️AgentApprovals/🟦️.tsx";
import { UIFind, UIFindProvider, UISearch, type UISearchItem } from "../🔎️ShellSearch/🟦️.tsx";
import { UTILITY_CATEGORY_ICON_ID } from "../🎛️UtilityTree/🟦️.tsx";
import { coerceWireBytes } from "../🔌️PluginRuntime/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️BuiltNodeReconciler

/** 🩹️ `WindowKindDefinition.label` has no owned schema mirror yet (`unknown` — see that generated type's own
 * doc comment, and `resolveManifestLabel`'s "some call sites may still see the pre-migration shape
 * until that typegen lands" note) — asserts each window kind's real, documented label shape for the
 * `*LayoutSeed`/`retitleWindowLayoutNode` helpers that still declare their `windowKinds` param
 * strictly as `{ id, label: LocalizedLabel | string }[]`. */
function withLocalizedWindowKindLabels(windowKinds: readonly { readonly id: string; readonly label: unknown }[]): readonly { readonly id: string; readonly label: LocalizedLabel | string }[] {
  return windowKinds as readonly { readonly id: string; readonly label: LocalizedLabel | string }[];
}

/** 🌳️ `flattenPanelTabLeaves`'s own twin for `PanelTabNode` trees — that shared helper's generic
 * constraint rejects `PanelTabNode` as a TS "weak type" (its `PanelTabLeaf` variant carries no
 * `children` key at all, see `flattenPanelTabLeaves`'s own doc comment), so this walks the union via
 * `panelTabChildren`'s own kind-aware accessor instead of a structural `.children` read. */
function flattenPanelTabNodeLeaves(tabs: readonly PanelTabNode[]): readonly PanelTabNode[] {
  return tabs.flatMap((tab) => {
    const children = panelTabChildren(tab);
    return children && children.length > 0 ? flattenPanelTabNodeLeaves(children) : [tab];
  });
}

function panelDefinitionPath(tabs: AppDefinition["panelTabs"], targetId: string): readonly string[] | null {
  for (const tab of tabs) {
    const id = panelTabKindId(tab.kind);
    if (id === targetId && tab.children.length === 0) return [id];
    const childPath = panelDefinitionPath(tab.children, targetId);
    if (childPath) return [id, ...childPath];
  }
  return null;
}

function requiredHostPanelLeafId(app: AppDefinition | undefined): string {
  const leaf = app ? flattenPanelTabLeaves(app.panelTabs)[0] : undefined;
  if (!leaf) throw new Error("configured host app has no panel leaf");
  return panelTabKindId(leaf.kind);
}
//#endregion 🔖️BuiltNodeReconciler

//#region FrameworkOsShell
/** @emoji 🏷️ Lets a per-window host rewrite its Mode window title (e.g. live projection label). */
export const SetWindowTitleContext = createContext<((windowId: string, title: string) => void) | null>(null);

/** @emoji 🖼️ Lets a per-window host rewrite its Mode window icon (e.g. live projection glyph). */
export const SetWindowIconContext = createContext<((windowId: string, iconId: IconName) => void) | null>(null);

/** 🔬️ Dev-only `window.__semioOsCatalogProbe` payload — see `#region 🔖️CatalogSmokeProbe`; schema
 * report fixture `🧑‍💻dev/🧫️fixtures/🔬️catalog-smoke.json`. */
export type ShellCatalogProbe = {
  readonly shellPluginId: string;
  readonly ready: boolean;
  /** 🧯️ `routerFault` carries the `surface.*` fault that excluded this plugin from `AppRouter`
   * (ticket 26/09/05/S-END-TO-END lane H) — the plugin installed fine, so `status` alone would read
   * healthy while none of its surfaces route. */
  readonly plugins: readonly { readonly pluginId: string; readonly status: string; readonly routerFault?: { readonly code: string; readonly message: string } }[];
  readonly programs: readonly { readonly pluginId: string; readonly appId: string; readonly label: string }[];
  readonly spawned: readonly { readonly id: string; readonly pluginId: string; readonly appId: string }[];
};

type MountedBrowserActorUiV1 = Omit<BrowserActorUiMountedV1, "kind" | "instanceId"> & Readonly<{ sessionInstanceId: number; windowKindId: string; store: UiDocumentStore }>;

export type MountedGisMapProbeV1 = Readonly<{
  scope: DocumentScope;
  clientInstanceId: string;
  activationGeneration: string;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  activeCheckpointId: string;
  descriptorDigestV1: string;
  frontier: ArtifactFrontier;
  uiRevision: number;
  rootKind: "tiled-map";
  regionIds: readonly string[];
}>;

/** 🔬️ Projects only public identity and GIS scene facts from the Shell's acknowledged retained
 * store. It has no dispatch, credential, receipt, grant, proposal or undo-handle surface. */
export function mountedGisMapProbeV1(source: MountedBrowserActorUiV1 | null): MountedGisMapProbeV1 | null {
  if (source === null) return null;
  const state = source.store.getState();
  if (
    state.revision < 1 ||
    state.revision !== source.uiRevision ||
    state.root === null ||
    !/^[0-9a-f]{64}$/u.test(source.activeCheckpointId) ||
    !/^[0-9a-f]{64}$/u.test(source.descriptorDigestV1) ||
    (!(artifactFrontierIsGenesisForV1(source.scope, source.frontier) || artifactFrontierIsEditedForV1(source.scope, source.frontier)) || source.frontier.lastCommitSeq > source.frontier.headEditOrdinal)
  ) return null;
  const root = state.nodes.get(state.root);
  if (root?.component.type !== "surface" || root.component.kind !== "tiled-map") return null;
  let decoded: unknown;
  try {
    decoded = decodePackValue(new Uint8Array(root.component.doc.bytes));
  } catch {
    return null;
  }
  if (decoded === null || typeof decoded !== "object" || Array.isArray(decoded)) return null;
  const regions = (decoded as Record<string, unknown>).regions;
  if (!Array.isArray(regions) || regions.length > 4_096) return null;
  const regionIds: string[] = [];
  for (const region of regions) {
    if (region === null || typeof region !== "object" || Array.isArray(region)) return null;
    const id = (region as Record<string, unknown>).id;
    if (typeof id !== "string" || id.length === 0 || new TextEncoder().encode(id).byteLength > 256 || /[\u0000-\u001f\u007f]/u.test(id) || regionIds.includes(id)) return null;
    regionIds.push(id);
  }
  regionIds.sort();
  return Object.freeze({
    scope: Object.freeze({ ...source.scope }),
    clientInstanceId: source.clientInstanceId,
    activationGeneration: source.activationGeneration,
    catalogGenerationId: source.catalogGenerationId,
    componentSha256: source.componentSha256,
    descriptorSha256: source.descriptorSha256,
    browserActorSha256: source.browserActorSha256,
    activeCheckpointId: source.activeCheckpointId,
    descriptorDigestV1: source.descriptorDigestV1,
    frontier: Object.freeze({ ...source.frontier, chainHash: Object.freeze([...source.frontier.chainHash]) }),
    uiRevision: state.revision,
    rootKind: "tiled-map",
    regionIds: Object.freeze(regionIds),
  });
}

/** 🔬️ One probe row per installed registry entry: its install status, plus the `surface.*` fault
 * that kept it out of {@link AppRouter} when there is one. Pure and total — a plugin excluded from
 * routing is reported here, never dropped from the list, so a catalog smoke sees the difference
 * between "not installed", "installed and routing" and "installed but unroutable". */
export function shellCatalogProbePlugins(
  registry: readonly { readonly pluginId: string }[],
  pluginStatusById: Readonly<Record<string, string | undefined>>,
  router: AppRouter,
): ShellCatalogProbe["plugins"] {
  return registry.map((entry) => {
    const routerFault = router.faultFor(entry.pluginId);
    return { pluginId: entry.pluginId, status: pluginStatusById[entry.pluginId] ?? "available", ...(routerFault ? { routerFault: { code: routerFault.code, message: routerFault.message } } : {}) };
  });
}

const EMPTY_KEYS_BY_ACTION_ID = new Map<string, string>();

/** ⚖️ `TransientNotice.kind` tone per `Severity` (contract freeze `26/08/16/MUTATION-OUTCOMES-
 * MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C1) — `info`/`warning` share the neutral popover
 * chrome (matches the pre-existing "not error" branch), `error`/`fatal` both render destructive,
 * `fatal` additionally bolded so a rejected dispatch's worst level reads as more severe than a
 * plain `error`. */
const TRANSIENT_NOTICE_TONE_CLASS: Record<Severity, string> = {
  info: "border-border bg-popover text-popover-foreground",
  warning: "border-amber-400 bg-amber-400/10 text-amber-400",
  error: "border-destructive bg-destructive text-destructive-foreground",
  fatal: "border-destructive bg-destructive text-destructive-foreground font-semibold",
};

/** ⚖️ `Fault.code` for a rejected local dispatch (contract freeze §C8/§C9) — never invented locally,
 * mirrors the Rust guest's `Fault("mutation.rejected")`. */
const MUTATION_REJECTED_FAULT_CODE = "mutation.rejected";

/** ⚖️ Maps one of the frozen seven `mutation.*` codes (contract freeze §C2 — no per-plugin codes,
 * ever) onto its `ui.mutation.code.*` label key; an unrecognized code falls back to the generic
 * rejected-title key rather than fabricating a key the schema doesn't have. */
function mutationCodeLabelKey(code: string): UiTranslationKey {
  switch (code) {
    case "mutation.target-missing":
      return "ui.mutation.code.targetMissing";
    case "mutation.no-op":
      return "ui.mutation.code.noOp";
    case "mutation.partial":
      return "ui.mutation.code.partial";
    case "mutation.clamped":
      return "ui.mutation.code.clamped";
    case "mutation.duplicate-id":
      return "ui.mutation.code.duplicateId";
    case "mutation.invariant":
      return "ui.mutation.code.invariant";
    case "mutation.cascade":
      return "ui.mutation.code.cascade";
    default:
      return "ui.mutation.rejected.title";
  }
}

//#region 🏁️OperationSettle
/** 🏁️ Concurrent `onAction` callers that may wait on a typed operation's terminal completion at once.
 * Sized for a live desktop session (a few background tick loops plus whatever the user is driving) —
 * a caller past the ceiling settles immediately rather than growing an unbounded map. */
const OPERATION_SETTLE_WAITER_SLOTS = 32;
/** 🏁️ Completions remembered for a waiter that has not registered yet — the admitting reply and the
 * `OperationCompleted` frame race, and the completion can win. Older ids fall off the ring. */
const OPERATION_SETTLE_RING_SLOTS = 64;
/** 🏁️ Ceiling on how long an `onAction` promise may wait for a completion frame that never arrives (a
 * faulted worker, a torn-down instance). Past it the caller resumes: a background tick loop must be
 * able to recover from a lost frame, never wedge on one. */
const OPERATION_SETTLE_WATCHDOG_MS = 30_000;

/** 🏁️ The typed-operation id an admitting `InvocationResult` started, or `undefined` when the action
 * started none. `dispatch_typed_command_inner` (`🔌️plugin/🦀️.rs`) answers a started operation with
 * `output = { operationId, generation }`, both decimal STRINGS; `OperationCompletionV1.operation` is
 * the same id as a number, so this is where the two representations meet. */
function startedTypedOperationId(output: unknown): number | undefined {
  if (typeof output !== "object" || output === null) return undefined;
  const raw = (output as { readonly operationId?: unknown }).operationId;
  const operation = typeof raw === "string" ? Number(raw) : typeof raw === "number" ? raw : Number.NaN;
  return Number.isSafeInteger(operation) && operation >= 0 ? operation : undefined;
}
//#endregion 🏁️OperationSettle

/** 🩺️ `WindowFaultClass` → its `ui.windowFault.*` label key. Total by construction, so a new class
 * cannot silently render as a blank body. */
const WINDOW_FAULT_LABEL_KEYS: Readonly<Record<WindowFaultClass, UiTranslationKey>> = {
  "abi-mismatch": "ui.windowFault.abiMismatch",
  "interactive-ceiling": "ui.windowFault.interactiveCeiling",
  clock: "ui.windowFault.clock",
  "plugin-internal": "ui.windowFault.pluginInternal",
  "install-failed": "ui.windowFault.installFailed",
  unknown: "ui.windowFault.unknown",
};

/** 🩺️ The accessible bilingual status a window body shows instead of staying silently empty when
 * its plugin instance faulted. `data-semio-window-fault` carries the machine-readable class so a
 * catalog smoke can name the cause straight off the DOM; the raw `Fault.code` and the plugin's own
 * message stay visible underneath for a human reading the window. */
function WindowFaultStatus({ fault }: { readonly fault: WindowFault }): React.ReactElement {
  return (
    <div className="flex h-full min-h-0 w-full flex-col gap-single p-double text-sm" role="status" aria-live="polite" data-semio-window-fault={fault.class} data-semio-window-fault-code={fault.code ?? ""} data-semio-window-fault-origin={fault.origin ?? ""}>
      <p className="font-semibold text-destructive">{shellLabel("ui.windowFault.title")}</p>
      <p className="text-muted-foreground">{shellLabel(WINDOW_FAULT_LABEL_KEYS[fault.class])}</p>
      <p className="font-mono text-xs text-muted-foreground">{uiDataLabel(fault.code ? `${fault.code}: ${fault.message}` : fault.message)}</p>
    </div>
  );
}

/** @emoji ⌨️ Last-wins app keybindings for enriching context-menu shortcut labels in scene hosts. */
const AppKeybindingsContext = createContext<ReadonlyMap<string, string>>(EMPTY_KEYS_BY_ACTION_ID);

/** @emoji ⌨️ Resolves action→keys bindings from the nearest {@link AppKeybindingsContext} provider. */
export function useAppKeybindingsByActionId(): ReadonlyMap<string, string> {
  return useContext(AppKeybindingsContext);
}

/** @emoji 🖱️ Maps program context-menu specs with app keybinding shortcut enrichment. */
export function useMapContextMenuSpecs(dispatch: (action: string, args?: Record<string, unknown>) => void) {
  const keysByActionId = useAppKeybindingsByActionId();
  return useCallback((specs: readonly ContextMenuItemSpec[]) => mapContextMenuSpecs(specs, dispatch, keysByActionId), [dispatch, keysByActionId]);
}

/** 🪟️ Builds the sole action wire shape from the exact target window instance and its owner chain. */
function windowActionInvocation(
  session: ActiveSession,
  action: ActionDescriptor,
  extraInstances: readonly ExtraWindowInstance[] = [],
  requestedWindowId?: string,
): ActionInvocation {
  const instances = sessionWindowInstances(session.app, extraInstances);
  const windowInstanceId = requestedWindowId ?? session.viewState.windowId ?? session.viewState.activeWindowKindId ?? instances[0]?.id ?? session.app.windowKinds[0]?.id ?? "";
  const windowKindId = instances.find((instance) => instance.id === windowInstanceId)?.windowKindId ?? session.app.windowKinds.find((kind) => kind.id === windowInstanceId)?.id ?? session.app.windowKinds[0]?.id ?? "";
  return {
    address: {
      pluginId: session.pluginId,
      appId: session.app.id,
      modeId: session.viewState.activeModeId ?? session.app.defaultModeId ?? session.app.modes[0]?.id ?? session.app.id,
      windowKindId,
      windowInstanceId,
      actionId: action.action,
    },
    arguments: {
      ...(typeof action.args === "object" && action.args != null ? (action.args as Record<string, unknown>) : {}),
      windowId: windowInstanceId,
    },
  };
}

/** 🧩️ Serializes the exact action invocation used by the retained actor and ordinary plugin routes. */
function encodeWindowActionInvocation(session: ActiveSession, action: ActionDescriptor, extraInstances: readonly ExtraWindowInstance[] = [], requestedWindowId?: string): string {
  return JSON.stringify(windowActionInvocation(session, action, extraInstances, requestedWindowId));
}

/** 📄️ Decode a live document pack for operator-kind reachability — pack value, then UTF-8 of pack/spr. */
function documentSourcesFromPack(pack: Uint8Array, spr: Uint8Array, ops?: string): unknown[] {
  const sources: unknown[] = [];
  try {
    sources.push(decodePackValue(pack));
  } catch {
    /* `.spk` / non-pack snapshot — UTF-8, spr, and ops still get a walk */
  }
  sources.push(new TextDecoder().decode(pack));
  sources.push(new TextDecoder().decode(spr));
  if (ops) sources.push(ops);
  return sources;
}

/** 🎛️ Builds an app-owned command wire without pretending host catalogue state is a window action. */
function encodeAppCommandInvocation(pluginId: string, app: AppDefinition, commandId: string, args: Readonly<Record<string, unknown>>): string {
  const invocation: CommandInvocation = {
    address: { owner: { app: { pluginId, appId: app.id } }, commandId },
    arguments: { ...args },
  };
  return JSON.stringify(invocation);
}

/** 📋️ Tests whether an app explicitly opts into a host-pushed command. */
export function appOwnsCommand(app: AppDefinition, commandId: string): boolean {
  return (app.commands ?? []).some((command) => command.id === commandId);
}

/** @emoji 📄️ Tests whether the app's OWN declaration of a host-pushed command includes `pageCount`.
 * The shell still sends exactly those arguments (`page: 0`, `pageCount: 1`) as ONE pack-encoded
 * `handleCommand`. It does not cut the payload into 4 KiB JSON pages
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function appCommandTakesPageRun(app: AppDefinition, commandId: string): boolean {
  const command = (app.commands ?? []).find((entry) => entry.id === commandId);
  return (command?.args ?? []).some((arg) => arg.id === "pageCount");
}

//#region 🎥️TutorialOverlayHosts
/** @emoji 📦️ Resolves a `TutorialAssetSrc` to a value usable as an `<video>`/`<audio>` `src` — `Blob` (a
 * studio `BlobStore` reference) isn't resolvable from this scope (no blob-store bridge here) and returns
 * `null` with a console warning; `Url`/`DataUrl` resolve directly. */
function tutorialAssetSrcToUrl(src: TutorialAssetSrc): string | null {
  if (src.kind === "url") return src.url;
  if (src.kind === "dataUrl") return src.data;
  console.warn("[DEBUG] tutorial blob asset src not resolvable in this scope", src.hash);
  return null;
}

/** @emoji 💬️ Self-subscribes to the tutorial clock (see `useTutorialClock`) so only THIS leaf re-renders every frame — never the whole shell — mirroring `TutorialBar`'s own subscription. */
const TutorialCaptionsHost: React.FC<{ readonly tutorial: TutorialDefinition; readonly clock: TutorialClockPort; readonly captionsOn: boolean; readonly terminology: string; readonly locale: string }> = ({ tutorial, clock, captionsOn, terminology, locale }) => {
  const timeMs = useTutorialClock(clock);
  const cue = tutorialCuesBetween(tutorial.tracks.narration, timeMs)[0] ?? null;
  return <TutorialCaptions text={cue ? resolveManifestLabel(cue.text, terminology, locale) : null} visible={captionsOn} />;
};

const TUTORIAL_DEFAULT_VIDEO_RECT = { x: 0.72, y: 0.7, width: 0.24, height: 0.24 } as const;

/** @emoji 📹️ Self-subscribes to the tutorial clock; resolves the covering `TutorialVideoCue` (if any) and its source-relative local time. */
const TutorialVideoOverlayHost: React.FC<{ readonly tutorial: TutorialDefinition; readonly clock: TutorialClockPort; readonly muted: boolean; readonly playing: boolean; readonly rate: number }> = ({
  tutorial,
  clock,
  muted,
  playing,
  rate,
}) => {
  const timeMs = useTutorialClock(clock);
  const cue: TutorialVideoCue | null = tutorialCuesBetween(tutorial.tracks.video, timeMs)[0] ?? null;
  const src = cue ? tutorialAssetSrcToUrl(cue.src) : null;
  const localTimeMs = cue ? timeMs - cue.at + cue.sourceOffsetMs : 0;
  return <TutorialVideoOverlay src={src} rect={cue?.rect ?? TUTORIAL_DEFAULT_VIDEO_RECT} muted={muted || (cue?.muted ?? false)} playing={playing} rate={rate} localTimeMs={localTimeMs} />;
};

/** @emoji 👻️ Self-subscribes to the tutorial clock; resolves the covering `TutorialGestureCue` (if any) and progress (0–1) through it, driving `TutorialGhostPointer` off the PLAYHEAD rather than its own internal clock (unlike the introduction demonstration overlay). */
const TutorialGhostPointerHost: React.FC<{ readonly tutorial: TutorialDefinition; readonly clock: TutorialClockPort }> = ({ tutorial, clock }) => {
  const timeMs = useTutorialClock(clock);
  const cue: TutorialGestureCue | null = tutorialCuesBetween(tutorial.tracks.gestures, timeMs)[0] ?? null;
  const progress = cue ? Math.min(1, Math.max(0, (timeMs - cue.at) / Math.max(cue.durationMs, 1))) : 0;
  return <TutorialGhostPointer cue={cue} progress={progress} />;
};
//#endregion 🎥️TutorialOverlayHosts

//#region 🎥️TutorialRecorder
const OBSERVED_INTERACTION_ACTION_IDS = new Set([CLEAR_SELECTION_ACTION_ID, INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID, SET_SELECTION_MODE_ACTION_ID]);

/** 📰️ Projects a tutorial selection restore onto ordinary framework interaction actions. */
export function tutorialInteractionSelectionActions(controllerId: string, selection: LocalInteractionState["selection"]): readonly ActionDescriptor[] {
  const actions: ActionDescriptor[] = [{ controllerId, action: CLEAR_SELECTION_ACTION_ID }];
  for (const domainId of Object.keys(selection).sort()) {
    const current = selection[domainId];
    if (!current || current.ids.length === 0) continue;
    if (current.ids.length > 1) actions.push({ controllerId, action: SET_SELECTION_MODE_ACTION_ID, args: { domainId, mode: "multiple" } });
    actions.push({
      controllerId,
      action: INTERACTION_SELECT_ACTION_ID,
      args: { domainId, merge: "replace", method: "pick", targets: JSON.stringify(current.ids.map((id) => ({ granularity: current.granularity, id }))) },
    });
    if (current.anchorId && current.anchorId !== current.ids[current.ids.length - 1] && current.ids.includes(current.anchorId)) {
      actions.push({
        controllerId,
        action: INTERACTION_SELECT_ACTION_ID,
        args: { domainId, merge: "additive", method: "pick", targets: JSON.stringify([{ granularity: current.granularity, id: current.anchorId }]) },
      });
    }
  }
  return actions;
}

/** @emoji ↔ Field-by-field structural diff of two `TutorialUiSnapshot`s into the sparse `TutorialUiChange`
 * alphabet — the recorder's UI-diff effect calls this every `ShellState` change while armed. */
function diffTutorialUiSnapshot(prev: TutorialUiSnapshot, next: TutorialUiSnapshot): TutorialUiChange[] {
  const changes: TutorialUiChange[] = [];
  if (prev.activeModeId !== next.activeModeId && next.activeModeId != null) changes.push({ kind: "activeMode", id: next.activeModeId });
  if (prev.focusedWindowId !== next.focusedWindowId) changes.push({ kind: "focusedWindow", id: next.focusedWindowId });
  const utilityWindowIds = new Set([...Object.keys(prev.activeUtilityByWindowId), ...Object.keys(next.activeUtilityByWindowId)]);
  for (const windowId of utilityWindowIds) {
    if (prev.activeUtilityByWindowId[windowId] !== next.activeUtilityByWindowId[windowId]) changes.push({ kind: "activeUtility", windowId, utilityId: next.activeUtilityByWindowId[windowId] });
  }
  if (prev.activeToolId !== next.activeToolId) changes.push({ kind: "activeTool", id: next.activeToolId });
  if (next.layout && JSON.stringify(prev.layout) !== JSON.stringify(next.layout)) changes.push({ kind: "layout", layout: next.layout });
  const groups = new Set([...Object.keys(prev.activePanelTabByGroup), ...Object.keys(next.activePanelTabByGroup)]);
  for (const group of groups) {
    if (prev.activePanelTabByGroup[group] !== next.activePanelTabByGroup[group]) changes.push({ kind: "panelTab", group, tabId: next.activePanelTabByGroup[group] });
  }
  if (next.panelJson != null && prev.panelJson !== next.panelJson) changes.push({ kind: "panelState", panelJson: next.panelJson });
  // 🕹️ `TutorialUiSnapshot.selectionJson` is gone — replaced by `interactionSelection`, the framework-
  // owned per-domain selection map. Diffed one domain at a time, matching the granularity every other
  // diff in this function already operates at (`activeUtilityByWindowId`/`activePanelTabByGroup`).
  const selectionDomainIds = new Set([...Object.keys(prev.interactionSelection), ...Object.keys(next.interactionSelection)]);
  for (const domainId of selectionDomainIds) {
    const prevSelection = prev.interactionSelection[domainId];
    const nextSelection = next.interactionSelection[domainId];
    const idsMatch = prevSelection !== undefined && nextSelection !== undefined && prevSelection.ids.length === nextSelection.ids.length && prevSelection.ids.every((id, index) => id === nextSelection.ids[index]);
    if (prevSelection?.granularity === nextSelection?.granularity && idsMatch) continue;
    const granularity = nextSelection?.granularity ?? prevSelection?.granularity;
    if (granularity !== undefined) changes.push({ kind: "selection", domainId, granularity, ids: nextSelection ? [...nextSelection.ids] : [] });
  }
  if (prev.openDialogId !== next.openDialogId) changes.push({ kind: "dialog", id: next.openDialogId });
  const prevTree = new Set(prev.expandedTreeIds);
  const nextTree = new Set(next.expandedTreeIds);
  for (const id of nextTree) if (!prevTree.has(id)) changes.push({ kind: "treeExpansion", id, expanded: true });
  for (const id of prevTree) if (!nextTree.has(id)) changes.push({ kind: "treeExpansion", id, expanded: false });
  if (prev.commandPanelOpen !== next.commandPanelOpen) changes.push({ kind: "commandPanel", open: next.commandPanelOpen });
  return changes;
}

/** @emoji 🎥️ Epsilon-equality for two camera poses — the recorder's 10Hz camera sampler skips writing a
 * new keyframe when the live pose hasn't meaningfully moved since the last sample. */
function tutorialCameraPoseEquals(a: TutorialCameraState, b: TutorialCameraState): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === "orbit" && b.kind === "orbit") return a.position.every((value, index) => Math.abs(value - b.position[index]) < 1e-4) && a.target.every((value, index) => Math.abs(value - b.target[index]) < 1e-4);
  if (a.kind === "canvas" && b.kind === "canvas") return Math.abs(a.x - b.x) < 1e-4 && Math.abs(a.y - b.y) < 1e-4 && Math.abs(a.zoom - b.zoom) < 1e-4;
  return false;
}

/** @emoji 🎥️ Captures a live session into a `TutorialDefinition` — a recording IS a `TutorialDefinition`,
 * so this class simply accumulates a densely-sampled one (see the Rust core doc comment on
 * `TutorialDefinition`). Deliberately produces events/UI/camera/document tracks only: webcam/mic capture
 * (`MediaRecorder`) is an explicit, reported scope cut — see the ticket close-out summary — a text-only
 * recording is still a fully valid, useful `TutorialDefinition` per the Rust model's own optionality
 * (narration/video tracks default to empty). Document `Edit` operations are NOT captured (that would
 * require intercepting the plugin's internal vcs operation stream in per-op form, which isn't exposed to
 * this shell) — also a reported scope cut; UI/camera/events still replay faithfully. */
export class TutorialRecorder {
  private readonly startedAtMs: number;
  private readonly baseUiSnapshot: TutorialUiSnapshot;
  private readonly baseDocumentJson: string | null;
  private readonly events: TutorialEvent[] = [];
  private readonly uiKeyframes: { readonly at: number; readonly sample: { readonly kind: "snapshot"; readonly state: TutorialUiSnapshot } | { readonly kind: "delta"; readonly changes: TutorialUiChange[] } }[] = [];
  private readonly cameraKeyframes: { readonly at: number; readonly windowId: string; readonly camera: TutorialCameraState; readonly easing: "easeInOut" }[] = [];
  private readonly chapters: TutorialChapter[] = [];
  private lastUiSnapshot: TutorialUiSnapshot;
  private readonly lastCameraByWindow = new Map<string, TutorialCameraState>();

  constructor(baseUiSnapshot: TutorialUiSnapshot, baseDocumentJson: string | null) {
    this.startedAtMs = performance.now();
    this.baseUiSnapshot = baseUiSnapshot;
    this.lastUiSnapshot = baseUiSnapshot;
    this.baseDocumentJson = baseDocumentJson;
  }

  private nowMs(): number {
    return Math.max(0, Math.round(performance.now() - this.startedAtMs));
  }

  recordEvent(kind: TutorialEvent["kind"]): void {
    this.events.push({ at: this.nowMs(), kind });
  }

  recordUiDiff(next: TutorialUiSnapshot): void {
    const changes = diffTutorialUiSnapshot(this.lastUiSnapshot, next);
    if (changes.length > 0) this.uiKeyframes.push({ at: this.nowMs(), sample: { kind: "delta", changes } });
    this.lastUiSnapshot = next;
  }

  recordSnapshot(state: TutorialUiSnapshot): void {
    this.uiKeyframes.push({ at: this.nowMs(), sample: { kind: "snapshot", state } });
    this.lastUiSnapshot = state;
  }

  sampleCamera(windowId: string, camera: TutorialCameraState): void {
    const prev = this.lastCameraByWindow.get(windowId);
    if (prev && tutorialCameraPoseEquals(prev, camera)) return;
    this.lastCameraByWindow.set(windowId, camera);
    this.cameraKeyframes.push({ at: this.nowMs(), windowId, camera, easing: "easeInOut" });
  }

  /** 📖️ `ui.tutorial.addChapter` — marks the current elapsed time as a scrub-bar chapter with an
   * auto-numbered title (no naming-prompt UI in this scope; a recorded tutorial's authored titles can
   * always be hand-edited in the downloaded JSON afterward). Synthesizes a `LocalizedLabel` matrix. */
  addChapter(title?: string | LocalizedLabel): void {
    const index = this.chapters.length + 1;
    const rawTitle = title ?? `Chapter ${index}`;
    this.chapters.push({ id: `chapter-${index}`, at: this.nowMs(), title: synthesizeLocalizedLabel(rawTitle) });
  }

  build(id: string, title: string | LocalizedLabel, exampleId?: string): TutorialDefinition {
    const durationMs = Math.max(1000, this.nowMs());
    return {
      id,
      title: synthesizeLocalizedLabel(title),
      durationMs,
      chapters: this.chapters,
      base: { documentDsl: this.baseDocumentJson ?? undefined, exampleId, ui: this.baseUiSnapshot, cameras: [] },
      tracks: { narration: [], video: [], events: this.events, ui: this.uiKeyframes, document: [], camera: this.cameraKeyframes, gestures: [] },
      recordedAt: new Date().toISOString(),
    };
  }
}
//#endregion 🎥️TutorialRecorder

//#region 🐚️ShellMount
/** @emoji 🐚️ Public props for {@link FrameworkOsShell} — the multi-instance-safe entry point. `shellId`,
 * `storageNamespace`, and `ownsPage` exist so several shells can be mounted on one page: `ownsPage`
 * gates the handful of behaviors that are legitimately page-global (document title, browser history
 * sync via `bootFrameworkOs`), `storageNamespace` prefixes this shell's durable storage keys so
 * co-mounted shells don't share `semio.os.dock`/`ui.chrome.*` state. */
export interface FrameworkOsShellProps {
  readonly surfaceSessionFactories?: readonly AppSurfaceSessionFactory[];
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  /** 👁️✏️ Boot-time surface role preference (contract freeze §5) — resolved by
   * `resolveBootAppRole`/`VITE_SEMIO_APP_ROLE` at the `bootFrameworkOs` call site, default `"editor"`.
   * Used only to prefer a same-role app when `appId` doesn't pin one explicitly; the actual role of an
   * open session always comes from `session.app.role`, never this prop. */
  readonly appRole?: AppRole;
  readonly locks?: ResolvedShellLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly brand?: ShellBrand;
  readonly shellId?: string;
  readonly storageNamespace?: string;
  readonly ownsPage?: boolean;
  /** 🐚️ Skips the brand/app introduction auto-start (and any brand-owned tutorial's own auto-considered
   * reveal) for a shell that's mounted but not the one the user is actually looking at — a live
   * multi-shell page (e.g. the mit-bestand demonstrator's background panes) has no iframe boundary for
   * the existing `window.self !== window.top` heuristic below to key off, so several shells would
   * otherwise all auto-play their onboarding at once the moment they boot. Defaults to `false` (existing
   * single-shell-per-page behavior unchanged). */
  readonly suppressAutoIntroduction?: boolean;
}

//#region 🔖️Identity
/** 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3 — reads one
 * `VITE_S_*` compile-time define (`💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`'s
 * `define` block). Guarded for non-Vite embeds (SSR/tests/other bundlers) where `import.meta.env` is
 * absent — mirrors `🐚️Shell/🟦️.tsx`'s `readViteAppRoleEnv` idiom (that file is out of this
 * lane's lease, so re-implemented locally rather than imported). Returns `undefined` for an unset/
 * empty define, never `""` — every call site treats "no hub env" as "skip identity entirely" (§C3
 * "No hub env ⇒ skip all of it and keep today's local-only behaviour exactly"). */
function readViteSEnv(name: "VITE_S_HUB_URL" | "VITE_S_DATA_DIR"): string | undefined {
  try {
    const env = (import.meta as unknown as { readonly env?: Readonly<Record<string, string | undefined>> }).env;
    return env?.[name] || undefined;
  } catch {
    return undefined;
  }
}

/** 🪪️ One `MutationEnvelope` wrapping an {@link IdentityConfigMutation} — mirrors the exact shape the
 * in-source `foldIdentityEvent` tests build (`🏪️store/👷️worker/🟦️.ts`'s `🔖️DirectoryLaneTests`/identity
 * fold vectors), since this facet's diff is whole-record (never merged) and has no history/undo chrome
 * wired to it this wave (no real inverse chain needed beyond a structurally valid envelope). */
function identityMutationEnvelope(actor: string, mutation: IdentityConfigMutation, base: Identity | null): MutationEnvelope {
  const nextPayload = applyIdentityConfigMutation(base, mutation);
  return {
    id: `identity-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`,
    actor,
    document: IDENTITY_CONFIG_SCHEMA,
    schemaVersion: IDENTITY_CONFIG_SCHEMA,
    payloadHash: "",
    diff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload: nextPayload },
    inverse: { targetOperation: "sign-in", inverseDiff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload: base }, baseVersion: 0, undoPolicy: "exactBaseOnly" },
  };
}

/** 🪪️ `decodePayload` for {@link foldIdentityEvent} — a remote/cross-tab envelope's `diff.payload` is
 * either a full `Identity` record or `null` (signed out); anything else (a different facet's envelope
 * riding the same `BroadcastChannel`, contract §C6 has no such case today but this stays defensive)
 * returns `undefined` so the fold leaves `base` untouched. */
function decodeIdentityPayload(payload: unknown): Identity | null | undefined {
  if (payload === null) return null;
  if (typeof payload !== "object") return undefined;
  const candidate = payload as Partial<Identity>;
  if (typeof candidate.userId === "string" && typeof candidate.email === "string" && typeof candidate.hubBaseUrl === "string") return candidate as Identity;
  return undefined;
}

/** 🪪️ Mints the shell actor id (contract §C0: `user:{userId}#{shellSessionId}`) once identity
 * resolves; the pre-identity default stays `client-{shellSessionId}` (unchanged shape, just the
 * random-suffix source now shared with the post-sign-in id so a reload's actor id is stable relative
 * to its own tab even before/without a hub). */
export function shellActorId(sessionId: string, identity: Identity | null): string {
  return identity ? `user:${identity.userId}#${sessionId}` : `client-${sessionId}`;
}

/** 🪪️ Canonical surface id (contract §C0 `<kind>@<standard>/<subset>#<role>`) for local UI routing.
 * Hub transport surface authority comes only from the complete verified installed execution target. */
export function canonicalSurfaceId(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/** 🔌️ Accepts extension-only hot swaps while protecting a plugin-owned active app session. */
export function reloadRetainsActiveApp(apps: readonly { readonly id: string }[], activeAppId?: string): boolean {
  return activeAppId === undefined || apps.some((app) => app.id === activeAppId);
}

/** 🪪️ ticket §C4 — the space's own artifact-index document: kind `s.space`, dialect
 * `s.space.space@1/*`, document id always the literal `"index"` (one per hub space). No TS constant
 * is exported for these (lane 1-E's TS twin re-exports types only, see `📓️w1-e-report.md`), so they
 * are mirrored here from the Rust source of truth
 * (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🦀️.rs`'s `S_SPACE_INDEX_DOCUMENT_SCHEMA`/
 * `SPACE_INDEX_DIALECT`). */
const S_SPACE_INDEX_DOCUMENT_SCHEMA = "s.space";
const S_SPACE_INDEX_DOCUMENT_ID = "index";
const SPACE_INDEX_DIALECT: ArtifactDialect = { artifactKind: "s.space.space", standard: "1", subset: "*" };

/** 📇️ §5 "wire the routing by dialect/surface id so it works the moment 2-B lands" — a direct
 * manifest scan (rather than the `AppRouter`/`appRouter.entriesFor` machinery, which is declared
 * later in `FrameworkOsShellInner` than `applyShellUri` and would be a `const` temporal-dead-zone
 * reference from there) for the one app a plugin declares for a given `(dialect, role)`. */
function findDialectApp(plugin: LoadedProgramState | undefined, dialect: ArtifactDialect, role: AppRole): AppDefinition | undefined {
  return plugin?.manifest.apps.find((app) => app.dialect && dialectCoordinate(app.dialect) === dialectCoordinate(dialect) && app.role === role);
}

/** 👁️✏️ Fixed per-role icons for the navbar role group. Deliberately NOT the target app's own
 * `iconId`: both surfaces of one artifact normally carry the SAME artifact icon, which would make the
 * two buttons indistinguishable at a glance. */
export const SURFACE_ROLE_ICON_IDS: Readonly<Record<AppRole, IconName>> = { editor: "pencil", viewer: "eye" };

export type { DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1 };

/** 🧾️ One retained, request-id-keyed command completion. A receipt lands here BEFORE any accepted
 * event is folded, so a completion always has an owner; a one-shot invite capability lives only in
 * this slot until an explicit administration copy handler consumes it. */
export type DirectoryCommandResultSlotV1 = { readonly kind: "receipt"; readonly receipt: DirectoryCommandReceiptV1 } | { readonly kind: "failed"; readonly code: DirectoryCommandErrorCodeV1 };

/** 📏️ Bounded retained result depth; the oldest completion is retired first. */
export const DIRECTORY_COMMAND_RESULT_SLOTS = 64;

/** 🧾️ Retains one bounded completion, newest last, replacing any prior entry for the same id. */
export function retainDirectoryCommandResult(slots: Map<string, DirectoryCommandResultSlotV1>, requestId: string, result: DirectoryCommandResultSlotV1): void {
  slots.delete(requestId);
  while (slots.size >= DIRECTORY_COMMAND_RESULT_SLOTS) {
    const oldest = slots.keys().next();
    if (oldest.done === true) break;
    slots.delete(oldest.value);
  }
  slots.set(requestId, result);
}

/** 🆔️ Mints one fresh 32-hex nonzero idempotency correlation. It is a correlation, never a
 * capability: the hub re-runs authentication and authorization before returning any completion. */
export function mintDirectoryCommandRequestId(): string {
  const bytes = new Uint8Array(16);
  globalThis.crypto.getRandomValues(bytes);
  bytes[0] = (bytes[0] ?? 0) | 1;
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 🌱️ Converts the trusted-catalog presentation choice into the closed host creation request.
 * The decoded schema/dialect/labels are deliberately discarded; Hub resolves `kindId` again. */
export type SpaceArtifactCreationCatalogAuthorityV1 = Readonly<{
  runtimeKey: string;
  clientInstanceId: string;
  spaceId: string;
  catalogGenerationId: string;
  kindChoices: readonly string[];
}>;

export type SpaceArtifactCreationCatalogProjectionV1 = Readonly<{
  phase: "loading" | "ready" | "unavailable";
  kinds: readonly ArtifactKindChoice[];
  choiceRevision: string;
  authority: SpaceArtifactCreationCatalogAuthorityV1 | null;
}>;

export type ArtifactKindChoiceDraftDefinitionV1 = Readonly<{
  ownerId: string;
  args: readonly Readonly<{ id: string; artifactKind: boolean }>[];
}>;

/** 🧹️ Selects only staged artifact-kind fields; static choices and text remain untouched. */
export function artifactKindChoiceDraftRetirementsV1(
  stagedByOwner: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  definitions: readonly ArtifactKindChoiceDraftDefinitionV1[],
): readonly Readonly<{ ownerId: string; argId: string }>[] {
  const retirements: { ownerId: string; argId: string }[] = [];
  for (const definition of definitions) {
    const staged = stagedByOwner[definition.ownerId];
    if (staged === undefined) continue;
    for (const arg of definition.args) if (arg.artifactKind && Object.prototype.hasOwnProperty.call(staged, arg.id) && staged[arg.id] !== undefined) retirements.push({ ownerId: definition.ownerId, argId: arg.id });
  }
  return retirements;
}

/** 🌱️ Captures only a ready catalog owned by the exact mounted Space-index instance. */
export function captureSpaceArtifactCreationCatalogAuthorityV1(
  catalog: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> | null,
  status: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> | null,
  origin: ShellDialogOriginV1 | null,
): SpaceArtifactCreationCatalogAuthorityV1 | null {
  const document = origin?.document;
  const scope = document?.scope;
  if (document === null || document === undefined || scope === null || scope === undefined || scope.documentId !== S_SPACE_INDEX_DOCUMENT_ID
    || status?.phase !== "ready" || status.clientInstanceId !== document.clientInstanceId || status.spaceId !== scope.spaceId
    || catalog === null || catalog.clientInstanceId !== document.clientInstanceId || catalog.spaceId !== scope.spaceId) return null;
  return {
    runtimeKey: document.runtimeKey,
    clientInstanceId: document.clientInstanceId,
    spaceId: scope.spaceId,
    catalogGenerationId: catalog.catalogGenerationId,
    kindChoices: catalog.kinds.map(encodeArtifactKindChoice),
  };
}

/** 🗂️ Projects the current mounted catalog without falling back to process manifests. */
export function selectedSpaceArtifactCreationCatalogV1(
  catalog: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> | null,
  status: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> | null,
  origin: ShellDialogOriginV1 | null,
): SpaceArtifactCreationCatalogProjectionV1 | null {
  const document = origin?.document;
  const scope = document?.scope;
  if (document === null || document === undefined || scope === null || scope === undefined || scope.documentId !== S_SPACE_INDEX_DOCUMENT_ID) return null;
  const authority = captureSpaceArtifactCreationCatalogAuthorityV1(catalog, status, origin);
  const phase = authority !== null ? "ready" : status?.clientInstanceId === document.clientInstanceId && status.spaceId === scope.spaceId ? status.phase : "loading";
  const safePhase = phase === "ready" ? "unavailable" : phase;
  const effectivePhase = authority === null ? safePhase : "ready";
  return {
    phase: effectivePhase,
    kinds: authority === null ? [] : catalog!.kinds,
    choiceRevision: `${document.runtimeKey}:${document.clientInstanceId}:${scope.spaceId}:${effectivePhase}:${authority?.catalogGenerationId ?? "none"}`,
    authority,
  };
}

/** 🌱️ Converts only an exact current selected-catalog member into the closed host request. */
export function spaceArtifactCreationRequestFromAction(
  actionId: string,
  args: Readonly<Record<string, unknown>> | undefined,
  spaceId: string,
  requestId: string,
  capturedCatalog: SpaceArtifactCreationCatalogAuthorityV1 | null,
  currentCatalog: SpaceArtifactCreationCatalogAuthorityV1 | null,
): Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" }> | null {
  if (actionId !== "os.create-space-artifact" || args === undefined || Object.keys(args).sort().join(",") !== "kindChoice,name" || typeof args.kindChoice !== "string" || typeof args.name !== "string") return null;
  try {
    const choice = decodeArtifactKindChoice(args.kindChoice);
    const encoded = encodeArtifactKindChoice(choice);
    if (encoded !== args.kindChoice || choice.kindId !== choice.dialect.artifactKind || capturedCatalog === null || currentCatalog === null
      || capturedCatalog.runtimeKey !== currentCatalog.runtimeKey || capturedCatalog.clientInstanceId !== currentCatalog.clientInstanceId
      || capturedCatalog.spaceId !== spaceId || currentCatalog.spaceId !== spaceId || capturedCatalog.catalogGenerationId !== currentCatalog.catalogGenerationId
      || !capturedCatalog.kindChoices.includes(encoded) || !currentCatalog.kindChoices.includes(encoded)) return null;
    return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId: capturedCatalog.catalogGenerationId, kindId: choice.kindId, name: args.name };
  } catch {
    return null;
  }
}

export type SpaceArtifactCreationOwnerV1 = ArtifactCreationProgressOwnerV1 & Readonly<{
  opening: boolean;
  cancelRequested: boolean;
  ready: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }> | null;
}>;

export function spaceArtifactCreationOwnerAcceptsStatus(
  owner: SpaceArtifactCreationOwnerV1,
  message: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>,
): boolean {
  if (owner.requestId !== message.requestId || owner.spaceId !== message.spaceId || owner.expectedCatalogGenerationId !== message.catalogGenerationId || (message.ready !== undefined && message.ready.kindId !== owner.kindId)) return false;
  if (owner.ready === null) return true;
  const retained = owner.ready.ready;
  return message.phase === "ready" && retained !== undefined && message.ready !== undefined
    && message.ready.documentId === retained.documentId && message.ready.kindId === retained.kindId
    && message.ready.artifactSchema === retained.artifactSchema
    && message.ready.parentDialect.artifactKind === retained.parentDialect.artifactKind
    && message.ready.parentDialect.standard === retained.parentDialect.standard
    && message.ready.parentDialect.subset === retained.parentDialect.subset;
}

/** 🔄️ Admits one catalog refresh only for the exact nonterminal creation, selected catalog and live Space-index mount. */
export function spaceArtifactCreationCatalogRefreshRequestV1(
  owner: SpaceArtifactCreationOwnerV1 | null,
  authority: SpaceArtifactCreationCatalogAuthorityV1 | null,
  origin: ShellDialogOriginV1 | null,
  message: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-refresh-required" }>,
): Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-creation-catalog-open" }> | null {
  const document = origin?.document;
  const scope = document?.scope;
  if (owner === null || owner.ready !== null || owner.opening || authority === null || origin === null || document === null || document === undefined || scope === null || scope === undefined
    || message.requestId !== owner.requestId || message.spaceId !== owner.spaceId || message.catalogGenerationId !== owner.expectedCatalogGenerationId
    || authority.runtimeKey !== owner.runtimeKey || authority.clientInstanceId !== owner.clientInstanceId || authority.spaceId !== owner.spaceId || authority.catalogGenerationId !== owner.expectedCatalogGenerationId
    || origin.sessionInstanceId !== owner.sessionInstanceId || document.runtimeKey !== owner.runtimeKey || document.clientInstanceId !== owner.clientInstanceId
    || scope.spaceId !== owner.spaceId || scope.documentId !== S_SPACE_INDEX_DOCUMENT_ID) return null;
  return { kind: "space-artifact-creation-catalog-open", clientInstanceId: owner.clientInstanceId, spaceId: owner.spaceId };
}

export function spaceArtifactCreationReadyOpening(
  message: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>,
): Readonly<Record<string, string>> | null {
  if (message.phase !== "ready" || message.ready === undefined || message.ready.kindId !== message.ready.parentDialect.artifactKind) return null;
  return {
    artifactRef: `${message.ready.parentDialect.artifactKind}@${message.ready.parentDialect.standard}/${message.ready.parentDialect.subset}`,
    documentId: message.ready.documentId,
    spaceId: message.spaceId,
    schema: message.ready.artifactSchema,
  };
}

/** 📇️ Maps an `os.directory.<verb>` action id (contract §C6's 7 command ids) + its relayed JSON args
 * onto a {@link DirectoryCommand} — `share-link` has no directory-schema command kind of its own
 * (contract §C1), so it's client-side sugar for `create-invite` (see the new
 * `🎮️commands/🔗️directory-share-link/🦀️.rs` header doc). Returns `null` for an
 * unrecognized verb (defensive — every relay source in this codebase today only emits the 7 frozen
 * ids, but a foreign/future caller should never crash the funnel). */
export function directoryCommandFromAction(actionId: string, args: Record<string, unknown> | undefined): DirectoryCommand | null {
  const a = args ?? {};
  switch (actionId) {
    case "os.directory.create-space":
      return { kind: "create-space", name: String(a.name ?? ""), spaceKind: (a.spaceKind as string) ?? "atelier", visibility: (a.visibility as string) ?? "private" } as DirectoryCommand;
    case "os.directory.delete-space":
      return { kind: "delete-space", spaceId: String(a.spaceId ?? "") } as DirectoryCommand;
    case "os.directory.rename-space":
      return { kind: "rename-space", spaceId: String(a.spaceId ?? ""), name: String(a.name ?? "") } as DirectoryCommand;
    case "os.directory.set-visibility":
      return { kind: "set-visibility", spaceId: String(a.spaceId ?? ""), visibility: (a.visibility as string) ?? "private" } as DirectoryCommand;
    case "os.directory.upsert-member":
      return { kind: "upsert-member", spaceId: String(a.spaceId ?? ""), email: String(a.email ?? ""), role: (a.role as string) ?? "spectator" } as DirectoryCommand;
    case "os.directory.remove-member":
      return { kind: "remove-member", spaceId: String(a.spaceId ?? ""), userId: String(a.userId ?? "") } as DirectoryCommand;
    case "os.directory.share-link":
      return { kind: "create-invite", spaceId: String(a.spaceId ?? ""), role: (a.role as string) ?? "spectator", ttlSecs: Number(a.ttlSecs ?? 3600) } as DirectoryCommand;
    default:
      return null;
  }
}

//#region 🏛️SpaceAdministration
/** 🏛️ The renderer-visible state of the one retained administration operation. `page` is the parsed
 * form of the exact canonical bytes the hub sealed; it is never merged, patched, or folded, only
 * replaced wholesale by a newer page. */
export interface ShellSpaceAdministrationStateV1 {
  readonly operationEpoch: number;
  readonly spaceId: string;
  readonly phase: DirectoryAdministrationPhaseV1;
  readonly page: DirectorySpaceAdministrationPageV1 | null;
  readonly canonicalJson?: string;
  readonly receiptSha256?: string;
  readonly code?: DirectoryCommandErrorCodeV1;
  readonly inviteCapabilityPending?: boolean;
  readonly inviteCapabilityStatus?: DirectoryAdministrationInviteCapabilityStatusV1;
}

const SHELL_SPACE_ADMINISTRATION_TERMINAL: readonly DirectoryAdministrationPhaseV1[] = ["cancelled", "denied", "stale", "failed"];

/** 🏛️ The exact Home-effect bridge into one retained canonical administration operation. */
export function shellSpaceAdministrationOpening(
  actionId: string,
  args: Readonly<Record<string, unknown>> | undefined,
  operationEpoch: number,
): { readonly state: ShellSpaceAdministrationStateV1; readonly request: BackboneWorkerRequest } | null {
  const spaceId = args?.spaceId;
  if (actionId !== "os.directory.open-administration" || typeof spaceId !== "string" || spaceId.length === 0 || new TextEncoder().encode(spaceId).byteLength > 256 || !Number.isSafeInteger(operationEpoch) || operationEpoch < 0) return null;
  return {
    state: { operationEpoch, spaceId, phase: "loading", page: null },
    request: { kind: "directory-administration-open", operationEpoch, spaceId },
  };
}

/** 🎟️ Accepts a worker capability only for the exact currently rendered author page. */
export function shellSpaceAdministrationCapabilityAllowed(state: ShellSpaceAdministrationStateV1 | null, operationEpoch: number): boolean {
  return state !== null
    && state.operationEpoch === operationEpoch
    && state.phase === "ready"
    && state.page?.access === "author"
    && state.inviteCapabilityPending === true;
}

/** 🧮️ Pure reducer over one worker administration message. A message for a superseded operation is
 * ignored wholesale; a terminal phase erases the page, receipt, and capability marker in the same
 * transition, so no renderer can read stale authority after a denial or an identity change. */
export function reduceShellSpaceAdministrationState(
  current: ShellSpaceAdministrationStateV1 | null,
  message: Extract<BackboneWorkerResponse, { kind: "directory-administration-state" }>,
  operationEpoch: number,
  page: DirectorySpaceAdministrationPageV1 | null,
): ShellSpaceAdministrationStateV1 | null {
  if (message.operationEpoch !== operationEpoch) return current;
  if (current === null || current.operationEpoch !== operationEpoch || current.spaceId !== message.spaceId) return current;
  if (message.phase === "deleted") {
    if ((message.outcome !== "accepted" && message.outcome !== "previously-accepted") || message.receiptSha256 === undefined || !/^[0-9a-f]{64}$/u.test(message.receiptSha256) || message.receiptSha256 === "0".repeat(64)) return current;
    return { operationEpoch, spaceId: message.spaceId, phase: "deleted", page: null, receiptSha256: message.receiptSha256 };
  }
  if (SHELL_SPACE_ADMINISTRATION_TERMINAL.includes(message.phase)) {
    return { operationEpoch, spaceId: message.spaceId, phase: message.phase, page: null, ...(message.code === undefined ? {} : { code: message.code }) };
  }
  return {
    operationEpoch,
    spaceId: message.spaceId,
    phase: message.phase,
    page,
    ...(message.canonicalJson === undefined ? {} : { canonicalJson: message.canonicalJson }),
    ...(message.receiptSha256 === undefined ? {} : { receiptSha256: message.receiptSha256 }),
    ...(message.code === undefined ? {} : { code: message.code }),
    ...(message.inviteCapabilityPending === true ? { inviteCapabilityPending: true } : {}),
    ...(message.inviteCapabilityStatus === undefined ? {} : { inviteCapabilityStatus: message.inviteCapabilityStatus }),
  };
}

/** 🎬️ Maps one pane intent onto the retained operation's worker request. Returns `null` for an
 * intent the current page does not authorize, so an unauthorized control can never reach the wire
 * even if a hostile renderer synthesizes the event. */
export function shellSpaceAdministrationRequest(
  state: ShellSpaceAdministrationStateV1,
  intent: SpaceAdministrationIntentV1,
  requestId: string,
): BackboneWorkerRequest | null {
  const operationEpoch = state.operationEpoch;
  if (intent.kind === "close") return { kind: "directory-administration-close", operationEpoch };
  if (intent.kind === "page") return { kind: "directory-administration-refresh", operationEpoch, cursor: intent.cursor };
  const capabilities = spaceAdministrationCapabilities(state.page);
  if (capabilities === null || state.phase !== "ready" || state.page === null || state.page.access !== "author" || state.page.spaceId !== state.spaceId || state.page.space.id !== state.spaceId) return null;
  if (intent.kind === "copy-invite-capability") return state.inviteCapabilityPending === true ? { kind: "directory-administration-capability-request", operationEpoch } : null;
  const spaceId = state.spaceId;
  const submit = (command: DirectoryCommand): BackboneWorkerRequest | null => directoryAdministrationCommandAllowedV1(state.page, spaceId, command) ? { kind: "directory-administration-submit", operationEpoch, requestId, command } : null;
  if (intent.kind === "delete-space") return submit({ kind: "delete-space", spaceId });
  if (intent.kind === "rename-space") {
    if (!spaceAdministrationNameValid(intent.name)) return null;
    return submit({ kind: "rename-space", spaceId, name: intent.name });
  }
  if (intent.kind === "set-visibility") {
    if (intent.visibility !== "public" && intent.visibility !== "private") return null;
    return submit({ kind: "set-visibility", spaceId, visibility: intent.visibility });
  }
  if (intent.kind === "set-role") {
    const row = state.page.members.rows.find((member) => member.userId === intent.userId);
    if (row === undefined) return null;
    return submit({ kind: "upsert-member", spaceId, email: row.email, role: intent.role });
  }
  if (intent.kind === "remove-member") {
    const row = state.page.members.rows.find((member) => member.userId === intent.userId);
    if (row === undefined || !spaceAdministrationMemberRemovable(row, capabilities)) return null;
    return submit({ kind: "remove-member", spaceId, userId: intent.userId });
  }
  if (intent.kind === "create-invite") {
    return submit({ kind: "create-invite", spaceId, role: intent.role, ttlSecs: SHELL_SPACE_ADMINISTRATION_INVITE_TTL_SECS });
  }
  const row = state.page.invites.rows.find((invite) => invite.inviteId === intent.inviteId);
  if (row === undefined || !spaceAdministrationInviteRevocable(row, capabilities)) return null;
  return submit({ kind: "revoke-invite", spaceId, inviteId: intent.inviteId });
}

/** ⏳️ One hour: the one invite lifetime the administration pane issues, stated once here rather
 * than smuggled through a renderer-supplied number. */
export const SHELL_SPACE_ADMINISTRATION_INVITE_TTL_SECS = 3600;

/** 📋️ Minimal clipboard boundary; callers can prove unavailable and rejected writes without
 * exporting a browser implementation type. */
export interface DirectoryInviteClipboardV1 {
  writeText(text: string): Promise<void>;
}

/** 📋️ Writes the one-shot invite capability to the clipboard. The worker remains its owner until
 * the exact transfer result returns; the token is never logged, folded, put in a URL, or React state. */
export async function copyDirectoryInviteCapabilityV1(
  inviteToken: string,
  clipboard: DirectoryInviteClipboardV1 | undefined = globalThis.navigator?.clipboard,
): Promise<boolean> {
  if (clipboard === undefined) return false;
  try {
    await clipboard.writeText(inviteToken);
    return true;
  } catch {
    return false;
  }
}
//#endregion 🏛️SpaceAdministration
//#endregion 🔖️Identity

/** @emoji 🐚️ Resolves the {@link ShellScope.storage} port for a shell mount: ephemeral brands always get
 * an in-memory port (never durable, regardless of namespace); a namespaced non-ephemeral shell gets a
 * scoped view over browser storage; a bare non-ephemeral shell (the historical single-app-per-page
 * case) gets the plain shared browser port. */
function resolveShellScopeStorage(ephemeral: boolean, storageNamespace: string | undefined): StoragePort {
  if (ephemeral) return createMemoryStoragePort();
  const browser = createBrowserStoragePort();
  return storageNamespace ? createScopedStoragePort(browser, storageNamespace) : browser;
}

//#region 🧵️ConcurrencyHelpers
/** 🧮️ terra-web-shellhost (finding 2) — MIRRORS `PluginRuntime/🟦️.tsx`'s private
 * `poolConcurrency()` formula exactly (`min(hardwareConcurrency-1, 4)`, same `5` SSR/test fallback so
 * the clamp still lands on `4`): the boot-time install fan-out below must not run more concurrent
 * plugin activations than there are shard workers to service them (`PluginRuntime`'s own
 * `getShardClient` sizes its pool with this same formula) — requesting more would only add
 * `ActivationRegistry.evictForMemoryPressure` LRU thrashing for zero extra real parallelism, exactly
 * the reasoning `poolConcurrency()`'s own doc comment gives. This is a duplicate of that private
 * function, not a second invented bound — see this packet's `📓️terra-web-shellhost-report.md`
 * `## lease-requests` for the ask to export it from `PluginRuntime` instead, at which point this
 * function should be deleted in favor of importing it directly. */
function pluginInstallConcurrency(): number {
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  return Math.max(1, Math.min(hardwareConcurrency - 1, 4));
}
//#endregion 🧵️ConcurrencyHelpers

//#region 🩺️RuntimeDiagnostics
/** @emoji 🩺️ The ONE key that arms this shell's per-action runtime traces — the refresh/completion
 * chatter a boot emits once per dispatched action. Off by default: a served boot of the
 * hexagonal-mushroom-column printed these on every one of ~1440 typed-operation completions
 * (`📓️runtime-verification-2026-09-09.md` boot #7), which is signal a perf run wants and an
 * interactive boot must never pay for.
 *
 * 🪞️ Mirrors the guest-side switch `RUNTIME_DIAGNOSTICS_ENV`
 * (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs`) key for key, so one name arms both sides of the wire; the
 * two cannot share a declaration across the language boundary, so
 * `🧪️tests/🔬️engine-contract/🟦️.ts` pins the string instead
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export const RUNTIME_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";

/** @emoji 🩺️ Armed by `1`/`true`/`on`/`yes`; anything else, including absent, leaves it off. */
function runtimeDiagnosticsArmed(value: unknown): boolean {
  return typeof value === "string" && ["1", "true", "on", "yes"].includes(value.trim().toLowerCase());
}

let runtimeDiagnosticsOverride: boolean | undefined;
let runtimeDiagnosticsResolved: boolean | undefined;

/** @emoji 🩺️ Arms or disarms the traces for this page, outranking build env and stored preference —
 * the browser's counterpart to the guest's `set_runtime_diagnostics`. */
export function setRuntimeDiagnostics(enabled: boolean | undefined): void {
  runtimeDiagnosticsOverride = enabled;
  runtimeDiagnosticsResolved = undefined;
}

/** @emoji 🩺️ Whether the shell's per-action traces may print. Resolved once per page: an explicit
 * override wins, then the build's `VITE_SEMIO_RUNTIME_DIAGNOSTICS`, then a `localStorage` key of the
 * same name so a live tab can be armed without a rebuild. Every reader is wrapped, because a
 * sandboxed tab throws on `localStorage` and a non-Vite host has no `import.meta.env`. */
export function runtimeDiagnosticsEnabled(): boolean {
  if (runtimeDiagnosticsOverride !== undefined) return runtimeDiagnosticsOverride;
  if (runtimeDiagnosticsResolved !== undefined) return runtimeDiagnosticsResolved;
  let armed = false;
  try {
    armed = runtimeDiagnosticsArmed((import.meta as { readonly env?: Record<string, unknown> }).env?.[`VITE_${RUNTIME_DIAGNOSTICS_KEY}`]);
  } catch {
    armed = false;
  }
  if (!armed) {
    try {
      armed = runtimeDiagnosticsArmed(globalThis.localStorage?.getItem(RUNTIME_DIAGNOSTICS_KEY));
    } catch {
      armed = false;
    }
  }
  runtimeDiagnosticsResolved = armed;
  return armed;
}
//#endregion 🩺️RuntimeDiagnostics

//#region 🔁️InvokeExtensionDispatch
function captureExtensionCompletion(requestingPlugin: LoadedProgramState, instanceId: number, req: bigint): PluginExtensionCompletion {
  const capture = requestingPlugin.handle.captureExtensionCompletion;
  if (typeof capture !== "function") throw new Error("extension.completion-unavailable");
  const completion = capture.call(requestingPlugin.handle, instanceId, req);
  if (!completion || typeof completion.complete !== "function" || typeof completion.assertActive !== "function") throw new Error("extension.completion-unavailable");
  if (completion.instanceId !== instanceId || completion.req !== req) throw new Error("extension.completion-owner-mismatch");
  completion.assertActive();
  return completion;
}

/** 🚑️ The typed fault an extension request carries when its callee's WORKER was taken down (the host
 * watchdog, or a crash) rather than the guest refusing. Retryable by construction: the shard is
 * rebuilt before this is even raised, so re-issuing the identical request is the correct response. */
export const EXTENSION_WORKER_LOST_FAULT = "extension.worker-lost";

async function runCapturedExtensionEffect(
  completion: PluginExtensionCompletion,
  extensionEntry: LoadedProgramState | undefined,
  extensionId: string,
  capability: string,
  requestJson: string,
  requesterActorKey: string,
): Promise<InvocationResponse> {
  completion.assertActive();
  const { instanceId, req } = completion;
  const unavailable = (code: string) => new SemioFaultError({
    origin: "os", code, severity: "error", message: code,
    scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: false,
  });
  let outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array };
  try {
    if (!extensionEntry) throw unavailable("extension.missing");
    // 📥️ `PluginWasmHandle.invoke` IS the ABI's inbound-request door (`🔌️PluginRuntime`'s own
    // `invoke` doc): it submits `Event::Request` on the extension's request actor and returns the
    // `respond` answer. The typed refusal below now only ever names a handle that genuinely has no
    // door — a test double, or a target that never adapted one — never a loaded extension.
    const { invoke } = extensionEntry.handle;
    if (typeof invoke !== "function") throw unavailable("extension.invoke-unavailable");
    // 🛑️ The door honours an `AbortSignal` at turn boundaries and `invoke` forwards one, but nothing
    // ever handed it one, so `cancelPreviewEval` reached the requesting guest's own bookkeeping and
    // never the request already parked on the extension actor. Registering the controller under the
    // REQUESTER's actor key is what lets a surface's declared cancel affordance retire it from
    // outside the per-actor queue those requests are serialized in — a guest-emitted cancel would
    // queue behind the very call it means to stop (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    const cancellation = beginCancellableExtensionRequest(requesterActorKey);
    let raw: Uint8Array;
    try {
      raw = await invoke.call(extensionEntry.handle, capability, requestJson, { originInstanceId: instanceId, signal: cancellation.signal });
    } finally {
      cancellation.finish();
    }
    completion.assertActive();
    // 📦️ The guest answers a capability in the capability's OWN encoding (`evaluate_invoke_json`
    // produces JSON text); the completion that carries it back is a `pack`, so the one re-encoding
    // happens here and nowhere else (`📓️extension-result-realloc-2026-09-10.md` §4.5).
    outcome = { ok: encodePackValue(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(raw))) };
  } catch (error) {
    completion.assertActive();
    // 🚑️ A worker the host's own watchdog took down is NOT an anonymous invocation failure: it is a
    // recoverable, retryable loss of the callee, and the requester's surface must be able to say so
    // (`flow.extension-evaluate-failed` carries `faultCode` straight into the preview status). Naming
    // it `extension.invoke-failed` alongside a genuine guest refusal left the surface unable to
    // distinguish "the geometry kernel refused this" from "the geometry kernel's worker died"
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
    const fault: Fault = error instanceof SemioFaultError ? error.fault : isShardLostError(error) ? {
      origin: "os", code: EXTENSION_WORKER_LOST_FAULT, severity: "error", message: error instanceof Error ? error.message : String(error),
      scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: true,
    } : {
      origin: "os", code: "extension.invoke-failed", severity: "error", message: error instanceof Error ? error.message : String(error),
      scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: false,
    };
    outcome = { fault: encodePackValue(fault) };
    // 🩺 The DECODED fault, not just "something faulted": the completion this builds reaches the
    // requesting guest as opaque bytes, so this line is the only place a boot log can say WHICH
    // door refused (ticket 26/09/09/PROCEDURAL-3D-END-TO-END — 154 fault bytes that turned out to
    // be `extension.invoke-unavailable` on every evaluate, for eight boots running).
    console.warn("[DEBUG] extension invocation refused", JSON.stringify({ extensionId, capability, origin: fault.origin, code: fault.code, message: fault.message }));
  }
  completion.assertActive();
  const response = await completion.complete(outcome);
  completion.assertActive();
  if (!("ok" in outcome)) console.warn("[DEBUG] extension invocation faulted", { extensionId, capability, instanceId, req });
  return response;
}

/** 🔁️ Captures one requester before evaluation and delivers only to that same activation. */
export async function runInvokeExtensionEffect(requestingPlugin: LoadedProgramState, extensionEntry: LoadedProgramState | undefined, instanceId: number, extensionId: string, capability: string, requestJson: string, req: bigint): Promise<InvocationResponse> {
  return runCapturedExtensionEffect(captureExtensionCompletion(requestingPlugin, instanceId, req), extensionEntry, extensionId, capability, requestJson, `${requestingPlugin.handle.pluginId}:${instanceId}`);
}

/** 📨️ Resolves an extension address and serializes requests belonging to one originating instance.
 *
 * 🪪️ `extensionId` IS a loaded program's `pluginId` — the one address this shell can resolve, and the
 * one the framework's own fixture pins (`🧫️fixtures/🔣️extension-invocation.json`). A producer whose
 * domain names extensions differently (flow calls the brep kernel `brep`, its plugin is
 * `flow-extension-brep`) translates on ITS side, where the owning plugin id is known — never here:
 * this shell stays domain-neutral and knows no topic vocabulary. The former fallback scanned
 * `manifest.contributions` for an `extensionId` field, which no manifest has ever carried (that
 * lane is `playbookBlockKind` rows; contributed extension payloads live under
 * `manifest.topicContributions[].payload`), so it matched nothing and every flow evaluation faulted
 * `extension.missing` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export async function dispatchInvokeExtensionEffect(
  plugins: readonly LoadedProgramState[],
  requester: Pick<ActiveSession, "pluginId" | "instanceId">,
  invocation: Extract<Effect, { readonly invokeExtension: unknown }>["invokeExtension"],
  publish: (requestingPlugin: LoadedProgramState, response: InvocationResponse) => Promise<void>,
): Promise<void> {
  const { extensionId, capability, requestJson, req } = invocation;
  const requestingPlugin = plugins.find((entry) => entry.handle.pluginId === requester.pluginId);
  if (!requestingPlugin) throw new Error("extension.requester-unavailable");
  const extensionEntry = plugins.find((entry) => entry.handle.pluginId === extensionId);
  if (!extensionEntry) console.warn("[DEBUG] invokeExtension unresolved", { extensionId, capability, req, loaded: plugins.map((entry) => entry.handle.pluginId) });
  const completion = captureExtensionCompletion(requestingPlugin, requester.instanceId, req);
  const requesterActorKey = `${requester.pluginId}:${requester.instanceId}`;
  const response = await serializePerActor(requesterActorKey, () => runCapturedExtensionEffect(completion, extensionEntry, extensionId, capability, requestJson, requesterActorKey));
  completion.assertActive();
  await publish(requestingPlugin, response);
}
//#endregion 🔁️InvokeExtensionDispatch

/** @emoji 🐚️ Mounts a `.semio-scope` root (theme/appearance/id scoping lands with later waves) carrying a
 * {@link ShellScope} — the seam that lets several of these coexist on one page — around the actual shell
 * implementation in {@link FrameworkOsShellInner}. */
export function FrameworkOsShell(props: FrameworkOsShellProps): React.ReactElement {
  const { shellId, storageNamespace, ownsPage = false, brand, locks, ...innerProps } = props;
  const ephemeral = isEphemeralShellBrand(brand);
  const [scope] = useState<ShellScope>(() => {
    const storage = resolveShellScopeStorage(ephemeral, storageNamespace);
    // 🐚️ Resolved synchronously (not in a `useEffect`) so an embedded shell never flashes the wrong
    // locale's chrome on its first paint, mirroring `initUiLocaleSync`'s reasoning for the page-owning
    // case. `locks.locale` and any previously-stored preference cover the common cases; a brand's own
    // `defaults.locale` (not available yet here) still lands moments later via the uiPrefs effect below.
    const initialLocale = locks?.locale ?? readUiPreferences(storage).locale ?? detectShellLocale(typeof navigator !== "undefined" ? navigator.language : undefined);
    return createShellScope({ shellId, ownsPage, storage, initialLocale });
  });
  // 🐚️ `scope.rootRef` is a stable object (its identity never changes), so a descendant hook that puts
  // the REF ITSELF in a `useEffect`/`useLayoutEffect` dependency array would never re-fire once the ref
  // attaches. This state bump forces one guaranteed re-render right after attachment so descendants that
  // read `scope.rootRef.current` fresh at render time (see `FrameworkOsShellInner`'s
  // `useElementsSurfaceChrome`/`useCanvasAppearanceSync` calls) pick up the real element instead of
  // sticking with whatever they saw (usually `null`) on the very first render.
  const [, bumpAfterRootAttach] = useState(0);
  const setRoot = useCallback((node: HTMLDivElement | null) => {
    scope.rootRef.current = node;
    bumpAfterRootAttach((n) => n + 1);
  }, [scope]);
  const setPortalLayer = useCallback((node: HTMLDivElement | null) => {
    scope.portalLayerRef.current = node;
    // Same attach bump as setRoot: UIIntroduction portals into this layer and must re-render once it exists.
    bumpAfterRootAttach((n) => n + 1);
  }, [scope]);
  useEffect(() => () => disposeShellI18nInstance(scope.i18n), [scope]);
  return (
    <div ref={setRoot} className="semio-scope" data-shell-id={scope.shellId} style={{ position: "relative", height: "100%", width: "100%", isolation: "isolate" }}>
      <ShellScopeProvider scope={scope}>
        <FrameworkOsShellInner {...innerProps} locks={locks} brand={brand} />
        <div data-semio-portal-layer ref={setPortalLayer} className="pointer-events-none absolute inset-0" />
      </ShellScopeProvider>
    </div>
  );
}
//#endregion 🐚️ShellMount

//#region 🔖️BuiltNodeStoreCache
/** 🦴 The one placeholder body every not-yet-refreshed window shares. Reference-stable on purpose:
 * `pendingWindowUiNode()` allocates a fresh node per call, so calling it inside the shell's JSX made
 * every render look like an authored-node change to {@link createBuiltNodeStoreCacheV1} and reloaded
 * that window's store on each pass. */
const PENDING_WINDOW_UI_NODE: BuiltNode = pendingWindowUiNode();

/** 🧬️ One `📃️UiDocumentStore` per window/panel key, reloaded from its authored `BuiltNode` only when
 * that node's own reference changed. `storeFor` is called from JSX during the shell's render, so it
 * MUST NOT mutate a store that already has mounted `UiNodeView` subscribers: `loadSnapshot` notifies
 * them, and a subscriber update raised while another component renders is exactly React's
 * "Cannot update a component (`UiNodeView`) while rendering a different component
 * (`FrameworkOsShellInner`)" warning (`📓️runtime-verification-2026-09-09.md` boot #3). A first load
 * happens inline — a store nobody has subscribed to yet cannot notify anyone — and every later reload
 * is queued for {@link BuiltNodeStoreCacheV1.flushPendingReloads}, which the shell calls from a layout
 * effect (post-commit, pre-paint, so nothing renders stale). */
export type BuiltNodeStoreCacheV1 = {
  readonly storeFor: (key: string, node: BuiltNode) => UiDocumentStore;
  readonly flushPendingReloads: () => void;
  readonly forceReload: (key: string, node: BuiltNode) => void;
  readonly pendingReloadKeys: () => readonly string[];
};

export function createBuiltNodeStoreCacheV1(): BuiltNodeStoreCacheV1 {
  const stores = new Map<string, { node: BuiltNode; readonly store: UiDocumentStore }>();
  const pending = new Map<string, BuiltNode>();
  return {
    storeFor: (key, node) => {
      const existing = stores.get(key);
      if (!existing) {
        const store = new UiDocumentStore(key);
        store.loadSnapshot(builtNodeToSnapshot(key, node));
        stores.set(key, { node, store });
        return store;
      }
      if (existing.node === node) pending.delete(key);
      else pending.set(key, node);
      return existing.store;
    },
    flushPendingReloads: () => {
      for (const [key, node] of pending) {
        const entry = stores.get(key);
        if (!entry) continue;
        entry.store.loadSnapshot(builtNodeToSnapshot(key, node));
        entry.node = node;
      }
      pending.clear();
    },
    forceReload: (key, node) => {
      const existing = stores.get(key);
      if (!existing) return;
      pending.set(key, node);
    },
    pendingReloadKeys: () => [...pending.keys()],
  };
}
//#endregion 🔖️BuiltNodeStoreCache

function FrameworkOsShellInner({
  pluginFilter,
  plugins,
  surfaceSessionFactories,
  appId,
  appRole,
  locks: locksProp,
  defaults: defaultsProp,
  brand,
  suppressAutoIntroduction = false,
}: {
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly surfaceSessionFactories?: readonly AppSurfaceSessionFactory[];
  readonly appId?: string;
  readonly appRole?: AppRole;
  readonly locks?: ResolvedShellLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly brand?: ShellBrand;
  readonly suppressAutoIntroduction?: boolean;
}) {
  const scope = useShellScope();
  const shellContextMenuTitleLabel = useLabel("ui.surfaceContextMenu.workspace");
  // 🏠️🧳️ `hostConfig` is the sole piece of per-plugin identity knowledge the shell needs (which app id is
  // "landing", which is "host") — every controller id / default panel tab derives from the *loaded*
  // manifest's own `controllerId`/`panelTabs` on those apps below, never from a separate literal.
  const hostConfig = pluginFilter ? resolvePluginHostConfig(PLUGIN_CATALOG, pluginFilter) : undefined;
  const hostMode = hostConfig !== undefined;
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const locks = locksProp ?? EMPTY_SHELL_LOCKS;
  const defaults = defaultsProp ?? EMPTY_SHELL_DEFAULTS;
  const ephemeral = isEphemeralShellBrand(brand);
  const [shellState, dispatch] = useReducer(shellReducer, undefined, () => initialShellState({ pluginFilter, plugins, locks, defaults, storage: scope.storage }));
  const [historyProjection, setHistoryProjection] = useState<{ readonly cursor: number; readonly entries: Readonly<Record<number, HistoryEntry>>; readonly canUndo: boolean; readonly canRedo: boolean; readonly currentCheckpointId: string | undefined }>({ cursor: 0, entries: {}, canUndo: false, canRedo: false, currentCheckpointId: undefined });
  const historyOrderRef = useRef(0);
  const exampleOptionsRef = useRef<readonly { readonly id: string }[]>([]);
  const lastDispatchedExampleIdRef = useRef("");
  const localHistoryOrderRef = useRef(0);
  const { loadedPlugins, pluginStatusById, pluginSupervisorById, session, error, sessionFault, instanceFault } = shellState.pluginRuntime;
  // 🌐️ Read with the other render-time `shellState` slice above the memo block below: `spacePrograms`'s dependency
  // array reads `uiLocale`/`uiTerminology` during this render, and a `const` declared after it is still in its
  // temporal dead zone there — the shell then throws `Cannot access 'uiLocale' before initialization` on first paint.
  const { uiAppearance, uiLayout, uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale, uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft, uiKeybindingOverrides } = shellState.uiPrefs;
  const boardSessionFactory = useMemo(() => resolveAppSurfaceSessionFactory(surfaceSessionFactories ?? [], session ? { pluginId: session.pluginId, appId: session.app.id, instanceId: session.instanceId } : null), [surfaceSessionFactories, session?.pluginId, session?.app.id, session?.instanceId]);
  const applyHistoryPatch = useCallback((patch: HistoryPatch | undefined, replace = false) => {
    if (!patch) return;
    let applied = false;
    setHistoryProjection((current) => {
      if (!historyPatchShouldApplyV1(current.cursor, patch, replace)) {
        console.warn("[DEBUG] history patch skipped", JSON.stringify({ replace, currentCursor: current.cursor, patchCursor: patch.cursor, upserts: patch.upserts?.length ?? 0, canUndo: patch.canUndo ?? null }));
        return current;
      }
      applied = true;
      console.warn("[DEBUG] history patch applied", JSON.stringify({ replace, currentCursor: current.cursor, patchCursor: patch.cursor, upserts: patch.upserts?.length ?? 0, labels: (patch.upserts ?? []).map((entry) => entry.label), canUndo: patch.canUndo ?? null }));
      localHistoryOrderRef.current = ++historyOrderRef.current;
      const entries = replace ? {} as Record<number, HistoryEntry> : { ...current.entries };
      for (const entry of patch.upserts ?? []) entries[entry.seq] = entry;
      // 📌️ §C5 — `currentCheckpointId` used to be dropped here even though `HistoryPatch` always
      // carried it; `🔖️CheckIn` below watches it change to know a checkpoint it asked for actually
      // landed (see `touchSpaceIndexArtifact`'s call site).
      return { cursor: patch.cursor, entries, canUndo: patch.canUndo ?? false, canRedo: patch.canRedo ?? false, currentCheckpointId: replace ? patch.currentCheckpointId : (patch.currentCheckpointId ?? current.currentCheckpointId) };
    });
    if (applied) {
      const navbarExample = navbarExampleIdFromHistoryUpserts(patch.upserts, lastDispatchedExampleIdRef.current, resolveBootExampleId("", exampleOptionsRef.current, defaults.exampleId));
      if (navbarExample !== undefined) {
        console.warn("[DEBUG] navbar example from history", JSON.stringify({ navbarExample, remembered: lastDispatchedExampleIdRef.current }));
        dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: navbarExample });
      }
    }
  }, []);
  const leftoverInspectionEpochRef = useRef(0);
  const [leftoverInspectionEpoch, setLeftoverInspectionEpoch] = useState(0);
  const leftoverInspectionHasSelectionRef = useRef(false);
  const leftoverInspectionSelectedKeyRef = useRef("");
  const leftoverInspectionIdsRef = useRef<readonly string[]>([]);
  const leftoverReplaceRefreshBodiesV1 = (): boolean => {
    const leftover = leftoverWorldArmedWindowOverlayV1();
    const hoverVortex = leftover?.hoveredId && (leftover.hoveredDomain === "vortex" || leftover.hoveredId.includes(":")) ? leftover.hoveredId : null;
    if (leftoverBrushPreviewWindowHash(leftover?.activeUtility, hoverVortex, "cached") === undefined) return leftoverBrushTickSettledRef.current;
    const leftoverIds = leftoverInspectionIdsRef.current.length > 0 ? leftoverInspectionIdsRef.current : leftover?.ids ?? [];
    return leftoverInspectionPanelHash(leftoverIds, "cached") === undefined;
  };
  const forceReloadLiveUiStoresV1 = (cache: Map<string, { hash?: string; value?: unknown }>) => {
    const storeCache = builtNodeStoreCacheRef.current;
    for (const [key, entry] of cache) {
      if (!entry?.value) continue;
      if (!key.startsWith("window:") && !key.startsWith("panel:")) continue;
      storeCache.forceReload(key, entry.value as BuiltNode);
    }
    storeCache.flushPendingReloads();
  };
  const leftoverBrushHoverKeyRef = useRef<string | null>(null);
  const leftoverBrushTickSettledRef = useRef(false);
  const leftoverBrushRefreshInFlightRef = useRef(false);
  const leftoverBrushRefreshPendingRef = useRef(false);
  const leftoverBrushPreviewEpochRef = useRef(0);
  const [leftoverBrushPreviewEpoch, setLeftoverBrushPreviewEpoch] = useState(0);
  // 🪟️ `windowId` is the pane the action that produced `output` addressed. A leftover carries per-window
  // state (hover, the armed utility, its brush preview), so it is published under THAT pane and never
  // over every pane's record (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B39); a windowless action
  // publishes the document's shared fields only and leaves every pane's own fields untouched.
  const applyLeftoverInteractionView = useCallback((output: unknown, actionId?: string, addressedWindowId?: string) => {
    const published = interactionViewFromLeftoverOutput(output);
    const windowId = addressedWindowId ?? published?.windowId ?? undefined;
    const armLeftoverBrushPreview = (utility: string | null | undefined, hoverVortex: string | null, previewJson?: string | null) => {
      if (leftoverBrushPreviewRefreshReady(actionId, utility, hoverVortex, previewJson)) leftoverBrushTickSettledRef.current = true;
      else if (leftoverBrushPreviewWindowHash(utility, hoverVortex, "cached") !== undefined) leftoverBrushTickSettledRef.current = false;
      if (!leftoverBrushTickSettledRef.current) return;
      leftoverBrushHoverKeyRef.current = hoverVortex;
      leftoverBrushRefreshPendingRef.current = true;
      if (!leftoverBrushRefreshInFlightRef.current) {
        leftoverBrushPreviewEpochRef.current += 1;
        setLeftoverBrushPreviewEpoch(leftoverBrushPreviewEpochRef.current);
      }
    };
    if (!published) {
      const leftover = windowId ? leftoverWorldWindowOverlayV1(windowId) : leftoverWorldArmedWindowOverlayV1();
      const hoverVortex = leftover?.hoveredId && (leftover.hoveredDomain === "vortex" || leftover.hoveredId.includes(":")) ? leftover.hoveredId : null;
      armLeftoverBrushPreview(leftover?.activeUtility, hoverVortex, leftover?.brushPreviewJson);
      return;
    }
    const priorLeftover = windowId ? leftoverWorldWindowOverlayV1(windowId) : leftoverWorldSelectionOverlayV1();
    const overlay = leftoverOverlayCarryingSelectionV1(
      leftoverOverlayCarryingUtilityV1(
        { ids: published.selectedIds, hoveredId: published.hoverTarget?.id ?? null, hoveredDomain: published.hoverTarget?.domain ?? null, gumballActive: published.gumballActive, gumballAnchorId: published.gumballAnchorId, ...(published.activeUtility !== undefined ? { activeUtility: published.activeUtility } : {}), ...(published.brushPreviewJson ? { brushPreviewJson: published.brushPreviewJson } : {}) },
        priorLeftover,
      ),
      priorLeftover,
    );
    publishLeftoverWorldSelectionV1(overlay, windowId ? { kind: "window", windowId } : { kind: "document" });
    dispatch({ type: "INTERACTION_STATE_OBSERVED", state: leftoverInteractionStateV1({ ...published, selectedIds: overlay.ids }) });
    leftoverInspectionHasSelectionRef.current = overlay.ids.length > 0;
    const selectedKey = overlay.ids.join("\0");
    if (overlay.ids.length > 0 && leftoverInspectionSelectedKeyRef.current !== selectedKey) {
      leftoverInspectionSelectedKeyRef.current = selectedKey;
      leftoverInspectionIdsRef.current = overlay.ids;
      leftoverInspectionEpochRef.current += 1;
      setLeftoverInspectionEpoch(leftoverInspectionEpochRef.current);
    }
    const hoverVortex = overlay.hoveredId && (overlay.hoveredDomain === "vortex" || overlay.hoveredId.includes(":")) ? overlay.hoveredId : null;
    const utility = overlay.activeUtility ?? priorLeftover?.activeUtility;
    armLeftoverBrushPreview(utility, hoverVortex, published.brushPreviewJson);
  }, []);
  const refreshHistorySnapshot = useCallback((instanceId: number) => {
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === sessionRef.current?.pluginId)?.handle;
    if (!plugin?.readHistory) return;
    const orderAtRequest = historyOrderRef.current;
    void plugin.readHistory(instanceId).then((snapshot) => {
      if (historyOrderRef.current > orderAtRequest) {
        console.warn("[DEBUG] history snapshot skipped — projection newer than request", JSON.stringify({ orderAtRequest, current: historyOrderRef.current }));
        return;
      }
      console.warn("[DEBUG] history snapshot refresh", JSON.stringify({ instanceId, cursor: snapshot.cursor, upserts: snapshot.upserts?.length ?? 0, canUndo: snapshot.canUndo ?? null }));
      applyHistoryPatch(snapshot, true);
    }).catch((error) => console.error("[DEBUG] history snapshot failed", error));
  }, [applyHistoryPatch]);
  const hostPlugin = useMemo(() => (hostConfig ? loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId) : undefined), [loadedPlugins, hostConfig]);
  const hostAppsResolution = useMemo(() => {
    if (!hostPlugin || !hostConfig) return { apps: undefined, error: null };
    try {
      return { apps: resolveRequiredHostApps(hostPlugin.manifest.apps, hostConfig), error: null };
    } catch (resolutionError) {
      return { apps: undefined, error: resolutionError instanceof Error ? resolutionError.message : String(resolutionError) };
    }
  }, [hostPlugin, hostConfig]);
  const hostApp = hostAppsResolution.apps?.host;
  const landingApp = hostAppsResolution.apps?.landing;
  const landingAppId = landingApp?.id;
  const hostAppId = hostApp?.id;
  const hostControllerId = hostApp?.controllerId;
  const landingControllerId = landingApp?.controllerId;
  const hostCatalogueTabId = hostApp ? requiredHostPanelLeafId(hostApp) : undefined;
  const spacePrograms = useMemo<readonly SpaceProgramEntry[]>(
    () =>
      loadedPlugins.flatMap((entry) =>
        entry.manifest.apps.map((app) => ({
          pluginId: entry.handle.pluginId,
          workflowStepId: app.id,
          appId: app.id,
          label: resolveManifestLabel(app.label as LocalizedLabel | string, uiTerminology, uiLocale),
          breadcrumb: app.breadcrumb,
          yields: "",
        })),
      ),
    [loadedPlugins, uiLocale, uiTerminology],
  );
  useEffect(() => {
    if (!hostAppsResolution.error) return;
    dispatch({ type: "SET_SESSION", value: null });
    dispatch({ type: "SET_ERROR", value: hostAppsResolution.error });
  }, [hostAppsResolution.error]);
  useEffect(() => {
    if (!session) return;
    // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-A — reads `loadedPluginsRef`
    // (kept in sync every render, line ~1124), NOT the `loadedPlugins` array itself: this effect only
    // needs to look up `session.pluginId`'s already-loaded handle, never to refire because some
    // UNRELATED plugin finished loading in the background. Depending on `loadedPlugins` directly made
    // this effect refire on every one of the ~50+ sequential catalogue plugin loads during boot,
    // dispatching a fresh `readHistory` exchange call each time for the SAME session/instance — a real
    // contributor to the `readHistory: missing HistorySnapshot frame` / `plugin instance busy` storm
    // observed live in `🧪️5-a-collab-e2e-run1.txt` (lines 29478-29708, correlating almost 1:1 with
    // `plugin worker + <name>` catalogue-load lines in the same window).
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    let cancelled = false;
    const orderAtRequest = historyOrderRef.current;
    void plugin.readHistory(session.instanceId).then((snapshot) => {
      if (cancelled) return;
      if (historyOrderRef.current > orderAtRequest) {
        console.warn("[DEBUG] history snapshot skipped — projection newer than request", JSON.stringify({ orderAtRequest, current: historyOrderRef.current }));
        return;
      }
      applyHistoryPatch(snapshot, true);
    }).catch((error) => console.error("[DEBUG] history snapshot failed", error));
    return () => {
      cancelled = true;
    };
    // 🧾️ Keyed on the plugin INSTANCE, not the `session` object: a full history snapshot is only ever
    // owed when the instance being shown changes. Depending on the whole session re-read the projection
    // on every session mint — see `applyHostEffects`'s `viewStateRewrittenByEffects` for the churn that
    // used to produce (one `readHistory` guest round trip per dispatch).
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [applyHistoryPatch, session?.pluginId, session?.instanceId]);
  const { windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId, toolMeasuresByToolId, panelUiByKey, appLabelsOverlay, appCatalogue } = shellState.windowUi;
  const { spawnedWindowUi, spawnedWindowFault, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;
  const { foldedByWindowId: actionPaneFoldedByWindowId, expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId, activeToolId } = shellState.actionPane;
  const { expandedCommandId, stagedArgsByCommandId: commandStagedArgsByCommandId } = shellState.commandPanel;
  const { panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId, shellLayout, activeExampleId, mobilePanelPath, mobilePanelVisible, extraWindowInstances, windowTitlesById, windowIconsById } = shellState.layout;
  const { searchOpen, findOpen, introductionStepIndex, introductionCompletedInteractions, dialog: overlayDialog, transientNotice, openWithFocusRole } = shellState.overlays;
  const dialogOpeningRef = useRef(0);
  const liveDialogRef = useRef(overlayDialog);
  liveDialogRef.current = overlayDialog;
  const { activeTutorialId, playing: tutorialPlaying, rate: tutorialRate, muted: tutorialMuted, captionsOn: tutorialCaptionsOn, recording: tutorialRecording, deviated: tutorialDeviated } = shellState.tutorial;
  useEffect(
    () =>
      subscribeUiPreferences(scope.storage, (preferences) => {
        const resolved = resolveUiPreferences(preferences, {
          appearance: "system",
          layout: "desktop",
          driverId: DEFAULT_UI_DRIVER.id,
          locale: "en",
          terminology: UI_TERMINOLOGY_NATIVE,
          themeId: "semio",
        });
        dispatch({ type: "SET_UI_APPEARANCE", value: locks.appearance ?? resolved.appearance });
        dispatch({ type: "SET_UI_LAYOUT", value: resolved.layout });
        dispatch({ type: "SET_UI_DRIVER_ID", value: resolved.driverId });
        dispatch({ type: "SET_UI_CUSTOM_DRIVERS", value: resolved.customDrivers });
        dispatch({ type: "SET_UI_LOCALE", value: locks.locale ?? resolved.locale });
        dispatch({ type: "SET_UI_TERMINOLOGY", value: locks.terminology ?? resolved.terminology });
        dispatch({ type: "SET_UI_THEME_ID", value: locks.themeId ?? resolved.themeId });
        dispatch({ type: "SET_UI_CUSTOM_THEMES", value: resolved.customThemes });
        dispatch({ type: "SET_UI_KEYBINDING_OVERRIDES", value: resolved.keybindingOverrides });
      }),
    [locks.appearance, locks.locale, locks.terminology, locks.themeId, scope.storage],
  );
  const { syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId } = shellState.sync;
  const { mergePolicy, conflicts, selectedConflictId } = shellState.merge;
  /** 💡️ The one document whose host-owned inference port is currently live, and its exact status. */
  const inferencePortRuntimeKey = shellState.inference.operationRuntimeKey;
  const inferencePort = inferencePortRuntimeKey === null ? undefined : shellState.inference.portByRuntimeKey[inferencePortRuntimeKey];
  const importSpaceInputRef = useRef<HTMLInputElement>(null);
  const refreshGenerationRef = useRef(0);
  const replaceBodiesGenerationRef = useRef(0);
  const clipboardFragmentRef = useRef<unknown>(undefined);
  const appRegistrationsJsonRef = useRef<string | null>(null);
  const spawnedRefreshGenerationRef = useRef(0);
  const contributorInstancesRef = useRef<Map<string, number>>(new Map());
  const layoutSeedKeyRef = useRef<string | null>(null);
  const noExampleResetInstanceIdRef = useRef<number | null>(null);
  const extraWindowCounterRef = useRef(0);
  // 🖱️ Shell-level context-menu fallback: opens for any right-click the shell hasn't already claimed
  // (every existing per-surface `onContextMenu` now calls `stopPropagation()` once it decides to show
  // its own menu — see the `🖱️ShellContextMenu` region below). Covers window-level declared actions
  // plus the OS command palette, so every window/background always shows *something*.
  const [shellContextMenu, setShellContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly ContextMenuItem[] } | null>(null);
  // 🪟️ Live extra-window list, updated synchronously on every seed/split/drop — `refreshUi` reads this
  // instead of the render-closure `extraWindowInstances` so a concurrent action refresh (e.g. boot
  // `setActiveExample`) that starts after the session-switch refresh wrote extras but before React
  // re-rendered cannot fetch with `[]` and wipe Top/Perspective bodies to "missing window".
  const extraWindowInstancesRef = useRef<readonly ExtraWindowInstance[]>([]);
  extraWindowInstancesRef.current = extraWindowInstances;
  //#region 🏁️OperationSettle
  /** 🏁️ Waiters for typed operations whose terminal completion has not arrived yet, keyed by operation
   * id, plus the ids that completed BEFORE anyone asked to wait (the admitting `handleAction` reply and
   * the `OperationCompleted` frame race — the completion can land first). Both are fixed-capacity: a
   * waiter that would exceed the ceiling resolves at once rather than growing the map, and the settled
   * ring keeps only the newest ids. Nothing here is state — a settle is not something to re-render for. */
  const operationSettlersRef = useRef(new Map<number, () => void>());
  const settledOperationsRef = useRef<number[]>([]);
  /** 🏁️ Resolves every waiter for `operation`, or records it as already settled for a waiter that has
   * not registered yet. Called from the operation-completion subscription. */
  const settleOperation = useCallback((operation: number) => {
    const settler = operationSettlersRef.current.get(operation);
    if (settler) {
      settler();
      return;
    }
    const settled = settledOperationsRef.current;
    settled.push(operation);
    if (settled.length > OPERATION_SETTLE_RING_SLOTS) settled.splice(0, settled.length - OPERATION_SETTLE_RING_SLOTS);
  }, []);
  /** 🏁️ The promise `onAction` returns: settles when the operation the admitting reply started reaches
   * its terminal completion. An action that started no operation (a framework-reserved verb, a
   * config-only emit, a rejected admission) settles at once, and a watchdog bounds a completion that
   * never arrives so a caller can never wedge forever on a lost frame. */
  const awaitOperationSettle = useCallback((output: unknown): Promise<void> => {
    const operation = startedTypedOperationId(output);
    if (operation === undefined) return Promise.resolve();
    const settled = settledOperationsRef.current;
    const alreadySettled = settled.indexOf(operation);
    if (alreadySettled !== -1) {
      settled.splice(alreadySettled, 1);
      return Promise.resolve();
    }
    const settlers = operationSettlersRef.current;
    if (settlers.has(operation) || settlers.size >= OPERATION_SETTLE_WAITER_SLOTS) return Promise.resolve();
    return new Promise<void>((resolve) => {
      const timer = setTimeout(() => {
        operationSettlersRef.current.delete(operation);
        resolve();
      }, OPERATION_SETTLE_WATCHDOG_MS);
      settlers.set(operation, () => {
        clearTimeout(timer);
        operationSettlersRef.current.delete(operation);
        resolve();
      });
    });
  }, []);
  //#endregion 🏁️OperationSettle
  //#region 🔀️SurfaceSwitch
  /** ⏳️ Guest-bound work outstanding per `(pluginId, instanceId)` — every `handleAction` round trip and
   * every brokered `invokeExtension` registers here for its whole lifetime, and `switchToPluginApp`'s
   * `quiesce` port reads it. This is the ONE fact a transactional switch needs and the shell did not
   * have: `operationSettlersRef` only knows about operations somebody is awaiting, and nothing at all
   * tracked an extension round trip. */
  const sessionWorkRef = useRef(createSessionWorkLedgerV1());
  /** 🪦️ Instances a switch has sealed. Read by every path that would otherwise address a retired
   * instance, so a late action/effect resolves to ONE {@link sealedInstanceDropTextV1} line instead of
   * `no actor for instance N` plus a stack. */
  const sealedInstancesRef = useRef(createSealedInstanceLedgerV1());
  const sessionSwitchGateRef = useRef(createSessionAppSwitchGateV1<AppDefinition, ViewModel>());
  const [surfaceSwitchBusy, setSurfaceSwitchBusy] = useState(false);
  /** 🔇️ `true` when this session is sealed — the drop is logged once per call site, never thrown. */
  const dropForSealedInstance = useCallback((target: { readonly pluginId: string; readonly instanceId: number }, what: string, detail?: string): boolean => {
    if (!sealedInstancesRef.current.sealed(target.pluginId, target.instanceId)) return false;
    console.warn(sealedInstanceDropTextV1(sealedInstanceDropV1(target.pluginId, target.instanceId, what, detail)));
    return true;
  }, []);
  //#endregion 🔀️SurfaceSwitch
  const setWindowTitle = useCallback((windowId: string, title: string) => {
    dispatch({ type: "SET_WINDOW_TITLE", windowId, title });
  }, []);
  const setWindowIcon = useCallback((windowId: string, iconId: IconName) => {
    dispatch({ type: "SET_WINDOW_ICON", windowId, iconId });
  }, []);
  // 🐢️ Per-instance content-hash cache for the batched `refresh-ui` call, keyed by the same
  // `pluginId:appId:instanceId` triple as `layoutSeedKeyRef` — cleared on session switch below.
  const uiRefreshCacheRef = useRef<UiRefreshCache>(new Map());
  // 🐢️ Same idea for the studio-mode spawned-instance view, keyed by spawned instanceId — cleared when
  // the spawned instance itself changes (tracked via `spawnedLayoutSeedRef`).
  const spawnedUiRefreshCacheRef = useRef<UiRefreshCache>(new Map());
  const spawnedLayoutSeedRef = useRef<string | null>(null);
  const openSpaceIdRef = useRef<string | null>(null);
  const openInstanceIdRef = useRef<string | null>(null);
  const sessionRef = useRef<ActiveSession | null>(null);
  /** 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-A — diagnostic + defensive
   * reentrancy guard for `applyShellUri`. Observed live (`🧪️5-a-collab-e2e-run1.txt`/`run3.txt`):
   * `[DEBUG] shell uri apply failed Error: Maximum call stack size exceeded`, immediately followed by a
   * `plugin instance busy` storm on the SAME session's `attachBackbone`/`refreshUi` — i.e. something
   * re-enters `applyShellUri` synchronously before an earlier in-flight call returns, deep enough to
   * overflow the JS stack, leaving the plugin's guest-side `InstanceGuard` stuck (a reentrant exchange
   * call never completes its `Drop`). Still present after lane 4-I's `spaceIndexAlreadyOpen` idempotency
   * fix, so this is a DIFFERENT reentrant path than the one that fix closed. Not yet root-caused to a
   * single call site — the guard below turns the crash into a bounded, logged no-op and captures a full
   * stack the next time this fires, instead of shipping another blind guess.
   */
  const applyShellUriDepthRef = useRef(0);
  //#region 🔖️BuiltNodeStores
  /** 🧬️ Per-window/panel `📃️UiDocumentStore`s, keyed the same way `windowUiByWindowId`/`panelUiByKey`/
   * `spawnedWindowUi` already are — one stable store instance per key, reloaded via
   * `loadSnapshot`/`builtNodeToSnapshot` whenever that key's authored `BuiltNode` reference actually
   * changes (the reducer's own `mergeRecordPreservingIdentity` already keeps an unchanged window's
   * `BuiltNode` reference-stable across a `refreshUi` cycle, so this rarely reloads). Plain ref-backed
   * memoization, not a custom hook — the call sites live inside `useMemo` callbacks over `.map()`,
   * where a hook call would violate the Rules of Hooks. */
  const builtNodeStoreCacheRef = useRef(createBuiltNodeStoreCacheV1());
  const builtNodeStoreFor = useCallback((key: string, node: BuiltNode): UiDocumentStore => builtNodeStoreCacheRef.current.storeFor(key, node), []);
  // 🎬️ Drains the reloads `storeFor` deferred out of this render — a layout effect, so the store is
  // current before the browser paints, and the `UiNodeView` subscribers it notifies are updated from a
  // committed tree instead of from inside `FrameworkOsShellInner`'s render.
  useLayoutEffect(() => builtNodeStoreCacheRef.current.flushPendingReloads());
  //#endregion 🔖️BuiltNodeStores
  const uiDevice: ElementsSurfaceDevice = mobile ? "mobile" : uiLayout;
  const uiTheme: UiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft]);
  const uiDriver: UiDriver = useMemo(() => uiDriverDraft ?? resolveUiDriver(uiDriverId, uiCustomDrivers), [uiDriverId, uiCustomDrivers, uiDriverDraft]);
  /** 🧵️ Lazily-created worker running `🟦️backbone-🟦️worker.ts` — one per shell instance, reused across `openDocument` calls. */
  const backboneWorkerRef = useRef<Worker | null>(null);
  /** 🪪️ Per-tab session id component of the actor (contract §C0 `user:{userId}#{shellSessionId}`) —
   * stable for this tab's whole lifetime, shared by both the pre-identity `client-{id}` actor and the
   * post-sign-in `user:{userId}#{id}` one, so a tab's actor id only ever changes its PREFIX on sign-in,
   * never re-mints the suffix mid-session. */
  const shellSessionIdRef = useRef<string>(Math.random().toString(36).slice(2));
  /** 🖋️ Stable per-tab actor id for hub `Hello`/presence frames and operation-origin filtering. */
  const shellActorIdRef = useRef<string>(`client-${shellSessionIdRef.current}`);
  // 🤖️ Agent bridge: stays `disabled` (never throws, never blocks render) until a gateway URL+token
  // is discovered, so a shell with no agent attached behaves exactly as before.
  const agentBridge = useAgentBridge({ shellSessionId: shellSessionIdRef.current });
  /** 🪪️ §C3 identity bootstrap — `null` until `DirectoryClient.me()`/`mintSession` resolves (or forever,
   * with no hub env). Mirrored into {@link shellActorIdRef}/`setPluginRuntimeActor` by the effect below,
   * never read directly for the actor id (that's always `shellActorIdRef.current`). */
  const [identity, setIdentity] = useState<Identity | null>(null);
  const identityRef = useRef<Identity | null>(null);
  identityRef.current = identity;
  /** 🪪️ True once a hub env is configured but the hub could not be reached (§C3 "keep the last
   * persisted identity, show an offline chip... never blocks the UI thread"). */
  const [identityOffline, setIdentityOffline] = useState(false);
  const [verifiedSessionAuthority, setVerifiedSessionAuthority] = useState<DirectorySessionAuthorityV1 | null>(null);
  const verifiedSessionAuthorityRef = useRef<DirectorySessionAuthorityV1 | null>(null);
  const sessionRefreshRef = useRef<DirectorySessionRefreshV1 | null>(null);
  /** 🪪️ REST-only client for the identity boot handshake (`me`/`mintSession`) — distinct from the
   * directory-lane's persistent `/directory/socket/v1` subscription, which the shell never opens itself (§C6:
   * `🏪️store/👷️worker/🟦️.ts`'s `🔖️Directory` region is the only socket owner). Re-created only if the
   * hub base url or token actually changes. */
  const localBrowserBrokerRef = useRef<BrowserBrokerPortClientV1 | null>(null);
  const hubEnv = useMemo(() => {
    const hubBaseUrl = readViteSEnv("VITE_S_HUB_URL");
    const dataDir = readViteSEnv("VITE_S_DATA_DIR");
    return hubBaseUrl ? { hubBaseUrl, dataDir } : null;
  }, []);
  /** 📇️ Retained directory owner bound to the same visible Home landing instance. */
  const directoryHomeOwnerRef = useRef<DirectoryHomeOwnerV1 | null>(null);
  const directoryHomeRetirementRef = useRef<{ readonly owner: DirectoryHomeOwnerV1; readonly promise: Promise<void> } | null>(null);
  const directoryHomeOpeningRef = useRef<Promise<void>>(Promise.resolve());
  const directoryBootstrapEpochRef = useRef(0);
  const [directoryBootstrapUi, setDirectoryBootstrapUi] = useState<DirectoryBootstrapUiState>({ kind: "idle" });
  const refreshDirectoryHomeRef = useRef<(session: ActiveSession) => Promise<void>>(async () => {});
  const handleDirectoryEventPageRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "directory-event-page" }>) => void>(() => {});
  const handleDirectoryBootstrapFailureRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "directory-bootstrap-failed" }>) => void>(() => {});
  const uiLocaleRef = useRef(uiLocale);
  const uiTerminologyRef = useRef(uiTerminology);
  uiLocaleRef.current = uiLocale;
  uiTerminologyRef.current = uiTerminology;
  const retireDirectoryHomeOwner = useCallback((owner: DirectoryHomeOwnerV1, worker: Worker | null): Promise<void> => {
    const current = directoryHomeRetirementRef.current;
    if (current?.owner === owner) return current.promise;
    const promise = closeDirectoryHomeOwnerV1(owner, (request) => worker?.postMessage({ wire: encodeBackboneWorkerRequest(request) }));
    directoryHomeRetirementRef.current = { owner, promise };
    void promise.finally(() => {
      if (directoryHomeRetirementRef.current?.owner === owner) directoryHomeRetirementRef.current = null;
    });
    return promise;
  }, []);
  /** 📇️ Set by the `foldDirectoryEvents` region below once it's defined — `ensureBackboneWorker`'s
   * `onmessage` (created once, `useCallback([])`) reads this indirection rather than the callback
   * itself so a fold that depends on `session`/`hostConfig`/`onActionRef` never goes stale. */
  const dispatchDirectoryEventsRef = useRef<(events: readonly DirectoryEvent[]) => void>(() => {});
  /** ⚖️ Same ref-forwarding idiom as {@link dispatchDirectoryEventsRef} — `ensureBackboneWorker`'s
   * `remoteMutations` handling (below) needs `showTransientNotice`/`shellLabel`, both declared LATER
   * in this component (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
   * CONFLICTS` §C6/§C9: a peer's `MergeReport`/`Conflicts` reply to `ApplyEnvelopes` must reach the
   * Conflicts panel and, for a `"degraded"` outcome, a transient notice). Set once `applyRemoteMerge`
   * itself is defined, read only from inside the worker's `onmessage` closure body. */
  const applyRemoteMergeRef = useRef<(conflicts: readonly Conflict[] | null, mergeReport: MergeReport | null) => void>(() => {});
  /** 🧯️ Same ref-forwarding idiom as {@link applyRemoteMergeRef} — `applyHostEffects`'s `notify`
   * branch (declared ABOVE `showTransientNotice`) is the shell end of `kernel::Effect::Notify`, the
   * one channel a plugin has for telling the user its work was refused (e.g. puzzle3d's
   * `addBrushObject`/`acceptSuggestion` rejecting a colliding placement). Without this branch the
   * effect reached `wireEffectToFriendly` and was dropped, so a refusal looked exactly like nothing
   * happening. Assigned right after `showTransientNotice` itself is defined. */
  const showTransientNoticeRef = useRef<(message: string, kind?: Severity, code?: string) => void>(() => {});
  /** 📇️ `applyHostEffects`'s new `replayShellCommand` branch (below) needs `openDocument`/
   * `openArtifactWithAppRef`, both declared LATER in this same component (after `applyHostEffects`
   * itself) — a direct reference in `applyHostEffects`'s own dependency array would be a `const`
   * temporal-dead-zone violation at the point `useCallback` evaluates that array. Same ref-forwarding
   * idiom `onActionRef`/`dispatchDirectoryEventsRef` already use: assigned as a plain statement right
   * after each real declaration, read only from inside a later callback body, never from a deps array. */
  type OpenDocumentSessionTarget = DocumentOpeningTarget<ActiveSession, PluginWasmHandle> & { readonly background?: boolean; readonly expectedCatalogGenerationId?: string };
  type PreparedArtifactOpeningTarget = OpenDocumentSessionTarget & Readonly<{ dialect: ArtifactDialect; role: AppRole }>;
  type OpenDocumentSession = {
    session: ActiveSession;
    plugin: PluginWasmHandle;
    documentId: string;
    clientInstanceId: string;
    scope?: DocumentScope;
    port: ActorDocumentMessagePortV1 | null;
    pending: Uint8Array[];
    pendingBytes: number;
    replacements: LatestDocumentReplacementV1<DocumentArchivePack>;
    archivePersistence: () => Promise<void>;
    creationMount: ArtifactCreationCatalogMountV1 | null;
    ready: Promise<void>;
    resolveReady(): void;
    rejectReady(error: Error): void;
  };
  type BackgroundSpaceIndexSession = {
    readonly plugin: PluginWasmHandle;
    readonly session: ActiveSession;
    readonly runtimeKey: string;
    readonly clientInstanceId: string;
    readonly hubBaseUrl: string;
    readonly userId: string;
  };
  const backgroundSpaceIndexSessionsRef = useRef(new BackgroundDocumentSessionsV1<BackgroundSpaceIndexSession>());
  const openDocumentRef = useRef<(ref: DocumentOpeningReference, bindings?: readonly PersistenceBinding[], target?: OpenDocumentSessionTarget) => Promise<DocumentOpeningReceiptV1 | null>>(async () => null);
  const closeDocumentRef = useRef<(runtimeKey: string, clientInstanceId?: string) => void>(() => {});
  const documentAttachmentLanesRef = useRef(new WeakMap<PluginWasmHandle, Map<number, DocumentAttachmentLaneV1>>());
  const documentPortsRef = useRef(new WeakMap<PluginWasmHandle, Map<number, ActorDocumentMessagePortV1>>());
  const documentAttachmentLane = useCallback((plugin: PluginWasmHandle, instanceId: number): DocumentAttachmentLaneV1 => {
    let lanes = documentAttachmentLanesRef.current.get(plugin);
    if (lanes === undefined) { lanes = new Map(); documentAttachmentLanesRef.current.set(plugin, lanes); }
    let lane = lanes.get(instanceId);
    if (lane === undefined) {
      lane = new DocumentAttachmentLaneV1(async () => {
        const ports = documentPortsRef.current.get(plugin);
        const port = ports?.get(instanceId);
        await port?.retire();
        if (port !== undefined && ports?.get(instanceId) === port) ports.delete(instanceId);
      });
      lanes.set(instanceId, lane);
    }
    return lane;
  }, []);
  const retireDocumentAttachment = useCallback(async (plugin: PluginWasmHandle, instanceId: number, clientInstanceId: string): Promise<void> => {
    const lanes = documentAttachmentLanesRef.current.get(plugin);
    const lane = lanes?.get(instanceId);
    if (lane === undefined) return;
    await lane.close(clientInstanceId);
    if (lane.idle && lanes?.get(instanceId) === lane) lanes.delete(instanceId);
  }, []);
  const openArtifactWithAppRefRef = useRef<(target: AppRef, dialect: ArtifactDialect, role: AppRole, admit?: () => boolean, publish?: boolean) => Promise<PreparedArtifactOpeningTarget | null>>(async () => null);
  const resolveArtifactOpeningRelayRef = useRef<(actionId: string, args: unknown) => ResolvedArtifactOpeningRelay | null>(() => null);
  const openReadySpaceArtifactCreationRef = useRef<(requestId: string) => void>(() => {});
  const spaceArtifactCreationOwnersRef = useRef(new Map<string, SpaceArtifactCreationOwnerV1>());
  const [spaceArtifactCreationUi, setSpaceArtifactCreationUi] = useState<ArtifactCreationProgressUiStateV1>({});
  const [spaceArtifactCreationCatalog, setSpaceArtifactCreationCatalog] = useState<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> | null>(null);
  const [spaceArtifactCreationCatalogUi, setSpaceArtifactCreationCatalogUi] = useState<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> | null>(null);
  const spaceArtifactCreationCatalogRef = useRef(spaceArtifactCreationCatalog);
  const spaceArtifactCreationCatalogUiRef = useRef(spaceArtifactCreationCatalogUi);
  spaceArtifactCreationCatalogRef.current = spaceArtifactCreationCatalog;
  spaceArtifactCreationCatalogUiRef.current = spaceArtifactCreationCatalogUi;
  const cancelSpaceArtifactCreationsForRuntime = useCallback((runtimeKey: string, worker: Worker | null) => {
    const cleared: string[] = [];
    for (const [requestId, owner] of spaceArtifactCreationOwnersRef.current) {
      if (owner.runtimeKey !== runtimeKey) continue;
      spaceArtifactCreationOwnersRef.current.delete(requestId);
      cleared.push(requestId);
      if (owner.ready === null) worker?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: owner.spaceId }) });
    }
    if (cleared.length > 0) setSpaceArtifactCreationUi((current) => cleared.reduce((next, requestId) => reduceArtifactCreationProgressUiV1(next, { kind: "cleared", requestId }), current));
  }, []);
  const cancelSpaceArtifactCreation = useCallback((requestId: string, spaceId: string) => {
    const owner = spaceArtifactCreationOwnersRef.current.get(requestId);
    if (owner === undefined || owner.spaceId !== spaceId || owner.cancelRequested || owner.opening || owner.ready !== null) return;
    spaceArtifactCreationOwnersRef.current.set(requestId, { ...owner, cancelRequested: true });
    setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "cancel-requested", requestId, spaceId }));
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId }) });
  }, []);
  /** 📇️ Offline-queue depth surfaced by the worker's `directory-status` — not yet rendered by any
   * chrome this lane owns (2-F/3-A's "row shows 'pending'" territory); kept as local state so a
   * consumer can read it once that chrome lands, with zero further plumbing here. */
  const [, setDirectoryPendingCommands] = useState(0);
  /** 🧾️ Retained, request-id-keyed command result slot. A receipt lands here BEFORE any accepted
   * event is folded, so a completion always has an owner; a one-shot invite capability lives only
   * in this slot until an explicit administration copy handler consumes it, and is never logged,
   * folded into Home, or put into React state telemetry. */
  const directoryCommandResultsRef = useRef(new Map<string, DirectoryCommandResultSlotV1>());
  /** 🏛️ The one Shell-owned administration pane state. It is filled ONLY from the worker's retained
   * `DirectoryAdministrationOperation`: the exact canonical page bytes the hub sealed, the phase,
   * and whether a one-shot invite capability is still held by the worker. No role, session identity,
   * bearer, cursor key, or invite token is ever stored here — a terminal phase arrives with the page
   * already erased, so an unmount, identity change, 401/403 or scoped 4401 clears the pane. */
  const [spaceAdministration, setSpaceAdministration] = useState<ShellSpaceAdministrationStateV1 | null>(null);
  const spaceAdministrationRef = useRef<ShellSpaceAdministrationStateV1 | null>(null);
  spaceAdministrationRef.current = spaceAdministration;
  const spaceAdministrationEpochRef = useRef(0);
  const spaceAdministrationMessageRef = useRef(0);
  /** 💡️ Monotonic owner of the one worker-side inference port; a stale epoch's status is ignored. */
  const inferencePortEpochRef = useRef(0);
  /** 🗂️ Exact scope and runtime owner for the current inference epoch. */
  const inferencePortOwnerRef = useRef<InferencePortOwnerV1 | null>(null);
  const inferencePortAuthorityRef = useRef<DirectorySessionAuthorityV1 | null>(null);
  const retiredSessionDocumentOwnersRef = useRef(new WeakSet<object>());
  const inferencePortOpeningRef = useRef<{ readonly owner: InferencePortOwnerV1; readonly clientInstanceId: string; readonly sessionInstanceId: number; readonly mailbox: InferencePortOpeningMailboxV1 } | null>(null);
  type MountedInferenceHistoryV1 = Readonly<{ historyEpoch: number; clientInstanceId: string; scope: DocumentScope; sessionInstanceId: number; status: GisMapApprovalHistoryStatusV1; order: number; authority: DirectorySessionAuthorityV1 }>;
  const [inferenceHistoryByRuntimeKey, setInferenceHistoryByRuntimeKey] = useState<Readonly<Record<string, MountedInferenceHistoryV1>>>({});
  const inferenceHistoryByRuntimeKeyRef = useRef<Readonly<Record<string, MountedInferenceHistoryV1>>>({});
  inferenceHistoryByRuntimeKeyRef.current = inferenceHistoryByRuntimeKey;
  /** 🪪️ Settled once by the identity bootstrap effect's very first `documentArchiveReplaced` for
   * `IDENTITY_CONFIG_SCHEMA` (a previously-persisted session) — see that effect for the bounded
   * timeout that resolves it to `null` when no such file exists (never blocks the UI thread). */
  const identitySnapshotResolverRef = useRef<((value: Identity | null) => void) | null>(null);
  const identityClientInstanceIdRef = useRef<string | null>(null);
  const identityBootstrapAbortRef = useRef<AbortController | null>(null);
  const presenceConnectedAtMsRef = useRef(Date.now());
  const presenceCursorRef = useRef<{ readonly x: number; readonly y: number } | undefined>(undefined);
  /** 🐚️ terra-web-shellhost (finding 5) — per-document `latestWins` triggers for the presence-beat
   * effect below: keyed lazily on first beat so each document gets its own single-flight-with-trailing-
   * coalesce wrapper instead of sharing one across every open document, which would serialize them.
   * Cleared on that effect's own cleanup (identity/ephemeral change or unmount). */
  const presenceBeatTriggersRef = useRef<Map<string, () => Promise<void>>>(new Map());
  /** 🌐️ terra-web-shellhost (finding 4) — one `AbortController` for the whole component's lifetime,
   * aborted from the unmount-teardown effect below (never recreated: this file's own `presenceConnectedAtMsRef`
   * a few lines up is the same "construct once, keep for the component's life" idiom). Ties every
   * extension install fetch (`installExtension`/`installExtensionFromFile`/`uninstallExtension`) to
   * the component's own lifetime instead of leaving them free-running past unmount. */
  const extensionFetchAbortRef = useRef(new AbortController());
  /** 🧵 Cancels every active operation-owned download drain when this shell unmounts. */
  const segmentedDownloadAbortRef = useRef(new AbortController());
  /** 🗂️ Which session/plugin owns each exact document runtime, so equal document ids in different
   * spaces cannot share socket, bootstrap, presence or plugin-routing state. */
  const openDocumentSessionsRef = useRef<Map<string, OpenDocumentSession>>(new Map());
  const failDocumentBackbone = useCallback((runtimeKey: string, entry: OpenDocumentSession, failure: unknown) => {
    if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
    const error = failure instanceof Error ? failure : new Error(String(failure));
    if (entry.replacements.pending) {
      console.error("[DEBUG] document backbone failed during replacement — keeping owner", error);
      return;
    }
    entry.creationMount?.close(error);
    entry.rejectReady(error);
    console.error("[DEBUG] document backbone failed", error);
    closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
  }, []);
  const receiveDocumentBackbone = useCallback((runtimeKey: string, entry: OpenDocumentSession, message: Uint8Array) => {
    if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
    try {
      if (!(message instanceof Uint8Array) || message.length === 0 || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || decodeBackboneMessage(message).kind !== "mutations") throw new Error("document-backbone.invalid-message");
      if (entry.port === null) {
        if (entry.pending.length >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || message.length > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - entry.pendingBytes) {
          console.warn("[DEBUG] document backbone pending overflow — dropping while rebound", JSON.stringify({ runtimeKey, clientInstanceId: entry.clientInstanceId, pending: entry.pending.length }));
          return;
        }
        entry.pending.push(message.slice());
        entry.pendingBytes += message.length;
      } else void entry.port.receive({ runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null }, message).catch(error => failDocumentBackbone(runtimeKey, entry, error));
    } catch (error) { failDocumentBackbone(runtimeKey, entry, error); }
  }, [failDocumentBackbone]);
  const bindDocumentBackbone = useCallback(async (runtimeKey: string, entry: OpenDocumentSession, admitted: () => boolean = () => true): Promise<void> => {
    const current = () => admitted() && openDocumentSessionsRef.current.get(runtimeKey) === entry;
    if (!current()) return;
    if (!entry.plugin.bindDocumentPort) throw new Error("document-backbone.binding-unavailable");
    const worker = backboneWorkerRef.current;
    if (worker === null) throw new Error("document-backbone.worker-unavailable");
    const preparation: { port?: ActorDocumentMessagePortV1 } = {};
    try { await entry.plugin.bindDocumentPort(entry.session.instanceId, {
      runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null, current,
      send: message => {
        if (!current() || backboneWorkerRef.current !== worker) return;
        if (documentBackboneEffectV1(message) === "remote-ingest-receipt") return;
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "documentBackbone", message } }) });
        void entry.archivePersistence().catch(error => failDocumentBackbone(runtimeKey, entry, error));
      },
      merge: (conflicts, report) => { if (current()) applyRemoteMergeRef.current(conflicts, report); },
      prepared: port => {
        preparation.port = port;
        entry.port = port;
        let ports = documentPortsRef.current.get(entry.plugin);
        if (ports === undefined) { ports = new Map(); documentPortsRef.current.set(entry.plugin, ports); }
        ports.set(entry.session.instanceId, port);
        const pending = entry.pending;
        entry.pending = [];
        entry.pendingBytes = 0;
        for (const bytes of pending) receiveDocumentBackbone(runtimeKey, entry, bytes);
      },
    }); } catch (error) {
      const candidate = preparation.port;
      await candidate?.retire();
      if (candidate !== undefined) {
        const ports = documentPortsRef.current.get(entry.plugin);
        if (ports?.get(entry.session.instanceId) === candidate) ports.delete(entry.session.instanceId);
        if (entry.port === candidate) {
          entry.port = null;
          entry.pending = [];
          entry.pendingBytes = 0;
        }
      }
      throw error;
    }
    if (current()) entry.resolveReady();
  }, [receiveDocumentBackbone]);
  const loadDocumentArchive = useCallback(async (plugin: PluginWasmHandle, instanceId: number, archive: DocumentArchivePack, current: () => boolean): Promise<boolean> => {
    const archiveBytes = encodeDocumentArchiveBytes(archive);
    if (archiveBytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document-backbone.archive-capacity");
    if (!plugin.loadAppDocumentArchive) throw new Error("document-backbone.archive-loader-unavailable");
    const load = plugin.loadAppDocumentArchive;
    const owned = [...openDocumentSessionsRef.current].find(([, entry]) => entry.plugin === plugin && entry.session.instanceId === instanceId);
    const lane = documentAttachmentLane(plugin, instanceId);
    if (owned === undefined) {
      const owner = `cold:${crypto.randomUUID()}`;
      const unbound = () => current() && ![...openDocumentSessionsRef.current.values()].some(entry => entry.plugin === plugin && entry.session.instanceId === instanceId);
      let loaded = false;
      try { await lane.replace(owner, unbound, async () => { if (unbound()) { await load(instanceId, archive); loaded = unbound(); } }); }
      finally { await lane.close(owner); }
      return loaded;
    }
    const [runtimeKey, entry] = owned;
    const retirement = entry.port?.retire();
    void retirement?.catch(() => {});
    entry.port = null;
    return entry.replacements.replace(archive, async (candidate, latest) => {
      const exact = () => latest() && current() && openDocumentSessionsRef.current.get(runtimeKey) === entry;
      await lane.replace(entry.clientInstanceId, exact, async () => {
        await retirement;
        if (!exact()) return;
        await load(instanceId, candidate);
        if (exact()) await bindDocumentBackbone(runtimeKey, entry, latest);
      });
    });
  }, [bindDocumentBackbone, documentAttachmentLane]);
  const loadDocumentPair = useCallback(async (plugin: PluginWasmHandle, instanceId: number, pack: Uint8Array, spr: Uint8Array, current: () => boolean): Promise<boolean> =>
    loadDocumentArchive(plugin, instanceId, { parent_pack: Array.from(pack), parent_spr: Array.from(spr), members: [] }, current), [loadDocumentArchive]);
  const captureDialogOrigin = useCallback((target: ActiveSession | null): ShellDialogOriginV1 | null =>
    shellDialogOriginV1(target, [...openDocumentSessionsRef.current].map(([runtimeKey, entry]) => ({ runtimeKey, ...entry }))), []);
  const isCurrentDialogOrigin = useCallback((origin: ShellDialogOriginV1 | null): boolean =>
    shellDialogOriginIsCurrentV1(origin, captureDialogOrigin(shellStateRef.current.pluginRuntime.session)), [captureDialogOrigin]);
  const captureEffectOwner = useCallback((source: ActiveSession, presentation: ShellDialogOriginV1 | null) => ({
    presentation,
    source: captureDialogOrigin(source),
    session: source,
    plugin: loadedPluginsRef.current.find((entry) => entry.handle.pluginId === source.pluginId)?.handle ?? null,
    creationCatalog: captureSpaceArtifactCreationCatalogAuthorityV1(spaceArtifactCreationCatalogRef.current, spaceArtifactCreationCatalogUiRef.current, presentation),
  }), [captureDialogOrigin]);
  const isCurrentEffectOwner = useCallback((owner: ReturnType<typeof captureEffectOwner>): boolean => {
    const primary = shellStateRef.current.pluginRuntime.session;
    return isCurrentDialogOrigin(owner.presentation) && owner.plugin !== null
      && loadedPluginsRef.current.find((entry) => entry.handle.pluginId === owner.session.pluginId)?.handle === owner.plugin
      && shellEffectSourceIsCurrentV1(owner.source, captureDialogOrigin(owner.session), primary, primary === null ? [] : parsePanelState(primary.viewState)?.spawnedApps ?? []);
  }, [captureDialogOrigin, isCurrentDialogOrigin]);
  const closeOwnedDialog = useCallback((openingId: number): boolean => {
    if (liveDialogRef.current?.openingId !== openingId) return false;
    liveDialogRef.current = null;
    dispatch({ type: "CLOSE_DIALOG", openingId });
    return true;
  }, []);
  const makeOwnedDialog = useCallback((dialogId: string, origin: ShellDialogOriginV1 | null, seedArgs?: Readonly<Record<string, unknown>>): ShellDialogV1 | null => {
    const active = shellStateRef.current.pluginRuntime.session;
    if (origin === null || !isCurrentDialogOrigin(origin) || !active?.app.dialogs?.some((entry) => entry.id === dialogId)) return null;
    return { openingId: ++dialogOpeningRef.current, dialogId, origin, ...(seedArgs === undefined ? {} : { seedArgs }) };
  }, [isCurrentDialogOrigin]);
  type RetainedBrowserActorUiV1 = { readonly clientInstanceId: string; readonly activationGeneration: string; readonly verifiedSurfaceId: string; readonly scope: DocumentScope; readonly sessionInstanceId: number; readonly windowKindId: string; readonly store: UiDocumentStore; readonly identity: BrowserActorUiMountedV1 | null; readonly actions: BrowserActorActionMailboxV1 };
  const browserActorUiByRuntimeKeyRef = useRef(new Map<string, RetainedBrowserActorUiV1>());
  const [browserActorUiVersion, setBrowserActorUiVersion] = useState(0);
  const retireBrowserActorUi = useCallback((runtimeKey: string, reason: string) => {
    browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.actions.close(reason);
    if (browserActorUiByRuntimeKeyRef.current.delete(runtimeKey)) setBrowserActorUiVersion((current) => current + 1);
  }, []);
  const directoryScopedOwnersRef = useRef<Map<string, DocumentScope>>(new Map());
  const socketActorReadyRef = useRef<Map<string, { readonly clientInstanceId: string; resolve(actorId: string): void; reject(error: Error): void }>>(new Map());
  const [bootstrapUiByDocument, setBootstrapUiByDocument] = useState<BootstrapUiState>({});
  const [executionTargetUiByDocument, setExecutionTargetUiByDocument] = useState<ExecutionTargetUiState>({});
  const [presencePeersByRuntimeKey, setPresencePeersByRuntimeKey] = useState<Readonly<Record<string, readonly PresencePeer[]>>>({});
  const rebootstrapDiscardedSessionsRef = useRef<Map<string, ActiveSession>>(new Map());
  /** 🐚️ Mirrors `loadedPlugins` for the unmount-cleanup effect below, which needs the latest value at
   * teardown time without depending on it (a dependency would tear down and re-run on every reload). */
  const loadedPluginsRef = useRef<readonly LoadedProgramState[]>([]);
  loadedPluginsRef.current = loadedPlugins;
  /** 🩺️ Read at catch time by the fault classifier so a crashed/quarantined supervisor outranks the
   * plugin's own fault code, without making every refresh effect depend on the supervisor roster. */
  const pluginSupervisorByIdRef = useRef<Readonly<Record<string, PluginSupervisorState>>>({});
  pluginSupervisorByIdRef.current = pluginSupervisorById;
  /** 🔌️ The exact (possibly cache-busted `?v=`) module URL each currently-loaded plugin was acquired
   * at — `LoadedProgramState`/`PluginWasmHandle` carry no URL of their own. 🧬️ H1-react: its old
   * reader, `evictPluginModule` (the deleted refcounted module-URL lease pool, packet H2's "must not
   * exist" list), is gone — `reloadPlugin`/`uninstallPlugin` now free resources entirely through
   * `handle.dispose()` (`ShardClient.dispose` per activated actor), so this map is unread bookkeeping
   * kept for a future consumer rather than removed mid-packet. */
  const pluginModuleUrlByIdRef = useRef<Map<string, string>>(new Map());
  /** 🔁️ The `rebuiltAt` each currently-loaded plugin's artifact was acquired at, so the `PluginSource`
   * pump can tell a genuine rebuild from a replayed availability snapshot
   * ({@link pluginAvailabilityRouteV1}). Absent means "acquired unbusted", i.e. the first load, which
   * names no build. */
  const pluginArtifactRebuiltAtRef = useRef<Map<string, number>>(new Map());
  /** 🔁️ Records what an acquisition was built from, forgetting it again when the acquisition named no
   * build — so the map never claims a build the loaded artifact does not have. */
  const recordPluginArtifactRebuiltAt = useCallback((pluginId: string, rebuiltAt: number | undefined) => {
    if (rebuiltAt === undefined) pluginArtifactRebuiltAtRef.current.delete(pluginId);
    else pluginArtifactRebuiltAtRef.current.set(pluginId, rebuiltAt);
  }, []);
  /** 🔌️ Per-pluginId mutual exclusion across `installPlugin`/`reloadPlugin`/`uninstallPlugin` — the
   * boot effect and the `PluginSource` subscription effect can both request the same pluginId around
   * mount (e.g. the host plugin already appears in the connect-time `snapshot`), and without this guard
   * both calls would independently acquire a module lease, race their `UPSERT_LOADED_PLUGIN` dispatches,
   * and leak whichever lease lost the race (nothing left holding a reference to release it). */
  const pluginOpInFlightRef = useRef<Set<string>>(new Set());

  const ensureBackboneWorker = useCallback((): Worker => {
    if (backboneWorkerRef.current) return backboneWorkerRef.current;
    const worker = new Worker(new URL("../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url), { type: "module" });
    const brokerChannel = new MessageChannel();
    worker.postMessage({ kind: "semio-browser-broker-port", port: brokerChannel.port2 }, [brokerChannel.port2]);
    localBrowserBrokerRef.current = new BrowserBrokerPortClientV1(brokerChannel.port1, localBrowserBrokerProof);
    localBrowserBrokerProof = undefined;
    worker.onmessage = (messageEvent: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>) => {
      const message = "wire" in messageEvent.data ? decodeBackboneWorkerResponse(messageEvent.data.wire) : messageEvent.data;
      if (message.kind === "browser-actor-action-result") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", ...message.scope });
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (retained === undefined || entry === undefined || retained.clientInstanceId !== message.clientInstanceId || entry.clientInstanceId !== message.clientInstanceId || retained.sessionInstanceId !== entry.session.instanceId) return;
        const { clientInstanceId: _clientInstanceId, ...result } = message;
        retained.actions.settle(result);
        return;
      }
      if (message.kind === "browser-actor-ui-patch") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", spaceId: message.scope.spaceId, documentId: message.scope.documentId });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const expectedSurfaceId = entry?.scope !== undefined && entry.session.app.dialect ? canonicalSurfaceId(entry.session.app.dialect, entry.session.app.role) : null;
        const windowKind = entry?.session.app.windowKinds.find((candidate) => candidate.id === message.patch.surface);
        if (entry === undefined || entry.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId || expectedSurfaceId !== message.verifiedSurfaceId || windowKind === undefined || message.instanceId !== 0) return;
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        if (retained !== undefined && (retained.clientInstanceId !== message.clientInstanceId || retained.activationGeneration !== message.activationGeneration || retained.verifiedSurfaceId !== message.verifiedSurfaceId || retained.sessionInstanceId !== entry.session.instanceId || retained.windowKindId !== windowKind.id)) return;
        if (retained === undefined && message.patch.baseRevision !== 0) return;
        const store = retained?.store ?? new UiDocumentStore(windowKind.id);
        const applied = store.applyPatch(message.patch);
        const revision = store.getRevisionSnapshot();
        if (applied.ok) {
          if (retained === undefined) {
            const actions = new BrowserActorActionMailboxV1((request) => worker.postMessage({ wire: encodeBackboneWorkerRequest({ ...request, clientInstanceId: entry.clientInstanceId }) }));
            browserActorUiByRuntimeKeyRef.current.set(runtimeKey, { clientInstanceId: message.clientInstanceId, activationGeneration: message.activationGeneration, verifiedSurfaceId: message.verifiedSurfaceId, scope: { ...message.scope }, sessionInstanceId: entry.session.instanceId, windowKindId: windowKind.id, store, identity: null, actions });
            setBrowserActorUiVersion((current) => current + 1);
          }
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "browser-actor-ui-patch-result", clientInstanceId: entry.clientInstanceId, scope: message.scope, verifiedSurfaceId: message.verifiedSurfaceId, activationGeneration: message.activationGeneration, instanceId: message.instanceId, receipt: message.receipt, outcome: "acknowledged", revision }) });
        } else {
          const reason = applied.rejection.type;
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "browser-actor-ui-patch-result", clientInstanceId: entry.clientInstanceId, scope: message.scope, verifiedSurfaceId: message.verifiedSurfaceId, activationGeneration: message.activationGeneration, instanceId: message.instanceId, receipt: message.receipt, outcome: "rejected", revision, reason }) });
        }
        return;
      }
      if (message.kind === "browser-actor-ui-mounted") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", ...message.scope });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        if (entry === undefined || retained === undefined || !browserDocumentMountIsCurrentV1(
          { clientInstanceId: entry.clientInstanceId, scope: entry.scope, instanceId: entry.session.instanceId },
          { clientInstanceId: retained.clientInstanceId, scope: retained.scope, instanceId: retained.sessionInstanceId, activationGeneration: retained.activationGeneration, verifiedSurfaceId: retained.verifiedSurfaceId, revision: retained.store.getRevisionSnapshot() },
          { ...message, revision: message.uiRevision },
        )) return;
        if (entry.creationMount !== null && !entry.creationMount.accept(message.catalogGenerationId)) {
          failDocumentBackbone(runtimeKey, entry, new Error("artifact-creation.catalog-generation-mismatch"));
          return;
        }
        browserActorUiByRuntimeKeyRef.current.set(runtimeKey, { ...retained, identity: message });
        entry.resolveReady();
        const discarded = rebootstrapDiscardedSessionsRef.current.get(runtimeKey);
        if (discarded !== undefined) {
          rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
          dispatch({ type: "SET_SESSION", value: (current) => current ?? discarded });
        }
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "snapshot-replaced", documentId: message.scope.documentId, scope: message.scope }));
        setBrowserActorUiVersion((current) => current + 1);
        return;
      }
      if (message.kind === "socket-actor") {
        const runtimeKey = scopeRuntimeKey(message);
        if (runtimeKey === null) return;
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const waiter = socketActorReadyRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || waiter?.clientInstanceId !== message.clientInstanceId) return;
        waiter.resolve(message.actorId);
        if (socketActorReadyRef.current.get(runtimeKey)?.clientInstanceId === message.clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
        return;
      }
      if (message.kind === "inference-port-opened") {
        inferencePortOpeningRef.current?.mailbox.settle(message);
        return;
      }
      if (message.kind === "inference-port-closed") {
        const owner = inferencePortOwnerRef.current;
        if (owner === null || owner.operationEpoch !== message.operationEpoch || owner.scope.spaceId !== message.scope.spaceId || owner.scope.documentId !== message.scope.documentId) return;
        inferencePortOwnerRef.current = null;
        inferencePortAuthorityRef.current = null;
        dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey: owner.runtimeKey });
        return;
      }
      if (message.kind === "inference-port-status") {
        if (!directorySessionAuthorityIsCurrentV1(inferencePortAuthorityRef.current, verifiedSessionAuthorityRef.current)) return;
        const runtimeKey = inferencePortStatusRuntimeKeyV1(inferencePortOwnerRef.current, inferencePortEpochRef.current, message);
        if (runtimeKey === null) return;
        dispatch({ type: "SET_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey, status: message.status });
        return;
      }
      if (message.kind === "inference-history-status") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", ...message.scope });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const authority = verifiedSessionAuthorityRef.current;
        if (authority === null || !directorySessionAuthorityIsCurrentV1(authority, authority) || entry === undefined || retiredSessionDocumentOwnersRef.current.has(entry) || entry.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId) return;
        setInferenceHistoryByRuntimeKey((current) => {
          if (!directorySessionAuthorityIsCurrentV1(authority, verifiedSessionAuthorityRef.current) || retiredSessionDocumentOwnersRef.current.has(entry)) return current;
          const previous = current[runtimeKey];
          if (previous !== undefined && (message.historyEpoch < previous.historyEpoch || previous.clientInstanceId !== message.clientInstanceId)) return current;
          const order = previous?.historyEpoch === message.historyEpoch ? previous.order : ++historyOrderRef.current;
          return { ...current, [runtimeKey]: { historyEpoch: message.historyEpoch, clientInstanceId: message.clientInstanceId, scope: message.scope, sessionInstanceId: entry.session.instanceId, status: message.status, order, authority } };
        });
        return;
      }
      if (message.kind === "execution-target-status") {
        const runtimeKey = scopeRuntimeKey(message);
        const entry = runtimeKey === null ? undefined : openDocumentSessionsRef.current.get(runtimeKey);
        if (runtimeKey === null || message.scope?.spaceId !== message.spaceId || entry?.clientInstanceId !== message.clientInstanceId) return;
        setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, message));
        return;
      }
      if (message.kind === "socket-actor-failed") {
        const runtimeKey = scopeRuntimeKey(message);
        if (runtimeKey === null) return;
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const waiter = socketActorReadyRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || waiter?.clientInstanceId !== message.clientInstanceId) return;
        waiter.reject(new Error(`socket actor unavailable (${message.code})`));
        if (socketActorReadyRef.current.get(runtimeKey)?.clientInstanceId === message.clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
        return;
      }
      if (message.kind === "space-artifact-creation-catalog-refresh-required") {
        const owner = spaceArtifactCreationOwnersRef.current.get(message.requestId) ?? null;
        const origin = captureDialogOrigin(shellStateRef.current.pluginRuntime.session);
        const authority = captureSpaceArtifactCreationCatalogAuthorityV1(
          spaceArtifactCreationCatalogRef.current,
          spaceArtifactCreationCatalogUiRef.current,
          origin,
        );
        const request = spaceArtifactCreationCatalogRefreshRequestV1(owner, authority, origin, message);
        if (request === null) return;
        const loading = {
          kind: "space-artifact-creation-catalog-status",
          clientInstanceId: request.clientInstanceId,
          spaceId: request.spaceId,
          phase: "loading",
        } as const;
        spaceArtifactCreationCatalogRef.current = null;
        spaceArtifactCreationCatalogUiRef.current = loading;
        setSpaceArtifactCreationCatalog(null);
        setSpaceArtifactCreationCatalogUi(loading);
        worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
        return;
      }
      if (message.kind === "space-artifact-creation-status") {
        const owner = spaceArtifactCreationOwnersRef.current.get(message.requestId);
        if (owner === undefined || !spaceArtifactCreationOwnerAcceptsStatus(owner, message)) return;
        const entry = openDocumentSessionsRef.current.get(owner.runtimeKey);
        if (
          entry === undefined
          || entry.clientInstanceId !== owner.clientInstanceId
          || entry.session.instanceId !== owner.sessionInstanceId
          || entry.scope?.spaceId !== owner.spaceId
          || entry.scope.documentId !== S_SPACE_INDEX_DOCUMENT_ID
        ) {
          spaceArtifactCreationOwnersRef.current.delete(owner.requestId);
          setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "cleared", requestId: owner.requestId }));
          if (message.phase !== "ready") worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId: owner.requestId, spaceId: owner.spaceId }) });
          return;
        }
        setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "status", message }));
        if (message.phase === "failed" || message.phase === "cancelled" || message.phase === "indeterminate") {
          spaceArtifactCreationOwnersRef.current.delete(owner.requestId);
          return;
        }
        const openingArgs = spaceArtifactCreationReadyOpening(message);
        if (openingArgs === null || owner.opening) return;
        const firstReady = owner.ready === null;
        const readyOwner: SpaceArtifactCreationOwnerV1 = firstReady ? { ...owner, ready: message } : owner;
        spaceArtifactCreationOwnersRef.current.set(owner.requestId, readyOwner);
        if (firstReady) openReadySpaceArtifactCreationRef.current(owner.requestId);
        return;
      }
      if (message.kind === "space-artifact-creation-catalog") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", spaceId: message.spaceId, documentId: S_SPACE_INDEX_DOCUMENT_ID });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.spaceId || entry.scope.documentId !== S_SPACE_INDEX_DOCUMENT_ID) return;
        setSpaceArtifactCreationCatalog(message);
        return;
      }
      if (message.kind === "space-artifact-creation-catalog-status") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", spaceId: message.spaceId, documentId: S_SPACE_INDEX_DOCUMENT_ID });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.spaceId || entry.scope.documentId !== S_SPACE_INDEX_DOCUMENT_ID) return;
        if (message.phase !== "ready") setSpaceArtifactCreationCatalog(null);
        setSpaceArtifactCreationCatalogUi(message);
        return;
      }
      // 📇️ §C6 directory lane — the worker's `directory-*` responses never carry a `documentId` this
      // shell already has an `openDocumentSessionsRef` entry for (they're not artifact-sync events at
      // all), so they're routed here, ahead of the artifact-event early return below.
      if (message.kind === "directory-event-page") {
        handleDirectoryEventPageRef.current(message);
        return;
      }
      if (message.kind === "directory-bootstrap-failed") {
        handleDirectoryBootstrapFailureRef.current(message);
        return;
      }
      if (message.kind === "directory-message") {
        if (message.message.kind === "event") dispatchDirectoryEventsRef.current([message.message.event]);
        return;
      }
      if (message.kind === "directory-scope-revoked") {
        const key = documentRuntimeKeyV1({ kind: "hub", spaceId: message.scope.spaceId, documentId: message.scope.documentId });
        if (!directoryScopedOwnersRef.current.delete(key)) return;
        cancelSpaceArtifactCreationsForRuntime(key, worker);
        setSpaceArtifactCreationCatalog((catalog) => catalog?.spaceId === message.scope.spaceId ? null : catalog);
        setSpaceArtifactCreationCatalogUi((status) => status?.spaceId === message.scope.spaceId ? null : status);
        const entry = openDocumentSessionsRef.current.get(key);
        if (entry !== undefined) closeDocumentRef.current(key, entry.clientInstanceId);
        retireBrowserActorUi(key, "browser-actor-action: scope revoked");
        rebootstrapDiscardedSessionsRef.current.delete(key);
        setInferenceHistoryByRuntimeKey((current) => {
          if (!(key in current)) return current;
          const next = { ...current };
          delete next[key];
          return next;
        });
        setPresencePeersByRuntimeKey((current) => {
          if (!(key in current)) return current;
          const next = { ...current };
          delete next[key];
          return next;
        });
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "detached", documentId: message.scope.documentId, scope: message.scope }));
        setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, { kind: "execution-target-cleared", documentId: message.scope.documentId, scope: message.scope }));
        return;
      }
      if (message.kind === "directory-status") {
        setDirectoryPendingCommands(message.pendingCommands);
        return;
      }
      if (message.kind === "directory-administration-state") {
        const epoch = spaceAdministrationEpochRef.current;
        const administrationMessage = message;
        if (administrationMessage.operationEpoch !== epoch || worker !== backboneWorkerRef.current || administrationMessage.spaceId !== spaceAdministrationRef.current?.spaceId) return;
        const revision = ++spaceAdministrationMessageRef.current;
        const verificationIsCurrent = (): boolean => epoch === spaceAdministrationEpochRef.current && revision === spaceAdministrationMessageRef.current && worker === backboneWorkerRef.current;
        if (administrationMessage.canonicalJson === undefined) {
          setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, administrationMessage, epoch, current?.page ?? null) : current);
          return;
        }
        void parseDirectorySpaceAdministrationPageV1(administrationMessage.canonicalJson)
          .then((page) => setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, administrationMessage, epoch, page) : current))
          .catch(() => setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, { ...administrationMessage, phase: "failed", code: "invalid" }, epoch, null) : current));
        return;
      }
      if (message.kind === "directory-administration-capability") {
        if (!shellSpaceAdministrationCapabilityAllowed(spaceAdministrationRef.current, message.operationEpoch) || message.operationEpoch !== spaceAdministrationEpochRef.current) return;
        const { operationEpoch, transferEpoch, inviteToken } = message;
        void copyDirectoryInviteCapabilityV1(inviteToken).then((copied) => {
          const currentWorker = backboneWorkerRef.current;
          if (currentWorker === null || operationEpoch !== spaceAdministrationEpochRef.current) return;
          currentWorker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-administration-capability-result", operationEpoch, transferEpoch, copied }) });
        });
        return;
      }
      if (message.kind === "directory-administration-capability-rejected") {
        return;
      }
      if (message.kind === "directory-command-failed") {
        retainDirectoryCommandResult(directoryCommandResultsRef.current, message.requestId, { kind: "failed", code: message.code });
        console.error("[os-shell] directory command failed", message.requestId, message.code);
        return;
      }
      if (message.kind === "directory-command-receipt") {
        // 📇️ The retained slot is the receipt's owner: it is filled FIRST, and only an `accepted`
        // receipt's durable events are folded. Defense-in-depth (ticket 26/08/16/HUB-SPACES-LIVE-
        // PRESENCE-AND-COLLABORATIVE-STUDIOS w4-h): the ORIGINATING client folds its own accepted
        // command's events instead of depending entirely on the live `/directory/socket/v1`
        // broadcast finding its way back to the same socket — correct only if that subscription is
        // guaranteed already-open at command-issue time, which a fresh page load racing identity
        // bootstrap does not guarantee. The live broadcast path (`directory-message` above) still
        // folds the same events for every OTHER client; a duplicate here is harmless —
        // `FoldDirectoryEvent`'s fold is idempotent per event id (config lane, "last envelope wins").
        retainDirectoryCommandResult(directoryCommandResultsRef.current, message.requestId, { kind: "receipt", receipt: message.receipt });
        if (message.receipt.outcome === "accepted" && message.receipt.events.length > 0) dispatchDirectoryEventsRef.current(message.receipt.events);
        return;
      }
      if (message.kind === "artifact-bootstrap-progress" || message.kind === "artifact-bootstrap-failed" || message.kind === "artifact-rebootstrap-required") {
        const runtimeKey = scopeRuntimeKey(message);
        if (runtimeKey === null) return;
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (!entry || entry.clientInstanceId !== message.clientInstanceId) return;
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, message));
        if (entry.creationMount !== null && message.kind !== "artifact-bootstrap-progress") {
          failDocumentBackbone(runtimeKey, entry, new Error(message.kind === "artifact-bootstrap-failed" ? message.message : "artifact-creation.rebootstrap-required"));
          return;
        }
        if (message.kind === "artifact-bootstrap-failed") {
          retireBrowserActorUi(runtimeKey, "browser-actor-action: bootstrap failed");
          entry.replacements.invalidate();
          entry.rejectReady(new Error(message.message));
          const retirement = entry.port?.retire();
          void retirement?.catch(error => failDocumentBackbone(runtimeKey, entry, error));
          entry.port = null;
          entry.pending = [];
          entry.pendingBytes = 0;
        }
        if (message.kind === "artifact-rebootstrap-required") {
          entry.replacements.invalidate();
          const retirement = entry.port?.retire();
          void retirement?.catch(error => failDocumentBackbone(runtimeKey, entry, error));
          entry.port = null;
          entry.pending = [];
          entry.pendingBytes = 0;
          cancelSpaceArtifactCreationsForRuntime(runtimeKey, worker);
          retireBrowserActorUi(runtimeKey, "browser-actor-action: rebootstrap required");
          const active = sessionRef.current;
          if (shellDialogSessionIsCurrentV1(active, entry.session)) {
            rebootstrapDiscardedSessionsRef.current.set(runtimeKey, entry.session);
            console.warn("[DEBUG] document rebootstrap kept session", JSON.stringify({ runtimeKey, instanceId: entry.session.instanceId }));
          }
        }
        return;
      }
      // 🪪️ §C3 identity facet — opened directly (never through `openDocument`, so it never gets an
      // `openDocumentSessionsRef` entry: it has no plugin/session, only the shell itself). Routed here,
      // ahead of the `!entry` early return below, which would otherwise silently drop every one of its
      // events. `documentArchiveReplaced` (whole-record archive, the folder read-back) resolves the bootstrap
      // effect's one-shot wait; `remoteMutations` (another tab's sign-in/out, echoed over this
      // document's `BroadcastChannel`) folds via {@link foldIdentityEvent} like `OpeningPreferences`.
      if (message.kind === "event" && message.documentId === IDENTITY_CONFIG_SCHEMA) {
        if (identityClientInstanceIdRef.current !== message.clientInstanceId) return;
        const identityEvent = message.event;
        if (identityEvent.kind === "documentArchiveReplaced") {
          const archive = decodeDocumentArchiveBytes(new Uint8Array(identityEvent.archive));
          const decoded = decodeIdentityPayload(decodePackValue(new Uint8Array(archive.parent_pack)));
          if (decoded !== undefined) {
            if (verifiedSessionAuthorityRef.current === null) setIdentity(decoded);
            identitySnapshotResolverRef.current?.(decoded);
            identitySnapshotResolverRef.current = null;
          }
        } else if (identityEvent.kind === "remoteMutations") {
          if (verifiedSessionAuthorityRef.current === null) setIdentity((current) => foldIdentityEvent(current, identityEvent, decodeIdentityPayload));
          void sessionRefreshRef.current?.refresh();
        }
        return;
      }
      if (message.kind !== "event") return;
      const runtimeKey = message.scope === undefined ? message.documentId : scopeRuntimeKey(message);
      if (runtimeKey === null) return;
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (!entry || entry.clientInstanceId !== message.clientInstanceId) return;
      const { event } = message;
      if (event.kind === "status") {
        dispatch({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: runtimeKey, status: { persisted: event.persisted, pendingMutations: event.pendingMutations, remote: event.remote } });
      } else if (event.kind === "presence") {
        const peers = entry.scope === undefined ? [] : scopedPresencePeersV1(message, entry.scope);
        setPresencePeersByRuntimeKey((current) => {
          if (current[runtimeKey] === peers) return current;
          return { ...current, [runtimeKey]: peers };
        });
      } else if (event.kind === "documentBackbone") {
        receiveDocumentBackbone(runtimeKey, entry, event.message);
      } else if (event.kind === "documentArchiveReplaced") {
        const archiveBytes = new Uint8Array(event.archive);
        void (async () => {
          try {
            const archive = decodeDocumentArchiveBytes(archiveBytes);
            if (!await loadDocumentArchive(entry.plugin, entry.session.instanceId, archive, () => openDocumentSessionsRef.current.get(runtimeKey) === entry)) return;
            if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
            const discarded = rebootstrapDiscardedSessionsRef.current.get(runtimeKey);
            if (discarded) {
              rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
              dispatch({ type: "SET_SESSION", value: (current) => current ?? discarded });
            }
            setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "snapshot-replaced", documentId: message.documentId, ...(message.scope === undefined ? {} : { scope: message.scope }) }));
          } catch (replacementError) {
            if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== message.clientInstanceId) return;
            if (entry.creationMount !== null) {
              failDocumentBackbone(runtimeKey, entry, replacementError);
              return;
            }
            entry.rejectReady(replacementError instanceof Error ? replacementError : new Error(String(replacementError)));
            setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, {
              kind: "artifact-bootstrap-failed",
              documentId: message.documentId,
              clientInstanceId: message.clientInstanceId,
              ...(message.scope === undefined ? {} : { scope: message.scope }),
              code: "invalid-bootstrap",
              message: (replacementError instanceof Error ? replacementError.message : String(replacementError)).slice(0, 4_096),
              retryable: false,
            }));
          }
        })();
      } else if (event.kind === "conflict") {
        // ⚖️ Investigated for contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
        // CLASS-CONFLICTS` §C6/§C9 (lane L1): this is `🏪️store/🔄️sync/🦀️.rs`'s hub-relay
        // `ArtifactEvent::Conflict(MutationMessage)` — a TRANSPORT-level diagnostic (external folder
        // divergence / a hub `ServerFrame::Error`), never a first-class `Conflict` roster (no
        // `id`/`status`/`actors` exists for either source event, and `protocol_wire::ServerFrame` has
        // no `MergeReport`/`Conflicts` variant to carry one). The REAL roster/merge-outcome delivery
        // this contract added is `AppCommand::ApplyEnvelopes`'s own reply — see the `remoteMutations`
        // branch above (`applyRemoteMergeRef`) — so this branch stays a passive log, same as before.
        console.warn("[os-shell] sync conflict", message.documentId, event.message);
      }
    };
    const failBrowserActorActions = () => {
      for (const retained of browserActorUiByRuntimeKeyRef.current.values()) retained.actions.close("browser-actor-action: worker completion unconfirmed");
    };
    worker.addEventListener("error", failBrowserActorActions);
    worker.addEventListener("messageerror", failBrowserActorActions);
    backboneWorkerRef.current = worker;
    return worker;
  }, [cancelSpaceArtifactCreationsForRuntime, captureDialogOrigin, failDocumentBackbone, hubEnv, loadDocumentArchive, receiveDocumentBackbone, retireBrowserActorUi]);

  handleDirectoryEventPageRef.current = (message) => {
    const owner = directoryHomeOwnerRef.current;
    const worker = backboneWorkerRef.current;
    if (!owner || !worker || message.bootstrapEpoch !== owner.bootstrapEpoch) return;
    setDirectoryBootstrapUi({ kind: "pending", throughSeqInclusive: message.throughSeqInclusive, cancellable: true });
    void applyDirectoryEventPageBootstrapV1(
      owner,
      message,
      (request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }),
      async (appliedOwner) => {
        const current = sessionRef.current;
        if (!current || current.pluginId !== appliedOwner.plugin.pluginId || current.app.id !== appliedOwner.app.id || current.instanceId !== appliedOwner.instanceId) throw new Error("directory-bootstrap.visible-owner-stale");
        await refreshDirectoryHomeRef.current(current);
      },
    ).then((result) => {
      if (directoryHomeOwnerRef.current !== owner) return;
      if (owner.abort.signal.aborted) directoryHomeOwnerRef.current = null;
      setDirectoryBootstrapUi(result.state);
    });
  };

  handleDirectoryBootstrapFailureRef.current = (message) => {
    const owner = directoryHomeOwnerRef.current;
    const worker = backboneWorkerRef.current;
    if (!owner || !worker || message.bootstrapEpoch !== owner.bootstrapEpoch) return;
    if (message.retryable) {
      setDirectoryBootstrapUi({ kind: "retrying", throughSeqInclusive: owner.pending?.throughSeqInclusive ?? 0 });
      return;
    }
    directoryHomeOwnerRef.current = null;
    void retireDirectoryHomeOwner(owner, worker);
    setDirectoryBootstrapUi({ kind: "fault", code: `directory-bootstrap.${message.code}` });
  };

  const cancelDirectoryBootstrap = useCallback(() => {
    const owner = directoryHomeOwnerRef.current;
    const worker = backboneWorkerRef.current;
    if (!owner || !worker) return;
    directoryHomeOwnerRef.current = null;
    void retireDirectoryHomeOwner(owner, worker).finally(() => {
      if (!directoryHomeOwnerRef.current) setDirectoryBootstrapUi({ kind: "idle" });
    });
  }, [retireDirectoryHomeOwner]);

  /** 🎬️ The pane's ONLY path to the wire. An intent the current canonical page does not authorize
   * produces no request at all, so an unauthorized control can never reach the hub even if the DOM
   * is tampered with; `close` additionally retires the epoch so a late worker message is dropped. */
  const dispatchSpaceAdministrationIntent = useCallback((intent: SpaceAdministrationIntentV1) => {
    const state = spaceAdministration;
    const worker = backboneWorkerRef.current;
    if (state === null || worker === null || state.operationEpoch !== spaceAdministrationEpochRef.current) return;
    const request = shellSpaceAdministrationRequest(state, intent, mintDirectoryCommandRequestId());
    if (request === null) return;
    if (intent.kind === "close") {
      spaceAdministrationEpochRef.current += 1;
      setSpaceAdministration(null);
    }
    worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, [spaceAdministration]);

  /** 🧯️ Unmount and identity change both retire the operation: the worker cancels its transport and
   * erases page, receipt, and capability before the pane can be remounted under a new identity. */
  useEffect(() => () => {
    const worker = backboneWorkerRef.current;
    const operationEpoch = spaceAdministrationEpochRef.current;
    if (worker) worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-administration-close", operationEpoch }) });
    spaceAdministrationEpochRef.current = operationEpoch + 1;
    setSpaceAdministration(null);
  }, [identity, verifiedSessionAuthority?.authorizationGeneration, verifiedSessionAuthority?.sessionBindingSha256]);

  /** 💡️ Relays one operator intent to the worker-owned port. Nothing is applied locally: `cancel`
   * and `approve` only ask, and the phase moves solely on the worker's own next status. `close`
   * waits for the worker's exact server-confirmed retirement. */
  const dispatchInferencePortIntent = useCallback((runtimeKey: string, action: InferencePortUiAction) => {
    const worker = backboneWorkerRef.current;
    const operationEpoch = inferencePortEpochRef.current;
    const owner = inferencePortOwnerRef.current;
    if (!worker || owner === null || owner.runtimeKey !== runtimeKey || owner.operationEpoch !== operationEpoch) return;
    if (action.kind !== "close" && !directorySessionAuthorityIsCurrentV1(inferencePortAuthorityRef.current, verifiedSessionAuthorityRef.current)) return;
    if (action.kind === "propose") {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-propose", operationEpoch, requestId: mintDirectoryCommandRequestId() }) });
      return;
    }
    if (action.kind === "cancel") {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-cancel", operationEpoch }) });
      return;
    }
    if (action.kind === "approve") {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-approve", operationEpoch }) });
      return;
    }
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-close", operationEpoch }) });
  }, []);

  /** 🧯️ Retires session-owned presentation immediately while the worker retains uncertain jobs. */
  const retireAuthenticatedInferencePresentation = useCallback(() => {
    inferencePortOpeningRef.current?.mailbox.close("inference-opening: identity retired");
    inferencePortOpeningRef.current = null;
    const worker = backboneWorkerRef.current;
    const operationEpoch = inferencePortEpochRef.current;
    if (worker) worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-close", operationEpoch }) });
    const owner = inferencePortOwnerRef.current;
    inferencePortOwnerRef.current = null;
    inferencePortAuthorityRef.current = null;
    inferencePortEpochRef.current = operationEpoch + 1;
    if (owner !== null) dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey: owner.runtimeKey });
    for (const entry of openDocumentSessionsRef.current.values()) if (entry.scope !== undefined) retiredSessionDocumentOwnersRef.current.add(entry);
    inferenceHistoryByRuntimeKeyRef.current = {};
    setInferenceHistoryByRuntimeKey({});
  }, []);
  useEffect(() => retireAuthenticatedInferencePresentation, [retireAuthenticatedInferencePresentation]);

  // 🪪️ §C3 identity bootstrap — pre-identity default actor, set once at mount so `PluginRuntime`'s
  // `AppChannelClient`s created before sign-in resolves (or with no hub env at all) still carry the
  // SAME `client-{sessionId}` id `shellActorIdRef` already defaults to.
  useEffect(() => {
    setPluginRuntimeActor(shellActorIdRef.current);
  }, []);

  const cancelSessionAuthorityBootstrap = useCallback(() => {
    const abort = identityBootstrapAbortRef.current;
    if (!hubEnv || abort === null || abort.signal.aborted) return;
    identityBootstrapAbortRef.current = null;
    abort.abort("directory.session-authority.cancelled");
    sessionRefreshRef.current?.close();
    sessionRefreshRef.current = null;
    localBrowserBrokerRef.current?.close();
    localBrowserBrokerRef.current = null;
    retireAuthenticatedInferencePresentation();
    verifiedSessionAuthorityRef.current = null;
    setVerifiedSessionAuthority(null);
    setIdentityOffline(true);
    shellActorIdRef.current = shellActorId(shellSessionIdRef.current, null);
    setPluginRuntimeActor(shellActorIdRef.current);
    const clientInstanceId = identityClientInstanceIdRef.current;
    if (clientInstanceId !== null) {
      backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId }) });
      if (identityClientInstanceIdRef.current === clientInstanceId) identityClientInstanceIdRef.current = null;
    }
  }, [hubEnv, retireAuthenticatedInferencePresentation]);

  useEffect(() => {
    // 📇️ §C3 "No hub env ⇒ skip all of it and keep today's local-only behaviour exactly" — the
    // existing `🛠️dev🖥️s⚛️react` launcher (no `S_HUB_URL`) never reaches any code below this guard.
    if (!hubEnv) return;
    let cancelled = false;
    const identityWaitAbort = new AbortController();
    identityBootstrapAbortRef.current = identityWaitAbort;
    (async () => {
      const worker = ensureBackboneWorker();
      const identityConfig = identityActorConfig(shellActorIdRef.current, hubEnv.dataDir);
      const identityAttempt = { clientInstanceId: crypto.randomUUID() };
      const { clientInstanceId } = identityAttempt;
      identityClientInstanceIdRef.current = clientInstanceId;
      // 📇️ Opens the identity document FIRST (folder poll starts immediately) so the snapshot wait
      // below has something to resolve against; re-opening later with the same `documentId` (once the
      // real actor id is known) is a harmless idempotent re-subscribe (`openArtifact` always closes
      // any prior state for the same id first).
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", clientInstanceId, ...identityConfig }) });
      // 📇️ terra-web-shellhost (finding 3) — event-driven wait for a previously-persisted identity's
      // `documentArchiveReplaced`, raced against a 2s `AbortSignal.timeout` instead of an unconditional fixed
      // `setTimeout`: a 404/no-file-yet folder read never emits a `documentArchiveReplaced` at all (see
      // `pollFolderOnce`'s doc in `🏪️store/👷️worker/🟦️.ts`), so the timeout branch is still the only way
      // a session with no persisted identity ever proceeds — but a REAL event now resolves as soon as
      // it arrives instead of always paying the full 2s. `identityWaitAbort` folds into the same raced
      // signal so an unmount tears the subscription down immediately (this effect's own cleanup below)
      // rather than leaking a live listener/timer until the timeout fires on its own.
      let cachedIdentity: Identity | null = null;
      try {
        cachedIdentity = await waitForEvent<Identity | null>(
          (handler) => {
            identitySnapshotResolverRef.current = handler;
            return () => {
              if (identitySnapshotResolverRef.current === handler) identitySnapshotResolverRef.current = null;
            };
          },
          { signal: AbortSignal.any([identityWaitAbort.signal, AbortSignal.timeout(2000)]) },
        );
      } catch {
        // 📇️ Either the 2s grace period elapsed (no persisted identity — proceed with `null`, same as
        // the old fixed-timeout fallback) or the component unmounted mid-wait (`cancelled` below is
        // what actually stops further work in that case).
        cachedIdentity = null;
      }
      if (cancelled || identityClientInstanceIdRef.current !== clientInstanceId) return;
      const broker = localBrowserBrokerRef.current;
      if (!broker) {
        setIdentityOffline(true);
        return;
      }
      const ownerIsCurrent = (): boolean => !cancelled && identityClientInstanceIdRef.current === clientInstanceId;
      sessionRefreshRef.current = startDirectorySessionRefreshV1({
        signal: identityWaitAbort.signal,
        read: (signal) => broker.me(signal),
        onAuthority: (authority) => {
          if (!ownerIsCurrent()) return;
          const previous = verifiedSessionAuthorityRef.current;
          const authorityChanged = previous === null || previous.sessionBindingSha256 !== authority.sessionBindingSha256 || previous.authorizationGeneration !== authority.authorizationGeneration;
          if (authorityChanged) retireAuthenticatedInferencePresentation();
          verifiedSessionAuthorityRef.current = authority;
          if (authorityChanged) setVerifiedSessionAuthority(authority);
          setIdentityOffline(false);
          const resolved: Identity = { userId: authority.userId, email: authority.email, displayName: authority.displayName, hubBaseUrl: hubEnv.hubBaseUrl, issuedAtMs: cachedIdentity?.userId === authority.userId ? cachedIdentity.issuedAtMs : Date.now() };
          shellActorIdRef.current = shellActorId(shellSessionIdRef.current, resolved);
          setPluginRuntimeActor(shellActorIdRef.current);
          const unchanged = cachedIdentity?.userId === resolved.userId && cachedIdentity.email === resolved.email && cachedIdentity.displayName === resolved.displayName && cachedIdentity.hubBaseUrl === resolved.hubBaseUrl;
          if (unchanged && !authorityChanged) return;
          setIdentity(resolved);
          const envelope = identityMutationEnvelope(shellActorIdRef.current, signIn(resolved), cachedIdentity);
          cachedIdentity = resolved;
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localMutations", envelopes: [envelope] } }) });
          const archive = encodeDocumentArchiveBytes({ parent_pack: Array.from(encodePackValue(resolved)), parent_spr: [], members: [] });
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localDocumentArchive", archive: Array.from(archive) } }) });
        },
        onUnavailable: () => {
          if (!ownerIsCurrent()) return;
          retireAuthenticatedInferencePresentation();
          verifiedSessionAuthorityRef.current = null;
          setVerifiedSessionAuthority(null);
          setIdentityOffline(true);
          shellActorIdRef.current = shellActorId(shellSessionIdRef.current, null);
          setPluginRuntimeActor(shellActorIdRef.current);
        },
      });
    })().catch((error) => console.error("[os-shell] identity bootstrap failed unexpectedly", error));
    return () => {
      cancelled = true;
      identityWaitAbort.abort();
      if (identityBootstrapAbortRef.current === identityWaitAbort) identityBootstrapAbortRef.current = null;
      sessionRefreshRef.current?.close();
      sessionRefreshRef.current = null;
      localBrowserBrokerRef.current?.close();
      localBrowserBrokerRef.current = null;
      verifiedSessionAuthorityRef.current = null;
      setVerifiedSessionAuthority(null);
      const clientInstanceId = identityClientInstanceIdRef.current;
      if (clientInstanceId !== null) {
        backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId }) });
        if (identityClientInstanceIdRef.current === clientInstanceId) identityClientInstanceIdRef.current = null;
      }
    };
  }, [hubEnv]);

  useEffect(() => {
    if (!identity || !verifiedSessionAuthority || verifiedSessionAuthority.userId !== identity.userId || !hubEnv || !hostPlugin || !landingApp || !session || session.pluginId !== hostPlugin.handle.pluginId || session.app.id !== landingApp.id) return;
    const worker = ensureBackboneWorker();
    const bootstrapEpoch = directoryBootstrapEpochRef.current + 1;
    directoryBootstrapEpochRef.current = bootstrapEpoch;
    const openingAbort = new AbortController();
    const visibleSession = session;
    const opening = directoryHomeOpeningRef.current.catch(() => {}).then(async () => {
      if (openingAbort.signal.aborted) return;
      const current = sessionRef.current;
      if (!current || current.pluginId !== visibleSession.pluginId || current.app.id !== visibleSession.app.id || current.instanceId !== visibleSession.instanceId) return;
      const owner = await openDirectoryHomeOwnerV1({
        plugin: hostPlugin.handle,
        app: landingApp,
        identity: { userId: identity.userId, displayName: identity.displayName },
        instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState },
        baseUrl: identity.hubBaseUrl,
        bootstrapEpoch,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        signal: openingAbort.signal,
        beforeBootstrap: async (openingOwner) => {
          const active = sessionRef.current;
          if (!active || active.pluginId !== openingOwner.plugin.pluginId || active.app.id !== openingOwner.app.id || active.instanceId !== openingOwner.instanceId) throw new Error("directory-bootstrap.visible-owner-stale");
          await refreshDirectoryHomeRef.current(active);
        },
        post: (request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }),
      });
      if (openingAbort.signal.aborted || directoryBootstrapEpochRef.current !== bootstrapEpoch) {
        await retireDirectoryHomeOwner(owner, worker);
        return;
      }
      directoryHomeOwnerRef.current = owner;
      setDirectoryBootstrapUi({ kind: "idle" });
    });
    directoryHomeOpeningRef.current = opening.catch(() => {});
    void opening.catch((ownerError) => {
      if (!openingAbort.signal.aborted && directoryBootstrapEpochRef.current === bootstrapEpoch) setDirectoryBootstrapUi({ kind: "fault", code: ownerError instanceof Error ? ownerError.message : "directory-bootstrap.owner-open-failed" });
    });
    return () => {
      openingAbort.abort("directory-bootstrap-owner-replaced");
      const owner = directoryHomeOwnerRef.current;
      if (!owner || owner.bootstrapEpoch !== bootstrapEpoch) return;
      directoryHomeOwnerRef.current = null;
      void retireDirectoryHomeOwner(owner, worker);
    };
  }, [ensureBackboneWorker, hostPlugin?.handle, identity?.displayName, identity?.hubBaseUrl, identity?.userId, landingApp?.id, retireDirectoryHomeOwner, session?.app.id, session?.instanceId, session?.pluginId, verifiedSessionAuthority?.authorizationGeneration, verifiedSessionAuthority?.sessionBindingSha256]);

  useEffect(() => {
    if (typeof window === "undefined") return;
    const trackPointer = (event: PointerEvent) => {
      presenceCursorRef.current = { x: event.clientX, y: event.clientY };
    };
    window.addEventListener("pointermove", trackPointer, { passive: true });
    return () => window.removeEventListener("pointermove", trackPointer);
  }, []);

  // 🐚️ Only a page-owning studio shell syncs to the real browser URL bar/history — an embedded shell
  // sharing the page with others must not fight them over `window.history`.
  const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", hostMode && scope.ownsPage);
  const shellRoute = useMemo(() => parseShellRoute(shellUri.split("?")[0] ?? "/"), [shellUri]);

  // 🐚️ `scope.storage` (not a separately-resolved ephemeral/browser port here) — two shells sharing a
  // page must not clobber each other's panel layout/dock state through an unnamespaced localStorage key.
  const shellStorage = scope.storage;
  const namedLayoutStore = useMemo(() => new NamedLayoutStore(session?.app.id ?? "framework-os", shellStorage), [session?.app.id, shellStorage]);
  const dockLayoutStore = useMemo(() => new DockLayoutStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);
  const dockUiStateStore = useMemo(() => new DockUiStateStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);

  const registry = useMemo(() => {
    const expanded = expandPluginRegistry(plugins, pluginFilter ? resolvePluginRegistryId(PLUGIN_CATALOG, pluginFilter) : undefined, hostMode);
    if (hostMode) return expanded;
    return pluginFilter ? expanded : plugins;
  }, [pluginFilter, plugins, hostMode]);

  //#region 🔌️PluginRuntime
  /** 🔌️ The one registry entry the shell must have loaded before it can create a session — the studio
   * host plugin (`hostConfig.pluginId`) in studio mode, otherwise the resolved single-app variant.
   * Every other registry entry streams in independently and is never fatal to boot. */
  const primaryPluginId = useMemo(() => hostConfig?.pluginId ?? (pluginFilter ? resolvePluginRegistryId(PLUGIN_CATALOG, pluginFilter) : undefined) ?? registry[0]?.pluginId, [hostConfig, pluginFilter, registry]);
  const shellPluginCanvasStatus = useMemo((): UiStatus | undefined => {
    return resolvePluginCanvasStatus(!!session, error, primaryPluginId ? pluginStatusById[primaryPluginId] : undefined, primaryPluginId ? pluginSupervisorById[primaryPluginId] : undefined);
  }, [session, error, primaryPluginId, pluginStatusById, pluginSupervisorById]);
  /** 🔌️ The dev catalog and installed extensions share one
   * {@link PluginSource} so the incremental runtime can load from either tree. */
  const pluginSource: PluginSource = useMemo(() => multiplexPluginSources(createDevPluginSource(registry, `${MODULE_PLUGIN_ROUTE}/watch`), createExtensionSource(PLUGIN_CATALOG, `${MODULE_EXTENSION_ROUTE}/watch`)), [registry]);

  /** 🔌️ Recreates the primary session instance for `handle` — the exact `hostConfig`/non-studio
   * app-resolution logic the boot effect used to run once inline, now shared with `reloadPlugin` so a
   * hot-swap of the session-owning plugin re-establishes the session the same way boot does.
   *
   * 🗣️ Locale and terminology are read through their refs, and are deliberately NOT dependencies. They
   * only label the layout SEED, i.e. they matter at the instant a session is established — which is
   * exactly what a ref answers — while an already-established session's baked-in titles are kept in the
   * current language by the retitle effect below (`syncShellLabelLocale`'s neighbour). Depending on them
   * reactively made this callback's identity change on every language switch, which propagated through
   * `installPlugin`/`reloadPlugin` into the `PluginSource` subscription effect's dep array; that effect's
   * re-run opens a FRESH `EventSource`, whose connect-time `snapshot` replays every already-built plugin
   * as a hot-swap, and a hot-swap of the session-owning plugin destroys the live instance —
   * `actor-activation.revoked` followed by `no channel for instance N` for everything still addressed to
   * it (ticket 26/09/02 wave B38 §1.7, fixed in wave B40). A UI-language change is a view-context
   * update; it must never reach the plugin install graph. */
  const establishPrimarySession = useCallback(
    async (handle: PluginWasmHandle) => {
      const manifest = handle.manifest;
      if (hostConfig) {
        const sApp = resolveRequiredHostApps(manifest.apps, hostConfig).landing;
        const panelState = buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
        const instanceId = await handle.createApp(sApp.id);
        const viewState: ViewModel = { activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id, panelJson: panelJsonFromState(panelState) };
        // 🪟️ Seed default-layout panes (Top/Perspective) before any effect can fire actions — otherwise
        // boot `setActiveExample` races the session-switch refresh and wipes pane bodies.
        const seeded = applyFrameworkLayoutSeed(sApp.defaultLayout, withLocalizedWindowKindLabels(sApp.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminologyRef.current, uiLocaleRef.current);
        extraWindowInstancesRef.current = seeded.extraInstances;
        extraWindowCounterRef.current = seeded.extraInstances.length;
        dispatch({ type: "SET_SESSION", value: { pluginId: handle.pluginId, instanceId, app: sApp, viewState } });
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
        dispatch({ type: "SET_ERROR", value: null });
        return;
      }
      // 👁️✏️ Boot-time role (contract freeze §5) resolved against the app-id axis by
      // `resolveBootPrimaryAppV1` — the role of an OPEN session always comes from `session.app.role`,
      // never `appRole` itself. `appId`/`defaultAppId` name the artifact surface; `appRole` picks
      // which of that dialect's surfaces opens.
      const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(PLUGIN_CATALOG, pluginFilter) : undefined;
      const primaryApp = resolveBootPrimaryAppV1(manifest.apps, appId, defaultAppId, appRole);
      if (appId !== undefined && primaryApp === undefined) {
        // 🔐️ A pinned app belongs to the selected primary aggregate, never a racing dependency.
        throw new Error(`primary plugin ${handle.pluginId} does not declare pinned app ${appId}`);
      }
      if (!primaryApp) return;
      const instanceId = await handle.createApp(primaryApp.id);
      const seeded = applyFrameworkLayoutSeed(primaryApp.defaultLayout, withLocalizedWindowKindLabels(primaryApp.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminologyRef.current, uiLocaleRef.current);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({
        type: "SET_SESSION",
        value: { pluginId: handle.pluginId, instanceId, app: primaryApp, viewState: { activeModeId: primaryApp.defaultModeId ?? primaryApp.modes[0]?.id } },
      });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      dispatch({ type: "SET_ERROR", value: null });
    },
    [hostConfig, hostApp, appId, appRole, pluginFilter],
  );

  /** 🚑️ ONE watchdog kill must not be a fatal boot. Losing the shard that was running the primary
   * plugin's first turn is a recoverable event by construction — `ShardClient.rebuild` has already
   * replaced the worker and `ActivationRegistry.handleShardLost` has already bumped and resumed every
   * actor that was on it — so the session is established once more against the rebuilt shard before
   * anything is reported. Only a SECOND loss is real, and it fails as a typed
   * {@link PluginBootShardLostError} (`plugin.boot.shard-lost`) that names the cause instead of
   * surfacing a bare `shard 0 terminated`. Every other boot error propagates untouched and unretried. */
  const establishPrimaryWithShardRetry = useCallback(
    async (handle: PluginWasmHandle): Promise<void> => {
      try {
        await establishPrimarySession(handle);
      } catch (bootError) {
        if (!isShardLostError(bootError)) throw bootError;
        console.warn(`[DEBUG] ShellHost: primary ${handle.pluginId} lost its shard while booting — retrying once on the rebuilt shard`, bootError);
        try {
          await establishPrimarySession(handle);
        } catch (retryError) {
          if (!isShardLostError(retryError)) throw retryError;
          throw new PluginBootShardLostError(handle.pluginId, retryError);
        }
      }
    },
    [establishPrimarySession],
  );

  /** 🚑️ A LIVE session whose worker was taken down is recoverable exactly the way a killed boot is:
   * `ShardClient.rebuild` has already replaced the worker and `PluginRuntime` has already retired the
   * dead instance's bookkeeping (see `onPluginInstancesLost`), so a fresh `createApp` on the rebuilt
   * shard re-opens the document and the session continues.
   *
   * Without this the shard restore brought the ACTOR back and nothing else: the guest returned empty,
   * every later command page failed its liveness check with `plugin.command-page-invalid`, and the
   * session was dead until the user reloaded — measured on 2026-09-12 after a 16 s `brep.bool.cut`
   * tripped the watchdog (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
   * `📓️extension-evaluate-budget-2026-09-12.md`).
   *
   * If the rebuild ALSO fails there is nothing left to try in-page, so the shell says so in the
   * user's own language rather than leaving a session that answers nothing. */
  useEffect(() => {
    return onPluginInstancesLost((_shardIndex, lost) => {
      const session = sessionRef.current;
      if (!session) return;
      const ours = lost.find((entry) => entry.pluginId === session.pluginId && entry.instanceId === session.instanceId);
      if (!ours) return;
      const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === ours.pluginId)?.handle;
      if (!handle) {
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.workerLost") });
        return;
      }
      void establishPrimaryWithShardRetry(handle).catch((error: unknown) => {
        console.error(`ShellHost: could not re-establish ${ours.pluginId} after a worker loss`, error);
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.workerLost") });
      });
    });
  }, [establishPrimaryWithShardRetry]);

  /** 🔌️ Installs a registry entry that isn't loaded yet: acquires its module (worker-backed, refcounted
   * — see `acquirePluginModule`), upserts it into `loadedPlugins`, and — if this is the primary plugin
   * and no session exists yet — establishes the session. Shared by the boot effect (primary plugin
   * only) and the `PluginSource` subscription effect (every other plugin, as its build lands). */
  const installPlugin = useCallback(
    async (pluginId: string, rebuiltAt?: number): Promise<PluginInstallOutcome> => {
      if (pluginOpInFlightRef.current.has(pluginId)) return "in-flight";
      if (loadedPluginsRef.current.some((entry) => entry.handle.pluginId === pluginId)) return "already-loaded";
      const entry = registry.find((candidate) => candidate.pluginId === pluginId);
      if (!entry) return "missing-registry";
      pluginOpInFlightRef.current.add(pluginId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "installing" });
      try {
        const moduleUrl = pluginSource.moduleUrl(pluginId, rebuiltAt);
        const handle = await loadPluginModuleResilient(pluginId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "failed" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
          return "failed";
        }
        pluginModuleUrlByIdRef.current.set(pluginId, moduleUrl);
        recordPluginArtifactRebuiltAt(pluginId, rebuiltAt);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "loaded" });
        // 🔐️ Only the configured primary may own the session; dependencies can expose the same
        // app ids and arrive first because registry installation is intentionally concurrent.
        const shouldEstablish = pluginShouldEstablishSession(pluginId, primaryPluginId, sessionRef.current !== null);
        if (shouldEstablish) {
          try {
            await establishPrimaryWithShardRetry(handle);
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "running" });
          } catch (bootError) {
            console.error("Framework OS boot failed", bootError);
            try {
              const parsed = JSON.parse(bootError instanceof Error ? bootError.message : String(bootError));
              const bytes = parsed?.val && typeof parsed.val === "object" ? Object.values(parsed.val as Record<string, number>) : undefined;
              if (bytes) console.error("[DEBUG] boot fault text", new TextDecoder().decode(Uint8Array.from(bytes)));
            } catch {}
            dispatch({ type: "SET_ERROR", value: bootError instanceof Error ? bootError.message : String(bootError) });
            return "failed";
          }
        }
        return "loaded";
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [registry, pluginSource, primaryPluginId, establishPrimaryWithShardRetry, appId, recordPluginArtifactRebuiltAt],
  );

  /** 🔌️ Hot-swaps an already-loaded plugin to a newly built module — mirrors the os-core kernel's
   * `PluginHost::hot_swap_plugin` contract (validate → destroy affected instances → swap → recreate the
   * session if it was this plugin's → release the old module) without inventing a separate one:
   * acquires the new module BEFORE tearing anything down (the old handle keeps serving concurrent
   * traffic during the swap), validates that a session-owning plugin still declares the session's app
   * id, then only commits. Extension-only programs intentionally declare no apps. A validation failure
   * disposes the new lease and leaves the old plugin exactly as it was — nothing destroyed, status back
   * to `"loaded"`. */
  const reloadPlugin = useCallback(
    async (pluginId: string, rebuiltAt?: number) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return installPlugin(pluginId, rebuiltAt);
      pluginOpInFlightRef.current.add(pluginId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "reloading" });
      dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
      let newHandle: PluginWasmHandle | null = null;
      let committed = false;
      let ownsSession = false;
      try {
        const moduleUrl = pluginSource.moduleUrl(pluginId, rebuiltAt);
        newHandle = await loadPluginModuleResilient(pluginId, moduleUrl);
        if (!newHandle) throw new Error(`program ${pluginId} failed to reload`);
        const activeSession = sessionRef.current;
        ownsSession = activeSession?.pluginId === pluginId;
        const activeAppId = ownsSession ? activeSession?.app.id : undefined;
        if (!reloadRetainsActiveApp(newHandle.manifest.apps, activeAppId)) {
          throw new Error(`program ${pluginId} reload dropped the active session's app "${activeAppId}"`);
        }

        const oldAppIds = new Set(current.manifest.apps.map((app) => app.id));
        const newAppIds = new Set(newHandle.manifest.apps.map((app) => app.id));
        const hotSwapEvent: ProgramHotSwapEvent = {
          pluginId,
          version: newHandle.manifest.version,
          addedApps: [...newAppIds].filter((id) => !oldAppIds.has(id)),
          removedApps: [...oldAppIds].filter((id) => !newAppIds.has(id)),
        };
        console.log(`[DEBUG] hot-swap ${pluginId}`, hotSwapEvent);

        const directoryHomeOwner = directoryHomeOwnerRef.current;
        if (directoryHomeOwner?.plugin.pluginId === pluginId) {
          directoryHomeOwnerRef.current = null;
          const worker = backboneWorkerRef.current;
          await retireDirectoryHomeOwner(directoryHomeOwner, worker);
        }

        // 🪦️ Destroy this plugin's live instances under the OLD handle before swapping — the primary
        // session instance (if owned), every studio-spawned instance, and any external-slot contributor
        // instance. Mirrors the shell-unmount teardown effect, scoped to one pluginId instead of every
        // loaded plugin.
        if (ownsSession && activeSession) {
          await current.handle.destroyApp(activeSession.instanceId).catch(() => {});
        }
        for (const spawned of spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {});
        }
        const contributorInstanceId = contributorInstancesRef.current.get(pluginId);
        if (contributorInstanceId != null) {
          await current.handle.destroyApp(contributorInstanceId).catch(() => {});
          contributorInstancesRef.current.delete(pluginId);
        }
        if (hostMode && activeSession) {
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            console.log(
              `[DEBUG] hot-swap ${pluginId} dropped ${dropped.length} spawned instance(s)`,
              dropped.map((entry) => entry.id),
            );
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? undefined : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => (nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession),
            });
          }
        }

        pluginModuleUrlByIdRef.current.set(pluginId, moduleUrl);
        recordPluginArtifactRebuiltAt(pluginId, rebuiltAt);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle: newHandle, manifest: newHandle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });
        committed = true;

        if (ownsSession) await establishPrimarySession(newHandle);

        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H1-react) — `evictPluginModule`'s refcounted
        // module-URL lease pool is gone (packet H2's "must not exist" list); disposing the OLD handle
        // already tears down every actor it ever activated via `ShardClient.dispose`, which is the
        // real replacement — there is no separate shared-module resource left to evict.
        await current.handle.dispose();
      } catch (error) {
        // 🚧️ Past the commit point there is nothing to roll BACK to: the store already names the new
        // handle and the session already runs on it, so disposing it here revoked the activation the
        // shell had just adopted and every later call answered `plugin-handle.closed`. A predecessor
        // that refuses to retire (`plugin-handle.retirement-failed`, raised by a close ladder that
        // faulted) is a leak to report, never a reason to destroy the successor
        // (ticket 26/09/02 wave B17).
        if (committed) {
          console.warn(`hot-swap ${pluginId} retained its committed handle; predecessor retirement failed`, error);
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });
        } else {
          console.warn(`[DEBUG] hot-swap rolled back for ${pluginId}`, error);
          await newHandle?.dispose();
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
        }
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [installPlugin, establishPrimarySession, hostMode, pluginSource, retireDirectoryHomeOwner, recordPluginArtifactRebuiltAt],
  );

  /** 🔌️ Removes an already-loaded plugin: refuses the host/primary plugin and whichever plugin owns the
   * active session (there is nothing to fall back to), otherwise destroys its live instances the same
   * way `reloadPlugin` does, drops it from `loadedPlugins`, and evicts its module lease immediately
   * (rather than the pool's normal 30s linger — freeing it right away is the point of an explicit
   * uninstall). */
  const uninstallPlugin = useCallback(
    async (pluginId: string) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return;
      if (pluginId === primaryPluginId) {
        console.warn(`[DEBUG] refusing to uninstall the host/primary plugin: ${pluginId}`);
        return;
      }
      if (sessionRef.current?.pluginId === pluginId) {
        console.warn(`[DEBUG] refusing to uninstall the active session's plugin: ${pluginId}`);
        return;
      }
      pluginOpInFlightRef.current.add(pluginId);
      try {
        for (const spawned of spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {});
        }
        const contributorInstanceId = contributorInstancesRef.current.get(pluginId);
        if (contributorInstanceId != null) {
          await current.handle.destroyApp(contributorInstanceId).catch(() => {});
          contributorInstancesRef.current.delete(pluginId);
        }
        if (hostMode && sessionRef.current) {
          const activeSession = sessionRef.current;
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? undefined : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => (nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession),
            });
          }
        }
        dispatch({ type: "REMOVE_LOADED_PLUGIN", pluginId });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "available" });
        await current.handle.dispose();
        pluginModuleUrlByIdRef.current.delete(pluginId);
        pluginArtifactRebuiltAtRef.current.delete(pluginId);
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [primaryPluginId, hostMode],
  );
  /** 🧩️ Durable-in-session extension ledger mirror — space document ops are dispatched
   * best-effort when a space/studio session can accept them; this state keeps Settings + contribution
   * filtering coherent even when the space app is not the active session. */
  type ExtensionLedgerEntry = {
    readonly extensionId: string;
    readonly version: string;
    readonly sourceUri: string;
    readonly packageHash: string;
    readonly enabled: boolean;
    readonly extendsHost: string;
  };
  const [extensionLedger, setExtensionLedger] = useState<readonly ExtensionLedgerEntry[]>([]);
  const extensionLedgerRef = useRef(extensionLedger);
  extensionLedgerRef.current = extensionLedger;
  const extensionTargetById = useMemo(() => new Map(EXTENSION_TARGETS.map((target) => [target.pluginId, target] as const)), []);
  const extensionIdSet = useMemo(() => new Set(EXTENSION_TARGETS.map((target) => target.pluginId)), []);

  const dispatchSpaceExtensionOp = useCallback(async (action: string, args: Record<string, unknown>) => {
    const active = sessionRef.current;
    if (!active) return;
    const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === active.pluginId);
    if (!pluginEntry) return;
    try {
      const viewState = panelViewContext({
        ...active.viewState,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        windowInstances: sessionWindowInstances(active.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
      });
      const wire = encodeWindowActionInvocation({ ...active, viewState }, { controllerId: active.app.controllerId, action, args }, extraWindowInstancesRef.current);
      await pluginEntry.handle.handleAction(active.instanceId, wire, viewState);
      console.log("[DEBUG] space extension ledger op dispatched", { action, args });
    } catch (error) {
      console.warn("[DEBUG] space extension ledger op skipped", action, error instanceof Error ? error.message : String(error));
    }
  }, []);

  /** 🧩️ Installs an extension package from a URL via the extension-store HTTP endpoint when
   * available, loads its module into `loadedPlugins`, and records it on the durable ledger. */
  const installExtension = useCallback(
    async (sourceUri: string) => {
      let extensionId = "";
      let version = "0.0.0";
      let moduleUrl = "";
      let packageHash = "";
      try {
        const response = await fetch(`${MODULE_EXTENSION_ROUTE}/install`, {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ url: sourceUri }),
          signal: extensionFetchAbortRef.current.signal,
        });
        if (!response.ok) {
          const body = await response.text();
          throw new Error(`extension store install failed (${response.status}): ${body}`);
        }
        const result = (await response.json()) as { extensionId: string; version: string; moduleUrl: string; packageHash?: string };
        extensionId = result.extensionId;
        version = result.version;
        moduleUrl = result.moduleUrl;
        packageHash = result.packageHash ?? "";
        console.log("[DEBUG] extension store install ok", result);
      } catch (error) {
        console.warn("[DEBUG] extension store unavailable or install failed; falling back to catalog id heuristic", error instanceof Error ? error.message : String(error));
        const guessedId = sourceUri.split("/").filter(Boolean).pop()?.replace(/\.sxt$/i, "") ?? "";
        if (!guessedId) return;
        extensionId = guessedId;
        try {
          moduleUrl = pluginSource.moduleUrl(extensionId);
        } catch (resolveError) {
          console.warn("[DEBUG] installExtension could not resolve moduleUrl", resolveError);
          return;
        }
      }
      if (!extensionId || !moduleUrl) return;
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "installing" });
      try {
        const handle = await loadPluginModuleResilient(extensionId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "failed" });
          return;
        }
        pluginModuleUrlByIdRef.current.set(extensionId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: extensionId, value: "loaded" });
        const extendsHost = extensionTargetById.get(extensionId)?.extends ?? "unscoped";
        const entry: ExtensionLedgerEntry = {
          extensionId,
          version: handle.manifest.version || version,
          sourceUri,
          packageHash,
          enabled: true,
          extendsHost,
        };
        setExtensionLedger((prev) => {
          const next = prev.filter((existing) => existing.extensionId !== extensionId);
          next.push(entry);
          return next;
        });
        void dispatchSpaceExtensionOp("installExtension", {
          extensionId: entry.extensionId,
          version: entry.version,
          sourceUri: entry.sourceUri,
          packageHash: entry.packageHash,
          enabled: entry.enabled,
        });
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionTargetById, pluginSource],
  );

  /** 🧩️ Installs an extension package from a local `.sxt` / `.semio` file via the extension store. */
  const installExtensionFromFile = useCallback(
    async (file: File) => {
      let extensionId = "";
      let version = "0.0.0";
      let moduleUrl = "";
      let packageHash = "";
      try {
        const bytes = await file.arrayBuffer();
        const response = await fetch(`${MODULE_EXTENSION_ROUTE}/install`, {
          method: "POST",
          headers: { "content-type": "application/octet-stream" },
          body: bytes,
          signal: extensionFetchAbortRef.current.signal,
        });
        if (!response.ok) {
          const body = await response.text();
          throw new Error(`extension store install failed (${response.status}): ${body}`);
        }
        const result = (await response.json()) as { extensionId: string; version: string; moduleUrl: string; packageHash?: string };
        extensionId = result.extensionId;
        version = result.version;
        moduleUrl = result.moduleUrl;
        packageHash = result.packageHash ?? "";
        console.log("[DEBUG] extension store install from file ok", { file: file.name, ...result });
      } catch (error) {
        console.warn("[DEBUG] installExtensionFromFile failed", error instanceof Error ? error.message : String(error));
        return;
      }
      if (!extensionId || !moduleUrl) return;
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "installing" });
      const sourceUri = `file:${file.name}`;
      try {
        const handle = await loadPluginModuleResilient(extensionId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "failed" });
          return;
        }
        pluginModuleUrlByIdRef.current.set(extensionId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: extensionId, value: "loaded" });
        const extendsHost = extensionTargetById.get(extensionId)?.extends ?? "unscoped";
        const entry: ExtensionLedgerEntry = {
          extensionId,
          version: handle.manifest.version || version,
          sourceUri,
          packageHash,
          enabled: true,
          extendsHost,
        };
        setExtensionLedger((prev) => {
          const next = prev.filter((existing) => existing.extensionId !== extensionId);
          next.push(entry);
          return next;
        });
        void dispatchSpaceExtensionOp("installExtension", {
          extensionId: entry.extensionId,
          version: entry.version,
          sourceUri: entry.sourceUri,
          packageHash: entry.packageHash,
          enabled: entry.enabled,
        });
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionTargetById],
  );

  /** 🧩️ Unloads an extension, drops it from the ledger, and best-effort asks the store / space
   * document to forget it. */
  const uninstallExtension = useCallback(
    async (extensionId: string) => {
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      try {
        const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === extensionId);
        if (current) {
          const contributorInstanceId = contributorInstancesRef.current.get(extensionId);
          if (contributorInstanceId != null) {
            await current.handle.destroyApp(contributorInstanceId).catch(() => {});
            contributorInstancesRef.current.delete(extensionId);
          }
          dispatch({ type: "REMOVE_LOADED_PLUGIN", pluginId: extensionId });
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "available" });
          await current.handle.dispose();
          pluginModuleUrlByIdRef.current.delete(extensionId);
          pluginArtifactRebuiltAtRef.current.delete(extensionId);
        }
        setExtensionLedger((prev) => prev.filter((entry) => entry.extensionId !== extensionId));
        void dispatchSpaceExtensionOp("uninstallExtension", { extensionId });
        try {
          await fetch(`${MODULE_EXTENSION_ROUTE}/install?extensionId=${encodeURIComponent(extensionId)}`, { method: "DELETE", signal: extensionFetchAbortRef.current.signal });
        } catch {
          /* store may not expose DELETE yet */
        }
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp],
  );

  /** 🧩️ Toggles whether an installed extension's contributions are pushed to host plugins. */
  const setExtensionEnabled = useCallback(
    async (extensionId: string, enabled: boolean) => {
      setExtensionLedger((prev) => {
        const existing = prev.find((entry) => entry.extensionId === extensionId);
        if (existing) {
          return prev.map((entry) => (entry.extensionId === extensionId ? { ...entry, enabled } : entry));
        }
        const target = extensionTargetById.get(extensionId);
        return [
          ...prev,
          {
            extensionId,
            version: "0.0.0",
            sourceUri: "",
            packageHash: "",
            enabled,
            extendsHost: target?.extends ?? "unscoped",
          },
        ];
      });
      void dispatchSpaceExtensionOp("setExtensionEnabled", { extensionId, enabled });
      console.log("[DEBUG] setExtensionEnabled", { extensionId, enabled });
    },
    [dispatchSpaceExtensionOp, extensionTargetById],
  );

  //#endregion 🔌️PluginRuntime

  // 🐢️ Memoized on the raw `panelJson` string (not `session` object identity, which churns every
  // action) so a `session` refresh that leaves `panelJson` untouched reuses the same parsed `panel`
  // object — a prerequisite for any downstream `useMemo`/`React.memo` keyed on `panel` to bail.
  const panel = useMemo(() => (session ? parsePanelState(session.viewState) : null), [session?.viewState.panelJson]);
  /** 🐚️ Mirrors `panel?.spawnedApps` for the unmount-cleanup effect below — same rationale as
   * `loadedPluginsRef`: needs the latest value at teardown time without depending on it. */
  const spawnedAppsRef = useRef<readonly SpawnedAppEntry[]>([]);
  spawnedAppsRef.current = panel?.spawnedApps ?? [];
  const activeSpawnedEntry = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
  const activeAppTitle = appBreadcrumb(activeSpawnedEntry ? resolveArtifactByAppId(loadedPlugins, activeSpawnedEntry.appId, activeSpawnedEntry.breadcrumb, uiTerminology) : session ? resolveAppBreadcrumb(session.app, uiTerminology) : []);

  useEffect(() => {
    sessionRef.current = session;
  }, [session]);

  // 🎓️ A brand-owned introduction fully replaces the app's own (already localized, rendered verbatim);
  // its first-run-seen flag is brand-scoped so the branded tour plays even on a device that saw the
  // unbranded one. Brands with `replayIntroductionOnLoad` skip persistence and auto-start every load.
  const activeIntroduction = brand?.introduction ?? session?.app.introduction;
  const introductionSeenKey = session ? (brand ? `${brand.id}:${session.app.id}` : session.app.id) : "";
  const replayIntroductionOnLoad = shouldReplayIntroductionOnLoad(brand);
  const persistIntroductionSeen = shouldPersistIntroductionSeen(brand);
  const activeIntroductionRef = useRef(activeIntroduction);
  activeIntroductionRef.current = activeIntroduction;
  /** 🎓️ App ids whose tour this shell session has already offered AND had answered (Skip or Done) — the
   * session-durable half of {@link shouldAutoStartIntroduction}, see its own reasoning for why the
   * device-local seen flag alone cannot carry this. Lives for the shell mount, exactly the lifetime of
   * "this session", so a `replayIntroductionOnLoad` brand still replays on the next load. */
  const dismissedIntroductionAppIdsRef = useRef<Set<string>>(new Set());

  // 🎓️ Auto-starts an app's introduction the first time it launches on this device (or every load when
  // the brand opts in); replaying stays available afterward via the shell-owned Introduce App command.
  // 🎥️ Never auto-starts while a tutorial is active (mutual exclusivity) — `activeTutorialId` is declared
  // just below (the TutorialOrchestration block's state resolution); read via `shellState.tutorial`
  // directly here rather than the not-yet-declared local to avoid a definition-order dependency.
  useEffect(() => {
    if (!session) return;
    if (typeof window !== "undefined" && window.self !== window.top) return;
    // Embedded multi-shell hosts (demonstrator grid) pass suppressAutoIntroduction while a pane is
    // backgrounded. That must both block auto-start AND tear down an already-running tour — otherwise the
    // unfocused shell keeps mounting UIIntroduction (veil/hotkeys/ghost cursor) and steals step chrome
    // from the focused pane.
    if (suppressAutoIntroduction) {
      dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
      return;
    }
    const armable = shouldAutoStartIntroduction({
      appId: session.app.id,
      hasIntroduction: activeIntroduction != null,
      tutorialActive: shellState.tutorial.activeTutorialId != null,
      suppressed: suppressAutoIntroduction,
      replayOnLoad: replayIntroductionOnLoad,
      seenOnDevice: readStoredIntroductionSeen(scope.storage, introductionSeenKey),
      dismissedAppIds: dismissedIntroductionAppIdsRef.current,
    });
    if (!armable) return;
    dispatch({ type: "AUTO_START_INTRODUCTION", key: introductionSeenKey });
  }, [session?.app.id, activeIntroduction, introductionSeenKey, replayIntroductionOnLoad, shellState.tutorial.activeTutorialId, suppressAutoIntroduction]);

  // 🎥️ Zero per-app work: any app/brand that declares `tutorials` gets shell support automatically.
  // Brand-owned tutorials are shown ALONGSIDE the app's own (never replacing them, unlike `introduction`).
  const activeTutorials = useMemo((): readonly TutorialDefinition[] => [...(brand?.tutorials ?? []), ...(session?.app.tutorials ?? [])], [brand?.tutorials, session?.app.tutorials]);
  /** ⏺️ The recorder is dev/studio-only — Vite always defines `import.meta.env.DEV`; guarded for non-Vite (e.g. `bun test`) evaluation. */
  const tutorialRecorderAvailable = useMemo(() => {
    try {
      return Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      return false;
    }
  }, []);

  // 🧰️ Refs so `refreshUi`/`onAction`/`applyHostEffects` can read the current host-owned active utility and
  // active window without re-creating those callbacks on every utility switch.
  const activeUtilityByWindowIdRef = useRef(activeUtilityByWindowId);
  const lastUtilityArmAtRef = useRef(0);
  activeUtilityByWindowIdRef.current = activeUtilityByWindowId;
  const activeToolIdRef = useRef(activeToolId);
  activeToolIdRef.current = activeToolId;
  /** 🧰️ Dispatch + sync the ref immediately — `refreshUi` reads the ref before the next render, so a
   * bare `dispatch(SET_ACTIVE_UTILITY)` alone leaves the map stale and the gumball never appears. */
  // 🧰️ The ONE place one window's arm changes — the utility-bar action AND a program's own
  // `setActiveUtility` effect (an engagement verb's `brush`) both come through here. The pane's
  // leftover overlay is published from here too, so an arm the guest raised itself is not masked by
  // the pane's stale `select` overlay until some later refresh
  // (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B39).
  const setActiveUtilityForWindow = useCallback((windowId: string, utilityId: string | null) => {
    activeUtilityByWindowIdRef.current = { ...activeUtilityByWindowIdRef.current, [windowId]: utilityId };
    dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId });
    const prior = leftoverWorldWindowOverlayV1(windowId);
    publishLeftoverWorldSelectionV1(
      {
        ids: prior?.ids ?? [],
        hoveredId: prior?.hoveredId ?? null,
        hoveredDomain: prior?.hoveredDomain,
        gumballActive: prior?.gumballActive ?? false,
        gumballAnchorId: prior?.gumballAnchorId ?? null,
        activeUtility: utilityId ?? "select",
        activeToolId: utilityId ? null : prior?.activeToolId ?? null,
      },
      { kind: "window", windowId },
    );
  }, []);
  /** 🧰️ Clear every window's utility in the ref + store at once (tool/utility mutual exclusion). */
  const clearAllWindowUtilities = useCallback(() => {
    const next: Record<string, string | null> = { ...activeUtilityByWindowIdRef.current };
    for (const windowId of Object.keys(next)) {
      if (next[windowId]) {
        next[windowId] = null;
        dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId: null });
      }
    }
    activeUtilityByWindowIdRef.current = next;
  }, []);
  const toolMeasuresByToolIdRef = useRef(toolMeasuresByToolId);
  toolMeasuresByToolIdRef.current = toolMeasuresByToolId;
  const activeWindowIdRef = useRef(activeWindowId);
  activeWindowIdRef.current = activeWindowId;
  const actionPaneExpandedByWindowIdRef = useRef(actionPaneExpandedByWindowId);
  actionPaneExpandedByWindowIdRef.current = actionPaneExpandedByWindowId;
  const actionPaneStagedArgsByKeyRef = useRef(actionPaneStagedArgsByKey);
  actionPaneStagedArgsByKeyRef.current = actionPaneStagedArgsByKey;
  const introductionStepIndexRef = useRef(introductionStepIndex);
  introductionStepIndexRef.current = introductionStepIndex;
  const introductionCompletedInteractionsRef = useRef(introductionCompletedInteractions);
  introductionCompletedInteractionsRef.current = introductionCompletedInteractions;

  // 🎥️ Forward-declared refs so `onAction` (defined below, before the full tutorial orchestration further
  // down this component) can shell-intercept `START_TUTORIAL_ACTION_ID`/`RECORD_TUTORIAL_ACTION_ID`
  // without a definition-order cycle — mirrors the `onActionRef` pattern used the other way around.
  // Populated by the TutorialOrchestration block's effect once the real callbacks exist.
  const startTutorialRef = useRef<(tutorialId: string) => void>(() => {});
  const stopTutorialRef = useRef<() => void>(() => {});
  const toggleTutorialRecordingRef = useRef<() => void>(() => {});
  /** 🧲️ True for the duration of any director/seek/converge-driven dispatch — `onAction`'s deviation
   * check below skips setting `deviated`/auto-pausing for anything stamped while this is true, mirroring
   * how the introduction mechanism's own interception distinguishes shell-originated from user-originated
   * activity. Never read during render, only inside event callbacks — a plain mutable ref is correct. */
  const tutorialDrivenRef = useRef(new TutorialDriveV1());
  const tutorialPlayingRef = useRef(tutorialPlaying);
  tutorialPlayingRef.current = tutorialPlaying;
  const tutorialRecordingRef = useRef(tutorialRecording);
  tutorialRecordingRef.current = tutorialRecording;
  /** ⏺️ Non-null while armed — mutated by `toggleTutorialRecording` (defined in the TutorialOrchestration block below), read/appended-to by `onAction`'s recorder tap right below. */
  const tutorialRecorderRef = useRef<TutorialRecorder | null>(null);
  const tutorialRecorderStartingRef = useRef(false);
  const shellStateRef = useRef(shellState);
  shellStateRef.current = shellState;
  useEffect(() => {
    if (overlayDialog !== null && !isCurrentDialogOrigin(overlayDialog.origin)) closeOwnedDialog(overlayDialog.openingId);
  }, [overlayDialog, session, closeOwnedDialog, isCurrentDialogOrigin]);
  const publishLocalInteraction = useCallback(async (plugin: PluginWasmHandle, instanceId: number): Promise<LocalInteractionState | null> => {
    const capture = await plugin.readLocalInteraction(instanceId);
    const active = sessionRef.current;
    const currentPlugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === active?.pluginId)?.handle;
    if (!active || active.instanceId !== instanceId || currentPlugin !== plugin) return null;
    dispatch({ type: "INTERACTION_STATE_OBSERVED", state: { ...capture.state, hover: shellStateRef.current.interaction.hover } });
    return capture.state;
  }, []);
  const observeLocalInteraction = useMemo(
    () => createLatestAsyncDispatcher(({ plugin, instanceId }: { readonly plugin: PluginWasmHandle; readonly instanceId: number }) => publishLocalInteraction(plugin, instanceId).catch((error) => console.error("[DEBUG] local interaction observation failed", error))),
    [publishLocalInteraction],
  );
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (plugin) observeLocalInteraction({ plugin, instanceId: session.instanceId });
  }, [loadedPlugins, observeLocalInteraction, session?.instanceId, session?.pluginId]);

  /** 🎓️ Ends the active introduction — records the answer for this shell session (see
   * {@link dismissedIntroductionAppIdsRef}), persists the device-local seen flag when configured, and on
   * successful completion (Done / last interaction) fires the tour-finale {@link celebrateAllElements}
   * stamp across every mounted UI element. Skip/escape passes `completed: false` and does not celebrate. */
  const dismissIntroduction = useCallback(
    (completed: boolean) => {
      if (completed && scope.rootRef.current) celebrateAllElements(CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      const appId = sessionRef.current?.app.id;
      if (appId) dismissedIntroductionAppIdsRef.current.add(appId);
      dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
      if (persistIntroductionSeen) writeStoredIntroductionSeen(scope.storage, introductionSeenKey);
    },
    [introductionSeenKey, persistIntroductionSeen],
  );

  /** 🎓️ Shared step-complete path: fires once every interaction-gated step's `interactions` are all done
   * (via `completeIntroductionInteraction` below), celebrating `introduce` on top of each interaction's
   * own celebration, then advances or finishes the tour. Finishing the last step celebrates every UI
   * element via {@link dismissIntroduction}(true) instead of only the introduce target. `celebrateOverride`
   * (threaded through from `completeIntroductionInteraction`) narrows this to the one element responsible
   * for the just-completed interaction — e.g. the specific 3D window pane that was orbited — instead of
   * every element aliased to the step's `introduce` kind (every open pane of that window kind). */
  const advanceIntroductionByDoing = useCallback(
    (celebrateOverride?: string) => {
      const stepIndex = introductionStepIndexRef.current;
      const introduction = activeIntroductionRef.current;
      if (stepIndex == null || !introduction) return;
      const step = introduction.steps[stepIndex];
      if (stepIndex >= introduction.steps.length - 1) {
        dismissIntroduction(true);
        return;
      }
      const celebrateId = celebrateOverride ?? step?.introduce;
      if (step && (step.interactions ?? []).length > 0 && celebrateId && scope.rootRef.current) celebrateElements(elementIdSelector(celebrateId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      dispatch({ type: "SET_INTRODUCTION_STEP", value: stepIndex + 1 });
    },
    [dismissIntroduction],
  );

  /** ✅️ Completes the first not-yet-done interaction of the active step matching `matches` (respecting
   * `step.ordered` — only the next in-order interaction may complete), celebrates its target element, and
   * advances the step once every interaction is done. Mirrors the wgpu shell's
   * `chrome_tour_complete_interaction`. `celebrateOverride` — passed by callers that know exactly which
   * DOM element caused the completion (e.g. the gesture intercept knows the one window pane that was
   * actually orbited) — takes precedence over `interaction.celebrate ?? step.introduce`. Without it, a
   * window-kind `introduce`/`celebrate` id would celebrate every pane aliased to that kind, not just the
   * one that completed the interaction. */
  const completeIntroductionInteraction = useCallback(
    (matches: (interaction: IntroductionInteraction) => boolean, celebrateOverride?: string) => {
      const stepIndex = introductionStepIndexRef.current;
      const introduction = activeIntroductionRef.current;
      if (stepIndex == null || !introduction) return;
      const step = introduction.steps[stepIndex];
      if (!step || (step.interactions ?? []).length === 0) return;
      const completed = introductionCompletedInteractionsRef.current;
      const interactions = step.interactions ?? [];
      const index = interactions.findIndex((interaction, i) => !completed.includes(i) && matches(interaction));
      if (index < 0) return;
      if (step.ordered && index !== completed.length) return;
      const celebrateId = celebrateOverride ?? interactions[index].celebrate ?? step.introduce;
      if (celebrateId && scope.rootRef.current) celebrateElements(elementIdSelector(celebrateId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      introductionCompletedInteractionsRef.current = [...completed, index];
      dispatch({ type: "COMPLETE_INTRODUCTION_INTERACTION", index });
      if (introductionCompletedInteractionsRef.current.length >= interactions.length) advanceIntroductionByDoing(celebrateOverride);
    },
    [advanceIntroductionByDoing],
  );
  // 🎛️ So the command-category leaves' lazily-resolved tree content (built once per resolved-commands
  // change, not per keystroke — see `buildCommandCategoryTabs`) can read the latest expand/staged-arg
  // state without becoming a `defaultDock` memo dependency, which would otherwise persist-write the dock
  // skeleton on every keystroke while staging a command argument.
  const expandedCommandIdRef = useRef(expandedCommandId);
  expandedCommandIdRef.current = expandedCommandId;
  const commandStagedArgsByCommandIdRef = useRef(commandStagedArgsByCommandId);
  commandStagedArgsByCommandIdRef.current = commandStagedArgsByCommandId;
  const pendingDocumentOpeningPublicationRef = useRef<Readonly<{ receipt: DocumentOpeningReceiptV1; plugin: PluginWasmHandle; session: ActiveSession }> | null>(null);

  /** 🛠️ Overlays the mode-level host-owned `activeToolId` onto a view state at plugin-call time —
   * mirrors `injectActiveUtility` but is windowless (a tool is scoped to the active mode, not a window). */
  const injectActiveTool = useCallback((viewState: ViewModel): ViewModel => {
    const toolId = activeToolIdRef.current ?? undefined;
    return viewState.activeToolId === toolId ? viewState : { ...viewState, activeToolId: toolId };
  }, []);

  /** 🧰️ Overlays the active window's host-owned `activeUtilityId` (and the mode's `activeToolId`) onto a view state at plugin-call time. */
  const injectActiveUtility = useCallback((viewState: ViewModel, windowId?: string | null): ViewModel => {
    const key = windowId ?? activeWindowIdRef.current;
    const utilityId = key ? (activeUtilityByWindowIdRef.current[key] ?? undefined) : undefined;
    const withUtility = viewState.activeUtilityId === utilityId ? viewState : { ...viewState, activeUtilityId: utilityId };
    return injectActiveTool(withUtility);
  }, [injectActiveTool]);

  const resolvedTargetViewState = useCallback(
    (targetSession: ActiveSession) =>
      parseResolvedPluginViewState(
        injectActiveTool({
          ...targetSession.viewState,
          locale: uiLocale,
          terminology: uiTerminology,
          sessionIdentity: identityRef.current ? { userId: identityRef.current.userId, displayName: identityRef.current.displayName } : undefined,
          windowInstances: sessionWindowInstances(targetSession.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
          activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
          activeUtilityId: undefined,
        }),
      ),
    [injectActiveTool, uiLocale, uiTerminology],
  );

  const postBrowserActorViewState = useCallback(
    (worker: Worker, entry: OpenDocumentSession, targetSession: ActiveSession = entry.session) => {
      if (entry.scope === undefined) return;
      worker.postMessage({
        wire: encodeBackboneWorkerRequest({
          kind: "browser-actor-view-state",
          clientInstanceId: entry.clientInstanceId,
          scope: entry.scope,
          viewState: resolvedTargetViewState(targetSession),
        }),
      });
    },
    [resolvedTargetViewState],
  );

  useEffect(() => {
    const pending = pendingDocumentOpeningPublicationRef.current;
    pendingDocumentOpeningPublicationRef.current = null;
    const entry = pending === null ? undefined : openDocumentSessionsRef.current.get(pending.receipt.runtimeKey);
    const retainedRuntimeKey = pending !== null
      && entry?.clientInstanceId === pending.receipt.clientInstanceId
      && entry.plugin === pending.plugin
      && shellDialogSessionIsCurrentV1(pending.session, session)
      ? pending.receipt.runtimeKey
      : null;
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: retainedRuntimeKey === null ? null : `actor://${retainedRuntimeKey}` });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [panel?.activeSpawnedId, session, hostMode]);

  useEffect(() => {
    return () => {
      const worker = backboneWorkerRef.current;
      for (const owner of spaceArtifactCreationOwnersRef.current.values()) {
        worker?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId: owner.requestId, spaceId: owner.spaceId }) });
      }
      spaceArtifactCreationOwnersRef.current.clear();
      for (const [runtimeKey, entry] of openDocumentSessionsRef.current) closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
      for (const retained of browserActorUiByRuntimeKeyRef.current.values()) retained.actions.close("browser-actor-action: Shell unmounted");
      browserActorUiByRuntimeKeyRef.current.clear();
      worker?.terminate();
      backboneWorkerRef.current = null;
    };
  }, []);

  useEffect(() => {
    return () => {
      // 🌐️ terra-web-shellhost (finding 4) — aborts any in-flight extension install fetch at real
      // component unmount; see `extensionFetchAbortRef`'s own doc a few hundred lines up.
      extensionFetchAbortRef.current.abort();
      segmentedDownloadAbortRef.current.abort(new Error("segmented-download-shell-unmounted"));
      localBrowserBrokerRef.current?.close();
      localBrowserBrokerRef.current = null;
      const retirements: Promise<unknown>[] = [];
      retirements.push(backgroundSpaceIndexSessionsRef.current.close());
      const destroyDetachedApp = async (plugin: PluginWasmHandle, instanceId: number) => {
        await documentAttachmentLanesRef.current.get(plugin)?.get(instanceId)?.drain();
        await plugin.destroyApp(instanceId);
      };
      const directoryHomeOwner = directoryHomeOwnerRef.current;
      directoryHomeOwnerRef.current = null;
      if (directoryHomeOwner) {
        const worker = backboneWorkerRef.current;
        retirements.push(retireDirectoryHomeOwner(directoryHomeOwner, worker));
      } else if (directoryHomeRetirementRef.current) {
        retirements.push(directoryHomeRetirementRef.current.promise);
      }
      const primary = sessionRef.current;
      if (primary) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === primary.pluginId)?.handle;
        if (plugin) retirements.push(destroyDetachedApp(plugin, primary.instanceId).catch(() => {}));
      }
      // 🪶️ Closes the previously-documented Wave-1 gap: studio-mode spawned apps (`panel.spawnedApps`)
      // and external-slot contributor instances (`contributorInstancesRef`) each hold a live plugin
      // instance too — leaving them running past shell unmount was pure leaked memory (see
      // REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT). Best-effort: an instance the guest already dropped,
      // or whose plugin already disposed, just rejects harmlessly via the same `.catch(() => {})`
      // pattern the primary session's own destroy already used above.
      for (const spawned of spawnedAppsRef.current) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === spawned.pluginId)?.handle;
        if (plugin) retirements.push(destroyDetachedApp(plugin, spawned.instanceId).catch(() => {}));
      }
      for (const [pluginId, instanceId] of contributorInstancesRef.current) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId)?.handle;
        if (plugin) retirements.push(destroyDetachedApp(plugin, instanceId).catch(() => {}));
      }
      contributorInstancesRef.current.clear();
      const handles = loadedPluginsRef.current.map((entry) => entry.handle);
      void Promise.allSettled(retirements).then(() => Promise.allSettled(handles.map(handle => handle.dispose()))).then(results => {
        for (const result of results) if (result.status === "rejected") console.error("[DEBUG] shell plugin retirement failed", result.reason);
      });
    };
  }, [retireDirectoryHomeOwner]);

  useEffect(() => {
    // 🐚️ Only the page-owning shell may write the browser tab title — an embedded shell (e.g. one
    // demonstrator pane) sharing the page with others must not fight them over it.
    if (!scope.ownsPage) return;
    if (brand) {
      document.title = brand.windowTitle;
    } else if (activeAppTitle) {
      document.title = activeAppTitle;
    }
  }, [activeAppTitle, brand, scope.ownsPage]);

  // 🔌️ Boot gates on the primary/host plugin ONLY — every other registry entry streams in via the
  // subscription effect below as its build lands, instead of the whole shell waiting on all ~37 crates
  // (see `buildPluginsStreaming` in the dev runner). A primary that fails to load (timeout/error) is
  // still fatal, mirroring the old `noPluginsLoaded`/"host program missing landing app" boot failures.
  useEffect(() => {
    if (!primaryPluginId) return;
    if (loadedPluginsRef.current.some((entry) => entry.handle.pluginId === primaryPluginId)) return;
    void (async () => {
      const outcome = await installPlugin(primaryPluginId);
      if (outcome === "failed") {
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.noPluginsLoaded") });
      }
    })();
  }, [primaryPluginId, installPlugin]);

  // 🔌️ Streams every registry entry in independently of boot: one connect-time `snapshot` (whatever's
  // already built, including a dev server that was already fully built before this shell mounted) plus
  // a `built` event per crate as `buildPluginsStreaming`/the folded-in watch loop finishes it. An event
  // for an already-loaded plugin routes to `reloadPlugin` (hot-swap) instead of `installPlugin`.
  // 🧮️ terra-web-shellhost (finding 2) — a cold boot's `snapshot` event can carry ~20 already-built
  // plugins at once; before this packet each one fired an unbounded, uncancellable
  // `void installPlugin(...)`, all instantiating real wasm modules through the shard pool
  // simultaneously. `pending`/`pump` below is a plain worker-pool queue (mirrors `PluginRuntime`'s own
  // `runBounded` shape): at most `pluginInstallConcurrency()` installs/reloads run at once, extras wait
  // their turn, and `installPlugin`/`reloadPlugin` are still called with a FRESH `alreadyLoaded` read
  // at dispatch time (not at enqueue time) so a plugin that finished loading while queued still routes
  // correctly. `aborted` stops handing out new work on unmount — an install already in flight (already
  // called) settles on its own, same "stop starting, let in-flight finish" contract
  // `loadPluginModulesInDependencyOrder`'s own `signal` documents.
  // 🔁️ An availability event for an artifact this shell ALREADY runs is dropped unless its `rebuiltAt`
  // is strictly newer (`pluginAvailabilityRouteV1`). A `PluginSource` streams availability, not
  // commands: `subscribe` opens a fresh `EventSource` and the dev endpoint answers every connect with a
  // full `snapshot`, so the same builds arrive again on every reconnect — and answering a replay with a
  // hot-swap destroys the session-owning plugin's live instance and its document
  // (`actor-activation.revoked`, then `no channel for instance N`; ticket 26/09/02 wave B40 §1).
  useEffect(() => {
    const registryIds = new Set(registry.map((entry) => entry.pluginId));
    let aborted = false;
    const pending: Array<{ readonly pluginId: string; readonly rebuiltAt: number }> = [];
    let activeWorkers = 0;
    const limit = pluginInstallConcurrency();

    const pump = (): void => {
      while (!aborted && activeWorkers < limit && pending.length > 0) {
        const next = pending.shift()!;
        const alreadyLoaded = loadedPluginsRef.current.some((entry) => entry.handle.pluginId === next.pluginId);
        const route = pluginAvailabilityRouteV1(alreadyLoaded, pluginArtifactRebuiltAtRef.current.get(next.pluginId), next.rebuiltAt);
        if (route === "drop") continue;
        activeWorkers += 1;
        void (route === "hot-swap" ? reloadPlugin(next.pluginId, next.rebuiltAt) : installPlugin(next.pluginId, next.rebuiltAt))
          .catch((error) => console.error("[os-shell] plugin install/reload failed", next.pluginId, error))
          .finally(() => {
            activeWorkers -= 1;
            pump();
          });
      }
    };

    const handlePluginAvailable = (pluginId: string, rebuiltAt: number) => {
      if (aborted || !registryIds.has(pluginId)) return;
      pending.push({ pluginId, rebuiltAt });
      pump();
    };
    const unsubscribe = pluginSource.subscribe((event: PluginSourceEvent) => {
      if (event.kind === "snapshot") {
        for (const plugin of event.plugins) handlePluginAvailable(plugin.pluginId, plugin.rebuiltAt);
        return;
      }
      handlePluginAvailable(event.pluginId, event.rebuiltAt);
    });
    return () => {
      aborted = true;
      pending.length = 0;
      unsubscribe();
    };
  }, [registry, pluginSource, installPlugin, reloadPlugin]);

  const requestContextMenu = useCallback(
    async (request: PluginContextMenuRequest): Promise<readonly ContextMenuItemSpec[]> => {
      if (!session) return [];
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin?.contextMenu) return [];
      const baseViewState: ViewModel = {
        ...session.viewState,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        windowInstances: sessionWindowInstances(session.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        focusedWindowId: activeWindowIdRef.current ?? undefined,
      };
      const viewState = request.windowInstanceId ? hostArmedViewContext(baseViewState, activeToolIdRef.current, request.windowInstanceId) : panelViewContext(injectActiveTool(baseViewState));
      if (!viewState) return [];
      return plugin.contextMenu(session.instanceId, request, viewState);
    },
    [injectActiveTool, loadedPlugins, session],
  );

  //#region 🧩️ContributionsPush
  /** 🧩️ What one contributions push is computed against — captured per publish, never read off a
   * render closure, so the unit a superseded refresh started still finishes against the closure it
   * was started with. */
  type ContributionsEnvironment = {
    readonly loadedPlugins: readonly LoadedProgramState[];
    readonly disabledExtensionIds: ReadonlySet<string>;
    readonly hostMode: boolean;
    readonly session: ActiveSession;
    readonly targetViewState: ViewModel;
    readonly dispatchDeferredEffects: (pluginId: string, instanceId: number, effects: readonly Effect[]) => void;
  };
  /**
   * 🧩️ The host→guest contributions push, owned OUTSIDE `refreshUi`'s generation race.
   *
   * `refreshUi` bumps `refreshGenerationRef` on every call and abandons itself whenever the
   * generation moves under one of its awaits. The push used to sit inside that guard and `await` a
   * guest document read, so a guest re-arming a faulting `flowEvalTick` (which settles into another
   * refresh) superseded every push mid-read and the closure never crossed — 45 s of live console
   * with zero `[DEBUG] contributions …` lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The unit
   * now lives in {@link createContributionsPublisher}: per `(pluginId, instanceId)`, joined on an
   * unmoved registry generation, cancelled only by `retire` (a session switch), and installed keyed
   * by `(instanceId, content)`.
   */
  const contributionsPublisherRef = useRef<ReturnType<typeof createContributionsPublisher<ContributionsEnvironment>> | null>(null);
  if (contributionsPublisherRef.current === null) {
    contributionsPublisherRef.current = createContributionsPublisher<ContributionsEnvironment>({
      // 🔢 The identity of the closure the pack is cut from: a plugin load/unload or an extension
      // toggle is a new generation and re-runs the unit; a plain refresh is not.
      registryGeneration: (environment) => `${environment.hostMode ? "host" : "focused"}|${environment.loadedPlugins.map((entry) => entry.handle.pluginId).join(",")}|${[...environment.disabledExtensionIds].sort().join(",")}`,
      resolveScope: async (session, environment): Promise<ContributionsOperatorScope> => {
        const receiverPlugin = environment.loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId);
        const fromDocument = await readDocumentOperatorScope(receiverPlugin, session.instanceId);
        if (fromDocument.status === "resolved" && fromDocument.kinds.length > 0) return fromDocument;
        // 📚️ A genesis `ReadDocument` is the NORMAL boot state — the app's published examples carry
        // the same operator kinds the first opened document will, so they scope the push instead of
        // stalling it.
        const exampleSources = exampleArtifactSources(receiverPlugin?.manifest.examples ?? [], environment.session.app.dialect);
        const fromExamples = resolveDocumentOperatorKinds(exampleSources);
        if (fromExamples.status === "resolved" && fromExamples.kinds.length > 0) {
          console.error("[DEBUG] contributions scoped from published examples", JSON.stringify({ plugin: session.pluginId, app: environment.session.app.id, kinds: fromExamples.kinds, examples: exampleSources.length }));
          return fromExamples;
        }
        if (fromDocument.status === "resolved") return fromDocument;
        console.error("[DEBUG] contributions push skipped unresolved document operators", JSON.stringify({ plugin: session.pluginId, app: environment.session.app.id, reason: fromDocument.reason }));
        return fromDocument;
      },
      buildPack: (session, kinds, environment) => {
        const loadedForScope = environment.loadedPlugins.filter((entry) => !environment.disabledExtensionIds.has(entry.handle.pluginId)).map((entry) => ({ pluginId: entry.handle.pluginId, manifest: { ...entry.manifest, workflows: [] } }));
        const scopedContributionsJson = scopeContributionsJson(loadedForScope, session.pluginId, kinds);
        if (scopedContributionsJson === "[]") console.error("[DEBUG] contributions push refused empty pack", JSON.stringify({ plugin: session.pluginId, app: environment.session.app.id, chars: 2, kinds }));
        else console.error("[DEBUG] contributions scoped pack", JSON.stringify({ chars: scopedContributionsJson.length, hasManifestJson: scopedContributionsJson.includes("manifestJson"), hasPolygon: scopedContributionsJson.includes("brep.curve.polygon"), kinds }));
        return scopedContributionsJson;
      },
      install: async (session, json, kinds, environment) => {
        for (const pluginEntry of environment.loadedPlugins) {
          if (!pluginEntry.manifest.apps?.length) continue;
          if (!pluginShouldReceiveContributions(pluginEntry.handle.pluginId, session.pluginId, environment.hostMode)) continue;
          const isActive = pluginEntry.handle.pluginId === session.pluginId;
          const targetApp = isActive ? environment.session.app : pluginEntry.manifest.apps.find((app) => appOwnsCommand(app, "setContributions"));
          const instanceId = targetApp ? (isActive ? session.instanceId : contributorInstancesRef.current.get(pluginEntry.handle.pluginId)) : undefined;
          const skipped = !targetApp || !appOwnsCommand(targetApp, "setContributions") ? "app-owns-no-setContributions" : !pluginEntry.handle.handleCommand ? "handle-cannot-command" : instanceId == null ? "no-bound-instance" : undefined;
          const takesPageRun = targetApp !== undefined && appCommandTakesPageRun(targetApp, "setContributions");
          if (runtimeDiagnosticsEnabled()) {
            console.log("[DEBUG] contributions push", JSON.stringify({ plugin: pluginEntry.handle.pluginId, app: targetApp?.id ?? null, active: isActive, takesPageRun, pageCount: skipped !== undefined ? 0 : 1, crossings: skipped !== undefined ? 0 : 1, chars: json.length, kinds, encoding: "pack", skipped: skipped ?? null }));
          }
          if (!targetApp || !pluginEntry.handle.handleCommand || instanceId == null || skipped !== undefined) continue;
          // 📦️ One pack crossing: handleCommand already encodePackValue's the invocation
          // (`PluginRuntime` performInvocation). The 4 KiB public-invocation string cap is the
          // JSON entry point, not this path. Paging that envelope was 99 guest turns / ~202 s
          // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). An app that declares pageCount still
          // receives page 0 of 1 so the addressed schema matches.
          const args = takesPageRun ? { json, page: 0, pageCount: 1 } : { json };
          try {
            const wire = encodeAppCommandInvocation(pluginEntry.handle.pluginId, targetApp, "setContributions", args);
            const contributionResponse = await pluginEntry.handle.handleCommand(instanceId, wire, environment.targetViewState);
            if (contributionResponse.requestedEffects?.length) {
              console.error("[DEBUG] setContributions deferred effects", JSON.stringify({ plugin: pluginEntry.handle.pluginId, instanceId, effects: contributionResponse.requestedEffects }));
              environment.dispatchDeferredEffects(pluginEntry.handle.pluginId, instanceId, contributionResponse.requestedEffects);
            }
          } catch (error) {
            console.error("setContributions command failed", pluginEntry.handle.pluginId, error instanceof Error ? error.message : String(error));
          }
        }
      },
    });
  }

  /** 📄️ The receiver's OWN open document, as an operator scope — a genesis envelope, a plugin that
   * cannot be read and a read that throws are all typed `unresolved` reasons, never a silent skip. */
  const readDocumentOperatorScope = useCallback(async (receiverPlugin: LoadedProgramState | undefined, instanceId: number): Promise<ContributionsOperatorScope> => {
    if (!receiverPlugin) return { status: "unresolved", reason: "no-receiver" };
    if (!receiverPlugin.handle.readAppDocumentPack) return { status: "unresolved", reason: "no-document-read" };
    const liveDocument = await receiverPlugin.handle.readAppDocumentPack(instanceId);
    if (!liveDocument) return { status: "unresolved", reason: "no-document-pack" };
    const sources = documentSourcesFromPack(liveDocument.pack, liveDocument.spr, liveDocument.ops);
    const scope: ContributionsOperatorScope = liveDocument.ops == null ? { status: "unresolved", reason: "document-ops-missing" } : resolveDocumentOperatorKinds(sources);
    console.error(
      "[DEBUG] contributions document sources",
      JSON.stringify({
        packBytes: liveDocument.pack.byteLength,
        sprBytes: liveDocument.spr.byteLength,
        opsChars: liveDocument.ops?.length ?? null,
        opsHead: (liveDocument.ops ?? "").slice(0, 280),
        status: scope.status,
        reason: scope.status === "unresolved" ? scope.reason : undefined,
        kinds: scope.status === "resolved" ? scope.kinds : [],
      }),
    );
    return scope;
  }, []);

  /** 🧩️ Starts (or joins) the contributions unit for one session and reports what it decided. The
   * environment is captured HERE, where the live host bindings are in scope, so the unit never reads
   * a stale render closure and `refreshUi` owes it nothing but an `await`. */
  const publishContributions = useCallback(
    async (nextSession: ActiveSession, loaded: readonly LoadedProgramState[]): Promise<ContributionsPublishOutcome> => {
      const key: ContributionsSessionKey = { pluginId: nextSession.pluginId, instanceId: nextSession.instanceId };
      const disabledExtensionIds = new Set(extensionLedgerRef.current.filter((entry) => !entry.enabled).map((entry) => entry.extensionId));
      const outcome = await contributionsPublisherRef.current!.publish(key, {
        loadedPlugins: loaded,
        disabledExtensionIds,
        hostMode,
        session: nextSession,
        targetViewState: resolvedTargetViewState(nextSession),
        // 🔁️ The install's own re-arm (`flowEvalTick` per attached preview) is dispatched on a fresh
        // microtask against the LIVE session, never inside the guest crossing that produced it.
        dispatchDeferredEffects: (deferredPluginId, deferredInstanceId, deferredEffects) =>
          queueMicrotask(() => {
            const live = sessionRef.current;
            if (!live) {
              console.error(`[os-shell] ${deferredEffects.length} deferred effect(s) for "${deferredPluginId}" dropped: no live session`);
              return;
            }
            const target = { ...live, pluginId: deferredPluginId, instanceId: deferredInstanceId };
            const owner = captureEffectOwner(target, captureDialogOrigin(target));
            void applyHostEffects(deferredEffects, target, { kind: "full" }, owner).catch((error) => console.error("[DEBUG] setContributions deferred effects failed", error));
          }),
      });
      if (outcome.status === "installed" || outcome.status === "failed") console.error("[DEBUG] contributions publish", JSON.stringify({ plugin: key.pluginId, instanceId: key.instanceId, outcome }));
      return outcome;
    },
    // 🐢️ `applyHostEffects` is declared later in this component and is referenced in the body only,
    // never in this array — the same temporal-dead-zone avoidance `refreshUi` below documents.
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [captureDialogOrigin, captureEffectOwner, hostMode, readDocumentOperatorScope, resolvedTargetViewState],
  );
  /** 🔚 A session switch abandons whatever contributions unit is still resolving for the instance
   * that is going away: its closure must never be installed into an instance nobody is looking at,
   * and the next session re-resolves its own operator scope from scratch. */
  const contributionsInstanceRef = useRef<number | null>(null);
  useEffect(() => {
    const previous = contributionsInstanceRef.current;
    contributionsInstanceRef.current = session?.instanceId ?? null;
    if (previous != null && previous !== (session?.instanceId ?? null)) contributionsPublisherRef.current?.retire(previous);
  }, [session?.instanceId]);
  //#endregion 🧩️ContributionsPush

  const runUiRefreshPass = useCallback(
    // 🪟️ `extraInstancesOverride` lets a caller that just synchronously computed a NEW extra-window list
    // (split/drop, layout/mode switch) hand it straight to this fetch instead of reading `extraWindowInstances`
    // from React state, which wouldn't reflect the just-dispatched change until the next render.
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }, extraInstancesOverride?: readonly ExtraWindowInstance[], replaceBodies = false) => {
      if (scopeArg.kind === "none") return;
      const refreshOwner = captureEffectOwner(nextSession, captureDialogOrigin(nextSession));
      const generation = ++refreshGenerationRef.current;
      if (replaceBodies) replaceBodiesGenerationRef.current = generation;
      let pendingRefreshEffects: readonly Effect[] = [];
      // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — reads `loadedPluginsRef`
      // (kept in sync every render, line ~1145), NOT the `loadedPlugins` array closed over by this
      // callback: this function's own identity depended on `loadedPlugins` (deps array below), so it was
      // RECREATED on every one of the ~50+ sequential catalogue plugin loads during boot — and every
      // `useEffect` that calls `refreshUi` (line ~2461's session-refresh effect, in particular) lists
      // `refreshUi` itself in its own deps array, so a fresh `refreshUi` identity re-fired a FULL
      // `refreshUi(session)` call on the SAME already-open session for each unrelated background plugin
      // finishing its load — live-confirmed (`🧪️5-e-live-postmessage-probe.md`) as the dominant contributor
      // to the `plugin.internal: plugin instance busy` storm: dozens of overlapping `refreshUi` calls
      // colliding on the wasm guest's single-flight `InstanceGuard`, each retried up to 8× on the worker
      // side and 8× more by `withSerializedPluginWasmHandle` on the host side. Mirrors lane 5-A's own
      // `readHistory` effect fix (line ~992) one level up the call graph — that fix stopped `readHistory`
      // itself from refiring but could not stop THIS callback's identity churn, since `readHistory`'s
      // effect and this one are independent call sites, not nested.
      const loadedPlugins = loadedPluginsRef.current;
      const program = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
      if (!program) return;
      // 🧩️ Started BEFORE this refresh's first await and never guarded by `generation`: the push is
      // its own owned unit (see the `🧩️ContributionsPush` region above). A refresh superseded mid
      // document-read therefore no longer abandons it, and a later refresh joins the same run rather
      // than starting a second one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
      const contributionsInstalled = publishContributions(nextSession, loadedPlugins);
      const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
      const isSessionSwitch = layoutSeedKeyRef.current !== layoutSeedKey;
      // 🐢️ A session switch invalidates every cached hash from the previous instance — force a full
      // fetch regardless of what scope this particular call was given.
      let scope = scopeArg;
      if (isSessionSwitch) {
        uiRefreshCacheRef.current = new Map();
        scope = { kind: "full" };
      }
      const cache = uiRefreshCacheRef.current;
      // 🪟️ On a session switch, seed the default layout's extra instances BEFORE fetching (not after), so
      // this very first fetch already requests every default-layout pane's body/measures/engagements
      // instead of leaving newly-seeded panes to show "missing window" until some later, unrelated refresh.
      const layoutSeed = isSessionSwitch ? applyFrameworkLayoutSeed(nextSession.app.defaultLayout, withLocalizedWindowKindLabels(nextSession.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale) : undefined;
      // 🪟️ Prefer the override, then the just-computed session-switch seed, then the live ref (never the
      // render-closure snapshot) so a concurrent refresh cannot drop default-layout panes.
      const extraInstancesForFetch = extraInstancesOverride ?? layoutSeed?.extraInstances ?? extraWindowInstancesRef.current;
      const windowInstances = sessionWindowInstances(nextSession.app, extraInstancesForFetch);
      dispatch({
        type: "SET_WINDOW_UI_BY_WINDOW_ID",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            windowInstances.map((instance) => [instance.id, current[instance.id] ?? pendingWindowUiNode()] as const),
          ),
      });
      // 🪐️ Every loaded plugin's declared apps, flattened for the space app's catalogue — an opt-in
      // hint-push (below), never a view-state field, because the space app is its own wasm component:
      // `semio_framework_os::APP_REGISTRATIONS` (populated at native/test
      // `PluginHost::load_plugin`/`hot_swap_plugin` time) lives in a separate linear memory from the
      // space app's own statically-linked copy of the same os-core crate, so nothing crosses the wasm
      // boundary unless this shell pushes it explicitly.
      const appRegistrationsJson = JSON.stringify(loadedPlugins.flatMap((entry) => (entry.manifest.apps ?? []).map((app) => ({ pluginId: entry.handle.pluginId, app }))));
      // 🧩️ Contributions are deliberately absent here: `publishContributions` (the unit started above)
      // installs them into the guest through the paged `setContributions` run, which the guest folds
      // into its own registry. Riding the aggregated closure inside this view state put 248 635
      // characters against the view-context schema's 65 536-character `panelJson`/long-field bound —
      // rejected at `parseResolvedPluginViewState` as `view context: invalid panel data`, which
      // replaced every window body with a fault card (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
      const viewState: ViewModel = injectActiveTool({
        ...nextSession.viewState,
        locale: uiLocale,
        terminology: uiTerminology,
        windowInstances: windowInstances.map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        activeUtilityId: undefined,
        // 🎯️ The pane the user is LOOKING at, carried into every section of this refresh. Window
        // sections are re-projected per instance by the guest and ignore it; an app-level panel body
        // has no `windowId` at all and this is the only thing that lets one address a live pane.
        focusedWindowId: activeWindowIdRef.current ?? undefined,
      });
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      if (replaceBodies) {
        const leftover = leftoverWorldArmedWindowOverlayV1();
        const hoverVortex = leftover?.hoveredId && (leftover.hoveredDomain === "vortex" || leftover.hoveredId.includes(":")) ? leftover.hoveredId : null;
        if (leftoverBrushPreviewWindowHash(leftover?.activeUtility, hoverVortex, "cached") === undefined) {
          for (const key of [...cache.keys()]) {
            if (key.startsWith("window:")) cache.delete(key);
          }
        }
      }
      // 🐢️ One batched, hash-conditional round trip replaces the old ~12 sequential
      // render/utilities/windowEngagements/windowMeasures/appLabels calls — the plugin omits payloads for
      // any section whose hash still matches what `cache` already holds.
      const request = buildUiRefreshRequest(scope, windowInstances, panelTabLeaves, viewState, cache);
      if (request) {
        const response = await program.refreshUi(nextSession.instanceId, request);
        if (generation !== refreshGenerationRef.current && !(replaceBodies && generation === replaceBodiesGenerationRef.current)) return;
        // 🩹️ `resolveExternalSlots`/`ensureContributorInstance`'s `PluginWasmHandle` (kernel/component.ts,
        // `manifest: () => Promise<Uint8Array>`/`enqueue`/`outcomes`/`dispose` — an actor/turn handle) is
        // a genuinely DIFFERENT abstraction from this file's own `PluginWasmHandle` (`PluginRuntime`'s,
        // `manifest: PluginManifest`) — PluginRuntime's own import already renames kernel's to
        // `KernelPluginWasmHandle` to keep the two apart. Read both call sites (`ensureContributorInstance`
        // only calls `.createApp`; `resolveExternalSlots` only checks the handle's truthiness — the
        // external-slot render path itself is an explicit, documented stub, "the dedicated follow-up work
        // package", always returning "Extension unavailable") before building a REAL adapter rather than
        // casting past the mismatch: `createApp`/`destroyApp` forward to this file's real handle;
        // `manifest`/`enqueue`/`outcomes`/`dispose` are genuinely never invoked by either function today,
        // so they're honest no-op/empty implementations, not a fabricated claim about behavior.
        const slotContext = {
          plugins: new Map(
            loadedPlugins.map((entry) => [
              entry.handle.pluginId,
              {
                manifest: async () => new Uint8Array(),
                createApp: entry.handle.createApp,
                destroyApp: entry.handle.destroyApp,
                takeSegmentedDownloadChunk: entry.handle.takeSegmentedDownloadChunk,
                enqueue: () => {},
                outcomes: (async function* () {})(),
                dispose: async () => {},
              },
            ]),
          ),
          contributorInstances: contributorInstancesRef.current,
          viewState,
        };
        // Resolve external slots on freshly-changed window/panel bodies only, before caching them, so a
        // later no-operation refresh reuses the already-resolved cached value instead of re-resolving.
        const resolveIfChanged = async (entry: PluginUiRefreshSectionResponse): Promise<PluginUiRefreshSectionResponse> => (entry.value !== undefined ? { ...entry, value: await resolveExternalSlots(entry.value as BuiltNode, slotContext) } : entry);
        const [resolvedWindows, resolvedPanels] = await Promise.all([Promise.all((response.windows ?? []).map(resolveIfChanged)), Promise.all((response.panels ?? []).map(resolveIfChanged))]);
        if (generation !== refreshGenerationRef.current && !(replaceBodies && generation === replaceBodiesGenerationRef.current)) return;
        applyUiRefreshResponseToCache(cache, { ...response, windows: resolvedWindows, panels: resolvedPanels });
        // 🩺️ Which sections this pass ASKED for and which the guest actually re-serialized — the one
        // line that separates "the guest re-rendered the previous document" from "this pass never
        // applied" (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] refreshUi sections", JSON.stringify({ scope, utilities: viewState.activeUtilityByWindowId ?? null, asked: (request.windows ?? []).map((entry) => entry.key), changed: (response.windows ?? []).filter((entry) => entry.value !== undefined).map((entry) => entry.key), hashes: Object.fromEntries((response.windows ?? []).map((entry) => [entry.key, entry.hash])) }));
        // ⏱️ pending_effects (e.g. flowEvalTick) wait until contributions are installed — an earlier
        // tick faults `flow.extension-not-contributed` and retires the window-transient authority.
        pendingRefreshEffects = response.requestedEffects ?? [];
      }
      // 🐢️ Merge-with-identity-preservation: unrequested/unchanged sections keep exactly the object
      // reference already in `cache` (dispatched from a prior refresh), so `mergeRecordPreservingIdentity`
      // bails on them via reference equality — this is what lets `InterpretedUiNode`'s `React.memo` (and
      // `modeWindows`'s `useMemo`) skip reconciling the whole shell on every interaction.
      dispatch({
        type: "SET_WINDOW_UI_BY_WINDOW_ID",
        value: (current) => {
          const entries = windowInstances.map((instance) => [instance.id, (cache.get(`window:${instance.id}`)?.value as BuiltNode | undefined) ?? current[instance.id] ?? pendingWindowUiNode()] as const);
          if (!replaceBodies) return mergeRecordPreservingIdentity(current, entries);
          const next: Record<string, BuiltNode> = { ...current };
          for (const [id, node] of entries) next[id] = node;
          return next;
        },
      });
      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicEngagements)),
      });
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_MEASURES_BY_WINDOW_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicMeasures)),
      });
      const dynamicToolMeasures = (cache.get("tools")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({
        type: "SET_TOOL_MEASURES_BY_TOOL_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicToolMeasures)),
      });
      const freshAppLabelsOverlay = normalizeAppLabelsOverlay(cache.get("labels")?.value as Partial<PluginAppLabelsOverlay> | undefined);
      dispatch({ type: "SET_APP_LABELS_OVERLAY", value: (current) => preserveJsonIdentity(current, freshAppLabelsOverlay) });
      // 🛍️ App-static: fetched once per app instance and kept by identity, so a scene host subscribing
      // through `AppCatalogueContext` never re-renders on an unchanged catalogue.
      const freshAppCatalogue = (cache.get("catalogue")?.value as AppCatalogue | undefined) ?? EMPTY_APP_CATALOGUE;
      dispatch({ type: "SET_APP_CATALOGUE", value: (current) => preserveJsonIdentity(current, freshAppCatalogue) });
      dispatch({
        type: "SET_PANEL_UI_BY_KEY",
        value: (current) => {
          const entries = panelTabLeaves
            .filter((tab) => tab.bodyKey)
            .map((tab) => [panelTabKindId(tab.kind), (cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined) ?? current[panelTabKindId(tab.kind)] ?? pendingPanelUiNode()] as const);
          if (!replaceBodies) return mergeRecordPreservingIdentity(current, entries);
          const next: Record<string, BuiltNode> = { ...current };
          for (const [id, node] of entries) next[id] = node;
          return next;
        },
      });
      if (replaceBodies) forceReloadLiveUiStoresV1(cache);
      if (isSessionSwitch && layoutSeed) {
        layoutSeedKeyRef.current = layoutSeedKey;
        extraWindowInstancesRef.current = layoutSeed.extraInstances;
        extraWindowCounterRef.current = layoutSeed.extraInstances.length;
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: layoutSeed.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: layoutSeed.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      }
      // 🧩️ The contributions unit started at the top of this refresh. `await`ing it HERE, and not
      // running it here, is what keeps `pendingRefreshEffects` (an early `flowEvalTick`) waiting until
      // the closure is installed, while leaving the push itself outside this refresh's generation
      // race (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
      await contributionsInstalled;
      // 🤝️ Handed BACK to the lane, never awaited here: applying an effect re-enters `refreshUi`, and a
      // pass that awaits its own re-entrant request waits on itself (`createUiRefreshCoalescerV1`
      // property 3). The lane voids this application and the effect's refresh lands as the next pass.
      if (pendingRefreshEffects.length) owedPassEffectsRef.current = { effects: pendingRefreshEffects, session: nextSession, owner: refreshOwner };
      if (appRegistrationsJson) {
        const appRegistrationsPushKey = `${nextSession.instanceId}::${appRegistrationsJson}`;
        if (appRegistrationsPushKey !== appRegistrationsJsonRef.current) {
          appRegistrationsJsonRef.current = appRegistrationsPushKey;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId);
          // 🪐️ The space app explicitly declares this hidden app command; other apps never receive it.
          if (pluginEntry?.handle.handleCommand && appOwnsCommand(nextSession.app, "setAppRegistrations")) {
            try {
              const wire = encodeAppCommandInvocation(nextSession.pluginId, nextSession.app, "setAppRegistrations", { json: appRegistrationsJson });
              await pluginEntry.handle.handleCommand(nextSession.instanceId, wire, resolvedTargetViewState(nextSession));
            } catch (error) {
              console.error("setAppRegistrations command failed", error instanceof Error ? error.message : String(error));
            }
          }
        }
      }
    },
    // 🐢️ `owedPassEffectsRef` is declared just below (it belongs to the lane that applies what this pass
    // asked for) — referenced here in the body only, never added to this array, which both avoids a
    // temporal-dead-zone reference-before-init and keeps this callback's identity off the effect lane.
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [appLabelsOverlay, hostMode, injectActiveTool, publishContributions, uiLocale, uiTerminology],
  );

  //#region 🤝️UiRefreshCoalescing
  type UiRefreshLaneRequest = { readonly session: ActiveSession; readonly scope: UiDirtyScope; readonly extraInstances?: readonly ExtraWindowInstance[]; readonly replaceBodies: boolean };
  // 🧰️ The lane is built ONCE (it owns the owed slot and the drain loop, which no render may reset), so
  // it reaches the current pass and the current effect applier through refs rather than through a
  // closure — the same reason `runUiRefreshPass` itself reads `loadedPluginsRef`.
  const runUiRefreshPassRef = useRef(runUiRefreshPass);
  runUiRefreshPassRef.current = runUiRefreshPass;
  const applyHostEffectsRef = useRef<(effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<typeof captureEffectOwner>) => Promise<void>>(async () => {});
  /** 🤝️ The shell's one ui-refresh lane — see {@link createUiRefreshCoalescerV1} for the three
   * properties it holds (every request owed until a pass covers it, a failed pass never ends the lane,
   * and a pass may ask for another pass but never wait for one).
   *
   * Why the lane exists at all: a pass abandons itself when `refreshGenerationRef` moves under one of
   * its awaits, which is right only while a pass finishes faster than passes are requested. On the
   * served procedural editor it does not — every `flowEvalTick` completion of a converging preview
   * demands a full pass, and one guest crossing during a brep solve was measured at 15-16 s while
   * completions arrived every few seconds, so each pass was superseded before it applied and NOTHING
   * the guest re-rendered reached the shell (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
   *
   * 🔁️ A pass's own `requestedEffects` are applied OUTSIDE the pass (`owedPassEffectsRef`, voided
   * below): applying an effect re-enters `refreshUi`, and a pass that awaits its own re-entrant
   * request waits on itself. */
  const owedPassEffectsRef = useRef<{ effects: readonly Effect[]; session: ActiveSession; owner: ReturnType<typeof captureEffectOwner> } | null>(null);
  const uiRefreshLaneRef = useRef<UiRefreshCoalescerV1<UiRefreshLaneRequest> | null>(null);
  if (!uiRefreshLaneRef.current) {
    uiRefreshLaneRef.current = createUiRefreshCoalescerV1<UiRefreshLaneRequest>(
      async (request) => {
        await runUiRefreshPassRef.current(request.session, request.scope, request.extraInstances, request.replaceBodies);
        const owedEffects = owedPassEffectsRef.current;
        owedPassEffectsRef.current = null;
        if (owedEffects) void applyHostEffectsRef.current(owedEffects.effects, owedEffects.session, { kind: "full" }, owedEffects.owner).catch((error) => console.error("refresh-owed host effects failed", error));
      },
      (owed, next) => ({ session: next.session, scope: mergeUiDirtyScopeV1(owed.scope, next.scope), extraInstances: next.extraInstances ?? owed.extraInstances, replaceBodies: owed.replaceBodies || next.replaceBodies }),
      (decision, scope, passes) => {
        if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] refreshUi lane", JSON.stringify({ decision, scope, passes }));
      },
    );
  }
  const refreshUi = useCallback(
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }, extraInstancesOverride?: readonly ExtraWindowInstance[], replaceBodies = false) =>
      uiRefreshLaneRef.current!.request({ session: nextSession, scope: scopeArg, extraInstances: extraInstancesOverride, replaceBodies }),
    [],
  );
  //#endregion 🤝️UiRefreshCoalescing
  refreshDirectoryHomeRef.current = async (nextSession) => refreshUi(nextSession);

  /** @emoji 🗣️ Keeps already-built window titles (workbench layout, extra spawned windows) in sync on every locale/terminology switch — `refreshUi` only rebuilds `shellLayout` from scratch on a session change, so an existing session's baked-in titles would otherwise go stale.
   *
   * 🌐️ And asks the GUEST to re-render, because the titles are only the shell's own half. Every label
   * inside a guest-authored body — the outliner's section headings, its per-row Hide/Lock actions, the
   * inspector's field names — is resolved by the program against `ViewModel::locale` at the moment it
   * rendered (`🗣️terminology/🦀️.rs`'s `puzzle3d_labels`), so a language switch that requests no refresh
   * leaves every one of those bodies in the previous language for as long as nothing else happens to
   * dirty them. Measured on `:6013`: with the actor-revocation of §1 fixed, `locale-switch` brought the
   * whole shell roster back German while the outliner still read "OBJECTS / Hide / Lock / REFERENCES"
   * (ticket 26/09/02 wave B40 §3). `replaceBodies` because a language change re-authors EVERY string:
   * the hash-conditional path must not be allowed to keep a body it cached under the old locale, and
   * the live UI stores hold rendered text of their own. */
  useEffect(() => {
    const windowKinds = session?.app.windowKinds;
    if (!windowKinds) return;
    dispatch({
      type: "SET_SHELL_LAYOUT",
      value: (current) => (current ? retitleWindowLayoutNode(current, withLocalizedWindowKindLabels(windowKinds), extraWindowInstancesRef.current, uiTerminology, uiLocale) : current),
    });
    dispatch({
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: (current) => {
        const next = current.map((entry) => {
          const kind = windowKinds.find((k) => k.id === entry.windowKindId || k.id === entry.id);
          const title = kind ? resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale) : entry.title;
          return { ...entry, title };
        });
        extraWindowInstancesRef.current = next;
        return next;
      },
    });
    const live = sessionRef.current;
    if (live) void refreshUi(live, { kind: "full" }, undefined, true).catch((error) => console.error("[os-shell] locale refresh failed", error));
  }, [uiTerminology, uiLocale, refreshUi]);

  const refreshSpawnedUi = useCallback(
    async (spawned: SpawnedAppEntry, viewState: ViewModel, scopeArg: UiDirtyScope = { kind: "full" }) => {
      if (scopeArg.kind === "none") return;
      const generation = ++spawnedRefreshGenerationRef.current;
      // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — same fix as `refreshUi`
      // above: reads `loadedPluginsRef.current` instead of closing over `loadedPlugins`, so this
      // callback's identity (and every effect that lists it in a deps array) stops churning on every
      // unrelated background catalogue plugin load.
      const loadedPlugins = loadedPluginsRef.current;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId);
      const plugin = pluginEntry?.handle;
      const app = pluginEntry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!plugin || !app) {
        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        dispatch({
          type: "SET_SPAWNED_WINDOW_UI",
          // 🦴 A `BuiltNode` needs its full field set (layout/style/accessibility/…) — reuse
          // `pendingWindowUiNode()`'s already-valid defaults rather than hand-authoring them, and
          // override just the component (a plain text node) and activity (no longer "loading").
          value: { ...pendingWindowUiNode(), activity: "idle", component: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}`, emphasize: null, dataAttributes: null } } satisfies BuiltNode,
        });
        dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
        dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
        return;
      }
      const spawnedSeed = `${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`;
      if (spawnedLayoutSeedRef.current !== spawnedSeed) {
        spawnedLayoutSeedRef.current = spawnedSeed;
        spawnedUiRefreshCacheRef.current = new Map();
      }
      const cache = spawnedUiRefreshCacheRef.current;
      const bodyKey = resolveCanvasBodyKey(app);
      // 🧩️ A spawned instance reads contributions from the registry the paged `setContributions` run
      // installed, exactly like a primary session — the view context carries none.
      const fullViewState: ViewModel = injectActiveUtility(
        { ...viewState, locale: uiLocale, terminology: uiTerminology, windowId: bodyKey, windowInstances: [{ id: bodyKey, windowKindId: bodyKey }] },
        spawned.id,
      );
      // 🐢️ A spawned instance's view is a single body + utilities + engagements + measures (no panels, no
      // labels) — that's already the minimal grouping, so there is no narrower-than-full "partial" scope
      // worth expressing here; only `none` (handled above) short-circuits the request.
      const singleWindowKind = [{ id: bodyKey, bodyKey }];
      const request = buildUiRefreshRequest({ kind: "full" }, singleWindowKind, [], fullViewState, cache);
      if (request) {
        const response = await plugin.refreshUi(spawned.instanceId, request);
        if (generation !== spawnedRefreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, response);
      }
      const ui = (cache.get(`window:${bodyKey}`)?.value as BuiltNode | undefined) ?? pendingWindowUiNode();
      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: (current: BuiltNode | null) => preserveJsonIdentity(current ?? undefined, ui) });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: dynamicEngagements });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: dynamicMeasures });
    },
    [injectActiveUtility, uiLocale, uiTerminology],
  );

  // 🐢️ Keyed on the pluginId/app/instance triple (not `session` object identity) so this only fires on
  // a genuine session switch (app open/spawn/instance change) — every other action already calls
  // `refreshUi` explicitly via `applyHostEffects`, and re-running it here too on every `session` object
  // churn was a second, redundant full-shell refresh cascade per interaction.
  // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — `loadedPlugins` dropped
  // from this effect's own deps (mirrors `refreshUi`'s own fix above, and lane 5-A's `readHistory`
  // effect fix, line ~992): a session can only exist for an already-loaded plugin (nothing sets
  // `session`/`sessionIdentityKey` for a plugin still mid-load), so this effect never needed to refire
  // on unrelated background catalogue loads in the first place — `refreshUi` itself now reads
  // `loadedPluginsRef.current` at call time regardless. Before this fix, `loadedPlugins` getting a new
  // array reference on every one of the ~50+ sequential background plugin loads during boot re-ran this
  // effect that many times for the SAME already-open session, each dispatching a fresh top-level
  // `refreshUi` against the wasm guest's single-flight `InstanceGuard` — live-confirmed as the dominant
  // contributor to the `plugin.internal: plugin instance busy` storm blocking collab-e2e STEP 2.
  const sessionIdentityKey = session ? `${session.pluginId}:${session.app.id}:${session.instanceId}` : null;
  useEffect(() => {
    const current = sessionRef.current;
    if (!current) return;
    void refreshUi(current).catch((renderError) => {
      const fault = windowFaultFromError(renderError, pluginSupervisorByIdRef.current[current.pluginId]);
      console.error(`[DEBUG] render failed [${fault.class}] ${fault.code ?? "no-code"}`, renderError);
      dispatch({ type: "SET_ERROR", value: fault.message, fault });
    });
  }, [refreshUi, sessionIdentityKey]);

  useEffect(() => {
    if (!hostMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    void refreshSpawnedUi(activeSpawned, session.viewState).catch((renderError) => {
      const fault = windowFaultFromError(renderError, pluginSupervisorByIdRef.current[activeSpawned.pluginId]);
      console.error(`[DEBUG] spawned render failed [${fault.class}] ${fault.code ?? "no-code"}`, renderError);
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null, fault });
    });
    // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — `loadedPlugins` dropped
    // (same reasoning as the session-refresh effect above): `refreshSpawnedUi` now reads
    // `loadedPluginsRef.current` at call time, and a spawned instance can only exist for an
    // already-loaded plugin, so this never needed to refire on unrelated background catalogue loads.
  }, [panel, refreshSpawnedUi, session, hostMode]);

  const updateSpacePanel = useCallback((panelState: SpacePanelState) => {
    dispatch({
      type: "SET_SESSION",
      value: (current) => {
        if (!current) return current;
        return { ...current, viewState: { ...current.viewState, panelJson: panelJsonFromState(panelState) } };
      },
    });
  }, []);

  /** 🪦️ The session-instance close ladder, lifted out of the shell-unmount teardown effect so an
   * in-place app switch retires its predecessor the same way unmounting does: every document session
   * still bound to that instance is closed (which retires its attachment), then the attachment lane is
   * drained, then the instance is destroyed. Without it every `switchToPluginApp` leaked one live
   * `createApp` — tolerable while only two host apps ever swapped, not for a user-facing toggle
   * (`📓️viewer-eval-chain-2026-09-12.md` §6 item 2). Never throws: a predecessor that refuses to
   * retire is a leak to report, never a reason to block the successor. */
  const retireSessionInstance = useCallback(async (retired: ActiveSession): Promise<void> => {
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === retired.pluginId)?.handle;
    if (!plugin) return;
    for (const [runtimeKey, entry] of [...openDocumentSessionsRef.current]) {
      if (entry.plugin !== plugin || entry.session.instanceId !== retired.instanceId) continue;
      closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
    }
    await documentAttachmentLanesRef.current.get(plugin)?.get(retired.instanceId)?.drain();
    await plugin.destroyApp(retired.instanceId);
  }, []);

  // 🏠️🧳️👁️✏️ Generalised from the host-only `switchToManagedApp`: switches the mounted session to ANY
  // loaded plugin's app by `(pluginId, appId)`, so the navbar role group can swap a playground between
  // `…#editor` and `…#viewer` the same way studio mode swaps its landing/host apps. The host-only
  // bookkeeping (`openSpaceIdRef`/`openInstanceIdRef` reset on the landing app) stays behind an
  // explicit `hostMode` guard rather than being reachable from a non-host switch.
  //
  // 🪦️ TRANSACTIONAL since 26/09/09/PROCEDURAL-3D-END-TO-END: the whole switch runs through
  // `sessionSwitchGateRef` (one at a time) and starts with a `quiesce` pass over `sessionWorkRef`, so
  // the predecessor is never torn down under its own in-flight typed operations and extension
  // invocations — the mid-chain tear-down measured on 6018, where `⌘️⌥️V` pressed during an
  // `interactionSelect`/`flow-extension-brep evaluate` round trip revoked instance 1's activation and
  // left every later action failing with `no actor for instance 1`
  // (`🗑️generated/journey-3/console.txt`). `seal` runs before the close ladder because the ladder's own
  // first step already revokes the actor.
  const switchToPluginApp = useCallback(
    async (pluginId: string, appId: string, viewState?: ViewModel): Promise<ActiveSession | null> => {
      setSurfaceSwitchBusy(true);
      try {
        const outcome = await sessionSwitchGateRef.current.run(
          {
            session,
            resolveApp: (targetPluginId, targetAppId) => loadedPlugins.find((entry) => entry.handle.pluginId === targetPluginId)?.manifest.apps.find((candidate) => candidate.id === targetAppId) ?? null,
            appId: (app) => app.id,
            quiesce: async (retiring) => {
              const quiet = await quiesceSessionWorkV1(() => sessionWorkRef.current.pending(retiring.pluginId, retiring.instanceId));
              // ⏳️ A refusal has to name what it waited on, or the next person reading the console sees
              // only a number and cannot tell a stuck guest turn from a stuck extension round trip.
              if (!quiet.settled) console.warn(`surface-switch draining ${retiring.pluginId}#${retiring.instanceId} ${sessionWorkRef.current.outstanding(retiring.pluginId, retiring.instanceId)}`);
              return quiet;
            },
            seal: (retiring) => sealedInstancesRef.current.seal(retiring.pluginId, retiring.instanceId),
            unseal: (kept) => sealedInstancesRef.current.unseal(kept.pluginId, kept.instanceId),
            createInstance: async (targetPluginId, app) => {
              const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === targetPluginId)?.handle;
              if (!handle) throw new Error(`switchToPluginApp: ${targetPluginId} is no longer loaded`);
              return handle.createApp(app.id);
            },
            retire: retireSessionInstance,
            // 🪶️ The empty studio panel belongs to `hostMode` only: a non-host playground's boot session
            // carries no `panelJson` at all, so a role switch must not invent one either.
            defaultViewState: (app) => ({ activeModeId: app.defaultModeId ?? app.modes[0]?.id, ...(hostMode ? { panelJson: panelJsonFromState(buildSpacePanelState([], requiredHostPanelLeafId(hostApp))) } : {}) }),
            publish: (next) => dispatch({ type: "SET_SESSION", value: next }),
            seedLayout: (app) => {
              const seeded = applyFrameworkLayoutSeed(app.defaultLayout, withLocalizedWindowKindLabels(app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
              extraWindowInstancesRef.current = seeded.extraInstances;
              extraWindowCounterRef.current = seeded.extraInstances.length;
              dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
              dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
              dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
              if (hostMode && app.id === landingAppId) {
                openSpaceIdRef.current = null;
                openInstanceIdRef.current = null;
              }
            },
            refresh: refreshUi,
          },
          { pluginId, appId, viewState },
          (retired, retireError) => console.warn(`switchToPluginApp: predecessor ${retired.pluginId}/${retired.app.id} retirement failed`, retireError),
        );
        // 🚦️ `draining` and `busy` both mean "the switch did NOT happen": the predecessor kept its
        // instance and the shell kept its surface. Telling the user so is the whole difference between
        // a refusal and a silent no-op — the alternative the fixture rules out is tearing the
        // predecessor down under its own in-flight work.
        if (outcome.status === "draining" || outcome.status === "busy") {
          showTransientNoticeRef.current(surfaceSwitchBusyTextV1(uiLocale), "info");
          return null;
        }
        return outcome.session;
      } finally {
        setSurfaceSwitchBusy(false);
      }
    },
    [loadedPlugins, refreshUi, retireSessionInstance, session, appLabelsOverlay, hostMode, hostApp, landingAppId, uiTerminology, uiLocale],
  );

  const syncSpawnedPluginDocument = useCallback(async (plugin: PluginWasmHandle, app: AppDefinition, pluginInstanceId: number, documentJson: string, viewState: ViewModel) => {
    try {
      const document = JSON.parse(documentJson) as Record<string, unknown>;
      const targetSession: ActiveSession = { pluginId: plugin.pluginId, instanceId: pluginInstanceId, app, viewState };
      await plugin.handleAction(pluginInstanceId, encodeWindowActionInvocation(targetSession, { controllerId: app.controllerId, action: "setDocument", args: { document } }), viewState);
    } catch (syncError) {
      console.error("[DEBUG] spawned program document sync failed", syncError);
    }
  }, []);

  const ensureSpawnedPlugin = useCallback(
    async (program: SpaceProgramEntry, isCurrent: () => boolean, label?: string, osInstanceId?: string, documentJson?: string, sourceViewState?: ViewModel): Promise<SpacePanelState | null> => {
      if (!isCurrent()) return null;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return null;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(sourceViewState ?? session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (documentJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, documentJson, sourceViewState ?? session.viewState);
        }
        return isCurrent() ? studioPanelFocusingSpawned(currentPanel, existing) : null;
      }
      const instanceId = await createAdmittedShellInstanceV1(isCurrent, () => pluginEntry.handle.createApp(program.appId), (id) => pluginEntry.handle.destroyApp(id));
      if (instanceId === null) return null;
      if (documentJson && app) {
        await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, documentJson, sourceViewState ?? session.viewState);
      }
      if (!isCurrent()) {
        await pluginEntry.handle.destroyApp(instanceId);
        return null;
      }
      const spawnedId = osInstanceId ?? `${program.pluginId}-${instanceId}`;
      return studioPanelFocusingSpawned(currentPanel, {
        id: spawnedId,
        pluginId: program.pluginId,
        instanceId,
        appId: program.appId,
        label: label ?? program.label,
        breadcrumb: program.breadcrumb,
      });
    },
    [loadedPlugins, session, syncSpawnedPluginDocument, hostApp],
  );

  /**
   * 🐚️ Consumes a plugin action's typed `requestedEffects: Effect[]` (WS-D's `InvocationResponse`) —
   * replaces the deleted `processPluginOperations` string-matching. The legacy `setDocument`-mirror
   * backbone-write block is gone entirely: document content sync now flows through
   * `openDocument`/`closeDocument`'s worker-backed `DocumentHost` lifecycle, not a per-operation JS mirror.
   */
  const requestInferenceProposal = useCallback(async (baseSession: ActiveSession, admit: () => boolean) => {
    if (!admit()) throw new Error("inference-opening: owner retired");
    if (inferencePortOwnerRef.current !== null || inferencePortOpeningRef.current !== null) throw new Error("inference.capacity");
    const owners = [...openDocumentSessionsRef.current.entries()].filter(([, entry]) => entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId && entry.scope !== undefined);
    const owner = owners.length === 1 ? owners[0] : undefined;
    const scope = owner?.[1].scope;
    if (!owner || !scope) throw new Error("inference-proposal: exact document owner required");
    const authority = verifiedSessionAuthorityRef.current;
    if (!directorySessionAuthorityIsCurrentV1(authority, authority) || authority?.userId !== identityRef.current?.userId || retiredSessionDocumentOwnersRef.current.has(owner[1])) throw new Error("inference-proposal: authenticated authority required");
    if (inferencePortEpochRef.current === Number.MAX_SAFE_INTEGER) throw new Error("inference-proposal: epoch exhausted");
    const worker = ensureBackboneWorker();
    const operationEpoch = ++inferencePortEpochRef.current;
    const [runtimeKey] = owner;
    const mailbox = new InferencePortOpeningMailboxV1((request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }));
    const opening = { owner: { operationEpoch, runtimeKey, scope }, clientInstanceId: owner[1].clientInstanceId, sessionInstanceId: baseSession.instanceId, mailbox };
    inferencePortOpeningRef.current = opening;
    try {
      await mailbox.open({ kind: "inference-open", operationEpoch, scope });
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (inferencePortOpeningRef.current !== opening || entry?.clientInstanceId !== opening.clientInstanceId || entry.session.instanceId !== opening.sessionInstanceId || retiredSessionDocumentOwnersRef.current.has(entry) || !directorySessionAuthorityIsCurrentV1(authority, verifiedSessionAuthorityRef.current) || !admit()) throw new Error("inference-opening: owner retired");
      inferencePortOwnerRef.current = opening.owner;
      inferencePortAuthorityRef.current = authority;
      dispatch({ type: "OPEN_INFERENCE_PORT", runtimeKey, operationEpoch });
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-propose", operationEpoch, requestId: mintDirectoryCommandRequestId() }) });
    } catch (error) {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-close", operationEpoch }) });
      throw error;
    } finally {
      if (inferencePortOpeningRef.current === opening) inferencePortOpeningRef.current = null;
      mailbox.close("inference-opening: settled");
    }
  }, [ensureBackboneWorker]);

  const directBrowserActorForSession = useCallback((baseSession: ActiveSession) => {
    const matches = [...browserActorUiByRuntimeKeyRef.current.entries()].flatMap(([runtimeKey, retained]) => {
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      return retained.identity !== null && entry?.clientInstanceId === retained.clientInstanceId && entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId && retained.sessionInstanceId === baseSession.instanceId && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session)
        ? [{ runtimeKey, retained, entry, identity: retained.identity }]
        : [];
    });
    if (matches.length > 1) throw new Error("browser-actor-action: ambiguous document owner");
    return matches[0] ?? null;
  }, []);

  const dispatchDirectBrowserActorCommand = useCallback(async (
    owned: NonNullable<ReturnType<typeof directBrowserActorForSession>>,
    invocation: ActionInvocation | CommandInvocation,
    viewState: ViewModel,
  ): Promise<void> => {
    const { runtimeKey, retained, entry, identity } = owned;
    const current = (): boolean => {
      const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      return live?.actions === retained.actions && live.identity === identity && openDocumentSessionsRef.current.get(runtimeKey) === entry && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session);
    };
    if (!current()) throw new Error("browser-actor-action: owner retired");
    const result = await retained.actions.dispatchCommand({
      scope: retained.scope,
      verifiedSurfaceId: retained.verifiedSurfaceId,
      activationGeneration: retained.activationGeneration,
      appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
      instanceId: identity.instanceId,
      surfaceRevision: retained.store.getRevisionSnapshot(),
    }, invocation, viewState);
    await publishBrowserActorHostEffectsV1(result.hostEffects, current, async (effect) => {
      if ("requestInferenceProposal" in effect) await requestInferenceProposal(entry.session, current);
      else window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
    });
    // 🖼️ Wave B9 lane 4: this is the ONE dispatch route that carries no `UiDirtyScope` of its own and
    // publishes no `OperationCompleted` frame for the verbs that commit inline, so without this the
    // document changes and nothing repaints until some later, unrelated action refreshes — `paste`
    // reached the world lane at +12 s in the browser (wave B6 §3). `browserActorDispatchUiScopeV1`
    // answers `none` for a refusal or a zero-mutation action, and `refreshUi` returns immediately on it.
    const actionId = "actionId" in invocation.address ? invocation.address.actionId : undefined;
    applyLeftoverInteractionView(("output" in result ? (result as { output?: unknown }).output : undefined) ?? null, actionId);
    const dirty = browserActorWindowConfigDispatchUiScopeV1(result, actionId);
    if (dirty.kind !== "none" && current()) {
      await refreshUi({ ...entry.session, viewState }, dirty, undefined, leftoverReplaceRefreshBodiesV1());
    }
  }, [directBrowserActorForSession, refreshUi, requestInferenceProposal]);

  const applyHostEffects = useCallback(
    async (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<typeof captureEffectOwner>) => {
      const effectOrigin = effectOwner.presentation;
      // 🪦️ An effect pass addressed to an instance a switch has SEALED is stale by construction — the
      // close ladder has already revoked its actor, so running the pass reaches `requireActorId` and
      // surfaces as `no actor for instance N`. One typed drop, no throw, no stack.
      if (dropForSealedInstance(baseSession, "host effects", `${effects.length} effect(s)`)) return;
      let nextViewState = baseSession.viewState;
      for (const effect of effects) {
        if (!isCurrentEffectOwner(effectOwner)) return;
        if (effect === "requestSync") continue;
        if ("notify" in effect) {
          // 🧯️ A plugin's own user-facing message (`kernel::Effect::Notify`) — already localized by the
          // plugin against the host's declared locale/terminology axes, so it is shown verbatim.
          if (effect.notify.message) showTransientNoticeRef.current(effect.notify.message, "warning");
          continue;
        }
        if ("clipboardWrite" in effect) {
          clipboardFragmentRef.current = clipboardWriteFragmentFromEffect(effect);
          continue;
        }
        if ("setPanel" in effect) {
          nextViewState = { ...nextViewState, panelJson: effect.setPanel.panelJson };
          continue;
        }
        if ("setActiveUtility" in effect) {
          // 🧰️ A program programmatically switched utility: mirror it into the host-owned store slice AND
          // the ref `refreshUi` reads (bare `dispatch` alone leaves the map stale until the next render —
          // which is after this same pass's refresh, so brush/suggestion ghosts and gumballs never appear).
          const { windowId, utilityId } = effect.setActiveUtility;
          setActiveUtilityForWindow(windowId, utilityId || null);
          if (utilityId && activeToolIdRef.current) {
            activeToolIdRef.current = null;
            dispatch({ type: "SET_ACTIVE_TOOL", toolId: null });
          }
          if (windowId === activeWindowIdRef.current) nextViewState = { ...nextViewState, activeUtilityId: utilityId || undefined, activeToolId: utilityId ? undefined : nextViewState.activeToolId };
          continue;
        }
        if ("setActiveTool" in effect) {
          // 🛠️ A program programmatically switched tools (e.g. puzzle3d fill via engagement text command):
          // mirror it into the host-owned store slice, clear every window's active utility (mutual
          // exclusion — a tool and a window utility never both claim the pointer), and fold it into the
          // view state fed to the follow-up refresh.
          const { toolId } = effect.setActiveTool;
          activeToolIdRef.current = toolId || null;
          dispatch({ type: "SET_ACTIVE_TOOL", toolId: toolId || null });
          if (toolId) clearAllWindowUtilities();
          nextViewState = { ...nextViewState, activeToolId: toolId || undefined, activeUtilityId: toolId ? undefined : nextViewState.activeUtilityId };
          continue;
        }
        if ("openDialog" in effect) {
          const { dialogId, args } = effect.openDialog;
          if (shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session)) {
            const value = makeOwnedDialog(dialogId, effectOrigin, args as Record<string, unknown> | undefined);
            if (value !== null) {
              liveDialogRef.current = value;
              dispatch({ type: "SET_DIALOG", value });
            }
          }
          continue;
        }
        if ("navigate" in effect) {
          navigateHistory(effect.navigate.uri);
          continue;
        }
        if ("loadDocument" in effect) {
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          const payload = effect.loadDocument;
          if (payload.pack && payload.spr && pluginEntry?.handle.loadAppDocumentPack) {
            const packBytes = coerceWireBytes(payload.pack);
            const sprBytes = coerceWireBytes(payload.spr);
            console.log("[DEBUG] loadDocument pack/spr for instance", baseSession.instanceId, "pack", packBytes.length, "spr", sprBytes.length);
            await loadDocumentPair(pluginEntry.handle, baseSession.instanceId, packBytes, sprBytes, () => isCurrentEffectOwner(effectOwner));
          } else {
            // 🚧️ `Effect::LoadDocument` is pack+spr bytes only now (no JSON-text fallback exists on the
            // wire anymore — see this variant's own doc comment on `@semio-tech/framework`'s `Effect`
            // type) — a program without `loadAppDocumentPack` simply cannot receive this effect.
            console.error("[os-shell] loadDocument: program has no pack loader", baseSession.pluginId, Object.keys(payload));
          }
          continue;
        }
        if ("openExternalUrl" in effect) {
          window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
          continue;
        }
        if ("downloadMediaExport" in effect) {
          const { filename, mimeType, data, encoding } = effect.downloadMediaExport;
          const encodingText = mediaExportEncodingText(encoding);
          if (encodingText?.startsWith(SEGMENTED_DOWNLOAD_MARKER_PREFIX)) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (!pluginEntry) throw new Error(`segmented-download-plugin-missing:${baseSession.pluginId}`);
            // 🌊 The ASSEMBLED sink, not the File System Access stream: a segmented download must reach the
            // user as the same kind of file the inline lane produces, and `createSegmentedDownloadSink`
            // fails closed wherever `showSaveFilePicker` is absent — which turned every over-budget export
            // into silence. Bounded by the drain's own `SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes` cap.
            await drainSegmentedMediaExport(filename, mimeType, data, encodingText, (operationId) => pluginEntry.handle.takeSegmentedDownloadChunk(baseSession.instanceId, operationId), {
              signal: segmentedDownloadAbortRef.current.signal,
              sinkFactory: shellSegmentedDownloadSinkFactory,
            });
          } else {
            downloadMediaExport(filename, mimeType, data, encodingText);
          }
          continue;
        }
        if ("iconRenderExport" in effect) {
          for (const item of effect.iconRenderExport.items) {
            try {
              const result = await iconRenderPort.render(item.request as Parameters<typeof iconRenderPort.render>[0]);
              downloadDataUrl(item.filename, result.dataUrl);
            } catch (error) {
              console.error(`icon render export failed for ${item.filename}`, error);
            }
          }
          continue;
        }
        if ("requestFileOpen" in effect) {
          const { accept, readAs, importAction, multiple } = effect.requestFileOpen;
          const resolvedImport = importAction || "importFixture";
          console.warn(`[DEBUG] import-picker hop accept=${accept} importAction=${importAction || "<empty>"} resolved=${resolvedImport} multiple=${Boolean(multiple)}`);
          const opened = await requestFileOpen(accept || ".spk,.dsl,.ops,application/octet-stream", readAs, multiple);
          console.warn(`[DEBUG] import-picker opened=${opened.length} name=${opened[0]?.name ?? "none"} bytes=${opened[0]?.contents.length ?? 0}`);
          if (opened.length > 0) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (pluginEntry) {
              // 📤️ Single-file (multiple absent/false): identical to the pre-multi-select shape, one
              // `handleAction` call with `{payload, name}`. Multi-file: one sequential call per selected
              // file, each extending args with `{index, total}` so the plugin can stage/merge imports.
              await dispatchOpenedFiles(opened, resolvedImport, Boolean(multiple), makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState));
            }
          }
          continue;
        }
        if ("dispatchAction" in effect) {
          // 🔁️ Self re-dispatch (D2): re-invokes the same plugin instance with `action` after `delayMs`,
          // without blocking the current `applyHostEffects` pass — the host's one continuation scheduler
          // fires the follow-up call and feeds its own `requestedEffects` back through `applyHostEffects`
          // recursively, so a plugin can chain several ticks of staged/progressive work (e.g. a
          // multi-pass reconstruction) purely by re-emitting `dispatchAction` from its own handler.
          // 🪃️ `delayMs: 0` — what every `rearm()` in flow/generation2d/generation3d asks for — is an
          // unthrottled macrotask, NOT a nested `setTimeout`: this branch re-enters from inside the
          // previous dispatch's own callback, and a hidden/unfocused/headless renderer clamps exactly
          // that shape to ~1 tick/s (this ticket's measured ~24 s per extension hop).
          const { action: dispatchActionId, args: dispatchArgs, delayMs } = effect.dispatchAction;
          // 🪦️ `loadedPluginsRef`, never the render closure's `loadedPlugins`: a DEFERRED effect pass
          // (`publishContributions`'s `dispatchDeferredEffects`) runs against the `applyHostEffects`
          // captured when the publish STARTED — one render before the program it just installed reached
          // `loadedPlugins` state. Read from the closure, both of the install's own `flowEvalTick`
          // re-arms found an EMPTY list and were dropped without a word, so the whole evaluation chain
          // never started (measured on 6018, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A miss is now
          // loud: a re-arm the host silently swallows is indistinguishable from a guest that stopped.
          const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (!pluginEntry) {
            console.error(`[os-shell] dispatchAction "${dispatchActionId}" dropped: no loaded program for "${baseSession.pluginId}" (loaded: ${loadedPluginsRef.current.map((entry) => entry.handle.pluginId).join(", ") || "none"})`);
            continue;
          }
          scheduleDispatchAction(dispatchActionId, dispatchArgs as Record<string, unknown> | undefined, delayMs, makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState));
          continue;
        }
        if ("requestInferenceProposal" in effect) {
          await requestInferenceProposal(baseSession, () => isCurrentEffectOwner(effectOwner));
          continue;
        }
        if ("replayShellCommand" in effect) {
          const { actionId, args } = effect.replayShellCommand;
          const argsRecord = args as Record<string, unknown> | undefined;
          if (actionId === "os.directory.open-administration") {
            const opening = shellSpaceAdministrationOpening(actionId, argsRecord, spaceAdministrationEpochRef.current + 1);
            if (opening === null) {
              console.warn("[os-shell] replayShellCommand: administration requires an exact space id");
            } else if (!identityRef.current) {
              console.warn("[os-shell] replayShellCommand: administration dropped, no signed-in identity");
            } else {
              const worker = ensureBackboneWorker();
              spaceAdministrationEpochRef.current = opening.state.operationEpoch;
              spaceAdministrationRef.current = opening.state;
              setSpaceAdministration(opening.state);
              worker.postMessage({ wire: encodeBackboneWorkerRequest(opening.request) });
            }
          } else if (actionId === "os.create-space-artifact") {
            const origins = [...openDocumentSessionsRef.current.entries()].filter(
              ([, entry]) => entry.session.pluginId === baseSession.pluginId
                && entry.session.instanceId === baseSession.instanceId
                && entry.scope?.documentId === S_SPACE_INDEX_DOCUMENT_ID,
            );
            const origin = origins.length === 1 ? origins[0] : undefined;
            if (origin === undefined || origin[1].scope === undefined || effectOrigin?.document?.runtimeKey !== origin[0]
              || effectOrigin.document.clientInstanceId !== origin[1].clientInstanceId
              || !shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session)) {
              console.warn("[os-shell] replayShellCommand: space artifact creation requires one mounted Space index");
            } else if (!identityRef.current) {
              console.warn("[os-shell] replayShellCommand: space artifact creation dropped, no signed-in identity");
            } else {
              const requestId = mintDirectoryCommandRequestId();
              const currentCatalog = captureSpaceArtifactCreationCatalogAuthorityV1(
                spaceArtifactCreationCatalogRef.current,
                spaceArtifactCreationCatalogUiRef.current,
                effectOrigin,
              );
              const request = spaceArtifactCreationRequestFromAction(actionId, argsRecord, origin[1].scope.spaceId, requestId, effectOwner.creationCatalog, currentCatalog);
              if (request === null) {
                console.warn("[os-shell] replayShellCommand: invalid space artifact creation request");
              } else {
                const owner: SpaceArtifactCreationOwnerV1 = {
                  requestId,
                  spaceId: request.spaceId,
                  expectedCatalogGenerationId: request.expectedCatalogGenerationId,
                  kindId: request.kindId,
                  name: request.name,
                  runtimeKey: origin[0],
                  clientInstanceId: origin[1].clientInstanceId,
                  sessionInstanceId: origin[1].session.instanceId,
                  opening: false,
                  cancelRequested: false,
                  ready: null,
                };
                spaceArtifactCreationOwnersRef.current.set(requestId, owner);
                setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "issued", owner }));
                const worker = ensureBackboneWorker();
                worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
              }
            }
          } else if (actionId.startsWith("os.directory.")) {
            const command = directoryCommandFromAction(actionId, argsRecord);
            if (!command) {
              console.warn("[os-shell] replayShellCommand: unrecognized directory action", actionId);
            } else if (!identityRef.current) {
              console.warn("[os-shell] replayShellCommand: directory command dropped, no signed-in identity", actionId);
            } else {
              const worker = ensureBackboneWorker();
              const requestId = mintDirectoryCommandRequestId();
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-command", requestId, command }) });
            }
          } else if (actionId === "os.open-artifact" || actionId === "os.open-artifact-with") {
            try {
              const opening = resolveArtifactOpeningRelayRef.current(actionId, argsRecord);
              if (!opening) {
                console.warn("[os-shell] replayShellCommand: artifact router is not ready", args);
                continue;
              }
              const target = await openArtifactWithAppRefRef.current(opening.app, opening.dialect, opening.role, () => isCurrentEffectOwner(effectOwner));
              if (target && opening.documentId && opening.schema) {
                await openDocumentRef.current({ documentId: opening.documentId, schema: opening.schema, ...(opening.spaceId ? { spaceId: opening.spaceId } : {}) }, undefined, target);
              }
            } catch (openingError) {
              console.warn("[os-shell] replayShellCommand: artifact opening rejected", openingError, args);
            }
          } else {
            if (actionId === SET_ACTIVE_EXAMPLE_ACTION_ID) {
              const raw = typeof argsRecord?.exampleId === "string" ? argsRecord.exampleId : "";
              const exampleId = raw || resolveBootExampleId("", exampleOptionsRef.current, defaults.exampleId);
              if (raw) lastDispatchedExampleIdRef.current = raw;
              dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
            }
            console.warn("[DEBUG] replayShellCommand dispatch", JSON.stringify({ actionId }));
            onActionRef.current({ controllerId: baseSession.app.controllerId, action: actionId, args: argsRecord });
          }
          continue;
        }
        if ("requestMediaFrames" in effect) {
          // 🎞️ D5: decodes a video (file picker, or `payload` bytes already in hand from a drop zone)
          // and fans sampled frames + a completion marker out through the same `dispatchOne` path as
          // every other effect branch — see `runRequestMediaFrames` for the Tier 1 (WebCodecs)/Tier 2
          // (`<video>` seek-and-capture)/fallback decision tree.
          const { accept, payload, frameAction, doneAction, fallbackAction, sampleStride, maxFrames, maxLongEdgePx, fpsHint, args } = effect.requestMediaFrames;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (pluginEntry) {
            await runRequestMediaFrames(
              {
                frameAction,
                doneAction,
                fallbackAction,
                sampleStride: sampleStride ?? 0,
                maxFrames: maxFrames ?? 0,
                maxLongEdgePx: maxLongEdgePx ?? 0,
                fpsHint: fpsHint ?? 0,
                args: args as Record<string, unknown> | undefined,
              },
              accept,
              payload,
              makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState),
            );
          }
          continue;
        }
        if ("invokeExtension" in effect) {
          const pluginsNow = loadedPluginsRef.current;
          const requesterId = pluginsNow.some((entry) => entry.handle.pluginId === baseSession.pluginId) ? baseSession.pluginId : sessionRef.current?.pluginId;
          const requester = requesterId && pluginsNow.some((entry) => entry.handle.pluginId === requesterId) ? { pluginId: requesterId, instanceId: baseSession.instanceId } : baseSession;
          // ⏳️ An extension round trip holds the REQUESTER's captured activation from dispatch until its
          // `Completed` frame lands, so a switch that retires the requester in between is exactly the
          // `invokeExtension dispatch failed … no actor for instance N` measured on 6018. Registering
          // it here is what lets `switchToPluginApp`'s quiesce pass wait for it.
          const releaseExtensionWork = sessionWorkRef.current.begin(requester.pluginId, requester.instanceId, "extension-invocation");
          void dispatchInvokeExtensionEffect(pluginsNow, requester, effect.invokeExtension, async (requestingPlugin, response) => {
            if (dropForSealedInstance(requester, "extension answer", effect.invokeExtension.extensionId)) throw new Error("extension.requester-retired");
            if (!isCurrentEffectOwner(effectOwner)) throw new Error("extension.requester-retired");
            const current = sessionRef.current;
            const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === requester.pluginId)?.handle;
            if (!current || handle !== requestingPlugin.handle) throw new Error("extension.requester-retired");
            const primary = current.pluginId === baseSession.pluginId && current.instanceId === baseSession.instanceId;
            const spawned = parsePanelState(current.viewState)?.spawnedApps.some((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
            if (!primary && !spawned) throw new Error("extension.requester-retired");
            applyHistoryPatch(response.historyPatch);
            applyLeftoverInteractionView(response.output);
            await applyHostEffects(response.requestedEffects ?? [], primary ? current : { ...baseSession, viewState: current.viewState }, resolveUiDirtyScope(response.uiScope), effectOwner);
          }).catch((error) => {
            const { extensionId, capability, req } = effect.invokeExtension;
            if (sealedInstancesRef.current.sealed(requester.pluginId, requester.instanceId)) return;
            console.error("[DEBUG] invokeExtension dispatch failed", { extensionId, capability, req, error });
          }).finally(releaseExtensionWork);
          continue;
        }
        if ("spawnPluginInstance" in effect) {
          const { pluginId, appId, osInstanceId, label, documentJson } = effect.spawnPluginInstance;
          const program = spacePrograms.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? spacePrograms.find((entry) => entry.pluginId === pluginId);
          if (program) {
            // 🪟️ Fold spawn into `nextViewState` — a separate SET_SESSION would be clobbered by the
            // final write below and leave the shell stuck on the studio surface.
            const nextPanel = await ensureSpawnedPlugin(program, () => isCurrentEffectOwner(effectOwner), label, osInstanceId, documentJson, nextViewState);
            if (!isCurrentEffectOwner(effectOwner)) return;
            if (nextPanel) nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
          }
          continue;
        }
        if ("openPluginInstance" in effect) {
          const { pluginId, appId, osInstanceId } = effect.openPluginInstance;
          const program = spacePrograms.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? spacePrograms.find((entry) => entry.pluginId === pluginId);
          if (program) {
            // 🪟️ Fold focus into `nextViewState` so the final SET_SESSION keeps `activeSpawnedId`
            // (opening a workflow node depends on this — otherwise nothing appears to happen).
            const nextPanel = await ensureSpawnedPlugin(program, () => isCurrentEffectOwner(effectOwner), undefined, osInstanceId, undefined, nextViewState);
            if (!isCurrentEffectOwner(effectOwner)) return;
            if (nextPanel) {
              nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
              console.log("[DEBUG] openPluginInstance focused spawned app", {
                pluginId,
                appId,
                osInstanceId,
                activeSpawnedId: nextPanel.activeSpawnedId,
                spawnedCount: nextPanel.spawnedApps.length,
              });
            }
            if (osInstanceId && openSpaceIdRef.current) {
              openInstanceIdRef.current = osInstanceId;
              navigateHistory(`/spaces/${openSpaceIdRef.current}/instances/${osInstanceId}`);
            }
          } else {
            console.warn(
              "[os-shell] openPluginInstance: no program matches",
              { pluginId, appId },
              "available:",
              catalog.map((entry) => `${entry.pluginId}/${entry.appId}`),
            );
          }
          continue;
        }
      }
      const nextSession = { ...baseSession, viewState: nextViewState };
      if (!isCurrentEffectOwner(effectOwner)) return;
      // 🐢️ Did any EFFECT above actually rewrite the view state? Reference-comparing `nextViewState`
      // against the LIVE session's own `viewState` cannot answer that: every dispatch enters here with a
      // freshly built per-call projection (`onAction`'s `dispatchViewState` — locale/terminology/
      // `windowInstances`/`activeUtilityByWindowId` re-derived from refs, plus `windowId` pinned to the
      // dispatch's target window), so the identity check below was false on EVERY action and minted a new
      // `session` object each time. That churn re-ran the history-snapshot effect (one `readHistory` guest
      // round trip per dispatch) and re-subscribed the operation-completion effect — measured 2026-09-09
      // as 7 `readHistory` calls for the 7 boot mesh pages and 90 during a 35 s fill run. Comparing
      // against `baseSession.viewState` asks the question the comment below always meant to ask, and a
      // per-call projection is never written back into the session (`refreshUi` re-derives it anyway).
      const viewStateRewrittenByEffects = nextViewState !== baseSession.viewState;
      const isSpawnedPluginSession = hostMode && !shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session);
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!shellDialogOriginIsCurrentV1(effectOrigin, captureDialogOrigin(current))) return current;
          if (!current) return current;
          if (isSpawnedPluginSession) return viewStateRewrittenByEffects ? { ...current, viewState: nextViewState } : current;
          if (!shellDialogSessionIsCurrentV1(current, nextSession)) return current;
          // 🐢️ Preserve `current`'s identity when the viewState didn't actually change — otherwise every
          // action mints a new `session` object, which cascades into a new `onAction` identity, which
          // busts every memo keyed on it (windows, panels, the boot-refresh effect below) even when
          // nothing about the session changed.
          return viewStateRewrittenByEffects ? { ...current, viewState: nextViewState } : current;
        },
      });
      // 🧰️ What the GUEST dirtied (`uiScope`) unioned with what applying these effects dirtied
      // ({@link hostEffectRefreshScopeV1}) — an armed utility is a host-owned render input, and a
      // completion that re-took nothing declares `none`, so without this union the arm is applied and
      // never published (wave B37).
      const refreshScope = hostEffectRefreshScopeV1(effects, uiScope ?? { kind: "full" }, nextSession.app.windowKinds.map((kind) => kind.bodyKey));
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState, refreshScope);
      } else if (shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, nextSession)) {
        if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] applyHostEffects refresh", JSON.stringify({ declared: uiScope, scope: refreshScope, viewStateSame: nextViewState === baseSession.viewState }));
        await refreshUi(nextSession, refreshScope, undefined, leftoverReplaceRefreshBodiesV1());
      } else {
        if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] applyHostEffects skipped refresh: session not current", JSON.stringify({ spawned: isSpawnedPluginSession, scope: refreshScope }));
      }
    },
    [captureDialogOrigin, captureEffectOwner, dropForSealedInstance, isCurrentEffectOwner, loadDocumentPair, makeOwnedDialog, clearAllWindowUtilities, ensureSpawnedPlugin, loadedPlugins, navigateHistory, refreshSpawnedUi, refreshUi, requestInferenceProposal, resolvedTargetViewState, session, setActiveUtilityForWindow, spacePrograms, hostMode],
  );
  // 🔁️ What the ui-refresh lane applies for a pass that asked for effects of its own, outside that pass.
  applyHostEffectsRef.current = applyHostEffects;

  /** 🏁️ Applies one retained typed operation's terminal publication. The command that started the
   * operation resolved on its FIRST reactor turn — its `InvocationResult` carries no outcome at all —
   * so a selection like "open the Nakagin example" used to leave the outliner, the inspection panel
   * and the History list showing the previous document forever. `PluginWasmHandle
   * .subscribeOperationCompletions` is the only carrier of that outcome: the operation's own history
   * delta, its final `UiDirtyScope`, and every host effect its continuation turns requested.
   *
   * 🫥️ What this pass is owed is decided by {@link typedOperationCompletionRefreshV1}, never by the
   * admitting reply: an admission reports `mutationCount: 0` for every typed operation, and a completion
   * that dirtied nothing, patched no history and requested no effect owes no pass at all — a retained
   * operation completes on every drain poll, so paying for those is a refresh storm (wave B27 §2). */
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    const target = session;
    try {
      return plugin.subscribeOperationCompletions(target.instanceId, (completion) => {
        // 🏁️ Release every `onAction` promise waiting on this operation BEFORE the (async) effect pass —
        // the caller's contract is "the guest work is finished", and the host-effect pass that follows is
        // this same subscription's own work, not the guest's.
        settleOperation(completion.operation);
        applyHistoryPatch(completion.historyPatch);
        const refresh = typedOperationCompletionRefreshV1(completion);
        const owner = captureEffectOwner(target, captureDialogOrigin(target));
        if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] completion apply", JSON.stringify({ operation: completion.operation, scope: completion.uiScope, refresh, historyPatch: completion.historyPatch !== undefined, ownerCurrent: isCurrentEffectOwner(owner), sessionCurrent: shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, target), targetInstance: target.instanceId, currentInstance: shellStateRef.current.pluginRuntime.session?.instanceId }));
        if (refresh === null) return;
        void applyHostEffects(completion.requestedEffects, target, refresh, owner).catch((error) => console.error("typed-operation completion effects failed", error));
      });
    } catch (error) {
      console.error("[DEBUG] typed-operation completion subscription failed", error);
      return;
    }
  }, [applyHistoryPatch, applyLeftoverInteractionView, applyHostEffects, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, session, settleOperation]);

  const applyShellUri = useCallback(
    async (uri: string, preservedViewState?: ViewModel) => {
      // 🩹️ See `applyShellUriDepthRef`'s own doc comment: turns an unbounded reentrant call chain into
      // a bounded, logged no-op instead of a JS stack overflow, and captures a real stack for the next
      // diagnosis pass.
      if (applyShellUriDepthRef.current > 0) {
        console.error(`[DEBUG] applyShellUri: reentrant call blocked at depth ${applyShellUriDepthRef.current}, uri=${uri}`, new Error("applyShellUri reentrancy").stack);
        return;
      }
      applyShellUriDepthRef.current += 1;
      try {
        const currentSession = sessionRef.current;
        if (!hostConfig || !currentSession || loadedPlugins.length === 0) return;
        const path = uri.split("?")[0] ?? "/";
        // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §5 — `/spaces/{id}/studio`
        // (optionally `/instances/{id}`) opens the workflow studio (the canonical resolved host app,
        // pre-existing behaviour every bare `/spaces/{id}` used to trigger). `parseShellRoute` (owned by
        // `ShellHelpers/🟦️.tsx`, outside this lane's lease) has no concept of a `/studio`
        // segment, so it's matched locally here first — `parseShellRoute` itself is never edited, and
        // its own existing route classification (and tests) stay exactly as they were.
        const studioMatch = /^\/spaces\/([^/]+)\/studio(?:\/instances\/([^/]+))?$/.exec(path);
        const route = studioMatch ? ({ kind: "space" as const, spaceId: studioMatch[1]!, instanceId: studioMatch[2] } as const) : parseShellRoute(path);
        const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId)?.handle;
        if (!sPlugin) return;
        if (route.kind === "landing") {
          openSpaceIdRef.current = null;
          openInstanceIdRef.current = null;
          if (!landingAppId) throw new Error("required landing app identity is unavailable");
          if (currentSession.app.id !== landingAppId) await switchToPluginApp(hostConfig.pluginId, landingAppId, preservedViewState);
          return;
        }
        if (route.kind === "notFound") {
          openSpaceIdRef.current = null;
          openInstanceIdRef.current = null;
          return;
        }
        const { spaceId, instanceId } = route;
        // 📇️ §5 — a bare `/spaces/{id}` (no `/studio`, no `/instances/{id}` deep link) now opens the
        // `s.space` artifact-index app (kind `s.space`, dialect `s.space.space@1/*`, §C4) instead of the
        // studio, resolved by dialect/surface id off the SAME "s" plugin's own manifest so this activates
        // the moment lane 2-B registers a real app for that dialect — no further change needed here.
        if (!studioMatch && !instanceId) {
          const hostPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId);
          const spaceApp = findDialectApp(hostPlugin, SPACE_INDEX_DIALECT, "editor") ?? findDialectApp(hostPlugin, SPACE_INDEX_DIALECT, "viewer");
          if (!spaceApp || !hostPlugin) {
            console.warn("[os-shell] applyShellUri: no app registered for dialect", dialectCoordinate(SPACE_INDEX_DIALECT), "— s.space (lane 2-B) not loaded yet");
            return;
          }
          // 🔁️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-I — idempotency
          // guard mirroring `studioChanged` below: without it, ANY re-render that mints a new
          // `applyShellUri` identity (e.g. `switchToPluginApp` depending on `session`, which
          // `openDocumentRef.current` below itself updates) re-fires this whole branch for the SAME
          // already-open space, tearing the document's sync session down and reopening it in a tight
          // loop — observed live as dozens of WS open/close cycles plus a `Maximum call stack size
          // exceeded` inside the 30s STEP 2 budget (`🧪️4-i-collab-e2e-run3.txt`), never previously
          // exercised because no earlier lane's fixes let a hard navigation to `/spaces/{id}` reach here.
          const spaceIndexAlreadyOpen = openSpaceIdRef.current === spaceId && currentSession.app.id === spaceApp.id;
          openSpaceIdRef.current = spaceId;
          openInstanceIdRef.current = null;
          if (spaceIndexAlreadyOpen) return;
          const spaceSession = currentSession.app.id === spaceApp.id ? currentSession : await switchToPluginApp(hostConfig.pluginId, spaceApp.id, preservedViewState);
          if (!spaceSession) return;
          await openDocumentRef.current({ documentId: S_SPACE_INDEX_DOCUMENT_ID, schema: S_SPACE_INDEX_DOCUMENT_SCHEMA, spaceId }, undefined, { session: spaceSession, plugin: hostPlugin.handle });
          return;
        }
        // 🧭️ Pin the route studio id before the async app switch so the boot example effect cannot
        // race-navigate to `/spaces/demo` while `switchToPluginApp` is still awaiting.
        const studioChanged = openSpaceIdRef.current !== spaceId;
        openSpaceIdRef.current = spaceId;
        if (!hostAppId) throw new Error("required host app identity is unavailable");
        const studioSession = currentSession.app.id === hostAppId ? currentSession : await switchToPluginApp(hostConfig.pluginId, hostAppId, preservedViewState);
        if (!studioSession) return;
        const studioControllerId = studioSession.app.controllerId;
        const routeOwner = captureEffectOwner(studioSession, captureDialogOrigin(studioSession));
        if (studioChanged) {
          openInstanceIdRef.current = null;
          console.log("[DEBUG] applyShellUri openSpace", spaceId);
          const openResponse = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "openSpace", args: { spaceId } }), studioSession.viewState);
          await applyHostEffects(openResponse.requestedEffects ?? [], studioSession, resolveUiDirtyScope(openResponse.uiScope), routeOwner);
        }
        if (openInstanceIdRef.current === (instanceId ?? null)) return;
        openInstanceIdRef.current = instanceId ?? null;
        if (instanceId) {
          const response = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "openInstance", args: { instanceId } }), studioSession.viewState);
          await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope), routeOwner);
        } else {
          const response = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "closeFocusedInstance" }), studioSession.viewState);
          const currentPanel = parsePanelState(studioSession.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
          updateSpacePanel(buildSpacePanelState(currentPanel.spawnedApps, currentPanel.activePanelTab, undefined));
          await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope), routeOwner);
        }
      } finally {
        applyShellUriDepthRef.current -= 1;
      }
    },
    [applyHostEffects, loadedPlugins, refreshUi, hostConfig, hostApp, landingAppId, hostAppId, switchToPluginApp, updateSpacePanel],
  );

  useEffect(() => {
    if (!hostMode || loadedPlugins.length === 0) return;
    void applyShellUri(shellUri).catch((uriError) => {
      console.error("[DEBUG] shell uri apply failed", uriError);
    });
  }, [applyShellUri, loadedPlugins.length, shellUri, hostMode]);

  const resolveSyncTargetSession = useCallback((): ActiveSession | null => {
    if (!session) return null;
    if (hostMode && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        if (app) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
      }
    }
    return session;
  }, [loadedPlugins, panel, session, hostMode]);

  /** 🧵️ Opens this exact document and optional shared space in its captured plugin session.
   * Shared requests require a signed-in identity and carry only a requested surface; the worker
   * verifies the Hub plan and catalog assets before acquiring socket authority. Explicit bindings
   * are used by the manual persistence picker. Local documents never inherit the current route. */
  const openDocument = useCallback(
    async (ref: DocumentOpeningReference, bindings?: readonly PersistenceBinding[], target?: OpenDocumentSessionTarget): Promise<DocumentOpeningReceiptV1 | null> => {
      const documentTarget = resolveDocumentOpeningTarget(target, resolveSyncTargetSession(), loadedPlugins);
      if (!documentTarget) return null;
      const { session: targetSession, plugin } = documentTarget;
      const worker = ensureBackboneWorker();
      const resolvedBindings = bindings ?? resolveDocumentOpeningBindings(ref, {
        identity: identityRef.current,
        dataDir: hubEnv?.dataDir,
        surface: targetSession.app.dialect ? canonicalSurfaceId(targetSession.app.dialect, targetSession.app.role) : undefined,
      });
      const hubBinding = resolvedBindings.find((binding): binding is Extract<PersistenceBinding, { kind: "hub" }> => binding.kind === "hub");
      if (target?.expectedCatalogGenerationId !== undefined && hubBinding === undefined) return null;
      const creationMount = target?.expectedCatalogGenerationId === undefined ? null : createArtifactCreationCatalogMountV1(target.expectedCatalogGenerationId);
      const scope: DocumentScope | undefined = hubBinding === undefined ? undefined : { spaceId: hubBinding.spaceId, documentId: ref.documentId };
      const runtimeKey = scope === undefined ? ref.documentId : documentRuntimeKeyV1({ kind: "hub", ...scope });
      const openingAttempt = { clientInstanceId: crypto.randomUUID() };
      const { clientInstanceId } = openingAttempt;
      let resolveReady!: () => void, rejectReady!: (error: Error) => void;
      const ready = new Promise<void>((resolve, reject) => { resolveReady = resolve; rejectReady = reject; });
      void ready.catch(() => {});
      const persistToFolder = resolvedBindings.some((binding) => binding.kind === "folder");
      const entry: OpenDocumentSession = { session: targetSession, plugin, documentId: ref.documentId, clientInstanceId, ...(scope === undefined ? {} : { scope }), port: null, pending: [], pendingBytes: 0, replacements: new LatestDocumentReplacementV1(), archivePersistence: async () => {}, creationMount, ready, resolveReady, rejectReady };
      entry.archivePersistence = latestWins(async () => {
        if (!persistToFolder || openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
        if (!plugin.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
        const archive = await plugin.readAppDocumentArchive(targetSession.instanceId);
        if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
        const archiveBytes = encodeDocumentArchiveBytes(archive);
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "localDocumentArchive", archive: Array.from(archiveBytes) } }) });
      });
      const parked = parkDocumentOpeningReplacementV1({ runtimeKey, plugin, instanceId: targetSession.instanceId, background: target?.background === true }, openDocumentSessionsRef.current, entry);
      if (parked === null) return null;
      const predecessorActor = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      if (predecessorActor !== undefined && predecessorActor.clientInstanceId !== clientInstanceId) {
        retireBrowserActorUi(runtimeKey, "browser-actor-action: opening parked");
      }
      console.warn("[DEBUG] document opening parked", JSON.stringify({ runtimeKey, clientInstanceId, restored: parked.previous?.clientInstanceId ?? null, superseded: parked.superseded.length, yieldedActor: predecessorActor?.clientInstanceId ?? null }));
      const request: BackboneWorkerRequest = {
        kind: "open",
        clientInstanceId,
        documentId: ref.documentId,
        schema: ref.schema,
        bindings: resolvedBindings,
        watchExternal: true,
        actor: shellActorIdRef.current,
      };
      const expectsSocketActor = resolvedBindings.some((binding) => binding.kind === "hub");
      const socketActor = expectsSocketActor
        ? new Promise<string>((resolve, reject) => socketActorReadyRef.current.set(runtimeKey, { clientInstanceId, resolve, reject }))
        : null;
      void socketActor?.catch(() => {});
      const uri = `actor://${runtimeKey}`;
      const committed = await runDocumentOpeningAttemptV1({
        deadlineMs: 60_000,
        current: () => openDocumentSessionsRef.current.get(runtimeKey) === entry && entry.creationMount?.current() !== false,
        socket: async () => {
          worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
          postBrowserActorViewState(worker, entry, targetSession);
          await socketActor;
        },
        retire: () => {
          const waiter = socketActorReadyRef.current.get(runtimeKey);
          if (waiter !== undefined && waiter.clientInstanceId === clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
        },
        close: () => {
          entry.creationMount?.close(new Error("document opening aborted"));
          entry.rejectReady(new Error("document opening aborted"));
          entry.replacements.invalidate();
          void entry.port?.retire().catch(() => {});
          const restored = settleDocumentOpeningReplacementV1(openDocumentSessionsRef.current, parked, false);
          const successorActor = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
          if (successorActor?.clientInstanceId === clientInstanceId) {
            retireBrowserActorUi(runtimeKey, "browser-actor-action: opening aborted");
          }
          console.warn("[DEBUG] document opening aborted — predecessor restored", JSON.stringify({ runtimeKey, clientInstanceId, restored: parked.previous?.clientInstanceId ?? null, retired: restored.length }));
        },
        detach: () => retireDocumentAttachment(plugin, targetSession.instanceId, clientInstanceId),
        attach: async () => {
          if (hubBinding && scope) {
            directoryScopedOwnersRef.current.set(runtimeKey, scope);
            worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-scope-open", baseUrl: hubBinding.baseUrl, scope, since: 0 }) });
            if (scope.documentId === S_SPACE_INDEX_DOCUMENT_ID && !target?.background) {
              setSpaceArtifactCreationCatalog(null);
              setSpaceArtifactCreationCatalogUi({ kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: scope.spaceId, phase: "loading" });
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: scope.spaceId }) });
            }
          }
          if (hubBinding || entry.replacements.pending) {
            if (entry.creationMount === null) await entry.ready;
            else await entry.creationMount.attach(entry.ready);
          }
          else await documentAttachmentLane(plugin, targetSession.instanceId).attach(clientInstanceId, () => openDocumentSessionsRef.current.get(runtimeKey) === entry, async () => {
            if (entry.port === null || entry.port.closing) await bindDocumentBackbone(runtimeKey, entry);
          });
        },
        commit: () => {
          entry.creationMount = null;
          if (target?.background) return;
          dispatch({ type: "SET_SYNC_BACKBONE_URI", value: uri });
          dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
        },
      });
      if (committed) {
        for (const [key, owner] of settleDocumentOpeningReplacementV1(openDocumentSessionsRef.current, parked, true)) {
          if (key === runtimeKey) {
            void owner.port?.retire().catch(() => {});
            void retireDocumentAttachment(owner.plugin, owner.session.instanceId, owner.clientInstanceId).catch((error) => console.error("[DEBUG] parked predecessor attachment retirement failed", error));
            backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: owner.documentId, clientInstanceId: owner.clientInstanceId, ...(owner.scope === undefined ? {} : { spaceId: owner.scope.spaceId }) }) });
            continue;
          }
          closeDocumentRef.current(key, owner.clientInstanceId);
        }
        console.warn("[DEBUG] document opening committed", JSON.stringify({ runtimeKey, clientInstanceId, actor: browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.clientInstanceId ?? null }));
      }
      return committed ? { committed: true, runtimeKey, clientInstanceId } : null;
    },
    [bindDocumentBackbone, documentAttachmentLane, ensureBackboneWorker, loadedPlugins, postBrowserActorViewState, resolveSyncTargetSession, hubEnv, retireDocumentAttachment, retireBrowserActorUi],
  );
  openDocumentRef.current = openDocument;

  useEffect(() => {
    const worker = backboneWorkerRef.current;
    if (worker === null) return;
    for (const entry of openDocumentSessionsRef.current.values()) {
      if (entry.scope === undefined) continue;
      const current = sessionRef.current;
      const targetSession = current?.pluginId === entry.session.pluginId && current.instanceId === entry.session.instanceId ? current : entry.session;
      postBrowserActorViewState(worker, entry, targetSession);
    }
  }, [activeToolId, activeUtilityByWindowId, extraWindowInstances, postBrowserActorViewState, session?.viewState]);

  const closeDocument = useCallback((runtimeKey: string, clientInstanceId?: string) => {
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    if (!entry) return;
    if (clientInstanceId !== undefined && entry.clientInstanceId !== clientInstanceId) return;
    console.warn("[DEBUG] closeDocument", JSON.stringify({ runtimeKey, clientInstanceId: entry.clientInstanceId, instanceId: entry.session.instanceId }));
    entry.creationMount?.close(new Error("document closed"));
    entry.rejectReady(new Error("document closed"));
    entry.replacements.invalidate();
    const retirement = entry.port?.retire();
    void retirement?.catch(error => console.error("[DEBUG] document backbone retirement failed", error));
    entry.port = null;
    entry.pending = [];
    entry.pendingBytes = 0;
    cancelSpaceArtifactCreationsForRuntime(runtimeKey, backboneWorkerRef.current);
    if (entry.scope?.documentId === S_SPACE_INDEX_DOCUMENT_ID) {
      void backgroundSpaceIndexSessionsRef.current.retire(entry.scope.spaceId, owned => owned.clientInstanceId === entry.clientInstanceId).catch(error => console.error("[DEBUG] background document retirement failed", error));
      setSpaceArtifactCreationCatalog((catalog) => catalog?.clientInstanceId === entry.clientInstanceId ? null : catalog);
      setSpaceArtifactCreationCatalogUi((status) => status?.clientInstanceId === entry.clientInstanceId ? null : status);
    }
    const waiter = socketActorReadyRef.current.get(runtimeKey);
    if (waiter?.clientInstanceId === entry.clientInstanceId) {
      waiter.reject(new Error("document closed"));
      socketActorReadyRef.current.delete(runtimeKey);
    }
    const pendingInference = inferencePortOpeningRef.current;
    if (pendingInference?.owner.runtimeKey === runtimeKey) pendingInference.mailbox.close("inference-opening: document retired");
    const inferenceOwner = inferencePortOwnerRef.current;
    const retainedInferenceOwner = retainInferencePortOwnerAfterCloseV1(inferenceOwner, runtimeKey);
    if (inferenceOwner !== null && retainedInferenceOwner === null) {
      backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-close", operationEpoch: inferenceOwner.operationEpoch }) });
      inferencePortOwnerRef.current = retainedInferenceOwner;
      inferencePortAuthorityRef.current = null;
      inferencePortEpochRef.current = inferenceOwner.operationEpoch + 1;
      dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey });
    }
    void retireDocumentAttachment(entry.plugin, entry.session.instanceId, entry.clientInstanceId).catch(error => console.error("[DEBUG] document attachment retirement failed", error));
    openDocumentSessionsRef.current.delete(runtimeKey);
    const dialog = liveDialogRef.current;
    if (dialog?.origin.document?.runtimeKey === runtimeKey && dialog.origin.document.clientInstanceId === entry?.clientInstanceId) closeOwnedDialog(dialog.openingId);
    const tutorial = tutorialRunRef.current;
    if (tutorial?.origin.document?.runtimeKey === runtimeKey && tutorial.origin.document.clientInstanceId === entry?.clientInstanceId) {
      tutorialTransitionEpochRef.current += 1;
      tutorialDrivenRef.current.retire();
      tutorialClockRef.current?.pause();
      void tutorial.stop().catch((error) => console.error("[DEBUG] tutorial retirement failed", error));
      dispatch({ type: "SET_TUTORIAL", value: null });
    }
    retireBrowserActorUi(runtimeKey, "browser-actor-action: document retired");
    rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
    setInferenceHistoryByRuntimeKey((current) => {
      if (!(runtimeKey in current)) return current;
      const next = { ...current };
      delete next[runtimeKey];
      return next;
    });
    setPresencePeersByRuntimeKey((current) => {
      if (!(runtimeKey in current)) return current;
      const next = { ...current };
      delete next[runtimeKey];
      return next;
    });
    setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "detached", documentId: entry.documentId, ...(entry.scope === undefined ? {} : { scope: entry.scope }) }));
    setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, { kind: "execution-target-cleared", documentId: entry.documentId, ...(entry.scope === undefined ? {} : { scope: entry.scope }) }));
    if (entry.scope !== undefined && directoryScopedOwnersRef.current.delete(runtimeKey)) {
      backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-scope-close", scope: entry.scope }) });
    }
    const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }) };
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, [retireBrowserActorUi]);
  closeDocumentRef.current = closeDocument;

  /** @deprecated superseded by {@link openDocument}; kept as a thin URI-parsing adapter only for the
   * existing sync-card UI (`onAction`'s `attach` handler below), which still collects a single uri
   * from file/folder/remote pickers — translates that uri into an `OsDocumentRef` + `PersistenceBinding`. */
  const attachSyncBackbone = useCallback(
    async (uri: string) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const documentId = syncDocumentId(targetSession, panel, hostMode);
      const bindings: PersistenceBinding[] = uri.startsWith("remote://")
        ? (() => {
            const rest = uri.slice("remote://".length);
            const slash = rest.indexOf("/");
            const baseUrl = slash > 0 ? `http://${rest.slice(0, slash)}` : `http://${rest}`;
            const spaceId = slash > 0 ? rest.slice(slash + 1) || "default" : "default";
            return [{ kind: "hub", baseUrl, spaceId }];
          })()
        : uri.startsWith("folder://")
          ? [{ kind: "folder", path: uri.slice("folder://".length) }]
          : uri.startsWith("file://")
            ? [{ kind: "folder", path: uri.slice("file://".length).replace(/\/[^/]*$/, "") }]
            : [];
      await openDocument({ documentId, schema: targetSession.app.breadcrumb.join(".") }, bindings);
    },
    [openDocument, panel, resolveSyncTargetSession, hostMode],
  );

  const detachSyncBackbone = useCallback(() => {
    if (syncBackboneUri) closeDocument(syncBackboneUri.replace(/^actor:\/\//, ""));
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [closeDocument, syncBackboneUri]);

  const spawnProgram = useCallback(
    async (program: SpaceProgramEntry) => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return;
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      const spawnedId = `${program.pluginId}-${instanceId}`;
      updateSpacePanel(
        studioPanelFocusingSpawned(currentPanel, {
          id: spawnedId,
          pluginId: program.pluginId,
          instanceId,
          appId: program.appId,
          label: program.label,
          breadcrumb: program.breadcrumb,
        }),
      );
    },
    [loadedPlugins, session, updateSpacePanel, hostApp],
  );

  const onAction = useCallback(
    (requested: ActionDescriptor, submittedOrigin?: ShellDialogOriginV1, propagateFailure = false) => {
      const action = pasteActionWithRetainedFragment(requested, clipboardFragmentRef.current);
      if (action.controllerId === "recovery") {
        const args = typeof action.args === "object" && action.args != null ? (action.args as { pluginId?: string }) : {};
        const pluginId = args.pluginId ?? primaryPluginId;
        if (!pluginId) return;
        if (action.action === "recovery.restartApp") {
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
          void reloadPlugin(pluginId);
          return;
        }
        if (action.action === "recovery.disablePlugin") {
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "quarantined" });
          if (pluginId !== primaryPluginId) void uninstallPlugin(pluginId);
          return;
        }
        if (action.action === "recovery.showDiagnostics") {
          console.log("[DEBUG] recovery diagnostics", { pluginId, supervisor: pluginSupervisorById[pluginId] });
          return;
        }
      }

      if (!session) return;
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
        console.warn("[DEBUG] extension requests aborted by surface cancel", JSON.stringify({ action: action.action, pluginId: session.pluginId, instanceId: session.instanceId, aborted }));
      }
      const actionOrigin = submittedOrigin ?? captureDialogOrigin(session);
      if (!isCurrentDialogOrigin(actionOrigin)) return;
      const primaryActionOwner = captureEffectOwner(session, actionOrigin);

      // 🎓️ First-run walkthrough (mirrors setActiveUtility below): fully shell-intercepted, resets
      // playback to the first step, never forwarded to the program.
      if (action.action === START_INTRODUCTION_ACTION_ID) {
        dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
        return;
      }

      // 🎥️ Fully shell-intercepted, mirroring `START_INTRODUCTION_ACTION_ID` above: sandboxes the
      // document and starts tutorial playback from t=0 (real work happens in `startTutorialRef`, wired up
      // by the TutorialOrchestration block further down this component).
      if (action.action === START_TUTORIAL_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? (action.args as { tutorialId?: unknown }) : {};
        if (typeof args.tutorialId === "string") startTutorialRef.current(args.tutorialId);
        return;
      }
      if (action.action === RECORD_TUTORIAL_ACTION_ID) {
        toggleTutorialRecordingRef.current();
        return;
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
        return;
      }

      // 🧰️ Utility activation (P5): host-owned session state, never a document operation. Re-clicking the active
      // utility (or an empty utilityId) deactivates. We resolve the target window from the descriptor's tagged
      // `windowId` (see `tagSetActiveUtilityWindow`), falling back to the active window, update the store,
      // then forward the resolved utility to the plugin so it can clear/prepare scratch.
      if (action.action === SET_ACTIVE_UTILITY_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? (action.args as { utilityId?: unknown; windowId?: unknown }) : {};
        const windowId = typeof args.windowId === "string" && args.windowId ? args.windowId : (activeWindowIdRef.current ?? "");
        if (!windowId) return;
        const requested = typeof args.utilityId === "string" ? args.utilityId : "";
        const next = resolveUtilityActivation(activeUtilityByWindowIdRef.current[windowId], requested);
        if (!next && performance.now() - lastUtilityArmAtRef.current < 8000) {
          console.warn(`[DEBUG] setActiveUtility hop ignored echo-off window=${windowId} requested=${requested}`);
          return;
        }
        if (next) lastUtilityArmAtRef.current = performance.now();
        setActiveUtilityForWindow(windowId, next);
        // 🛠️ A tool and a window utility are mutually exclusive interaction owners — activating a real
        // utility clears any active mode-level tool.
        if (next && activeToolIdRef.current) {
          activeToolIdRef.current = null;
          dispatch({ type: "SET_ACTIVE_TOOL", toolId: null });
        }
        if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "utility" && interaction.on.id === next);
        const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId);
        const program = pluginEntry?.handle;
        if (program) {
          const viewState = windowViewContext(
            {
              ...session.viewState,
              locale: uiLocaleRef.current,
              terminology: uiTerminologyRef.current,
              activeToolId: next ? undefined : activeToolIdRef.current ?? undefined,
              activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
              windowInstances: sessionWindowInstances(session.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
            },
            windowId,
          );
          if (!viewState) return;
          const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { utilityId: next } };
          console.warn(`[DEBUG] setActiveUtility hop window=${windowId} next=${next ?? ""}`);
          void program
            .handleAction(session.instanceId, encodeWindowActionInvocation({ ...session, viewState }, forwarded, extraWindowInstancesRef.current, windowId), viewState)
            .then((response) => {
              applyHistoryPatch(response.historyPatch);
            applyLeftoverInteractionView(response.output, forwarded.action, windowId);
              if (!isCurrentEffectOwner(primaryActionOwner)) return;
              return applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, resolveUiDirtyScope(response.uiScope), primaryActionOwner);
            })
            .catch((utilityError) => console.error("[DEBUG] setActiveUtility failed", utilityError));
        }
        return;
      }

      // 🛠️ Tool activation: host-owned session state (mode-scoped, windowless), never a document operation.
      // Re-clicking the active tool (or an empty toolId) deactivates. Mutually exclusive with every
      // window's active utility — activating a tool clears them all, mirroring `SET_ACTIVE_UTILITY_ACTION_ID`.
      if (action.action === SET_ACTIVE_TOOL_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? (action.args as { toolId?: unknown }) : {};
        const requested = typeof args.toolId === "string" ? args.toolId : "";
        const next = resolveUtilityActivation(activeToolIdRef.current, requested);
        activeToolIdRef.current = next;
        dispatch({ type: "SET_ACTIVE_TOOL", toolId: next });
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
        const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId);
        const program = pluginEntry?.handle;
        if (program) {
          const toolWindowId = activeWindowIdRef.current ?? undefined;
          const baseToolViewState: ViewModel = {
            ...session.viewState,
            locale: uiLocaleRef.current,
            terminology: uiTerminologyRef.current,
            activeToolId: next ?? undefined,
            activeUtilityId: next ? undefined : session.viewState.activeUtilityId,
            windowInstances: sessionWindowInstances(session.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
            activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
          };
          const viewState = toolWindowId ? windowViewContext(baseToolViewState, toolWindowId) : panelViewContext(baseToolViewState);
          if (!viewState) return;
          const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { toolId: next } };
          void program
            .handleAction(session.instanceId, encodeWindowActionInvocation({ ...session, viewState }, forwarded, extraWindowInstancesRef.current, toolWindowId), viewState)
            .then((response) => {
              applyHistoryPatch(response.historyPatch);
            applyLeftoverInteractionView(response.output, forwarded.action);
              if (!isCurrentEffectOwner(primaryActionOwner)) return;
              return applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, { kind: "full" }, primaryActionOwner);
            })
        }
        return;
      }

      completeIntroductionInteraction((interaction) => interaction.on.kind === "action" && interaction.on.id === action.action);

      if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
        if (action.action === "selectFile") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "file" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("file://") ? syncBackboneUri.slice("file://".length) : "" });
          return;
        }
        if (action.action === "selectFolder") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "folder" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("folder://") ? syncBackboneUri.slice("folder://".length) : "" });
          return;
        }
        if (action.action === "selectRemote") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "remote" });
          const remote = syncBackboneUri?.startsWith("remote://") ? syncBackboneUri.slice("remote://".length) : "";
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: remote });
          return;
        }
        if (action.action === "attach") {
          const path = typeof action.args === "object" && action.args != null && "path" in action.args ? String((action.args as { path?: string }).path ?? "") : syncDraftPath;
          if (!path.trim()) return;
          const uri =
            action.args && typeof action.args === "object" && "kind" in action.args
              ? String((action.args as { kind?: string }).kind) === "remote"
                ? (() => {
                    const [hostPort, ...rest] = path.split("/");
                    const [spaceId, documentId] = rest.length >= 2 ? [rest[0], rest.slice(1).join("/")] : ["default", rest[0] || syncDocumentId(session, panel, hostMode)];
                    return buildRemoteBackboneUri(hostPort ?? "127.0.0.1:8787", spaceId, documentId);
                  })()
                : String((action.args as { kind?: string }).kind) === "folder"
                  ? buildFolderBackboneUri(path)
                  : buildFileBackboneUri(path)
              : buildFileBackboneUri(path);
          void attachSyncBackbone(uri);
          return;
        }
        if (action.action === "detach") {
          void detachSyncBackbone();
          return;
        }
        return;
      }

      if (hostMode && action.controllerId === landingControllerId && action.action === "importSpace") {
        importSpaceInputRef.current?.click();
        return;
      }

      if (hostMode && action.action === "spawnApp" && action.controllerId !== hostControllerId) {
        const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String((action.args as { pluginId?: string }).pluginId ?? "") : "";
        const program = spacePrograms.find((entry) => entry.pluginId === pluginId);
        if (program) void spawnProgram(program);
        return;
      }

      if (hostMode && action.action === "setActivePanelTab" && (action.controllerId === hostControllerId || action.controllerId === session.app.controllerId)) {
        const tabId = typeof action.args === "object" && action.args != null && typeof (action.args as { tabId?: unknown }).tabId === "string" ? (action.args as { tabId: string }).tabId : "";
        const targetApp = action.controllerId === session.app.controllerId ? session.app : hostApp;
        if (!targetApp || tabId.length === 0 || Array.from(tabId).length > 256 || /[\u0000-\u001f\u007f]/u.test(tabId)) return;
        const leaf = flattenPanelTabLeaves(targetApp.panelTabs).find((tab) => panelTabKindId(tab.kind) === tabId);
        const path = panelDefinitionPath(targetApp.panelTabs, tabId);
        if (!leaf || !path) return;
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
        return;
      }

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
              return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
            })()
          : session;
      if (action.action === "undo") {
        const mounted = Object.entries(inferenceHistoryByRuntimeKeyRef.current).filter(([runtimeKey, history]) => {
          const document = openDocumentSessionsRef.current.get(runtimeKey);
          return document?.session.pluginId === targetSession.pluginId && document.session.instanceId === targetSession.instanceId && document.clientInstanceId === history.clientInstanceId && history.sessionInstanceId === targetSession.instanceId;
        });
        const remote = mounted.length === 1 ? mounted[0] : undefined;
        const route = shellHistoryUndoRouteV1(remote?.[1] === undefined ? null : { ...remote[1].status, order: remote[1].order }, { canUndo: historyProjection.canUndo, order: localHistoryOrderRef.current });
        console.warn("[DEBUG] undo route", JSON.stringify({ route, canUndo: historyProjection.canUndo, localOrder: localHistoryOrderRef.current, mounted: mounted.length }));
        if (route === "blocked") return;
        if (route === "remote") {
          const history = remote![1];
          if (!directorySessionAuthorityIsCurrentV1(history.authority, verifiedSessionAuthorityRef.current)) return;
          ensureBackboneWorker().postMessage({ wire: encodeBackboneWorkerRequest({ kind: "inference-history-undo", historyEpoch: history.historyEpoch, clientInstanceId: history.clientInstanceId, scope: history.scope }) });
          return;
        }
      }
      // ⏪️ Reserved history verbs act on the DOCUMENT owner, not the app-chrome session the navbar/
      // keybinding dispatch carries — the retained browser actor is keyed to the open document session,
      // so without this remap `directBrowserActorForSession` misses and the dispatch dies on the retired
      // `plugin.handleAction` path (ticket 26/09/02/PUZZLE-3D-END-TO-END).
      if (action.action === "undo" || action.action === "redo") {
        const directNow = (() => { try { return directBrowserActorForSession(targetSession) !== null ? "yes" : "null"; } catch (error) { return `throw:${String(error)}`; } })();
        const owners = [...openDocumentSessionsRef.current.values()].map((entry) => ({ pluginId: entry.session.pluginId, instanceId: entry.session.instanceId }));
        console.warn("[DEBUG] undo remap state", JSON.stringify({ directNow, sameAsSession: targetSession === session, targetPluginId: targetSession.pluginId, targetInstanceId: targetSession.instanceId, sessionInstanceId: session?.instanceId, owners }));
        if (directNow === "null") {
          const documentOwners = [...openDocumentSessionsRef.current.values()].filter((entry) => entry.session.pluginId === targetSession.pluginId);
          if (documentOwners.length === 1) {
            targetSession = documentOwners[0]!.session;
            console.warn("[DEBUG] undo remapped to document session", JSON.stringify({ instanceId: targetSession.instanceId }));
          }
        }
      }
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      if (!plugin) return;
      // 🪦️ A control that outlived its session (a long-lived canvas host's own callback, a queued
      // pointer gesture) still addresses the instance a switch sealed. Dropping here is what turns the
      // twenty `no actor for instance N` stacks the 6018 journey logged after one role chord into one
      // typed line per late dispatch.
      if (dropForSealedInstance(targetSession, "action", action.action)) return;
      const actionOwner = captureEffectOwner(targetSession, actionOrigin);
      if (!isCurrentEffectOwner(actionOwner)) {
        if (action.action === "undo" || action.action === "redo") console.warn("[DEBUG] history route blocked effect-owner", JSON.stringify({ action: action.action }));
        return;
      }

      // 🚫️ The old `setDocument` → `patchAppSource` mirror (spawned-instance content write-back on the
      // os document) is deleted — app content no longer embeds on the os document at all
      // (`OsAppInstance.document` is now just an `OsDocumentRef` handle). A spawned instance's content
      // sync now goes through its own `openDocument`-opened `DocumentHost` channel, same as any other
      // document; there is no host-side JS mirroring step anymore.
      // 🪟️ `windowId` is read back off the tagged `action.args` (see `windowMeasuresChrome`/`tagSetActiveUtilityWindow`),
      // falling back to the active window — stamped into the dispatched view state so the plugin can key any
      // per-window option mutation off `view_state.windowId` instead of ever guessing at the active window.
      const actionWindowId = typeof action.args === "object" && action.args != null && typeof (action.args as { windowId?: unknown }).windowId === "string" ? (action.args as { windowId: string }).windowId : undefined;
      const dispatchWindowId = actionWindowId ?? activeWindowIdRef.current ?? undefined;
      const baseDispatchViewState: ViewModel = {
        ...targetSession.viewState,
        locale: uiLocale,
        terminology: uiTerminology,
        windowInstances: sessionWindowInstances(targetSession.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        focusedWindowId: activeWindowIdRef.current ?? undefined,
      };
      const dispatchViewState = hostArmedViewContext(baseDispatchViewState, activeToolIdRef.current, dispatchWindowId);
      if (!dispatchViewState) {
        if (action.action === "undo" || action.action === "redo") console.warn("[DEBUG] history route blocked view-state", JSON.stringify({ action: action.action, windowId: dispatchWindowId ?? null }));
        return;
      }
      // 🚨️ Undeclared-action drop — ALWAYS visible, never `[DEBUG]`/diagnostics-gated: this is the one
      // place a fully wired binding dies without a fault reaching anyone, so it names the app, the action
      // and the dispatching window kind (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
      const undeclared = undeclaredActionDiagnostic(targetSession.app.id, action.action, targetSession.app.windowKinds, (baseDispatchViewState.windowInstances ?? []).find((instance) => instance.id === dispatchWindowId)?.windowKindId ?? null);
      if (undeclared) {
        if (action.action === "undo" || action.action === "redo") console.warn("[DEBUG] history route blocked undeclared", JSON.stringify({ action: action.action }));
        console.error(undeclared.message, undeclared);
        return;
      }
      // 👁️✏️ Client-side half of the read-only guarantee (contract freeze §2.3/§5) — the SDK-side
      // `VcsArtifactApp` guard is the source of truth (a `ArtifactViewer`-declared session can never
      // even construct a `Mutation`-kind action), this just avoids a pointless round trip and shows the
      // same notice a `"viewer.read-only"` fault reply gets in the `.catch` below. `showTransientNotice`/
      // `isViewerReadOnlyFault` are deliberately NOT in this callback's dep list below — both are stable
      // across renders (refs + `dispatch` only), and are declared later in this component, so adding
      // them would read a not-yet-initialized `const` on the render that first creates this callback.
      if (targetSession.app.role === "viewer" && targetSession.app.windowKinds.some((kind) => (kind.actions ?? []).some((entry) => entry.id === action.action && entry.kind === "mutation"))) {
        showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
        return;
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

      if (action.action === "openImportFixture") {
        console.warn("[DEBUG] import-picker hop host-arm openImportFixture");
        void requestFileOpen("application/json,.json", "text", false)
          .then(async (opened) => {
            console.warn(`[DEBUG] import-picker opened=${opened.length} name=${opened[0]?.name ?? "none"} bytes=${opened[0]?.contents.length ?? 0}`);
            if (!opened[0]) return;
            onAction({ controllerId: action.controllerId, action: "importFixture", args: { payload: opened[0].contents, name: opened[0].name } });
          })
          .catch((error) => console.error("[DEBUG] import-picker host-arm failed", error));
        return;
      }
      const interactiveAction = action.action !== "suggestionsTick" && action.action !== "fillBuildTick";
      let directBrowserActor: ReturnType<typeof directBrowserActorForSession>;
      try {
        directBrowserActor = directBrowserActorForSession(targetSession);
      } catch (error) {
        if (propagateFailure) throw error;
        console.error("[DEBUG] authenticated browser actor action owner failed", error);
        return;
      }
      if (directBrowserActor !== null) {
        if (action.action === "undo" || action.action === "redo") console.warn("[DEBUG] history route action=" + action.action);
        if (interactiveAction) beginInteractivePluginAction();
        return dispatchDirectBrowserActorCommand(
          directBrowserActor,
          windowActionInvocation({ ...targetSession, viewState: dispatchViewState }, action, extraWindowInstancesRef.current, dispatchWindowId),
          dispatchViewState,
        ).catch((actionError) => {
          if (propagateFailure) throw actionError;
          console.error("[DEBUG] authenticated browser actor action failed", action.action, action.args, actionError);
          showTransientNotice(shellLabel("ui.common.renderError"), "error");
        }).finally(() => {
          if (interactiveAction) endInteractivePluginAction();
        });
      }
      if (action.action === "undo" || action.action === "redo") console.warn("[DEBUG] history route fallback handleAction", JSON.stringify({ action: action.action }));
      if (interactiveAction) beginInteractivePluginAction();
      // ⏳️ The whole round trip — admitting turn, host-effect pass and the `OperationCompleted` frame
      // `awaitOperationSettle` waits for — is what a switch has to outlive, so the ledger entry spans
      // the entire chain, not just `handleAction`'s own promise.
      const releaseActionWork = sessionWorkRef.current.begin(targetSession.pluginId, targetSession.instanceId, "typed-operation");
      return plugin
        .handleAction(targetSession.instanceId, encodeWindowActionInvocation({ ...targetSession, viewState: dispatchViewState }, action, extraWindowInstancesRef.current, dispatchWindowId), dispatchViewState)
        .then(async (response) => {
          if (action.action === "undo" || action.action === "redo") console.warn("[DEBUG] undo handleAction resolved", JSON.stringify({ uiScope: response.uiScope, effects: (response.requestedEffects ?? []).length, historyUpserts: response.historyPatch?.upserts?.length ?? 0, historyCanUndo: response.historyPatch?.canUndo ?? null }));
          applyHistoryPatch(response.historyPatch);
            applyLeftoverInteractionView(response.output, action.action, dispatchWindowId);
          const navbarExample = navbarExampleIdFromHistoryUpserts(response.historyPatch?.upserts, lastDispatchedExampleIdRef.current, resolveBootExampleId("", exampleOptionsRef.current, defaults.exampleId));
          if (navbarExample !== undefined) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: navbarExample });
          const needsHistoryRefresh = historyRefreshNeededV1(action.action, response.historyPatch);
          if (!isCurrentEffectOwner(actionOwner)) {
            if (needsHistoryRefresh) refreshHistorySnapshot(targetSession.instanceId);
            return;
          }
          await applyHostEffects(response.requestedEffects ?? [], { ...targetSession, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope), actionOwner);
          if (OBSERVED_INTERACTION_ACTION_IDS.has(action.action)) observeLocalInteraction({ plugin, instanceId: targetSession.instanceId });
          // 🏁️ `handleAction` answers on the guest's FIRST reactor turn — a typed command is only ADMITTED
          // there, its work runs on later turns and lands as an `OperationCompleted` frame. Awaiting that
          // frame here is what makes this promise mean "the action finished", which every self-gating
          // background tick loop depends on (`ComponentSceneHostProps.onAction`).
          await awaitOperationSettle(response.output);
          if (needsHistoryRefresh) refreshHistorySnapshot(targetSession.instanceId);
        })
        .catch((actionError) => {
          if (propagateFailure) throw actionError;
          if (isViewerReadOnlyFault(actionError)) {
            showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
            return;
          }
          if (isMutationRejectedFault(actionError)) {
            showMutationRejectedNotice((actionError as SemioFaultError).fault);
            return;
          }
          if (dropForSealedInstance(targetSession, "action failure", action.action)) return;
          console.error("[DEBUG] action failed", action.action, action.args, actionError);
        })
        .finally(() => {
          releaseActionWork();
          if (interactiveAction) endInteractivePluginAction();
        });
    },
    [
      applyHostEffects,
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
      attachSyncBackbone,
      clearAllWindowUtilities,
      detachSyncBackbone,
      injectActiveUtility,
      loadedPlugins,
      observeLocalInteraction,
      panel,
      session,
      setActiveUtilityForWindow,
      spawnProgram,
      hostMode,
      syncBackboneUri,
      syncDraftPath,
      updateSpacePanel,
      hostControllerId,
      hostApp,
      landingControllerId,
      hostCatalogueTabId,
      historyProjection.canUndo,
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
   * No-ops when there's no active app session. */
  const noteShellCommand = useCallback(
    (commandId: string, label: string, detail?: Record<string, unknown>) => {
      if (!session) return;
      onAction(buildNoteShellCommandAction(session.app.controllerId, commandId, label, detail));
    },
    [session, onAction],
  );

  const onActionRef = useRef(onAction);
  useEffect(() => {
    onActionRef.current = onAction;
  }, [onAction]);

  // 🐢️ `onAction`'s own identity churns every action (its deps include `session`, `panel`, …). Render
  // trees built from `UiNode`s only need a *callable* action dispatcher, not a fresh one each time —
  // route them through this permanently-stable ref indirection so `interpretUiNode`'s `React.memo`
  // (and any `useMemo` keyed on the dispatcher passed to it) can actually bail.
  const onActionStable = useCallback((action: Parameters<typeof onAction>[0]) => onActionRef.current(action), []);
  const onIntentStable = useCallback((intent: Parameters<typeof uiIntentToActionDescriptor>[0]) => onActionStable(uiIntentToActionDescriptor(intent)), [onActionStable]);
  const onBrowserActorIntent = useCallback((runtimeKey: string, captured: RetainedBrowserActorUiV1, intent: Parameters<typeof uiIntentToActionDescriptor>[0]) => {
    const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    if (retained?.actions !== captured.actions || retained.identity === null || entry?.clientInstanceId !== captured.clientInstanceId || entry.session.instanceId !== captured.sessionInstanceId || !shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session)) return;
    const current = (): boolean => {
      const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      const opening = openDocumentSessionsRef.current.get(runtimeKey);
      return live?.actions === captured.actions && live.identity !== null && opening?.clientInstanceId === captured.clientInstanceId && opening.session.instanceId === captured.sessionInstanceId && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, opening.session);
    };
    beginInteractivePluginAction();
    void retained.actions.dispatchIntent({
      scope: retained.scope,
      verifiedSurfaceId: retained.verifiedSurfaceId,
      activationGeneration: retained.activationGeneration,
      appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
      instanceId: retained.identity.instanceId,
      surfaceRevision: retained.store.getRevisionSnapshot(),
    }, retained.windowKindId, intent).then((result) => {
      return publishBrowserActorHostEffectsV1(result.hostEffects, current, (effect) => {
        if ("requestInferenceProposal" in effect) return requestInferenceProposal(entry.session, current);
        else window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
      });
    }).catch((error) => {
      if (browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.actions !== captured.actions) return;
      console.error("[DEBUG] authenticated browser actor intent failed", error);
      showTransientNotice(shellLabel("ui.common.renderError"), "error");
    }).finally(endInteractivePluginAction);
  }, [requestInferenceProposal]);
  const refuseBrowserActorActionDescriptor = useCallback(() => {
    console.error("[DEBUG] authenticated browser actor requires the complete UI intent");
  }, []);

  //#region 🎥️TutorialOrchestration
  /** ⏱️ Real-time throttle for the director's UI/document/event application (~10Hz) — camera stays
   * smooth every clock tick regardless (see the `subscribe` callback below). */
  const TUTORIAL_DIRECTOR_TICK_MS = 90;

  const activeTutorial = useMemo(() => activeTutorials.find((tutorial) => tutorial.id === activeTutorialId) ?? null, [activeTutorials, activeTutorialId]);

  const tutorialClockRef = useRef<TutorialClock | null>(null);
  if (!tutorialClockRef.current) tutorialClockRef.current = createTutorialClock(activeTutorial?.durationMs ?? 0);
  const tutorialClock = tutorialClockRef.current;
  useEffect(() => () => {
    const run = tutorialRunRef.current;
    tutorialRunRef.current = null;
    tutorialTransitionEpochRef.current += 1;
    tutorialDrivenRef.current.retire();
    void run?.stop().catch((error) => console.error("[DEBUG] tutorial retirement failed", error));
    tutorialClockRef.current?.dispose();
  }, []);
  useEffect(() => {
    tutorialClock.setDurationMs(activeTutorial?.durationMs ?? 0);
  }, [activeTutorial?.durationMs, tutorialClock]);
  useEffect(() => {
    tutorialClock.setRate(tutorialRate);
  }, [tutorialRate, tutorialClock]);
  useEffect(() => {
    if (tutorialPlaying && !tutorialDrivenRef.current.busy) tutorialClock.play();
    else tutorialClock.pause();
  }, [tutorialPlaying, tutorialClock]);

  const publishTutorialInteractionSelection = useCallback((selection: LocalInteractionState["selection"]) => {
    const active = sessionRef.current;
    if (!active || !tutorialRunRef.current?.ready) return;
    for (const action of tutorialInteractionSelectionActions(active.app.controllerId, selection)) onActionRef.current(action);
  }, []);
  const tutorialRunRef = useRef<OwnedTutorialRunV1<DocumentArchivePack> | null>(null);
  const tutorialTransitionEpochRef = useRef(0);
  const tutorialInitializingRunRef = useRef<OwnedTutorialRunV1<DocumentArchivePack> | null>(null);
  const uiBridgeCtxRef = useRef<TutorialUiBridgeContext>({
    session,
    restoreDialog: (dialogId, seedArgs) => {
      const run = tutorialRunRef.current;
      return run?.ready ? makeOwnedDialog(dialogId, run.origin, seedArgs) : null;
    },
    appLabelsOverlay,
    terminology: uiTerminology,
    locale: uiLocale,
    interactionSelection: () => shellStateRef.current.interaction.selection,
    publishInteractionSelection: publishTutorialInteractionSelection,
  });
  uiBridgeCtxRef.current = {
    session,
    restoreDialog: (dialogId, seedArgs) => {
      const run = tutorialRunRef.current;
      return run?.ready ? makeOwnedDialog(dialogId, run.origin, seedArgs) : null;
    },
    appLabelsOverlay,
    terminology: uiTerminology,
    locale: uiLocale,
    interactionSelection: () => shellStateRef.current.interaction.selection,
    publishInteractionSelection: publishTutorialInteractionSelection,
  };

  /** ⏱️ Playhead (ms) the director/seek last applied document/UI tracks up to — the "from" side of the
   * next `tutorialSlice(def, from, to)` call. Reset to 0 on sandbox (re)start. */
  const tutorialLastAppliedMsRef = useRef(0);
  useEffect(() => {
    const run = tutorialRunRef.current;
    if (run === null) {
      if (activeTutorialId !== null) dispatch({ type: "SET_TUTORIAL", value: null });
      return;
    }
    if (!run.isCurrent() || activeTutorialId !== run.tutorialId || session === null) {
      tutorialClock.pause();
      tutorialDrivenRef.current.retire();
      if (activeTutorialId !== null) dispatch({ type: "SET_TUTORIAL", value: null });
      void run.stop().catch((error) => console.error("[DEBUG] tutorial sandbox restore failed", error)).finally(() => {
        if (tutorialRunRef.current !== run) return;
        tutorialRunRef.current = null;
      });
      return;
    }
    if (tutorialInitializingRunRef.current === run) return;
    const def = activeTutorials.find((tutorial) => tutorial.id === run.tutorialId);
    if (def === undefined) {
      tutorialDrivenRef.current.retire();
      void run.stop().catch((error) => console.error("[DEBUG] tutorial retirement failed", error));
      dispatch({ type: "SET_TUTORIAL", value: null });
      return;
    }
    tutorialInitializingRunRef.current = run;
    const driveToken = tutorialDrivenRef.current.claim();
    void (async () => {
      if (!await run.start() || tutorialRunRef.current !== run || !run.isCurrent() || !tutorialDrivenRef.current.accepts(driveToken)) return;
      if (def.base.exampleId) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: def.base.exampleId });
      applyTutorialUiSnapshotToShell(dispatch, def.base.ui, uiBridgeCtxRef.current);
      for (const cameraKeyframe of def.base.cameras) getTutorialCameraDriver(cameraKeyframe.windowId)?.set(cameraKeyframe.camera);
      tutorialLastAppliedMsRef.current = 0;
      tutorialClock.seek(0);
      await refreshUi(session, { kind: "full" });
    })().catch((error) => {
      console.error("[DEBUG] tutorial sandbox start failed", error);
      if (tutorialRunRef.current === run) dispatch({ type: "SET_TUTORIAL", value: null });
    }).finally(() => {
      tutorialDrivenRef.current.release(driveToken);
    });
  }, [activeTutorialId, activeTutorials, session, loadedPlugins, tutorialClock, refreshUi]);

  /** 🎬️ Applies every entry of one `TutorialSlice` (a director tick or a seek span) onto the live
   * session — UI changes first, then document-track entries through the plugin bridge: `Edit` via
   * `applyMutations` (forward/backward per `slice.forward`), `Load` is DSL/JSON document text the
   * pack-only channel cannot replay (see the `kind.kind === "load"` branch below),
   * `Undo`/`Redo`/`Checkpoint`/`CheckoutCheckpoint`/`SwitchAlternative` via the SAME History-action
   * `onAction` funnel the app's own undo/redo buttons dispatch through (never a bespoke channel) — then
   * pulses any annotational event's target element via the existing `celebrateElements` vocabulary. */
  const applyTutorialSliceToShell = useCallback(
    async (slice: TutorialSlice, activeSession: ActiveSession, run: OwnedTutorialRunV1, driveToken: number) => {
      if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken) || !shellDialogOriginIsCurrentV1(run.origin, captureDialogOrigin(activeSession))) return;
      for (const change of slice.uiChanges) applyTutorialUiChangeToShell(dispatch, change, uiBridgeCtxRef.current);
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === activeSession.pluginId)?.handle;
      let documentTouched = false;
      for (const documentEvent of slice.document) {
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
        const kind: TutorialArtifactEventKind = documentEvent.kind;
        if (kind.kind === "edit") {
          documentTouched = true;
          const mutations = (slice.forward ? kind.forwards : kind.backwards) as readonly MutationEnvelope[];
          if (plugin?.applyMutations) await plugin.applyMutations(activeSession.instanceId, encodeMutationEnvelopesPack(mutations));
        } else if (kind.kind === "load") {
          // 🚧️ `kind.documentDsl`/`kind.previousDsl` is authored DSL/JSON document TEXT — the channel
          // only carries binary pack/spr containers (`AppCommand::LoadDocument`), and no TS-side
          // text→pack encoder exists (a separate, much larger work package), so a `Load` history entry
          // cannot replay through the plugin bridge; the UI/camera/event tracks alongside it still do.
          documentTouched = true;
        } else if (kind.kind === "undo") {
          await onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "undo" : "redo" }, run.origin, true);
        } else if (kind.kind === "redo") {
          await onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "redo" : "undo" }, run.origin, true);
        } else if (kind.kind === "checkpoint") {
          if (slice.forward) await onActionRef.current({ controllerId: activeSession.app.controllerId, action: "commitCheckpoint" }, run.origin, true);
        } else if (kind.kind === "checkoutCheckpoint") {
          await onActionRef.current({ controllerId: activeSession.app.controllerId, action: "checkoutCheckpoint", args: { checkpointId: kind.checkpointId } }, run.origin, true);
        } else if (kind.kind === "switchAlternative") {
          await onActionRef.current({ controllerId: activeSession.app.controllerId, action: "switchAlternative", args: { alternativeId: kind.alternativeId } }, run.origin, true);
        }
      }
      for (const event of slice.events) {
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
        const kind = event.kind;
        const targetId = kind.kind === "action" ? kind.action : kind.kind === "command" ? kind.command : undefined;
        if (targetId && scope.rootRef.current) celebrateElements(elementIdSelector(targetId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      }
      if (documentTouched && tutorialRunRef.current === run && run.ready && tutorialDrivenRef.current.accepts(driveToken)) await refreshUi(activeSession, { kind: "full" });
    },
    [loadedPlugins, refreshUi],
  );

  // 🎬️ Director: one subscription to the clock's rAF-driven ticks. Camera interpolation applies every
  // tick (smooth); UI/document/event application throttles to `TUTORIAL_DIRECTOR_TICK_MS`.
  useEffect(() => {
    const def = activeTutorial;
    if (!def || !session) return;
    const run = tutorialRunRef.current;
    if (run === null || run.tutorialId !== def.id) return;
    let lastHeavyTickAt = 0;
    const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
    const unsubscribe = tutorialClock.subscribe(() => {
      if (tutorialRunRef.current !== run || !run.ready || tutorialDrivenRef.current.busy) return;
      const t = tutorialClock.getTimeMs();
      for (const windowId of cameraWindowIds) {
        const pose = tutorialCameraAt(def, windowId, t);
        if (pose) getTutorialCameraDriver(windowId)?.set(pose);
      }
      if (!tutorialClock.isPlaying()) return;
      const now = performance.now();
      if (now - lastHeavyTickAt < TUTORIAL_DIRECTOR_TICK_MS) return;
      lastHeavyTickAt = now;
      void tutorialDrivenRef.current.enqueue(() => tutorialRunRef.current === run && run.ready, async driveToken => {
        const from = tutorialLastAppliedMsRef.current;
        if (from === t) return;
        await applyTutorialSliceToShell(tutorialSlice(def, from, t), session, run, driveToken);
        if (tutorialRunRef.current === run && run.ready && tutorialDrivenRef.current.accepts(driveToken)) tutorialLastAppliedMsRef.current = t;
      }).catch((error) => {
        tutorialClock.pause();
        console.error("[DEBUG] tutorial director failed", error);
      });
    });
    return unsubscribe;
  }, [activeTutorial, session, tutorialClock, applyTutorialSliceToShell]);

  /** ✂️ Seek/rebuild (design point 5): composes UI wholesale (never accumulates deltas across a seek —
   * mirrors the Rust `tutorial_slice` doc comment's own warning), applies the forward/backward document
   * span crossed since the last applied playhead, sets every camera exactly (no interpolation on a seek),
   * and moves the clock. */
  const seekTutorial = useCallback(
    (ms: number) => {
      const def = activeTutorial;
      if (!def || !session) return;
      const run = tutorialRunRef.current;
      if (run === null || run.tutorialId !== def.id || !run.ready) return;
      const clamped = Math.min(def.durationMs, Math.max(0, ms));
      void runPausedTutorialSeekV1(tutorialClock, tutorialDrivenRef.current, () => tutorialRunRef.current === run && run.ready, () => tutorialPlayingRef.current, async driveToken => {
        const from = tutorialLastAppliedMsRef.current;
        applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, clamped), uiBridgeCtxRef.current);
        const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
        const slice = tutorialSlice(def, from, clamped);
        let documentTouched = false;
        for (const documentEvent of slice.document) {
          if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
          const kind: TutorialArtifactEventKind = documentEvent.kind;
          if (kind.kind === "edit") {
            documentTouched = true;
            const mutations = (slice.forward ? kind.forwards : kind.backwards) as readonly MutationEnvelope[];
            if (plugin?.applyMutations) await plugin.applyMutations(session.instanceId, encodeMutationEnvelopesPack(mutations));
          } else if (kind.kind === "load") {
            // 🚧️ `kind.documentDsl`/`kind.previousDsl` is authored DSL/JSON document TEXT — the channel
            // only carries binary pack/spr containers (`AppCommand::LoadDocument`), and no TS-side
            // text→pack encoder exists (a separate, much larger work package), so a `Load` history
            // entry cannot replay through the plugin bridge on a seek either.
            documentTouched = true;
          }
          // 🚧️ Undo/Redo/Checkpoint/CheckoutCheckpoint/SwitchAlternative crossings mid-seek are an honest
          // scope cut here (replaying a crossed history op out of its natural live-dispatch order is
          // ambiguous without more VCS-side infrastructure) — the director's per-tick forward playback
          // above still applies them correctly; only a large scrub jumping OVER one of these entries misses it.
        }
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
        const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
        for (const windowId of cameraWindowIds) {
          const pose = tutorialCameraAt(def, windowId, clamped);
          if (pose) getTutorialCameraDriver(windowId)?.set(pose);
        }
        tutorialLastAppliedMsRef.current = clamped;
        tutorialClock.seek(clamped);
        if (documentTouched) await refreshUi(session, { kind: "full" });
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
        console.log("[DEBUG] tutorial rebuild", { atMs: clamped });
      }).catch((error) => console.error("[DEBUG] tutorial seek failed", error));
    },
    [activeTutorial, session, loadedPlugins, tutorialClock, refreshUi],
  );

  /** ▶️ Play/pause toggle — the deviation-converge path (design point 6): snaps document+UI to the
   * composed target at the current playhead, tweens the camera over `TUTORIAL_CONVERGE_MS` (real-time,
   * rate-independent) from each window's LIVE pose to its target pose, then resumes the clock. */
  const playPauseTutorial = useCallback(() => {
    if (!activeTutorial) return;
    const run = tutorialRunRef.current;
    if (run === null || run.tutorialId !== activeTutorial.id || !run.ready) return;
    if (tutorialDrivenRef.current.busy) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: !tutorialPlaying });
      return;
    }
    if (tutorialPlaying) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
      return;
    }
    if (tutorialDeviated && session) {
      const def = activeTutorial;
      const atMs = tutorialClock.getTimeMs();
      const driveToken = tutorialDrivenRef.current.claim();
      applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, atMs), uiBridgeCtxRef.current);
      const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
      const startPoseByWindow = new Map<string, TutorialCameraState>();
      for (const windowId of cameraWindowIds) {
        const live = getTutorialCameraDriver(windowId)?.get();
        if (live) startPoseByWindow.set(windowId, live);
      }
      const startedAt = performance.now();
      const tween = (now: number) => {
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) {
          tutorialDrivenRef.current.release(driveToken);
          return;
        }
        const progress = Math.min(1, (now - startedAt) / TUTORIAL_CONVERGE_MS);
        for (const windowId of cameraWindowIds) {
          const targetPose = tutorialCameraAt(def, windowId, atMs);
          if (!targetPose) continue;
          const driver = getTutorialCameraDriver(windowId);
          if (!driver) continue;
          const startPose = startPoseByWindow.get(windowId);
          if (startPose && startPose.kind === targetPose.kind) {
            driver.set(interpolateTutorialCamera({ at: 0, windowId, camera: startPose, easing: "linear" }, { at: TUTORIAL_CONVERGE_MS, windowId, camera: targetPose, easing: "linear" }, progress * TUTORIAL_CONVERGE_MS));
          } else {
            driver.set(targetPose);
          }
        }
        if (progress < 1) requestAnimationFrame(tween);
        else {
          tutorialDrivenRef.current.release(driveToken);
          dispatch({ type: "SET_TUTORIAL_DEVIATED", value: false });
          dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
        }
      };
      requestAnimationFrame(tween);
      return;
    }
    dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
  }, [activeTutorial, tutorialPlaying, tutorialDeviated, session, tutorialClock]);

  const startTutorial = useCallback(
    (tutorialId: string) => {
      if (!session || !activeTutorials.some((tutorial) => tutorial.id === tutorialId)) return;
      const origin = captureDialogOrigin(session);
      const owner = captureEffectOwner(session, origin);
      if (origin === null || !isCurrentEffectOwner(owner) || owner.plugin === null) return;
      const plugin = owner.plugin;
      const epoch = ++tutorialTransitionEpochRef.current;
      tutorialClock.pause();
      tutorialDrivenRef.current.retire();
      dispatch({ type: "SET_TUTORIAL", value: null });
      void (async () => {
        const previous = tutorialRunRef.current;
        await previous?.stop().catch((error) => console.error("[DEBUG] tutorial sandbox restore failed", error));
        if (tutorialRunRef.current === previous) tutorialRunRef.current = null;
        if (epoch !== tutorialTransitionEpochRef.current || !isCurrentEffectOwner(owner)) return;
        const run: OwnedTutorialRunV1<DocumentArchivePack> = new OwnedTutorialRunV1(tutorialId, origin, (): boolean => tutorialRunRef.current === run && isCurrentEffectOwner(owner), {
          read: async () => plugin.readAppDocumentArchive ? plugin.readAppDocumentArchive(session.instanceId) : null,
          drain: () => tutorialDrivenRef.current.drain(),
          restore: async (snapshot) => {
            if (!isCurrentEffectOwner(owner)) return;
            await loadDocumentArchive(plugin, session.instanceId, snapshot, () => isCurrentEffectOwner(owner));
            if (isCurrentEffectOwner(owner)) await refreshUi(session, { kind: "full" });
          },
        });
        tutorialRunRef.current = run;
        tutorialInitializingRunRef.current = null;
        dispatch({ type: "SET_TUTORIAL", value: tutorialId });
      })().catch((error) => console.error("[DEBUG] tutorial transition failed", error));
    },
    [activeTutorials, session, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, loadDocumentArchive, tutorialClock, refreshUi],
  );
  const stopTutorial = useCallback(() => {
    tutorialTransitionEpochRef.current += 1;
    tutorialClock.pause();
    tutorialDrivenRef.current.retire();
    void tutorialRunRef.current?.stop().catch((error) => console.error("[DEBUG] tutorial sandbox restore failed", error));
    dispatch({ type: "SET_TUTORIAL", value: null });
  }, [tutorialClock]);

  /** ⏺️ Arms/disarms `TutorialRecorder` against the LIVE (never sandboxed) document — a recording IS the
   * user's work. On stop: light `validateTutorial` sanity check, then serialize + trigger a browser
   * download, matching the repo's existing media-export download pattern. */
  const toggleTutorialRecording = useCallback(() => {
    if (!session) return;
    const recorder = tutorialRecorderRef.current;
    if (recorder) {
      tutorialRecorderRef.current = null;
      const id = `recorded-${session.app.id}-${Date.now()}`;
      const def = recorder.build(id, `${session.app.id} recording`);
      const validationError = validateTutorial(def);
      if (validationError) console.error("[DEBUG] tutorial recording validation failed", validationError);
      const json = JSON.stringify(def, null, 2);
      console.log("[DEBUG] tutorial recording", json);
      downloadMediaExport(`tutorial-${session.app.id}-${Date.now()}.ops`, "text/plain", json);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: false });
      return;
    }
    if (tutorialRecorderStartingRef.current) return;
    // 🚧️ `TutorialRecorder`'s base document is authored DSL/JSON TEXT — the channel only carries
    // binary pack/spr containers, and no TS-side pack→text decoder exists (deliberately out of scope
    // for `🔖️PackValueCodec`), so a recording's `base.documentDsl` fixture can't be captured from the
    // live channel; its UI/camera/event tracks still capture faithfully.
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    tutorialRecorderStartingRef.current = true;
    void publishLocalInteraction(plugin, session.instanceId).then((interaction) => {
      const active = sessionRef.current;
      if (!interaction || !active || active.pluginId !== session.pluginId || active.instanceId !== session.instanceId) return;
      const observed = { ...shellStateRef.current, interaction: { ...interaction, hover: shellStateRef.current.interaction.hover } };
      tutorialRecorderRef.current = new TutorialRecorder(captureTutorialUiSnapshot(observed, active), null);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: true });
    }).catch((captureError) => console.error("[DEBUG] tutorial interaction capture failed", captureError)).finally(() => {
      tutorialRecorderStartingRef.current = false;
    });
  }, [loadedPlugins, publishLocalInteraction, session]);

  useEffect(() => {
    startTutorialRef.current = startTutorial;
    stopTutorialRef.current = stopTutorial;
    toggleTutorialRecordingRef.current = toggleTutorialRecording;
  }, [startTutorial, stopTutorial, toggleTutorialRecording]);

  // ⏺️ Recorder: UI-state diff on every `ShellState` change (catches panel-tab clicks/tree expands/etc.
  // that bypass `onAction`), a periodic full-snapshot keyframe every 5s, and a 10Hz epsilon-filtered
  // camera sampler per registered driver (world drags bypass `onAction` entirely).
  useEffect(() => {
    if (!tutorialRecording) return;
    tutorialRecorderRef.current?.recordUiDiff(captureTutorialUiSnapshot(shellState, session));
  }, [tutorialRecording, shellState, session]);

  useEffect(() => {
    if (!tutorialRecording || !session || typeof window === "undefined") return;
    const interval = window.setInterval(() => {
      tutorialRecorderRef.current?.recordSnapshot(captureTutorialUiSnapshot(shellStateRef.current, session));
    }, 5000);
    return () => window.clearInterval(interval);
  }, [tutorialRecording, session]);

  useEffect(() => {
    if (!tutorialRecording || !session || typeof window === "undefined") return;
    const interval = window.setInterval(() => {
      const recorder = tutorialRecorderRef.current;
      if (!recorder) return;
      for (const instance of sessionWindowInstances(session.app, extraWindowInstancesRef.current)) {
        const pose = getTutorialCameraDriver(instance.id)?.get();
        if (pose) recorder.sampleCamera(instance.id, pose);
      }
    }, 100);
    return () => window.clearInterval(interval);
  }, [tutorialRecording, session]);

  const addTutorialChapter = useCallback(() => {
    tutorialRecorderRef.current?.addChapter();
  }, []);

  const tutorialChapterMarkers = useMemo(
    (): readonly TutorialChapterMarker[] => (activeTutorial ? activeTutorial.chapters.map((chapter) => ({ id: chapter.id, title: resolveManifestLabel(chapter.title, uiTerminology, uiLocale), atMs: chapter.at })) : []),
    [activeTutorial, uiTerminology, uiLocale],
  );
  //#endregion 🎥️TutorialOrchestration

  //#region 🔖️DirectoryLane
  /** 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C6 — folds one batch of
   * `DirectoryEvent`s into the CURRENTLY mounted session (home, studio, or the new `s.space` app) as
   * the `foldDirectoryEvents` plugin view action; the fold itself is `…ConfigMutation::FoldDirectoryEvent`
   * on the plugin side (contract §C6), this only relays the raw batch through the existing action
   * funnel. Brief's prose says "home session AND the open space session" — this shell keeps exactly
   * ONE plugin session mounted at a time (`session`/`switchToPluginApp`), so only whichever of
   * home/studio/space is actually live right now can receive it.
   *
   * w4-h root-cause fix #1: every `command_from_action` implementation on the Rust side
   * (`home/…/✏️editor/🦀️.rs`, `space/…/✏️editor/🦀️.rs`) reads the batch as a
   * `eventsJson: string` field (`args.get("eventsJson")` → `.as_str()` → `serde_json::from_str`), never
   * a raw `events` array — sending `{ events }` left `.get("eventsJson")` finding nothing and silently
   * falling back to `"[]"` on every call, so the fold ran on zero events every single time regardless
   * of whether the live broadcast or the command-result round trip delivered the real payload. Proven
   * live: the collab-e2e harness's new console capture shows the WS frame with the real
   * `space.created` event arriving, yet no row ever appeared — a data-shape bug, not a thrown error,
   * which is why neither `pageerror` nor any `console.error` ever caught it.
   *
   * Host aliases are resolved once against the immutable live manifest. Directory gating therefore
   * compares exact canonical app identities and never relies on registry aliases or app ordering. */
  const dispatchDirectoryEventBatch = useCallback(
    (events: readonly DirectoryEvent[]) => {
      if (events.length === 0 || !hostConfig) return;
      const current = sessionRef.current;
      if (!current) return;
      const isHome = current.app.id === landingApp?.id;
      const isStudio = current.app.id === hostApp?.id;
      const isSpaceIndex = current.app.dialect !== undefined && dialectCoordinate(current.app.dialect) === dialectCoordinate(SPACE_INDEX_DIALECT);
      if (!isHome && !isStudio && !isSpaceIndex) return;
      onActionRef.current({ controllerId: current.app.controllerId, action: "foldDirectoryEvents", args: { eventsJson: JSON.stringify(events) } });
    },
    [hostConfig, landingApp, hostApp],
  );
  dispatchDirectoryEventsRef.current = dispatchDirectoryEventBatch;
  //#endregion 🔖️DirectoryLane

  const hostSessionActive = hostMode && session?.app.id === hostAppId;
  // 🏠️🧳️ Once `hostSessionActive` is true, `session.app` *is* the host app, so its own self-declared
  // `controllerId` is the right value — no separate app-identity lookup needed.
  const studioSessionControllerId = hostSessionActive ? session?.app.controllerId : undefined;
  useEffect(() => {
    if (!hostSessionActive || !studioSessionControllerId || typeof window === "undefined") return;
    // 🪪️ §C3 — a resolved sign-in shows the real user in presence chrome instead of a random Guest.
    const presenceIdentity = presenceClientIdentity(ephemeral, identityRef.current ? { clientId: shellActorIdRef.current, name: identityRef.current.displayName } : undefined);
    const beat = () => onActionRef.current({ controllerId: studioSessionControllerId, action: "presenceHeartbeat", args: presenceIdentity });
    const initial = window.setTimeout(beat, 1000);
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      window.clearTimeout(initial);
      window.clearInterval(timer);
    };
  }, [hostSessionActive, studioSessionControllerId, ephemeral, identity]);

  useEffect(() => {
    if (typeof window === "undefined") return;
    const presenceIdentity = presenceClientIdentity(ephemeral, identityRef.current ? { clientId: shellActorIdRef.current, name: identityRef.current.displayName } : undefined);
    // 🐚️ terra-web-shellhost (finding 5) — one `latestWins` trigger PER open document (lazily created,
    // cached in `presenceBeatTriggersRef`): a slow `ephemeralSnapshot` for one document no longer
    // delays the heartbeat for every OTHER open document — each document's beat now runs independently
    // and concurrently. `latestWins` still protects a SINGLE document against overlapping beats if its
    // own snapshot takes longer than one heartbeat interval (collapses to at most one trailing follow-
    // up), the same guarantee the old shell-WIDE `publishing` flag gave the whole loop, now scoped per
    // document instead of stalling every document behind the slowest one. `run` reads
    // `backboneWorkerRef`/`openDocumentSessionsRef` fresh on every actual invocation (never captured at
    // trigger-creation time), so a still-open document's beat never serves a stale plugin/session pair.
    const beatOneDocument = (runtimeKey: string): Promise<void> => {
      let trigger = presenceBeatTriggersRef.current.get(runtimeKey);
      if (!trigger) {
        trigger = latestWins(async () => {
          const worker = backboneWorkerRef.current;
          const entry = openDocumentSessionsRef.current.get(runtimeKey);
          if (!worker || !entry) return;
          const snapshot = await entry.plugin.ephemeralSnapshot?.(entry.session.instanceId);
          if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== entry.clientInstanceId) return;
          const request: BackboneWorkerRequest = {
            kind: "send",
            documentId: entry.documentId,
            clientInstanceId: entry.clientInstanceId,
            ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }),
            message: {
              kind: "presenceHeartbeat",
              // 🚧️ `cursor`/`viewport` are gone from `ArtifactPresencePeer`'s current wire shape (see
              // that type's own field list, `@semio-tech/framework-replication`) — `presenceCursorRef`
              // is still tracked (pointermove listener above) but has no landing spot on the wire
              // anymore; left tracked rather than torn out, since removing the listener is a design
              // call about whether cursor-sharing comes back, not a type fix.
              peer: {
                actor: shellActorIdRef.current,
                label: presenceIdentity.name,
                presencePack: snapshot?.presence,
                connectedAtMs: presenceConnectedAtMsRef.current,
                // 🪟️ No per-window camera/pointer tracking is wired into this heartbeat yet — an
                // honest "no open windows reported" default, matching this field's own doc comment.
                views: [],
              },
            },
          };
          worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
        });
        presenceBeatTriggersRef.current.set(runtimeKey, trigger);
      }
      return trigger();
    };
    const beat = () => {
      // 🧹️ Drops triggers for documents closed since the last tick — bounded growth instead of one
      // entry per document ever opened this session.
      for (const runtimeKey of presenceBeatTriggersRef.current.keys()) {
        if (!openDocumentSessionsRef.current.has(runtimeKey)) presenceBeatTriggersRef.current.delete(runtimeKey);
      }
      for (const runtimeKey of openDocumentSessionsRef.current.keys()) {
        void beatOneDocument(runtimeKey).catch((error) => console.error("[os-shell] presence heartbeat failed for document", runtimeKey, error));
      }
    };
    beat();
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      window.clearInterval(timer);
      presenceBeatTriggersRef.current.clear();
    };
  }, [ephemeral, identity]);

  usePanelChromeHotkeys({
    // 📱️ All eight anchor hotkeys collapse onto the single mobile panel toggle on mobile. Same `shell.panelToggle`
    // commandId as the mouse-driven toggle in `buildPanelSelectionProps` (so keyboard/mouse fold together),
    // flagged `hotkey: true` in detail.
    onToggle: (anchor) => {
      if (mobile) dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: (visible) => !visible });
      else dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: (visible) => !visible });
      noteShellCommand("shell.panelToggle", shellLabel("ui.shellCommand.panelToggle"), { anchor: mobile ? undefined : anchor, hotkey: true });
    },
  });

  useElementsSurfaceChrome({ appearance: uiAppearance, device: uiDevice, driver: uiDriver }, scope.rootRef.current ?? undefined);

  //#region 🎨️ Ui preference projection effects
  useEffect(() => {
    void scope.i18n.changeLanguage(uiLocale);
    syncShellLabelLocale(uiLocale);
    if (scope.ownsPage) {
      if (typeof document !== "undefined") document.documentElement.lang = uiLocale;
    } else if (scope.rootRef.current) {
      scope.rootRef.current.lang = uiLocale;
    }
    if (scope.ownsPage) {
      setActiveUiTheme(uiTheme);
    } else if (scope.rootRef.current) {
      applyUiThemeToRoot(scope.rootRef.current, uiTheme);
    }
  }, [uiLocale, uiTheme, scope]);

  // 🐚️ Unmount cleanup for the embedded (non-page-owning) case — a shell that painted its own root's
  // theme tokens must remove them on unmount, or a later, unrelated element reused at the same DOM
  // position (React/vite HMR reuse, or another shell's canvas-clone assets in a dev harness) would
  // silently inherit a stale theme's inline overrides. The page-owning case is intentionally left alone:
  // `document.documentElement` outlives any single shell's lifetime.
  useEffect(() => {
    if (scope.ownsPage) return;
    return () => {
      if (scope.rootRef.current) clearUiThemeFromRoot(scope.rootRef.current);
    };
  }, [scope]);
  //#endregion

  useActionHotkey(
    "ui.nav.back",
    useCallback(() => {
      if (canGoBack) goBack();
    }, [canGoBack, goBack]),
    undefined,
    [canGoBack, goBack],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.nav.forward",
    useCallback(() => {
      if (canGoForward) goForward();
    }, [canGoForward, goForward]),
    undefined,
    [canGoForward, goForward],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.nav.up",
    useCallback(() => {
      if (canGoUp) goUp();
    }, [canGoUp, goUp]),
    undefined,
    [canGoUp, goUp],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.search.toggle",
    useCallback(() => dispatch({ type: "SET_SEARCH_OPEN", value: (open) => !open }), []),
    undefined,
    [],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.find.toggle",
    useCallback(() => dispatch({ type: "SET_FIND_OPEN", value: (open) => !open }), []),
    undefined,
    [],
    { overrides: uiKeybindingOverrides },
  );

  const applyNamedLayout = useCallback(
    (layout: WindowLayout) => {
      if (!session) return;
      const seeded = applyFrameworkLayoutSeed(layout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      // 🪟️ Hand the just-computed instance list straight to the fetch rather than reading `extraWindowInstances`
      // state (which wouldn't reflect this dispatch until the next render) — every newly-seeded pane's own
      // body/measures/engagement gets fetched immediately instead of showing "missing window" until later.
      void refreshUi(session, { kind: "full" }, seeded.extraInstances);
    },
    [session, appLabelsOverlay, refreshUi, uiTerminology, uiLocale],
  );

  const applyModeChange = useCallback(
    (modeId: string) => {
      // 🛠️ Tools are scoped to a mode — switching modes always clears the active tool (and every
      // window's active utility), mirroring how a fresh mode starts with no utility pressed either.
      dispatch({ type: "SET_ACTIVE_TOOL", toolId: null });
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return current;
          const layout = resolveLayoutForMode(current.app, modeId);
          const nextSession: ActiveSession = { ...current, viewState: { ...current.viewState, activeModeId: modeId, activeToolId: undefined } };
          if (layout) {
            const seeded = applyFrameworkLayoutSeed(layout, withLocalizedWindowKindLabels(current.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
            extraWindowInstancesRef.current = seeded.extraInstances;
            extraWindowCounterRef.current = seeded.extraInstances.length;
            dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
            dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
            dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
            void refreshUi(nextSession, { kind: "full" }, seeded.extraInstances);
          }
          return nextSession;
        },
      });
    },
    [appLabelsOverlay, refreshUi, uiTerminology, uiLocale],
  );

  /** 🎛️⌨️ Keyboard half of the navbar mode group: cycles `session.app.modes` by one step, wrapping.
   * Positional rather than one binding per mode id because mode ids are plugin-authored while
   * `SHELL_KEYBINDINGS` is a static framework table — see its own doc comment. Reads `sessionRef` so
   * the bound callback survives every session change without re-arming the listener. */
  const applyModeStep = useCallback(
    (step: 1 | -1) => {
      const current = sessionRef.current;
      if (!current) return;
      const next = stepModeIdV1(current.app.modes.map((mode) => mode.id), current.viewState.activeModeId ?? current.app.defaultModeId, step);
      if (next) applyModeChange(next);
    },
    [applyModeChange],
  );

  /** 👁️✏️⌨️ Switches the mounted session to this plugin's OTHER surface for the open document's dialect.
   * Every `session.app.role` gate (VCS check-in, mutation-command filtering, the viewer mutation
   * refusal, `canonicalSurfaceId` scope validation) reads the live session, so swapping `session.app`
   * flips all of them in one step — nothing here reproduces a role rule. */
  const switchToSessionRole = useCallback(
    async (role: AppRole): Promise<void> => {
      const current = sessionRef.current;
      if (!current) return;
      const apps = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === current.pluginId)?.manifest.apps ?? [];
      const target = roleSwitchTargetV1(apps, current.app.dialect, current.app.role, role);
      if (!target) return;
      await switchToPluginApp(current.pluginId, target.id);
    },
    [switchToPluginApp],
  );

  useActionHotkey(
    "ui.shell.mode.next",
    useCallback(() => applyModeStep(1), [applyModeStep]),
    { preventDefault: true },
    [applyModeStep],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.shell.mode.previous",
    useCallback(() => applyModeStep(-1), [applyModeStep]),
    { preventDefault: true },
    [applyModeStep],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    SURFACE_ROLE_CONTROL_IDS.editor,
    useCallback(() => void switchToSessionRole("editor").catch((switchError) => console.error("role switch to editor failed", switchError)), [switchToSessionRole]),
    { preventDefault: true },
    [switchToSessionRole],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    SURFACE_ROLE_CONTROL_IDS.viewer,
    useCallback(() => void switchToSessionRole("viewer").catch((switchError) => console.error("role switch to viewer failed", switchError)), [switchToSessionRole]),
    { preventDefault: true },
    [switchToSessionRole],
    { overrides: uiKeybindingOverrides },
  );

  const handleTemplateDrop = useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      if (!session) return;
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      const projectionSpec = decodeWorldProjectionTemplateId(payload.templateId);
      if (projectionSpec) registerPendingWorldProjection(instanceId, projectionSpec);
      const title = projectionSpec ? worldProjectionSpecLabel(projectionSpec) : resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale));
      const nextExtraInstances = [...extraWindowInstancesRef.current, { id: instanceId, windowKindId: payload.windowKindId, title }];
      extraWindowInstancesRef.current = nextExtraInstances;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: nextExtraInstances });
      if (projectionSpec) {
        dispatch({ type: "SET_WINDOW_TITLE", windowId: instanceId, title });
        dispatch({ type: "SET_WINDOW_ICON", windowId: instanceId, iconId: worldProjectionSpecIconId(projectionSpec) as IconName });
      }
      // 🪟️ The new split pane is its own window instance — fetch its body/measures/engagement right away
      // (see `applyNamedLayout`'s comment) rather than waiting for an unrelated action to trigger a refresh.
      void refreshUi(session, { kind: "full" }, nextExtraInstances);
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: (current) => {
          const base =
            current ??
            resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout;
          return insertWindowAtDropZone(base, instanceId, target);
        },
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
      noteShellCommand("shell.windowSplit", shellLabel("ui.shellCommand.windowSplit"), { windowKindId: payload.windowKindId, instanceId });
    },
    [appLabelsOverlay, refreshUi, session, noteShellCommand, uiTerminology, uiLocale],
  );

  const displayHostRef = useRef<DisplayHostApi | null>(null);
  const displayHost = useNamedLayoutHost({
    appId: session?.app.id ?? "framework-os",
    windowKinds: session?.app.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)) })) ?? [],
    builtinLayouts: session?.app.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, extraWindowInstances, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    namedLayoutStore,
  });
  displayHostRef.current = displayHost;

  //#region 🔖️SurfaceRoles
  /** 👁️✏️ `(dialect, role) -> AppRef[]`, contract freeze §3 — built fresh from every loaded plugin's
   * manifest. `AppRouter.build` is total: a plugin with a genuine authoring defect
   * (`surface.conflict`/`surface.contribution-not-permitted`) is excluded on its own and reported
   * through {@link AppRouter.pluginFaults}, so one bad manifest costs that plugin's surfaces, never
   * every route in the session (ticket 26/09/05/S-END-TO-END lane H). */
  const appRouter = useMemo(
    (): AppRouter =>
      AppRouter.build(
        loadedPlugins.map((entry): AppRouterManifest => ({
          pluginId: entry.handle.pluginId,
          apps: entry.manifest.apps as unknown as Record<string, unknown>[],
          artifactKinds: entry.manifest.artifactKinds,
          dependencies: entry.manifest.dependencies,
        })),
      ),
    [loadedPlugins, registry],
  );
  /** 🧯️ `pluginId -> the surface fault that excluded it from {@link appRouter}` — merged onto the
   * plugin's own status so an excluded plugin is visible instead of silently unroutable. */
  const routerFaultByPluginId = useMemo(() => new Map(appRouter.pluginFaults().map((fault) => [fault.scope.pluginId ?? "", fault] as const)), [appRouter]);
  const pluginLabelById = useMemo(() => new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.manifest.label || entry.handle.pluginId])), [loadedPlugins]);
  /** 👁️✏️ Every dialect any loaded app declares — read straight off `AppDefinition.dialect`
   * (contract freeze §1), never inferred from a surface id string. Feeds the Settings
   * `SettingsDefaultApps` table; `appRouter` itself has no public "every registered dialect"
   * accessor (by design — it's addressed one `(dialect, role)` pair at a time). */
  const knownDialects = useMemo((): readonly ArtifactDialect[] => {
    const byCoordinate = new Map<string, ArtifactDialect>();
    for (const entry of loadedPlugins) for (const app of entry.manifest.apps) if (app.dialect) byCoordinate.set(dialectCoordinate(app.dialect), app.dialect);
    return [...byCoordinate.values()];
  }, [cancelSpaceArtifactCreationsForRuntime, loadedPlugins]);

  /** 🎚️ Client-side fold of `os.config.opening` (contract freeze §4) — event-sourced the same way
   * the host materializes it (`foldOpeningPreferences`), never a mutated map. There is no host
   * readback call on {@link AppChannelClient} yet (only the two write commands), so this mirror is
   * advanced ONLY by this shell's own `setDefaultApp`/`clearDefaultApp` calls below — a pin made by
   * another session/shell is not reflected here until that gap closes. See `📓️w1-c-report.md`. */
  const [openingPreferences, setOpeningPreferences] = useState<OpeningPreferences>(EMPTY_OPENING_PREFERENCES);
  const pinnedAppFor = useCallback((dialect: ArtifactDialect, role: AppRole): AppRef | undefined => openingPreferences.defaults.find((entry) => dialectCoordinate(entry.dialect) === dialectCoordinate(dialect) && entry.role === role)?.app, [openingPreferences]);
  resolveArtifactOpeningRelayRef.current = (actionId, args) => (appRouter ? resolveArtifactOpeningRelay(actionId, args, appRouter, openingPreferences) : null);

  /** 👁️✏️ `PluginRuntime`'s `PluginWasmHandle` wraps the raw `exchange` ABI behind typed methods —
   * `transactionPrepare`/`transactionCommit`/`transactionUndo`/`transactionRedo` and (as of ticket
   * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` lane K2)
   * `setMergePolicy`/`resolveConflict`/`readConflicts` are wrapped this way (`adaptPluginHandle`,
   * `PluginRuntime/🟦️.tsx`), each internally riding its own `AppChannelClient` — see
   * {@link pluginHandleFor} below. `openArtifact`/`setDefaultApp`/`clearDefaultApp` (contract freeze
   * §3) are NOT wrapped yet — the raw `exchange` method itself isn't re-exposed for them, so this
   * lease cannot construct its own `AppChannelClient` from the handle it's given. Feature-detected
   * here (structurally optional, not a type-lying cast) so this activates the moment `PluginRuntime`
   * adds the same three methods the transaction/merge families already have, with zero changes on
   * this side. See `📓️w1-c-report.md` "NOT done" — until then these three are local-only (the
   * opening-prefs fold below and the session switch in `openArtifactWithAppRef` both still work;
   * only the wire notify is inert). */
  type PendingAppChannelMethods = {
    readonly openArtifact?: (artifactRef: string, role: number, pluginId?: string, appId?: string) => Promise<unknown>;
    readonly setDefaultApp?: (artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string) => Promise<unknown>;
    readonly clearDefaultApp?: (artifactKind: string, standard: string, subset: string, role: number) => Promise<unknown>;
  };
  const pendingAppChannelFor = useCallback(
    (pluginId: string): PendingAppChannelMethods | undefined => loadedPlugins.find((entry) => entry.handle.pluginId === pluginId)?.handle as PendingAppChannelMethods | undefined,
    [loadedPlugins],
  );

  /** ⚖️ Finds a loaded plugin's real `PluginWasmHandle` by pluginId, for the merge-policy/conflict
   * pass-throughs below — unlike {@link pendingAppChannelFor}'s ad hoc cast, `setMergePolicy`/
   * `resolveConflict`/`readConflicts` are genuine, always-present `PluginWasmHandle` members now
   * (`adaptPluginHandle`, `PluginRuntime/🟦️.tsx`), so no feature-detection is needed. */
  const pluginHandleFor = useCallback((pluginId: string): PluginWasmHandle | undefined => loadedPlugins.find((entry) => entry.handle.pluginId === pluginId)?.handle, [loadedPlugins]);

  const dispatchSetDefaultApp = useCallback(
    (dialect: ArtifactDialect, role: AppRole, app: AppRef) => {
      const mutation: OpeningConfigMutation = { mutation: "setDefaultApp", dialect, role, app };
      setOpeningPreferences((current) => foldOpeningPreferences([mutation], current));
      const roleNum = role === "editor" ? 1 : 0;
      void pendingAppChannelFor(app.pluginId)
        ?.setDefaultApp?.(dialect.artifactKind, dialect.standard, dialect.subset, roleNum, app.pluginId, app.appId)
        ?.catch((commandError) => console.error("[DEBUG] setDefaultApp failed", commandError));
    },
    [pendingAppChannelFor],
  );
  const dispatchClearDefaultApp = useCallback(
    (dialect: ArtifactDialect, role: AppRole) => {
      const mutation: OpeningConfigMutation = { mutation: "clearDefaultApp", dialect, role };
      setOpeningPreferences((current) => foldOpeningPreferences([mutation], current));
      const roleNum = role === "editor" ? 1 : 0;
      void pendingAppChannelFor(session?.pluginId ?? "")
        ?.clearDefaultApp?.(dialect.artifactKind, dialect.standard, dialect.subset, roleNum)
        ?.catch((commandError) => console.error("[DEBUG] clearDefaultApp failed", commandError));
    },
    [pendingAppChannelFor, session?.pluginId],
  );

  /** ⚖️ Persists `os.config.merge-policy` through the `🛡️change-merge-policy` config triad's own
   * event-sourced fold (local, always works), then genuinely forwards `AppCommand::SetMergePolicy`
   * (contract freeze §C8) through `PluginWasmHandle.setMergePolicy` — a real handle member now (see
   * {@link pluginHandleFor}'s doc), so a session-less/plugin-not-loaded guard is the only thing
   * standing between this and the wire call; a failed dispatch surfaces loudly via the `.catch`
   * (never a silent optional-chain no-op). */
  const dispatchSetMergePolicy = useCallback(
    (policy: MergePolicy) => {
      dispatch({ type: "SET_MERGE_POLICY", value: policy });
      if (!session) return;
      const plugin = pluginHandleFor(session.pluginId);
      if (!plugin) return;
      void plugin.setMergePolicy(session.instanceId, policy).catch((commandError) => console.error("[DEBUG] setMergePolicy failed", commandError));
    },
    [pluginHandleFor, session],
  );

  /** ⚖️ `📌️ChromePanels`' Conflicts panel Accept/Discard — forwards `AppCommand::ResolveConflict`
   * (contract freeze §C6/§C8/§C9) through `PluginWasmHandle.resolveConflict`; the local roster is
   * replaced from the `Conflicts` frame the reply itself batches (contract freeze §C6
   * `resolve_conflict`: "Returns the authoritative `MergeReport` + `Conflicts` frames"), never
   * optimistically, since `resolve_conflict` can itself reject (Quarantined+Accept still enforces
   * Fatal — that path returns no `Conflicts` frame, so the roster is left untouched). */
  const dispatchResolveConflict = useCallback(
    (conflictId: string, resolution: ConflictResolution) => {
      if (!session) return;
      const plugin = pluginHandleFor(session.pluginId);
      if (!plugin) return;
      void plugin
        .resolveConflict(session.instanceId, conflictId, resolution)
        .then((result) => {
          if (result.conflicts) dispatch({ type: "SET_CONFLICTS", value: result.conflicts });
        })
        .catch((commandError) => console.error("[DEBUG] resolveConflict failed", commandError));
    },
    [pluginHandleFor, session],
  );

  const publishPreparedArtifactOpening = useCallback((target: PreparedArtifactOpeningTarget, receipt?: DocumentOpeningReceiptV1): void => {
    const { session: nextSession, dialect, role } = target;
    const seeded = applyFrameworkLayoutSeed(nextSession.app.defaultLayout, withLocalizedWindowKindLabels(nextSession.app.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
    extraWindowInstancesRef.current = seeded.extraInstances;
    extraWindowCounterRef.current = seeded.extraInstances.length;
    pendingDocumentOpeningPublicationRef.current = receipt === undefined ? null : { receipt, plugin: target.plugin, session: nextSession };
    dispatch({ type: "SET_SESSION", value: nextSession });
    dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
    dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
    dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
    try {
      void (target.plugin as PendingAppChannelMethods).openArtifact?.(canonicalSurfaceId(dialect, role), role === "editor" ? 1 : 0, nextSession.pluginId, nextSession.app.id)
        ?.catch((commandError) => console.error("[DEBUG] openArtifact failed", commandError));
    } catch (commandError) {
      console.error("[DEBUG] openArtifact failed", commandError);
    }
  }, [uiLocale, uiTerminology]);

  /** 👁️✏️ Re-points the primary session at a different registered `AppRef` for the SAME artifact
   * (contract freeze §3/§5's "Open with…") — installs the target plugin first if it isn't loaded
   * yet, then mirrors `establishPrimarySession`'s non-studio create/seed/dispatch sequence. Also
   * best-effort notifies the host once `openArtifact` is wrapped (see `PendingAppChannelMethods`
   * above) using the canonical role-suffixed surface id. */
  const openArtifactWithAppRef = useCallback(
    async (target: AppRef, dialect: ArtifactDialect, role: AppRole, admit?: () => boolean, publish = true): Promise<PreparedArtifactOpeningTarget | null> => {
      const current = shellStateRef.current.pluginRuntime.session;
      const owner = current === null ? null : captureEffectOwner(current, captureDialogOrigin(current));
      const canOpen = admit ?? (() => owner === null ? shellStateRef.current.pluginRuntime.session === null : isCurrentEffectOwner(owner));
      if (!canOpen()) return null;
      let plugin = loadedPlugins.find((entry) => entry.handle.pluginId === target.pluginId);
      if (!plugin) {
        const outcome = await installPlugin(target.pluginId);
        if (outcome !== "loaded" && outcome !== "already-loaded") return null;
        plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === target.pluginId);
      }
      if (!canOpen()) return null;
      const app = plugin?.manifest.apps.find((candidate) => candidate.id === target.appId);
      if (!plugin || !app) {
        console.error(`[DEBUG] openArtifactWithAppRef: ${target.pluginId}/${target.appId} not found after install`);
        return null;
      }
      const handle = plugin.handle;
      const instanceId = await createAdmittedShellInstanceV1(canOpen, () => handle.createApp(app.id), (id) => handle.destroyApp(id));
      if (instanceId === null) return null;
      const nextSession: ActiveSession = { pluginId: plugin.handle.pluginId, instanceId, app, viewState: { activeModeId: app.defaultModeId ?? app.modes[0]?.id } };
      const prepared = { session: nextSession, plugin: plugin.handle, dialect, role };
      if (publish) publishPreparedArtifactOpening(prepared);
      return prepared;
    },
    [loadedPlugins, installPlugin, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, publishPreparedArtifactOpening],
  );
  openArtifactWithAppRefRef.current = openArtifactWithAppRef;

  const openReadySpaceArtifactCreation = useCallback((requestId: string): void => {
    const owner = spaceArtifactCreationOwnersRef.current.get(requestId);
    if (owner === undefined || owner.ready === null || owner.opening) return;
    const openingArgs = spaceArtifactCreationReadyOpening(owner.ready);
    const origin = openDocumentSessionsRef.current.get(owner.runtimeKey);
    if (openingArgs === null || origin === undefined) return;
    const openingOwner: SpaceArtifactCreationOwnerV1 = { ...owner, opening: true };
    const current = (): boolean => {
      const retained = spaceArtifactCreationOwnersRef.current.get(openingOwner.requestId);
      const entry = openDocumentSessionsRef.current.get(openingOwner.runtimeKey);
      return retained === openingOwner && entry?.clientInstanceId === openingOwner.clientInstanceId
        && entry.session.instanceId === openingOwner.sessionInstanceId
        && shellDialogSessionIsCurrentV1(entry.session, shellStateRef.current.pluginRuntime.session)
        && entry.scope?.spaceId === openingOwner.spaceId && entry.scope.documentId === S_SPACE_INDEX_DOCUMENT_ID;
    };
    spaceArtifactCreationOwnersRef.current.set(requestId, openingOwner);
    setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "opening", requestId, spaceId: owner.spaceId }));
    void runArtifactCreationReadyOpeningV1<PreparedArtifactOpeningTarget, DocumentOpeningReceiptV1>({
      current,
      prepare: async () => {
        const opening = resolveArtifactOpeningRelayRef.current("os.open-artifact", openingArgs);
        if (opening === null) return null;
        const target = await openArtifactWithAppRef(opening.app, opening.dialect, opening.role, current, false);
        return target === null ? null : { ...target, expectedCatalogGenerationId: openingOwner.expectedCatalogGenerationId };
      },
      open: (target) => openDocument(
        { documentId: openingArgs.documentId!, schema: openingArgs.schema!, spaceId: openingOwner.spaceId },
        undefined,
        target,
      ),
      publish: (target, receipt) => publishPreparedArtifactOpening(target, receipt),
      release: async (target, receipt) => {
        try {
          if (receipt !== null) {
            const entry = openDocumentSessionsRef.current.get(receipt.runtimeKey);
            if (entry?.clientInstanceId === receipt.clientInstanceId && entry.plugin === target.plugin && shellDialogSessionIsCurrentV1(entry.session, target.session)) {
              closeDocument(receipt.runtimeKey, receipt.clientInstanceId);
            }
            await retireDocumentAttachment(target.plugin, target.session.instanceId, receipt.clientInstanceId);
          }
        } finally {
          await target.plugin.destroyApp(target.session.instanceId);
        }
      },
      failed: (openingError) => {
        if (spaceArtifactCreationOwnersRef.current.get(requestId) !== openingOwner) return;
        spaceArtifactCreationOwnersRef.current.set(requestId, { ...openingOwner, opening: false });
        setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "open-failed", requestId, spaceId: openingOwner.spaceId }));
        console.warn("[os-shell] created artifact remains ready after opening failed", openingError);
      },
    }).then((outcome) => {
      if (outcome === "failed" || spaceArtifactCreationOwnersRef.current.get(requestId) !== openingOwner) return;
      spaceArtifactCreationOwnersRef.current.delete(requestId);
      setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "cleared", requestId }));
    });
  }, [closeDocument, openArtifactWithAppRef, openDocument, publishPreparedArtifactOpening, retireDocumentAttachment]);
  openReadySpaceArtifactCreationRef.current = openReadySpaceArtifactCreation;

  /** 👁️✏️ `DefaultAppsHostApi.rows` — one row per `(dialect, role)` pair among `knownDialects`, both
   * roles even when only one has registered surfaces (an empty `options` list still renders the row,
   * matching the Settings table's "dialect × {viewer, editor}" framing in contract freeze §5). */
  const defaultAppsRows = useMemo((): readonly DefaultAppRow[] => {
    if (!appRouter) return [];
    const rows: DefaultAppRow[] = [];
    for (const dialect of knownDialects) {
      for (const role of ["viewer", "editor"] as const) {
        const entries = appRouter.entriesFor(dialect, role);
        const pinned = pinnedAppFor(dialect, role);
        rows.push({
          dialect,
          role,
          options: entries.map((app) => ({ value: encodeDefaultAppValue(app), label: pluginLabelById.get(app.pluginId) ?? app.pluginId })),
          value: pinned ? encodeDefaultAppValue(pinned) : DEFAULT_APP_NONE_VALUE,
        });
      }
    }
    return rows;
  }, [appRouter, knownDialects, pinnedAppFor, pluginLabelById]);

  const defaultAppsHost: DefaultAppsHostApi = useMemo(
    () => ({ rows: defaultAppsRows, locale: uiLocale, setDefault: dispatchSetDefaultApp, clearDefault: dispatchClearDefaultApp }),
    [defaultAppsRows, uiLocale, dispatchSetDefaultApp, dispatchClearDefaultApp],
  );
  const defaultAppsHostRef = useRef(defaultAppsHost);
  defaultAppsHostRef.current = defaultAppsHost;

  /** ⚖️ `📌️ChromePanels`' Conflicts settings tab — `Shell`'s `selectOpenConflicts` selector feeds the
   * roster, `kindLabel`/`messageText` localize a `Conflict`'s own `ConflictKind`/first `MutationMessage`
   * (never parsed from English prose). */
  const openConflicts = useMemo(() => selectOpenConflicts(shellState), [shellState]);
  const conflictKindLabel = useCallback(
    (conflict: Conflict): string => shellLabel(conflict.kind.kind === "quarantined" ? "ui.conflict.quarantined" : "ui.conflict.degraded"),
    [],
  );
  const conflictMessageText = useCallback((conflict: Conflict): string => {
    const worst = conflict.messages[0];
    if (!worst) return "";
    return `${shellLabel(mutationCodeLabelKey(worst.code))} — ${worst.message}`;
  }, []);
  const conflictsHost: ConflictsHostApi = useMemo(
    () => ({
      conflicts: openConflicts,
      locale: uiLocale,
      selectedConflictId,
      kindLabel: conflictKindLabel,
      messageText: conflictMessageText,
      onSelect: (conflictId) => dispatch({ type: "SET_SELECTED_CONFLICT_ID", value: conflictId }),
      onResolve: dispatchResolveConflict,
      // 🐢️ No synchronous "current document as JSON" accessor is in this lease's reach yet (the
      // decoded artifact lives inside the plugin wasm instance, not mirrored into `ShellState`) — an
      // empty string is an honest "no local snapshot", never a fabricated diff side.
      currentDocumentText: "",
    }),
    [openConflicts, uiLocale, selectedConflictId, conflictKindLabel, conflictMessageText, dispatchResolveConflict],
  );
  const conflictsHostRef = useRef(conflictsHost);
  conflictsHostRef.current = conflictsHost;

  /** 📖️ Seeds the Conflicts panel with this session's authoritative roster on session start/switch
   * (`AppCommand::ReadConflicts`, contract freeze §C8/§C9) — otherwise the panel would only ever
   * show conflicts a later `setMergePolicy`/`resolveConflict` reply happened to carry, staying empty
   * across a reload even when the guest already holds `Open` conflicts. Reads
   * `loadedPluginsRef.current` rather than depending on `loadedPlugins` so a plugin hot-swap that
   * leaves the session's pluginId in place doesn't re-fire this on every unrelated roster change. */
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    let cancelled = false;
    void plugin
      .readConflicts(session.instanceId)
      .then((conflicts) => {
        if (cancelled) return;
        dispatch({ type: "SET_CONFLICTS", value: conflicts });
        dispatch({ type: "SET_INSTANCE_FAULT", value: null });
      })
      .catch((commandError) => {
        const fault = windowFaultFromError(commandError, pluginSupervisorByIdRef.current[session.pluginId]);
        console.error(`[DEBUG] readConflicts failed [${fault.class}] ${fault.code ?? "no-code"} origin=${fault.origin ?? "unknown"}`, commandError);
        if (!cancelled) dispatch({ type: "SET_INSTANCE_FAULT", value: fault });
      });
    return () => {
      cancelled = true;
    };
  }, [session?.instanceId, session?.pluginId]);

  /** 👁️✏️ "Open with…" entries for the CURRENT session's own dialect, grouped by role — the Document
   * panel section, and what the context-menu/palette entries focus. `undefined` with no session or
   * router (nothing to list yet), and for a non-surface app — one bound to no subset, such as the
   * workflow studio — which has no dialect to open anything else against. */
  const openWithEntries = useMemo(() => {
    if (!session || !appRouter || !session.app.dialect) return undefined;
    return groupOpenWithEntries(appRouter, session.app.dialect, { pluginId: session.pluginId, appId: session.app.id }, (role) => pinnedAppFor(session.app.dialect, role), pluginLabelById);
  }, [session, appRouter, pinnedAppFor, pluginLabelById]);
  const hasOpenArtifactSurfaces = (openWithEntries?.viewer.length ?? 0) + (openWithEntries?.editor.length ?? 0) > 0;

  const transientNoticeIdRef = useRef(0);
  const transientNoticeTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  /** 🧯️ Shows a non-blocking, auto-dismissing notice (contract freeze §2.3/§5) — replaces whatever
   * notice is currently showing rather than queuing, since only one can render at a time. */
  const showTransientNotice = useCallback(
    (message: string, kind: Severity = "info", code?: string) => {
      if (transientNoticeTimerRef.current) clearTimeout(transientNoticeTimerRef.current);
      transientNoticeIdRef.current += 1;
      const id = transientNoticeIdRef.current;
      dispatch({ type: "SET_TRANSIENT_NOTICE", value: { id, message, kind, code } });
      transientNoticeTimerRef.current = setTimeout(() => dispatch({ type: "SET_TRANSIENT_NOTICE", value: null }), 4000);
    },
    [dispatch],
  );
  showTransientNoticeRef.current = showTransientNotice;
  /** 🧯️ `true` for a `SemioFaultError` carrying `"viewer.read-only"` — the one host-raised fault this
   * lease knows to render as a notice instead of letting it crash into `ShellFaultBoundary`. */
  const isViewerReadOnlyFault = useCallback((error: unknown): boolean => error instanceof SemioFaultError && error.fault.code === SURFACE_FAULT_CODES.ViewerReadOnly, []);
  /** ⚖️ `true` for a `SemioFaultError` carrying `"mutation.rejected"` — one LOCAL dispatch's
   * `store.dispatch` was rejected by this authority's `MergePolicy` (contract freeze `26/08/16/
   * MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C8/§C9: `Fault.code ==
   * "mutation.rejected"`, `Fault.severity` mirrors the rejected `DispatchReport.worst`). */
  const isMutationRejectedFault = useCallback((error: unknown): boolean => error instanceof SemioFaultError && error.fault.code === MUTATION_REJECTED_FAULT_CODE, []);
  /** ⚖️ One toast per gesture for a rejected local dispatch — worst level already IS `fault.severity`
   * (that field mirrors `DispatchReport.worst`), body = the first cause's localized `ui.mutation.
   * code.*` label + its English prose, falling back to `ui.mutation.rejected.body` when the fault
   * carries no `causes` yet (the guest-side wiring that populates them is a different lane). */
  const showMutationRejectedNotice = useCallback(
    (fault: Fault) => {
      const cause = fault.causes?.[0];
      const codeLabel = cause?.code ? shellLabel(mutationCodeLabelKey(cause.code)) : shellLabel("ui.mutation.rejected.title");
      const body = cause ? `${codeLabel} — ${cause.message}` : shellLabel("ui.mutation.rejected.body");
      showTransientNotice(`${shellLabel("ui.mutation.rejected.title")}: ${body}`, fault.severity, MUTATION_REJECTED_FAULT_CODE);
    },
    [showTransientNotice],
  );
  /** ⚖️ Remote-origin merge-outcome bridge (contract freeze §C6/§C9) — fed by `applyRemoteMergeRef`
   * (see its declaration doc) from `ensureBackboneWorker`'s `remoteMutations` handling. A non-null
   * `conflicts` roster replaces the Conflicts panel's roster exactly like `dispatchResolveConflict`'s
   * reply does — `📌️ChromePanels`' panel and `ShellSync`'s quarantine badge both derive from `state.
   * merge.conflicts` (`selectOpenConflicts`/`selectQuarantinedConflicts`), so a REMOTE quarantined
   * conflict lands there with zero further wiring. A `"degraded"` outcome additionally gets THIS
   * authority's only "surfaces without being asked" channel — the same transient-notice convention
   * {@link showMutationRejectedNotice} uses for a LOCAL rejected dispatch — since a degraded merge
   * already applied silently and has no other passive indicator (unlike quarantine's badge). */
  const applyRemoteMerge = useCallback(
    (conflicts: readonly Conflict[] | null, mergeReport: MergeReport | null) => {
      if (conflicts) dispatch({ type: "SET_CONFLICTS", value: conflicts });
      if (!mergeReport?.worst || !mergeReport.conflict) return;
      const flagged = (conflicts ?? []).find((conflict) => conflict.id === mergeReport.conflict);
      if (flagged?.kind.kind !== "degraded") return;
      const worst = flagged.messages[0];
      const kindLabel = shellLabel("ui.conflict.degraded");
      const body = worst ? `${shellLabel(mutationCodeLabelKey(worst.code))} — ${worst.message}` : undefined;
      showTransientNotice(body ? `${kindLabel}: ${body}` : kindLabel, mergeReport.worst);
    },
    [showTransientNotice],
  );
  applyRemoteMergeRef.current = applyRemoteMerge;
  //#endregion 🔖️SurfaceRoles

  //#region 🔖️ThemeMutators
  const uiThemeBase = uiThemeDraft ?? uiTheme;
  const uiThemeDirty = uiThemeDraft !== null;
  const uiThemeList = useMemo((): readonly UiTheme[] => [...builtinUiThemes(), ...Object.values(uiCustomThemes)], [uiCustomThemes]);
  const uiDriverList = useMemo((): readonly UiDriver[] => [...builtinUiDrivers(), ...Object.values(uiCustomDrivers)], [uiCustomDrivers]);
  const keysByActionId = useMemo(() => buildKeysByActionId(session?.app.keybindings ?? []), [session?.app.keybindings]);
  const controlKeybindings = useMemo(() => composeControlKeybindings(keysByActionId, uiKeybindingOverrides), [keysByActionId, uiKeybindingOverrides]);
  const osCommands = useMemo(
    () => buildOsCommands(uiThemeList, [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])], activeIntroduction != null, locks, uiDriverList, activeTutorials, tutorialRecorderAvailable, uiTerminology, uiLocale, hasOpenArtifactSurfaces),
    [uiThemeList, session?.app.terminologies, activeIntroduction, uiLocale, uiTerminology, locks, uiDriverList, activeTutorials, tutorialRecorderAvailable, hasOpenArtifactSurfaces],
  );

  /** 🧭️ Direct theme/appearance/locale/terminology/driver/layout setters below (settings panel, theme/driver
   * editors) bypass `dispatchOsCommand`'s named-command path entirely — this reuses the exact same `os.*`
   * command id (and its `osCommands`-resolved, locale-adapted label) so a direct-path change folds together
   * with a command-palette-triggered one in the history panel regardless of which path triggered it. */
  const noteOsCommand = useCallback(
    (commandId: string, detail?: Record<string, unknown>) => {
      // 🩹️ `CommandDefinition.label` has no owned schema mirror yet (`unknown` — see that generated type's own
      // doc comment); resolved the same way every other manifest label in this file is.
      const rawLabel = osCommands.find((entry) => entry.id === commandId)?.label as LocalizedLabel | string | undefined;
      const label = rawLabel !== undefined ? resolveManifestLabel(rawLabel, uiTerminology, uiLocale) : commandId;
      noteShellCommand(commandId, label, detail);
    },
    [osCommands, noteShellCommand, uiTerminology, uiLocale],
  );

  const commitUiPreference = useCallback(
    (mutation: UiPreferencesConfigMutation) => {
      if (
        (mutation.mutation === "setAppearance" && locks.appearance) ||
        (mutation.mutation === "setLocale" && locks.locale) ||
        (mutation.mutation === "setTerminology" && locks.terminology) ||
        (mutation.mutation === "setTheme" && locks.themeId)
      ) return;
      commitUiPreferencesConfigMutation(scope.storage, mutation);
    },
    [locks, scope.storage],
  );

  const draftThemePatch = useCallback(
    (patch: (next: UiTheme) => void) => {
      const next = structuredClone(uiThemeBase);
      patch(next);
      dispatch({ type: "SET_UI_THEME_DRAFT", value: next });
    },
    [uiThemeBase],
  );

  const setThemeId = useCallback(
    (id: string) => {
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
      commitUiPreference(setTheme(id));
      noteOsCommand("os.setThemeId", { themeId: id });
    },
    [commitUiPreference, noteOsCommand],
  );

  const setThemeColor = useCallback(
    (key: string, hex: string) =>
      draftThemePatch((next) => {
        next.colors[key] = hex;
      }),
    [draftThemePatch],
  );
  const setThemeSpacing = useCallback(
    (key: string, value: string) =>
      draftThemePatch((next) => {
        next.spacing[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeFontStack = useCallback(
    (key: string, value: string) =>
      draftThemePatch((next) => {
        next.fontStacks[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeStroke = useCallback(
    (key: string, value: number | number[]) =>
      draftThemePatch((next) => {
        next.strokes[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeRadius = useCallback(
    (key: string, value: number) =>
      draftThemePatch((next) => {
        next.radii[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeOpacity = useCallback(
    (key: string, value: number) =>
      draftThemePatch((next) => {
        next.opacities[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeMetric = useCallback(
    (section: string, key: string, value: number | number[]) =>
      draftThemePatch((next) => {
        next.metrics[section] = { ...(next.metrics[section] ?? {}), [key]: value };
      }),
    [draftThemePatch],
  );
  const setThemeAppearancePaint = useCallback(
    (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) =>
      draftThemePatch((next) => {
        next.appearances[appearance][group][key] = alpha === undefined ? { hex } : { hex, alpha };
      }),
    [draftThemePatch],
  );

  const resetTheme = useCallback(() => {
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
    commitUiPreference(setTheme("semio"));
  }, [commitUiPreference]);

  const saveTheme = useCallback(
    (label: string) => {
      const trimmed = label.trim();
      if (!trimmed) return;
      const slug = trimmed
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "-")
        .replace(/(^-+|-+$)/g, "");
      if (!slug) return;
      const id = `custom.${slug}`;
      const saved: UiTheme = { ...uiThemeBase, id, label: trimmed };
      commitUiPreference(setCustomTheme(id, canonicalUiTheme(saved)));
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
      commitUiPreference(setTheme(id));
    },
    [commitUiPreference, uiThemeBase],
  );

  const deleteTheme = useCallback(
    (id: string) => {
      if (!id.startsWith("custom.")) return;
      commitUiPreference(setCustomTheme(id, null));
      if (uiThemeId === id) commitUiPreference(setTheme("semio"));
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
    },
    [commitUiPreference, uiThemeId],
  );

  const exportTheme = useCallback(() => {
    downloadMediaExport(`${uiThemeBase.id}.theme.dsl`, "text/plain", serializeUiTheme(uiThemeBase));
  }, [uiThemeBase]);

  const importTheme = useCallback(async () => {
    const opened = (await requestFileOpen(".theme.dsl,.dsl,text/plain"))[0];
    if (!opened) return;
    try {
      const parsed = parseUiTheme(JSON.parse(opened.contents));
      saveTheme(parsed.label || parsed.id);
    } catch {
      /* invalid theme file, ignore */
    }
  }, [saveTheme]);
  //#endregion 🔖️ThemeMutators

  //#region 🚗️DriverMutators
  const uiDriverBase = uiDriverDraft ?? uiDriver;
  const uiDriverDirty = uiDriverDraft !== null;

  const setDriverId = useCallback(
    (id: string) => {
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
      commitUiPreference(setDriver(id));
      noteOsCommand("os.setDriver", { driver: id });
    },
    [commitUiPreference, noteOsCommand],
  );

  const setDriverField = useCallback(
    <K extends keyof Omit<UiDriver, "id" | "label">>(key: K, value: UiDriver[K]) => {
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: { ...uiDriverBase, [key]: value } });
    },
    [uiDriverBase],
  );

  const saveDriver = useCallback(
    (label: string) => {
      const trimmed = label.trim();
      if (!trimmed) return;
      const slug = trimmed
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "-")
        .replace(/(^-+|-+$)/g, "");
      if (!slug) return;
      const id = `custom.${slug}`;
      const saved: UiDriver = { ...uiDriverBase, id, label: trimmed };
      commitUiPreference(setCustomDriver(id, canonicalUiDriver(saved)));
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
      commitUiPreference(setDriver(id));
    },
    [commitUiPreference, uiDriverBase],
  );

  const deleteDriver = useCallback(
    (id: string) => {
      if (!id.startsWith("custom.")) return;
      commitUiPreference(setCustomDriver(id, null));
      if (uiDriverId === id) commitUiPreference(setDriver(DEFAULT_UI_DRIVER.id));
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
    },
    [commitUiPreference, uiDriverId],
  );
  //#endregion 🚗️DriverMutators

  const [themeSaveLabel, setThemeSaveLabel] = useState("");
  const [driverSaveLabel, setDriverSaveLabel] = useState("");
  const [keybindingCaptureControlId, setKeybindingCaptureControlId] = useState<string | null>(null);
  const setKeybindingOverride = useCallback((controlId: string, keys: string) => {
    commitUiPreference(setKeybindingOverrideMutation(controlId, keys));
  }, [commitUiPreference]);
  const resetKeybindingOverride = useCallback((controlId: string) => {
    commitUiPreference(setKeybindingOverrideMutation(controlId, null));
  }, [commitUiPreference]);
  useEffect(() => {
    const onNavigateToHotkey = (event: Event) => {
      const path = (event as CustomEvent<{ readonly path?: string }>).detail?.path;
      if (path) setKeybindingCaptureControlId(path);
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-right", value: true });
      dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-right", value: [FRAMEWORK_SETTINGS_PANEL_ID, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID] });
    };
    window.addEventListener("navigate-to-hotkey", onNavigateToHotkey);
    return () => window.removeEventListener("navigate-to-hotkey", onNavigateToHotkey);
  }, [dispatch]);
  const settingsHostRef = useRef<SettingsHostApi | null>(null);
  const settingsHost: SettingsHostApi = useMemo(
    () => ({
      appId: session?.app.id,
      appLabel: session ? appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology)) : undefined,
      controllerId: session?.app.controllerId,
      pluginId: session?.pluginId,
      driverId: uiDriverId,
      driver: uiDriverBase,
      driverDirty: uiDriverDirty,
      drivers: uiDriverList,
      setDriverId,
      setDriverField,
      saveDriver,
      deleteDriver,
      driverSaveLabel,
      setDriverSaveLabel,
      appearance: uiAppearance,
      setAppearance: (value: string) => {
        commitUiPreference(setAppearance(value as ElementsSurfaceAppearance));
        noteOsCommand("os.setAppearance", { appearance: value });
      },
      layout: uiLayout,
      setLayout: (value: UiChromeLayout) => {
        commitUiPreference(setLayout(value));
        noteOsCommand("os.setLayout", { layout: value });
      },
      mobileActive: mobile,
      onResetDock: () => {
        dispatch({ type: "RESET_DOCK" });
        dockLayoutStore.reset();
        dockUiStateStore.reset();
        noteOsCommand("os.resetDock");
      },
      locale: uiLocale,
      setLocale: (value: UiLocale) => {
        commitUiPreference(setLocale(value));
        noteOsCommand("os.setLocale", { locale: value });
      },
      terminology: uiTerminology,
      setTerminology: (value: string) => {
        commitUiPreference(setTerminology(value));
        noteOsCommand("os.setTerminology", { terminology: value });
      },
      terminologies: [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])],
      theme: uiThemeBase,
      themeId: uiThemeId,
      themeDirty: uiThemeDirty,
      themes: uiThemeList,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      controlKeybindings,
      keybindingCaptureControlId,
      setKeybindingCaptureControlId,
      setKeybindingOverride,
      resetKeybindingOverride,
      locks,
      mergePolicy,
      setMergePolicy: dispatchSetMergePolicy,
    }),
    [
      session,
      dockLayoutStore,
      uiDriverId,
      uiDriverBase,
      uiDriverDirty,
      uiDriverList,
      setDriverId,
      setDriverField,
      saveDriver,
      deleteDriver,
      driverSaveLabel,
      setDriverSaveLabel,
      controlKeybindings,
      keybindingCaptureControlId,
      setKeybindingOverride,
      resetKeybindingOverride,
      uiAppearance,
      uiLayout,
      mobile,
      uiLocale,
      uiTerminology,
      uiThemeBase,
      uiThemeId,
      uiThemeDirty,
      uiThemeList,
      locks,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      noteOsCommand,
      commitUiPreference,
      mergePolicy,
      dispatchSetMergePolicy,
    ],
  );
  settingsHostRef.current = settingsHost;

  const frameworkDisplayTabs = useMemo(() => createFrameworkDisplayPanelTabs(() => displayHostRef.current), [displayHost, uiLocale]);
  const frameworkSettingsTab = useMemo(
    () => createFrameworkSettingsPanelTab(() => settingsHostRef.current, () => defaultAppsHostRef.current, () => conflictsHostRef.current),
    [settingsHost, defaultAppsHost, conflictsHost],
  );

  const marketplaceHostRef = useRef<MarketplaceHostApi | null>(null);
  const marketplaceHost: MarketplaceHostApi = useMemo(
    () => ({
      plugins: registry
        .filter((entry) => !extensionIdSet.has(entry.pluginId))
        .map((entry): MarketplacePluginEntry => {
          const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === entry.pluginId);
          return {
            pluginId: entry.pluginId,
            label: loadedEntry?.manifest.label ?? entry.pluginId,
            version: loadedEntry?.manifest.version,
            status: pluginStatusById[entry.pluginId] ?? "available",
            sourceId: pluginSource.id,
            canUninstall: entry.pluginId !== primaryPluginId && session?.pluginId !== entry.pluginId,
          };
        }),
      extensions: (() => {
        const byId = new Map<string, MarketplaceExtensionEntry>();
        for (const target of EXTENSION_TARGETS) {
          const ledger = extensionLedger.find((entry) => entry.extensionId === target.pluginId);
          const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === target.pluginId);
          byId.set(target.pluginId, {
            extensionId: target.pluginId,
            label: loadedEntry?.manifest.label ?? target.pluginId,
            version: ledger?.version ?? loadedEntry?.manifest.version,
            extendsHost: ledger?.extendsHost ?? target.extends ?? "unscoped",
            enabled: ledger?.enabled ?? false,
            status: pluginStatusById[target.pluginId] ?? (ledger ? "loaded" : "available"),
          });
        }
        for (const ledger of extensionLedger) {
          if (byId.has(ledger.extensionId)) continue;
          const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === ledger.extensionId);
          byId.set(ledger.extensionId, {
            extensionId: ledger.extensionId,
            label: loadedEntry?.manifest.label ?? ledger.extensionId,
            version: ledger.version,
            extendsHost: ledger.extendsHost,
            enabled: ledger.enabled,
            status: pluginStatusById[ledger.extensionId] ?? "loaded",
          });
        }
        return [...byId.values()];
      })(),
      installPlugin: (pluginId) => void installPlugin(pluginId),
      uninstallPlugin: (pluginId) => void uninstallPlugin(pluginId),
      reloadPlugin: (pluginId) => void reloadPlugin(pluginId),
      installExtensionFromUrl: (sourceUri) => void installExtension(sourceUri),
      installExtensionFromFile: (file) => void installExtensionFromFile(file),
      uninstallExtension: (extensionId) => void uninstallExtension(extensionId),
      setExtensionEnabled: (extensionId, enabled) => void setExtensionEnabled(extensionId, enabled),
    }),
    [
      registry,
      extensionIdSet,
      extensionLedger,
      loadedPlugins,
      pluginStatusById,
      pluginSource.id,
      primaryPluginId,
      session?.pluginId,
      installPlugin,
      uninstallPlugin,
      reloadPlugin,
      installExtension,
      installExtensionFromFile,
      uninstallExtension,
      setExtensionEnabled,
    ],
  );
  marketplaceHostRef.current = marketplaceHost;
  const frameworkMarketplaceTab = useMemo(() => createFrameworkMarketplacePanelTab(() => marketplaceHostRef.current), [marketplaceHost]);

  // 🐚️ Gated to this shell via `useShellKeydown` below — was an unconditional `window` keydown listener,
  // so every mounted shell fired its bound action (and could `preventDefault()` out from under another
  // shell) for every keystroke on the page regardless of which shell the user was actually using.
  const handleAppKeydown = useCallback(
    (event: globalThis.KeyboardEvent) => {
      if (event.defaultPrevented) return;
      if (!session) return;
      const parseKeys = (keys: string) =>
        keys
          .split(",")
          .map((key) => key.trim().toLowerCase())
          .filter(Boolean);
      const isEditableTarget = (target: EventTarget | null) => {
        if (!(target instanceof HTMLElement)) return false;
        const tag = target.tagName;
        if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
        if (target.isContentEditable) return true;
        return target.closest("[contenteditable='true'], [role='textbox']") != null;
      };
      const matches = (event: globalThis.KeyboardEvent, binding: string) => {
        const parts = binding.split("+").map((part) => part.trim());
        const key = parts[parts.length - 1] ?? "";
        const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
        const needsShift = parts.includes("shift");
        const needsAlt = parts.includes("alt");
        const hasCtrl = event.ctrlKey || event.metaKey;
        if (needsCtrl !== hasCtrl) return false;
        if (needsShift !== event.shiftKey) return false;
        if (needsAlt !== event.altKey) return false;
        return event.key.toLowerCase() === key;
      };
      const focusedWindowId = activeWindowIdRef.current ?? session.viewState.windowId ?? session.viewState.activeWindowKindId;
      const focusedWindowKindId = sessionWindowInstances(session.app, extraWindowInstancesRef.current).find((instance) => instance.id === focusedWindowId)?.windowKindId ?? focusedWindowId;
      const actionById = new Map((session.app.windowKinds.find((kind) => kind.id === focusedWindowKindId)?.actions ?? []).map((action) => [action.id, action]));
      if (isEditableTarget(event.target)) return;
      // 🧰️🛠️ Escape deactivates the active window's active utility (P5), or — when no utility is active —
      // the active mode-level tool, when nothing is being typed.
      if (event.key === "Escape") {
        const windowId = activeWindowIdRef.current;
        if (windowId && activeUtilityByWindowIdRef.current[windowId]) {
          event.preventDefault();
          onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { windowId, utilityId: "" } });
          return;
        }
        if (activeToolIdRef.current) {
          event.preventDefault();
          onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "" } });
          return;
        }
      }
      for (const binding of session.app.keybindings) {
        for (const chord of parseKeys(binding.keys)) {
          if (!matches(event, chord)) continue;
          const definition = actionById.get(binding.action.action);
          if (!definition) continue;
          event.preventDefault();
          // ✍️ Arg-carrying hotkeys never silent-fire defaults (P4): open the staged form, or — if that
          // form is already expanded in the active window — treat the hotkey as Execute (with validation).
          if (actionRequiresStagedForm(definition)) {
            const retainedPaste = pasteActionWithRetainedFragment({ action: definition.id }, clipboardFragmentRef.current);
            if (pasteArgsFragment(retainedPaste) !== undefined) {
              onAction({ controllerId: session.app.controllerId, action: retainedPaste.action, args: retainedPaste.args });
              return;
            }
            const windowId = activeWindowIdRef.current;
            if (!windowId) return;
            const expanded = actionPaneExpandedByWindowIdRef.current[windowId] ?? null;
            const staged = actionPaneStagedArgsByKeyRef.current[actionStageKey(windowId, definition.id)] ?? {};
            const intent = resolveKeybindingIntent(definition, expanded, staged);
            if (intent.kind === "execute") {
              onAction({ controllerId: session.app.controllerId, action: intent.actionId, args: intent.args });
            } else if (intent.kind === "open") {
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: intent.actionId });
            }
            return;
          }
          onAction(binding.action);
          return;
        }
      }
      // ⏪️ Framework-universal undo/redo chords — apps shadow them via their own keybindings above;
      // routed through the same `onAction` funnel as the History panel's Undo/Redo tree rows, so the
      // remote/local routing in the `action === "undo"` branch applies identically.
      if ((event.ctrlKey || event.metaKey) && !event.altKey && event.key.toLowerCase() === "z") {
        event.preventDefault();
        onAction({ controllerId: session.app.controllerId, action: event.shiftKey ? "redo" : "undo" });
        return;
      }
      if ((event.ctrlKey || event.metaKey) && !event.altKey && !event.shiftKey && event.key.toLowerCase() === "y") {
        event.preventDefault();
        onAction({ controllerId: session.app.controllerId, action: "redo" });
        return;
      }
    },
    [onAction, session],
  );
  useShellKeydown(scope.rootRef, handleAppKeydown, [handleAppKeydown]);

  const activeRightPanelTab = session?.app.panelTabs.find((tab) => panelAnchorForGroup(tab.group) === "top-right");
  const activePanelTabId = panel?.activePanelTab ?? (activeRightPanelTab ? panelTabKindId(activeRightPanelTab.kind) : undefined) ?? (session?.app.panelTabs[0] ? panelTabKindId(session.app.panelTabs[0].kind) : undefined);

  const workbenchLeftTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    const pluginLeftTabs = session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-left").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay, uiTerminology, uiLocale));
    if (hostMode && session.app.id === hostAppId && pluginLeftTabs.length > 0) return pluginLeftTabs;
    const hasPluginArtifactTab = flattenPanelTabNodeLeaves(pluginLeftTabs).some((tab) => tab.id === FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    if (hasPluginArtifactTab) return pluginLeftTabs;
    // 👁️✏️ "Open with…" — contract freeze §5's Document-panel surface: one section per role,
    // `AppRouter` entries owner-first, each row opens that surface for the SAME artifact; the
    // pinned default gets a "Set as default" toggle already on, everyone else gets it off.
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
  }, [appLabelsOverlay, onAction, panel?.spawnedApps.length, panelUiByKey, session, hostMode, uiLocale, uiTerminology, hostAppId, openWithEntries, openWithFocusRole, openArtifactWithAppRef, dispatchSetDefaultApp, dispatchClearDefaultApp]);

  const detailsRightTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    return session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-right").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay, uiTerminology, uiLocale));
  }, [appLabelsOverlay, onAction, panelUiByKey, session, uiTerminology, uiLocale]);

  /**
   * 🧭️ The two bottom anchors' app-declared tabs — `PanelGroup::Display` → `bottom-left`,
   * `PanelGroup::Settings` → `bottom-right`. Without these an app could declare either group and the
   * dock assembly dropped the tab silently: the guest still rendered the body (every declared tab is
   * flattened into `buildUiRefreshRequest`) and the result was cached in `panelUiByKey` and then never
   * mounted, which is exactly how puzzle3d's own Settings section — grid spacing, chunk size,
   * proximity radius, overlap budget — was unreachable while the framework's own `framework.settings`
   * branch beside it opened fine.
   *
   * 🕰️ {@link shellRendersPanelTabItself} keeps the framework-injected `framework.panel.history` out: the
   * shell builds that tab itself ({@link frameworkUtilitiesHistoryTab}), and mounting the app's copy beside
   * it put two identically-named tab buttons carrying one DOM id in this anchor — the guest-rendered twin
   * winning the id lookup and renaming every row to `panel:<key>/framework.history.entry.<seq>`.
   */
  const appTabsForBottomAnchor = useCallback(
    (anchor: ReturnType<typeof panelAnchorForGroup>): PanelTabNode[] =>
      session
        ? session.app.panelTabs
            .filter((tab) => panelAnchorForGroup(tab.group) === anchor && !shellRendersPanelTabItself(panelTabKindId(tab.kind)))
            .map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay, uiTerminology, uiLocale))
        : [],
    [appLabelsOverlay, onAction, panelUiByKey, session, uiTerminology, uiLocale],
  );
  const displayBottomLeftTabs = useMemo(() => appTabsForBottomAnchor("bottom-left"), [appTabsForBottomAnchor]);
  const settingsBottomRightTabs = useMemo(() => appTabsForBottomAnchor("bottom-right"), [appTabsForBottomAnchor]);

  //#region 🔖️CheckIn — ticket §C5 "when the user edits an artifact, the mutations are saved and
  // checked into vcs": status pill (`#s-sync-status`), auto check-in (idle ≥ 20s or ≥ 200 uncommitted
  // edits), explicit check-in (`#s-checkin`), checkpoint-on-close, and the post-checkpoint
  // `TouchArtifact` relay to the space index. Placed ahead of `🧰️FooterUtilityLeaves`/`🔄️SyncLeaf`
  // (their `useMemo`s below close over these) — everything here is additive, no existing behaviour
  // changed for a session outside a hub-bound space.
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
  const currentDocumentId = currentDocumentRuntimeKey === null ? null : (openDocumentSessionsRef.current.get(currentDocumentRuntimeKey)?.documentId ?? null);
  const selectedSpaceArtifactCreationCatalog = useMemo(() => selectedSpaceArtifactCreationCatalogV1(
    spaceArtifactCreationCatalog,
    spaceArtifactCreationCatalogUi,
    session === null ? null : captureDialogOrigin(session),
  ), [captureDialogOrigin, currentDocumentId, currentDocumentRuntimeKey, session, spaceArtifactCreationCatalog, spaceArtifactCreationCatalogUi]);
  const selectedSpaceArtifactKinds = selectedSpaceArtifactCreationCatalog?.kinds;
  const artifactCreationChoiceRevisionRef = useRef<string | undefined>(undefined);
  const currentBrowserActorUi = useMemo(
    () => currentDocumentRuntimeKey === null ? undefined : browserActorUiByRuntimeKeyRef.current.get(currentDocumentRuntimeKey),
    [browserActorUiVersion, currentDocumentRuntimeKey],
  );

  // 👥️ Host-only normalized roster, keyed by the exact verified document runtime. It never enters a
  // plugin view-state payload, so an app cannot forge or persist Shell presence chrome.
  const presencePeers = useMemo((): readonly PresencePeer[] => currentDocumentRuntimeKey === null ? [] : (presencePeersByRuntimeKey[currentDocumentRuntimeKey] ?? []), [currentDocumentRuntimeKey, presencePeersByRuntimeKey]);

  const currentSyncStatus = currentDocumentRuntimeKey ? (syncStatusByDocumentId[currentDocumentRuntimeKey] ?? null) : null;
  const syncPillState: SyncPillState = useMemo(() => computeSyncPillState(currentSyncStatus), [currentSyncStatus]);

  /** 📌️ Uncommitted-since-last-checkpoint count, derived purely from the already-tracked
   * `historyProjection.entries` (no new wire field): every applied `mutation`-kind entry counts,
   * reset to 0 the moment a `commitCheckpoint` (`kind: "history"`) entry is seen — mirrors
   * `store::uncommitted_edit_ids`'s own "since the last Change" semantics closely enough for an
   * auto-checkin heuristic (undo/redo of an already-committed edit is the one case this
   * under/over-counts by one entry; not worth threading `applied_edit_ids` all the way to the host
   * for this). */
  const uncommittedEditCount = useMemo(() => {
    const entries = Object.values(historyProjection.entries).sort((left, right) => left.seq - right.seq);
    let pending = 0;
    for (const entry of entries) {
      if (entry.kind === "history" && entry.actionId === "commitCheckpoint") {
        pending = 0;
        continue;
      }
      if (entry.kind === "mutation" && entry.applied !== false) pending += 1;
    }
    return pending;
  }, [historyProjection.entries]);

  const isEditorSession = canCheckIn(session?.app.role);

  useEffect(() => {
    void backgroundSpaceIndexSessionsRef.current.retain(owned => identity !== null && owned.hubBaseUrl === identity.hubBaseUrl && owned.userId === identity.userId && loadedPlugins.some(entry => entry.handle === owned.plugin) && openDocumentSessionsRef.current.get(owned.runtimeKey)?.clientInstanceId === owned.clientInstanceId)
      .catch(error => console.error("[DEBUG] background document authority retirement failed", error));
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
        const liveEntry = currentDocumentId === S_SPACE_INDEX_DOCUMENT_ID && currentDocumentRuntimeKey !== null ? openDocumentSessionsRef.current.get(currentDocumentRuntimeKey) : undefined;
        if (liveEntry?.scope?.spaceId === spaceId && liveEntry.session.app.role === "editor") {
          await touch(liveEntry.plugin, liveEntry.session);
          return;
        }
        const current = (owned: BackgroundSpaceIndexSession) => identityIsCurrent() && owned.hubBaseUrl === identity.hubBaseUrl && owned.userId === identity.userId && loadedPluginsRef.current.some(entry => entry.handle === owned.plugin) && openDocumentSessionsRef.current.get(owned.runtimeKey)?.clientInstanceId === owned.clientInstanceId;
        await backgroundSpaceIndexSessionsRef.current.run(spaceId, {
          current,
          create: async () => {
            const pluginEntry = loadedPluginsRef.current.find(entry => findDialectApp(entry, SPACE_INDEX_DIALECT, "editor"));
            const app = pluginEntry && findDialectApp(pluginEntry, SPACE_INDEX_DIALECT, "editor");
            if (!pluginEntry || !app || !identityIsCurrent()) return null;
            const plugin = pluginEntry.handle;
            const instanceId = await createAdmittedShellInstanceV1(() => identityIsCurrent() && loadedPluginsRef.current.some(entry => entry.handle === plugin), () => plugin.createApp(app.id), id => plugin.destroyApp(id));
            if (instanceId === null) return null;
            const targetSession: ActiveSession = { pluginId: plugin.pluginId, instanceId, app, viewState: { activeModeId: app.defaultModeId ?? app.modes[0]?.id } };
            let retained = false;
            try {
              if (!identityIsCurrent() || !loadedPluginsRef.current.some(entry => entry.handle === plugin)) return null;
              const receipt = await openDocumentRef.current({ documentId: S_SPACE_INDEX_DOCUMENT_ID, schema: S_SPACE_INDEX_DOCUMENT_SCHEMA, spaceId }, undefined, { session: targetSession, plugin, background: true });
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
        console.error("[DEBUG] touchSpaceIndexArtifact failed", touchError);
      }
    },
    [currentDocumentId, currentDocumentRuntimeKey, retireDocumentAttachment],
  );

  /** 📌️ Fires `commitCheckpoint` through the SAME action funnel the History panel's own quick
   * "Checkpoint" button uses (`history_command` in `🔌️plugin/component.rs`) — `message` is optional
   * (auto check-ins pass `"auto"`), `authors` rides along for when the framework threads it (today it
   * doesn't — `history_command` hardcodes `authors: Vec::new()`, see `📓️w3-a-report.md`'s
   * sharedFileRequest). `checkpointDispatchedRef` lets the effect below tell "a checkpoint we asked
   * for landed" apart from "the session just mounted with a pre-existing checkpoint". */
  const checkpointDispatchedRef = useRef(false);
  const dispatchCheckpoint = useCallback(
    (message: string) => {
      if (!session) return;
      checkpointDispatchedRef.current = true;
      const authors = identityRef.current ? [{ id: identityRef.current.userId, name: identityRef.current.displayName }] : [];
      onAction({ controllerId: session.app.controllerId, action: "commitCheckpoint", args: { message, authors } });
    },
    [session, onAction],
  );

  // 📌️ §C5 item 6 continued — `TouchArtifact` fires once per checkpoint THIS shell asked for,
  // detected as `historyProjection.currentCheckpointId` changing away from whatever it was the last
  // time this effect ran (never on the initial mount/session-snapshot, which isn't a checkpoint WE
  // just made).
  const previousCheckpointIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    const previous = previousCheckpointIdRef.current;
    const next = historyProjection.currentCheckpointId;
    previousCheckpointIdRef.current = next;
    if (!checkpointDispatchedRef.current || !next || next === previous) return;
    checkpointDispatchedRef.current = false;
    const spaceId = openSpaceIdRef.current;
    if (spaceId && currentDocumentId && currentDocumentId !== S_SPACE_INDEX_DOCUMENT_ID) {
      void touchSpaceIndexArtifact(spaceId, currentDocumentId);
    }
  }, [historyProjection.currentCheckpointId, currentDocumentId, touchSpaceIndexArtifact]);

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
    const scheduler = new AutoCheckinScheduler(() => dispatchCheckpoint("auto"));
    autoCheckinSchedulerRef.current = scheduler;
    return () => scheduler.cancel();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isEditorSession, currentDocumentId, dispatchCheckpoint]);
  useEffect(() => {
    autoCheckinSchedulerRef.current?.notify(uncommittedEditCount);
  }, [uncommittedEditCount]);

  // 📌️ §C5 item 4 — checkpoint on close: fires from the cleanup of an effect keyed on
  // `[session, currentDocumentId]`, so it runs the instant either changes (switching document/app —
  // this shell keeps exactly one session mounted, so "switch away" IS "close" here) as well as on true
  // unmount. Best-effort (fire-and-forget, not gated on the success-detection effect above — by the
  // time the response arrives `historyProjection` may already belong to the NEW session).
  const uncommittedEditCountRef = useRef(uncommittedEditCount);
  uncommittedEditCountRef.current = uncommittedEditCount;
  useEffect(() => {
    if (!isEditorSession || !currentDocumentId) return;
    const documentId = currentDocumentId;
    const spaceId = openSpaceIdRef.current;
    return () => {
      if (uncommittedEditCountRef.current > 0) {
        dispatchCheckpoint("auto");
        if (spaceId && documentId !== S_SPACE_INDEX_DOCUMENT_ID) void touchSpaceIndexArtifact(spaceId, documentId);
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [session, currentDocumentId]);

  // 📌️ §C5 item 3 — explicit check-in: `#s-checkin` opens a small message dialog (local, ephemeral
  // state — never persisted, never survives a session switch), then dispatches with that message.
  const [checkinDialog, setCheckinDialog] = useState<{ readonly message: string } | null>(null);
  const submitCheckin = useCallback(() => {
    if (!checkinDialog) return;
    dispatchCheckpoint(checkinDialog.message.trim().length > 0 ? checkinDialog.message.trim() : "check-in");
    setCheckinDialog(null);
  }, [checkinDialog, dispatchCheckpoint]);
  //#endregion 🔖️CheckIn

  //#region 🧰️FooterUtilityLeaves — bottom-right's History tab, sourced from the framework-injected
  // `framework.panel.history` panel tab (every app gets one — see `AppBuilder::build_definition`).
  const mountedInferenceHistory = useMemo(() => {
    if (!session) return null;
    const matches = Object.entries(inferenceHistoryByRuntimeKey).filter(([runtimeKey, history]) => {
      const document = openDocumentSessionsRef.current.get(runtimeKey);
      return document?.session.pluginId === session.pluginId && document.session.instanceId === session.instanceId && document.clientInstanceId === history.clientInstanceId && history.sessionInstanceId === session.instanceId;
    });
    return matches.length === 1 ? matches[0]![1] : null;
  }, [inferenceHistoryByRuntimeKey, session]);
  const shellUndoRoute = shellHistoryUndoRouteV1(mountedInferenceHistory === null ? null : { ...mountedInferenceHistory.status, order: mountedInferenceHistory.order }, { canUndo: historyProjection.canUndo, order: localHistoryOrderRef.current });
  const shellCanUndo = shellUndoRoute === "remote" || shellUndoRoute === "local";
  const frameworkUtilitiesHistoryTab = useMemo((): PanelTabNode | null => {
    if (!session) return null;
    const tab = session.app.panelTabs.find((candidate) => panelTabKindId(candidate.kind) === FRAMEWORK_PANEL_TAB_HISTORY_ID);
    if (!tab) return null;
    // 👁️✏️ "renders the history panel read-only" (contract freeze §2.3) — undo/redo stay visible but
    // disabled (contract freeze §5's "disables undo/redo"), checkpoint/revert-to-command (both
    // mutating) are hidden outright rather than disabled, since neither has a meaningful disabled
    // affordance for a session that can never enable them.
    const isViewer = session.app.role === "viewer";
    const entries = Object.values(historyProjection.entries).sort((left, right) => right.seq - left.seq);
    return singleTreeLeaf({
      id: FRAMEWORK_PANEL_TAB_HISTORY_ID,
      icon: shellTabIcon("undo"),
      name: resolvePanelTabLabel(appLabelsOverlay, FRAMEWORK_PANEL_TAB_HISTORY_ID, resolveManifestLabel(tab.label as LocalizedLabel | string, uiTerminology, uiLocale)),
      order: 1,
      tree: {
        sections: [
          {
            id: "framework.history.actions",
            label: shellLabel("ui.panel.history"),
            items: [
              { id: "framework.history.undo", label: "", control: <button type="button" disabled={isViewer || !shellCanUndo} onClick={() => onAction({ controllerId: session.app.controllerId, action: "undo" })}>Undo</button> },
              { id: "framework.history.redo", label: "", control: <button type="button" disabled={isViewer || !historyProjection.canRedo} onClick={() => onAction({ controllerId: session.app.controllerId, action: "redo" })}>Redo</button> },
              // 📌️ §C5 items 3/5 — `#s-checkin` (explicit check-in, opens a message dialog) is a
              // SEPARATE affordance from the no-message quick "Checkpoint" button above; both are
              // absent outright for a viewer (never disabled — a viewer role has no meaningful
              // disabled affordance for either, mirroring undo/redo's own comment above `isViewer`).
              // `!canCheckIn(...)` here specifically (not the local `isViewer`) so this gate is the
              // SAME tested predicate `📓️w3-a-report.md`'s viewer-guard test exercises.
              ...(!canCheckIn(session.app.role)
                ? []
                : [
                    { id: "framework.history.checkpoint", label: "", control: <button type="button" onClick={() => onAction({ controllerId: session.app.controllerId, action: "commitCheckpoint" })}>Checkpoint</button> },
                    {
                      id: "framework.history.checkin",
                      label: "",
                      control: checkinDialog ? (
                        <span style={{ display: "inline-flex", gap: 4 }}>
                          <input
                            id="s-checkin-message"
                            type="text"
                            value={checkinDialog.message}
                            placeholder={checkinMessagePlaceholderText(uiLocale)}
                            onChange={(event) => setCheckinDialog({ message: event.target.value })}
                            onKeyDown={(event) => {
                              if (event.key === "Enter") submitCheckin();
                              if (event.key === "Escape") setCheckinDialog(null);
                            }}
                          />
                          <button type="button" onClick={submitCheckin}>{checkinSubmitText(uiLocale)}</button>
                          <button type="button" onClick={() => setCheckinDialog(null)}>{checkinCancelText(uiLocale)}</button>
                        </span>
                      ) : (
                        <button type="button" id="s-checkin" onClick={() => setCheckinDialog({ message: "" })}>
                          {checkinActionText(uiLocale)}
                          {uncommittedEditCount > 0 ? ` (${uncommittedEditCount})` : ""}
                        </button>
                      ),
                    },
                  ]),
            ],
          },
          {
            id: "framework.history.commands",
            label: "Commands",
            items: entries.map((entry) => ({
              id: `framework.history.entry.${entry.seq}`,
              label: entry.count && entry.count > 1 ? `${entry.label} ×${entry.count}` : entry.label,
              description: entry.opLines?.join(" · "),
              dimmed: entry.applied === false,
              control: entry.revertible && !isViewer ? <button type="button" onClick={() => onAction({ controllerId: session.app.controllerId, action: "revertToCommand", args: { entrySeq: entry.seq } })}>↶</button> : undefined,
            })),
          },
        ],
      },
    });
  }, [appLabelsOverlay, checkinDialog, historyProjection, onAction, session, shellCanUndo, submitCheckin, uiLocale, uiTerminology, uncommittedEditCount]);
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
                    onAttach={attachSyncBackbone}
                    onDetach={detachSyncBackbone}
                  />
                ),
              },
            ],
          },
        ],
      },
    });
  }, [attachSyncBackbone, detachSyncBackbone, onAction, syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId, syncPillState, quarantinedConflicts, uiLocale]);
  //#endregion 🔄️SyncLeaf

  const activePluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest, [loadedPlugins, session?.pluginId]);
  const activeModeId = session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "";

  // 📱️ Moved ahead of `mobilePanelTabs` (below) so its synthetic mobile "App" tab can share the exact
  // example-select/mode-switcher elements the desktop navbar center cluster renders — single source of truth.
  // 📚️ Resolved by DIALECT, never by app id: an example is a document of the artifact's subset, so
  // the viewer of a dialect offers exactly the same picker its editor does (ticket
  // 26/09/09/PROCEDURAL-3D-END-TO-END). `examplesForApp` is the shared predicate — Rust
  // `manifest::examples_for_app` is its twin, both pinned by `📚️example-picker.json`.
  const exampleOptions = useMemo(() => {
    const app = session?.app;
    if (!app) return [];
    return examplesForApp(activePluginManifest?.examples ?? [], app).map((example) => ({
      id: example.id,
      label: resolveAppLabel(appLabelsOverlay, "example", example.id, resolveManifestLabel(example.label, uiTerminology, uiLocale)),
      // 🩹️ `PluginManifest.examples` entries carry no icon (`{id, label, artifactJson, dialect}` only,
      // same shape kernel's own `PluginManifest` declares) — a generic fallback, not a fabricated field.
      icon: "file" as IconName,
    }));
  }, [activePluginManifest, session?.app, appLabelsOverlay, uiTerminology, uiLocale]);
  exampleOptionsRef.current = exampleOptions;

  const dispatchActiveExample = useCallback(
    (exampleId: string) => {
      if (!session) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin) return;
      void onAction(buildActiveExampleAction(session.app.controllerId, exampleId));
    },
    [applyHostEffects, injectActiveUtility, loadedPlugins, onAction, session],
  );

  /** @emoji 🎛️ Shared by the desktop navbar center cluster and the mobile panel's synthetic "App" tab (see `mobilePanelTabs`). */
  const exampleSelectElement = useMemo(() => {
    if (!session || exampleOptions.length === 0 || locks.exampleId || (hostMode && session.app.id === landingAppId)) return null;
    return (
      <NavbarExampleSelect
        key="fixture"
        id="playground.navbar.fixture"
        value={activeExampleId}
        options={exampleOptions}
        onValueChange={(exampleId) => {
          if (exampleId) lastDispatchedExampleIdRef.current = exampleId;
          dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
          dispatchActiveExample(exampleId || "");
        }}
      />
    );
  }, [session, exampleOptions, locks.exampleId, hostMode, landingAppId, activeExampleId, dispatchActiveExample]);

  /** @emoji 🎛️ Shared by the desktop navbar center cluster and the mobile panel's synthetic "App" tab (see `mobilePanelTabs`).
   * `aria-keyshortcuts` republishes the two framework mode-cycling chords on the GROUP, which is where
   * they belong: the chords step through the group rather than addressing any one button. */
  const modeSwitcherElement = useMemo(() => {
    if (!session || session.app.modes.length <= 1) return null;
    const stepShortcuts = ariaKeyshortcutsText([controlKeybindings.get(MODE_STEP_CONTROL_IDS.next), controlKeybindings.get(MODE_STEP_CONTROL_IDS.previous)].filter(Boolean).join(","));
    return (
      <ButtonGroup key="modes" id="playground.navbar.modes" role="group" aria-label={appModeGroupText(uiLocale)} aria-keyshortcuts={stepShortcuts}>
        {session.app.modes.map((mode) => {
          const isActive = activeModeId === mode.id;
          return (
            <ButtonGroupItem
              key={mode.id}
              id={`playground.navbar.modes.${mode.id}`}
              className={cn(isActive && interactiveActiveFillClass)}
              data-state={isActive ? "on" : undefined}
              aria-pressed={isActive}
              onClick={() => applyModeChange(mode.id)}
              icon={mode.iconId}
              text={resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label as LocalizedLabel | string, uiTerminology, uiLocale))}
            />
          );
        })}
      </ButtonGroup>
    );
  }, [session, activeModeId, applyModeChange, appLabelsOverlay, controlKeybindings, uiTerminology, uiLocale]);

  /** 👁️✏️ The plugin's editor/viewer pair for the OPEN document's dialect, or `null` when it declares
   * fewer than two surfaces for it — which is exactly when the role group must not render. */
  const sessionRoleApps = useMemo((): Readonly<Record<AppRole, AppDefinition>> | null => {
    if (!session) return null;
    const apps = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.manifest.apps ?? [];
    return surfaceRoleAppsV1(apps, session.app.dialect);
  }, [loadedPlugins, session]);

  /** @emoji 👁️✏️ Navbar role group — the in-shell control that makes `…#viewer` reachable without a
   * reload (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 2). Labels come from each target
   * `AppDefinition`'s OWN localized label, so en/de follow the plugin rather than a shell dictionary.
   * Shares the mode switcher's placement in both the desktop navbar cluster and the mobile "App" tab. */
  const roleSwitcherElement = useMemo(() => {
    if (!session || sessionRoleApps === null) return null;
    return (
      // 🚦️ `aria-busy` on the GROUP, disabled on the items: a switch is a transaction over the whole
      // group (it quiesces, retires and mounts), never over one button, and a second press while one is
      // running is refused by `sessionSwitchGateRef` rather than queued — so the group must say so.
      <ButtonGroup key="roles" id="playground.navbar.roles" role="group" aria-label={surfaceRoleGroupText(uiLocale)} aria-busy={surfaceSwitchBusy || undefined}>
        {SURFACE_ROLE_ORDER.map((role) => {
          const app = sessionRoleApps[role];
          const isActive = session.app.role === role;
          return (
            <ButtonGroupItem
              key={role}
              id={SURFACE_ROLE_CONTROL_IDS[role]}
              className={cn(isActive && interactiveActiveFillClass)}
              data-state={isActive ? "on" : undefined}
              data-role={role}
              aria-pressed={isActive}
              disabled={surfaceSwitchBusy && !isActive}
              aria-keyshortcuts={ariaKeyshortcutsText(controlKeybindings.get(SURFACE_ROLE_CONTROL_IDS[role]))}
              onClick={() => void switchToSessionRole(role).catch((switchError) => console.error(`role switch to ${role} failed`, switchError))}
              icon={SURFACE_ROLE_ICON_IDS[role]}
              text={resolveManifestLabel(app.label as LocalizedLabel | string, uiTerminology, uiLocale)}
            />
          );
        })}
      </ButtonGroup>
    );
  }, [session, sessionRoleApps, surfaceSwitchBusy, switchToSessionRole, controlKeybindings, uiTerminology, uiLocale]);

  const resolvedCommands = useMemo(() => {
    const resolved = resolveCommands(osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale, loadedPlugins.map((entry) => entry.manifest), selectedSpaceArtifactKinds);
    // 👁️✏️ Hides every `Mutation`-kind command from a viewer session's palette (contract freeze §5) —
    // `resolveCommands` itself stays role-agnostic (os hosts/tests call it without a session at all).
    return session?.app.role === "viewer" ? resolved.filter((entry) => !isMutationKindDefinition(entry.definition)) : resolved;
  }, [osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale, loadedPlugins, selectedSpaceArtifactKinds]);

  useLayoutEffect(() => {
    const revision = selectedSpaceArtifactCreationCatalog?.choiceRevision;
    if (artifactCreationChoiceRevisionRef.current === revision) return;
    artifactCreationChoiceRevisionRef.current = revision;
    const choiceArgs = (args: readonly { readonly id: string; readonly schema: { readonly kind: string; readonly format?: { readonly kind: string } } }[]) =>
      args.map((arg) => ({ id: arg.id, artifactKind: arg.schema.kind === "string" && arg.schema.format?.kind === "artifactKind" }));
    const actionDefinitions: ArtifactKindChoiceDraftDefinitionV1[] = [];
    if (session !== null) {
      for (const instance of sessionWindowInstances(session.app, extraWindowInstances)) {
        const kind = session.app.windowKinds.find((candidate) => candidate.id === instance.windowKindId);
        if (kind === undefined) continue;
        for (const definition of resolveWindowActions(session.app, kind)) actionDefinitions.push({ ownerId: actionStageKey(instance.id, definition.id), args: choiceArgs(definition.args) });
      }
    }
    for (const spawned of panel?.spawnedApps ?? []) {
      const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      const kind = app?.windowKinds[0];
      if (app === undefined || kind === undefined) continue;
      for (const definition of resolveWindowActions(app, kind)) actionDefinitions.push({ ownerId: actionStageKey(spawned.id, definition.id), args: choiceArgs(definition.args) });
    }
    for (const retirement of artifactKindChoiceDraftRetirementsV1(actionPaneStagedArgsByKey, actionDefinitions)) {
      const split = retirement.ownerId.lastIndexOf(":");
      if (split < 1) continue;
      dispatch({ type: "STAGE_ACTION_ARG", windowId: retirement.ownerId.slice(0, split), actionId: retirement.ownerId.slice(split + 1), argId: retirement.argId, value: undefined });
    }
    const commandDefinitions = resolvedCommands.map((entry) => ({ ownerId: commandAddressKey(entry.address), args: choiceArgs(entry.definition.args) }));
    for (const retirement of artifactKindChoiceDraftRetirementsV1(commandStagedArgsByCommandId, commandDefinitions)) {
      dispatch({ type: "STAGE_COMMAND_ARG", commandId: retirement.ownerId, argId: retirement.argId, value: undefined });
    }
  }, [actionPaneStagedArgsByKey, commandStagedArgsByCommandId, extraWindowInstances, loadedPlugins, panel?.spawnedApps, resolvedCommands, selectedSpaceArtifactCreationCatalog?.choiceRevision, session]);

  const commandCategoryList = useMemo(() => commandCategories(resolvedCommands), [resolvedCommands, uiLocale]);

  useEffect(() => {
    const valid = new Set(resolvedCommands.map((entry) => commandAddressKey(entry.address)));
    if (expandedCommandIdRef.current && !valid.has(expandedCommandIdRef.current)) dispatch({ type: "SET_COMMAND_EXPANDED", value: null });
    for (const commandKey of Object.keys(commandStagedArgsByCommandIdRef.current)) {
      if (!valid.has(commandKey)) dispatch({ type: "RESET_COMMAND_ARGS", commandId: commandKey });
    }
  }, [resolvedCommands]);

  /**
   * 🎛️ Dispatches a resolved command: os-scope commands are handled locally (no program round trip);
   * plugin/app/mode-scope commands route through the active session's program `handleCommand`, mirroring
   * `onAction`'s tail. Plugin commands are only resolvable/dispatchable for the active session's program
   * instance (no headless-instance routing for non-focused plugins yet).
   */
  const onCommand = useCallback(
    (address: CommandAddress, args?: Record<string, unknown>) => {
      const commandId = address.commandId;
      // 🎥️ Same sandbox-start/recorder-arm side effects `START_TUTORIAL_ACTION_ID`/`RECORD_TUTORIAL_ACTION_ID`
      // need — routed through the `startTutorialRef`/`toggleTutorialRecordingRef` bridge since they need
      // more context (plugin bridge, sandbox snapshot) than a bare `dispatch` gives `dispatchOsCommand`.
      if (isOsCommandAddress(address) && commandId === "os.playTutorial") {
        const tutorialId = typeof args?.tutorialId === "string" ? args.tutorialId : "";
        if (tutorialId) startTutorialRef.current(tutorialId);
        return;
      }
      if (isOsCommandAddress(address) && commandId === "os.recordTutorial") {
        toggleTutorialRecordingRef.current();
        return;
      }
      if (isOsCommandAddress(address) && commandId === "os.toggleFullscreen") {
        void toggleDocumentFullscreen(scope.rootRef.current ?? document.documentElement).catch((error) => console.error("Fullscreen request was rejected", error));
      }
      if (isOsCommandAddress(address)) {
        dispatchOsCommand(commandId, args, commitUiPreference, dispatch, dockLayoutStore, dockUiStateStore, locks);
        const rawCommandLabel = resolvedCommands.find((entry) => commandAddressKey(entry.address) === commandAddressKey(address))?.definition.label as LocalizedLabel | string | undefined;
        const label = rawCommandLabel !== undefined ? resolveManifestLabel(rawCommandLabel, uiTerminology, uiLocale) : commandId;
        noteShellCommand(commandId, label, args);
        return;
      }
      if (!session) return;
      // ⏺️ Recorder tap for plugin/app/mode-scope commands — mirrors `onAction`'s tap above.
      if (tutorialRecordingRef.current && !tutorialDrivenRef.current.active) {
        tutorialRecorderRef.current?.recordEvent({ kind: "command", command: commandId, args });
      }
      const ownerPluginId = commandOwnerPluginId(address.owner);
      if (!ownerPluginId || ownerPluginId !== session.pluginId) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === ownerPluginId)?.handle;
      if (!plugin) return;
      // 👁️✏️ Same client-side half of the read-only guarantee `onAction` applies above, for commands.
      if (session.app.role === "viewer" && resolvedCommands.find((entry) => commandAddressKey(entry.address) === commandAddressKey(address))?.definition.kind === "mutation") {
        showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
        return;
      }
      const dispatchViewState = injectActiveUtility(session.viewState);
      const invocation: CommandInvocation = { address, arguments: args ?? {} };
      const commandOrigin = captureDialogOrigin(session);
      if (!isCurrentDialogOrigin(commandOrigin)) return;
      let directBrowserActor: ReturnType<typeof directBrowserActorForSession>;
      try {
        directBrowserActor = directBrowserActorForSession(session);
      } catch (error) {
        console.error("[DEBUG] authenticated browser actor command owner failed", error);
        return;
      }
      if (directBrowserActor !== null) {
        void dispatchDirectBrowserActorCommand(directBrowserActor, invocation, dispatchViewState).catch((error) => {
          console.error("[DEBUG] authenticated browser actor command failed", error);
          showTransientNotice(shellLabel("ui.common.renderError"), "error");
        });
        return;
      }
      if (!plugin.handleCommand) return;
      const commandOwner = captureEffectOwner(session, commandOrigin);
      if (!isCurrentEffectOwner(commandOwner)) return;
      void plugin
        .handleCommand(session.instanceId, JSON.stringify(invocation), dispatchViewState)
        .then((response) => {
          if (!isCurrentEffectOwner(commandOwner)) return;
          applyHistoryPatch(response.historyPatch);
            applyLeftoverInteractionView(response.output, "actionId" in invocation.address ? invocation.address.actionId : undefined);
          return applyHostEffects(response.requestedEffects ?? [], { ...session, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope), commandOwner);
        })
        .catch((error) => {
          if (isViewerReadOnlyFault(error)) {
            showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
            return;
          }
          if (isMutationRejectedFault(error)) {
            showMutationRejectedNotice((error as SemioFaultError).fault);
            return;
          }
          console.error("Command execution failed", error);
        });
    },
    [applyHostEffects, applyHistoryPatch, applyLeftoverInteractionView, captureDialogOrigin, isCurrentDialogOrigin, captureEffectOwner, directBrowserActorForSession, dispatchDirectBrowserActorCommand, isCurrentEffectOwner, commitUiPreference, dockLayoutStore, dockUiStateStore, injectActiveUtility, loadedPlugins, session, locks, resolvedCommands, noteShellCommand, showTransientNotice, isViewerReadOnlyFault, uiLocale],
  );

  const handleCommandKeydown = useCallback(
    (event: globalThis.KeyboardEvent) => {
      if (event.defaultPrevented || isEditableEventTarget(event.target)) return;
      for (const entry of [...resolvedCommands].reverse()) {
        if (!entry.definition.inPalette) continue;
        const entryKey = commandAddressKey(entry.address);
        const platform = detectCommandPlatform(typeof navigator !== "undefined" ? `${navigator.platform} ${navigator.userAgent}` : "");
        const keys = commandKeybindingChords(entry.definition, platform).join(",");
        if (!keys?.split(",").some((chord) => keyboardEventMatchesChord(event, chord.trim().toLowerCase()))) continue;
        event.preventDefault();
        const staged = commandStagedArgsByCommandIdRef.current[entryKey] ?? {};
        const intent = resolveKeybindingIntent(entry.definition, expandedCommandIdRef.current === entryKey ? entry.definition.id : null, staged);
        if (intent.kind === "fire") onCommand(entry.address);
        else if (intent.kind === "execute") onCommand(entry.address, intent.args);
        else {
          const commandPath = [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${entry.definition.category}`];
          if (mobile) {
            dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
            dispatch({ type: "SET_MOBILE_PANEL_PATH", value: commandPath });
          } else {
            dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
            dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: commandPath });
          }
          dispatch({ type: "SET_COMMAND_EXPANDED", value: entryKey });
        }
        return;
      }
    },
    [mobile, onCommand, resolvedCommands],
  );
  useShellKeydown(scope.rootRef, handleCommandKeydown, [handleCommandKeydown]);

  const commandCategoryTabs = useMemo(() => buildCommandCategoryTabs(resolvedCommands, commandCategoryList, expandedCommandIdRef, commandStagedArgsByCommandIdRef, onCommand, dispatch), [resolvedCommands, commandCategoryList, onCommand]);

  // 🗺️ `ToolDefinition.label` is a manifest `LocalizedLabel` field — resolved here, right after
  // `resolveModeTools` (an external `framework-os-core` helper this file cannot edit), so every
  // downstream consumer (`buildToolTree`/`buildToolTabs`) keeps reading an already-plain-string `label`.
  const resolvedModeTools = useMemo(
    () => resolveModeTools(session?.app, activeModeId).map((tool) => ({ ...tool, label: resolveManifestLabel(tool.label as LocalizedLabel | string, uiTerminology, uiLocale) })),
    [session?.app, activeModeId, uiTerminology, uiLocale],
  );

  // 🐢️ Only the presence of a session gates the tool tabs, so this memo (and `defaultDock`'s, which
  // consumes it) keeps its identity across every session object churn.
  const hasToolSession = session !== null && session !== undefined;
  const toolTabs = useMemo(() => (hasToolSession ? buildToolTabs(resolvedModeTools, toolMeasuresByToolIdRef, onActionStable) : []), [hasToolSession, resolvedModeTools, onActionStable]);

  //#region 🧭️DockAssembly — default four-corner arrangement (the two middle anchors start empty save the command palette in bottom-middle) + persisted-override reconciliation + drag-and-drop wiring.
  const defaultDock = useMemo((): PanelDock => {
    // 🧭️ Top-left (Workbench: Document/Catalogue) and top-right (Details: Inspection/Parameters) stay flat.
    // Bottom-right exposes one Settings branch whose children are internal tabs, plus one Marketplace leaf.
    const topLeft: PanelTabNode[] = [...workbenchLeftTabs];
    const bottomLeft: PanelTabNode[] = [];
    if (frameworkDisplayTabs.length > 0) {
      bottomLeft.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 0, children: frameworkDisplayTabs });
    }
    bottomLeft.push(...displayBottomLeftTabs);
    if (frameworkSyncTab) bottomLeft.push(frameworkSyncTab);
    const topRight: PanelTabNode[] = [...detailsRightTabs];
    // 🧭️ The open document's own settings lead the anchor; the shell-wide Settings branch and the
    // Marketplace are chrome that applies to every app, so they sit behind it — the same "document first,
    // shell last" ordering the bottom-left anchor uses for its sync leaf.
    const bottomRight: PanelTabNode[] = [...settingsBottomRightTabs, frameworkSettingsTab, frameworkMarketplaceTab];
    if (frameworkUtilitiesHistoryTab) bottomRight.push(frameworkUtilitiesHistoryTab);
    // 🛠️ Tool categories stay nested under one expandable Tool branch, exactly like Command categories,
    // placed left of Command (order 0 vs 1) — like commands not being window-level, tools are not
    // window-level either; both live only on this shared mode-scoped anchor.
    // 🎛️ Command categories stay nested under one expandable Command branch so the folded bottom-middle
    // chrome shows a single Command toggle, not every category leaf inlined along the footer.
    const bottomMiddle: PanelTabNode[] = [
      ...(toolTabs.length > 0 ? [{ kind: "branch" as const, id: FRAMEWORK_CATEGORY_TOOL_ID, icon: categoryTabIcon(toolTabs, "hammer"), name: shellLabel("ui.panelToggle.tool"), order: 0, children: toolTabs }] : []),
      ...(commandCategoryTabs.length > 0 ? [{ kind: "branch" as const, id: FRAMEWORK_CATEGORY_COMMAND_ID, icon: categoryTabIcon(commandCategoryTabs, "wrench"), name: shellLabel("ui.panelToggle.command"), order: 1, children: commandCategoryTabs }] : []),
    ];
    return { anchors: { "top-left": topLeft, "top-middle": [], "top-right": topRight, "right-middle": [], "bottom-right": bottomRight, "bottom-middle": bottomMiddle, "bottom-left": bottomLeft, "left-middle": [] } };
  }, [commandCategoryTabs, detailsRightTabs, displayBottomLeftTabs, frameworkDisplayTabs, frameworkMarketplaceTab, frameworkSettingsTab, frameworkSyncTab, frameworkUtilitiesHistoryTab, settingsBottomRightTabs, toolTabs, uiLocale, workbenchLeftTabs]);

  useEffect(() => {
    dispatch({ type: "SET_DOCK_OVERRIDE", value: dockLayoutStore.getSnapshot() });
  }, [dockLayoutStore]);

  const dock = useMemo((): PanelDock => applyDockSkeleton(defaultDock, dockOverride), [defaultDock, dockOverride]);

  // 📱️ All eight anchors' tabs flattened into the single mobile panel's tab list — defined here (ahead of the
  // dock-assembly override effects below) so those effects can resolve a mobile-panel path alongside the
  // desktop per-anchor one.
  const mobilePanelTabs = useMemo(() => {
    const anchorTabs = ANCHORS.flatMap((anchor) => defaultDock.anchors[anchor]);
    // 📱️ The example selector and mode switcher have no navbar room on mobile (see `navbarItems`) — they
    // surface as one more tab in the merged mobile panel instead, sharing the exact same elements the
    // desktop navbar center cluster renders.
    if (!exampleSelectElement && !modeSwitcherElement && !roleSwitcherElement) return anchorTabs;
    const appTab = singleTreeLeaf({
      id: "framework.mobile.app",
      icon: shellTabIcon("smartphone"),
      name: shellLabel("ui.mobilePanel.app"),
      order: 99,
      tree: {
        sections: [
          {
            id: "framework.mobile.app.root",
            label: "",
            items: [
              ...(exampleSelectElement ? [{ id: "framework.mobile.app.example", label: "", control: exampleSelectElement }] : []),
              ...(modeSwitcherElement ? [{ id: "framework.mobile.app.modes", label: "", control: modeSwitcherElement }] : []),
              ...(roleSwitcherElement ? [{ id: "framework.mobile.app.roles", label: "", control: roleSwitcherElement }] : []),
            ],
          },
        ],
      },
    });
    return [...anchorTabs, appTab];
  }, [defaultDock, exampleSelectElement, modeSwitcherElement, roleSwitcherElement]);

  /** 🗄️ Skips the very first (pre-hydration) commit so a persisted skeleton isn't clobbered with `null` before the seeding effect above has a chance to read and apply it. */
  const dockPersistedOnceRef = useRef(false);
  useEffect(() => {
    if (!dockPersistedOnceRef.current) {
      dockPersistedOnceRef.current = true;
      return;
    }
    const nextSkeleton = dockSkeletonOf(dock);
    const defaultSkeleton = dockSkeletonOf(defaultDock);
    dockLayoutStore.save(dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton);
  }, [dock, defaultDock, dockLayoutStore]);

  useEffect(() => {
    dispatch({ type: "HYDRATE_DOCK_UI", value: dockUiStateStore.getSnapshot() });
  }, [dockUiStateStore]);

  /** 🗄️ Same first-commit-skip as the dock skeleton effect above, but also re-arms when the store identity itself changes (app switch) — otherwise the new app's pre-hydration state would be written into its own key on the first post-switch commit. */
  const dockUiPersistedOnceRef = useRef(false);
  const dockUiPersistedStoreRef = useRef(dockUiStateStore);
  useEffect(() => {
    if (dockUiPersistedStoreRef.current !== dockUiStateStore) {
      dockUiPersistedStoreRef.current = dockUiStateStore;
      dockUiPersistedOnceRef.current = false;
    }
    if (!dockUiPersistedOnceRef.current) {
      dockUiPersistedOnceRef.current = true;
      return;
    }
    const anchors: Partial<Record<Anchor, DockUiPanelState>> = {};
    for (const anchor of ANCHORS) {
      const panelState = panels[anchor];
      const entry: DockUiPanelState = {};
      if (panelState.visible) entry.visible = true;
      if (panelState.size !== DEFAULT_PANEL_WIDTH_PX) entry.size = panelState.size;
      if (panelState.path.length > 0) entry.path = panelState.path;
      if (Object.keys(entry).length > 0) anchors[anchor] = entry;
    }
    const hasPathMemory = Object.keys(panelPathMemory).length > 0;
    const hasTreeOpen = Object.keys(treeOpenStates).length > 0;
    const isDefault = Object.keys(anchors).length === 0 && !hasPathMemory && !hasTreeOpen;
    dockUiStateStore.save(isDefault ? null : { version: 3, anchors, pathMemory: hasPathMemory ? panelPathMemory : undefined, treeOpen: hasTreeOpen ? treeOpenStates : undefined });
  }, [panels, panelPathMemory, treeOpenStates, dockUiStateStore]);

  const handleTabDockDrop = useCallback(
    (move: PanelTabDockMove) => {
      const nextDock = moveTabInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      const targetPath = findPanelTabPath(nextDock.anchors[move.target.anchor], move.tabId);
      if (targetPath) dispatch({ type: "SET_PANEL_PATH", anchor: move.target.anchor, value: targetPath });
      if (move.fromAnchor !== move.target.anchor) {
        const sourceTabs = nextDock.anchors[move.fromAnchor];
        dispatch({ type: "SET_PANEL_PATH", anchor: move.fromAnchor, value: (prev) => reconcileActivePath(sourceTabs, prev, panelTabChildren) });
      }
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
      noteShellCommand("shell.dockMove", shellLabel("ui.shellCommand.dockMove"), { tabId: move.tabId, fromAnchor: move.fromAnchor, toAnchor: move.target.anchor });
    },
    [dock, defaultDock, noteShellCommand],
  );

  const handleTreeUnitDockDrop = useCallback(
    (move: PanelTreeUnitDockMove) => {
      const nextDock = moveTreeUnitInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
      noteShellCommand("shell.dockMove", shellLabel("ui.shellCommand.dockMove"), { toAnchor: move.target.anchor });
    },
    [dock, defaultDock, noteShellCommand],
  );

  const hostOverrideTabId = hostMode && session?.app.id === hostAppId ? (panel?.activePanelTab ?? hostCatalogueTabId) : undefined;
  const studioOverrideAnchor = hostOverrideTabId ? findPanelTabInDock(dock, hostOverrideTabId)?.anchor : undefined;
  const detailsOverrideTabId = panel?.activePanelTab;
  const detailsOverrideAnchor = detailsOverrideTabId ? findPanelTabInDock(dock, detailsOverrideTabId)?.anchor : undefined;

  /** @emoji 🎓️ The current introduction step's target element ids (`introduce` + `show`), classified by
   * shape — `null` unless that shape is present, so every reveal override below (here and in
   * `modeWindows`) is a plain truthiness check. A folded utility bar/Actions rail/dock panel would
   * otherwise hide the target from ever mounting (see `useIntroductionAnchorRect`), leaving the step
   * centered with no cutout and no way for the user to find what to do. Ids are matched, never
   * reconstructed: a `framework.window.{segment}` id's segment is `elementIdSegment(windowId)`, a lossy
   * camelCase normalization — comparing `elementIdSegment(windowId) === segment` OR the same for the
   * instance's window-kind id is the only safe check (Top/Perspective instances share a kind). */
  const activeIntroductionStep = activeIntroduction && introductionStepIndex != null ? (activeIntroduction.steps[introductionStepIndex] ?? null) : null;
  const introductionElementIds = useMemo(
    (): readonly string[] => (activeIntroductionStep ? [activeIntroductionStep.introduce, ...activeIntroductionStep.show].filter((id): id is string => Boolean(id)) : []),
    [activeIntroductionStep],
  );
  const introductionUtilityId = useMemo(() => {
    if (!session) return null;
    const utilities = session.app.utilities ?? [];
    return introductionElementIds.find((id) => utilities.some((utility) => utility.id === id)) ?? null;
  }, [introductionElementIds, session]);
  const introductionActionWindowSegment = useMemo(() => {
    for (const id of introductionElementIds) {
      const rest = id.startsWith("framework.window.") ? id.slice("framework.window.".length) : null;
      const actionIndex = rest?.indexOf(".action.") ?? -1;
      if (rest && actionIndex >= 0) return rest.slice(0, actionIndex);
    }
    return null;
  }, [introductionElementIds]);
  const introductionPanelTabId = useMemo(() => {
    for (const id of introductionElementIds) {
      if (id.startsWith("framework.panelTab.")) {
        const rest = id.slice("framework.panelTab.".length);
        return rest.endsWith(".firstDraggable") ? rest.slice(0, -".firstDraggable".length) : rest;
      }
    }
    return null;
  }, [introductionElementIds]);
  /** 🛠️ Tool ids the active step asks the user to activate (`interactions` of kind `tool`, or a bare
   * `tool.<id>` introduce/show). Reveals the Tool category chrome so the leaf tab can be pressed —
   * never drills into the leaf itself (that would open the inactive activate-toggle tree and, via tab
   * selection, auto-activate + celebrate before the user acts). */
  const introductionToolPickIds = useMemo((): readonly string[] => {
    const fromInteractions = (activeIntroductionStep?.interactions ?? [])
      .filter((interaction): interaction is IntroductionInteraction & { readonly on: { readonly kind: "tool"; readonly id: string } } => interaction.on.kind === "tool")
      .map((interaction) => interaction.on.id);
    if (fromInteractions.length > 0) return fromInteractions;
    return introductionElementIds.flatMap((id) => {
      const match = /^tool\.([a-z][a-zA-Z0-9]*)$/.exec(id);
      return match?.[1] ? [match[1]] : [];
    });
  }, [activeIntroductionStep, introductionElementIds]);
  const introductionPanelTabAnchor = introductionPanelTabId ? findPanelTabInDock(dock, introductionPanelTabId)?.anchor : undefined;
  const introductionUtilityWindowId = useMemo(() => {
    if (!introductionUtilityId || !session) return null;
    for (const kind of session.app.windowKinds) {
      const utilities = resolveUtilityNodes(session.app, kind, null, kind.id, appLabelsOverlay, uiTerminology, uiLocale);
      if (utilityNodeTreeContainsId(utilities, introductionUtilityId)) return kind.id;
    }
    return null;
  }, [appLabelsOverlay, introductionUtilityId, session, uiTerminology, uiLocale]);
  /** 🎓️ Window-kind id whose measures tree owns an introduce/show measure id — force-unfolds the Window
   * Options rail so targets like `puzzle3d-play-vortex-show` can mount for the tour. */
  const introductionMeasureWindowId = useMemo(() => {
    if (!session || introductionElementIds.length === 0) return null;
    for (const kind of session.app.windowKinds) {
      const kindMeasures = kind.options.measures ?? [];
      if (introductionElementIds.some((id) => windowMeasureTreeContainsId(kindMeasures, id))) return kind.id;
      for (const [windowId, measures] of Object.entries(windowMeasuresByWindowId)) {
        if (!introductionElementIds.some((id) => windowMeasureTreeContainsId(measures, id))) continue;
        if (windowId === kind.id || extraWindowInstances.some((instance) => instance.id === windowId && instance.windowKindId === kind.id)) return kind.id;
      }
    }
    return null;
  }, [extraWindowInstances, introductionElementIds, session, windowMeasuresByWindowId]);

  /** 🛠️ Tool id whose measure tree owns an introduce/show id — keeps mode-level tools like fill
   * active so targets such as `puzzle3d-play-distribution` stay mounted for the tour. */
  const introductionToolId = useMemo(() => {
    if (introductionElementIds.length === 0) return null;
    for (const [toolId, measures] of Object.entries(toolMeasuresByToolId)) {
      if (introductionElementIds.some((id) => windowMeasureTreeContainsId(measures, id))) return toolId;
    }
    return null;
  }, [introductionElementIds, toolMeasuresByToolId]);

  const lastIntroductionToolIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (!introductionToolId || !session) {
      lastIntroductionToolIdRef.current = null;
      return;
    }
    if (lastIntroductionToolIdRef.current === introductionToolId && activeToolIdRef.current === introductionToolId) return;
    lastIntroductionToolIdRef.current = introductionToolId;
    if (activeToolIdRef.current === introductionToolId) return;
    onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: introductionToolId } });
  }, [introductionToolId, onActionStable, session]);

  /** 🛠️ Tool-pick steps (e.g. Füllen): open the Tool category so `tool.<id>` leaf tabs mount in the
   * panel chrome, clear any already-active tool so the user must activate it, and never select the
   * leaf path (selecting auto-activates and would celebrate before they act). */
  const lastIntroductionToolPickStepIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (!session || introductionToolPickIds.length === 0 || !activeIntroductionStep) {
      lastIntroductionToolPickStepIdRef.current = null;
      return;
    }
    // 🛠️ Measure-driven keep-alive (`introductionToolId`) owns activation for steps that introduce
    // tool measures (fill-distribution) — don't fight it by clearing the tool.
    if (introductionToolId) return;
    if (lastIntroductionToolPickStepIdRef.current === activeIntroductionStep.id) return;
    lastIntroductionToolPickStepIdRef.current = activeIntroductionStep.id;
    for (const toolId of introductionToolPickIds) {
      if (activeToolIdRef.current === toolId) {
        onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "" } });
      }
    }
    if (mobile) {
      const resolved = findPanelTabPath(mobilePanelTabs, FRAMEWORK_CATEGORY_TOOL_ID);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
      return;
    }
    const toolAnchor = findPanelTabInDock(dock, FRAMEWORK_CATEGORY_TOOL_ID)?.anchor ?? "bottom-middle";
    const resolved = findPanelTabPath(dock.anchors[toolAnchor], FRAMEWORK_CATEGORY_TOOL_ID);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: toolAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: toolAnchor, value: true });
  }, [activeIntroductionStep, dock, introductionToolId, introductionToolPickIds, mobile, mobilePanelTabs, onActionStable, session]);

  const lastIntroductionPanelTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!introductionPanelTabId || !introductionPanelTabAnchor) {
      lastIntroductionPanelTabIdRef.current = undefined;
      return;
    }
    if (lastIntroductionPanelTabIdRef.current === introductionPanelTabId) return;
    lastIntroductionPanelTabIdRef.current = introductionPanelTabId;
    if (mobile) {
      const resolved = findPanelTabPath(mobilePanelTabs, introductionPanelTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
      return;
    }
    const resolved = findPanelTabPath(dock.anchors[introductionPanelTabAnchor], introductionPanelTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: introductionPanelTabAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: introductionPanelTabAnchor, value: true });
  }, [introductionPanelTabId, introductionPanelTabAnchor, dock, mobile, mobilePanelTabs]);

  /** 🎓️ Panel interactions complete when their named panel tab is open and visible — checked for every
   * `panel` interaction of the active step, not just the first, so a step can require opening several. */
  useEffect(() => {
    if (!activeIntroductionStep) return;
    for (const interaction of activeIntroductionStep.interactions ?? []) {
      if (interaction.on.kind !== "panel") continue;
      const tabId = interaction.on.id;
      const located = findPanelTabInDock(dock, tabId);
      if (!located) continue;
      const panel = panels[located.anchor];
      if (!panel.visible || !panel.path.includes(tabId)) continue;
      completeIntroductionInteraction((candidate) => candidate.on.kind === "panel" && candidate.on.id === tabId);
    }
  }, [activeIntroductionStep, completeIntroductionInteraction, dock, panels]);

  /** 🎓️ Expand interactions start with every named tree section forced closed on step entry, then
   * complete individually as the user opens each one. */
  const lastIntroductionExpandStepIdRef = useRef<string | null>(null);
  useEffect(() => {
    const expandInteractions = (activeIntroductionStep?.interactions ?? []).filter((interaction) => interaction.on.kind === "expand");
    if (!activeIntroductionStep || expandInteractions.length === 0) {
      lastIntroductionExpandStepIdRef.current = null;
      return;
    }
    if (lastIntroductionExpandStepIdRef.current !== activeIntroductionStep.id) {
      lastIntroductionExpandStepIdRef.current = activeIntroductionStep.id;
      for (const interaction of expandInteractions) {
        const stateSuffix = `tree-section-${interaction.on.id}`;
        const catalogueKey = `${FRAMEWORK_PANEL_TAB_CATALOGUE_ID}.tree:${stateSuffix}`;
        dispatch({ type: "SET_TREE_OPEN_STATE", id: catalogueKey, open: false });
      }
      return;
    }
    for (const interaction of expandInteractions) {
      const sectionId = interaction.on.id;
      const stateSuffix = `tree-section-${sectionId}`;
      const expanded = Object.entries(treeOpenStates).some(([key, open]) => open && key.endsWith(stateSuffix));
      if (expanded) completeIntroductionInteraction((candidate) => candidate.on.kind === "expand" && candidate.on.id === sectionId);
    }
  }, [activeIntroductionStep, completeIntroductionInteraction, treeOpenStates]);

  /** 🧭️ Progressive reveal means a stored path can legitimately end at a branch (or be empty) — this is now a plain per-anchor truncation-validate, no override reassertion (see the write-through effects below). */
  const panelActivePaths = useMemo((): Record<Anchor, readonly string[]> => {
    const result = {} as Record<Anchor, readonly string[]>;
    for (const anchor of ANCHORS) result[anchor] = reconcileActivePath(dock.anchors[anchor], panels[anchor].path, panelTabChildren);
    return result;
  }, [panels, dock]);

  const historyTabOpenRef = useRef(false);
  useEffect(() => {
    if (!session) {
      historyTabOpenRef.current = false;
      return;
    }
    const historyOpen = Object.values(panelActivePaths).some((path) => path.includes(FRAMEWORK_PANEL_TAB_HISTORY_ID));
    const opened = historyOpen && !historyTabOpenRef.current;
    historyTabOpenRef.current = historyOpen;
    if (!opened) return;
    refreshHistorySnapshot(session.instanceId);
  }, [panelActivePaths, refreshHistorySnapshot, session]);

  /**
   * 🧭️ Generalizes the old `leftPanelActivePath`/`rightPanelActivePath` studio/plugin "snap to the active panel
   * tab" overrides across all eight anchors. Write-through rather than read-time: each override dispatches
   * `SET_PANEL_PATH` only when its target tab id actually changes, so a user's own collapse/navigation
   * afterward sticks instead of being reasserted on every render (progressive reveal made read-time reassertion
   * fight the user's own collapses). Studio wins over details when both would touch the same anchor.
   **/
  const lastStudioOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!hostOverrideTabId || !studioOverrideAnchor) {
      lastStudioOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastStudioOverrideTabIdRef.current === hostOverrideTabId) return;
    lastStudioOverrideTabIdRef.current = hostOverrideTabId;
    if (mobile) {
      if (mobilePanelPath[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
      const resolved = findPanelTabPath(mobilePanelTabs, hostOverrideTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      return;
    }
    if (panels[studioOverrideAnchor].path[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
    const resolved = findPanelTabPath(dock.anchors[studioOverrideAnchor], hostOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: studioOverrideAnchor, value: resolved });
  }, [hostOverrideTabId, studioOverrideAnchor, dock, panels, mobile, mobilePanelTabs, mobilePanelPath]);

  useEffect(() => {
    const leftoverIds = leftoverInspectionIdsRef.current.length > 0 ? leftoverInspectionIdsRef.current : leftoverWorldSelectionOverlayV1()?.ids ?? [];
    if (leftoverInspectionEpoch === 0 || leftoverIds.length === 0) return;
    const located = findPanelTabInDock(dock, FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    if (located) {
      const resolved = findPanelTabPath(dock.anchors[located.anchor], FRAMEWORK_PANEL_TAB_INSPECTION_ID);
      const current = panels[located.anchor]?.path ?? [];
      if (resolved && current.join("/") !== resolved.join("/")) dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: resolved });
      if (!panels[located.anchor]?.visible) dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
      console.warn("[DEBUG] leftover Inspection tab", JSON.stringify({ anchor: located.anchor, path: resolved ?? [FRAMEWORK_PANEL_TAB_INSPECTION_ID] }));
    }
    const inspectionCacheKey = `panel:${FRAMEWORK_PANEL_TAB_INSPECTION_ID}`;
    const cachedInspection = uiRefreshCacheRef.current.get(inspectionCacheKey);
    const inspectionHash = leftoverInspectionPanelHash(leftoverIds, cachedInspection?.hash);
    if (inspectionHash === undefined) {
      uiRefreshCacheRef.current.delete(inspectionCacheKey);
      for (const key of [...uiRefreshCacheRef.current.keys()]) {
        if (key.includes("inspection") || key.includes("inspector")) uiRefreshCacheRef.current.delete(key);
      }
    }
    const scope = leftoverInspectionRefreshScope(leftoverIds);
    const currentSession = sessionRef.current;
    if (scope && currentSession) {
      let cancelled = false;
      const timer = window.setTimeout(() => {
        if (cancelled) return;
        void refreshUi(currentSession, scope, undefined, true);
      }, 250);
      return () => {
        cancelled = true;
        window.clearTimeout(timer);
      };
    }
  }, [leftoverInspectionEpoch, refreshUi]);

  useEffect(() => {
    return subscribeLeftoverWorldSelectionV1(() => {
      const leftover = leftoverWorldArmedWindowOverlayV1();
      const hoverVortex = leftover?.hoveredId && (leftover.hoveredDomain === "vortex" || leftover.hoveredId.includes(":")) ? leftover.hoveredId : null;
      const utility = leftover?.activeUtility;
      if (leftoverBrushTickSettledRef.current && leftoverBrushPreviewWindowHash(utility, hoverVortex, "cached") === undefined) {
        leftoverBrushHoverKeyRef.current = hoverVortex;
        leftoverBrushRefreshPendingRef.current = true;
        if (!leftoverBrushRefreshInFlightRef.current) {
          leftoverBrushPreviewEpochRef.current += 1;
          setLeftoverBrushPreviewEpoch(leftoverBrushPreviewEpochRef.current);
        }
      }
    });
  }, []);

  useEffect(() => {
    const leftover = leftoverWorldArmedWindowOverlayV1();
    const hoverVortex = leftover?.hoveredId && (leftover.hoveredDomain === "vortex" || leftover.hoveredId.includes(":")) ? leftover.hoveredId : null;
    const scope = leftoverBrushPreviewRefreshScope(leftover?.activeUtility, hoverVortex);
    if (leftoverBrushPreviewEpoch === 0 || !scope || !leftoverBrushTickSettledRef.current) return;
    for (const key of [...uiRefreshCacheRef.current.keys()]) {
      if (key.startsWith("window:")) uiRefreshCacheRef.current.delete(key);
    }
    leftoverBrushRefreshPendingRef.current = true;
    if (leftoverBrushRefreshInFlightRef.current) return;
    leftoverBrushRefreshInFlightRef.current = true;
    void (async () => {
      try {
        while (leftoverBrushRefreshPendingRef.current) {
          leftoverBrushRefreshPendingRef.current = false;
          const live = leftoverWorldArmedWindowOverlayV1();
          const liveHover = live?.hoveredId && (live.hoveredDomain === "vortex" || live.hoveredId.includes(":")) ? live.hoveredId : null;
          const liveSignal = leftoverBrushPreviewRefreshScope(live?.activeUtility, liveHover);
          const liveSession = sessionRef.current;
          if (!liveSignal || !liveSession) continue;
          const windowBodies = sessionWindowInstances(liveSession.app, extraWindowInstancesRef.current).map((instance) => instance.bodyKey).filter((key): key is string => Boolean(key));
          const liveScope = windowBodies.length > 0 ? { kind: "partial" as const, windowBodies } : liveSignal;
          for (const key of [...uiRefreshCacheRef.current.keys()]) {
            if (key.startsWith("window:")) uiRefreshCacheRef.current.delete(key);
          }
          await refreshUi(liveSession, liveScope, undefined, true);
        }
      } finally {
        leftoverBrushRefreshInFlightRef.current = false;
      }
    })();
  }, [leftoverBrushPreviewEpoch, refreshUi]);

  // 🎯️ Moving focus between panes changes what every app-level panel body is addressed at
  // (`ViewModel::focused_window_id`), so the panel bodies are re-fetched for the newly focused pane.
  // Only the panel half: window bodies are re-projected per instance and cannot move on focus alone,
  // and the refresh is hash-conditional, so a panel that does not read the focused pane answers with
  // its unchanged hash and costs one compare (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B15).
  const lastFocusedPanelWindowIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (lastFocusedPanelWindowIdRef.current === activeWindowId) return;
    lastFocusedPanelWindowIdRef.current = activeWindowId;
    const currentSession = sessionRef.current;
    if (!currentSession) return;
    const panelBodies = flattenPanelTabLeaves(currentSession.app.panelTabs)
      .map((tab) => tab.bodyKey)
      .filter((bodyKey): bodyKey is string => Boolean(bodyKey));
    if (panelBodies.length === 0) return;
    void refreshUi(currentSession, { kind: "partial", panelBodies });
  }, [activeWindowId, refreshUi]);

  const lastDetailsOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!detailsOverrideTabId || !detailsOverrideAnchor) {
      lastDetailsOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastDetailsOverrideTabIdRef.current === detailsOverrideTabId) return;
    lastDetailsOverrideTabIdRef.current = detailsOverrideTabId;
    if (detailsOverrideAnchor === studioOverrideAnchor) return;
    // 🧭️ Skip the override while the Settings branch is active, so browsing Theme/Hotkeys is never stomped.
    if (mobile) {
      if (mobilePanelPath[0] === FRAMEWORK_SETTINGS_PANEL_ID) return;
      const resolved = findPanelTabPath(mobilePanelTabs, detailsOverrideTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      return;
    }
    if (panels[detailsOverrideAnchor].path[0] === FRAMEWORK_SETTINGS_PANEL_ID) return;
    const resolved = findPanelTabPath(dock.anchors[detailsOverrideAnchor], detailsOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: detailsOverrideAnchor, value: resolved });
  }, [detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, dock, panels, mobile, mobilePanelTabs, mobilePanelPath]);

  /**
   * 🛠️ Single owner of "the selected `tool.<id>` leaf tab IS the active tool" (see
   * {@link reconcileToolTabSelection}). Every route into the Tool category's path lands here — a user
   * press, a `DockUiStateStore` arrangement restored on boot, an introduction step, a program
   * `setActiveTool` effect, a utility claiming the pointer — so the tab and the tool can never disagree,
   * and the very first press on a restored `tool.fill` leaf arms Fill instead of reading as a re-press.
   * Skipped wholesale (leaving `toolTabSelectionRef` untouched) while the Tool category is not the active
   * root, so an armed tool survives browsing the Command palette and is reconciled again on re-entry.
   */
  const toolTabSelectionRef = useRef<ToolTabSelection | null>(null);
  const pendingToolActivateRef = useRef<string | null | undefined>(undefined);
  const revealedToolIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (!session) return;
    const toolAnchor = findPanelTabInDock(dock, FRAMEWORK_CATEGORY_TOOL_ID)?.anchor ?? "bottom-middle";
    const toolCategoryTabs = mobile ? mobilePanelTabs : dock.anchors[toolAnchor];
    const branchPath = findPanelTabPath(toolCategoryTabs, FRAMEWORK_CATEGORY_TOOL_ID);
    if (!branchPath) return;
    const path = mobile ? mobilePanelPath : panelActivePaths[toolAnchor];
    const toolCategoryActive = !branchPath.some((segment, index) => path[index] !== segment);
    if (programArmedToolRevealV1(revealedToolIdRef.current, activeToolId, toolCategoryActive)) {
      revealedToolIdRef.current = activeToolId;
      const value = findPanelTabPath(toolCategoryTabs, `tool.${activeToolId}`) ?? branchPath;
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_PATH", value } : { type: "SET_PANEL_PATH", anchor: toolAnchor, value });
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_VISIBLE", value: true } : { type: "SET_PANEL_VISIBLE", anchor: toolAnchor, value: true });
      return;
    }
    if (!toolCategoryActive) return;
    revealedToolIdRef.current = activeToolId;
    const selectedToolId = toolIdFromPanelTabId(path[path.length - 1]);
    if (pendingToolActivateRef.current !== undefined) {
      const pending = pendingToolActivateRef.current;
      if ((pending ?? "") === (activeToolId ?? "")) {
        pendingToolActivateRef.current = undefined;
      } else if (selectedToolId === pending) {
        return;
      } else {
        pendingToolActivateRef.current = undefined;
      }
    }
    const { next, effect } = reconcileToolTabSelection(toolTabSelectionRef.current, activeToolId, selectedToolId);
    toolTabSelectionRef.current = next;
    if (effect.kind === "activate") {
      pendingToolActivateRef.current = effect.toolId;
      onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: effect.toolId ?? "" } });
      return;
    }
    if (effect.kind === "select") {
      const value = (effect.toolId ? findPanelTabPath(toolCategoryTabs, `tool.${effect.toolId}`) : undefined) ?? branchPath;
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_PATH", value } : { type: "SET_PANEL_PATH", anchor: toolAnchor, value });
    }
  }, [activeToolId, dock, mobile, mobilePanelPath, mobilePanelTabs, onActionStable, panelActivePaths, session]);
  //#endregion 🧭️DockAssembly

  const mobilePanel = useMemo(() => {
    if (mobilePanelTabs.length === 0) return undefined;
    return {
      visible: mobilePanelVisible,
      tabs: mobilePanelTabs,
      activeTabPath: mobilePanelPath,
      onActiveTabPathChange: (path: readonly string[]) => {
        dispatch({ type: "SET_MOBILE_PANEL_PATH", value: path });
        const tabId = path[path.length - 1];
        // 🌱️ Progressive paths often end at a branch (or are empty) — only leaves are meaningful "active panel tab" selections.
        if (tabId && hostMode && session && session.app.id === hostAppId && findPanelTabNode(mobilePanelTabs, path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
      // ♻️ Lazy tool/command trees read measures + active tool from refs — revision forces re-resolve.
      treeContentRevision: toolPanelTreeContentRevision(activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey),
    };
  }, [mobilePanelVisible, mobilePanelPath, mobilePanelTabs, onAction, panelPathMemory, session, hostMode, treeOpenStates, hostAppId, activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey]);

  useEffect(() => {
    if (exampleOptions.length === 0) return;
    dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: (current) => (!current || exampleOptions.some((option) => option.id === current) ? current : "") });
  }, [exampleOptions, session?.app.id, session?.pluginId]);

  // 🎛️ Announces the boot example to the fresh session exactly once per instance. When nothing is
  // locked/defaulted, seed the first registered example so the dropdown matches the plugin default
  // document (e.g. procedural3d hexagonal column) — same rule as wgpu `sync_session_chrome`.
  // Studio-mode routes load documents via `applyShellUri`/`openSpace`; never boot-override those.
  useEffect(() => {
    if (exampleOptions.length === 0 || !session) return;
    if (hostMode) {
      noExampleResetInstanceIdRef.current = session.instanceId;
      return;
    }
    if (noExampleResetInstanceIdRef.current === session.instanceId) return;
    noExampleResetInstanceIdRef.current = session.instanceId;
    const exampleId = resolveBootExampleId(activeExampleId, exampleOptions, defaults.exampleId);
    if (exampleId !== activeExampleId) {
      dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
    }
    dispatchActiveExample(exampleId);
  }, [activeExampleId, defaults.exampleId, dispatchActiveExample, exampleOptions, session, hostMode]);

  //#region 🎛️PanelTabBarHosting — `buildPanelSelectionProps` is the single source of an anchor's tab
  // selection state, shared by the chrome-hosted `PanelChromeTabBar` (below, for anchors in
  // {@link PANEL_TAB_BAR_HOSTS}) and the floating `Panel` itself (`buildPanelProps`) — the two hosts of the
  // SAME anchor always read/write the exact same controlled state.
  const buildPanelSelectionProps = useCallback(
    (anchor: Anchor): PanelTabSelectionOptions => ({
      tabs: dock.anchors[anchor],
      visible: panels[anchor].visible,
      onVisibleChange: (value: boolean) => {
        dispatch({ type: "SET_PANEL_VISIBLE", anchor, value });
        noteShellCommand("shell.panelToggle", shellLabel("ui.shellCommand.panelToggle"), { anchor, visible: value });
      },
      activeTabPath: panelActivePaths[anchor],
      onActiveTabPathChange: (path: readonly string[]) => {
        const previous = panelActivePaths[anchor] ?? [];
        const inactiveRepress = toolLeafInactiveRepress(previous, path, activeToolId ?? null);
        if (inactiveRepress && session) {
          path = inactiveRepress.path;
          onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: inactiveRepress.toolId } });
        }
        const pathChanged = previous.join("/") !== path.join("/");
        dispatch({ type: "SET_PANEL_PATH", anchor, value: path });
        // 🎛️ Command palette only: switching category leaves always collapses any expanded arg form — the
        // next hierarchy level up only makes sense under its own category's command list (mirrors the old
        // dedicated `SET_COMMAND_CATEGORY` reducer case, now expressed at the generic path-change call site
        // since category-active state itself is just this anchor's `activeTabPath`). Categories sit under
        // the Command branch, so compare the category segment (path[1]), not the shared branch root.
        if (anchor === "bottom-middle" && panels[anchor].path[1] !== path[1]) {
          dispatch({ type: "SET_COMMAND_EXPANDED", value: null });
        }
        const tabId = path[path.length - 1];
        // 🛠️ Selecting/deselecting a mode-tool leaf (`tool.<id>`) arms or disarms that tool — owned by the
        // single `reconcileToolTabSelection` pass in 🧭️DockAssembly, never by this press callback, so a
        // restored or programmatically-set path reaches exactly the same state a press does.
        // 🌱️ Progressive paths often end at a branch (or are empty) — only leaves are meaningful "active panel tab" selections.
        if (tabId && hostMode && session && session.app.id === hostAppId && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
        if (pathChanged && tabId) noteShellCommand("shell.panelTab", shellLabel("ui.shellCommand.panelTab"), { anchor, tabId });
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      drillOnOpen: anchor === "bottom-middle" ? (path, memory) => toolCategoryOpenPath(path, memory, toolTabs.map((tab) => tab.id)) : undefined,
    }),
    [activeToolId, dock, onAction, panelActivePaths, panelPathMemory, panels, session, hostMode, hostAppId, noteShellCommand, toolTabs],
  );
  //#endregion 🎛️PanelTabBarHosting

  const navbarItems = useMemo((): NavbarItem[] => {
    if (!session) return [];
    const logoAndTitle = (
      <div key="logoAndTitle" className="flex min-w-0 shrink-0 items-center gap-single">
        {brand?.logoSvg ? <ShellBrandLogo svg={brand.logoSvg} className="size-workbench shrink-0" /> : <SemioLogo className="size-workbench shrink-0" />}
        <span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>
          {appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology))}
        </span>
        {/* 👁️✏️ Window title chip / read-only badge (contract freeze §5) — role read off the resolved
         * `session.app.role`, never parsed out of `session.app.id`. */}
        <span data-slot="surface-role-chip" data-role={session.app.role} title={session.app.role === "viewer" ? viewerReadOnlyNoticeText(uiLocale) : undefined} className="rounded-sm border border-border px-single text-xs text-muted-foreground">
          {surfaceRoleChipText(session.app.role, uiLocale)}
        </span>
      </div>
    );
    const showExampleSelect = exampleOptions.length > 0 && !locks.exampleId && (!hostMode || session.app.id !== landingAppId);
    // 📱️ Mobile has no room for tab bars, example selector, or mode switcher in the navbar — just the
    // logo/title and the single toggle for the merged mobile panel (the two dropped controls resurface as
    // the panel's synthetic "App" tab, see `mobilePanelTabs`).
    if (mobile) {
      return [
        { key: "logoAndTitle", content: logoAndTitle },
        navbarFillItem("navbarTrailingFill"),
        {
          key: "mobilePanelToggle",
          content: <Toggle id="ui.mobilePanel.toggle" pressed={mobilePanelVisible} onPressedChange={(value) => dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value })} icon="panel-left" />,
        },
      ];
    }
    // Logo/title, example selector, and mode switcher render as one cluster, centered as a group in the navbar
    // (via `centered`) rather than left-anchored with fill spacers pushing the rest toward the trailing edge.
    const centerContent: ReactNode[] = [logoAndTitle];
    if (showExampleSelect && exampleSelectElement) centerContent.push(exampleSelectElement);
    if (modeSwitcherElement) centerContent.push(modeSwitcherElement);
    if (roleSwitcherElement) centerContent.push(roleSwitcherElement);
    return [
      { key: "topLeftPanelTabs", content: <PanelChromeTabBar anchor="top-left" {...buildPanelSelectionProps("top-left")} /> },
      navbarFillItem("navbarTrailingFill"),
      { key: "topRightPanelTabs", content: <PanelChromeTabBar anchor="top-right" {...buildPanelSelectionProps("top-right")} /> },
      {
        key: "center",
        centered: true,
        content: (
          <div className="flex min-w-0 items-center gap-double">
            {centerContent}
            <PanelChromeTabBar anchor="top-middle" {...buildPanelSelectionProps("top-middle")} />
          </div>
        ),
      },
    ];
  }, [brand, buildPanelSelectionProps, exampleOptions, exampleSelectElement, locks.exampleId, mobile, mobilePanelVisible, modeSwitcherElement, roleSwitcherElement, session, uiLocale, uiTerminology, hostMode, landingAppId]);

  const searchItems = useMemo(() => {
    if (!session) return [];
    const items: UISearchItem[] = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      const tabId = panelTabKindId(tab.kind);
      items.push({
        id: `panel.${tabId}`,
        label: resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label as LocalizedLabel | string, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.panels"),
        icon: <Icon icon="panel-left" size="small" />,
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } }),
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.windows"),
        icon: <Icon icon="app-window" size="small" />,
        onSelect: () => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: kind.id }),
      });
    }
    // 🎛️ Commands (os/plugin/app/mode) — the footer twin of the window-rail P3 redirect above: an
    // arg-carrying command never fires from the palette, it opens the bottom-middle command panel at its
    // category and expands its form instead.
    for (const { definition, address } of resolvedCommands) {
      if (!definition.inPalette) continue;
      const argCarrying = (definition.args?.length ?? 0) > 0;
      items.push({
        id: `command.${commandAddressKey(address).replaceAll(":", ".")}`,
        // 🩹️ CommandDefinition.label has no owned schema mirror yet (unknown) — resolved the same way every
        // other manifest label in this file is.
        label: (() => { const resolved = resolveManifestLabel(definition.label as LocalizedLabel | string, uiTerminology, uiLocale); return argCarrying ? `${resolved}…` : resolved; })(),
        description: commandKeybindingChords(definition, detectCommandPlatform(typeof navigator !== "undefined" ? `${navigator.platform} ${navigator.userAgent}` : "")).join(",") || undefined,
        category: commandCategoryLabel(definition.category),
        onSelect: () => {
          if (argCarrying) {
            const commandPath = [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${definition.category}`];
            // 📱️ On mobile every anchor's tabs are merged into the single mobile panel — route the same
            // path there instead of the (unrendered) bottom-middle anchor, and open the mobile panel itself.
            if (mobile) {
              dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
              dispatch({ type: "SET_MOBILE_PANEL_PATH", value: commandPath });
            } else {
              dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
              dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: commandPath });
            }
            dispatch({ type: "SET_COMMAND_EXPANDED", value: commandAddressKey(address) });
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onCommand(address);
        },
      });
    }
    if (hostMode && panel) {
      for (const program of spacePrograms) {
        items.push({
          id: `spawn.${program.pluginId}`,
          label: `${shellLabel("ui.palette.spawnPrefix")} ${appBreadcrumb(resolveArtifactByAppId(loadedPlugins, program.appId, program.breadcrumb, uiTerminology))}`,
          category: shellLabel("ui.search.category.catalogue"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "spawnApp", args: { pluginId: program.pluginId } }),
        });
      }
      items.push(
        {
          id: "studio.undo",
          label: shellLabel("ui.palette.undo"),
          category: shellLabel("ui.search.category.hostApp"),
          icon: <Icon icon="undo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "undo" }),
        },
        {
          id: "studio.redo",
          label: shellLabel("ui.palette.redo"),
          category: shellLabel("ui.search.category.hostApp"),
          icon: <Icon icon="redo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "redo" }),
        },
        {
          id: "studio.home",
          label: shellLabel("ui.palette.goHome"),
          category: shellLabel("ui.search.category.navigation"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "goHome" }),
        },
      );
    }
    return items;
  }, [activeWindowId, appLabelsOverlay, loadedPlugins, mobile, onAction, onCommand, panel, resolvedCommands, session, spacePrograms, hostMode, uiLocale, uiTerminology, hostControllerId]);

  const modeWindows = useMemo((): ModeWindowDescriptor[] => {
    if (!session) return [];
    const actionPaneSlice: ActionPaneSlice = { expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId };
    const actionsFoldedFor = (windowId: string, windowKindId: string = windowId) =>
      introductionTargetsWindow(windowId, windowKindId, null, introductionActionWindowSegment) ? false : (actionPaneFoldedByWindowId[windowId] ?? true);
    // 🎓️ `undefined` keeps the Window's own internal fold state — only windows of the introduction's
    // target kind (including every open instance) are force-controlled to `false` while its utility step
    // is active.
    const utilityBarFoldedFor = (windowId: string, windowKindId: string = windowId): boolean | undefined =>
      introductionTargetsWindow(windowId, windowKindId, introductionUtilityWindowId) ? false : undefined;
    const measuresFoldedFor = (windowId: string, windowKindId: string = windowId): boolean | undefined =>
      introductionTargetsWindow(windowId, windowKindId, introductionMeasureWindowId) ? false : undefined;
    const onActionsFoldedFor = (windowId: string) => (folded: boolean) => dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: folded });
    // 🖱️ Window-body cursor follows the active utility's declared `cursor` (P5).
    const cursorFor = (app: AppDefinition, windowId: string): CSSProperties | undefined => {
      const utilityId = activeUtilityByWindowId[windowId];
      const cursor = utilityId ? (app.utilities ?? []).find((utility) => utility.id === utilityId)?.cursor : undefined;
      return cursor ? { cursor } : undefined;
    };
    if (hostMode && (spawnedWindowUi || spawnedWindowFault) && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const spawnedApp = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        const windowKind = spawnedApp?.windowKinds[0];
        const chrome = windowKind ? spawnedWindowChromeForKind(windowKind, spawned.id, spawnedWindowEngagements, spawnedWindowMeasures, activeUtilityByWindowId[spawned.id] ?? undefined, onActionStable) : undefined;
        const spawnedUtilities = spawnedApp && windowKind ? resolveUtilityNodes(spawnedApp, windowKind, activeUtilityByWindowId[spawned.id], spawned.id, appLabelsOverlay, uiTerminology, uiLocale) : [];
        return [
          {
            id: spawned.id,
            iconId: windowIconsById[spawned.id] ?? windowKind?.iconId ?? "app-window",
            title: wireLabel(appBreadcrumb(spawnedApp ? resolveAppBreadcrumb(spawnedApp, uiTerminology) : spawned.breadcrumb)),
            fill: true,
            showControls: true,
            measures: chrome?.measures,
            measuresFolded: measuresFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
            engagement: chrome?.engagement,
            search: chrome?.search,
            utilityBar: spawnedApp && windowKind ? utilityBarNode(spawnedUtilities, spawned.id, onActionStable, introductionUtilityId, chrome?.utilityOptions) : undefined,
            utilityBarFolded: utilityBarFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
            actionPane: spawnedApp && windowKind ? windowActionPaneNode(spawnedApp, windowKind, spawned.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale, loadedPlugins.map((entry) => entry.manifest), selectedSpaceArtifactKinds) : undefined,
            actionsFolded: actionsFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
            onActionsFoldedChange: onActionsFoldedFor(spawned.id),
            children: (
              <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={spawnedApp ? cursorFor(spawnedApp, spawned.id) : undefined}>
                <ShellFaultBoundary boundaryId={`window-${spawned.id}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                  {spawnedWindowUi ? <InterpretedUiNode store={builtNodeStoreFor("spawned", spawnedWindowUi)} onAction={onActionStable} onIntent={onIntentStable} /> : <WindowFaultStatus fault={spawnedWindowFault!} />}
                </ShellFaultBoundary>
              </ChromeAwareWindowScrollSurface>
            ),
          },
        ];
      }
    }
    const baseWindows = session.app.windowKinds.map((kind) => {
      const browserActorStore = currentBrowserActorUi?.sessionInstanceId === session.instanceId && currentBrowserActorUi.windowKindId === kind.id ? currentBrowserActorUi.store : undefined;
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[kind.id], kind.id, appLabelsOverlay, uiTerminology, uiLocale);
      const chrome = windowMeasuresChrome(windowMeasuresByWindowId[kind.id] ?? kind.options.measures, activeUtilityByWindowId[kind.id] ?? undefined, kind.id, onActionStable);
      const resolvedEngagement = resolveWindowEngagement(kind, kind.id, windowEngagementsByWindowId);
      return {
        id: kind.id,
        iconId: windowIconsById[kind.id] ?? kind.iconId,
        title: wireLabel(windowTitlesById[kind.id] ?? appWindowLabel(session.app, uiTerminology, resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)), uiLocale)),
        fill: true,
        showControls: true,
        measures: chrome.measures,
        measuresFolded: measuresFoldedFor(kind.id, kind.id),
        engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
        search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
        utilityBar: utilityBarNode(utilities, kind.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
        utilityBarFolded: utilityBarFoldedFor(kind.id, kind.id),
        actionPane: windowActionPaneNode(session.app, kind, kind.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale, loadedPlugins.map((entry) => entry.manifest), selectedSpaceArtifactKinds),
        actionsFolded: actionsFoldedFor(kind.id, kind.id),
        onActionsFoldedChange: onActionsFoldedFor(kind.id),
        status: browserActorStore?.getState().root === null ? windowUiByWindowId[kind.id]?.activity : browserActorStore?.getState().nodes.get(browserActorStore.getState().root!)?.activity ?? windowUiByWindowId[kind.id]?.activity,
        skeleton: <WindowBodySkeleton />,
        children: (
          <ChromeAwareWindowScrollSurface id={childElementId("framework.window", kind.id)} className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={cursorFor(session.app, kind.id)}>
            <WindowInstanceIdContext.Provider value={kind.id}>
              <ShellFaultBoundary boundaryId={`window-${kind.id}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                {instanceFault ? <WindowFaultStatus fault={instanceFault} /> : null}
                <InterpretedUiNode store={browserActorStore ?? builtNodeStoreFor(`window:${kind.id}`, windowUiByWindowId[kind.id] ?? PENDING_WINDOW_UI_NODE)} onAction={browserActorStore === undefined ? onActionStable : refuseBrowserActorActionDescriptor} onIntent={browserActorStore === undefined ? onIntentStable : (intent) => { if (currentDocumentRuntimeKey !== null && currentBrowserActorUi !== undefined) onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, intent); }} />
              </ShellFaultBoundary>
            </WindowInstanceIdContext.Provider>
          </ChromeAwareWindowScrollSurface>
        ),
      };
    });
    // 🪟️ Each extra (split/spawned) instance renders its OWN `windowUiByWindowId[instance.id]` body,
    // measures, and engagement — never the base kind's shared entry — so two instances of the same kind
    // (e.g. split top/perspective panes) never show or affect each other's options. `data-element-alias`
    // aliases the instance to its window kind's element id so an introduction `show` target of the kind
    // (not a specific instance) raises every open instance above the glass, not only the base one.
    const extraWindows = extraWindowInstances.flatMap((instance) => {
      const kind = session.app.windowKinds.find((entry) => entry.id === instance.windowKindId);
      if (!kind) return [];
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[instance.id], instance.id, appLabelsOverlay, uiTerminology, uiLocale);
      const chrome = windowMeasuresChrome(windowMeasuresByWindowId[instance.id] ?? kind.options.measures, activeUtilityByWindowId[instance.id] ?? undefined, instance.id, onActionStable);
      const resolvedEngagement = resolveWindowEngagement(kind, instance.id, windowEngagementsByWindowId);
      return [
        {
          id: instance.id,
          iconId: windowIconsById[instance.id] ?? kind.iconId,
          title: wireLabel(windowTitlesById[instance.id] ?? instance.title),
          fill: true,
          showControls: true,
          measures: chrome.measures,
          measuresFolded: measuresFoldedFor(instance.id, instance.windowKindId),
          engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
          search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
          utilityBar: utilityBarNode(utilities, instance.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
          utilityBarFolded: utilityBarFoldedFor(instance.id, instance.windowKindId),
          actionPane: windowActionPaneNode(session.app, kind, instance.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale, loadedPlugins.map((entry) => entry.manifest), selectedSpaceArtifactKinds),
          actionsFolded: actionsFoldedFor(instance.id, instance.windowKindId),
          onActionsFoldedChange: onActionsFoldedFor(instance.id),
          status: windowUiByWindowId[instance.id]?.activity,
          skeleton: <WindowBodySkeleton />,
          children: (
            <ChromeAwareWindowScrollSurface
              id={childElementId("framework.window", instance.id)}
              data-element-alias={childElementId("framework.window", kind.id)}
              className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
              style={cursorFor(session.app, instance.id)}
            >
              <WindowInstanceIdContext.Provider value={instance.id}>
                <ShellFaultBoundary boundaryId={`window-${instance.id}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                  {instanceFault ? <WindowFaultStatus fault={instanceFault} /> : null}
                  <InterpretedUiNode store={builtNodeStoreFor(`window:${instance.id}`, windowUiByWindowId[instance.id] ?? PENDING_WINDOW_UI_NODE)} onAction={onActionStable} onIntent={onIntentStable} />
                </ShellFaultBoundary>
              </WindowInstanceIdContext.Provider>
            </ChromeAwareWindowScrollSurface>
          ),
        },
      ];
    });
    return [...baseWindows, ...extraWindows];
  }, [
    actionPaneExpandedByWindowId,
    actionPaneFoldedByWindowId,
    actionPaneStagedArgsByKey,
    activeUtilityByWindowId,
    appLabelsOverlay,
    currentBrowserActorUi,
    currentDocumentRuntimeKey,
    extraWindowInstances,
    introductionActionWindowSegment,
    introductionUtilityId,
    introductionUtilityWindowId,
    loadedPlugins,
    onActionStable,
    onBrowserActorIntent,
    onIntentStable,
    refuseBrowserActorActionDescriptor,
    panel,
    session,
    spawnedWindowEngagements,
    spawnedWindowMeasures,
    spawnedWindowUi,
    spawnedWindowFault,
    instanceFault,
    hostMode,
    uiLocale,
    uiTerminology,
    windowEngagementsByWindowId,
    windowMeasuresByWindowId,
    windowTitlesById,
    windowIconsById,
    windowUiByWindowId,
  ]);

  const effectiveModeLayout = useMemo(
    () =>
      shellLayout ??
      (session ? resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout : { kind: "stack" as const, children: [] }),
    [appLabelsOverlay, session, shellLayout, uiTerminology, uiLocale],
  );

  const handleActiveWindowChange = useCallback(
    (value: string | null) => {
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value });
      if (value) noteShellCommand("shell.windowActivate", shellLabel("ui.shellCommand.windowActivate"), { windowId: value });
    },
    [noteShellCommand],
  );

  // 🪟️ `Mode.onLayoutChange` fires continuously during a live drag/resize (one call per frame) — classify
  // each delta against the last-seen layout, remember only the latest non-null classification, and note a
  // single shell command once the drag settles (see `LAYOUT_CHANGE_SETTLE_MS`). A pure active-window-flag
  // echo classifies `null` and is silently skipped here (handled by `handleActiveWindowChange` instead).
  const layoutChangeSettleTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const layoutChangeClassificationRef = useRef<"resize" | "rearrange" | null>(null);
  const layoutChangePreviousRef = useRef<WindowLayoutNode | null>(effectiveModeLayout);
  useEffect(() => {
    layoutChangePreviousRef.current = effectiveModeLayout;
  }, [effectiveModeLayout]);
  useEffect(
    () => () => {
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
    },
    [],
  );
  const handleModeLayoutChange = useCallback(
    (value: WindowLayoutNode) => {
      dispatch({ type: "SET_SHELL_LAYOUT", value });
      const classification = classifyWindowLayoutChange(layoutChangePreviousRef.current, value);
      layoutChangePreviousRef.current = value;
      if (classification) layoutChangeClassificationRef.current = classification;
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
      layoutChangeSettleTimeoutRef.current = setTimeout(() => {
        layoutChangeSettleTimeoutRef.current = null;
        const finalClassification = layoutChangeClassificationRef.current;
        layoutChangeClassificationRef.current = null;
        if (finalClassification === "resize") noteShellCommand("shell.windowResize", shellLabel("ui.shellCommand.windowResize"));
        else if (finalClassification === "rearrange") noteShellCommand("shell.windowMove", shellLabel("ui.shellCommand.windowMove"));
      }, LAYOUT_CHANGE_SETTLE_MS);
    },
    [noteShellCommand],
  );

  const canvas = useMemo(() => {
    if (hostMode && shellRoute.kind === "notFound") {
      return <ShellRouteNotFoundPage path={shellRoute.path} onHome={() => navigateHistory("/")} />;
    }
    const supervisorPluginId = primaryPluginId;
    const supervisorState = supervisorPluginId ? pluginSupervisorById[supervisorPluginId] : undefined;
    if (supervisorState === "crashed" || supervisorState === "quarantined") {
      return (
        <PluginRecoveryPanel
          pluginId={supervisorPluginId!}
          quarantined={supervisorState === "quarantined"}
          onRestart={() => {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: supervisorPluginId!, value: "restarting" });
            void reloadPlugin(supervisorPluginId!);
          }}
          onDisable={() => {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: supervisorPluginId!, value: "quarantined" });
            if (supervisorPluginId !== primaryPluginId) void uninstallPlugin(supervisorPluginId!);
          }}
        />
      );
    }
    if (error)
      return (
        <p className="p-double text-sm text-destructive" role="alert" data-semio-os-shell-error="" data-semio-window-fault={sessionFault?.class ?? "unknown"} data-semio-window-fault-code={sessionFault?.code ?? ""}>
          {uiDataLabel(error)}
        </p>
      );
    if (!session) return <CanvasSkeleton label={shellLabel("ui.common.loadingPlugins")} className={cn(loadingBorderClass, "h-full w-full")} />;
    const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology)) }];
    const studioHomeBar =
      hostMode && session.app.id === hostAppId && !panel?.activeSpawnedId ? (
        <button
          type="button"
          className={cn(borderNormalBottomClass, "px-single py-single text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground")}
          onClick={() => onAction({ controllerId: session.app.controllerId, action: "goHome" })}
        >
          ← {shellLabel("ui.common.home")}
        </button>
      ) : null;
    const focusedSpawned = panel?.activeSpawnedId ? panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId) : undefined;
    const focusedBar = focusedSpawned ? (
      <div className={cn(borderNormalBottomClass, "flex items-center gap-single px-single py-single text-sm text-muted-foreground")}>
        <button type="button" className="hover:text-foreground" onClick={() => (openSpaceIdRef.current ? navigateHistory(`/spaces/${openSpaceIdRef.current}`) : onAction({ controllerId: session.app.controllerId, action: "closeFocusedInstance" }))}>
          ← {shellLabel("ui.common.backToWorkflow")}
        </button>
        <span>·</span>
        <span>{appBreadcrumb(resolveArtifactByAppId(loadedPlugins, focusedSpawned.appId, focusedSpawned.breadcrumb, uiTerminology))}</span>
      </div>
    ) : null;
    return (
      <div className="flex h-full min-h-0 flex-col overflow-hidden">
        {studioHomeBar}
        {focusedBar}
        <input
          ref={importSpaceInputRef}
          type="file"
          // 📦️ `.pack` files branch to `s/plugin`'s pack-aware `importSpacePackPayload` action
          // (`semio_framework_os::import_os_space_from_pack`, wave 2 s+shome+sstudio family) —
          // read as a dataUrl, same shape as the generic `RequestFileOpen`/`readAs: "dataUrl"` path
          // below. Anything else keeps reading as text and dispatching the JSON-envelope "importSpace".
          accept=".spk,.dsl,.ops,application/octet-stream"
          className="hidden"
          onChange={(event) => {
            const file = event.target.files?.[0];
            if (!file) return;
            if (file.name.toLowerCase().endsWith(".pack")) {
              const reader = new FileReader();
              reader.onload = () => {
                const payload = typeof reader.result === "string" ? reader.result : "";
                onAction({ controllerId: landingControllerId ?? "", action: "importSpacePackPayload", args: { payload } });
                event.target.value = "";
              };
              reader.readAsDataURL(file);
              return;
            }
            void file.text().then((json) => {
              onAction({ controllerId: landingControllerId ?? "", action: "importSpace", args: { json } });
              event.target.value = "";
            });
          }}
        />
        <div className="min-h-0 flex-1">
          <ShellFaultBoundary boundaryId="session-canvas" fallbackLabel={shellLabel("ui.common.renderError")}>
            <App
            modes={modes.map((mode) => ({ id: mode.id, label: wireLabel(resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label as LocalizedLabel | string, uiTerminology, uiLocale))), children: null }))}
            activeModeId={session.viewState.activeModeId ?? modes[0]?.id ?? session.app.id}
            onActiveModeChange={applyModeChange}
            chrome={false}
          >
            <Mode
              className="h-full w-full"
              mobile={mobile}
              windows={modeWindows}
              layout={effectiveModeLayout}
              activeWindowId={activeWindowId}
              onActiveWindowChange={handleActiveWindowChange}
              onLayoutChange={handleModeLayoutChange}
              onTemplateDrop={mobile ? undefined : handleTemplateDrop}
              onWindowClose={(windowId) => {
                noteShellCommand("shell.windowClose", shellLabel("ui.shellCommand.windowClose"), { windowId });
                if (hostMode && panel?.spawnedApps.some((entry) => entry.id === windowId)) {
                  const closedSpawned = panel.spawnedApps.find((entry) => entry.id === windowId);
                  const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== windowId);
                  updateSpacePanel(buildSpacePanelState(nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
                  // 🪶️ Closing a spawned app's window used to leave its plugin instance running forever
                  // (see REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT's documented teardown gap) — the panel
                  // entry was dropped from the UI, but nothing ever told the guest to free it.
                  if (closedSpawned) {
                    const closedPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === closedSpawned.pluginId)?.handle;
                    void closedPlugin?.destroyApp(closedSpawned.instanceId).catch(() => {});
                  }
                }
                clearPendingWorldProjection(windowId);
                dispatch({
                  type: "SET_EXTRA_WINDOW_INSTANCES",
                  value: (current) => {
                    const next = current.filter((entry) => entry.id !== windowId);
                    extraWindowInstancesRef.current = next;
                    return next;
                  },
                });
                dispatch({
                  type: "SET_SHELL_LAYOUT",
                  value: (current) => current ?? resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout,
                });
              }}
              onWindowOpenInNewWindow={(windowId) => {
                if (!session) return;
                const extra = extraWindowInstancesRef.current.find((entry) => entry.id === windowId);
                const windowKindId = extra?.windowKindId ?? session.app.windowKinds.find((kind) => kind.id === windowId)?.id;
                if (!windowKindId) return;
                const kind = session.app.windowKinds.find((entry) => entry.id === windowKindId);
                if (!kind) return;
                extraWindowCounterRef.current += 1;
                const instanceId = `${windowKindId}-${extraWindowCounterRef.current}`;
                const title = resolveAppLabel(
                  appLabelsOverlay,
                  "windowKind",
                  kind.id,
                  resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale),
                );
                const nextExtraInstances = [...extraWindowInstancesRef.current, { id: instanceId, windowKindId, title }];
                extraWindowInstancesRef.current = nextExtraInstances;
                dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: nextExtraInstances });
                void refreshUi(session, { kind: "full" }, nextExtraInstances);
                dispatch({
                  type: "SET_SHELL_LAYOUT",
                  value: (current) => {
                    const base =
                      current ??
                      resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout;
                    const stackPath = resolveStackPathForWindowId(base, windowId);
                    if (stackPath === null) {
                      return insertWindowAtDropZone(base, instanceId, { kind: "root-split", side: "right" });
                    }
                    return splitWithWindow(base, stackPath, instanceId, "right");
                  },
                });
                dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
                noteShellCommand("shell.windowOpenInNewWindow", shellLabel("ui.shellCommand.windowOpenInNewWindow"), {
                  windowId,
                  windowKindId,
                  instanceId,
                });
              }}
            />
          </App>
          </ShellFaultBoundary>
        </div>
      </div>
    );
  }, [activeWindowId, appLabelsOverlay, effectiveModeLayout, error, sessionFault, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, loadedPlugins, mobile, modeWindows, navigateHistory, noteShellCommand, onAction, panel, pluginSupervisorById, primaryPluginId, refreshUi, reloadPlugin, session, shellRoute, hostMode, uiLocale, uiTerminology, updateSpacePanel, dispatch, uninstallPlugin]);

  const footerItems = useMemo((): NavbarItem[] => {
    // 🏛️ Mit Bestand Aggregator partner credits: left "Ein Projekt von LUH und UdK", right "Gefördert durch Zukunft Bau".
    // A single middle flex-1 fill pushes the funding credit to the trailing edge; fixed `w-huge` gaps keep each credit
    // off the exact corner pixel that floating corner panels also anchor to (a second flex-1 would center the funding
    // credit under the Command overlay; `w-double` reads as flush against the toggle group).
    // 📱️ The three tab bars have no anchor on mobile (all anchors merge into the mobile panel) — only the credits stay.
    const items: NavbarItem[] = mobile
      ? []
      : [
          { key: "bottomLeftPanelTabs", content: <PanelChromeTabBar anchor="bottom-left" {...buildPanelSelectionProps("bottom-left")} /> },
          { key: "bottomMiddlePanelTabs", centered: true, content: <PanelChromeTabBar anchor="bottom-middle" {...buildPanelSelectionProps("bottom-middle")} /> },
        ];
    if (brand?.id && (ENTWERFEN_MIT_BESTAND_BRAND_IDS as readonly string[]).includes(brand.id)) {
      items.push(
        { key: "footerProjectOfGap", className: "w-huge", content: null },
        aProjectOfLuhUdkFooterItem("aProjectOfLuhUdk", uiLocale, mobile),
        navbarFillItem("footerLeadingFill"),
        fundedByZukunftBauFooterItem("fundedByZukunftBau", uiLocale, mobile),
        { key: "footerFundedByGap", className: "w-huge", content: null },
      );
    } else {
      items.push(navbarFillItem("footerLeadingFill"));
    }
    // 👥️ ticket §C0/§5 lane 4-F — `#s-presence-peers`, right-aligned in the footer, mirroring the wgpu
    // shell's own `render_presence_bar` placement (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) rather than hiding behind a
    // panel tab click: presence is ambient chrome, always visible while a document is open.
    if (!mobile) items.push({ key: "presenceBar", content: <PresenceBar id="s-presence-peers" peers={presencePeers} /> });
    if (!mobile) items.push({ key: "bottomRightPanelTabs", content: <PanelChromeTabBar anchor="bottom-right" {...buildPanelSelectionProps("bottom-right")} /> });
    return items;
  }, [brand?.id, buildPanelSelectionProps, mobile, presencePeers, uiLocale]);

  const buildPanelProps = useCallback(
    (anchor: Anchor) => ({
      ...buildPanelSelectionProps(anchor),
      size: panels[anchor].size,
      onSizeChange: (value: number) => dispatch({ type: "SET_PANEL_SIZE", anchor, value }),
      tabBarHost: (PANEL_TAB_BAR_HOSTS[anchor] ? "chrome" : "panel") as "panel" | "chrome",
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
      treeContentRevision: toolPanelTreeContentRevision(activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey),
    }),
    [actionPaneStagedArgsByKey, activeToolId, buildPanelSelectionProps, panels, toolMeasuresByToolId, treeOpenStates],
  );

  // #region 🔖️ReadinessBeacon
  /** 🚦️ Deterministic DOM beacon for headless smoke tests (e.g. Storybook's OS-shell plugin-boot matrix)
   * to wait on instead of screenshots/timeouts — set once a session resolves or errors, cleared on unmount.
   * Mirrored onto this shell's own `[data-shell-id]` scope root (`data-shell-ready`/`data-shell-error`/
   * `data-shell-not-found`) because the `document.documentElement` slot above is a single global value and
   * cannot distinguish several `FrameworkOsShell` instances mounted on one page (e.g. the demonstrator). */
  useEffect(() => {
    const root = document.documentElement;
    const shellRoot = scope.rootRef.current;
    const beaconId = pluginFilter ?? "unknown";
    const notFound = hostMode && shellRoute.kind === "notFound";
    if (notFound) {
      root.dataset.semioOsNotFound = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      if (shellRoot) {
        shellRoot.dataset.shellNotFound = beaconId;
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellError;
      }
    } else if (error) {
      root.dataset.semioOsError = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        shellRoot.dataset.shellError = beaconId;
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellNotFound;
      }
    } else if (session) {
      root.dataset.semioOsReady = beaconId;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        shellRoot.dataset.shellReady = beaconId;
        delete shellRoot.dataset.shellError;
        delete shellRoot.dataset.shellNotFound;
      }
    }
    return () => {
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellError;
        delete shellRoot.dataset.shellNotFound;
      }
    };
  }, [session, error, pluginFilter, shellRoute.kind, hostMode, scope.rootRef]);
  // #endregion 🔖️ReadinessBeacon

  // #region 🔖️CatalogSmokeProbe
  /** 🔬️ Dev-only mirror of the two facts a catalog-wide smoke cannot otherwise read from the DOM: which
   * programs this session can spawn, and which installed plugins ended up `failed`/`crashed` (the shell
   * only console-logs a non-primary install failure — see the streaming install effect). Shape is
   * {@link ShellCatalogProbe}; consumed by
   * `framework-os-dev verify catalog`. Never defined in a production build. */
  useEffect(() => {
    let dev = false;
    try {
      dev = Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      dev = false;
    }
    if (!dev || typeof window === "undefined") return;
    const host = window as unknown as { __semioOsCatalogProbe?: ShellCatalogProbe };
    host.__semioOsCatalogProbe = {
      shellPluginId: pluginFilter ?? "unknown",
      ready: !!session && !error,
      plugins: shellCatalogProbePlugins(registry, pluginStatusById, appRouter),
      programs: spacePrograms.map((program) => ({ pluginId: program.pluginId, appId: program.appId, label: program.label })),
      spawned: (panel?.spawnedApps ?? []).map((entry) => ({ id: entry.id, pluginId: entry.pluginId, appId: entry.appId })),
    };
    return () => {
      delete host.__semioOsCatalogProbe;
    };
  }, [session, error, pluginFilter, registry, pluginStatusById, panel, spacePrograms, appRouter]);

  /** 🔬️ Dev/test-only read seam over the actual acknowledged Shell store. The caller supplies only
   * a scope; the returned closed projection contains no capability or mutation surface. */
  useEffect(() => {
    let dev = false;
    try {
      dev = Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      dev = false;
    }
    if (!dev || typeof window === "undefined") return;
    const read = (spaceId: string, documentId: string): MountedGisMapProbeV1 | null => {
      if (typeof spaceId !== "string" || typeof documentId !== "string" || spaceId.length === 0 || documentId.length === 0) return null;
      const retained = browserActorUiByRuntimeKeyRef.current.get(documentRuntimeKeyV1({ kind: "hub", spaceId, documentId }));
      if (retained?.identity === null || retained?.identity === undefined) return null;
      return mountedGisMapProbeV1({ ...retained.identity, sessionInstanceId: retained.sessionInstanceId, windowKindId: retained.windowKindId, store: retained.store });
    };
    const host = window as unknown as { __semioMountedGisMapProbe?: typeof read };
    host.__semioMountedGisMapProbe = read;
    return () => {
      if (host.__semioMountedGisMapProbe === read) delete host.__semioMountedGisMapProbe;
    };
  }, []);

  /** 🧯️ One console record per plugin the router excluded — permanent, not a `[DEBUG]` trace: an
   * excluded plugin installs cleanly, so this is the only signal outside the dev probe that its
   * surfaces are unroutable. */
  useEffect(() => {
    for (const fault of routerFaultByPluginId.values()) console.error(`AppRouter excluded plugin ${JSON.stringify(fault.scope.pluginId ?? "")}: ${fault.code}: ${fault.message}`);
  }, [routerFaultByPluginId]);
  // #endregion 🔖️CatalogSmokeProbe

  //#region 🖱️ShellContextMenu
  /** 🖱️ Dispatch sink for the shell fallback menu's `ContextMenuItemSpec`s (see
   * `buildShellContextMenuItems`) — intercepts the two reserved ids the builder emits in place of a
   * real dispatch (`"shell.openActionPane"`/`"shell.openPalette"`) and forwards everything else to
   * `onAction`, mirroring the command palette's own arg-carrying redirect. */
  const dispatchShellMenuAction = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (!session) return;
      if (action === "shell.openActionPane") {
        const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
        const actionId = typeof args?.actionId === "string" ? args.actionId : undefined;
        if (!windowKind || !actionId) return;
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowKind.id });
        dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId: windowKind.id, value: false });
        dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId: windowKind.id, value: actionId });
        return;
      }
      if (action === "shell.openPalette") {
        dispatch({ type: "SET_SEARCH_OPEN", value: true });
        return;
      }
      // 👁️✏️ Artifact/document context-menu "Open with…" (contract freeze §5) — same local-only
      // navigation the palette's `open-artifact-with-viewer`/`open-artifact-with-editor` commands do,
      // not scoped to either role here since the menu row is one generic "Open with…" entry.
      if (action === "shell.openArtifactWith") {
        dispatch({ type: "SET_OPEN_WITH_FOCUS_ROLE", value: null });
        dispatch({ type: "SET_PANEL_PATH", anchor: "top-left", value: [FRAMEWORK_PANEL_TAB_ARTIFACT_ID] });
        dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
        return;
      }
      onAction({ controllerId: session.app.controllerId, action });
    },
    [session, activeWindowId, onAction, dispatch],
  );

  /** 🖱️ Builds the shell-level fallback menu: the active window's declared actions (undo/redo, view
   * actions, ...) plus a command-palette opener — shown for any right-click no inner surface claimed
   * (window background, empty panel/navbar/footer space, an app with no scene at all). Arg-carrying
   * actions route through the reserved `"shell.openActionPane"` id (parity with the wgpu shell's
   * `build_shell_context_menu_specs`), the whole spec list runs through `organizeContextMenu`, then
   * `mapContextMenuSpecs` binds it to `dispatchShellMenuAction`. */
  const buildShellContextMenuItems = useCallback((): ContextMenuItem[] => {
    if (!session) return [];
    const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
    const specs: ContextMenuItemSpec[] = [];
    const categoryByActionId = new Map<string, string>();
    if (windowKind) {
      // 👁️✏️ Hides every `Mutation`-kind window action from a viewer session's fallback menu
      // (contract freeze §5) — same predicate `resolvedCommands` filters the palette with.
      for (const action of filterDefinitionsForRole(resolveWindowActions(session.app, windowKind), session.app.role)) {
        // 🧹️ Same curation as the command palette (`if (!action.inPalette) continue`) — most apps
        // declare internal/pointer-tracking view actions (worldHover, engagementInput, ...) as window
        // actions purely for dispatch plumbing; only palette-worthy ones belong in a user-facing menu.
        if (!action.inPalette) continue;
        const argCarrying = actionRequiresStagedForm(action);
        categoryByActionId.set(action.id, actionCategoryId(action));
        specs.push({
          id: `shell-menu.action.${action.id}`,
          label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label as LocalizedLabel | string, uiTerminology, uiLocale)) + (argCarrying ? "…" : ""),
          icon: action.iconId,
          shortcut: action.keys ?? keysByActionId.get(action.id),
          destructive: action.kind === "mutation" && action.id.toLowerCase().includes("delete"),
          action: argCarrying ? "shell.openActionPane" : action.id,
          args: argCarrying ? { actionId: action.id } : undefined,
        });
      }
    }
    if (specs.length > 0) specs.push({ id: "shell-menu.separator", separator: true });
    // 👁️✏️ Artifact/document context-menu "Open with…" (contract freeze §5) — only when the current
    // dialect actually has at least one registered surface to list.
    if (hasOpenArtifactSurfaces) {
      specs.push({ id: "shell.openArtifactWith", label: openArtifactWithText(uiLocale), icon: "app-window", action: "shell.openArtifactWith" });
    }
    specs.push({
      id: "shell.openPalette",
      label: shellLabel("ui.search.toggle"),
      icon: "search",
      action: "shell.openPalette",
    });
    const organized = organizeContextMenu(specs, (id) => categoryByActionId.get(id));
    return mapContextMenuSpecs(organized, dispatchShellMenuAction, keysByActionId);
  }, [session, activeWindowId, appLabelsOverlay, keysByActionId, dispatchShellMenuAction, uiTerminology, uiLocale, hasOpenArtifactSurfaces]);

  useEffect(() => {
    const handleContextMenu = (event: globalThis.MouseEvent) => {
      if (isContextMenuPointerTarget(event.target)) return;
      // 🖱️ `preventDefault()` IS how an inner surface claims a right-click (every `ComponentSceneHost`'s
      // own `onContextMenu` calls it synchronously before awaiting its plugin menu), and this fallback is
      // defined as "shown for any right-click no inner surface claimed" — without this guard the shell
      // menu opened on TOP of the plugin's own viewport menu, which is what a probe reading
      // `[role="menuitem"]` saw instead of the puzzle3d rows
      // (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 8).
      if (event.defaultPrevented) return;
      const items = buildShellContextMenuItems();
      if (items.length === 0) return;
      event.preventDefault();
      setShellContextMenu({ x: event.clientX, y: event.clientY, items });
    };
    window.addEventListener("contextmenu", handleContextMenu);
    return () => window.removeEventListener("contextmenu", handleContextMenu);
  }, [buildShellContextMenuItems]);
  //#endregion 🖱️ShellContextMenu

  return (
    <BoardSessionFactoryContext.Provider value={boardSessionFactory}>
    <SetWindowTitleContext.Provider value={setWindowTitle}>
    <SetWindowIconContext.Provider value={setWindowIcon}>
    <AppKeybindingsContext.Provider value={keysByActionId}>
    <AppCatalogueContext.Provider value={appCatalogue}>
    <UiKeybindingsProvider bindings={controlKeybindings}>
    <PluginSurfaceActionsContext.Provider value={requestContextMenu}>
    <ShellContextMenuFallbackContext.Provider value={buildShellContextMenuItems}>
    <ShellFaultBoundary boundaryId="shell-root" fallbackLabel={shellLabel("ui.common.renderError")}>
    <UIFindProvider>
      <LevelProvider level="base">
        <div className="flex h-screen min-h-0 w-screen flex-col bg-transparent" data-level="base" data-semio-os-ready={session && !error ? "" : undefined}>
          {hubEnv && verifiedSessionAuthority === null ? <SessionAuthorityNotice state={identityOffline ? "unavailable" : "pending"} locale={uiLocale} onCancel={cancelSessionAuthorityBootstrap} /> : null}
          {Object.values(bootstrapUiByDocument).length > 0 ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 flex -translate-x-1/2 flex-col gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              {Object.entries(bootstrapUiByDocument).map(([runtimeKey, status]) => (
                <BootstrapStatusNotice key={runtimeKey} status={status} locale={uiLocale} onCancel={() => closeDocument(runtimeKey)} />
              ))}
            </div>
          ) : null}
          {Object.values(executionTargetUiByDocument).length > 0 ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 mt-double flex -translate-x-1/2 flex-col gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              {Object.entries(executionTargetUiByDocument).map(([runtimeKey, status]) => (
                <ExecutionTargetStatusNotice key={runtimeKey} status={status} locale={uiLocale} />
              ))}
            </div>
          ) : null}
          {Object.values(spaceArtifactCreationUi).length > 0 || spaceArtifactCreationCatalogUi !== null ? (
            <div className="pointer-events-auto absolute top-workbench left-double z-50 flex max-w-[28rem] flex-col gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              {spaceArtifactCreationCatalogUi === null ? null : <ArtifactCreationCatalogNotice status={spaceArtifactCreationCatalogUi} locale={uiLocale} />}
              {Object.values(spaceArtifactCreationUi).map((status) => (
                <ArtifactCreationProgressNotice key={status.requestId} state={status} locale={uiLocale} onCancel={cancelSpaceArtifactCreation} onOpen={openReadySpaceArtifactCreation} />
              ))}
            </div>
          ) : null}
          {/* 💡️ The host-owned ephemeral inference port for exactly one document. It is mounted
           * only while the retained worker operation is live, renders solely from the worker's own
           * bounded status, and writes nothing into the document. */}
          {inferencePortRuntimeKey === null && inferencePort === undefined && session?.app.dialect.artifactKind === "s.gis.gismap" ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 w-[28rem] -translate-x-1/2 rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              <GisMapInferenceRequestControl locale={uiLocale === "de" ? "de" : "en"} onRequest={() => { void requestInferenceProposal(session, () => shellStateRef.current.pluginRuntime.session === session).catch((error) => { console.log("[DEBUG] gis-map-inference-request", error); }); }} />
            </div>
          ) : null}
          {inferencePortRuntimeKey !== null && inferencePort !== undefined ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 w-[28rem] -translate-x-1/2 rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              <InferencePortPanel status={inferencePort} locale={uiLocale === "de" ? "de" : "en"} onAction={(action) => dispatchInferencePortIntent(inferencePortRuntimeKey, action)} />
            </div>
          ) : null}
          {directoryBootstrapUi.kind !== "idle" ? (
            <div className="pointer-events-auto absolute top-workbench right-double z-50 rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              <DirectoryBootstrapStatusNotice state={directoryBootstrapUi} locale={uiLocale} onCancel={cancelDirectoryBootstrap} />
            </div>
          ) : null}
          {/* 🏛️ The Shell-owned administration pane for exactly one space. It is mounted only while
           * the retained worker operation is live, and it renders solely from the hub's own canonical
           * page — never from a locally stored role. */}
          {spaceAdministration !== null ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 max-h-[70vh] w-[36rem] -translate-x-1/2 overflow-auto rounded-sm border bg-base shadow-sm">
              <SpaceAdministrationPane
                spaceId={spaceAdministration.spaceId}
                phase={spaceAdministration.phase}
                page={spaceAdministration.page}
                receiptSha256={spaceAdministration.receiptSha256}
                code={spaceAdministration.code}
                inviteCapabilityPending={spaceAdministration.inviteCapabilityPending}
                inviteCapabilityStatus={spaceAdministration.inviteCapabilityStatus}
                onIntent={dispatchSpaceAdministrationIntent}
              />
            </div>
          ) : null}
          {/* 🧯️ Non-blocking notice — e.g. a `"viewer.read-only"` fault (contract freeze §2.3/§5): never
           * a crash, never blocks interaction with the rest of the shell. */}
          {transientNotice ? (
            <div
              role="status"
              aria-live="polite"
              data-semio-transient-notice=""
              data-notice-code={transientNotice.code}
              className={cn("pointer-events-auto absolute top-workbench left-1/2 z-50 -translate-x-1/2 rounded-sm border px-double py-single text-sm shadow-sm", TRANSIENT_NOTICE_TONE_CLASS[transientNotice.kind])}
            >
              {transientNotice.message}
              <button type="button" className="ml-single underline" onClick={() => dispatch({ type: "SET_TRANSIENT_NOTICE", value: null })}>
                {shellLabel("ui.common.close")}
              </button>
            </div>
          ) : null}
          <PanelDockProvider dock={dock} onTabDockDrop={handleTabDockDrop} onTreeUnitDockDrop={handleTreeUnitDockDrop}>
            <Layout
              mobile={mobile}
              mobilePanel={mobilePanel}
              navbar={<Navbar items={navbarItems} showFullscreenToggle={!mobile} onFullscreenToggle={() => onCommand({ owner: "os", commandId: "os.toggleFullscreen" })} />}
              subnavbar={
                activeTutorial ? (
                  <TutorialBar
                    title={resolveManifestLabel(activeTutorial.title, uiTerminology, uiLocale)}
                    durationMs={activeTutorial.durationMs}
                    playing={tutorialPlaying}
                    rate={tutorialRate}
                    muted={tutorialMuted}
                    captionsOn={tutorialCaptionsOn}
                    recording={tutorialRecording}
                    recordAvailable={tutorialRecorderAvailable}
                    chapters={tutorialChapterMarkers}
                    clock={tutorialClock}
                    onPlayPause={playPauseTutorial}
                    onStop={stopTutorial}
                    onSeek={seekTutorial}
                    onRateChange={(value) => dispatch({ type: "SET_TUTORIAL_RATE", value })}
                    onMutedChange={(value) => dispatch({ type: "SET_TUTORIAL_MUTED", value })}
                    onCaptionsChange={(value) => dispatch({ type: "SET_TUTORIAL_CAPTIONS", value })}
                    onRecordToggle={toggleTutorialRecording}
                    onAddChapter={addTutorialChapter}
                  />
                ) : undefined
              }
              footer={<Footer items={footerItems} />}
              panels={Object.fromEntries(ANCHORS.map((anchor) => [anchor, buildPanelProps(anchor)])) as Record<Anchor, ReturnType<typeof buildPanelProps>>}
              canvasStatus={shellPluginCanvasStatus}
              canvasSkeleton={<CanvasSkeleton label={shellLabel("ui.common.loadingPlugins")} />}
              canvas={
                <ShellFaultBoundary boundaryId="route-canvas" fallbackLabel={shellLabel("ui.common.renderError")}>
                  {canvas}
                </ShellFaultBoundary>
              }
            />
          </PanelDockProvider>
        </div>
        <UISearch items={searchItems} open={searchOpen} onOpenChange={(value) => dispatch({ type: "SET_SEARCH_OPEN", value })} />
        <UIFind open={findOpen} onOpenChange={(value) => dispatch({ type: "SET_FIND_OPEN", value })} />
        <TextSelectionContextMenuHost />
        <AgentPresence status={agentBridge.status} presence={agentBridge.presence} />
        <AgentApprovals approvals={agentBridge.pendingApprovals} onDecision={agentBridge.resolveApproval} />
        <ContextMenuController
          title={shellContextMenuTitleLabel}
          open={shellContextMenu != null}
          position={shellContextMenu}
          items={shellContextMenu?.items ?? []}
          onOpenChange={(open) => {
            if (!open) setShellContextMenu(null);
          }}
        />
        {session && activeIntroduction && introductionStepIndex != null && (
          <UIIntroduction
            introduction={brand?.introduction ?? resolveIntroductionDefinition(activeIntroduction, appLabelsOverlay, uiTerminology, uiLocale)}
            stepIndex={introductionStepIndex}
            completedInteractionIndices={introductionCompletedInteractions}
            onStepIndexChange={(value) => dispatch({ type: "SET_INTRODUCTION_STEP", value })}
            onDismiss={dismissIntroduction}
          />
        )}
        {activeTutorial && (
          <>
            <TutorialCaptionsHost tutorial={activeTutorial} clock={tutorialClock} captionsOn={tutorialCaptionsOn} terminology={uiTerminology} locale={uiLocale} />
            <TutorialVideoOverlayHost tutorial={activeTutorial} clock={tutorialClock} muted={tutorialMuted} playing={tutorialPlaying} rate={tutorialRate} />
            <TutorialGhostPointerHost tutorial={activeTutorial} clock={tutorialClock} />
          </>
        )}
        {session &&
          overlayDialog &&
          (() => {
            if (!isCurrentDialogOrigin(overlayDialog.origin)) return null;
            const dialog = session.app.dialogs?.find((entry) => entry.id === overlayDialog.dialogId);
            if (!dialog) return null;
            const resolved = resolveDialogDefinition(dialog, appLabelsOverlay, uiTerminology, uiLocale, loadedPlugins.map((entry) => entry.manifest), selectedSpaceArtifactKinds);
            const artifactKindArgIds = resolved.args.filter((def) => def.schema.kind === "string" && def.schema.format?.kind === "artifactKind").map((def) => def.id);
            const creationCatalog = artifactKindArgIds.length > 0 ? selectedSpaceArtifactCreationCatalog : null;
            const choiceRevisions = creationCatalog === null ? undefined : Object.fromEntries(artifactKindArgIds.map((id) => [id, creationCatalog.choiceRevision]));
            return (
              <OwnedShellDialog
                owner={overlayDialog}
                dialog={resolved}
                renderField={(def, value, onChange, field) => renderStagedArgControl(def, value, onChange, false, field)}
                notice={creationCatalog === null ? undefined : <ArtifactCreationCatalogNotice status={creationCatalog} locale={uiLocale} hasChoices={creationCatalog.kinds.length > 0} />}
                choiceRevisions={choiceRevisions}
                isCurrent={isCurrentDialogOrigin}
                close={closeOwnedDialog}
                dispatch={(action, origin, args) => onAction({ controllerId: origin.controllerId, action, args }, origin)}
              />
            );
          })()}
      </LevelProvider>
    </UIFindProvider>
    </ShellFaultBoundary>
    </ShellContextMenuFallbackContext.Provider>
    </PluginSurfaceActionsContext.Provider>
    </UiKeybindingsProvider>
    </AppCatalogueContext.Provider>
    </AppKeybindingsContext.Provider>
    </SetWindowIconContext.Provider>
    </SetWindowTitleContext.Provider>
    </BoardSessionFactoryContext.Provider>
  );
}
//#endregion FrameworkOsShell

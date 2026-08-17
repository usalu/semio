// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/component.tsx
/** @emoji 🏗️ `ShellHost` — the `FrameworkOsShell` orchestrator: boots/hot-swaps plugin wasm modules,
 * owns the window/dock/panel layout, wires the tutorial recorder/player, presence, backbone sync,
 * command/tool/utility ribbons, context menus, and mounts every per-app window via `Interpreter`.
 * The single largest component in the renderer-react package. */
// #endregion 🧲️Header

// #region 🔌️Adapters
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
  dialectCoordinate,
  parseDialectCoordinate,
  EMPTY_OPENING_PREFERENCES,
  foldOpeningPreferences,
  type OpeningConfigMutation,
  type OpeningPreferences,
  SemioFaultError,
  SURFACE_FAULT_CODES,
  type CommandAddress,
  type CommandInvocation,
  buildContributionsJson,
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
  evictPluginModule,
  expandPluginRegistry,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  type HostEffect,
  type HistoryEntry,
  type HistoryPatch,
  type IntroductionInteraction,
  type LocalizedLabel,
  NamedLayoutStore,
  normalizeAppLabelsOverlay,
  organizeContextMenu,
  panelTabKindId,
  pendingPanelUiNode,
  pendingWindowUiNode,
  type PluginAppLabelsOverlay,
  type PluginContextMenuRequest,
  type PluginSource,
  type PluginSourceEvent,
  type PluginUiRefreshSectionResponse,
  postPluginBackboneInbound,
  type ProgramHotSwapEvent,
  RECORD_TUTORIAL_ACTION_ID,
  registerPluginBackboneRoute,
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
  type ShellBrand,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  type StoragePort,
  TUTORIAL_CONVERGE_MS,
  type TutorialAssetSrc,
  type TutorialCameraState,
  type TutorialChapter,
  type TutorialDefinition,
  type TutorialDocumentEventKind,
  type TutorialEvent,
  type TutorialGestureCue,
  type TutorialUiChange,
  type TutorialUiSnapshot,
  type TutorialVideoCue,
  type UiDirtyScope,
  type UiNode,
  type UtilityNode,
  windowElementId,
  type WindowEngagement,
  type WindowLayout,
  type WindowMeasure,
  type Conflict,
  type ConflictResolution,
  type Fault,
  type MergePolicy,
  type MergeReport,
  type Severity,
} from "@semio-tech/framework";
import {
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncUtilities,
  buildRemoteBackboneUri,
  decodeBackboneMessage,
  decodeBackboneWorkerResponse,
  decodePackValue,
  type ArtifactActorMsg,
  encodeBackboneMessage,
  encodeBackboneWorkerRequest,
  encodeMutationEnvelopesPack,
  encodePackValue,
  FRAMEWORK_SYNC_CONTROLLER_ID,
  mutationEnvelopeFromWire,
  mutationEnvelopeToWire,
  type MutationEnvelope,
  type PersistenceBinding,
  DirectoryClient,
  DirectoryHttpError,
  type DirectoryCommand,
  type DirectoryEvent,
  type DirectoryStreamMessage,
} from "@semio-tech/framework-os";
/** 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C3 — the config-lane
 * identity facet's documentId/schema + fold. `@semio-tech/framework-os/backbone-worker` is the
 * package's own subpath export for this (`💻️os/📦️packages/🟦️typescript/🟦️glue.backbone-worker.ts`),
 * but this package's own `🧪️vitest.config.ts` (outside this lane's lease) only aliases the bare
 * `@semio-tech/framework-os` specifier, not its subpaths — imported by the same relative path the
 * `new Worker(new URL(...))` call below already uses, sidestepping that gap rather than editing a
 * foreign-leased config file. Never redefined here. */
import { IDENTITY_CONFIG_SCHEMA, identityActorConfig, foldIdentityEvent } from "../../../../../🟦️backbone-worker.ts";
/** 🪪️ Self-contained identity facet (see that file's header doc for why it isn't re-exported through
 * `🎚️config/🧬️schema/**`) — `Identity`/mutation vocabulary, never redeclared here. */
import { type Identity, type IdentityConfigMutation, applyIdentityConfigMutation, signIn } from "../../../../../🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️component.ts";
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
  readStoredUiChromeLocale,
  readStoredUiChromeThemeSnapshot,
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
  UIDialog,
  type UiDriver,
  UIIntroduction,
  UiKeybindingsProvider,
  type UiLocale,
  type UiTranslationKey,
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
  writeStoredUiChromeAppearance,
  writeStoredUiChromeLayout,
  writeStoredUiChromeLocale,
  writeStoredUiChromeTerminology,
  writeStoredUiChromeThemeId,
  writeStoredUiChromeThemeSnapshot,
  writeStoredUiCustomDrivers,
  writeStoredUiCustomThemes,
  writeStoredUiDriverId,
  writeStoredUiKeybindingOverrides,
} from "@semio-tech/ui-react";
import {
  declarativeSurfaceStatus,
  InterpretedUiNode,
  PluginSurfaceActionsContext,
  ShellContextMenuFallbackContext,
  wireLabel,
} from "../Interpreter/🟦️component.tsx";
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
  resolveBootExampleId,
  type ResolvedShellLocks,
  selectOpenConflicts,
  selectQuarantinedConflicts,
  ShellFaultBoundary,
  shellReducer,
  shouldPersistIntroductionSeen,
  shouldReplayIntroductionOnLoad,
  type SpacePanelState,
  type SpaceProgramEntry,
  type SpawnedAppEntry,
  type ViewModel,
} from "../Shell/🟦️component.tsx";
import {
  beginInteractivePluginAction,
  clearPendingWorldProjection,
  endInteractivePluginAction,
  mapContextMenuSpecs,
  registerPendingWorldProjection,
  WindowInstanceIdContext,
} from "../World3dHost/🟦️component.tsx";
import {
  DEFAULT_PANEL_WIDTH_PX,
  EMPTY_APP_LABELS_OVERLAY,
  FRAMEWORK_CATEGORY_COMMAND_ID,
  FRAMEWORK_CATEGORY_DISPLAY_ID,
  FRAMEWORK_CATEGORY_TOOL_ID,
  FRAMEWORK_RESERVED_ACTION_IDS,
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
  buildActiveUtilityByWindowId,
  buildCommandCategoryTabs,
  buildNoteShellCommandAction,
  buildOsCommands,
  buildSpacePanelState,
  buildToolTabs,
  buildUiRefreshRequest,
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
  downloadDataUrl,
  downloadMediaExport,
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
  openArtifactWithText,
  OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID,
  OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID,
  type OpenWithEntry,
  panelAnchorForGroup,
  panelJsonFromState,
  panelTabDefinitionToNode,
  parsePanelState,
  parseShellRoute,
  patchDocumentTreeSelectedIds,
  AutoCheckinScheduler,
  canCheckIn,
  checkinActionText,
  checkinCancelText,
  checkinMessagePlaceholderText,
  checkinSubmitText,
  computeSyncPillState,
  patchWorld3dChromeOntoNode,
  presenceClientIdentity,
  preserveJsonIdentity,
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
  resolveManifestLabel,
  resolvePanelTabLabel,
  resolveUtilityActivation,
  resolveUtilityNodes,
  resolveWindowEngagement,
  retitleWindowLayoutNode,
  runRequestMediaFrames,
  scheduleDispatchAction,
  sessionWindowInstances,
  setAsDefaultText,
  shellLabel,
  shellTabIcon,
  spawnedWindowChromeForKind,
  studioPanelFocusingSpawned,
  surfaceRoleChipText,
  syncDocumentId,
  syncPillText,
  synthesizeLocalizedLabel,
  toolIdFromPanelTabId,
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
  type ResolvedCommand,
  type UiRefreshCache,
} from "../ShellHelpers/🟦️component.tsx";

import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "../../../../../../../../♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx";
import { ENTWERFEN_MIT_BESTAND_BRAND_IDS } from "../../../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts";
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
} from "../ChromePanels/🟦️component.tsx";
import { type PluginWasmHandle, setPluginRuntimeActor } from "../PluginRuntime/🟦️component.tsx";
import { EXTENSION_TARGETS } from "../../../../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️plugins.ts";
import { PLUGIN_CATALOG } from "../../../../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️catalog.ts";


import { SyncAttachCard } from "../ShellSync/🟦️component.tsx";
import { UIFind, UIFindProvider, UISearch, type UISearchItem } from "../ShellSearch/🟦️component.tsx";
import { UTILITY_CATEGORY_ICON_ID } from "../UtilityTree/🟦️component.tsx";
import { coerceWireBytes } from "../PluginRuntime/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region FrameworkOsShell
/** @emoji 🏷️ Lets a per-window host rewrite its Mode window title (e.g. live projection label). */
export const SetWindowTitleContext = createContext<((windowId: string, title: string) => void) | null>(null);

/** @emoji 🖼️ Lets a per-window host rewrite its Mode window icon (e.g. live projection glyph). */
export const SetWindowIconContext = createContext<((windowId: string, iconId: IconName) => void) | null>(null);

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
function encodeWindowActionInvocation(
  session: ActiveSession,
  action: ActionDescriptor,
  extraInstances: readonly ExtraWindowInstance[] = [],
  requestedWindowId?: string,
): string {
  const instances = sessionWindowInstances(session.app, extraInstances);
  const windowInstanceId = requestedWindowId ?? session.viewState.windowId ?? session.viewState.activeWindowKindId ?? instances[0]?.id ?? session.app.windowKinds[0]?.id ?? "";
  const windowKindId = instances.find((instance) => instance.id === windowInstanceId)?.windowKindId ?? session.app.windowKinds.find((kind) => kind.id === windowInstanceId)?.id ?? session.app.windowKinds[0]?.id ?? "";
  const invocation: ActionInvocation = {
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
  return JSON.stringify(invocation);
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
function appOwnsCommand(app: AppDefinition, commandId: string): boolean {
  return (app.commands ?? []).some((command) => command.id === commandId);
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
  if (next.selectionJson != null && prev.selectionJson !== next.selectionJson) changes.push({ kind: "selection", selectionJson: next.selectionJson });
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
      base: { documentJson: this.baseDocumentJson ?? undefined, exampleId, ui: this.baseUiSnapshot, cameras: [] },
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
 * `VITE_S_*` compile-time define (`💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts`'s
 * `define` block). Guarded for non-Vite embeds (SSR/tests/other bundlers) where `import.meta.env` is
 * absent — mirrors `Shell/🟦️component.tsx`'s `readViteAppRoleEnv` idiom (that file is out of this
 * lane's lease, so re-implemented locally rather than imported). Returns `undefined` for an unset/
 * empty define, never `""` — every call site treats "no hub env" as "skip identity entirely" (§C3
 * "No hub env ⇒ skip all of it and keep today's local-only behaviour exactly"). */
function readViteSEnv(name: "VITE_S_HUB_URL" | "VITE_S_USER" | "VITE_S_DATA_DIR"): string | undefined {
  try {
    const env = (import.meta as unknown as { readonly env?: Readonly<Record<string, string | undefined>> }).env;
    return env?.[name] || undefined;
  } catch {
    return undefined;
  }
}

/** 🪪️ One `MutationEnvelope` wrapping an {@link IdentityConfigMutation} — mirrors the exact shape the
 * in-source `foldIdentityEvent` tests build (`🟦️backbone-worker.ts`'s `🔖️DirectoryLaneTests`/identity
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
  if (typeof candidate.userId === "string" && typeof candidate.email === "string" && typeof candidate.sessionToken === "string") return candidate as Identity;
  return undefined;
}

/** 🪪️ Mints the shell actor id (contract §C0: `user:{userId}#{shellSessionId}`) once identity
 * resolves; the pre-identity default stays `client-{shellSessionId}` (unchanged shape, just the
 * random-suffix source now shared with the post-sign-in id so a reload's actor id is stable relative
 * to its own tab even before/without a hub). */
export function shellActorId(sessionId: string, identity: Identity | null): string {
  return identity ? `user:${identity.userId}#${sessionId}` : `client-${sessionId}`;
}

/** 🪪️ Canonical surface id (contract §C0 `<kind>@<standard>/<subset>#<role>`) for whichever app/role
 * a session is opening — the `PersistenceBinding.hub.surface` `openDocument`'s default bindings (§2)
 * stamp onto the hub document WS URL's `?surface=`. */
export function canonicalSurfaceId(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/** 🪪️ ticket §C4 — the space's own artifact-index document: kind `s.space`, dialect
 * `s.space.space@1/*`, document id always the literal `"index"` (one per hub space). No TS constant
 * is exported for these (lane 1-E's TS twin re-exports types only, see `📓️w1-e-report.md`), so they
 * are mirrored here from the Rust source of truth
 * (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🦀️component.rs`'s `S_SPACE_INDEX_DOCUMENT_SCHEMA`/
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

/** 📇️ Maps an `os.directory.<verb>` action id (contract §C6's 7 command ids) + its relayed JSON args
 * onto a {@link DirectoryCommand} — `share-link` has no directory-schema command kind of its own
 * (contract §C1), so it's client-side sugar for `create-invite` (see the new
 * `🎮️commands/📇️directory-share-link/🦀️component.rs` header doc). Returns `null` for an
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
    const initialLocale = locks?.locale ?? readStoredUiChromeLocale(storage) ?? detectShellLocale(typeof navigator !== "undefined" ? navigator.language : undefined);
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

function FrameworkOsShellInner({
  pluginFilter,
  plugins,
  appId,
  appRole,
  locks: locksProp,
  defaults: defaultsProp,
  brand,
  suppressAutoIntroduction = false,
}: {
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
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
  const { loadedPlugins, pluginStatusById, pluginSupervisorById, session, error } = shellState.pluginRuntime;
  const applyHistoryPatch = useCallback((patch: HistoryPatch | undefined, replace = false) => {
    if (!patch) return;
    setHistoryProjection((current) => {
      if (!replace && patch.cursor <= current.cursor) return current;
      const entries = replace ? {} as Record<number, HistoryEntry> : { ...current.entries };
      for (const entry of patch.upserts ?? []) entries[entry.seq] = entry;
      // 📌️ §C5 — `currentCheckpointId` used to be dropped here even though `HistoryPatch` always
      // carried it; `🔖️CheckIn` below watches it change to know a checkpoint it asked for actually
      // landed (see `touchSpaceIndexArtifact`'s call site).
      return { cursor: patch.cursor, entries, canUndo: patch.canUndo ?? false, canRedo: patch.canRedo ?? false, currentCheckpointId: replace ? patch.currentCheckpointId : (patch.currentCheckpointId ?? current.currentCheckpointId) };
    });
  }, []);
  const hostPlugin = useMemo(() => (hostConfig ? loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId) : undefined), [loadedPlugins, hostConfig]);
  const hostApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.id === hostConfig?.hostAppId), [hostPlugin, hostConfig]);
  const landingApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.id === hostConfig?.landingAppId) ?? hostPlugin?.manifest.apps[0], [hostPlugin, hostConfig]);
  const landingAppId = hostConfig?.landingAppId;
  const hostAppId = hostConfig?.hostAppId;
  const hostControllerId = hostApp?.controllerId;
  const landingControllerId = landingApp?.controllerId;
  const hostCatalogueTabId = hostApp?.panelTabs[0] ? panelTabKindId(hostApp.panelTabs[0].kind) : undefined;
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
    void plugin.readHistory(session.instanceId).then((snapshot) => {
      if (!cancelled) applyHistoryPatch(snapshot, true);
    }).catch((error) => console.error("[DEBUG] history snapshot failed", error));
    return () => {
      cancelled = true;
    };
  }, [applyHistoryPatch, session]);
  const { windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId, toolMeasuresByToolId, panelUiByKey, appLabelsOverlay } = shellState.windowUi;
  const { spawnedWindowUi, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;
  const { foldedByWindowId: actionPaneFoldedByWindowId, expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId, activeToolId } = shellState.actionPane;
  const { expandedCommandId, stagedArgsByCommandId: commandStagedArgsByCommandId } = shellState.commandPanel;
  const { panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId, shellLayout, activeExampleId, mobilePanelPath, mobilePanelVisible, extraWindowInstances, windowTitlesById, windowIconsById } = shellState.layout;
  const { searchOpen, findOpen, introductionStepIndex, introductionCompletedInteractions, dialog: overlayDialog, transientNotice, openWithFocusRole } = shellState.overlays;
  const { activeTutorialId, playing: tutorialPlaying, rate: tutorialRate, muted: tutorialMuted, captionsOn: tutorialCaptionsOn, recording: tutorialRecording, deviated: tutorialDeviated } = shellState.tutorial;
  const { uiAppearance, uiLayout, uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale, uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft, uiKeybindingOverrides } = shellState.uiPrefs;
  const { syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId } = shellState.sync;
  const { mergePolicy, conflicts, selectedConflictId } = shellState.merge;
  const importSpaceInputRef = useRef<HTMLInputElement>(null);
  const refreshGenerationRef = useRef(0);
  const contributionsJsonRef = useRef<string | null>(null);
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
  const uiDevice: ElementsSurfaceDevice = mobile ? "mobile" : uiLayout;
  const uiTheme: UiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? readStoredUiChromeThemeSnapshot(scope.storage) ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft, scope.storage]);
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
  /** 🪪️ §C3 identity bootstrap — `null` until `DirectoryClient.me()`/`mintSession` resolves (or forever,
   * with no hub env). Mirrored into {@link shellActorIdRef}/`setPluginRuntimeActor` by the effect below,
   * never read directly for the actor id (that's always `shellActorIdRef.current`). */
  const [identity, setIdentity] = useState<Identity | null>(null);
  const identityRef = useRef<Identity | null>(null);
  identityRef.current = identity;
  /** 🪪️ True once a hub env is configured but the hub could not be reached (§C3 "keep the last
   * persisted identity, show an offline chip... never blocks the UI thread"). */
  const [identityOffline, setIdentityOffline] = useState(false);
  /** 🪪️ REST-only client for the identity boot handshake (`me`/`mintSession`) — distinct from the
   * directory-lane's persistent `/directory/ws` subscription, which the shell never opens itself (§C6:
   * `🟦️backbone-worker.ts`'s `🔖️Directory` region is the only socket owner). Re-created only if the
   * hub base url or token actually changes. */
  const directoryClientRef = useRef<DirectoryClient | null>(null);
  const hubEnv = useMemo(() => {
    const hubBaseUrl = readViteSEnv("VITE_S_HUB_URL");
    const email = readViteSEnv("VITE_S_USER");
    const dataDir = readViteSEnv("VITE_S_DATA_DIR");
    return hubBaseUrl ? { hubBaseUrl, email, dataDir } : null;
  }, []);
  /** 📇️ §C6 — guards `directory-open` to once per shell (a `useEffect` re-running on an unrelated
   * identity re-render must not reopen the socket). */
  const directoryOpenedRef = useRef(false);
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
  /** 📇️ `applyHostEffects`'s new `replayShellCommand` branch (below) needs `openDocument`/
   * `openArtifactWithAppRef`, both declared LATER in this same component (after `applyHostEffects`
   * itself) — a direct reference in `applyHostEffects`'s own dependency array would be a `const`
   * temporal-dead-zone violation at the point `useCallback` evaluates that array. Same ref-forwarding
   * idiom `onActionRef`/`dispatchDirectoryEventsRef` already use: assigned as a plain statement right
   * after each real declaration, read only from inside a later callback body, never from a deps array. */
  const openDocumentRef = useRef<(ref: { readonly documentId: string; readonly schema: string }, bindings?: readonly PersistenceBinding[]) => Promise<void>>(async () => {});
  const openArtifactWithAppRefRef = useRef<(target: AppRef, dialect: ArtifactDialect, role: AppRole) => Promise<void>>(async () => {});
  /** 📇️ Offline-queue depth surfaced by the worker's `directory-status` — not yet rendered by any
   * chrome this lane owns (2-F/3-A's "row shows 'pending'" territory); kept as local state so a
   * consumer can read it once that chrome lands, with zero further plumbing here. */
  const [, setDirectoryPendingCommands] = useState(0);
  /** 🪪️ Settled once by the identity bootstrap effect's very first `snapshotReplaced` for
   * `IDENTITY_CONFIG_SCHEMA` (a previously-persisted session) — see that effect for the bounded
   * timeout that resolves it to `null` when no such file exists (never blocks the UI thread). */
  const identitySnapshotResolverRef = useRef<((value: Identity | null) => void) | null>(null);
  const presenceConnectedAtMsRef = useRef(Date.now());
  const presenceCursorRef = useRef<{ readonly x: number; readonly y: number } | undefined>(undefined);
  /** 🗂️ Which session/plugin owns each open document id, so incoming worker events route correctly. */
  const openDocumentSessionsRef = useRef<Map<string, { session: ActiveSession; plugin: PluginWasmHandle }>>(new Map());
  /** 🐚️ Unregisters this shell's `registerPluginBackboneRoute` entry for each open document id — called
   * from `closeDocument` and (for whatever is still open) on shell unmount. */
  const pluginBackboneRouteUnregistersRef = useRef<Map<string, () => void>>(new Map());
  /** 🐚️ Mirrors `loadedPlugins` for the unmount-cleanup effect below, which needs the latest value at
   * teardown time without depending on it (a dependency would tear down and re-run on every reload). */
  const loadedPluginsRef = useRef<readonly LoadedProgramState[]>([]);
  loadedPluginsRef.current = loadedPlugins;
  /** 🔌️ The exact (possibly cache-busted `?v=`) module URL each currently-loaded plugin was acquired
   * at — `LoadedProgramState`/`PluginWasmHandle` carry no URL of their own, but `reloadPlugin`/
   * `uninstallPlugin` need the OLD url to `evictPluginModule` after swapping in a new lease at a
   * different url (see the lease pool's key convention in `@semio-tech/framework`). */
  const pluginModuleUrlByIdRef = useRef<Map<string, string>>(new Map());
  /** 🔌️ Per-pluginId mutual exclusion across `installPlugin`/`reloadPlugin`/`uninstallPlugin` — the
   * boot effect and the `PluginSource` subscription effect can both request the same pluginId around
   * mount (e.g. the host plugin already appears in the connect-time `snapshot`), and without this guard
   * both calls would independently acquire a module lease, race their `UPSERT_LOADED_PLUGIN` dispatches,
   * and leak whichever lease lost the race (nothing left holding a reference to release it). */
  const pluginOpInFlightRef = useRef<Set<string>>(new Set());

  const ensureBackboneWorker = useCallback((): Worker => {
    if (backboneWorkerRef.current) return backboneWorkerRef.current;
    const worker = new Worker(new URL("../../../../../🟦️backbone-worker.ts", import.meta.url), { type: "module" });
    worker.onmessage = (messageEvent: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>) => {
      const message = "wire" in messageEvent.data ? decodeBackboneWorkerResponse(messageEvent.data.wire) : messageEvent.data;
      // 📇️ §C6 directory lane — the worker's `directory-*` responses never carry a `documentId` this
      // shell already has an `openDocumentSessionsRef` entry for (they're not artifact-sync events at
      // all), so they're routed here, ahead of the artifact-event early return below.
      if (message.kind === "directory-message") {
        if (message.message.kind === "event") dispatchDirectoryEventsRef.current([message.message.event]);
        return;
      }
      if (message.kind === "directory-status") {
        setDirectoryPendingCommands(message.pendingCommands);
        return;
      }
      if (message.kind === "directory-command-result") {
        if (!message.ok) {
          console.error("[os-shell] directory command failed", message.requestId, message.error);
        } else if (message.events && message.events.length > 0) {
          // 📇️ Defense-in-depth (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
          // w4-h): the ORIGINATING client folds its own accepted command's events directly instead of
          // depending entirely on the live `/directory/ws` broadcast finding its way back to the same
          // socket — correct only if that subscription is guaranteed already-open at command-issue
          // time, which a fresh page load racing identity bootstrap does not guarantee. The live
          // broadcast path (`directory-message` above) still folds the same events for every OTHER
          // client; a duplicate here is harmless — `FoldDirectoryEvent`'s fold is idempotent per event
          // id (config lane, "last envelope wins" — see `📓️w1-c-report.md`).
          dispatchDirectoryEventsRef.current(message.events);
        }
        return;
      }
      // 🪪️ §C3 identity facet — opened directly (never through `openDocument`, so it never gets an
      // `openDocumentSessionsRef` entry: it has no plugin/session, only the shell itself). Routed here,
      // ahead of the `!entry` early return below, which would otherwise silently drop every one of its
      // events. `snapshotReplaced` (whole-record pack, the folder read-back) resolves the bootstrap
      // effect's one-shot wait; `remoteMutations` (another tab's sign-in/out, echoed over this
      // document's `BroadcastChannel`) folds via {@link foldIdentityEvent} like `OpeningPreferences`.
      if (message.kind === "event" && message.documentId === IDENTITY_CONFIG_SCHEMA) {
        const identityEvent = message.event;
        if (identityEvent.kind === "snapshotReplaced") {
          const decoded = decodeIdentityPayload(decodePackValue(new Uint8Array(identityEvent.pack)));
          if (decoded !== undefined) {
            setIdentity(decoded);
            identitySnapshotResolverRef.current?.(decoded);
            identitySnapshotResolverRef.current = null;
          }
        } else if (identityEvent.kind === "remoteMutations") {
          setIdentity((current) => foldIdentityEvent(current, identityEvent, decodeIdentityPayload));
        }
        return;
      }
      if (message.kind !== "event") return;
      const entry = openDocumentSessionsRef.current.get(message.documentId);
      if (!entry) return;
      const { event } = message;
      if (event.kind === "status") {
        dispatch({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: message.documentId, status: { persisted: event.persisted, pendingMutations: event.pendingMutations, remote: event.remote } });
      } else if (event.kind === "presence") {
        const peersJson = JSON.stringify(event.peers.map((peer) => ({ clientId: peer.actor, name: peer.label ?? peer.actor, selectionCount: 0 })));
        dispatch({
          type: "SET_SESSION",
          value: (current) => (current && current.instanceId === entry.session.instanceId ? { ...current, viewState: { ...current.viewState, presencePeersJson: peersJson } } : current),
        });
      } else if (event.kind === "remoteMutations" && entry.plugin.applyMutations) {
        // ⚖️ `AppCommand::ApplyEnvelopes`'s reply to THIS remote ingest batches `MergeReport`/
        // `Conflicts` frames alongside it (contract freeze §C6/§C9 "pushed unsolicited after every
        // ingest") — routed through `applyRemoteMergeRef` (see its declaration doc) so a peer's
        // quarantined/degraded merge reaches the Conflicts panel / a transient notice without the
        // user asking for it, instead of being dropped after the (still-present) error check.
        void entry.plugin
          .applyMutations(entry.session.instanceId, encodeMutationEnvelopesPack(event.envelopes))
          .then((result) => applyRemoteMergeRef.current(result.conflicts, result.mergeReport))
          .catch((commandError) => console.error("[DEBUG] applyMutations failed", commandError));
        const actorUri = `actor://${message.documentId}`;
        postPluginBackboneInbound(entry.session.pluginId, actorUri, [
          encodeBackboneMessage({
            kind: "mutations",
            envelopes: event.envelopes.map((envelope, index) =>
              mutationEnvelopeToWire(envelope, { actor: 0, physical_ms: Date.now(), logical: index + 1 }),
            ),
          }),
        ]);
      } else if (event.kind === "snapshotReplaced" && entry.plugin.loadAppDocument) {
        const packBytes = new Uint8Array(event.pack);
        let documentJson: string;
        try {
          documentJson = JSON.stringify(decodePackValue(packBytes));
        } catch {
          documentJson = JSON.stringify({ pack: Array.from(event.pack), spr: Array.from(event.spr) });
        }
        void entry.plugin.loadAppDocument(entry.session.instanceId, documentJson);
        const actorUri = `actor://${message.documentId}`;
        postPluginBackboneInbound(entry.session.pluginId, actorUri, [
          encodeBackboneMessage({ kind: "snapshot", pack: packBytes, spr: new Uint8Array(event.spr) }),
        ]);
      } else if (event.kind === "conflict") {
        // ⚖️ Investigated for contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
        // CLASS-CONFLICTS` §C6/§C9 (lane L1): this is `🏪️store/🔄️sync/🦀️component.rs`'s hub-relay
        // `ArtifactEvent::Conflict(MutationMessage)` — a TRANSPORT-level diagnostic (external folder
        // divergence / a hub `ServerFrame::Error`), never a first-class `Conflict` roster (no
        // `id`/`status`/`actors` exists for either source event, and `protocol_wire::ServerFrame` has
        // no `MergeReport`/`Conflicts` variant to carry one). The REAL roster/merge-outcome delivery
        // this contract added is `AppCommand::ApplyEnvelopes`'s own reply — see the `remoteMutations`
        // branch above (`applyRemoteMergeRef`) — so this branch stays a passive log, same as before.
        console.warn("[os-shell] sync conflict", message.documentId, event.message);
      }
    };
    backboneWorkerRef.current = worker;
    return worker;
  }, []);

  // 🪪️ §C3 identity bootstrap — pre-identity default actor, set once at mount so `PluginRuntime`'s
  // `AppChannelClient`s created before sign-in resolves (or with no hub env at all) still carry the
  // SAME `client-{sessionId}` id `shellActorIdRef` already defaults to.
  useEffect(() => {
    setPluginRuntimeActor(shellActorIdRef.current);
  }, []);

  useEffect(() => {
    // 📇️ §C3 "No hub env ⇒ skip all of it and keep today's local-only behaviour exactly" — the
    // existing `🛠️dev🖥️s⚛️react` launcher (no `S_HUB_URL`) never reaches any code below this guard.
    if (!hubEnv) return;
    let cancelled = false;
    (async () => {
      const worker = ensureBackboneWorker();
      const identityConfig = identityActorConfig(shellActorIdRef.current, hubEnv.dataDir);
      // 📇️ Opens the identity document FIRST (folder poll starts immediately) so the snapshot wait
      // below has something to resolve against; re-opening later with the same `documentId` (once the
      // real actor id is known) is a harmless idempotent re-subscribe (`openArtifact` always closes
      // any prior state for the same id first).
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", ...identityConfig }) });
      // 📇️ Bounded one-shot wait for a previously-persisted identity's `snapshotReplaced` (a 404/no-
      // file-yet folder read never emits one at all — see `pollFolderOnce`'s doc in
      // `🟦️backbone-worker.ts` — so this can only be resolved by a timeout, not a second event).
      // 2s is generous for a local folder read; never blocks the UI thread (this whole effect body
      // runs off-render, in a microtask/timer chain).
      const cachedIdentity = await new Promise<Identity | null>((resolve) => {
        identitySnapshotResolverRef.current = resolve;
        setTimeout(() => resolve(null), 2000);
      });
      identitySnapshotResolverRef.current = null;
      if (cancelled) return;
      const client = new DirectoryClient(hubEnv.hubBaseUrl, cachedIdentity?.sessionToken);
      directoryClientRef.current = client;
      let resolved: Identity | null = null;
      try {
        const me = cachedIdentity?.sessionToken ? await client.me() : null;
        if (me) {
          resolved = { userId: me.userId, email: me.email, displayName: me.displayName, hubBaseUrl: hubEnv.hubBaseUrl, sessionToken: cachedIdentity!.sessionToken, issuedAtMs: cachedIdentity!.issuedAtMs };
        } else {
          const email = hubEnv.email ?? cachedIdentity?.email;
          if (!email) throw new DirectoryHttpError(0, "no VITE_S_USER and no cached identity to mint a session for");
          const minted = await client.mintSession(email);
          resolved = { userId: minted.userId, email, displayName: email, hubBaseUrl: hubEnv.hubBaseUrl, sessionToken: minted.token, issuedAtMs: Date.now() };
        }
      } catch (error) {
        // 📇️ §C3 "Hub unreachable ⇒ keep the last persisted identity, show an offline state, never
        // block the UI, never throw" — `cachedIdentity` (if any) was already applied via the
        // `snapshotReplaced` handler above; nothing further to roll back.
        console.error("[os-shell] identity bootstrap: hub unreachable, staying offline", error);
        if (!cancelled) setIdentityOffline(true);
        return;
      }
      if (cancelled || !resolved) return;
      setIdentityOffline(false);
      shellActorIdRef.current = shellActorId(shellSessionIdRef.current, resolved);
      setPluginRuntimeActor(shellActorIdRef.current);
      setIdentity(resolved);
      const mutation = signIn(resolved);
      const envelope = identityMutationEnvelope(shellActorIdRef.current, mutation, cachedIdentity);
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: IDENTITY_CONFIG_SCHEMA, message: { kind: "localMutations", envelopes: [envelope] } }) });
      worker.postMessage({
        wire: encodeBackboneWorkerRequest({
          kind: "send",
          documentId: IDENTITY_CONFIG_SCHEMA,
          message: { kind: "localSnapshot", pack: Array.from(encodePackValue(resolved)), spr: [] },
        }),
      });
      // 📇️ §C6 — one directory socket per shell, opened only once identity resolves (never on the UI
      // thread itself: this posts a request into `🟦️backbone-worker.ts`'s `🔖️Directory` region, the
      // socket's real owner).
      if (!directoryOpenedRef.current) {
        directoryOpenedRef.current = true;
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-open", baseUrl: resolved.hubBaseUrl, token: resolved.sessionToken, since: 0 }) });
      }
    })().catch((error) => console.error("[os-shell] identity bootstrap failed unexpectedly", error));
    return () => {
      cancelled = true;
    };
  }, [hubEnv]);

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
    if (!session) return "loading";
    if (!primaryPluginId) return undefined;
    const pluginStatus = pluginStatusById[primaryPluginId];
    if (pluginStatus === "installing" || pluginStatus === "reloading") return "loading";
    return undefined;
  }, [session, primaryPluginId, pluginStatusById]);
  /** 🔌️ Dev `/plugin-modules` catalog plus extension-store `/extensions` installs share one
   * {@link PluginSource} so the incremental runtime can load from either tree. */
  const pluginSource: PluginSource = useMemo(() => multiplexPluginSources(createDevPluginSource(registry), createExtensionSource(PLUGIN_CATALOG)), [registry]);

  /** 🔌️ Recreates the primary session instance for `handle` — the exact `hostConfig`/non-studio
   * app-resolution logic the boot effect used to run once inline, now shared with `reloadPlugin` so a
   * hot-swap of the session-owning plugin re-establishes the session the same way boot does. */
  const establishPrimarySession = useCallback(
    async (handle: PluginWasmHandle) => {
      const manifest = handle.manifest;
      if (hostConfig) {
        const sApp = manifest.apps.find((app) => app.id === hostConfig.landingAppId) ?? manifest.apps[0];
        if (!sApp) throw new Error("host program missing landing app");
        // 🪦️ `manifest.workflows` (the source `buildSpacePrograms` used to read) was deleted from the
        // Rust `PluginManifest` — the studio catalogue is now registry-driven (see `SpaceCommand::SetAppRegistrations`),
        // so `SpacePanelState.programs` is permanently empty; `spawnedApps`/`activePanelTab`/`activeSpawnedId` are
        // still real, live state, so `SpacePanelState` itself stays.
        const panelState = buildSpacePanelState([], []);
        const instanceId = await handle.createApp(sApp.id);
        const viewState: ViewModel = { activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id, panelJson: panelJsonFromState(panelState) };
        // 🪟️ Seed default-layout panes (Top/Perspective) before any effect can fire actions — otherwise
        // boot `setActiveExample` races the session-switch refresh and wipes pane bodies.
        const seeded = applyFrameworkLayoutSeed(sApp.defaultLayout, sApp.windowKinds, EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
        extraWindowInstancesRef.current = seeded.extraInstances;
        extraWindowCounterRef.current = seeded.extraInstances.length;
        dispatch({ type: "SET_SESSION", value: { pluginId: handle.pluginId, instanceId, app: sApp, viewState } });
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
        dispatch({ type: "SET_ERROR", value: null });
        return;
      }
      const primaryApp = appId
        ? (() => {
            const found = manifest.apps.find((app) => app.id === appId);
            if (!found) throw new Error(`appId "${appId}" does not resolve to any app in the loaded program manifest`);
            return found;
          })()
        : (() => {
            const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(PLUGIN_CATALOG, pluginFilter) : undefined;
            // 👁️✏️ An unpinned `appId` still prefers the boot-time role (contract freeze §5) — the
            // role of an OPEN session always comes from `session.app.role`, never `appRole` itself;
            // this only breaks a tie among apps the manifest already offers for the default dialect.
            return (defaultAppId ? manifest.apps.find((app) => app.id === defaultAppId) : undefined) ?? manifest.apps.find((app) => app.role === appRole) ?? manifest.apps[0];
          })();
      if (!primaryApp) return;
      const instanceId = await handle.createApp(primaryApp.id);
      const seeded = applyFrameworkLayoutSeed(primaryApp.defaultLayout, primaryApp.windowKinds, EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
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
    [hostConfig, appId, appRole, pluginFilter, uiTerminology, uiLocale],
  );

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
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "loaded" });
        if (pluginId === primaryPluginId && !sessionRef.current) {
          try {
            await establishPrimarySession(handle);
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "running" });
          } catch (bootError) {
            console.error("[DEBUG] framework os boot failed", bootError);
            dispatch({ type: "SET_ERROR", value: bootError instanceof Error ? bootError.message : String(bootError) });
            return "failed";
          }
        }
        return "loaded";
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [registry, pluginSource, primaryPluginId, establishPrimarySession],
  );

  /** 🔌️ Hot-swaps an already-loaded plugin to a newly built module — mirrors the os-core kernel's
   * `PluginHost::hot_swap_plugin` contract (validate → destroy affected instances → swap → recreate the
   * session if it was this plugin's → release the old module) without inventing a separate one:
   * acquires the new module BEFORE tearing anything down (the old handle keeps serving concurrent
   * traffic during the swap), validates the new manifest still declares apps (and, if this plugin owns
   * the active session, still declares the session's app id), then only commits. A validation failure
   * disposes the new lease and leaves the old plugin exactly as it was — nothing destroyed, status back
   * to `"loaded"`. */
  const reloadPlugin = useCallback(
    async (pluginId: string, rebuiltAt?: number) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return installPlugin(pluginId, rebuiltAt);
      const oldModuleUrl = pluginModuleUrlByIdRef.current.get(pluginId);
      pluginOpInFlightRef.current.add(pluginId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "reloading" });
      dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
      let newHandle: PluginWasmHandle | null = null;
      try {
        const moduleUrl = pluginSource.moduleUrl(pluginId, rebuiltAt);
        newHandle = await loadPluginModuleResilient(pluginId, moduleUrl);
        if (!newHandle) throw new Error(`program ${pluginId} failed to reload`);
        if (newHandle.manifest.apps.length === 0) throw new Error(`program ${pluginId} reload declares no apps`);
        const activeSession = sessionRef.current;
        const ownsSession = activeSession?.pluginId === pluginId;
        if (ownsSession && activeSession && !newHandle.manifest.apps.some((app) => app.id === activeSession.app.id)) {
          throw new Error(`program ${pluginId} reload dropped the active session's app "${activeSession.app.id}"`);
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
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle: newHandle, manifest: newHandle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });

        if (ownsSession) await establishPrimarySession(newHandle);

        current.handle.dispose();
        if (oldModuleUrl) evictPluginModule(oldModuleUrl);
      } catch (error) {
        console.warn(`[DEBUG] hot-swap rolled back for ${pluginId}`, error);
        newHandle?.dispose();
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [installPlugin, establishPrimarySession, hostMode, pluginSource],
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
        current.handle.dispose();
        const moduleUrl = pluginModuleUrlByIdRef.current.get(pluginId);
        pluginModuleUrlByIdRef.current.delete(pluginId);
        if (moduleUrl) evictPluginModule(moduleUrl);
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
      const wire = encodeWindowActionInvocation(active, { controllerId: active.app.controllerId, action, args }, extraWindowInstancesRef.current);
      await pluginEntry.handle.handleAction(active.instanceId, wire, active.viewState);
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
        const response = await fetch("/extensions/install", {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ url: sourceUri }),
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
        const response = await fetch("/extensions/install", {
          method: "POST",
          headers: { "content-type": "application/octet-stream" },
          body: bytes,
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
          current.handle.dispose();
          const moduleUrl = pluginModuleUrlByIdRef.current.get(extensionId);
          pluginModuleUrlByIdRef.current.delete(extensionId);
          if (moduleUrl) evictPluginModule(moduleUrl);
        }
        setExtensionLedger((prev) => prev.filter((entry) => entry.extensionId !== extensionId));
        void dispatchSpaceExtensionOp("uninstallExtension", { extensionId });
        try {
          await fetch(`/extensions/install?extensionId=${encodeURIComponent(extensionId)}`, { method: "DELETE" });
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

  // 🎓️ Auto-starts an app's introduction the first time it launches on this device (or every load when
  // the brand opts in); replaying stays available afterward via the shell-owned Introduce App command.
  // 🎥️ Never auto-starts while a tutorial is active (mutual exclusivity) — `activeTutorialId` is declared
  // just below (the TutorialOrchestration block's state resolution); read via `shellState.tutorial`
  // directly here rather than the not-yet-declared local to avoid a definition-order dependency.
  useEffect(() => {
    if (!session || !activeIntroduction || shellState.tutorial.activeTutorialId != null) return;
    if (typeof window !== "undefined" && window.self !== window.top) return;
    // Embedded multi-shell hosts (demonstrator grid) pass suppressAutoIntroduction while a pane is
    // backgrounded. That must both block auto-start AND tear down an already-running tour — otherwise the
    // unfocused shell keeps mounting UIIntroduction (veil/hotkeys/ghost cursor) and steals step chrome
    // from the focused pane.
    if (suppressAutoIntroduction) {
      dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
      return;
    }
    if (!replayIntroductionOnLoad && readStoredIntroductionSeen(scope.storage, introductionSeenKey)) return;
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
  activeUtilityByWindowIdRef.current = activeUtilityByWindowId;
  const activeToolIdRef = useRef(activeToolId);
  activeToolIdRef.current = activeToolId;
  /** 🧰️ Dispatch + sync the ref immediately — `refreshUi` reads the ref before the next render, so a
   * bare `dispatch(SET_ACTIVE_UTILITY)` alone leaves the map stale and the gumball never appears. */
  const setActiveUtilityForWindow = useCallback((windowId: string, utilityId: string | null) => {
    activeUtilityByWindowIdRef.current = { ...activeUtilityByWindowIdRef.current, [windowId]: utilityId };
    dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId });
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
  const tutorialDrivenRef = useRef(false);
  const tutorialPlayingRef = useRef(tutorialPlaying);
  tutorialPlayingRef.current = tutorialPlaying;
  const tutorialRecordingRef = useRef(tutorialRecording);
  tutorialRecordingRef.current = tutorialRecording;
  /** ⏺️ Non-null while armed — mutated by `toggleTutorialRecording` (defined in the TutorialOrchestration block below), read/appended-to by `onAction`'s recorder tap right below. */
  const tutorialRecorderRef = useRef<TutorialRecorder | null>(null);
  const shellStateRef = useRef(shellState);
  shellStateRef.current = shellState;

  /** 🎓️ Ends the active introduction — persists the seen flag when configured, and on successful
   * completion (Done / last interaction) fires the tour-finale {@link celebrateAllElements} stamp
   * across every mounted UI element. Skip/escape passes `completed: false` and does not celebrate. */
  const dismissIntroduction = useCallback(
    (completed: boolean) => {
      if (completed && scope.rootRef.current) celebrateAllElements(CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
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

  useEffect(() => {
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [panel?.activeSpawnedId, session, hostMode]);

  /** 🐚️ The relay a document's `registerPluginBackboneRoute` entry uses — forwards a plugin's outbound
   * backbone bytes into THIS shell's own backbone worker. Registered per open document (in
   * `openDocument`/`closeDocument` below) rather than once for the whole shell: the old single
   * page-global relay slot (`setPluginBackboneOutboundRelay`) meant a second mounted shell silently
   * stole every document's outbound routing, then severed it entirely on that shell's unmount. */
  const relayPluginBackboneMessage = useCallback((uri: string, messageBytes: Uint8Array) => {
    const documentId = uri.startsWith("actor://") ? uri.slice("actor://".length) : null;
    if (!documentId) return;
    const worker = backboneWorkerRef.current;
    if (!worker) return;
    let actorMessage: ArtifactActorMsg;
    try {
      const parsed = decodeBackboneMessage(messageBytes);
      if (parsed.kind === "mutations") {
        actorMessage = {
          kind: "localMutations",
          envelopes: parsed.envelopes.map((envelope) => mutationEnvelopeFromWire(envelope)),
        };
      } else if (parsed.kind === "snapshot") {
        actorMessage = { kind: "localSnapshot", pack: Array.from(parsed.pack), spr: Array.from(parsed.spr) };
      } else {
        return;
      }
    } catch {
      return;
    }
    const request: BackboneWorkerRequest = { kind: "send", documentId, message: actorMessage };
    worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, []);

  useEffect(() => {
    const worker = backboneWorkerRef.current;
    return () => worker?.terminate();
  }, []);

  useEffect(() => {
    return () => {
      for (const unregister of pluginBackboneRouteUnregistersRef.current.values()) unregister();
      pluginBackboneRouteUnregistersRef.current.clear();
      const primary = sessionRef.current;
      if (primary) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === primary.pluginId)?.handle;
        void plugin?.destroyApp(primary.instanceId).catch(() => {});
      }
      // 🪶️ Closes the previously-documented Wave-1 gap: studio-mode spawned apps (`panel.spawnedApps`)
      // and external-slot contributor instances (`contributorInstancesRef`) each hold a live plugin
      // instance too — leaving them running past shell unmount was pure leaked memory (see
      // REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT). Best-effort: an instance the guest already dropped,
      // or whose plugin already disposed, just rejects harmlessly via the same `.catch(() => {})`
      // pattern the primary session's own destroy already used above.
      for (const spawned of spawnedAppsRef.current) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === spawned.pluginId)?.handle;
        void plugin?.destroyApp(spawned.instanceId).catch(() => {});
      }
      for (const [pluginId, instanceId] of contributorInstancesRef.current) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId)?.handle;
        void plugin?.destroyApp(instanceId).catch(() => {});
      }
      contributorInstancesRef.current.clear();
      for (const entry of loadedPluginsRef.current) entry.handle.dispose();
    };
  }, []);

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
  useEffect(() => {
    const registryIds = new Set(registry.map((entry) => entry.pluginId));
    const handlePluginAvailable = (pluginId: string, rebuiltAt: number) => {
      if (!registryIds.has(pluginId)) return;
      const alreadyLoaded = loadedPluginsRef.current.some((entry) => entry.handle.pluginId === pluginId);
      void (alreadyLoaded ? reloadPlugin(pluginId, rebuiltAt) : installPlugin(pluginId, rebuiltAt));
    };
    return pluginSource.subscribe((event: PluginSourceEvent) => {
      if (event.kind === "snapshot") {
        for (const plugin of event.plugins) handlePluginAvailable(plugin.pluginId, plugin.rebuiltAt);
        return;
      }
      handlePluginAvailable(event.pluginId, event.rebuiltAt);
    });
  }, [registry, pluginSource, installPlugin, reloadPlugin]);

  const requestContextMenu = useCallback(
    async (request: PluginContextMenuRequest): Promise<readonly ContextMenuItemSpec[]> => {
      if (!session) return [];
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin?.contextMenu) return [];
      // 🖱️ No view state on the wire — the SDK's ContextMenuWireRequest dropped it (the plugin's
      // own persisted selection/hover state already answers "what's selected", see AppActionRegistry
      // funnel); sending one here would just be silently discarded on the Rust side.
      return plugin.contextMenu(session.instanceId, request);
    },
    [loadedPlugins, session],
  );

  const refreshUi = useCallback(
    // 🪟️ `extraInstancesOverride` lets a caller that just synchronously computed a NEW extra-window list
    // (split/drop, layout/mode switch) hand it straight to this fetch instead of reading `extraWindowInstances`
    // from React state, which wouldn't reflect the just-dispatched change until the next render.
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }, extraInstancesOverride?: readonly ExtraWindowInstance[]) => {
      if (scopeArg.kind === "none") return;
      const generation = ++refreshGenerationRef.current;
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
      const layoutSeed = isSessionSwitch ? applyFrameworkLayoutSeed(nextSession.app.defaultLayout, nextSession.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale) : undefined;
      // 🪟️ Prefer the override, then the just-computed session-switch seed, then the live ref (never the
      // render-closure snapshot) so a concurrent refresh cannot drop default-layout panes.
      const extraInstancesForFetch = extraInstancesOverride ?? layoutSeed?.extraInstances ?? extraWindowInstancesRef.current;
      const windowInstances = sessionWindowInstances(nextSession.app, extraInstancesForFetch);
      const disabledExtensionIds = new Set(extensionLedgerRef.current.filter((entry) => !entry.enabled).map((entry) => entry.extensionId));
      const contributionsJson = buildContributionsJson(
        loadedPlugins
          .filter((entry) => !disabledExtensionIds.has(entry.handle.pluginId))
          .map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })),
      );
      // 🪐️ Every loaded plugin's declared apps, flattened for the space app's catalogue — mirrors
      // `contributionsJson` above exactly (same opt-in hint-push shape below), because the space app is
      // its own wasm component: `semio_framework_os::APP_REGISTRATIONS` (populated at native/test
      // `PluginHost::load_plugin`/`hot_swap_plugin` time) lives in a separate linear memory from the
      // space app's own statically-linked copy of the same os-core crate, so nothing crosses the wasm
      // boundary unless this shell pushes it explicitly.
      const appRegistrationsJson = JSON.stringify(loadedPlugins.flatMap((entry) => (entry.manifest.apps ?? []).map((app) => ({ pluginId: entry.handle.pluginId, app }))));
      const viewState: ViewModel = injectActiveTool({
        ...nextSession.viewState,
        contributionsJson,
        locale: uiLocale,
        terminology: uiTerminology,
        windowInstances: windowInstances.map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        activeUtilityId: undefined,
      });
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      // 🐢️ One batched, hash-conditional round trip replaces the old ~12 sequential
      // render/utilities/windowEngagements/windowMeasures/appLabels calls — the plugin omits payloads for
      // any section whose hash still matches what `cache` already holds.
      const request = buildUiRefreshRequest(scope, windowInstances, panelTabLeaves, viewState, cache);
      if (request) {
        const response = await program.refreshUi(nextSession.instanceId, request);
        if (generation !== refreshGenerationRef.current) return;
        const slotContext = {
          plugins: new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.handle])),
          contributorInstances: contributorInstancesRef.current,
          viewState,
        };
        // Resolve external slots on freshly-changed window/panel bodies only, before caching them, so a
        // later no-operation refresh reuses the already-resolved cached value instead of re-resolving.
        const resolveIfChanged = async (entry: PluginUiRefreshSectionResponse): Promise<PluginUiRefreshSectionResponse> => (entry.value !== undefined ? { ...entry, value: await resolveExternalSlots(entry.value as UiNode, slotContext) } : entry);
        const [resolvedWindows, resolvedPanels] = await Promise.all([Promise.all((response.windows ?? []).map(resolveIfChanged)), Promise.all((response.panels ?? []).map(resolveIfChanged))]);
        if (generation !== refreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, { ...response, windows: resolvedWindows, panels: resolvedPanels });
        // ⏱️ See `DocumentApp::pending_effects` — e.g. resuming a `flowEvalTick` chain after this refresh.
        if (response.requestedEffects?.length) await applyHostEffects(response.requestedEffects, nextSession);
      }
      // 🎯 Both push guards below are keyed on `${nextSession.instanceId}::${json}`, NOT on the json
      // content alone — the content is derived purely from `loadedPlugins`, which stabilizes right after
      // boot, so a content-only key would only ever unlock ONE push for the process lifetime (the very
      // first `refreshUi` call, which always targets whatever session exists at boot — usually `home`,
      // which doesn't own either catalogue command). Folding `instanceId` into the key makes a
      // session switch (new studio/space instance opened, same unchanged json) retrigger the push instead
      // of being silently swallowed by a guard that already considered this content "delivered".
      if (contributionsJson) {
        const contributionsPushKey = `${nextSession.instanceId}::${contributionsJson}`;
        if (contributionsPushKey !== contributionsJsonRef.current) {
          contributionsJsonRef.current = contributionsPushKey;
          for (const pluginEntry of loadedPlugins) {
            if (!pluginEntry.manifest.apps?.length) continue;
            const isActive = pluginEntry.handle.pluginId === nextSession.pluginId;
            const targetApp = isActive ? nextSession.app : pluginEntry.manifest.apps.find((app) => appOwnsCommand(app, "setContributions"));
            if (!targetApp || !appOwnsCommand(targetApp, "setContributions") || !pluginEntry.handle.handleCommand) continue;
            const instanceId = isActive ? nextSession.instanceId : contributorInstancesRef.current.get(pluginEntry.handle.pluginId);
            if (instanceId == null) continue;
            try {
              const wire = encodeAppCommandInvocation(pluginEntry.handle.pluginId, targetApp, "setContributions", { json: contributionsJson });
              await pluginEntry.handle.handleCommand(instanceId, wire, nextSession.viewState);
            } catch (error) {
              console.error("setContributions command failed", pluginEntry.handle.pluginId, error instanceof Error ? error.message : String(error));
            }
          }
        }
      }
      if (appRegistrationsJson) {
        const appRegistrationsPushKey = `${nextSession.instanceId}::${appRegistrationsJson}`;
        if (appRegistrationsPushKey !== appRegistrationsJsonRef.current) {
          appRegistrationsJsonRef.current = appRegistrationsPushKey;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId);
          // 🪐️ The space app explicitly declares this hidden app command; other apps never receive it.
          if (pluginEntry?.handle.handleCommand && appOwnsCommand(nextSession.app, "setAppRegistrations")) {
            try {
              const wire = encodeAppCommandInvocation(nextSession.pluginId, nextSession.app, "setAppRegistrations", { json: appRegistrationsJson });
              await pluginEntry.handle.handleCommand(nextSession.instanceId, wire, nextSession.viewState);
            } catch (error) {
              console.error("setAppRegistrations command failed", error instanceof Error ? error.message : String(error));
            }
          }
        }
      }
      // 🐢️ Merge-with-identity-preservation: unrequested/unchanged sections keep exactly the object
      // reference already in `cache` (dispatched from a prior refresh), so `mergeRecordPreservingIdentity`
      // bails on them via reference equality — this is what lets `InterpretedUiNode`'s `React.memo` (and
      // `modeWindows`'s `useMemo`) skip reconciling the whole shell on every interaction.
      dispatch({
        type: "SET_WINDOW_UI_BY_WINDOW_ID",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            windowInstances.map((instance) => [instance.id, (cache.get(`window:${instance.id}`)?.value as UiNode | undefined) ?? current[instance.id] ?? pendingWindowUiNode()] as const),
          ),
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
      dispatch({
        type: "SET_PANEL_UI_BY_KEY",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            panelTabLeaves
              .filter((tab) => tab.bodyKey)
              .map((tab) => [panelTabKindId(tab.kind), (cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as UiNode | undefined) ?? current[panelTabKindId(tab.kind)] ?? pendingPanelUiNode()] as const),
          ),
      });
      if (isSessionSwitch && layoutSeed) {
        layoutSeedKeyRef.current = layoutSeedKey;
        extraWindowInstancesRef.current = layoutSeed.extraInstances;
        extraWindowCounterRef.current = layoutSeed.extraInstances.length;
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: layoutSeed.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: layoutSeed.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      }
    },
    // 🐢️ `applyHostEffects` is declared later in this component (its own deps need `updateSpacePanel`/
    // `syncSpawnedPluginDocument`, declared later still) — referencing it here in the body only (never
    // added to this array) avoids a temporal-dead-zone reference-before-init; safe because this callback
    // is only ever invoked after render completes, by which point `applyHostEffects` is initialized.
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [appLabelsOverlay, injectActiveTool, uiLocale, uiTerminology],
  );

  /** @emoji 🗣️ Keeps already-built window titles (workbench layout, extra spawned windows) in sync on every locale/terminology switch — `refreshUi` only rebuilds `shellLayout` from scratch on a session change, so an existing session's baked-in titles would otherwise go stale. */
  useEffect(() => {
    const windowKinds = session?.app.windowKinds;
    if (!windowKinds) return;
    dispatch({
      type: "SET_SHELL_LAYOUT",
      value: (current) => (current ? retitleWindowLayoutNode(current, windowKinds, extraWindowInstancesRef.current, uiTerminology, uiLocale) : current),
    });
    dispatch({
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: (current) => {
        const next = current.map((entry) => {
          const kind = windowKinds.find((k) => k.id === entry.windowKindId || k.id === entry.id);
          const title = kind ? resolveManifestLabel(kind.label, uiTerminology, uiLocale) : entry.title;
          return { ...entry, title };
        });
        extraWindowInstancesRef.current = next;
        return next;
      },
    });
  }, [uiTerminology, uiLocale]);

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
        dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}` } as UiNode });
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
      const disabledExtensionIds = new Set(extensionLedgerRef.current.filter((entry) => !entry.enabled).map((entry) => entry.extensionId));
      const contributionsJson = buildContributionsJson(
        loadedPlugins
          .filter((entry) => !disabledExtensionIds.has(entry.handle.pluginId))
          .map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })),
      );
      const bodyKey = resolveCanvasBodyKey(app);
      const fullViewState: ViewModel = injectActiveUtility(
        { ...viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology, windowId: bodyKey, windowInstances: [{ id: bodyKey, windowKindId: bodyKey }] },
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
      const ui = (cache.get(`window:${bodyKey}`)?.value as UiNode | undefined) ?? pendingWindowUiNode();
      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: (current: UiNode | null) => preserveJsonIdentity(current ?? undefined, ui) });
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
      console.error("[DEBUG] render failed", renderError);
      dispatch({ type: "SET_ERROR", value: renderError instanceof Error ? renderError.message : String(renderError) });
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
      console.error("[DEBUG] spawned render failed", renderError);
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
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

  // 🏠️🧳️ Generic replacement for the old `switchToSApp` — switches to either the host plugin's landing
  // or host app by id (both resolved via `hostConfig`, never a specific app's identity).
  const switchToManagedApp = useCallback(
    async (appId: string, viewState?: ViewModel): Promise<ActiveSession | null> => {
      const sPlugin = hostConfig ? loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId) : undefined;
      const app = sPlugin?.manifest.apps.find((candidate) => candidate.id === appId);
      if (!sPlugin || !app) return null;
      if (session?.pluginId === sPlugin.handle.pluginId && session.app.id === appId) {
        if (!viewState) return session;
        const nextSession: ActiveSession = { ...session, viewState };
        dispatch({ type: "SET_SESSION", value: nextSession });
        await refreshUi(nextSession);
        return nextSession;
      }
      const instanceId = await sPlugin.handle.createApp(app.id);
      // 🪦️ See `establishPrimarySession`'s comment above — `programs` is permanently empty now.
      const nextViewState: ViewModel = viewState ?? {
        activeModeId: app.defaultModeId ?? app.modes[0]?.id,
        panelJson: panelJsonFromState(buildSpacePanelState([], [])),
      };
      const nextSession: ActiveSession = { pluginId: sPlugin.handle.pluginId, instanceId, app, viewState: nextViewState };
      dispatch({ type: "SET_SESSION", value: nextSession });
      const seeded = applyFrameworkLayoutSeed(app.defaultLayout, app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      if (appId === landingAppId) {
        openSpaceIdRef.current = null;
        openInstanceIdRef.current = null;
      }
      await refreshUi(nextSession);
      return nextSession;
    },
    [loadedPlugins, refreshUi, session, appLabelsOverlay, hostConfig, landingAppId, uiTerminology, uiLocale],
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
    async (program: SpaceProgramEntry, label?: string, osInstanceId?: string, documentJson?: string, sourceViewState?: ViewModel): Promise<SpacePanelState | null> => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return null;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(sourceViewState ?? session.viewState) ?? buildSpacePanelState([], []);
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (documentJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, documentJson, sourceViewState ?? session.viewState);
        }
        return studioPanelFocusingSpawned(currentPanel, existing);
      }
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      if (documentJson && app) {
        await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, documentJson, sourceViewState ?? session.viewState);
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
    [loadedPlugins, session, syncSpawnedPluginDocument],
  );

  /**
   * 🐚️ Consumes a plugin action's typed `requestedEffects: HostEffect[]` (WS-D's `InvocationResponse`) —
   * replaces the deleted `processPluginOperations` string-matching. The legacy `setDocument`-mirror
   * backbone-write block is gone entirely: document content sync now flows through
   * `openDocument`/`closeDocument`'s worker-backed `DocumentHost` lifecycle, not a per-operation JS mirror.
   */
  const applyHostEffects = useCallback(
    async (effects: readonly HostEffect[], baseSession: ActiveSession, uiScope: UiDirtyScope = { kind: "full" }) => {
      let nextViewState = baseSession.viewState;
      for (const effect of effects) {
        if (effect === "requestSync") continue;
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
        if ("patchWorld3dChrome" in effect) {
          const { selectionJson, vorticesJson, documentSelectedIds, documentHighlightedIds } = effect.patchWorld3dChrome;
          const patch = { selectionJson, vorticesJson };
          const windowInstances = sessionWindowInstances(baseSession.app, extraWindowInstancesRef.current);
          const documentPanelKey = FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
          dispatch({
            type: "SET_WINDOW_UI_BY_WINDOW_ID",
            value: (current) =>
              mergeRecordPreservingIdentity(
                current,
                windowInstances.map((instance) => {
                  const node = current[instance.id];
                  return [instance.id, node ? patchWorld3dChromeOntoNode(node, patch) : node] as const;
                }),
              ),
          });
          dispatch({
            type: "SET_PANEL_UI_BY_KEY",
            value: (current) => {
              const documentNode = current[documentPanelKey];
              if (!documentNode) return current;
              return mergeRecordPreservingIdentity(current, [[documentPanelKey, patchDocumentTreeSelectedIds(documentNode, documentSelectedIds, documentHighlightedIds)]]);
            },
          });
          const cache = uiRefreshCacheRef.current;
          for (const instance of windowInstances) {
            const cached = cache.get(`window:${instance.id}`);
            if (cached?.value) {
              cache.set(`window:${instance.id}`, { hash: cached.hash, value: patchWorld3dChromeOntoNode(cached.value as UiNode, patch) });
            }
          }
          const documentCached = cache.get(`panel:${documentPanelKey}`);
          if (documentCached?.value) {
            cache.set(`panel:${documentPanelKey}`, {
              hash: documentCached.hash,
              value: patchDocumentTreeSelectedIds(documentCached.value as UiNode, documentSelectedIds, documentHighlightedIds),
            });
          }
          continue;
        }
        if ("openDialog" in effect) {
          // 🗨️ Renders from the active `baseSession.app` — dialogs opened by spawned program
          // instances are v1-out-of-scope, mirroring the introduction's active-session-only scope.
          const { dialogId, args } = effect.openDialog;
          if (baseSession.app.dialogs?.some((entry) => entry.id === dialogId)) {
            dispatch({ type: "SET_DIALOG", value: { dialogId, seedArgs: args as Record<string, unknown> | undefined } });
          } else {
            console.error(`[os-shell] openDialog: app ${baseSession.app.id} declares no dialog "${dialogId}"`);
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
            await pluginEntry.handle.loadAppDocumentPack(baseSession.instanceId, packBytes, sprBytes);
          } else if (payload.documentJson && pluginEntry?.handle.loadAppDocument) {
            console.log("[DEBUG] loadDocument for instance", baseSession.instanceId, "bytes", payload.documentJson.length);
            await pluginEntry.handle.loadAppDocument(baseSession.instanceId, payload.documentJson);
          } else {
            console.error("[os-shell] loadDocument: program has no pack/json loader", baseSession.pluginId, Object.keys(payload));
          }
          continue;
        }
        if ("openExternalUrl" in effect) {
          window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
          continue;
        }
        if ("downloadMediaExport" in effect) {
          const { filename, mimeType, data, encoding } = effect.downloadMediaExport;
          downloadMediaExport(filename, mimeType, data, encoding);
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
          const opened = await requestFileOpen(accept || ".spk,.dsl,.ops,application/octet-stream", readAs, multiple);
          if (opened.length > 0) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (pluginEntry) {
              // 📤️ Single-file (multiple absent/false): identical to the pre-multi-select shape, one
              // `handleAction` call with `{payload, name}`. Multi-file: one sequential call per selected
              // file, each extending args with `{index, total}` so the plugin can stage/merge imports.
              await dispatchOpenedFiles(opened, importAction, Boolean(multiple), makeEffectDispatchOne(pluginEntry, baseSession, applyHostEffects));
            }
          }
          continue;
        }
        if ("dispatchAction" in effect) {
          // 🔁️ Self re-dispatch (D2): re-invokes the same plugin instance with `action` after `delayMs`,
          // without blocking the current `applyHostEffects` pass — `setTimeout` (0 is "next tick") fires
          // the follow-up call and feeds its own `requestedEffects` back through `applyHostEffects`
          // recursively, so a plugin can chain several ticks of staged/progressive work (e.g. a
          // multi-pass reconstruction) purely by re-emitting `dispatchAction` from its own handler.
          const { action: dispatchActionId, args: dispatchArgs, delayMs } = effect.dispatchAction;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (pluginEntry) {
            scheduleDispatchAction(dispatchActionId, dispatchArgs as Record<string, unknown> | undefined, delayMs, makeEffectDispatchOne(pluginEntry, baseSession, applyHostEffects));
          }
          continue;
        }
        if ("replayShellCommand" in effect) {
          // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C6/§5 — the
          // `os.directory.*` funnel and the `os.open-artifact`/`os.open-artifact-with` opening relay
          // (§3-B "closes the known gap where `openArtifactWithAppRef` opened an app but no
          // document" — `documentId`/`spaceId` are optional here since the relay side of that gap
          // closure is a different, not-yet-landed lane; every other `replayShellCommand` action id
          // (e.g. `os.setThemeId`'s Backwards replay) has no handler in this lease and is a no-op).
          const { actionId, args } = effect.replayShellCommand;
          const argsRecord = args as Record<string, unknown> | undefined;
          if (actionId.startsWith("os.directory.")) {
            const command = directoryCommandFromAction(actionId, argsRecord);
            if (!command) {
              console.warn("[os-shell] replayShellCommand: unrecognized directory action", actionId);
            } else if (!identityRef.current) {
              console.warn("[os-shell] replayShellCommand: directory command dropped, no signed-in identity", actionId);
            } else {
              const worker = ensureBackboneWorker();
              const requestId = `${actionId}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`;
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-command", requestId, command }) });
            }
          } else if (actionId === "os.open-artifact" || actionId === "os.open-artifact-with") {
            const artifactRef = typeof argsRecord?.artifactRef === "string" ? argsRecord.artifactRef : undefined;
            const pluginId = typeof argsRecord?.pluginId === "string" ? argsRecord.pluginId : undefined;
            const appId = typeof argsRecord?.appId === "string" ? argsRecord.appId : undefined;
            const documentId = typeof argsRecord?.documentId === "string" ? argsRecord.documentId : undefined;
            const spaceId = typeof argsRecord?.spaceId === "string" ? argsRecord.spaceId : undefined;
            const role: AppRole = argsRecord?.role === 0 ? "viewer" : "editor";
            if (artifactRef && pluginId && appId) {
              const dialect = parseDialectCoordinate(artifactRef);
              await openArtifactWithAppRefRef.current({ pluginId, appId }, dialect, role);
              if (documentId) {
                if (spaceId) openSpaceIdRef.current = spaceId;
                // 📇️ `schema` has no general dialect-coordinate → document-schema formula (verified
                // against `s.space`'s own three DIFFERENT id strings — artifact-kind id, dialect
                // coordinate, document schema — none derivable from the others); the one mapping known
                // to this lane is used, `artifactRef` itself is the best-effort fallback for anything
                // else until lane 3-B's opening relay carries a real `schema` field.
                const schema = dialectCoordinate(dialect) === dialectCoordinate(SPACE_INDEX_DIALECT) ? S_SPACE_INDEX_DOCUMENT_SCHEMA : artifactRef;
                await openDocumentRef.current({ documentId, schema });
              }
            } else {
              console.warn("[os-shell] replayShellCommand: os.open-artifact missing artifactRef/pluginId/appId", args);
            }
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
              makeEffectDispatchOne(pluginEntry, baseSession, applyHostEffects),
            );
          }
          continue;
        }
        if ("invokeExtension" in effect) {
          const { extensionId, capability, requestJson, responseAction } = effect.invokeExtension;
          const request = JSON.parse(requestJson) as { operatorId?: string; inputJson?: string; nodeHash?: number };
          const requestingPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          const extensionEntry = loadedPlugins.find((entry) => entry.handle.pluginId === extensionId || entry.manifest.contributions?.some((c) => "extensionId" in c && (c as { extensionId?: string }).extensionId === extensionId));
          if (requestingPlugin && request.operatorId && request.inputJson != null && request.nodeHash != null) {
            try {
              let outputJson = "";
              const invoke = (extensionEntry?.handle as { invoke?: (capability: string, request: Uint8Array | string) => Promise<string | Uint8Array> } | undefined)?.invoke;
              if (typeof invoke === "function" && extensionEntry) {
                const raw = await invoke(capability, requestJson);
                outputJson = typeof raw === "string" ? raw : new TextDecoder().decode(raw);
                console.log("[DEBUG] invokeExtension via handle.invoke", { extensionId, capability, operatorId: request.operatorId, nodeHash: request.nodeHash });
              } else {
                console.warn("[DEBUG] invokeExtension: extension handle missing invoke; returning empty output", { extensionId, capability });
              }
              await makeEffectDispatchOne(requestingPlugin, baseSession, applyHostEffects)(responseAction, {
                nodeHash: request.nodeHash,
                outputJson,
              });
            } catch (error) {
              console.warn("[os-shell] invokeExtension failed", { extensionId, capability, error });
            }
          }
          continue;
        }
        if ("spawnPluginInstance" in effect) {
          const { pluginId, appId, osInstanceId, label, documentJson } = effect.spawnPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildSpacePanelState([], []);
          // 🪦️ See `establishPrimarySession`'s comment above — the `manifest.workflows` fallback source is dead; `catalog` is `currentPanel.programs` or empty.
          const catalog = currentPanel.programs.length > 0 ? currentPanel.programs : [];
          const program = catalog.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? catalog.find((entry) => entry.pluginId === pluginId);
          if (program) {
            // 🪟️ Fold spawn into `nextViewState` — a separate SET_SESSION would be clobbered by the
            // final write below and leave the shell stuck on the studio surface.
            const nextPanel = await ensureSpawnedPlugin(program, label, osInstanceId, documentJson, nextViewState);
            if (nextPanel) nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
          }
          continue;
        }
        if ("openPluginInstance" in effect) {
          const { pluginId, appId, osInstanceId } = effect.openPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildSpacePanelState([], []);
          // 🪦️ See `establishPrimarySession`'s comment above — the `manifest.workflows` fallback source is dead; `catalog` is `currentPanel.programs` or empty.
          const catalog = currentPanel.programs.length > 0 ? currentPanel.programs : [];
          const program = catalog.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? catalog.find((entry) => entry.pluginId === pluginId);
          if (program) {
            // 🪟️ Fold focus into `nextViewState` so the final SET_SESSION keeps `activeSpawnedId`
            // (opening a workflow node depends on this — otherwise nothing appears to happen).
            const nextPanel = await ensureSpawnedPlugin(program, undefined, osInstanceId, undefined, nextViewState);
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
      const isSpawnedPluginSession = hostMode && session && baseSession.pluginId !== session.pluginId;
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return nextSession;
          if (isSpawnedPluginSession) return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
          if (current.instanceId !== nextSession.instanceId) return current;
          // 🐢️ Preserve `current`'s identity when the viewState didn't actually change — otherwise every
          // action mints a new `session` object, which cascades into a new `onAction` identity, which
          // busts every memo keyed on it (windows, panels, the boot-refresh effect below) even when
          // nothing about the session changed.
          return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
        },
      });
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState, uiScope);
      } else if (session?.instanceId === nextSession.instanceId || baseSession.instanceId === nextSession.instanceId) {
        await refreshUi(nextSession, uiScope);
      }
    },
    [clearAllWindowUtilities, ensureSpawnedPlugin, loadedPlugins, navigateHistory, refreshSpawnedUi, refreshUi, session, setActiveUtilityForWindow, hostMode],
  );

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
        // (optionally `/instances/{id}`) opens the workflow studio (`hostConfig.hostAppId`, the
        // pre-existing behaviour every bare `/spaces/{id}` used to trigger). `parseShellRoute` (owned by
        // `ShellHelpers/🟦️component.tsx`, outside this lane's lease) has no concept of a `/studio`
        // segment, so it's matched locally here first — `parseShellRoute` itself is never edited, and
        // its own existing route classification (and tests) stay exactly as they were.
        const studioMatch = /^\/spaces\/([^/]+)\/studio(?:\/instances\/([^/]+))?$/.exec(path);
        const route = studioMatch ? ({ kind: "space" as const, spaceId: studioMatch[1]!, instanceId: studioMatch[2] } as const) : parseShellRoute(path);
        const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId)?.handle;
        if (!sPlugin) return;
        if (route.kind === "landing") {
          openSpaceIdRef.current = null;
          openInstanceIdRef.current = null;
          if (currentSession.app.id !== hostConfig.landingAppId) await switchToManagedApp(hostConfig.landingAppId, preservedViewState);
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
          if (!spaceApp) {
            console.warn("[os-shell] applyShellUri: no app registered for dialect", dialectCoordinate(SPACE_INDEX_DIALECT), "— s.space (lane 2-B) not loaded yet");
            return;
          }
          // 🔁️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-I — idempotency
          // guard mirroring `studioChanged` below: without it, ANY re-render that mints a new
          // `applyShellUri` identity (e.g. `switchToManagedApp` depending on `session`, which
          // `openDocumentRef.current` below itself updates) re-fires this whole branch for the SAME
          // already-open space, tearing the document's sync session down and reopening it in a tight
          // loop — observed live as dozens of WS open/close cycles plus a `Maximum call stack size
          // exceeded` inside the 30s STEP 2 budget (`🧪️4-i-collab-e2e-run3.txt`), never previously
          // exercised because no earlier lane's fixes let a hard navigation to `/spaces/{id}` reach here.
          const spaceIndexAlreadyOpen = openSpaceIdRef.current === spaceId && currentSession.app.id === spaceApp.id;
          openSpaceIdRef.current = spaceId;
          openInstanceIdRef.current = null;
          if (spaceIndexAlreadyOpen) return;
          const spaceSession = currentSession.app.id === spaceApp.id ? currentSession : await switchToManagedApp(spaceApp.id, preservedViewState);
          if (!spaceSession) return;
          await openDocumentRef.current({ documentId: S_SPACE_INDEX_DOCUMENT_ID, schema: S_SPACE_INDEX_DOCUMENT_SCHEMA });
          return;
        }
        // 🧭️ Pin the route studio id before the async app switch so the boot example effect cannot
        // race-navigate to `/spaces/demo` while `switchToManagedApp` is still awaiting.
        const studioChanged = openSpaceIdRef.current !== spaceId;
        openSpaceIdRef.current = spaceId;
        const studioSession = currentSession.app.id === hostConfig.hostAppId ? currentSession : await switchToManagedApp(hostConfig.hostAppId, preservedViewState);
        if (!studioSession) return;
        const studioControllerId = studioSession.app.controllerId;
        if (studioChanged) {
          openInstanceIdRef.current = null;
          console.log("[DEBUG] applyShellUri openSpace", spaceId);
          const openResponse = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "openSpace", args: { spaceId } }), studioSession.viewState);
          await applyHostEffects(openResponse.requestedEffects ?? [], studioSession, resolveUiDirtyScope(openResponse.uiScope));
        }
        if (openInstanceIdRef.current === (instanceId ?? null)) return;
        openInstanceIdRef.current = instanceId ?? null;
        if (instanceId) {
          const response = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "openInstance", args: { instanceId } }), studioSession.viewState);
          await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
        } else {
          const response = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "closeFocusedInstance" }), studioSession.viewState);
          const currentPanel = parsePanelState(studioSession.viewState) ?? buildSpacePanelState([], []);
          updateSpacePanel(buildSpacePanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, undefined));
          await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
        }
      } finally {
        applyShellUriDepthRef.current -= 1;
      }
    },
    [applyHostEffects, loadedPlugins, refreshUi, hostConfig, switchToManagedApp, updateSpacePanel],
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

  /**
   * 🧵️ `openDocument(ref, bindings?)` — replaces `attachSyncBackbone`'s URI-string mirror. Spins up
   * (or reuses) `🟦️backbone-🟦️worker.ts`, tells it to open the document, subscribes to its postMessage
   * events, and calls the plugin instance's `attachBackbone`/`loadAppDocument` WIT-exported methods
   * (WS-D) so the plugin-side store starts pumping through the same logical channel. The
   * `actor://<documentId>` uri mirrors `framework/sync`'s `ChannelBackbone::pair` convention on the
   * Rust side.
   *
   * 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C3/§2 — `bindings`
   * omitted (as opposed to `attachSyncBackbone`'s always-explicit array, which stays a working manual
   * override with zero change here) defaults to
   * `[{kind:"hub", baseUrl, spaceId, token, surface}, {kind:"folder", path: "${dataDir}/spaces/${spaceId}"}]`
   * when an identity is resolved and the current route has an open space, else `[{kind:"folder", …}]`
   * (identity or dataDir missing), else `[]` (no route space — the OS config/home documents stay
   * folder-only per contract §C3 and never call `openDocument` with omitted bindings for this reason).
   *
   * Full loop note: this wires the main-thread half of the contract. The remaining hop — the
   * sandboxed plugin's own `backbone-send`/`backbone-poll` WIT host-import calls relaying through its
   * dedicated program worker, through this main thread, into `🟦️backbone-🟦️worker.ts` — is
   * `framework/os/dev/script.ts`'s `pluginWorkerSource` responsibility (dev workflow, deferred
   * per this session's priority order if not otherwise completed); see that file's own notes.
   */
  const openDocument = useCallback(
    async (ref: { readonly documentId: string; readonly schema: string }, bindings?: readonly PersistenceBinding[]) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      if (!plugin) return;
      const worker = ensureBackboneWorker();
      const resolvedBindings: readonly PersistenceBinding[] =
        bindings ??
        (() => {
          const spaceId = openSpaceIdRef.current;
          const dataDir = hubEnv?.dataDir;
          const folder: PersistenceBinding[] = spaceId && dataDir ? [{ kind: "folder", path: `${dataDir}/spaces/${spaceId}` }] : [];
          const currentIdentity = identityRef.current;
          if (!currentIdentity || !spaceId) return folder;
          const surface = targetSession.app.dialect ? canonicalSurfaceId(targetSession.app.dialect, targetSession.app.role) : undefined;
          return [{ kind: "hub", baseUrl: currentIdentity.hubBaseUrl, spaceId, token: currentIdentity.sessionToken, surface }, ...folder];
        })();
      openDocumentSessionsRef.current.set(ref.documentId, { session: targetSession, plugin });
      // 🐚️ Registers THIS shell as the route for this document's outbound backbone bytes before the
      // plugin can possibly emit any (attachBackbone below) — see `relayPluginBackboneMessage`'s doc.
      pluginBackboneRouteUnregistersRef.current.get(ref.documentId)?.();
      pluginBackboneRouteUnregistersRef.current.set(ref.documentId, registerPluginBackboneRoute(ref.documentId, relayPluginBackboneMessage));
      const request: BackboneWorkerRequest = {
        kind: "open",
        documentId: ref.documentId,
        schema: ref.schema,
        bindings: resolvedBindings,
        watchExternal: true,
        actor: shellActorIdRef.current,
      };
      worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
      const uri = `actor://${ref.documentId}`;
      if (plugin.attachBackbone) await plugin.attachBackbone(targetSession.instanceId, uri);
      dispatch({ type: "SET_SYNC_BACKBONE_URI", value: uri });
      dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
    },
    [loadedPlugins, relayPluginBackboneMessage, resolveSyncTargetSession, hubEnv],
  );
  openDocumentRef.current = openDocument;

  const closeDocument = useCallback((documentId: string) => {
    const entry = openDocumentSessionsRef.current.get(documentId);
    if (entry?.plugin.detachBackbone) void entry.plugin.detachBackbone(entry.session.instanceId);
    openDocumentSessionsRef.current.delete(documentId);
    pluginBackboneRouteUnregistersRef.current.get(documentId)?.();
    pluginBackboneRouteUnregistersRef.current.delete(documentId);
    const request: BackboneWorkerRequest = { kind: "close", documentId };
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, []);

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
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], []);
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
    [loadedPlugins, session, updateSpacePanel],
  );

  const onAction = useCallback(
    (action: ActionDescriptor) => {
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
      if (tutorialPlayingRef.current && !tutorialDrivenRef.current) {
        dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
        dispatch({ type: "SET_TUTORIAL_DEVIATED", value: true });
      }

      // ⏺️ Recorder tap: annotational-only capture (see `TutorialTracks.events` doc comment) — never
      // re-dispatched on playback. Skips navigation/introduction/tutorial-control actions (noise, or
      // meaningless to replay) and anything the director itself just dispatched.
      if (tutorialRecordingRef.current && !tutorialDrivenRef.current) {
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
          const viewState: ViewModel = { ...session.viewState, activeUtilityId: next ?? undefined, activeToolId: next ? undefined : activeToolIdRef.current ?? undefined, windowId };
          const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { utilityId: next } };
          void program
            .handleAction(session.instanceId, encodeWindowActionInvocation({ ...session, viewState }, forwarded, extraWindowInstancesRef.current, windowId), viewState)
            .then((response) => {
              applyHistoryPatch(response.historyPatch);
              return applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, resolveUiDirtyScope(response.uiScope));
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
        if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "tool" && interaction.on.id === next);
        const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId);
        const program = pluginEntry?.handle;
        if (program) {
          const viewState: ViewModel = { ...session.viewState, activeToolId: next ?? undefined, activeUtilityId: next ? undefined : session.viewState.activeUtilityId };
          const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { toolId: next } };
          void program
            .handleAction(session.instanceId, encodeWindowActionInvocation({ ...session, viewState }, forwarded, extraWindowInstancesRef.current, activeWindowIdRef.current ?? undefined), viewState)
            .then((response) => {
              applyHistoryPatch(response.historyPatch);
              return applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, resolveUiDirtyScope(response.uiScope));
            })
            .catch((toolError) => console.error("[DEBUG] setActiveTool failed", toolError));
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
        const currentPanel = parsePanelState(session.viewState);
        const program = currentPanel?.programs.find((entry) => entry.pluginId === pluginId);
        if (program) void spawnProgram(program);
        return;
      }

      if (hostMode && action.controllerId === hostControllerId && action.action === "setActivePanelTab") {
        const tabId = typeof action.args === "object" && action.args != null && "tabId" in action.args ? String((action.args as { tabId?: string }).tabId ?? hostCatalogueTabId ?? "") : (hostCatalogueTabId ?? "");
        const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], []);
        updateSpacePanel(buildSpacePanelState(currentPanel.programs, currentPanel.spawnedApps, tabId, currentPanel.activeSpawnedId));
        return;
      }

      const targetSession =
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
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      if (!plugin) return;

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
      const dispatchViewState = injectActiveUtility(
        {
          ...targetSession.viewState,
          windowId: dispatchWindowId,
          windowInstances: sessionWindowInstances(targetSession.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        },
        dispatchWindowId,
      );
      const declaredAction = targetSession.app.windowKinds.some((kind) => (kind.actions ?? []).some((entry) => entry.id === action.action));
      if (!declaredAction && !FRAMEWORK_RESERVED_ACTION_IDS.has(action.action)) {
        console.warn("[DEBUG] skipping undeclared action", action.action, targetSession.app.id);
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

      const interactiveAction = action.action !== "suggestionsTick" && action.action !== "fillBuildTick";
      if (interactiveAction) beginInteractivePluginAction();
      return plugin
        .handleAction(targetSession.instanceId, encodeWindowActionInvocation({ ...targetSession, viewState: dispatchViewState }, action, extraWindowInstancesRef.current, dispatchWindowId), dispatchViewState)
        .then((response) => {
          applyHistoryPatch(response.historyPatch);
          return applyHostEffects(response.requestedEffects ?? [], { ...targetSession, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope));
        })
        .catch((actionError) => {
          if (isViewerReadOnlyFault(actionError)) {
            showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
            return;
          }
          if (isMutationRejectedFault(actionError)) {
            showMutationRejectedNotice((actionError as SemioFaultError).fault);
            return;
          }
          console.error("[DEBUG] action failed", action.action, action.args, actionError);
        })
        .finally(() => {
          if (interactiveAction) endInteractivePluginAction();
        });
    },
    [
      applyHostEffects,
      applyHistoryPatch,
      attachSyncBackbone,
      clearAllWindowUtilities,
      detachSyncBackbone,
      injectActiveUtility,
      loadedPlugins,
      panel,
      session,
      setActiveUtilityForWindow,
      spawnProgram,
      hostMode,
      syncBackboneUri,
      syncDraftPath,
      updateSpacePanel,
      hostControllerId,
      landingControllerId,
      hostCatalogueTabId,
      completeIntroductionInteraction,
      primaryPluginId,
      reloadPlugin,
      uninstallPlugin,
      pluginSupervisorById,
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

  //#region 🎥️TutorialOrchestration
  /** ⏱️ Real-time throttle for the director's UI/document/event application (~10Hz) — camera stays
   * smooth every clock tick regardless (see the `subscribe` callback below). */
  const TUTORIAL_DIRECTOR_TICK_MS = 90;

  const activeTutorial = useMemo(() => activeTutorials.find((tutorial) => tutorial.id === activeTutorialId) ?? null, [activeTutorials, activeTutorialId]);

  const tutorialClockRef = useRef<TutorialClock | null>(null);
  if (!tutorialClockRef.current) tutorialClockRef.current = createTutorialClock(activeTutorial?.durationMs ?? 0);
  const tutorialClock = tutorialClockRef.current;
  useEffect(() => () => tutorialClockRef.current?.dispose(), []);
  useEffect(() => {
    tutorialClock.setDurationMs(activeTutorial?.durationMs ?? 0);
  }, [activeTutorial?.durationMs, tutorialClock]);
  useEffect(() => {
    tutorialClock.setRate(tutorialRate);
  }, [tutorialRate, tutorialClock]);
  useEffect(() => {
    if (tutorialPlaying) tutorialClock.play();
    else tutorialClock.pause();
  }, [tutorialPlaying, tutorialClock]);

  const uiBridgeCtxRef = useRef<TutorialUiBridgeContext>({ session, appLabelsOverlay, terminology: uiTerminology, locale: uiLocale });
  uiBridgeCtxRef.current = { session, appLabelsOverlay, terminology: uiTerminology, locale: uiLocale };

  /** ⏱️ Playhead (ms) the director/seek last applied document/UI tracks up to — the "from" side of the
   * next `tutorialSlice(def, from, to)` call. Reset to 0 on sandbox (re)start. */
  const tutorialLastAppliedMsRef = useRef(0);
  /** 🎬️ Sandboxed-out live document (full `DocumentEnvelope` JSON), restored on stop/exit. */
  const tutorialDocumentSnapshotRef = useRef<string | null>(null);

  // 🎬️ Sandbox start/stop (design point 3): on activation, snapshot the live document, load `base`, apply
  // `base.ui`/`base.cameras`, and seek the clock to 0; on deactivation, restore the snapshot.
  const prevActiveTutorialIdRef = useRef<string | null>(null);
  useEffect(() => {
    const previousId = prevActiveTutorialIdRef.current;
    prevActiveTutorialIdRef.current = activeTutorialId;
    if (previousId === activeTutorialId || !session) return;
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    if (activeTutorialId) {
      const def = activeTutorials.find((tutorial) => tutorial.id === activeTutorialId);
      if (!def) return;
      tutorialDrivenRef.current = true;
      void (async () => {
        try {
          if (plugin.readAppDocument) tutorialDocumentSnapshotRef.current = await plugin.readAppDocument(session.instanceId);
        } catch (snapshotError) {
          console.error("[DEBUG] tutorial sandbox snapshot failed", snapshotError);
        }
        try {
          if (def.base.documentJson && plugin.loadAppDocument) await plugin.loadAppDocument(session.instanceId, def.base.documentJson);
          else if (def.base.exampleId) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: def.base.exampleId });
        } catch (loadError) {
          console.error("[DEBUG] tutorial base document load failed", loadError);
        }
        applyTutorialUiSnapshotToShell(dispatch, def.base.ui, uiBridgeCtxRef.current);
        for (const cameraKeyframe of def.base.cameras) getTutorialCameraDriver(cameraKeyframe.windowId)?.set(cameraKeyframe.camera);
        tutorialLastAppliedMsRef.current = 0;
        tutorialClock.seek(0);
        await refreshUi(session, { kind: "full" });
        tutorialDrivenRef.current = false;
      })();
    } else if (previousId) {
      tutorialDrivenRef.current = true;
      void (async () => {
        try {
          const snapshotJson = tutorialDocumentSnapshotRef.current;
          if (snapshotJson && plugin.loadAppDocument) await plugin.loadAppDocument(session.instanceId, snapshotJson);
        } catch (restoreError) {
          console.error("[DEBUG] tutorial sandbox restore failed", restoreError);
        }
        tutorialDocumentSnapshotRef.current = null;
        await refreshUi(session, { kind: "full" });
        tutorialDrivenRef.current = false;
      })();
    }
  }, [activeTutorialId, activeTutorials, session, loadedPlugins, tutorialClock, refreshUi]);

  /** 🎬️ Applies every entry of one `TutorialSlice` (a director tick or a seek span) onto the live
   * session — UI changes first, then document-track entries through the plugin bridge: `Edit` via
   * `applyMutations` (forward/backward per `slice.forward`), `Load` via `loadAppDocument`,
   * `Undo`/`Redo`/`Checkpoint`/`CheckoutCheckpoint`/`SwitchAlternative` via the SAME History-action
   * `onAction` funnel the app's own undo/redo buttons dispatch through (never a bespoke channel) — then
   * pulses any annotational event's target element via the existing `celebrateElements` vocabulary. */
  const applyTutorialSliceToShell = useCallback(
    async (slice: TutorialSlice, activeSession: ActiveSession) => {
      for (const change of slice.uiChanges) applyTutorialUiChangeToShell(dispatch, change, uiBridgeCtxRef.current);
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === activeSession.pluginId)?.handle;
      let documentTouched = false;
      for (const documentEvent of slice.document) {
        const kind: TutorialDocumentEventKind = documentEvent.kind;
        if (kind.kind === "edit") {
          documentTouched = true;
          const mutations = slice.forward ? kind.forwards : kind.backwards;
          if (plugin?.applyMutations) await plugin.applyMutations(activeSession.instanceId, encodeMutationEnvelopesPack(mutations));
        } else if (kind.kind === "load") {
          documentTouched = true;
          const documentJson = slice.forward ? kind.documentJson : kind.previousJson;
          if (plugin?.loadAppDocument) await plugin.loadAppDocument(activeSession.instanceId, documentJson);
        } else if (kind.kind === "undo") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "undo" : "redo" });
        } else if (kind.kind === "redo") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "redo" : "undo" });
        } else if (kind.kind === "checkpoint") {
          if (slice.forward) onActionRef.current({ controllerId: activeSession.app.controllerId, action: "commitCheckpoint" });
        } else if (kind.kind === "checkoutCheckpoint") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: "checkoutCheckpoint", args: { checkpointId: kind.checkpointId } });
        } else if (kind.kind === "switchAlternative") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: "switchAlternative", args: { alternativeId: kind.alternativeId } });
        }
      }
      for (const event of slice.events) {
        const kind = event.kind;
        const targetId = kind.kind === "action" ? kind.action : kind.kind === "command" ? kind.command : undefined;
        if (targetId && scope.rootRef.current) celebrateElements(elementIdSelector(targetId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      }
      if (documentTouched) await refreshUi(activeSession, { kind: "full" });
    },
    [loadedPlugins, refreshUi],
  );

  // 🎬️ Director: one subscription to the clock's rAF-driven ticks. Camera interpolation applies every
  // tick (smooth); UI/document/event application throttles to `TUTORIAL_DIRECTOR_TICK_MS`.
  useEffect(() => {
    const def = activeTutorial;
    if (!def || !session) return;
    let lastHeavyTickAt = 0;
    const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
    const unsubscribe = tutorialClock.subscribe(() => {
      const t = tutorialClock.getTimeMs();
      for (const windowId of cameraWindowIds) {
        const pose = tutorialCameraAt(def, windowId, t);
        if (pose) getTutorialCameraDriver(windowId)?.set(pose);
      }
      if (!tutorialClock.isPlaying()) return;
      const now = performance.now();
      if (now - lastHeavyTickAt < TUTORIAL_DIRECTOR_TICK_MS) return;
      lastHeavyTickAt = now;
      const from = tutorialLastAppliedMsRef.current;
      if (from === t) return;
      const slice = tutorialSlice(def, from, t);
      tutorialLastAppliedMsRef.current = t;
      tutorialDrivenRef.current = true;
      void applyTutorialSliceToShell(slice, session).finally(() => {
        tutorialDrivenRef.current = false;
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
      const clamped = Math.min(def.durationMs, Math.max(0, ms));
      const from = tutorialLastAppliedMsRef.current;
      tutorialDrivenRef.current = true;
      void (async () => {
        applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, clamped), uiBridgeCtxRef.current);
        const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
        const slice = tutorialSlice(def, from, clamped);
        let documentTouched = false;
        for (const documentEvent of slice.document) {
          const kind: TutorialDocumentEventKind = documentEvent.kind;
          if (kind.kind === "edit") {
            documentTouched = true;
            const mutations = slice.forward ? kind.forwards : kind.backwards;
            if (plugin?.applyMutations) await plugin.applyMutations(session.instanceId, encodeMutationEnvelopesPack(mutations));
          } else if (kind.kind === "load") {
            documentTouched = true;
            const documentJson = slice.forward ? kind.documentJson : kind.previousJson;
            if (plugin?.loadAppDocument) await plugin.loadAppDocument(session.instanceId, documentJson);
          }
          // 🚧️ Undo/Redo/Checkpoint/CheckoutCheckpoint/SwitchAlternative crossings mid-seek are an honest
          // scope cut here (replaying a crossed history op out of its natural live-dispatch order is
          // ambiguous without more VCS-side infrastructure) — the director's per-tick forward playback
          // above still applies them correctly; only a large scrub jumping OVER one of these entries misses it.
        }
        const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
        for (const windowId of cameraWindowIds) {
          const pose = tutorialCameraAt(def, windowId, clamped);
          if (pose) getTutorialCameraDriver(windowId)?.set(pose);
        }
        tutorialLastAppliedMsRef.current = clamped;
        tutorialClock.seek(clamped);
        if (documentTouched) await refreshUi(session, { kind: "full" });
        console.log("[DEBUG] tutorial rebuild", { atMs: clamped });
        tutorialDrivenRef.current = false;
      })();
    },
    [activeTutorial, session, loadedPlugins, tutorialClock, refreshUi],
  );

  /** ▶️ Play/pause toggle — the deviation-converge path (design point 6): snaps document+UI to the
   * composed target at the current playhead, tweens the camera over `TUTORIAL_CONVERGE_MS` (real-time,
   * rate-independent) from each window's LIVE pose to its target pose, then resumes the clock. */
  const playPauseTutorial = useCallback(() => {
    if (!activeTutorial) return;
    if (tutorialPlaying) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
      return;
    }
    if (tutorialDeviated && session) {
      const def = activeTutorial;
      const atMs = tutorialClock.getTimeMs();
      tutorialDrivenRef.current = true;
      applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, atMs), uiBridgeCtxRef.current);
      const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
      const startPoseByWindow = new Map<string, TutorialCameraState>();
      for (const windowId of cameraWindowIds) {
        const live = getTutorialCameraDriver(windowId)?.get();
        if (live) startPoseByWindow.set(windowId, live);
      }
      const startedAt = performance.now();
      const tween = (now: number) => {
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
          tutorialDrivenRef.current = false;
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
      if (!activeTutorials.some((tutorial) => tutorial.id === tutorialId)) return;
      dispatch({ type: "SET_TUTORIAL", value: tutorialId });
    },
    [activeTutorials],
  );
  const stopTutorial = useCallback(() => {
    dispatch({ type: "SET_TUTORIAL", value: null });
  }, []);

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
    void (async () => {
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      let documentJson: string | null = null;
      try {
        if (plugin?.readAppDocument) documentJson = await plugin.readAppDocument(session.instanceId);
      } catch (captureError) {
        console.error("[DEBUG] tutorial recorder base capture failed", captureError);
      }
      tutorialRecorderRef.current = new TutorialRecorder(captureTutorialUiSnapshot(shellStateRef.current, session), documentJson);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: true });
    })();
  }, [session, loadedPlugins]);

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
   * ONE plugin session mounted at a time (`session`/`switchToManagedApp`), so only whichever of
   * home/studio/space is actually live right now can receive it.
   *
   * w4-h root-cause fix #1: every `command_from_action` implementation on the Rust side
   * (`home/…/✏️editor/🦀️component.rs`, `space/…/✏️editor/🦀️component.rs`) reads the batch as a
   * `eventsJson: string` field (`args.get("eventsJson")` → `.as_str()` → `serde_json::from_str`), never
   * a raw `events` array — sending `{ events }` left `.get("eventsJson")` finding nothing and silently
   * falling back to `"[]"` on every call, so the fold ran on zero events every single time regardless
   * of whether the live broadcast or the command-result round trip delivered the real payload. Proven
   * live: the collab-e2e harness's new console capture shows the WS frame with the real
   * `space.created` event arriving, yet no row ever appeared — a data-shape bug, not a thrown error,
   * which is why neither `pageerror` nor any `console.error` ever caught it.
   *
   * w4-h root-cause fix #2: `hostConfig.landingAppId`/`hostAppId` are the plugin's OWN Cargo.toml
   * metadata aliases (`host = { landing = "home", shell = "studio" }`, a human-readable nickname pair
   * the registry generator carries through verbatim) — NEVER the real canonical `app.id` a mounted
   * session actually carries (`s.space.home@1/*#editor`, `s.space.studio@1/*#editor` — dialect-derived,
   * confirmed via `engine::space::component::tests::space_manifest_uses_studio_app_id`). Comparing
   * `current.app.id` against the raw alias string can never be true; this file's OWN `landingApp`/
   * `hostApp` (a few lines up, `hostPlugin?.manifest.apps.find(app => app.id === hostConfig.landingAppId
   * /hostAppId)`) already exist to bridge alias → real app object (`landingApp` masks the same dead
   * comparison with a `?? manifest.apps[0]` fallback that happens to land on Home since it's the
   * plugin's first-registered app; `hostApp` has no such fallback and is consequently always
   * `undefined` today — a separate, pre-existing, wider bug this lane's lease does not cover fixing).
   * Comparing against the resolved OBJECTS' `.id` instead of the raw alias strings is what actually
   * works; proven live via a temporary debug log (`🧪️4-h-collab-e2e-run3.txt`): `isHome` was `false` on
   * every single invocation even though `currentAppId` (`s.space.home@1/*#editor`) and the fold's own
   * event payload were both correct. */
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
    let publishing = false;
    const presenceIdentity = presenceClientIdentity(ephemeral, identityRef.current ? { clientId: shellActorIdRef.current, name: identityRef.current.displayName } : undefined);
    const beat = async () => {
      const worker = backboneWorkerRef.current;
      if (!worker || publishing) return;
      publishing = true;
      try {
        for (const [documentId, entry] of openDocumentSessionsRef.current) {
          const snapshot = await entry.plugin.ephemeralSnapshot?.(entry.session.instanceId);
          const request: BackboneWorkerRequest = {
            kind: "send",
            documentId,
            message: {
              kind: "presenceHeartbeat",
              peer: {
                actor: shellActorIdRef.current,
                label: presenceIdentity.name,
                presencePack: snapshot?.presence,
                connectedAtMs: presenceConnectedAtMsRef.current,
                cursor: presenceCursorRef.current,
                viewport: { x: window.scrollX, y: window.scrollY, zoom: window.devicePixelRatio || 1 },
              },
            },
          };
          worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
        }
      } finally {
        publishing = false;
      }
    };
    void beat();
    const timer = window.setInterval(() => void beat(), PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => window.clearInterval(timer);
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

  //#region 💾️ uiPrefs persistence (skips writes for any locked preference; an ephemeral brand's
  // `scope.storage` is already an in-memory port, so the writes below are harmless there too — no more
  // `ephemeral` branch needed to skip them outright)
  useEffect(() => {
    if (!locks.appearance) writeStoredUiChromeAppearance(scope.storage, uiAppearance);
    writeStoredUiChromeLayout(scope.storage, uiLayout);
    writeStoredUiDriverId(scope.storage, uiDriverId);
    writeStoredUiCustomDrivers(scope.storage, uiCustomDrivers);
    writeStoredUiKeybindingOverrides(scope.storage, uiKeybindingOverrides);
    if (!locks.locale) writeStoredUiChromeLocale(scope.storage, uiLocale);
    // 🐚️ This shell's own i18next instance (not the shared `uiI18n` singleton) — and its own root's
    // `lang` attribute; `document.documentElement.lang` stays reserved for the page-owning case.
    void scope.i18n.changeLanguage(uiLocale);
    if (scope.ownsPage) {
      if (typeof document !== "undefined") document.documentElement.lang = uiLocale;
    } else if (scope.rootRef.current) {
      scope.rootRef.current.lang = uiLocale;
    }
    if (!locks.terminology) writeStoredUiChromeTerminology(scope.storage, uiTerminology);
    // 🐚️ `setActiveUiTheme` is page-global (writes `document.documentElement`'s CSS vars) — correct only
    // for the page-owning shell. A co-mounted embedded shell paints its own theme tokens onto its own
    // `.semio-scope` root instead, via `applyUiThemeToRoot`, so two shells with different `themeId` locks
    // never fight over the same document-wide tokens.
    if (scope.ownsPage) {
      setActiveUiTheme(uiTheme);
    } else if (scope.rootRef.current) {
      applyUiThemeToRoot(scope.rootRef.current, uiTheme);
    }
    if (!locks.themeId) {
      writeStoredUiChromeThemeSnapshot(scope.storage, uiTheme);
      writeStoredUiChromeThemeId(scope.storage, uiThemeId);
    }
    writeStoredUiCustomThemes(scope.storage, uiCustomThemes);
  }, [uiAppearance, uiLayout, uiDriverId, uiCustomDrivers, uiKeybindingOverrides, uiLocale, uiTerminology, uiTheme, uiThemeId, uiCustomThemes, locks, scope]);

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
      const seeded = applyFrameworkLayoutSeed(layout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale);
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
            const seeded = applyFrameworkLayoutSeed(layout, current.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale);
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

  const handleTemplateDrop = useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      if (!session) return;
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      const projectionSpec = decodeWorldProjectionTemplateId(payload.templateId);
      if (projectionSpec) registerPendingWorldProjection(instanceId, projectionSpec);
      const title = projectionSpec ? worldProjectionSpecLabel(projectionSpec) : resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale));
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
            resolveFrameworkLayoutSeed(session.app.defaultLayout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale).modeLayout;
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
    windowKinds: session?.app.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale)) })) ?? [],
    builtinLayouts: session?.app.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, extraWindowInstances, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    namedLayoutStore,
  });
  displayHostRef.current = displayHost;

  //#region 🔖️SurfaceRoles
  /** 👁️✏️ `(dialect, role) -> AppRef[]`, contract freeze §3 — built fresh from every loaded plugin's
   * manifest. `AppRouter.build` throws on a genuine authoring conflict (`surface.conflict`/
   * `surface.contribution-not-permitted`); that's a real defect in the loaded plugin set, not
   * something a session should crash over, so it's caught and logged, leaving "Open with…"/Settings
   * empty rather than the whole shell. */
  const appRouter = useMemo((): AppRouter | null => {
    try {
      return AppRouter.build(loadedPlugins.map((entry): AppRouterManifest => ({ pluginId: entry.handle.pluginId, apps: entry.manifest.apps as unknown as Record<string, unknown>[], artifactKinds: entry.manifest.artifactKinds, dependencies: entry.manifest.dependencies })));
    } catch (buildError) {
      console.error("[DEBUG] AppRouter.build failed", buildError);
      return null;
    }
  }, [loadedPlugins]);
  const pluginLabelById = useMemo(() => new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.manifest.label || entry.handle.pluginId])), [loadedPlugins]);
  /** 👁️✏️ Every dialect any loaded app declares — read straight off `AppDefinition.dialect`
   * (contract freeze §1), never inferred from a surface id string. Feeds the Settings
   * `SettingsDefaultApps` table; `appRouter` itself has no public "every registered dialect"
   * accessor (by design — it's addressed one `(dialect, role)` pair at a time). */
  const knownDialects = useMemo((): readonly ArtifactDialect[] => {
    const byCoordinate = new Map<string, ArtifactDialect>();
    for (const entry of loadedPlugins) for (const app of entry.manifest.apps) if (app.dialect) byCoordinate.set(dialectCoordinate(app.dialect), app.dialect);
    return [...byCoordinate.values()];
  }, [loadedPlugins]);

  /** 🎚️ Client-side fold of `os.config.opening` (contract freeze §4) — event-sourced the same way
   * the host materializes it (`foldOpeningPreferences`), never a mutated map. There is no host
   * readback call on {@link AppChannelClient} yet (only the two write commands), so this mirror is
   * advanced ONLY by this shell's own `setDefaultApp`/`clearDefaultApp` calls below — a pin made by
   * another session/shell is not reflected here until that gap closes. See `📓️w1-c-report.md`. */
  const [openingPreferences, setOpeningPreferences] = useState<OpeningPreferences>(EMPTY_OPENING_PREFERENCES);
  const pinnedAppFor = useCallback((dialect: ArtifactDialect, role: AppRole): AppRef | undefined => openingPreferences.defaults.find((entry) => dialectCoordinate(entry.dialect) === dialectCoordinate(dialect) && entry.role === role)?.app, [openingPreferences]);

  /** 👁️✏️ `PluginRuntime`'s `PluginWasmHandle` wraps the raw `exchange` ABI behind typed methods —
   * `transactionPrepare`/`transactionCommit`/`transactionUndo`/`transactionRedo` and (as of ticket
   * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` lane K2)
   * `setMergePolicy`/`resolveConflict`/`readConflicts` are wrapped this way (`adaptPluginHandle`,
   * `PluginRuntime/🟦️component.tsx`), each internally riding its own `AppChannelClient` — see
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
   * (`adaptPluginHandle`, `PluginRuntime/🟦️component.tsx`), so no feature-detection is needed. */
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

  /** ⚖️ `ChromePanels`' Conflicts panel Accept/Discard — forwards `AppCommand::ResolveConflict`
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

  /** 👁️✏️ Re-points the primary session at a different registered `AppRef` for the SAME artifact
   * (contract freeze §3/§5's "Open with…") — installs the target plugin first if it isn't loaded
   * yet, then mirrors `establishPrimarySession`'s non-studio create/seed/dispatch sequence. Also
   * best-effort notifies the host once `openArtifact` is wrapped (see `PendingAppChannelMethods`
   * above); `artifactRef` is approximated as the dialect coordinate — there is no per-document
   * identity surfaced to the shell yet, only per-dialect (see `📓️w1-c-report.md`). */
  const openArtifactWithAppRef = useCallback(
    async (target: AppRef, dialect: ArtifactDialect, role: AppRole) => {
      let plugin = loadedPlugins.find((entry) => entry.handle.pluginId === target.pluginId);
      if (!plugin) {
        const outcome = await installPlugin(target.pluginId);
        if (outcome !== "loaded" && outcome !== "already-loaded") return;
        plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === target.pluginId);
      }
      const app = plugin?.manifest.apps.find((candidate) => candidate.id === target.appId);
      if (!plugin || !app) {
        console.error(`[DEBUG] openArtifactWithAppRef: ${target.pluginId}/${target.appId} not found after install`);
        return;
      }
      void (plugin.handle as PendingAppChannelMethods).openArtifact?.(dialectCoordinate(dialect), role === "editor" ? 1 : 0, target.pluginId, target.appId)?.catch((commandError) => console.error("[DEBUG] openArtifact failed", commandError));
      const instanceId = await plugin.handle.createApp(app.id);
      const seeded = applyFrameworkLayoutSeed(app.defaultLayout, app.windowKinds, EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({ type: "SET_SESSION", value: { pluginId: plugin.handle.pluginId, instanceId, app, viewState: { activeModeId: app.defaultModeId ?? app.modes[0]?.id } } });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
    },
    [loadedPlugins, installPlugin, uiTerminology, uiLocale],
  );
  openArtifactWithAppRefRef.current = openArtifactWithAppRef;

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

  /** ⚖️ `ChromePanels`' Conflicts settings tab — `Shell`'s `selectOpenConflicts` selector feeds the
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
        if (!cancelled) dispatch({ type: "SET_CONFLICTS", value: conflicts });
      })
      .catch((commandError) => console.error("[DEBUG] readConflicts failed", commandError));
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
   * reply does — `ChromePanels`' panel and `ShellSync`'s quarantine badge both derive from `state.
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
      const label = osCommands.find((entry) => entry.id === commandId)?.label ?? commandId;
      noteShellCommand(commandId, label, detail);
    },
    [osCommands, noteShellCommand],
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
      dispatch({ type: "SET_UI_THEME_ID", value: id });
      noteOsCommand("os.setThemeId", { themeId: id });
    },
    [noteOsCommand],
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
    dispatch({ type: "SET_UI_THEME_ID", value: "semio" });
  }, []);

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
      dispatch({ type: "SET_UI_CUSTOM_THEMES", value: (current) => ({ ...current, [id]: saved }) });
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
      dispatch({ type: "SET_UI_THEME_ID", value: id });
    },
    [uiThemeBase],
  );

  const deleteTheme = useCallback((id: string) => {
    if (!id.startsWith("custom.")) return;
    dispatch({
      type: "SET_UI_CUSTOM_THEMES",
      value: (current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      },
    });
    dispatch({ type: "SET_UI_THEME_ID", value: (current) => (current === id ? "semio" : current) });
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
  }, []);

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
      dispatch({ type: "SET_UI_DRIVER_ID", value: id });
      noteOsCommand("os.setDriver", { driver: id });
    },
    [noteOsCommand],
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
      dispatch({ type: "SET_UI_CUSTOM_DRIVERS", value: (current) => ({ ...current, [id]: saved }) });
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
      dispatch({ type: "SET_UI_DRIVER_ID", value: id });
    },
    [uiDriverBase],
  );

  const deleteDriver = useCallback((id: string) => {
    if (!id.startsWith("custom.")) return;
    dispatch({
      type: "SET_UI_CUSTOM_DRIVERS",
      value: (current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      },
    });
    dispatch({ type: "SET_UI_DRIVER_ID", value: (current) => (current === id ? DEFAULT_UI_DRIVER.id : current) });
    dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
  }, []);
  //#endregion 🚗️DriverMutators

  const [themeSaveLabel, setThemeSaveLabel] = useState("");
  const [driverSaveLabel, setDriverSaveLabel] = useState("");
  const [keybindingCaptureControlId, setKeybindingCaptureControlId] = useState<string | null>(null);
  const setKeybindingOverride = useCallback((controlId: string, keys: string) => {
    dispatch({ type: "SET_UI_KEYBINDING_OVERRIDES", value: (current) => ({ ...current, [controlId]: keys }) });
  }, []);
  const resetKeybindingOverride = useCallback((controlId: string) => {
    dispatch({
      type: "SET_UI_KEYBINDING_OVERRIDES",
      value: (current) => {
        const { [controlId]: _removed, ...rest } = current;
        return rest;
      },
    });
  }, []);
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
        dispatch({ type: "SET_UI_APPEARANCE", value: value as ElementsSurfaceAppearance });
        noteOsCommand("os.setAppearance", { appearance: value });
      },
      layout: uiLayout,
      setLayout: (value: UiChromeLayout) => {
        dispatch({ type: "SET_UI_LAYOUT", value });
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
        dispatch({ type: "SET_UI_LOCALE", value });
        noteOsCommand("os.setLocale", { locale: value });
      },
      terminology: uiTerminology,
      setTerminology: (value: string) => {
        dispatch({ type: "SET_UI_TERMINOLOGY", value });
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
    (event: KeyboardEvent) => {
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
      const matches = (event: KeyboardEvent, binding: string) => {
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
    const hasPluginArtifactTab = flattenPanelTabLeaves(pluginLeftTabs).some((tab) => tab.id === FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
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

  //#region 🔖️CheckIn — ticket §C5 "when the user edits an artifact, the mutations are saved and
  // checked into vcs": status pill (`#s-sync-status`), auto check-in (idle ≥ 20s or ≥ 200 uncommitted
  // edits), explicit check-in (`#s-checkin`), checkpoint-on-close, and the post-checkpoint
  // `TouchArtifact` relay to the space index. Placed ahead of `🧰️FooterUtilityLeaves`/`🔄️SyncLeaf`
  // (their `useMemo`s below close over these) — everything here is additive, no existing behaviour
  // changed for a session outside a hub-bound space.
  /** 📌️ Reverse-lookup: `openDocumentSessionsRef` is keyed by documentId → `{session, plugin}`, never
   * the other way around (no `ActiveSession.documentId` field exists — see `📓️w3-a-report.md`'s
   * "Design decisions"). `session` changing is the only thing that can change which entry matches, so
   * keying the memo on `[session]` alone is sufficient even though the ref itself isn't reactive. */
  const currentDocumentId = useMemo(() => {
    if (!session) return null;
    for (const [documentId, entry] of openDocumentSessionsRef.current) {
      if (entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId) return documentId;
    }
    return null;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [session]);

  // 👥️ ticket §C0/§5 lane 4-F — `#s-presence-peers`, fed by `presencePeersJson` (set above, in
  // `ensureBackboneWorker`'s `event.kind === "presence"` branch, keyed to `entry.session.instanceId`
  // — i.e. the SAME open document `currentDocumentId` already resolves for THIS session, so reading it
  // straight off `session.viewState` is already filtered to the current `(space, document, surface)`
  // scope with no extra plumbing: a background/non-visible session's presence never lands on the one
  // `session` this shell renders. Reshapes the wire's `{clientId, name}` pair (the SAME shape
  // `NodeGraph`'s own `presencePeersJson` decoding already relies on) into `PresenceBar`'s
  // `{actor, label}` — the two shells' peer identity vocabulary otherwise matches byte-for-byte
  // (`peer:<actor>` row id, contract §C0).
  const presencePeers = useMemo((): readonly PresencePeer[] => {
    const json = session?.viewState.presencePeersJson;
    if (!json) return [];
    try {
      const raw = JSON.parse(json) as readonly { readonly clientId: string; readonly name: string }[];
      return raw.map((peer) => ({ actor: peer.clientId, label: peer.name || peer.clientId }));
    } catch {
      return [];
    }
  }, [session?.viewState.presencePeersJson]);

  const currentSyncStatus = currentDocumentId ? (syncStatusByDocumentId[currentDocumentId] ?? null) : null;
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

  const spaceIndexInstanceRef = useRef<Map<string, { readonly pluginId: string; readonly instanceId: number }>>(new Map());

  /** 📌️ §C5 item 6 — after a successful checkpoint, `TouchArtifact` the space's `index` document so
   * every connected user's home/space table `updated`/`updated-by` columns move. The index document
   * is almost never the one mounted in this shell's single visible session while an artifact editor
   * is open, so this opens (once per space, cached) a background, non-visible instance of the `s.space`
   * editor bound to `index` and dispatches its `touchArtifact` command directly — never touches
   * `dispatch({type:"SET_SESSION"...})`, so the user's own editor stays exactly where it is. */
  const touchSpaceIndexArtifact = useCallback(
    async (spaceId: string, artifactId: string) => {
      try {
        let handle = spaceIndexInstanceRef.current.get(spaceId);
        let pluginEntry = handle ? loadedPlugins.find((entry) => entry.handle.pluginId === handle!.pluginId) : undefined;
        // 🐚️ If the space index document happens to already be THIS shell's own visibly-mounted
        // session (the user is on `/spaces/{id}` itself, RIGHT NOW — `currentDocumentId`, not a
        // possibly-stale `openDocumentSessionsRef` entry from a space visited earlier this session
        // and never explicitly `closeDocument`d), reuse THAT session instead of a second instance.
        const liveEntry = currentDocumentId === S_SPACE_INDEX_DOCUMENT_ID ? openDocumentSessionsRef.current.get(S_SPACE_INDEX_DOCUMENT_ID) : undefined;
        if (liveEntry) {
          pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === liveEntry.plugin.pluginId);
          handle = pluginEntry ? { pluginId: pluginEntry.handle.pluginId, instanceId: liveEntry.session.instanceId } : undefined;
        }
        if (!handle || !pluginEntry) {
          pluginEntry = loadedPlugins.find((entry) => findDialectApp(entry, SPACE_INDEX_DIALECT, "editor"));
          if (!pluginEntry) return;
          const app = findDialectApp(pluginEntry, SPACE_INDEX_DIALECT, "editor");
          if (!app) return;
          const instanceId = await pluginEntry.handle.createApp(app.id);
          handle = { pluginId: pluginEntry.handle.pluginId, instanceId };
          spaceIndexInstanceRef.current.set(spaceId, handle);
          const worker = ensureBackboneWorker();
          const currentIdentity = identityRef.current;
          const dataDir = hubEnv?.dataDir;
          const folder: PersistenceBinding[] = dataDir ? [{ kind: "folder", path: `${dataDir}/spaces/${spaceId}` }] : [];
          const bindings: PersistenceBinding[] = currentIdentity ? [{ kind: "hub", baseUrl: currentIdentity.hubBaseUrl, spaceId, token: currentIdentity.sessionToken, surface: canonicalSurfaceId(SPACE_INDEX_DIALECT, "editor") }, ...folder] : folder;
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", documentId: S_SPACE_INDEX_DOCUMENT_ID, schema: S_SPACE_INDEX_DOCUMENT_SCHEMA, bindings, watchExternal: true, actor: shellActorIdRef.current }) });
          const uri = `actor://${S_SPACE_INDEX_DOCUMENT_ID}`;
          if (pluginEntry.handle.attachBackbone) await pluginEntry.handle.attachBackbone(instanceId, uri);
        }
        const app = findDialectApp(pluginEntry, SPACE_INDEX_DIALECT, "editor");
        if (!app || !pluginEntry.handle.handleCommand) return;
        const wire = encodeAppCommandInvocation(pluginEntry.handle.pluginId, app, "touchArtifact", { id: artifactId, nowMs: Date.now(), actor: identityRef.current?.userId ?? shellActorIdRef.current });
        await pluginEntry.handle.handleCommand(handle.instanceId, wire, { activeModeId: app.defaultModeId ?? app.modes[0]?.id });
      } catch (touchError) {
        console.error("[DEBUG] touchSpaceIndexArtifact failed", touchError);
      }
    },
    [loadedPlugins, ensureBackboneWorker, hubEnv, currentDocumentId],
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
      name: resolvePanelTabLabel(appLabelsOverlay, FRAMEWORK_PANEL_TAB_HISTORY_ID, resolveManifestLabel(tab.label, uiTerminology, uiLocale)),
      order: 1,
      tree: {
        sections: [
          {
            id: "framework.history.actions",
            label: shellLabel("ui.panel.history"),
            items: [
              { id: "framework.history.undo", label: "", control: <button type="button" disabled={isViewer || !historyProjection.canUndo} onClick={() => onAction({ controllerId: session.app.controllerId, action: "undo" })}>Undo</button> },
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
  }, [appLabelsOverlay, checkinDialog, historyProjection, onAction, session, submitCheckin, uiLocale, uiTerminology, uncommittedEditCount]);
  //#endregion 🧰️FooterUtilityLeaves

  //#region 🔄️SyncLeaf — bottom-left's sync tab, replacing the old floating footer SyncAttachCard.
  const quarantinedConflicts = useMemo(() => selectQuarantinedConflicts(shellState), [shellState]);
  const frameworkSyncTab = useMemo((): PanelTabNode | null => {
    const syncUtilities = buildFrameworkSyncUtilities(syncBackboneUri) as readonly UtilityNode[];
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
  const exampleOptions = useMemo(() => {
    const appId = session?.app.id ?? "";
    if (!appId) return [];
    const seen = new Set<string>();
    return (activePluginManifest?.examples ?? [])
      .filter((example) => example.appId === appId)
      .filter((example) => {
        if (seen.has(example.id)) return false;
        seen.add(example.id);
        return true;
      })
      .map((example) => ({
        id: example.id,
        label: resolveAppLabel(appLabelsOverlay, "example", example.id, resolveManifestLabel(example.label, uiTerminology, uiLocale)),
        icon: example.iconId,
      }));
  }, [activePluginManifest, session?.app.id, appLabelsOverlay, uiTerminology, uiLocale]);

  const dispatchActiveExample = useCallback(
    (exampleId: string) => {
      if (!session) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin) return;
      onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId: exampleId || "" } });
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
          dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
          dispatchActiveExample(exampleId || "");
        }}
      />
    );
  }, [session, exampleOptions, locks.exampleId, hostMode, landingAppId, activeExampleId, dispatchActiveExample]);

  /** @emoji 🎛️ Shared by the desktop navbar center cluster and the mobile panel's synthetic "App" tab (see `mobilePanelTabs`). */
  const modeSwitcherElement = useMemo(() => {
    if (!session || session.app.modes.length <= 1) return null;
    return (
      <ButtonGroup key="modes" id="playground.navbar.modes">
        {session.app.modes.map((mode) => {
          const isActive = activeModeId === mode.id;
          return (
            <ButtonGroupItem
              key={mode.id}
              id={`playground.navbar.modes.${mode.id}`}
              className={cn(isActive && interactiveActiveFillClass)}
              data-state={isActive ? "on" : undefined}
              onClick={() => applyModeChange(mode.id)}
              icon={mode.iconId}
              text={resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label, uiTerminology, uiLocale))}
            />
          );
        })}
      </ButtonGroup>
    );
  }, [session, activeModeId, applyModeChange, appLabelsOverlay, uiTerminology, uiLocale]);

  const resolvedCommands = useMemo(() => {
    const resolved = resolveCommands(osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale);
    // 👁️✏️ Hides every `Mutation`-kind command from a viewer session's palette (contract freeze §5) —
    // `resolveCommands` itself stays role-agnostic (os hosts/tests call it without a session at all).
    return session?.app.role === "viewer" ? resolved.filter((entry) => !isMutationKindDefinition(entry.definition)) : resolved;
  }, [osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale]);

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
        dispatchOsCommand(commandId, args, dispatch, dockLayoutStore, dockUiStateStore, locks);
        const label = resolvedCommands.find((entry) => commandAddressKey(entry.address) === commandAddressKey(address))?.definition.label ?? commandId;
        noteShellCommand(commandId, label, args);
        return;
      }
      if (!session) return;
      // ⏺️ Recorder tap for plugin/app/mode-scope commands — mirrors `onAction`'s tap above.
      if (tutorialRecordingRef.current && !tutorialDrivenRef.current) {
        tutorialRecorderRef.current?.recordEvent({ kind: "command", command: commandId, args });
      }
      const ownerPluginId = commandOwnerPluginId(address.owner);
      if (!ownerPluginId || ownerPluginId !== session.pluginId) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === ownerPluginId)?.handle;
      if (!plugin?.handleCommand) return;
      // 👁️✏️ Same client-side half of the read-only guarantee `onAction` applies above, for commands.
      if (session.app.role === "viewer" && resolvedCommands.find((entry) => commandAddressKey(entry.address) === commandAddressKey(address))?.definition.kind === "mutation") {
        showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
        return;
      }
      const dispatchViewState = injectActiveUtility(session.viewState);
      const invocation: CommandInvocation = { address, arguments: args ?? {} };
      void plugin
        .handleCommand(session.instanceId, JSON.stringify(invocation), dispatchViewState)
        .then((response) => {
          applyHistoryPatch(response.historyPatch);
          return applyHostEffects(response.requestedEffects ?? [], { ...session, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope));
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
    [applyHostEffects, dockLayoutStore, dockUiStateStore, injectActiveUtility, loadedPlugins, session, locks, resolvedCommands, noteShellCommand, showTransientNotice, isViewerReadOnlyFault, uiLocale],
  );

  const handleCommandKeydown = useCallback(
    (event: KeyboardEvent) => {
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
    () => resolveModeTools(session?.app, activeModeId).map((tool) => ({ ...tool, label: resolveManifestLabel(tool.label, uiTerminology, uiLocale) })),
    [session?.app, activeModeId, uiTerminology, uiLocale],
  );

  const toolTabs = useMemo(
    () => (session ? buildToolTabs(resolvedModeTools, session.app.controllerId, activeToolIdRef, toolMeasuresByToolIdRef, onActionStable) : []),
    [resolvedModeTools, session?.app.controllerId, onActionStable],
  );

  //#region 🧭️DockAssembly — default four-corner arrangement (the two middle anchors start empty save the command palette in bottom-middle) + persisted-override reconciliation + drag-and-drop wiring.
  const defaultDock = useMemo((): PanelDock => {
    // 🧭️ Top-left (Workbench: Document/Catalogue) and top-right (Details: Inspection/Parameters) stay flat.
    // Bottom-right exposes one Settings branch whose children are internal tabs, plus one Marketplace leaf.
    const topLeft: PanelTabNode[] = [...workbenchLeftTabs];
    const bottomLeft: PanelTabNode[] = [];
    if (frameworkDisplayTabs.length > 0) {
      bottomLeft.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 0, children: frameworkDisplayTabs });
    }
    if (frameworkSyncTab) bottomLeft.push(frameworkSyncTab);
    const topRight: PanelTabNode[] = [...detailsRightTabs];
    const bottomRight: PanelTabNode[] = [frameworkSettingsTab, frameworkMarketplaceTab];
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
  }, [commandCategoryTabs, detailsRightTabs, frameworkDisplayTabs, frameworkMarketplaceTab, frameworkSettingsTab, frameworkSyncTab, frameworkUtilitiesHistoryTab, toolTabs, uiLocale, workbenchLeftTabs]);

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
    if (!exampleSelectElement && !modeSwitcherElement) return anchorTabs;
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
            ],
          },
        ],
      },
    });
    return [...anchorTabs, appTab];
  }, [defaultDock, exampleSelectElement, modeSwitcherElement]);

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
        if (tabId && hostMode && session?.app.id === hostAppId && findPanelTabNode(mobilePanelTabs, path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
      // ♻️ Lazy tool/command trees read measures + active tool from refs — revision forces re-resolve.
      treeContentRevision: { activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey },
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
        const pathChanged = (panelActivePaths[anchor] ?? []).join("/") !== path.join("/");
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
        // 🛠️ Selecting a mode-tool leaf (`tool.<id>`) activates that tool so its measures render immediately
        // under the tab — no nested Fill toggle inside the tree.
        if (anchor === "bottom-middle" && session && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          const selectedToolId = toolIdFromPanelTabId(tabId);
          if (selectedToolId && selectedToolId !== activeToolIdRef.current) {
            onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: selectedToolId } });
          }
        }
        // 🌱️ Progressive paths often end at a branch (or are empty) — only leaves are meaningful "active panel tab" selections.
        if (tabId && hostMode && session?.app.id === hostAppId && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
        if (pathChanged && tabId) noteShellCommand("shell.panelTab", shellLabel("ui.shellCommand.panelTab"), { anchor, tabId });
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
    }),
    [dock, onAction, panelActivePaths, panelPathMemory, panels, session, hostMode, hostAppId, noteShellCommand],
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
  }, [brand, buildPanelSelectionProps, exampleOptions, exampleSelectElement, locks.exampleId, mobile, mobilePanelVisible, modeSwitcherElement, session, uiTerminology, hostMode, landingAppId]);

  const searchItems = useMemo(() => {
    if (!session) return [];
    const items: UISearchItem[] = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      const tabId = panelTabKindId(tab.kind);
      items.push({
        id: `panel.${tabId}`,
        label: resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.panels"),
        icon: <Icon icon="panel-left" size="small" />,
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } }),
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale)),
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
        label: argCarrying ? `${definition.label}…` : definition.label,
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
      for (const program of panel.programs) {
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
  }, [activeWindowId, appLabelsOverlay, loadedPlugins, mobile, onAction, onCommand, panel, resolvedCommands, session, hostMode, uiLocale, uiTerminology, hostControllerId]);

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
    if (hostMode && spawnedWindowUi && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const spawnedApp = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        const windowKind = spawnedApp?.windowKinds[0];
        const chrome = windowKind ? spawnedWindowChromeForKind(windowKind, spawned.id, spawnedWindowEngagements, spawnedWindowMeasures, activeUtilityByWindowId[spawned.id], onActionStable) : undefined;
        const spawnedUtilities = spawnedApp && windowKind ? resolveUtilityNodes(spawnedApp, windowKind, activeUtilityByWindowId[spawned.id], spawned.id, appLabelsOverlay, uiTerminology, uiLocale) : [];
        return [
          {
            id: spawned.id,
            title: wireLabel(appBreadcrumb(spawnedApp ? resolveAppBreadcrumb(spawnedApp, uiTerminology) : spawned.breadcrumb)),
            fill: true,
            showControls: true,
            measures: chrome?.measures,
            measuresFolded: measuresFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
            engagement: chrome?.engagement,
            search: chrome?.search,
            utilityBar: spawnedApp && windowKind ? utilityBarNode(spawnedUtilities, spawned.id, onActionStable, introductionUtilityId, chrome?.utilityOptions) : undefined,
            utilityBarFolded: utilityBarFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
            actionPane: spawnedApp && windowKind ? windowActionPaneNode(spawnedApp, windowKind, spawned.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale) : undefined,
            actionsFolded: actionsFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
            onActionsFoldedChange: onActionsFoldedFor(spawned.id),
            children: (
              <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={spawnedApp ? cursorFor(spawnedApp, spawned.id) : undefined}>
                <ShellFaultBoundary boundaryId={`window-${spawned.id}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                  <InterpretedUiNode node={spawnedWindowUi} onAction={onActionStable} />
                </ShellFaultBoundary>
              </ChromeAwareWindowScrollSurface>
            ),
          },
        ];
      }
    }
    if (Object.keys(windowUiByWindowId).length === 0) return [];
    const baseWindows = session.app.windowKinds.map((kind) => {
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[kind.id], kind.id, appLabelsOverlay, uiTerminology, uiLocale);
      const chrome = windowMeasuresChrome(windowMeasuresByWindowId[kind.id] ?? kind.options.measures, activeUtilityByWindowId[kind.id], kind.id, onActionStable);
      const resolvedEngagement = resolveWindowEngagement(kind, kind.id, windowEngagementsByWindowId);
      return {
        id: kind.id,
        iconId: windowIconsById[kind.id] ?? kind.iconId,
        title: windowTitlesById[kind.id] ?? appWindowLabel(session.app, uiTerminology, resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale)), uiLocale),
        fill: true,
        showControls: true,
        measures: chrome.measures,
        measuresFolded: measuresFoldedFor(kind.id, kind.id),
        engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
        search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
        utilityBar: utilityBarNode(utilities, kind.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
        utilityBarFolded: utilityBarFoldedFor(kind.id, kind.id),
        actionPane: windowActionPaneNode(session.app, kind, kind.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale),
        actionsFolded: actionsFoldedFor(kind.id, kind.id),
        onActionsFoldedChange: onActionsFoldedFor(kind.id),
        status: declarativeSurfaceStatus(windowUiByWindowId[kind.id]),
        skeleton: <WindowBodySkeleton />,
        children: (
          <ChromeAwareWindowScrollSurface id={childElementId("framework.window", kind.id)} className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={cursorFor(session.app, kind.id)}>
            <WindowInstanceIdContext.Provider value={kind.id}>
              <ShellFaultBoundary boundaryId={`window-${kind.id}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                <InterpretedUiNode node={windowUiByWindowId[kind.id] ?? pendingWindowUiNode()} onAction={onActionStable} />
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
      const chrome = windowMeasuresChrome(windowMeasuresByWindowId[instance.id] ?? kind.options.measures, activeUtilityByWindowId[instance.id], instance.id, onActionStable);
      const resolvedEngagement = resolveWindowEngagement(kind, instance.id, windowEngagementsByWindowId);
      return [
        {
          id: instance.id,
          iconId: windowIconsById[instance.id] ?? kind.iconId,
          title: windowTitlesById[instance.id] ?? instance.title,
          fill: true,
          showControls: true,
          measures: chrome.measures,
          measuresFolded: measuresFoldedFor(instance.id, instance.windowKindId),
          engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
          search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
          utilityBar: utilityBarNode(utilities, instance.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
          utilityBarFolded: utilityBarFoldedFor(instance.id, instance.windowKindId),
          actionPane: windowActionPaneNode(session.app, kind, instance.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale),
          actionsFolded: actionsFoldedFor(instance.id, instance.windowKindId),
          onActionsFoldedChange: onActionsFoldedFor(instance.id),
          status: declarativeSurfaceStatus(windowUiByWindowId[instance.id]),
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
                  <InterpretedUiNode node={windowUiByWindowId[instance.id] ?? pendingWindowUiNode()} onAction={onActionStable} />
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
    extraWindowInstances,
    introductionActionWindowSegment,
    introductionUtilityId,
    introductionUtilityWindowId,
    loadedPlugins,
    onActionStable,
    panel,
    session,
    spawnedWindowEngagements,
    spawnedWindowMeasures,
    spawnedWindowUi,
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
      (session ? resolveFrameworkLayoutSeed(session.app.defaultLayout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale).modeLayout : { kind: "stack" as const, children: [] }),
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
        <p className="p-double text-sm text-destructive" role="alert" data-semio-os-shell-error="">
          {error}
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
            modes={modes.map((mode) => ({ id: mode.id, label: resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label, uiTerminology, uiLocale)), children: null }))}
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
                  updateSpacePanel(buildSpacePanelState(panel.programs, nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
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
                  value: (current) => current ?? resolveFrameworkLayoutSeed(session.app.defaultLayout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale).modeLayout,
                });
              }}
            />
          </App>
          </ShellFaultBoundary>
        </div>
      </div>
    );
  }, [activeWindowId, effectiveModeLayout, error, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, loadedPlugins, mobile, modeWindows, navigateHistory, noteShellCommand, onAction, panel, pluginSupervisorById, primaryPluginId, reloadPlugin, session, shellRoute, hostMode, uiLocale, uiTerminology, updateSpacePanel, dispatch, uninstallPlugin]);

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
    // shell's own `render_presence_bar` placement (`Shell/🧊️component.rs`) rather than hiding behind a
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
    }),
    [buildPanelSelectionProps, panels, treeOpenStates],
  );

  // #region 🔖️ReadinessBeacon
  /** 🚦️ Deterministic DOM beacon for headless smoke tests (e.g. Storybook's OS-shell plugin-boot matrix)
   * to wait on instead of screenshots/timeouts — set once a session resolves or errors, cleared on unmount. */
  useEffect(() => {
    const root = document.documentElement;
    const beaconId = pluginFilter ?? "unknown";
    const notFound = hostMode && shellRoute.kind === "notFound";
    if (notFound) {
      root.dataset.semioOsNotFound = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
    } else if (error) {
      root.dataset.semioOsError = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsNotFound;
    } else if (session) {
      root.dataset.semioOsReady = beaconId;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
    }
    return () => {
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
    };
  }, [session, error, pluginFilter, shellRoute.kind, hostMode]);
  // #endregion 🔖️ReadinessBeacon

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
          label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, uiTerminology, uiLocale)) + (argCarrying ? "…" : ""),
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
    const handleContextMenu = (event: MouseEvent) => {
      if (isContextMenuPointerTarget(event.target)) return;
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
    <SetWindowTitleContext.Provider value={setWindowTitle}>
    <SetWindowIconContext.Provider value={setWindowIcon}>
    <AppKeybindingsContext.Provider value={keysByActionId}>
    <UiKeybindingsProvider bindings={controlKeybindings}>
    <PluginSurfaceActionsContext.Provider value={requestContextMenu}>
    <ShellContextMenuFallbackContext.Provider value={buildShellContextMenuItems}>
    <ShellFaultBoundary boundaryId="shell-root" fallbackLabel={shellLabel("ui.common.renderError")}>
    <UIFindProvider>
      <LevelProvider level="base">
        <div className="flex h-screen min-h-0 w-screen flex-col bg-transparent" data-level="base">
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
            const dialog = session.app.dialogs?.find((entry) => entry.id === overlayDialog.dialogId);
            if (!dialog) return null;
            return (
              <UIDialog
                dialog={resolveDialogDefinition(dialog, appLabelsOverlay, uiTerminology, uiLocale)}
                seedArgs={overlayDialog.seedArgs}
                renderField={(def, value, onChange) => renderStagedArgControl(def, value, onChange)}
                onSubmit={(args) => {
                  dispatch({ type: "SET_DIALOG", value: null });
                  onAction({ controllerId: session.app.controllerId, action: dialog.submitAction, args });
                }}
                onCancel={() => {
                  dispatch({ type: "SET_DIALOG", value: null });
                  if (dialog.cancelAction) onAction({ controllerId: session.app.controllerId, action: dialog.cancelAction });
                }}
              />
            );
          })()}
      </LevelProvider>
    </UIFindProvider>
    </ShellFaultBoundary>
    </ShellContextMenuFallbackContext.Provider>
    </PluginSurfaceActionsContext.Provider>
    </UiKeybindingsProvider>
    </AppKeybindingsContext.Provider>
    </SetWindowIconContext.Provider>
    </SetWindowTitleContext.Provider>
  );
}
//#endregion FrameworkOsShell

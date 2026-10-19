/** 🧩️ The shell's base state: scope, locale and device, the shell reducer, history and interaction stores, panel, dialog and tutorial state, the extra-window registry, operation settling, surface switching, tree windows and the built-node stores.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useShellCoreState` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { type ShellDialogOriginV1 } from "../🗨️dialog-origin/🟦️.ts";
import { type DocumentCheckInStatusV1 } from "@semio-tech/framework/document/check-in";
import { OwnedTutorialRunV1, type TutorialInverseV1 } from "../🗨️dialog-origin/🎥️tutorial/🟦️.ts";
// #region 🔌️Adapters
import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useMemo,
  useReducer,
  useRef,
  useState
} from "react";
import {
  type ActionDispatchLifecycleV1,
  type AppDefinition,
  type AppRole,
  type Effect,
  type HistoryPatch,
  type LocalizedLabel,
  type PluginRegistryEntry,
  type ContributorInstance,
  resolvePluginHostConfig,
  type ShellBrand,
  type BuiltNode,
  type UiDirtyScope,
  type Fault,
  type Severity
} from "@semio-tech/framework";
import { type DocumentScope } from "@semio-tech/framework/document/scope";
import { type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import {
  createSealedInstanceLedgerV1,
  createSessionAppSwitchGateV1,
  createSessionWorkLedgerV1,
  createShellSessionLaneV1,
  sealedInstanceDropTextV1,
  sealedInstanceDropV1
} from "../🔀️surface-switch/🟦️.ts";
import { programHistoryKeyV1, programHistoryProjectionsAfterPatchV1, createProgramHistoryStoreV1 } from "../🪟️spawned-program/🟦️.ts";
import { type InputOutcomeV1, type ShellInputActionV1 } from "../🎯️input-ledger/🟦️.ts";
import { type UiPreferencesConfigMutation } from "@semio-tech/framework/config/mutations";
import {
  type ContextMenuItem,
  DEFAULT_UI_DRIVER,
  elementsSurfaceDeviceIsMobile,
  type IconName,
  type TutorialClock,
  UI_TERMINOLOGY_NATIVE,
  type UiLocale,
  useLabel,
  useUiDevice,
  useShellScope
} from "@semio-tech/ui-react";
import { readUiPreferences, resolveUiPreferences, subscribeUiPreferences } from "../../🎚️UiPreferences/🟦️.ts";
import { type TreeWindowContextValue } from "../../🗣️Interpreter/🪟️tree-windows/🟦️.ts";
import { LocalDocumentOwnerRegistryV1 } from "../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type SpawnedAppEntry } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type ActiveSession, type LoadedProgramState, localProgramsV1, type SpaceProgramEntry } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { EMPTY_SHELL_DEFAULTS, EMPTY_SHELL_LOCKS, type FrameworkOsDefaults, isEphemeralShellBrand, resolveBootExampleId, type ResolvedShellLocks } from "../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { type ExtraWindowInstance } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { createLocalInteractionStoreV1, initialShellState, shellReducer } from "../../🐚️Shell/🕹️shell-store/🟦️.ts";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { leftoverOverlayCarryingSelectionV1, leftoverOverlayCarryingUtilityV1, leftoverWorldArmedWindowOverlayV1, leftoverWorldSelectionOverlayV1, leftoverWorldWindowOverlayV1, publishLeftoverWorldSelectionV1 } from "../../🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { clearPendingWorldProjections, World3dWindowViewRegistryV1 } from "../../🌐️World3dHost/🪟️window-view-store/🟦️.tsx";
import { navbarExampleIdFromHistoryUpserts } from "../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import { interactionViewFromLeftoverOutput, leftoverInteractionStateV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🎮️leftover-views/🟦️.ts";
import { type DocumentTransferNoticeV1 } from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import {
  createTreeWindowSchedulerV1,
  createTreeWindowContextV1,
  type PanelTreeConfigCacheV1,
  type TreeWindowHostV1,
  type TreeWindowSchedulerV1
} from "../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import {
  EMPTY_SHELL_HISTORY_PROJECTION_V1,
  shellHistoryProjectionAfterPatchV1,
  type ShellHistoryProjectionV1,
  historyPatchShouldApplyV1
} from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { resolveManifestLabel, shellLabelLocale, syncShellLabelLocale } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { type UiRefreshCache } from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { isPluginInstanceRetiredV1, leftoverInspectionPanelHash, type PluginWasmHandle, type PluginOperationCompletion } from "../../🔌️PluginRuntime/🟦️.tsx";
import { createShellRouteLedgerV1 } from "../🧭️route-ledger/🟦️.ts";
import type { PluginCatalog } from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { resolveRequiredHostApps } from "../🪪️host-bootstrap/🟦️.tsx";
import type { InstalledServicePresentationV1 } from "@semio-tech/framework/inference/service";
import type { ReactNode } from "react";
import type { ShellHostPortsV1 } from "../🪝️host-ports/🟦️.ts";
import { OPERATION_SETTLE_RING_SLOTS, OPERATION_SETTLE_WAITER_SLOTS, OPERATION_SETTLE_WATCHDOG_MS, operationSettleKeyV1 } from "../🏁️operation-settle/🟦️.ts";
import { createBuiltNodeStoreCacheV1, logUnlessRetiredV1, requiredHostPanelLeafId, type SpawnedRefreshSlotV1 } from "../🧱️built-node-stores/🟦️.ts";
import type { Dispatch, RefObject, SetStateAction } from "react";
import type { ShellState } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import type { SpaceArtifactCreationCatalogAuthorityV1 } from "../🧾️space-artifact-creation/🟦️.ts";
import { type AppSurfaceSessionFactory } from "@semio-tech/framework/canvas/react-renderer";

type ShellEffectOwnerForward = {
  presentation: ShellDialogOriginV1 | null;
  source: ShellDialogOriginV1 | null;
  session: ActiveSession;
  plugin: PluginWasmHandle | null;
  creationCatalog: SpaceArtifactCreationCatalogAuthorityV1 | null;
  inputSeq: number | null;
};

type ApplyHostEffectsForward = (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ShellEffectOwnerForward) => Promise<void>;

type OnActionForward = (requested: ShellInputActionV1, submittedOrigin?: ShellDialogOriginV1, propagateFailure?: boolean, lifecycle?: ActionDispatchLifecycleV1, requiredOperationOwner?: ShellEffectOwnerForward) => Promise<InputOutcomeV1>;

type DocumentCheckInUiForward = Readonly<Record<string, { readonly requestId: string; readonly scope: DocumentScope; readonly status: DocumentCheckInStatusV1 }>>;

/** 🔮️ The closure bindings an earlier hook reads before a later hook declares them. The declaring hook assigns its
 * slot right after the declaration, so a callback built earlier reads the same binding it read as a closure variable. */
export type ShellForwardSlots = {
  loadedPluginsRef: RefObject<readonly LoadedProgramState[]>;
  shellStateRef: RefObject<ShellState>;
  applyHostEffectsRef: RefObject<ApplyHostEffectsForward>;
  spawnedIdsRef: RefObject<readonly string[]>;
  documentCheckInOwnersRef: RefObject<Map<string, { readonly runtimeKey: string }>>;
  setDocumentCheckInUi: Dispatch<SetStateAction<DocumentCheckInUiForward>>;
  documentTransferRef: RefObject<{
    exportDocumentArchive: () => Promise<void>;
    importDocumentArchive: () => Promise<void>;
    saveNaturalFile: () => Promise<void>;
    openNaturalFile: () => Promise<void>;
    trackDocumentTransfer: (file: string, pluginId: string) => { readonly signal: AbortSignal; readonly progress: (completed: number, total: number | null) => void; readonly finish: () => void };
    notifyDocumentTransfer: (notice: DocumentTransferNoticeV1, file: string, kind: Severity) => void;
  }>;
  spawnedAppsRef: RefObject<readonly SpawnedAppEntry[]>;
  activeUtilityByWindowIdRef: RefObject<Readonly<Record<string, string | null>>>;
  applyHostEffects: ApplyHostEffectsForward;
  onActionRef: RefObject<OnActionForward>;
  tutorialRunRef: RefObject<OwnedTutorialRunV1<TutorialInverseV1> | null>;
  tutorialTransitionEpochRef: RefObject<number>;
  tutorialClockRef: RefObject<TutorialClock | null>;
  isViewerReadOnlyFault: (error: unknown) => boolean;
  isMutationRejectedFault: (error: unknown) => boolean;
  showTransientNotice: (message: string, kind?: Severity, code?: string) => void;
  showMutationRejectedNotice: (fault: Fault) => void;
};

export type FrameworkOsShellInnerProps = {
  readonly catalog: PluginCatalog;
  readonly pluginFilter?: string;
  readonly plugins: readonly PluginRegistryEntry[];
  readonly surfaceSessionFactories?: readonly AppSurfaceSessionFactory[];
  readonly backboneWorkerFactory?: () => Worker;
  readonly documentServices?: readonly InstalledServicePresentationV1<ReactNode>[];
  readonly hostPorts?: ShellHostPortsV1;
  readonly appId?: string;
  readonly appRole?: AppRole;
  readonly locks?: ResolvedShellLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly brand?: ShellBrand;
  readonly suppressAutoIntroduction?: boolean;
};

export type ShellCoreAfterShellCoreState = ShellForwardSlots & ReturnType<typeof useShellCoreState>;

//#region FrameworkOsShell
export function useShellCoreState({
  catalog,
  pluginFilter,
  plugins,
  surfaceSessionFactories,
  backboneWorkerFactory,
  documentServices = [],
  hostPorts,
  appId,
  appRole,
  locks: locksProp,
  defaults: defaultsProp,
  brand,
  suppressAutoIntroduction = false,
}: FrameworkOsShellInnerProps, core: ShellForwardSlots) {
  const scope = useShellScope();
  const [localDocumentOwnerRegistry] = useState(() => new LocalDocumentOwnerRegistryV1());
  useEffect(() => () => localDocumentOwnerRegistry.retireAll(), [localDocumentOwnerRegistry]);
  const shellContextMenuTitleLabel = useLabel("ui.surfaceContextMenu.workspace");
  // 🏠️🧳️ `hostConfig` is the sole piece of per-plugin identity knowledge the shell needs (which app id is
  // "landing", which is "host") — every controller id / default panel tab derives from the *loaded*
  // manifest's own `controllerId`/`panelTabs` on those apps below, never from a separate literal.
  const hostConfig = pluginFilter ? resolvePluginHostConfig(catalog, pluginFilter) : undefined;
  const hostMode = hostConfig !== undefined;
  // 📱️ `measuredDevice` is the THREE-way viewport read (`🖱️ui/📱️device/🟦️.ts`, shared byte-for-byte with
  // the wgpu dock); `mobile` stays the binary flag every panel-collapse call site below already reads.
  const measuredDevice = useUiDevice();
  const mobile = elementsSurfaceDeviceIsMobile(measuredDevice);
  const locks = locksProp ?? EMPTY_SHELL_LOCKS;
  const defaults = defaultsProp ?? EMPTY_SHELL_DEFAULTS;
  const ephemeral = isEphemeralShellBrand(brand);
  const [shellState, dispatch] = useReducer(shellReducer, undefined, () => initialShellState({ pluginFilter, plugins, locks, defaults, storage: scope.storage, locale: scope.i18n.language as UiLocale }));
  /** 🕹️ This shell's local interaction ({@link createLocalInteractionStoreV1}) — outside `shellState`, so a hover never
   * re-renders the shell. */
  const [localInteraction] = useState(createLocalInteractionStoreV1);
  /** 🧾️ One history projection PER PROGRAM ({@link programHistoryKeyV1}), never one for the window.
   * A spawned program is a member of this session with a document and a cursor of its own, and the
   * cursor rule below is what decides whether a patch is a step forward — taken against the wrong
   * program it silently drops every patch the other one sends (`🪟️spawned-program`'s
   * {@link programHistoryProjectionsAfterPatchV1} carries the measurement). Kept in a store, not in render state: the
   * entries move on every keystroke and only their own readers follow them ({@link createProgramHistoryStoreV1}). */
  const [historyStore] = useState(() => createProgramHistoryStoreV1<ShellHistoryProjectionV1>());
  const historyOrderRef = useRef(0);
  /** 🧾️ The order at which each program's projection last moved — what a full snapshot's own
   * staleness guard compares against. Shell-wide it was the wrong question the moment two programs
   * are open: a spawned program's patch would cancel the session's in-flight snapshot and vice versa. */
  const historyOrderByProgramRef = useRef(new Map<string, number>());
  const exampleOptionsRef = useRef<readonly { readonly id: string }[]>([]);
  const lastDispatchedExampleIdRef = useRef("");
  const localHistoryOrderRef = useRef(0);
  const { loadedPlugins, pluginStatusById, pluginSupervisorById, session, error, sessionFault, instanceFault } = shellState.pluginRuntime;
  /** 🔌️ The device's own programs ({@link localProgramsV1}): hub documents' catalog-resolved programs run beside them and
   * never enter routing, the program launcher, contributions or the dialect index. */
  const localPrograms = useMemo(() => localProgramsV1(loadedPlugins), [loadedPlugins]);
  // 🌐️ Read with the other render-time `shellState` slice above the memo block below: `spacePrograms`'s dependency
  // array reads `uiLocale`/`uiTerminology` during this render, and a `const` declared after it is still in its
  // temporal dead zone there — the shell then throws `Cannot access 'uiLocale' before initialization` on first paint.
  const { uiAppearance, uiLayout, uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale, uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft, uiKeybindingOverrides } = shellState.uiPrefs;
  // 🌐️ The shared `shellLabel` port moves HERE, in render, not in the appearance effect below that
  // also calls it: every builder memo under this line (`osCommands`, the command/tool category
  // trees, the panel tab names) calls `shellLabel` while rendering, so a port moved in a post-paint
  // effect is one render too late — those memos recompute against the previous language and, since
  // `uiLocale` does not change again, keep that text for good. The effect's call stays: it is what
  // moves the scope's own i18n instance and `documentElement.lang`, and the two ports must move
  // together (`syncShellLabelLocale`'s own doc). Guarded so an unchanged locale costs one compare.
  if (shellLabelLocale() !== uiLocale) syncShellLabelLocale(uiLocale);
  /** 🧾️ Applies one `HistoryPatch` to the projection of the program that produced it. `owner` is that
   * program — the dispatch's target session, the completion's spawned instance, the snapshot's
   * instance — and defaults to the live session for the shell's own lanes. */
  const applyHistoryPatch = useCallback((patch: HistoryPatch | undefined, replace = false, owner?: { readonly pluginId: string; readonly instanceId: number }) => {
    if (!patch) return;
    const key = programHistoryKeyV1(owner ?? sessionRef.current);
    if (key === "") return;
    let applied = false;
    historyStore.update((projections) => {
      const next = programHistoryProjectionsAfterPatchV1(
        projections,
        key,
        EMPTY_SHELL_HISTORY_PROJECTION_V1,
        (currentCursor) => historyPatchShouldApplyV1(currentCursor, patch, replace),
        (current) => {
          historyOrderByProgramRef.current.set(key, ++historyOrderRef.current);
          localHistoryOrderRef.current = historyOrderRef.current;
          // 📌️ §C5 — `currentCheckpointId` rides every patch; `🔖️CheckIn` below watches it change to know a
          // checkpoint it asked for actually landed (see `touchSpaceIndexArtifact`'s call site).
          return shellHistoryProjectionAfterPatchV1(current, patch, replace);
        },
      );
      if (next.applied) applied = true;
      return next.projections;
    });
    if (applied) {
      const navbarExample = navbarExampleIdFromHistoryUpserts(patch.upserts, lastDispatchedExampleIdRef.current, resolveBootExampleId("", exampleOptionsRef.current, defaults.exampleId));
      if (navbarExample !== undefined) {
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
  // 🪟️ `windowId` is the pane the action that produced `output` addressed. A leftover carries per-window
  // state (hover, the armed utility), so it is published under THAT pane and never
  // over every pane's record (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B39); a windowless action
  // publishes the document's shared fields only and leaves every pane's own fields untouched.
  const applyLeftoverInteractionView = useCallback((output: unknown, addressedWindowId?: string) => {
    const published = interactionViewFromLeftoverOutput(output);
    const windowId = addressedWindowId ?? published?.windowId ?? undefined;
    if (!published) return;
    const priorLeftover = windowId ? leftoverWorldWindowOverlayV1(windowId) : leftoverWorldSelectionOverlayV1();
    const overlay = leftoverOverlayCarryingSelectionV1(
      leftoverOverlayCarryingUtilityV1(
        {
          ids: published.selectedIds,
          hoveredId: published.hoverTarget?.id ?? null,
          hoveredDomain: published.hoverTarget?.domain ?? null,
          gumballActive: published.gumballActive,
          gumballAnchorId: published.gumballAnchorId,
          ...(published.selectionCleared ? { selectionCleared: true } : {}),
          ...(published.activeUtility !== undefined ? { activeUtility: published.activeUtility } : {}),
        },
        priorLeftover,
      ),
      priorLeftover,
      published.selectionCleared === true,
    );
    publishLeftoverWorldSelectionV1(overlay, windowId ? { kind: "window", windowId } : { kind: "document" });
    localInteraction.observe(leftoverInteractionStateV1({ ...published, selectedIds: overlay.selectionCleared ? [] : overlay.ids }));
    leftoverInspectionHasSelectionRef.current = overlay.selectionCleared ? false : overlay.ids.length > 0;
    const selectedKey = overlay.selectionCleared ? "" : overlay.ids.join("\0");
    if (overlay.selectionCleared || (overlay.ids.length === 0 && leftoverInspectionIdsRef.current.length > 0)) {
      leftoverInspectionSelectedKeyRef.current = "";
      leftoverInspectionIdsRef.current = [];
      leftoverInspectionEpochRef.current += 1;
      setLeftoverInspectionEpoch(leftoverInspectionEpochRef.current);
    } else if (overlay.ids.length > 0 && leftoverInspectionSelectedKeyRef.current !== selectedKey) {
      leftoverInspectionSelectedKeyRef.current = selectedKey;
      leftoverInspectionIdsRef.current = overlay.ids;
      leftoverInspectionEpochRef.current += 1;
      setLeftoverInspectionEpoch(leftoverInspectionEpochRef.current);
    }
  }, []);
  /** 🧾️ Re-reads ONE program's full history and replaces that program's projection with it. The
   * handle is resolved from the OWNER's plugin, never from the session's: a spawned editor's
   * instance id addressed through the host plugin's handle is a different instance (or none). */
  const refreshHistorySnapshot = useCallback((owner: { readonly pluginId: string; readonly instanceId: number }): Promise<void> => {
    const plugin = core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === owner.pluginId)?.handle;
    if (!plugin?.readHistory) return Promise.resolve();
    const key = programHistoryKeyV1(owner);
    const orderAtRequest = historyOrderByProgramRef.current.get(key) ?? 0;
    return plugin.readHistory(owner.instanceId).then((snapshot) => {
      if ((historyOrderByProgramRef.current.get(key) ?? 0) > orderAtRequest) {
        return;
      }
      applyHistoryPatch(snapshot, true, owner);
    }).catch((error) => undefined);
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
  const hostCatalogueTabId = hostApp ? requiredHostPanelLeafId(hostApp) : undefined;
  const spacePrograms = useMemo<readonly SpaceProgramEntry[]>(
    () =>
      localPrograms.flatMap((entry) =>
        entry.manifest.apps.map((app) => ({
          pluginId: entry.handle.pluginId,
          workflowStepId: app.id,
          appId: app.id,
          label: resolveManifestLabel(app.label as LocalizedLabel | string, uiTerminology, uiLocale),
          breadcrumb: app.breadcrumb,
          yields: "",
        })),
      ),
    [localPrograms, uiLocale, uiTerminology],
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
    const plugin = core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    let cancelled = false;
    const owner = { pluginId: session.pluginId, instanceId: session.instanceId };
    const orderAtRequest = historyOrderByProgramRef.current.get(programHistoryKeyV1(owner)) ?? 0;
    void plugin.readHistory(session.instanceId).then((snapshot) => {
      if (cancelled) return;
      if ((historyOrderByProgramRef.current.get(programHistoryKeyV1(owner)) ?? 0) > orderAtRequest) {
        return;
      }
      applyHistoryPatch(snapshot, true, owner);
    }).catch((error) => undefined);
    return () => {
      cancelled = true;
    };
    // 🧾️ Keyed on the plugin INSTANCE, not the `session` object: a full history snapshot is only ever
    // owed when the instance being shown changes. Depending on the whole session re-read the projection
    // on every session mint — see `applyHostEffects`'s `viewStateRewrittenByEffects` for the churn that
    // used to produce (one `readHistory` guest round trip per dispatch).
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [applyHistoryPatch, session?.pluginId, session?.instanceId]);
  const { windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId, toolMeasuresByToolId, panelBodyStoreByKey, appLabelsOverlay, appCatalogue } = shellState.windowUi;
  const { spawnedWindowActivityByWindowId, spawnedWindowFault, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;
  const { foldedByWindowId: actionPaneFoldedByWindowId, expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId, activeToolId } = shellState.actionPane;
  const { expandedCommandId, stagedArgsByCommandId: commandStagedArgsByCommandId } = shellState.commandPanel;
  const { panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId, shellLayout, activeExampleId, mobilePanelPath, mobilePanelVisible, extraWindowInstances, windowTitlesById, windowIconsById } = shellState.layout;
  const world3dWindowViewRegistry = useMemo(() => new World3dWindowViewRegistryV1(), []);
  const primaryWorld3dOwner = session ? `${session.pluginId}:${session.app.id}:${session.instanceId}` : null;
  const previousPrimaryWorld3dOwnerRef = useRef<{ readonly owner: string | null; readonly windowIds: readonly string[] }>({ owner: null, windowIds: [] });
  const previousPrimaryWorld3dOwner = previousPrimaryWorld3dOwnerRef.current;
  if (previousPrimaryWorld3dOwner.owner !== primaryWorld3dOwner) {
    clearPendingWorldProjections(previousPrimaryWorld3dOwner.windowIds);
    world3dWindowViewRegistry.retireOwner(previousPrimaryWorld3dOwner.owner);
  }
  previousPrimaryWorld3dOwnerRef.current = {
    owner: primaryWorld3dOwner,
    windowIds: session ? [...session.app.windowKinds.map((kind) => kind.id), ...extraWindowInstances.map((instance) => instance.id)] : [],
  };
  const world3dWindowViews = primaryWorld3dOwner ? world3dWindowViewRegistry.scope(primaryWorld3dOwner) : null;
  const { searchOpen, findOpen, introductionStepIndex, introductionCompletedInteractions, dialog: overlayDialog, transientNotice, openWithFocusRole } = shellState.overlays;
  const dialogOpeningRef = useRef(0);
  const liveDialogRef = useRef(overlayDialog);
  liveDialogRef.current = overlayDialog;
  const { activeTutorialId, playing: tutorialPlaying, rate: tutorialRate, muted: tutorialMuted, captionsOn: tutorialCaptionsOn, recording: tutorialRecording, deviated: tutorialDeviated } = shellState.tutorial;
  /** 🗂️ The user's saved layouts per app id: the projection of their `setNamedLayout` preference events, so a layout saved
   * on one device appears on every device the preference lane reaches. */
  const [uiNamedLayouts, setUiNamedLayouts] = useState(() => readUiPreferences(scope.storage).namedLayouts);
  /** ✍️ The one preference commit ({@link commitUiPreference}), for callbacks built before it is declared. */
  const commitUiPreferenceRef = useRef<(mutation: UiPreferencesConfigMutation) => void>(() => {});
  useEffect(
    () =>
      subscribeUiPreferences(scope.storage, (preferences) => {
        const resolved = resolveUiPreferences(preferences, {
          appearance: "system",
          layout: "desktop",
          driverId: DEFAULT_UI_DRIVER.id,
          locale: scope.i18n.language as UiLocale,
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
        setUiNamedLayouts(resolved.namedLayouts);
      }),
    [locks.appearance, locks.locale, locks.terminology, locks.themeId, scope.storage],
  );
  const { syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId } = shellState.sync;
  const { mergePolicy, conflicts, selectedConflictId } = shellState.merge;
  /** 💡️ The one document whose host-owned inference port is currently live, and its exact status. */
  const inferencePortRuntimeKey = shellState.inference.operationRuntimeKey;
  const inferencePort = inferencePortRuntimeKey === null ? undefined : shellState.inference.portByRuntimeKey[inferencePortRuntimeKey];
  const refreshGenerationRef = useRef(0);
  const replaceBodiesGenerationRef = useRef(0);
  const clipboardFragmentRef = useRef<unknown>(undefined);
  const appRegistrationsJsonRef = useRef<string | null>(null);
  const spawnedRefreshSlotsRef = useRef(new Map<string, SpawnedRefreshSlotV1>());
  const contributorInstancesRef = useRef<Map<string, ContributorInstance>>(new Map());
  const contributorPassesRef = useRef<Map<string, symbol>>(new Map());
  const externalSlotNodesRef = useRef<Map<string, BuiltNode>>(new Map());
  const layoutSeedKeyRef = useRef<string | null>(null);
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
  const operationSettlersRef = useRef(new Map<string, (completion: PluginOperationCompletion) => void>());
  const settledOperationsRef = useRef<PluginOperationCompletion[]>([]);
  /** 🏁️ Resolves every waiter for `operation`, or records it as already settled for a waiter that has
   * not registered yet. Called from the operation-completion subscription. */
  const settleOperation = useCallback((completion: PluginOperationCompletion) => {
    const key = operationSettleKeyV1(completion.instanceId, completion.operation);
    const settler = operationSettlersRef.current.get(key);
    if (settler) {
      settler(completion);
      return;
    }
    const settled = settledOperationsRef.current;
    settled.push(completion);
    if (settled.length > OPERATION_SETTLE_RING_SLOTS) settled.splice(0, settled.length - OPERATION_SETTLE_RING_SLOTS);
  }, []);
  /** 🏁️ The promise `onAction` returns: settles when the operation the admitting reply started reaches
   * its terminal completion. An action that started no operation (a framework-reserved verb, a
   * config-only emit, a rejected admission) settles at once, and a watchdog bounds a completion that
   * never arrives so a caller can never wedge forever on a lost frame. */
  const awaitOperationSettle = useCallback((instanceId: number, operation: bigint | undefined): Promise<PluginOperationCompletion | null> => {
    if (operation === undefined) return Promise.resolve(null);
    const key = operationSettleKeyV1(instanceId, operation);
    const settled = settledOperationsRef.current;
    const alreadySettled = settled.findIndex((completion) => completion.instanceId === instanceId && completion.operation === operation);
    if (alreadySettled !== -1) {
      return Promise.resolve(settled.splice(alreadySettled, 1)[0] ?? null);
    }
    const settlers = operationSettlersRef.current;
    if (settlers.has(key) || settlers.size >= OPERATION_SETTLE_WAITER_SLOTS) return Promise.resolve(null);
    return new Promise<PluginOperationCompletion | null>((resolve) => {
      const timer = setTimeout(() => {
        operationSettlersRef.current.delete(key);
        resolve(null);
      }, OPERATION_SETTLE_WATCHDOG_MS);
      settlers.set(key, (completion) => {
        clearTimeout(timer);
        operationSettlersRef.current.delete(key);
        resolve(completion);
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
  /** 🪦️ The same drop for the OTHER way an instance goes away. `sealedInstancesRef` is written by the
   * session-switch gate only, so a hot-swap — which destroys the session-owning plugin's live instance
   * without any switch at all — left every late arrival to fall through to a stack. The runtime layer
   * that owns the dispatch queue now says so on the failure itself
   * ({@link isPluginInstanceRetiredV1}), so this reads the answer rather than keeping a second ledger. */
  const dropForRetiredInstance = useCallback((target: { readonly pluginId: string; readonly instanceId: number }, what: string, error: unknown): boolean => {
    if (!isPluginInstanceRetiredV1(error)) return false;
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
  /** 🪐️ The space the user is in, as state for what renders it (the hub workspace's active space): the ref is what
   * the async route and session code reads mid-flight, this is what a render reads. The hub pane used to read the ref
   * during render, so a space re-opened after an identity re-establishment never reached an already-open pane — it kept
   * saying "Open a space to manage the agents that work in it." while the user was in a space (ticket 26/09/23 U5, G10
   * S4 relay). Every write goes through {@link rememberOpenSpaceId}. */
  const [openSpaceId, setOpenSpaceId] = useState<string | null>(null);
  const rememberOpenSpaceId = useCallback((spaceId: string | null): void => {
    openSpaceIdRef.current = spaceId;
    setOpenSpaceId(spaceId);
  }, []);
  const openInstanceIdRef = useRef<string | null>(null);
  const sessionRef = useRef<ActiveSession | null>(null);
  //#region 🪟️TreeWindows
  /** 🪟️ Host-owned tree expansion + on-screen row windows for every panel body, and the scheduler that
   * turns a change into exactly one partial refresh of that body. This is the ONLY store of guest-tree
   * expansion: `<Tree>`'s own per-mount `TreeStateProvider` state dies with every body refresh and the
   * guest keeps none, which is why an opened container used to snap shut on the next unrelated refresh
   * (📓️audit-host-tree-pipeline.md §2). Keyed by AUTHORED node key, so it survives a store re-mint. */
  const treeWindowSchedulerRef = useRef<TreeWindowSchedulerV1 | null>(null);
  /** 🔁️ `refreshUi` is declared far below (it closes over the whole refresh lane); the scheduler's own
   * refresh closure is created here, on the first render, so it reaches it through this ref. */
  const refreshUiRef = useRef<((nextSession: ActiveSession, scopeArg?: UiDirtyScope) => Promise<unknown>) | null>(null);
  /** 🌲️ Per-tab `TreePanelConfig` reuse — see {@link PanelTreeConfigCacheV1}. Reset with the scheduler. */
  const panelTreeConfigCacheRef = useRef<PanelTreeConfigCacheV1>(new Map());
  /** 🪟️ Bumped by an OPEN toggle so the panel-tab memos re-run for it. A window report needs no bump:
   * it changes the guest body, which the panel's own body store reloads in place. */
  const [treeWindowGeneration, setTreeWindowGeneration] = useState(0);
  if (!treeWindowSchedulerRef.current) {
    treeWindowSchedulerRef.current = createTreeWindowSchedulerV1({
      refresh: (bodyKey) => {
        const live = sessionRef.current;
        const refresh = refreshUiRef.current;
        if (!live || !refresh) {
          treeWindowSchedulerRef.current?.settled(bodyKey);
          return;
        }
        const windowBody = live.app.windowKinds.some((kind) => kind.bodyKey === bodyKey);
        void refresh(live, windowBody ? { kind: "partial", windowBodies: [bodyKey] } : { kind: "partial", panelBodies: [bodyKey] })
          .catch(logUnlessRetiredV1("[os-shell] tree window refresh failed"))
          .finally(() => treeWindowSchedulerRef.current?.settled(bodyKey));
      },
    });
  }
  const treeWindowHost = useMemo<TreeWindowHostV1>(
    () => ({
      openStatesFor: (bodyKey) => treeWindowSchedulerRef.current!.openStatesFor(bodyKey),
      setOpen: (bodyKey, nodeKey, open) => {
        treeWindowSchedulerRef.current!.setOpen(bodyKey, nodeKey, open);
        setTreeWindowGeneration((generation) => generation + 1);
      },
      reportWindows: (bodyKey, requests, viewportRows) => treeWindowSchedulerRef.current!.reportWindows(bodyKey, requests, viewportRows),
    }),
    [],
  );
  /** 🪟️ The streaming channel of one WINDOW body — the same host-owned windows and the same scheduler the
   * panel bodies speak, addressed by the window kind's `bodyKey`, so a windowed table (or tree) in a window
   * streams its rows exactly like a panel tree does. One stable value per body key. */
  const windowTreeContextsRef = useRef(new Map<string, TreeWindowContextValue>());
  const windowTreeContext = useCallback(
    (bodyKey: string): TreeWindowContextValue => {
      const known = windowTreeContextsRef.current.get(bodyKey);
      if (known) return known;
      const created = createTreeWindowContextV1(bodyKey, treeWindowHost);
      windowTreeContextsRef.current.set(bodyKey, created);
      return created;
    },
    [treeWindowHost],
  );
  //#endregion 🪟️TreeWindows
  /** 🧭️ The route and every identity re-establishment share ONE serial lane ({@link createShellSessionLaneV1}):
   * `applyShellUri` never re-enters (lane 5-A's measured `Maximum call stack size exceeded` came from a
   * reentrant chain, ticket 26/08/17), and — unlike the depth guard that used to stand here — a route asked
   * for mid-flight is applied after the flight instead of being dropped (ticket 26/09/23 U5). The lane
   * applies the NEWEST `applyShellUri`, so a queued route sees the session the flight left behind.
   *
   * Only routes that REPLACE the session enter it. `/hub` (AU2's hub connection surface: sign in, browse/switch/create
   * a space, invite, redeem) is an overlay over whatever is open — the route effect opens and closes it on the spot,
   * so it can never wait behind a session switch (G10 S4: "Open Hub and Spaces" set `/hub` and showed nothing while a
   * space route was still in flight). A route's document opening is started in lane order but not awaited there: a hub
   * socket that is slow to attach must not hold every later route. */
  const applyShellUriRef = useRef<(uri: string, preservedViewState?: ViewModel) => Promise<boolean>>(async () => false);
  /** 🧭️ The route in effect ({@link createShellRouteLedgerV1}): the route effect never re-applies it; a replaced session
   * ({@link establishPrimarySession}) or an explicit navigation ({@link navigateShellUri}) invalidates it. */
  const shellRouteLedgerRef = useRef(createShellRouteLedgerV1());
  const shellSessionLaneRef = useRef(createShellSessionLaneV1<{ readonly uri: string; readonly preservedViewState?: ViewModel }>(async (request) => {
    const generation = shellRouteLedgerRef.current.begin();
    shellRouteLedgerRef.current.settle(generation, request.uri, await applyShellUriRef.current(request.uri, request.preservedViewState));
  }));
  //#region 🔖️BuiltNodeStores
  /** 🧬️ Per-window/panel `📃️UiDocumentStore`s, keyed the way `windowUiByWindowId`/`panelBodyStoreByKey` are
   * (a spawned window's store, `spawned:<windowId>`, is the ONLY holder of its body) — one stable store instance per key, reloaded via
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

  return {
    catalog, pluginFilter, plugins, surfaceSessionFactories, backboneWorkerFactory, documentServices, hostPorts, appId, appRole,
    brand, suppressAutoIntroduction, scope, localDocumentOwnerRegistry, shellContextMenuTitleLabel, hostConfig, hostMode,
    measuredDevice, mobile, locks, defaults, ephemeral, shellState, dispatch, localInteraction, historyStore,
    historyOrderRef, historyOrderByProgramRef, exampleOptionsRef, lastDispatchedExampleIdRef, localHistoryOrderRef,
    loadedPlugins, pluginStatusById, pluginSupervisorById, session, error, sessionFault, instanceFault, localPrograms,
    uiAppearance, uiLayout, uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale, uiTerminology, uiThemeId,
    uiCustomThemes, uiThemeDraft, uiKeybindingOverrides, applyHistoryPatch, leftoverInspectionEpoch,
    leftoverInspectionIdsRef, leftoverReplaceRefreshBodiesV1, forceReloadLiveUiStoresV1, applyLeftoverInteractionView,
    refreshHistorySnapshot, hostPlugin, hostApp, landingApp, landingAppId, hostAppId, hostControllerId,
    hostCatalogueTabId, spacePrograms, windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId,
    toolMeasuresByToolId, panelBodyStoreByKey, appLabelsOverlay, appCatalogue, spawnedWindowActivityByWindowId,
    spawnedWindowFault, spawnedWindowEngagements, spawnedWindowMeasures, actionPaneFoldedByWindowId,
    actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, activeUtilityByWindowId, activeToolId, expandedCommandId,
    commandStagedArgsByCommandId, panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId, shellLayout,
    activeExampleId, mobilePanelPath, mobilePanelVisible, extraWindowInstances, windowTitlesById, windowIconsById,
    world3dWindowViewRegistry, primaryWorld3dOwner, world3dWindowViews, searchOpen, findOpen, introductionStepIndex,
    introductionCompletedInteractions, overlayDialog, transientNotice, openWithFocusRole, dialogOpeningRef,
    liveDialogRef, activeTutorialId, tutorialPlaying, tutorialRate, tutorialMuted, tutorialCaptionsOn, tutorialRecording,
    tutorialDeviated, uiNamedLayouts, commitUiPreferenceRef, syncBackboneUri, syncCardKind, syncDraftPath,
    syncStatusByDocumentId, mergePolicy, selectedConflictId, inferencePortRuntimeKey, inferencePort,
    refreshGenerationRef, replaceBodiesGenerationRef, clipboardFragmentRef, appRegistrationsJsonRef,
    spawnedRefreshSlotsRef, contributorInstancesRef, contributorPassesRef, externalSlotNodesRef, layoutSeedKeyRef,
    extraWindowCounterRef, shellContextMenu, setShellContextMenu, extraWindowInstancesRef, settleOperation,
    awaitOperationSettle, sessionWorkRef, sealedInstancesRef, sessionSwitchGateRef, surfaceSwitchBusy,
    setSurfaceSwitchBusy, dropForSealedInstance, dropForRetiredInstance, setWindowTitle, setWindowIcon,
    uiRefreshCacheRef, spawnedUiRefreshCacheRef, spawnedLayoutSeedRef, openSpaceIdRef, openSpaceId, rememberOpenSpaceId,
    openInstanceIdRef, sessionRef, treeWindowSchedulerRef, refreshUiRef, panelTreeConfigCacheRef, treeWindowGeneration,
    treeWindowHost, windowTreeContext, applyShellUriRef, shellRouteLedgerRef, shellSessionLaneRef,
    builtNodeStoreCacheRef, builtNodeStoreFor
  };
}
//#endregion FrameworkOsShell

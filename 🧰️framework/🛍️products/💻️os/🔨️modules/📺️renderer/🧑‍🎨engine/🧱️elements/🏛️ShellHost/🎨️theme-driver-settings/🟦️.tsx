/** 🎨️ Theme and driver mutators, keybinding overrides, the preference lane, the settings and marketplace hosts and the task manager sources.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useThemeDriverSettings` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState
} from "react";
import { type LocalizedLabel, type VideoRenderJobRow } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
// #region 🔌️Adapters
import {
  type UiPreferencesConfigMutation,
  setAppearance,
  setCustomDriver,
  setCustomTheme,
  setDriver,
  setKeybindingOverride as setKeybindingOverrideMutation,
  setLayout,
  setLocale,
  setTerminology,
  setTheme
} from "@semio-tech/framework/config/mutations";
import {
  buildKeysByActionId,
  builtinUiDrivers,
  builtinUiThemes,
  composeControlKeybindings,
  DEFAULT_UI_DRIVER,
  type ElementsSurfaceAppearance,
  patchUiThemeGeometry,
  parseUiTheme,
  serializeUiTheme,
  type ThemeAppearanceName,
  type ThemePaletteGroup,
  UI_TERMINOLOGY_NATIVE,
  type UiChromeLayout,
  type UiDriver,
  type UiLocale,
  type UiTheme
} from "@semio-tech/ui-react";
import {
  canonicalUiDriver,
  canonicalUiTheme,
  commitUiPreferenceOnLaneV1,
  foldUiPreferenceLanePageV1,
  readUiPreferenceLane,
  UI_PREFERENCE_UNSHARED_TEXT_V1,
  UI_PREFERENCES_LANE_SCHEMA_V1,
  uiPreferenceEnvelopeTextV1,
  uiPreferenceLaneKeyV1
} from "../../../🎚️UiPreferences/🟦️.ts";
import {
  appBreadcrumb,
  buildOsCommands,
  downloadMediaExport,
  manifestLabelTextV1,
  requestFileOpen,
  resolveAppLabel,
  resolveAppBreadcrumb,
  resolveManifestLabel,
  actionBindingToActionDescriptor
} from "../../🛠️ShellHelpers/🟦️.tsx";
import { toolRunPanelTasksV1, type ToolRunPanelControlV1 } from "../../🛠️ShellHelpers/⏯️tool-run-panel/🟦️.ts";
import {
  createFrameworkChatPanelTab,
  createFrameworkDisplayPanelTabs,
  createFrameworkMarketplacePanelTab,
  createFrameworkTaskManagerPanelTab,
  createFrameworkSettingsPanelTab,
  FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID,
  FRAMEWORK_SETTINGS_PANEL_ID,
  type MarketplaceExtensionEntry,
  type MarketplaceHostApi,
  type MarketplacePluginEntry,
  type SettingsHostApi
} from "../../📌️ChromePanels/🟦️.tsx";
// #endregion 🔌️Adapters
import { hubCatalogOnlyPluginsV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import type { TrustedPluginModuleIndexV1 } from "@semio-tech/framework/plugin-module";
import { AgentChatPanel } from "../../💬️AgentChatPanel/🟦️.tsx";
import {
  TaskManagerWindow,
  installTasksV1,
  spawnedJobTasksV1,
  taskManagerToolRunIdV1,
  toolCallTasksV1,
  toolRunTasksV1,
  videoRenderExportTasksV1,
  type TaskManagerDocumentTransferV1,
  type TaskManagerSourcesV1,
  type TaskManagerTaskV1
} from "../../🧵️TaskManager/🟦️.tsx";
import { cancelVideoRenderExportV1, subscribeVideoRenderExportJobsV1, videoRenderExportJobsSnapshotV1 } from "../../🎥️VideoRenderHost/🟦️.ts";
import {
  cancelSpawnedJobV1,
  pluginRuntimeActivationRegistryV1,
  spawnedJobsSnapshotV1,
  subscribeSpawnedJobsV1,
  type SpawnedJobRowV1
} from "../../🔌️PluginRuntime/🟦️.tsx";
import { HUB_CATALOG_REFRESH_MS } from "../🔌️extension-runtime/🟦️.ts";
import { mintPreferenceLaneIdV1 } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterArtifactOpening } from "../📬️artifact-opening/🟦️.ts";

export type ShellCoreAfterThemeDriverSettings = ShellCoreAfterArtifactOpening & ReturnType<typeof useThemeDriverSettings>;

export function useThemeDriverSettings(core: ShellCoreAfterArtifactOpening) {
  const {
    catalog, scope, mobile, locks, dispatch, loadedPlugins, pluginStatusById, session, uiAppearance, uiLayout,
    uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale, uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft,
    uiKeybindingOverrides, appLabelsOverlay, commitUiPreferenceRef, mergePolicy, uiTheme, uiDriver, backboneWorkerRef,
    agentBridge, identity, identityRef, verifiedSessionAuthority, handlePreferenceLaneMessageRef, uiLocaleRef,
    pluginInstallAbortsRef, ensureBackboneWorker, dockLayoutStore, dockUiStateStore, registry, primaryPluginId,
    hubPluginSource, extensionRegistry, pluginSource, installPlugin, installingPluginIds, reloadPlugin, uninstallPlugin,
    extensionLedger, extensionIdSet, installExtension, installExtensionFromFile, uninstallExtension, setExtensionEnabled,
    focusedProgram, toolRunPanelNode, activeIntroduction, activeTutorials, tutorialRecorderAvailable, noteShellCommand,
    onActionStable, displayHostRef, displayHost, dispatchSetMergePolicy, defaultAppsHost, defaultAppsHostRef,
    conflictsHost, conflictsHostRef, hasOpenArtifactSurfaces, showTransientNotice
  } = core;
  //#region FrameworkOsShell
  //#region 🔖️ThemeMutators
  const uiThemeBase = uiThemeDraft ?? uiTheme;
  const uiThemeDirty = uiThemeDraft !== null;
  const uiThemeList = useMemo((): readonly UiTheme[] => [...builtinUiThemes(), ...Object.values(uiCustomThemes)], [uiCustomThemes]);
  const uiDriverList = useMemo((): readonly UiDriver[] => [...builtinUiDrivers(), ...Object.values(uiCustomDrivers)], [uiCustomDrivers]);
  const keysByActionId = useMemo(() => buildKeysByActionId(session?.app.keybindings ?? []), [session?.app.keybindings]);
  const controlKeybindings = useMemo(() => composeControlKeybindings(keysByActionId, uiKeybindingOverrides), [keysByActionId, uiKeybindingOverrides]);
  /** ⌨️ Settings ▸ Hotkeys names a bound app action or OS command by its own manifest label in the live language; the
   * rows used to show humanized control ids ("Undo", "Toggle Panel") that stayed English in German (ticket 26/09/23 U5). */
  const controlKeybindingLabel = useCallback((controlId: string): string | null => {
    const action = session?.app.actions.find((candidate) => candidate.id === controlId);
    if (action !== undefined) return resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label as LocalizedLabel | string, uiTerminology, uiLocale));
    const command = session?.app.commands.find((candidate) => candidate.id === controlId);
    if (command !== undefined) return resolveManifestLabel(command.label as LocalizedLabel | string, uiTerminology, uiLocale);
    return null;
  }, [appLabelsOverlay, session?.app.actions, session?.app.commands, uiLocale, uiTerminology]);
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
      noteShellCommand(commandId, manifestLabelTextV1(rawLabel ?? commandId, uiTerminology), detail);
    },
    [osCommands, noteShellCommand, uiTerminology],
  );

  const commitUiPreference = useCallback(
    (mutation: UiPreferencesConfigMutation) => {
      if (
        (mutation.mutation === "setAppearance" && locks.appearance) ||
        (mutation.mutation === "setLocale" && locks.locale) ||
        (mutation.mutation === "setTerminology" && locks.terminology) ||
        (mutation.mutation === "setTheme" && locks.themeId)
      ) return;
      const { queued, unshared } = commitUiPreferenceOnLaneV1(scope.storage, mutation, preferenceLaneRef.current, mintPreferenceLaneIdV1);
      if (queued !== null) flushPreferenceLaneRef.current();
      if (unshared) showTransientNotice(UI_PREFERENCE_UNSHARED_TEXT_V1[uiLocaleRef.current === "de" ? "de" : "en"], "warning");
    },
    [locks, scope.storage, showTransientNotice],
  );
  commitUiPreferenceRef.current = commitUiPreference;

  /** 🌐️ The signed-in user's preference lane (ticket 26/09/23 U5): while a verified hub session stands, the worker pages
   * this user's `user.preference-recorded` events from the lane's frontier and wakes on every directory head past it;
   * each page folds into this device's log in hub order ({@link foldUiPreferenceLanePageV1}), and every change of a
   * `persistedShared` preference made here is recorded as a `record-user-preference` command, re-sent under its own
   * request id until a page acknowledges it — so a change made offline reaches every device once the hub is back. A
   * `persistedLocalOnly` preference (the chrome layout) never leaves the device. */
  const preferenceLaneRef = useRef<string | null>(null);
  const preferenceLaneInFlightRef = useRef(new Set<string>());
  const flushPreferenceLaneRef = useRef<() => void>(() => undefined);
  flushPreferenceLaneRef.current = () => {
    const lane = preferenceLaneRef.current;
    const worker = backboneWorkerRef.current;
    if (lane === null || worker === null) return;
    for (const entry of readUiPreferenceLane(scope.storage, lane).outbox) {
      if (preferenceLaneInFlightRef.current.has(entry.requestId)) continue;
      preferenceLaneInFlightRef.current.add(entry.requestId);
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "preference-lane-record", requestId: entry.requestId, schema: UI_PREFERENCES_LANE_SCHEMA_V1, mutation: uiPreferenceEnvelopeTextV1(entry) }) });
    }
  };
  handlePreferenceLaneMessageRef.current = (message) => {
    const lane = preferenceLaneRef.current;
    if (message.kind === "preference-lane-page") {
      if (lane === null || identityRef.current?.userId !== message.userId) return;
      foldUiPreferenceLanePageV1(scope.storage, lane, message);
      flushPreferenceLaneRef.current();
      return;
    }
    if (message.requestId !== null) preferenceLaneInFlightRef.current.delete(message.requestId);
  };
  const preferenceLaneUserId = verifiedSessionAuthority !== null && identity?.hubBaseUrl ? identity.userId : null;
  const preferenceLaneHubBaseUrl = verifiedSessionAuthority !== null ? identity?.hubBaseUrl ?? null : null;
  useEffect(() => {
    if (preferenceLaneUserId === null || preferenceLaneHubBaseUrl === null) return;
    const lane = uiPreferenceLaneKeyV1(preferenceLaneHubBaseUrl, preferenceLaneUserId);
    const worker = ensureBackboneWorker();
    preferenceLaneRef.current = lane;
    preferenceLaneInFlightRef.current.clear();
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "preference-lane-open", baseUrl: preferenceLaneHubBaseUrl, userId: preferenceLaneUserId, after: readUiPreferenceLane(scope.storage, lane).afterSeq }) });
    return () => {
      if (preferenceLaneRef.current === lane) preferenceLaneRef.current = null;
      preferenceLaneInFlightRef.current.clear();
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "preference-lane-close" }) });
    };
  }, [ensureBackboneWorker, preferenceLaneHubBaseUrl, preferenceLaneUserId, scope.storage]);

  const draftThemePatch = useCallback(
    (patch: (next: UiTheme) => void) => {
      const next = patchUiThemeGeometry(uiThemeBase, patch);
      if (next) dispatch({ type: "SET_UI_THEME_DRAFT", value: next });
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
      controlKeybindingLabel,
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
      controlKeybindingLabel,
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

  /** 🌎️ The serving hub's current catalog of plugin modules, read when the shell starts and once a minute after (a
   * catalog generation changes rarely and publishes no stream); `null` without a hub or before its first answer. A read
   * that fails keeps the last catalog: one unanswered read is not a hub without plugins. */
  const [hubCatalog, setHubCatalog] = useState<TrustedPluginModuleIndexV1 | null>(null);
  useEffect(() => {
    if (hubPluginSource === null) {
      setHubCatalog(null);
      return;
    }
    const abort = new AbortController();
    const read = (): void => {
      void hubPluginSource.catalog(abort.signal).then((index) => {
        if (!abort.signal.aborted && index !== null) setHubCatalog((current) => (current?.generationId === index.generationId ? current : index));
      }, () => undefined);
    };
    read();
    const timer = setInterval(read, HUB_CATALOG_REFRESH_MS);
    return () => {
      abort.abort();
      clearInterval(timer);
    };
  }, [hubPluginSource]);
  /** 🧩️ The shell's plugin registry is this device's build plus every plugin the serving hub's catalog carries that the build
   * lacks ({@link hubCatalogOnlyPluginsV1}): a hub-only plugin is listed, installs from the hub (verified, into the store) through
   * the same `installPlugin`, and then runs like any other device program. */
  const hubOnlyPlugins = useMemo(() => (hubCatalog === null ? [] : hubCatalogOnlyPluginsV1(hubCatalog, new Set([...registry, ...extensionRegistry].map((entry) => entry.pluginId)))), [hubCatalog, registry, extensionRegistry]);
  const marketplaceHostRef = useRef<MarketplaceHostApi | null>(null);
  const marketplaceHost: MarketplaceHostApi = useMemo(
    () => ({
      plugins: [
        ...registry.filter((entry) => !extensionIdSet.has(entry.pluginId)).map((entry) => ({ pluginId: entry.pluginId, sourceId: pluginSource.id })),
        ...hubOnlyPlugins.map((entry) => ({ pluginId: entry.pluginId, sourceId: `hub · ${entry.generationId.slice(0, 12)}` })),
      ].map(({ pluginId, sourceId }): MarketplacePluginEntry => {
        const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === pluginId);
        return {
          pluginId,
          label: loadedEntry?.manifest.label ?? pluginId,
          version: loadedEntry?.manifest.version,
          status: pluginStatusById[pluginId] ?? "available",
          sourceId,
          canUninstall: pluginId !== primaryPluginId && session?.pluginId !== pluginId,
        };
      }),
      extensions: (() => {
        const byId = new Map<string, MarketplaceExtensionEntry>();
        for (const target of catalog.extensions) {
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
      catalog,
      registry,
      hubOnlyPlugins,
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
  const frameworkMarketplaceTab = useMemo(() => createFrameworkMarketplacePanelTab(() => marketplaceHostRef.current), [marketplaceHost, uiLocale]);
  const frameworkChatTab = useMemo(
    () => createFrameworkChatPanelTab(() => <AgentChatPanel status={agentBridge.status} presence={agentBridge.presence} versionMismatch={agentBridge.versionMismatch} conversation={agentBridge.conversation} onSendMessage={agentBridge.sendAgentMessage} onCancelToolCall={agentBridge.cancelToolCall} onResolveApproval={agentBridge.resolveApproval} />),
    [agentBridge.presence, agentBridge.status, agentBridge.versionMismatch, agentBridge.conversation, agentBridge.sendAgentMessage, agentBridge.cancelToolCall, uiLocale],
  );
  /** 🧵️ The `os.task-manager` window — the one central progress + cancel surface: every spawned plugin job
   * (the plugin runtime's own ledger), every plugin installation and every agent tool call still running,
   * each cancellable, above the actor table of the runtime this tab's plugins actually run on. The window
   * reads through stable `sources`, so a job advancing re-renders the window, never the dock. */
  const taskManagerShellTasksRef = useRef<readonly TaskManagerTaskV1[]>([]);
  const taskManagerListenersRef = useRef(new Set<() => void>());
  const taskManagerTasksCacheRef = useRef<{ readonly jobs: readonly SpawnedJobRowV1[]; readonly exports: readonly VideoRenderJobRow[]; readonly shell: readonly TaskManagerTaskV1[]; readonly transfers: readonly TaskManagerTaskV1[]; readonly tasks: readonly TaskManagerTaskV1[] } | null>(null);
  const documentTransfersRef = useRef(new Map<string, { readonly transfer: TaskManagerDocumentTransferV1; readonly abort: AbortController }>());
  const documentTransferTasksRef = useRef<readonly TaskManagerTaskV1[]>([]);
  const installStartedAtRef = useRef(new Map<string, number>());
  const toolRunStartedAtRef = useRef(new Map<string, number>());
  const toolRunControlsRef = useRef(new Map<string, { readonly pause: ToolRunPanelControlV1 | null; readonly resume: ToolRunPanelControlV1 | null; readonly abort: ToolRunPanelControlV1 | null }>());
  const fireToolRunControl = (task: TaskManagerTaskV1, control: "pause" | "resume" | "abort"): void => {
    const owned = toolRunControlsRef.current.get(task.id)?.[control];
    if (owned && !owned.disabled) void onActionStable(actionBindingToActionDescriptor(owned.binding));
  };
  const taskManagerCancelRef = useRef<(task: TaskManagerTaskV1) => void>(() => undefined);
  taskManagerCancelRef.current = (task) => {
    if (task.lane === "job") cancelSpawnedJobV1(task.id.slice("job:".length));
    else if (task.lane === "activation") pluginInstallAbortsRef.current.get(task.id.slice("install:".length))?.abort();
    else if (task.lane === "toolRun") fireToolRunControl(task, "abort");
    else if (task.lane === "documentTransfer") documentTransfersRef.current.get(task.id.slice("documentTransfer:".length))?.abort.abort();
    else if (task.lane === "export") cancelVideoRenderExportV1(task.id);
    else agentBridge.cancelToolCall(task.id.slice("tool:".length));
  };
  const taskManagerSuspendRef = useRef<(task: TaskManagerTaskV1, suspend: boolean) => void>(() => undefined);
  taskManagerSuspendRef.current = (task, suspend) => {
    if (task.lane === "toolRun") fireToolRunControl(task, suspend ? "pause" : "resume");
  };
  const taskManagerSources = useMemo((): TaskManagerSourcesV1 => ({
    registry: pluginRuntimeActivationRegistryV1,
    tasks: () => {
      const jobs = spawnedJobsSnapshotV1();
      const exports = videoRenderExportJobsSnapshotV1();
      const shell = taskManagerShellTasksRef.current;
      const transfers = documentTransferTasksRef.current;
      const cached = taskManagerTasksCacheRef.current;
      if (cached && cached.jobs === jobs && cached.exports === exports && cached.shell === shell && cached.transfers === transfers) return cached.tasks;
      const tasks = [...spawnedJobTasksV1(jobs), ...videoRenderExportTasksV1(exports, uiLocaleRef.current), ...shell, ...transfers];
      taskManagerTasksCacheRef.current = { jobs, exports, shell, transfers, tasks };
      return tasks;
    },
    subscribe: (listener) => {
      taskManagerListenersRef.current.add(listener);
      const unsubscribeJobs = subscribeSpawnedJobsV1(listener);
      const unsubscribeExports = subscribeVideoRenderExportJobsV1(listener);
      return () => {
        taskManagerListenersRef.current.delete(listener);
        unsubscribeJobs();
        unsubscribeExports();
      };
    },
    cancel: (task) => taskManagerCancelRef.current(task),
    suspend: (task) => taskManagerSuspendRef.current(task, true),
    resume: (task) => taskManagerSuspendRef.current(task, false),
  }), []);
  useEffect(() => {
    const startedAt = installStartedAtRef.current;
    for (const pluginId of [...startedAt.keys()]) if (!installingPluginIds.includes(pluginId)) startedAt.delete(pluginId);
    for (const pluginId of installingPluginIds) if (!startedAt.has(pluginId)) startedAt.set(pluginId, Date.now());
    const runs = toolRunPanelTasksV1(toolRunPanelNode);
    const program = { pluginId: focusedProgram?.pluginId ?? "", spawnedId: focusedProgram?.spawnedId ?? "" };
    const runIds = new Set(runs.map((run) => taskManagerToolRunIdV1(program.spawnedId, run.run)));
    const runStartedAt = toolRunStartedAtRef.current;
    for (const id of [...runStartedAt.keys()]) if (!runIds.has(id)) runStartedAt.delete(id);
    for (const id of runIds) if (!runStartedAt.has(id)) runStartedAt.set(id, Date.now());
    toolRunControlsRef.current = new Map(runs.map((run) => [taskManagerToolRunIdV1(program.spawnedId, run.run), { pause: run.pause, resume: run.resume, abort: run.abort }] as const));
    const toolRuns = toolRunTasksV1(runs.map((run) => ({ run: run.run, label: run.label, completed: run.completed, total: run.total, valueText: run.valueText, paused: run.pause === null })), program, runStartedAt);
    taskManagerShellTasksRef.current = [...installTasksV1(installingPluginIds, startedAt), ...toolCallTasksV1(agentBridge.conversation), ...toolRuns];
    for (const listener of [...taskManagerListenersRef.current]) listener();
  }, [agentBridge.conversation, installingPluginIds, toolRunPanelNode, focusedProgram?.pluginId, focusedProgram?.spawnedId]);
  const frameworkTaskManagerTab = useMemo(() => createFrameworkTaskManagerPanelTab(() => <TaskManagerWindow sources={taskManagerSources} />), [taskManagerSources, uiLocale]);

  return {
    keysByActionId, controlKeybindings, osCommands, commitUiPreference, frameworkDisplayTabs, frameworkSettingsTab,
    frameworkMarketplaceTab, frameworkChatTab, taskManagerListenersRef, documentTransfersRef, documentTransferTasksRef,
    frameworkTaskManagerTab
  };
  //#endregion FrameworkOsShell
}

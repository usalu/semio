/** 🎨️ The theme and driver state and the theme save callbacks of the theme driver settings hook. */
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { type LocalizedLabel } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { type UiPreferencesConfigMutation, setCustomDriver, setCustomTheme, setDriver, setKeybindingOverride as setKeybindingOverrideMutation, setTheme } from "@semio-tech/framework/config/mutations";
import { buildKeysByActionId, builtinUiDrivers, builtinUiThemes, composeControlKeybindings, DEFAULT_UI_DRIVER, patchUiThemeGeometry, parseUiTheme, serializeUiTheme, type ThemeAppearanceName, type ThemePaletteGroup, UI_TERMINOLOGY_NATIVE, type UiDriver, type UiTheme } from "@semio-tech/ui-react";
import { canonicalUiDriver, canonicalUiTheme, commitUiPreferenceOnLaneV1, foldUiPreferenceLanePageV1, LOCAL_ONLY_UI_PREFERENCE_VOCABULARY_V1, readUiPreferenceLane, UI_PREFERENCE_UNSHARED_TEXT_V1, UI_PREFERENCES_LANE_SCHEMA_V1, uiPreferenceEnvelopeTextV1, uiPreferenceLaneKeyV1 } from "../../../🎚️UiPreferences/🟦️.ts";
import { buildOsCommands } from "../../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { downloadMediaExport, requestFileOpen } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { manifestLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { resolveAppLabel, resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, FRAMEWORK_SETTINGS_PANEL_ID } from "../../../📌️ChromePanels/🆔️panel-ids/🟦️.ts";
import { type SettingsHostApi } from "../../../📌️ChromePanels/⚙️settings-general/🟦️.tsx";
import { mintPreferenceLaneIdV1 } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterArtifactOpening } from "../../📬️artifact-opening/🟦️.ts";

/** 🌈 Binds the theme and driver state and the theme selection callbacks. */
export function useThemeAndDriverState({ core }: { readonly core: ShellCoreAfterArtifactOpening }) {
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
    conflictsHost, conflictsHostRef, hasOpenArtifactSurfaces, showTransientNotice, hostPorts
  } = core;
  const preferenceVocabulary = hostPorts?.preferenceVocabulary ?? LOCAL_ONLY_UI_PREFERENCE_VOCABULARY_V1;
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
      const { queued, unshared } = commitUiPreferenceOnLaneV1(preferenceVocabulary, scope.storage, mutation, preferenceLaneRef.current, mintPreferenceLaneIdV1);
      if (queued !== null) flushPreferenceLaneRef.current();
      if (unshared) showTransientNotice(UI_PREFERENCE_UNSHARED_TEXT_V1[uiLocaleRef.current === "de" ? "de" : "en"], "warning");
    },
    [locks, scope.storage, showTransientNotice, preferenceVocabulary],
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
      foldUiPreferenceLanePageV1(preferenceVocabulary, scope.storage, lane, message);
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
  return { uiThemeBase, commitUiPreference, dispatch, uiThemeId, uiDriverDraft, uiDriver, noteOsCommand, uiDriverId, session, uiTerminology, uiDriverList, uiAppearance, uiLayout, mobile, dockLayoutStore, dockUiStateStore, uiLocale, uiThemeDirty, uiThemeList, setThemeId, setThemeColor, setThemeSpacing, setThemeFontStack, setThemeStroke, setThemeRadius, setThemeOpacity, setThemeMetric, setThemeAppearancePaint, resetTheme, controlKeybindings, controlKeybindingLabel, locks, mergePolicy, dispatchSetMergePolicy, displayHostRef, displayHost, defaultAppsHostRef, conflictsHostRef, defaultAppsHost, conflictsHost, hubPluginSource, registry, extensionRegistry, extensionIdSet, pluginSource, loadedPlugins, pluginStatusById, primaryPluginId, catalog, extensionLedger, installPlugin, uninstallPlugin, reloadPlugin, installExtension, installExtensionFromFile, uninstallExtension, setExtensionEnabled, agentBridge, onActionStable, pluginInstallAbortsRef, uiLocaleRef, installingPluginIds, toolRunPanelNode, focusedProgram, keysByActionId, osCommands };
}

/** 💾 Builds the theme save callbacks and the settings host ref. */
export function useThemeSaveAndSettingsHostRef({ uiThemeBase, commitUiPreference, dispatch, uiThemeId, uiDriverDraft, uiDriver, noteOsCommand, uiDriverId }: Pick<ReturnType<typeof useThemeAndDriverState>, "uiThemeBase" | "commitUiPreference" | "dispatch" | "uiThemeId" | "uiDriverDraft" | "uiDriver" | "noteOsCommand" | "uiDriverId">) {
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
    } catch {}
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
  return { uiDriverBase, uiDriverDirty, setDriverId, setDriverField, saveDriver, deleteDriver, driverSaveLabel, setDriverSaveLabel, saveTheme, deleteTheme, exportTheme, importTheme, themeSaveLabel, setThemeSaveLabel, keybindingCaptureControlId, setKeybindingCaptureControlId, setKeybindingOverride, resetKeybindingOverride, settingsHostRef };
}

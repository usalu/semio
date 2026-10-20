/** 🗃 Shell store reducer and generic Shell peer interaction. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { applyPatch } from "fast-json-patch";
import { Layout } from "@semio-tech/ui-react";
import { resolvePluginCanvasStatus, type PluginSupervisorState } from "../../🧱️elements/🐚️Shell/📦️program-sessions/🟦️.ts";
import { createLocalInteractionStoreV1 } from "../../🧱️elements/🐚️Shell/🕹️shell-store/🟦️.ts";
import bootCanvasFixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🔣️.json";
import windowIconOverrideFixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🪟️window-icon-overrides/🔣️.json" with { type: "json" };
import { tutorialInteractionSelectionActions } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import rendererSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import dialogOriginFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🗨️dialog-origin/🔣️.json";
import tutorialInteractionFixture from "../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🎥️tutorial-interaction/🔣️.json";
import { interactionSchema } from "@semio-tech/framework-interaction/testing/fixtures";
import type { InteractionState } from "@semio-tech/framework-interaction";
import type { TutorialUiChange, TutorialUiSnapshot } from "@semio-tech/framework";
import { decodeLocalInteractionCaptureJson, LOCAL_INTERACTION_CAPTURE_MAX_BYTES } from "@semio-tech/framework-replication";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { describe, expect, it } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { initialShellState, mergeRecordPreservingIdentity, type ShellAction, shellReducer, shellStateUnchanged, type PluginPanelStatus, type PluginManifest, type PluginWasmHandle, TutorialRecorder, type ShellPresencePeer, derivePeerInteractionByDomain, peerIdsSelecting, peerIdsHovering } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import {
  applyTutorialUiChangeToShell,
  applyTutorialUiSnapshotToShell,
  captureTutorialUiSnapshot
} from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("shell store reducer", () => {
  const baseState = () => initialShellState({ locale: "en", plugins: [], storage: createMemoryStoragePort() });
  const fixtureInteractionState = (value: {
    readonly selection: Readonly<Record<string, { readonly granularity: string; readonly ids: readonly string[]; readonly anchorId?: string }>>;
    readonly hover: Readonly<Record<string, { readonly channel: string; readonly ids: readonly string[] }>>;
    readonly activeMode: Readonly<Record<string, string>>;
    readonly activeGranularity: Readonly<Record<string, string>>;
  }): InteractionState => {
    const activeMode: Record<string, "single" | "multiple"> = {};
    for (const [domainId, mode] of Object.entries(value.activeMode)) {
      if (mode !== "single" && mode !== "multiple") throw new Error(`invalid fixture selection mode: ${mode}`);
      Object.defineProperty(activeMode, domainId, { value: mode, enumerable: true, writable: true, configurable: true });
    }
    return { selection: value.selection, hover: value.hover, activeMode, activeGranularity: value.activeGranularity };
  };
  const fixtureSelectionChange = (value: { readonly domainId: string; readonly granularity: string; readonly ids: readonly string[] }): TutorialUiChange => ({
    kind: "selection",
    domainId: value.domainId,
    granularity: value.granularity,
    ids: [...value.ids],
  });
  const fixtureTutorialSelection = (value: InteractionState["selection"]): TutorialUiSnapshot["interactionSelection"] => {
    const selection: TutorialUiSnapshot["interactionSelection"] = {};
    for (const [domainId, current] of Object.entries(value)) {
      Object.defineProperty(selection, domainId, { value: { ...current, ids: [...current.ids] }, enumerable: true, writable: true, configurable: true });
    }
    return selection;
  };
  const tutorialBridgeContext = (
    interactionSelection: () => InteractionState["selection"] = () => ({}),
    publishInteractionSelection: (selection: InteractionState["selection"]) => void = () => {},
  ): import("../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts").TutorialUiBridgeContext => ({
    session: null,
    restoreDialog: () => null,
    appLabelsOverlay: {
      windowKindLabels: {},
      panelTabLabels: {},
      modeLabels: {},
      actionLabels: {},
      utilityLabels: {},
      exampleLabels: {},
      actionArgLabels: {},
      dialogLabels: {},
      introductionLabels: {},
      groupLabels: {},
    },
    terminology: "native",
    locale: "en",
    interactionSelection,
    publishInteractionSelection,
  });

  it("shows terminal boot content instead of an infinite loading canvas", () => {
    for (const row of bootCanvasFixture) {
      const status = resolvePluginCanvasStatus(row.session, row.error, row.plugin as PluginPanelStatus, row.supervisor as PluginSupervisorState | undefined);
      expect(status === "loading", row.name).toBe(row.loading);
      const html = renderToStaticMarkup(
        createElement(Layout, {
          canvasStatus: status,
          canvasSkeleton: createElement("p", { role: "status" }, "Loading"),
          canvas: createElement("p", { role: "alert" }, row.error ?? "Ready"),
        }),
      );
      expect(html.includes('role="status"'), row.name).toBe(row.loading);
      expect(html.includes('role="alert"'), row.name).toBe(!row.loading);
    }
  });

  it("starts every panel anchor at the same 300px width", () => {
    const widths = Object.values(baseState().layout.panels).map((panel) => panel.size);
    expect(new Set(widths)).toEqual(new Set([300]));
  });

  it("toggles the overlays slice via a direct value without touching unrelated slices", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_SEARCH_OPEN", value: true });
    expect(next.overlays.searchOpen).toBe(true);
    expect(next.overlays.findOpen).toBe(false);
    expect(next.pluginRuntime).toBe(state.pluginRuntime);
    expect(next.uiPrefs).toBe(state.uiPrefs);
  });

  it("keeps the shell state's identity when a dispatch changes nothing, so a same-value refresh fan-out never re-renders the shell", () => {
    const state = baseState();
    expect(shellReducer(state, { type: "SET_SEARCH_OPEN", value: state.overlays.searchOpen })).toBe(state);
    expect(shellReducer(state, { type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_APP_LABELS_OVERLAY", value: (current) => current })).toBe(state);
    const opened = shellReducer(state, { type: "SET_SEARCH_OPEN", value: !state.overlays.searchOpen });
    expect(opened).not.toBe(state);
    expect(opened.overlays).not.toBe(state.overlays);
    expect(opened.windowUi).toBe(state.windowUi);
    expect(shellStateUnchanged(state, opened)).toBe(false);
    expect(shellStateUnchanged(state, state)).toBe(true);
  });

  it("keeps local interaction out of the shell state: a re-observation notifies nobody, a hover keeps every other field, an unchanged spawned-window projection is the same state", () => {
    const state = baseState();
    expect(Object.hasOwn(state, "interaction")).toBe(false);
    const read = (): InteractionState => ({ selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } }, hover: {}, activeMode: { vortex: "multiple" }, activeGranularity: { vortex: "object" } });
    const interaction = createLocalInteractionStoreV1();
    let notifications = 0;
    const unsubscribe = interaction.subscribe(() => {
      notifications += 1;
    });
    interaction.observe(read());
    const observed = interaction.get();
    expect(observed).toEqual(read());
    expect(notifications).toBe(1);
    interaction.observe(read());
    expect(interaction.get()).toBe(observed);
    expect(notifications).toBe(1);
    interaction.observe({ ...read(), hover: { vortex: { channel: "pointer", ids: ["seed-left-001"] } } });
    const hovered = interaction.get();
    expect(notifications).toBe(2);
    expect(hovered.hover).toEqual({ vortex: { channel: "pointer", ids: ["seed-left-001"] } });
    expect(hovered.selection).toBe(observed.selection);
    expect(hovered.activeMode).toBe(observed.activeMode);
    expect(hovered.activeGranularity).toBe(observed.activeGranularity);
    unsubscribe();
    interaction.observe(read());
    expect(notifications).toBe(2);
    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => current, fault: null })).toBe(state);
    const opened = shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: { "puzzle-2::puzzle3d-main": "idle" } });
    expect(opened.spawnedWindow.spawnedWindowActivityByWindowId).toEqual({ "puzzle-2::puzzle3d-main": "idle" });
    expect(shellReducer(opened, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => mergeRecordPreservingIdentity(current, [["puzzle-2::puzzle3d-main", "idle"]]) })).toBe(opened);
  });

  it("starts, advances, and dismisses an introduction via SET_INTRODUCTION_STEP without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.introductionStepIndex).toBeNull();
    const started = shellReducer(state, { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(started.overlays.introductionStepIndex).toBe(0);
    expect(started.layout).toBe(state.layout);
    const advanced = shellReducer(started, { type: "SET_INTRODUCTION_STEP", value: (prev) => (prev ?? 0) + 1 });
    expect(advanced.overlays.introductionStepIndex).toBe(1);
    const dismissed = shellReducer(advanced, { type: "SET_INTRODUCTION_STEP", value: null });
    expect(dismissed.overlays.introductionStepIndex).toBeNull();
  });

  it("auto-starts each introduction launch once and keeps a skipped replay-on-load introduction dismissed", () => {
    const state = baseState();
    const started = shellReducer(state, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:app" });
    expect(started.overlays.introductionStepIndex).toBe(0);
    expect(started.overlays.introductionAutoStartedKeys).toEqual(["demonstrator:app"]);

    const skipped = shellReducer(started, { type: "SET_INTRODUCTION_STEP", value: null });
    const repeatedAutoStart = shellReducer(skipped, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:app" });
    expect(repeatedAutoStart.overlays.introductionStepIndex).toBeNull();
    expect(repeatedAutoStart.overlays).toBe(skipped.overlays);

    const manuallyAdvanced = shellReducer(repeatedAutoStart, { type: "SET_INTRODUCTION_STEP", value: 2 });
    const nextApp = shellReducer(manuallyAdvanced, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:other-app" });
    expect(nextApp.overlays.introductionStepIndex).toBe(0);
    expect(nextApp.overlays.introductionAutoStartedKeys).toEqual(["demonstrator:app", "demonstrator:other-app"]);
  });

  it("COMPLETE_INTRODUCTION_INTERACTION appends and dedupes indices; SET_INTRODUCTION_STEP resets them", () => {
    const state = baseState();
    expect(state.overlays.introductionCompletedInteractions).toEqual([]);
    const started = shellReducer(state, { type: "SET_INTRODUCTION_STEP", value: 0 });
    const first = shellReducer(started, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 1 });
    expect(first.overlays.introductionCompletedInteractions).toEqual([1]);
    const second = shellReducer(first, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 0 });
    expect(second.overlays.introductionCompletedInteractions).toEqual([1, 0]);
    const deduped = shellReducer(second, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 1 });
    expect(deduped.overlays.introductionCompletedInteractions).toEqual([1, 0]);
    expect(deduped.layout).toBe(state.layout);
    const nextStep = shellReducer(deduped, { type: "SET_INTRODUCTION_STEP", value: 1 });
    expect(nextStep.overlays.introductionCompletedInteractions).toEqual([]);
  });

  it("opens, replaces, and closes a dialog via SET_DIALOG without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.dialog).toBeNull();
    const first = { openingId: 1, dialogId: "addObject", origin: dialogOriginFixture.owner, seedArgs: { objectKind: "Object" } };
    const opened = shellReducer(state, { type: "SET_DIALOG", value: first });
    expect(opened.overlays.dialog).toEqual(first);
    expect(opened.layout).toBe(state.layout);
    expect(opened.pluginRuntime).toBe(state.pluginRuntime);
    const second = { openingId: 2, dialogId: "confirmDelete", origin: dialogOriginFixture.owner };
    const replaced = shellReducer(opened, { type: "SET_DIALOG", value: second });
    expect(replaced.overlays.dialog).toEqual(second);
    const staleClose = shellReducer(replaced, { type: "CLOSE_DIALOG", openingId: first.openingId });
    expect(staleClose.overlays).toBe(replaced.overlays);
    const closed = shellReducer(staleClose, { type: "CLOSE_DIALOG", openingId: second.openingId });
    expect(closed.overlays.dialog).toBeNull();
  });

  it("toggles the layout slice via an updater function", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const toggled = shellReducer(opened, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: (prev) => !prev });
    expect(opened.layout.panels["top-left"].visible).toBe(true);
    expect(toggled.layout.panels["top-left"].visible).toBe(false);
    expect(toggled.overlays).toBe(opened.overlays);
  });

  it("toggles a middle anchor via SET_PANEL_VISIBLE the same way as a corner", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-middle", value: true });
    expect(opened.layout.panels["top-middle"].visible).toBe(true);
    expect(opened.layout.panels["top-left"].visible).toBe(state.layout.panels["top-left"].visible);
  });

  it("applies the neutral per-instance icon sequence through Reacts SET_WINDOW_ICON reducer", () => {
    
    
    
    let state = shellReducer(baseState(), {
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: windowIconOverrideFixture.windowIds.slice(1).map((id) => ({ id, windowKindId: windowIconOverrideFixture.windowKindId, title: id })),
    });
    const resolvedIcon = (windowId: string, projectionIconId?: string) => state.layout.windowIconsById[windowId] ?? projectionIconId ?? windowIconOverrideFixture.kindIconId;
    for (const [index, command] of windowIconOverrideFixture.commands.entries()) {
      if (index === windowIconOverrideFixture.commands.length - 1) {
        expect(resolvedIcon(windowIconOverrideFixture.projectionFallback.windowId, windowIconOverrideFixture.projectionFallback.expectedIconId)).toBe(windowIconOverrideFixture.projectionFallback.expectedIconId);
      }
      state = shellReducer(state, { type: "SET_WINDOW_ICON", windowId: command.windowId, iconId: command.iconId as never });
      expect(resolvedIcon(command.windowId, command.windowId === windowIconOverrideFixture.projectionFallback.windowId ? windowIconOverrideFixture.projectionFallback.expectedIconId : undefined)).toBe(command.expectedIconId);
    }
    expect(state.layout.windowIconsById.main).toBe("diamond");
    expect(state.layout.windowIconsById["main-2"]).toBe("circle");
    expect(resolvedIcon(windowIconOverrideFixture.projectionFallback.windowId, windowIconOverrideFixture.projectionFallback.expectedIconId)).toBe("star");
    expect(resolvedIcon(windowIconOverrideFixture.kindFallback.windowId)).toBe(windowIconOverrideFixture.kindFallback.expectedIconId);
  });

  it("rewrites window titles via SET_WINDOW_TITLE for extras and base kinds", () => {
    const state = shellReducer(baseState(), {
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: [{ id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" }],
    });
    const renamedExtra = shellReducer(state, { type: "SET_WINDOW_TITLE", windowId: "puzzle3d-main-top", title: "Front" });
    expect(renamedExtra.layout.windowTitlesById["puzzle3d-main-top"]).toBe("Front");
    expect(renamedExtra.layout.extraWindowInstances[0]?.title).toBe("Front");
    expect(renamedExtra.overlays).toBe(state.overlays);
    const renamedBase = shellReducer(renamedExtra, { type: "SET_WINDOW_TITLE", windowId: "puzzle3d-main", title: "Isometric" });
    expect(renamedBase.layout.windowTitlesById["puzzle3d-main"]).toBe("Isometric");
    expect(renamedBase.layout.extraWindowInstances[0]?.title).toBe("Front");
  });

  it("resets the dock override, every anchor's active path/visible/size, drill-down memory, and tree expansion via RESET_DOCK", () => {
    const state = baseState();
    const rearranged = shellReducer(state, {
      type: "SET_DOCK_OVERRIDE",
      value: {
        version: 3,
        anchors: { "top-left": [{ id: "moved" }], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      },
    });
    const withPath = shellReducer(rearranged, { type: "SET_PANEL_PATH", anchor: "top-left", value: ["moved"] });
    const withVisible = shellReducer(withPath, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const withSize = shellReducer(withVisible, { type: "SET_PANEL_SIZE", anchor: "top-left", value: 999 });
    const withMemory = shellReducer(withSize, { type: "SET_PANEL_PATH_MEMORY", value: { moved: "child" } });
    const withTreeOpen = shellReducer(withMemory, { type: "SET_TREE_OPEN_STATE", id: "unit:section", open: true });
    const reset = shellReducer(withTreeOpen, { type: "RESET_DOCK" });
    expect(reset.layout.dockOverride).toBeNull();
    expect(reset.layout.panels["top-left"].path).toEqual([]);
    expect(reset.layout.panels["top-left"].visible).toBe(false);
    expect(reset.layout.panels["top-left"].size).toBe(state.layout.panels["top-left"].size);
    expect(reset.layout.panelPathMemory).toEqual({});
    expect(reset.layout.treeOpenStates).toEqual({});
  });

  it("HYDRATE_DOCK_UI restores a persisted size for the top-middle anchor", () => {
    const state = baseState();
    const hydrated = shellReducer(state, { type: "HYDRATE_DOCK_UI", value: { version: 3, anchors: { "top-middle": { visible: true, size: 420 } } } });
    expect(hydrated.layout.panels["top-middle"].visible).toBe(true);
    expect(hydrated.layout.panels["top-middle"].size).toBe(420);
    expect(hydrated.layout.panels["top-left"]).toEqual(state.layout.panels["top-left"]);
  });

  it("updates the uiPrefs slice and leaves the sync slice referentially unchanged", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_UI_DRIVER_ID", value: "compact" });
    expect(next.uiPrefs.uiDriverId).toBe("compact");
    expect(next.sync).toBe(state.sync);
  });

  it("action-panel slice: fold/expand/stage/reset/active-utility update only their own keys and preserve identity on no-operations", () => {
    const state = baseState();

    const folded = shellReducer(state, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false });
    expect(folded.actionPane.foldedByWindowId).toEqual({ w1: false });
    expect(folded.layout).toBe(state.layout);
    expect(shellReducer(folded, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false }).actionPane).toBe(folded.actionPane);

    const expanded = shellReducer(folded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" });
    expect(expanded.actionPane.expandedByWindowId).toEqual({ w1: "extrude" });
    expect(shellReducer(expanded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" }).actionPane).toBe(expanded.actionPane);

    const staged = shellReducer(expanded, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 });
    expect(staged.actionPane.stagedArgsByKey).toEqual({ "w1:extrude": { depth: 3 } });
    const stagedMore = shellReducer(staged, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "segments", value: 2 });
    expect(stagedMore.actionPane.stagedArgsByKey["w1:extrude"]).toEqual({ depth: 3, segments: 2 });
    expect(shellReducer(stagedMore, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 }).actionPane).toBe(stagedMore.actionPane);

    const reset = shellReducer(stagedMore, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" });
    expect(reset.actionPane.stagedArgsByKey["w1:extrude"]).toBeUndefined();
    expect(reset.actionPane.expandedByWindowId["w1"]).toBe("extrude");
    expect(shellReducer(reset, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" }).actionPane).toBe(reset.actionPane);

    const activated = shellReducer(reset, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" });
    expect(activated.actionPane.activeUtilityByWindowId).toEqual({ w1: "pen" });
    expect(shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" }).actionPane).toBe(activated.actionPane);
    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: null });
    expect(deactivated.actionPane.activeUtilityByWindowId["w1"]).toBeNull();
  });

  it("actionPane slice: SET_ACTIVE_TOOL updates only activeToolId and preserves identity on no-operations (mode-scoped, not per-window)", () => {
    const state = baseState();
    expect(state.actionPane.activeToolId).toBeNull();

    const activated = shellReducer(state, { type: "SET_ACTIVE_TOOL", toolId: "fill" });
    expect(activated.actionPane.activeToolId).toBe("fill");
    expect(activated.actionPane.activeUtilityByWindowId).toBe(state.actionPane.activeUtilityByWindowId);
    expect(shellReducer(activated, { type: "SET_ACTIVE_TOOL", toolId: "fill" }).actionPane).toBe(activated.actionPane);

    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_TOOL", toolId: null });
    expect(deactivated.actionPane.activeToolId).toBeNull();
  });

  it("commandPanel slice: expand/collapse and stage/reset update only their own keys and preserve identity on no-operations (category active/fold state now lives in layout.panels['bottom-middle'], not this slice)", () => {
    const state = baseState();

    const expanded = shellReducer(state, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");
    expect(expanded.layout).toBe(state.layout);
    expect(shellReducer(expanded, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" }).commandPanel).toBe(expanded.commandPanel);

    const staged = shellReducer(expanded, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" });
    expect(staged.commandPanel.stagedArgsByCommandId).toEqual({ "os.setThemeId": { themeId: "semio" } });
    expect(shellReducer(staged, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" }).commandPanel).toBe(staged.commandPanel);

    const reset = shellReducer(staged, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" });
    expect(reset.commandPanel.stagedArgsByCommandId["os.setThemeId"]).toBeUndefined();
    expect(shellReducer(reset, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" }).commandPanel).toBe(reset.commandPanel);
  });

  it("command palette category is the bottom-middle anchor's own SET_PANEL_PATH; the UI's category-switch handler additionally dispatches SET_COMMAND_EXPANDED:null (reproducing the old single-action collapse-on-switch behavior across two actions)", () => {
    const state = baseState();
    const onCategory = shellReducer(state, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.appearance"] });
    expect(onCategory.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.appearance"]);

    const expanded = shellReducer(onCategory, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");

    const switchedPath = shellReducer(expanded, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.layout"] });
    const switched = shellReducer(switchedPath, { type: "SET_COMMAND_EXPANDED", value: null });
    expect(switched.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.layout"]);
    expect(switched.commandPanel.expandedCommandId).toBeNull();
  });

  it("tutorial slice: SET_TUTORIAL starts a tutorial, resets rate/deviated, and clears an active introduction (mutual exclusivity)", () => {
    const introducing = shellReducer(baseState(), { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(introducing.overlays.introductionStepIndex).toBe(0);
    const started = shellReducer(introducing, { type: "SET_TUTORIAL", value: "welcome-tour" });
    expect(started.tutorial).toEqual({ activeTutorialId: "welcome-tour", playing: false, rate: 1, muted: false, captionsOn: true, recording: false, deviated: false });
    expect(started.overlays.introductionStepIndex).toBeNull();
  });

  it("tutorial slice: SET_INTRODUCTION_STEP (non-null) clears an active tutorial (mutual exclusivity, reverse direction)", () => {
    const started = shellReducer(baseState(), { type: "SET_TUTORIAL", value: "welcome-tour" });
    const playing = shellReducer(started, { type: "SET_TUTORIAL_PLAYING", value: true });
    expect(playing.tutorial.playing).toBe(true);
    const introduced = shellReducer(playing, { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(introduced.tutorial.activeTutorialId).toBeNull();
    expect(introduced.tutorial.playing).toBe(false);
    expect(introduced.overlays.introductionStepIndex).toBe(0);
  });

  it("tutorial slice: play/pause resets deviated only when transitioning to playing; rate/muted/captions/recording/deviated update independently", () => {
    const started = shellReducer(baseState(), { type: "SET_TUTORIAL", value: "welcome-tour" });
    const deviated = shellReducer(started, { type: "SET_TUTORIAL_DEVIATED", value: true });
    expect(deviated.tutorial.deviated).toBe(true);
    const stillPaused = shellReducer(deviated, { type: "SET_TUTORIAL_PLAYING", value: false });
    expect(stillPaused.tutorial.deviated).toBe(true);
    const resumed = shellReducer(deviated, { type: "SET_TUTORIAL_PLAYING", value: true });
    expect(resumed.tutorial.deviated).toBe(false);
    const rated = shellReducer(resumed, { type: "SET_TUTORIAL_RATE", value: 2 });
    expect(rated.tutorial.rate).toBe(2);
    const muted = shellReducer(rated, { type: "SET_TUTORIAL_MUTED", value: true });
    expect(muted.tutorial.muted).toBe(true);
    const captionsOff = shellReducer(muted, { type: "SET_TUTORIAL_CAPTIONS", value: false });
    expect(captionsOff.tutorial.captionsOn).toBe(false);
    const recording = shellReducer(captionsOff, { type: "SET_TUTORIAL_RECORDING", value: true });
    expect(recording.tutorial.recording).toBe(true);
  });

  it("APPLY_TUTORIAL_UI_SNAPSHOT restores shell-owned fields without forging actor-owned interaction state", () => {
    const state = baseState();
    const snapshot = shellReducer(state, {
      type: "APPLY_TUTORIAL_UI_SNAPSHOT",
      snapshot: {
        activeWindowId: "puzzle3d-main",
        shellLayout: { kind: "window", id: "puzzle3d-main" },
        extraWindowInstances: [],
        panelPatches: { "top-left": { visible: true, path: ["catalogue"] } },
        treeOpenStates: { "catalogue.section": true },
        activeUtilityByWindowId: { "puzzle3d-main": "transform" },
        activeToolId: "fill",
        dialog: { openingId: 3, dialogId: "addObject", origin: dialogOriginFixture.owner },
        commandPanelOpen: true,
      },
    });
    expect(snapshot.layout.activeWindowId).toBe("puzzle3d-main");
    expect(snapshot.layout.panels["top-left"]).toMatchObject({ visible: true, path: ["catalogue"] });
    expect(snapshot.layout.treeOpenStates).toEqual({ "catalogue.section": true });
    expect(snapshot.actionPane.activeUtilityByWindowId).toEqual({ "puzzle3d-main": "transform" });
    expect(snapshot.actionPane.activeToolId).toBe("fill");
    expect(snapshot.overlays.dialog).toEqual({ openingId: 3, dialogId: "addObject", origin: dialogOriginFixture.owner });
    expect(snapshot.overlays.searchOpen).toBe(true);
    expect(Object.hasOwn(snapshot, "interaction")).toBe(false);
    expect(snapshot.pluginRuntime).toBe(state.pluginRuntime);
  });

  it("validates the language-neutral interaction recording vectors against the canonical schema", () => {
    const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
    ajv;
    ajv.addSchema(interactionSchema).addSchema(rendererSchema);
    const validateCapture = ajv.getSchema(`${rendererSchema.$id}#/$defs/TutorialInteractionCaptureCapture`)!;
    expect(validateCapture(tutorialInteractionFixture.capture), JSON.stringify(validateCapture.errors)).toBe(true);
    expect(decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify(tutorialInteractionFixture.capture)))).toEqual(tutorialInteractionFixture.capture);
    expect(validateCapture({ ...tutorialInteractionFixture.capture, extra: true })).toBe(false);
  });

  it("decodes the bounded actor interaction capture and rejects noncanonical authority or state", () => {
    const bytes = new TextEncoder().encode(JSON.stringify(tutorialInteractionFixture.capture));
    expect(decodeLocalInteractionCaptureJson(bytes)).toEqual(tutorialInteractionFixture.capture);
    expect(() => decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify({ ...tutorialInteractionFixture.capture, extra: true })))).toThrow("local-interaction.capture");
    expect(() => decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify({ ...tutorialInteractionFixture.capture, identity: { ...tutorialInteractionFixture.capture.identity, generation: "01" } })))).toThrow(
      "local-interaction.identity.generation",
    );
    expect(() =>
      decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify({ ...tutorialInteractionFixture.capture, state: { ...tutorialInteractionFixture.capture.state, selection: { mesh: { granularity: "face", ids: ["same", "same"] } } } }))),
    ).toThrow("local-interaction.state.selection.mesh.ids");
    expect(() => decodeLocalInteractionCaptureJson(new Uint8Array(LOCAL_INTERACTION_CAPTURE_MAX_BYTES + 1))).toThrow("local-interaction.capture-length");
  });

  it("captures the observed typed interaction selection without aliasing ids or special domain keys", () => {
    const observed = fixtureInteractionState(tutorialInteractionFixture.observed);
    const selection = { ...observed.selection };
    Object.defineProperty(selection, "__proto__", { value: { granularity: "node", ids: ["prototype-safe"] }, enumerable: true, writable: true, configurable: true });
    const interaction = createLocalInteractionStoreV1();
    interaction.observe({ ...observed, selection });
    const snapshot = captureTutorialUiSnapshot(baseState(), interaction.get(), null);
    expect(snapshot.interactionSelection.mesh).toEqual(tutorialInteractionFixture.snapshotSelection.mesh);
    expect(snapshot.interactionSelection["special.domain"]).toEqual(tutorialInteractionFixture.snapshotSelection["special.domain"]);
    expect(snapshot.interactionSelection).not.toBe(interaction.get().selection);
    expect(snapshot.interactionSelection.mesh?.ids).not.toBe(interaction.get().selection.mesh?.ids);
    expect(Object.hasOwn(snapshot.interactionSelection, "__proto__")).toBe(true);
    expect(snapshot.interactionSelection["__proto__"]).toEqual({ granularity: "node", ids: ["prototype-safe"] });
  });

  it("plays full and sparse typed selections through the Shell reducer and the local interaction store with JSON Patch parity", () => {
    let state = baseState();
    const interaction = createLocalInteractionStoreV1(fixtureInteractionState(tutorialInteractionFixture.playbackBefore));
    const dispatch = (action: ShellAction) => {
      state = shellReducer(state, action);
    };
    const bridge = tutorialBridgeContext(
      () => interaction.get().selection,
      (selection) => interaction.observe({ ...interaction.get(), selection }),
    );
    applyTutorialUiSnapshotToShell(
      dispatch,
      {
        activeUtilityByWindowId: {},
        activePanelTabByGroup: {},
        interactionSelection: fixtureTutorialSelection(fixtureInteractionState(tutorialInteractionFixture.observed).selection),
        expandedTreeIds: [],
        commandPanelOpen: false,
      },
      bridge,
    );
    expect(interaction.get()).toEqual(applyPatch(structuredClone(tutorialInteractionFixture.playbackBefore), [{ op: "replace", path: "/selection", value: structuredClone(tutorialInteractionFixture.snapshotSelection) }], true, false).newDocument);

    state = baseState();
    interaction.observe(fixtureInteractionState(tutorialInteractionFixture.playbackBefore));
    applyTutorialUiChangeToShell(dispatch, fixtureSelectionChange({ ...tutorialInteractionFixture.delta, domainId: "mesh" }), bridge);
    const oracleAfterDelta = applyPatch(
      structuredClone(tutorialInteractionFixture.playbackBefore),
      [{ op: "replace", path: "/selection/mesh", value: { granularity: tutorialInteractionFixture.delta.granularity, ids: [...tutorialInteractionFixture.delta.ids] } }],
      true,
      false,
    ).newDocument;
    expect(interaction.get()).toEqual(oracleAfterDelta);
    expect(interaction.get()).toEqual(tutorialInteractionFixture.afterDelta);
    applyTutorialUiChangeToShell(dispatch, fixtureSelectionChange({ ...tutorialInteractionFixture.clearDelta, domainId: "mesh" }), bridge);
    expect(interaction.get()).toEqual(tutorialInteractionFixture.afterClear);
  });

  it("projects tutorial selection playback through ordinary interaction CQRS actions", () => {
    expect(tutorialInteractionSelectionActions("controller", fixtureInteractionState(tutorialInteractionFixture.observed).selection)).toEqual([
      { controllerId: "controller", action: "clearSelection" },
      { controllerId: "controller", action: "setSelectionMode", args: { domainId: "mesh", mode: "multiple" } },
      {
        controllerId: "controller",
        action: "interactionSelect",
        args: {
          domainId: "mesh",
          merge: "replace",
          method: "pick",
          targets: JSON.stringify([
            { granularity: "face", id: "face,west" },
            { granularity: "face", id: "face-east" },
          ]),
        },
      },
      { controllerId: "controller", action: "interactionSelect", args: { domainId: "mesh", merge: "additive", method: "pick", targets: JSON.stringify([{ granularity: "face", id: "face,west" }]) } },
      { controllerId: "controller", action: "interactionSelect", args: { domainId: "special.domain", merge: "replace", method: "pick", targets: JSON.stringify([{ granularity: "node", id: "owned" }]) } },
    ]);
  });

  it("records comma-bearing selection changes and explicit domain clearing as typed deltas", () => {
    const base: TutorialUiSnapshot = {
      activeUtilityByWindowId: {},
      activePanelTabByGroup: {},
      interactionSelection: fixtureTutorialSelection(fixtureInteractionState(tutorialInteractionFixture.playbackBefore).selection),
      expandedTreeIds: [],
      commandPanelOpen: false,
    };
    const recorder = new TutorialRecorder(base, null);
    recorder.recordUiDiff({ ...base, interactionSelection: fixtureTutorialSelection(fixtureInteractionState(tutorialInteractionFixture.afterDelta).selection) });
    recorder.recordUiDiff({ ...base, interactionSelection: {} });
    expect(recorder.build("interaction", "Interaction").tracks.ui.map((keyframe) => keyframe.sample)).toEqual([
      { kind: "delta", changes: [tutorialInteractionFixture.delta] },
      { kind: "delta", changes: [{ kind: "selection", domainId: "mesh", granularity: "face", ids: [] }] },
    ]);
  });

  //#region 🔌️PluginRuntime hot-swap actions
  function fakeLoadedPlugin(pluginId: string, version = "0"): { readonly handle: PluginWasmHandle; readonly manifest: PluginManifest } {
    const manifest: PluginManifest = { pluginId, label: pluginId, version, apps: [], examples: [], commands: [], artifactKinds: [], dependencies: [], contributions: [] };
    const handle = {
      pluginId,
      manifest,
      createApp: async () => 0,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }, diagnostics: [], requestedEffects: [], events: [] }),
      refreshUi: async () => ({}),
      contextMenu: async () => [],
      dispose: () => {},
    } as unknown as PluginWasmHandle;
    return { handle, manifest };
  }

  it("UPSERT_LOADED_PLUGIN inserts a new pluginId and replaces an existing one in place (order preserved)", () => {
    const state = baseState();
    const note1 = fakeLoadedPlugin("note", "1");
    const withNote = shellReducer(state, { type: "UPSERT_LOADED_PLUGIN", value: note1 });
    expect(withNote.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note"]);
    const withS = shellReducer(withNote, { type: "UPSERT_LOADED_PLUGIN", value: fakeLoadedPlugin("s") });
    expect(withS.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note", "s"]);
    const note2 = fakeLoadedPlugin("note", "2");
    const reloaded = shellReducer(withS, { type: "UPSERT_LOADED_PLUGIN", value: note2 });
    expect(reloaded.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note", "s"]);
    expect(reloaded.pluginRuntime.loadedPlugins[0]!.manifest.version).toBe("2");
    expect(reloaded.layout).toBe(withS.layout);
  });

  it("REMOVE_LOADED_PLUGIN drops only the matching pluginId", () => {
    const withBoth = [fakeLoadedPlugin("note"), fakeLoadedPlugin("s")].reduce((state, entry) => shellReducer(state, { type: "UPSERT_LOADED_PLUGIN", value: entry }), baseState());
    const removed = shellReducer(withBoth, { type: "REMOVE_LOADED_PLUGIN", pluginId: "note" });
    expect(removed.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["s"]);
  });

  it("SET_PLUGIN_STATUS tracks per-pluginId status independent of loadedPlugins membership", () => {
    const state = baseState();
    const installing = shellReducer(state, { type: "SET_PLUGIN_STATUS", pluginId: "note", value: "installing" });
    expect(installing.pluginRuntime.pluginStatusById).toEqual({ note: "installing" });
    const loaded = shellReducer(installing, { type: "SET_PLUGIN_STATUS", pluginId: "note", value: "loaded" });
    const failed = shellReducer(loaded, { type: "SET_PLUGIN_STATUS", pluginId: "s", value: "failed" });
    expect(failed.pluginRuntime.pluginStatusById).toEqual({ note: "loaded", s: "failed" });
  });
  //#endregion 🔌️PluginRuntime hot-swap actions
});

// 🕹️wave-2b: `derivePeerInteractionByDomain`/`peerIdsSelecting`/`peerIdsHovering` regroup the typed,
// wave-0 `PresenceInteraction` roster field into an app-agnostic per-domain shape — the replacement for
// today's per-app `presencePeersJson` decoding (see "s workflow flow routing"'s "renders presence peers
// from the scene payload" above, one of the only apps that renders peer selection at all today).
describe("Shell peer interaction (generic, app-agnostic)", () => {
  const peer = (clientId: string, name: string, interaction?: ShellPresencePeer["interaction"]): ShellPresencePeer => ({ clientId, name, interaction });

  it("regroups per-peer PresenceInteraction domains into a per-domain roster, keyed by clientId", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1", "n2"], hovered: [] }] }),
      peer("client-b", "Bo", { appId: "lowpoly", domains: [{ domain: "mesh", granularity: "face", selected: [], hovered: ["f7"] }] }),
    ]);
    expect(roster["graph"]).toEqual({ selectedByPeer: { "client-a": ["n1", "n2"] }, hoveredByPeer: {} });
    expect(roster["mesh"]).toEqual({ selectedByPeer: {}, hoveredByPeer: { "client-b": ["f7"] } });
  });

  it("is app-agnostic: two different apps sharing one domain id merge into the same entry", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1"], hovered: [] }] }),
      peer("client-b", "Bo", { appId: "dag", domains: [{ domain: "graph", granularity: "node", selected: ["n2"], hovered: [] }] }),
    ]);
    expect(roster["graph"]?.selectedByPeer).toEqual({ "client-a": ["n1"], "client-b": ["n2"] });
  });

  it("peerIdsSelecting/peerIdsHovering find which peers have a given target id", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1", "n2"], hovered: ["n3"] }] }),
      peer("client-b", "Bo", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n2"], hovered: [] }] }),
    ]);
    expect(peerIdsSelecting(roster, "graph", "n1")).toEqual(["client-a"]);
    expect([...peerIdsSelecting(roster, "graph", "n2")].sort()).toEqual(["client-a", "client-b"]);
    expect(peerIdsSelecting(roster, "graph", "n99")).toEqual([]);
    expect(peerIdsHovering(roster, "graph", "n3")).toEqual(["client-a"]);
  });

  it("is defensive about an absent interaction field (older heartbeat, or wave 2a's wire field not yet landed) and an unknown domain", () => {
    const roster = derivePeerInteractionByDomain([peer("client-a", "Ada", undefined), peer("client-b", "Bo")]);
    expect(roster).toEqual({});
    expect(peerIdsSelecting(roster, "graph", "n1")).toEqual([]);
    expect(peerIdsHovering({}, "unknown-domain", "x")).toEqual([]);
  });
});

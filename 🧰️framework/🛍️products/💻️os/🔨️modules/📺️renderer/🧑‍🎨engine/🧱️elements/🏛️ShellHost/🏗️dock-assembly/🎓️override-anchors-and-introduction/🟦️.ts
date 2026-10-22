/** 🎓️ The host, studio and details override anchors and the introduction step derived ids with their reveal effects. */
import { useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { FRAMEWORK_PANEL_TAB_CATALOGUE_ID, type IntroductionInteraction, SET_ACTIVE_TOOL_ACTION_ID } from "@semio-tech/framework";
import { findPanelTabInDock, findPanelTabPath } from "@semio-tech/ui-react";
import { FRAMEWORK_CATEGORY_TOOL_ID } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { resolveUtilityNodes } from "../../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import { utilityNodeTreeContainsId } from "../../../🛠️ShellHelpers/🪟️window-panes/🪛️utility-bar-options/🟦️.tsx";
import { windowMeasureTreeContainsId } from "../../../🛠️ShellHelpers/🪟️window-panes/🎚️window-measures/🟦️.tsx";
import type { useDockModel } from "../🧱️dock-model-and-persistence/🟦️.ts";

/** 🧭 Derives the override anchors and every introduction element, utility, window and tool id. */
export function useOverrideAnchorsAndIntroductionIds({ hostMode, session, hostAppId, panel, hostCatalogueTabId, dock, activeIntroduction, introductionStepIndex, appLabelsOverlay, uiTerminology, uiLocale, windowMeasuresByWindowId, extraWindowInstances, focusedToolMeasuresByToolId }: Pick<ReturnType<typeof useDockModel>, "hostMode" | "session" | "hostAppId" | "panel" | "hostCatalogueTabId" | "dock" | "activeIntroduction" | "introductionStepIndex" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "windowMeasuresByWindowId" | "extraWindowInstances" | "focusedToolMeasuresByToolId">) {
  const hostOverrideTabId = hostMode && session?.app.id === hostAppId ? (panel?.activePanelTab ?? hostCatalogueTabId) : undefined;
  const studioOverrideAnchor = hostOverrideTabId ? findPanelTabInDock(dock, hostOverrideTabId)?.anchor : undefined;
  const detailsOverrideTabId = panel?.activePanelTab;
  const detailsOverrideAnchor = detailsOverrideTabId ? findPanelTabInDock(dock, detailsOverrideTabId)?.anchor : undefined;

  /** 🎓️ The current introduction step's target element ids (`introduce` + `show`), classified by
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
    for (const [toolId, measures] of Object.entries(focusedToolMeasuresByToolId)) {
      if (introductionElementIds.some((id) => windowMeasureTreeContainsId(measures, id))) return toolId;
    }
    return null;
  }, [introductionElementIds, focusedToolMeasuresByToolId]);
  return { introductionToolId, introductionToolPickIds, activeIntroductionStep, introductionPanelTabId, introductionPanelTabAnchor, hostOverrideTabId, studioOverrideAnchor, detailsOverrideTabId, detailsOverrideAnchor, introductionUtilityId, introductionActionWindowSegment, introductionUtilityWindowId, introductionMeasureWindowId };
}

/**
 * 🎬 Reveals the introduction tool, tool pick, panel tab and expand targets as the steps advance.
 *
 * 🎓️ Panel interactions complete when their named panel tab is open and visible — checked for every
 * `panel` interaction of the active step, not just the first, so a step can require opening several.
 */
export function useIntroductionRevealEffects({ introductionToolId, session, activeToolIdRef, onActionStable, introductionToolPickIds, activeIntroductionStep, mobile, mobilePanelTabs, dispatch, dock, introductionPanelTabId, introductionPanelTabAnchor, panels, completeIntroductionInteraction, treeOpenStates }: Pick<ReturnType<typeof useOverrideAnchorsAndIntroductionIds>, "introductionToolId" | "introductionToolPickIds" | "activeIntroductionStep" | "introductionPanelTabId" | "introductionPanelTabAnchor"> & Pick<ReturnType<typeof useDockModel>, "session" | "activeToolIdRef" | "onActionStable" | "mobile" | "mobilePanelTabs" | "dispatch" | "dock" | "panels" | "completeIntroductionInteraction" | "treeOpenStates">) {
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
}

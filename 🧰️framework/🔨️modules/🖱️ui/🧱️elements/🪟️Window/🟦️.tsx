// #region 🧲️Header
// 💻️ framework/ui/elements/🪟️Window/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { childElementId } from "../🆔️ElementId/🟦️.tsx";
import { ActionGroup, ActionGroupItem } from "../⚡️ActionGroup/🟦️.tsx";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { loadingBorderStateClass, waitingBorderStateClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { useShellScopeOptional, NULL_SHELL_ROOT_REF, useShellKeydown } from "../🐚️ShellScope/🟦️.tsx";
import { SurfaceScope, isSurfaceActiveBackgroundPointer, getLevelZClass } from "../🌈️Surface/🟦️.tsx";
import { measureWindowChromeScrollClearancePx, windowChromeScrollClearanceVar, windowContentDeadLineVar } from "../🚧️WindowContentDeadLine/🟦️.tsx";
import { interactiveActiveFillClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { type UiStatus, uiSpacingPx } from "@semio-tech/ui-styling";
import { ExternalLinkIcon, Minimize2Icon, Maximize2Icon, CloseIcon } from "../🔣️Icons/🟦️.tsx";
import { PaneHost, Pane } from "../📌️WindowPane/🟦️.tsx";
import { type EngagementSpec, Engagement } from "../🎮️Engagement/🟦️.tsx";
import { type SearchSpec, UI_WINDOW_SEARCH, routeWindowSearchEscape, shouldRouteKeysToWindowSearch, Search, focusActiveSearchInput } from "../🔍️WindowSearch/🟦️.tsx";
import { windowMeasuresDefaultWidthPx, windowMeasuresMinWidthPx, windowMeasuresMaxWidthPx, WINDOW_PANE_MEASURES_ICON, WINDOW_PANE_ACTIONS_ICON, WINDOW_PANE_SEARCH_ICON, WINDOW_PANE_UTILITIES_ICON, windowChromeClearedTopOffset } from "../../🔨️modules/📐️window-measures/🟦️.ts";
import { useChromePanelSafeArea, chromePanelSafeAreaStyle } from "../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";
import { panelResizeEdgeAccentClass, windowMeasuresBodyClass, windowEngagementBodyClass, utilityBarBodyClass } from "../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { useUiMobile } from "../../🔨️modules/📱️ui-mobile-context/🟦️.tsx";
import { type WindowPaneId, type WindowPaneOpen, resolveWindowPaneExpansion, windowPaneExclusiveMaxWidthPx, windowQuickActionsCovered } from "../../🔨️modules/🪟️window-pane-exclusivity/🟦️.ts";
import { GhostRegionShell } from "../👻️PanelGhost/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🌊️Window

export interface WindowConfig {
  id: string;
  children: React.ReactNode;
  defaultSize?: number;
  onDoubleClick?: () => void;
  className?: string;
  status?: UiStatus;
  error?: Error | null;
  skeleton?: React.ReactNode;
  onOpenInNewWindow?: () => void;
  onMaximize?: () => void;
  onMinimize?: () => void;
  onClose?: () => void;
  controls?: React.ReactNode;
  measures?: React.ReactNode;
  utilityBar?: React.ReactNode;
  /** 🎛️ Controlled fold state for the Window Options (measures) rail (default true); externally settable so the introduction walkthrough can force-unfold a step's measure target into view. */
  measuresFolded?: boolean;
  /** 📏️ Fires when the user or a redirect toggles the Window Options rail fold state. */
  onMeasuresFoldedChange?: (folded: boolean) => void;
  /** 🛠️ Controlled fold state for the Utilities rail (default true); externally settable so the introduction walkthrough can force-unfold a step's utility anchor into view. */
  utilityBarFolded?: boolean;
  /** 🔔️ Fires when the user or a redirect toggles the Utilities rail fold state. */
  onUtilityBarFoldedChange?: (folded: boolean) => void;
  /** 🌳️ Categorized ad-hoc actions tree, merged into the top-left Actions pane below the active {@link engagement} (when present); folds to a chip by default. */
  actionPane?: React.ReactNode;
  /** ⬅️ Controlled fold state for the merged top-left Actions pane (default true); externally settable so the palette/keybinding redirect can force-unfold. */
  actionsFolded?: boolean;
  /** 🎬️ Fires when the user or a redirect toggles the Actions pane fold state. */
  onActionsFoldedChange?: (folded: boolean) => void;
  /** 💬️ Active engagement content (options/status/control) rendered above {@link actionPane} inside the same top-left Actions pane. */
  engagement?: EngagementSpec;
  /** 🔎️ Top-middle floating search pane: typed action input with autocomplete possibles. */
  search?: SearchSpec;
  active?: boolean;
  onActivate?: () => void;
  /** 📐️ When true, the window body grows to fill its dock pane (canvas hosts). */
  fill?: boolean;
}

/**
 * 🪟️ WindowProps holds the data fields for a WindowProps record.
 **/
interface WindowProps extends WindowConfig {
  isVisible?: boolean;
}

/**
 * 🚨️ DefaultErrorDisplay holds the data fields for a DefaultErrorDisplay record.
 **/
const DefaultErrorDisplay: React.FC<{ error: Error }> = ({ error }) => {
  const bgClass = "bg-transparent";
  const errorLabel = useLabel("ui.common.error");
  return (
    <div className={cn("flex flex-col items-center justify-center h-full w-full p-small", bgClass)}>
      <div className="text-center space-y-2 max-w-md">
        <div className="text-4xl mb-4">⚠️</div>
        <h3 className="text-lg font-medium">{errorLabel}</h3>
        <p className="text-sm text-muted-foreground">{error.message}</p>
      </div>
    </div>
  );
};

/**
 * 🖼️ Window holds the data fields for a Window record.
 **/
function useWindowUtilityBarMaxHeightPx(enabled: boolean, bodyRef: React.RefObject<HTMLDivElement | null>): number {
  const [maxHeightPx, setMaxHeightPx] = reactHostPort.useState(0);
  reactHostPort.useLayoutEffect(() => {
    const body = bodyRef.current;
    if (!enabled || !body) {
      setMaxHeightPx(0);
      return;
    }
    const update = () => {
      const topClearancePx = measureWindowChromeScrollClearancePx(body);
      const available = Math.max(0, Math.floor(body.getBoundingClientRect().height) - topClearancePx - uiSpacingPx(1));
      setMaxHeightPx((prev) => (prev === available ? prev : available));
    };
    update();
    const observer = new ResizeObserver(update);
    observer.observe(body);
    for (const slot of ["window-engagement-overlay", "window-search-overlay", "window-measures-overlay"] as const) {
      const overlay = body.querySelector(`[data-slot="${slot}"]`);
      if (overlay) observer.observe(overlay);
    }
    return () => observer.disconnect();
  }, [bodyRef, enabled]);
  return maxHeightPx;
}

const windowQuickActionsWidthVar = "--window-quick-actions-width";

const Window: React.FC<WindowProps> = ({
  id,
  children,
  onDoubleClick,
  className = "",
  isVisible = true,
  status = "idle",
  error = null,
  skeleton,
  onOpenInNewWindow,
  onMaximize,
  onMinimize,
  onClose,
  controls,
  measures,
  measuresFolded: measuresFoldedProp,
  onMeasuresFoldedChange,
  utilityBar,
  utilityBarFolded: utilityBarFoldedProp,
  onUtilityBarFoldedChange,
  actionPane,
  actionsFolded: actionsFoldedProp,
  onActionsFoldedChange,
  engagement,
  search,
  active = false,
  onActivate,
  fill = false,
}) => {
  const loading = status === "loading";
  const waiting = status === "waiting";
  const bgClass = surfaceClass;
  const newWindowLabel = useLabel("ui.common.newWindow");
  const closeLabel = useLabel("ui.common.close");
  const controlsFocusLabel = useLabel("ui.common.focus");
  const controlsUnfocusLabel = useLabel("ui.common.unfocus");
  const windowOptionsLabel = useLabel("ui.common.windowOptions");
  const actionLabel = useLabel("ui.common.actions");
  const searchLabel = useLabel(UI_WINDOW_SEARCH.title);
  const utilitiesLabel = useLabel("ui.common.utilities");
  const mobile = useUiMobile();
  const focusControl = !mobile && (onMaximize || onMinimize);
  const shellScope = useShellScopeOptional();
  const windowRef = reactHostPort.useRef<HTMLDivElement>(null);
  const windowBodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const measuresOverlayRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [controlsWidthPx, setControlsWidthPx] = reactHostPort.useState(0);
  const controlsRef = reactHostPort.useCallback((node: HTMLDivElement | null) => {
    if (!node) return setControlsWidthPx(0);
    setControlsWidthPx(node.getBoundingClientRect().width);
    if (typeof ResizeObserver === "undefined") return;
    new ResizeObserver(() => setControlsWidthPx(node.getBoundingClientRect().width)).observe(node);
  }, []);
  const [measuresFoldedInternal, setMeasuresFoldedInternal] = reactHostPort.useState(true);
  const measuresFolded = measuresFoldedProp ?? measuresFoldedInternal;
  const setMeasuresFolded = (folded: boolean) => {
    onMeasuresFoldedChange?.(folded);
    if (measuresFoldedProp === undefined) setMeasuresFoldedInternal(folded);
  };
  const [utilityBarFoldedInternal, setUtilityBarFoldedInternal] = reactHostPort.useState(true);
  const utilityBarFolded = utilityBarFoldedProp ?? utilityBarFoldedInternal;
  const setUtilityBarFolded = (folded: boolean) => {
    onUtilityBarFoldedChange?.(folded);
    if (utilityBarFoldedProp === undefined) setUtilityBarFoldedInternal(folded);
  };
  const [actionsFoldedInternal, setActionsFoldedInternal] = reactHostPort.useState(true);
  const actionsFolded = actionsFoldedProp ?? actionsFoldedInternal;
  const setActionsFolded = (folded: boolean) => {
    onActionsFoldedChange?.(folded);
    if (actionsFoldedProp === undefined) setActionsFoldedInternal(folded);
  };
  const [searchFolded, setSearchFolded] = reactHostPort.useState(true);
  const [bodyNarrow, setBodyNarrow] = reactHostPort.useState(false);
  const exclusive = mobile || bodyNarrow;
  const [measuresWidthPx, setMeasuresWidthPx] = reactHostPort.useState(windowMeasuresDefaultWidthPx);
  const [measuresResizeLeftActive, setMeasuresResizeLeftActive] = reactHostPort.useState(false);
  const readMeasuresMaxWidthPx = reactHostPort.useCallback(() => {
    const body = windowBodyRef.current;
    const bodyRect = body?.getBoundingClientRect();
    const bodyWidth = Math.max(body?.clientWidth ?? 0, bodyRect?.width ?? 0, windowMeasuresMaxWidthPx);
    return Math.max(windowMeasuresMinWidthPx, Math.min(windowMeasuresMaxWidthPx, Math.round(bodyWidth) - 8));
  }, []);
  const measuresMaxWidthPx = readMeasuresMaxWidthPx();
  const quickActionsRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const quickActionsSafeArea = useChromePanelSafeArea({ hostRef: windowBodyRef, affordanceRef: quickActionsRef, anchor: "top-left", yieldAxis: "either", gapPx: 0, enabled: !mobile });
  const engagementVisible = !!(engagement || actionPane);
  const searchVisible = !!search;
  const rawOpen: WindowPaneOpen = { engagement: engagementVisible && !actionsFolded, search: searchVisible && !searchFolded, measures: !!measures && !measuresFolded, utility: !!utilityBar && !utilityBarFolded };
  const lastOpenedRef = reactHostPort.useRef<{ last: WindowPaneId | null; seen: WindowPaneOpen }>({ last: null, seen: { engagement: false, search: false, measures: false, utility: false } });
  for (const pane of ["utility", "measures", "search", "engagement"] as const) if (rawOpen[pane] && !lastOpenedRef.current.seen[pane]) lastOpenedRef.current.last = pane;
  lastOpenedRef.current.seen = rawOpen;
  const expanded = resolveWindowPaneExpansion(rawOpen, lastOpenedRef.current.last, exclusive);
  const engagementExpanded = expanded.engagement;
  const searchExpanded = expanded.search;
  const measuresExpanded = expanded.measures;
  const utilityExpanded = expanded.utility;
  const quickActionsCovered = windowQuickActionsCovered(expanded, exclusive);
  const utilityBarMaxHeightPx = useWindowUtilityBarMaxHeightPx(utilityExpanded, windowBodyRef);
  const togglePane = (pane: WindowPaneId, shown: boolean, setFolded: (folded: boolean) => void) => {
    if (shown) return setFolded(true);
    lastOpenedRef.current.last = pane;
    setFolded(false);
    if (!exclusive) return;
    if (pane !== "engagement" && rawOpen.engagement) setActionsFolded(true);
    if (pane !== "search" && rawOpen.search) setSearchFolded(true);
    if (pane !== "measures" && rawOpen.measures) setMeasuresFolded(true);
    if (pane !== "utility" && rawOpen.utility) setUtilityBarFolded(true);
  };

  reactHostPort.useLayoutEffect(() => {
    const body = windowBodyRef.current;
    if (!body) return;
    const sync = () => {
      const width = body.getBoundingClientRect().width;
      setBodyNarrow(width > 0 && width < windowPaneExclusiveMaxWidthPx);
      const quickWidth = quickActionsRef.current?.getBoundingClientRect().width ?? 0;
      if (quickWidth > 0) body.style.setProperty(windowQuickActionsWidthVar, `${Math.ceil(quickWidth)}px`);
      else body.style.removeProperty(windowQuickActionsWidthVar);
    };
    sync();
    const ro = new ResizeObserver(sync);
    ro.observe(body);
    if (quickActionsRef.current) ro.observe(quickActionsRef.current);
    return () => ro.disconnect();
  }, [engagementVisible, engagementExpanded, quickActionsCovered, engagement?.options?.length]);

  useShellKeydown(
    shellScope?.rootRef ?? NULL_SHELL_ROOT_REF,
    (event) => {
      if (!active || !search?.input?.onAbort) return;
      if (routeWindowSearchEscape(search, event, { chromeVisible: searchExpanded, actionActive: searchExpanded })) {
        event.preventDefault();
        event.stopPropagation();
      }
    },
    [active, search, searchExpanded],
  );

  useShellKeydown(
    shellScope?.rootRef ?? NULL_SHELL_ROOT_REF,
    (event) => {
      if (!active || !search?.input || searchExpanded) return;
      if (event.defaultPrevented || event.isComposing) return;
      if (event.key.length !== 1 || event.key === " " || event.ctrlKey || event.metaKey || event.altKey) return;
      if (!shouldRouteKeysToWindowSearch(event.target)) return;
      togglePane("search", false, setSearchFolded);
    },
    [active, search, searchExpanded],
  );

  reactHostPort.useLayoutEffect(() => {
    const body = windowBodyRef.current;
    if (!body || !(engagement || search || measures || actionPane)) return;
    const sync = () => {
      const px = measureWindowChromeScrollClearancePx(body);
      if (px > 0) {
        body.style.setProperty(windowChromeScrollClearanceVar, `${px}px`);
        body.style.setProperty(windowContentDeadLineVar, `${px}px`);
      } else {
        body.style.removeProperty(windowChromeScrollClearanceVar);
        body.style.removeProperty(windowContentDeadLineVar);
      }
    };
    sync();
    const ro = new ResizeObserver(sync);
    ro.observe(body);
    for (const slot of ["window-engagement-overlay", "window-search-overlay", "window-measures-overlay"] as const) {
      const overlay = body.querySelector(`[data-slot="${slot}"]`);
      if (overlay) ro.observe(overlay);
    }
    return () => ro.disconnect();
  }, [active, engagement, search, measures, actionPane, engagementExpanded, searchExpanded, measuresExpanded]);

  if (!isVisible) return null;

  const hasWindowControls = Boolean(onOpenInNewWindow || focusControl || onClose);
  const hasControls = hasWindowControls || controls;

  const controlsContent = hasControls && (
    <div data-dim data-slot="window-controls" data-shared-border-group="" data-shared-border-cells="" className="flex items-stretch">
      {controls}
      {hasWindowControls && (
        <ActionGroup id={childElementId("framework.window", id, "windowControls")}>
          {onOpenInNewWindow && <ActionGroupItem id={childElementId("framework.window", id, "windowControls", "external")} onClick={onOpenInNewWindow} icon={<ExternalLinkIcon />} text={newWindowLabel} />}
          {focusControl && (
            <ActionGroupItem
              id={childElementId("framework.window", id, "windowControls", "maximize")}
              onClick={onMaximize ?? onMinimize}
              icon={onMinimize ? <Minimize2Icon /> : <Maximize2Icon />}
              text={onMinimize ? controlsUnfocusLabel : controlsFocusLabel}
            />
          )}
          {onClose && <ActionGroupItem id={childElementId("framework.window", id, "windowControls", "close")} onClick={onClose} icon={<CloseIcon />} text={closeLabel} />}
        </ActionGroup>
      )}
    </div>
  );

  return (
    <SurfaceScope level="window" fill="surface">
      <GhostRegionShell
        ref={windowRef}
        id={id}
        data-slot="window"
        data-level="window"
        data-elevation-root=""
        data-active={active ? "true" : undefined}
        onDoubleClick={onDoubleClick}
        onPointerDownCapture={(event) => {
          if (!isSurfaceActiveBackgroundPointer(event)) onActivate?.();
        }}
        className={cn(
          "relative flex w-full min-w-0 flex-col overflow-hidden",
          fill ? "h-full min-h-0" : "h-auto max-h-full self-start",
          bgClass,
          getLevelZClass("window"),
          loadingBorderStateClass(loading, active) || waitingBorderStateClass(waiting, active),
          className,
        )}
      >
        {hasControls ? <div ref={controlsRef} data-slot="window-controls-anchor" className="absolute top-0 right-0 z-panel flex items-stretch">{controlsContent}</div> : null}
        <div ref={windowBodyRef} data-slot="window-body" className={cn("relative flex min-w-0 flex-col overflow-hidden", fill ? "min-h-0 flex-1" : "h-auto shrink-0")}>
          <PaneHost className={cn("flex min-w-0 flex-col", fill ? "min-h-0 flex-1" : undefined)}>{error ? <DefaultErrorDisplay error={error} /> : loading && skeleton ? skeleton : children}</PaneHost>
          {measures ? (
            <Pane
              id={childElementId("framework.window", id, "measures")}
              overlaySlot="window-measures-overlay"
              overlayRef={measuresOverlayRef}
              hostRef={windowBodyRef}
              anchor="top-right"
              inlineEdgeReservePx={controlsWidthPx}
              icon={WINDOW_PANE_MEASURES_ICON}
              label={windowOptionsLabel}
              folded={!measuresExpanded}
              onFoldToggle={() => togglePane("measures", measuresExpanded, setMeasuresFolded)}
              toggleId={childElementId("framework.window", id, "measures", measuresExpanded ? "fold" : "unfold")}
              size={measuresWidthPx}
              onSizeChange={setMeasuresWidthPx}
              minSize={windowMeasuresMinWidthPx}
              maxSize={measuresMaxWidthPx}
              onResizeActiveChange={setMeasuresResizeLeftActive}
              stackSlot="window-measures-stack"
              bodySlot="window-measures-body"
              bodyClassName={windowMeasuresBodyClass}
              stackClassName={cn("relative", panelResizeEdgeAccentClass("left", measuresResizeLeftActive))}
              stackDataAttrs={{ "data-level": "pane", "data-folded": measuresExpanded ? undefined : "true" }}
              dimWhenOpen
            >
              {measures}
            </Pane>
          ) : null}
          {engagementVisible && !engagementExpanded && !quickActionsCovered && engagement?.options?.length ? (
            <div
              ref={quickActionsRef}
              data-slot="window-engagement-quick-actions"
              data-safe-area-block={quickActionsSafeArea.blockPx || undefined}
              data-safe-area-inline={quickActionsSafeArea.inlinePx || undefined}
              className={cn("pointer-events-auto absolute flex min-w-0 items-stretch", getLevelZClass("pane"))}
              style={chromePanelSafeAreaStyle("top-left", quickActionsSafeArea, { block: "var(--chip-row-height)", inline: "0px" })}
            >
              <ActionGroup id={childElementId("framework.window", id, "engagement", "quick")} data-shared-border-edges="right bottom left">
                {engagement.options.map((option) => (
                  <ActionGroupItem key={option.id} id={option.id} icon={option.icon} text={typeof option.label === "string" ? option.label : undefined} aria-pressed={option.pressed} data-state={option.pressed === undefined ? undefined : option.pressed ? "on" : "off"} className={cn(option.pressed && interactiveActiveFillClass)} onClick={option.onPress} disabled={option.disabled} />
                ))}
              </ActionGroup>
            </div>
          ) : null}
          {engagementVisible ? (
            <Pane
              id={childElementId("framework.window", id, "engagement")}
              overlaySlot="window-engagement-overlay"
              hostRef={windowBodyRef}
              anchor="top-left"
              icon={WINDOW_PANE_ACTIONS_ICON}
              label={actionLabel}
              folded={!engagementExpanded}
              onFoldToggle={() => togglePane("engagement", engagementExpanded, setActionsFolded)}
              toggleId={childElementId("framework.window", id, "engagement", "toggle")}
              stackSlot="window-engagement-zone"
              bodySlot="window-engagement-body"
              bodyClassName={windowEngagementBodyClass}
              className={!engagementExpanded && !quickActionsCovered && engagement?.options?.length ? "min-w-[var(--window-quick-actions-width,0px)]" : undefined}
              stackClassName={!engagementExpanded && !quickActionsCovered && engagement?.options?.length ? "w-full" : undefined}
              stackDataAttrs={{ "data-pane-chip-fill": !engagementExpanded && !quickActionsCovered && engagement?.options?.length ? "true" : undefined }}
              dimWhenOpen
            >
              {engagement ? <Engagement {...engagement} /> : null}
              {actionPane}
            </Pane>
          ) : null}
          {search && searchVisible ? (
            <Pane
              id={childElementId("framework.window", id, "search")}
              overlaySlot="window-search-overlay"
              hostRef={windowBodyRef}
              anchor="top-middle"
              icon={WINDOW_PANE_SEARCH_ICON}
              label={searchLabel}
              folded={!searchExpanded}
              onFoldToggle={() => {
                togglePane("search", searchExpanded, setSearchFolded);
                if (!searchExpanded && search.input) queueMicrotask(() => focusActiveSearchInput());
              }}
              toggleId={childElementId("framework.window", id, "search", "toggle")}
              stackSlot="window-search-zone"
              bodySlot="window-search-body"
              bodyClassName={windowEngagementBodyClass}
              dimWhenOpen
            >
              <Search {...search} active={searchExpanded} />
            </Pane>
          ) : null}
          <Pane
            id={childElementId("framework.window", id, "utilityBar")}
            overlaySlot="utility-bar-overlay"
            hostRef={windowBodyRef}
            anchor="bottom-left"
            icon={WINDOW_PANE_UTILITIES_ICON}
            label={utilitiesLabel}
            folded={!utilityExpanded}
            toggleDisabled={!utilityBar}
            onFoldToggle={() => togglePane("utility", utilityExpanded, setUtilityBarFolded)}
            toggleId={childElementId("framework.window", id, "utilityBar", utilityExpanded ? "fold" : "unfold")}
            stackSlot="utility-bar"
            bodySlot="utility-bar-body"
            bodyClassName={utilityBarBodyClass}
            bodyStyle={utilityBarMaxHeightPx > 0 ? { maxHeight: utilityBarMaxHeightPx } : undefined}
            stackDataAttrs={{ "data-level": "pane", "data-folded": utilityExpanded ? undefined : "true" }}
            dimWhenOpen={Boolean(utilityBar)}
          >
            {utilityBar}
          </Pane>
        </div>
      </GhostRegionShell>
    </SurfaceScope>
  );
};

export { Window };

// #endregion 🌊️Window

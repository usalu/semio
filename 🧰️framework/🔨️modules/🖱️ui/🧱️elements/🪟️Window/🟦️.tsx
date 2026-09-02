// #region 🧲️Header
// 💻️ framework/ui/elements/🪟️Window/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { childElementId } from "../🆔️ElementId/🟦️.tsx";
import { ActionGroup, ActionGroupItem } from "../⚡️ActionGroup/🟦️.tsx";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { loadingBorderStateClass, waitingBorderStateClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { useShellScopeOptional, NULL_SHELL_ROOT_REF, useShellKeydown } from "../🐚️ShellScope/🟦️.tsx";
import { SurfaceScope, isSurfaceActiveBackgroundPointer, getLevelZClass } from "../🌈️Surface/🟦️.tsx";
import { measureWindowChromeScrollClearancePx, windowChromeScrollClearanceVar, windowContentDeadLineVar } from "../🚧️WindowContentDeadLine/🟦️.tsx";
import { type UiStatus, type EngagementSpec, type SearchSpec, UI_WINDOW_SEARCH, useUiMobile, routeWindowSearchEscape, shouldRouteKeysToWindowSearch, windowMeasuresDefaultWidthPx, windowMeasuresMinWidthPx, windowMeasuresMaxWidthPx, uiSpacingPx, ExternalLinkIcon, GhostRegionShell, PaneHost, Pane, WINDOW_PANE_MEASURES_ICON, WINDOW_PANE_ACTIONS_ICON, WINDOW_PANE_SEARCH_ICON, WINDOW_PANE_UTILITIES_ICON, Engagement, Search, panelResizeEdgeAccentClass, windowMeasuresBodyClass, windowEngagementBodyClass, windowSearchBodyClass, utilityBarBodyClass, focusActiveSearchInput } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import { Minimize2Icon, Maximize2Icon, CloseIcon } from "../🔣️Icons/🟦️.tsx";
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
  showControls?: boolean;
  onOpenInNewWindow?: () => void;
  onMaximize?: () => void;
  onMinimize?: () => void;
  onClose?: () => void;
  controls?: React.ReactNode;
  measures?: React.ReactNode;
  utilityBar?: React.ReactNode;
  /** @emoji 🎛️ Controlled fold state for the Window Options (measures) rail (default true); externally settable so the introduction walkthrough can force-unfold a step's measure target into view. */
  measuresFolded?: boolean;
  /** @emoji 🎛️ Fires when the user or a redirect toggles the Window Options rail fold state. */
  onMeasuresFoldedChange?: (folded: boolean) => void;
  /** @emoji 🎛️ Controlled fold state for the Utilities rail (default true); externally settable so the introduction walkthrough can force-unfold a step's utility anchor into view. */
  utilityBarFolded?: boolean;
  /** @emoji 🎛️ Fires when the user or a redirect toggles the Utilities rail fold state. */
  onUtilityBarFoldedChange?: (folded: boolean) => void;
  /** @emoji 🎛️ Categorized ad-hoc actions tree, merged into the top-left Actions pane below the active {@link engagement} (when present); folds to a chip by default. */
  actionPane?: React.ReactNode;
  /** @emoji 🎛️ Controlled fold state for the merged top-left Actions pane (default true); externally settable so the palette/keybinding redirect can force-unfold. */
  actionsFolded?: boolean;
  /** @emoji 🎛️ Fires when the user or a redirect toggles the Actions pane fold state. */
  onActionsFoldedChange?: (folded: boolean) => void;
  /** @emoji 💬️ Active engagement content (options/status/control) rendered above {@link actionPane} inside the same top-left Actions pane. */
  engagement?: EngagementSpec;
  /** @emoji 🔎️ Top-middle floating search pane: typed action input with autocomplete possibles. */
  search?: SearchSpec;
  active?: boolean;
  onActivate?: () => void;
  /** @emoji 📐️ When true, the window body grows to fill its dock pane (canvas hosts). */
  fill?: boolean;
}

/**
 * WindowProps holds the data fields for a WindowProps record.
 **/
interface WindowProps extends WindowConfig {
  isVisible?: boolean;
}

/**
 * DefaultErrorDisplay holds the data fields for a DefaultErrorDisplay record.
 **/
const DefaultErrorDisplay: React.FC<{ error: Error }> = ({ error }) => {
  // 🎨️ Transparent — rendered inside the Window body, which already paints the window-level surface.
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
 * Window holds the data fields for a Window record.
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

const Window: React.FC<WindowProps> = ({
  id,
  children,
  onDoubleClick,
  className = "",
  isVisible = true,
  status = "idle",
  error = null,
  skeleton,
  showControls = false,
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
  const measuresFocusLabel = useLabel("ui.common.focus");
  const measuresUnfocusLabel = useLabel("ui.common.unfocus");
  const actionLabel = useLabel("ui.common.actions");
  const searchLabel = useLabel(UI_WINDOW_SEARCH.title);
  const utilitiesLabel = useLabel("ui.common.utilities");
  // 📱️ Windows always take the full space on mobile — Focus/Unfocus is meaningless there and is hidden.
  const mobile = useUiMobile();
  const focusControl = !mobile && (onMaximize || onMinimize);
  // 🐚️ Gates the search-routing keydown listeners below to this shell — absent outside a `ShellScopeProvider` (tests), where they simply stay inert.
  const shellScope = useShellScopeOptional();
  const windowRef = reactHostPort.useRef<HTMLDivElement>(null);
  const windowBodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const measuresOverlayRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [measuresFoldedInternal, setMeasuresFoldedInternal] = reactHostPort.useState(true);
  const measuresFolded = measuresFoldedProp ?? measuresFoldedInternal;
  const setMeasuresFolded = (folded: boolean) => {
    onMeasuresFoldedChange?.(folded);
    if (measuresFoldedProp === undefined) setMeasuresFoldedInternal(folded);
  };
  const [measuresExpanded, setMeasuresExpanded] = reactHostPort.useState(false);
  const [searchFolded, setSearchFolded] = reactHostPort.useState(true);
  // 🎛️ Controlled-with-default fold state for the bottom-left Utilities rail (default true).
  const [utilityBarFoldedInternal, setUtilityBarFoldedInternal] = reactHostPort.useState(true);
  const utilityBarFolded = utilityBarFoldedProp ?? utilityBarFoldedInternal;
  const setUtilityBarFolded = (folded: boolean) => {
    onUtilityBarFoldedChange?.(folded);
    if (utilityBarFoldedProp === undefined) setUtilityBarFoldedInternal(folded);
  };
  // 🎛️ Controlled-with-default fold state for the merged top-left Actions pane (default true) —
  // shared by the active engagement and the categorized ad-hoc actions tree, one toggle for both.
  const [actionsFoldedInternal, setActionsFoldedInternal] = reactHostPort.useState(true);
  const actionsFolded = actionsFoldedProp ?? actionsFoldedInternal;
  const setActionsFolded = (folded: boolean) => {
    onActionsFoldedChange?.(folded);
    if (actionsFoldedProp === undefined) setActionsFoldedInternal(folded);
  };
  const [measuresWidthPx, setMeasuresWidthPx] = reactHostPort.useState(windowMeasuresDefaultWidthPx);
  const [measuresResizeLeftActive, setMeasuresResizeLeftActive] = reactHostPort.useState(false);
  const utilityBarMaxHeightPx = useWindowUtilityBarMaxHeightPx(!utilityBarFolded && !!utilityBar, windowBodyRef);
  const readMeasuresMaxWidthPx = reactHostPort.useCallback(() => {
    const body = windowBodyRef.current;
    const bodyRect = body?.getBoundingClientRect();
    const bodyWidth = Math.max(body?.clientWidth ?? 0, bodyRect?.width ?? 0, windowMeasuresMaxWidthPx);
    return Math.max(windowMeasuresMinWidthPx, Math.min(windowMeasuresMaxWidthPx, Math.round(bodyWidth) - 8));
  }, []);
  const measuresMaxWidthPx = readMeasuresMaxWidthPx();
  const engagementVisible = !measuresExpanded && !!(engagement || actionPane);
  const engagementExpanded = engagementVisible && !actionsFolded;
  const searchVisible = !measuresExpanded && !!search;
  const searchExpanded = searchVisible && !searchFolded;

  reactHostPort.useEffect(() => {
    if (!measuresExpanded) return;
    setActionsFolded(true);
    setSearchFolded(true);
  }, [measuresExpanded, onActionsFoldedChange, actionsFoldedProp]);

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

  // 🔎️ Typing anywhere in an active window with a folded search pane unfolds it, mirroring a spotlight
  // search, so the shell-scoped routing that applies the keystroke (see Mode) has a mounted field to
  // land in. Purely additive — it only flips the fold flag, never touches the value/dispatch itself.
  useShellKeydown(
    shellScope?.rootRef ?? NULL_SHELL_ROOT_REF,
    (event) => {
      if (!active || !search?.input || searchExpanded) return;
      if (event.defaultPrevented || event.isComposing) return;
      if (event.key.length !== 1 || event.key === " " || event.ctrlKey || event.metaKey || event.altKey) return;
      if (!shouldRouteKeysToWindowSearch(event.target)) return;
      setSearchFolded(false);
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
  }, [active, engagement, search, measures, actionPane, engagementExpanded, searchExpanded, measuresFolded, measuresExpanded]);

  if (!isVisible) return null;

  const hasControls = showControls || controls || onOpenInNewWindow || focusControl || onClose;

  const controlsContent = hasControls && (
    <div data-dim className="flex items-stretch gap-single">
      {controls}
      {(showControls || onOpenInNewWindow || focusControl || onClose) && (
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
        {hasControls ? <div className="absolute top-1 right-1 z-panel flex items-stretch gap-single">{controlsContent}</div> : null}
        <div ref={windowBodyRef} data-slot="window-body" className={cn("relative flex min-w-0 flex-col overflow-hidden", fill ? "min-h-0 flex-1" : "h-auto shrink-0")}>
          {/* 🪟️ PaneHost wraps window body content so deep canvas hosts (e.g. projection switcher via usePaneSlot) receive PaneHostContext; the portal mount is a sibling overlay. */}
          <PaneHost className={cn("flex min-w-0 flex-col", fill ? "min-h-0 flex-1" : undefined)}>{error ? <DefaultErrorDisplay error={error} /> : loading && skeleton ? skeleton : children}</PaneHost>
          {measures ? (
            <Pane
              id={childElementId("framework.window", id, "measures")}
              overlaySlot="window-measures-overlay"
              overlayRef={measuresOverlayRef}
              anchor="top-right"
              icon={WINDOW_PANE_MEASURES_ICON}
              label={windowOptionsLabel}
              folded={measuresFolded}
              expanded={measuresExpanded}
              onFoldToggle={() => {
                if (measuresFolded) setMeasuresFolded(false);
                else {
                  setMeasuresExpanded(false);
                  setMeasuresFolded(true);
                }
              }}
              toggleId={childElementId("framework.window", id, "measures", measuresFolded ? "unfold" : "fold")}
              foldControlId={childElementId("framework.window", id, "measures", "fold")}
              enlarge={{
                id: childElementId("framework.window", id, "measures", "span"),
                slot: "window-measures-span",
                icon: measuresExpanded ? <Minimize2Icon className="size-small" /> : <Maximize2Icon className="size-small" />,
                label: measuresExpanded ? measuresUnfocusLabel : measuresFocusLabel,
                onClick: () => (measuresExpanded ? setMeasuresExpanded(false) : setMeasuresExpanded(true)),
              }}
              size={measuresWidthPx}
              onSizeChange={setMeasuresWidthPx}
              minSize={windowMeasuresMinWidthPx}
              maxSize={measuresMaxWidthPx}
              onResizeActiveChange={setMeasuresResizeLeftActive}
              stackSlot="window-measures-stack"
              bodySlot="window-measures-body"
              bodyClassName={windowMeasuresBodyClass}
              stackClassName={cn("relative", panelResizeEdgeAccentClass("left", measuresResizeLeftActive))}
              stackDataAttrs={{ "data-level": "pane", "data-folded": measuresFolded ? "true" : undefined }}
              dimWhenOpen
            >
              {measures}
            </Pane>
          ) : null}
          {engagementVisible ? (
            <Pane
              id={childElementId("framework.window", id, "engagement")}
              overlaySlot="window-engagement-overlay"
              anchor="top-left"
              icon={WINDOW_PANE_ACTIONS_ICON}
              label={actionLabel}
              folded={!engagementExpanded}
              onFoldToggle={() => setActionsFolded(!actionsFolded)}
              toggleId={childElementId("framework.window", id, "engagement", "toggle")}
              stackSlot="window-engagement-zone"
              bodySlot="window-engagement-body"
              bodyClassName={windowEngagementBodyClass}
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
              anchor="top-middle"
              icon={WINDOW_PANE_SEARCH_ICON}
              label={searchLabel}
              folded={!searchExpanded}
              onFoldToggle={() => {
                if (searchExpanded) {
                  setSearchFolded(true);
                  return;
                }
                setSearchFolded(false);
                if (search?.input) queueMicrotask(() => focusActiveSearchInput());
              }}
              toggleId={childElementId("framework.window", id, "search", "toggle")}
              stackSlot="window-search-zone"
              bodySlot="window-search-body"
              bodyClassName={windowSearchBodyClass}
              dimWhenOpen
            >
              <Search {...search} active={searchExpanded} />
            </Pane>
          ) : null}
          {!measuresExpanded ? (
            <Pane
              id={childElementId("framework.window", id, "utilityBar")}
              overlaySlot="utility-bar-overlay"
              anchor="bottom-left"
              icon={WINDOW_PANE_UTILITIES_ICON}
              label={utilitiesLabel}
              folded={utilityBarFolded}
              toggleDisabled={!utilityBar}
              onFoldToggle={() => setUtilityBarFolded(!utilityBarFolded)}
              toggleId={childElementId("framework.window", id, "utilityBar", utilityBarFolded ? "unfold" : "fold")}
              stackSlot="utility-bar"
              bodySlot="utility-bar-body"
              bodyClassName={utilityBarBodyClass}
              bodyStyle={utilityBarMaxHeightPx > 0 ? { maxHeight: utilityBarMaxHeightPx } : undefined}
              stackDataAttrs={{ "data-level": "pane", "data-folded": utilityBarFolded ? "true" : undefined }}
              dimWhenOpen={Boolean(utilityBar)}
            >
              {utilityBar}
            </Pane>
          ) : null}
        </div>
      </GhostRegionShell>
    </SurfaceScope>
  );
};

export { Window };

// #endregion 🌊️Window

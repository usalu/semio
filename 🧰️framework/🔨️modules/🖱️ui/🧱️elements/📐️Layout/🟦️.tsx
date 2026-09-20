// #region 🧲️Header
// 💻️ framework/ui/elements/📐️Layout/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { panelTabFirstDraggableElementId } from "@semio-tech/framework";
import { type UiStatus } from "@semio-tech/ui-styling";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { useFirstDraggableElementAlias } from "../🆔️ElementId/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { glassClass, surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { type PanelProps, PANEL_DEFAULT_SIZE_PX, Panel, PanelTreeUnitsPane } from "../🖼️Panel/🟦️.tsx";
import { type PanelTabNode, usePanelTabSelection, findPanelTabNode, PanelTabBar, progressPanelTabSelection, resolvePanelBranchBodyLeaf } from "../🧭️PanelTabBar/🟦️.tsx";
import { Scrollable } from "../📜️Scrollable/🟦️.tsx";
import { CanvasSkeleton } from "../🦴️Skeletons/🟦️.tsx";
import { LevelProvider, SurfaceScope } from "../🌈️Surface/🟦️.tsx";
import { type Anchor, ANCHORS, UiMobileProvider, GhostProvider, PanelGhostRoot, chromeStatusBorderClass, shellChromeFrameLayerClass } from "../../🎯️targets/⚛️react/🟦️";
// #endregion 🔌️Adapters

// #region 🪨️Layout
// Top-level layout orchestrating navbar, panels, canvas, and footer.
// Consumers MUST provide a canvas element.

// #region 📱️LayoutMobilePanel
/** @emoji 📱️ The mobile panel configuration owned by {@link Layout}. */
export interface LayoutMobilePanelProps {
  visible?: boolean;
  tabs: readonly PanelTabNode[];
  activeTabPath?: readonly string[];
  onActiveTabPathChange?: (path: readonly string[]) => void;
  /** @emoji 🌱️ Per-branch drill-down memory (see {@link progressPanelTabSelection}). */
  pathMemory?: Readonly<Record<string, string>>;
  onPathMemoryChange?: (memory: Readonly<Record<string, string>>) => void;
  /** @emoji 🌱️ Persisted tree section/group expansion across every leaf tab's units (see {@link PanelTreeUnitsPane}). */
  treeOpenStates?: Readonly<Record<string, boolean>>;
  onTreeOpenStateChange?: (id: string, open: boolean) => void;
  /** @emoji ♻️ See {@link PanelProps.treeContentRevision}. */
  treeContentRevision?: unknown;
  className?: string;
}

/** @emoji 📱️ Full-height tabbed panel for Layout's mobile branch. */
const LayoutMobilePanel: React.FC<LayoutMobilePanelProps> = ({ visible = false, tabs, activeTabPath, onActiveTabPathChange, pathMemory, onPathMemoryChange, treeOpenStates, onTreeOpenStateChange, treeContentRevision, className = "" }) => {
  // 🌱️ `visible: true` — LayoutMobilePanel has no folded state of its own (it renders nothing at all instead, below);
  // this just keeps `usePanelTabSelection`'s open/fold branches inert so it behaves as pure path/memory selection.
  const { resolvedPath, handlePathChange } = usePanelTabSelection({ tabs, visible: true, activeTabPath, onActiveTabPathChange, pathMemory, onPathMemoryChange });
  const panelContentRef = reactHostPort.useRef<HTMLDivElement>(null);
  const activeNode = findPanelTabNode(tabs, resolvedPath);
  const firstDraggableAlias = visible && activeNode ? panelTabFirstDraggableElementId(activeNode.id) : null;
  useFirstDraggableElementAlias(panelContentRef, firstDraggableAlias);

  if (!visible || tabs.length === 0) return null;

  const showTabBar = tabs.length > 0;
  const bodyLeaf = activeNode?.kind === "leaf" ? activeNode : activeNode ? resolvePanelBranchBodyLeaf(activeNode, pathMemory ?? {}) : undefined;
  const activeTabTrees = bodyLeaf?.trees ?? null;

  return (
    <LevelProvider level="panel">
      <PanelGhostRoot
        data-slot="panel"
        data-panel="mobilePanel"
        data-panel-visible="true"
        data-active-tab-id={activeNode?.id}
        id={activeNode ? `framework.panelTab.${activeNode.id}` : undefined}
        className={cn("relative w-full flex-1 min-h-0 text-foreground flex flex-col box-border overflow-hidden", className)}
      >
        <div data-dim aria-hidden className={cn("pointer-events-none absolute inset-0 z-0", glassClass)} />
        <div data-dim data-slot="chrome-frame" aria-hidden className={shellChromeFrameLayerClass} />
        {showTabBar ? <PanelTabBar activePath={resolvedPath} onActivePathChange={handlePathChange} tabs={tabs} variant="mobile" /> : null}
        <Scrollable className="relative z-10 flex-1 min-h-0">
          <div ref={panelContentRef} data-dim data-slot="mobile-panel-content" className="flex min-h-0 flex-1 flex-col">
            {activeTabTrees && bodyLeaf ? <PanelTreeUnitsPane tabId={bodyLeaf.id} units={activeTabTrees} treeOpenStates={treeOpenStates} onTreeOpenStateChange={onTreeOpenStateChange} treeContentRevision={treeContentRevision} /> : null}
          </div>
        </Scrollable>
      </PanelGhostRoot>
    </LevelProvider>
  );
};
// #endregion 📱️LayoutMobilePanel

/**
 * Props interface for the top-level Layout component.
 **/
export interface LayoutProps {
  navbar?: React.ReactNode;
  /** @emoji 🎥️ Optional chrome row directly under `navbar`, above the canvas/panels row (e.g. {@link TutorialBar}) — `flex-shrink-0` like `navbar`/`footer`, never affecting the middle column's z-index invariant below. */
  subnavbar?: React.ReactNode;
  footer?: React.ReactNode;
  /** @emoji 🧭️ Per-anchor panel config — panels float over the navbar/footer/canvas, keyed by which anchor they grow from. */
  panels?: Partial<Record<Anchor, Omit<PanelProps, "anchor">>>;
  mobilePanel?: LayoutMobilePanelProps;
  canvas: React.ReactNode;
  /** @emoji 🌀️ When set, paints a loading/waiting ring on the canvas viewport wrapper. */
  canvasStatus?: UiStatus;
  /** @emoji 🦴 Optional skeleton shown inside the canvas ring while `canvasStatus` is busy. */
  canvasSkeleton?: React.ReactNode;
  mobile?: boolean;
  className?: string;
}

/** @emoji 🛟️ Which side of the middle region each anchor grows from — a middle anchor reserves nothing, because it is centered and has no edge of its own to give. */
const LAYOUT_LEFT_ANCHORS: readonly Anchor[] = ["top-left", "left-middle", "bottom-left"];
const LAYOUT_RIGHT_ANCHORS: readonly Anchor[] = ["top-right", "right-middle", "bottom-right"];

/**
 * @emoji 🛟️ The canvas column's own padding, one side at a time, so an open anchored {@link Panel}
 * never paints over the window area — and therefore never over an interactive rail inside a window.
 *
 * A panel is absolutely positioned in this same region at `z-panel`, above `z-window` and above a
 * window's engagement overlay, and the two are different stacking contexts, so a covered rail can
 * neither restack nor (being full height) step aside: {@link chromePanelSafeArea} answers "clear"
 * for it, because neither axis can clear the panel inside the window. Measured on 📐️generation3d at
 * 1600×1000 (ticket 26/09/18, slices PB1 §4 / PB3 §4): the right-docked History panel spans
 * x 1297–1597 and the window's Actions rail x 1094–1394, so the rail's last 97 px — including the
 * centre of `addWidget`'s `kind` combobox at x 1311 — were unclickable.
 *
 * Reserving the band is the only fix that holds for a rail of any height: the window column stops at
 * the panel's outer edge, and a shell with no open panel keeps its authored layout byte-for-byte
 * (both sides answer `undefined`). The reserve carries the panel's own flush inset twice, once for
 * the region edge it sits on and once as the gap between it and the canvas.
 */
export function layoutPanelReserveStyle(panels: LayoutProps["panels"]): React.CSSProperties {
  const widest = (anchors: readonly Anchor[]) =>
    anchors.reduce((reserved, anchor) => {
      const panel = panels?.[anchor];
      return panel?.visible && panel.tabs.length > 0 ? Math.max(reserved, panel.size ?? PANEL_DEFAULT_SIZE_PX) : reserved;
    }, 0);
  const left = widest(LAYOUT_LEFT_ANCHORS);
  const right = widest(LAYOUT_RIGHT_ANCHORS);
  return {
    paddingLeft: left > 0 ? `calc(${left}px + 2 * var(--spacing-single))` : undefined,
    paddingRight: right > 0 ? `calc(${right}px + 2 * var(--spacing-single))` : undefined,
  };
}

const Layout: React.FC<LayoutProps> = ({ navbar, subnavbar, footer, panels, mobilePanel, canvas, canvasStatus, canvasSkeleton, mobile = false, className = "" }) => (
  <UiMobileProvider mobile={mobile}>
    <GhostProvider>
      {/* 🎨️ One continuous base floor for navbar + canvas + footer — chrome rows stay transparent over this paint. */}
      <div data-slot="layout" data-level="base" className={cn("relative flex flex-col overflow-hidden", surfaceClass, mobile ? "h-full w-full" : "h-screen w-screen", className)}>
        <SurfaceScope level="base" fill="surface">
          {navbar && <div className="flex-shrink-0">{navbar}</div>}
          {subnavbar && <div className="flex-shrink-0">{subnavbar}</div>}
          {mobile ? (
            <div className="flex flex-col flex-1 min-h-0">
              {mobilePanel && mobilePanel.visible && <LayoutMobilePanel {...mobilePanel} />}
              {/* 📱️ The canvas stays mounted (never unmounted) while the mobile panel covers it, so the WASM/3D
                  world keeps its context instead of replugging on every toggle — it just stops being visible. */}
              <div className={cn("flex-1 min-w-0 min-h-0 relative", mobilePanel?.visible && "hidden", chromeStatusBorderClass(canvasStatus))}>
                {canvasStatus === "loading" || canvasStatus === "waiting" ? (canvasSkeleton ?? <CanvasSkeleton />) : canvas}
              </div>
            </div>
          ) : (
            // Positioned within this region (relative, between navbar and footer), not the whole display — panels
            // open below the navbar / above the footer instead of floating over them, while still overlaying canvas
            // the same way a window's options rail overlays its own canvas.
            <div className="flex flex-1 min-h-0 relative">
              {/* 🎓️ No z-index here (was z-0): trapping this column in its own stacking context would make
                  windows unreachable by [data-introduction-elevated] — a window can only rise above the
                  fullscreen introduction veil if it participates in the root stacking context. */}
              {/* 🛟️ Reserved, not overlaid — see {@link layoutPanelReserveStyle}: a docked panel's band is
                  taken out of the window column so no interactive rail can end up underneath it. */}
              <div data-slot="layout-canvas-column" className="flex flex-col flex-1 min-w-0 relative" style={layoutPanelReserveStyle(panels)}>
                <div className="flex flex-1 min-h-0 relative">
                  <div className={cn("flex-1 min-w-0 min-h-0 relative", chromeStatusBorderClass(canvasStatus))}>
                    {canvasStatus === "loading" || canvasStatus === "waiting" ? (canvasSkeleton ?? <CanvasSkeleton />) : canvas}
                  </div>
                </div>
              </div>
              {ANCHORS.map((anchor) => {
                const panelProps = panels?.[anchor];
                return panelProps ? <Panel key={anchor} {...panelProps} anchor={anchor} /> : null;
              })}
            </div>
          )}
          {footer && <div className="flex-shrink-0">{footer}</div>}
        </SurfaceScope>
      </div>
    </GhostProvider>
  </UiMobileProvider>
);

export { Layout };

// #endregion 🪨️Layout

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
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { useFirstDraggableElementAlias } from "../🆔️ElementId/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { glassClass, surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️component.ts";
import { type PanelProps, Panel, PanelTreeUnitsPane } from "../🖼️Panel/🟦️component.tsx";
import { type PanelTabNode, usePanelTabSelection, findPanelTabNode, PanelTabBar, progressPanelTabSelection } from "../📑️PanelTabBar/🟦️component.tsx";
import { Scrollable } from "../📜️Scrollable/🟦️component.tsx";
import { CanvasSkeleton } from "../🦴️Skeletons/🟦️component.tsx";
import { LevelProvider, SurfaceScope } from "../🌈️Surface/🟦️component.tsx";
import { type Anchor, ANCHORS, UiMobileProvider, GhostProvider, PanelGhostRoot, chromeStatusBorderClass, shellChromeFrameLayerClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
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
  const activeTabTrees = activeNode?.kind === "leaf" ? activeNode.trees : null;

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
            {activeTabTrees && activeNode ? <PanelTreeUnitsPane tabId={activeNode.id} units={activeTabTrees} treeOpenStates={treeOpenStates} onTreeOpenStateChange={onTreeOpenStateChange} treeContentRevision={treeContentRevision} /> : null}
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
              <div className="flex flex-col flex-1 min-w-0 relative">
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

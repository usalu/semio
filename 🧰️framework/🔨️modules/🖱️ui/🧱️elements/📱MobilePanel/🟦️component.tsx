// #region 🧲️Header
// 💻️ framework/ui/elements/📱MobilePanel/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { panelTabFirstDraggableElementId } from "@semio-tech/framework-core";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { useFirstDraggableElementAlias } from "../🆔ElementId/🟦️component.tsx";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { Scrollable } from "../📜Scrollable/🟦️component.tsx";
import { PanelTreeUnitsPane, type PanelProps } from "../🖼️Panel/🟦️component.tsx";
import { glassClass } from "../🏷️ClassNames/🟦️component.tsx";
import { LevelProvider } from "../🌈️Surface/🟦️component.tsx";
import { PanelGhostRoot, shellChromeFrameLayerClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { type PanelTabNode, usePanelTabSelection, findPanelTabNode, PanelTabBar, progressPanelTabSelection } from "../📑PanelTabBar/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 💧️MobilePanel
// Full-height tabbed panel for mobile layouts, filling the space between navbar and footer. Not
// resizable. All tabs in one panel.

/**
 * Props interface for the MobilePanel component.
 **/
export interface MobilePanelProps {
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

/**
 * MobilePanel is a full-height tabbed panel for mobile layouts.
 * It merges all tabs into a single non-resizable panel filling the available space.
 **/
const MobilePanel: React.FC<MobilePanelProps> = ({ visible = false, tabs, activeTabPath, onActiveTabPathChange, pathMemory, onPathMemoryChange, treeOpenStates, onTreeOpenStateChange, treeContentRevision, className = "" }) => {
  // 🌱️ `visible: true` — MobilePanel has no folded state of its own (it renders nothing at all instead, below);
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
export { MobilePanel };

// #endregion 💧️MobilePanel

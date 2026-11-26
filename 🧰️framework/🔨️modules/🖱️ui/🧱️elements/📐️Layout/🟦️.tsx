// #region 🧲️Header
// 💻️ framework/ui/elements/📐️Layout/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { panelTabFirstDraggableElementId } from "../../🔨️modules/🆔️element-identity/🟦️.ts";
import { type UiStatus } from "@semio-tech/ui-styling";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { useFirstDraggableElementAlias } from "../🆔️ElementId/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { type PanelProps, Panel, PanelTreeUnitsPane } from "../🖼️Panel/🟦️.tsx";
import { type PanelTabNode, usePanelTabSelection, findPanelTabNode, PanelTabBar, progressPanelTabSelection, resolvePanelBranchBodyLeaf } from "../🧭️PanelTabBar/🟦️.tsx";
import { Scrollable } from "../📜️Scrollable/🟦️.tsx";
import { CanvasSkeleton } from "../🦴️Skeletons/🟦️.tsx";
import { CanvasBottomChipRow, type CanvasBottomChips } from "../🎨️Canvas/🦶️bottom-chips/🟦️.tsx";
import { Surface, SurfaceScope } from "../🌈️Surface/🟦️.tsx";
import { chromeStatusBorderClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { type Anchor, ANCHORS } from "../../🔨️modules/⚓️panel-anchor/🟦️.ts";
import { publishShellChromePanelBox, safeAreaBoxFromRect } from "../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";
import { useShellScopeOptional } from "../🐚️ShellScope/🟦️.tsx";
import { useSharedBorderFrameRef } from "../🗂️WindowChrome/shared-border.ts";
import { UiMobileProvider } from "../../🔨️modules/📱️ui-mobile-context/🟦️.tsx";
import { GhostProvider, PanelGhostRoot } from "../👻️PanelGhost/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🪨️Layout
// Top-level layout orchestrating navbar, panels, canvas, and footer.
// Consumers MUST provide a canvas element.

// #region 📱️LayoutMobilePanel
/** 📱️ The mobile panel configuration owned by {@link Layout}. */
export interface LayoutMobilePanelProps {
  visible?: boolean;
  tabs: readonly PanelTabNode[];
  activeTabPath?: readonly string[];
  onActiveTabPathChange?: (path: readonly string[]) => void;
  /** 🌱️ Per-branch drill-down memory (see {@link progressPanelTabSelection}). */
  pathMemory?: Readonly<Record<string, string>>;
  onPathMemoryChange?: (memory: Readonly<Record<string, string>>) => void;
  /** 🌲️ Persisted tree section/group expansion across every leaf tab's units (see {@link PanelTreeUnitsPane}). */
  treeOpenStates?: Readonly<Record<string, boolean>>;
  onTreeOpenStateChange?: (id: string, open: boolean) => void;
  /** ♻️ See {@link PanelProps.treeContentRevision}. */
  treeContentRevision?: unknown;
  className?: string;
}

/** 🗂️ Full-height tabbed panel for Layout's mobile branch. */
const LayoutMobilePanel: React.FC<LayoutMobilePanelProps> = ({ visible = false, tabs, activeTabPath, onActiveTabPathChange, pathMemory, onPathMemoryChange, treeOpenStates, onTreeOpenStateChange, treeContentRevision, className = "" }) => {
  const { resolvedPath, handlePathChange } = usePanelTabSelection({ tabs, visible: true, activeTabPath, onActiveTabPathChange, pathMemory, onPathMemoryChange });
  const panelContentRef = reactHostPort.useRef<HTMLDivElement>(null);
  const activeNode = findPanelTabNode(tabs, resolvedPath);
  const firstDraggableAlias = visible && activeNode ? panelTabFirstDraggableElementId(activeNode.id) : null;
  useFirstDraggableElementAlias(panelContentRef, firstDraggableAlias);
  const frameRef = useSharedBorderFrameRef<HTMLDivElement>();

  if (!visible || tabs.length === 0) return null;

  const showTabBar = tabs.length > 0;
  const bodyLeaf = activeNode?.kind === "leaf" ? activeNode : activeNode ? resolvePanelBranchBodyLeaf(activeNode, pathMemory ?? {}) : undefined;
  const activeTabTrees = bodyLeaf?.trees ?? null;

  return (
    <SurfaceScope level="panel" fill="glass">
      <PanelGhostRoot
        ref={frameRef}
        data-shared-border-frame=""
        data-slot="panel"
        data-panel="mobilePanel"
        data-level="panel"
        data-panel-visible="true"
        data-active-tab-id={activeNode?.id}
        id={activeNode ? `framework.panelTab.${activeNode.id}` : undefined}
        className={cn("relative w-full flex-1 min-h-0 text-foreground flex flex-col box-border overflow-hidden", className)}
      >
        <div data-dim aria-hidden className={cn("pointer-events-none absolute inset-0 z-0", glassClass)} />
        {showTabBar ? <PanelTabBar activePath={resolvedPath} onActivePathChange={handlePathChange} tabs={tabs} variant="mobile" /> : null}
        <Scrollable className="relative z-10 flex-1 min-h-0">
          <div ref={panelContentRef} data-dim data-slot="mobile-panel-content" className="flex min-h-0 flex-1 flex-col">
            {activeTabTrees && bodyLeaf ? <PanelTreeUnitsPane tabId={bodyLeaf.id} units={activeTabTrees} treeOpenStates={treeOpenStates} onTreeOpenStateChange={onTreeOpenStateChange} treeContentRevision={treeContentRevision} /> : null}
          </div>
        </Scrollable>
      </PanelGhostRoot>
    </SurfaceScope>
  );
};
// #endregion 📱️LayoutMobilePanel

/**
 * 📐️ Props interface for the top-level Layout component.
 **/
export interface LayoutProps {
  navbar?: React.ReactNode;
  /** 🎥️ Optional chrome row directly under `navbar`, above the canvas/panels row (e.g. {@link TutorialBar}) — `flex-shrink-0` like `navbar`/`footer`, never affecting the middle column's z-index invariant below. */
  subnavbar?: React.ReactNode;
  footer?: React.ReactNode;
  /** 🧱️ Optional chrome row directly under `footer` — the last row of the shell, in flow (`flex-shrink-0`), so what it holds
   * (the shell's status bands) reserves its own space and lies over nothing and under nothing. It cannot sit between the
   * canvas/panels row and the footer: an open chrome-hosted bottom panel pulls its cap half a row below that region, into
   * the footer band (`chromeHostedOpenPanelPositionStyle`), and would paint over a row placed there. */
  subfooter?: React.ReactNode;
  /** 🧭️ Per-anchor panel config — panels float over the navbar/footer/canvas, keyed by which anchor they grow from. */
  panels?: Partial<Record<Anchor, Omit<PanelProps, "anchor">>>;
  mobilePanel?: LayoutMobilePanelProps;
  canvas: React.ReactNode;
  /** 🦶️ Glass chips overlaid inside the canvas rectangle on its bottom edge (start / center / end) — they replace tab rows a footer band would hold. */
  canvasChips?: CanvasBottomChips;
  /** 📜️ Scrolls page content beneath the navbar and footer while keeping its first and last items clear of the bars. */
  scrollCanvas?: boolean;
  /** 🌀️ When set, paints a loading/waiting ring on the canvas viewport wrapper. */
  canvasStatus?: UiStatus;
  /** 🦴 Optional skeleton shown inside the canvas ring while `canvasStatus` is busy. */
  canvasSkeleton?: React.ReactNode;
  mobile?: boolean;
  className?: string;
}

/** 📏️ Keeps browser focus and anchor scrolling clear of the full sticky band. */
function bindScrollBand(band: HTMLDivElement | null) {
  const scrollport = band?.parentElement;
  if (!band || !scrollport) return;
  const property = band.dataset.slot === "layout-scroll-start" ? "scroll-padding-block-start" : "scroll-padding-block-end";
  const setPadding = (height: number) => scrollport.style.setProperty(property, `${height}px`);
  setPadding(band.offsetHeight);
  const observer = typeof ResizeObserver === "function" ? new ResizeObserver(([entry]) => setPadding(entry?.borderBoxSize?.[0]?.blockSize ?? band.offsetHeight)) : undefined;
  observer?.observe(band);
  return () => {
    observer?.disconnect();
    scrollport.style.removeProperty(property);
  };
}

/** 📐️ Publishes the heights of the top and bottom band rows as the clearance custom properties of the layout root, so every scroller, dock, panel and overlay under it can keep clear of the glass bands while the canvas runs beneath them, and each row as an occupied chrome box, so anchored window panes step clear of it like of any chrome panel.
 *
 * @see https://developer.mozilla.org/docs/Web/API/ResizeObserver */
function useBandClearance(rootRef: React.RefObject<HTMLDivElement | null>, enabled: boolean, bands: string) {
  const shellRoot = useShellScopeOptional()?.rootRef.current ?? undefined;
  const key = `layout-band:${React.useId()}`;
  React.useLayoutEffect(() => {
    const root = rootRef.current;
    if (!root || !enabled) return;
    const bands = (edge: string) => [...root.querySelectorAll<HTMLElement>(`:scope > [data-layout-band="${edge}"]`)];
    const boxes = [...bands("start").map((band) => ["top-middle", band] as const), ...bands("end").map((band) => ["bottom-middle", band] as const)];
    const publish = () => {
      boxes.forEach(([anchor, band], index) => publishShellChromePanelBox(shellRoot, `${key}:${index}`, anchor, safeAreaBoxFromRect(band.getBoundingClientRect())));
      root.style.setProperty("--layout-clearance-top", `${bands("start").reduce((sum, band) => sum + band.offsetHeight, 0)}px`);
      root.style.setProperty("--layout-clearance-bottom", `${bands("end").reduce((sum, band) => sum + band.offsetHeight, 0)}px`);
      root.style.setProperty("--layout-subfooter-height", `${root.querySelector<HTMLElement>(':scope > [data-slot="layout-subfooter"]')?.offsetHeight ?? 0}px`);
    };
    publish();
    const observer = typeof ResizeObserver === "function" ? new ResizeObserver(publish) : undefined;
    for (const band of [...bands("start"), ...bands("end"), ...root.querySelectorAll<HTMLElement>(':scope > [data-slot="layout-subfooter"]')]) observer?.observe(band);
    window.addEventListener("resize", publish);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", publish);
      boxes.forEach(([anchor], index) => publishShellChromePanelBox(shellRoot, `${key}:${index}`, anchor, null));
      root.style.removeProperty("--layout-clearance-top");
      root.style.removeProperty("--layout-clearance-bottom");
      root.style.removeProperty("--layout-subfooter-height");
    };
  }, [rootRef, enabled, bands, shellRoot, key]);
}

const bleedStyle: React.CSSProperties = { insetInline: 0, top: 0, bottom: "var(--layout-subfooter-height, 0px)" };

/** 📏️ The canvas is a full-bleed layer without a height of its own, so a layout in a host of automatic height keeps this floor between its bands instead of collapsing onto them. */
const LAYOUT_MIN_HEIGHT = "24rem";
const layoutFloorStyle: React.CSSProperties = { minBlockSize: LAYOUT_MIN_HEIGHT };

const Layout: React.FC<LayoutProps> = ({ navbar, subnavbar, footer, subfooter, panels, mobilePanel, canvas, canvasChips, scrollCanvas = false, canvasStatus, canvasSkeleton, mobile = false, className = "" }) => {
  const rootRef = React.useRef<HTMLDivElement>(null);
  useBandClearance(rootRef, !scrollCanvas, `${Boolean(navbar)}${Boolean(subnavbar)}${Boolean(footer)}${Boolean(subfooter)}`);
  const canvasBody = canvasStatus === "loading" || canvasStatus === "waiting" ? (canvasSkeleton ?? <CanvasSkeleton />) : canvas;
  const canvasClassName = cn("flex-1 min-w-0 min-h-0 relative", scrollCanvas && "isolate", chromeStatusBorderClass(canvasStatus));
  const bleedClassName = cn("absolute z-0 isolate min-w-0", chromeStatusBorderClass(canvasStatus));
  const middle = mobile ? (
    <div className="relative flex flex-col flex-1 min-h-0">
      {mobilePanel?.visible && <LayoutMobilePanel {...mobilePanel} />}
      {scrollCanvas ? <div className={cn(canvasClassName, mobilePanel?.visible && "hidden")}>{canvasBody}</div> : null}
    </div>
  ) : (
    <div className="flex flex-1 min-h-0 relative">
      <div data-slot="layout-canvas-column" className="flex flex-col flex-1 min-w-0 relative">
        <div className="flex flex-1 min-h-0 relative">
          {scrollCanvas ? <div className={canvasClassName}>{canvasBody}<CanvasBottomChipRow chips={canvasChips} /></div> : <CanvasBottomChipRow chips={canvasChips} />}
        </div>
      </div>
      {ANCHORS.map((anchor) => {
        const panelProps = panels?.[anchor];
        return panelProps ? <Panel key={anchor} {...panelProps} anchor={anchor} /> : null;
      })}
    </div>
  );
  return (
    <UiMobileProvider mobile={mobile}>
      <GhostProvider>
        <Surface ref={rootRef} data-slot="layout" style={layoutFloorStyle} className={cn("relative isolate flex h-full w-full flex-col overflow-hidden", className)}>
          {scrollCanvas ? (
            <div data-slot="layout-scrollport" tabIndex={0}>
              {navbar || subnavbar ? (
                <div ref={bindScrollBand} data-slot="layout-scroll-start">
                  {navbar}
                  {subnavbar}
                </div>
              ) : null}
              {middle}
              {footer && <div ref={bindScrollBand} data-slot="layout-scroll-end">{footer}</div>}
            </div>
          ) : (
            <>
              {navbar && <div data-layout-band="start" className="relative z-[1] flex-shrink-0">{navbar}</div>}
              {subnavbar && <div data-layout-band="start" className="relative z-[1] flex-shrink-0">{subnavbar}</div>}
              {middle}
              <div data-slot="layout-canvas-bleed" className={cn(bleedClassName, mobile && mobilePanel?.visible && "hidden")} style={bleedStyle}>{canvasBody}</div>
              {footer && <div data-layout-band="end" className="relative z-[1] flex-shrink-0">{footer}</div>}
            </>
          )}
          {subfooter && (
            <div data-slot="layout-subfooter" className="flex-shrink-0">
              {subfooter}
            </div>
          )}
        </Surface>
      </GhostProvider>
    </UiMobileProvider>
  );
};

export { Layout };

// #endregion 🪨️Layout

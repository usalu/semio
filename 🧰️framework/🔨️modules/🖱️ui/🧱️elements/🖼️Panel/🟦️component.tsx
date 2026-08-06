// #region 🧲️Header
// 💻️ framework/ui/elements/🖼️Panel/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { panelTabFirstDraggableElementId } from "@semio-tech/framework-core";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.useRef/.useMemo at module top level, which requires a non-circular import (see
// 🧱️elements/🫀️core/🔌Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🫀️core/🔌Ports/🟦️component.tsx";
import { Scrollable } from "../📜Scrollable/🟦️component.tsx";
import { Tree, type TreeDataItem, type TreeDataSection, type TreeDragAndDropController, type TreeSelectionMode } from "../🪵Tree/🟦️component.tsx";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { useFirstDraggableElementAlias } from "../🫀️core/🆔ElementId/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { borderNormalClass, dropZoneReadyFillClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { type Anchor, CloseIcon, FlowProvider, Icon, LevelProvider, PANEL_TREE_UNIT_MIME, PanelGhostRoot, PanelTabBar, type PanelTabNode, type PanelTreeUnit, WindowChrome, anchorHorizontal, anchorPositionStyle, beginPanelTreeUnitDrag, chromeHostedOpenPanelPositionStyle, endPanelTreeUnitDrag, findPanelTabNode, flowFromAnchor, getLevelZClass, progressPanelTabSelection, readActivePanelTreeUnitDrag, shellNavbarTrailingEndReserveStyle, useFlow, useLabel, useNativeDragArm, usePanelDockContext, usePanelTabSelection, usePanelTreeUnitDragActive, useShellNavbarTrailingEndWidthPx, useShellScopeOptional, useSurfaceActive, useUiDriverDragSurface, type UiStatus } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { DragHandle } from "../🧱DragHandle/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🧭️Panel
// Collapsible panel growing from one of the display's eight anchors (four corners plus four edge middles), with tabbed content.
// Consumers MUST provide PanelTabNode entries (see {@link PanelTabNode} in #region 🩻️Ribbon Components).

export interface TreePanelConfig {
  sections: TreeDataSection[];
  dragAndDropController?: TreeDragAndDropController;
  selectionMode?: TreeSelectionMode;
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  onSelectionChange?: (selectedIds: string[], items: TreeDataItem[]) => void;
  highlightedIds?: readonly string[];
  emptyState?: React.ReactNode;
  indentMultiplier?: number;
  className?: string;
  /** @emoji ↕️ Override section-reorder grips; defaults to enabled when {@link sections} has more than one entry. */
  sortableSections?: boolean;
  onSectionsReorder?: (orderedIds: readonly string[]) => void;
}

export interface TreePanelDefinition {
  resolveTree(): TreePanelConfig;
}

export type TreePanelSource = TreePanelConfig | TreePanelDefinition;

/** @emoji 🌲️ Factory for a static {@link TreePanelDefinition}. */
export function staticTreePanelDefinition(config: TreePanelConfig): TreePanelDefinition {
  return { resolveTree: () => config };
}

function resolveTreePanelSource(tree: TreePanelSource): TreePanelConfig {
  if ("resolveTree" in tree) {
    return tree.resolveTree();
  }
  return tree;
}

/** @emoji 🖱️ Pointer-drag props for a host element (replaces imperative drag controllers). */
export function usePointerDrag<TElement extends HTMLElement = HTMLDivElement>(handlers: {
  onStart?: (event: React.PointerEvent<TElement>) => void;
  onMove?: (event: React.PointerEvent<TElement>) => void;
  onEnd?: (event: React.PointerEvent<TElement>) => void;
  onCancel?: (event: React.PointerEvent<TElement>) => void;
}): Pick<React.HTMLAttributes<TElement>, "onPointerCancel" | "onPointerDown" | "onPointerMove" | "onPointerUp"> {
  const activePointerIdRef = reactHostPort.useRef<number | null>(null);
  return reactHostPort.useMemo(
    () => ({
      onPointerDown: (event: React.PointerEvent<TElement>) => {
        activePointerIdRef.current = event.pointerId;
        event.currentTarget.setPointerCapture(event.pointerId);
        handlers.onStart?.(event);
      },
      onPointerMove: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        handlers.onMove?.(event);
      },
      onPointerUp: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        activePointerIdRef.current = null;
        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
          event.currentTarget.releasePointerCapture(event.pointerId);
        }
        handlers.onEnd?.(event);
      },
      onPointerCancel: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        activePointerIdRef.current = null;
        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
          event.currentTarget.releasePointerCapture(event.pointerId);
        }
        handlers.onCancel?.(event);
      },
    }),
    [handlers],
  );
}

/** @emoji 📦️ Native HTML drag-and-drop event props for a host element. */
/** @emoji 🌲 Panel tree-unit dock header — handle-only under the default driver, whole-header under surface drag. */
function PanelTreeUnitHeader({
  anchor,
  tabId,
  unit,
  index,
  unitDragActive,
}: {
  readonly anchor?: Anchor;
  readonly tabId: string;
  readonly unit: PanelTreeUnit;
  readonly index: number;
  readonly unitDragActive: boolean;
}) {
  const dock = usePanelDockContext();
  const surfaceDrag = useUiDriverDragSurface();
  const { armed, arm } = useNativeDragArm();
  const unitDockDraggable = Boolean(dock && anchor);
  const effectiveDraggable = unitDockDraggable && (surfaceDrag || armed);
  const UnitIcon = unit.icon;
  return (
    <div
      data-slot="panel-tree-unit-header"
      draggable={effectiveDraggable}
      onDragStart={
        unitDockDraggable
          ? (event) => {
              event.dataTransfer.effectAllowed = "move";
              event.dataTransfer.setData(PANEL_TREE_UNIT_MIME, unit.id);
              beginPanelTreeUnitDrag({ tabId, unitId: unit.id, label: unit.label ?? tabId });
            }
          : undefined
      }
      onDragEnd={unitDockDraggable ? () => endPanelTreeUnitDrag() : undefined}
      onDragOver={
        unitDockDraggable
          ? (event) => {
              if (event.dataTransfer.types.includes(PANEL_TREE_UNIT_MIME)) event.preventDefault();
            }
          : undefined
      }
      onDrop={
        unitDockDraggable
          ? (event) => {
              if (!event.dataTransfer.types.includes(PANEL_TREE_UNIT_MIME) || !dock || !anchor) return;
              event.preventDefault();
              const session = readActivePanelTreeUnitDrag();
              if (!session) return;
              dock.onTreeUnitDockDrop({ unitId: session.unitId, fromTabId: session.tabId, target: { anchor, tabId, index } });
              endPanelTreeUnitDrag();
            }
          : undefined
      }
      className={cn(
        "flex shrink-0 items-center gap-single px-single py-half text-2xs",
        unitDragActive ? "text-emphasized" : "text-muted-foreground",
        unitDockDraggable && surfaceDrag && "cursor-grab active:cursor-grabbing",
        unitDragActive && dropZoneReadyFillClass,
      )}
    >
      {UnitIcon ? <UnitIcon size={12} /> : null}
      <span className="min-w-0 truncate">{unit.label}</span>
      {unitDockDraggable && !surfaceDrag ? (
        <DragHandle labelId="ui.tree.drag.sort" className="ms-auto" onPointerDown={arm} emphasized={unitDragActive} />
      ) : null}
    </div>
  );
}

export function useNativeDragAndDrop<TElement extends HTMLElement = HTMLDivElement>(
  handlers: {
    onDragStart?: React.DragEventHandler<TElement>;
    onDragEnd?: React.DragEventHandler<TElement>;
    onDragOver?: React.DragEventHandler<TElement>;
    onDrop?: React.DragEventHandler<TElement>;
  },
  draggable = true,
): Pick<React.HTMLAttributes<TElement>, "draggable" | "onDragEnd" | "onDragOver" | "onDragStart" | "onDrop"> {
  return reactHostPort.useMemo(
    () => ({
      draggable,
      onDragStart: handlers.onDragStart,
      onDragEnd: handlers.onDragEnd,
      onDragOver: handlers.onDragOver,
      onDrop: handlers.onDrop,
    }),
    [draggable, handlers.onDragEnd, handlers.onDragOver, handlers.onDragStart, handlers.onDrop],
  );
}

/**
 * Props interface for the Panel component.
 **/
export interface PanelProps {
  anchor: Anchor;
  visible?: boolean;
  /** @emoji 🎛️ Fired when the panel's own tab button group opens or folds it (see {@link Panel}). */
  onVisibleChange?: (visible: boolean) => void;
  size?: number;
  onSizeChange?: (size: number) => void;
  tabs: readonly PanelTabNode[];
  activeTabPath?: readonly string[];
  onActiveTabPathChange?: (path: readonly string[]) => void;
  /** @emoji 🌱️ Per-branch drill-down memory (see {@link progressPanelTabSelection}) — which child was last active under each branch, so returning to it restores the drill-down. */
  pathMemory?: Readonly<Record<string, string>>;
  onPathMemoryChange?: (memory: Readonly<Record<string, string>>) => void;
  /** @emoji 🌱️ Persisted tree section/group expansion across every leaf tab's units (see {@link PanelTreeUnitsPane}). */
  treeOpenStates?: Readonly<Record<string, boolean>>;
  onTreeOpenStateChange?: (id: string, open: boolean) => void;
  /**
   * ♻️ Busts {@link PanelTreeUnitsPane}'s memo when lazy `resolveTree` sources read host refs (tool measures,
   * active tool, staged command args). Without this, slider/option payloads refresh in shell state but the
   * memoized pane never re-calls `resolveTree`, so sibling distribution sliders stay visually stuck.
   */
  treeContentRevision?: unknown;
  minSize?: number;
  maxSize?: number;
  zIndex?: 10 | 20 | 30 | 40;
  className?: string;
  /** @emoji 🌀️ Drives the panel chrome silhouette while a tab body is still loading. */
  status?: UiStatus;
  /**
   * @emoji 🎛️ Where this anchor's folded root tab row lives — `"panel"` (default) keeps the chip-only
   * folded bar on the floating panel; `"chrome"` parks the folded root row in a sibling
   * {@link PanelChromeTabBar} in the navbar/footer. While open, the floating panel hosts the full tab
   * strip as {@link WindowChrome} left chips (same row as fold, U-cutout between) and is positioned via
   * {@link chromeHostedOpenPanelPositionStyle} so that cap stays in the shell chrome band (unfold in
   * place). Pair the two by passing the SAME controlled `visible`/`activeTabPath`/`pathMemory` (+ their
   * `on*Change`) to both.
   */
  tabBarHost?: "panel" | "chrome";
}

/** @emoji 🌲️ Leaf-tab tree body shared by {@link Panel} and {@link MobilePanel} — one section per unit (sorted by order); skipped when the active tab has no units. Under a {@link PanelDockProvider}, labeled (or multi-unit) headers become native-DnD handles draggable to another leaf tab's unit list (see {@link PANEL_TREE_UNIT_MIME}). Unlabeled single-unit tabs omit the unit header so trees are not topped by a lonely grip. */
export const PanelTreeUnitsPane = reactHostPort.memo(function PanelTreeUnitsPane({
  anchor,
  tabId,
  units,
  treeOpenStates,
  onTreeOpenStateChange,
  treeContentRevision: _treeContentRevision,
}: {
  readonly anchor?: Anchor;
  readonly tabId: string;
  readonly units: readonly PanelTreeUnit[];
  /** @emoji 🌱️ Persisted tree expansion, namespaced `${unitId}:${innerId}` across every unit this pane hosts. */
  readonly treeOpenStates?: Readonly<Record<string, boolean>>;
  readonly onTreeOpenStateChange?: (id: string, open: boolean) => void;
  /** @emoji ♻️ Identity-only prop — included so memo re-renders when lazy tree sources must re-resolve. */
  readonly treeContentRevision?: unknown;
}) {
  const flow = useFlow();
  const sortedUnits = [...units].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
  const unitDragActive = usePanelTreeUnitDragActive();
  return (
    <>
      {sortedUnits.map((unit, index) => {
        const config = resolveTreePanelSource(unit.tree);
        const unitPrefix = `${unit.id}:`;
        const unitOpenStates = treeOpenStates
          ? Object.fromEntries(
              Object.entries(treeOpenStates)
                .filter(([key]) => key.startsWith(unitPrefix))
                .map(([key, value]) => [key.slice(unitPrefix.length), value]),
            )
          : undefined;
        const onUnitOpenStateChange = onTreeOpenStateChange ? (id: string, open: boolean) => onTreeOpenStateChange(`${unitPrefix}${id}`, open) : undefined;
        const showUnitHeader = Boolean(unit.label || unit.icon) || sortedUnits.length > 1;
        return (
          <React.Fragment key={unit.id}>
            {showUnitHeader ? (
              <PanelTreeUnitHeader anchor={anchor} tabId={tabId} unit={unit} index={index} unitDragActive={unitDragActive} />
            ) : null}
            <Tree
              className={cn("min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden", config.className)}
              defaultSelectedIds={config.defaultSelectedIds}
              dragAndDropController={config.dragAndDropController}
              emptyState={config.emptyState}
              highlightedIds={config.highlightedIds}
              indentMultiplier={config.indentMultiplier}
              onSelectionChange={config.onSelectionChange}
              onSectionsReorder={config.onSectionsReorder}
              sections={config.sections}
              selectedIds={config.selectedIds}
              selectionMode={config.selectionMode}
              sortableSections={config.sortableSections ?? config.sections.length > 1}
              direction={flow.block}
              openStates={unitOpenStates}
              onOpenStateChange={onUnitOpenStateChange}
            />
          </React.Fragment>
        );
      })}
    </>
  );
});

/**
 * 🎯️ Dashed drop-zone placeholder shown at an anchor with zero tabs while a dock drag is in flight — otherwise an
 * empty anchor renders nothing and could never be dropped onto. Registers under the same `` `${anchor}:` `` row key
 * as the anchor's root {@link PanelTabRow} — the two never coexist: the root row only mounts once `tabs.length > 0`,
 * and this zone only mounts while it's `0`.
 **/
export function PanelEmptyDockZone({ anchor }: { readonly anchor: Anchor }) {
  const dock = usePanelDockContext();
  const setRef = reactHostPort.useCallback(
    (element: HTMLDivElement | null) => {
      dock?.registerTabRowDropTarget(anchor, [], element);
    },
    [anchor, dock],
  );
  const dropTarget = dock?.dropTarget;
  const isDropAnchor = Boolean(dropTarget?.kind === "insert" && dropTarget.anchor === anchor);
  return (
    <div
      ref={setRef}
      data-slot="panel-empty-drop-zone"
      className={cn("flex h-medium min-w-[10rem] items-center justify-center rounded-sm border border-dashed", borderNormalClass, dropZoneReadyFillClass, isDropAnchor ? "border-accent text-accent" : "text-emphasized")}
    >
      <Icon icon="grip-vertical" size="small" />
    </div>
  );
}

/** @emoji ↔ Panel resize handle with ghost wiring (inside {@link PanelGhostRoot}) — a corner panel gets one inner (canvas-facing) handle; a middle panel gets one on each edge, `deltaFactor` encoding both which way growth goes and (for a centered middle panel, where the opposite edge moves too) the 2× multiplier. */
function PanelResizeHandle({
  side,
  deltaFactor,
  minSize,
  maxSize,
  onSizeChange,
  sizeRef,
  resizeHandleClass,
  resizingSide,
  setResizingSide,
  setHoveredSide,
}: {
  side: "left" | "right";
  deltaFactor: number;
  minSize: number;
  maxSize: number;
  onSizeChange?: (size: number) => void;
  sizeRef: React.RefObject<number>;
  resizeHandleClass: string;
  resizingSide: "left" | "right" | null;
  setResizingSide: (side: "left" | "right" | null) => void;
  setHoveredSide: (side: "left" | "right" | null) => void;
}) {
  const isResizing = resizingSide === side;
  const resizeStartRef = reactHostPort.useRef<{ pointerX: number; size: number } | null>(null);
  const resizePointerProps = usePointerDrag<HTMLDivElement>({
    onStart: (event) => {
      event.preventDefault();
      resizeStartRef.current = { pointerX: event.clientX, size: sizeRef.current };
      setResizingSide(side);
    },
    onMove: (event) => {
      const start = resizeStartRef.current;
      if (!start) return;
      const delta = event.clientX - start.pointerX;
      const nextSize = start.size + deltaFactor * delta;
      if (nextSize >= minSize && nextSize <= maxSize) {
        onSizeChange?.(nextSize);
      }
    },
    onEnd: () => {
      resizeStartRef.current = null;
      setResizingSide(null);
    },
    onCancel: () => {
      resizeStartRef.current = null;
      setResizingSide(null);
    },
  });
  return <div data-slot="panel-resize-handle" className={resizeHandleClass} onMouseEnter={() => setHoveredSide(side)} onMouseLeave={() => !isResizing && setHoveredSide(null)} {...resizePointerProps} />;
}

const Panel: React.FC<PanelProps> = ({
  anchor,
  visible = false,
  onVisibleChange,
  size = 300,
  onSizeChange,
  tabs,
  activeTabPath,
  onActiveTabPathChange,
  pathMemory,
  onPathMemoryChange,
  treeOpenStates,
  onTreeOpenStateChange,
  treeContentRevision,
  minSize = 200,
  maxSize = 600,
  zIndex,
  className = "",
  tabBarHost = "panel",
  status,
}) => {
  const dock = usePanelDockContext();
  const panelShellScope = useShellScopeOptional();
  const trailingEndWidthPx = useShellNavbarTrailingEndWidthPx(panelShellScope?.rootRef.current ?? undefined);
  const collapseLabel = useLabel("ui.common.collapse");
  const panelRootRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [surfaceActive, surfaceActiveProps] = useSurfaceActive(panelRootRef);
  const [hoveredSide, setHoveredSide] = reactHostPort.useState<"left" | "right" | null>(null);
  const [resizingSide, setResizingSide] = reactHostPort.useState<"left" | "right" | null>(null);
  const sizeRef = reactHostPort.useRef(size);
  const panelContentRef = reactHostPort.useRef<HTMLDivElement>(null);

  reactHostPort.useEffect(() => {
    sizeRef.current = size;
  }, [size]);

  const flow = flowFromAnchor(anchor);
  const horizontal = anchorHorizontal(anchor);
  const isBottom = flow.block === "up";
  const isChromeHosted = tabBarHost === "chrome";
  const { resolvedPath, handlePathChange } = usePanelTabSelection({ tabs, visible, onVisibleChange, activeTabPath, onActiveTabPathChange, pathMemory, onPathMemoryChange });
  const activeNode = reactHostPort.useMemo(() => findPanelTabNode(tabs, resolvedPath), [tabs, resolvedPath]);
  const activeTabTrees = activeNode?.kind === "leaf" ? activeNode.trees : null;
  const firstDraggableAlias = visible && activeNode ? panelTabFirstDraggableElementId(activeNode.id) : null;
  useFirstDraggableElementAlias(panelContentRef, firstDraggableAlias);

  // Positioned within the region between navbar and footer (Layout's middle flex row), not the whole display — spacing is relative to that region's edges only, like a window's options rail over its canvas.
  // Height hugs content up to that same region bound (`maxHeight`, not a fixed `bottom`) — taller content scrolls internally instead of forcing the box to fill the region. A corner or edge-middle panel grows in one horizontal direction and is resizable only on its inner (canvas-facing) edge; a top/bottom-middle panel is centered and grows both ways, resizable from either edge.
  const positionStyle = {
    ...(isChromeHosted && visible ? chromeHostedOpenPanelPositionStyle(anchor) : anchorPositionStyle(anchor)),
    width: visible ? `${size}px` : undefined,
    ...(zIndex !== undefined ? { zIndex } : {}),
  };
  const panelZClass = getLevelZClass("panel");
  const panelFoldControl =
    visible && onVisibleChange
      ? {
          id: `framework.panel.${anchor}.fold`,
          slot: "panel-fold",
          icon: <CloseIcon className="size-small" />,
          label: collapseLabel,
          onClick: () => onVisibleChange(false),
        }
      : undefined;
  const chromeHostedTrailingEndReserveStyle =
    isChromeHosted && visible && horizontal === "right" ? shellNavbarTrailingEndReserveStyle(trailingEndWidthPx) : undefined;

  // 🎯️ An anchor with no tabs renders nothing at rest. Panel-hosted: it becomes a drop target only while a dock
  // drag is in flight, so a tab can be dragged into an otherwise-empty anchor. Chrome-hosted: the sibling
  // {@link PanelChromeTabBar} owns that empty-drop-zone role instead (registering under the same `${anchor}:`
  // row key), so this always renders nothing when empty.
  if (tabs.length === 0) {
    if (isChromeHosted || !dock?.dragTabId) return null;
    return (
      <LevelProvider level="panel">
        <PanelGhostRoot
          ref={panelRootRef}
          {...surfaceActiveProps}
          data-slot="panel"
          data-anchor={anchor}
          data-panel-visible="false"
          data-panel-empty="true"
          className={cn("absolute min-w-0 overflow-visible flex box-border text-foreground w-fit", panelZClass, isBottom ? "flex-col-reverse" : "flex-col", className)}
          style={{ ...positionStyle, width: undefined }}
        >
          <PanelEmptyDockZone anchor={anchor} />
        </PanelGhostRoot>
      </LevelProvider>
    );
  }

  // 🎛️ Chrome-hosted: the sibling {@link PanelChromeTabBar} is this anchor's folded representation — no floating button group of our own.
  if (isChromeHosted && !visible) return null;

  const resizeSides: readonly ("left" | "right")[] = horizontal === "middle" ? ["left", "right"] : [horizontal === "left" ? "right" : "left"];

  return (
    <LevelProvider level="panel">
      <PanelGhostRoot
        ref={panelRootRef}
        {...surfaceActiveProps}
        data-slot="panel"
        data-anchor={anchor}
        data-panel-visible={visible ? "true" : "false"}
        data-panel-chrome-hosted={isChromeHosted ? "true" : undefined}
        data-active-tab-id={activeNode?.id}
        id={visible && activeNode ? `framework.panelTab.${activeNode.id}` : undefined}
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        className={cn("absolute min-w-0 overflow-visible flex box-border text-foreground", panelZClass, isBottom ? "flex-col-reverse" : "flex-col", !visible && "w-fit", className)}
        style={positionStyle}
      >
        <FlowProvider inline={flow.inline} block={flow.block}>
          {visible ? (
            <>
              <WindowChrome
                stackSlot="window-chrome-stack"
                active={surfaceActive}
                level="panel"
                borderKind={status === "loading" ? "loading" : status === "waiting" ? "waiting" : undefined}
                capDock={isBottom ? "bottom" : "top"}
                capRowStyle={chromeHostedTrailingEndReserveStyle}
                stackClassName="w-full flex-1 min-h-0 bg-transparent"
                bodyClassName="flex min-h-0 flex-1 flex-col"
                bodySlot="panel-content"
                bodyRef={panelContentRef}
                close={panelFoldControl}
                titleChips={<PanelTabBar anchor={anchor} activePath={resolvedPath} onActivePathChange={handlePathChange} tabs={tabs} variant="panel" direction={flow.block} maxRows={1} showActiveColor={visible} />}
                body={
                  <div data-slot="panel-body-stack" className={cn("flex min-h-0 min-w-0 w-full flex-1", isBottom ? "flex-col-reverse" : "flex-col")}>
                    <PanelTabBar anchor={anchor} activePath={resolvedPath} onActivePathChange={handlePathChange} tabs={tabs} variant="panel" direction={flow.block} startDepth={1} showActiveColor={visible} />
                    <Scrollable className="relative flex-1 min-h-0">
                      {activeTabTrees && activeNode ? (
                        <PanelTreeUnitsPane anchor={anchor} tabId={activeNode.id} units={activeTabTrees} treeOpenStates={treeOpenStates} onTreeOpenStateChange={onTreeOpenStateChange} treeContentRevision={treeContentRevision} />
                      ) : null}
                    </Scrollable>
                  </div>
                }
              />
              {onSizeChange
                ? resizeSides.map((side) => (
                    <PanelResizeHandle
                      key={side}
                      side={side}
                      deltaFactor={(side === "right" ? 1 : -1) * (horizontal === "middle" ? 2 : 1)}
                      maxSize={maxSize}
                      minSize={minSize}
                      onSizeChange={onSizeChange}
                      resizeHandleClass={`absolute top-0 bottom-0 z-20 ${side === "left" ? "left-0" : "right-0"} w-single cursor-ew-resize`}
                      resizingSide={resizingSide}
                      setHoveredSide={setHoveredSide}
                      setResizingSide={setResizingSide}
                      sizeRef={sizeRef}
                    />
                  ))
                : null}
            </>
          ) : (
            <WindowChrome
              chipOnly
              level="panel"
              capDock={isBottom ? "bottom" : "top"}
              stackSlot="window-chrome-stack"
              titleChips={<PanelTabBar anchor={anchor} activePath={resolvedPath} onActivePathChange={handlePathChange} tabs={tabs} variant="panel" direction={flow.block} maxRows={1} showActiveColor={visible} />}
            />
          )}
        </FlowProvider>
      </PanelGhostRoot>
    </LevelProvider>
  );
};
export { Panel };

// #endregion 🧭️Panel

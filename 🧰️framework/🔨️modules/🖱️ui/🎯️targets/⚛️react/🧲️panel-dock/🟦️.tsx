import { type PanelTabNode, type PanelDock, findPanelTabPath, findPanelTabNode, type PanelTreeUnit } from "@semio-tech/ui/elements/PanelTabBar";
import { type Anchor, ANCHORS } from "@semio-tech/ui/modules/panel-anchor";
import { type PanelTabDockTarget, type PanelTreeUnitDockMove, type PanelDockContextValue, PanelDockContext } from "@semio-tech/ui/modules/panel-dock";
import * as React from "react";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { surfaceClass } from "@semio-tech/ui/modules/surface-presentation";
import { borderNormalClass } from "@semio-tech/ui/modules/border-presentation";
import { usePanelGhost } from "@semio-tech/ui/elements/PanelGhost";
import { reactHostPort } from "@semio-tech/ui/elements/Ports/react";

/** 🌱️ One draggable tree granule inside a leaf tab — dockable between leaf tabs, rendered as its own collapsible section. */
// Composable drag-and-drop: tabs dock between all eight anchors (pointer-capture drag, mirrors 🧭️ModeDockDrag);
// tree units dock between leaf tabs (native HTML5 drag-and-drop, mirrors the window-template palette-drag session).

//#region 🔀️Transforms

/** 🔀️ True if `id` is `node.id` itself or belongs to one of its descendants. */
export function isPanelTabInSubtree(node: PanelTabNode, id: string): boolean {
  if (node.id === id) return true;
  return node.kind === "branch" && node.children.some((child) => isPanelTabInSubtree(child, id));
}

/** 🔀️ Locates a tab anywhere in `dock`, returning its home anchor and the node itself. */
export function findPanelTabInDock(dock: PanelDock, id: string): { readonly anchor: Anchor; readonly node: PanelTabNode } | null {
  for (const anchor of ANCHORS) {
    const path = findPanelTabPath(dock.anchors[anchor], id);
    if (!path) continue;
    const node = findPanelTabNode(dock.anchors[anchor], path);
    if (node) return { anchor, node };
  }
  return null;
}

function collectPanelTabSubtreeIds(node: PanelTabNode, into: Set<string>): void {
  into.add(node.id);
  if (node.kind === "branch") node.children.forEach((child) => collectPanelTabSubtreeIds(child, into));
}

/** 🔀️ Reassigns `order` to match array position — call after any transform that reorders a sibling array so the existing order-based sort renders the new arrangement. */
export function normalizePanelTabOrder(tabs: readonly PanelTabNode[]): readonly PanelTabNode[] {
  return tabs.map((tab, index) => (tab.order === index ? tab : { ...tab, order: index }));
}

/** 🔀️ Recursively drops branch tabs left with zero children. The complementary half of "deliberately emptied" — keeping such branches out of {@link applyDockSkeleton}'s auto-append — is handled there by its subtree-mention check, so an emptied branch never resurfaces with default children restored. */
export function pruneEmptyPanelBranches(tabs: readonly PanelTabNode[]): readonly PanelTabNode[] {
  const pruned = tabs.map((tab) => (tab.kind === "branch" ? { ...tab, children: pruneEmptyPanelBranches(tab.children) } : tab)).filter((tab) => tab.kind !== "branch" || tab.children.length > 0);
  return pruned.length === tabs.length && pruned.every((tab, index) => tab === tabs[index]) ? tabs : pruned;
}

/** 🔀️ Removes `id` wherever it lives in `tabs` (searching recursively into branches), pruning any ancestor branch left empty by the removal. Removal-first: callers compute insertion indices against this result, never the pre-removal tree. */
function removePanelTabFromSiblings(tabs: readonly PanelTabNode[], id: string): { readonly tabs: readonly PanelTabNode[]; readonly removed: PanelTabNode | null } {
  const directIndex = tabs.findIndex((tab) => tab.id === id);
  if (directIndex >= 0) {
    const removed = tabs[directIndex]!;
    return { tabs: normalizePanelTabOrder([...tabs.slice(0, directIndex), ...tabs.slice(directIndex + 1)]), removed };
  }
  for (let index = 0; index < tabs.length; index++) {
    const tab = tabs[index]!;
    if (tab.kind !== "branch") continue;
    const { tabs: nextChildren, removed } = removePanelTabFromSiblings(tab.children, id);
    if (!removed) continue;
    const next = nextChildren.length > 0 ? [...tabs.slice(0, index), { ...tab, children: nextChildren }, ...tabs.slice(index + 1)] : [...tabs.slice(0, index), ...tabs.slice(index + 1)];
    return { tabs: normalizePanelTabOrder(next), removed };
  }
  return { tabs, removed: null };
}

function insertPanelTabAtPath(tabs: readonly PanelTabNode[], parentPath: readonly string[], node: PanelTabNode, index: number): readonly PanelTabNode[] {
  if (parentPath.length === 0) {
    const clampedIndex = Math.max(0, Math.min(index, tabs.length));
    return normalizePanelTabOrder([...tabs.slice(0, clampedIndex), node, ...tabs.slice(clampedIndex)]);
  }
  const [headId, ...restPath] = parentPath;
  const headIndex = tabs.findIndex((tab) => tab.id === headId);
  if (headIndex < 0) return tabs;
  const head = tabs[headIndex]!;
  if (head.kind !== "branch") return tabs;
  const next = [...tabs];
  next[headIndex] = { ...head, children: insertPanelTabAtPath(head.children, restPath, node, index) };
  return next;
}

function appendPanelTabAsChild(tabs: readonly PanelTabNode[], parentId: string, node: PanelTabNode): readonly PanelTabNode[] {
  return tabs.map((tab) => {
    if (tab.id === parentId && tab.kind === "branch") return { ...tab, children: normalizePanelTabOrder([...tab.children, node]) };
    if (tab.kind === "branch") return { ...tab, children: appendPanelTabAsChild(tab.children, parentId, node) };
    return tab;
  });
}

/** 🎯️ A completed tab drag: move `tabId` (found via {@link findPanelTabInDock}, not `fromAnchor` alone) to `target`. */
export interface PanelTabDockMove {
  readonly tabId: string;
  readonly fromAnchor: Anchor;
  readonly target: PanelTabDockTarget;
}

/**
 * 🎯️ Pure move transform: removes the dragged tab's subtree from wherever it lives and reinserts it at `target`.
 * Dropping a subtree into itself (as a child of one of its own descendants, or among the children of one) is a
 * no-operation returning the exact same {@link PanelDock} reference. Visibility is never touched here — the shell unfolds
 * a folded target anchor on drop.
 **/
export function moveTabInDock(dock: PanelDock, move: PanelTabDockMove): PanelDock {
  const located = findPanelTabInDock(dock, move.tabId);
  if (!located) return dock;
  const { anchor: fromAnchor, node } = located;
  const { target } = move;
  if (target.kind === "child" && isPanelTabInSubtree(node, target.parentId)) return dock;
  if (target.kind === "insert" && target.parentPath.some((id) => isPanelTabInSubtree(node, id))) return dock;

  const { tabs: sourceTabs, removed } = removePanelTabFromSiblings(dock.anchors[fromAnchor], move.tabId);
  if (!removed) return dock;

  const anchors: Record<Anchor, readonly PanelTabNode[]> = { ...dock.anchors, [fromAnchor]: sourceTabs };
  anchors[target.anchor] = target.kind === "child" ? appendPanelTabAsChild(anchors[target.anchor], target.parentId, removed) : insertPanelTabAtPath(anchors[target.anchor], target.parentPath, removed, target.index);
  return { anchors };
}

function replacePanelTabNode(tabs: readonly PanelTabNode[], id: string, nextNode: PanelTabNode): readonly PanelTabNode[] {
  return tabs.map((tab) => {
    if (tab.id === id) return nextNode;
    if (tab.kind === "branch") return { ...tab, children: replacePanelTabNode(tab.children, id, nextNode) };
    return tab;
  });
}

function replacePanelTabInDock(dock: PanelDock, id: string, nextNode: PanelTabNode): PanelDock {
  const located = findPanelTabInDock(dock, id);
  if (!located) return dock;
  const anchors = { ...dock.anchors };
  anchors[located.anchor] = replacePanelTabNode(dock.anchors[located.anchor], id, nextNode);
  return { anchors };
}

/** 🎯️ Pure move transform for tree units — reorders within a leaf's own unit list, or moves a unit from one leaf tab's list into another's. */
export function moveTreeUnitInDock(dock: PanelDock, move: PanelTreeUnitDockMove): PanelDock {
  const from = findPanelTabInDock(dock, move.fromTabId);
  if (!from || from.node.kind !== "leaf") return dock;
  const fromNode = from.node;
  const unit = fromNode.trees.find((candidate) => candidate.id === move.unitId);
  if (!unit) return dock;

  if (move.fromTabId === move.target.tabId) {
    const withoutUnit = fromNode.trees.filter((candidate) => candidate.id !== move.unitId);
    const clampedIndex = Math.max(0, Math.min(move.target.index, withoutUnit.length));
    const nextUnits = normalizePanelTreeUnitOrder([...withoutUnit.slice(0, clampedIndex), unit, ...withoutUnit.slice(clampedIndex)]);
    if (nextUnits.length === fromNode.trees.length && nextUnits.every((candidate, index) => candidate === fromNode.trees[index])) return dock;
    return replacePanelTabInDock(dock, move.fromTabId, { ...fromNode, trees: nextUnits });
  }

  const to = findPanelTabInDock(dock, move.target.tabId);
  if (!to || to.node.kind !== "leaf") return dock;
  const toNode = to.node;
  const fromNextUnits = normalizePanelTreeUnitOrder(fromNode.trees.filter((candidate) => candidate.id !== move.unitId));
  const clampedIndex = Math.max(0, Math.min(move.target.index, toNode.trees.length));
  const toNextUnits = normalizePanelTreeUnitOrder([...toNode.trees.slice(0, clampedIndex), unit, ...toNode.trees.slice(clampedIndex)]);
  const afterSource = replacePanelTabInDock(dock, move.fromTabId, { ...fromNode, trees: fromNextUnits });
  return replacePanelTabInDock(afterSource, move.target.tabId, { ...toNode, trees: toNextUnits });
}

function normalizePanelTreeUnitOrder(units: readonly PanelTreeUnit[]): readonly PanelTreeUnit[] {
  return units.map((unit, index) => (unit.order === index ? unit : { ...unit, order: index }));
}

//#endregion 🔀️Transforms

//#region 🎯️HitTesting

/** 🎯️ A registered {@link PanelTabRow} drop surface — v1's drop surfaces are the base row of every anchor plus the rows along each anchor's current active path (branches not on the active path aren't visible, so aren't registered). */
export interface PanelTabRowDropTarget {
  readonly anchor: Anchor;
  readonly parentPath: readonly string[];
  readonly rowElement: HTMLElement;
}

function panelDockRowTabButtons(rowElement: HTMLElement, excludedIds: ReadonlySet<string>): HTMLElement[] {
  return [...rowElement.querySelectorAll<HTMLElement>("[data-tab-id]")].filter((element) => !excludedIds.has(element.dataset.tabId ?? ""));
}

/**
 * 🎯️ Resolves a tab-drag drop target from registered rows. A row hit lands on a branch button's 30–70% center
 * band as `"child"` (nest into it), elsewhere as a midpoint `"insert"`. {@link PanelTabRow} preserves declared
 * left-to-right order at every anchor, so physical and model insertion order always agree.
 **/
export function computeTabDockDropZone(pointerX: number, pointerY: number, rows: readonly PanelTabRowDropTarget[], excludedIds: ReadonlySet<string>): PanelTabDockTarget | null {
  for (const row of rows) {
    const rect = row.rowElement.getBoundingClientRect();
    if (pointerX < rect.left || pointerX > rect.right || pointerY < rect.top || pointerY > rect.bottom) continue;
    const buttons = panelDockRowTabButtons(row.rowElement, excludedIds);
    for (let index = 0; index < buttons.length; index++) {
      const button = buttons[index]!;
      const buttonRect = button.getBoundingClientRect();
      if (pointerX < buttonRect.left || pointerX > buttonRect.right) continue;
      const fraction = buttonRect.width > 0 ? (pointerX - buttonRect.left) / buttonRect.width : 0.5;
      if (button.dataset.tabKind === "branch" && fraction > 0.3 && fraction < 0.7) {
        return { kind: "child", anchor: row.anchor, parentId: button.dataset.tabId! };
      }
      return { kind: "insert", anchor: row.anchor, parentPath: row.parentPath, index: index + (fraction >= 0.5 ? 1 : 0) };
    }
    return { kind: "insert", anchor: row.anchor, parentPath: row.parentPath, index: buttons.length };
  }
  return null;
}

//#endregion 🎯️HitTesting

//#region 🪟️DragPreview

const DOCK_DRAG_CURSOR_OFFSET_X = 8;
const DOCK_DRAG_CURSOR_OFFSET_Y = 10;

/** 🪟️ Floating label following the cursor while dragging a panel tab or tree unit. */
const DockDragChip: React.FC<{ readonly label: string; readonly x: number; readonly y: number }> = ({ label, x, y }) => (
  <div
    data-slot="dock-drag-chip"
    data-level="window"
    className={cn("pointer-events-none fixed z-tutorial flex max-w-[12rem] shrink-0 items-center gap-half rounded-sm border px-single py-half text-xs text-element shadow-md select-none", surfaceClass, borderNormalClass)}
    style={{ left: x + DOCK_DRAG_CURSOR_OFFSET_X, top: y + DOCK_DRAG_CURSOR_OFFSET_Y }}
  >
    <span className="truncate">{label}</span>
  </div>
);

//#endregion 🪟️DragPreview

interface PanelDockDragState {
  readonly anchor: Anchor;
  readonly tabId: string;
  readonly pointerId: number;
  readonly label: string;
  readonly x: number;
  readonly y: number;
}

interface PanelDockPendingDrag {
  readonly anchor: Anchor;
  readonly tabId: string;
  readonly pointerId: number;
  readonly label: string;
  readonly startX: number;
  readonly startY: number;
}

/** 🎛️ Props for {@link PanelDockProvider}. */
export interface PanelDockProviderProps {
  readonly dock: PanelDock;
  readonly onTabDockDrop: (move: PanelTabDockMove) => void;
  readonly onTreeUnitDockDrop: (move: PanelTreeUnitDockMove) => void;
  readonly children: React.ReactNode;
}

/** 🎛️ Wraps a layout's panels, wiring pointer-capture tab dragging (mirrors {@link Mode}'s window-tab drag) across all of them. Tree-unit drags are native HTML5 DnD and don't need this provider — see {@link beginPanelTreeUnitDrag}. */
export const PanelDockProvider: React.FC<PanelDockProviderProps> = ({ dock, onTabDockDrop, onTreeUnitDockDrop, children }) => {
  const panelGhost = usePanelGhost();
  const [pendingDrag, setPendingDrag] = reactHostPort.useState<PanelDockPendingDrag | null>(null);
  const [dragState, setDragState] = reactHostPort.useState<PanelDockDragState | null>(null);
  const [dropTarget, setDropTarget] = reactHostPort.useState<PanelTabDockTarget | null>(null);
  const dropTargetRef = reactHostPort.useRef<PanelTabDockTarget | null>(null);
  const rowsRef = reactHostPort.useRef(new Map<string, PanelTabRowDropTarget>());
  const excludedIdsRef = reactHostPort.useRef<ReadonlySet<string>>(new Set());
  const dockRef = reactHostPort.useRef(dock);
  dockRef.current = dock;

  const registerTabRowDropTarget = reactHostPort.useCallback((anchor: Anchor, parentPath: readonly string[], element: HTMLElement | null) => {
    const key = `${anchor}:${parentPath.join("/")}`;
    if (!element) {
      rowsRef.current.delete(key);
      return;
    }
    rowsRef.current.set(key, { anchor, parentPath, rowElement: element });
  }, []);

  const refreshDropTarget = reactHostPort.useCallback((x: number, y: number) => {
    const zone = computeTabDockDropZone(x, y, [...rowsRef.current.values()], excludedIdsRef.current);
    dropTargetRef.current = zone;
    setDropTarget(zone);
  }, []);

  const startTabDrag = reactHostPort.useCallback((anchor: Anchor, tabId: string, label: string, event: React.PointerEvent<HTMLElement>) => {
    if (event.button !== 0) return;
    setPendingDrag({ anchor, tabId, pointerId: event.pointerId, label, startX: event.clientX, startY: event.clientY });
  }, []);

  reactHostPort.useEffect(() => {
    if (!pendingDrag && !dragState) return;
    const handleMove = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (pendingDrag && !dragState) {
        const distance = Math.hypot(event.clientX - pendingDrag.startX, event.clientY - pendingDrag.startY);
        if (distance < 6) return;
        const located = findPanelTabInDock(dockRef.current, pendingDrag.tabId);
        const subtreeIds = new Set<string>();
        if (located) collectPanelTabSubtreeIds(located.node, subtreeIds);
        excludedIdsRef.current = subtreeIds;
        panelGhost?.begin(null);
        setDragState({ anchor: pendingDrag.anchor, tabId: pendingDrag.tabId, pointerId: pendingDrag.pointerId, label: pendingDrag.label, x: event.clientX, y: event.clientY });
        setPendingDrag(null);
        refreshDropTarget(event.clientX, event.clientY);
        return;
      }
      if (!dragState) return;
      setDragState((prev) => (prev ? { ...prev, x: event.clientX, y: event.clientY } : prev));
      refreshDropTarget(event.clientX, event.clientY);
    };
    const handleUp = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (dragState && dropTargetRef.current) {
        onTabDockDrop({ tabId: dragState.tabId, fromAnchor: dragState.anchor, target: dropTargetRef.current });
      }
      panelGhost?.end();
      setDragState(null);
      setPendingDrag(null);
      dropTargetRef.current = null;
      setDropTarget(null);
    };
    document.addEventListener("pointermove", handleMove);
    document.addEventListener("pointerup", handleUp);
    return () => {
      document.removeEventListener("pointermove", handleMove);
      document.removeEventListener("pointerup", handleUp);
    };
  }, [pendingDrag, dragState, onTabDockDrop, panelGhost, refreshDropTarget]);

  reactHostPort.useEffect(() => {
    if (!dragState) return;
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      panelGhost?.end();
      setDragState(null);
      setPendingDrag(null);
      dropTargetRef.current = null;
      setDropTarget(null);
    };
    document.addEventListener("keydown", handleKeyDown, true);
    return () => document.removeEventListener("keydown", handleKeyDown, true);
  }, [dragState, panelGhost]);

  const contextValue = reactHostPort.useMemo<PanelDockContextValue>(
    () => ({ dragTabId: dragState?.tabId ?? null, draggedSubtreeIds: dragState ? excludedIdsRef.current : null, dropTarget, startTabDrag, registerTabRowDropTarget, onTreeUnitDockDrop }),
    [dragState, dropTarget, startTabDrag, registerTabRowDropTarget, onTreeUnitDockDrop],
  );

  return (
    <PanelDockContext.Provider value={contextValue}>
      {children}
      {dragState ? <DockDragChip label={dragState.label} x={dragState.x} y={dragState.y} /> : null}
    </PanelDockContext.Provider>
  );
};

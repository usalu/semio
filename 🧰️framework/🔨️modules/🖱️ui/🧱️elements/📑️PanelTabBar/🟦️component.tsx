// #region 🧲️Header
// 💻️ framework/ui/elements/📑️PanelTabBar/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { type IconName } from "@semio-tech/assets";
import { type DockSkeleton, type DockTabSkeleton } from "@semio-tech/framework";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { Icon, type ControlIcon, renderControlIcon } from "../🔣️Icons/🟦️component.tsx";
import { DragHandle } from "../🧱️DragHandle/🟦️component.tsx";
import { Ribbon, type RibbonRow } from "../🎀️Ribbon/🟦️component.tsx";
import { type FlowBlock, FlowProvider, useFlow } from "../🧭️Flow/🟦️component.tsx";
import { useLabel, useControlInlineText } from "../🏷️Label/🟦️component.tsx";
import { useLevel, useSurfaceActive } from "../🌈️Surface/🟦️component.tsx";
import { interactiveActiveFillClass } from "../🏷️ClassNames/🟦️component.tsx";
import { type Anchor, flowFromAnchor, ChromeControlHint, useNativeDragArm, usePanelDockContext, useUiDriverDragSurface, panelTabIconSlotClass, panelTabLabelClass, chromeControlTabItemClass, panelTabButtonClass, panelTabBarClass, panelTabButtonDividerClass, panelWindowInactiveTabClass, modeDockInactiveTabBeforeGapClass, modeDockInactiveTabClass, modeDockTabClassName, modeDockActiveTabClass, modeDockActiveTabFillClass, modeDockTabLabelClassName, dropZoneReadyClass, panelAnchorTabBarClass, mobilePanelTabBarClass, mobilePanelTabButtonClass, panelAnchorTabButtonClass, PANEL_TREE_UNIT_MIME, beginPanelTreeUnitDrag, endPanelTreeUnitDrag, readActivePanelTreeUnitDrag, usePanelTreeUnitDragActive, type PanelDockContextValue, type UiStatus, type TreePanelSource, ANCHORS } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 📑️PanelTabBar
export type PanelTabBarVariant = "panel" | "mobile" | "chrome";

/** @emoji 🧭️ Validates a path's segments against a node tree, truncating at the first segment that no longer exists at its level — no first-sibling substitution, no auto-descend (progressive reveal owns how deep a path goes). `[]` is a valid result. */
export function reconcileActivePath<T extends { readonly id: string }>(nodes: readonly T[], path: readonly string[], childrenOf: (node: T) => readonly T[] | undefined): string[] {
  let current = nodes;
  const reconciled: string[] = [];
  for (const id of path) {
    const node = current.find((candidate) => candidate.id === id);
    if (!node) break;
    reconciled.push(id);
    current = childrenOf(node) ?? [];
  }
  return reconciled;
}

/** @emoji 🧭️ Eight anchors a panel or pane can grow from: the display's four corners, plus the four edge middles (top/bottom/left/right) — no center anchor, since floating chrome must never fully occlude the canvas. */
export interface PanelTreeUnit {
  readonly id: string;
  readonly tree: TreePanelSource;
  readonly label?: UiLabel;
  readonly icon?: React.ComponentType<{ size?: number }>;
  readonly order?: number;
}

/** @emoji 🍃️ Leaf tab — its `trees` are the panel-body units shown when active, each rendered as its own section. */
export interface PanelTabLeaf {
  readonly kind: "leaf";
  readonly id: string;
  readonly icon: React.ComponentType<{ size?: number }>;
  /** @emoji 🏷️ Mandatory tab label shown after the icon. */
  readonly name: string;
  readonly order?: number;
  /** @emoji 🌲️ Tree units for this tab — draggable between leaf tabs, rendered as sections. */
  readonly trees: readonly PanelTreeUnit[];
}

/** @emoji 🍃️ Builds a {@link PanelTabLeaf} with exactly one tree, wrapped as a single {@link PanelTreeUnit} (unit id: `` `${id}.tree` ``). */
export function singleTreeLeaf(leaf: { readonly id: string; readonly icon: React.ComponentType<{ size?: number }>; readonly name: string; readonly order?: number; readonly tree: TreePanelSource }): PanelTabLeaf {
  return { kind: "leaf", id: leaf.id, icon: leaf.icon, name: leaf.name, order: leaf.order, trees: [{ id: `${leaf.id}.tree`, tree: leaf.tree }] };
}

/** @emoji 🌳️ Branch tab — its `children` render as the row below this one when active. */
export interface PanelTabBranch {
  readonly kind: "branch";
  readonly id: string;
  readonly icon: React.ComponentType<{ size?: number }>;
  readonly name: string;
  readonly order?: number;
  readonly children: readonly PanelTabNode[];
}

/** @emoji 🌲️ One node in the arbitrarily nestable panel tab tree. */
export type PanelTabNode = PanelTabLeaf | PanelTabBranch;

/** @emoji 🌳️ `childrenOf` for {@link reconcileActivePath} over a {@link PanelTabNode} tree. */
export function panelTabChildren(node: PanelTabNode): readonly PanelTabNode[] | undefined {
  return node.kind === "branch" ? node.children : undefined;
}

/** @emoji 🔍️ Walks a path from the root, returning the node at its end (or undefined if the path doesn't resolve). */
export function findPanelTabNode(tabs: readonly PanelTabNode[], path: readonly string[]): PanelTabNode | undefined {
  let nodes = tabs;
  let found: PanelTabNode | undefined;
  for (const id of path) {
    found = nodes.find((node) => node.id === id);
    if (!found) return undefined;
    nodes = panelTabChildren(found) ?? [];
  }
  return found;
}

/** @emoji 🔍️ Depth-first path from the root to the tab with `id`, or undefined if absent. */
export function findPanelTabPath(tabs: readonly PanelTabNode[], id: string): string[] | undefined {
  for (const node of tabs) {
    if (node.id === id) return [node.id];
    if (node.kind === "branch") {
      const childPath = findPanelTabPath(node.children, id);
      if (childPath) return [node.id, ...childPath];
    }
  }
  return undefined;
}

/** @emoji 🌱️ A node's own id plus every descendant's id — the memory entries a collapsed/reset branch must forget. */
function panelTabSubtreeIds(node: PanelTabNode): string[] {
  const children = panelTabChildren(node);
  return children ? [node.id, ...children.flatMap(panelTabSubtreeIds)] : [node.id];
}

/** @emoji 🌱️ Result of interpreting one raw tab press: the next active path, the next per-branch drill-down memory, and whether the press should fold the hosting panel instead. */
export interface PanelTabSelectionResult {
  readonly path: readonly string[];
  readonly memory: Readonly<Record<string, string>>;
  readonly fold: boolean;
}

/**
 * 🌱️ Interprets a raw tab press for progressive reveal (see {@link PanelTabBar}): re-pressing the already-active
 * segment collapses it; a root re-press always folds the panel without changing its path or remembered state; a fresh pick adopts
 * the pressed path and extends it from `memory` — each branch remembers the last child drilled into, pruning stale
 * entries that no longer match the tree — recording a fresh hop for every step of the resulting path.
 **/
export function progressPanelTabSelection(tabs: readonly PanelTabNode[], currentPath: readonly string[], selectedPath: readonly string[], memory: Readonly<Record<string, string>>): PanelTabSelectionResult {
  const validatedSelected = reconcileActivePath(tabs, selectedPath, panelTabChildren);
  const d = validatedSelected.length - 1;
  if (d < 0) return { path: currentPath, memory, fold: false };
  const pressed = validatedSelected[d];

  const clearSubtreeMemory = (node: PanelTabNode | undefined, fallbackId: string): Readonly<Record<string, string>> => {
    const clearedIds = new Set(node ? panelTabSubtreeIds(node) : [fallbackId]);
    return Object.fromEntries(Object.entries(memory).filter(([key]) => !clearedIds.has(key)));
  };

  if (currentPath[d] === pressed) {
    if (d === 0) {
      return { path: currentPath, memory, fold: true };
    }
    return { path: currentPath.slice(0, d), memory: clearSubtreeMemory(findPanelTabNode(tabs, validatedSelected), pressed), fold: false };
  }

  let path: readonly string[] = validatedSelected;
  let nextMemory: Record<string, string> = { ...memory };
  let tailNode = findPanelTabNode(tabs, path);
  while (tailNode && tailNode.kind === "branch") {
    const remembered = nextMemory[tailNode.id];
    if (!remembered) break;
    const child = tailNode.children.find((candidate) => candidate.id === remembered);
    if (!child) {
      const { [tailNode.id]: _stale, ...rest } = nextMemory;
      nextMemory = rest;
      break;
    }
    path = [...path, child.id];
    tailNode = child;
  }
  for (let i = 0; i < path.length - 1; i++) nextMemory[path[i]] = path[i + 1];
  return { path, memory: nextMemory, fold: false };
}

/** @emoji 🌱️ Controlled/uncontrolled selection state shared by every {@link PanelTabBar} host ({@link Panel}, {@link PanelChromeTabBar}, {@link MobilePanel}). */
export interface PanelTabSelectionOptions {
  readonly tabs: readonly PanelTabNode[];
  readonly visible: boolean;
  /** @emoji 🎛️ Fired when a tab press opens or folds the hosting surface (see {@link usePanelTabSelection}). */
  readonly onVisibleChange?: (visible: boolean) => void;
  readonly activeTabPath?: readonly string[];
  readonly onActiveTabPathChange?: (path: readonly string[]) => void;
  /** @emoji 🌱️ Per-branch drill-down memory (see {@link progressPanelTabSelection}) — which child was last active under each branch, so returning to it restores the drill-down. */
  readonly pathMemory?: Readonly<Record<string, string>>;
  readonly onPathMemoryChange?: (memory: Readonly<Record<string, string>>) => void;
}

/**
 * 🌱️ Shared fold/open/drill-down state machine for every {@link PanelTabBar} host: resolves the active path
 * against `tabs`, and interprets a raw tab press via {@link progressPanelTabSelection} — opening a closed host
 * on first press (swallowing that same press if it only re-selects the already-active leaf), folding it on an
 * active-root re-press, and otherwise advancing the path/memory (controlled when the matching `on*Change` is
 * given, else internal state). One instance of this state must back a single anchor's tabs, however many
 * hosts (panel chrome, navbar/footer chrome bar, mobile) render it — hosting is presentation-only.
 **/
export function usePanelTabSelection({ tabs, visible, onVisibleChange, activeTabPath, onActiveTabPathChange, pathMemory, onPathMemoryChange }: PanelTabSelectionOptions): {
  readonly resolvedPath: readonly string[];
  readonly memory: Readonly<Record<string, string>>;
  readonly handlePathChange: (raw: readonly string[]) => void;
} {
  const [internalActivePath, setInternalActivePath] = reactHostPort.useState<readonly string[]>(() => reconcileActivePath(tabs, [], panelTabChildren));
  const [internalMemory, setInternalMemory] = reactHostPort.useState<Readonly<Record<string, string>>>({});
  const memory = pathMemory ?? internalMemory;
  const resolvedPath = reactHostPort.useMemo(() => reconcileActivePath(tabs, activeTabPath ?? internalActivePath, panelTabChildren), [tabs, activeTabPath, internalActivePath]);

  const handlePathChange = (raw: readonly string[]) => {
    if (!visible) {
      onVisibleChange?.(true);
      if (raw[raw.length - 1] === resolvedPath[raw.length - 1]) return;
    }
    const result = progressPanelTabSelection(tabs, resolvedPath, raw, memory);
    if (visible && result.fold) {
      onVisibleChange?.(false);
      return;
    }
    if (onActiveTabPathChange) {
      onActiveTabPathChange(result.path);
    } else {
      setInternalActivePath(result.path);
    }
    if (onPathMemoryChange) {
      onPathMemoryChange(result.memory);
    } else {
      setInternalMemory(result.memory);
    }
  };

  return { resolvedPath, memory, handlePathChange };
}

/** @emoji 🗄️ Full arrangement of tabs across all eight anchors — the pure, draggable dock model. */
export interface PanelDock {
  readonly anchors: Record<Anchor, readonly PanelTabNode[]>;
}

function panelTabNodeToSkeleton(node: PanelTabNode): DockTabSkeleton {
  if (node.kind === "branch") return { id: node.id, children: node.children.map(panelTabNodeToSkeleton) };
  return node.trees.length > 0 ? { id: node.id, trees: node.trees.map((unit) => unit.id) } : { id: node.id };
}

/** @emoji 🗄️ Reduces a full {@link PanelDock} to the id-only {@link DockSkeleton} persisted across sessions. */
export function dockSkeletonOf(dock: PanelDock): DockSkeleton {
  const anchors = {} as Record<Anchor, readonly DockTabSkeleton[]>;
  for (const anchor of ANCHORS) anchors[anchor] = dock.anchors[anchor].map(panelTabNodeToSkeleton);
  return { version: 3, anchors };
}

function dockTabSkeletonsEqual(a: DockTabSkeleton, b: DockTabSkeleton): boolean {
  if (a.id !== b.id) return false;
  const aChildren = a.children ?? [];
  const bChildren = b.children ?? [];
  if (aChildren.length !== bChildren.length || !aChildren.every((child, index) => dockTabSkeletonsEqual(child, bChildren[index]!))) return false;
  const aTrees = a.trees ?? [];
  const bTrees = b.trees ?? [];
  return aTrees.length === bTrees.length && aTrees.every((id, index) => id === bTrees[index]);
}

/** @emoji 🗄️ Structural equality between two {@link DockSkeleton} values — used to decide whether an arrangement equals its computed default and therefore needs no persistence. */
export function dockSkeletonsEqual(a: DockSkeleton | null, b: DockSkeleton | null): boolean {
  if (a === b) return true;
  if (!a || !b) return false;
  return ANCHORS.every((anchor) => {
    const aTabs = a.anchors[anchor] ?? [];
    const bTabs = b.anchors[anchor] ?? [];
    return aTabs.length === bTabs.length && aTabs.every((tab, index) => dockTabSkeletonsEqual(tab, bTabs[index]!));
  });
}

/** @emoji 🗄️ Every node/unit in a default {@link PanelDock}, indexed by id, for identity-preserving reconciliation. */
function indexPanelDockById(dock: PanelDock): { readonly nodes: Map<string, PanelTabNode>; readonly units: Map<string, PanelTreeUnit> } {
  const nodes = new Map<string, PanelTabNode>();
  const units = new Map<string, PanelTreeUnit>();
  const visit = (node: PanelTabNode) => {
    nodes.set(node.id, node);
    if (node.kind === "branch") node.children.forEach(visit);
    else node.trees.forEach((unit) => units.set(unit.id, unit));
  };
  for (const anchor of ANCHORS) dock.anchors[anchor].forEach(visit);
  return { nodes, units };
}

/**
 * 🗄️ Rearranges (never reconstructs) `defaultDock`'s nodes/units to match a persisted {@link DockSkeleton} diff:
 * ids the default no longer declares are dropped, a tab whose kind no longer matches its default shape falls back
 * to the default's own shape, and any default tab/unit the skeleton doesn't mention is appended at its default
 * location. Subtrees untouched by the skeleton keep their exact default object identity.
 **/
/** @emoji 🗄️ Collects every tab id and tree-unit id the skeleton explicitly mentions (across all anchors), gated by whether the corresponding default node's own kind agrees — a mismatched entry's `children`/`trees` are never recursed into. */
function collectDockSkeletonMentions(entries: readonly DockTabSkeleton[], nodes: ReadonlyMap<string, PanelTabNode>, tabIds: Set<string>, unitIds: Set<string>): void {
  for (const entry of entries) {
    tabIds.add(entry.id);
    const defaultNode = nodes.get(entry.id);
    if (defaultNode?.kind === "branch" && entry.children) collectDockSkeletonMentions(entry.children, nodes, tabIds, unitIds);
    if (defaultNode?.kind === "leaf" && entry.trees) entry.trees.forEach((id) => unitIds.add(id));
  }
}

/** @emoji 🗄️ True if `node` itself, or any node in its default subtree, is explicitly mentioned somewhere in the persisted skeleton — distinguishes "this whole branch was deliberately emptied out by a move" (some descendant reappears elsewhere) from "this branch is simply missing from a stale skeleton" (nothing under it is mentioned anywhere, safe to re-seed from defaults). */
function defaultSubtreeMentioned(node: PanelTabNode, mentionedTabIds: ReadonlySet<string>): boolean {
  if (mentionedTabIds.has(node.id)) return true;
  return node.kind === "branch" && node.children.some((child) => defaultSubtreeMentioned(child, mentionedTabIds));
}

export function applyDockSkeleton(defaultDock: PanelDock, skeleton: DockSkeleton | null): PanelDock {
  if (!skeleton) return defaultDock;
  const { nodes, units } = indexPanelDockById(defaultDock);
  // Mentions are collected up front across the WHOLE skeleton (not incrementally during resolution) so that which
  // anchor/branch gets processed first never affects which default children/units get auto-appended back — a tab
  // moved to an anchor processed later must not be reclaimed by its old branch's "append missing" pass.
  const mentionedTabIds = new Set<string>();
  const mentionedUnitIds = new Set<string>();
  for (const anchor of ANCHORS) collectDockSkeletonMentions(skeleton.anchors[anchor] ?? [], nodes, mentionedTabIds, mentionedUnitIds);
  const resolvedTabIds = new Set<string>(); // guards only against the same id appearing twice in the skeleton

  const resolveNode = (entry: DockTabSkeleton): PanelTabNode | null => {
    const defaultNode = nodes.get(entry.id);
    if (!defaultNode || resolvedTabIds.has(entry.id)) return null;
    resolvedTabIds.add(entry.id);
    if (defaultNode.kind === "branch") {
      if (!entry.children) return defaultNode;
      const explicit = entry.children.map(resolveNode).filter((node): node is PanelTabNode => node !== null);
      const appended = defaultNode.children.filter((child) => !defaultSubtreeMentioned(child, mentionedTabIds));
      appended.forEach((child) => resolvedTabIds.add(child.id));
      const merged = [...explicit, ...appended];
      const unchanged = merged.length === defaultNode.children.length && merged.every((child, index) => child === defaultNode.children[index]);
      return unchanged ? defaultNode : { ...defaultNode, children: merged };
    }
    if (!entry.trees) return defaultNode;
    const explicitUnits = entry.trees.map((id) => units.get(id)).filter((unit): unit is PanelTreeUnit => unit !== undefined);
    const appendedUnits = defaultNode.trees.filter((unit) => !mentionedUnitIds.has(unit.id));
    const mergedUnits = [...explicitUnits, ...appendedUnits];
    const unchanged = mergedUnits.length === defaultNode.trees.length && mergedUnits.every((unit, index) => unit === defaultNode.trees[index]);
    return unchanged ? defaultNode : { ...defaultNode, trees: mergedUnits };
  };

  const anchors = {} as Record<Anchor, readonly PanelTabNode[]>;
  for (const anchor of ANCHORS) {
    const explicit = (skeleton.anchors[anchor] ?? []).map(resolveNode).filter((node): node is PanelTabNode => node !== null);
    const defaultTabs = defaultDock.anchors[anchor];
    const appended = defaultTabs.filter((tab) => !defaultSubtreeMentioned(tab, mentionedTabIds));
    appended.forEach((tab) => resolvedTabIds.add(tab.id));
    const merged = [...explicit, ...appended];
    const unchanged = merged.length === defaultTabs.length && merged.every((tab, index) => tab === defaultTabs[index]);
    anchors[anchor] = unchanged ? defaultTabs : merged;
  }
  return { anchors };
}

/** @emoji ↔ Insert-position indicator shown between tab buttons while a drag hovers a row. */
const panelTabInsertPreviewClass = "w-0.5 self-stretch rounded-full bg-accent shrink-0";

/** @emoji 📑️ One tab button; a child component (not inlined in {@link PanelTabRow}'s `.map`) so it can call driver-aware hooks per tab. */
const PanelTabButton: React.FC<{
  readonly tab: PanelTabNode;
  readonly variant: PanelTabBarVariant;
  readonly buttonClass: string;
  readonly tabSlot: string;
  readonly isActive: boolean;
  readonly showActiveColor: boolean;
  readonly stackIndex: number;
  readonly stackSize: number;
  readonly isDragSource: boolean;
  readonly isChildDropTarget: boolean;
  readonly isUnitDropReady: boolean;
  readonly anchor?: Anchor;
  readonly dock: PanelDockContextValue | null;
  readonly onSelect: (tabId: string) => void;
}> = ({ tab, variant, buttonClass, tabSlot, isActive, showActiveColor, stackIndex, stackSize, isDragSource, isChildDropTarget, isUnitDropReady, anchor, dock, onSelect }) => {
  const Icon = tab.icon;
  const inlineText = useControlInlineText(tab.id, tab.name);
  const surfaceDrag = useUiDriverDragSurface();
  const level = useLevel();
  const draggable = Boolean(anchor && dock);
  const windowTabChrome = variant === "panel" || variant === "chrome";
  const inactiveTabChromeClass = windowTabChrome ? panelWindowInactiveTabClass : stackIndex === stackSize - 1 ? modeDockInactiveTabBeforeGapClass : modeDockInactiveTabClass;
  return (
    <ChromeControlHint id={tab.id} text={tab.name}>
      <button
        data-slot={`${tabSlot}-tab-button`}
        data-hover-scope
        data-tab-id={tab.id}
        data-tab-kind={tab.kind}
        data-drag-source={isDragSource ? "true" : undefined}
        data-drop-nest={isChildDropTarget ? "true" : undefined}
        id={tab.id}
        data-level={level}
        data-active={isActive ? "true" : undefined}
        data-state={isActive && showActiveColor ? "on" : undefined}
        onClick={() => onSelect(tab.id)}
        onPointerDown={draggable && surfaceDrag ? (event) => dock!.startTabDrag(anchor!, tab.id, tab.name, event) : undefined}
        onDragOver={
          anchor && dock && tab.kind === "leaf"
            ? (event) => {
                if (event.dataTransfer.types.includes(PANEL_TREE_UNIT_MIME)) event.preventDefault();
              }
            : undefined
        }
        onDrop={
          anchor && dock && tab.kind === "leaf"
            ? (event) => {
                if (!event.dataTransfer.types.includes(PANEL_TREE_UNIT_MIME)) return;
                event.preventDefault();
                const session = readActivePanelTreeUnitDrag();
                if (!session) return;
                dock.onTreeUnitDockDrop({ unitId: session.unitId, fromTabId: session.tabId, target: { anchor, tabId: tab.id, index: Number.MAX_SAFE_INTEGER } });
                endPanelTreeUnitDrag();
              }
            : undefined
        }
        className={cn(
          windowTabChrome ? modeDockTabClassName : buttonClass,
          windowTabChrome && "h-full",
          windowTabChrome && !isActive && inactiveTabChromeClass,
          windowTabChrome && isActive && showActiveColor && modeDockActiveTabClass,
          !windowTabChrome && isActive && showActiveColor && modeDockActiveTabFillClass,
          windowTabChrome && panelTabButtonDividerClass,
          isDragSource && "opacity-40",
          isChildDropTarget && "ring-2 ring-accent",
          isUnitDropReady && dropZoneReadyClass,
        )}
      >
        {windowTabChrome ? (
          <div className={modeDockTabLabelClassName}>
            <span className="shrink-0"><Icon size={12} /></span>
            {inlineText !== undefined ? (
              <span data-slot="inline-label" className="truncate">
                {inlineText}
              </span>
            ) : null}
          </div>
        ) : (
          <>
            <span className={panelTabIconSlotClass}>
              <Icon size={12} />
            </span>
            {inlineText !== undefined ? (
              <span data-slot="inline-label" className={panelTabLabelClass}>
                {inlineText}
              </span>
            ) : null}
          </>
        )}
        {draggable && !surfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" onPointerDown={(event) => dock!.startTabDrag(anchor!, tab.id, tab.name, event)} onClick={(event) => event.stopPropagation()} emphasized={(isActive && showActiveColor) || isUnitDropReady} /> : null}
      </button>
    </ChromeControlHint>
  );
};

/** @emoji 📑️ Props for {@link PanelTabRow}. */
interface PanelTabRowProps {
  readonly variant: PanelTabBarVariant;
  /** @emoji 🧲️ Present only for {@link Panel} rows — enables drag-and-drop wiring via {@link usePanelDockContext}. */
  readonly anchor?: Anchor;
  readonly parentPath?: readonly string[];
  readonly tabs: readonly PanelTabNode[];
  readonly activeId?: string;
  readonly onSelect: (tabId: string) => void;
  /** @emoji 🎨️ Paints the active tab's fill/border — off for a folded {@link Panel}, whose button group shouldn't claim a tab is "active" while nothing is showing. */
  readonly showActiveColor?: boolean;
  /** @emoji 🎀️ Stacking direction from {@link PanelTabBar} — flips the row's divider to the content-facing side for `"panel"` variant. */
  readonly direction?: "up" | "down";
  /** @emoji 📏️ Extends a body-hosted tab line across the panel instead of sizing it like a silhouette cap chip. */
  readonly fullWidth?: boolean;
}

/** @emoji 📑️ One row of sibling tabs; stacked by {@link PanelTabBar} into a {@link Ribbon}. Tab rows keep declared left-to-right order independently of a right anchor's spatially mirrored panel flow, so folding and unfolding never reverses visual or keyboard progression. */
const PanelTabRow: React.FC<PanelTabRowProps> = ({ variant, anchor, parentPath = [], tabs, activeId, onSelect, showActiveColor = true, direction = "down", fullWidth = false }) => {
  const barRef = reactHostPort.useRef<HTMLDivElement>(null);
  const dock = usePanelDockContext();
  const tabSlot = variant === "mobile" ? "mobile-panel" : "panel";
  const sortedTabs = reactHostPort.useMemo(() => [...tabs].sort((a, b) => (a.order ?? 0) - (b.order ?? 0)), [tabs]);
  const resolvedActiveId = activeId;

  reactHostPort.useLayoutEffect(() => {
    const bar = barRef.current;
    if (!bar) return;
    bar.scrollLeft = 0;
    const activeButton = bar.querySelector<HTMLElement>(`[data-slot="${tabSlot}-tab-button"][data-active="true"]`);
    activeButton?.scrollIntoView({ block: "nearest", inline: "nearest" });
  }, [resolvedActiveId, sortedTabs, tabSlot]);

  const parentPathKey = parentPath.join("/");
  const setRowRef = reactHostPort.useCallback(
    (element: HTMLDivElement | null) => {
      barRef.current = element;
      if (anchor && dock) dock.registerTabRowDropTarget(anchor, parentPath, element);
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [anchor, dock, parentPathKey],
  );

  if (sortedTabs.length === 0) return null;

  const resolvedVariant = variant === "chrome" ? "panel" : variant;
  const barClass = resolvedVariant === "panel" ? panelAnchorTabBarClass(direction, showActiveColor) : mobilePanelTabBarClass;
  const buttonClass = resolvedVariant === "mobile" ? mobilePanelTabButtonClass : panelAnchorTabButtonClass;
  const dropTarget = dock?.dropTarget;
  const isDropRow = Boolean(anchor && dropTarget?.kind === "insert" && dropTarget.anchor === anchor && dropTarget.parentPath.length === parentPath.length && dropTarget.parentPath.every((id, index) => id === parentPath[index]));
  const dropInsertIndex = isDropRow && dropTarget?.kind === "insert" ? dropTarget.index : null;
  const tabDragActive = Boolean(dock?.dragTabId);
  const rowDropReady = tabDragActive && Boolean(anchor) && !parentPath.some((id) => dock?.draggedSubtreeIds?.has(id));
  const unitDragActive = usePanelTreeUnitDragActive();
  // 👻️ Navbar/footer chrome toggles and folded panel root rows stay visible during canvas ghost; only open panel tab strips dim.
  const ghostDim = showActiveColor;

  return (
    <div ref={setRowRef} {...(ghostDim ? { "data-dim": true } : {})} dir="ltr" data-slot={`${tabSlot}-tabs`} className={cn(barClass, fullWidth && "w-full", rowDropReady && dropZoneReadyClass)}>
      {sortedTabs.map((tab, index) => {
        const isActive = tab.id === resolvedActiveId;
        const isDragSource = Boolean(anchor && dock?.dragTabId === tab.id);
        const isChildDropTarget = Boolean(anchor && dropTarget?.kind === "child" && dropTarget.anchor === anchor && dropTarget.parentId === tab.id);
        const isUnitDropReady = unitDragActive && Boolean(anchor) && tab.kind === "leaf";
        return (
          <React.Fragment key={tab.id}>
            {dropInsertIndex === index ? <div data-slot="panel-tab-insert-preview" aria-hidden className={panelTabInsertPreviewClass} /> : null}
            <PanelTabButton
              tab={tab}
              variant={variant}
              buttonClass={buttonClass}
              tabSlot={tabSlot}
              isActive={isActive}
              showActiveColor={showActiveColor}
              stackIndex={index}
              stackSize={sortedTabs.length}
              isDragSource={isDragSource}
              isChildDropTarget={isChildDropTarget}
              isUnitDropReady={isUnitDropReady}
              anchor={anchor}
              dock={dock}
              onSelect={onSelect}
            />
          </React.Fragment>
        );
      })}
      {dropInsertIndex === sortedTabs.length ? <div data-slot="panel-tab-insert-preview" aria-hidden className={panelTabInsertPreviewClass} /> : null}
    </div>
  );
};

/** @emoji 📑️ Props for {@link PanelTabBar}. */
export interface PanelTabBarProps {
  readonly variant: PanelTabBarVariant;
  /** @emoji 🧲️ Present only when hosted by a {@link Panel} under a {@link PanelDockProvider}. */
  readonly anchor?: Anchor;
  readonly tabs: readonly PanelTabNode[];
  readonly activePath: readonly string[];
  readonly onActivePathChange: (path: readonly string[]) => void;
  /** @emoji 🎀️ Stacking direction for nested rows — `"up"` for bottom panels (rows grow toward the display center), `"down"` otherwise. */
  readonly direction?: "up" | "down";
  /** @emoji 🗜️ Skips rows above this depth without skipping the descent through them — used when a host intentionally starts mid-tree (e.g. a secondary strip that continues after another bar already showed shallower rows). */
  readonly startDepth?: number;
  /** @emoji 🗜️ Stops after this many emitted rows — `1` is the generalization of the old "root row only" (a folded {@link Panel}'s button group, or a chrome-hosted bar, both of which only ever show one row). */
  readonly maxRows?: number;
  /** @emoji 🎨️ Paints the active tab's fill/border — off for a folded {@link Panel}, whose button group shouldn't claim a tab is "active" while nothing is showing. */
  readonly showActiveColor?: boolean;
}

/** @emoji 📑️ Panel tab strip shared by {@link Panel}, {@link PanelChromeTabBar} and {@link MobilePanel} — one {@link PanelTabRow} per tree level (within `[startDepth, startDepth + maxRows)`), stacked in a {@link Ribbon}. */
export const PanelTabBar: React.FC<PanelTabBarProps> = ({ variant, anchor, tabs, activePath, onActivePathChange, direction = "down", startDepth = 0, maxRows = Infinity, showActiveColor = true }) => {
  const rows: RibbonRow[] = [];
  let level = tabs;
  let depth = 0;
  while (level.length > 0) {
    const sorted = [...level].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
    const activeId = sorted.some((tab) => tab.id === activePath[depth]) ? activePath[depth] : undefined;
    const rowDepth = depth;
    const parentPath = activePath.slice(0, rowDepth);
    if (depth >= startDepth) {
      rows.push({
        key: `${variant}-row-${rowDepth}`,
        content: (
          <PanelTabRow
            variant={variant}
            anchor={anchor}
            parentPath={parentPath}
            tabs={sorted}
            activeId={activeId}
            onSelect={(tabId) => onActivePathChange([...activePath.slice(0, rowDepth), tabId])}
            showActiveColor={showActiveColor}
            direction={direction}
            fullWidth={startDepth > 0}
          />
        ),
      });
      if (rows.length >= maxRows) break;
    }
    const active = activeId ? sorted.find((tab) => tab.id === activeId) : undefined;
    level = (active && panelTabChildren(active)) ?? [];
    depth++;
  }
  if (rows.length === 0) return null;
  return <Ribbon direction={direction} rows={rows} className={variant === "panel" ? (startDepth > 0 ? "w-full max-w-full" : "w-fit max-w-full") : undefined} />;
};

// #endregion 📑️PanelTabBar

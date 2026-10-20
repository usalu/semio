/** 🧲 Owns the panel dock context, dock targets and the tree-unit drag session shared by panels and tab bars. */
import { ephemeralBox, ephemeralSet } from "@semio-tech/framework-transient";
import * as React from "react";
import { reactHostPort } from "../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";
import { panelGhostSessionBridge } from "../../🧱️elements/👻️PanelGhost/🟦️.tsx";
import { type Anchor } from "../⚓️panel-anchor/🟦️.ts";

/** 🎯️ Where a dragged tab lands: `"insert"` places it among `parentPath`'s children at `index` (root when `parentPath` is empty); `"child"` appends it as the last child of the branch tab `parentId` (leaf targets never promote to branches in v1). */
export type PanelTabDockTarget = { readonly kind: "insert"; readonly anchor: Anchor; readonly parentPath: readonly string[]; readonly index: number } | { readonly kind: "child"; readonly anchor: Anchor; readonly parentId: string };

/** 🎯️ Where a dragged tree unit lands: `tabId`'s (a leaf's) unit list, at `index`. */
export interface PanelTreeUnitDockTarget {
  readonly anchor: Anchor;
  readonly tabId: string;
  readonly index: number;
}

/** 🎯️ A completed tree-unit drag from leaf `fromTabId` to `target`. */
export interface PanelTreeUnitDockMove {
  readonly unitId: string;
  readonly fromTabId: string;
  readonly target: PanelTreeUnitDockTarget;
}

/** 🌱️ `dataTransfer` MIME identifying a tree-unit drag between leaf tabs. */
export const PANEL_TREE_UNIT_MIME = "application/x-semio-panel-tree-unit";

/** ↕️ `dataTransfer` MIME identifying a tree-section reorder drag within one {@link Tree}. */
export const TREE_SECTION_REORDER_MIME = "application/x-semio-tree-section-reorder";

export interface PanelTreeUnitDragSession {
  readonly tabId: string;
  readonly unitId: string;
  readonly label: string;
}

const activePanelTreeUnitDragSession = ephemeralBox<PanelTreeUnitDragSession | null>("framework.modules.ui.packages.typescript.targets.react.index.tsx.activePanelTreeUnitDragSession", null);

const panelTreeUnitDragListeners = ephemeralSet<() => void>("framework.modules.ui.packages.typescript.targets.react.index.tsx.panelTreeUnitDragListeners");

/** 🌱️ Records the active tree-unit drag until drop or dragend. */
export function beginPanelTreeUnitDrag(session: PanelTreeUnitDragSession): void {
  activePanelTreeUnitDragSession.current = session;
  panelGhostSessionBridge?.begin(null);
  panelTreeUnitDragListeners.forEach((listener) => listener());
}

/** 🌱️ Clears the active tree-unit drag session. */
export function endPanelTreeUnitDrag(): void {
  activePanelTreeUnitDragSession.current = null;
  panelGhostSessionBridge?.end();
  panelTreeUnitDragListeners.forEach((listener) => listener());
}

/** 🌱️ Returns the in-flight tree-unit drag, if any. */
export function readActivePanelTreeUnitDrag(): PanelTreeUnitDragSession | null {
  return activePanelTreeUnitDragSession.current;
}

/** 🌱️ True while a tree-unit drag is in flight — re-renders drop-zone consumers. */
export function usePanelTreeUnitDragActive(): boolean {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      panelTreeUnitDragListeners.add(listener);
      return () => panelTreeUnitDragListeners.delete(listener);
    },
    () => activePanelTreeUnitDragSession.current !== null,
    () => false,
  );
}

/** 🎛️ Business-free ui↔shell contract shared by every {@link Panel} under one {@link PanelDockProvider}. */
export interface PanelDockContextValue {
  readonly dragTabId: string | null;
  readonly draggedSubtreeIds: ReadonlySet<string> | null;
  readonly dropTarget: PanelTabDockTarget | null;
  readonly startTabDrag: (anchor: Anchor, tabId: string, label: string, event: React.PointerEvent<HTMLElement>) => void;
  readonly registerTabRowDropTarget: (anchor: Anchor, parentPath: readonly string[], element: HTMLElement | null) => void;
  readonly onTreeUnitDockDrop: (move: PanelTreeUnitDockMove) => void;
}

export const PanelDockContext = reactHostPort.createContext<PanelDockContextValue | null>(null);

/** 🎛️ The enclosing {@link PanelDockProvider} contract, or `null` outside one, including Layout's private mobile panel. */
export function usePanelDockContext(): PanelDockContextValue | null {
  return reactHostPort.useContext(PanelDockContext);
}

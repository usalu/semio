import { treeBranchSlots, rowForBranch, treeRowSlots, resolveHoverRow } from "../🧭️hover-path/🟦️.ts";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import * as React from "react";

const treeHoverPathAttr = "data-tree-hover-path";
const treeSelectionPathAttr = "data-tree-selection-path";

const clearTreePath = (root: HTMLElement, pathAttr: string) => {
  root.querySelectorAll(`[${pathAttr}]`).forEach((el) => el.removeAttribute(pathAttr));
};

const clearTreeHoverPath = (root: HTMLElement) => {
  clearTreePath(root, treeHoverPathAttr);
};

const markTerminalBranch = (row: Element, pathAttr: string) => {
  const slot = row.getAttribute("data-slot");
  if (slot === "tree-item-row" || slot === "control-tree-row" || slot === "window-measure-tree-row") {
    for (const sibling of [row.nextElementSibling, row.previousElementSibling]) {
      if (!sibling) continue;
      const siblingSlot = sibling.getAttribute("data-slot");
      if (siblingSlot && treeBranchSlots.has(siblingSlot)) {
        sibling.setAttribute(pathAttr, "branch");
        return;
      }
    }
  } else if (slot === "tree-section-row") {
    for (const sibling of [row.nextElementSibling, row.previousElementSibling]) {
      if (sibling?.getAttribute("data-slot") !== "collapsible-content") continue;
      for (const child of Array.from(sibling.children)) {
        if (child.getAttribute("data-slot") === "tree-section-content") {
          child.setAttribute(pathAttr, "branch");
          return;
        }
      }
    }
  } else if (slot === "tree-property-item") {
    for (const sibling of [row.nextElementSibling, row.previousElementSibling]) {
      if (sibling?.getAttribute("data-slot") === "tree-property-content") {
        sibling.setAttribute(pathAttr, "branch");
        return;
      }
    }
  }
};

const markTreeRowPath = (row: Element, root: HTMLElement, pathAttr: string) => {
  row.setAttribute(pathAttr, "row");
  markTerminalBranch(row, pathAttr);
  let el: Element | null = row.parentElement;
  while (el && el !== root) {
    const slot = el.getAttribute("data-slot");
    if (slot && treeBranchSlots.has(slot)) {
      el.setAttribute(pathAttr, "branch");
      const ownerRow = rowForBranch(el);
      if (ownerRow) {
        ownerRow.setAttribute(pathAttr, "row");
        markTerminalBranch(ownerRow, pathAttr);
      }
    }
    el = el.parentElement;
  }
};

const applyTreeHoverPath = (row: Element, root: HTMLElement) => {
  clearTreeHoverPath(root);
  markTreeRowPath(row, root, treeHoverPathAttr);
};

export const syncTreeSelectionPath = (root: HTMLElement, selectedIds: readonly string[]) => {
  clearTreePath(root, treeSelectionPathAttr);
  for (const selectedId of selectedIds) {
    const row = root.ownerDocument.getElementById(selectedId);
    if (!row || !root.contains(row)) {
      continue;
    }
    const slot = row.getAttribute("data-slot");
    if (!slot || !treeRowSlots.has(slot)) {
      continue;
    }
    markTreeRowPath(row, root, treeSelectionPathAttr);
  }
};

/** 🖱️ Pointer handlers that mark ancestor branch guide lines on row hover. */
export const useTreeHoverPathRootHandlers = () => {
  const treeRootRef = reactHostPort.useRef<HTMLDivElement>(null);
  const lastHoverRowRef = reactHostPort.useRef<Element | null>(null);
  const hoverPathFrameRef = reactHostPort.useRef<number | null>(null);
  const pendingHoverRowRef = reactHostPort.useRef<Element | null>(null);

  const flushTreeHoverPath = reactHostPort.useCallback(() => {
    hoverPathFrameRef.current = null;
    const root = treeRootRef.current;
    const row = pendingHoverRowRef.current;
    pendingHoverRowRef.current = null;
    if (!root) return;
    if (row) {
      applyTreeHoverPath(row, root);
      lastHoverRowRef.current = row;
      return;
    }
    clearTreeHoverPath(root);
    lastHoverRowRef.current = null;
  }, []);

  const scheduleTreeHoverPath = reactHostPort.useCallback(
    (row: Element | null) => {
      pendingHoverRowRef.current = row;
      if (hoverPathFrameRef.current !== null) {
        return;
      }
      hoverPathFrameRef.current = requestAnimationFrame(flushTreeHoverPath);
    },
    [flushTreeHoverPath],
  );

  const handleTreePointerOver = reactHostPort.useCallback(
    (e: React.PointerEvent<HTMLDivElement>) => {
      const root = treeRootRef.current;
      if (!root) return;
      const row = resolveHoverRow(e.target as HTMLElement, root);
      if (row === lastHoverRowRef.current) return;
      scheduleTreeHoverPath(row);
    },
    [scheduleTreeHoverPath],
  );

  const handleTreePointerLeave = reactHostPort.useCallback(() => {
    if (hoverPathFrameRef.current !== null) {
      cancelAnimationFrame(hoverPathFrameRef.current);
      hoverPathFrameRef.current = null;
    }
    pendingHoverRowRef.current = null;
    lastHoverRowRef.current = null;
    const root = treeRootRef.current;
    if (root) clearTreeHoverPath(root);
  }, []);

  const refreshTreeHoverPath = reactHostPort.useCallback(() => {
    const root = treeRootRef.current;
    const row = lastHoverRowRef.current;
    if (!root || !row) {
      return;
    }
    applyTreeHoverPath(row, root);
  }, []);

  return { treeRootRef, handleTreePointerOver, handleTreePointerLeave, refreshTreeHoverPath };
};

/** ✅️ Marks ancestor section rows when a descendant tree item is selected. */
export const useTreeSelectionPathSync = (treeRootRef: React.RefObject<HTMLDivElement | null>, selectedIds: readonly string[]) => {
  reactHostPort.useLayoutEffect(() => {
    const root = treeRootRef.current;
    if (!root) {
      return;
    }
    syncTreeSelectionPath(root, selectedIds);
  }, [selectedIds, treeRootRef]);
};

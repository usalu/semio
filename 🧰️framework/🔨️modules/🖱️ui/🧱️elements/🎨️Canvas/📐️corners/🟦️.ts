import type { WindowStackCorner, WindowLayoutWindowNode, WindowLayoutNode, WindowLayoutAxisNode, WindowLayoutStackNode } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import type { ResizableGroupHandle } from "../../↔️Resizable/🟦️.tsx";
import { type ModeDockSide, type ModeLayoutPath, collapseLayout, modeJoinPath, modePathSegments, readLayoutAtPath, removeWindowFromLayout, setActiveWindowInLayout, updateLayoutAtPath } from "../🗺️layout/🟦️.ts";

export const WINDOW_STACK_CORNERS: readonly WindowStackCorner[] = ["topLeft", "topRight", "bottomLeft", "bottomRight"];

/** 🔎️ Resolves a window node's stack corner, defaulting to top-left. */
export function resolveWindowCorner(node: { corner?: WindowStackCorner }): WindowStackCorner {
  return node.corner ?? "topLeft";
}

/** 🧭️ Groups stack window children by chrome corner while preserving relative order. */
export function modeStackTabsByCorner(children: readonly WindowLayoutWindowNode[]): Record<WindowStackCorner, WindowLayoutWindowNode[]> {
  const groups = { topLeft: [], topRight: [], bottomLeft: [], bottomRight: [] } as Record<WindowStackCorner, WindowLayoutWindowNode[]>;
  for (const child of children) groups[resolveWindowCorner(child)].push(child);
  return groups;
}

function flatIndexForCornerInsert(children: readonly WindowLayoutWindowNode[], corner: WindowStackCorner, cornerIndex?: number): number {
  const indices: number[] = [];
  for (let i = 0; i < children.length; i++) {
    if (resolveWindowCorner(children[i]!) === corner) indices.push(i);
  }
  if (indices.length === 0) return children.length;
  if (cornerIndex === undefined || cornerIndex < 0 || cornerIndex >= indices.length) return indices[indices.length - 1]! + 1;
  return indices[cornerIndex]!;
}

function insertWindowAsTab(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, index?: number, corner?: WindowStackCorner): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const insertAt = index === undefined || index < 0 ? children.length : index;
    const windowNode: WindowLayoutWindowNode = corner ? { kind: "window", id: windowId, corner } : { kind: "window", id: windowId };
    children.splice(insertAt, 0, windowNode);
    return { ...node, children, activeId: windowId };
  });
}

/** 🗺️ Inserts a window as a tab in a stack corner, mapping a corner-local index onto the flat children list. */
export function insertWindowAsTabAtCorner(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, corner: WindowStackCorner, index?: number): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const insertAt = flatIndexForCornerInsert(children, corner, index);
    children.splice(insertAt, 0, { kind: "window", id: windowId, corner });
    return { ...node, children, activeId: windowId };
  });
}

/** 📌️ Sets the chrome corner for a window node anywhere in the layout tree. */
export function setWindowCornerInLayout(layout: WindowLayoutNode, windowId: string, corner: WindowStackCorner): WindowLayoutNode {
  if (layout.kind === "window") return layout.id === windowId ? { ...layout, corner } : layout;
  if (layout.kind === "stack") {
    return {
      ...layout,
      children: layout.children.map((child) => (child.id === windowId ? { ...child, corner } : child)),
    };
  }
  return {
    ...layout,
    children: layout.children.map((child) => setWindowCornerInLayout(child as WindowLayoutNode, windowId, corner) as WindowLayoutAxisNode | WindowLayoutStackNode),
  };
}

/** 📑️ Merges every tab from a dragged stack into another stack corner at the given corner-local index. */
export function mergeStackTabsIntoStack(layout: WindowLayoutNode, targetStackPath: ModeLayoutPath, stack: WindowLayoutStackNode, index: number, corner: WindowStackCorner = "topLeft"): WindowLayoutNode {
  const insertAt = index < 0 ? undefined : index;
  let result = layout;
  stack.children.forEach((child, offset) => {
    result = insertWindowAsTabAtCorner(result, targetStackPath, child.id, corner, insertAt === undefined ? undefined : insertAt + offset);
  });
  const activeId = stack.activeId ?? stack.children[0]?.id;
  return activeId ? setActiveWindowInLayout(result, activeId) : result;
}

function reorderTabInStack(layout: WindowLayoutNode, stackPath: ModeLayoutPath, fromIndex: number, toIndex: number): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const [item] = children.splice(fromIndex, 1);
    if (!item) return node;
    children.splice(toIndex, 0, item);
    return { ...node, children };
  });
}

/** 🪟️ Splits a stack with a dragged window on the given side. */
export function splitWithWindow(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  return updateLayoutAtPath(without, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
    const horizontal = side === "left" || side === "right";
    const children = side === "left" || side === "top" ? [incoming, node] : [node, incoming];
    return { kind: horizontal ? "row" : "column", children, size: node.size } as WindowLayoutAxisNode;
  });
}

export function splitRootWithWindow(layout: WindowLayoutNode, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
  const horizontal = side === "left" || side === "right";
  const children = side === "left" || side === "top" ? [incoming, without] : [without, incoming];
  return { kind: horizontal ? "row" : "column", children: children as (WindowLayoutAxisNode | WindowLayoutStackNode)[] };
}

/** ⛏️ Detaches a tab stack from the layout tree for stack-level drag-dock. */
export function extractStackFromLayout(layout: WindowLayoutNode, stackPath: ModeLayoutPath): { layout: WindowLayoutNode | null; stack: WindowLayoutStackNode | null } {
  const stack = readLayoutAtPath(layout, stackPath);
  if (!stack || stack.kind !== "stack") return { layout, stack: null };
  if (!stackPath) return { layout: null, stack };
  const segments = modePathSegments(stackPath);
  const stackIndex = segments[segments.length - 1]!;
  const parentPath = segments.slice(0, -1).join(".");
  const parent = readLayoutAtPath(layout, parentPath);
  if (!parent || (parent.kind !== "row" && parent.kind !== "column")) return { layout, stack: null };
  const nextChildren = parent.children.filter((_, index) => index !== stackIndex);
  const nextParent = collapseLayout({ ...parent, children: nextChildren });
  if (!parentPath) return { layout: nextParent, stack };
  const without = updateLayoutAtPath(layout, parentPath, () => nextParent ?? { kind: "stack", children: [] });
  return { layout: collapseLayout(without), stack };
}

/** ✂️ Splits a stack with a dragged tab stack on the given side. */
export function splitWithStack(layout: WindowLayoutNode, targetStackPath: ModeLayoutPath, stack: WindowLayoutStackNode, side: ModeDockSide): WindowLayoutNode {
  return updateLayoutAtPath(layout, targetStackPath, (node) => {
    if (node.kind !== "stack") return node;
    const horizontal = side === "left" || side === "right";
    const children = side === "left" || side === "top" ? [stack, node] : [node, stack];
    return { kind: horizontal ? "row" : "column", children, size: node.size } as WindowLayoutAxisNode;
  });
}

/** 🏠️ Splits the mode root with a dragged tab stack on the given side. */
export function splitRootWithStack(layout: WindowLayoutNode, stack: WindowLayoutStackNode, side: ModeDockSide): WindowLayoutNode {
  const horizontal = side === "left" || side === "right";
  const children = side === "left" || side === "top" ? [stack, layout] : [layout, stack];
  return { kind: horizontal ? "row" : "column", children: children as (WindowLayoutAxisNode | WindowLayoutStackNode)[] };
}

/** 📝️ Writes resizable panel percentages back onto axis children. */
export function safePanelGroupSetLayout(group: ResizableGroupHandle, layout: Record<string, number>): void {
  try {
    group.setLayout(layout);
  } catch {}
}

export function applyAxisSizes(layout: WindowLayoutNode, axisPath: ModeLayoutPath, sizes: Record<string, number> | readonly number[]): WindowLayoutNode {
  return updateLayoutAtPath(layout, axisPath, (node) => {
    if (node.kind !== "row" && node.kind !== "column") return node;
    const sizesRecord = sizes as Record<string | number, number>;
    const arraySizes = Array.isArray(sizes) ? (sizes as readonly number[]) : null;
    if (arraySizes && arraySizes.length !== node.children.length) {
      const even = 100 / node.children.length;
      return { ...node, children: node.children.map((child) => ({ ...child, size: even })) };
    }
    if (!arraySizes) {
      const expectedKeys = node.children.map((_, index) => modeJoinPath(axisPath, index));
      const matched = expectedKeys.filter((key) => typeof sizesRecord[key] === "number").length;
      if (matched > 0 && matched !== node.children.length) {
        const even = 100 / node.children.length;
        return { ...node, children: node.children.map((child) => ({ ...child, size: even })) };
      }
    }
    const children = node.children.map((child, index) => {
      const panelKey = modeJoinPath(axisPath, index);
      const size = sizesRecord[panelKey] ?? sizesRecord[index] ?? sizesRecord[String(index)] ?? child.size;
      return { ...child, size };
    });
    return { ...node, children };
  });
}

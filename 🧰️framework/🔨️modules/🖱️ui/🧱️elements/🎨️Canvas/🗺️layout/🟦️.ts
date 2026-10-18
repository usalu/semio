import { type WindowLayoutNode, type WindowLayoutAxisNode, type WindowLayoutStackNode, createEvenWindowLayout } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import type { ModeWindowDescriptor } from "../🪟️template-drag/🟦️.ts";

export type ModeLayoutPath = string;
export type ModeDockSide = "left" | "right" | "top" | "bottom";

export function modePathSegments(path: ModeLayoutPath): number[] {
  return path ? path.split(".").map((segment) => Number(segment)) : [];
}

export function modeJoinPath(parent: ModeLayoutPath, index: number): ModeLayoutPath {
  return parent ? `${parent}.${index}` : String(index);
}

export function modeCollectWindowIds(node: WindowLayoutNode): string[] {
  if (node.kind === "window") return [node.id];
  if (node.kind === "stack") return node.children.map((child) => child.id);
  return node.children.flatMap(modeCollectWindowIds);
}

/** 🪟️ Ensures every window leaf sits inside a tab stack. */
export function normalizeLayoutToStacks(node: WindowLayoutNode): WindowLayoutNode {
  if (node.kind === "window") return { kind: "stack", children: [node], activeId: node.id };
  if (node.kind === "stack") return { ...node, activeId: node.activeId ?? node.children[0]?.id };
  return { ...node, children: node.children.map((child) => normalizeLayoutToStacks(child) as WindowLayoutAxisNode | WindowLayoutStackNode) };
}

/** 🪟️ Collapses empty axes and hoists single-child axes. */
export function collapseLayout(node: WindowLayoutNode | null): WindowLayoutNode | null {
  if (!node) return null;
  if (node.kind === "window") return node;
  if (node.kind === "stack") return node.children.length === 0 ? null : node;
  const children = node.children.map((child) => collapseLayout(child)).filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => child !== null);
  if (children.length === 0) return null;
  if (children.length === 1) {
    const only = children[0]!;
    return { ...only, size: only.size ?? node.size };
  }
  return { ...node, children };
}

export function updateLayoutAtPath(layout: WindowLayoutNode, path: ModeLayoutPath, updater: (node: WindowLayoutNode) => WindowLayoutNode): WindowLayoutNode {
  if (!path) return updater(layout);
  const [head, ...rest] = modePathSegments(path);
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return updater(layout);
  const child = layout.children[head!];
  if (!child) return layout;
  if (rest.length === 0) {
    const nextChildren = [...layout.children];
    nextChildren[head!] = updater(child as WindowLayoutNode) as WindowLayoutAxisNode | WindowLayoutStackNode;
    return { ...layout, children: nextChildren };
  }
  const nextChildren = [...layout.children];
  nextChildren[head!] = updateLayoutAtPath(child as WindowLayoutNode, rest.join("."), updater) as WindowLayoutAxisNode | WindowLayoutStackNode;
  return { ...layout, children: nextChildren };
}

export function readLayoutAtPath(layout: WindowLayoutNode, path: ModeLayoutPath): WindowLayoutNode | null {
  if (!path) return layout;
  const [head, ...rest] = modePathSegments(path);
  if (layout.kind === "window") return null;
  if (layout.kind === "stack") return layout;
  const child = layout.children[head!];
  if (!child) return null;
  if (rest.length === 0) return child as WindowLayoutNode;
  return readLayoutAtPath(child as WindowLayoutNode, rest.join("."));
}

export function mapLayoutStacks(layout: WindowLayoutNode, mapper: (stack: WindowLayoutStackNode, path: ModeLayoutPath) => WindowLayoutStackNode, path = ""): WindowLayoutNode {
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return mapper(layout, path);
  return {
    ...layout,
    children: layout.children.map((child, index) => mapLayoutStacks(child as WindowLayoutNode, mapper, modeJoinPath(path, index)) as WindowLayoutAxisNode | WindowLayoutStackNode),
  };
}

/** 🧭️ Finds the stack path that currently holds `windowId`, or null if absent. */
export function resolveStackPathForWindowId(layout: WindowLayoutNode, windowId: string): ModeLayoutPath | null {
  let found: ModeLayoutPath | null = null;
  mapLayoutStacks(layout, (stack, path) => {
    if (stack.children.some((child) => child.id === windowId)) found = path;
    return stack;
  });
  return found;
}

/** 🪟️ Drops windows that are no longer declared without auto-opening undeclared layout slots. */
export function reconcileWindows(layout: WindowLayoutNode, windowIds: readonly string[]): WindowLayoutNode {
  const normalized = normalizeLayoutToStacks(layout);
  const allowed = new Set(windowIds);
  const result = windowIds.length === 0 ? normalized : removeAbsentWindowsFromLayout(normalized, allowed);
  return collapseLayout(result) ?? { kind: "stack", children: [] };
}

function removeAbsentWindowsFromLayout(layout: WindowLayoutNode, allowed: ReadonlySet<string>): WindowLayoutNode {
  if (layout.kind === "window") return allowed.has(layout.id) ? layout : { kind: "stack", children: [] };
  if (layout.kind === "stack") {
    const children = layout.children.filter((child) => allowed.has(child.id));
    const activeId = layout.activeId && allowed.has(layout.activeId) ? layout.activeId : children[0]?.id;
    return { ...layout, children, activeId };
  }
  return {
    ...layout,
    children: layout.children.map((child) => removeAbsentWindowsFromLayout(child as WindowLayoutNode, allowed)).filter((child) => child.kind !== "stack" || child.children.length > 0) as (WindowLayoutAxisNode | WindowLayoutStackNode)[],
  };
}

/** 🪟️ Removes a window from the layout tree and collapses empty nodes. */
export function removeWindowFromLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode | null {
  if (layout.kind === "window") return layout.id === windowId ? null : layout;
  if (layout.kind === "stack") {
    const children = layout.children.filter((child) => child.id !== windowId);
    if (children.length === 0) return null;
    const activeId = layout.activeId === windowId ? children[0]?.id : layout.activeId;
    return { ...layout, children, activeId };
  }
  const children = layout.children.map((child) => removeWindowFromLayout(child as WindowLayoutNode, windowId)).filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => child !== null);
  if (children.length === 0) return null;
  return collapseLayout({ ...layout, children });
}

export function setActiveWindowInLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode {
  return mapLayoutStacks(layout, (stack) => {
    if (!stack.children.some((child) => child.id === windowId)) return stack;
    if (stack.activeId === windowId) return stack;
    return { ...stack, activeId: windowId };
  });
}

export function resolveModeLayout(windows: readonly ModeWindowDescriptor[], layout?: WindowLayoutNode): WindowLayoutNode {
  const base = layout ?? createEvenWindowLayout(windows.map((window) => window.id));
  return reconcileWindows(
    base,
    windows.map((window) => window.id),
  );
}

/** 🪟️ THE window layout contract: types of `🧬️schema/🔣️.json` and its pure reference operations, twin of Rust `window_layout_ops`. Verified by the shared corpus `🧪️tests/🧫️fixtures/🔣️.json`. A window's id is `instanceId ?? windowKindId`; paths are child-index lists and every drop path addresses the layout AFTER the dragged window or stack was removed. */
export type WindowStackCorner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";
export type WindowLayoutAxis = "horizontal" | "vertical";
export type WindowLayoutSide = "left" | "right" | "top" | "bottom";
export type WindowLayoutPath = readonly number[];

export interface WindowLayoutWindowNode {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly instanceId?: string;
  readonly templateId?: string;
  readonly title?: string;
  readonly corner?: WindowStackCorner;
}
export interface WindowLayoutStackNode {
  readonly kind: "stack";
  readonly size?: number;
  readonly activeWindowId?: string;
  readonly children: readonly WindowLayoutWindowNode[];
}
export interface WindowLayoutSplitNode {
  readonly kind: "split";
  readonly axis: WindowLayoutAxis;
  readonly size?: number;
  readonly children: readonly WindowLayoutPanel[];
}
export type WindowLayoutPanel = WindowLayoutStackNode | WindowLayoutSplitNode;
export type WindowLayoutNode = WindowLayoutPanel | WindowLayoutWindowNode;
export interface WindowLayout {
  readonly root: WindowLayoutPanel;
}

export type WindowLayoutDropZone =
  | { readonly kind: "tab"; readonly stackPath: WindowLayoutPath; readonly corner: WindowStackCorner; readonly index?: number }
  | { readonly kind: "split"; readonly stackPath: WindowLayoutPath; readonly side: WindowLayoutSide }
  | { readonly kind: "rootSplit"; readonly side: WindowLayoutSide };

export type WindowLayoutOp =
  | { readonly op: "setActive"; readonly windowId: string }
  | { readonly op: "closeWindow"; readonly windowId: string }
  | { readonly op: "reorderTab"; readonly stackPath: WindowLayoutPath; readonly from: number; readonly to: number }
  | { readonly op: "setCorner"; readonly windowId: string; readonly corner: WindowStackCorner }
  | { readonly op: "applySizes"; readonly splitPath: WindowLayoutPath; readonly sizes: readonly number[] }
  | { readonly op: "reconcile"; readonly windowIds: readonly string[] }
  | { readonly op: "dropWindow"; readonly window: WindowLayoutWindowNode; readonly zone: WindowLayoutDropZone }
  | { readonly op: "dropStack"; readonly sourcePath: WindowLayoutPath; readonly zone: WindowLayoutDropZone };

export const WINDOW_STACK_CORNERS: readonly WindowStackCorner[] = ["topLeft", "topRight", "bottomLeft", "bottomRight"];

export const emptyWindowStack = (): WindowLayoutStackNode => ({ kind: "stack", children: [] });
export const windowIdOf = (window: Pick<WindowLayoutWindowNode, "windowKindId" | "instanceId">): string => window.instanceId ?? window.windowKindId;
export const windowCornerOf = (window: { readonly corner?: WindowStackCorner }): WindowStackCorner => window.corner ?? "topLeft";

const sizeOf = (node: WindowLayoutNode): number | undefined => (node.kind === "window" ? undefined : node.size);
const withSize = <T extends WindowLayoutNode>(node: T, size: number | undefined): T => (node.kind === "window" ? node : { ...node, size });
const withCorner = (window: WindowLayoutWindowNode, corner: WindowStackCorner | undefined): WindowLayoutWindowNode => ({ ...window, corner });
const singleStack = (window: WindowLayoutWindowNode): WindowLayoutStackNode => ({ kind: "stack", activeWindowId: windowIdOf(window), children: [withCorner(window, undefined)] });

function splitPair(side: WindowLayoutSide, incoming: WindowLayoutPanel, existing: WindowLayoutPanel, size?: number): WindowLayoutSplitNode {
  const first = side === "left" || side === "top";
  return { kind: "split", axis: side === "left" || side === "right" ? "horizontal" : "vertical", size, children: first ? [incoming, existing] : [existing, incoming] };
}

export function collectWindowIds(node: WindowLayoutNode): string[] {
  if (node.kind === "window") return [windowIdOf(node)];
  if (node.kind === "stack") return node.children.map(windowIdOf);
  return node.children.flatMap(collectWindowIds);
}

export function readLayoutAt(layout: WindowLayoutNode, path: WindowLayoutPath): WindowLayoutNode | null {
  if (path.length === 0) return layout;
  if (layout.kind === "window") return null;
  if (layout.kind === "stack") return layout;
  const child = layout.children[path[0]!];
  return child ? readLayoutAt(child, path.slice(1)) : null;
}

export function updateLayoutAt<T extends WindowLayoutNode>(layout: T, path: WindowLayoutPath, updater: (node: WindowLayoutNode) => WindowLayoutNode): WindowLayoutNode {
  if (path.length === 0) return updater(layout);
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return updater(layout);
  const child = layout.children[path[0]!];
  if (!child) return layout;
  const children = [...layout.children];
  children[path[0]!] = updateLayoutAt(child, path.slice(1), updater) as WindowLayoutPanel;
  return { ...layout, children };
}

export function mapLayoutStacks(layout: WindowLayoutNode, mapper: (stack: WindowLayoutStackNode, path: WindowLayoutPath) => WindowLayoutStackNode, path: WindowLayoutPath = []): WindowLayoutNode {
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return mapper(layout, path);
  return { ...layout, children: layout.children.map((child, index) => mapLayoutStacks(child, mapper, [...path, index]) as WindowLayoutPanel) };
}

export function stackPathOf(layout: WindowLayoutNode, windowId: string): WindowLayoutPath | null {
  let found: WindowLayoutPath | null = null;
  mapLayoutStacks(layout, (stack, path) => {
    if (stack.children.some((child) => windowIdOf(child) === windowId)) found = path;
    return stack;
  });
  return found;
}

/** 🗜️ Drops empty stacks and hoists single-child splits (the child keeps its size, else takes the split's). */
export function collapseLayout(node: WindowLayoutNode | null): WindowLayoutNode | null {
  if (!node) return null;
  if (node.kind === "window") return node;
  if (node.kind === "stack") return node.children.length === 0 ? null : node;
  const children = node.children.map(collapseLayout).filter((child): child is WindowLayoutPanel => child !== null);
  if (children.length === 0) return null;
  if (children.length === 1) return withSize(children[0]!, sizeOf(children[0]!) ?? node.size);
  return { ...node, children };
}

export function removeWindow(layout: WindowLayoutNode, windowId: string): WindowLayoutNode | null {
  if (layout.kind === "window") return windowIdOf(layout) === windowId ? null : layout;
  if (layout.kind === "stack") {
    const children = layout.children.filter((child) => windowIdOf(child) !== windowId);
    if (children.length === 0) return null;
    return { ...layout, children, activeWindowId: layout.activeWindowId === windowId ? windowIdOf(children[0]!) : layout.activeWindowId };
  }
  const children = layout.children.map((child) => removeWindow(child, windowId)).filter((child): child is WindowLayoutPanel => child !== null);
  return children.length === 0 ? null : collapseLayout({ ...layout, children });
}

export function setActiveWindow(layout: WindowLayoutNode, windowId: string): WindowLayoutNode {
  return mapLayoutStacks(layout, (stack) => (stack.activeWindowId !== windowId && stack.children.some((child) => windowIdOf(child) === windowId) ? { ...stack, activeWindowId: windowId } : stack));
}

function normalizeActive(node: WindowLayoutNode): WindowLayoutPanel {
  if (node.kind === "window") return { kind: "stack", children: [node], activeWindowId: windowIdOf(node) };
  if (node.kind === "stack") return { ...node, activeWindowId: node.activeWindowId ?? (node.children[0] && windowIdOf(node.children[0])) };
  return { ...node, children: node.children.map(normalizeActive) };
}

function removeAbsent(node: WindowLayoutNode, allowed: ReadonlySet<string>): WindowLayoutPanel {
  if (node.kind === "window") return allowed.has(windowIdOf(node)) ? { kind: "stack", children: [node] } : emptyWindowStack();
  if (node.kind === "stack") {
    const children = node.children.filter((child) => allowed.has(windowIdOf(child)));
    return { ...node, children, activeWindowId: node.activeWindowId && allowed.has(node.activeWindowId) ? node.activeWindowId : children[0] && windowIdOf(children[0]) };
  }
  return { ...node, children: node.children.map((child) => removeAbsent(child, allowed)).filter((child) => child.kind !== "stack" || child.children.length > 0) };
}

/** 🪟️ Drops windows that are no longer declared; an empty id list keeps everything. */
export function reconcileWindows(layout: WindowLayoutNode, windowIds: readonly string[]): WindowLayoutPanel {
  const normalized = normalizeActive(layout);
  return (collapseLayout(windowIds.length === 0 ? normalized : removeAbsent(normalized, new Set(windowIds))) as WindowLayoutPanel | null) ?? emptyWindowStack();
}

export function setWindowCorner(layout: WindowLayoutNode, windowId: string, corner: WindowStackCorner): WindowLayoutNode {
  if (layout.kind === "window") return windowIdOf(layout) === windowId ? withCorner(layout, corner) : layout;
  if (layout.kind === "stack") return { ...layout, children: layout.children.map((child) => (windowIdOf(child) === windowId ? withCorner(child, corner) : child)) };
  return { ...layout, children: layout.children.map((child) => setWindowCorner(child, windowId, corner) as WindowLayoutPanel) };
}

export function applySplitSizes(layout: WindowLayoutNode, splitPath: WindowLayoutPath, sizes: readonly number[]): WindowLayoutNode {
  return updateLayoutAt(layout, splitPath, (node) => {
    if (node.kind !== "split") return node;
    const even = 1 / node.children.length;
    return { ...node, children: node.children.map((child, index) => withSize(child, sizes.length === node.children.length ? sizes[index] : even)) };
  });
}

export function reorderTab(layout: WindowLayoutNode, stackPath: WindowLayoutPath, from: number, to: number): WindowLayoutNode {
  return updateLayoutAt(layout, stackPath, (node) => {
    if (node.kind !== "stack" || from >= node.children.length) return node;
    const children = [...node.children];
    const [item] = children.splice(from, 1);
    children.splice(to, 0, item!);
    return { ...node, children };
  });
}

function flatIndexForCorner(children: readonly WindowLayoutWindowNode[], corner: WindowStackCorner, cornerIndex?: number): number {
  const indices = children.flatMap((child, index) => (windowCornerOf(child) === corner ? [index] : []));
  if (indices.length === 0) return children.length;
  return cornerIndex === undefined || cornerIndex >= indices.length ? indices[indices.length - 1]! + 1 : indices[cornerIndex]!;
}

export function insertTab(layout: WindowLayoutNode, stackPath: WindowLayoutPath, window: WindowLayoutWindowNode, corner: WindowStackCorner, index?: number): WindowLayoutNode {
  return updateLayoutAt(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    children.splice(flatIndexForCorner(children, corner, index), 0, withCorner(window, corner));
    return { ...node, children, activeWindowId: windowIdOf(window) };
  });
}

function splitWith(layout: WindowLayoutNode, stackPath: WindowLayoutPath, incoming: WindowLayoutPanel, side: WindowLayoutSide): WindowLayoutNode {
  return updateLayoutAt(layout, stackPath, (node) => (node.kind === "stack" ? splitPair(side, incoming, node, node.size) : node));
}

/** ⛏️ Detaches a stack; `layout` is null when it was the whole tree. */
export function extractStack(layout: WindowLayoutNode, path: WindowLayoutPath): { layout: WindowLayoutNode | null; stack: WindowLayoutStackNode | null } {
  const stack = readLayoutAt(layout, path);
  if (!stack || stack.kind !== "stack") return { layout, stack: null };
  if (path.length === 0) return { layout: null, stack };
  const parentPath = path.slice(0, -1);
  const parent = readLayoutAt(layout, parentPath);
  if (!parent || parent.kind !== "split") return { layout, stack: null };
  const nextParent = collapseLayout({ ...parent, children: parent.children.filter((_, index) => index !== path[path.length - 1]) });
  if (parentPath.length === 0) return { layout: nextParent, stack };
  return { layout: collapseLayout(updateLayoutAt(layout, parentPath, () => nextParent ?? emptyWindowStack())), stack };
}

const isStackAt = (layout: WindowLayoutNode, path: WindowLayoutPath): boolean => readLayoutAt(layout, path)?.kind === "stack";

export function dropWindow(layout: WindowLayoutNode, window: WindowLayoutWindowNode, zone: WindowLayoutDropZone): WindowLayoutNode {
  const without = removeWindow(layout, windowIdOf(window));
  if (zone.kind === "rootSplit") return splitPair(zone.side, singleStack(window), (without as WindowLayoutPanel | null) ?? emptyWindowStack());
  if (!without || !isStackAt(without, zone.stackPath)) return layout;
  return zone.kind === "split" ? splitWith(without, zone.stackPath, singleStack(window), zone.side) : insertTab(without, zone.stackPath, window, zone.corner, zone.index);
}

export function dropStack(layout: WindowLayoutNode, sourcePath: WindowLayoutPath, zone: WindowLayoutDropZone): WindowLayoutNode {
  const { layout: without, stack } = extractStack(layout, sourcePath);
  if (!stack) return layout;
  const base = without ?? emptyWindowStack();
  if (zone.kind === "rootSplit") return splitPair(zone.side, stack, base as WindowLayoutPanel);
  if (!isStackAt(base, zone.stackPath)) return layout;
  if (zone.kind === "split") return splitWith(base, zone.stackPath, stack, zone.side);
  let result = base;
  stack.children.forEach((child, offset) => {
    result = insertTab(result, zone.stackPath, child, zone.corner, zone.index === undefined ? undefined : zone.index + offset);
  });
  const active = stack.activeWindowId ?? (stack.children[0] && windowIdOf(stack.children[0]));
  return active ? setActiveWindow(result, active) : result;
}

/** 🧰️ Applies one edit and returns the new layout. */
export function applyWindowLayoutOp(layout: WindowLayoutNode, op: WindowLayoutOp): WindowLayoutNode {
  switch (op.op) {
    case "setActive": return setActiveWindow(layout, op.windowId);
    case "closeWindow": return removeWindow(layout, op.windowId) ?? emptyWindowStack();
    case "reorderTab": return reorderTab(layout, op.stackPath, op.from, op.to);
    case "setCorner": return setWindowCorner(layout, op.windowId, op.corner);
    case "applySizes": return applySplitSizes(layout, op.splitPath, op.sizes);
    case "reconcile": return reconcileWindows(layout, op.windowIds);
    case "dropWindow": return dropWindow(layout, op.window, op.zone);
    case "dropStack": return dropStack(layout, op.sourcePath, op.zone);
  }
}

/** ⚖️ Even row of single-window stacks (one stack for one window), each sized `1/n` and active on its window. */
export function evenWindowLayout(windowIds: readonly string[]): WindowLayoutPanel {
  const stack = (id: string, size?: number): WindowLayoutStackNode => ({ kind: "stack", size, activeWindowId: id, children: [{ kind: "window", windowKindId: id }] });
  if (windowIds.length === 0) return emptyWindowStack();
  if (windowIds.length === 1) return stack(windowIds[0]!);
  return { kind: "split", axis: "horizontal", children: windowIds.map((id) => stack(id, 1 / windowIds.length)) };
}

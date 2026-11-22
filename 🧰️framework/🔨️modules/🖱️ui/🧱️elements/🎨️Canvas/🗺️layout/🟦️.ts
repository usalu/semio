/** 🗺️ Canvas-only views of the contract window layout (`@semio-tech/ui-layout`): DOM path keys, the mode layout resolution, the window lookup and the resizable boundary conversions (fractions to percent). */
import { type WindowLayoutNode, type WindowLayoutPanel, type WindowLayoutPath, type WindowLayoutStackNode, type WindowLayoutWindowNode, type WindowStackCorner, applySplitSizes, evenWindowLayout, mapLayoutStacks, readLayoutAt, reconcileWindows, stackPathOf, windowCornerOf, windowIdOf } from "@semio-tech/ui-layout";
import type { ModeWindowDescriptor } from "../🪟️template-drag/🟦️.ts";

/** 🔑️ Stable string identity of a layout path for maps, React keys, DOM ids and resizable panel ids. */
export const modePathKey = (path: WindowLayoutPath): string => path.join(".");
export const modePathFromKey = (key: string): WindowLayoutPath => (key ? key.split(".").map(Number) : []);
export const modePathEquals = (left: WindowLayoutPath | null, right: WindowLayoutPath | null): boolean => left !== null && right !== null && modePathKey(left) === modePathKey(right);

/** 🧭️ Resolves the mode layout: the given layout or an even one, reconciled onto the declared windows. */
export function resolveModeLayout(windows: readonly ModeWindowDescriptor[], layout?: WindowLayoutNode): WindowLayoutPanel {
  const ids = windows.map((window) => window.id);
  return reconcileWindows(layout ?? evenWindowLayout(ids), ids);
}

/** 🔎️ Every window node of a layout in document order. */
export function collectModeWindows(layout: WindowLayoutNode): WindowLayoutWindowNode[] {
  const windows: WindowLayoutWindowNode[] = [];
  mapLayoutStacks(layout, (stack) => {
    windows.push(...stack.children);
    return stack;
  });
  return layout.kind === "window" ? [layout] : windows;
}

/** 🔎️ The window node with `windowId` anywhere in the layout. */
export const findModeWindow = (layout: WindowLayoutNode, windowId: string): WindowLayoutWindowNode | undefined => collectModeWindows(layout).find((window) => windowIdOf(window) === windowId);

/** 🧱️ The stack at `path`, when it is one. */
export function readModeStack(layout: WindowLayoutNode, path: WindowLayoutPath): WindowLayoutStackNode | null {
  const node = readLayoutAt(layout, path);
  return node?.kind === "stack" ? node : null;
}

/** 🧭️ The path of the stack that holds `windowId`. */
export const modeStackPathOf = (layout: WindowLayoutNode, windowId: string): WindowLayoutPath | null => stackPathOf(layout, windowId);

/** 🧭️ Groups stack window children by chrome corner while preserving relative order. */
export function modeStackTabsByCorner(children: readonly WindowLayoutWindowNode[]): Record<WindowStackCorner, WindowLayoutWindowNode[]> {
  const groups = { topLeft: [], topRight: [], bottomLeft: [], bottomRight: [] } as Record<WindowStackCorner, WindowLayoutWindowNode[]>;
  for (const child of children) groups[windowCornerOf(child)].push(child);
  return groups;
}

/** 📝️ Writes resizable panel percentages (an array or a record keyed by child path key) back onto a split as fractions. */
export function applyModeSplitPercents(layout: WindowLayoutNode, splitPath: WindowLayoutPath, percents: Record<string, number> | readonly number[]): WindowLayoutNode {
  const split = readLayoutAt(layout, splitPath);
  if (!split || split.kind !== "split") return layout;
  const count = split.children.length;
  const resolved = (() => {
    if (Array.isArray(percents)) return percents.length === count ? (percents as readonly number[]) : null;
    const record = percents as Record<string | number, number>;
    const matched = split.children.filter((_, index) => typeof record[modePathKey([...splitPath, index])] === "number").length;
    if (matched > 0 && matched !== count) return null;
    return split.children.map((child, index) => record[modePathKey([...splitPath, index])] ?? record[index] ?? record[String(index)] ?? (child.size === undefined ? undefined : child.size * 100));
  })();
  if (!resolved) return applySplitSizes(layout, splitPath, []);
  return applySplitSizes(layout, splitPath, resolved.map((percent) => (percent === undefined ? 1 / count : percent / 100)));
}

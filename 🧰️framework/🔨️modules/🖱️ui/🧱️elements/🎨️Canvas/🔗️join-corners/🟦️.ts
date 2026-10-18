import type { WindowLayoutNode } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import type { ResizableJoinCornerSpec, ResizableJoinEdgeSide } from "../../↔️Resizable/🟦️.tsx";
import { type ModeLayoutPath, modeJoinPath, readLayoutAtPath, updateLayoutAtPath } from "../🗺️layout/🟦️.ts";

/** ↔ True when a child axis runs perpendicular to its parent axis. */
export function modeAxisIsPerpendicularChild(parentKind: "row" | "column", child: WindowLayoutNode): boolean {
  return (parentKind === "row" && child.kind === "column") || (parentKind === "column" && child.kind === "row");
}

/** ↔ Separator indices and size-weighted fractions for joins inside a perpendicular child axis. */
export function modePerpendicularJoinSeparators(node: WindowLayoutNode): readonly { index: number; fraction: number }[] {
  if (node.kind !== "row" && node.kind !== "column") return [];
  const count = node.children.length;
  if (count < 2) return [];
  const sizes = node.children.map((child) => child.size ?? 100 / count);
  const total = sizes.reduce((sum, size) => sum + size, 0);
  if (total <= 0) return [];
  let cumulative = 0;
  const joins: { index: number; fraction: number }[] = [];
  for (let index = 1; index < count; index += 1) {
    cumulative += sizes[index - 1]!;
    joins.push({ index, fraction: cumulative / total });
  }
  return joins;
}

/** ↔ Corner join specs for a main-axis separator that crosses perpendicular child splits. */
export function modeJoinCornerSpecsForSeparator(parentPath: ModeLayoutPath, parentKind: "row" | "column", separatorIndex: number, prevChild: WindowLayoutNode, nextChild: WindowLayoutNode): ResizableJoinCornerSpec[] {
  const specs: ResizableJoinCornerSpec[] = [];
  const pushSpecs = (beforeSide: boolean, crossPath: ModeLayoutPath, crossChild: WindowLayoutNode) => {
    const joins = modePerpendicularJoinSeparators(crossChild);
    joins.forEach(({ index, fraction }) => {
      specs.push({
        parentKind,
        mainAxisPath: parentPath,
        mainSeparatorIndex: separatorIndex,
        crossAxisPath: crossPath,
        crossSeparatorIndex: index,
        edgeSide: beforeSide ? "leading" : "trailing",
        alongFraction: fraction,
      });
    });
  };
  if (modeAxisIsPerpendicularChild(parentKind, prevChild)) {
    pushSpecs(true, modeJoinPath(parentPath, separatorIndex - 1), prevChild);
  }
  if (modeAxisIsPerpendicularChild(parentKind, nextChild)) {
    pushSpecs(false, modeJoinPath(parentPath, separatorIndex), nextChild);
  }
  return specs;
}

/** ↔ Corner join specs on a cross-axis separator where it meets a parent split. */
export function modeJoinCornerSpecsForCrossSeparator(crossPath: ModeLayoutPath, crossKind: "row" | "column", crossSeparatorIndex: number, parent: { path: ModeLayoutPath; kind: "row" | "column"; panelIndex: number }): ResizableJoinCornerSpec[] {
  if ((parent.kind === "row" && crossKind !== "column") || (parent.kind === "column" && crossKind !== "row")) return [];
  const parentSeparatorIndex = parent.panelIndex === 0 ? parent.panelIndex + 1 : parent.panelIndex;
  const edgeSide: ResizableJoinEdgeSide = parent.panelIndex === 0 ? "trailing" : "leading";
  const alongFraction = parent.panelIndex === 0 ? 1 : 0;
  return [
    {
      parentKind: parent.kind,
      mainAxisPath: parent.path,
      mainSeparatorIndex: parentSeparatorIndex,
      crossAxisPath: crossPath,
      crossSeparatorIndex,
      edgeSide,
      alongFraction,
    },
  ];
}

/** ↔ One perpendicular axis that participates in a corner join. */
export type ModeJoinCornerCrossAxis = {
  path: ModeLayoutPath;
  separatorIndex: number;
};

/** ↔ Max fraction delta for two perpendicular joins to count as the same touching corner. */
export const MODE_JOIN_CORNER_TOUCH_EPS = 0.0025;

/** ↔ Every perpendicular axis whose join actually touches this corner (same main separator + size fraction), including the dragged cross axis. */
export function resolveJoinCornerPeerCrossAxes(layout: WindowLayoutNode, spec: ResizableJoinCornerSpec): ModeJoinCornerCrossAxis[] {
  const fallback: ModeJoinCornerCrossAxis[] = [{ path: spec.crossAxisPath, separatorIndex: spec.crossSeparatorIndex }];
  const mainNode = spec.mainAxisPath ? readLayoutAtPath(layout, spec.mainAxisPath) : layout;
  if (!mainNode || (mainNode.kind !== "row" && mainNode.kind !== "column")) return fallback;
  const prevChild = mainNode.children[spec.mainSeparatorIndex - 1];
  const nextChild = mainNode.children[spec.mainSeparatorIndex];
  if (!prevChild || !nextChild) return fallback;
  const crossNode = readLayoutAtPath(layout, spec.crossAxisPath);
  const ownJoin = crossNode ? modePerpendicularJoinSeparators(crossNode).find((join) => join.index === spec.crossSeparatorIndex) : undefined;
  const targetFraction = ownJoin?.fraction ?? spec.alongFraction;
  const peers: ModeJoinCornerCrossAxis[] = [];
  const consider = (crossPath: ModeLayoutPath, crossChild: WindowLayoutNode) => {
    if (!modeAxisIsPerpendicularChild(spec.parentKind, crossChild)) return;
    for (const join of modePerpendicularJoinSeparators(crossChild)) {
      if (Math.abs(join.fraction - targetFraction) > MODE_JOIN_CORNER_TOUCH_EPS) continue;
      peers.push({ path: crossPath, separatorIndex: join.index });
    }
  };
  consider(modeJoinPath(spec.mainAxisPath, spec.mainSeparatorIndex - 1), prevChild);
  consider(modeJoinPath(spec.mainAxisPath, spec.mainSeparatorIndex), nextChild);
  return peers.length > 0 ? peers : fallback;
}

/** ↔ Percentage delta for one panel pair on an axis separator. */
export function applyAxisResizeDelta(layout: WindowLayoutNode, axisPath: ModeLayoutPath, separatorIndex: number, deltaPct: number, minPct = 8): WindowLayoutNode {
  if (Math.abs(deltaPct) < 0.001) return layout;
  return updateLayoutAtPath(layout, axisPath, (node) => {
    if (node.kind !== "row" && node.kind !== "column") return node;
    const leftIndex = separatorIndex - 1;
    const rightIndex = separatorIndex;
    if (leftIndex < 0 || rightIndex >= node.children.length) return node;
    const count = node.children.length;
    const leftSize = node.children[leftIndex]?.size ?? 100 / count;
    const rightSize = node.children[rightIndex]?.size ?? 100 / count;
    const pairSize = leftSize + rightSize;
    const nextLeft = Math.max(minPct, Math.min(pairSize - minPct, leftSize + deltaPct));
    const nextRight = pairSize - nextLeft;
    const children = node.children.map((child, index) => {
      if (index === leftIndex) return { ...child, size: nextLeft };
      if (index === rightIndex) return { ...child, size: nextRight };
      return child;
    });
    return { ...node, children };
  });
}

/** ↔ Pointer delta for a corner join on perpendicular row/column axes. */
export function resolveJoinCornerResizeDeltas(parentKind: "row" | "column", deltaXPx: number, deltaYPx: number, mainAxisPixelSize: number, crossAxisPixelSize: number): { mainDeltaPct: number; crossDeltaPct: number } {
  if (mainAxisPixelSize <= 0 || crossAxisPixelSize <= 0) return { mainDeltaPct: 0, crossDeltaPct: 0 };
  if (parentKind === "row") {
    return {
      mainDeltaPct: (deltaXPx / mainAxisPixelSize) * 100,
      crossDeltaPct: (deltaYPx / crossAxisPixelSize) * 100,
    };
  }
  return {
    mainDeltaPct: (deltaYPx / mainAxisPixelSize) * 100,
    crossDeltaPct: (deltaXPx / crossAxisPixelSize) * 100,
  };
}

/** ↔ Applies a separator percentage delta to a live resizable group layout. */
export function applyAxisGroupLayoutDelta(layout: Record<string, number>, axisPath: ModeLayoutPath, separatorIndex: number, deltaPct: number, minPct = 8): Record<string, number> {
  const leadingId = modeJoinPath(axisPath, separatorIndex - 1);
  const trailingId = modeJoinPath(axisPath, separatorIndex);
  const leadingSize = layout[leadingId];
  const trailingSize = layout[trailingId];
  if (leadingSize === undefined || trailingSize === undefined) return layout;
  const pairSize = leadingSize + trailingSize;
  const nextLeading = Math.max(minPct, Math.min(pairSize - minPct, leadingSize + deltaPct));
  return {
    ...layout,
    [leadingId]: nextLeading,
    [trailingId]: pairSize - nextLeading,
  };
}

/** ↔ Resolves persisted percentages for one resizable axis. */
export function modeAxisGroupLayout(layout: WindowLayoutNode, axisPath: ModeLayoutPath): Record<string, number> {
  const node = axisPath ? readLayoutAtPath(layout, axisPath) : layout;
  if (!node || (node.kind !== "row" && node.kind !== "column")) return {};
  return Object.fromEntries(node.children.map((child, index) => [modeJoinPath(axisPath, index), child.size ?? 100 / node.children.length]));
}

/** ↔ Applies a corner grab delta to the main axis and every peer cross axis at the join. */
export function applyModeJoinCornerResize(layout: WindowLayoutNode, spec: ResizableJoinCornerSpec, deltaXPx: number, deltaYPx: number, mainAxisPixelSize: number, crossAxisPixelSize: number): WindowLayoutNode {
  const { mainDeltaPct, crossDeltaPct } = resolveJoinCornerResizeDeltas(spec.parentKind, deltaXPx, deltaYPx, mainAxisPixelSize, crossAxisPixelSize);
  let next = applyAxisResizeDelta(layout, spec.mainAxisPath, spec.mainSeparatorIndex, mainDeltaPct);
  for (const peer of resolveJoinCornerPeerCrossAxes(layout, spec)) {
    next = applyAxisResizeDelta(next, peer.path, peer.separatorIndex, crossDeltaPct);
  }
  return next;
}

export function readModeAxisPixelSize(element: HTMLElement | null | undefined, kind: "row" | "column"): number {
  if (!element) return 0;
  const rect = element.getBoundingClientRect();
  return kind === "row" ? rect.width : rect.height;
}

export function readModeAxisKind(layout: WindowLayoutNode, axisPath: ModeLayoutPath): "row" | "column" | null {
  const node = axisPath ? readLayoutAtPath(layout, axisPath) : layout;
  if (!node || (node.kind !== "row" && node.kind !== "column")) return null;
  return node.kind;
}

import { type WindowLayoutAxis, type WindowLayoutNode, type WindowLayoutPath, applySplitSizes, readLayoutAt } from "@semio-tech/ui-layout";
import type { ResizableGroupHandle, ResizableJoinCornerSpec, ResizableJoinEdgeSide } from "../../↔️Resizable/🟦️.tsx";
import { modePathFromKey, modePathKey } from "../🗺️layout/🟦️.ts";

const joinKey = (parent: WindowLayoutPath, index: number): string => modePathKey([...parent, index]);

/** ↔ True when a child axis runs perpendicular to its parent axis. */
export function modeAxisIsPerpendicularChild(parentAxis: WindowLayoutAxis, child: WindowLayoutNode): boolean {
  return child.kind === "split" && child.axis !== parentAxis;
}

/** 📏️ Separator indices and size-weighted fractions for joins inside a perpendicular child axis. */
export function modePerpendicularJoinSeparators(node: WindowLayoutNode): readonly { index: number; fraction: number }[] {
  if (node.kind !== "split") return [];
  const count = node.children.length;
  if (count < 2) return [];
  const sizes = node.children.map((child) => child.size ?? 1 / count);
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

/** 📕️ Corner join specs for a main-axis separator that crosses perpendicular child splits. */
export function modeJoinCornerSpecsForSeparator(parentPath: WindowLayoutPath, parentAxis: WindowLayoutAxis, separatorIndex: number, prevChild: WindowLayoutNode, nextChild: WindowLayoutNode): ResizableJoinCornerSpec[] {
  const specs: ResizableJoinCornerSpec[] = [];
  const pushSpecs = (beforeSide: boolean, crossPath: WindowLayoutPath, crossChild: WindowLayoutNode) => {
    const joins = modePerpendicularJoinSeparators(crossChild);
    joins.forEach(({ index, fraction }) => {
      specs.push({
        parentAxis,
        mainAxisPath: modePathKey(parentPath),
        mainSeparatorIndex: separatorIndex,
        crossAxisPath: modePathKey(crossPath),
        crossSeparatorIndex: index,
        edgeSide: beforeSide ? "leading" : "trailing",
        alongFraction: fraction,
      });
    });
  };
  if (modeAxisIsPerpendicularChild(parentAxis, prevChild)) {
    pushSpecs(true, [...parentPath, separatorIndex - 1], prevChild);
  }
  if (modeAxisIsPerpendicularChild(parentAxis, nextChild)) {
    pushSpecs(false, [...parentPath, separatorIndex], nextChild);
  }
  return specs;
}

/** 📖️ Corner join specs on a cross-axis separator where it meets a parent split. */
export function modeJoinCornerSpecsForCrossSeparator(crossPath: WindowLayoutPath, crossAxis: WindowLayoutAxis, crossSeparatorIndex: number, parent: { path: WindowLayoutPath; axis: WindowLayoutAxis; panelIndex: number }): ResizableJoinCornerSpec[] {
  if (parent.axis === crossAxis) return [];
  const parentSeparatorIndex = parent.panelIndex === 0 ? parent.panelIndex + 1 : parent.panelIndex;
  const edgeSide: ResizableJoinEdgeSide = parent.panelIndex === 0 ? "trailing" : "leading";
  const alongFraction = parent.panelIndex === 0 ? 1 : 0;
  return [
    {
      parentAxis: parent.axis,
      mainAxisPath: modePathKey(parent.path),
      mainSeparatorIndex: parentSeparatorIndex,
      crossAxisPath: modePathKey(crossPath),
      crossSeparatorIndex,
      edgeSide,
      alongFraction,
    },
  ];
}

/** ✖️ One perpendicular axis that participates in a corner join. */
export type ModeJoinCornerCrossAxis = {
  path: WindowLayoutPath;
  separatorIndex: number;
};

/** 🔬️ Max fraction delta for two perpendicular joins to count as the same touching corner. */
export const MODE_JOIN_CORNER_TOUCH_EPS = 0.0025;

/** 👥️ Every perpendicular axis whose join actually touches this corner (same main separator + size fraction), including the dragged cross axis. */
export function resolveJoinCornerPeerCrossAxes(layout: WindowLayoutNode, spec: ResizableJoinCornerSpec): ModeJoinCornerCrossAxis[] {
  const mainPath = modePathFromKey(spec.mainAxisPath);
  const crossPath = modePathFromKey(spec.crossAxisPath);
  const fallback: ModeJoinCornerCrossAxis[] = [{ path: crossPath, separatorIndex: spec.crossSeparatorIndex }];
  const mainNode = readLayoutAt(layout, mainPath);
  if (!mainNode || mainNode.kind !== "split") return fallback;
  const prevChild = mainNode.children[spec.mainSeparatorIndex - 1];
  const nextChild = mainNode.children[spec.mainSeparatorIndex];
  if (!prevChild || !nextChild) return fallback;
  const crossNode = readLayoutAt(layout, crossPath);
  const ownJoin = crossNode ? modePerpendicularJoinSeparators(crossNode).find((join) => join.index === spec.crossSeparatorIndex) : undefined;
  const targetFraction = ownJoin?.fraction ?? spec.alongFraction;
  const peers: ModeJoinCornerCrossAxis[] = [];
  const consider = (peerPath: WindowLayoutPath, crossChild: WindowLayoutNode) => {
    if (!modeAxisIsPerpendicularChild(spec.parentAxis, crossChild)) return;
    for (const join of modePerpendicularJoinSeparators(crossChild)) {
      if (Math.abs(join.fraction - targetFraction) > MODE_JOIN_CORNER_TOUCH_EPS) continue;
      peers.push({ path: peerPath, separatorIndex: join.index });
    }
  };
  consider([...mainPath, spec.mainSeparatorIndex - 1], prevChild);
  consider([...mainPath, spec.mainSeparatorIndex], nextChild);
  return peers.length > 0 ? peers : fallback;
}

/** 🥢️ Percentage delta for one panel pair on a split separator, applied through the contract `applySplitSizes`. */
export function applyAxisResizeDelta(layout: WindowLayoutNode, axisPath: WindowLayoutPath, separatorIndex: number, deltaPct: number, minPct = 8): WindowLayoutNode {
  if (Math.abs(deltaPct) < 0.001) return layout;
  const node = readLayoutAt(layout, axisPath);
  if (!node || node.kind !== "split") return layout;
  const leftIndex = separatorIndex - 1;
  const rightIndex = separatorIndex;
  if (leftIndex < 0 || rightIndex >= node.children.length) return layout;
  const count = node.children.length;
  const leftSize = node.children[leftIndex]?.size ?? 1 / count;
  const rightSize = node.children[rightIndex]?.size ?? 1 / count;
  const pairSize = leftSize + rightSize;
  const min = minPct / 100;
  const nextLeft = Math.max(min, Math.min(pairSize - min, leftSize + deltaPct / 100));
  return applySplitSizes(
    layout,
    axisPath,
    node.children.map((child, index) => (index === leftIndex ? nextLeft : index === rightIndex ? pairSize - nextLeft : (child.size ?? 1 / count))),
  );
}

/** 🖱️ Pointer delta for a corner join on perpendicular row/column axes. */
export function resolveJoinCornerResizeDeltas(parentAxis: WindowLayoutAxis, deltaXPx: number, deltaYPx: number, mainAxisPixelSize: number, crossAxisPixelSize: number): { mainDeltaPct: number; crossDeltaPct: number } {
  if (mainAxisPixelSize <= 0 || crossAxisPixelSize <= 0) return { mainDeltaPct: 0, crossDeltaPct: 0 };
  if (parentAxis === "horizontal") {
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

/** 📐️ Applies a separator percentage delta to a live resizable group layout. */
export function applyAxisGroupLayoutDelta(layout: Record<string, number>, axisPath: WindowLayoutPath, separatorIndex: number, deltaPct: number, minPct = 8): Record<string, number> {
  const leadingId = joinKey(axisPath, separatorIndex - 1);
  const trailingId = joinKey(axisPath, separatorIndex);
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

/** 🗂️ Resolves persisted percentages for one resizable split, keyed by child path key. */
export function modeAxisGroupLayout(layout: WindowLayoutNode, axisPath: WindowLayoutPath): Record<string, number> {
  const node = readLayoutAt(layout, axisPath);
  if (!node || node.kind !== "split") return {};
  return Object.fromEntries(node.children.map((child, index) => [joinKey(axisPath, index), (child.size ?? 1 / node.children.length) * 100]));
}

/** 🏁️ Applies a corner grab delta to the main axis and every peer cross axis at the join. */
export function applyModeJoinCornerResize(layout: WindowLayoutNode, spec: ResizableJoinCornerSpec, deltaXPx: number, deltaYPx: number, mainAxisPixelSize: number, crossAxisPixelSize: number): WindowLayoutNode {
  const { mainDeltaPct, crossDeltaPct } = resolveJoinCornerResizeDeltas(spec.parentAxis, deltaXPx, deltaYPx, mainAxisPixelSize, crossAxisPixelSize);
  let next = applyAxisResizeDelta(layout, modePathFromKey(spec.mainAxisPath), spec.mainSeparatorIndex, mainDeltaPct);
  for (const peer of resolveJoinCornerPeerCrossAxes(layout, spec)) {
    next = applyAxisResizeDelta(next, peer.path, peer.separatorIndex, crossDeltaPct);
  }
  return next;
}

export function readModeAxisPixelSize(element: HTMLElement | null | undefined, axis: WindowLayoutAxis): number {
  if (!element) return 0;
  const rect = element.getBoundingClientRect();
  return axis === "horizontal" ? rect.width : rect.height;
}

export function readModeAxis(layout: WindowLayoutNode, axisPath: WindowLayoutPath): WindowLayoutAxis | null {
  const node = readLayoutAt(layout, axisPath);
  return node?.kind === "split" ? node.axis : null;
}

/** 📝️ Writes live resizable group percentages back, ignoring a group that already disposed. */
export function safePanelGroupSetLayout(group: ResizableGroupHandle, layout: Record<string, number>): void {
  try {
    group.setLayout(layout);
  } catch {}
}

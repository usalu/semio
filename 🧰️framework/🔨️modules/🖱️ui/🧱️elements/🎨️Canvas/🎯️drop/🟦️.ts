import type { WindowLayoutNode, WindowStackCorner } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import type { IconName } from "@semio-tech/assets";
import type { ModeCanvasDropTarget } from "../🪟️template-drag/🟦️.ts";
import { WINDOW_STACK_CORNERS, extractStackFromLayout, insertWindowAsTabAtCorner, mergeStackTabsIntoStack, splitRootWithStack, splitRootWithWindow, splitWithStack, splitWithWindow } from "../📐️corners/🟦️.ts";
import { type ModeDockSide, type ModeLayoutPath, readLayoutAtPath, removeWindowFromLayout } from "../🗺️layout/🟦️.ts";

/** 🪟️ Inserts a new window leaf at a dock drop target (external template drag). */
export function insertWindowAtDropZone(layout: WindowLayoutNode, windowId: string, target: ModeCanvasDropTarget): WindowLayoutNode {
  if (target.kind === "root-split") return splitRootWithWindow(layout, windowId, target.side as ModeDockSide);
  if (target.kind === "split") return splitWithWindow(layout, target.stackPath, windowId, target.side as ModeDockSide);
  return insertWindowAsTabAtCorner(layout, target.stackPath, windowId, target.corner ?? "topLeft", target.index < 0 ? undefined : target.index);
}

export type ModeDropZone = { kind: "tab"; stackPath: ModeLayoutPath; corner: WindowStackCorner; index: number } | { kind: "split"; stackPath: ModeLayoutPath; side: ModeDockSide } | { kind: "root-split"; side: ModeDockSide };

type ModeDragKind = "tab" | "stack";

export interface ModeDragState {
  dragKind: ModeDragKind;
  windowId: string;
  stackPath: ModeLayoutPath;
  tabIndex: number;
  pointerId: number;
  ghostLabel: string;
  x: number;
  y: number;
}

export interface ModePendingDrag {
  dragKind: ModeDragKind;
  windowId: string;
  stackPath: ModeLayoutPath;
  tabIndex: number;
  pointerId: number;
  ghostLabel: string;
  startX: number;
  startY: number;
}

export interface ModeStackDropTargets {
  corners: Partial<Record<WindowStackCorner, { rect: DOMRect; element: HTMLElement }>>;
  body: DOMRect | null;
}

function listModeDockTabElements(tabBarElement: HTMLElement | null): HTMLElement[] {
  if (!tabBarElement) return [];
  return [...tabBarElement.querySelectorAll<HTMLElement>('[data-slot="mode-dock-tab"]')].filter((tab) => tab.getAttribute("data-drag-source") !== "true");
}

function computeTabInsertIndex(pointerX: number, tabBarElement: HTMLElement | null): number {
  const tabs = listModeDockTabElements(tabBarElement);
  for (let index = 0; index < tabs.length; index++) {
    const tabRect = tabs[index]!.getBoundingClientRect();
    if (pointerX < tabRect.left + tabRect.width / 2) return index;
  }
  return tabs.length;
}

/** 📍️ Resolves tab-bar insertion line and slot preview geometry for drag feedback. */
export function computeTabInsertPreview(tabBarElement: HTMLElement | null, insertIndex: number): { insertX: number; top: number; height: number; slotLeft: number; slotWidth: number } | null {
  if (!tabBarElement) return null;
  const tabBarRect = tabBarElement.getBoundingClientRect();
  const tabRects = listModeDockTabElements(tabBarElement).map((tab) => tab.getBoundingClientRect());
  const top = tabBarRect.top;
  const height = tabBarRect.height;
  const defaultWidth = tabRects[0]?.width ?? 96;
  const resolvedIndex = insertIndex < 0 ? tabRects.length : insertIndex;

  if (tabRects.length === 0) {
    return { insertX: tabBarRect.left + 6, top, height, slotLeft: tabBarRect.left + 4, slotWidth: defaultWidth };
  }
  if (resolvedIndex <= 0) {
    return { insertX: tabRects[0]!.left, top, height, slotLeft: tabRects[0]!.left, slotWidth: defaultWidth };
  }
  if (resolvedIndex >= tabRects.length) {
    const last = tabRects[tabRects.length - 1]!;
    return { insertX: last.right, top, height, slotLeft: last.right, slotWidth: defaultWidth };
  }
  const prev = tabRects[resolvedIndex - 1]!;
  const next = tabRects[resolvedIndex]!;
  const insertX = (prev.right + next.left) / 2;
  return { insertX, top, height, slotLeft: insertX - defaultWidth / 2, slotWidth: defaultWidth };
}

function pointerInRect(x: number, y: number, rect: DOMRect): boolean {
  return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom;
}

/** 🧭️ Maps pointer position in a rectangle to a split side using half-panel zones (dominant axis from center). */
export function resolveModeSplitSideInBody(localX: number, localY: number, bodyWidth: number, bodyHeight: number): ModeDockSide {
  const midX = bodyWidth / 2;
  const midY = bodyHeight / 2;
  const dx = Math.abs(localX - midX);
  const dy = Math.abs(localY - midY);
  if (dx >= dy) return localX < midX ? "left" : "right";
  return localY < midY ? "top" : "bottom";
}

/** 📐️ Half-panel rectangle for split drop preview inside a stack body (origin top-left of body). */
export function computeModeSplitPreviewInBody(bodyWidth: number, bodyHeight: number, side: ModeDockSide): { left: number; top: number; width: number; height: number } {
  const halfWidth = bodyWidth / 2;
  const halfHeight = bodyHeight / 2;
  if (side === "left") return { left: 0, top: 0, width: halfWidth, height: bodyHeight };
  if (side === "right") return { left: bodyWidth - halfWidth, top: 0, width: halfWidth, height: bodyHeight };
  if (side === "top") return { left: 0, top: 0, width: bodyWidth, height: halfHeight };
  return { left: 0, top: bodyHeight - halfHeight, width: bodyWidth, height: halfHeight };
}

export function computeModeDropZone(pointerX: number, pointerY: number, stackTargets: ReadonlyMap<ModeLayoutPath, ModeStackDropTargets>, modeRect: DOMRect | null): ModeDropZone | null {
  for (const [stackPath, targets] of stackTargets) {
    for (const corner of WINDOW_STACK_CORNERS) {
      const hit = targets.corners[corner];
      if (!hit || !pointerInRect(pointerX, pointerY, hit.rect)) continue;
      return { kind: "tab", stackPath, corner, index: computeTabInsertIndex(pointerX, hit.element) };
    }
  }
  for (const [stackPath, targets] of stackTargets) {
    const rect = targets.body;
    if (!rect || !pointerInRect(pointerX, pointerY, rect)) continue;
    const side = resolveModeSplitSideInBody(pointerX - rect.left, pointerY - rect.top, rect.width, rect.height);
    return { kind: "split", stackPath, side };
  }
  if (!modeRect || !pointerInRect(pointerX, pointerY, modeRect)) return null;
  const side = resolveModeSplitSideInBody(pointerX - modeRect.left, pointerY - modeRect.top, modeRect.width, modeRect.height);
  return { kind: "root-split", side };
}

export function applyModeDrop(layout: WindowLayoutNode, drag: ModeDragState, zone: ModeDropZone): WindowLayoutNode {
  const { dragKind, windowId, stackPath: sourcePath } = drag;
  if (dragKind === "stack") {
    const { layout: withoutSource, stack } = extractStackFromLayout(layout, sourcePath);
    if (!stack) return layout;
    const base = withoutSource ?? { kind: "stack", children: [] };
    if (zone.kind === "root-split") return splitRootWithStack(base, stack, zone.side);
    if (readLayoutAtPath(base, zone.stackPath)?.kind !== "stack") return layout;
    if (zone.kind === "split") return splitWithStack(base, zone.stackPath, stack, zone.side);
    return mergeStackTabsIntoStack(base, zone.stackPath, stack, zone.index, zone.corner);
  }
  if (zone.kind === "root-split") return splitRootWithWindow(layout, windowId, zone.side);
  const without = removeWindowFromLayout(layout, windowId);
  if (!without || readLayoutAtPath(without, zone.stackPath)?.kind !== "stack") return layout;
  if (zone.kind === "split") return splitWithWindow(layout, zone.stackPath, windowId, zone.side);
  return insertWindowAsTabAtCorner(without, zone.stackPath, windowId, zone.corner, zone.index < 0 ? undefined : zone.index);
}

/** 🪓️ Removes the dragged tab or stack from the committed layout while it floats on the cursor. */
export function modeDockOutLayout(committed: WindowLayoutNode, drag: Pick<ModeDragState, "dragKind" | "windowId" | "stackPath">): WindowLayoutNode {
  if (drag.dragKind === "stack") {
    const { layout } = extractStackFromLayout(committed, drag.stackPath);
    return layout ?? { kind: "stack", children: [] };
  }
  return removeWindowFromLayout(committed, drag.windowId) ?? committed;
}

export interface ModeTabInsertPreview {
  stackPath: ModeLayoutPath;
  corner: WindowStackCorner;
  index: number;
}

type ModeDockTabDisplayItem = { id: string; title: string; iconId: IconName; preview?: "ghost" };

/** 📑️ Tab bar row with ghost tab(s) at the drop index so layout matches the committed drop. */
export function modeDockTabsWithInsertPreview(
  tabs: readonly { id: string; title: string; iconId: IconName }[],
  insertPreview: ModeTabInsertPreview | null,
  stackPath: ModeLayoutPath,
  corner: WindowStackCorner,
  ghostTabs: readonly { id: string; title: string; iconId: IconName }[],
): ModeDockTabDisplayItem[] {
  if (!insertPreview || insertPreview.stackPath !== stackPath || insertPreview.corner !== corner || ghostTabs.length === 0) return tabs.map((tab) => ({ ...tab }));
  const insertAt = Math.min(Math.max(0, insertPreview.index), tabs.length);
  const row: ModeDockTabDisplayItem[] = tabs.map((tab) => ({ ...tab }));
  row.splice(insertAt, 0, ...ghostTabs.map((tab) => ({ id: tab.id, title: tab.title, iconId: tab.iconId, preview: "ghost" as const })));
  return row;
}

/** 👻️ Tab descriptors shown as insert-preview ghosts for the current drag. */
export function modeDockDragInsertTabs(layout: WindowLayoutNode, drag: ModeDragState, windowTitle: (windowId: string) => string, windowIconId: (windowId: string) => IconName): readonly { id: string; title: string; iconId: IconName }[] {
  if (drag.dragKind === "tab") return [{ id: drag.windowId, title: windowTitle(drag.windowId), iconId: windowIconId(drag.windowId) }];
  const stack = readLayoutAtPath(layout, drag.stackPath);
  if (!stack || stack.kind !== "stack") return [{ id: drag.windowId, title: windowTitle(drag.windowId), iconId: windowIconId(drag.windowId) }];
  return stack.children.map((child) => ({ id: child.id, title: windowTitle(child.id), iconId: windowIconId(child.id) }));
}

export function resolveModeTabInsertPreview(drag: ModeDragState | null, zone: ModeDropZone | null): ModeTabInsertPreview | null {
  if (!drag || zone?.kind !== "tab") return null;
  if (drag.dragKind === "stack" && zone.stackPath === drag.stackPath) return null;
  return { stackPath: zone.stackPath, corner: zone.corner, index: zone.index };
}

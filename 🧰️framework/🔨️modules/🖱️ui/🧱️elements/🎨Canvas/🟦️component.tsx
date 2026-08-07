// #region 🧲️Header
// 💻️ framework/ui/elements/🎨Canvas/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework-core";
import * as React from "react";
import * as ResizablePrimitive from "react-resizable-panels";
import { type IconName } from "@semio-tech/assets";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext/.forwardRef at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { CanvasSkeleton } from "../🦴Skeletons/🟦️component.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../☑️Select/🟦️component.tsx";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup, type ResizableJoinCornerResizeHandler, type ResizableJoinCornerSpec, type ResizableJoinEdgeSide } from "../↕️Resizable/🟦️component.tsx";
import { Navbar, type NavbarItem } from "../🔝Navbar/🟦️component.tsx";
import { Window, type WindowConfig } from "../🪟Window/🟦️component.tsx";
import { type TreeDragAndDropController } from "../🪵Tree/🟦️component.tsx";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { interactiveHoverClass, interactiveActiveFillClass, glassClass, surfaceClass, shellFloorFillClass } from "../🏷️ClassNames/🟦️component.tsx";
import { chromeStatusBorderClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { modeDockTabClassName } from "../🏷️ClassNames/🟦️component.tsx";
import { useLabel, resolveTranslationLabel, useUiTranslation } from "../🏷️Label/🟦️component.tsx";
import { useShellKeydown, useShellScopeOptional, NULL_SHELL_ROOT_REF } from "../🐚️ShellScope/🟦️component.tsx";
import { isSurfaceActiveBackgroundPointer, useSurface, useSurfaceActive, LevelProvider, SurfaceScope } from "../🌈️Surface/🟦️component.tsx";
import { createEvenWindowLayout, focusActiveSearchInput, modeDockChromeGridPlacement, routeWindowSearchEscape, routeWindowSearchKeydown, routeWindowSearchSpace, setSurfaceActiveRoot, WindowChromeSilhouetteBorder, dropZoneReadyClass, modeDockTabLabelClassName, modeDockActiveTabClass, modeDockActiveTabFillClass, modeDockInactiveTabBeforeGapClass, modeDockInactiveTabClass, windowBodyFrameActiveClass, windowBodyFrameClass, windowCapFrameActiveClass, windowCapFrameClass, windowControlsCapActiveClass, windowControlsCapActiveSplitClass, windowControlsCapClass, windowGapFrameActiveClass, windowGapFrameClass, type ModeDockChromeGrid, type WindowLayoutAxisNode, type WindowLayoutNode, type WindowLayoutStackNode, type UiStatus, type PanelGhostValue } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { CloseIcon, Maximize2Icon, Minimize2Icon, Icon, type ControlIcon } from "../🔣Icons/🟦️component.tsx";
import { DragHandle } from "../🧱DragHandle/🟦️component.tsx";
import { ButtonGroup, ButtonGroupItem } from "../🎛️ButtonGroup/🟦️component.tsx";
// #endregion 🔌️Adapters
// #region ⚙️Canvas

/**
 * Container component for canvas window layout.
 **/
export const Canvas: React.FC<{ children: React.ReactNode; id?: string; status?: UiStatus }> = ({ children, id, status = "idle" }) => {
  const parent = useSurface();
  const bgClass = shellFloorFillClass(parent);
  const busy = status === "loading" || status === "waiting";
  return (
    <LevelProvider level="base">
      <div id={id} data-slot="canvas" data-level="base" data-ui-status={busy ? status : undefined} className={cn("box-border h-full w-full p-single", bgClass, chromeStatusBorderClass(status))}>
        {busy ? <CanvasSkeleton /> : children}
      </div>
    </LevelProvider>
  );
};

/**
 * Layout component arranging windows horizontally.
 **/
export const HorizontalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-row h-full w-full gap-single">{children}</div>;
};

/**
 * Layout component arranging windows vertically.
 **/
export const VerticalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-col h-full w-full gap-single">{children}</div>;
};

// #region 🧭️Mode

/** @emoji 🪟️ Window descriptor rendered inside {@link Mode}. */
export interface ModeWindowDescriptor extends Omit<WindowConfig, "children" | "onOpenInNewWindow" | "onMaximize" | "onMinimize" | "onClose"> {
  title?: UiLabel;
  iconId: IconName;
  children: React.ReactNode;
}

export const COMPOSE_WINDOW_TEMPLATE_MIME = "application/x-compose-window-template";

/** @emoji 👻️ Ephemeral window id used while previewing an external template drag on {@link Mode}. */
export const MODE_TEMPLATE_PREVIEW_WINDOW_ID = "__compose-mode-template-preview__";

export interface WindowTemplateDragSession {
  readonly payload: WindowTemplateDropPayload;
  readonly label: string;
}

type PanelGhostSessionBridge = Pick<PanelGhostValue, "begin" | "end">;

export let panelGhostSessionBridge: PanelGhostSessionBridge | null = null;

export function setPanelGhostSessionBridge(bridge: PanelGhostSessionBridge | null): void {
  panelGhostSessionBridge = bridge;
}

const activeWindowTemplateDragSession = ephemeralBox<WindowTemplateDragSession | null>("framework.modules.ui.elements.Canvas.component.tsx.activeWindowTemplateDragSession", null);

/** @emoji 🪟️ Records the active palette template drag until drop or dragend. */
export function beginWindowTemplateDrag(session: WindowTemplateDragSession): void {
  activeWindowTemplateDragSession.current = session;
  panelGhostSessionBridge?.begin(null);
}

/** @emoji 🪟️ Clears the active palette template drag session. */
export function endWindowTemplateDrag(): void {
  activeWindowTemplateDragSession.current = null;
  panelGhostSessionBridge?.end();
}

/** @emoji 🪟️ Returns the in-flight palette template drag, if any. */
export function readActiveWindowTemplateDragSession(): WindowTemplateDragSession | null {
  return activeWindowTemplateDragSession.current;
}

/** @emoji 🖱️ Active pointer-driven window-template drag from the Display tree. */
export const windowTemplatePointerDragRef = { active: false };

/** @emoji 🖱️ Marks a pointer-driven window-template drag as active. */
export function beginWindowTemplatePointerDrag(_encoded: string): void {
  windowTemplatePointerDragRef.active = true;
}

/** @emoji 🖱️ Cancels a pointer-driven window-template drag. */
export function cancelWindowTemplatePointerDrag(): void {
  windowTemplatePointerDragRef.active = false;
  endWindowTemplateDrag();
}

/** @emoji 🖱️ {@link TreeDragAndDropController} for Display rows that carry window-template `dragData`. */
export function windowTemplatePaletteTreeDragController(): TreeDragAndDropController {
  const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
    const payload = dragData?.[COMPOSE_WINDOW_TEMPLATE_MIME];
    return payload?.trim() ? payload : undefined;
  };
  return {
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: beginWindowTemplatePointerDrag,
      cancel: cancelWindowTemplatePointerDrag,
    },
    onDragStart: ({ sourceItem }) => {
      const encoded = readEncoded(sourceItem.dragData);
      if (!encoded) {
        return;
      }
      try {
        const payload = JSON.parse(encoded) as WindowTemplateDropPayload;
        if (typeof payload.windowKindId !== "string") {
          return;
        }
        const label = typeof sourceItem.label === "string" ? sourceItem.label : typeof sourceItem.label === "number" ? String(sourceItem.label) : "Window";
        beginWindowTemplateDrag({ payload, label });
      } catch {
        /* ignore */
      }
    },
    onDragEnd: () => {
      if (windowTemplatePointerDragRef.active) {
        return;
      }
      endWindowTemplateDrag();
    },
  };
}

export type ModeCanvasDropTarget = { kind: "tab"; stackPath: string; index: number } | { kind: "split"; stackPath: string; side: "left" | "right" | "top" | "bottom" } | { kind: "root-split"; side: "left" | "right" | "top" | "bottom" };

export interface WindowTemplateDropPayload {
  readonly windowKindId: string;
  readonly templateId?: string;
}

export interface ModeProps {
  windows: ModeWindowDescriptor[];
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string | null) => void;
  onWindowClose?: (windowId: string) => void;
  layout?: WindowLayoutNode;
  onLayoutChange?: (layout: WindowLayoutNode) => void;
  onTemplateDrop?: (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => void;
  children?: React.ReactNode;
  className?: string;
  /** @emoji 📱️ Renders only the active window full-bleed with no tab bar, drag, dock, or maximize chrome. */
  mobile?: boolean;
}

//#region 🧭️ModeCanvasSpacing

/** @emoji 📐️ Canvas inset on {@link Mode} body; inter-panel splitters use the same {@link --spacing-single} step as navbar and footer chrome. */
export const MODE_CANVAS_INSET_CLASS = "p-single";

//#endregion 🧭️ModeCanvasSpacing

//#region 🧭️ModeLayoutUtils

type ModeLayoutPath = string;
type ModeDockSide = "left" | "right" | "top" | "bottom";

function modePathSegments(path: ModeLayoutPath): number[] {
  return path ? path.split(".").map((segment) => Number(segment)) : [];
}

function modeJoinPath(parent: ModeLayoutPath, index: number): ModeLayoutPath {
  return parent ? `${parent}.${index}` : String(index);
}

export function modeCollectWindowIds(node: WindowLayoutNode): string[] {
  if (node.kind === "window") return [node.id];
  if (node.kind === "stack") return node.children.map((child) => child.id);
  return node.children.flatMap(modeCollectWindowIds);
}

/** @emoji 🪟️ Ensures every window leaf sits inside a tab stack. */
function normalizeLayoutToStacks(node: WindowLayoutNode): WindowLayoutNode {
  if (node.kind === "window") return { kind: "stack", children: [node], activeId: node.id };
  if (node.kind === "stack") return { ...node, activeId: node.activeId ?? node.children[0]?.id };
  return { ...node, children: node.children.map((child) => normalizeLayoutToStacks(child) as WindowLayoutAxisNode | WindowLayoutStackNode) };
}

/** @emoji 🪟️ Collapses empty axes and hoists single-child axes. */
function collapseLayout(node: WindowLayoutNode | null): WindowLayoutNode | null {
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

function updateLayoutAtPath(layout: WindowLayoutNode, path: ModeLayoutPath, updater: (node: WindowLayoutNode) => WindowLayoutNode): WindowLayoutNode {
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

function readLayoutAtPath(layout: WindowLayoutNode, path: ModeLayoutPath): WindowLayoutNode | null {
  if (!path) return layout;
  const [head, ...rest] = modePathSegments(path);
  if (layout.kind === "window") return null;
  if (layout.kind === "stack") return layout;
  const child = layout.children[head!];
  if (!child) return null;
  if (rest.length === 0) return child as WindowLayoutNode;
  return readLayoutAtPath(child as WindowLayoutNode, rest.join("."));
}

function mapLayoutStacks(layout: WindowLayoutNode, mapper: (stack: WindowLayoutStackNode, path: ModeLayoutPath) => WindowLayoutStackNode, path = ""): WindowLayoutNode {
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return mapper(layout, path);
  return {
    ...layout,
    children: layout.children.map((child, index) => mapLayoutStacks(child as WindowLayoutNode, mapper, modeJoinPath(path, index)) as WindowLayoutAxisNode | WindowLayoutStackNode),
  };
}

function resolveStackPathForWindowId(layout: WindowLayoutNode, windowId: string): ModeLayoutPath | null {
  let found: ModeLayoutPath | null = null;
  mapLayoutStacks(layout, (stack, path) => {
    if (stack.children.some((child) => child.id === windowId)) found = path;
    return stack;
  });
  return found;
}

/** @emoji 🪟️ Drops windows that are no longer declared without auto-opening undeclared layout slots. */
function reconcileWindows(layout: WindowLayoutNode, windowIds: readonly string[]): WindowLayoutNode {
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

/** @emoji 🪟️ Removes a window from the layout tree and collapses empty nodes. */
function removeWindowFromLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode | null {
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

function insertWindowAsTab(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, index?: number): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const insertAt = index === undefined || index < 0 ? children.length : index;
    children.splice(insertAt, 0, { kind: "window", id: windowId });
    return { ...node, children, activeId: windowId };
  });
}

/** @emoji 📑️ Merges every tab from a dragged stack into another stack at the given index. */
function mergeStackTabsIntoStack(layout: WindowLayoutNode, targetStackPath: ModeLayoutPath, stack: WindowLayoutStackNode, index: number): WindowLayoutNode {
  const insertAt = index < 0 ? undefined : index;
  let result = layout;
  stack.children.forEach((child, offset) => {
    result = insertWindowAsTab(result, targetStackPath, child.id, insertAt === undefined ? undefined : insertAt + offset);
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

/** @emoji 🪟️ Splits a stack with a dragged window on the given side. */
function splitWithWindow(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  return updateLayoutAtPath(without, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
    const horizontal = side === "left" || side === "right";
    const children = side === "left" || side === "top" ? [incoming, node] : [node, incoming];
    return { kind: horizontal ? "row" : "column", children, size: node.size } as WindowLayoutAxisNode;
  });
}

function splitRootWithWindow(layout: WindowLayoutNode, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
  const horizontal = side === "left" || side === "right";
  const children = side === "left" || side === "top" ? [incoming, without] : [without, incoming];
  return { kind: horizontal ? "row" : "column", children: children as (WindowLayoutAxisNode | WindowLayoutStackNode)[] };
}

/** @emoji 🪟️ Detaches a tab stack from the layout tree for stack-level drag-dock. */
function extractStackFromLayout(layout: WindowLayoutNode, stackPath: ModeLayoutPath): { layout: WindowLayoutNode | null; stack: WindowLayoutStackNode | null } {
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

/** @emoji 🪟️ Splits a stack with a dragged tab stack on the given side. */
function splitWithStack(layout: WindowLayoutNode, targetStackPath: ModeLayoutPath, stack: WindowLayoutStackNode, side: ModeDockSide): WindowLayoutNode {
  return updateLayoutAtPath(layout, targetStackPath, (node) => {
    if (node.kind !== "stack") return node;
    const horizontal = side === "left" || side === "right";
    const children = side === "left" || side === "top" ? [stack, node] : [node, stack];
    return { kind: horizontal ? "row" : "column", children, size: node.size } as WindowLayoutAxisNode;
  });
}

/** @emoji 🪟️ Splits the mode root with a dragged tab stack on the given side. */
function splitRootWithStack(layout: WindowLayoutNode, stack: WindowLayoutStackNode, side: ModeDockSide): WindowLayoutNode {
  const horizontal = side === "left" || side === "right";
  const children = side === "left" || side === "top" ? [stack, layout] : [layout, stack];
  return { kind: horizontal ? "row" : "column", children: children as (WindowLayoutAxisNode | WindowLayoutStackNode)[] };
}

/** @emoji 🪟️ Writes resizable panel percentages back onto axis children. */
function applyAxisSizes(layout: WindowLayoutNode, axisPath: ModeLayoutPath, sizes: Record<string, number> | readonly number[]): WindowLayoutNode {
  return updateLayoutAtPath(layout, axisPath, (node) => {
    if (node.kind !== "row" && node.kind !== "column") return node;
    const sizesRecord = sizes as Record<string | number, number>;
    const children = node.children.map((child, index) => {
      const panelKey = modeJoinPath(axisPath, index);
      const size = sizesRecord[panelKey] ?? sizesRecord[index] ?? sizesRecord[String(index)] ?? child.size;
      return { ...child, size };
    });
    return { ...node, children };
  });
}


/** @emoji ↔ True when a child axis runs perpendicular to its parent axis. */
export function modeAxisIsPerpendicularChild(parentKind: "row" | "column", child: WindowLayoutNode): boolean {
  return (parentKind === "row" && child.kind === "column") || (parentKind === "column" && child.kind === "row");
}

/** @emoji ↔ Separator indices and size-weighted fractions for joins inside a perpendicular child axis. */
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

/** @emoji ↔ Corner join specs for a main-axis separator that crosses perpendicular child splits. */
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

/** @emoji ↔ Corner join specs on a cross-axis separator where it meets a parent split. */
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

/** @emoji ↔ One perpendicular axis that participates in a corner join. */
export type ModeJoinCornerCrossAxis = {
  path: ModeLayoutPath;
  separatorIndex: number;
};

/** @emoji ↔ Max fraction delta for two perpendicular joins to count as the same touching corner. */
export const MODE_JOIN_CORNER_TOUCH_EPS = 0.0025;

/** @emoji ↔ Every perpendicular axis whose join actually touches this corner (same main separator + size fraction), including the dragged cross axis. */
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

/** @emoji ↔ Percentage delta for one panel pair on an axis separator. */
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

/** @emoji ↔ Pointer delta for a corner join on perpendicular row/column axes. */
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

/** @emoji ↔ Applies a separator percentage delta to a live resizable group layout. */
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

/** @emoji ↔ Resolves persisted percentages for one resizable axis. */
export function modeAxisGroupLayout(layout: WindowLayoutNode, axisPath: ModeLayoutPath): Record<string, number> {
  const node = axisPath ? readLayoutAtPath(layout, axisPath) : layout;
  if (!node || (node.kind !== "row" && node.kind !== "column")) return {};
  return Object.fromEntries(node.children.map((child, index) => [modeJoinPath(axisPath, index), child.size ?? 100 / node.children.length]));
}

/** @emoji ↔ Applies a corner grab delta to the main axis and every peer cross axis at the join. */
export function applyModeJoinCornerResize(layout: WindowLayoutNode, spec: ResizableJoinCornerSpec, deltaXPx: number, deltaYPx: number, mainAxisPixelSize: number, crossAxisPixelSize: number): WindowLayoutNode {
  const { mainDeltaPct, crossDeltaPct } = resolveJoinCornerResizeDeltas(spec.parentKind, deltaXPx, deltaYPx, mainAxisPixelSize, crossAxisPixelSize);
  let next = applyAxisResizeDelta(layout, spec.mainAxisPath, spec.mainSeparatorIndex, mainDeltaPct);
  for (const peer of resolveJoinCornerPeerCrossAxes(layout, spec)) {
    next = applyAxisResizeDelta(next, peer.path, peer.separatorIndex, crossDeltaPct);
  }
  return next;
}

function readModeAxisPixelSize(element: HTMLElement | null | undefined, kind: "row" | "column"): number {
  if (!element) return 0;
  const rect = element.getBoundingClientRect();
  return kind === "row" ? rect.width : rect.height;
}

function readModeAxisKind(layout: WindowLayoutNode, axisPath: ModeLayoutPath): "row" | "column" | null {
  const node = axisPath ? readLayoutAtPath(layout, axisPath) : layout;
  if (!node || (node.kind !== "row" && node.kind !== "column")) return null;
  return node.kind;
}

function setActiveWindowInLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode {
  return mapLayoutStacks(layout, (stack) => {
    if (!stack.children.some((child) => child.id === windowId)) return stack;
    if (stack.activeId === windowId) return stack;
    return { ...stack, activeId: windowId };
  });
}

function resolveModeLayout(windows: readonly ModeWindowDescriptor[], layout?: WindowLayoutNode): WindowLayoutNode {
  const base = layout ?? createEvenWindowLayout(windows.map((window) => window.id));
  return reconcileWindows(
    base,
    windows.map((window) => window.id),
  );
}

/** @emoji 🪟️ Inserts a new window leaf at a dock drop target (external template drag). */
export function insertWindowAtDropZone(layout: WindowLayoutNode, windowId: string, target: ModeCanvasDropTarget): WindowLayoutNode {
  if (target.kind === "root-split") return splitRootWithWindow(layout, windowId, target.side as ModeDockSide);
  if (target.kind === "split") return splitWithWindow(layout, target.stackPath, windowId, target.side as ModeDockSide);
  return insertWindowAsTab(layout, target.stackPath, windowId, target.index < 0 ? undefined : target.index);
}

//#endregion 🧭️ModeLayoutUtils

//#region 🧭️ModeDockDrag

type ModeDropZone = { kind: "tab"; stackPath: ModeLayoutPath; index: number } | { kind: "split"; stackPath: ModeLayoutPath; side: ModeDockSide } | { kind: "root-split"; side: ModeDockSide };

type ModeDragKind = "tab" | "stack";

interface ModeDragState {
  dragKind: ModeDragKind;
  windowId: string;
  stackPath: ModeLayoutPath;
  tabIndex: number;
  pointerId: number;
  ghostLabel: string;
  x: number;
  y: number;
}

interface ModePendingDrag {
  dragKind: ModeDragKind;
  windowId: string;
  stackPath: ModeLayoutPath;
  tabIndex: number;
  pointerId: number;
  ghostLabel: string;
  startX: number;
  startY: number;
}

interface ModeStackDropTargets {
  tabBar: DOMRect | null;
  body: DOMRect | null;
  tabBarElement: HTMLElement | null;
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

/** @emoji 📍️ Resolves tab-bar insertion line and slot preview geometry for drag feedback. */
function computeTabInsertPreview(tabBarElement: HTMLElement | null, insertIndex: number): { insertX: number; top: number; height: number; slotLeft: number; slotWidth: number } | null {
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

/** @emoji 🧭️ Maps pointer position in a rectangle to a split side using half-panel zones (dominant axis from center). */
function resolveModeSplitSideInBody(localX: number, localY: number, bodyWidth: number, bodyHeight: number): ModeDockSide {
  const midX = bodyWidth / 2;
  const midY = bodyHeight / 2;
  const dx = Math.abs(localX - midX);
  const dy = Math.abs(localY - midY);
  if (dx >= dy) return localX < midX ? "left" : "right";
  return localY < midY ? "top" : "bottom";
}

/** @emoji 📐️ Half-panel rectangle for split drop preview inside a stack body (origin top-left of body). */
function computeModeSplitPreviewInBody(bodyWidth: number, bodyHeight: number, side: ModeDockSide): { left: number; top: number; width: number; height: number } {
  const halfWidth = bodyWidth / 2;
  const halfHeight = bodyHeight / 2;
  if (side === "left") return { left: 0, top: 0, width: halfWidth, height: bodyHeight };
  if (side === "right") return { left: bodyWidth - halfWidth, top: 0, width: halfWidth, height: bodyHeight };
  if (side === "top") return { left: 0, top: 0, width: bodyWidth, height: halfHeight };
  return { left: 0, top: bodyHeight - halfHeight, width: bodyWidth, height: halfHeight };
}

function computeModeDropZone(pointerX: number, pointerY: number, stackTargets: ReadonlyMap<ModeLayoutPath, ModeStackDropTargets>, modeRect: DOMRect | null): ModeDropZone | null {
  for (const [stackPath, targets] of stackTargets) {
    if (targets.tabBar && pointerInRect(pointerX, pointerY, targets.tabBar)) {
      return { kind: "tab", stackPath, index: computeTabInsertIndex(pointerX, targets.tabBarElement) };
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

function applyModeDrop(layout: WindowLayoutNode, drag: ModeDragState, zone: ModeDropZone): WindowLayoutNode {
  const { dragKind, windowId, stackPath: sourcePath, tabIndex } = drag;
  if (dragKind === "stack") {
    const targetStack = zone.kind === "tab" ? readLayoutAtPath(layout, zone.stackPath) : null;
    const targetAnchorId = targetStack?.kind === "stack" ? (targetStack.activeId ?? targetStack.children[0]?.id) : undefined;
    const { layout: withoutSource, stack } = extractStackFromLayout(layout, sourcePath);
    if (!stack) return layout;
    const base = withoutSource ?? { kind: "stack", children: [] };
    if (zone.kind === "root-split") return splitRootWithStack(base, stack, zone.side);
    if (zone.stackPath === sourcePath) return layout;
    if (zone.kind === "split") {
      const splitTargetPath = targetAnchorId !== undefined ? (resolveStackPathForWindowId(base, targetAnchorId) ?? zone.stackPath) : zone.stackPath;
      return splitWithStack(base, splitTargetPath, stack, zone.side);
    }
    const mergeTargetPath = targetAnchorId !== undefined ? (resolveStackPathForWindowId(base, targetAnchorId) ?? zone.stackPath) : zone.stackPath;
    return mergeStackTabsIntoStack(base, mergeTargetPath, stack, zone.index);
  }
  if (zone.kind === "root-split") return splitRootWithWindow(layout, windowId, zone.side);
  if (zone.kind === "split") return splitWithWindow(layout, zone.stackPath, windowId, zone.side);
  if (zone.stackPath === sourcePath) {
    const stackNode = readLayoutAtPath(layout, sourcePath);
    const childCount = stackNode?.kind === "stack" ? stackNode.children.length : 0;
    const withoutLength = Math.max(0, childCount - 1);
    const toIndex = zone.index < 0 ? tabIndex : Math.min(zone.index, withoutLength);
    if (toIndex === tabIndex) return layout;
    return reorderTabInStack(layout, sourcePath, tabIndex, toIndex);
  }
  const without = removeWindowFromLayout(layout, windowId);
  if (!without) return layout;
  return insertWindowAsTab(without, zone.stackPath, windowId, zone.index < 0 ? undefined : zone.index);
}

/** @emoji 🪓️ Removes the dragged tab or stack from the committed layout while it floats on the cursor. */
function modeDockOutLayout(committed: WindowLayoutNode, drag: Pick<ModeDragState, "dragKind" | "windowId" | "stackPath">): WindowLayoutNode {
  if (drag.dragKind === "stack") {
    const { layout } = extractStackFromLayout(committed, drag.stackPath);
    return layout ?? { kind: "stack", children: [] };
  }
  return removeWindowFromLayout(committed, drag.windowId) ?? committed;
}

interface ModeTabInsertPreview {
  stackPath: ModeLayoutPath;
  index: number;
}

type ModeDockTabDisplayItem = { id: string; title: string; iconId: IconName; preview?: "ghost" };

/** @emoji 📑️ Tab bar row with ghost tab(s) at the drop index so layout matches the committed drop. */
function modeDockTabsWithInsertPreview(
  tabs: readonly { id: string; title: string; iconId: IconName }[],
  insertPreview: ModeTabInsertPreview | null,
  stackPath: ModeLayoutPath,
  ghostTabs: readonly { id: string; title: string; iconId: IconName }[],
): ModeDockTabDisplayItem[] {
  if (!insertPreview || insertPreview.stackPath !== stackPath || ghostTabs.length === 0) return tabs.map((tab) => ({ ...tab }));
  const insertAt = Math.min(Math.max(0, insertPreview.index), tabs.length);
  const row: ModeDockTabDisplayItem[] = tabs.map((tab) => ({ ...tab }));
  row.splice(insertAt, 0, ...ghostTabs.map((tab) => ({ id: tab.id, title: tab.title, iconId: tab.iconId, preview: "ghost" as const })));
  return row;
}

/** @emoji 📑️ Tab descriptors shown as insert-preview ghosts for the current drag. */
function modeDockDragInsertTabs(layout: WindowLayoutNode, drag: ModeDragState, windowTitle: (windowId: string) => string, windowIconId: (windowId: string) => IconName): readonly { id: string; title: string; iconId: IconName }[] {
  if (drag.dragKind === "tab") return [{ id: drag.windowId, title: windowTitle(drag.windowId), iconId: windowIconId(drag.windowId) }];
  const stack = readLayoutAtPath(layout, drag.stackPath);
  if (!stack || stack.kind !== "stack") return [{ id: drag.windowId, title: windowTitle(drag.windowId), iconId: windowIconId(drag.windowId) }];
  return stack.children.map((child) => ({ id: child.id, title: windowTitle(child.id), iconId: windowIconId(child.id) }));
}

function resolveModeTabInsertPreview(drag: ModeDragState | null, zone: ModeDropZone | null): ModeTabInsertPreview | null {
  if (!drag || zone?.kind !== "tab") return null;
  if (drag.dragKind === "stack" && zone.stackPath === drag.stackPath) return null;
  return { stackPath: zone.stackPath, index: zone.index };
}

const modeDockTabInsertPreviewClass = "mx-half my-half flex h-[calc(100%-var(--spacing-single))] min-w-[5.5rem] max-w-[12rem] shrink-0 items-center rounded-sm border-2 border-accent bg-accent/20 px-single text-xs text-foreground/80 select-none";

//#endregion 🧭️ModeDockDrag

//#region 🧭️ModeDockDragPreview

const MODE_DRAG_CURSOR_OFFSET_X = 8;
const MODE_DRAG_CURSOR_OFFSET_Y = 10;

interface ModeDockDragPreviewProps {
  title: string;
  iconId?: IconName;
  content?: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
  tabOnly?: boolean;
}

/** @emoji 🪟️ Floating tab or window preview shown while docking. */
const ModeDockDragPreview: React.FC<ModeDockDragPreviewProps> = ({ title, iconId, content, className, style, tabOnly = false }) =>
  tabOnly ? (
    <div data-slot="mode-dock-drag-preview" data-level="window" className={cn(modeDockInactiveTabClass, "pointer-events-none flex max-w-[12rem] shrink-0 items-center px-single text-xs text-element shadow-md select-none", glassClass, className)} style={style}>
      <div className={modeDockTabLabelClassName}>
        {iconId ? <Icon icon={iconId} size="small" className="shrink-0" /> : null}
        <span data-slot="inline-label" className="truncate">
          {title}
        </span>
      </div>
    </div>
  ) : (
    <div data-slot="mode-dock-drag-preview" className={cn("pointer-events-none flex flex-col overflow-hidden rounded shadow-lg", className)} style={style}>
      <div data-slot="mode-dock-drag-preview-cap" data-level="window" className={cn(windowCapFrameClass, "relative z-[2] flex h-medium shrink-0 items-stretch px-single", glassClass)}>
        <div className={modeDockTabLabelClassName}>
          {iconId ? <Icon icon={iconId} size="small" className="shrink-0" /> : null}
          <span data-slot="inline-label" className="truncate">
            {title}
          </span>
        </div>
      </div>
      <div data-slot="mode-dock-drag-preview-body" data-level="base" className={cn("relative min-h-0 flex-1 overflow-hidden p-single opacity-95", windowBodyFrameClass)}>
        {content ? (
          <div data-level="window" className={cn("h-full w-full overflow-hidden [&_*]:pointer-events-none", surfaceClass)}>
            {content}
          </div>
        ) : null}
      </div>
    </div>
  );

//#endregion 🧭️ModeDockDragPreview

//#region 🧭️ModeDockTabBar

interface ModeDockContextValue {
  dragState: ModeDragState | null;
  tabInsertPreview: ModeTabInsertPreview | null;
  draggedInsertTabs: readonly { id: string; title: string; iconId: IconName }[];
  registerStackDropTargets: (path: ModeLayoutPath, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => void;
  startTabDrag: (windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => void;
  clearPendingDrag: (pointerId: number) => void;
  closeWindow: (windowId: string) => void;
  activateWindow: (windowId: string) => void;
  deactivateActiveWindow: () => void;
  maximizedStackPath: ModeLayoutPath | null;
  /** @emoji ⛶️ False when the canvas has only one window — Focus/Unfocus has nothing to enlarge against. */
  canMaximize: boolean;
  toggleMaximize: (stackPath: ModeLayoutPath) => void;
}

const ModeDockContext = reactHostPort.createContext<ModeDockContextValue | null>(null);

interface ModeDockTabBarProps {
  stackPath: ModeLayoutPath;
  tabs: readonly { id: string; title: string; iconId: IconName }[];
  activeId: string | undefined;
  activeWindowId: string | null;
  onSelectTab: (windowId: string) => void;
  chromeGrid?: ModeDockChromeGrid;
  chromeBody?: React.ReactNode;
  /** @emoji 📱️ Windows always take the full space on mobile — the Focus/Unfocus control is meaningless there and is hidden; Close stays. */
  mobile?: boolean;
}

const ModeDockTabBar = reactHostPort.forwardRef<HTMLDivElement, ModeDockTabBarProps>(({ stackPath, tabs, activeId, activeWindowId, onSelectTab, chromeGrid, chromeBody, mobile = false }, ref) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const dockFocusLabel = useLabel("ui.common.focus");
  const dockUnfocusLabel = useLabel("ui.common.unfocus");
  const dockCloseLabel = useLabel("ui.common.close");
  const isMaximized = dock?.maximizedStackPath === stackPath;
  const showMaximize = !mobile && Boolean(dock?.canMaximize);
  const modeDragActive = Boolean(dock?.dragState);
  const stackGloballyActive = Boolean(activeId && activeWindowId === activeId);
  const perTabActiveChrome = Boolean(chromeGrid);
  const capFrameClass = stackGloballyActive ? windowCapFrameActiveClass : windowCapFrameClass;
  const gapFrameClass = stackGloballyActive ? windowGapFrameActiveClass : windowGapFrameClass;
  const displayTabs = reactHostPort.useMemo(() => modeDockTabsWithInsertPreview(tabs, dock?.tabInsertPreview ?? null, stackPath, dock?.draggedInsertTabs ?? []), [tabs, dock?.tabInsertPreview, stackPath, dock?.draggedInsertTabs]);
  const displayChromeGrid =
    displayTabs.length > 1
      ? modeDockChromeGridPlacement(
          displayTabs.map(({ id, title }) => ({ id, title })),
          activeId,
        )
      : undefined;

  const renderGhostTab = (tab: { id: string; title: string; iconId: IconName }) => (
    <div data-slot="mode-dock-tab-insert-preview" className={modeDockTabInsertPreviewClass} aria-hidden>
      <div className={modeDockTabLabelClassName}>
        <Icon icon={tab.iconId} size="small" className="shrink-0" />
        <span data-slot="inline-label" className="truncate">
          {tab.title}
        </span>
      </div>
    </div>
  );

  const inactiveTabChromeClass = (stackIndex: number) => {
    const isLastBeforeGap = perTabActiveChrome && stackIndex === tabs.length - 1;
    return isLastBeforeGap ? modeDockInactiveTabBeforeGapClass : modeDockInactiveTabClass;
  };

  const renderTab = (tab: (typeof tabs)[number], stackIndex: number) => {
    const tabActive = perTabActiveChrome ? activeId === tab.id && stackGloballyActive : activeWindowId === tab.id;
    return (
      <div
        key={tab.id}
        data-slot="mode-dock-tab"
        data-hover-scope
        data-window-id={tab.id}
        data-stack-active={activeId === tab.id ? "true" : undefined}
        data-active={activeWindowId === tab.id ? "true" : undefined}
        className={cn(
          "pointer-events-auto",
          modeDockTabClassName,
          perTabActiveChrome && activeId !== tab.id && inactiveTabChromeClass(stackIndex),
          perTabActiveChrome && activeId === tab.id && !stackGloballyActive && inactiveTabChromeClass(stackIndex),
          perTabActiveChrome && activeId === tab.id && stackGloballyActive && modeDockActiveTabClass,
          !perTabActiveChrome && activeWindowId === tab.id && modeDockActiveTabFillClass,
        )}
        onClick={() => onSelectTab(tab.id)}
        onPointerUp={(event) => {
          if (event.button !== 0) return;
          dock?.clearPendingDrag?.(event.pointerId);
        }}
      >
        <div className={modeDockTabLabelClassName}>
          <Icon icon={tab.iconId} size="small" className="shrink-0" />
          <span data-slot="inline-label" className="truncate">
            {tab.title}
          </span>
        </div>
        <DragHandle labelId="ui.tree.drag.sort" onPointerDown={(event) => dock?.startTabDrag(tab.id, stackPath, stackIndex, tab.title, event)} onClick={(event) => event.stopPropagation()} emphasized={tabActive} />
      </div>
    );
  };

  const controlsCap = (
    <div
      data-slot="mode-dock-controls-cap"
      data-window-silhouette-chip
      data-dock="top"
      className={cn(
        "pointer-events-auto",
        perTabActiveChrome ? (stackGloballyActive ? windowControlsCapActiveSplitClass : windowControlsCapClass) : stackGloballyActive ? windowControlsCapActiveClass : windowControlsCapClass,
        glassClass,
      )}
    >
      {showMaximize ? (
        <button
          type="button"
          data-slot="mode-dock-maximize"
          className={cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
          onClick={() => dock?.toggleMaximize(stackPath)}
        >
          {isMaximized ? <Minimize2Icon className="size-small" /> : <Maximize2Icon className="size-small" />}
          <span className="text-tiny whitespace-nowrap">{isMaximized ? dockUnfocusLabel : dockFocusLabel}</span>
        </button>
      ) : null}
      {activeId ? (
        <button
          type="button"
          data-slot="mode-dock-close"
          className={cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
          onClick={() => dock?.closeWindow(activeId)}
        >
          <CloseIcon className="size-small" />
          <span className="text-tiny whitespace-nowrap">{dockCloseLabel}</span>
        </button>
      ) : null}
    </div>
  );

  const tabGap = (
    <div
      data-slot="mode-dock-tab-gap"
      data-window-silhouette-gap
      className={cn("pointer-events-none relative min-h-medium min-w-0 flex-1", perTabActiveChrome ? "z-0" : "z-[1]", gapFrameClass)}
    />
  );

  if (perTabActiveChrome && displayChromeGrid && chromeBody) {
    return (
      <div data-slot="mode-dock-chrome-column" className="relative z-[2] grid h-full min-h-0 min-w-0 flex-1 grid-rows-[auto_minmax(0,1fr)]" style={{ gridTemplateColumns: displayChromeGrid.templateColumns }}>
        <div
          ref={ref}
          data-slot="mode-dock-tabbar"
          data-ui-reveal-region="window-cap"
          className={cn("grid min-h-medium min-w-0 items-stretch bg-transparent", modeDragActive && dropZoneReadyClass)}
          style={{ gridColumn: "1 / -1", gridRow: 1, gridTemplateColumns: displayChromeGrid.templateColumns }}
        >
          {displayTabs.map((tab, index) =>
            tab.preview === "ghost" ? (
              <div key={`ghost-${tab.id}`} className={cn("relative z-20 flex min-h-medium items-stretch justify-self-start", glassClass)} style={{ gridColumn: displayChromeGrid.tabCol(index) }}>
                {renderGhostTab(tab)}
              </div>
            ) : (
              <div
                key={tab.id}
                data-slot={activeId === tab.id && stackGloballyActive ? "mode-dock-tab-active-cell" : "mode-dock-tab-cell"}
                className={cn(
                  "relative flex min-h-medium items-stretch justify-self-start overflow-visible",
                  activeId === tab.id && stackGloballyActive ? "z-10" : cn("z-20", glassClass),
                )}
                style={{ gridColumn: displayChromeGrid.tabCol(index) }}
              >
                {renderTab(
                  tab,
                  tabs.findIndex((row) => row.id === tab.id),
                )}
              </div>
            ),
          )}
          <div className="relative z-0 flex min-h-medium min-w-0 items-stretch bg-transparent" style={{ gridColumn: displayChromeGrid.gapCol }}>
            {tabGap}
          </div>
          <div className="relative z-10 flex min-h-medium items-stretch justify-self-end" style={{ gridColumn: displayChromeGrid.controlsCol }}>
            {controlsCap}
          </div>
        </div>
        <div className="flex min-h-0 min-w-0 flex-col overflow-hidden" style={{ gridColumn: displayChromeGrid.bodyColumnSpan, gridRow: 2 }}>
          {chromeBody}
        </div>
      </div>
    );
  }

  return (
    <div ref={ref} data-slot="mode-dock-tabbar" data-ui-reveal-region="window-cap" className="relative z-[2] flex w-full min-w-0 shrink-0 items-stretch bg-transparent">
      <div data-slot="mode-dock-tab-cap" data-window-silhouette-chip data-dock="top" className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", capFrameClass, glassClass, modeDragActive && dropZoneReadyClass)}>
        <div data-slot="mode-dock-tabs" className="flex min-w-0 items-stretch justify-start overflow-x-auto overflow-y-hidden">
          {displayTabs.map((tab) =>
            tab.preview === "ghost" ? (
              <div key={`ghost-${tab.id}`}>{renderGhostTab(tab)}</div>
            ) : (
              renderTab(
                tab,
                tabs.findIndex((row) => row.id === tab.id),
              )
            ),
          )}
        </div>
      </div>
      {tabGap}
      {controlsCap}
    </div>
  );
});

ModeDockTabBar.displayName = "ModeDockTabBar";

//#endregion 🧭️ModeDockTabBar

//#region 🧭️ModeDockStack

/** @emoji 🪟️ Mode-dock silhouette — delegates to {@link WindowChromeSilhouetteBorder} with the legacy slot name. */
const ModeDockStackSilhouetteBorder: React.FC<{ stack: HTMLElement | null; active: boolean }> = ({ stack, active }) => <WindowChromeSilhouetteBorder stack={stack} active={active} silhouetteSlot="mode-dock-silhouette-border" />;

interface ModeDockStackProps {
  stackPath: ModeLayoutPath;
  node: WindowLayoutStackNode;
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
  /** @emoji 📱️ Skips the per-tab active-chrome grid (which sizes tab columns to their content and doesn't scroll) in favor of the plain scrollable tab strip, and drops the Focus control (windows always take the full space on mobile) so only Close stays reachable when many windows collapse into one mobile tab stack. Desktop also hides Focus when the canvas has only one window. */
  mobile?: boolean;
}

const ModeDockStack: React.FC<ModeDockStackProps> = ({ stackPath, node, windowsById, activeWindowId, mobile = false }) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const stackRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [stackEl, setStackEl] = reactHostPort.useState<HTMLDivElement | null>(null);
  const [surfaceActive, surfaceActiveProps] = useSurfaceActive(stackRef);
  const setStackNode = reactHostPort.useCallback((element: HTMLDivElement | null) => {
    stackRef.current = element;
    setStackEl(element);
  }, []);
  const tabBarRef = reactHostPort.useRef<HTMLDivElement>(null);
  const bodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const activeId = node.activeId ?? node.children[0]?.id;
  const tabs = node.children.map((child) => ({
    id: child.id,
    title: child.title ?? windowsById.get(child.id)?.title ?? child.id,
    iconId: windowsById.get(child.id)?.iconId ?? "app-window",
  }));

  reactHostPort.useLayoutEffect(() => {
    dock?.registerStackDropTargets(stackPath, tabBarRef.current, bodyRef.current);
    return () => dock?.registerStackDropTargets(stackPath, null, null);
  }, [dock, stackPath, node.children.length]);

  const activeDescriptor = activeId ? windowsById.get(activeId) : undefined;
  // 🪟️ Layout focus (command routing / tab fills) stays on `activeWindowId`. The silhouette primary
  // stroke only follows surface selection — same click/focus lifecycle as panels and panes.
  const stackGloballyActive = Boolean(activeId && activeWindowId === activeId);
  const chromeGrid = !mobile && tabs.length > 1 ? modeDockChromeGridPlacement(tabs, activeId) : undefined;

  const stackBody = (
    <SurfaceScope level="base" fill="surface">
      <div ref={bodyRef} data-slot="mode-dock-stack-body" data-level="base" className={cn("pointer-events-auto relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden p-single", stackGloballyActive ? windowBodyFrameActiveClass : windowBodyFrameClass)}>
        {activeDescriptor
          ? (() => {
              const { children, engagement, ...windowProps } = activeDescriptor;
              return (
                <Window {...windowProps} fill={activeDescriptor.fill ?? true} engagement={engagement} active={activeWindowId === activeId} onActivate={() => dock?.activateWindow(activeId!)}>
                  {children}
                </Window>
              );
            })()
          : null}
      </div>
    </SurfaceScope>
  );

  // 🪟️ `z-window` isolates dock chrome / silhouette z-indexes so they cannot paint above floating Panels
  // (`zIndex` ≥ `--z-panel`). `[data-introduction-elevated]` still overrides to `z-tutorial + 1`.
  return (
    <SurfaceScope level="window">
      <div ref={setStackNode} {...surfaceActiveProps} data-slot="mode-dock-stack" data-window-silhouette data-level="window" data-stack-path={stackPath} data-active={surfaceActive ? "true" : undefined} className="pointer-events-none relative z-window flex h-full min-h-0 w-full min-w-0 flex-col overflow-visible bg-transparent">
        <ModeDockStackSilhouetteBorder stack={stackEl} active={surfaceActive} />
        {chromeGrid ? (
          <ModeDockTabBar ref={tabBarRef} stackPath={stackPath} tabs={tabs} activeId={activeId} activeWindowId={activeWindowId} chromeGrid={chromeGrid} chromeBody={stackBody} onSelectTab={(windowId) => dock?.activateWindow(windowId)} mobile={mobile} />
        ) : (
          <>
            <ModeDockTabBar ref={tabBarRef} stackPath={stackPath} tabs={tabs} activeId={activeId} activeWindowId={activeWindowId} onSelectTab={(windowId) => dock?.activateWindow(windowId)} mobile={mobile} />
            {stackBody}
          </>
        )}
      </div>
    </SurfaceScope>
  );
};

//#endregion 🧭️ModeDockStack

//#region 🧭️ModeRender

interface ModeRenderContext {
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
  onAxisLayoutChanged: (axisPath: ModeLayoutPath, sizes: Record<string, number>) => void;
  onJoinCornerResize: ResizableJoinCornerResizeHandler;
  registerAxisGroup: (axisPath: ModeLayoutPath, group: ResizablePrimitive.GroupImperativeHandle | null) => void;
  registerAxisElement: (axisPath: ModeLayoutPath, element: HTMLDivElement | null) => void;
}

interface ModeRenderParentAxis {
  path: ModeLayoutPath;
  kind: "row" | "column";
  panelIndex: number;
}

function renderModeDockNode(node: WindowLayoutAxisNode | WindowLayoutStackNode, path: ModeLayoutPath, ctx: ModeRenderContext, parentAxis?: ModeRenderParentAxis): React.ReactNode {
  if (node.kind === "stack") {
    return <ModeDockStack key={path || "root-stack"} stackPath={path} node={node} windowsById={ctx.windowsById} activeWindowId={ctx.activeWindowId} />;
  }
  const orientation = node.kind === "row" ? "horizontal" : "vertical";
  const childCount = node.children.length;
  const rawSizes = node.children.map((child) => child.size);
  const hasAllDefinedSizes = rawSizes.every((s) => typeof s === "number" && s > 0);
  let defaultLayout: number[];
  if (hasAllDefinedSizes) {
    const sum = (rawSizes as number[]).reduce((a, b) => a + b, 0);
    if (Math.abs(sum - 100) < 0.01 && sum > 0) {
      defaultLayout = rawSizes as number[];
    } else if (sum > 0) {
      defaultLayout = (rawSizes as number[]).map((s) => (s / sum) * 100);
    } else {
      defaultLayout = node.children.map(() => 100 / childCount);
    }
  } else {
    defaultLayout = node.children.map(() => 100 / childCount);
  }
  const panels: React.ReactNode[] = [];
  node.children.forEach((child, index) => {
    const childPath = modeJoinPath(path, index);
    if (index > 0) {
      const prevChild = node.children[index - 1]!;
      const joinCorners = [...modeJoinCornerSpecsForSeparator(path, node.kind, index, prevChild, child), ...(parentAxis ? modeJoinCornerSpecsForCrossSeparator(path, node.kind, index, parentAxis) : [])];
      panels.push(<ResizableHandle key={`sep-${childPath}`} joinCorners={joinCorners} onJoinCornerResize={ctx.onJoinCornerResize} orientation={orientation} />);
    }
    panels.push(
      <ResizablePanel key={childPath} id={childPath} defaultSize={defaultLayout[index] ?? 100 / childCount} minSize={8} className="box-border min-h-0 min-w-0 overflow-visible">
        {renderModeDockNode(child, childPath, ctx, { path, kind: node.kind, panelIndex: index })}
      </ResizablePanel>,
    );
  });
  return (
    <ResizablePanelGroup
      key={`${path || "root-axis"}-${childCount}`}
      id={`mode-axis-${path || "root"}`}
      elementRef={(element) => ctx.registerAxisElement(path, element)}
      groupRef={(group) => ctx.registerAxisGroup(path, group)}
      orientation={orientation}
      defaultLayout={defaultLayout}
      onLayoutChanged={(sizes) => ctx.onAxisLayoutChanged(path, sizes)}
      className="h-full min-h-0 w-full min-w-0"
    >
      {panels}
    </ResizablePanelGroup>
  );
}

//#endregion 🧭️ModeRender

/** @emoji 🪟️ Golden-Layout-style docking mode shell with tab stacks, drag-dock, resize, maximize, and close. */
const Mode: React.FC<ModeProps> = ({ windows, activeWindowId, onActiveWindowChange, onWindowClose, layout, onLayoutChange, onTemplateDrop, children, className = "", mobile = false }) => {
  // 🐚️ Gates the active-window search-routing keydown listener below to this shell — absent outside a `ShellScopeProvider` (tests), where it simply stays inert.
  const shellScope = useShellScopeOptional();
  // 🐚️ Resolves via the nearest `I18nextProvider` (this shell's own instance), not the shared `uiI18n` singleton.
  const { t: modeT } = useUiTranslation();
  const parentSurface = useSurface();
  const modeBodyFillClass = shellFloorFillClass(parentSurface);
  const windowsById = reactHostPort.useMemo(() => new Map(windows.map((window) => [window.id, window])), [windows]);
  const windowsKey = reactHostPort.useMemo(() => windows.map((window) => window.id).join("|"), [windows]);
  const layoutKey = reactHostPort.useMemo(() => JSON.stringify(layout ?? null), [layout]);
  const initialLayout = reactHostPort.useMemo(() => resolveModeLayout(windows, layout), [layout, windows]);
  const [layoutState, setLayoutState] = reactHostPort.useState<WindowLayoutNode>(() => initialLayout);
  const [maximizedStackPath, setMaximizedStackPath] = reactHostPort.useState<ModeLayoutPath | null>(null);
  const [dragState, setDragState] = reactHostPort.useState<ModeDragState | null>(null);
  const [templateDrag, setTemplateDrag] = reactHostPort.useState<{ readonly label: string; readonly x: number; readonly y: number } | null>(null);
  const [pendingDrag, setPendingDrag] = reactHostPort.useState<ModePendingDrag | null>(null);
  const [dropZone, setDropZone] = reactHostPort.useState<ModeDropZone | null>(null);
  const dropZoneRef = reactHostPort.useRef<ModeDropZone | null>(null);
  const modeBodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const axisGroupRefsRef = reactHostPort.useRef(new Map<ModeLayoutPath, ResizablePrimitive.GroupImperativeHandle>());
  const axisGroupElementsRef = reactHostPort.useRef(new Map<ModeLayoutPath, HTMLDivElement>());
  const stackDropElementsRef = reactHostPort.useRef(new Map<ModeLayoutPath, { tabBar: HTMLElement | null; body: HTMLElement | null }>());
  const layoutStateRef = reactHostPort.useRef(layoutState);
  const dragLayoutSnapshotRef = reactHostPort.useRef<WindowLayoutNode | null>(null);
  const layoutKeyRef = reactHostPort.useRef(layoutKey);
  const windowsKeyRef = reactHostPort.useRef(windowsKey);
  layoutStateRef.current = layoutState;

  // 🚧️ `onLayoutChange` is for user-driven layout mutations (drag/resize/close/activate) only.
  // Prop-derived states — the mount initializer and every prop resync — must never echo back:
  // during a session switch the windows prop changes before the layout prop, so the resync
  // reconciles against a stale layout and would write a pruned (possibly empty) layout into the
  // host's state, clobbering the layout the host is about to seed.
  const propDerivedLayoutJsonRef = reactHostPort.useRef(JSON.stringify(initialLayout));
  reactHostPort.useEffect(() => {
    if (JSON.stringify(layoutState) === propDerivedLayoutJsonRef.current) return;
    onLayoutChange?.(layoutState);
  }, [layoutState, onLayoutChange]);

  reactHostPort.useEffect(() => {
    const layoutChanged = layoutKeyRef.current !== layoutKey;
    const windowsChanged = windowsKeyRef.current !== windowsKey;
    if (!layoutChanged && !windowsChanged) return;
    layoutKeyRef.current = layoutKey;
    windowsKeyRef.current = windowsKey;
    const resolved = resolveModeLayout(windows, layout);
    propDerivedLayoutJsonRef.current = JSON.stringify(resolved);
    setLayoutState(resolved);
    setMaximizedStackPath(null);
  }, [layout, layoutKey, windows, windowsKey]);

  reactHostPort.useEffect(() => {
    if (!activeWindowId) return;
    setLayoutState((prev) => setActiveWindowInLayout(prev, activeWindowId));
  }, [activeWindowId]);

  useShellKeydown(
    shellScope?.rootRef ?? NULL_SHELL_ROOT_REF,
    (event) => {
      if (!activeWindowId) return;
      const search = windowsById.get(activeWindowId)?.search;
      if (!search?.input) return;
      if (
        routeWindowSearchEscape(search, event, {
          chromeVisible: true,
          actionActive: true,
        })
      ) {
        event.preventDefault();
        event.stopPropagation();
        return;
      }
      if (routeWindowSearchSpace(search, event)) {
        event.preventDefault();
        event.stopPropagation();
        return;
      }
      if (!routeWindowSearchKeydown(search, event)) return;
      event.preventDefault();
      event.stopPropagation();
      queueMicrotask(() => focusActiveSearchInput());
    },
    [activeWindowId, windowsById, windowsKey],
  );

  const registerStackDropTargets = reactHostPort.useCallback((path: ModeLayoutPath, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => {
    if (!tabBarElement && !bodyElement) {
      stackDropElementsRef.current.delete(path);
      return;
    }
    const prev = stackDropElementsRef.current.get(path) ?? { tabBar: null, body: null };
    stackDropElementsRef.current.set(path, {
      tabBar: tabBarElement ?? prev.tabBar,
      body: bodyElement ?? prev.body,
    });
  }, []);

  const activateWindow = reactHostPort.useCallback(
    (windowId: string) => {
      if (activeWindowId === windowId) return;
      setLayoutState((prev) => setActiveWindowInLayout(prev, windowId));
      onActiveWindowChange?.(windowId);
    },
    [activeWindowId, onActiveWindowChange],
  );

  const deactivateActiveWindow = reactHostPort.useCallback(() => {
    setSurfaceActiveRoot(null);
    if (activeWindowId === null) return;
    onActiveWindowChange?.(null);
  }, [activeWindowId, onActiveWindowChange]);

  const handleCanvasBackgroundPointerDown = reactHostPort.useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (isSurfaceActiveBackgroundPointer(event)) {
        deactivateActiveWindow();
        return;
      }
      if (!(event.target instanceof HTMLElement)) return;
      if (event.target.closest('[data-slot="mode-dock-stack"]')) return;
      if (event.target !== event.currentTarget && event.target.dataset.slot !== "resizable-panel") return;
      deactivateActiveWindow();
    },
    [deactivateActiveWindow],
  );

  const closeWindow = reactHostPort.useCallback(
    (windowId: string) => {
      onWindowClose?.(windowId);
      setLayoutState((prev) => {
        const next = collapseLayout(removeWindowFromLayout(prev, windowId)) ?? { kind: "stack", children: [] };
        const remaining = modeCollectWindowIds(next);
        if (activeWindowId === windowId) onActiveWindowChange?.(remaining[0] ?? null);
        return next;
      });
    },
    [activeWindowId, onActiveWindowChange, onWindowClose],
  );

  const toggleMaximize = reactHostPort.useCallback((stackPath: ModeLayoutPath) => {
    setMaximizedStackPath((prev) => (prev === stackPath ? null : stackPath));
  }, []);

  const refreshDropZone = reactHostPort.useCallback((clientX: number, clientY: number) => {
    const targets = new Map<ModeLayoutPath, ModeStackDropTargets>();
    stackDropElementsRef.current.forEach((elements, path) => {
      targets.set(path, {
        tabBar: elements.tabBar?.getBoundingClientRect() ?? null,
        body: elements.body?.getBoundingClientRect() ?? null,
        tabBarElement: elements.tabBar,
      });
    });
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    const zone = computeModeDropZone(clientX, clientY, targets, modeRect);
    dropZoneRef.current = zone;
    setDropZone(zone);
  }, []);

  const finishDrag = reactHostPort.useCallback(
    (drag: ModeDragState, zone: ModeDropZone | null) => {
      if (!zone) return;
      setLayoutState((prev) => applyModeDrop(prev, drag, zone));
      activateWindow(drag.windowId);
    },
    [activateWindow],
  );

  const clearPendingDrag = reactHostPort.useCallback((pointerId: number) => {
    setPendingDrag((prev) => (prev?.pointerId === pointerId ? null : prev));
  }, []);

  const startTabDrag = reactHostPort.useCallback((windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => {
    if (event.button !== 0) return;
    setPendingDrag({
      dragKind: "tab",
      windowId,
      stackPath,
      tabIndex,
      pointerId: event.pointerId,
      ghostLabel: label,
      startX: event.clientX,
      startY: event.clientY,
    });
  }, []);

  reactHostPort.useEffect(() => {
    if (!pendingDrag && !dragState) return;
    const handleMove = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (pendingDrag && !dragState) {
        const distance = Math.hypot(event.clientX - pendingDrag.startX, event.clientY - pendingDrag.startY);
        if (distance < 6) return;
        dragLayoutSnapshotRef.current = layoutStateRef.current;
        setDragState({
          dragKind: pendingDrag.dragKind,
          windowId: pendingDrag.windowId,
          stackPath: pendingDrag.stackPath,
          tabIndex: pendingDrag.tabIndex,
          pointerId: pendingDrag.pointerId,
          ghostLabel: pendingDrag.ghostLabel,
          x: event.clientX,
          y: event.clientY,
        });
        setPendingDrag(null);
        refreshDropZone(event.clientX, event.clientY);
        return;
      }
      if (!dragState) return;
      setDragState((prev) => (prev ? { ...prev, x: event.clientX, y: event.clientY } : prev));
      refreshDropZone(event.clientX, event.clientY);
    };
    const handleUp = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (dragState) finishDrag(dragState, dropZoneRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    document.addEventListener("pointermove", handleMove);
    document.addEventListener("pointerup", handleUp);
    return () => {
      document.removeEventListener("pointermove", handleMove);
      document.removeEventListener("pointerup", handleUp);
    };
  }, [pendingDrag, dragState, finishDrag, refreshDropZone]);

  reactHostPort.useEffect(() => {
    if (!dragState) return;
    const cancelDrag = () => {
      if (dragLayoutSnapshotRef.current) setLayoutState(dragLayoutSnapshotRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      cancelDrag();
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [dragState]);

  const onAxisLayoutChanged = reactHostPort.useCallback((axisPath: ModeLayoutPath, sizes: Record<string, number>) => {
    setLayoutState((prev) => applyAxisSizes(prev, axisPath, sizes));
  }, []);

  const registerAxisGroup = reactHostPort.useCallback((axisPath: ModeLayoutPath, group: ResizablePrimitive.GroupImperativeHandle | null) => {
    if (group) axisGroupRefsRef.current.set(axisPath, group);
    else axisGroupRefsRef.current.delete(axisPath);
  }, []);

  const registerAxisElement = reactHostPort.useCallback((axisPath: ModeLayoutPath, element: HTMLDivElement | null) => {
    if (element) axisGroupElementsRef.current.set(axisPath, element);
    else axisGroupElementsRef.current.delete(axisPath);
  }, []);

  const onJoinCornerResize = reactHostPort.useCallback((spec: ResizableJoinCornerSpec, deltaXPx: number, deltaYPx: number) => {
    const current = layoutStateRef.current;
    const mainKind = readModeAxisKind(current, spec.mainAxisPath);
    const crossKind = readModeAxisKind(current, spec.crossAxisPath);
    const mainGroup = axisGroupRefsRef.current.get(spec.mainAxisPath);
    const crossGroup = axisGroupRefsRef.current.get(spec.crossAxisPath);
    if (!mainKind || !crossKind || !mainGroup || !crossGroup) return;
    const peers = resolveJoinCornerPeerCrossAxes(current, spec);
    const deltas = resolveJoinCornerResizeDeltas(
      spec.parentKind,
      deltaXPx,
      deltaYPx,
      readModeAxisPixelSize(axisGroupElementsRef.current.get(spec.mainAxisPath), mainKind),
      readModeAxisPixelSize(axisGroupElementsRef.current.get(spec.crossAxisPath), crossKind),
    );
    const liveMainLayout = mainGroup.getLayout();
    const currentMainLayout = Object.keys(liveMainLayout).length > 0 ? liveMainLayout : modeAxisGroupLayout(current, spec.mainAxisPath);
    const mainLayout = applyAxisGroupLayoutDelta(currentMainLayout, spec.mainAxisPath, spec.mainSeparatorIndex, deltas.mainDeltaPct);
    mainGroup.setLayout(mainLayout);
    const peerLayouts: { path: ModeLayoutPath; layout: Record<string, number> }[] = [];
    for (const peer of peers) {
      const peerGroup = axisGroupRefsRef.current.get(peer.path);
      if (!peerGroup) continue;
      const livePeerLayout = peerGroup.getLayout();
      const currentPeerLayout = Object.keys(livePeerLayout).length > 0 ? livePeerLayout : modeAxisGroupLayout(current, peer.path);
      const peerLayout = applyAxisGroupLayoutDelta(currentPeerLayout, peer.path, peer.separatorIndex, deltas.crossDeltaPct);
      peerGroup.setLayout(peerLayout);
      peerLayouts.push({ path: peer.path, layout: peerLayout });
    }
    setLayoutState((prev) => peerLayouts.reduce((next, peer) => applyAxisSizes(next, peer.path, peer.layout), applyAxisSizes(prev, spec.mainAxisPath, mainLayout)));
  }, []);

  const clearTemplateDragPreview = reactHostPort.useCallback(() => {
    setTemplateDrag(null);
    dropZoneRef.current = null;
    setDropZone(null);
  }, []);

  reactHostPort.useEffect(() => {
    if (!templateDrag) return;
    const onDragEnd = () => {
      endWindowTemplateDrag();
      clearTemplateDragPreview();
    };
    document.addEventListener("dragend", onDragEnd);
    return () => document.removeEventListener("dragend", onDragEnd);
  }, [templateDrag, clearTemplateDragPreview]);

  const resolveTemplateDropZone = reactHostPort.useCallback((clientX: number, clientY: number): ModeDropZone | null => {
    const targets = new Map<ModeLayoutPath, ModeStackDropTargets>();
    stackDropElementsRef.current.forEach((elements, path) => {
      targets.set(path, {
        tabBar: elements.tabBar?.getBoundingClientRect() ?? null,
        body: elements.body?.getBoundingClientRect() ?? null,
        tabBarElement: elements.tabBar,
      });
    });
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    return computeModeDropZone(clientX, clientY, targets, modeRect);
  }, []);

  const completeExternalTemplateDrop = reactHostPort.useCallback(
    (clientX: number, clientY: number, payload: WindowTemplateDropPayload): boolean => {
      if (!onTemplateDrop || typeof payload.windowKindId !== "string") {
        return false;
      }
      const zone = resolveTemplateDropZone(clientX, clientY);
      if (!zone) {
        return false;
      }
      endWindowTemplateDrag();
      windowTemplatePointerDragRef.active = false;
      clearTemplateDragPreview();
      onTemplateDrop(payload, zone);
      return true;
    },
    [clearTemplateDragPreview, onTemplateDrop, resolveTemplateDropZone],
  );

  const handleExternalTemplateDragOver = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      if (!onTemplateDrop || !event.dataTransfer.types.includes(COMPOSE_WINDOW_TEMPLATE_MIME)) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) return;
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
      setTemplateDrag({ label: session.label, x: event.clientX, y: event.clientY });
      refreshDropZone(event.clientX, event.clientY);
    },
    [onTemplateDrop, refreshDropZone],
  );

  const handleExternalTemplateDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      if (!onTemplateDrop) return;
      const raw = event.dataTransfer.getData(COMPOSE_WINDOW_TEMPLATE_MIME);
      if (!raw) return;
      event.preventDefault();
      let payload: WindowTemplateDropPayload;
      try {
        payload = JSON.parse(raw) as WindowTemplateDropPayload;
      } catch {
        return;
      }
      completeExternalTemplateDrop(event.clientX, event.clientY, payload);
    },
    [completeExternalTemplateDrop, onTemplateDrop],
  );

  reactHostPort.useEffect(() => {
    if (!onTemplateDrop) return;
    const handlePointerMove = (event: PointerEvent) => {
      if (!windowTemplatePointerDragRef.active) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) return;
      setTemplateDrag({ label: session.label, x: event.clientX, y: event.clientY });
      refreshDropZone(event.clientX, event.clientY);
    };
    const handlePointerUp = (event: PointerEvent) => {
      if (!windowTemplatePointerDragRef.active) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) {
        windowTemplatePointerDragRef.active = false;
        clearTemplateDragPreview();
        return;
      }
      const modeRect = modeBodyRef.current?.getBoundingClientRect();
      if (!modeRect || event.clientX < modeRect.left || event.clientX > modeRect.right || event.clientY < modeRect.top || event.clientY > modeRect.bottom) {
        cancelWindowTemplatePointerDrag();
        clearTemplateDragPreview();
        return;
      }
      if (!completeExternalTemplateDrop(event.clientX, event.clientY, session.payload)) {
        cancelWindowTemplatePointerDrag();
        clearTemplateDragPreview();
      }
    };
    window.addEventListener("pointermove", handlePointerMove, true);
    window.addEventListener("pointerup", handlePointerUp, true);
    window.addEventListener("pointercancel", handlePointerUp, true);
    return () => {
      window.removeEventListener("pointermove", handlePointerMove, true);
      window.removeEventListener("pointerup", handlePointerUp, true);
      window.removeEventListener("pointercancel", handlePointerUp, true);
    };
  }, [clearTemplateDragPreview, completeExternalTemplateDrop, onTemplateDrop, refreshDropZone]);

  const templatePreviewDrag = reactHostPort.useMemo((): ModeDragState | null => {
    if (!templateDrag) return null;
    return {
      dragKind: "tab",
      windowId: MODE_TEMPLATE_PREVIEW_WINDOW_ID,
      stackPath: "",
      tabIndex: -1,
      pointerId: -1,
      ghostLabel: templateDrag.label,
      x: templateDrag.x,
      y: templateDrag.y,
    };
  }, [templateDrag]);

  const previewDragState = dragState ?? templatePreviewDrag;

  const draggedPreviewTitle = previewDragState ? (windowsById.get(previewDragState.windowId)?.title ?? previewDragState.ghostLabel) : "";
  const draggedPreviewIconId = previewDragState ? windowsById.get(previewDragState.windowId)?.iconId : undefined;
  const tabInsertPreview = resolveModeTabInsertPreview(previewDragState, dropZone);
  const draggedInsertTabs = reactHostPort.useMemo(() => {
    if (templateDrag) return [{ id: MODE_TEMPLATE_PREVIEW_WINDOW_ID, title: templateDrag.label, iconId: "app-window" }];
    if (!dragState) return [];
    return modeDockDragInsertTabs(
      layoutState,
      dragState,
      (windowId) => windowsById.get(windowId)?.title ?? windowId,
      (windowId) => windowsById.get(windowId)?.iconId ?? "app-window",
    );
  }, [dragState, layoutState, templateDrag, windowsById]);

  const noopDrag = reactHostPort.useCallback(() => {}, []);
  const canMaximize = modeCollectWindowIds(layoutState).length > 1;

  reactHostPort.useEffect(() => {
    if (canMaximize || maximizedStackPath === null) return;
    setMaximizedStackPath(null);
  }, [canMaximize, maximizedStackPath]);

  const dockContext = reactHostPort.useMemo<ModeDockContextValue>(
    () => ({
      dragState: previewDragState,
      tabInsertPreview,
      draggedInsertTabs,
      registerStackDropTargets,
      startTabDrag: mobile ? noopDrag : startTabDrag,
      clearPendingDrag,
      closeWindow,
      activateWindow,
      deactivateActiveWindow,
      maximizedStackPath,
      canMaximize,
      toggleMaximize,
    }),
    [mobile, noopDrag, previewDragState, tabInsertPreview, draggedInsertTabs, registerStackDropTargets, startTabDrag, clearPendingDrag, closeWindow, activateWindow, deactivateActiveWindow, maximizedStackPath, canMaximize, toggleMaximize],
  );

  const renderContext = reactHostPort.useMemo<ModeRenderContext>(
    () => ({ windowsById, activeWindowId, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement }),
    [windowsById, activeWindowId, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement],
  );

  const dockOutLayout = reactHostPort.useMemo(() => (dragState ? modeDockOutLayout(layoutState, dragState) : layoutState), [layoutState, dragState]);

  const maximizedStack =
    maximizedStackPath !== null
      ? (() => {
          let found: WindowLayoutStackNode | null = null;
          mapLayoutStacks(dockOutLayout, (stack, path) => {
            if (path === maximizedStackPath) found = stack;
            return stack;
          });
          return found;
        })()
      : null;

  const orderedWindowIds = modeCollectWindowIds(dockOutLayout);
  const hasWindows = orderedWindowIds.length > 0;
  const emptyShellNotice = resolveTranslationLabel(modeT("ui.display.emptyShell"));

  /** @emoji 📱️ Mobile has no split-pane window manager: every window collapses into one tab stack, rendered through the same {@link ModeDockStack} chrome (tab bar, utility bar, measures, engagement) as desktop. */
  const mobileFlatStack: WindowLayoutStackNode | null = mobile
    ? {
        kind: "stack",
        children: orderedWindowIds.map((id) => ({ kind: "window", id })),
        activeId: (activeWindowId && orderedWindowIds.includes(activeWindowId) ? activeWindowId : orderedWindowIds[0]) ?? undefined,
      }
    : null;

  const body =
    children ??
    (mobile ? (
      mobileFlatStack && mobileFlatStack.children.length > 0 ? (
        <ModeDockContext.Provider value={dockContext}>
          <ModeDockStack stackPath="" node={mobileFlatStack} windowsById={windowsById} activeWindowId={activeWindowId} mobile />
        </ModeDockContext.Provider>
      ) : null
    ) : maximizedStack ? (
      <ModeDockContext.Provider value={dockContext}>
        <ModeDockStack stackPath={maximizedStackPath!} node={maximizedStack} windowsById={windowsById} activeWindowId={activeWindowId} />
      </ModeDockContext.Provider>
    ) : (
      <ModeDockContext.Provider value={dockContext}>{renderModeDockNode(dockOutLayout as WindowLayoutAxisNode | WindowLayoutStackNode, "", renderContext)}</ModeDockContext.Provider>
    ));

  return (
    <div data-slot="mode" data-mobile={mobile ? "true" : undefined} data-dragging={previewDragState ? "true" : undefined} data-maximized-path={maximizedStackPath ?? undefined} className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      <LevelProvider level="base">
        <div
          ref={modeBodyRef}
          data-slot="mode-body"
          data-level="base"
          className={cn("relative box-border flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden", modeBodyFillClass, MODE_CANVAS_INSET_CLASS)}
          onPointerDownCapture={handleCanvasBackgroundPointerDown}
          onDragOver={!mobile && onTemplateDrop ? handleExternalTemplateDragOver : undefined}
          onDrop={!mobile && onTemplateDrop ? handleExternalTemplateDrop : undefined}
        >
          {!hasWindows ? (
            <div data-slot="mode-empty" className="flex flex-1 items-center justify-center p-large text-center text-sm text-muted-foreground">
              {emptyShellNotice}
            </div>
          ) : (
            body
          )}
          {!mobile && previewDragState ? (
            <>
              {dropZone?.kind !== "tab" ? (
                <ModeDockDragPreview
                  title={draggedPreviewTitle}
                  iconId={draggedPreviewIconId}
                  content={previewDragState.dragKind === "stack" ? windowsById.get(previewDragState.windowId)?.children : undefined}
                  tabOnly={previewDragState.dragKind === "tab"}
                  style={{
                    position: "fixed",
                    left: previewDragState.x + MODE_DRAG_CURSOR_OFFSET_X,
                    top: previewDragState.y - MODE_DRAG_CURSOR_OFFSET_Y,
                    zIndex: 70,
                  }}
                />
              ) : null}
              {dropZone && (dropZone.kind === "split" || dropZone.kind === "root-split") ? (
                <div data-slot="mode-dock-drop-indicator" className="pointer-events-none absolute inset-0 z-panel">
                  <div
                    className="absolute rounded-sm border-2 border-accent bg-accent/20"
                    style={(() => {
                      if (dropZone.kind === "root-split") {
                        const side = dropZone.side;
                        if (side === "left") return { left: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "right") return { right: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "top") return { left: 0, top: 0, width: "100%", height: "50%" };
                        return { left: 0, bottom: 0, width: "100%", height: "50%" };
                      }
                      const elements = stackDropElementsRef.current.get(dropZone.stackPath);
                      const rect = elements?.body?.getBoundingClientRect();
                      const modeRect = modeBodyRef.current?.getBoundingClientRect();
                      if (!rect || !modeRect) return { display: "none" };
                      const bodyOriginLeft = rect.left - modeRect.left;
                      const bodyOriginTop = rect.top - modeRect.top;
                      const preview = computeModeSplitPreviewInBody(rect.width, rect.height, dropZone.side);
                      return {
                        left: bodyOriginLeft + preview.left,
                        top: bodyOriginTop + preview.top,
                        width: preview.width,
                        height: preview.height,
                      };
                    })()}
                  />
                </div>
              ) : null}
            </>
          ) : null}
        </div>
      </LevelProvider>
    </div>
  );
};

export {
  Mode,
  removeWindowFromLayout,
  splitWithWindow,
  splitWithStack,
  extractStackFromLayout,
  applyModeDrop,
  reconcileWindows,
  normalizeLayoutToStacks,
  collapseLayout,
  computeModeDropZone,
  computeModeSplitPreviewInBody,
  resolveModeSplitSideInBody,
  computeTabInsertPreview,
  modeDockOutLayout,
  modeDockTabsWithInsertPreview,
  modeDockDragInsertTabs,
  mergeStackTabsIntoStack,
  resolveModeTabInsertPreview,
};

// #endregion 🧭️Mode

// #region 🧭️App

/** @emoji 📱️ Mode descriptor rendered inside {@link App}. */
export interface AppModeDescriptor {
  id: string;
  label?: UiLabel;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export interface AppProps {
  modes: AppModeDescriptor[];
  activeModeId: string;
  onActiveModeChange?: (modeId: string) => void;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** @emoji 📱️ App shell with optional mode switcher and one active mode body. */
const App: React.FC<AppProps> = ({ modes, activeModeId, onActiveModeChange, children, className = "", chrome = true }) => {
  const modeLabel = useLabel("ui.selection.mode");
  const activeMode = modes.find((mode) => mode.id === activeModeId) ?? modes[0];
  const body = children ?? activeMode?.children;
  const showModeNav = chrome && modes.length > 1 && !!onActiveModeChange;

  return (
    <div data-slot="app" className={cn("flex h-full min-h-0 w-full flex-col", className)}>
      {showModeNav ? (
        <div data-slot="app-mode-nav" className="flex shrink-0 items-center gap-single border-b p-single">
          <Select id="app.mode.select" value={activeModeId} onValueChange={onActiveModeChange}>
            <SelectTrigger className="w-[min(100%,16rem)]">
              <SelectValue placeholder={modeLabel} />
            </SelectTrigger>
            <SelectContent>
              {modes.map((mode) => (
                <SelectItem key={mode.id} value={mode.id}>
                  {mode.label ?? mode.id}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      ) : null}
      <div data-slot="app-body" className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
      </div>
    </div>
  );
};

export { App };

// #endregion 🧭️App

// #region 🧭️Ui

/** @emoji 🖥️ App descriptor rendered inside {@link Ui}. */
export interface UiAppDescriptor {
  id: string;
  label?: UiLabel;
  icon?: ControlIcon;
  children: React.ReactNode;
}

export interface UiProps {
  apps: UiAppDescriptor[];
  activeAppId: string;
  onActiveAppChange?: (appId: string) => void;
  navbar?: React.ReactNode;
  footer?: React.ReactNode;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** @emoji 🖥️ Top-level UI shell with optional app switcher and one active app body. */
const Ui: React.FC<UiProps> = ({ apps, activeAppId, onActiveAppChange, navbar, footer, children, className = "", chrome = true }) => {
  const activeApp = apps.find((app) => app.id === activeAppId) ?? apps[0];
  const body = children ?? activeApp?.children;
  const showAppNav = chrome && apps.length > 1 && !!onActiveAppChange;

  const navbarItems: NavbarItem[] = [];
  if (showAppNav) {
    navbarItems.push({
      key: "appNav",
      content: (
        <ButtonGroup id="ui.appNav">
          {apps.map((app) => (
            <ButtonGroupItem
              key={app.id}
              id={`ui.appNav.${app.id}`}
              className={cn(activeAppId === app.id && interactiveActiveFillClass)}
              data-state={activeAppId === app.id ? "on" : undefined}
              onClick={() => onActiveAppChange?.(app.id)}
              icon={app.icon ?? "layout-grid"}
              text={app.label}
            />
          ))}
        </ButtonGroup>
      ),
    });
  }
  if (navbar) {
    navbarItems.push({ key: "navbar", className: "flex-1 min-w-0", content: navbar });
  }

  return (
    <div data-slot="ui" className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      {navbarItems.length > 0 ? <Navbar items={navbarItems} /> : null}
      <div data-slot="ui-body" className="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
      </div>
      {footer && (
        <div data-slot="ui-footer" className="shrink-0">
          {footer}
        </div>
      )}
    </div>
  );
};

export { Ui };

// #endregion 🧭️Ui

// #endregion ⚙️Canvas

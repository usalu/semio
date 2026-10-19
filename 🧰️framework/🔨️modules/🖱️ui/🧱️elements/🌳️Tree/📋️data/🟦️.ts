import * as React from "react";
import type { ContextMenuItem } from "../../🖱️ContextMenu/🎨️item-render/🟦️.tsx";
import type { TreeHeaderAction } from "../⚡️actions/🟦️.tsx";
import type { TreeDataWindow } from "../🔭️virtual-window/🟦️.tsx";

export enum TreeItemCollapsibleState {
  None = 0,
  Collapsed = 1,
  Expanded = 2,
}

export type TreeSelectionMode = "single" | "multiple";

export interface TreeDataActivationContext {
  path: string[];
  selectedIds: readonly string[];
  sectionId: string;
}

/** 🖱️ Builds the exact `(event, context)` pair a row activation carries, for hosts that press a
 * {@link TreeDataItem.onClick} without a rendered tree (row-gate and category-tree tests). The event is a
 * real DOM `MouseEvent` wrapped in React's synthetic surface, so a handler that reads `button`,
 * `metaKey` or calls `preventDefault()` behaves exactly as it does under a rendered row. */
export function treeDataActivation(options: { readonly sectionId?: string; readonly path?: string[]; readonly selectedIds?: readonly string[]; readonly target?: Element; readonly init?: MouseEventInit } = {}): readonly [React.MouseEvent, TreeDataActivationContext] {
  const target = options.target ?? document.createElement("div");
  const nativeEvent = new MouseEvent("click", { bubbles: true, cancelable: true, ...options.init });
  let defaultPrevented = false;
  let propagationStopped = false;
  const event: React.MouseEvent = {
    altKey: nativeEvent.altKey,
    bubbles: nativeEvent.bubbles,
    button: nativeEvent.button,
    buttons: nativeEvent.buttons,
    cancelable: nativeEvent.cancelable,
    clientX: nativeEvent.clientX,
    clientY: nativeEvent.clientY,
    ctrlKey: nativeEvent.ctrlKey,
    currentTarget: target,
    get defaultPrevented() { return defaultPrevented; },
    detail: nativeEvent.detail,
    eventPhase: nativeEvent.eventPhase,
    getModifierState: (key) => nativeEvent.getModifierState(key),
    isDefaultPrevented: () => defaultPrevented,
    isPropagationStopped: () => propagationStopped,
    isTrusted: nativeEvent.isTrusted,
    metaKey: nativeEvent.metaKey,
    movementX: nativeEvent.movementX,
    movementY: nativeEvent.movementY,
    nativeEvent,
    pageX: nativeEvent.pageX,
    pageY: nativeEvent.pageY,
    persist: () => {},
    preventDefault: () => { defaultPrevented = true; nativeEvent.preventDefault(); },
    relatedTarget: nativeEvent.relatedTarget,
    screenX: nativeEvent.screenX,
    screenY: nativeEvent.screenY,
    shiftKey: nativeEvent.shiftKey,
    stopPropagation: () => { propagationStopped = true; nativeEvent.stopPropagation(); },
    target,
    timeStamp: nativeEvent.timeStamp,
    type: nativeEvent.type,
    view: { styleMedia: { type: "screen", matchMedium: (query: string) => target.ownerDocument.defaultView?.matchMedia(query).matches === true }, document: target.ownerDocument },
  };
  return [event, { path: options.path ?? [], selectedIds: options.selectedIds ?? [], sectionId: options.sectionId ?? "" }];
}

export interface TreeDataItem {
  id: string;
  label: React.ReactNode;
  icon?: React.ReactNode;
  description?: React.ReactNode;
  /** 🎛️ Inline control rendered in a property-style tree row. */
  control?: React.ReactNode;
  items?: TreeDataItem[];
  getItems?: () => Promise<TreeDataItem[]>;
  /** 🌿️ Alternative branches for this item. Each branch is an array of child items. Navigation < > switches between branches. */
  alternatives?: TreeDataItem[][];
  actions?: TreeHeaderAction[];
  className?: string;
  isHighlighted?: boolean;
  isSelected?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  /** 🌀️ Host-declared loading state, ORed with the tree's own async {@link getItems} pending state. */
  loading?: boolean;
  /** 🌀️ Host-declared waiting state; dashed, slower ring than {@link loading}. */
  waiting?: boolean;
  collapsibleState?: TreeItemCollapsibleState;
  emptyState?: React.ReactNode;
  draggable?: boolean;
  /** 📤️ Extra `dataTransfer` MIME entries merged on drag start (in-app palette drags). */
  dragData?: Record<string, string>;
  onClick?: (event: React.MouseEvent, context: TreeDataActivationContext) => void;
  onDoubleClick?: (event: React.MouseEvent, context: TreeDataActivationContext) => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  /** 👁️ Muted row styling for hidden entities in scene outliners. */
  isHidden?: boolean;
  /** 🖱️ Right-click menu for the row (selection-aware actions are built by the host). */
  contextMenu?: ContextMenuItem[];
  /** 🪟️ The slice of this group's children {@link TreeDataItem.items} actually carries — see {@link TreeDataWindow}. */
  window?: TreeDataWindow;
  /** 🔑️ The authored node key — the pick target id, NOT {@link TreeDataItem.id}, which is a DOM id. */
  windowKey?: string;
  /** 🪟️ This window's IDENTITY: {@link treeWindowPathOf} of the enclosing windowed containers and this one. What the host keys its state by and reports back; the same key under two parents is two windows. Defaults to {@link TreeDataItem.windowKey} for a top-level container. */
  windowPath?: string;
}

export interface TreeDataSection {
  id: string;
  label?: React.ReactNode;
  icon?: React.ReactNode;
  items?: TreeDataItem[];
  getItems?: () => Promise<TreeDataItem[]>;
  actions?: TreeHeaderAction[];
  className?: string;
  defaultOpen?: boolean;
  /** 🌀️ Host-declared loading state, ORed with the tree's own async {@link getItems} pending state. */
  loading?: boolean;
  /** 🌀️ Host-declared waiting state; dashed, slower ring than {@link loading}. */
  waiting?: boolean;
  emptyState?: React.ReactNode;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
  /** ↕️ When true (or when the host Tree enables {@link TreeRootProps.sortableSections}), this section header shows a drag handle for reordering among sibling sections. */
  draggable?: boolean;
  /** 🪟️ The slice of this section's children {@link TreeDataSection.items} actually carries — see {@link TreeDataWindow}. */
  window?: TreeDataWindow;
  /** 🔑️ The authored node key — the pick target id, NOT {@link TreeDataSection.id}, which is a DOM id. */
  windowKey?: string;
  /** 🪟️ This window's IDENTITY: {@link treeWindowPathOf} of the enclosing windowed containers and this one. A top-level section's path IS its key. */
  windowPath?: string;
}

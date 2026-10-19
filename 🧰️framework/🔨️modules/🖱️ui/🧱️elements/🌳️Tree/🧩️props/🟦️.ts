import * as React from "react";
import type { ContextMenuItem } from "../../🖱️ContextMenu/🎨️item-render/🟦️.tsx";
import type { UiLabel } from "../../🎗️UiLabel/🟦️.tsx";
import type { TreeWindowRowExtent, TreePresentation } from "../../../../🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts";
import type { TreeDirection } from "../📏️row-alignment/🟦️.tsx";
import { reactHostPort } from "../../🔌️Ports/🟦️.tsx";
import type { TreeDragRole, TreeHeaderAction } from "../⚡️actions/🟦️.tsx";
import type { TreeWindowDomAttributes } from "../🔭️virtual-window/🟦️.tsx";
import type { TreeDataItem, TreeDataSection, TreeSelectionMode } from "../📋️data/🟦️.ts";
import type { TreeDragAndDropController, TreeDropPosition } from "../🧲️drag-drop/🟦️.tsx";

/**
 * TreeSectionProps holds the data fields for a TreeSectionProps record.
 **/
export interface TreeSectionProps {
  label?: React.ReactNode;
  id?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  expandable?: boolean;
  loading?: boolean;
  waiting?: boolean;
  className?: string;
  actions?: TreeHeaderAction[];
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragEnd?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  isLastSection?: boolean;
  /** 🎯️ Passive drop-zone highlight while a compatible tree drag is in flight. */
  isDropReady?: boolean;
  /** 🫳️ When true, renders a trailing {@link DragHandle} for reorder (with handle-only initiation unless {@link dragInitiation} is `"surface"`). */
  isDragHandle?: boolean;
  /** 🫳️ `"handle"` restricts native drag start to the trailing grip; `"surface"` keeps the whole section header draggable. Defaults to `"handle"` when {@link isDragHandle} or {@link draggable} is set. */
  dragInitiation?: "handle" | "surface";
  /** 🪟️ Window mirror stamped on this section's branch content element — build it with {@link treeWindowDomAttributes}. */
  windowAttributes?: TreeWindowDomAttributes;
}

/**
 * SortableTreeItemProps holds the data fields for a SortableTreeItemProps record.
 **/
export interface SortableTreeItemProps {
  id: string;
  label?: React.ReactNode;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  isLastItem?: boolean;
  actions?: TreeHeaderAction[];
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  layoutKind?: "default" | "property";
}

/**
 * TreeItemProps holds the data fields for a TreeItemProps record.
 **/
export interface TreeItemProps {
  focusPath?: readonly string[];
  label?: React.ReactNode;
  id?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  sortable?: boolean;
  sortableId?: string;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  expandable?: boolean;
  loading?: boolean;
  waiting?: boolean;
  isLastItem?: boolean;
  actions?: TreeHeaderAction[];
  isDragging?: boolean;
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragEnd?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  /** Total number of alternative branches. When > 0, shows branch navigation. */
  branchCount?: number;
  /** Currently active branch index (0-based). */
  activeBranchIndex?: number;
  /** Callback when the user navigates to a different branch. */
  onBranchChange?: (index: number) => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onPointerDown?: React.PointerEventHandler<HTMLDivElement>;
  onPointerMove?: React.PointerEventHandler<HTMLDivElement>;
  onPointerUp?: React.PointerEventHandler<HTMLDivElement>;
  onPointerCancel?: React.PointerEventHandler<HTMLDivElement>;
  layoutKind?: "default" | "property";
  /** 🖱️ Whether this row carries an activation OF ITS OWN, as opposed to the selection handler the
   * tree wires onto every row. Only the `property` layout reads it, and only for an EXPANDABLE row: there
   * the label click folds the group, which is right for an inspector heading and wrong for a row that
   * declares an action (the puzzle3d catalogue's object-kind rows nest their rim-vortex templates AND bind
   * `activate` to `addObjectKind`). `onClick` cannot answer this — `TreeDataItemView` always passes one. */
  activatable?: boolean;
  isHidden?: boolean;
  contextMenu?: ContextMenuItem[];
  /** 🎚️ Control rendered on the header row of expandable property groups (label left, control right). */
  headerControl?: React.ReactNode;
  /** 💬️ A property row's description (a refused value's reason, the unit, an item count): always shown beside the label and
   * named by the row's control through `aria-describedby` ({@link useTreeRowDescribedControls}). */
  description?: React.ReactNode;
  /** 🫳️ `"handle"` restricts native drag start to the trailing grip (arms `draggable` only while the grip is pressed); `"surface"` keeps the whole row draggable. */
  dragInitiation?: "handle" | "surface";
  /** 🫳️ Explicit drag handles for sort vs palette transfer; overrides {@link dragInitiation} when set. */
  dragRoles?: readonly TreeDragRole[];
  /** 🫳️ Pointer-down handler for palette transfer drags — wired to the transfer handle only. */
  transferPointerDown?: React.PointerEventHandler<HTMLSpanElement>;
  /** 📮️ The row's transfer payload by MIME, mirrored onto the row as `data-drag-mime`/`data-drag-payload` so the transferred bytes are readable from the DOM without opening a real drag. */
  dragData?: Record<string, string>;
  /** 🎯️ Passive drop-zone highlight while a compatible tree drag is in flight. */
  isDropReady?: boolean;
  /** 🪟️ Window mirror stamped on this group's branch content element — build it with {@link treeWindowDomAttributes}. */
  windowAttributes?: TreeWindowDomAttributes;
  /** 📐️ This row's own entry index inside its PARENT's window, stamped as `data-tree-window-row` so the
   * host observer can read the real top of every materialised row instead of assuming a uniform row pitch
   * (a row that is itself an open windowed group is many rows tall). Build it with {@link treeWindowRowIndexOf}. */
  windowRowIndex?: number;
  /** 🧮️ The total of this row's PARENT window — with {@link windowRowIndex} the row's `aria-posinset`/`aria-setsize`
   * ({@link treeWindowRowAriaAttributes}). */
  windowRowTotal?: number;
  /** 📏️ Required closed-row geometry inherited from this row's parent virtual window. */
  windowRowExtent?: TreeWindowRowExtent;
}

/**
 * SortableTreeItemsProps holds the data fields for a SortableTreeItemsProps record.
 **/
export interface SortableTreeItemsProps {
  items: { id: string; [key: string]: any }[];
  onReorder: (oldIndex: number, newIndex: number) => void;
  children: (item: any, index: number) => React.ReactNode;
}

/**
 * TreeRootProps holds the data fields for a TreeRootProps record.
 **/
export const EMPTY_TREE_SECTIONS: TreeDataSection[] = [];

export interface TreeRootProps {
  className?: string;
  presentation?: TreePresentation;
  showLines?: boolean;
  sections?: TreeDataSection[];
  selectionMode?: TreeSelectionMode;
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  onSelectionChange?: (selectedIds: string[], items: TreeDataItem[]) => void;
  highlightedIds?: readonly string[];
  dragAndDropController?: TreeDragAndDropController;
  /** 🈳️ What the tree shows while it has no sections — a message or a whole window (Tasks, Chat). The tree
   * itself stays `dir="auto"` and left-to-right inline flow, so a right-docked panel can mirror its chrome
   * without flipping rows. The empty message keeps its own `dir="auto"` so a sentence is never reversed. */
  emptyState?: React.ReactNode;
  indentMultiplier?: number;
  /** 🧭️ `"up"` makes every foldable group in this tree unfold above its own header (children in reverse order), mirroring the {@link Ribbon} `"up"` pattern — for trees hosted in a panel that grows upward. Defaults to `"down"`. */
  direction?: TreeDirection;
  /** 🌱️ Controlled section/group expansion (see {@link TreeStateProvider}) — lets a host persist which groups are open across remounts. Uncontrolled (per-mount) when omitted. */
  openStates?: Readonly<Record<string, boolean>>;
  onOpenStateChange?: (id: string, open: boolean) => void;
  /** ↕️ When true, every section header renders a drag handle and sibling sections can be reordered. Defaults to true when there are two or more sections. */
  sortableSections?: boolean;
  /** ↕️ Fires after a section-handle reorder with the new section-id order (host may persist; Tree also keeps an internal merge so program re-renders do not snap order back). */
  onSectionsReorder?: (orderedIds: readonly string[]) => void;
}

/** ↕️ Merges a remembered section-id order with the latest section list — keeps prior relative order for surviving ids and
 * inserts a newly appeared section where the source puts it: right after the nearest earlier source section already
 * placed, else first. Appending it instead sank a section a program leads with (the history-edit band and draft editor)
 * below every section the tree already showed. */
export function mergeTreeSectionOrder(previousIds: readonly string[], sections: readonly TreeDataSection[]): TreeDataSection[] {
  const byId = new Map(sections.map((section) => [section.id, section]));
  const ordered: TreeDataSection[] = previousIds.flatMap((id) => byId.get(id) ?? []);
  const remembered = new Set(ordered.map((section) => section.id));
  sections.forEach((section, index) => {
    if (remembered.has(section.id)) return;
    const anchor = sections.slice(0, index).findLast((earlier) => ordered.includes(earlier));
    ordered.splice(anchor === undefined ? 0 : ordered.indexOf(anchor) + 1, 0, section);
  });
  return ordered;
}

/** 🌲️ Stable context for hoisted Tree data rows (avoids remounting rows when Tree re-renders). */
export interface TreeDataRenderingContextValue {
  readonly sectionItemsById: Record<string, TreeDataItem[]>;
  readonly itemItemsById: Record<string, TreeDataItem[]>;
  readonly loadingById: Record<string, boolean>;
  readonly dragAndDropController?: TreeDragAndDropController;
  readonly loadSectionItems: (section: TreeDataSection) => Promise<void>;
  readonly loadItemItems: (item: TreeDataItem) => Promise<void>;
  readonly handleSelectItem: (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => void;
  readonly handleDoubleClickItem: (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => void;
  readonly handleDragStart: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  readonly handleDragEnd: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  readonly handleDropOnItem: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  readonly handleDropOnSection: (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => void;
  readonly handleDragOver: (event: React.DragEvent<HTMLDivElement>) => void;
  readonly handleDragOverItem: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem) => void;
  readonly draggedIds: readonly string[];
  readonly dropPreview: { readonly targetId: string; readonly position: TreeDropPosition } | null;
  readonly buildPalettePointerProps: (item: TreeDataItem, section: TreeDataSection) => Pick<TreeItemProps, "onPointerDown">;
  /** ↕️ Section headers render reorder grips and accept sibling-section drops. */
  readonly sortableSections: boolean;
  readonly draggedSectionId: string | null;
  readonly handleSectionDragStart: (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => void;
  readonly handleSectionDragEnd: () => void;
  readonly handleSectionDragOver: (event: React.DragEvent<HTMLDivElement>) => void;
  readonly handleSectionDrop: (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => void;
}

export const TreeDataRenderingContext = reactHostPort.createContext<TreeDataRenderingContextValue | null>(null);

export function useTreeDataRendering(): TreeDataRenderingContextValue {
  const value = reactHostPort.useContext(TreeDataRenderingContext);
  if (!value) {
    throw new Error("Tree data row components must render inside Tree");
  }
  return value;
}

export const treeDefaultDragMimeKind = "application/vnd.code.tree.item";

export const getTreeSectionStateId = (sectionId: string): string => `tree-section-${sectionId}`;

export const getTreeItemStateId = (itemId: string): string => `tree-item-${itemId}`;

export const getTreeSectionLoadingId = (sectionId: string): string => `tree-section-loading-${sectionId}`;

export const getTreeItemLoadingId = (itemId: string): string => `tree-item-loading-${itemId}`;

/** 🧺️ A section's rows: its own `items` straight off the prop — the truth of THIS render, never a copy held in state, which
 * is one render behind the prop it was copied from — else the rows its `getItems` loaded (`loadedSectionItemsById`). A row
 * control bound to an external store by a position its parent re-binds (the interpreter's per-node views after a body
 * refresh renumbered the nodes) would otherwise render once against rows the prop no longer holds: it was unmounted and
 * mounted anew, and an open editor, a typed draft and the focus were lost. */
export const getTreeSectionItems = (section: TreeDataSection, loadedSectionItemsById: Record<string, TreeDataItem[]>): TreeDataItem[] => section.items ?? loadedSectionItemsById[section.id] ?? [];

export const getTreeItemItems = (item: TreeDataItem, itemItemsById: Record<string, TreeDataItem[]>): TreeDataItem[] => itemItemsById[item.id] ?? item.items ?? [];

/**
 * Data interface for a node in a file tree.
 **/
export interface FileTreeNode {
  title: string;
  path: string;
  icon?: string;
  isFolder: boolean;
  children?: FileTreeNode[];
}

/**
 * TreeFilesProps holds the data fields for a TreeFilesProps record.
 **/
export interface TreeFilesProps {
  title?: UiLabel;
  nodes: FileTreeNode[];
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
  className?: string;
}

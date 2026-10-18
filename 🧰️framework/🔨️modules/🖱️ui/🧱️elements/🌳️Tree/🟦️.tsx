// #region 🧲️Header
// 💻️ framework/ui/elements/🪵️Tree/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

import type { TreeWindowRowExtent } from "@semio-tech/framework";
import { retainTreeFocus } from "./🎯️focus/🟦️.ts";

/** 📏️ Re-exported from the UI contract because this element's whole window API is stated in it — a consumer of `TreeDataWindow`/`TreeWindowContainerMeasure` must be able to name the token without reaching past this module. */
export type { TreeWindowRowExtent };
import * as React from "react";
import { STYLING_DOM, uiSpacingLen } from "@semio-tech/ui-styling";
export { uiSpacingLen };
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext/.useState at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import type { UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { FlowProvider } from "../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { TREE_SECTION_REORDER_MIME } from "../../🔨️modules/🧲️panel-dock/🟦️.tsx";
import { usePanelGhost, interactionMergeFromModifiers } from "../👻️PanelGhost/🟦️.tsx";
import { DocumentIcon, FolderIcon } from "../🔣️Icons/🟦️.tsx";
// 🕹️wave-0: imported directly from the module's own source (see react/🟦️.tsx's header comment on
// this same import for why not via `@semio-tech/framework`'s barrel).

import { TreeContext, detailPanelIndentPx } from "./📏️row-alignment/🟦️.tsx";
import { EMPTY_TREE_SECTIONS, TreeDataRenderingContext, type TreeDataRenderingContextValue, type TreeItemProps, type TreeRootProps, getTreeItemLoadingId, getTreeSectionItems, getTreeSectionLoadingId, mergeTreeSectionOrder, treeDefaultDragMimeKind } from "./🧩️props/🟦️.ts";
import { TreeStateProvider, hasNonEmptyChildren, useTreeOpenState } from "./🌿️state/🟦️.tsx";
import type { TreeDataItem, TreeDataSection } from "./📋️data/🟦️.ts";
import { TreeHighlightContext, TreeSelectionContext, collectTreeItemMap, createTreeHighlightStore, createTreeSelectionStore, getTreeNextSelectionState, normalizeTreeSelectedIds } from "./🎚️selection/🟦️.ts";
import { type TreeDropPosition, TreeReorderDropPreview, isTreeReorderDragEvent, resolveTreeDropPosition } from "./🧲️drag-drop/🟦️.tsx";
import { getTreeItemOrderedIds } from "./🎨️chrome/🟦️.tsx";
import { getTreeDropData } from "./📄️row/🟦️.tsx";
import { useTreeHoverPathRootHandlers, useTreeSelectionPathSync } from "./🧷️path-marking/🟦️.ts";
import { TreeHoverPathRefreshContext } from "./🌲️branch/🟦️.tsx";
import { type TreeComponent, TreeDataSectionView } from "./🪢️views/🟦️.tsx";

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

export const Tree = (({
  className = "",
  presentation = "standard",
  showLines = true,
  sections,
  selectionMode = "single",
  selectedIds: controlledSelectedIds,
  defaultSelectedIds = [],
  onSelectionChange,
  highlightedIds: controlledHighlightedIds = [],
  dragAndDropController,
  emptyState,
  indentMultiplier = 1,
  direction = "down",
  openStates,
  onOpenStateChange,
  sortableSections: sortableSectionsProp,
  onSectionsReorder,
  children,
}: TreeRootProps & { children?: React.ReactNode }) => {
  if (hasNonEmptyChildren(children)) {
    throw new Error("Tree only accepts section data through the sections prop.");
  }
  const panelGhost = usePanelGhost();
  const [sectionItemsById, setSectionItemsById] = reactHostPort.useState<Record<string, TreeDataItem[]>>({});
  const [itemItemsById, setItemItemsById] = reactHostPort.useState<Record<string, TreeDataItem[]>>({});
  const [loadingById, setLoadingById] = reactHostPort.useState<Record<string, boolean>>({});
  const [uncontrolledSelectedIds, setUncontrolledSelectedIds] = reactHostPort.useState<string[]>(() => normalizeTreeSelectedIds(defaultSelectedIds, selectionMode));
  const [draggedIds, setDraggedIds] = reactHostPort.useState<string[]>([]);
  const [dropPreview, setDropPreview] = reactHostPort.useState<{ targetId: string; position: TreeDropPosition } | null>(null);
  const [draggedSectionId, setDraggedSectionId] = reactHostPort.useState<string | null>(null);
  const resolvedSections = sections ?? EMPTY_TREE_SECTIONS;
  const sortableSections = sortableSectionsProp ?? resolvedSections.length > 1;
  const [sectionOrderIds, setSectionOrderIds] = reactHostPort.useState<readonly string[]>(() => resolvedSections.map((section) => section.id));
  reactHostPort.useEffect(() => {
    setSectionOrderIds((previous) => {
      const merged = mergeTreeSectionOrder(previous, resolvedSections).map((section) => section.id);
      if (merged.length === previous.length && merged.every((id, index) => id === previous[index])) return previous;
      return merged;
    });
  }, [resolvedSections]);
  const orderedByPreference = reactHostPort.useMemo(() => mergeTreeSectionOrder(sectionOrderIds, resolvedSections), [resolvedSections, sectionOrderIds]);
  const suppressPaletteClickRef = reactHostPort.useRef(false);
  const palettePointerGestureRef = reactHostPort.useRef<{ pending: boolean; dragging: boolean; encoded: string | null; types: readonly string[]; startX: number; startY: number; target: EventTarget | null }>({
    pending: false,
    dragging: false,
    encoded: null,
    types: [],
    startX: 0,
    startY: 0,
    target: null,
  });
  const palettePointerWindowCleanupRef = reactHostPort.useRef<(() => void) | null>(null);
  const clearPalettePointerWindowListeners = reactHostPort.useCallback(() => {
    palettePointerWindowCleanupRef.current?.();
    palettePointerWindowCleanupRef.current = null;
  }, []);
  reactHostPort.useEffect(() => () => clearPalettePointerWindowListeners(), [clearPalettePointerWindowListeners]);
  const anchorIdRef = reactHostPort.useRef<string | undefined>(normalizeTreeSelectedIds(defaultSelectedIds, selectionMode)[0]);
  const resolvedSelectedIds = reactHostPort.useMemo(() => normalizeTreeSelectedIds(controlledSelectedIds ?? uncontrolledSelectedIds, selectionMode), [controlledSelectedIds, uncontrolledSelectedIds, selectionMode]);
  const [selectionStore] = reactHostPort.useState(createTreeSelectionStore);
  const selectionStoreRef = reactHostPort.useRef(selectionStore);
  selectionStoreRef.current = selectionStore;
  const [highlightStore] = reactHostPort.useState(createTreeHighlightStore);
  const resolvedHighlightedIds = reactHostPort.useMemo(() => normalizeTreeSelectedIds(controlledHighlightedIds, "multiple"), [controlledHighlightedIds]);

  reactHostPort.useEffect(() => {
    selectionStore.setSelectedIds(resolvedSelectedIds);
  }, [resolvedSelectedIds, selectionStore]);

  reactHostPort.useLayoutEffect(() => {
    highlightStore.setHighlightedIds(resolvedHighlightedIds);
  }, [highlightStore, resolvedHighlightedIds]);

  reactHostPort.useEffect(() => {
    setSectionItemsById((previous) => (Object.keys(previous).length === 0 ? previous : {}));
  }, [resolvedSections]);

  const itemMap = reactHostPort.useMemo(() => {
    const map: Record<string, TreeDataItem> = {};
    resolvedSections.forEach((section) => {
      collectTreeItemMap(getTreeSectionItems(section, sectionItemsById), map);
    });
    Object.values(itemItemsById).forEach((items) => {
      collectTreeItemMap(items, map);
    });
    return map;
  }, [itemItemsById, resolvedSections, sectionItemsById]);

  const updateSelection = reactHostPort.useCallback(
    (nextSelectedIds: string[]) => {
      const normalizedIds = normalizeTreeSelectedIds(nextSelectedIds, selectionMode);
      if (controlledSelectedIds === undefined) {
        setUncontrolledSelectedIds(normalizedIds);
      }
      onSelectionChange?.(normalizedIds, normalizedIds.map((id) => itemMap[id]).filter((item):item is TreeDataItem=>item!==undefined));
    },
    [controlledSelectedIds, itemMap, onSelectionChange, selectionMode],
  );

  const loadSectionItems = reactHostPort.useCallback(
    async (section: TreeDataSection) => {
      if (!section.getItems || section.items !== undefined || sectionItemsById[section.id] !== undefined || loadingById[getTreeSectionLoadingId(section.id)]) {
        return;
      }
      setLoadingById((previousItems) => ({ ...previousItems, [getTreeSectionLoadingId(section.id)]: true }));
      try {
        const nextItems = await section.getItems();
        setSectionItemsById((previousItems) => ({ ...previousItems, [section.id]: nextItems }));
      } finally {
        setLoadingById((previousItems) => ({ ...previousItems, [getTreeSectionLoadingId(section.id)]: false }));
      }
    },
    [loadingById, sectionItemsById],
  );

  const loadItemItems = reactHostPort.useCallback(
    async (item: TreeDataItem) => {
      if (!item.getItems || itemItemsById[item.id] !== undefined || loadingById[getTreeItemLoadingId(item.id)]) {
        return;
      }
      setLoadingById((previousItems) => ({ ...previousItems, [getTreeItemLoadingId(item.id)]: true }));
      try {
        const nextItems = await item.getItems();
        setItemItemsById((previousItems) => ({ ...previousItems, [item.id]: nextItems }));
      } finally {
        setLoadingById((previousItems) => ({ ...previousItems, [getTreeItemLoadingId(item.id)]: false }));
      }
    },
    [itemItemsById, loadingById],
  );

  const handleDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    const types = [...event.dataTransfer.types];
    const acceptsCopy = types.some((kind) => kind !== treeDefaultDragMimeKind && kind.startsWith("application/"));
    event.dataTransfer.dropEffect = acceptsCopy ? "copy" : "move";
  }, []);

  const handleDragOverItem = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem) => {
      handleDragOver(event);
      if (isTreeReorderDragEvent(event)) {
        const position = resolveTreeDropPosition(event);
        setDropPreview((previous) => (previous?.targetId === item.id && previous.position === position ? previous : { targetId: item.id, position }));
        return;
      }
      setDropPreview(null);
    },
    [handleDragOver],
  );

  const handleSelectItem = reactHostPort.useCallback(
    (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
      if (suppressPaletteClickRef.current) {
        suppressPaletteClickRef.current = false;
        return;
      }
      const currentSelectedIds = selectionStoreRef.current.getSelectedIds();
      const orderedIds = getTreeItemOrderedIds(resolvedSections, sectionItemsById, itemItemsById);
      // 🎯️ Routed through the ONE shared modifier→merge policy (see `interactionMergeFromModifiers`'s
      // doc) rather than hand-checking `event.metaKey`/`event.ctrlKey`/`event.shiftKey` here directly.
      const merge = interactionMergeFromModifiers(event);
      const nextSelection = getTreeNextSelectionState({
        selectionMode,
        selectedIds: currentSelectedIds,
        orderedIds,
        targetId: item.id,
        anchorId: anchorIdRef.current,
        additiveKey: merge === "invertive",
        rangeKey: merge === "range",
      });
      anchorIdRef.current = nextSelection.anchorId;
      updateSelection(nextSelection.selectedIds);
      item.onClick?.(event, { path, selectedIds: nextSelection.selectedIds, sectionId: section.id });
    },
    [itemItemsById, resolvedSections, sectionItemsById, selectionMode, updateSelection],
  );

  const handleDoubleClickItem = reactHostPort.useCallback((event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
    item.onDoubleClick?.(event, { path, selectedIds: selectionStoreRef.current.getSelectedIds(), sectionId: section.id });
  }, []);

  const handleDragStart = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
      event.stopPropagation();
      const currentSelectedIds = selectionStoreRef.current.getSelectedIds();
      const nextDraggedIds = currentSelectedIds.includes(item.id) ? [...currentSelectedIds] : [item.id];
      const sourceItems = nextDraggedIds.map((id) => itemMap[id]).filter((item):item is TreeDataItem=>item!==undefined);
      setDraggedIds(nextDraggedIds);
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData(treeDefaultDragMimeKind, JSON.stringify(nextDraggedIds));
      const customData = dragAndDropController?.getDragData?.({ items: sourceItems, sourceItem: item, section }) ?? item.dragData;
      Object.entries(customData ?? {}).forEach(([kind, value]) => {
        event.dataTransfer.setData(kind, value);
      });
      if (customData && Object.keys(customData).length > 0) {
        event.dataTransfer.effectAllowed = "copy";
        const labelText = typeof item.label === "string" ? item.label : typeof item.label === "number" ? String(item.label) : "Kind";
        const ghost = document.createElement("div");
        ghost.textContent = labelText;
        ghost.setAttribute("data-puzzle3d-fixture-drag-ghost", "true");
        ghost.setAttribute("data-level", "panel");
        ghost.className = cn("border-primary text-foreground pointer-events-none fixed left-offscreen top-0 z-tutorial rounded-md border px-2 py-1 text-xs shadow-md", surfaceClass);
        document.body.appendChild(ghost);
        event.dataTransfer.setDragImage(ghost, ghost.offsetWidth / 2, ghost.offsetHeight / 2);
        requestAnimationFrame(() => ghost.remove());
        panelGhost?.begin(event.currentTarget);
      }
      dragAndDropController?.onDragStart?.({ items: sourceItems, sourceItem: item, section });
    },
    [dragAndDropController, itemMap, panelGhost],
  );

  const handleDragEnd = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
      const sourceIds = draggedIds.length > 0 ? draggedIds : [item.id];
      const sourceItems = sourceIds.map((id) => itemMap[id]).filter((item):item is TreeDataItem=>item!==undefined);
      dragAndDropController?.onDragEnd?.({ items: sourceItems, sourceItem: item, section });
      setDraggedIds([]);
      setDropPreview(null);
      panelGhost?.end();
    },
    [dragAndDropController, draggedIds, itemMap, panelGhost],
  );

  const handleDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, target: TreeDataItem | TreeDataSection, targetKind: "item" | "section", section: TreeDataSection) => {
      event.preventDefault();
      const sourceIds = draggedIds.length > 0 ? draggedIds : JSON.parse(event.dataTransfer.getData(treeDefaultDragMimeKind) || "[]");
      const dropPosition = targetKind === "item" ? resolveTreeDropPosition(event) : undefined;
      dragAndDropController?.handleDrop?.({
        target,
        targetKind,
        data: getTreeDropData(event),
        sourceItems: sourceIds.map((id: string) => itemMap[id]).filter(Boolean),
        section,
        dropPosition,
      });
      setDraggedIds([]);
      setDropPreview(null);
    },
    [dragAndDropController, draggedIds, itemMap],
  );

  const resolveItemDragData = reactHostPort.useCallback(
    (treeItem: TreeDataItem, treeSection: TreeDataSection) => dragAndDropController?.getDragData?.({ items: [treeItem], sourceItem: treeItem, section: treeSection }) ?? treeItem.dragData,
    [dragAndDropController],
  );

  const buildPalettePointerProps = reactHostPort.useCallback(
    (item: TreeDataItem, section: TreeDataSection): Pick<TreeItemProps, "onPointerDown"> => {
      const palettePointer = dragAndDropController?.pointerPaletteDrag;
      if (!palettePointer) {
        return {};
      }
      const beginPalettePointerDrag = (): void => {
        const gesture = palettePointerGestureRef.current;
        if (!gesture.pending || !gesture.encoded) {
          return;
        }
        gesture.pending = false;
        gesture.dragging = true;
        suppressPaletteClickRef.current = true;
        palettePointer.begin(gesture.encoded, gesture.types);
        panelGhost?.begin(gesture.target);
        dragAndDropController?.onDragStart?.({ items: [item], sourceItem: item, section });
      };
      const finishPalettePointerGesture = (): void => {
        clearPalettePointerWindowListeners();
        if (palettePointerGestureRef.current.dragging) {
          suppressPaletteClickRef.current = true;
          panelGhost?.end();
        }
        palettePointerGestureRef.current = { pending: false, dragging: false, encoded: null, types: [], startX: 0, startY: 0, target: null };
      };
      return {
        onPointerDown: (event) => {
          if (event.button !== 0) {
            return;
          }
          const dragData = resolveItemDragData(item, section);
          if (!dragData) {
            return;
          }
          const encoded = palettePointer.readEncodedDragPayload(dragData);
          if (!encoded) {
            return;
          }
          clearPalettePointerWindowListeners();
          palettePointerGestureRef.current = { pending: true, dragging: false, encoded, types: Object.keys(dragData), startX: event.clientX, startY: event.clientY, target: event.currentTarget };
          event.preventDefault();
          event.stopPropagation();
          const onWindowPointerMove = (moveEvent: PointerEvent): void => {
            const gesture = palettePointerGestureRef.current;
            if (!gesture.pending && !gesture.dragging) {
              return;
            }
            const deltaX = moveEvent.clientX - gesture.startX;
            const deltaY = moveEvent.clientY - gesture.startY;
            if (gesture.pending && deltaX * deltaX + deltaY * deltaY < 36) {
              return;
            }
            beginPalettePointerDrag();
          };
          const onWindowPointerUp = (): void => {
            if (palettePointerGestureRef.current.pending) {
              finishPalettePointerGesture();
              return;
            }
            if (palettePointerGestureRef.current.dragging) {
              dragAndDropController?.onDragEnd?.({ items: [item], sourceItem: item, section });
            }
            finishPalettePointerGesture();
          };
          const onWindowPointerCancel = (): void => {
            if (palettePointerGestureRef.current.dragging) {
              palettePointer.cancel();
              dragAndDropController?.onDragEnd?.({ items: [item], sourceItem: item, section });
              panelGhost?.end();
            }
            finishPalettePointerGesture();
          };
          window.addEventListener("pointermove", onWindowPointerMove);
          window.addEventListener("pointerup", onWindowPointerUp, true);
          window.addEventListener("pointercancel", onWindowPointerCancel, true);
          palettePointerWindowCleanupRef.current = () => {
            window.removeEventListener("pointermove", onWindowPointerMove);
            window.removeEventListener("pointerup", onWindowPointerUp, true);
            window.removeEventListener("pointercancel", onWindowPointerCancel, true);
          };
        },
      };
    },
    [clearPalettePointerWindowListeners, dragAndDropController, panelGhost, resolveItemDragData],
  );

  const handleDropOnItem = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
      handleDrop(event, item, "item", section);
    },
    [handleDrop],
  );

  const handleDropOnSection = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => {
      handleDrop(event, section, "section", section);
    },
    [handleDrop],
  );

  const handleSectionDragStart = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => {
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData(TREE_SECTION_REORDER_MIME, section.id);
    setDraggedSectionId(section.id);
  }, []);

  const handleSectionDragEnd = reactHostPort.useCallback(() => {
    setDraggedSectionId(null);
  }, []);

  const handleSectionDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
    if (event.dataTransfer.types.includes(TREE_SECTION_REORDER_MIME)) {
      event.preventDefault();
      event.dataTransfer.dropEffect = "move";
    }
  }, []);

  const handleSectionDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, targetSection: TreeDataSection) => {
      if (!event.dataTransfer.types.includes(TREE_SECTION_REORDER_MIME)) return;
      event.preventDefault();
      event.stopPropagation();
      const sourceId = event.dataTransfer.getData(TREE_SECTION_REORDER_MIME) || draggedSectionId;
      setDraggedSectionId(null);
      if (!sourceId || sourceId === targetSection.id) return;
      setSectionOrderIds((previous) => {
        const current = mergeTreeSectionOrder(previous, resolvedSections).map((section) => section.id);
        const fromIndex = current.indexOf(sourceId);
        const toIndex = current.indexOf(targetSection.id);
        if (fromIndex < 0 || toIndex < 0) return previous;
        const next = [...current];
        const [moved] = next.splice(fromIndex, 1);
        if (!moved) return previous;
        next.splice(toIndex, 0, moved);
        onSectionsReorder?.(next);
        return next;
      });
    },
    [draggedSectionId, onSectionsReorder, resolvedSections],
  );

  const treeDataRenderingValue = reactHostPort.useMemo<TreeDataRenderingContextValue>(
    () => ({
      sectionItemsById,
      itemItemsById,
      loadingById,
      dragAndDropController,
      loadSectionItems,
      loadItemItems,
      handleSelectItem,
      handleDoubleClickItem,
      handleDragStart,
      handleDragEnd,
      handleDropOnItem,
      handleDropOnSection,
      handleDragOver,
      handleDragOverItem,
      draggedIds,
      dropPreview,
      buildPalettePointerProps,
      sortableSections,
      draggedSectionId,
      handleSectionDragStart,
      handleSectionDragEnd,
      handleSectionDragOver,
      handleSectionDrop,
    }),
    [
      buildPalettePointerProps,
      dragAndDropController,
      draggedIds,
      draggedSectionId,
      dropPreview,
      handleDoubleClickItem,
      handleDragEnd,
      handleDragOver,
      handleDragOverItem,
      handleDragStart,
      handleDropOnItem,
      handleDropOnSection,
      handleSectionDragEnd,
      handleSectionDragOver,
      handleSectionDragStart,
      handleSectionDrop,
      handleSelectItem,
      itemItemsById,
      loadItemItems,
      loadSectionItems,
      loadingById,
      sectionItemsById,
      sortableSections,
    ],
  );

  const { treeRootRef, handleTreePointerOver, handleTreePointerLeave, refreshTreeHoverPath } = useTreeHoverPathRootHandlers();
  useTreeSelectionPathSync(treeRootRef, resolvedSelectedIds);
  reactHostPort.useLayoutEffect(() => {
    const root = treeRootRef.current;
    return root ? retainTreeFocus(root) : undefined;
  }, [treeRootRef]);

  const orderedSections = direction === "up" ? [...orderedByPreference].reverse() : orderedByPreference;

  return (
    <FlowProvider inline="ltr">
      <TreeStateProvider openStates={openStates} onOpenStateChange={onOpenStateChange}>
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines, isTree: true, indentMultiplier, direction }}>
          <TreeReorderDropPreview preview={dropPreview} />
          <div
            ref={treeRootRef}
            data-slot="tree"
            tabIndex={-1}
            data-tree-presentation={presentation}
            role="tree"
            aria-multiselectable={selectionMode === "multiple" ? true : undefined}
            dir="auto"
            className={`w-full min-w-0 overflow-hidden ${className}`}
            style={{
              "--tree-row-height": presentation === "compact" ? "auto" : "var(--size-workbench)",
              "--tree-row-min-height": presentation === "compact" ? "calc(var(--size-tiny) * 1.5)" : "var(--size-workbench)",
              "--tree-row-max-height": presentation === "compact" ? "none" : "var(--size-workbench)",
              "--tree-label-size": presentation === "compact" ? "var(--size-tiny)" : "var(--text-xs)",
              "--tree-label-line-height": presentation === "compact" ? "1.5" : "1",
              "--tree-gutter-center": presentation === "compact" ? "50%" : "calc(var(--size-workbench) / 2)",
              "--tree-inline-control-height": presentation === "compact" ? "var(--size-small)" : "var(--size-medium)",
              "--tree-value-justify": presentation === "compact" ? "flex-start" : "flex-end",
              "--tree-value-column": uiSpacingLen(presentation === "compact" ? STYLING_DOM.windowMeasureValueColumnUiSpacing : STYLING_DOM.controlValueColumnUiSpacing),
            } as React.CSSProperties}
            onPointerOver={handleTreePointerOver}
            onPointerLeave={handleTreePointerLeave}
          >
            <TreeHoverPathRefreshContext.Provider value={refreshTreeHoverPath}>
              <TreeSelectionContext.Provider value={selectionStore}>
                <TreeHighlightContext.Provider value={highlightStore}>
                  <TreeDataRenderingContext.Provider value={treeDataRenderingValue}>
                    {orderedSections.map((section, sectionIndex) => (
                      <div key={section.id} data-slot="tree-section-wrapper" className="w-full min-w-0">
                        <TreeDataSectionView section={section} isLastSection={sectionIndex === orderedSections.length - 1} />
                      </div>
                    ))}
                  </TreeDataRenderingContext.Provider>
                </TreeHighlightContext.Provider>
              </TreeSelectionContext.Provider>
            </TreeHoverPathRefreshContext.Provider>
            {resolvedSections.length === 0 && emptyState ? (
              <div data-slot="tree-empty-state" dir="auto" className="w-full min-w-0">
                {emptyState}
              </div>
            ) : null}
          </div>
        </TreeContext.Provider>
      </TreeStateProvider>
    </FlowProvider>
  );
}) as TreeComponent;


interface FileTreeItemProps {
  node: FileTreeNode;
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
}

/**
 * FileTreeItem holds the data fields for a FileTreeItem record.
 **/
const FileTreeItem: React.FC<FileTreeItemProps> = ({ node, currentPath, onNavigate, as = "a" }) => {
  const { level, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const itemId = `file-${node.path}`;
  const { open, setOpen } = useTreeOpenState(itemId, true);

  const isActive = currentPath === node.path;
  const hasChildren = node.children && node.children.length > 0;
  const Icon = node.isFolder ? FolderIcon : DocumentIcon;

  const baseClasses = "flex items-center gap-single text-sm rounded-small cursor-selectable select-none transition-colors hover:bg-hover-interactive-fill";
  const stateClasses = isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:text-emphasized";
  const itemClasses = `${baseClasses} ${stateClasses}`;
  const handleClick = (e: React.MouseEvent) => {
    if (hasChildren) {
      e.preventDefault();
      setOpen(!open);
    }
    if (onNavigate) {
      onNavigate(node.path);
    }
  };

  const content = (
    <>
      {node.icon ? <span className="text-sm shrink-0">{node.icon}</span> : <Icon className="size-tiny shrink-0" />}
      <span className="text-sm">{node.title}</span>
    </>
  );

  const sharedProps = {
    className: itemClasses,
    style: { paddingInlineStart: `${detailPanelIndentPx(level, indentMultiplier) + 12}px` },
    onClick: handleClick,
  };

  const itemElement =
    as === "a" ? (
      <a href={`/${node.path}`} {...sharedProps}>
        {content}
      </a>
    ) : (
      <div {...sharedProps}>{content}</div>
    );

  if (hasChildren && node.isFolder) {
    return (
      <>
        {itemElement}
        {open && (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [], showLines: false, isTree, indentMultiplier }}>
            {node.children!.map((child, idx) => (
              <FileTreeItem key={idx} node={child} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </TreeContext.Provider>
        )}
      </>
    );
  }

  return itemElement;
};

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

const TreeFiles: React.FC<TreeFilesProps> = ({ title, nodes, currentPath, onNavigate, as = "a", className = "" }) => {
  return (
    <TreeStateProvider>
      <div className={`not-prose my-medium p-medium rounded-lg border bg-card ${className}`}>
        {title && <h3 className="text-lg font-semibold mb-4">{title}</h3>}
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false, isTree: true, indentMultiplier: 1 }}>
          <div className="flex flex-col gap-single">
            {nodes.map((node, idx) => (
              <FileTreeItem key={idx} node={node} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </div>
        </TreeContext.Provider>
      </div>
    </TreeStateProvider>
  );
};

Tree.Files = TreeFiles;
Tree.Section = Tree.Files;

/** Alias for Tree.Files rendering a file tree from FileTreeNode data.
 **/
export const FileTree = TreeFiles;
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import type { TreeWindowRowExtent } from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
import * as React from "react";
import { TreeContext } from "../📏️row-alignment/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { TREE_SECTION_REORDER_MIME } from "../../../🔨️modules/🧲️panel-dock/🟦️.tsx";
import { type TreeDataItem, type TreeDataSection, TreeItemCollapsibleState } from "../📋️data/🟦️.ts";
import { type TreeRootProps, getTreeItemItems, getTreeItemLoadingId, getTreeItemStateId, getTreeSectionItems, getTreeSectionLoadingId, getTreeSectionStateId, useTreeDataRendering } from "../🧩️props/🟦️.ts";
import { getTreeItemDefaultOpen, useTreeItemRowHighlighted, useTreeItemRowSelected } from "../🎚️selection/🟦️.ts";
import { renderTreeWindowSpacer, treeWindowDomAttributes, treeWindowRowIndexOf, treeWindowSpacerRows } from "../🔭️virtual-window/🟦️.tsx";
import { useTreeOpenState } from "../🌿️state/🟦️.tsx";
import { deriveTreeDragRoles } from "../⚡️actions/🟦️.tsx";
import { TreeItem } from "../🍃️item/🟦️.tsx";
import { HelperRow, getTreeItemLabel } from "../📄️row/🟦️.tsx";
import { TreeContent } from "../🌲️branch/🟦️.tsx";
import { TreeSection } from "../🌱️section/🟦️.tsx";
import type { TreeFilesProps } from "../🧩️props/🟦️.ts";

/** 🌿️ Hoisted data-tree item row (stable component type across Tree re-renders). */
const TreeDataItemView = reactHostPort.memo(function TreeDataItemView(props: { readonly item: TreeDataItem; readonly section: TreeDataSection; readonly path: readonly string[]; readonly isLastItem: boolean; readonly windowRowIndex?: number; readonly windowRowTotal?: number; readonly windowRowExtent?: TreeWindowRowExtent }): React.ReactElement {
  const { item, section, path, isLastItem, windowRowIndex, windowRowTotal, windowRowExtent } = props;
  const { direction = "down" } = reactHostPort.useContext(TreeContext);
  const { itemItemsById, loadingById, dragAndDropController, loadItemItems, handleSelectItem, handleDoubleClickItem, handleDragStart, handleDragEnd, handleDragOverItem, handleDropOnItem, buildPalettePointerProps, draggedIds } =
    useTreeDataRendering();
  const isRowSelected = useTreeItemRowSelected(item.id, item.isSelected);
  const isRowHighlighted = useTreeItemRowHighlighted(item.id, item.isHighlighted);
  const isDragging = draggedIds.includes(item.id);
  const baseChildItems = getTreeItemItems(item, itemItemsById);
  const alternatives = item.alternatives ?? [];
  const branchCount = alternatives.length;
  const [activeBranchIndex, setActiveBranchIndex] = reactHostPort.useState(0);
  const clampedBranchIndex = branchCount > 0 ? Math.min(activeBranchIndex, branchCount - 1) : 0;
  const rawChildItems = branchCount > 0 ? (alternatives[clampedBranchIndex] ?? []) : baseChildItems;
  const childItems = direction === "up" ? [...rawChildItems].reverse() : rawChildItems;
  const childWindow = branchCount > 0 ? undefined : item.window;
  const windowTotal = Math.max(0, Math.floor(childWindow?.total ?? 0));
  const spacerRows = treeWindowSpacerRows(childWindow, childItems.length);
  const isWaiting = Boolean(item.waiting);
  const hasDynamicChildren = Boolean(item.getItems);
  const hasExpandableChildren = childItems.length > 0 || hasDynamicChildren || Boolean(item.emptyState) || branchCount > 0 || windowTotal > 0;
  const isExpandable = item.collapsibleState === TreeItemCollapsibleState.None ? false : hasExpandableChildren;
  const hasControl = Boolean(item.control);
  const propertyLayout = hasControl;
  const hasNestedTreeItems = childItems.length > 0 || hasDynamicChildren || Boolean(item.emptyState) || branchCount > 0 || windowTotal > 0;
  const defaultOpen = hasControl ? !hasNestedTreeItems || getTreeItemDefaultOpen(item) : getTreeItemDefaultOpen(item);
  const treeOpenState = useTreeOpenState(getTreeItemStateId(item.id), defaultOpen);
  const propertyExpandable = hasControl ? hasNestedTreeItems : isExpandable;
  const isWindowPending = windowTotal > 0 && childItems.length === 0 && treeOpenState.open;
  const isLoading = (loadingById[getTreeItemLoadingId(item.id)] ?? false) || Boolean(item.loading) || isWindowPending;

  reactHostPort.useEffect(() => {
    if (treeOpenState.open && hasDynamicChildren) {
      void loadItemItems(item);
    }
  }, [hasDynamicChildren, item, loadItemItems, treeOpenState.open]);

  const palettePointerProps = buildPalettePointerProps(item, section);
  const dragRoles = reactHostPort.useMemo(() => deriveTreeDragRoles(item, Boolean(dragAndDropController?.pointerPaletteDrag)), [dragAndDropController?.pointerPaletteDrag, item]);
  const palettePointerClassName = dragAndDropController?.pointerPaletteDrag && dragRoles.includes("transfer") ? "touch-none" : undefined;

  return (
    <TreeItem
      id={item.id}
      focusPath={path}
      label={hasControl ? item.label : getTreeItemLabel(item)}
      description={hasControl ? item.description : undefined}
      icon={item.icon}
      className={cn(item.className, palettePointerClassName)}
      isSelected={isRowSelected}
      isHighlighted={isRowHighlighted}
      isHidden={item.isHidden}
      isDragging={isDragging}
      isDragHandle={item.isDragHandle}
      defaultOpen={defaultOpen}
      open={treeOpenState.open}
      onOpenChange={treeOpenState.setOpen}
      expandable={propertyExpandable}
      loading={isLoading}
      waiting={isWaiting}
      isLastItem={isLastItem}
      actions={item.actions}
      contextMenu={item.contextMenu}
      draggable={Boolean(item.draggable) || Boolean(item.dragData)}
      dragRoles={dragRoles}
      dragInitiation="handle"
      transferPointerDown={palettePointerProps.onPointerDown}
      dragData={item.dragData}
      isDropReady={draggedIds.length > 0 && !isDragging && Boolean(dragAndDropController?.handleDrop)}
      layoutKind={propertyLayout ? "property" : undefined}
      activatable={Boolean(item.onClick)}
      headerControl={hasControl && hasNestedTreeItems ? item.control : undefined}
      onClick={(event) => handleSelectItem(event, item, section, [...path])}
      onDoubleClick={(event) => handleDoubleClickItem(event, item, section, [...path])}
      onDragStart={(event) => handleDragStart(event, item, section)}
      onDragEnd={(event) => handleDragEnd(event, item, section)}
      onDragOver={(event) => handleDragOverItem(event, item)}
      onDrop={(event) => handleDropOnItem(event, item, section)}
      onPointerEnter={item.onPointerEnter}
      onPointerLeave={item.onPointerLeave}
      branchCount={branchCount}
      activeBranchIndex={clampedBranchIndex}
      onBranchChange={setActiveBranchIndex}
      windowAttributes={treeWindowDomAttributes(childWindow, childItems.length, item.windowKey, item.windowPath)}
      windowRowIndex={windowRowIndex}
      windowRowTotal={windowRowTotal}
      windowRowExtent={windowRowExtent}
    >
      {hasControl && !hasNestedTreeItems ? item.control : null}
      {renderTreeWindowSpacer(direction === "up" ? spacerRows.trailing : spacerRows.leading, direction === "up" ? "trailing" : "leading", childWindow?.rowExtent)}
      {childItems.map((childItem, index) => (
        <TreeDataItemView key={childItem.id} item={childItem} section={section} path={[...path, childItem.id]} isLastItem={index === childItems.length - 1} windowRowIndex={treeWindowRowIndexOf(childWindow, childItems.length, index, direction)} windowRowTotal={childWindow?.total} windowRowExtent={childWindow?.rowExtent} />
      ))}
      {renderTreeWindowSpacer(direction === "up" ? spacerRows.leading : spacerRows.trailing, direction === "up" ? "leading" : "trailing", childWindow?.rowExtent)}
      {!isLoading && childItems.length === 0 && item.emptyState && (
        <TreeItem>
          <TreeContent>{item.emptyState}</TreeContent>
        </TreeItem>
      )}
    </TreeItem>
  );
});

/** 🌿️ Hoisted data-tree section row (stable component type across Tree re-renders). */
export const TreeDataSectionView = reactHostPort.memo(function TreeDataSectionView(props: { readonly section: TreeDataSection; readonly isLastSection: boolean }): React.ReactElement {
  const { section, isLastSection } = props;
  const { direction = "down" } = reactHostPort.useContext(TreeContext);
  const {
    sectionItemsById,
    loadingById,
    loadSectionItems,
    handleDragOver,
    handleDropOnSection,
    dragAndDropController,
    draggedIds,
    sortableSections,
    draggedSectionId,
    handleSectionDragStart,
    handleSectionDragEnd,
    handleSectionDragOver,
    handleSectionDrop,
  } = useTreeDataRendering();
  const treeOpenState = useTreeOpenState(getTreeSectionStateId(section.id), section.defaultOpen ?? false);
  const rawItems = getTreeSectionItems(section, sectionItemsById);
  const items = direction === "up" ? [...rawItems].reverse() : rawItems;
  const childWindow = section.window;
  const windowTotal = Math.max(0, Math.floor(childWindow?.total ?? 0));
  const spacerRows = treeWindowSpacerRows(childWindow, items.length);
  const isWindowPending = windowTotal > 0 && items.length === 0 && treeOpenState.open;
  const isLoading = (loadingById[getTreeSectionLoadingId(section.id)] ?? false) || Boolean(section.loading) || isWindowPending;
  const isWaiting = Boolean(section.waiting);
  const hasDynamicChildren = Boolean(section.getItems);
  const isExpandable = items.length > 0 || hasDynamicChildren || Boolean(section.emptyState) || windowTotal > 0;
  const sectionReorderable = sortableSections || Boolean(section.draggable);
  const sectionDragging = draggedSectionId === section.id;
  const sectionDropReady = Boolean(draggedSectionId && draggedSectionId !== section.id) || (draggedIds.length > 0 && Boolean(dragAndDropController?.handleDrop));

  reactHostPort.useEffect(() => {
    if (treeOpenState.open && hasDynamicChildren) {
      void loadSectionItems(section);
    }
  }, [hasDynamicChildren, loadSectionItems, section, treeOpenState.open]);

  return (
    <TreeSection
      id={section.id}
      label={section.label}
      icon={section.icon}
      className={cn(section.className, sectionDragging && "opacity-40")}
      defaultOpen={section.defaultOpen}
      open={treeOpenState.open}
      onOpenChange={treeOpenState.setOpen}
      expandable={isExpandable}
      loading={isLoading}
      waiting={isWaiting}
      actions={section.actions}
      onPointerEnter={section.onPointerEnter}
      onPointerLeave={section.onPointerLeave}
      onDoubleClick={section.onDoubleClick}
      draggable={sectionReorderable}
      isDragHandle={sectionReorderable}
      dragInitiation="handle"
      onDragStart={sectionReorderable ? (event) => handleSectionDragStart(event, section) : undefined}
      onDragEnd={sectionReorderable ? () => handleSectionDragEnd() : undefined}
      onDragOver={(event) => {
        if (sectionReorderable) handleSectionDragOver(event);
        handleDragOver(event);
      }}
      onDrop={(event) => {
        if (event.dataTransfer.types.includes(TREE_SECTION_REORDER_MIME)) {
          handleSectionDrop(event, section);
          return;
        }
        handleDropOnSection(event, section);
      }}
      isLastSection={isLastSection}
      isDropReady={sectionDropReady}
      windowAttributes={treeWindowDomAttributes(childWindow, items.length, section.windowKey, section.windowPath)}
    >
      {renderTreeWindowSpacer(direction === "up" ? spacerRows.trailing : spacerRows.leading, direction === "up" ? "trailing" : "leading", childWindow?.rowExtent)}
      {items.map((item, index) => (
        <TreeDataItemView key={item.id} item={item} section={section} path={[section.id, item.id]} isLastItem={index === items.length - 1} windowRowIndex={treeWindowRowIndexOf(childWindow, items.length, index, direction)} windowRowTotal={childWindow?.total} windowRowExtent={childWindow?.rowExtent} />
      ))}
      {renderTreeWindowSpacer(direction === "up" ? spacerRows.leading : spacerRows.trailing, direction === "up" ? "leading" : "trailing", childWindow?.rowExtent)}
      {!isLoading && items.length === 0 && section.emptyState && <HelperRow>{section.emptyState}</HelperRow>}
    </TreeSection>
  );
});

/**
 * 🌳️ Hierarchical tree view component with optional file tree rendering.
 **/
export type TreeComponent = ((props: TreeRootProps) => React.ReactElement) & {
  Files: React.FC<TreeFilesProps>;
  Section: React.FC<TreeFilesProps>;
};

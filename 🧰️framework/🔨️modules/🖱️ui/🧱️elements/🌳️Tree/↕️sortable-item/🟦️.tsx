import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { TreeContext, TreeAlignedRow, treeHeaderRowClassName, treeInspectorInnerRowClassName, treeHeaderMainClassName } from "../📏️row-alignment/🟦️.tsx";
import { useFlow } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { useIdLabel } from "../../🏷️Label/🟦️.tsx";
import { useSortable, SortableContext } from "@dnd-kit/sortable";
import { useUiDriverDragSurface } from "../../🚗️UiDriver/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { dragLabelOf, dragSourceAttributes, dropPlaceAttributes, useDndGhost } from "../../../🔨️modules/🫳️pointer-drag/🟦️.tsx";
import { DragHandle } from "../../🧱️DragHandle/🟦️.tsx";
import { type DragEndEvent, DndContext, closestCenter } from "@dnd-kit/core";
import type { SortableTreeItemProps, SortableTreeItemsProps } from "../🧩️props/🟦️.ts";
import { hasNonEmptyChildren, useTreeOpenState } from "../🌿️state/🟦️.tsx";
import { renderTreeRowIcon, treeFoldChevronIcon, treeItemContentPaddingTopPx, treeItemLabelSlotClassName, treePresentationLabelStyle, treeRowChromeContentFillClasses, treeRowChromeShellClasses, treeRowShellClassName } from "../🎨️chrome/🟦️.tsx";
import { renderTreeHeaderActions } from "../⚡️actions/🟦️.tsx";
import { TreeBranchContent } from "../🌲️branch/🟦️.tsx";
import { sharedBorderlessBoxProps } from "../../🗂️WindowChrome/shared-border.ts";

/**
 * 🌳️ SortableTreeItem holds the data fields for a SortableTreeItem record.
 *
 * 🖱️ `onClick` belongs to the `role="treeitem"` SHELL — the element that carries `id` and
 * `aria-selected` — in every one of the four layouts, group rows included. A group row used to wire it to
 * the `[data-slot="tree-label"]` span alone, so clicking anywhere else on the row (its padding, the gap
 * before the row actions, the stretch a short label leaves empty) activated nothing, while the otherwise
 * identical leaf row selected. Measured live on the puzzle3d serve: the outliner's object rows nest their
 * vortices, which makes every one of them a group row, and a click on the row selected nothing at all. The
 * fold chevron, the row actions and the drag handle each `stopPropagation`, so they keep owning their own
 * clicks; `event.detail > 1` still yields the row to `onDoubleClick`.
 **/
export const SortableTreeItem: React.FC<SortableTreeItemProps> = ({
  id,
  label,
  icon,
  children,
  onClick,
  className = "",
  isSelected = false,
  isHighlighted = false,
  isDragHandle = false,
  defaultOpen = false,
  isLastItem = false,
  actions = [],
  onDoubleClick,
  layoutKind = "default",
}) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier, direction = "down" } = reactHostPort.useContext(TreeContext);
  const { inline } = useFlow();
  const localizedLabel = useIdLabel(id);
  const displayLabel = label ?? localizedLabel;
  const itemKey = id ?? displayLabel ?? id;
  const itemId = `item-${id}-${itemKey}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const hasChildren = hasNonEmptyChildren(children);
  const { attributes, listeners, setNodeRef, isDragging, isSorting, isOver } = useSortable({ id });

  const dropPlace = isSorting && !isDragging ? (isOver ? "active" : "available") : undefined;
  const rowEmphasized = isSelected || isHighlighted || dropPlace !== undefined;
  const dragPaint = { ...dragSourceAttributes(isDragging), ...dropPlaceAttributes(dropPlace) };
  const driverSurfaceDrag = useUiDriverDragSurface();
  const surfaceDragProps = isDragHandle && driverSurfaceDrag ? { ...attributes, ...listeners } : {};
  const itemShellClasses = cn(
    treeRowShellClassName,
    treeRowChromeShellClasses(isSelected, isHighlighted),
    hasChildren ? "cursor-foldable" : "cursor-selectable",
    isDragHandle && driverSurfaceDrag ? "cursor-grab active:cursor-grabbing" : "",
    className,
  );
  const itemContentFillClassName = cn(treeRowChromeContentFillClasses(isSelected, isHighlighted));
  const GroupFoldChevron = treeFoldChevronIcon(direction, inline, open);

  if (hasChildren && displayLabel) {
    if (layoutKind === "property") {
      return (
        <>
          <div
            data-dim
            data-slot="tree-item-row"
            data-hover-scope
            data-tree-row-kind="group"
            data-tree-group
            data-draggable={isDragHandle ? "true" : undefined}
            role="treeitem"
            {...sharedBorderlessBoxProps}
            aria-selected={isSelected}
            data-selected={isSelected ? "true" : undefined}
            data-highlighted={isHighlighted ? "true" : undefined}
            id={id}
            ref={setNodeRef}
            {...dragPaint}
            className={itemShellClasses}
            {...surfaceDragProps}
            onClick={(event) => {
              if (event.detail > 1) return;
              setOpen(!open);
              onClick?.(event);
            }}
            onDoubleClick={(event) => {
              if (!onDoubleClick) return;
              event.preventDefault();
              event.stopPropagation();
              onDoubleClick(event);
            }}
          >
            <TreeAlignedRow
              level={level}
              isLastAtLevel={isLastAtLevel}
              isLastItem={isLastItem}
              showLines={showLines}
              connectCurrentLevel={level > 0}
              extendBranchStem={open && hasChildren}
              slot={
                <button
                  className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    setOpen(!open);
                  }}
                >
                  <GroupFoldChevron className="size-small flex-shrink-0" />
                </button>
              }
              contentClassName="min-w-0"
              contentChromeClassName={itemContentFillClassName}
            >
              <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
                <div className={treeHeaderMainClassName}>
                  {renderTreeRowIcon(icon, "folder", rowEmphasized)}
                  <span data-slot="tree-label" className={cn(treeItemLabelSlotClassName, "cursor-selectable")} style={treePresentationLabelStyle}>
                    {displayLabel as React.ReactNode}
                  </span>
                </div>
                {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
                {isDragHandle && !driverSurfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(e) => e.stopPropagation()} emphasized={rowEmphasized} /> : null}
              </div>
            </TreeAlignedRow>
          </div>
          {open && (
            <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier, direction }}>
              <TreeBranchContent slot="tree-item-content" ownerRowKind="group" ownerExpanded={open && hasChildren} className="min-w-0" topPaddingPx={treeItemContentPaddingTopPx}>
                {children}
              </TreeBranchContent>
            </TreeContext.Provider>
          )}
        </>
      );
    }

    return (
      <>
        <div
          data-dim
          data-slot="tree-item-row"
          data-hover-scope
          data-tree-row-kind="group"
          data-tree-group
          role="treeitem"
          {...sharedBorderlessBoxProps}
          aria-selected={isSelected}
          data-selected={isSelected ? "true" : undefined}
          data-highlighted={isHighlighted ? "true" : undefined}
          id={id}
          ref={setNodeRef}
          {...dragPaint}
          className={itemShellClasses}
          {...surfaceDragProps}
          onClick={(event) => {
            if (event.detail > 1) return;
            setOpen(!open);
            onClick?.(event);
          }}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
        >
          <TreeAlignedRow
            level={level}
            isLastAtLevel={isLastAtLevel}
            isLastItem={isLastItem}
            showLines={showLines}
            connectCurrentLevel={level > 0}
            extendBranchStem={open && hasChildren}
            slot={
              <button
                className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  setOpen(!open);
                }}
              >
                <GroupFoldChevron className="size-small flex-shrink-0" />
              </button>
            }
            contentClassName="min-w-0"
            contentChromeClassName={itemContentFillClassName}
          >
            <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
              <div className={treeHeaderMainClassName}>
                {renderTreeRowIcon(icon, "folder", rowEmphasized)}
                <span data-slot="tree-label" className={cn(treeItemLabelSlotClassName, "cursor-selectable")} style={treePresentationLabelStyle}>
                  {displayLabel as React.ReactNode}
                </span>
              </div>
              {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
              {isDragHandle && !driverSurfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(e) => e.stopPropagation()} emphasized={rowEmphasized} /> : null}
            </div>
          </TreeAlignedRow>
        </div>
        {open && (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier, direction }}>
            <TreeBranchContent slot="tree-item-content" ownerRowKind="group" ownerExpanded={open && hasChildren} topPaddingPx={treeItemContentPaddingTopPx}>
              {children}
            </TreeBranchContent>
          </TreeContext.Provider>
        )}
      </>
    );
  }

  if (!displayLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree, indentMultiplier, direction }}>{children}</TreeContext.Provider>;
  }

  if (layoutKind === "property") {
    return (
      <div
        data-dim
        data-slot="tree-item-row"
        data-hover-scope
        data-tree-row-kind="property"
        role="treeitem"
        {...sharedBorderlessBoxProps}
        aria-selected={isSelected}
        data-selected={isSelected ? "true" : undefined}
        data-highlighted={isHighlighted ? "true" : undefined}
        id={id}
        ref={setNodeRef}
        {...dragPaint}
        className={itemShellClasses}
        {...surfaceDragProps}
        onClick={(event) => {
          if (event.detail > 1) return;
          onClick?.(event);
        }}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
      >
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} isLastItem={isLastItem} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0" contentChromeClassName={itemContentFillClassName}>
          <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
            <div className={treeHeaderMainClassName}>
              {renderTreeRowIcon(icon, "file-text", rowEmphasized)}
              <span data-slot="tree-label" className={treeItemLabelSlotClassName} style={treePresentationLabelStyle}>
                {displayLabel as React.ReactNode}
              </span>
            </div>
            {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
            {isDragHandle && !driverSurfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(e) => e.stopPropagation()} emphasized={rowEmphasized} /> : null}
          </div>
        </TreeAlignedRow>
      </div>
    );
  }

  return (
    <div
      data-dim
      data-slot="tree-item-row"
      data-hover-scope
      data-tree-row-kind="leaf"
      role="treeitem"
      {...sharedBorderlessBoxProps}
      aria-selected={isSelected}
      data-selected={isSelected ? "true" : undefined}
      data-highlighted={isHighlighted ? "true" : undefined}
      id={id}
      ref={setNodeRef}
      {...dragPaint}
      className={itemShellClasses}
      {...surfaceDragProps}
      onClick={(event) => {
        if (event.detail > 1) return;
        onClick?.(event);
      }}
      onDoubleClick={(event) => {
        if (!onDoubleClick) return;
        event.preventDefault();
        event.stopPropagation();
        onDoubleClick(event);
      }}
    >
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} isLastItem={isLastItem} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0" contentChromeClassName={itemContentFillClassName}>
        <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
          <div className={treeHeaderMainClassName}>
            {renderTreeRowIcon(icon, "file-text", rowEmphasized)}
            <span data-slot="tree-label" className={treeItemLabelSlotClassName} style={treePresentationLabelStyle}>
              {displayLabel as React.ReactNode}
            </span>
          </div>
          {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
          {isDragHandle && !driverSurfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(e) => e.stopPropagation()} emphasized={rowEmphasized} /> : null}
        </div>
      </TreeAlignedRow>
    </div>
  );
};

/** 🧲️ No row shifts to make room: the dragged row stays where it is, hidden, and the places show where it can go. */
const keepPlaces = (): null => null;

/**
 * 🌲️ Drag-and-drop sortable container for tree items.
 **/
export const SortableTreeItems: React.FC<SortableTreeItemsProps> = ({ items, onReorder, children }) => {
  const { ghost, start, move, end } = useDndGhost((id) => dragLabelOf(document.getElementById(id)));
  const handleDragEnd = (event: DragEndEvent) => {
    end();
    const { active, over } = event;
    if (over && active.id !== over.id) {
      const oldIndex = items.findIndex((item) => item.id === active.id);
      const newIndex = items.findIndex((item) => item.id === over.id);
      if (oldIndex !== -1 && newIndex !== -1) {
        onReorder(oldIndex, newIndex);
      }
    }
  };

  return (
    <DndContext collisionDetection={closestCenter} onDragStart={start} onDragMove={move} onDragCancel={end} onDragEnd={handleDragEnd}>
      <SortableContext items={items.map((item) => item.id)} strategy={keepPlaces}>
        {items.map((item, index) => (
          <React.Fragment key={item.id}>{children(item, index)}</React.Fragment>
        ))}
      </SortableContext>
      {ghost}
    </DndContext>
  );
};

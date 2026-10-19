import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { TreeContext, TreeAlignedRow, treeHeaderRowClassName, treeInspectorInnerRowClassName, treeHeaderMainClassName } from "../📏️row-alignment/🟦️.tsx";
import { useFlow } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { useIdLabel } from "../../🏷️Label/🟦️.tsx";
import { useSortable, SortableContext, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { useUiDriverDragSurface } from "../../🚗️UiDriver/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { dropZoneReadyTextClass, dropZoneReadyFillClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { DragHandle } from "../../🧱️DragHandle/🟦️.tsx";
import { type DragEndEvent, DndContext, closestCenter } from "@dnd-kit/core";
import type { SortableTreeItemProps, SortableTreeItemsProps } from "../🧩️props/🟦️.ts";
import { hasNonEmptyChildren, useTreeOpenState } from "../🌿️state/🟦️.tsx";
import { renderTreeRowIcon, treeFoldChevronIcon, treeItemContentPaddingTopPx, treeItemLabelSlotClassName, treePresentationLabelStyle, treeRowChromeContentFillClasses, treeRowChromeShellClasses, treeRowShellClassName } from "../🎨️chrome/🟦️.tsx";
import { renderTreeHeaderActions } from "../⚡️actions/🟦️.tsx";
import { TreeBranchContent } from "../🌲️branch/🟦️.tsx";

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
  const { attributes, listeners, setNodeRef, transform, transition, isDragging, isSorting } = useSortable({ id });
  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  const isDropReady = isSorting && !isDragging;
  const rowEmphasized = isSelected || isHighlighted || isDropReady;
  const driverSurfaceDrag = useUiDriverDragSurface();
  const surfaceDragProps = isDragHandle && driverSurfaceDrag ? { ...attributes, ...listeners } : {};
  const itemShellClasses = cn(
    treeRowShellClassName,
    treeRowChromeShellClasses(isSelected, isHighlighted),
    hasChildren ? "cursor-foldable" : "cursor-selectable",
    isDragHandle && driverSurfaceDrag ? "cursor-grab active:cursor-grabbing" : "",
    isDropReady && dropZoneReadyTextClass,
    className,
  );
  const itemContentFillClassName = cn(treeRowChromeContentFillClasses(isSelected, isHighlighted), isDropReady && dropZoneReadyFillClass);
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
            aria-selected={isSelected}
            data-selected={isSelected ? "true" : undefined}
            data-highlighted={isHighlighted ? "true" : undefined}
            id={id}
            ref={setNodeRef}
            style={style}
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
          aria-selected={isSelected}
          data-selected={isSelected ? "true" : undefined}
          data-highlighted={isHighlighted ? "true" : undefined}
          id={id}
          ref={setNodeRef}
          style={style}
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
        aria-selected={isSelected}
        data-selected={isSelected ? "true" : undefined}
        data-highlighted={isHighlighted ? "true" : undefined}
        id={id}
        ref={setNodeRef}
        style={style}
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
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0" contentChromeClassName={itemContentFillClassName}>
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
      aria-selected={isSelected}
      data-selected={isSelected ? "true" : undefined}
      data-highlighted={isHighlighted ? "true" : undefined}
      id={id}
      ref={setNodeRef}
      style={style}
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
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0" contentChromeClassName={itemContentFillClassName}>
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

/**
 * 🌲️ Drag-and-drop sortable container for tree items.
 **/
export const SortableTreeItems: React.FC<SortableTreeItemsProps> = ({ items, onReorder, children }) => {
  const handleDragEnd = (event: DragEndEvent) => {
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
    <DndContext collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <SortableContext items={items.map((item) => item.id)} strategy={verticalListSortingStrategy}>
        {items.map((item, index) => (
          <React.Fragment key={item.id}>{children(item, index)}</React.Fragment>
        ))}
      </SortableContext>
    </DndContext>
  );
};

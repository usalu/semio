import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { isElementId, childElementId, elementIdSegment } from "../../🆔️ElementId/🟦️.tsx";
import { useIdLabel, useControlAccessibleLabel } from "../../🏷️Label/🟦️.tsx";
import { TreeContext, TreeAlignedRow, treeInspectorInnerRowClassName, treeHeaderMainClassName, PropertyValueColumnContext, treeHeaderRowClassName } from "../📏️row-alignment/🟦️.tsx";
import { useFlow } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { useUiDriverDragSurface } from "../../🚗️UiDriver/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { dropZoneReadyTextClass, dropZoneReadyFillClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { ChevronRightIcon, ChevronLeftIcon } from "../../🔣️Icons/🟦️.tsx";
import { type TreeItemProps, getTreeItemStateId } from "../🧩️props/🟦️.ts";
import { type TreeDragRole, deriveTreeDragRoles, mergeTreeRowContextMenu, renderTreeDragHandles, renderTreeHeaderActions, treeRowDragPayloadAttributes, useTreeRowDescribedControls } from "../⚡️actions/🟦️.tsx";
import { assertNoNestedTreeSections, hasNonEmptyChildren, useTreeOpenState } from "../🌿️state/🟦️.tsx";
import { SortableTreeItem } from "../↕️sortable-item/🟦️.tsx";
import { TreeItemRowContextMenu, shouldDispatchTreeRowPointerLeave } from "../🌱️section/🟦️.tsx";
import { renderTreeRowIcon, treeFoldChevronIcon, treeItemContentPaddingTopPx, treeItemControlClassName, treeItemLabelSlotClassName, treeItemSecondaryTextClassName, treePresentationLabelStyle, treePropertyHeaderGridClassName, treePropertyHeaderGridStyle, treeRowChromeContentFillClasses, treeRowChromeShellClasses, treeRowShellClassName } from "../🎨️chrome/🟦️.tsx";
import { treeWindowRowAriaAttributes, treeWindowRowExtentPx } from "../🔭️virtual-window/🟦️.tsx";
import { TreeBranchContent } from "../🌲️branch/🟦️.tsx";

/**
 * Single tree item row with icon, label, and interaction handlers.
 *
 * 🖱️ Every `role="treeitem"` row here activates from the SHELL — the element carrying `id` — not from its
 * label span, and every one of them publishes `aria-selected`. Both used to hold only for the leaf layout:
 * an expandable row wired `onClick` to `[data-slot="tree-label"]` alone, so a click on the row's padding,
 * on the gap before its row actions, or on the stretch a short label leaves empty did nothing at all, and
 * no row of this component announced its selection to assistive tech or to a scripted reader. Measured live
 * on the puzzle3d serve: the outliner's object rows nest their vortices, which makes every one of them an
 * expandable row, and clicking one selected nothing. The fold chevron, the branch-navigation buttons, the
 * row actions and the drag handles each `stopPropagation`, so they keep owning their own clicks, and
 * `event.detail > 1` still yields the row to `onDoubleClick`. A row with children also folds from that
 * same shell click — the chevron is the same fold, not the only one — and a declared activation still
 * fires. The value column stops propagation, so a header slider keeps its own gesture.
 **/
export const TreeItem: React.FC<TreeItemProps> = ({
  focusPath,
  label,
  id,
  icon,
  children,
  onClick,
  className = "",
  isSelected = false,
  isHighlighted = false,
  sortable = false,
  sortableId,
  isDragHandle = false,
  defaultOpen = false,
  isLastItem = false,
  actions = [],
  onDoubleClick,
  open: controlledOpen,
  onOpenChange,
  expandable,
  loading = false,
  waiting = false,
  draggable = false,
  onDragStart,
  onDragEnd,
  onDragOver,
  onDragLeave,
  onDrop,
  branchCount = 0,
  activeBranchIndex = 0,
  onBranchChange,
  onPointerEnter,
  onPointerLeave,
  onPointerDown,
  onPointerMove,
  onPointerUp,
  onPointerCancel,
  layoutKind = "default",
  activatable = false,
  isHidden = false,
  contextMenu,
  headerControl,
  isDragging = false,
  dragInitiation = "handle",
  dragRoles,
  transferPointerDown,
  dragData,
  isDropReady = false,
  windowAttributes,
  windowRowIndex,
  windowRowTotal,
  windowRowExtent,
  description,
}) => {
  const generatedLabelId = reactHostPort.useId();
  const disclosureLabelId = id !== undefined && isElementId(id) ? childElementId(id, "disclosureLabel") : childElementId("ui.tree.disclosureLabel", elementIdSegment(generatedLabelId));
  const descriptionId = description ? (id !== undefined && isElementId(id) ? childElementId(id, "description") : childElementId("ui.tree.description", elementIdSegment(generatedLabelId))) : undefined;
  const valueSlotRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  useTreeRowDescribedControls(valueSlotRef, descriptionId);
  const localizedLabel = useIdLabel(id);
  const resolvedLabel = label !== undefined ? label : localizedLabel;
  const controlHint = useControlAccessibleLabel(id);
  const resolvedContextMenu = mergeTreeRowContextMenu(actions, contextMenu);
  assertNoNestedTreeSections(children, "TreeItem");
  if (sortable && sortableId) {
    return (
      <SortableTreeItem
        id={sortableId}
        label={resolvedLabel}
        icon={icon}
        className={className}
        isSelected={isSelected}
        isHighlighted={isHighlighted}
        isDragHandle={true}
        defaultOpen={defaultOpen}
        isLastItem={isLastItem}
        actions={actions}
        onDoubleClick={onDoubleClick}
        onPointerEnter={onPointerEnter}
        onPointerLeave={onPointerLeave}
      >
        {children}
      </SortableTreeItem>
    );
  }

  const { level, isLastAtLevel, showLines, isTree, indentMultiplier, direction = "down" } = reactHostPort.useContext(TreeContext);
  const { inline } = useFlow();
  const itemKey = id ?? resolvedLabel ?? sortableId ?? "tree-item";
  const itemId = getTreeItemStateId(String(itemKey));
  const treeOpenState = useTreeOpenState(itemId, defaultOpen);
  const open = controlledOpen ?? treeOpenState.open;
  const setOpen = reactHostPort.useCallback(
    (value: boolean) => {
      treeOpenState.setOpen(value);
      onOpenChange?.(value);
    },
    [onOpenChange, treeOpenState],
  );
  const handlePointerEnter = reactHostPort.useCallback(() => {
    onPointerEnter?.();
  }, [onPointerEnter]);
  const handlePointerLeave = reactHostPort.useCallback(
    (event: React.MouseEvent) => {
      if (!shouldDispatchTreeRowPointerLeave(event.relatedTarget)) {
        return;
      }
      onPointerLeave?.();
    },
    [onPointerLeave],
  );
  const hasChildren = hasNonEmptyChildren(children);
  const isExpandable = expandable ?? hasChildren;
  const driverSurfaceDrag = useUiDriverDragSurface();
  const LeafLabel = activatable && !isExpandable ? "button" : "span";
  const leafActionClassName = LeafLabel === "button" ? "p-0 border-0 bg-transparent text-start" : undefined;
  const resolvedDragRoles: readonly TreeDragRole[] =
    dragRoles ?? (driverSurfaceDrag ? [] : draggable ? (dragInitiation === "surface" ? [] : deriveTreeDragRoles({ draggable, isDragHandle }, Boolean(transferPointerDown))) : isDragHandle ? ["sort"] : []);
  const effectiveDragInitiation = driverSurfaceDrag ? "surface" : dragInitiation;
  const [dragArmed, setDragArmed] = reactHostPort.useState(false);
  const armDrag = reactHostPort.useCallback(() => {
    setDragArmed(true);
    window.addEventListener("pointerup", () => setDragArmed(false), { once: true });
  }, []);
  const effectiveDraggable = draggable && (driverSurfaceDrag || (resolvedDragRoles.length === 0 && effectiveDragInitiation === "surface") || ((resolvedDragRoles.length > 0 || effectiveDragInitiation === "handle") && dragArmed));
  const handleDragEnd = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      setDragArmed(false);
      onDragEnd?.(event);
    },
    [onDragEnd],
  );
  const itemShellClasses = cn(
    treeRowShellClassName,
    treeRowChromeShellClasses(isSelected, isHighlighted, isHidden),
    isExpandable ? "cursor-foldable" : "cursor-selectable",
    draggable && (driverSurfaceDrag || (resolvedDragRoles.length === 0 && effectiveDragInitiation === "surface")) ? "cursor-grab active:cursor-grabbing" : "",
    isDragging ? "opacity-40" : "",
    isDropReady && dropZoneReadyTextClass,
    className,
  );
  const itemContentFillClassName = cn(treeRowChromeContentFillClasses(isSelected, isHighlighted, loading, waiting), isDropReady && dropZoneReadyFillClass);
  const windowRowStyle =
    windowRowExtent === undefined
      ? undefined
      : ({
          "--tree-row-height": `${treeWindowRowExtentPx(windowRowExtent)}px`,
          "--tree-row-min-height": `${treeWindowRowExtentPx(windowRowExtent)}px`,
          "--tree-row-max-height": `${treeWindowRowExtentPx(windowRowExtent)}px`,
        } as React.CSSProperties);
  const windowRowAria = treeWindowRowAriaAttributes(windowRowIndex, windowRowTotal);
  const treeLabelSelectClass = draggable && (driverSurfaceDrag || (resolvedDragRoles.length === 0 && effectiveDragInitiation === "surface")) ? "select-none" : "select-text";
  const rowEmphasized = isSelected || isHighlighted || isDropReady;
  const dragHandleProps = {
    roles: resolvedDragRoles,
    driverSurfaceDrag,
    rowEmphasized,
    armDrag,
    transferPointerDown,
  };

  if (layoutKind === "property") {
    const PropertyFoldChevron = treeFoldChevronIcon(direction, inline, open);
    const propertyHeader = (
      <div
        data-dim
        data-slot="tree-property-item"
        data-tree-focus-path={focusPath ? JSON.stringify(focusPath.slice(-64)) : undefined}
        data-hover-scope
        data-tree-window-row={windowRowIndex}
        style={windowRowStyle}
        data-tree-row-kind={isExpandable ? "group" : "property"}
        data-activatable={activatable ? "true" : undefined}
        role="treeitem"
        {...windowRowAria}
        aria-selected={isSelected}
        aria-expanded={isExpandable ? open : undefined}
        data-selected={isSelected ? "true" : undefined}
        data-highlighted={isHighlighted ? "true" : undefined}
        id={id}
        data-state={open ? "open" : "closed"}
        className={cn("min-w-0 w-full", isExpandable ? "cursor-foldable" : "cursor-selectable", treeRowChromeShellClasses(isSelected, isHighlighted, isHidden), isDropReady && dropZoneReadyTextClass, className)}
        draggable={effectiveDraggable}
        onDragStart={onDragStart}
        onDragEnd={handleDragEnd}
        onDragOver={onDragOver}
        onDragLeave={onDragLeave}
        onDrop={onDrop}
        onClick={(event) => {
          if (event.detail > 1) return;
          if (isExpandable) setOpen(!open);
          onClick?.(event);
        }}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
        onMouseEnter={handlePointerEnter}
        onMouseLeave={handlePointerLeave}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerCancel}
      >
        <TreeAlignedRow
          level={level}
          isLastAtLevel={isLastAtLevel}
          showLines={showLines}
          connectCurrentLevel={level > 0}
          extendBranchStem={isExpandable && open && hasChildren}
          slot={
            isExpandable ? (
              <button
                type="button"
                aria-labelledby={disclosureLabelId}
                aria-expanded={open}
                className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                onClick={(event) => {
                  event.preventDefault();
                  event.stopPropagation();
                  setOpen(!open);
                }}
              >
                <PropertyFoldChevron className="size-small flex-shrink-0" />
              </button>
            ) : undefined
          }
          contentClassName="min-w-0"
          contentChromeClassName={itemContentFillClassName}
        >
          <div className={cn(treePropertyHeaderGridClassName, treeInspectorInnerRowClassName)} style={treePropertyHeaderGridStyle}>
            <div className={treeHeaderMainClassName}>
              {renderTreeRowIcon(icon, isExpandable ? "folder" : "file-text", rowEmphasized)}
              <LeafLabel
                id={disclosureLabelId}
                type={LeafLabel === "button" ? "button" : undefined}
                data-slot="tree-label"
                title={controlHint}
                className={cn(treeItemLabelSlotClassName, "truncate font-medium transition-colors", isExpandable ? "cursor-foldable" : "cursor-selectable", "select-text", leafActionClassName)}
                style={treePresentationLabelStyle}
              >
                {resolvedLabel as React.ReactNode}
              </LeafLabel>
              {description ? (
                <span id={descriptionId} data-slot="tree-description" className={cn(treeItemSecondaryTextClassName, "min-w-0 truncate")}>
                  {description}
                </span>
              ) : null}
            </div>
            <div data-slot="tree-item-control" className={cn(treeItemControlClassName, "gap-double")} onClick={(event) => event.stopPropagation()}>
              <div ref={valueSlotRef} data-slot="tree-item-value" className="contents">
                {!isExpandable ? (
                  <PropertyValueColumnContext.Provider value={true}>{children}</PropertyValueColumnContext.Provider>
                ) : headerControl ? (
                  <PropertyValueColumnContext.Provider value={true}>{headerControl}</PropertyValueColumnContext.Provider>
                ) : null}
              </div>
              {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
              {renderTreeDragHandles(dragHandleProps)}
            </div>
          </div>
        </TreeAlignedRow>
      </div>
    );
    const propertyContent = isExpandable ? (
      open ? (
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier, direction }}>
          <TreeBranchContent slot="tree-property-content" ownerRowKind="group" ownerExpanded={open && hasChildren} className="min-w-0" topPaddingPx={treeItemContentPaddingTopPx} windowAttributes={windowAttributes}>
            {children}
          </TreeBranchContent>
        </TreeContext.Provider>
      ) : (
        <div data-slot="tree-property-content" className="w-full min-w-0" />
      )
    ) : null;
    const propertyBlock =
      direction === "up" ? (
        <>
          {propertyContent}
          {propertyHeader}
        </>
      ) : (
        <>
          {propertyHeader}
          {propertyContent}
        </>
      );
    return <TreeItemRowContextMenu items={resolvedContextMenu}>{isExpandable ? <div className="min-w-0 w-full">{propertyBlock}</div> : propertyHeader}</TreeItemRowContextMenu>;
  }

  if (isExpandable && resolvedLabel) {
    return (
      <TreeItemRowContextMenu items={resolvedContextMenu}>
        {(() => {
          const DefaultFoldChevron = treeFoldChevronIcon(direction, inline, open);
          const defaultHeader = (
            <div
              data-dim
              data-slot="tree-item-row"
              data-tree-focus-path={focusPath ? JSON.stringify(focusPath.slice(-64)) : undefined}
              data-hover-scope
              data-tree-window-row={windowRowIndex}
              style={windowRowStyle}
              data-tree-row-kind="group"
              data-tree-group
              data-activatable={activatable ? "true" : undefined}
              data-draggable={draggable ? "true" : undefined}
              {...treeRowDragPayloadAttributes(dragData)}
              role="treeitem"
              {...windowRowAria}
              aria-selected={isSelected}
              aria-expanded={open}
              data-selected={isSelected ? "true" : undefined}
              data-highlighted={isHighlighted ? "true" : undefined}
              id={id}
              className={itemShellClasses}
              draggable={effectiveDraggable}
              onDragStart={onDragStart}
              onDragEnd={handleDragEnd}
              onDragOver={onDragOver}
              onDragLeave={onDragLeave}
              onDrop={onDrop}
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
              onMouseEnter={handlePointerEnter}
              onMouseLeave={handlePointerLeave}
              onPointerDown={onPointerDown}
              onPointerMove={onPointerMove}
              onPointerUp={onPointerUp}
              onPointerCancel={onPointerCancel}
            >
              <TreeAlignedRow
                level={level}
                isLastAtLevel={isLastAtLevel}
                showLines={showLines}
                connectCurrentLevel={level > 0}
                extendBranchStem={open && hasChildren}
                slot={
                  <button
                    type="button"
                    aria-labelledby={disclosureLabelId}
                    aria-expanded={open}
                    className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      setOpen(!open);
                    }}
                  >
                    <DefaultFoldChevron className="size-small flex-shrink-0" />
                  </button>
                }
                contentClassName="min-w-0"
                contentChromeClassName={itemContentFillClassName}
              >
                <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
                  <div className={treeHeaderMainClassName}>
                    {renderTreeRowIcon(icon, "folder", rowEmphasized)}
                    <span id={disclosureLabelId} data-slot="tree-label" className={cn(treeItemLabelSlotClassName, "cursor-selectable")} style={treePresentationLabelStyle}>
                      {resolvedLabel as React.ReactNode}
                    </span>
                  </div>
                  {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
                  {branchCount > 0 && (
                    <div data-slot="tree-branch-nav" className="flex items-center gap-single flex-shrink-0">
                      <button
                        data-slot="tree-branch-prev"
                        className="p-0 border-0 bg-transparent cursor-selectable hover:bg-hover-interactive-fill disabled:opacity-30 disabled:cursor-default"
                        disabled={activeBranchIndex <= 0}
                        onClick={(e) => {
                          e.preventDefault();
                          e.stopPropagation();
                          onBranchChange?.(activeBranchIndex - 1);
                        }}
                      >
                        {inline === "rtl" ? <ChevronRightIcon className="size-tiny text-muted-foreground" /> : <ChevronLeftIcon className="size-tiny text-muted-foreground" />}
                      </button>
                      <span data-slot="tree-branch-indicator" className="text-2xs text-muted-foreground tabular-nums select-none">
                        {activeBranchIndex + 1}/{branchCount}
                      </span>
                      <button
                        data-slot="tree-branch-next"
                        className="p-0 border-0 bg-transparent cursor-selectable hover:bg-hover-interactive-fill disabled:opacity-30 disabled:cursor-default"
                        disabled={activeBranchIndex >= branchCount - 1}
                        onClick={(e) => {
                          e.preventDefault();
                          e.stopPropagation();
                          onBranchChange?.(activeBranchIndex + 1);
                        }}
                      >
                        {inline === "rtl" ? <ChevronLeftIcon className="size-tiny text-muted-foreground" /> : <ChevronRightIcon className="size-tiny text-muted-foreground" />}
                      </button>
                    </div>
                  )}
                  {renderTreeDragHandles(dragHandleProps)}
                </div>
              </TreeAlignedRow>
            </div>
          );
          const defaultContent = open && (
            <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier, direction }}>
              <TreeBranchContent slot="tree-item-content" ownerRowKind="group" ownerExpanded={open && hasChildren} topPaddingPx={treeItemContentPaddingTopPx} windowAttributes={windowAttributes}>
                {children}
              </TreeBranchContent>
            </TreeContext.Provider>
          );
          return direction === "up" ? (
            <>
              {defaultContent}
              {defaultHeader}
            </>
          ) : (
            <>
              {defaultHeader}
              {defaultContent}
            </>
          );
        })()}
      </TreeItemRowContextMenu>
    );
  }

  if (!resolvedLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree, indentMultiplier, direction }}>{children}</TreeContext.Provider>;
  }

  return (
    <TreeItemRowContextMenu items={resolvedContextMenu}>
      <div
        data-dim
        data-slot="tree-item-row"
        data-hover-scope
        data-tree-window-row={windowRowIndex}
        style={windowRowStyle}
        data-tree-row-kind="leaf"
        data-tree-focus-path={focusPath ? JSON.stringify(focusPath.slice(-64)) : undefined}
        data-activatable={activatable ? "true" : undefined}
        data-draggable={draggable ? "true" : undefined}
        {...treeRowDragPayloadAttributes(dragData)}
        role="treeitem"
        {...windowRowAria}
        aria-selected={isSelected}
        data-selected={isSelected ? "true" : undefined}
        data-highlighted={isHighlighted ? "true" : undefined}
        id={id}
        className={itemShellClasses}
        draggable={effectiveDraggable}
        onDragStart={onDragStart}
        onDragEnd={handleDragEnd}
        onDragOver={onDragOver}
        onDragLeave={onDragLeave}
        onDrop={onDrop}
        onClick={onClick}
        onMouseEnter={handlePointerEnter}
        onMouseLeave={handlePointerLeave}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerCancel}
      >
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0" contentChromeClassName={itemContentFillClassName}>
          <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
            <div className={treeHeaderMainClassName}>
              {renderTreeRowIcon(icon, "file-text", rowEmphasized)}
              <LeafLabel type={LeafLabel === "button" ? "button" : undefined} data-slot="tree-label" className={cn(treeItemLabelSlotClassName, draggable && effectiveDragInitiation === "surface" ? "cursor-grab" : "cursor-selectable", treeLabelSelectClass, leafActionClassName)} style={treePresentationLabelStyle}>
                {resolvedLabel as React.ReactNode}
              </LeafLabel>
            </div>
            {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
            {branchCount > 0 && (
              <div data-slot="tree-branch-nav" className="flex items-center gap-single flex-shrink-0">
                <button
                  data-slot="tree-branch-prev"
                  className="p-0 border-0 bg-transparent cursor-selectable disabled:opacity-30 disabled:cursor-default"
                  disabled={activeBranchIndex <= 0}
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    onBranchChange?.(activeBranchIndex - 1);
                  }}
                >
                  {inline === "rtl" ? <ChevronRightIcon className="size-tiny text-muted-foreground" /> : <ChevronLeftIcon className="size-tiny text-muted-foreground" />}
                </button>
                <span data-slot="tree-branch-indicator" className="text-2xs text-muted-foreground tabular-nums select-none">
                  {activeBranchIndex + 1}/{branchCount}
                </span>
                <button
                  data-slot="tree-branch-next"
                  className="p-0 border-0 bg-transparent cursor-selectable disabled:opacity-30 disabled:cursor-default"
                  disabled={activeBranchIndex >= branchCount - 1}
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    onBranchChange?.(activeBranchIndex + 1);
                  }}
                >
                  {inline === "rtl" ? <ChevronLeftIcon className="size-tiny text-muted-foreground" /> : <ChevronRightIcon className="size-tiny text-muted-foreground" />}
                </button>
              </div>
            )}
            {renderTreeDragHandles(dragHandleProps)}
          </div>
        </TreeAlignedRow>
      </div>
    </TreeItemRowContextMenu>
  );
};

/**
 * Iterator rendering a list of tree item children.
 **/
export const TreeItems: React.FC<{ children: React.ReactNode[]; renderItem: (child: React.ReactNode, index: number, isLast: boolean) => React.ReactNode }> = ({ children, renderItem }) => {
  return <>{children.map((child, index) => renderItem(child, index, index === children.length - 1))}</>;
};

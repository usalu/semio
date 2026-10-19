import * as React from "react";
import { type ContextMenuItem } from "../../🖱️ContextMenu/🎨️item-render/🟦️.tsx";
import { ContextMenu } from "../../🖱️ContextMenu/🟦️.tsx";
import { useLabel, useIdLabel, useControlAccessibleLabel } from "../../🏷️Label/🟦️.tsx";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { TreeContext, TreeAlignedRow, treeHeaderRowClassName, treeInspectorInnerRowClassName, treeHeaderMainClassName } from "../📏️row-alignment/🟦️.tsx";
import { useFlow } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { useUiDriverDragSurface } from "../../🚗️UiDriver/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { dropZoneReadyTextClass, dropZoneReadyFillClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { DragHandle } from "../../🧱️DragHandle/🟦️.tsx";
import { CollapsibleTrigger, CollapsibleContent, Collapsible } from "../../↕️Collapsible/🟦️.tsx";
import { renderTreeRowIcon, treeFoldChevronIcon, treePresentationLabelStyle, treeRowChromeContentFillClasses, treeRowChromeShellClasses, treeRowShellClassName, treeSectionChevronClassName, treeSectionContentPaddingTopPx, treeSectionLabelSlotClassName, treeSemanticHoverStaySelector } from "../🎨️chrome/🟦️.tsx";
import { type TreeSectionProps, getTreeSectionStateId } from "../🧩️props/🟦️.ts";
import { type TreeComponentMarker, assertNoNestedTreeSections, hasNonEmptyChildren, treeSectionDoubleClickDelayMs, treeSectionElementMarker, useTreeOpenState } from "../🌿️state/🟦️.tsx";
import { renderTreeHeaderActions } from "../⚡️actions/🟦️.tsx";
import { TreeBranchContent } from "../🌲️branch/🟦️.tsx";

export const TreeItemRowContextMenu: React.FC<{ readonly items?: readonly ContextMenuItem[]; readonly children: React.ReactNode }> = ({ items, children }) => {
  const contextMenuTitle = useLabel("ui.common.actions");
  if (!items?.length) {
    return <>{children}</>;
  }
  return (
    <ContextMenu items={items} title={contextMenuTitle}>
      {children}
    </ContextMenu>
  );
};

/** 🖱️ Skip row leave when pointer moves to another tree row or nested branch (avoids stale leave clearing fast-hover highlight). */
export function shouldDispatchTreeRowPointerLeave(relatedTarget: EventTarget | null): boolean {
  if (!(relatedTarget instanceof Element)) {
    return true;
  }
  return relatedTarget.closest(treeSemanticHoverStaySelector) === null;
}

/**
 * 🌳️ Collapsible tree section header with optional action buttons.
 **/
export const TreeSection: React.FC<TreeSectionProps> = ({
  label,
  id,
  icon,
  children,
  defaultOpen = false,
  open: controlledOpen,
  onOpenChange,
  expandable,
  loading = false,
  waiting = false,
  className = "",
  actions = [],
  onPointerEnter: onSectionPointerEnter,
  onPointerLeave: onSectionPointerLeave,
  onDoubleClick,
  draggable = false,
  onDragStart,
  onDragEnd,
  onDragOver,
  onDragLeave,
  onDrop,
  isLastSection = false,
  isDropReady = false,
  isDragHandle = false,
  dragInitiation,
  windowAttributes,
}) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier, direction = "down" } = reactHostPort.useContext(TreeContext);
  const { inline } = useFlow();
  const suppressLocalizedLabel = label == null || label === "";
  const resolvedLabel = suppressLocalizedLabel ? undefined : label;
  const idLabel = useIdLabel(id);
  const localizedLabel = !suppressLocalizedLabel && resolvedLabel === undefined && id ? idLabel : undefined;
  const displayLabel = resolvedLabel ?? localizedLabel;
  const controlHint = useControlAccessibleLabel(id);
  assertNoNestedTreeSections(children, "TreeSection");
  const sectionStateId = getTreeSectionStateId(id ?? String(displayLabel ?? "section"));
  const treeOpenState = useTreeOpenState(sectionStateId, defaultOpen);
  const open = controlledOpen ?? treeOpenState.open;
  const setOpen = reactHostPort.useCallback(
    (value: boolean) => {
      treeOpenState.setOpen(value);
      onOpenChange?.(value);
    },
    [onOpenChange, treeOpenState],
  );
  const pendingPointerActivation = reactHostPort.useRef<number | null>(null);
  const clearPendingPointerActivation = reactHostPort.useCallback(() => {
    if (pendingPointerActivation.current === null) return;
    window.clearTimeout(pendingPointerActivation.current);
    pendingPointerActivation.current = null;
  }, []);
  reactHostPort.useEffect(() => clearPendingPointerActivation, [clearPendingPointerActivation]);
  const handleExpandableSectionClick = reactHostPort.useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!onDoubleClick || event.defaultPrevented || event.detail === 0) return;
      event.preventDefault();
      clearPendingPointerActivation();
      if (event.detail !== 1) return;
      pendingPointerActivation.current = window.setTimeout(() => {
        pendingPointerActivation.current = null;
        setOpen(!open);
      }, treeSectionDoubleClickDelayMs);
    },
    [clearPendingPointerActivation, onDoubleClick, open, setOpen],
  );
  const handleExpandableSectionDoubleClick = reactHostPort.useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!onDoubleClick) return;
      clearPendingPointerActivation();
      event.preventDefault();
      event.stopPropagation();
      onDoubleClick(event);
    },
    [clearPendingPointerActivation, onDoubleClick],
  );
  const hasChildren = hasNonEmptyChildren(children);
  const isExpandable = expandable ?? hasChildren;
  const showDragHandle = Boolean(draggable || isDragHandle);
  const driverSurfaceDrag = useUiDriverDragSurface();
  const resolvedDragInitiation = driverSurfaceDrag ? "surface" : (dragInitiation ?? (showDragHandle ? "handle" : "surface"));
  const [dragArmed, setDragArmed] = reactHostPort.useState(false);
  const armDrag = reactHostPort.useCallback(() => {
    setDragArmed(true);
    window.addEventListener("pointerup", () => setDragArmed(false), { once: true });
  }, []);
  const effectiveDraggable = showDragHandle && (resolvedDragInitiation === "surface" || dragArmed);
  const handleDragEnd = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      setDragArmed(false);
      onDragEnd?.(event);
    },
    [onDragEnd],
  );
  const isHeaderlessSection = suppressLocalizedLabel && localizedLabel === undefined && !icon && actions.length === 0 && !loading && !waiting && !showDragHandle;
  const rowClassName = cn(
    treeRowShellClassName,
    treeRowChromeShellClasses(false, false),
    isExpandable ? "cursor-foldable" : "cursor-selectable",
    showDragHandle && resolvedDragInitiation === "surface" ? "cursor-grab active:cursor-grabbing" : "",
    isDropReady && dropZoneReadyTextClass,
    className,
  );
  const rowContentFillClassName = cn(treeRowChromeContentFillClasses(false, false, loading, waiting), isDropReady && dropZoneReadyFillClass);
  const sectionDragHandle = showDragHandle && !driverSurfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" onPointerDown={resolvedDragInitiation === "handle" ? armDrag : undefined} emphasized={isDropReady} /> : null;

  if (isHeaderlessSection) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree, indentMultiplier, direction }}>{children}</TreeContext.Provider>;
  }

  if (!isExpandable) {
    return (
      <div
        data-dim
        data-slot="tree-section-row"
        data-hover-scope
        data-tree-row-kind="section"
        id={id}
        className={rowClassName}
        draggable={effectiveDraggable}
        onPointerEnter={onSectionPointerEnter}
        onPointerLeave={(event) => {
          if (!shouldDispatchTreeRowPointerLeave(event.relatedTarget)) {
            return;
          }
          onSectionPointerLeave?.();
        }}
        onDragStart={onDragStart}
        onDragEnd={handleDragEnd}
        onDragOver={onDragOver}
        onDragLeave={onDragLeave}
        onDrop={onDrop}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
      >
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} slot={null} contentClassName="min-w-0" contentChromeClassName={rowContentFillClassName}>
          <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
            <div className={treeHeaderMainClassName}>
              {renderTreeRowIcon(icon, "folder", isDropReady)}
              <span data-slot="tree-label" title={controlHint} className={cn(treeSectionLabelSlotClassName, isDropReady && "text-emphasized")} style={treePresentationLabelStyle}>
                {displayLabel}
              </span>
            </div>
            {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
            {sectionDragHandle}
          </div>
        </TreeAlignedRow>
      </div>
    );
  }

  const SectionFoldChevron = treeFoldChevronIcon(direction, inline, open);
  const sectionTrigger = (
    <CollapsibleTrigger asChild>
      <div
        data-dim
        data-slot="tree-section-row"
        data-hover-scope
        data-tree-row-kind="section"
        id={id}
        className={rowClassName}
        role="button"
        draggable={effectiveDraggable}
        onClick={handleExpandableSectionClick}
        onPointerEnter={onSectionPointerEnter}
        onPointerLeave={(event) => {
          if (!shouldDispatchTreeRowPointerLeave(event.relatedTarget)) {
            return;
          }
          onSectionPointerLeave?.();
        }}
        onDragStart={onDragStart}
        onDragEnd={handleDragEnd}
        onDragOver={onDragOver}
        onDragLeave={onDragLeave}
        onDrop={onDrop}
        onDoubleClick={handleExpandableSectionDoubleClick}
      >
        <TreeAlignedRow
          level={level}
          isLastAtLevel={isLastAtLevel}
          showLines={showLines}
          connectCurrentLevel={level > 0}
          extendBranchStem={open && hasChildren}
          slot={<SectionFoldChevron className={treeSectionChevronClassName} />}
          contentClassName="min-w-0"
          contentChromeClassName={rowContentFillClassName}
        >
          <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
            <div className={treeHeaderMainClassName}>
              {renderTreeRowIcon(icon, "folder", isDropReady)}
              <span data-slot="tree-label" title={controlHint} className={cn(treeSectionLabelSlotClassName, isDropReady && "text-emphasized")} style={treePresentationLabelStyle}>
                {displayLabel}
              </span>
            </div>
            {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
            {sectionDragHandle}
          </div>
        </TreeAlignedRow>
      </div>
    </CollapsibleTrigger>
  );
  const sectionContent = (
    <CollapsibleContent className="w-full min-w-0">
      <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastSection], showLines, isTree, indentMultiplier, direction }}>
        <TreeBranchContent slot="tree-section-content" ownerRowKind="section" ownerExpanded={open && hasChildren} topPaddingPx={treeSectionContentPaddingTopPx} windowAttributes={windowAttributes}>
          {children}
        </TreeBranchContent>
      </TreeContext.Provider>
    </CollapsibleContent>
  );

  return (
    <Collapsible open={open} onOpenChange={setOpen}>
      {direction === "up" ? (
        <>
          {sectionContent}
          {sectionTrigger}
        </>
      ) : (
        <>
          {sectionTrigger}
          {sectionContent}
        </>
      )}
    </Collapsible>
  );
};

(TreeSection as TreeComponentMarker)[treeSectionElementMarker] = true;
TreeSection.displayName = "TreeSection";

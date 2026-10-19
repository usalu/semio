import * as React from "react";
import { Label, useIdLabel } from "../../🏷️Label/🟦️.tsx";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { TreeContext, TreeRowAlignmentContext, TreeAlignedRow, detailPanelHeaderLineCenterPx, detailPanelIndentPx, detailPanelIndentLen, detailPanelPropertyRowClassName, detailPanelPropertyControlClassName } from "../📏️row-alignment/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import type { TreeHeaderAction } from "../⚡️actions/🟦️.tsx";
import { TreeItem } from "../🍃️item/🟦️.tsx";
import { treeItemSecondaryTextClassName, treeRowShellClassName } from "../🎨️chrome/🟦️.tsx";
import { TreeContent } from "../🌲️branch/🟦️.tsx";
import type { TreeDataItem } from "../📋️data/🟦️.ts";

/**
 * 🌳️ Leaf form row combining TreeItem and TreeContent into [Indent][Label][Control].
 * When a label resolves (via id or explicit label prop), delegates to TreeItem for the standard header row.
 * When no label resolves, wraps children in TreeAlignedRow so controls always get proper gutter alignment
 * and tree guide lines regardless of whether the child control uses showLabel.
 **/
const treeRowUsesPropertyHeaderAnchor = (children: React.ReactNode): boolean => {
  const childArray = React.Children.toArray(children);
  return childArray.some((child) => {
    if (!React.isValidElement(child)) {
      return false;
    }
    if (child.type === React.Fragment) {
      return treeRowUsesPropertyHeaderAnchor((child.props as { children?: React.ReactNode }).children);
    }
    const childProps = child.props as { children?: React.ReactNode; showLabel?: boolean };
    return child.type === Label || childProps.showLabel === true;
  });
};

export const TreeRow: React.FC<{
  children: React.ReactNode;
  className?: string;
  id?: string;
  /** 🏷️ When set (including explicit `null`), overrides useLabel(id) for the row title. Use `null` for content-only rows. */
  label?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
  actions?: TreeHeaderAction[];
}> = ({ children, className, id, label, onClick, onDoubleClick, actions }) => {
  const localizedLabel = useIdLabel(id);
  const resolvedLabel = label !== undefined ? label : localizedLabel;
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const rowKind = treeRowUsesPropertyHeaderAnchor(children) ? "property" : "content";

  if (resolvedLabel) {
    return (
      <TreeItem className={className} id={id} label={label} onClick={onClick} onDoubleClick={onDoubleClick} actions={actions}>
        {children}
      </TreeItem>
    );
  }

  if (!isTree) {
    return (
      <TreeRowAlignmentContext.Provider value={true}>
        <div data-dim data-slot="tree-row" data-tree-row-kind={rowKind} className={cn(treeRowShellClassName, className)}>
          {children}
        </div>
      </TreeRowAlignmentContext.Provider>
    );
  }

  return (
    <TreeRowAlignmentContext.Provider value={true}>
      <div data-dim data-slot="tree-row" data-tree-row-kind={rowKind} className={cn(treeRowShellClassName, className)}>
        <TreeAlignedRow
          level={level}
          isLastAtLevel={isLastAtLevel}
          showLines={showLines}
          connectCurrentLevel={level > 0}
          className="h-full"
          contentClassName="min-w-0"
          anchorOffsetPx={rowKind === "property" ? detailPanelHeaderLineCenterPx : undefined}
        >
          {children}
        </TreeAlignedRow>
      </div>
    </TreeRowAlignmentContext.Provider>
  );
};

/**
 * 🛠️ Informational text row spanning the full control column width.
 * When `propertyAligned` is true and inside a tree, renders content in the
 * value-column of the shared property-row grid (same layout as Label).
 **/
export const HelperRow: React.FC<{ children: React.ReactNode; className?: string; propertyAligned?: boolean }> = ({ children, className, propertyAligned = false }) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const helperContent = (
    <div data-slot="helper-row" data-detail-panel-control="fill" className={cn("text-xs text-muted-foreground leading-tight py-single", className)}>
      {children}
    </div>
  );
  if (propertyAligned && isTree) {
    const treePropertyRowOffsetPx = detailPanelIndentPx(level, indentMultiplier);
    return (
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} align="start" connectCurrentLevel={level > 0} anchorOffsetPx={detailPanelHeaderLineCenterPx}>
        <div
          data-dim
          data-slot="property-row"
          style={{ marginInlineStart: `calc(-1 * ${detailPanelIndentLen(level, indentMultiplier)})`, width: level > 0 ? `calc(100% + ${detailPanelIndentLen(level, indentMultiplier)})` : "100%" }}
          className={cn(detailPanelPropertyRowClassName, "grid-cols-[var(--layout-label)_minmax(0,1fr)]")}
        >
          <div />
          <div data-slot="property-control" className={detailPanelPropertyControlClassName}>
            {helperContent}
          </div>
        </div>
      </TreeAlignedRow>
    );
  }
  return (
    <TreeItem className={className}>
      <TreeContent>{helperContent}</TreeContent>
    </TreeItem>
  );
};

export const getTreeItemLabel = (item: TreeDataItem): React.ReactNode => {
  if (!item.description) {
    return <span className="min-w-0 truncate">{item.label}</span>;
  }

  return (
    <span className="min-w-0 truncate leading-none">
      <span className="min-w-0 truncate">{item.label}</span> <span className={treeItemSecondaryTextClassName}>{item.description}</span>
    </span>
  );
};

export const getTreeDropData = (event: React.DragEvent<HTMLDivElement>): Record<string, string> => {
  return Array.from(event.dataTransfer.types).reduce<Record<string, string>>((result, kind) => {
    try {
      result[kind] = event.dataTransfer.getData(kind);
    } catch {
      result[kind] = "";
    }
    return result;
  }, {});
};

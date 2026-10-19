import { domSizePx, uiSpacingLen, STYLING_DOM } from "@semio-tech/ui-styling";
import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { loadingBorderStateClass, waitingBorderStateClass } from "../../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { type FlowBlock, FlowProvider, useFlow } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { TreeContext, TreeAlignedRow, treeItemLabelStyle } from "../📏️row-alignment/🟦️.tsx";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { windowMeasureTreeGroupLabelClass, windowMeasureTreeLeafLabelClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import type { UiLabel } from "../../🎗️UiLabel/🟦️.tsx";
import { useTreeHoverPathRootHandlers } from "../🧷️path-marking/🟦️.ts";
import { TreeBranchContent, TreeHoverPathRefreshContext } from "../🌲️branch/🟦️.tsx";
import { hasNonEmptyChildren, useTreeOpenState } from "../🌿️state/🟦️.tsx";
import { renderTreeRowIcon, treeFoldChevronIcon } from "../🎨️chrome/🟦️.tsx";

const windowMeasureTreeValueColumnWidthPx = domSizePx("windowMeasureValueColumnUiSpacing");
const windowMeasureTreeChromeClass = "text-tiny [&_[data-slot=select-trigger]]:h-small";
const windowMeasureTreeRowClassName = "group hover:bg-hover-interactive-fill select-none min-h-tiny w-full min-w-0";

interface WindowMeasureTreeRowProps {
  left: React.ReactNode;
  right?: React.ReactNode;
  loading?: boolean;
  waiting?: boolean;
}

const WindowMeasureTreeRow: React.FC<WindowMeasureTreeRowProps> = ({ left, right, loading = false, waiting = false }) => (
  <div
    data-dim
    data-slot="window-measure-tree-row"
    data-loading={loading ? "true" : undefined}
    data-waiting={waiting ? "true" : undefined}
    className={cn("grid min-w-0 w-full items-center gap-double", windowMeasureTreeRowClassName, loadingBorderStateClass(loading) || waitingBorderStateClass(waiting))}
    style={{ gridTemplateColumns: right === undefined ? "minmax(0, 1fr)" : `minmax(0, 1fr) ${uiSpacingLen(STYLING_DOM.windowMeasureValueColumnUiSpacing)}` }}
  >
    <div data-slot="window-measure-tree-row-left" className="relative min-w-0">
      {left}
    </div>
    {right !== undefined ? (
      <div data-slot="window-measure-tree-row-right" className="min-w-0">
        {right}
      </div>
    ) : null}
  </div>
);

/** 🌳️ Root tree shell for the window measures rail (guide lines + indentation). */
export const WindowMeasuresTree: React.FC<{ children: React.ReactNode; className?: string; direction?: FlowBlock }> = ({ children, className, direction = "down" }) => {
  const { treeRootRef, handleTreePointerOver, handleTreePointerLeave, refreshTreeHoverPath } = useTreeHoverPathRootHandlers();
  const orderedChildren = direction === "up" ? React.Children.toArray(children).reverse() : children;

  return (
    <FlowProvider inline="ltr" block={direction}>
      <div
        ref={treeRootRef}
        data-slot="window-measures-tree"
        dir="auto"
        data-direction={direction}
        className={cn("pointer-events-auto w-full min-w-0", windowMeasureTreeChromeClass, className)}
        onPointerOver={handleTreePointerOver}
        onPointerLeave={handleTreePointerLeave}
      >
        <TreeHoverPathRefreshContext.Provider value={refreshTreeHoverPath}>
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1, direction }}>{orderedChildren}</TreeContext.Provider>
        </TreeHoverPathRefreshContext.Provider>
      </div>
    </FlowProvider>
  );
};

export interface WindowMeasureTreeGroupProps {
  id: string;
  label: string;
  defaultOpen?: boolean;
  headerControl?: React.ReactNode;
  children?: React.ReactNode;
}

/** 🌳️ Collapsible measure group row (same geometry as {@link ControlTree} folders). */
export const WindowMeasureTreeGroup: React.FC<WindowMeasureTreeGroupProps> = ({ id, label, defaultOpen = false, headerControl, children }) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier, direction = "down" } = reactHostPort.useContext(TreeContext);
  const { block, inline } = useFlow();
  const itemId = `window-measure-group-${id}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const hasChildren = hasNonEmptyChildren(children);
  const ToggleChevron = treeFoldChevronIcon(block, inline, open);
  const toggleIcon = <ToggleChevron className="size-tiny flex-shrink-0 text-muted-foreground" />;
  const row = (
    <WindowMeasureTreeRow
      left={
        <TreeAlignedRow
          level={level}
          isLastAtLevel={isLastAtLevel}
          showLines={showLines}
          connectCurrentLevel={level > 0}
          extendBranchStem={open && hasChildren}
          slotOffsetPx={2}
          slot={
            hasChildren ? (
              <button
                type="button"
                className="flex-shrink-0 cursor-pointer border-0 bg-transparent p-0"
                onClick={(event) => {
                  event.preventDefault();
                  event.stopPropagation();
                  setOpen(!open);
                }}
              >
                {toggleIcon}
              </button>
            ) : undefined
          }
          contentClassName="flex min-w-0 items-center gap-double"
        >
          <span data-slot="tree-label" className={cn("flex-1 truncate select-none", windowMeasureTreeGroupLabelClass)} style={treeItemLabelStyle}>
            {label}
          </span>
        </TreeAlignedRow>
      }
      right={headerControl}
    />
  );
  const branch =
    open && hasChildren ? (
      <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines, isTree, indentMultiplier, direction }}>
        <TreeBranchContent slot="window-measure-tree-content">{block === "up" ? React.Children.toArray(children).reverse() : children}</TreeBranchContent>
      </TreeContext.Provider>
    ) : null;
  return block === "up" ? (
    <>
      {branch}
      {row}
    </>
  ) : (
    <>
      {row}
      {branch}
    </>
  );
};

export interface WindowMeasureTreeLeafProps {
  label?: UiLabel;
  icon?: React.ReactNode;
  children: React.ReactNode;
  fullWidth?: boolean;
  loading?: boolean;
  waiting?: boolean;
}

/** 🌳️ Measure control leaf aligned like a tree row (icon + label + value or full-width control). */
export const WindowMeasureTreeLeaf: React.FC<WindowMeasureTreeLeafProps> = ({ label, icon, children, fullWidth = false, loading = false, waiting = false }) => {
  const { level, isLastAtLevel, showLines } = reactHostPort.useContext(TreeContext);
  const labelNode = label ? (
    <span data-slot="tree-label" className={cn("truncate select-none", windowMeasureTreeLeafLabelClass)} style={treeItemLabelStyle}>
      {label}
    </span>
  ) : null;
  const leading = (
    <>
      {icon != null ? renderTreeRowIcon(icon, "file-text") : null}
      {labelNode}
    </>
  );
  if (fullWidth) {
    return (
      <WindowMeasureTreeRow
        loading={loading}
        waiting={waiting}
        left={
          <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} slotOffsetPx={2} contentClassName="min-w-0 w-full">
            <div data-slot="window-measure-tree-leaf-body" className="min-w-0 w-full">
              {children}
            </div>
          </TreeAlignedRow>
        }
      />
    );
  }
  return (
    <WindowMeasureTreeRow
      loading={loading}
      waiting={waiting}
      left={
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} slotOffsetPx={2} contentClassName="flex min-w-0 items-center gap-double">
          {leading}
        </TreeAlignedRow>
      }
      right={
        <div data-slot="window-measure-tree-leaf-body" className="min-w-0">
          {children}
        </div>
      }
    />
  );
};

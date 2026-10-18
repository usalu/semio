import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/🟦️.tsx";
import { TreeContext, TreeAlignedRow } from "../📏️row-alignment/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { IndentationLines, getTreeSiblingGapPx, treeBranchContentStyle, treeRowVerticalPaddingPx, treeSubtreeGapPx } from "../🎨️chrome/🟦️.tsx";
import type { TreeWindowDomAttributes } from "../🔭️virtual-window/🟦️.tsx";

/**
 * Wrapper rendering tree children with connecting lines.
 **/
export const TreeContent: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { level, isLastAtLevel, showLines } = reactHostPort.useContext(TreeContext);
  return (
    <div data-slot="tree-content" data-tree-row-kind="content" className="relative w-full min-w-0" style={{ paddingTop: `${treeRowVerticalPaddingPx}px`, paddingBottom: `${treeRowVerticalPaddingPx}px` }}>
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} align="start" connectCurrentLevel={level > 0}>
        {children}
      </TreeAlignedRow>
    </div>
  );
};

interface TreeBranchContentProps {
  slot: string;
  children: React.ReactNode;
  className?: string;
  topPaddingPx?: number;
  ownerRowKind?: string;
  ownerExpanded?: boolean;
  /** 🪟️ Window mirror for a virtualised container — this element's top edge is row 0 of the child list. */
  windowAttributes?: TreeWindowDomAttributes;
}

export const TreeHoverPathRefreshContext = reactHostPort.createContext<(() => void) | null>(null);

export const TreeBranchContent: React.FC<TreeBranchContentProps> = ({ slot, children, className, topPaddingPx = 0, ownerRowKind, ownerExpanded = false, windowAttributes }) => {
  const { level, showLines, isTree } = reactHostPort.useContext(TreeContext);
  const refreshTreeHoverPath = reactHostPort.useContext(TreeHoverPathRefreshContext);
  const branchRef = reactHostPort.useRef<HTMLDivElement>(null);
  reactHostPort.useLayoutEffect(() => {
    if (!isTree) {
      return;
    }
    refreshTreeHoverPath?.();
  }, [children, isTree, ownerExpanded, refreshTreeHoverPath]);
  reactHostPort.useLayoutEffect(() => {
    const branchElement = branchRef.current;
    if (!branchElement || !isTree) {
      return;
    }

    const branchSlots = new Set(["tree-section-content", "tree-item-content", "tree-property-content", "control-tree-folder-content", "window-measure-tree-content"]);
    const rowSlots = new Set(["tree-item-row", "tree-section-row", "tree-property-item", "tree-row", "tree-content", "control-tree-row", "window-measure-tree-row"]);
    const directChildren = Array.from(branchElement.children) as HTMLElement[];
    const isRowElement = (el: HTMLElement): boolean => rowSlots.has(el.dataset.slot ?? "");
    const isBranchElement = (el: HTMLElement): boolean => branchSlots.has(el.dataset.slot ?? "");
    const getRowKind = (el: HTMLElement): string => el.dataset.treeRowKind ?? "leaf";
    const setMarginTop = (el: HTMLElement, marginTopPx: number) => {
      el.style.marginTop = marginTopPx > 0 ? `${marginTopPx}px` : "0px";
    };

    for (const child of directChildren) {
      setMarginTop(child, 0);
    }

    let previousDirect: HTMLElement | null = null;
    for (const child of directChildren) {
      if (!previousDirect) {
        previousDirect = child;
        continue;
      }

      if (isBranchElement(child)) {
        setMarginTop(child, treeSubtreeGapPx);
        previousDirect = child;
        continue;
      }

      if (!isRowElement(child)) {
        previousDirect = child;
        continue;
      }

      if (isBranchElement(previousDirect)) {
        setMarginTop(child, treeSubtreeGapPx);
        previousDirect = child;
        continue;
      }

      if (isRowElement(previousDirect)) {
        const currentKind = getRowKind(child);
        const previousKind = getRowKind(previousDirect);
        setMarginTop(child, getTreeSiblingGapPx(previousKind, currentKind));
      }

      previousDirect = child;
    }
  }, [children, isTree]);

  return (
    <div
      ref={branchRef}
      data-slot={slot}
      data-tree-owner-kind={ownerRowKind}
      data-tree-owner-expanded={ownerExpanded ? "true" : "false"}
      {...windowAttributes}
      className={cn("relative flex w-full min-w-0 flex-col", className)}
      style={treeBranchContentStyle(topPaddingPx)}
    >
      {isTree ? <IndentationLines level={level} showLines={showLines} /> : null}
      {children}
    </div>
  );
};

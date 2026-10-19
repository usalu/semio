import { type TreeDirection, detailPanelIndentLen, detailPanelIndentPx, TreeContext, indentationLineLen, treeGuideLineStrokeClassName } from "../📏️row-alignment/🟦️.tsx";
import type { FlowInline } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import * as React from "react";
import { ChevronUpIcon, ChevronDownIcon, ChevronRightIcon, ChevronLeftIcon, Icon } from "../../🔣️Icons/🟦️.tsx";
import { domSizePx, uiSpacingLen, STYLING_DOM } from "@semio-tech/ui-styling";
import type { IconName } from "@semio-tech/assets";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { interactiveControlTransitionClass, hoverExcludingHandleTextEmphasizedClass, interactiveActiveFillClass, groupHoverExcludingHandleBgFillClass } from "../../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { loadingBorderStateClass, waitingBorderStateClass } from "../../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import type { TreeDataItem, TreeDataSection } from "../📋️data/🟦️.ts";
import { getTreeItemItems, getTreeSectionItems } from "../🧩️props/🟦️.ts";

/** 🧭️ Fold-affordance chevron for a tree group row — points toward where the content actually is: along the block axis when open (down, or up when {@link direction} is `"up"`), along the {@link FlowInline}-mirrored inline axis when closed (right in ltr/down or rtl/up, left in ltr/up or rtl/down). */
export function treeFoldChevronIcon(direction: TreeDirection, inline: FlowInline, open: boolean): React.ComponentType<{ className?: string }> {
  if (open) return direction === "up" ? ChevronUpIcon : ChevronDownIcon;
  const towardStart = direction === "up";
  return towardStart === (inline === "rtl") ? ChevronRightIcon : ChevronLeftIcon;
}
export const treeRowShellClassName = "relative h-[var(--tree-row-height,var(--size-workbench))] min-h-[var(--tree-row-min-height,var(--tree-row-height))] max-h-[var(--tree-row-max-height,var(--tree-row-height))] w-full min-w-0 select-none overflow-hidden";
const detailPanelPropertyLabelColumnWidthPx = domSizePx("propertyLabelColumnUiSpacing");
const detailPanelPropertyStackedRowGapPx = domSizePx("propertyStackedGapUiSpacing");
export const treePropertyHeaderGridClassName = "grid min-w-0 w-full items-center gap-x-tiny min-h-[var(--tree-row-min-height,var(--tree-row-height))]";
export const treePropertyHeaderGridStyle: React.CSSProperties = { gridTemplateColumns: `minmax(0, 1fr) var(--tree-value-column, ${uiSpacingLen(STYLING_DOM.controlValueColumnUiSpacing)})` };
export const treeItemControlClassName =
  "min-w-0 w-full flex items-stretch justify-[var(--tree-value-justify,flex-end)] [&_[data-detail-panel-control='fill']]:min-w-0 [&_[data-detail-panel-control='fill']]:w-full [&_[data-detail-panel-control='fit']]:ms-auto [&_[data-detail-panel-control='fit']]:max-w-full [&_[data-detail-panel-control='fit']]:shrink-0";
/** 🌳️ Ancestor guide indices for a branch at {@link level}: parent level always continues through expanded children; deeper ancestors stop after last siblings. */
const treeBranchGuideIndices = (level: number, isLastAtLevel: readonly boolean[]): number[] => Array.from({ length: level }, (_, index) => index).filter((index) => index === level - 1 || !isLastAtLevel[index]);
const treeRowInlineGapPx = domSizePx("propertyInlineGapUiSpacing");
const treeToggleSlotWidthPx = domSizePx("treeToggleUiSpacing");
export const treeRowVerticalPaddingPx = 0;
const treeBranchRowGapPx = 0;
export const treeSectionContentPaddingTopPx = 0;
export const treeItemContentPaddingTopPx = 0;
export const treeCompactSiblingGapPx = 0;
export const treeSubtreeGapPx = 0;
const treeGutterToContentGapPx = treeRowInlineGapPx;
export const treePresentationLabelStyle: React.CSSProperties = { fontSize: "var(--tree-label-size, var(--text-xs))", lineHeight: "var(--tree-label-line-height, 1)" };
export const treeItemLabelSlotClassName = "flex h-full min-w-0 flex-1 items-center overflow-hidden text-xs font-normal leading-none select-text";
export const treeItemSecondaryTextClassName = "text-2xs leading-none text-muted-foreground";
export const treeSectionLabelSlotClassName = "flex h-full min-w-0 flex-1 items-center truncate text-xs font-semibold uppercase leading-none tracking-wide text-element transition-colors select-text";
export const treeSectionChevronClassName = "size-small flex-shrink-0 text-element transition-colors";
const treeRowDefaultIconClassName = "size-tiny flex-shrink-0 transition-colors";

/** 🖼️ Renders a tree row glyph before the label; uses {@link DefaultIcon} when `icon` is omitted. `emphasized` mirrors the row's active/highlighted/drop-ready state so the icon reads as clearly as the label beside it. */
export const renderTreeRowIcon = (icon: React.ReactNode | undefined, defaultIcon: IconName, emphasized = false) => (
  <span data-slot="tree-icon" className={cn("flex items-center justify-center flex-shrink-0 transition-colors", emphasized ? "text-emphasized" : "text-element")}>
    {icon ?? <Icon icon={defaultIcon} size={12} className={treeRowDefaultIconClassName} />}
  </span>
);
const treeGutterSlotLeftLen = (level: number, extraMultiplier = 0, multiplier = 1): string => (extraMultiplier > 0 ? `calc(${detailPanelIndentLen(level, multiplier)} + ${uiSpacingLen(extraMultiplier)})` : detailPanelIndentLen(level, multiplier));
const treeGutterWidthPx = (level: number, multiplier = 1): number => detailPanelIndentPx(level, multiplier) + treeToggleSlotWidthPx;
export const treeBranchContentStyle = (topPaddingPx = 0): React.CSSProperties => ({
  rowGap: treeBranchRowGapPx > 0 ? `${treeBranchRowGapPx}px` : "0",
  ...(topPaddingPx > 0 ? { paddingTop: `${topPaddingPx}px` } : {}),
});
export const getTreeSiblingGapPx = (_previousKind: string, _currentKind: string): number => treeCompactSiblingGapPx;

/** 📏️ IndentationLines holds the data fields for a IndentationLines record.
 **/
/**
 **/
export const IndentationLines: React.FC<{ level: number; showLines: boolean }> = ({ level, showLines }) => {
  const { indentMultiplier, isLastAtLevel } = reactHostPort.useContext(TreeContext);
  if (!showLines || level === 0) return null;

  const guideIndices = treeBranchGuideIndices(level, isLastAtLevel);
  return (
    <div data-dim data-slot="tree-guide" className="absolute start-0 top-0 bottom-0 pointer-events-none">
      {guideIndices.map((guideIndex) => (
        <div key={guideIndex} className="absolute top-0 bottom-0" style={{ insetInlineStart: `calc(${indentationLineLen(guideIndex, indentMultiplier)} - var(--stroke-hairline) / 2)` }}>
          <div data-tree-guide-line="" className={cn("w-px h-full", treeGuideLineStrokeClassName)} />
        </div>
      ))}
    </div>
  );
};

export const getTreeItemOrderedIds = (sections: TreeDataSection[], sectionItemsById: Record<string, TreeDataItem[]>, itemItemsById: Record<string, TreeDataItem[]>): string[] => {
  const orderedIds: string[] = [];

  const visitItems = (items: TreeDataItem[]) => {
    items.forEach((item) => {
      orderedIds.push(item.id);
      const childItems = getTreeItemItems(item, itemItemsById);
      if (childItems.length > 0) {
        visitItems(childItems);
      }
    });
  };

  sections.forEach((section) => {
    visitItems(getTreeSectionItems(section, sectionItemsById));
  });

  return orderedIds;
};

const treeSemanticHoverRowSelector = '[data-slot="tree-item-row"], [data-slot="tree-section-row"], [data-slot="tree-property-item"], [data-slot="tree-row"], [data-slot="control-tree-row"]';

export const treeSemanticHoverStaySelector = `${treeSemanticHoverRowSelector}, [data-slot="tree-section-content"], [data-slot="tree-item-content"], [data-slot="tree-property-content"], [data-slot="control-tree-folder-content"], [data-slot="window-measure-tree-content"]`;

/** 🎨️ Tree row shell: group + text tokens; gutter stays transparent so branch guides remain visible. */
export function treeRowChromeShellClasses(isSelected: boolean, isHighlighted: boolean, isHidden = false): string {
  const hiddenClass = isHidden ? "opacity-50 text-muted-foreground" : "";
  if (isSelected) {
    return cn("group/tree-row", "text-emphasized", interactiveControlTransitionClass, hiddenClass);
  }
  if (isHighlighted) {
    return cn("group/tree-row", "text-emphasized", interactiveControlTransitionClass, hiddenClass);
  }
  return cn("group/tree-row", "text-element", interactiveControlTransitionClass, hoverExcludingHandleTextEmphasizedClass, hiddenClass);
}

/** 🎨️ Tree row content fill: backgrounds apply only on the label column, not the guide gutter. */
export function treeRowChromeContentFillClasses(isSelected: boolean, isHighlighted: boolean, isLoading = false, isWaiting = false): string {
  const ringClass = loadingBorderStateClass(isLoading, isSelected) || waitingBorderStateClass(isWaiting, isSelected);
  if (isSelected) {
    return cn(interactiveActiveFillClass, interactiveControlTransitionClass, ringClass);
  }
  if (isHighlighted) {
    return cn("bg-hover-interactive-fill", interactiveControlTransitionClass, ringClass);
  }
  return cn(interactiveControlTransitionClass, groupHoverExcludingHandleBgFillClass, ringClass);
}

/** 🎨️ Tree row chrome: element gray at rest; hover highlight; selected primary + emphasized (no hover fill override). */
export function treeRowChromeClasses(isSelected: boolean, isHighlighted: boolean, isHidden = false, isLoading = false, isWaiting = false): string {
  return cn(treeRowChromeShellClasses(isSelected, isHighlighted, isHidden), treeRowChromeContentFillClasses(isSelected, isHighlighted, isLoading, isWaiting));
}

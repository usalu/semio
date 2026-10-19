/** 📏 Owns the tree row alignment contexts and detail-panel row metrics shared by Label and the tree. */
import * as React from "react";
import { STYLING_DOM, domSizePx, sizeVar, uiSpacingLen } from "@semio-tech/ui-styling";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";

/** 🧭️ Vertical unfold direction for a tree's foldable groups — `"up"` mirrors the {@link Ribbon} `"up"` pattern: a group's children render above its own header row, in reverse order, so the tree grows toward a fixed anchor (e.g. a bottom corner panel's pinned chrome) instead of away from it. */
export type TreeDirection = "down" | "up";

export const TreeContext = reactHostPort.createContext<{ level: number; isLastAtLevel: boolean[]; showLines: boolean; isTree: boolean; indentMultiplier: number; direction?: TreeDirection }>({
  level: 0,
  isLastAtLevel: [],
  showLines: true,
  isTree: false,
  indentMultiplier: 1,
  direction: "down",
});

export const TreeRowAlignmentContext = reactHostPort.createContext(false);

// True when children are rendered inside the value column of a Label property row.
export const PropertyValueColumnContext = reactHostPort.createContext(false);

export const detailPanelIndentLen = (level: number, multiplier = 1): string => `calc(${level * multiplier} * var(--tree-indent-per-level))`;

export const detailPanelIndentPx = (level: number, multiplier = 1): number => domSizePx("treeIndentPerLevelUiSpacing") * level * multiplier;

/** 📏️ The ONE tree row pitch, straight off `dom.treeRowUiSpacing` — every row shell is exactly this tall and every sibling gap is zero, so a virtual window's spacers are `rows × treeRowHeightPx` and nothing else. */
export const treeRowHeightPx = domSizePx("treeRowUiSpacing");

export const detailPanelHeaderLineCenterPx = treeRowHeightPx / 2;

const treeRowLayoutClassName = "grid min-w-0 h-full w-full";

const treeRowContentClassName = "min-w-0 h-full flex items-center";

export const detailPanelPropertyInlineGapPx = domSizePx("propertyInlineGapUiSpacing");

export const detailPanelPropertyStackedToInlineHysteresisPx = domSizePx("propertyStackedHysteresisUiSpacing");

export const detailPanelPropertyRowClassName = "group grid min-w-0 items-start gap-x-tiny min-h-workbench";

export const detailPanelPropertyControlClassName =
  "min-w-0 w-full self-start flex items-stretch justify-end [&_[data-detail-panel-control='fill']]:min-w-0 [&_[data-detail-panel-control='fill']]:w-full [&_[data-detail-panel-control='fit']]:ms-auto [&_[data-detail-panel-control='fit']]:max-w-full [&_[data-detail-panel-control='fit']]:shrink-0";

export const treeInspectorInnerRowClassName = "min-w-0 w-full";

export const treeHeaderRowClassName = "flex h-full min-w-0 w-full items-center gap-double";

export const treeHeaderMainClassName = "flex h-full min-w-0 flex-1 items-center gap-double overflow-hidden";

export const indentationLineLen = (i: number, multiplier = 1): string => `calc(${detailPanelIndentLen(i, multiplier)} + ${uiSpacingLen(STYLING_DOM.treeIndentLineExtraUiSpacing)})`;

const indentationLinePx = (i: number, multiplier = 1): number => detailPanelIndentPx(i, multiplier) + domSizePx("treeIndentLineExtraUiSpacing");

export const treeItemLabelStyle: React.CSSProperties = {};

export const treeGuideLineStrokeClassName = "bg-muted-foreground/40 group-hover/tree-row:bg-emphasized transition-[width,background-color] duration-150";

const treeGutterSlotLeftPx = (level: number, extraLeftPx = 0, multiplier = 1): number => detailPanelIndentPx(level, multiplier) + extraLeftPx;

const treeGutterAnchorTop = (_anchorOffsetPx?: number): string => "var(--tree-gutter-center, calc(var(--tree-row-height) / 2))";

const treeGutterSlotStyle = (level: number, extraLeftPx = 0, multiplier = 1, anchorOffsetPx?: number): React.CSSProperties => ({
  top: treeGutterAnchorTop(anchorOffsetPx),
  insetInlineStart: extraLeftPx > 0 ? `calc(${detailPanelIndentLen(level, multiplier)} + ${extraLeftPx}px)` : detailPanelIndentLen(level, multiplier),
});

const treeGutterWidthLen = (level: number, multiplier = 1): string => `calc(${detailPanelIndentLen(level, multiplier)} + var(--tree-toggle-width))`;

const treeAlignedRowStyle = (level: number, multiplier = 1): React.CSSProperties => ({
  gridTemplateColumns: `${treeGutterWidthLen(level, multiplier)} minmax(0, 1fr)`,
  columnGap: sizeVar("spacingDouble"),
});

interface TreeDocumentGutterProps {
  level: number;
  showLines: boolean;
  slot?: React.ReactNode;
  connectCurrentLevel?: boolean;
  /** 🌿️ Draw the current-level stem from the row anchor toward this group's children (`down` → bottom, `up` → top). */
  extendBranchStem?: boolean;
  slotOffsetPx?: number;
  anchorOffsetPx?: number;
}

const TreeDocumentGutter: React.FC<TreeDocumentGutterProps> = ({ level, showLines, slot, connectCurrentLevel = false, extendBranchStem = false, slotOffsetPx = 0, anchorOffsetPx }) => {
  const { indentMultiplier, direction = "down" } = reactHostPort.useContext(TreeContext);
  const currentGuidePx = indentationLinePx(level, indentMultiplier);
  const parentGuidePx = level > 0 ? indentationLinePx(level - 1, indentMultiplier) : 0;
  const hasSlot = slot !== null && slot !== undefined && slot !== false;
  const slotLeftPx = treeGutterSlotLeftPx(level, slotOffsetPx, indentMultiplier);
  const elbowEndPx = hasSlot ? slotLeftPx : currentGuidePx;
  const elbowWidthPx = Math.max(elbowEndPx - parentGuidePx, 0);
  const positionedSlot = hasSlot ? (
    <span data-slot="tree-gutter-slot" className="absolute flex -translate-y-1/2 items-center justify-center" style={treeGutterSlotStyle(level, slotOffsetPx, indentMultiplier, anchorOffsetPx)}>
      {slot}
    </span>
  ) : null;
  const branchStemStyle: React.CSSProperties =
    direction === "up"
      ? { top: 0, bottom: treeGutterAnchorTop(anchorOffsetPx), insetInlineStart: `calc(${indentationLineLen(level, indentMultiplier)} - var(--stroke-hairline) / 2)` }
      : { top: treeGutterAnchorTop(anchorOffsetPx), bottom: 0, insetInlineStart: `calc(${indentationLineLen(level, indentMultiplier)} - var(--stroke-hairline) / 2)` };

  return (
    <div data-dim data-slot="tree-gutter" className="relative min-h-full" style={{ width: treeGutterWidthLen(level, indentMultiplier), minWidth: treeGutterWidthLen(level, indentMultiplier) }}>
      {showLines && level > 0 && connectCurrentLevel && (
        <div
          data-slot="tree-branch-elbow"
          className={cn("pointer-events-none absolute h-px -translate-y-1/2", treeGuideLineStrokeClassName)}
          style={{ top: treeGutterAnchorTop(anchorOffsetPx), insetInlineStart: indentationLineLen(level - 1, indentMultiplier), width: `${elbowWidthPx}px` }}
        />
      )}
      {showLines && extendBranchStem && <div data-slot="tree-branch-stem" className={cn("pointer-events-none absolute w-px", treeGuideLineStrokeClassName)} style={branchStemStyle} />}
      {positionedSlot}
    </div>
  );
};

interface TreeAlignedRowProps {
  level: number;
  isLastAtLevel: boolean[];
  showLines: boolean;
  slot?: React.ReactNode;
  children: React.ReactNode;
  className?: string;
  contentClassName?: string;
  contentChromeClassName?: string;
  align?: "start" | "center";
  connectCurrentLevel?: boolean;
  extendBranchStem?: boolean;
  slotOffsetPx?: number;
  anchorOffsetPx?: number;
}

export const TreeAlignedRow: React.FC<TreeAlignedRowProps> = ({
  level,
  isLastAtLevel,
  showLines,
  slot,
  children,
  className,
  contentClassName,
  contentChromeClassName,
  align = "center",
  connectCurrentLevel = false,
  extendBranchStem = false,
  slotOffsetPx = 0,
  anchorOffsetPx,
}) => {
  const { indentMultiplier } = reactHostPort.useContext(TreeContext);
  return (
    <div data-slot="tree-row-layout" className={cn(treeRowLayoutClassName, align === "start" ? "items-start" : "items-center", className)} style={treeAlignedRowStyle(level, indentMultiplier)}>
      <TreeDocumentGutter level={level} showLines={showLines} slot={slot} connectCurrentLevel={connectCurrentLevel} extendBranchStem={extendBranchStem} slotOffsetPx={slotOffsetPx} anchorOffsetPx={anchorOffsetPx} />
      <div data-slot="tree-row-content" className={cn(align === "start" ? "min-w-0 h-full" : treeRowContentClassName, contentChromeClassName, contentClassName)}>
        {children}
      </div>
    </div>
  );
};

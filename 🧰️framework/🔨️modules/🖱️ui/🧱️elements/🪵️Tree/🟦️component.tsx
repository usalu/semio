// #region 🧲️Header
// 💻️ framework/ui/elements/🪵️Tree/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
import * as React from "react";
import { closestCenter, DndContext, type DragEndEvent } from "@dnd-kit/core";
import { SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { createPortal } from "react-dom";
import { STYLING_DOM, STYLING_COMPACT_ROOT_PX, domSizePx, sizeVar } from "@semio-tech/ui-styling";
import { type IconName } from "@semio-tech/assets";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext/.useState at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { type UiLabel, uiDataLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { Action } from "../⚡️ActionGroup/🟦️component.tsx";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "../↕️Collapsible/🟦️component.tsx";
import { Input } from "../✏️Input/🟦️component.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../☑️Select/🟦️component.tsx";
import { Slider } from "../🎚️Slider/🟦️component.tsx";
import { Stepper } from "../🪜️Stepper/🟦️component.tsx";
import { Textarea } from "../📄️Textarea/🟦️component.tsx";
import { Toggle } from "../🎚️Toggle/🟦️component.tsx";
import { childElementId } from "../🆔️ElementId/🟦️component.tsx";
import { borderNormalClass } from "../../🔨️modules/📏️border-presentation/🟦️component.ts";
import { interactiveActiveFillClass, interactiveControlTransitionClass, hoverExcludingHandleTextEmphasizedClass, groupHoverExcludingHandleBgFillClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️component.ts";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️component.ts";
import {
  dropZoneReadyFillClass,
  dropZoneReadyTextClass,
  loadingBorderStateClass,
  waitingBorderStateClass,
  panelTabIconSlotClass,
  panelTabLabelClass,
  windowMeasureTreeGroupLabelClass,
  windowMeasureTreeLeafLabelClass,
  windowPaneChromeToggleClass,
} from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { useLabel, Label, resolveTranslationLabel, useIdLabel, useUiTranslation, useControlAccessibleLabel, useControlInlineText, useControlTooltipText } from "../🏷️Label/🟦️component.tsx";
import { useFlow, FlowProvider, type FlowBlock, type FlowInline } from "../../🔨️modules/🧭️flow-direction-context/🟦️component.tsx";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️component.ts";
import { useShellScopeOptional } from "../🐚️ShellScope/🟦️component.tsx";
import { usePanelGhost, useUiDriverDragSurface, TREE_SECTION_REORDER_MIME, interactionMergeFromModifiers } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { Icon, renderControlIcon, type ControlIcon, CheckIcon, ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon, ChevronUpIcon, CloseIcon, DocumentIcon, FolderIcon } from "../🔣️Icons/🟦️component.tsx";
import { DragHandle } from "../🧱️DragHandle/🟦️component.tsx";
import { ContextMenu, type ContextMenuItem } from "../🖱️ContextMenu/🟦️component.tsx";
import { Button } from "../🔘️Button/🟦️component.tsx";
// 🕹️wave-0: imported directly from the module's own source (see react/📦️index.tsx's header comment on
// this same import for why not via `@semio-tech/framework`'s barrel).
import { nextSelection, validateState, type DomainSelection, type DomainTopology, type InteractionDefinition } from "../../../🕹️interaction/🟦️component.ts";
// #endregion 🔌️Adapters

// #region 📜️Tree
// Hierarchical tree view with sections, items, and file trees.
// Consumers MUST wrap components in TreeStateProvider.

/**
 * TreeStateContextValue holds the data fields for a TreeStateContextValue record.
 **/
interface TreeStateContextValue {
  openStates: Readonly<Record<string, boolean>>;
  setOpenState: (id: string, open: boolean) => void;
  getOpenState: (id: string, defaultOpen: boolean) => boolean;
}

/**
 * TreeStateContext holds the data fields for a TreeStateContext record.
 **/
const TreeStateContext = reactHostPort.createContext<TreeStateContextValue | null>(null);

/**
 * Context provider managing tree expansion state — controlled via `openStates`/`onOpenStateChange` when a host
 * wants expansion to survive remounts (e.g. persisted across tab switches or reloads); uncontrolled (internal
 * state) otherwise. The map only ever holds explicit toggles — {@link useTreeOpenState}'s `getOpenState` falls
 * back to each item's own `defaultOpen` — so it already is the diff-from-default a host would persist.
 **/
export const TreeStateProvider: React.FC<{
  children: React.ReactNode;
  openStates?: Readonly<Record<string, boolean>>;
  onOpenStateChange?: (id: string, open: boolean) => void;
}> = ({ children, openStates: controlledOpenStates, onOpenStateChange }) => {
  const [internalOpenStates, setInternalOpenStates] = reactHostPort.useState<Record<string, boolean>>({});
  const openStates = controlledOpenStates ?? internalOpenStates;

  const setOpenState = (id: string, open: boolean) => {
    if (onOpenStateChange) {
      onOpenStateChange(id, open);
    } else {
      setInternalOpenStates((prev) => ({ ...prev, [id]: open }));
    }
  };

  const getOpenState = (id: string, defaultOpen: boolean) => {
    return openStates[id] !== undefined ? openStates[id] : defaultOpen;
  };

  return <TreeStateContext.Provider value={{ openStates, setOpenState, getOpenState }}>{children}</TreeStateContext.Provider>;
};

/**
 * Hook returning tree expansion state and toggle functions.
 **/
export const useTreeState = () => {
  const context = reactHostPort.useContext(TreeStateContext);
  if (!context) throw new Error("useTreeState must be used within TreeStateProvider");
  return context;
};

const useTreeOpenState = (itemId: string, defaultOpen: boolean) => {
  const treeState = reactHostPort.useContext(TreeStateContext);
  const [fallbackOpen, setFallbackOpen] = reactHostPort.useState(defaultOpen);
  const open = treeState ? treeState.getOpenState(itemId, defaultOpen) : fallbackOpen;
  const setOpen = reactHostPort.useCallback(
    (value: boolean) => {
      if (treeState) {
        treeState.setOpenState(itemId, value);
        return;
      }
      setFallbackOpen(value);
    },
    [itemId, treeState],
  );
  return { open, setOpen };
};

const treeSectionElementMarker = Symbol.for("ui.tree.section");
const treeSectionDoubleClickDelayMs = 300;

type TreeComponentMarker = {
  [treeSectionElementMarker]?: boolean;
  displayName?: string;
};

const isTreeSectionElementType = (value: unknown): boolean => {
  if ((typeof value !== "function" && typeof value !== "object") || value === null) {
    return false;
  }
  return Boolean((value as TreeComponentMarker)[treeSectionElementMarker]);
};

const hasNonEmptyChildren = (children: React.ReactNode): boolean => {
  if (!children) return false;
  const childArray = React.Children.toArray(children);
  return (
    childArray.length > 0 &&
    childArray.some((child) => {
      if (React.isValidElement(child)) return true;
      if (typeof child === "string" && child.trim().length > 0) return true;
      if (typeof child === "number") return true;
      return false;
    })
  );
};

const isIgnorableTreeChild = (child: React.ReactNode): boolean => child === null || child === undefined || typeof child === "boolean" || (typeof child === "string" && child.trim().length === 0);

const assertNoNestedTreeSections = (children: React.ReactNode, ownerName: "TreeSection" | "TreeItem") => {
  const visitNestedChildren = (value: React.ReactNode) => {
    React.Children.forEach(value, (child) => {
      if (isIgnorableTreeChild(child)) {
        return;
      }
      if (!React.isValidElement(child)) {
        return;
      }
      const childProps = child.props as { children?: React.ReactNode };
      if (child.type === React.Fragment) {
        visitNestedChildren(childProps.children);
        return;
      }
      if (isTreeSectionElementType(child.type)) {
        throw new Error(`${ownerName} cannot contain a TreeSection. Only TreeItem elements can be nested.`);
      }
      visitNestedChildren(childProps.children);
    });
  };

  visitNestedChildren(children);
};

/** @emoji 🧭️ Vertical unfold direction for a tree's foldable groups — `"up"` mirrors the {@link Ribbon} `"up"` pattern: a group's children render above its own header row, in reverse order, so the tree grows toward a fixed anchor (e.g. a bottom corner panel's pinned chrome) instead of away from it. */
export type TreeDirection = "down" | "up";

export const TreeContext = reactHostPort.createContext<{ level: number; isLastAtLevel: boolean[]; showLines: boolean; isTree: boolean; indentMultiplier: number; direction?: TreeDirection }>({
  level: 0,
  isLastAtLevel: [],
  showLines: true,
  isTree: false,
  indentMultiplier: 1,
  direction: "down",
});

/** @emoji 🧭️ Fold-affordance chevron for a tree group row — points toward where the content actually is: along the block axis when open (down, or up when {@link direction} is `"up"`), along the {@link FlowInline}-mirrored inline axis when closed (right in ltr/down or rtl/up, left in ltr/up or rtl/down). */
export function treeFoldChevronIcon(direction: TreeDirection, inline: FlowInline, open: boolean): React.ComponentType<{ className?: string }> {
  if (open) return direction === "up" ? ChevronUpIcon : ChevronDownIcon;
  const towardStart = direction === "up";
  return towardStart === (inline === "rtl") ? ChevronRightIcon : ChevronLeftIcon;
}
export const TreeRowAlignmentContext = reactHostPort.createContext(false);
// True when children are rendered inside the value column of a Label property row.
export const PropertyValueColumnContext = reactHostPort.createContext(false);
export const uiSpacingLen = (multiplier: number): string => `calc(${multiplier} * var(--ui-spacing))`;
export const detailPanelIndentLen = (level: number, multiplier = 1): string => uiSpacingLen(level * STYLING_DOM.treeIndentPerLevelUiSpacing * multiplier);
export const detailPanelIndentPx = (level: number, multiplier = 1): number => domSizePx("treeIndentPerLevelUiSpacing") * level * multiplier;
const treeRowHeightPx = domSizePx("treeRowUiSpacing");
export const detailPanelHeaderLineCenterPx = treeRowHeightPx / 2;
const treeRowShellClassName = "relative h-workbench min-h-workbench max-h-workbench w-full min-w-0 select-none overflow-hidden";
const treeRowLayoutClassName = "grid min-w-0 h-full w-full";
const treeRowContentClassName = "min-w-0 h-full flex items-center";
const detailPanelPropertyLabelColumnWidthPx = domSizePx("propertyLabelColumnUiSpacing");
export const detailPanelPropertyInlineGapPx = domSizePx("propertyInlineGapUiSpacing");
const detailPanelPropertyStackedRowGapPx = domSizePx("propertyStackedGapUiSpacing");
export const detailPanelPropertyStackedToInlineHysteresisPx = domSizePx("propertyStackedHysteresisUiSpacing");
export const detailPanelPropertyRowClassName = "group grid min-w-0 items-start gap-x-tiny min-h-workbench";
export const detailPanelPropertyControlClassName =
  "min-w-0 w-full self-start flex items-stretch justify-end [&_[data-detail-panel-control='fill']]:min-w-0 [&_[data-detail-panel-control='fill']]:w-full [&_[data-detail-panel-control='fit']]:ms-auto [&_[data-detail-panel-control='fit']]:max-w-full [&_[data-detail-panel-control='fit']]:shrink-0";
export const treeInspectorInnerRowClassName = "min-w-0 w-full";
export const treeHeaderRowClassName = "flex h-full min-w-0 w-full items-center gap-double";
export const treeHeaderMainClassName = "flex h-full min-w-0 flex-1 items-center gap-double overflow-hidden";
const treeHeaderActionsClassName = "flex flex-shrink-0 items-center gap-single";
const treePropertyHeaderGridClassName = "grid min-w-0 w-full items-center gap-x-tiny min-h-workbench";
const treePropertyHeaderGridStyle: React.CSSProperties = { gridTemplateColumns: `minmax(0, 1fr) ${uiSpacingLen(STYLING_DOM.controlValueColumnUiSpacing)}` };
const treeItemControlClassName =
  "min-w-0 w-full flex items-stretch justify-end [&_[data-detail-panel-control='fill']]:min-w-0 [&_[data-detail-panel-control='fill']]:w-full [&_[data-detail-panel-control='fit']]:ms-auto [&_[data-detail-panel-control='fit']]:max-w-full [&_[data-detail-panel-control='fit']]:shrink-0";
const indentationLineLen = (i: number, multiplier = 1): string => `calc(${detailPanelIndentLen(i, multiplier)} + ${uiSpacingLen(STYLING_DOM.treeIndentLineExtraUiSpacing)})`;
const indentationLinePx = (i: number, multiplier = 1): number => detailPanelIndentPx(i, multiplier) + domSizePx("treeIndentLineExtraUiSpacing");
/** @emoji 🌳️ Ancestor guide indices for a branch at {@link level}: parent level always continues through expanded children; deeper ancestors stop after last siblings. */
const treeBranchGuideIndices = (level: number, isLastAtLevel: readonly boolean[]): number[] => Array.from({ length: level }, (_, index) => index).filter((index) => index === level - 1 || !isLastAtLevel[index]);
const treeRowInlineGapPx = domSizePx("propertyInlineGapUiSpacing");
const treeToggleSlotWidthPx = domSizePx("treeToggleUiSpacing");
const treeRowVerticalPaddingPx = 0;
const treeBranchRowGapPx = 0;
const treeSectionContentPaddingTopPx = 0;
const treeItemContentPaddingTopPx = 0;
export const treeCompactSiblingGapPx = 0;
const treeSubtreeGapPx = 0;
const treeGutterToContentGapPx = treeRowInlineGapPx;
export const treeItemLabelStyle: React.CSSProperties = {};
const treeGuideLineStrokeClassName = "bg-muted-foreground/40 group-hover/tree-row:bg-emphasized transition-[width,background-color] duration-150";
const treeItemLabelSlotClassName = "flex h-full min-w-0 flex-1 items-center overflow-hidden text-xs font-normal leading-none select-text";
export const treeItemSecondaryTextClassName = "text-2xs leading-none text-muted-foreground";
const treeSectionLabelSlotClassName = "flex h-full min-w-0 flex-1 items-center truncate text-xs font-semibold uppercase leading-none tracking-wide text-element transition-colors select-text";
const treeSectionChevronClassName = "size-small flex-shrink-0 text-element transition-colors";
const treeRowDefaultIconClassName = "size-tiny flex-shrink-0 transition-colors";

/** @emoji 🖼️ Renders a tree row glyph before the label; uses {@link DefaultIcon} when `icon` is omitted. `emphasized` mirrors the row's active/highlighted/drop-ready state so the icon reads as clearly as the label beside it. */
const renderTreeRowIcon = (icon: React.ReactNode | undefined, defaultIcon: IconName, emphasized = false) => (
  <span data-slot="tree-icon" className={cn("flex items-center justify-center flex-shrink-0 transition-colors", emphasized ? "text-emphasized" : "text-element")}>
    {icon ?? <Icon icon={defaultIcon} size={12} className={treeRowDefaultIconClassName} />}
  </span>
);
const treeGutterSlotLeftLen = (level: number, extraMultiplier = 0, multiplier = 1): string => (extraMultiplier > 0 ? `calc(${detailPanelIndentLen(level, multiplier)} + ${uiSpacingLen(extraMultiplier)})` : detailPanelIndentLen(level, multiplier));
const treeGutterSlotLeftPx = (level: number, extraLeftPx = 0, multiplier = 1): number => detailPanelIndentPx(level, multiplier) + extraLeftPx;
const treeGutterAnchorTop = (_anchorOffsetPx?: number): string => "calc(var(--size-workbench) / 2)";
const treeGutterSlotStyle = (level: number, extraLeftPx = 0, multiplier = 1, anchorOffsetPx?: number): React.CSSProperties => ({
  top: treeGutterAnchorTop(anchorOffsetPx),
  insetInlineStart: extraLeftPx > 0 ? `calc(${detailPanelIndentLen(level, multiplier)} + ${uiSpacingLen(extraLeftPx / (STYLING_COMPACT_ROOT_PX * 0.2))})` : detailPanelIndentLen(level, multiplier),
});
const treeGutterWidthLen = (level: number, multiplier = 1): string => `calc(${detailPanelIndentLen(level, multiplier)} + ${uiSpacingLen(STYLING_DOM.treeToggleUiSpacing)})`;
const treeGutterWidthPx = (level: number, multiplier = 1): number => detailPanelIndentPx(level, multiplier) + treeToggleSlotWidthPx;
const treeBranchContentStyle = (topPaddingPx = 0): React.CSSProperties => ({
  rowGap: treeBranchRowGapPx > 0 ? uiSpacingLen(treeBranchRowGapPx / (STYLING_COMPACT_ROOT_PX * 0.2)) : "0",
  ...(topPaddingPx > 0 ? { paddingTop: uiSpacingLen(topPaddingPx / (STYLING_COMPACT_ROOT_PX * 0.2)) } : {}),
});
export const getTreeSiblingGapPx = (_previousKind: string, _currentKind: string): number => treeCompactSiblingGapPx;
const treeAlignedRowStyle = (level: number, multiplier = 1): React.CSSProperties => ({
  gridTemplateColumns: `${treeGutterWidthLen(level, multiplier)} minmax(0, 1fr)`,
  columnGap: sizeVar("spacingDouble"),
});

/** IndentationLines holds the data fields for a IndentationLines record.
 **/
/**
 **/
const IndentationLines: React.FC<{ level: number; showLines: boolean }> = ({ level, showLines }) => {
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

interface TreeDocumentGutterProps {
  level: number;
  showLines: boolean;
  slot?: React.ReactNode;
  connectCurrentLevel?: boolean;
  /** @emoji 🌿️ Draw the current-level stem from the row anchor toward this group's children (`down` → bottom, `up` → top). */
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
          style={{ top: treeGutterAnchorTop(anchorOffsetPx), insetInlineStart: indentationLineLen(level - 1, indentMultiplier), width: `calc(${uiSpacingLen(elbowWidthPx / (STYLING_COMPACT_ROOT_PX * 0.2))})` }}
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
}

const TreeHoverPathRefreshContext = reactHostPort.createContext<(() => void) | null>(null);

const TreeBranchContent: React.FC<TreeBranchContentProps> = ({ slot, children, className, topPaddingPx = 0, ownerRowKind, ownerExpanded = false }) => {
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
    <div ref={branchRef} data-slot={slot} data-tree-owner-kind={ownerRowKind} data-tree-owner-expanded={ownerExpanded ? "true" : "false"} className={cn("relative flex w-full min-w-0 flex-col", className)} style={treeBranchContentStyle(topPaddingPx)}>
      {isTree ? <IndentationLines level={level} showLines={showLines} /> : null}
      {children}
    </div>
  );
};

export type TreeActionPlacement = "row" | "menu";

/**
 * Configuration interface for an action button on a tree section.
 **/
export interface TreeSectionAction {
  kind?: "button";
  icon: ControlIcon;
  onClick: () => void;
  title?: UiLabel;
  text?: string;
  id?: string;
  disabled?: boolean;
  /** @emoji 📍️ Row actions paint on the header; menu actions appear in the row context menu. */
  placement?: TreeActionPlacement;
}

/**
 * Configuration interface for a checkbox action on a tree header row.
 **/
export interface TreeCheckboxAction {
  kind: "checkbox";
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  title?: UiLabel;
  id?: string;
  disabled?: boolean;
  ariaLabel?: string;
}

export type TreeHeaderAction = TreeSectionAction | TreeCheckboxAction;

export type TreeDragRole = "sort" | "transfer";

/** @emoji 🫳️ Which drag handles a tree row exposes under the default driver. */
export function deriveTreeDragRoles(item: { readonly draggable?: boolean; readonly dragData?: Record<string, string>; readonly isDragHandle?: boolean }, paletteDragEnabled: boolean): readonly TreeDragRole[] {
  const roles: TreeDragRole[] = [];
  const hasTransfer = Boolean(item.dragData) || (paletteDragEnabled && (item.draggable || item.dragData));
  const hasSort = Boolean(item.isDragHandle) || Boolean(item.draggable && !item.dragData);
  if (hasSort) roles.push("sort");
  if (hasTransfer) roles.push("transfer");
  return roles;
}

function treeSectionActionPlacement(action: TreeSectionAction): TreeActionPlacement {
  return action.placement ?? "row";
}

function rowTreeHeaderActions(actions: readonly TreeHeaderAction[]): TreeHeaderAction[] {
  return actions.filter((action) => action.kind === "checkbox" || treeSectionActionPlacement(action) === "row");
}

function menuTreeHeaderActionsToContextItems(actions: readonly TreeHeaderAction[]): ContextMenuItem[] {
  return actions
    .filter((action): action is TreeSectionAction => action.kind !== "checkbox" && treeSectionActionPlacement(action) === "menu")
    .map((action, index) => ({
      id: action.id ?? `tree-row-action-${index}`,
      label: action.text !== undefined ? uiDataLabel(action.text) : (action.title ?? uiDataLabel("")),
      icon: action.icon,
      disabled: action.disabled,
      onSelect: () => action.onClick(),
    }));
}

/** @emoji 🖱️ Merges menu-placement row actions into a host-built context menu. */
export function mergeTreeRowContextMenu(actions: readonly TreeHeaderAction[] | undefined, contextMenu: readonly ContextMenuItem[] | undefined): ContextMenuItem[] | undefined {
  const fromActions = menuTreeHeaderActionsToContextItems(actions ?? []);
  if (fromActions.length === 0) {
    return contextMenu?.length ? [...contextMenu] : undefined;
  }
  if (!contextMenu?.length) {
    return fromActions;
  }
  return [...contextMenu, { id: "tree-row-action-separator", separator: true }, ...fromActions];
}

type TreeDragHandleRenderProps = {
  readonly roles: readonly TreeDragRole[];
  readonly driverSurfaceDrag: boolean;
  readonly rowEmphasized: boolean;
  readonly armDrag?: () => void;
  readonly sortDndProps?: { readonly attributes?: object; readonly listeners?: Record<string, unknown> };
  readonly transferPointerDown?: React.PointerEventHandler<HTMLSpanElement>;
  readonly onSortHandleClick?: React.MouseEventHandler<HTMLSpanElement>;
};

const renderTreeDragHandles = ({ roles, driverSurfaceDrag, rowEmphasized, armDrag, sortDndProps, transferPointerDown, onSortHandleClick }: TreeDragHandleRenderProps): React.ReactNode => {
  if (driverSurfaceDrag || roles.length === 0) {
    return null;
  }
  return (
    <>
      {roles.includes("sort") ? (
        <DragHandle
          labelId="ui.tree.drag.sort"
          iconKind="grip-vertical"
          emphasized={rowEmphasized}
          onPointerDown={armDrag ? () => armDrag() : undefined}
          attributes={sortDndProps?.attributes}
          listeners={sortDndProps?.listeners}
          onClick={onSortHandleClick}
        />
      ) : null}
      {roles.includes("transfer") ? (
        <DragHandle
          labelId="ui.tree.drag.transfer"
          iconKind="move"
          emphasized={rowEmphasized}
          onPointerDown={(event) => {
            armDrag?.();
            transferPointerDown?.(event);
          }}
        />
      ) : null}
    </>
  );
};

export interface TreeCheckboxProps {
  id?: string;
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  title?: UiLabel;
  disabled?: boolean;
  ariaLabel?: string;
}

/** @emoji ☑️ Compact tree-row checkbox used as a property control or header action. */
export const TreeCheckbox: React.FC<TreeCheckboxProps> = ({ id, checked, onCheckedChange, title, disabled, ariaLabel }) => (
  <label
    data-slot="tree-action-checkbox-wrapper"
    className="inline-flex h-medium min-w-tiny flex-shrink-0 cursor-pointer items-center justify-center"
    title={title}
    onClick={(event) => {
      event.preventDefault();
      event.stopPropagation();
    }}
  >
    <input
      data-slot="tree-action-checkbox"
      id={id}
      type="checkbox"
      className="m-0 size-tiny cursor-pointer accent-foreground"
      aria-label={ariaLabel ?? title ?? id}
      checked={checked}
      disabled={disabled}
      onChange={(event) => {
        event.stopPropagation();
        onCheckedChange(event.currentTarget.checked);
      }}
    />
  </label>
);

const renderTreeHeaderAction = (action: TreeHeaderAction, key: React.Key) =>
  action.kind === "checkbox" ? (
    <TreeCheckbox key={key} id={action.id} checked={action.checked} onCheckedChange={action.onCheckedChange} title={action.title} disabled={action.disabled} ariaLabel={action.ariaLabel} />
  ) : (
    <span key={key}>
      <Action
        onClick={(event) => {
          event.preventDefault();
          event.stopPropagation();
          action.onClick();
        }}
        id={action.id}
        icon={action.icon}
        text={action.text ?? action.title}
        disabled={action.disabled}
      />
    </span>
  );

const renderTreeHeaderActions = (actions: TreeHeaderAction[]) => {
  const rowActions = rowTreeHeaderActions(actions);
  if (rowActions.length === 0) {
    return null;
  }
  return (
    <div data-slot="tree-header-actions" data-ui-reveal-region="" className={treeHeaderActionsClassName}>
      {rowActions.map((action, index) => renderTreeHeaderAction(action, action.id ?? index))}
    </div>
  );
};

export enum TreeItemCollapsibleState {
  None = 0,
  Collapsed = 1,
  Expanded = 2,
}

export type TreeSelectionMode = "single" | "multiple";

export interface TreeDataActivationContext {
  path: string[];
  selectedIds: readonly string[];
  sectionId: string;
}

export interface TreeDataItem {
  id: string;
  label: React.ReactNode;
  icon?: React.ReactNode;
  description?: React.ReactNode;
  /** @emoji 🎛️ Inline control rendered in a property-style tree row. */
  control?: React.ReactNode;
  items?: TreeDataItem[];
  getItems?: () => Promise<TreeDataItem[]>;
  /** Alternative branches for this item. Each branch is an array of child items. Navigation < > switches between branches. */
  alternatives?: TreeDataItem[][];
  actions?: TreeHeaderAction[];
  className?: string;
  isHighlighted?: boolean;
  isSelected?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  /** @emoji 🌀️ Host-declared loading state, ORed with the tree's own async {@link getItems} pending state. */
  loading?: boolean;
  /** @emoji 🌀️ Host-declared waiting state; dashed, slower ring than {@link loading}. */
  waiting?: boolean;
  collapsibleState?: TreeItemCollapsibleState;
  emptyState?: React.ReactNode;
  draggable?: boolean;
  /** @emoji 📤️ Extra `dataTransfer` MIME entries merged on drag start (in-app palette drags). */
  dragData?: Record<string, string>;
  onClick?: (event: React.MouseEvent, context: TreeDataActivationContext) => void;
  onDoubleClick?: (event: React.MouseEvent, context: TreeDataActivationContext) => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  /** @emoji 👁️ Muted row styling for hidden entities in scene outliners. */
  isHidden?: boolean;
  /** @emoji 🖱️ Right-click menu for the row (selection-aware actions are built by the host). */
  contextMenu?: ContextMenuItem[];
}

export interface TreeDataSection {
  id: string;
  label?: React.ReactNode;
  icon?: React.ReactNode;
  items?: TreeDataItem[];
  getItems?: () => Promise<TreeDataItem[]>;
  actions?: TreeHeaderAction[];
  className?: string;
  defaultOpen?: boolean;
  /** @emoji 🌀️ Host-declared loading state, ORed with the tree's own async {@link getItems} pending state. */
  loading?: boolean;
  /** @emoji 🌀️ Host-declared waiting state; dashed, slower ring than {@link loading}. */
  waiting?: boolean;
  emptyState?: React.ReactNode;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
  /** @emoji ↕️ When true (or when the host Tree enables {@link TreeRootProps.sortableSections}), this section header shows a drag handle for reordering among sibling sections. */
  draggable?: boolean;
}

/** @emoji 🖱️ Pointer-driven external drag when native `draggable` does not start inside scroll panels. */
export interface TreePointerPaletteDragController {
  readEncodedDragPayload: (dragData: Record<string, string>) => string | undefined;
  begin: (encoded: string) => void;
  cancel: () => void;
}

export interface TreeDragAndDropController {
  getDragData?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => Record<string, string> | undefined;
  pointerPaletteDrag?: TreePointerPaletteDragController;
  onDragStart?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => void;
  onDragEnd?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => void;
  handleDrop?: (context: { target: TreeDataItem | TreeDataSection; targetKind: "item" | "section"; data: Record<string, string>; sourceItems: TreeDataItem[]; section: TreeDataSection; dropPosition?: TreeDropPosition }) => void | Promise<void>;
}

export type TreeDropPosition = "before" | "after" | "inside";

/** @emoji 📍️ Resolves whether a tree drop lands before, after, or inside a row. */
export function resolveTreeDropPosition(event: React.DragEvent<HTMLElement>): TreeDropPosition {
  const rect = event.currentTarget.getBoundingClientRect();
  const y = event.clientY - rect.top;
  if (y < rect.height * 0.25) return "before";
  if (y > rect.height * 0.75) return "after";
  return "inside";
}

/** @emoji 🔀️ True when a drag carries only internal tree reorder data (not palette payloads). */
export function isTreeReorderDragEvent(event: React.DragEvent): boolean {
  const types = [...event.dataTransfer.types];
  if (!types.includes("application/vnd.code.tree.item")) return false;
  return !types.some((kind) => kind !== "application/vnd.code.tree.item" && kind.startsWith("application/"));
}

/** @emoji 👁️ Fixed overlay showing where a tree reorder drop will land. */
function TreeReorderDropPreview(props: { readonly preview: { readonly targetId: string; readonly position: TreeDropPosition } | null }): React.ReactElement | null {
  const [frame, setFrame] = reactHostPort.useState<DOMRect | null>(null);
  // 🐚️ Falls back to `document`/`document.body` outside any shell — inside one, scopes the id lookup and
  // portal target to that shell's own root/overlay layer (a tree row id is not guaranteed unique across
  // several mounted shells of the same plugin).
  const shellScope = useShellScopeOptional();
  reactHostPort.useLayoutEffect(() => {
    if (!props.preview || typeof document === "undefined") {
      setFrame(null);
      return;
    }
    const searchRoot: ParentNode = shellScope?.rootRef.current ?? document;
    const row = searchRoot.querySelector(`[id=${JSON.stringify(props.preview.targetId)}]`);
    setFrame(row?.getBoundingClientRect() ?? null);
  }, [props.preview, shellScope]);
  const portalTarget = shellScope?.portalLayerRef.current ?? (typeof document !== "undefined" ? document.body : null);
  if (!props.preview || !frame || !portalTarget) return null;
  if (props.preview.position === "before") {
    return createPortal(<div data-slot="tree-drop-preview" className="pointer-events-none fixed z-tutorial h-0.5 bg-primary" style={{ left: frame.left, top: frame.top, width: frame.width }} />, portalTarget);
  }
  if (props.preview.position === "after") {
    return createPortal(<div data-slot="tree-drop-preview" className="pointer-events-none fixed z-tutorial h-0.5 bg-primary" style={{ left: frame.left, top: frame.bottom - 2, width: frame.width }} />, portalTarget);
  }
  return createPortal(
    <div data-slot="tree-drop-preview" className="pointer-events-none fixed z-tutorial border-2 border-primary/80 bg-primary/10" style={{ left: frame.left, top: frame.top, width: frame.width, height: frame.height }} />,
    portalTarget,
  );
}

export interface TreeReorderMove {
  readonly itemId: string;
  readonly fromParentId: string;
  readonly toParentId: string;
  readonly index: number;
  readonly position: TreeDropPosition;
}

export interface TreeReorderControllerOptions {
  readonly onMove: (move: TreeReorderMove) => void;
  readonly resolveParentId?: (item: TreeDataItem) => string | undefined;
}

/** @emoji 🔀️ Builds a {@link TreeDragAndDropController} that emits cross-container reorder moves. */
export function treeReorderDragController(options: TreeReorderControllerOptions): TreeDragAndDropController {
  return {
    handleDrop: ({ target, targetKind, sourceItems, dropPosition }) => {
      const sourceItem = sourceItems[0];
      if (!sourceItem || targetKind !== "item") return;
      const targetItem = target as TreeDataItem;
      const fromParentId = options.resolveParentId?.(sourceItem) ?? sourceItem.id.split(":")[0] ?? "";
      const toParentId = options.resolveParentId?.(targetItem) ?? targetItem.id.split(":")[0] ?? "";
      const position = dropPosition ?? "inside";
      const index = position === "before" ? 0 : position === "after" ? Number.MAX_SAFE_INTEGER : 0;
      options.onMove({ itemId: sourceItem.id, fromParentId, toParentId, index, position });
    },
  };
}

export interface UseTreeReorderResult {
  readonly dropIndicatorId: string | null;
  readonly dropPosition: TreeDropPosition | null;
  readonly dragController: TreeDragAndDropController;
}

/** @emoji 🔀️ Hook wiring tree reorder callbacks to drop-position aware drag handling. */
export function useTreeReorder(onMove: (move: TreeReorderMove) => void, resolveParentId?: (item: TreeDataItem) => string | undefined): UseTreeReorderResult {
  const dropStateRef = reactHostPort.useRef<{ id: string | null; position: TreeDropPosition | null }>({ id: null, position: null });
  const [, bump] = reactHostPort.useState(0);
  const dragController = reactHostPort.useMemo(
    () =>
      treeReorderDragController({
        onMove,
        resolveParentId,
      }),
    [onMove, resolveParentId],
  );
  return {
    dropIndicatorId: dropStateRef.current.id,
    dropPosition: dropStateRef.current.position,
    dragController,
  };
}

export const CATALOGUE_DRAG_MIME = "application/x-semio-catalogue-item";

export interface CatalogueItem {
  readonly id: string;
  readonly label: React.ReactNode;
  readonly icon?: React.ReactNode;
  readonly payload: Record<string, unknown>;
}

export interface CatalogueProps {
  readonly title: React.ReactNode;
  readonly items: readonly CatalogueItem[];
  readonly mime?: string;
  readonly className?: string;
  readonly dragController?: TreeDragAndDropController;
}

/** @emoji 🗂️ Draggable catalogue palette rendered as a tree section. */
export const Catalogue: React.FC<CatalogueProps> = ({ title, items, mime = CATALOGUE_DRAG_MIME, className, dragController }) => {
  const sections = reactHostPort.useMemo<TreeDataSection[]>(
    () => [
      {
        id: "catalogue",
        label: title,
        items: items.map((item) => ({
          id: item.id,
          label: item.label,
          icon: item.icon,
          draggable: true,
          dragData: { [mime]: JSON.stringify(item.payload) },
        })),
      },
    ],
    [items, mime, title],
  );
  return <Tree className={className} sections={sections} dragAndDropController={dragController ?? catalogueTreeDragController(mime)} />;
};

const activeCatalogueDragPayload = ephemeralBox<string | null>("framework.modules.ui.elements.Tree.component.tsx.activeCatalogueDragPayload", null);

/** @emoji 🖱️ Payload of the catalogue drag currently in flight — native HTML5 `dragover` can't read `dataTransfer` until drop, so drop targets (e.g. a canvas host previewing a fixture drop) read this instead. */
export function getActiveCatalogueDragPayload(): string | null {
  return activeCatalogueDragPayload.current;
}

/** @emoji 🖱️ {@link TreeDragAndDropController} for catalogue rows carrying encoded payloads. */
export function catalogueTreeDragController(mime: string = CATALOGUE_DRAG_MIME): TreeDragAndDropController {
  const pointerRef = { active: false };
  const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
    const payload = dragData?.[mime];
    return payload?.trim() ? payload : undefined;
  };
  return {
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: () => {
        pointerRef.active = true;
      },
      cancel: () => {
        pointerRef.active = false;
      },
    },
    onDragStart: ({ sourceItem }) => {
      activeCatalogueDragPayload.current = readEncoded(sourceItem.dragData) ?? null;
    },
    onDragEnd: () => {
      pointerRef.active = false;
      activeCatalogueDragPayload.current = null;
    },
    handleDrop: ({ data, target, targetKind, dropPosition }) => {
      const encoded = readEncoded(data);
      if (!encoded) return;
      const payload = JSON.parse(encoded) as Record<string, unknown>;
      if (targetKind === "item" && typeof (target as TreeDataItem).onClick === "function") {
        (target as TreeDataItem).onClick?.({} as React.MouseEvent, {
          path: [],
          selectedIds: [],
          sectionId: "catalogue",
        });
      }
      void payload;
      void dropPosition;
    },
  };
}

interface TreeSelectionComputationArgs {
  selectionMode: TreeSelectionMode;
  selectedIds: readonly string[];
  orderedIds: readonly string[];
  targetId: string;
  anchorId?: string;
  additiveKey: boolean;
  rangeKey: boolean;
}

interface TreeSelectionComputationResult {
  selectedIds: string[];
  anchorId?: string;
}

// #region 🕹️InteractionDelegation
/** 🕹️ Tree has no real granularity/domain concept of its own — every row is addressed under this one
 * fixed granularity id when adapting to `🕹️interaction`'s domain-keyed machine below. */
const TREE_INTERACTION_GRANULARITY = "item";
const TREE_INTERACTION_DOMAIN = "tree";
const TREE_INTERACTION_LABEL = { native: { en: "Tree", de: "Baum" }, reuse: { en: "Tree", de: "Baum" } } as const;
const TREE_INTERACTION_ITEM_LABEL = { native: { en: "Item", de: "Element" }, reuse: { en: "Item", de: "Element" } } as const;

/** 🕹️ Minimal `InteractionDefinition` wrapping `selectionMode` — only `granularities[0].id` and
 * `selection.modes[0]` are read by `nextSelection`/`validateState`; `hover`/`hierarchy`/`iconId` are
 * unused filler required by the type. */
const treeInteractionDefinition = (selectionMode: TreeSelectionMode): InteractionDefinition => ({
  id: TREE_INTERACTION_DOMAIN,
  label: TREE_INTERACTION_LABEL,
  granularities: [{ id: TREE_INTERACTION_GRANULARITY, label: TREE_INTERACTION_ITEM_LABEL, iconId: "circle" }],
  hierarchy: { kind: "flat" },
  hover: { enabled: true, transitive: false, channels: ["pointer"], broadcast: false },
  selection: { modes: [selectionMode], methods: ["pick"], merges: ["replace", "additive", "subtractive", "invertive", "range"], transitive: false, broadcast: false },
});
// #endregion 🕹️InteractionDelegation

/** 🕹️ Delegates to `🕹️interaction`'s `validateState` (wave 0) for the FIRST-id clamp Tree has always
 * applied to externally-supplied/persisted ids (`defaultSelectedIds`, controlled `selectedIds`/
 * `highlightedIds`) — distinct from {@link getTreeNextSelectionState}'s LAST-target clamp for a live
 * pick. Falsy-id filtering + dedup stay local sanitization ahead of the shared machine, which has no
 * notion of either. */
export const normalizeTreeSelectedIds = (selectedIds: readonly string[], selectionMode: TreeSelectionMode): string[] => {
  const sanitizedIds = Array.from(new Set(selectedIds.filter(Boolean)));
  const validated = validateState(
    [treeInteractionDefinition(selectionMode)],
    { domains: {} },
    {
      selection: { [TREE_INTERACTION_DOMAIN]: { granularity: TREE_INTERACTION_GRANULARITY, ids: sanitizedIds } },
      hover: {},
      activeMode: {},
      activeGranularity: {},
    },
  );
  return [...(validated.selection[TREE_INTERACTION_DOMAIN]?.ids ?? [])];
};

const getTreeItemDefaultOpen = (item: TreeDataItem): boolean => item.defaultOpen ?? item.collapsibleState === TreeItemCollapsibleState.Expanded;

/** 🕹️ Delegates to `🕹️interaction`'s `nextSelection` (wave 0) for a live pick's LAST-target clamp — see
 * {@link normalizeTreeSelectedIds}'s doc for the two clamps this wave deliberately keeps distinct.
 * `additiveKey`/`rangeKey` fold to one `MergeMode` the same way `interactionMergeFromModifiers` folds raw
 * pointer modifiers (shift wins over ctrl/meta) — this function's own boolean-pair signature is kept
 * unchanged since `📁️VirtualFileSystem`/tests already call it this shape. */
export const getTreeNextSelectionState = ({ selectionMode, selectedIds, orderedIds, targetId, anchorId, additiveKey, rangeKey }: TreeSelectionComputationArgs): TreeSelectionComputationResult => {
  const merge = rangeKey ? "range" : additiveKey ? "invertive" : "replace";
  const topology: DomainTopology = { ordered: orderedIds.map((id) => ({ id, granularity: TREE_INTERACTION_GRANULARITY })) };
  const current: DomainSelection = { granularity: TREE_INTERACTION_GRANULARITY, ids: [...selectedIds], anchorId };
  const result = nextSelection(treeInteractionDefinition(selectionMode).selection, current, topology, {
    targets: [{ granularity: TREE_INTERACTION_GRANULARITY, id: targetId }],
    merge,
    mode: selectionMode,
  });
  return { selectedIds: [...result.ids], anchorId: result.anchorId };
};

const collectTreeItemMap = (items: TreeDataItem[], map: Record<string, TreeDataItem> = {}): Record<string, TreeDataItem> => {
  items.forEach((item) => {
    map[item.id] = item;
    if (item.items?.length) {
      collectTreeItemMap(item.items, map);
    }
  });
  return map;
};

/**
 * TreeSectionProps holds the data fields for a TreeSectionProps record.
 **/
interface TreeSectionProps {
  label?: React.ReactNode;
  id?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  expandable?: boolean;
  loading?: boolean;
  waiting?: boolean;
  className?: string;
  actions?: TreeHeaderAction[];
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragEnd?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  isLastSection?: boolean;
  /** @emoji 🎯️ Passive drop-zone highlight while a compatible tree drag is in flight. */
  isDropReady?: boolean;
  /** @emoji 🫳️ When true, renders a trailing {@link DragHandle} for reorder (with handle-only initiation unless {@link dragInitiation} is `"surface"`). */
  isDragHandle?: boolean;
  /** @emoji 🫳️ `"handle"` restricts native drag start to the trailing grip; `"surface"` keeps the whole section header draggable. Defaults to `"handle"` when {@link isDragHandle} or {@link draggable} is set. */
  dragInitiation?: "handle" | "surface";
}

/**
 * SortableTreeItemProps holds the data fields for a SortableTreeItemProps record.
 **/
interface SortableTreeItemProps {
  id: string;
  label?: React.ReactNode;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  isLastItem?: boolean;
  actions?: TreeHeaderAction[];
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  layoutKind?: "default" | "property";
}

/**
 * TreeItemProps holds the data fields for a TreeItemProps record.
 **/
interface TreeItemProps {
  label?: React.ReactNode;
  id?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  sortable?: boolean;
  sortableId?: string;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  expandable?: boolean;
  loading?: boolean;
  waiting?: boolean;
  isLastItem?: boolean;
  actions?: TreeHeaderAction[];
  isDragging?: boolean;
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragEnd?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  /** Total number of alternative branches. When > 0, shows branch navigation. */
  branchCount?: number;
  /** Currently active branch index (0-based). */
  activeBranchIndex?: number;
  /** Callback when the user navigates to a different branch. */
  onBranchChange?: (index: number) => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onPointerDown?: React.PointerEventHandler<HTMLDivElement>;
  onPointerMove?: React.PointerEventHandler<HTMLDivElement>;
  onPointerUp?: React.PointerEventHandler<HTMLDivElement>;
  onPointerCancel?: React.PointerEventHandler<HTMLDivElement>;
  layoutKind?: "default" | "property";
  isHidden?: boolean;
  contextMenu?: ContextMenuItem[];
  /** @emoji 🎚️ Control rendered on the header row of expandable property groups (label left, control right). */
  headerControl?: React.ReactNode;
  /** @emoji 🫳️ `"handle"` restricts native drag start to the trailing grip (arms `draggable` only while the grip is pressed); `"surface"` keeps the whole row draggable. */
  dragInitiation?: "handle" | "surface";
  /** @emoji 🫳️ Explicit drag handles for sort vs palette transfer; overrides {@link dragInitiation} when set. */
  dragRoles?: readonly TreeDragRole[];
  /** @emoji 🫳️ Pointer-down handler for palette transfer drags — wired to the transfer handle only. */
  transferPointerDown?: React.PointerEventHandler<HTMLSpanElement>;
  /** @emoji 🎯️ Passive drop-zone highlight while a compatible tree drag is in flight. */
  isDropReady?: boolean;
}

/**
 * SortableTreeItemsProps holds the data fields for a SortableTreeItemsProps record.
 **/
interface SortableTreeItemsProps {
  items: { id: string; [key: string]: any }[];
  onReorder: (oldIndex: number, newIndex: number) => void;
  children: (item: any, index: number) => React.ReactNode;
}

/**
 * TreeRootProps holds the data fields for a TreeRootProps record.
 **/
const EMPTY_TREE_SECTIONS: TreeDataSection[] = [];

interface TreeRootProps {
  className?: string;
  showLines?: boolean;
  sections?: TreeDataSection[];
  selectionMode?: TreeSelectionMode;
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  onSelectionChange?: (selectedIds: string[], items: TreeDataItem[]) => void;
  highlightedIds?: readonly string[];
  dragAndDropController?: TreeDragAndDropController;
  emptyState?: React.ReactNode;
  indentMultiplier?: number;
  /** @emoji 🧭️ `"up"` makes every foldable group in this tree unfold above its own header (children in reverse order), mirroring the {@link Ribbon} `"up"` pattern — for trees hosted in a panel that grows upward. Defaults to `"down"`. */
  direction?: TreeDirection;
  /** @emoji 🌱️ Controlled section/group expansion (see {@link TreeStateProvider}) — lets a host persist which groups are open across remounts. Uncontrolled (per-mount) when omitted. */
  openStates?: Readonly<Record<string, boolean>>;
  onOpenStateChange?: (id: string, open: boolean) => void;
  /** @emoji ↕️ When true, every section header renders a drag handle and sibling sections can be reordered. Defaults to true when there are two or more sections. */
  sortableSections?: boolean;
  /** @emoji ↕️ Fires after a section-handle reorder with the new section-id order (host may persist; Tree also keeps an internal merge so program re-renders do not snap order back). */
  onSectionsReorder?: (orderedIds: readonly string[]) => void;
}

/** @emoji ↕️ Merges a remembered section-id order with the latest section list — keeps prior relative order for surviving ids, appends newly appeared sections in source order. */
export function mergeTreeSectionOrder(previousIds: readonly string[], sections: readonly TreeDataSection[]): TreeDataSection[] {
  const byId = new Map(sections.map((section) => [section.id, section]));
  const ordered: TreeDataSection[] = [];
  for (const id of previousIds) {
    const section = byId.get(id);
    if (!section) continue;
    ordered.push(section);
    byId.delete(id);
  }
  for (const section of sections) {
    if (byId.has(section.id)) ordered.push(section);
  }
  return ordered;
}

/** @emoji ✅️ Per-tree selection store; rows subscribe via {@link useSyncExternalStore} without invalidating {@link TreeDataRenderingContext}. */
interface TreeSelectionStore {
  subscribe: (listener: () => void) => () => void;
  getSelectedIds: () => readonly string[];
  isSelected: (itemId: string) => boolean;
  setSelectedIds: (selectedIds: readonly string[]) => void;
}

export function createTreeSelectionStore(): TreeSelectionStore {
  let selectedIds: readonly string[] = [];
  let selectedIdSet = new Set<string>();
  const listeners = new Set<() => void>();
  const notify = (): void => {
    for (const listener of listeners) {
      listener();
    }
  };
  return {
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    getSelectedIds() {
      return selectedIds;
    },
    isSelected(itemId) {
      return selectedIdSet.has(itemId);
    },
    setSelectedIds(nextIds) {
      if (nextIds.length === selectedIds.length && nextIds.every((id, index) => id === selectedIds[index])) {
        return;
      }
      selectedIds = nextIds;
      selectedIdSet = new Set(nextIds);
      notify();
    },
  };
}

const TreeSelectionContext = reactHostPort.createContext<TreeSelectionStore | null>(null);

function useTreeSelectionStore(): TreeSelectionStore {
  const value = reactHostPort.useContext(TreeSelectionContext);
  if (!value) {
    throw new Error("Tree selection hooks must render inside Tree");
  }
  return value;
}

function useTreeItemRowSelected(itemId: string, itemSelectedOverride: boolean | undefined): boolean {
  const selectionStore = useTreeSelectionStore();
  const subscribedSelected = reactHostPort.useSyncExternalStore(
    selectionStore.subscribe,
    () => selectionStore.isSelected(itemId),
    () => selectionStore.isSelected(itemId),
  );
  return itemSelectedOverride ?? subscribedSelected;
}

/** @emoji 🖱️ Per-tree highlight store; rows subscribe via {@link useSyncExternalStore} without invalidating {@link TreeDataRenderingContext}. */
interface TreeHighlightStore {
  subscribe: (listener: () => void) => () => void;
  getHighlightedIds: () => readonly string[];
  isHighlighted: (itemId: string) => boolean;
  setHighlightedIds: (highlightedIds: readonly string[]) => void;
}

export function createTreeHighlightStore(): TreeHighlightStore {
  let highlightedIds: readonly string[] = [];
  let highlightedIdSet = new Set<string>();
  const listeners = new Set<() => void>();
  const notify = (): void => {
    for (const listener of listeners) {
      listener();
    }
  };
  return {
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    getHighlightedIds() {
      return highlightedIds;
    },
    isHighlighted(itemId) {
      return highlightedIdSet.has(itemId);
    },
    setHighlightedIds(nextIds) {
      if (nextIds.length === highlightedIds.length && nextIds.every((id, index) => id === highlightedIds[index])) {
        return;
      }
      highlightedIds = nextIds;
      highlightedIdSet = new Set(nextIds);
      notify();
    },
  };
}

const TreeHighlightContext = reactHostPort.createContext<TreeHighlightStore | null>(null);

function useTreeHighlightStore(): TreeHighlightStore {
  const value = reactHostPort.useContext(TreeHighlightContext);
  if (!value) {
    throw new Error("Tree highlight hooks must render inside Tree");
  }
  return value;
}

function useTreeItemRowHighlighted(itemId: string, itemHighlightedOverride: boolean | undefined): boolean {
  const highlightStore = useTreeHighlightStore();
  const subscribedHighlighted = reactHostPort.useSyncExternalStore(
    highlightStore.subscribe,
    () => highlightStore.isHighlighted(itemId),
    () => highlightStore.isHighlighted(itemId),
  );
  return itemHighlightedOverride ?? subscribedHighlighted;
}

/** @emoji 🌲️ Stable context for hoisted Tree data rows (avoids remounting rows when Tree re-renders). */
interface TreeDataRenderingContextValue {
  readonly sectionItemsById: Record<string, TreeDataItem[]>;
  readonly itemItemsById: Record<string, TreeDataItem[]>;
  readonly loadingById: Record<string, boolean>;
  readonly dragAndDropController?: TreeDragAndDropController;
  readonly loadSectionItems: (section: TreeDataSection) => Promise<void>;
  readonly loadItemItems: (item: TreeDataItem) => Promise<void>;
  readonly handleSelectItem: (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => void;
  readonly handleDoubleClickItem: (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => void;
  readonly handleDragStart: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  readonly handleDragEnd: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  readonly handleDropOnItem: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  readonly handleDropOnSection: (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => void;
  readonly handleDragOver: (event: React.DragEvent<HTMLDivElement>) => void;
  readonly handleDragOverItem: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem) => void;
  readonly draggedIds: readonly string[];
  readonly dropPreview: { readonly targetId: string; readonly position: TreeDropPosition } | null;
  readonly buildPalettePointerProps: (item: TreeDataItem, section: TreeDataSection) => Pick<TreeItemProps, "onPointerDown">;
  /** @emoji ↕️ Section headers render reorder grips and accept sibling-section drops. */
  readonly sortableSections: boolean;
  readonly draggedSectionId: string | null;
  readonly handleSectionDragStart: (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => void;
  readonly handleSectionDragEnd: () => void;
  readonly handleSectionDragOver: (event: React.DragEvent<HTMLDivElement>) => void;
  readonly handleSectionDrop: (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => void;
}

const TreeDataRenderingContext = reactHostPort.createContext<TreeDataRenderingContextValue | null>(null);

function useTreeDataRendering(): TreeDataRenderingContextValue {
  const value = reactHostPort.useContext(TreeDataRenderingContext);
  if (!value) {
    throw new Error("Tree data row components must render inside Tree");
  }
  return value;
}

const treeDefaultDragMimeKind = "application/vnd.code.tree.item";

const getTreeSectionStateId = (sectionId: string): string => `tree-section-${sectionId}`;

const getTreeItemStateId = (itemId: string): string => `tree-item-${itemId}`;

const getTreeSectionLoadingId = (sectionId: string): string => `tree-section-loading-${sectionId}`;

const getTreeItemLoadingId = (itemId: string): string => `tree-item-loading-${itemId}`;

const treeSectionItemsSeed = (sections: readonly TreeDataSection[]): Record<string, TreeDataItem[]> => {
  const nextItems: Record<string, TreeDataItem[]> = {};
  for (const section of sections) {
    if (section.items) {
      nextItems[section.id] = section.items;
    }
  }
  return nextItems;
};

const treeSectionItemsMapsEqual = (left: Record<string, TreeDataItem[]>, right: Record<string, TreeDataItem[]>): boolean => {
  const leftKeys = Object.keys(left);
  const rightKeys = Object.keys(right);
  if (leftKeys.length !== rightKeys.length) {
    return false;
  }
  for (const key of leftKeys) {
    if (left[key] !== right[key]) {
      return false;
    }
  }
  return true;
};

const getTreeSectionItems = (section: TreeDataSection, sectionItemsById: Record<string, TreeDataItem[]>): TreeDataItem[] => sectionItemsById[section.id] ?? section.items ?? [];

const getTreeItemItems = (item: TreeDataItem, itemItemsById: Record<string, TreeDataItem[]>): TreeDataItem[] => itemItemsById[item.id] ?? item.items ?? [];

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

const treeSemanticHoverStaySelector = `${treeSemanticHoverRowSelector}, [data-slot="tree-section-content"], [data-slot="tree-item-content"], [data-slot="tree-property-content"], [data-slot="control-tree-folder-content"], [data-slot="window-measure-tree-content"]`;

/** @emoji 🎨️ Tree row shell: group + text tokens; gutter stays transparent so branch guides remain visible. */
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

/** @emoji 🎨️ Tree row content fill: backgrounds apply only on the label column, not the guide gutter. */
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

/** @emoji 🎨️ Tree row chrome: element gray at rest; hover highlight; selected primary + emphasized (no hover fill override). */
export function treeRowChromeClasses(isSelected: boolean, isHighlighted: boolean, isHidden = false, isLoading = false, isWaiting = false): string {
  return cn(treeRowChromeShellClasses(isSelected, isHighlighted, isHidden), treeRowChromeContentFillClasses(isSelected, isHighlighted, isLoading, isWaiting));
}

const TreeItemRowContextMenu: React.FC<{ readonly items?: readonly ContextMenuItem[]; readonly children: React.ReactNode }> = ({ items, children }) => {
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

/** @emoji 🖱️ Skip row leave when pointer moves to another tree row or nested branch (avoids stale leave clearing fast-hover highlight). */
export function shouldDispatchTreeRowPointerLeave(relatedTarget: EventTarget | null): boolean {
  if (!(relatedTarget instanceof Element)) {
    return true;
  }
  return relatedTarget.closest(treeSemanticHoverStaySelector) === null;
}

/**
 * Collapsible tree section header with optional action buttons.
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
              <span data-slot="tree-label" title={controlHint} className={cn(treeSectionLabelSlotClassName, isDropReady && "text-emphasized")} style={treeItemLabelStyle}>
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
              <span data-slot="tree-label" title={controlHint} className={cn(treeSectionLabelSlotClassName, isDropReady && "text-emphasized")} style={treeItemLabelStyle}>
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
        <TreeBranchContent slot="tree-section-content" ownerRowKind="section" ownerExpanded={open && hasChildren} topPaddingPx={treeSectionContentPaddingTopPx}>
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

/**
 * SortableTreeItem holds the data fields for a SortableTreeItem record.
 **/
const SortableTreeItem: React.FC<SortableTreeItemProps> = ({
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
            id={id}
            ref={setNodeRef}
            style={style}
            className={itemShellClasses}
            {...surfaceDragProps}
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
                  <span
                    data-slot="tree-label"
                    className={cn(treeItemLabelSlotClassName, "cursor-selectable")}
                    style={treeItemLabelStyle}
                    onClick={(e) => {
                      if (e.detail > 1) return;
                      e.preventDefault();
                      e.stopPropagation();
                      onClick?.(e);
                    }}
                  >
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
          id={id}
          ref={setNodeRef}
          style={style}
          className={itemShellClasses}
          {...surfaceDragProps}
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
                <span
                  data-slot="tree-label"
                  className={cn(treeItemLabelSlotClassName, "cursor-selectable")}
                  style={treeItemLabelStyle}
                  onClick={(e) => {
                    if (e.detail > 1) return;
                    e.preventDefault();
                    e.stopPropagation();
                    onClick?.(e);
                  }}
                >
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
              <span data-slot="tree-label" className={treeItemLabelSlotClassName} style={treeItemLabelStyle}>
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
            <span data-slot="tree-label" className={treeItemLabelSlotClassName} style={treeItemLabelStyle}>
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
 * Drag-and-drop sortable container for tree items.
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

/**
 * Single tree item row with icon, label, and interaction handlers.
 **/
export const TreeItem: React.FC<TreeItemProps> = ({
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
  isHidden = false,
  contextMenu,
  headerControl,
  isDragging = false,
  dragInitiation = "handle",
  dragRoles,
  transferPointerDown,
  isDropReady = false,
}) => {
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
    // 🌳️ Header-only hover `group` — nested children must be siblings, not descendants, otherwise
    // parent `group-hover` fill lights up every nested vortex/distribution row at once.
    const propertyHeader = (
      <div
        data-dim
        data-slot="tree-property-item"
        data-hover-scope
        data-tree-row-kind={isExpandable ? "group" : "property"}
        role="treeitem"
        id={id}
        data-state={open ? "open" : "closed"}
        className={cn("min-w-0 w-full", treeRowChromeShellClasses(isSelected, isHighlighted, isHidden), isDropReady && dropZoneReadyTextClass, className)}
        draggable={effectiveDraggable}
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
              <span
                data-slot="tree-label"
                title={controlHint}
                className={cn(treeItemLabelSlotClassName, "truncate font-medium transition-colors", isExpandable ? "cursor-foldable" : "cursor-selectable", "select-text")}
                style={treeItemLabelStyle}
                onClick={(event) => {
                  if (event.detail > 1) return;
                  event.preventDefault();
                  event.stopPropagation();
                  if (isExpandable) {
                    setOpen(!open);
                    return;
                  }
                  onClick?.(event);
                }}
              >
                {resolvedLabel as React.ReactNode}
              </span>
            </div>
            <div data-slot="tree-item-control" className={cn(treeItemControlClassName, "gap-double")}>
              {!isExpandable ? (
                <PropertyValueColumnContext.Provider value={true}>{children}</PropertyValueColumnContext.Provider>
              ) : headerControl ? (
                <PropertyValueColumnContext.Provider value={true}>{headerControl}</PropertyValueColumnContext.Provider>
              ) : null}
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
          <TreeBranchContent slot="tree-property-content" ownerRowKind="group" ownerExpanded={open && hasChildren} className="min-w-0" topPaddingPx={treeItemContentPaddingTopPx}>
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
              data-hover-scope
              data-tree-row-kind="group"
              data-tree-group
              data-draggable={draggable ? "true" : undefined}
              role="treeitem"
              id={id}
              className={itemShellClasses}
              draggable={effectiveDraggable}
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
                    <span
                      data-slot="tree-label"
                      className={cn(treeItemLabelSlotClassName, "cursor-selectable")}
                      style={treeItemLabelStyle}
                      onClick={(e) => {
                        if (e.detail > 1) return;
                        e.preventDefault();
                        e.stopPropagation();
                        onClick?.(e);
                      }}
                    >
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
              <TreeBranchContent slot="tree-item-content" ownerRowKind="group" ownerExpanded={open && hasChildren} topPaddingPx={treeItemContentPaddingTopPx}>
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
        data-tree-row-kind="leaf"
        data-draggable={draggable ? "true" : undefined}
        role="treeitem"
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
              <span data-slot="tree-label" className={cn(treeItemLabelSlotClassName, draggable && effectiveDragInitiation === "surface" ? "cursor-grab" : "cursor-selectable", treeLabelSelectClass)} style={treeItemLabelStyle}>
                {resolvedLabel as React.ReactNode}
              </span>
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

/**
 * Leaf form row combining TreeItem and TreeContent into [Indent][Label][Control].
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
  /** When set (including explicit `null`), overrides useLabel(id) for the row title. Use `null` for content-only rows. */
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
 * Informational text row spanning the full control column width.
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

const getTreeItemLabel = (item: TreeDataItem): React.ReactNode => {
  if (!item.description) {
    return <span className="min-w-0 truncate">{item.label}</span>;
  }

  return (
    <span className="min-w-0 truncate leading-none">
      <span className="min-w-0 truncate">{item.label}</span> <span className={treeItemSecondaryTextClassName}>{item.description}</span>
    </span>
  );
};

const getTreeDropData = (event: React.DragEvent<HTMLDivElement>): Record<string, string> => {
  return Array.from(event.dataTransfer.types).reduce<Record<string, string>>((result, kind) => {
    try {
      result[kind] = event.dataTransfer.getData(kind);
    } catch {
      result[kind] = "";
    }
    return result;
  }, {});
};

/**
 * Data interface for a node in a file tree.
 **/
export interface FileTreeNode {
  title: string;
  path: string;
  icon?: string;
  isFolder: boolean;
  children?: FileTreeNode[];
}

//#region 🎃️TreeHoverPath
// 🌳️Branch containers that hold child rows and render IndentationLines.
const treeBranchSlots = new Set(["tree-section-content", "tree-item-content", "tree-property-content", "control-tree-folder-content", "window-measure-tree-content"]);
// 🔷️Row-level elements that own an elbow connector.
const treeRowSlots = new Set(["tree-item-row", "tree-section-row", "tree-property-item", "tree-row", "control-tree-row", "window-measure-tree-row"]);
const treeHoverPathRowSelector =
  '[data-slot="tree-item-row"], [data-slot="tree-section-row"], [data-slot="tree-property-item"], [data-slot="tree-row"], [data-slot="control-tree-row"], [data-slot="tree-content"], [data-slot="window-measure-tree-row"]';
const treeHoverPathBranchSelector = '[data-slot="tree-section-content"], [data-slot="tree-item-content"], [data-slot="tree-property-content"], [data-slot="control-tree-folder-content"], [data-slot="window-measure-tree-content"]';
const treeHoverPathAttr = "data-tree-hover-path";
const treeSelectionPathAttr = "data-tree-selection-path";

const clearTreePath = (root: HTMLElement, pathAttr: string) => {
  root.querySelectorAll(`[${pathAttr}]`).forEach((el) => el.removeAttribute(pathAttr));
};

const clearTreeHoverPath = (root: HTMLElement) => {
  clearTreePath(root, treeHoverPathAttr);
};

/**
 * 📦️Derive the row element that owns a branch container.
 * Handles all DOM shapes: tree-item-row/control-tree-row/tree-property-item siblings (branch below for `down`, above for `up`),
 * tree-section-row beside collapsible-content.
 */
const rowForBranch = (branch: Element): Element | null => {
  const prev = branch.previousElementSibling;
  if (prev) {
    const prevSlot = prev.getAttribute("data-slot");
    if (prevSlot && treeRowSlots.has(prevSlot)) return prev;
  }
  const next = branch.nextElementSibling;
  if (next) {
    const nextSlot = next.getAttribute("data-slot");
    if (nextSlot && treeRowSlots.has(nextSlot)) return next;
  }
  const parent = branch.parentElement;
  const parentSlot = parent?.getAttribute("data-slot");
  if (parentSlot === "collapsible-content") {
    const previousRow = parent!.previousElementSibling;
    if (previousRow?.getAttribute("data-slot") === "tree-section-row") return previousRow;
    const nextRow = parent!.nextElementSibling;
    if (nextRow?.getAttribute("data-slot") === "tree-section-row") return nextRow;
  }
  return null;
};

/**
 * 🎛️Resolve the conceptual tree row from a pointer target.
 * First tries matching a known row slot via closest(). When no row wrapper
 * exists (pass-through TreeRow, raw controls), falls back to the nearest
 * branch container and returns its owner row.
 */
export const resolveHoverRow = (target: HTMLElement, root: HTMLElement): Element | null => {
  const direct = target.closest(treeHoverPathRowSelector);
  if (direct && root.contains(direct)) return direct;
  const branch = target.closest(treeHoverPathBranchSelector);
  if (branch && root.contains(branch)) return rowForBranch(branch);
  return null;
};

const markTerminalBranch = (row: Element, pathAttr: string) => {
  const slot = row.getAttribute("data-slot");
  if (slot === "tree-item-row" || slot === "control-tree-row" || slot === "window-measure-tree-row") {
    for (const sibling of [row.nextElementSibling, row.previousElementSibling]) {
      if (!sibling) continue;
      const siblingSlot = sibling.getAttribute("data-slot");
      if (siblingSlot && treeBranchSlots.has(siblingSlot)) {
        sibling.setAttribute(pathAttr, "branch");
        return;
      }
    }
  } else if (slot === "tree-section-row") {
    for (const sibling of [row.nextElementSibling, row.previousElementSibling]) {
      if (sibling?.getAttribute("data-slot") !== "collapsible-content") continue;
      for (const child of Array.from(sibling.children)) {
        if (child.getAttribute("data-slot") === "tree-section-content") {
          child.setAttribute(pathAttr, "branch");
          return;
        }
      }
    }
  } else if (slot === "tree-property-item") {
    for (const sibling of [row.nextElementSibling, row.previousElementSibling]) {
      if (sibling?.getAttribute("data-slot") === "tree-property-content") {
        sibling.setAttribute(pathAttr, "branch");
        return;
      }
    }
  }
};

const markTreeRowPath = (row: Element, root: HTMLElement, pathAttr: string) => {
  row.setAttribute(pathAttr, "row");
  markTerminalBranch(row, pathAttr);
  let el: Element | null = row.parentElement;
  while (el && el !== root) {
    const slot = el.getAttribute("data-slot");
    if (slot && treeBranchSlots.has(slot)) {
      el.setAttribute(pathAttr, "branch");
      const ownerRow = rowForBranch(el);
      if (ownerRow) {
        ownerRow.setAttribute(pathAttr, "row");
        markTerminalBranch(ownerRow, pathAttr);
      }
    }
    el = el.parentElement;
  }
};

export const markGhostTreeInteraction = (target: Element, region: Element): HTMLElement[] => {
  const row = resolveHoverRow(target as HTMLElement, region as HTMLElement);
  if (!row) return [];

  const marked = new Set<HTMLElement>();
  const mark = (element: Element | null, attribute: "data-active-interaction" | "data-active-ancestor") => {
    if (!(element instanceof HTMLElement)) return;
    element.setAttribute(attribute, "");
    marked.add(element);
  };
  const markRow = (element: Element, active: boolean) => {
    mark(element, active ? "data-active-interaction" : "data-active-ancestor");
    for (const gutter of Array.from(element.querySelectorAll('[data-slot="tree-gutter"]'))) {
      mark(gutter, "data-active-ancestor");
    }
  };

  markRow(row, true);
  let element: Element | null = row.parentElement;
  while (element && element !== region) {
    const slot = element.getAttribute("data-slot");
    if (slot && treeBranchSlots.has(slot)) {
      mark(element, "data-active-ancestor");
      for (const child of Array.from(element.children)) {
        if (child.getAttribute("data-slot") === "tree-guide") mark(child, "data-active-ancestor");
      }
      const ownerRow = rowForBranch(element);
      if (ownerRow) markRow(ownerRow, false);
    }
    element = element.parentElement;
  }
  return [...marked];
};

const applyTreeHoverPath = (row: Element, root: HTMLElement) => {
  clearTreeHoverPath(root);
  markTreeRowPath(row, root, treeHoverPathAttr);
};

export const syncTreeSelectionPath = (root: HTMLElement, selectedIds: readonly string[]) => {
  clearTreePath(root, treeSelectionPathAttr);
  for (const selectedId of selectedIds) {
    const row = root.ownerDocument.getElementById(selectedId);
    if (!row || !root.contains(row)) {
      continue;
    }
    const slot = row.getAttribute("data-slot");
    if (!slot || !treeRowSlots.has(slot)) {
      continue;
    }
    markTreeRowPath(row, root, treeSelectionPathAttr);
  }
};

/** @emoji 🖱️ Pointer handlers that mark ancestor branch guide lines on row hover. */
const useTreeHoverPathRootHandlers = () => {
  const treeRootRef = reactHostPort.useRef<HTMLDivElement>(null);
  const lastHoverRowRef = reactHostPort.useRef<Element | null>(null);
  const hoverPathFrameRef = reactHostPort.useRef<number | null>(null);
  const pendingHoverRowRef = reactHostPort.useRef<Element | null>(null);

  const flushTreeHoverPath = reactHostPort.useCallback(() => {
    hoverPathFrameRef.current = null;
    const root = treeRootRef.current;
    const row = pendingHoverRowRef.current;
    pendingHoverRowRef.current = null;
    if (!root) return;
    if (row) {
      applyTreeHoverPath(row, root);
      lastHoverRowRef.current = row;
      return;
    }
    clearTreeHoverPath(root);
    lastHoverRowRef.current = null;
  }, []);

  const scheduleTreeHoverPath = reactHostPort.useCallback(
    (row: Element | null) => {
      pendingHoverRowRef.current = row;
      if (hoverPathFrameRef.current !== null) {
        return;
      }
      hoverPathFrameRef.current = requestAnimationFrame(flushTreeHoverPath);
    },
    [flushTreeHoverPath],
  );

  const handleTreePointerOver = reactHostPort.useCallback(
    (e: React.PointerEvent<HTMLDivElement>) => {
      const root = treeRootRef.current;
      if (!root) return;
      const row = resolveHoverRow(e.target as HTMLElement, root);
      if (row === lastHoverRowRef.current) return;
      scheduleTreeHoverPath(row);
    },
    [scheduleTreeHoverPath],
  );

  const handleTreePointerLeave = reactHostPort.useCallback(() => {
    if (hoverPathFrameRef.current !== null) {
      cancelAnimationFrame(hoverPathFrameRef.current);
      hoverPathFrameRef.current = null;
    }
    pendingHoverRowRef.current = null;
    lastHoverRowRef.current = null;
    const root = treeRootRef.current;
    if (root) clearTreeHoverPath(root);
  }, []);

  const refreshTreeHoverPath = reactHostPort.useCallback(() => {
    const root = treeRootRef.current;
    const row = lastHoverRowRef.current;
    if (!root || !row) {
      return;
    }
    applyTreeHoverPath(row, root);
  }, []);

  return { treeRootRef, handleTreePointerOver, handleTreePointerLeave, refreshTreeHoverPath };
};

/** @emoji ✅️ Marks ancestor section rows when a descendant tree item is selected. */
const useTreeSelectionPathSync = (treeRootRef: React.RefObject<HTMLDivElement | null>, selectedIds: readonly string[]) => {
  reactHostPort.useLayoutEffect(() => {
    const root = treeRootRef.current;
    if (!root) {
      return;
    }
    syncTreeSelectionPath(root, selectedIds);
  }, [selectedIds, treeRootRef]);
};
//#endregion 🎃️TreeHoverPath

/** @emoji 🌿️ Hoisted data-tree item row (stable component type across Tree re-renders). */
const TreeDataItemView = reactHostPort.memo(function TreeDataItemView(props: { readonly item: TreeDataItem; readonly section: TreeDataSection; readonly path: readonly string[]; readonly isLastItem: boolean }): React.ReactElement {
  const { item, section, path, isLastItem } = props;
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
  const isLoading = (loadingById[getTreeItemLoadingId(item.id)] ?? false) || Boolean(item.loading);
  const isWaiting = Boolean(item.waiting);
  const hasDynamicChildren = Boolean(item.getItems);
  const hasExpandableChildren = childItems.length > 0 || hasDynamicChildren || Boolean(item.emptyState) || branchCount > 0;
  const isExpandable = item.collapsibleState === TreeItemCollapsibleState.None ? false : hasExpandableChildren;
  const hasControl = Boolean(item.control);
  const propertyLayout = hasControl;
  const hasNestedTreeItems = childItems.length > 0 || hasDynamicChildren || Boolean(item.emptyState) || branchCount > 0;
  const defaultOpen = hasControl ? !hasNestedTreeItems || getTreeItemDefaultOpen(item) : getTreeItemDefaultOpen(item);
  const treeOpenState = useTreeOpenState(getTreeItemStateId(item.id), defaultOpen);
  const propertyExpandable = hasControl ? hasNestedTreeItems : isExpandable;

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
      label={hasControl ? item.label : getTreeItemLabel(item)}
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
      isDropReady={draggedIds.length > 0 && !isDragging && Boolean(dragAndDropController?.handleDrop)}
      layoutKind={propertyLayout ? "property" : undefined}
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
    >
      {hasControl && !hasNestedTreeItems ? item.control : null}
      {childItems.map((childItem, index) => (
        <TreeDataItemView key={childItem.id} item={childItem} section={section} path={[...path, childItem.id]} isLastItem={index === childItems.length - 1} />
      ))}
      {!isLoading && childItems.length === 0 && item.emptyState && (
        <TreeItem>
          <TreeContent>{item.emptyState}</TreeContent>
        </TreeItem>
      )}
    </TreeItem>
  );
});

/** @emoji 🌿️ Hoisted data-tree section row (stable component type across Tree re-renders). */
const TreeDataSectionView = reactHostPort.memo(function TreeDataSectionView(props: { readonly section: TreeDataSection; readonly isLastSection: boolean }): React.ReactElement {
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
  const isLoading = (loadingById[getTreeSectionLoadingId(section.id)] ?? false) || Boolean(section.loading);
  const isWaiting = Boolean(section.waiting);
  const hasDynamicChildren = Boolean(section.getItems);
  const isExpandable = items.length > 0 || hasDynamicChildren || Boolean(section.emptyState);
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
    >
      {items.map((item, index) => (
        <TreeDataItemView key={item.id} item={item} section={section} path={[section.id, item.id]} isLastItem={index === items.length - 1} />
      ))}
      {!isLoading && items.length === 0 && section.emptyState && <HelperRow>{section.emptyState}</HelperRow>}
    </TreeSection>
  );
});

/**
 * Hierarchical tree view component with optional file tree rendering.
 **/
type TreeComponent = ((props: TreeRootProps) => React.ReactElement) & {
  Files: React.FC<TreeFilesProps>;
  Section: React.FC<TreeFilesProps>;
};

export const Tree = (({
  className = "",
  showLines = true,
  sections,
  selectionMode = "single",
  selectedIds: controlledSelectedIds,
  defaultSelectedIds = [],
  onSelectionChange,
  highlightedIds: controlledHighlightedIds = [],
  dragAndDropController,
  emptyState,
  indentMultiplier = 1,
  direction = "down",
  openStates,
  onOpenStateChange,
  sortableSections: sortableSectionsProp,
  onSectionsReorder,
  children,
}: TreeRootProps & { children?: React.ReactNode }) => {
  if (hasNonEmptyChildren(children)) {
    throw new Error("Tree only accepts section data through the sections prop.");
  }
  const panelGhost = usePanelGhost();
  const [sectionItemsById, setSectionItemsById] = reactHostPort.useState<Record<string, TreeDataItem[]>>(() =>
    (sections ?? EMPTY_TREE_SECTIONS).reduce<Record<string, TreeDataItem[]>>((result, section) => {
      if (section.items) {
        result[section.id] = section.items;
      }
      return result;
    }, {}),
  );
  const [itemItemsById, setItemItemsById] = reactHostPort.useState<Record<string, TreeDataItem[]>>({});
  const [loadingById, setLoadingById] = reactHostPort.useState<Record<string, boolean>>({});
  const [uncontrolledSelectedIds, setUncontrolledSelectedIds] = reactHostPort.useState<string[]>(() => normalizeTreeSelectedIds(defaultSelectedIds, selectionMode));
  const [draggedIds, setDraggedIds] = reactHostPort.useState<string[]>([]);
  const [dropPreview, setDropPreview] = reactHostPort.useState<{ targetId: string; position: TreeDropPosition } | null>(null);
  const [draggedSectionId, setDraggedSectionId] = reactHostPort.useState<string | null>(null);
  const resolvedSections = sections ?? EMPTY_TREE_SECTIONS;
  const sortableSections = sortableSectionsProp ?? resolvedSections.length > 1;
  const [sectionOrderIds, setSectionOrderIds] = reactHostPort.useState<readonly string[]>(() => resolvedSections.map((section) => section.id));
  reactHostPort.useEffect(() => {
    setSectionOrderIds((previous) => {
      const merged = mergeTreeSectionOrder(previous, resolvedSections).map((section) => section.id);
      if (merged.length === previous.length && merged.every((id, index) => id === previous[index])) return previous;
      return merged;
    });
  }, [resolvedSections]);
  const orderedByPreference = reactHostPort.useMemo(() => mergeTreeSectionOrder(sectionOrderIds, resolvedSections), [resolvedSections, sectionOrderIds]);
  const suppressPaletteClickRef = reactHostPort.useRef(false);
  const palettePointerGestureRef = reactHostPort.useRef<{ pending: boolean; dragging: boolean; encoded: string | null; startX: number; startY: number; target: EventTarget | null }>({
    pending: false,
    dragging: false,
    encoded: null,
    startX: 0,
    startY: 0,
    target: null,
  });
  const palettePointerWindowCleanupRef = reactHostPort.useRef<(() => void) | null>(null);
  const clearPalettePointerWindowListeners = reactHostPort.useCallback(() => {
    palettePointerWindowCleanupRef.current?.();
    palettePointerWindowCleanupRef.current = null;
  }, []);
  reactHostPort.useEffect(() => () => clearPalettePointerWindowListeners(), [clearPalettePointerWindowListeners]);
  const anchorIdRef = reactHostPort.useRef<string | undefined>(normalizeTreeSelectedIds(defaultSelectedIds, selectionMode)[0]);
  const resolvedSelectedIds = reactHostPort.useMemo(() => normalizeTreeSelectedIds(controlledSelectedIds ?? uncontrolledSelectedIds, selectionMode), [controlledSelectedIds, uncontrolledSelectedIds, selectionMode]);
  const [selectionStore] = reactHostPort.useState(createTreeSelectionStore);
  const selectionStoreRef = reactHostPort.useRef(selectionStore);
  selectionStoreRef.current = selectionStore;
  const [highlightStore] = reactHostPort.useState(createTreeHighlightStore);
  const resolvedHighlightedIds = reactHostPort.useMemo(() => normalizeTreeSelectedIds(controlledHighlightedIds, "multiple"), [controlledHighlightedIds]);

  reactHostPort.useEffect(() => {
    selectionStore.setSelectedIds(resolvedSelectedIds);
  }, [resolvedSelectedIds, selectionStore]);

  reactHostPort.useLayoutEffect(() => {
    highlightStore.setHighlightedIds(resolvedHighlightedIds);
  }, [highlightStore, resolvedHighlightedIds]);

  reactHostPort.useEffect(() => {
    const nextItems = treeSectionItemsSeed(resolvedSections);
    setSectionItemsById((previous) => (treeSectionItemsMapsEqual(previous, nextItems) ? previous : nextItems));
  }, [resolvedSections]);

  const itemMap = reactHostPort.useMemo(() => {
    const map: Record<string, TreeDataItem> = {};
    resolvedSections.forEach((section) => {
      collectTreeItemMap(getTreeSectionItems(section, sectionItemsById), map);
    });
    Object.values(itemItemsById).forEach((items) => {
      collectTreeItemMap(items, map);
    });
    return map;
  }, [itemItemsById, resolvedSections, sectionItemsById]);

  const updateSelection = reactHostPort.useCallback(
    (nextSelectedIds: string[]) => {
      const normalizedIds = normalizeTreeSelectedIds(nextSelectedIds, selectionMode);
      if (controlledSelectedIds === undefined) {
        setUncontrolledSelectedIds(normalizedIds);
      }
      onSelectionChange?.(normalizedIds, normalizedIds.map((id) => itemMap[id]).filter(Boolean));
    },
    [controlledSelectedIds, itemMap, onSelectionChange, selectionMode],
  );

  const loadSectionItems = reactHostPort.useCallback(
    async (section: TreeDataSection) => {
      if (!section.getItems || sectionItemsById[section.id] !== undefined || loadingById[getTreeSectionLoadingId(section.id)]) {
        return;
      }
      setLoadingById((previousItems) => ({ ...previousItems, [getTreeSectionLoadingId(section.id)]: true }));
      try {
        const nextItems = await section.getItems();
        setSectionItemsById((previousItems) => ({ ...previousItems, [section.id]: nextItems }));
      } finally {
        setLoadingById((previousItems) => ({ ...previousItems, [getTreeSectionLoadingId(section.id)]: false }));
      }
    },
    [loadingById, sectionItemsById],
  );

  const loadItemItems = reactHostPort.useCallback(
    async (item: TreeDataItem) => {
      if (!item.getItems || itemItemsById[item.id] !== undefined || loadingById[getTreeItemLoadingId(item.id)]) {
        return;
      }
      setLoadingById((previousItems) => ({ ...previousItems, [getTreeItemLoadingId(item.id)]: true }));
      try {
        const nextItems = await item.getItems();
        setItemItemsById((previousItems) => ({ ...previousItems, [item.id]: nextItems }));
      } finally {
        setLoadingById((previousItems) => ({ ...previousItems, [getTreeItemLoadingId(item.id)]: false }));
      }
    },
    [itemItemsById, loadingById],
  );

  const handleDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    const types = [...event.dataTransfer.types];
    const acceptsCopy = types.some((kind) => kind !== treeDefaultDragMimeKind && kind.startsWith("application/"));
    event.dataTransfer.dropEffect = acceptsCopy ? "copy" : "move";
  }, []);

  const handleDragOverItem = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem) => {
      handleDragOver(event);
      if (isTreeReorderDragEvent(event)) {
        const position = resolveTreeDropPosition(event);
        setDropPreview((previous) => (previous?.targetId === item.id && previous.position === position ? previous : { targetId: item.id, position }));
        return;
      }
      setDropPreview(null);
    },
    [handleDragOver],
  );

  const handleSelectItem = reactHostPort.useCallback(
    (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
      if (suppressPaletteClickRef.current) {
        suppressPaletteClickRef.current = false;
        return;
      }
      const currentSelectedIds = selectionStoreRef.current.getSelectedIds();
      const orderedIds = getTreeItemOrderedIds(resolvedSections, sectionItemsById, itemItemsById);
      // 🎯️ Routed through the ONE shared modifier→merge policy (see `interactionMergeFromModifiers`'s
      // doc) rather than hand-checking `event.metaKey`/`event.ctrlKey`/`event.shiftKey` here directly.
      const merge = interactionMergeFromModifiers(event);
      const nextSelection = getTreeNextSelectionState({
        selectionMode,
        selectedIds: currentSelectedIds,
        orderedIds,
        targetId: item.id,
        anchorId: anchorIdRef.current,
        additiveKey: merge === "invertive",
        rangeKey: merge === "range",
      });
      anchorIdRef.current = nextSelection.anchorId;
      updateSelection(nextSelection.selectedIds);
      item.onClick?.(event, { path, selectedIds: nextSelection.selectedIds, sectionId: section.id });
    },
    [itemItemsById, resolvedSections, sectionItemsById, selectionMode, updateSelection],
  );

  const handleDoubleClickItem = reactHostPort.useCallback((event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
    item.onDoubleClick?.(event, { path, selectedIds: selectionStoreRef.current.getSelectedIds(), sectionId: section.id });
  }, []);

  const handleDragStart = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
      event.stopPropagation();
      const currentSelectedIds = selectionStoreRef.current.getSelectedIds();
      const nextDraggedIds = currentSelectedIds.includes(item.id) ? [...currentSelectedIds] : [item.id];
      const sourceItems = nextDraggedIds.map((id) => itemMap[id]).filter(Boolean);
      setDraggedIds(nextDraggedIds);
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData(treeDefaultDragMimeKind, JSON.stringify(nextDraggedIds));
      const customData = dragAndDropController?.getDragData?.({ items: sourceItems, sourceItem: item, section }) ?? item.dragData;
      Object.entries(customData ?? {}).forEach(([kind, value]) => {
        event.dataTransfer.setData(kind, value);
      });
      if (customData && Object.keys(customData).length > 0) {
        event.dataTransfer.effectAllowed = "copy";
        const labelText = typeof item.label === "string" ? item.label : typeof item.label === "number" ? String(item.label) : "Kind";
        const ghost = document.createElement("div");
        ghost.textContent = labelText;
        ghost.setAttribute("data-puzzle3d-fixture-drag-ghost", "true");
        ghost.setAttribute("data-level", "panel");
        ghost.className = cn("border-primary text-foreground pointer-events-none fixed left-offscreen top-0 z-tutorial rounded-md border px-2 py-1 text-xs shadow-md", surfaceClass);
        document.body.appendChild(ghost);
        event.dataTransfer.setDragImage(ghost, ghost.offsetWidth / 2, ghost.offsetHeight / 2);
        requestAnimationFrame(() => ghost.remove());
        panelGhost?.begin(event.currentTarget);
      }
      dragAndDropController?.onDragStart?.({ items: sourceItems, sourceItem: item, section });
    },
    [dragAndDropController, itemMap, panelGhost],
  );

  const handleDragEnd = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
      const sourceIds = draggedIds.length > 0 ? draggedIds : [item.id];
      const sourceItems = sourceIds.map((id) => itemMap[id]).filter(Boolean);
      dragAndDropController?.onDragEnd?.({ items: sourceItems, sourceItem: item, section });
      setDraggedIds([]);
      setDropPreview(null);
      panelGhost?.end();
    },
    [dragAndDropController, draggedIds, itemMap, panelGhost],
  );

  const handleDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, target: TreeDataItem | TreeDataSection, targetKind: "item" | "section", section: TreeDataSection) => {
      event.preventDefault();
      const sourceIds = draggedIds.length > 0 ? draggedIds : JSON.parse(event.dataTransfer.getData(treeDefaultDragMimeKind) || "[]");
      const dropPosition = targetKind === "item" ? resolveTreeDropPosition(event) : undefined;
      dragAndDropController?.handleDrop?.({
        target,
        targetKind,
        data: getTreeDropData(event),
        sourceItems: sourceIds.map((id: string) => itemMap[id]).filter(Boolean),
        section,
        dropPosition,
      });
      setDraggedIds([]);
      setDropPreview(null);
    },
    [dragAndDropController, draggedIds, itemMap],
  );

  const resolveItemDragData = reactHostPort.useCallback(
    (treeItem: TreeDataItem, treeSection: TreeDataSection) => dragAndDropController?.getDragData?.({ items: [treeItem], sourceItem: treeItem, section: treeSection }) ?? treeItem.dragData,
    [dragAndDropController],
  );

  const buildPalettePointerProps = reactHostPort.useCallback(
    (item: TreeDataItem, section: TreeDataSection): Pick<TreeItemProps, "onPointerDown"> => {
      const palettePointer = dragAndDropController?.pointerPaletteDrag;
      if (!palettePointer) {
        return {};
      }
      const beginPalettePointerDrag = (): void => {
        const gesture = palettePointerGestureRef.current;
        if (!gesture.pending || !gesture.encoded) {
          return;
        }
        gesture.pending = false;
        gesture.dragging = true;
        suppressPaletteClickRef.current = true;
        palettePointer.begin(gesture.encoded);
        panelGhost?.begin(gesture.target);
        dragAndDropController?.onDragStart?.({ items: [item], sourceItem: item, section });
      };
      const finishPalettePointerGesture = (): void => {
        clearPalettePointerWindowListeners();
        if (palettePointerGestureRef.current.dragging) {
          suppressPaletteClickRef.current = true;
          panelGhost?.end();
        }
        palettePointerGestureRef.current = { pending: false, dragging: false, encoded: null, startX: 0, startY: 0, target: null };
      };
      return {
        onPointerDown: (event) => {
          if (event.button !== 0) {
            return;
          }
          const dragData = resolveItemDragData(item, section);
          if (!dragData) {
            return;
          }
          const encoded = palettePointer.readEncodedDragPayload(dragData);
          if (!encoded) {
            return;
          }
          clearPalettePointerWindowListeners();
          palettePointerGestureRef.current = { pending: true, dragging: false, encoded, startX: event.clientX, startY: event.clientY, target: event.currentTarget };
          event.preventDefault();
          event.stopPropagation();
          const onWindowPointerMove = (moveEvent: PointerEvent): void => {
            const gesture = palettePointerGestureRef.current;
            if (!gesture.pending && !gesture.dragging) {
              return;
            }
            const deltaX = moveEvent.clientX - gesture.startX;
            const deltaY = moveEvent.clientY - gesture.startY;
            if (gesture.pending && deltaX * deltaX + deltaY * deltaY < 36) {
              return;
            }
            beginPalettePointerDrag();
          };
          const onWindowPointerUp = (): void => {
            if (palettePointerGestureRef.current.pending) {
              finishPalettePointerGesture();
              return;
            }
            if (palettePointerGestureRef.current.dragging) {
              dragAndDropController?.onDragEnd?.({ items: [item], sourceItem: item, section });
            }
            finishPalettePointerGesture();
          };
          const onWindowPointerCancel = (): void => {
            if (palettePointerGestureRef.current.dragging) {
              palettePointer.cancel();
              dragAndDropController?.onDragEnd?.({ items: [item], sourceItem: item, section });
              panelGhost?.end();
            }
            finishPalettePointerGesture();
          };
          window.addEventListener("pointermove", onWindowPointerMove);
          window.addEventListener("pointerup", onWindowPointerUp, true);
          window.addEventListener("pointercancel", onWindowPointerCancel, true);
          palettePointerWindowCleanupRef.current = () => {
            window.removeEventListener("pointermove", onWindowPointerMove);
            window.removeEventListener("pointerup", onWindowPointerUp, true);
            window.removeEventListener("pointercancel", onWindowPointerCancel, true);
          };
        },
      };
    },
    [clearPalettePointerWindowListeners, dragAndDropController, panelGhost, resolveItemDragData],
  );

  const handleDropOnItem = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
      handleDrop(event, item, "item", section);
    },
    [handleDrop],
  );

  const handleDropOnSection = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => {
      handleDrop(event, section, "section", section);
    },
    [handleDrop],
  );

  const handleSectionDragStart = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => {
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData(TREE_SECTION_REORDER_MIME, section.id);
    setDraggedSectionId(section.id);
  }, []);

  const handleSectionDragEnd = reactHostPort.useCallback(() => {
    setDraggedSectionId(null);
  }, []);

  const handleSectionDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
    if (event.dataTransfer.types.includes(TREE_SECTION_REORDER_MIME)) {
      event.preventDefault();
      event.dataTransfer.dropEffect = "move";
    }
  }, []);

  const handleSectionDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, targetSection: TreeDataSection) => {
      if (!event.dataTransfer.types.includes(TREE_SECTION_REORDER_MIME)) return;
      event.preventDefault();
      event.stopPropagation();
      const sourceId = event.dataTransfer.getData(TREE_SECTION_REORDER_MIME) || draggedSectionId;
      setDraggedSectionId(null);
      if (!sourceId || sourceId === targetSection.id) return;
      setSectionOrderIds((previous) => {
        const current = mergeTreeSectionOrder(previous, resolvedSections).map((section) => section.id);
        const fromIndex = current.indexOf(sourceId);
        const toIndex = current.indexOf(targetSection.id);
        if (fromIndex < 0 || toIndex < 0) return previous;
        const next = [...current];
        const [moved] = next.splice(fromIndex, 1);
        if (!moved) return previous;
        next.splice(toIndex, 0, moved);
        onSectionsReorder?.(next);
        return next;
      });
    },
    [draggedSectionId, onSectionsReorder, resolvedSections],
  );

  const treeDataRenderingValue = reactHostPort.useMemo<TreeDataRenderingContextValue>(
    () => ({
      sectionItemsById,
      itemItemsById,
      loadingById,
      dragAndDropController,
      loadSectionItems,
      loadItemItems,
      handleSelectItem,
      handleDoubleClickItem,
      handleDragStart,
      handleDragEnd,
      handleDropOnItem,
      handleDropOnSection,
      handleDragOver,
      handleDragOverItem,
      draggedIds,
      dropPreview,
      buildPalettePointerProps,
      sortableSections,
      draggedSectionId,
      handleSectionDragStart,
      handleSectionDragEnd,
      handleSectionDragOver,
      handleSectionDrop,
    }),
    [
      buildPalettePointerProps,
      dragAndDropController,
      draggedIds,
      draggedSectionId,
      dropPreview,
      handleDoubleClickItem,
      handleDragEnd,
      handleDragOver,
      handleDragOverItem,
      handleDragStart,
      handleDropOnItem,
      handleDropOnSection,
      handleSectionDragEnd,
      handleSectionDragOver,
      handleSectionDragStart,
      handleSectionDrop,
      handleSelectItem,
      itemItemsById,
      loadItemItems,
      loadSectionItems,
      loadingById,
      sectionItemsById,
      sortableSections,
    ],
  );

  const { treeRootRef, handleTreePointerOver, handleTreePointerLeave, refreshTreeHoverPath } = useTreeHoverPathRootHandlers();
  useTreeSelectionPathSync(treeRootRef, resolvedSelectedIds);

  const orderedSections = direction === "up" ? [...orderedByPreference].reverse() : orderedByPreference;

  return (
    <TreeStateProvider openStates={openStates} onOpenStateChange={onOpenStateChange}>
      <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines, isTree: true, indentMultiplier, direction }}>
        <TreeReorderDropPreview preview={dropPreview} />
        <div
          ref={treeRootRef}
          data-slot="tree"
          role="tree"
          aria-multiselectable={selectionMode === "multiple" ? true : undefined}
          dir="auto"
          className={`w-full min-w-0 overflow-hidden ${className}`}
          onPointerOver={handleTreePointerOver}
          onPointerLeave={handleTreePointerLeave}
        >
          <TreeHoverPathRefreshContext.Provider value={refreshTreeHoverPath}>
            <TreeSelectionContext.Provider value={selectionStore}>
              <TreeHighlightContext.Provider value={highlightStore}>
                <TreeDataRenderingContext.Provider value={treeDataRenderingValue}>
                  {orderedSections.map((section, sectionIndex) => (
                    <div key={section.id} data-slot="tree-section-wrapper" className="w-full min-w-0">
                      <TreeDataSectionView section={section} isLastSection={sectionIndex === orderedSections.length - 1} />
                    </div>
                  ))}
                </TreeDataRenderingContext.Provider>
              </TreeHighlightContext.Provider>
            </TreeSelectionContext.Provider>
          </TreeHoverPathRefreshContext.Provider>
          {resolvedSections.length === 0 && emptyState}
        </div>
      </TreeContext.Provider>
    </TreeStateProvider>
  );
}) as TreeComponent;

// #region 🎇️Basic Chat Panel
// Shared side-panel chat UI with local-only message storage.
// Consumers MUST provide a stable id and title per app tab.

interface BasicChatPanelProps extends ElementProps {
  title: string;
}

type BasicChatMessageRole = "assistant" | "user";

interface BasicChatMessage {
  id: string;
  role: BasicChatMessageRole;
  body: string;
}

export const BasicChatPanel: React.FC<BasicChatPanelProps> = ({ id, title }) => {
  const borderClass = borderNormalClass;
  const readyForLabel = useLabel("ui.chat.readyFor", { title });
  const localOnlyLabel = useLabel("ui.chat.localOnly");
  const instructionsLabel = useLabel("ui.chat.instructions", { title });
  const placeholderLabel = useLabel("ui.chat.placeholder", { title: title.toLowerCase() });
  const clearLabel = useLabel("ui.common.clear");
  const sendLabel = useLabel("ui.chat.send");
  const { t } = useUiTranslation();
  const savedLocally = reactHostPort.useCallback((preview: string) => resolveTranslationLabel(t("ui.chat.savedLocally", { preview })) ?? preview, [t]);
  const createBasicChatMessages = reactHostPort.useCallback(
    (chatId: string): BasicChatMessage[] => [
      { id: `${chatId}.assistant.0`, role: "assistant", body: readyForLabel },
      { id: `${chatId}.assistant.1`, role: "assistant", body: localOnlyLabel },
    ],
    [readyForLabel, localOnlyLabel],
  );
  const [messages, setMessages] = reactHostPort.useState<BasicChatMessage[]>(() => createBasicChatMessages(id));
  const [draft, setDraft] = reactHostPort.useState("");
  const nextMessageIndexRef = reactHostPort.useRef(2);
  const appendMessage = (role: BasicChatMessageRole, body: string) => {
    const nextMessageId = `${id}.${role}.${nextMessageIndexRef.current}`;
    nextMessageIndexRef.current += 1;
    setMessages((previousMessages) => [
      ...previousMessages,
      {
        id: nextMessageId,
        role,
        body,
      },
    ]);
  };
  const clearMessages = () => {
    nextMessageIndexRef.current = 2;
    setMessages(createBasicChatMessages(id));
    setDraft("");
  };
  const sendDraft = () => {
    const trimmedDraft = draft.trim();
    if (!trimmedDraft) {
      return;
    }
    const responsePreview = trimmedDraft.length > 72 ? `${trimmedDraft.slice(0, 69)}...` : trimmedDraft;
    setDraft("");
    appendMessage("user", trimmedDraft);
    appendMessage("assistant", savedLocally(responsePreview));
  };

  reactHostPort.useEffect(() => {
    nextMessageIndexRef.current = 2;
    setMessages(createBasicChatMessages(id));
    setDraft("");
  }, [id, createBasicChatMessages]);

  return (
    <div id={id} className="flex h-full min-h-0 flex-col gap-single">
      <HelperRow>{instructionsLabel}</HelperRow>
      <div id={childElementId(id, "feed")} className={cn("min-h-0 flex-1 overflow-y-auto rounded-sm border", borderClass)}>
        <div className="flex min-w-0 flex-col p-single">
          {messages.map((message) => (
            <TreeRow key={message.id}>
              <div id={childElementId(id, "message", message.id)} data-chat-role={message.role} className="flex min-w-0 flex-col gap-single">
                <span className="text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{message.role}</span>
                <p className="text-xs text-foreground whitespace-pre-wrap break-words">{message.body}</p>
              </div>
            </TreeRow>
          ))}
        </div>
      </div>
      <div className="flex shrink-0 flex-col gap-single">
        <Textarea
          id={childElementId(id, "draft")}
          value={draft}
          onChange={(event) => setDraft(event.target.value)}
          onKeyDown={(event) => {
            if (event.key !== "Enter" || event.shiftKey) {
              return;
            }
            event.preventDefault();
            sendDraft();
          }}
          rows={3}
          placeholder={placeholderLabel}
        />
        <div className="flex items-center justify-end gap-single">
          <Button type="button" id={childElementId(id, "clear")} text={clearLabel} icon="trash-2" onClick={clearMessages} />
          <Button type="button" id={childElementId(id, "send")} text={sendLabel} icon="arrow-right" onClick={sendDraft} disabled={!draft.trim()} />
        </div>
      </div>
    </div>
  );
};

// #endregion 🎇️Basic Chat Panel

interface FileTreeItemProps {
  node: FileTreeNode;
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
}

/**
 * FileTreeItem holds the data fields for a FileTreeItem record.
 **/
const FileTreeItem: React.FC<FileTreeItemProps> = ({ node, currentPath, onNavigate, as = "a" }) => {
  const { level, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const itemId = `file-${node.path}`;
  const { open, setOpen } = useTreeOpenState(itemId, true);

  const isActive = currentPath === node.path;
  const hasChildren = node.children && node.children.length > 0;
  const Icon = node.isFolder ? FolderIcon : DocumentIcon;

  const baseClasses = "flex items-center gap-single text-sm rounded-small cursor-selectable select-none transition-colors hover:bg-hover-interactive-fill";
  const stateClasses = isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:text-emphasized";
  const itemClasses = `${baseClasses} ${stateClasses}`;
  const handleClick = (e: React.MouseEvent) => {
    if (hasChildren) {
      e.preventDefault();
      setOpen(!open);
    }
    if (onNavigate) {
      onNavigate(node.path);
    }
  };

  const content = (
    <>
      {node.icon ? <span className="text-sm shrink-0">{node.icon}</span> : <Icon className="size-tiny shrink-0" />}
      <span className="text-sm">{node.title}</span>
    </>
  );

  const sharedProps = {
    className: itemClasses,
    style: { paddingInlineStart: `${detailPanelIndentPx(level, indentMultiplier) + 12}px` },
    onClick: handleClick,
  };

  const itemElement =
    as === "a" ? (
      <a href={`/${node.path}`} {...sharedProps}>
        {content}
      </a>
    ) : (
      <div {...sharedProps}>{content}</div>
    );

  if (hasChildren && node.isFolder) {
    return (
      <>
        {itemElement}
        {open && (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [], showLines: false, isTree, indentMultiplier }}>
            {node.children!.map((child, idx) => (
              <FileTreeItem key={idx} node={child} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </TreeContext.Provider>
        )}
      </>
    );
  }

  return itemElement;
};

/**
 * TreeFilesProps holds the data fields for a TreeFilesProps record.
 **/
interface TreeFilesProps {
  title?: UiLabel;
  nodes: FileTreeNode[];
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
  className?: string;
}

const TreeFiles: React.FC<TreeFilesProps> = ({ title, nodes, currentPath, onNavigate, as = "a", className = "" }) => {
  return (
    <TreeStateProvider>
      <div className={`not-prose my-medium p-medium rounded-lg border bg-card ${className}`}>
        {title && <h3 className="text-lg font-semibold mb-4">{title}</h3>}
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false, isTree: true, indentMultiplier: 1 }}>
          <div className="flex flex-col gap-single">
            {nodes.map((node, idx) => (
              <FileTreeItem key={idx} node={node} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </div>
        </TreeContext.Provider>
      </div>
    </TreeStateProvider>
  );
};

Tree.Files = TreeFiles;
Tree.Section = Tree.Files;

/** Alias for Tree.Files rendering a file tree from FileTreeNode data.
 **/
export const FileTree = TreeFiles;

// #region 🔬️ControlTree
// Leva-like nested folder+controls tree UI using existing design system components.
// Consumers MUST provide ControlDef[] and optional ControlTreeFolderSettings.

/**
 * Leaf control definition for the ControlTree.
 **/
export interface ControlDef {
  path: string;
  key?: string;
  order?: number;
  controlKind: string;
  value: any;
  onChange: (next: any) => void;
  meta?: Record<string, any>;
}

/**
 * Folder settings for the ControlTree.
 **/
export interface ControlTreeFolderSettings {
  path: string;
  order?: number;
  collapsed?: boolean;
  color?: string;
}

/**
 * Styling classname overrides for ControlTree visual slots.
 **/
export interface ControlTreeClassNames {
  panel?: string;
  folderRow?: string;
  folderTitle?: string;
  folderChevron?: string;
  folderChildren?: string;
  controlRow?: string;
  controlLabel?: string;
  controlBody?: string;
}

interface ControlTreeNode {
  kind: "folder" | "control";
  key: string;
  path: string;
  order: number;
  control?: ControlDef;
  children?: Record<string, ControlTreeNode>;
}

/**
 * Pure function converting flat ControlDef[] paths into a nested tree. Filtering matches leaf keys case-insensitively.
 **/
export function buildControlTree(controls: ControlDef[], filterText: string, folderSettings?: Record<string, ControlTreeFolderSettings>): Record<string, ControlTreeNode> {
  const root: Record<string, ControlTreeNode> = {};
  const lowerFilter = filterText.toLowerCase();
  for (const control of controls) {
    const leafKey = control.key ?? control.path.split("/").pop() ?? control.path;
    if (lowerFilter && !leafKey.toLowerCase().includes(lowerFilter)) continue;
    const segments = control.path.split("/");
    let current = root;
    let pathAccum = "";
    for (let i = 0; i < segments.length - 1; i++) {
      const seg = segments[i];
      pathAccum = pathAccum ? `${pathAccum}/${seg}` : seg;
      if (!current[seg]) {
        current[seg] = {
          kind: "folder",
          key: seg,
          path: pathAccum,
          order: folderSettings?.[pathAccum]?.order ?? 0,
          children: {},
        };
      }
      current = current[seg].children!;
    }
    const lastSeg = segments[segments.length - 1];
    current[lastSeg] = {
      kind: "control",
      key: leafKey,
      path: control.path,
      order: control.order ?? 0,
      control,
    };
  }
  return root;
}

function sortControlTreeNodes(nodes: Record<string, ControlTreeNode>): ControlTreeNode[] {
  return Object.values(nodes).sort((a, b) => {
    if (a.order !== b.order) return a.order - b.order;
    return a.key.localeCompare(b.key);
  });
}

/**
 * Default control renderer mapping controlKind to built-in components.
 **/
export const defaultControlRenderer = (def: ControlDef): React.ReactNode => {
  const controlId = def.path.replace(/\//g, ".");
  switch (def.controlKind) {
    case "number":
      return <Stepper id={controlId} value={def.value} onChange={def.onChange} min={def.meta?.min} max={def.meta?.max} step={def.meta?.step ?? 1} />;
    case "slider":
      return <Slider id={controlId} value={[def.value]} onValueChange={(v) => def.onChange(v[0])} min={def.meta?.min ?? 0} max={def.meta?.max ?? 100} />;
    case "boolean": {
      const labelText = typeof def.meta?.label === "string" ? def.meta.label : def.key;
      return <Toggle id={controlId} pressed={def.value} onPressedChange={def.onChange} icon={def.value ? <CheckIcon className="size-small" /> : <CloseIcon className="size-small" />} text={labelText} />;
    }
    case "string":
      return <Input id={controlId} lazy value={def.value} onLazyChange={def.onChange} />;
    case "color":
      return <Input id={controlId} type="color" value={def.value} onChange={(e) => def.onChange(e.target.value)} />;
    case "select":
      return (
        <Select id={controlId} value={def.value} onValueChange={def.onChange}>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {(def.meta?.options ?? []).map((opt: string | { value: string; label: string }) => {
              const v = typeof opt === "string" ? opt : opt.value;
              const l = typeof opt === "string" ? opt : opt.label;
              return (
                <SelectItem key={v} value={v}>
                  {l}
                </SelectItem>
              );
            })}
          </SelectContent>
        </Select>
      );
    case "text":
      return <Textarea id={controlId} lazy value={def.value} onLazyChange={def.onChange} />;
    default:
      return <Input id={controlId} lazy value={String(def.value)} onLazyChange={def.onChange} />;
  }
};

interface ControlTreeFolderProps {
  node: ControlTreeNode;
  folderSettings?: Record<string, ControlTreeFolderSettings>;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
  renderControl: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
}
const controlTreeValueColumnWidthPx = domSizePx("controlValueColumnUiSpacing");
interface ControlTreeRowProps {
  className?: string;
  left: React.ReactNode;
  right?: React.ReactNode;
}
const ControlTreeRow: React.FC<ControlTreeRowProps> = ({ className, left, right }) => (
  <div data-dim data-slot="control-tree-row" className={cn("grid min-w-0 w-full items-center gap-x-tiny min-h-small", className)} style={{ gridTemplateColumns: `minmax(0, 1fr) ${uiSpacingLen(STYLING_DOM.controlValueColumnUiSpacing)}` }}>
    <div data-slot="control-tree-row-left" className="relative min-w-0">
      {left}
    </div>
    <div data-slot="control-tree-row-right" className="min-w-0">
      {right}
    </div>
  </div>
);
interface ControlTreeFolderRowProps {
  node: ControlTreeNode;
  classNames?: ControlTreeClassNames;
  children?: React.ReactNode;
  defaultOpen: boolean;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
}
const ControlTreeFolderRow: React.FC<ControlTreeFolderRowProps> = ({ node, classNames, children, defaultOpen, onToggleFolder }) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier, direction = "down" } = reactHostPort.useContext(TreeContext);
  const { inline } = useFlow();
  const itemId = `control-tree-folder-${node.path}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const hasChildren = hasNonEmptyChildren(children);
  const FolderChevron = treeFoldChevronIcon(direction, inline, open);
  return (
    <>
      <ControlTreeRow
        className={cn("hover:bg-hover-interactive-fill select-none overflow-hidden group", classNames?.folderRow)}
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
                  className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                  onClick={(event) => {
                    event.preventDefault();
                    event.stopPropagation();
                    const nextOpen = !open;
                    setOpen(nextOpen);
                    onToggleFolder?.(node.path, !nextOpen);
                  }}
                >
                  <FolderChevron className={cn("size-small flex-shrink-0", classNames?.folderChevron)} />
                </button>
              ) : undefined
            }
            contentClassName="flex min-w-0 items-center gap-double"
          >
            <span data-slot="control-tree-folder-label" className={cn("text-xs font-semibold uppercase tracking-wide truncate text-element group-hover:text-emphasized transition-colors", classNames?.folderTitle)} style={treeItemLabelStyle}>
              {node.key}
            </span>
          </TreeAlignedRow>
        }
      />
      {open && hasChildren && (
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines, isTree, indentMultiplier, direction }}>
          <TreeBranchContent slot="control-tree-folder-content" className={classNames?.folderChildren}>
            {children}
          </TreeBranchContent>
        </TreeContext.Provider>
      )}
    </>
  );
};
interface ControlTreeLeafRowProps {
  node: ControlTreeNode;
  renderControl: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
}
const ControlTreeLeafRow: React.FC<ControlTreeLeafRowProps> = ({ node, renderControl, classNames }) => {
  const { level, isLastAtLevel, showLines } = reactHostPort.useContext(TreeContext);
  return (
    <ControlTreeRow
      className={cn("hover:bg-hover-interactive-fill select-none overflow-hidden group", classNames?.controlRow)}
      left={
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} slotOffsetPx={2} contentClassName="flex min-w-0 items-center gap-double">
          <span data-slot="control-tree-control-label" className={cn("text-xs font-normal truncate text-element group-hover:text-emphasized", classNames?.controlLabel)} style={treeItemLabelStyle}>
            {node.key}
          </span>
        </TreeAlignedRow>
      }
      right={
        <div data-slot="control-tree-control-body" className={cn("min-w-0", classNames?.controlBody)}>
          {renderControl(node.control!)}
        </div>
      }
    />
  );
};

const ControlTreeFolder: React.FC<ControlTreeFolderProps> = ({ node, folderSettings, onToggleFolder, renderControl, classNames }) => {
  const settings = folderSettings?.[node.path];
  const defaultOpen = !(settings?.collapsed ?? false);
  const sorted = sortControlTreeNodes(node.children ?? {});
  return (
    <ControlTreeFolderRow node={node} classNames={classNames} defaultOpen={defaultOpen} onToggleFolder={onToggleFolder}>
      {sorted.map((child) =>
        child.kind === "folder" ? (
          <ControlTreeFolder key={child.path} node={child} folderSettings={folderSettings} onToggleFolder={onToggleFolder} renderControl={renderControl} classNames={classNames} />
        ) : (
          <ControlTreeLeafRow key={child.path} node={child} renderControl={renderControl} classNames={classNames} />
        ),
      )}
    </ControlTreeFolderRow>
  );
};

/**
 * Props interface for the ControlTree component.
 **/
export interface ControlTreeProps {
  controls: ControlDef[];
  filterText?: string;
  folderSettings?: Record<string, ControlTreeFolderSettings>;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
  renderControl?: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
  className?: string;
}

/**
 * Leva-like nested folder+controls tree panel using existing design system components.
 **/
export const ControlTree: React.FC<ControlTreeProps> = ({ controls, filterText = "", folderSettings, onToggleFolder, renderControl = defaultControlRenderer, classNames, className }) => {
  const tree = reactHostPort.useMemo(() => buildControlTree(controls, filterText, folderSettings), [controls, filterText, folderSettings]);
  const sorted = reactHostPort.useMemo(() => sortControlTreeNodes(tree), [tree]);
  return (
    <div data-slot="control-tree" className={cn("w-full min-w-0", classNames?.panel, className)}>
      <TreeStateProvider>
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          {sorted.map((node) =>
            node.kind === "folder" ? (
              <ControlTreeFolder key={node.path} node={node} folderSettings={folderSettings} onToggleFolder={onToggleFolder} renderControl={renderControl} classNames={classNames} />
            ) : (
              <ControlTreeLeafRow key={node.path} node={node} renderControl={renderControl} classNames={classNames} />
            ),
          )}
        </TreeContext.Provider>
      </TreeStateProvider>
    </div>
  );
};

// #endregion 🔬️ControlTree

// #region 🌳️WindowMeasuresTree

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

/** @emoji 🌳️ Root tree shell for the window measures rail (guide lines + indentation). */
export const WindowMeasuresTree: React.FC<{ children: React.ReactNode; className?: string; direction?: FlowBlock }> = ({ children, className, direction = "down" }) => {
  const { treeRootRef, handleTreePointerOver, handleTreePointerLeave, refreshTreeHoverPath } = useTreeHoverPathRootHandlers();
  const orderedChildren = direction === "up" ? React.Children.toArray(children).reverse() : children;

  return (
    <FlowProvider block={direction}>
      <div
        ref={treeRootRef}
        data-slot="window-measures-tree"
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

/** @emoji 🌳️ Collapsible measure group row (same geometry as {@link ControlTree} folders). */
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

/** @emoji 🌳️ Measure control leaf aligned like a tree row (icon + label + value or full-width control). */
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

// #endregion 🌳️WindowMeasuresTree

// #region 🪟️WindowPaneChromeToggle

/** @emoji 🪟️ Props for {@link WindowPaneChromeToggle} — the panel-toggle twin for every window pane header. */
export interface WindowPaneChromeToggleProps {
  readonly id: string;
  readonly icon: IconName;
  readonly label: string;
  readonly onClick?: () => void;
  readonly disabled?: boolean;
  readonly className?: string;
  /** @emoji 🫳️ Pointer-drag props forwarded to the trailing {@link DragHandle} (omit when the pane is not re-anchorable yet). */
  readonly dragPointerProps?: Pick<React.HTMLAttributes<HTMLSpanElement>, "onPointerCancel" | "onPointerDown" | "onPointerMove" | "onPointerUp">;
  readonly showDragHandle?: boolean;
  readonly emphasized?: boolean;
}

/** @emoji 🪟️ Pane chrome toggle matching panel toggles: leading semantic icon, label, trailing {@link DragHandle} — never a fold-direction chevron. */
export const WindowPaneChromeToggle: React.FC<WindowPaneChromeToggleProps> = ({ id, icon, label, onClick, disabled, className, dragPointerProps, showDragHandle = true, emphasized = false }) => {
  const inlineText = useControlInlineText(id, label);
  const tooltipText = useControlTooltipText(id, label);
  const surfaceDrag = useUiDriverDragSurface();
  const canDrag = showDragHandle && Boolean(dragPointerProps);
  return (
    <button
      type="button"
      id={id}
      data-slot="window-pane-chrome-toggle"
      data-hover-scope
      title={tooltipText}
      disabled={disabled}
      onClick={onClick}
      className={cn(windowPaneChromeToggleClass, className)}
      {...(canDrag && surfaceDrag ? dragPointerProps : {})}
    >
      <span className={panelTabIconSlotClass}>{renderControlIcon(icon, "tiny")}</span>
      {inlineText !== undefined ? (
        <span data-slot="inline-label" className={panelTabLabelClass}>
          {inlineText}
        </span>
      ) : null}
      {showDragHandle && !surfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" subject={label} {...dragPointerProps} onClick={(event) => event.stopPropagation()} emphasized={emphasized} /> : null}
    </button>
  );
};

// #endregion 🪟️WindowPaneChromeToggle

// #endregion 📜️Tree

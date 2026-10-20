import { UIDialog, type UIDialogProps, type UIDialogFieldBinding } from "@semio-tech/ui/elements/UIDialog";
import { type Level, LEVELS, LevelProvider, useLevel, getLevelZClass, type SurfaceFill, surfaceFillClass, type SurfaceScopeValue, SurfaceScope, useSurface, type SurfaceProps, Surface, isSurfaceActiveBackgroundPointer, type SurfaceActiveBindProps, useSurfaceActive, setSurfaceActiveRoot } from "@semio-tech/ui/elements/Surface";
import { MenuItem, menuItemClassName, type MenuItemProps } from "@semio-tech/ui/elements/MenuItem";
import { DragHandle, HANDLE_HOVER_SCOPE_ATTR } from "@semio-tech/ui/elements/DragHandle";
import { reconcileActivePath, singleTreeLeaf, panelTabChildren, findPanelTabNode, findPanelTabPath, resolvePanelBranchBodyLeaf, progressPanelTabSelection, usePanelTabSelection, dockSkeletonOf, dockSkeletonsEqual, applyDockSkeleton, PanelTabBar, type PanelTabBarVariant, type PanelTreeUnit, type PanelTabLeaf, type PanelTabBranch, type PanelTabNode, type PanelTabSelectionResult, type PanelTabSelectionOptions, type PanelDock, type PanelTabBarProps } from "@semio-tech/ui/elements/PanelTabBar";
import { type FlowInline, type FlowBlock, type Flow, FlowProvider, useFlow } from "@semio-tech/ui/modules/flow-direction-context";
import { WINDOW_CHROME_BODY_PLANE_STYLE, WINDOW_CHROME_CHIP_ROW_STYLE, WINDOW_SILHOUETTE_BORDER_KINDS, WindowChrome, WindowChromeSilhouetteBorder, isWindowChromeIntroducedTarget, measureWindowSilhouetteMetrics, resolveWindowSilhouetteBorderKind, useWindowSilhouetteGeometry, windowCapFrameClass, windowChromeTitleChipClass, windowControlsCapClass, windowGapFrameClass, windowSilhouetteBorderPaint, type WindowChromeControlAction, type WindowChromeProps, type WindowSilhouetteBorderKind } from "@semio-tech/ui/elements/WindowChrome";
import { OverviewCard, OverviewCardAction, OverviewCardOpenChip, overviewCardChipClass, type OverviewCardActionProps, type OverviewCardBaseProps, type OverviewCardButtonProps, type OverviewCardProps, type OverviewCardSectionProps } from "@semio-tech/ui/elements/OverviewCard";
import { windowChromeScrollClearanceVar, windowContentDeadLineVar, windowContentDeadLineScrollClass, readWindowChromeScrollClearancePx, measureWindowChromeScrollClearancePx, isWindowContentDeadLineHost, readWindowContentDeadLinePx, readScrollerContentOverflows, useWindowContentDeadLineInset } from "@semio-tech/ui/elements/WindowContentDeadLine";
import type { ElementProps } from "@semio-tech/ui/modules/element-identity";
import type { UiState, UiStatus, UiElementState, ElementFillKind } from "@semio-tech/ui-styling";
import { ELEMENT_ID_PATTERN, isElementId, elementIdSegment, childElementId, assertElementId, elementIdSelector, useFirstDraggableElementAlias } from "@semio-tech/ui/elements/ElementId";
import { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut } from "@semio-tech/ui/elements/Command";
import { Footer, type FooterProps } from "@semio-tech/ui/elements/Footer";
import { Layout, type LayoutMobilePanelProps, type LayoutProps } from "@semio-tech/ui/elements/Layout";
import { Popover, PopoverAnchor, PopoverContent, PopoverTrigger, resolvePopoverPlacement, type PopoverAlign, type PopoverAnchorProps, type PopoverContentProps, type PopoverPreventableEvent, type PopoverProps, type PopoverSide, type PopoverTriggerProps } from "@semio-tech/ui/elements/Popover";
import { Label, useLabel, useLabelFormatter, useIdLabel, useControlAccessibleLabel, useControlInlineText, useControlTooltipText, resolveTranslationLabel, useUiTranslation } from "@semio-tech/ui/elements/Label";
import { TableAvatar, type TableAvatarProps } from "@semio-tech/ui/elements/TableAvatar";
import { PresenceBar, presenceColor, presenceCssVar, PRESENCE_BAR_DEFAULT_MAX, type PresenceAppearance, type PresenceBarProps, type PresenceHsl, type PresencePeer, type PresenceRole } from "@semio-tech/ui/elements/PresenceBar";
import { skeletonPulseClass, SkeletonBlock, elementSkeleton, WindowBodySkeleton, PanelTreeSkeleton, CanvasSkeleton, type ElementSkeletonKind } from "@semio-tech/ui/elements/Skeletons";
import { Field, type FieldProps } from "@semio-tech/ui/elements/Field";

export { UIDialog, type UIDialogProps, type UIDialogFieldBinding };
export {
  type Level,
  LEVELS,
  LevelProvider,
  useLevel,
  getLevelZClass,
  type SurfaceFill,
  surfaceFillClass,
  type SurfaceScopeValue,
  SurfaceScope,
  useSurface,
  type SurfaceProps,
  Surface,
  isSurfaceActiveBackgroundPointer,
  type SurfaceActiveBindProps,
  useSurfaceActive,
  setSurfaceActiveRoot,
};
export { MenuItem, menuItemClassName, type MenuItemProps };
export { DragHandle, HANDLE_HOVER_SCOPE_ATTR };
export {
  reconcileActivePath,
  singleTreeLeaf,
  panelTabChildren,
  findPanelTabNode,
  findPanelTabPath,
  resolvePanelBranchBodyLeaf,
  progressPanelTabSelection,
  usePanelTabSelection,
  dockSkeletonOf,
  dockSkeletonsEqual,
  applyDockSkeleton,
  PanelTabBar,
  type PanelTabBarVariant,
  type PanelTreeUnit,
  type PanelTabLeaf,
  type PanelTabBranch,
  type PanelTabNode,
  type PanelTabSelectionResult,
  type PanelTabSelectionOptions,
  type PanelDock,
  type PanelTabBarProps,
};
export { type FlowInline, type FlowBlock, type Flow, FlowProvider, useFlow };

/** 🪟️ Default mode-dock tab label — element gray; emphasize on hover/active only. */

export { ResponsiveLabel, shellChromeTitleClassName } from "../🪟️chrome/🏷️responsive-label/🟦️.tsx";
export { WINDOW_CHROME_BODY_PLANE_STYLE, WINDOW_CHROME_CHIP_ROW_STYLE, WINDOW_SILHOUETTE_BORDER_KINDS, WindowChrome, WindowChromeSilhouetteBorder, isWindowChromeIntroducedTarget, measureWindowSilhouetteMetrics, resolveWindowSilhouetteBorderKind, useWindowSilhouetteGeometry, windowCapFrameClass, windowChromeTitleChipClass, windowControlsCapClass, windowGapFrameClass, windowSilhouetteBorderPaint, type WindowChromeControlAction, type WindowChromeProps, type WindowSilhouetteBorderKind };
export { OverviewCard, OverviewCardAction, OverviewCardOpenChip, overviewCardChipClass, type OverviewCardActionProps, type OverviewCardBaseProps, type OverviewCardButtonProps, type OverviewCardProps, type OverviewCardSectionProps };

export { LayeredOverview, type LayeredDirection } from "@semio-tech/ui/elements/LayeredOverview";
export { capturePosterFromCanvases } from "@semio-tech/ui/elements/LayeredOverview/pane-helpers";
export { type LayeredCardState, type LayeredChromeState, type LayeredInsets, type LayeredLabels, type LayeredMode, type LayeredOverviewProps, type LayeredPane, type LayeredPaneState } from "@semio-tech/ui/elements/LayeredOverview/types";
export {
  LAYERED_DEFAULT_LIFECYCLE,
  LAYERED_FOLLOW_EPSILON,
  LAYERED_FOLLOW_LERP,
  LAYERED_GLIDE_MS,
  cellOffset,
  centeredLastRowCells,
  centeredRowSpan,
  clampOffset,
  easeInOutCubic,
  followStep,
  glideOffset,
  inWindow,
  LAYERED_FOLLOW_FRAME_MS,
  followFactor,
  lerpOffset,
  nearSquareGrid,
  nextWarmBoot,
  occupiedColumns,
  paneAxisBounds,
  panesOverBudget,
  panesToRelease,
  pointerOffset,
  resolveLifecycle,
  scheduleIdle,
  stripGrid,
  stripTransform,
  veilClip,
  veilClipPath,
  veilPolygon,
  warmDelay,
  windowAround,
  type LayeredCell,
  type LayeredGrid,
  type LayeredIdleScheduler,
  type LayeredLifecycle,
  type LayeredLivePane,
  type LayeredOffset,
  type LayeredSpan,
  type LayeredVeil,
  type LayeredWarmStep,
  type LayeredWindow,
  type PaneAxisBounds,
} from "@semio-tech/ui/modules/layered-overview-geometry";
export {
  windowChromeScrollClearanceVar,
  windowContentDeadLineVar,
  windowContentDeadLineScrollClass,
  readWindowChromeScrollClearancePx,
  measureWindowChromeScrollClearancePx,
  isWindowContentDeadLineHost,
  readWindowContentDeadLinePx,
  readScrollerContentOverflows,
  useWindowContentDeadLineInset,
};
export type { ElementProps };

/** 🧭️ The shared, compile-time-enforced state model every rendered UI element carries — explicit
 * re-export from `@semio-tech/ui-styling` (this package must not leak types from outside the codebase).
 * `state`/`status`/`hover`/`selected` mirror the Rust `UiState`/`UiStatus`/`UiPresence` model in `ui_wgpu`
 * (see `framework/ui/wgpu/rs/lib.rs`'s 🔖️Presence region) byte-for-byte. */
export type { UiState, UiStatus, UiElementState, ElementFillKind };
export { ELEMENT_ID_PATTERN, isElementId, elementIdSegment, childElementId, assertElementId, elementIdSelector, useFirstDraggableElementAlias };
export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };
export { Footer, type FooterProps };
export { Layout, type LayoutMobilePanelProps, type LayoutProps };
export {
  Popover,
  PopoverAnchor,
  PopoverContent,
  PopoverTrigger,
  resolvePopoverPlacement,
  type PopoverAlign,
  type PopoverAnchorProps,
  type PopoverContentProps,
  type PopoverPreventableEvent,
  type PopoverProps,
  type PopoverSide,
  type PopoverTriggerProps,
};
export { Label, useLabel, useLabelFormatter, useIdLabel, useControlAccessibleLabel, useControlInlineText, useControlTooltipText, resolveTranslationLabel, useUiTranslation };
export type { ControlTooltipTextOptions } from "@semio-tech/ui/elements/Label";
export { TableAvatar, type TableAvatarProps };
export { PresenceBar, presenceColor, presenceCssVar, PRESENCE_BAR_DEFAULT_MAX, type PresenceAppearance, type PresenceBarProps, type PresenceHsl, type PresencePeer, type PresenceRole };
export { presencePaint } from "@semio-tech/ui/modules/presence-presentation";
export { skeletonPulseClass, SkeletonBlock, elementSkeleton, WindowBodySkeleton, PanelTreeSkeleton, CanvasSkeleton, type ElementSkeletonKind };
export { Field, type FieldProps };

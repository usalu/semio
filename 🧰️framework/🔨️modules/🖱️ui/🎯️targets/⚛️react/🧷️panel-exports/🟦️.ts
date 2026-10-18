import { UIDialog, type UIDialogProps, type UIDialogFieldBinding } from "../../../🧱️elements/📨️UIDialog/🟦️.tsx";
import { type Level, LEVELS, LevelProvider, useLevel, getLevelZClass, type SurfaceFill, surfaceFillClass, type SurfaceScopeValue, SurfaceScope, useSurface, type SurfaceProps, Surface, isSurfaceActiveBackgroundPointer, type SurfaceActiveBindProps, useSurfaceActive, setSurfaceActiveRoot } from "../../../🧱️elements/🌈️Surface/🟦️.tsx";
import { MenuItem, menuItemClassName, type MenuItemProps } from "../../../🧱️elements/📋️MenuItem/🟦️.tsx";
import { DragHandle, HANDLE_HOVER_SCOPE_ATTR } from "../../../🧱️elements/🧱️DragHandle/🟦️.tsx";
import { reconcileActivePath, singleTreeLeaf, panelTabChildren, findPanelTabNode, findPanelTabPath, resolvePanelBranchBodyLeaf, progressPanelTabSelection, usePanelTabSelection, dockSkeletonOf, dockSkeletonsEqual, applyDockSkeleton, PanelTabBar, type PanelTabBarVariant, type PanelTreeUnit, type PanelTabLeaf, type PanelTabBranch, type PanelTabNode, type PanelTabSelectionResult, type PanelTabSelectionOptions, type PanelDock, type PanelTabBarProps } from "../../../🧱️elements/🧭️PanelTabBar/🟦️.tsx";
import { type FlowInline, type FlowBlock, type Flow, FlowProvider, useFlow } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { WINDOW_CHROME_BODY_PLANE_STYLE, WINDOW_CHROME_CHIP_ROW_STYLE, WINDOW_SILHOUETTE_BORDER_KINDS, WindowChrome, WindowChromeSilhouetteBorder, isWindowChromeIntroducedTarget, measureWindowSilhouetteMetrics, resolveWindowSilhouetteBorderKind, useWindowSilhouetteGeometry, windowCapFrameClass, windowChromeTitleChipClass, windowControlsCapClass, windowGapFrameClass, windowSilhouetteBorderPaint, type WindowChromeControlAction, type WindowChromeProps, type WindowSilhouetteBorderKind } from "../../../🧱️elements/🗂️WindowChrome/🟦️.tsx";
import { OverviewCard, OverviewCardAction, OverviewCardOpenChip, overviewCardChipClass, type OverviewCardActionProps, type OverviewCardBaseProps, type OverviewCardButtonProps, type OverviewCardProps, type OverviewCardSectionProps } from "../../../🧱️elements/🃏️OverviewCard/🟦️.tsx";
import { windowChromeScrollClearanceVar, windowContentDeadLineVar, windowContentDeadLineScrollClass, readWindowChromeScrollClearancePx, measureWindowChromeScrollClearancePx, isWindowContentDeadLineHost, readWindowContentDeadLinePx, readScrollerContentOverflows, useWindowContentDeadLineInset } from "../../../🧱️elements/🚧️WindowContentDeadLine/🟦️.tsx";
import type { ElementProps } from "../../../🔨️modules/🆔️element-identity/🟦️.ts";
import type { UiState, UiStatus, UiElementState, ElementFillKind } from "@semio-tech/ui-styling";
import { ELEMENT_ID_PATTERN, isElementId, elementIdSegment, childElementId, assertElementId, elementIdSelector, useFirstDraggableElementAlias } from "../../../🧱️elements/🆔️ElementId/🟦️.tsx";
import { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut } from "../../../🧱️elements/⌨️Command/🟦️.tsx";
import { Footer, type FooterProps } from "../../../🧱️elements/🔚️Footer/🟦️.tsx";
import { Layout, type LayoutMobilePanelProps, type LayoutProps } from "../../../🧱️elements/📐️Layout/🟦️.tsx";
import { Popover, PopoverAnchor, PopoverContent, PopoverTrigger, resolvePopoverPlacement, type PopoverAlign, type PopoverAnchorProps, type PopoverContentProps, type PopoverPreventableEvent, type PopoverProps, type PopoverSide, type PopoverTriggerProps } from "../../../🧱️elements/🗨️Popover/🟦️.tsx";
import { Label, useLabel, useLabelFormatter, useIdLabel, useControlAccessibleLabel, useControlInlineText, useControlTooltipText, resolveTranslationLabel, useUiTranslation } from "../../../🧱️elements/🏷️Label/🟦️.tsx";
import { TableAvatar, type TableAvatarProps } from "../../../🧱️elements/📻️TableAvatar/🟦️.tsx";
import { PresenceBar, presenceColor, presenceCssVar, PRESENCE_BAR_DEFAULT_MAX, type PresenceAppearance, type PresenceBarProps, type PresenceHsl, type PresencePeer, type PresenceRole } from "../../../🧱️elements/👥️PresenceBar/🟦️.tsx";
import { skeletonPulseClass, SkeletonBlock, elementSkeleton, WindowBodySkeleton, PanelTreeSkeleton, CanvasSkeleton, type ElementSkeletonKind } from "../../../🧱️elements/🦴️Skeletons/🟦️.tsx";
import { Field, type FieldProps } from "../../../🧱️elements/📝️Field/🟦️.tsx";

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

/** @emoji 🪟️ Default mode-dock tab label — element gray; emphasize on hover/active only. */

export { ResponsiveLabel, shellChromeTitleClassName } from "../🪟️chrome/🏷️responsive-label/🟦️.tsx";
export { WINDOW_CHROME_BODY_PLANE_STYLE, WINDOW_CHROME_CHIP_ROW_STYLE, WINDOW_SILHOUETTE_BORDER_KINDS, WindowChrome, WindowChromeSilhouetteBorder, isWindowChromeIntroducedTarget, measureWindowSilhouetteMetrics, resolveWindowSilhouetteBorderKind, useWindowSilhouetteGeometry, windowCapFrameClass, windowChromeTitleChipClass, windowControlsCapClass, windowGapFrameClass, windowSilhouetteBorderPaint, type WindowChromeControlAction, type WindowChromeProps, type WindowSilhouetteBorderKind };
export { OverviewCard, OverviewCardAction, OverviewCardOpenChip, overviewCardChipClass, type OverviewCardActionProps, type OverviewCardBaseProps, type OverviewCardButtonProps, type OverviewCardProps, type OverviewCardSectionProps };

export { LayeredOverview, type LayeredDirection } from "../../../🧱️elements/🥞️LayeredOverview/🟦️.tsx";
export { capturePosterFromCanvases } from "../../../🧱️elements/🥞️LayeredOverview/🧰️pane-helpers/🟦️.ts";
export { type LayeredCardState, type LayeredChromeState, type LayeredInsets, type LayeredLabels, type LayeredMode, type LayeredOverviewProps, type LayeredPane, type LayeredPaneState } from "../../../🧱️elements/🥞️LayeredOverview/📋️types/🟦️.ts";
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
} from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
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

/** @emoji 🧭️ The shared, compile-time-enforced state model every rendered UI element carries — explicit
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
export type { ControlTooltipTextOptions } from "../../../🧱️elements/🏷️Label/🟦️.tsx";
export { TableAvatar, type TableAvatarProps };
export { PresenceBar, presenceColor, presenceCssVar, PRESENCE_BAR_DEFAULT_MAX, type PresenceAppearance, type PresenceBarProps, type PresenceHsl, type PresencePeer, type PresenceRole };
export { presencePaint } from "../../../🔨️modules/👥️presence-presentation/🟦️.ts";
export { skeletonPulseClass, SkeletonBlock, elementSkeleton, WindowBodySkeleton, PanelTreeSkeleton, CanvasSkeleton, type ElementSkeletonKind };
export { Field, type FieldProps };

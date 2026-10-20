import { resolveColorHex, resolveSemanticColorHex, resolveSpatialAxisColors, themeColorVar, tokenVar, uiSpacingPx } from "@semio-tech/ui-styling";
import { type IconName, type MetabolismIconName, isMetabolismIconName, resolveMetabolismIconSvgFromTheme } from "@semio-tech/assets";
import { reactHostPort, type ReactHostPort, sceneHostPort, type SceneHostPort, flowHostPort, type FlowHostPort, HostReactFlow, HostReactFlowProvider, HostThreeCanvas } from "@semio-tech/ui/elements/Ports";
import { type SelectionModeStore, type ShellScope, createShellScope, ShellScopeContext, ShellScopeProvider, useShellFloatingSurfaceHost, useShellScope, useShellScopeOptional, shellScopeStorageOrBrowserFallback, registerShellActivityRoot, activeShellRoot, useShellKeydown, useIsActiveShellRoot, NULL_SHELL_ROOT_REF } from "@semio-tech/ui/elements/ShellScope";
import { ThreeEvent } from "@react-three/fiber";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { waitingBorderClass, waitingBorderActiveClass, loadingBorderClass, loadingBorderActiveClass, waitingBorderStateClass, loadingBorderStateClass, chromeStatusBorderClass, loadingBorderElementClass, waitingBorderElementClass } from "@semio-tech/ui/modules/status-border-presentation";
import { interactiveControlTransitionClass, interactiveOnClass, interactiveTabActiveClass, groupHoverExcludingHandleBgFillClass, hoverExcludingHandleTextEmphasizedClass, hoverExcludingHandleBgFillClass, interactiveHoverClass, interactiveHoverFillClass, interactiveActiveFillClass, interactiveActiveBorderClass } from "@semio-tech/ui/modules/interaction-presentation";
import { applyUiFormControlBrowserDefaults, formControlFocusBorderClass, uiFormControlBrowserDefaultProps } from "@semio-tech/ui/modules/form-control-presentation";
import { borderNormalBottomClass, borderNormalClass, borderElementClass } from "@semio-tech/ui/modules/border-presentation";
import { veilClass, glassClass, surfaceClass } from "@semio-tech/ui/modules/surface-presentation";
import { menuListItemClassName } from "@semio-tech/ui/modules/menu-item-presentation";
import { shellFloorPaints, shellFloorFillClass } from "@semio-tech/ui/modules/shell-floor-presentation";
import { chromeControlItemBaseClass, chromeControlItemClass, chromeControlTabItemClass, modeDockTabClassName, windowPaneChromeToggleClass, chromeControlGroupShellClass, chromeControlGroupClass, chromeControlItemOnClass, chromeControlTabActiveClass } from "@semio-tech/ui/modules/chrome-control-presentation";
import { setThemedIconCatalog, type ThemedIconCatalog, resolveIconSizePx, decodeIcon, encodeIcon, classifyIconSelectorMode, resolveCatalogIconSvg, resolveMetabolismIconSvg, renderControlIcon, iconSvgMarkup, iconMaskImage, Icon, createIconComponent, AddIcon, AlertCircleIcon, ArrowLeftIcon, AwardIcon, BookIcon, BoxIcon, CameraIcon, ChatIcon, CheckIcon, CheckIconAlt, ChevronDownIcon, ChevronDownIconAlt, ChevronLeftIcon, ChevronRightIcon, ChevronUpIcon, ChevronsUpDownIcon, CircleDotIcon, CloseIcon, CloseIconAlt, CodeIcon, ComponentIcon, ConnectionIcon, ConnectorIcon, CopyIcon, DetailsIcon, DiagramIcon, DisconnectIcon, DocumentIcon, ExternalLinkIcon, FileArchiveIcon, FileCodeIcon, FileImageIcon, FileJsonIcon, FileSpreadsheetIcon, FileTypeIcon, FileVideoIcon, FilterIcon, FindInViewIcon, FocusIcon, FolderIcon, FolderOpenIcon, GlobeIcon, GripVerticalIcon, HandIcon, HashIcon, HomeIcon, HudIcon, HudPanelIcon, InfoIcon, IntersectIcon, LandmarkIcon, LassoIcon, LayoutIcon, LayoutGridIcon, LeftSidePanelIcon, LightbulbIcon, LinkIcon, LoaderIcon, LocalKitIcon, Maximize2Icon, MessageCircle, MessageSquareIcon, Minimize2Icon, MonitorIcon, MoonIcon, MoreHorizontalIcon, MousePointerIcon, MoveIcon, NavigateBackIcon, NavigateForwardIcon, NavigateUpIcon, PanelRightIcon, PauseIcon, PieceIcon, PlayIcon, PlugIcon, PlusIcon, PortIcon, Puzzle2dIconFileImportIcon, Puzzle2dIconMathGlyphIcon, Puzzle2dIconRasterGlyphIcon, RecordIcon, RemoteKitIcon, RemoveIcon, ResetIcon, RightSidePanelIcon, SceneIcon, SearchIcon, SelectUtilityIcon, Settings2Icon, SettingsIcon, SkipBackIcon, SkipForwardIcon, SmartphoneIcon, SortAscendingIcon, SortDescendingIcon, StatsIcon, StopIcon, SunIcon, TabletIcon, TableViewIcon, TemporaryKitIcon, TriangleAlertIcon, TutorialIcon, TypeIcon, UserIcon, UsersIcon, UtilitiesIcon, UtilityBarIcon, WorkbenchIcon, Cursor, type IconSizeToken, type IconSelectorMode, type IconSource, type ControlIcon, type IconProps } from "@semio-tech/ui/elements/Icons";
import { createDOMEventBinding, ContextMenu, isContextMenuPointerTarget, contextMenuDigitFromKey, findCheckedContextMenuItem, ContextMenuController, TextSelectionContextMenuHost, type ContextMenuProps, type ContextMenuControllerProps } from "@semio-tech/ui/elements/ContextMenu";
import { getElementById, queryElement, type ContextMenuItem } from "@semio-tech/ui/elements/ContextMenu/item-render";
import { contextMenuOrdinals, contextMenuItemsAtLevel, contextMenuItemAtPath, findContextMenuCheckedPath, contextMenuNavigationFromKey, moveContextMenuActivePath, contextMenuPathForOrdinal, contextMenuOpenSubmenuPath, type ContextMenuNavDirection } from "@semio-tech/ui/elements/ContextMenu/navigation";
import { readDomTextSelection, isPointerEventOnDomTextSelection, isDomTextEditableTarget, buildTextSelectionContextMenuItems, copyDomTextSelection, cutDomTextSelection, pasteDomTextSelection, selectAllDomText, type TextSelectionContextMenuLabels, type TextSelectionContextMenuActions } from "@semio-tech/ui/elements/ContextMenu/text-selection";
import { type UiDriverLabels, type UiDriverLabelTier, type UiDriverDrag, type UiDriverReveal, type UiDriverTooltips, type UiDriverHotkeys, type UiDriver, DEFAULT_UI_DRIVER, COMPACT_UI_DRIVER, builtinUiDrivers, parseUiDriver, serializeUiDriver, resolveUiDriver, UI_CHROME_DRIVER_STORAGE_KEY, readStoredUiDriverId, writeStoredUiDriverId, UI_CUSTOM_DRIVERS_STORAGE_KEY, readStoredUiCustomDrivers, writeStoredUiCustomDrivers, readStoredUiDriver, setUiDriverProvider, activeUiDriver, useUiDriver, UiDriverProvider, useUiDriverDragSurface, useNativeDragArm, useUiDriverTooltips, setControlLabelIdResolver, resolveControlLabelId, panelKindFromPanelToggleControlId, isInternalChromeControlId, humanizeControlSegment, humanizeControlId, humanizeEngagementStepId } from "@semio-tech/ui/elements/UiDriver";
import { parseKeybindingChords, formatKeybindingShortcut, ariaKeyshortcutsText } from "@semio-tech/ui/modules/keybinding-text-interpretation";
import { formatControlTooltipText } from "@semio-tech/ui/modules/control-tooltip-presentation";
import { buildKeysByActionId, SHELL_KEYBINDINGS, composeControlKeybindings, UiKeybindingsProvider, useUiKeybindingsByControlId, resolveControlKeybindingRaw, useControlHotkey, useControlKeybinding, type ControlKeybindingAction, type ControlKeybindingDefinition, type ControlKeybindingCallback, type ControlKeybindingOptions, type ControlKeybindingDependencies } from "@semio-tech/ui/modules/control-keybinding-context";
import { ControlHotkeyBadge, type ControlHotkeyBadgeProps } from "@semio-tech/ui/modules/control-hotkey-presentation";
import { readStoredUiKeybindingOverrides, writeStoredUiKeybindingOverrides } from "@semio-tech/ui/modules/keybinding-persistence";

// 🚧️W3-interim: explicit re-export so 🧱️elements/<Element>/ leaf files (which only ever import from
// this barrel, never straight from "@semio-tech/ui-styling") can resolve these — a bare `import {...}
// from "@semio-tech/ui-styling"` above does not itself make a name part of this module's public surface.
export { resolveColorHex, resolveSemanticColorHex, resolveSpatialAxisColors, themeColorVar, tokenVar, uiSpacingPx };
// 🚧️W8-interim: explicit re-export of the raw React/react-dom runtime values and types that s plugins
// previously imported straight from those packages — plugins depend on this package already, so they no
// longer need "react"/"react-dom" in their own `dependencies`. three.js is re-exported by name in 📰️Three.js.
export { Fragment, createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
export type { CSSProperties, FC, KeyboardEvent, MouseEvent as ReactMouseEvent, ReactNode, RefObject } from "react";
export { createRoot } from "react-dom/client";
export type { Root } from "react-dom/client";
export type { Camera as ThreeCamera, NormalBufferAttributes, Ray, Scene as ThreeScene, Texture as ThreeTexture } from "three";
/** 🔺️ Geometry acceleration used by the public scene host's collision API. */
export { MeshBVH } from "three-mesh-bvh";
export type { HitPointInfo } from "three-mesh-bvh";
export type { IconName, MetabolismIconName };
export { reactHostPort, type ReactHostPort };
export {
  interactiveJobPort,
  type InteractiveJobDescriptor,
  type InteractiveJobLease,
  type InteractiveJobPage,
  type InteractiveJobPort,
  type InteractiveJobPortSnapshot,
  type InteractiveJobTerminal,
} from "@semio-tech/ui/elements/Ports";

// 🧱️core-extracted: SceneHostPort + sceneHostPort moved to 🧱️elements/🔌️Ports/🟦️.tsx (same
// module-top-level circular-import fix as reactHostPort above, triggered by Scene's top-level
// `sceneHostPort.drei.Line` read) — imported below, not redefined.
export { sceneHostPort, type SceneHostPort };

// #region 🔌️PortWiring
export { flowHostPort, type FlowHostPort, HostReactFlow, HostReactFlowProvider, HostThreeCanvas };
export { type SelectionModeStore, type ShellScope, createShellScope, ShellScopeContext, ShellScopeProvider, useShellFloatingSurfaceHost, useShellScope, useShellScopeOptional, shellScopeStorageOrBrowserFallback };
// #endregion 🐚️ShellScope

export { registerShellActivityRoot, activeShellRoot, useShellKeydown, useIsActiveShellRoot, NULL_SHELL_ROOT_REF };

export type { IconRenderCamera, IconRenderFit, IconRenderFormat, IconRenderShape, IconRenderLights, IconRenderMaterial, IconRenderPort, IconRenderRequest, IconRenderResult, ThemeAppearanceName, ThemePaletteGroup, UiTheme } from "@semio-tech/ui-styling";
export { patchUiThemeGeometry, activeUiTheme, applyUiThemeToRoot, builtinUiThemes, clearUiThemeFromRoot, parseUiTheme, resolveThemeAppearancePalettes, semioTheme, serializeUiTheme, setActiveUiTheme, subscribeActiveUiTheme } from "@semio-tech/ui-styling";
export type { ThreeEvent };
export { cn };
export { waitingBorderClass, waitingBorderActiveClass, loadingBorderClass, loadingBorderActiveClass, waitingBorderStateClass, loadingBorderStateClass, chromeStatusBorderClass, loadingBorderElementClass, waitingBorderElementClass };
export {
  interactiveControlTransitionClass,
  interactiveOnClass,
  interactiveTabActiveClass,
  groupHoverExcludingHandleBgFillClass,
  hoverExcludingHandleTextEmphasizedClass,
  hoverExcludingHandleBgFillClass,
  interactiveHoverClass,
  interactiveHoverFillClass,
  interactiveActiveFillClass,
  interactiveActiveBorderClass,
};
export { applyUiFormControlBrowserDefaults, formControlFocusBorderClass, uiFormControlBrowserDefaultProps };
export { borderNormalBottomClass, borderNormalClass, borderElementClass };
export { veilClass, glassClass, surfaceClass };
export { menuListItemClassName };
export { shellFloorPaints, shellFloorFillClass };
export { chromeControlItemBaseClass, chromeControlItemClass, chromeControlTabItemClass, modeDockTabClassName, windowPaneChromeToggleClass, chromeControlGroupShellClass, chromeControlGroupClass, chromeControlItemOnClass, chromeControlTabActiveClass };

export { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_PICK_MENU } from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
export { canvasHoverFocusFromTarget, canvasPickTargetKey, pickMostSpecificCanvasTarget, sortCanvasPickTargetsGeneralFirst } from "@semio-tech/ui/modules/canvas-pick-targets";
export { windowElementId, panelTabElementId, panelTabFirstDraggableElementId } from "@semio-tech/ui/modules/element-identity";
export type { CanvasHoverFocus, CanvasPickRequest, CanvasPickTarget } from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
export type { DockSkeleton, DockTabSkeleton } from "@semio-tech/ui/modules/dock-skeleton";
export type { IntroductionStepDefinition, TutorialCameraKeyframe, TutorialDefinition } from "@semio-tech/framework-onboarding/declaration";
setThemedIconCatalog({ has: isMetabolismIconName, svg: resolveMetabolismIconSvgFromTheme });
export {
  setThemedIconCatalog,
  type ThemedIconCatalog,
  resolveIconSizePx,
  decodeIcon,
  encodeIcon,
  classifyIconSelectorMode,
  resolveCatalogIconSvg,
  resolveMetabolismIconSvg,
  renderControlIcon,
  iconSvgMarkup,
  iconMaskImage,
  Icon,
  createIconComponent,
  AddIcon,
  AlertCircleIcon,
  ArrowLeftIcon,
  AwardIcon,
  BookIcon,
  BoxIcon,
  CameraIcon,
  ChatIcon,
  CheckIcon,
  CheckIconAlt,
  ChevronDownIcon,
  ChevronDownIconAlt,
  ChevronLeftIcon,
  ChevronRightIcon,
  ChevronUpIcon,
  ChevronsUpDownIcon,
  CircleDotIcon,
  CloseIcon,
  CloseIconAlt,
  CodeIcon,
  ComponentIcon,
  ConnectionIcon,
  ConnectorIcon,
  CopyIcon,
  DetailsIcon,
  DiagramIcon,
  DisconnectIcon,
  DocumentIcon,
  ExternalLinkIcon,
  FileArchiveIcon,
  FileCodeIcon,
  FileImageIcon,
  FileJsonIcon,
  FileSpreadsheetIcon,
  FileTypeIcon,
  FileVideoIcon,
  FilterIcon,
  FindInViewIcon,
  FocusIcon,
  FolderIcon,
  FolderOpenIcon,
  GlobeIcon,
  GripVerticalIcon,
  HandIcon,
  HashIcon,
  HomeIcon,
  HudIcon,
  HudPanelIcon,
  InfoIcon,
  IntersectIcon,
  LandmarkIcon,
  LassoIcon,
  LayoutIcon,
  LayoutGridIcon,
  LeftSidePanelIcon,
  LightbulbIcon,
  LinkIcon,
  LoaderIcon,
  LocalKitIcon,
  Maximize2Icon,
  MessageCircle,
  MessageSquareIcon,
  Minimize2Icon,
  MonitorIcon,
  MoonIcon,
  MoreHorizontalIcon,
  MousePointerIcon,
  MoveIcon,
  NavigateBackIcon,
  NavigateForwardIcon,
  NavigateUpIcon,
  PanelRightIcon,
  PauseIcon,
  PieceIcon,
  PlayIcon,
  PlugIcon,
  PlusIcon,
  PortIcon,
  Puzzle2dIconFileImportIcon,
  Puzzle2dIconMathGlyphIcon,
  Puzzle2dIconRasterGlyphIcon,
  RecordIcon,
  RemoteKitIcon,
  RemoveIcon,
  ResetIcon,
  RightSidePanelIcon,
  SceneIcon,
  SearchIcon,
  SelectUtilityIcon,
  Settings2Icon,
  SettingsIcon,
  SkipBackIcon,
  SkipForwardIcon,
  SmartphoneIcon,
  SortAscendingIcon,
  SortDescendingIcon,
  StatsIcon,
  StopIcon,
  SunIcon,
  TabletIcon,
  TableViewIcon,
  TemporaryKitIcon,
  TriangleAlertIcon,
  TutorialIcon,
  TypeIcon,
  UserIcon,
  UsersIcon,
  UtilitiesIcon,
  UtilityBarIcon,
  WorkbenchIcon,
  Cursor,
  type IconSizeToken,
  type IconSelectorMode,
  type IconSource,
  type ControlIcon,
  type IconProps,
};
export {
  createDOMEventBinding,
  getElementById,
  queryElement,
  ContextMenu,
  contextMenuOrdinals,
  contextMenuItemsAtLevel,
  contextMenuItemAtPath,
  findContextMenuCheckedPath,
  contextMenuNavigationFromKey,
  moveContextMenuActivePath,
  contextMenuPathForOrdinal,
  contextMenuOpenSubmenuPath,
  isContextMenuPointerTarget,
  contextMenuDigitFromKey,
  findCheckedContextMenuItem,
  ContextMenuController,
  readDomTextSelection,
  isPointerEventOnDomTextSelection,
  isDomTextEditableTarget,
  buildTextSelectionContextMenuItems,
  copyDomTextSelection,
  cutDomTextSelection,
  pasteDomTextSelection,
  selectAllDomText,
  TextSelectionContextMenuHost,
  type ContextMenuItem,
  type ContextMenuProps,
  type ContextMenuControllerProps,
  type ContextMenuNavDirection,
  type TextSelectionContextMenuLabels,
  type TextSelectionContextMenuActions,
};
export type { UiDriverLabels, UiDriverLabelTier, UiDriverDrag, UiDriverReveal, UiDriverTooltips, UiDriverHotkeys, UiDriver };
export {
  DEFAULT_UI_DRIVER,
  COMPACT_UI_DRIVER,
  builtinUiDrivers,
  parseUiDriver,
  serializeUiDriver,
  resolveUiDriver,
  UI_CHROME_DRIVER_STORAGE_KEY,
  readStoredUiDriverId,
  writeStoredUiDriverId,
  UI_CUSTOM_DRIVERS_STORAGE_KEY,
  readStoredUiCustomDrivers,
  writeStoredUiCustomDrivers,
  readStoredUiDriver,
  setUiDriverProvider,
  activeUiDriver,
  useUiDriver,
  UiDriverProvider,
  useUiDriverDragSurface,
  useNativeDragArm,
  useUiDriverTooltips,
  setControlLabelIdResolver,
  resolveControlLabelId,
  panelKindFromPanelToggleControlId,
  isInternalChromeControlId,
  humanizeControlSegment,
  humanizeControlId,
  humanizeEngagementStepId,
};
export {
  parseKeybindingChords,
  formatKeybindingShortcut,
  ariaKeyshortcutsText,
  formatControlTooltipText,
  buildKeysByActionId,
  SHELL_KEYBINDINGS,
  composeControlKeybindings,
  UiKeybindingsProvider,
  useUiKeybindingsByControlId,
  resolveControlKeybindingRaw,
  useControlHotkey,
  useControlKeybinding,
  ControlHotkeyBadge,
  readStoredUiKeybindingOverrides,
  writeStoredUiKeybindingOverrides,
};
export type { ControlKeybindingAction, ControlKeybindingDefinition, ControlKeybindingCallback, ControlKeybindingOptions, ControlKeybindingDependencies, ControlHotkeyBadgeProps };

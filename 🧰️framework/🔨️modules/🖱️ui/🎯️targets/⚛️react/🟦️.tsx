// #region 🧲️Header

// 💻️ framework/ui/js/react/index.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Shared export surface for elements ui primitives.

// #endregion 🧲️Header

// #region 🔌️Adapters


import "@xyflow/react/dist/style.css";
import * as React from "react";
import { createPortal } from "react-dom";
// 🕹️wave-0: imported directly from the module's own source (not via `@semio-tech/framework`) — the
// `🛂️manifest` module already re-exports a same-named, owned-schema-generated `MergeMode`/`SelectionMode`
// family through that barrel, so a second barrel export of the hand-written mirror would collide.

// #endregion 🔌️Adapters

// #region 🔌️Ports
// 🧱️core-extracted: ReactHostPort + reactHostPort moved to 🧱️elements/🔌️Ports/🟦️.tsx (a
// module-top-level circular-import fix, see that file's header comment) — imported below, not redefined.
import { sceneHostPort } from "@semio-tech/ui/elements/Ports";
// #endregion 🔌️PortWiring

// #region 🐚️ShellScope

// #endregion 🖼️ReferenceMedia

// #region 🔌️PortWiringAliases
export const HostSceneCanvas = sceneHostPort.fiber.canvas;
// #endregion 🔖️CanvasPickMenu

// #region 🔖️Icon

// #endregion 🖱️ContextMenu

// #region UiDriver

// #endregion UiDriver

// #region ⌨️UiKeybindings


// #endregion ⌨️UiKeybindings

// #region 🌈️SurfaceChrome

// #endregion 🎛️UiChromeCompact


// #region 🪁️I18n Resources

// Domain-neutral UI translation bundles (settings, tooltip, generic shell `ui.*` ids).
// Product-specific bundles (e.g. compose sketchpad) register via {@link registerUiTranslationBundles}.

// #region 🔑️Schema & Keys
// Type/key-derivation machinery: locale codes, label shapes, the deep dot-path key type, and compile-time key-coverage checks.


// #endregion UiLabel


// #endregion 🔑️Schema & Keys

// #region 🔌️I18n Port
// The port lives in `./🌐️i18n/🟦️.ts` (importable alone as `@semio-tech/ui-react/i18n`) and is re-exported here unchanged.

//#endregion 🎬️TutorialGhostPointer

// #region 🗨️Dialog

// #endregion 🗨️Dialog

// #region 🎈️Level Context


// #region 📋️MenuItem


// #region 🫳️DragAffordance

// #endregion 🫳️DragAffordance


// #endregion 📑️PanelTabBar


// #region 🧭️Flow Context


//#region 🪟️WindowChrome


//#endregion 🪟️WindowChrome

// #region 🃏️OverviewCard

//#endregion 🧭️ElementState

//#region 🆔️ElementId

//#endregion 🆔️ElementId

// #endregion 🐹️Element

// #region 🪆️Command

// #endregion 🪆️Command

// #region 🎛️CommandPanel
// The footer command palette used to be its own bespoke component; commands now render as category leaf
// tabs under one expandable Command branch of the real `Panel` at anchor="bottom-middle" (see
// `buildCommandCategoryTabs`/`FRAMEWORK_CATEGORY_COMMAND_ID` in framework/os/renderer/js/react/index.tsx) —
// no ui-react component needed here anymore.
// #endregion 🎛️CommandPanel

// #region 🎮️Footer

// #endregion 🎮️Footer

// #region 🪨️Layout

// #endregion 🪨️Layout

// #region 🌐️Popover

// #endregion 🌐️Popover

// #region 🌥️Base Components
// #region 🏷️Label


// #endregion 📣️Aside

// #region 📻️TableAvatar

// #endregion 📻️TableAvatar

// #region 👥️PresenceBar

//#endregion 🧭️UiElementRegistry

// #region 🦴️Skeletons


// #endregion 🖲️Section

// #region 🏷️Field

// #endregion 🏷️Field

// #endregion 🏷️Display Components

// #region 🛒️Input Components

// #region 🌩️ActionGroup

// #endregion 🌩️ActionGroup

// #region 🌩️ButtonGroup

// #endregion 🌩️ButtonGroup

// #region 🌩️Button

// #endregion 🌩️Button

// #region 🧾️Form

// #endregion 🧾️Form

// #region ☑️Checkbox

// #endregion ☑️Checkbox

// #region 🩺️Input

// #endregion 🩺️Input

// #region 🔎️Select

// #endregion 🔎️Select

// #region 🏩️Slider

// #endregion 🏩️Slider

// #region 🏬️Stepper

// #endregion 🏬️Stepper

// #region 🎏️Textarea

// #endregion 🎏️Textarea

// #region 🗡️Toggle

// #endregion 🗡️Toggle

// #region 🧩️ToggleGroup

// #endregion 🧩️ToggleGroup

// #region 🧫️Ring

// #endregion 🧫️Ring

// #endregion 🛒️Input Components

// #region 🗼️Aggregation Components

// #region 🖥️Collapsible

// #endregion 🖥️Collapsible

// #region 🧸️Dialog

// #endregion 🧸️Dialog

// #region 🪬️Resizable

// #endregion 🪬️Resizable

// #region 🎮️Scrollable


// #endregion 🖥️Fullscreen

// #region 🩺️Navbar

// #endregion 🩺️Navbar

// #region 🧪️NavbarExampleSelect


// #endregion 🪟️DesktopTitlebar

// #region 🏷️Tabs

// #endregion 🏷️Tabs

// #region 🖼️IconSelector

// #endregion 🖼️IconSelector

// #region 📜️Tree

// #endregion 📜️Tree

// #endregion 🗼️Aggregation Components

// #region 📷️Panel Components

// #region 🧭️Panel

// #endregion 🧭️Panel

// #region 🪟️Pane


/** 🪟️ Portals `pane` into the nearest {@link PaneHost} — lets a component deep inside a window's canvas
 * (e.g. a per-tool floating control) contribute a draggable pane without threading it through the window's props.
 * Renders nothing outside a `PaneHost`. */
export function usePaneSlot(pane: React.ReactElement): React.ReactPortal | null {
  const host = usePaneHostContext();
  return host?.container ? (createPortal(pane, host.container) as React.ReactPortal) : null;
}


// #endregion 🪟️Pane

// #endregion 📷️Panel Components

// #region 🩻️Ribbon Components


// #endregion 🔎️WindowSearch


// #endregion 🧭️Shell


// #region 🌊️Window

// #endregion 🧫️Diagram

// #region 📍️Scene

// #endregion 📍️Scene

// #region 🛎️Table

// #endregion 🛎️Table

// #region 🗄️HistoryTable

// #endregion 🗄️HistoryTable

// #region 📁️VirtualFileSystem

// #endregion 📁️VirtualFileSystem


import { usePaneHostContext } from "@semio-tech/ui/elements/WindowPane";
import "./📤️adapter-exports/🟦️.ts";
import "./🧱️window-chrome/🟦️.tsx";
import "./🖥️fullscreen/🟦️.tsx";
export * from "./📤️adapter-exports/🟦️.ts";
export * from "./🪞️chrome-exports/🟦️.ts";
export * from "./🧷️panel-exports/🟦️.ts";
export * from "./📚️component-exports/🟦️.ts";
export * from "./🗿️framework-exports/🟦️.ts";

export { IconShotFrame, buildIconCamera, clipIconSvgMarkupToEllipse, configureHostPorts, finalizeIconSvgMarkup, iconRenderCameraIsOrthographic, iconRenderCameraPose, iconRenderOrthoZoomForRadius, iconRenderPerspectiveDistanceForRadius, iconRenderPort, iconShotFrameClass, iconShotFrameStyle, isTransparentIconBackground, resolveIconRenderStroke, sunPositionFromAzimuthElevation, threeHostPort, type HostPortOverrides, type ThreeHostPort } from "./🔌️host-ports/🟦️.tsx";
export { referenceMediaKindFromUrl, referenceMediaPort, type ReferenceMediaKind, type ReferenceMediaLoadResult, type ReferenceMediaPort, type ReferenceMediaSource } from "./🖼️reference-media/🟦️.ts";
export { SELECTION_DRAG_DIRECTION_THRESHOLD_PX, SelectionMarquee, marqueeCoverageFromDrag, marqueeCoverageFromGesture, marqueeCoverageFromPath, marqueeIsCrossing, marqueeIsCrossingFromPath, marqueeModeFromModifiers, screenRectContainsRect, screenRectFromPoints, screenRectIntersectsRect, selectionMergeIds, type ScreenRect, type SelectionMarqueeCoverage, type SelectionMarqueeMethod, type SelectionMarqueePoint, type SelectionMarqueeProps, type SelectionMarqueeRect } from "./🔲️selection-marquee/🟦️.tsx";
export { CanvasPickMenu, type CanvasPickInteraction, type CanvasPickMenuProps, type UseCanvasPickInteractionOptions, useCanvasPickInteraction } from "./📋️canvas-pick/🟦️.tsx";
export { SHELL_PANEL_ANCHOR_KEY_IDS } from "./⌨️keybindings/🟦️.ts";
export { UI_CHROME_THEME_ID_STORAGE_KEY, UI_CHROME_THEME_SNAPSHOT_STORAGE_KEY, UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, UI_CUSTOM_THEMES_STORAGE_KEY, UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX, defaultComputeWorkerCount, effectiveComputeWorkerCount, isCrossOriginIsolatedRuntime, readStoredComputeWorkerCount, readStoredIntroductionSeen, readStoredUiChromeThemeId, readStoredUiChromeThemeSnapshot, readStoredUiCustomThemes, writeStoredComputeWorkerCount, writeStoredIntroductionSeen, writeStoredUiChromeThemeId, writeStoredUiChromeThemeSnapshot, writeStoredUiCustomThemes } from "./🔑️chrome-prefs/🟦️.ts";
export { createTerminologyLabelResolver, type UiTerminologyLabelSet } from "./🗣️i18n-terminology/🟦️.ts";
export { PANEL_TOGGLE_HOTKEYS, useActionHotkey, usePanelChromeHotkeys } from "./🔥️hotkeys/🟦️.ts";
export { InteractionProvider, SectionSpecificity, UiDeviceProvider, useUiDevice } from "./📱️device-context/🟦️.tsx";
export { INTRODUCTION_INFO_BOX_GAP_PX, clampIntroductionInfoBoxPosition, introductionPointRelativeToHost, introductionRectRelativeToHost, ndcToViewportPoint, registerIntroductionSurfaceResolver, resolveIntroductionPlacement, splitIntroductionBodyParagraphs, type IntroductionResolvedGeometry, type IntroductionSurfaceResolver } from "./📐️introduction-geometry/🟦️.ts";
export { INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, polylinePointAt, resolveIntroductionPoint, sampleBezierSegments, useIntroductionPointerIdle } from "./🧭️introduction-resolve/🟦️.ts";
export { introductionDemoArcPoint, introductionDemoResolveVisual } from "./🎬️introduction-demo/🟦️.tsx";
export { UIIntroduction, type UIIntroductionProps } from "./🎓️introduction/🟦️.tsx";
export { applyTutorialUiChange, composeTutorialUi, createTutorialClock, formatTutorialTime, interpolateTutorialCamera, tutorialCameraAt, tutorialCuesBetween, tutorialSlice, type TutorialClock, type TutorialClockPort, type TutorialSlice, useTutorialClock, validateTutorial } from "./🎞️tutorial-engine/🟦️.ts";
export { TutorialBar, TutorialCaptions, TutorialGhostPointer, TutorialVideoOverlay, getTutorialCameraDriver, registerTutorialCameraDriver, type TutorialBarProps, type TutorialCameraDriver, type TutorialCaptionsProps, type TutorialChapterMarker, type TutorialGhostPointerProps, type TutorialVideoOverlayProps } from "./🎥️tutorial-ui/🟦️.tsx";
export { activeLineClass, borderEmphasizedBottomClass, borderEmphasizedClass, borderEmphasizedFrameClass, borderEmphasizedTopClass, borderNormalFrameClass, canvasHostRootClass, canvasViewportClass, editorShellRootClass, floatingFieldSurfaceClass, floatingMenuSurfaceClass, floatingPaneAsideClass, floatingRibbonSurfaceClass, floatingTagClass, floatingTagOffClass, floatingTagOnClass, flowChevronIconName, shellChromeBorderClass } from "./🎨️chrome-classes/🟦️.ts";
export { PanelDockProvider, computeTabDockDropZone, findPanelTabInDock, isPanelTabInSubtree, moveTabInDock, moveTreeUnitInDock, normalizePanelTabOrder, pruneEmptyPanelBranches, type PanelDockProviderProps, type PanelTabDockMove, type PanelTabRowDropTarget } from "./🧲️panel-dock/🟦️.tsx";
export { PanelChromeTabBar, modeDockChromeGridPlacement, type ModeDockChromeGrid, type PanelChromeTabBarProps } from "./📑️panel-chrome-tab-bar/🟦️.tsx";
export { CELEBRATE_STAMP_DURATION_MS, celebrateAllElements, celebrateElement, celebrateElements, type UiElementStateProps, useElementState } from "./🧬️element-state/🟦️.ts";
export { Aside, NotFound, RouteLink, Spinner, navigateOwnedRoute, parseOwnedRouteTarget, type AsideProps, type NotFoundProps, type SpinnerProps } from "./📣️display-components/🟦️.tsx";
export { Badge, EmptyState, ErrorView, StatCard, StatusChip, UI_ELEMENT_REGISTRY, type BadgeProps, type EmptyStateProps, type ErrorViewProps, type StatCardProps, type StatusChipProps, type StatusTone } from "./🚦️status-surface/🟦️.tsx";
export { Section, type SectionProps } from "./🖲️section/🟦️.tsx";
export { DesktopTitlebar, NavbarTrailingChromeSlot, publishShellNavbarTrailingEndWidthPx, readDocumentFullscreenActive, toggleDocumentFullscreen, type DesktopTitlebarProps, useDocumentFullscreen } from "./🖥️fullscreen/🟦️.tsx";
export { WINDOW_SEARCH_USER, engagementActionTokenEquals, isSearchSuggestionActionTarget, searchActiveCompletionSuffix, searchCompletionSuffix } from "./🔎️window-search/🟦️.ts";
export { ChromeAwareWindowScrollSurface, shellChromeSectionTitleClassName, windowChromeControlButtonClass, windowControlsCapActiveSplitClass, windowEngagementMaxWidthPx, windowMeasureControlClass, windowMeasureGroupChildrenClass, windowMeasureGroupHeaderClass, windowMeasureLabelClass, windowMeasureSectionClass, windowMeasureTileClass, windowMeasureTileNestedClass, windowMeasureToggleClass, windowMeasureToggleCompactClass, windowMeasuresStackInnerClass, windowRailChromeLabelActionClass } from "./🧱️window-chrome/🟦️.tsx";

export { CatalogIcon, type CatalogIconProps } from "@semio-tech/ui/elements/Icons";
export { INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR } from "./🎬️introduction-demo/🟦️.tsx";

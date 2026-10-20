/** 🪟️ The slim chrome of the React target as `@semio-tech/ui-react/chrome`: the window chrome, the overview card and the
 * layered overview (strip, glass, card grid) with its pure geometry, the navbar with its brand logo, catalog icons, class composition, the breakpoints, the document's surface chrome
 * (appearance, device, driver) and the presence palette of shared sessions. Documents that show cards rather than an OS shell — the quiz site, landing pages —
 * import this instead of the barrel and ship none of the shell's scene, flow, pdf or panel machinery. Everything here
 * is re-exported unchanged by the barrel as well.
 *
 * @see ../🟦️.tsx — the full React target barrel
 * @see ../🌓️appearance/🟦️.ts — the surface chrome
 * @see ../../../🧱️elements/🃏️OverviewCard/🟦️.tsx — the overview card
 * @see ../../../🧱️elements/🥞️LayeredOverview/🟦️.tsx — the layered overview
 */

export { cn } from "@semio-tech/ui/modules/class-name-composition";
export { WindowChrome, windowChromeTitleChipClass, type WindowChromeControlAction, type WindowChromeProps } from "@semio-tech/ui/elements/WindowChrome";
export {
  OverviewCard,
  OverviewCardAction,
  OverviewCardOpenChip,
  overviewCardChipClass,
  type OverviewCardActionProps,
  type OverviewCardBaseProps,
  type OverviewCardButtonProps,
  type OverviewCardProps,
  type OverviewCardSectionProps,
} from "@semio-tech/ui/elements/OverviewCard";
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
  inWindow,
  LAYERED_FOLLOW_FRAME_MS,
  followFactor,
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
export { Navbar, SemioLogo, ShellBrandLogo, navbarFillItem, type NavbarItem, type NavbarProps } from "@semio-tech/ui/elements/Navbar";
export { CatalogIcon as Icon, type IconName, type CatalogIconProps as IconProps } from "@semio-tech/ui/elements/Icons";
export { DEFAULT_UI_DRIVER, type UiDriver } from "@semio-tech/ui/elements/UiDriver";
export { UI_AVAILABLE_HEIGHT, UI_MOBILE_MAX_WIDTH_PX, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MAX_WIDTH_PX, UI_TABLET_MEDIA_QUERY, elementsSurfaceDeviceForLayout, elementsSurfaceDeviceForMatches, type ElementsSurfaceDevice, type ElementsSurfaceLayout } from "@semio-tech/ui/device";
export {
  bootstrapElementsSurfaceChromeDocument,
  readStoredUiChromeAppearance,
  resolveElementsSurfaceChromeDark,
  useElementsSurfaceChrome,
  useMediaQuery,
  writeStoredUiChromeAppearance,
  type ElementsSurfaceAppearance,
  type ElementsSurfaceBrowserDefaults,
  type ElementsSurfaceChromeInput,
} from "@semio-tech/ui/appearance";
export { presenceColor, presenceCssVar, presencePaint, type PresenceAppearance, type PresenceHsl } from "@semio-tech/ui/modules/presence-presentation";
export { ResponsiveLabel, shellChromeTitleClassName } from "./🏷️responsive-label/🟦️.tsx";

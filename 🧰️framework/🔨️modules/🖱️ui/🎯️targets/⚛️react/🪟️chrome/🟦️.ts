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

export { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
export { WindowChrome, windowChromeTitleChipClass, type WindowChromeControlAction, type WindowChromeProps } from "../../../🧱️elements/🗂️WindowChrome/🟦️.tsx";
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
} from "../../../🧱️elements/🃏️OverviewCard/🟦️.tsx";
export {
  LayeredOverview,
  capturePosterFromCanvases,
  type LayeredCardState,
  type LayeredChromeState,
  type LayeredLabels,
  type LayeredMode,
  type LayeredOverviewProps,
  type LayeredRest,
  type LayeredPane,
  type LayeredPaneState,
} from "../../../🧱️elements/🥞️LayeredOverview/🟦️.tsx";
export {
  LAYERED_DEFAULT_LIFECYCLE,
  LAYERED_VIEW,
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
  coverPlacement,
  glideRect,
  lerpRect,
  restRect,
  spanAxisBounds,
  trackSpans,
  trackTemplate,
  veilForRect,
  viewRect,
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
  type LayeredRect,
  type LayeredTracks,
} from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
export { Navbar, SemioLogo, ShellBrandLogo, navbarFillItem, type NavbarItem, type NavbarProps } from "../../../🧱️elements/🔝️Navbar/🟦️.tsx";
export { Icon, type IconName, type IconProps, type IconSource } from "../../../🧱️elements/🔣️Icons/🟦️.tsx";
export { DEFAULT_UI_DRIVER, type UiDriver } from "../../../🧱️elements/🚗️UiDriver/🟦️.tsx";
export { UI_AVAILABLE_HEIGHT, UI_MOBILE_MAX_WIDTH_PX, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MAX_WIDTH_PX, UI_TABLET_MEDIA_QUERY, elementsSurfaceDeviceForMatches, type ElementsSurfaceDevice } from "../../../📱️device/🟦️.ts";
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
} from "../🌓️appearance/🟦️.ts";
export { presenceColor, presenceCssVar, presencePaint, type PresenceAppearance, type PresenceHsl } from "../../../🔨️modules/👥️presence-presentation/🟦️.ts";

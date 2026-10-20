import { UI_CHROME_APPEARANCE_STORAGE_KEY, UI_CHROME_LAYOUT_STORAGE_KEY, applyChromeRevealAtPoint, applyElementsSurfaceChrome, bootstrapElementsSurfaceChromeDocument, installElementsSurfaceBrowserDefaultSuppression, isElementsSurfaceChromeDarkApplied, readStoredUiChromeAppearance, readStoredUiChromeLayout, resetElementsSurfaceChromeForTests, resolveElementsSurfaceChromeDark, resolveElementsSurfaceChromeRoot, useCanvasAppearanceSync, useElementsSurfaceChrome, useMediaQuery, writeStoredUiChromeAppearance, writeStoredUiChromeLayout, type ElementsSurfaceAppearance, type ElementsSurfaceBrowserDefaults, type ElementsSurfaceChromeInput, type UiChromeLayout } from "@semio-tech/ui/appearance";
import { WINDOW_SILHOUETTE_GEOMETRY_SCHEMA, WINDOW_SILHOUETTE_PATH_INSET, WINDOW_SILHOUETTE_CHIP_EPSILON, normalizeWindowSilhouetteChips, normalizeWindowSilhouetteMetrics, windowSilhouetteEdgePoints, windowSilhouetteEdgePointsRtl, windowSilhouetteOutline, simplifyWindowSilhouetteOutline, windowSilhouetteOutlineViolations, windowSilhouettePathFromOutline, windowSilhouettePath, windowSilhouetteContentClipPath, windowSilhouetteSafeClearances, windowSilhouetteBodyRegion, windowSilhouetteGlassRegions, windowSilhouetteContentRegions, windowSilhouetteRegionContains, windowSilhouetteContains, pendingWindowSilhouetteMetrics, createWindowSilhouetteGeometry, type WindowSilhouetteChip, type WindowSilhouetteEdge, type WindowSilhouetteMetrics, type WindowSilhouettePoint, type WindowSilhouetteDock, type WindowSilhouetteRegion, type WindowSilhouetteSafeClearances, type PendingWindowSilhouetteMetrics, type WindowSilhouetteGeometry } from "@semio-tech/ui/elements/WindowSilhouette";
import { CHROME_CONTROL_TOOLTIP_DELAY_MS, ChromeControlHint, DisabledReasonHint } from "@semio-tech/ui/elements/ChromeControlHint";
import { uiDataLabel, type UiLabel } from "@semio-tech/ui/elements/UiLabel";
import { type UiLocale, type UiLabelPair, type UiLabelValue, type UiRibbonParentCategory, type UiRibbonParentEntries, type DeepUiTranslationKeys, type UiTranslationSchema, type UiTranslationKey, type AssertUiRibbonParentKeysCovered, type AssertUiSettingsLanguageKeysCovered, type UiChromeTerminologyId, type AssertUiSettingsTerminologyKeysCovered, type UiTranslateFn, type UiI18nPort, type UiRegisteredTranslationKey, UI_RIBBON_PARENT_CATEGORIES } from "@semio-tech/ui/elements/I18n";
import { UI_CHROME_LOCALE_STORAGE_KEY, createShellI18nInstance, detectShellLocale, disposeShellI18nInstance, initUiLocaleSync, readStoredUiChromeLocale, registerUiTranslationBundles, setUiLocale, uiChromeTranslationBundles, uiI18n, writeStoredUiChromeLocale, type UiTranslationBundlesInput, type UiTranslationLocaleCode } from "@semio-tech/ui/i18n";

export { UI_CHROME_APPEARANCE_STORAGE_KEY, UI_CHROME_LAYOUT_STORAGE_KEY, applyChromeRevealAtPoint, applyElementsSurfaceChrome, bootstrapElementsSurfaceChromeDocument, installElementsSurfaceBrowserDefaultSuppression, isElementsSurfaceChromeDarkApplied, readStoredUiChromeAppearance, readStoredUiChromeLayout, resetElementsSurfaceChromeForTests, resolveElementsSurfaceChromeDark, resolveElementsSurfaceChromeRoot, useCanvasAppearanceSync, useElementsSurfaceChrome, useMediaQuery, writeStoredUiChromeAppearance, writeStoredUiChromeLayout, type ElementsSurfaceAppearance, type ElementsSurfaceBrowserDefaults, type ElementsSurfaceChromeInput, type UiChromeLayout };

// 📱️ The device vocabulary and the breakpoint policy are NOT declared here: `📱️device/🟦️.ts` owns them
// once for both renderer targets (the wgpu dock compares the same thresholds against its own `screen_w`),
// so React cannot drift from wgpu about what width is a phone, a tablet or a desktop.
export {
  UI_AVAILABLE_HEIGHT,
  UI_MOBILE_MAX_WIDTH_PX,
  UI_MOBILE_MEDIA_QUERY,
  UI_TABLET_MAX_WIDTH_PX,
  UI_TABLET_MEDIA_QUERY,
  availableViewportHeightPx,
  elementsSurfaceDeviceForMatches,
  elementsSurfaceDeviceForWidth,
  elementsSurfaceDeviceIsMobile,
  elementsSurfaceDeviceSupportsTabDrag,
} from "@semio-tech/ui/device";
export type { ElementsSurfaceDevice } from "@semio-tech/ui/device";
export {
  WINDOW_SILHOUETTE_GEOMETRY_SCHEMA,
  WINDOW_SILHOUETTE_PATH_INSET,
  WINDOW_SILHOUETTE_CHIP_EPSILON,
  normalizeWindowSilhouetteChips,
  normalizeWindowSilhouetteMetrics,
  windowSilhouetteEdgePoints,
  windowSilhouetteEdgePointsRtl,
  windowSilhouetteOutline,
  simplifyWindowSilhouetteOutline,
  windowSilhouetteOutlineViolations,
  windowSilhouettePathFromOutline,
  windowSilhouettePath,
  windowSilhouetteContentClipPath,
  windowSilhouetteSafeClearances,
  windowSilhouetteBodyRegion,
  windowSilhouetteGlassRegions,
  windowSilhouetteContentRegions,
  windowSilhouetteRegionContains,
  windowSilhouetteContains,
  pendingWindowSilhouetteMetrics,
  createWindowSilhouetteGeometry,
  type WindowSilhouetteChip,
  type WindowSilhouetteEdge,
  type WindowSilhouetteMetrics,
  type WindowSilhouettePoint,
  type WindowSilhouetteDock,
  type WindowSilhouetteRegion,
  type WindowSilhouetteSafeClearances,
  type PendingWindowSilhouetteMetrics,
  type WindowSilhouetteGeometry,
};
export { CHROME_CONTROL_TOOLTIP_DELAY_MS, ChromeControlHint, DisabledReasonHint };
export { uiDataLabel, type UiLabel };
export type {
  UiLocale,
  UiLabelPair,
  UiLabelValue,
  UiRibbonParentCategory,
  UiRibbonParentEntries,
  DeepUiTranslationKeys,
  UiTranslationSchema,
  UiTranslationKey,
  AssertUiRibbonParentKeysCovered,
  AssertUiSettingsLanguageKeysCovered,
  UiChromeTerminologyId,
  AssertUiSettingsTerminologyKeysCovered,
  UiTranslateFn,
  UiI18nPort,
  UiRegisteredTranslationKey,
};
export { UI_RIBBON_PARENT_CATEGORIES };
export { UI_CHROME_LOCALE_STORAGE_KEY, createShellI18nInstance, detectShellLocale, disposeShellI18nInstance, initUiLocaleSync, readStoredUiChromeLocale, registerUiTranslationBundles, setUiLocale, uiChromeTranslationBundles, uiI18n, writeStoredUiChromeLocale };
export type { UiTranslationBundlesInput, UiTranslationLocaleCode };

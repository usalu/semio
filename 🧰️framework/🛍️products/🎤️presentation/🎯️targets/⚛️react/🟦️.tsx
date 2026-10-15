// #region 🧲️Header
/** 📽️ `@semio-tech/presentation-react` — React + reveal.js renderer for `@semio-tech/presentation` declarative decks. */
// #endregion 🧲️Header

import "reveal.js/dist/reveal.css";
import { readPresentationSlideIndicesFromUrl, syncPresentationSlideUrl } from "./🔨️modules/🎚️deck-chrome/🟦️.ts";
import { resolveRevealArrangement } from "./🔨️modules/🔀️reveal-morph/🟦️.ts";
import {
  clearRevealAutoAnimateInlineLayout,
  elementIsInteractiveFigureDisposition,
  elementIsSourceGhostAnchor,
  elementIsTargetGhostAnchor,
  finalizeRevealAutoAnimateRestState,
  isManyToOneMorphTransition,
  isRevealAutoAnimatePairSource,
  isRevealSlideAutoAnimating,
  isSectionAutoAnimating,
  patchAutoAnimateUniformScale,
  patchPresentationAutoAnimateRunStyleSheet,
  patchPresentationAutoAnimateStyleSheet,
  prepareArrangementBeforeAutoAnimate,
  PRESENTATION_MANY_TO_ONE_MORPH_CLASS,
  presentationAutoAnimateMatcher,
  relaxHiddenPreflight,
  resolvePresentationAutoAnimateRunSlides,
  revealInkMeasureForAutoAnimate,
  revealTextAutoAnimatePairOptions,
  slidesShareAutoAnimateId,
  syncArrangementSettledState,
  syncManyToOneGhostMorphFramesFromDom
} from "./🔨️modules/✨️auto-animate/🟦️.ts";
import {
  pdfAdjacentPage,
  pdfCanGoToNextPage,
  pdfCanGoToPreviousPage,
  pdfCoverScale,
  pdfEmbodimentInitialPage,
  pdfPageNavEnabled,
  pdfScrollCoverScale,
  resetPdfCanvasPort,
  setPdfCanvasPort
} from "./🔨️modules/📑️pdf-canvas-view/🟦️.ts";
import {
  figureCropBackgroundPosition,
  figureCropBackgroundSize,
  figureCropBackgroundVars,
  figureCropBackgroundVarsTargetGhost,
  morphFrameCssVars,
  mosaicWindowedCoverVars,
  resolvePresentationAssetUrl
} from "./🔨️modules/📐️figure-frames/🟦️.ts";
import {
  FIGURE_WHEEL_ZOOM_MAX,
  figureBackgroundSizeScrollAxis,
  figureBackgroundSizeZoomed,
  figureCoverOverflowAxis,
  figureCoverScrollContentSize,
  figureCoverScrollElementStyle,
  figureCropScrollBackgroundSize,
  figureEmbodimentScrollEnabled,
  figureScrollOffsetForBackgroundPosition,
  figureScrollOverlayThumbMetrics,
  figureWheelZoomStep,
  mediaTeaserActive
} from "./🔨️modules/🖼️figure-scroll/🟦️.tsx";
import {
  buildInteractiveSlideLayout,
  clientToSectionFraction,
  dispositionHasEphemeralLayout,
  dispositionPlacementContainer,
  flowDispositionManipulationRect,
  flowDispositionOffsetStyle,
  flowPixelOffsetToSectionRect,
  flowPointerDeltaToLocal,
  groupBoundingRect,
  interactiveDispositionChromeStyle,
  interactiveDispositionContentScale,
  interactiveDispositionContentScaleStyle,
  isFlowPixelOffsetTransform,
  isNormalizedSlideFrame,
  isUsableMeasuredRect,
  marqueeSelectionRule,
  marqueeSelects,
  measureDispositionBoundsInContainer,
  measureDispositionBoundsInSection,
  normalizeMarquee,
  pointerNearSlideResetHotspot,
  rectContains,
  rectsIntersect,
  resizeDispositionRect,
  revealDeckInOverview,
  scaleRectWithinGroup,
  SLIDE_INTERACTIVE_ENLARGE_FRAME,
  slideCoordinateRoot,
  slideHasEphemeralLayout,
  suppressRevealOverviewSlideNavigation,
  tightElementBoundsRect,
  toggleEnlargeRect,
  transformFrameStyle,
  translateDispositionRect
} from "./🔨️modules/🧮️interaction-model/🟦️.ts";
import { mountPresentation, PresentationDeck, unmountPresentation } from "./🔨️modules/🎞️deck/🟦️.tsx";
import { jsonPreview, renderJsonTree } from "./🔨️modules/🌳️json-tree/🟦️.tsx";

export {
  compileMarkdownToHtml,
  type MarkdownHtmlCompiler,
  type PresentationMountOptions,
  readPresentationSlideIndicesFromUrl,
  setMarkdownHtmlCompiler,
  syncPresentationAutoAnimateDurationVar,
  syncPresentationSlideSizeVars,
  syncPresentationSlideUrl,
  syncRevealBackgroundKind
} from "./🔨️modules/🎚️deck-chrome/🟦️.ts";

export {
  centerRevealResolvedArrangement,
  resolveRevealArrangement,
  type RevealMorphCompanionKind,
  type RevealResolvedDisposition
} from "./🔨️modules/🔀️reveal-morph/🟦️.ts";

export {
  type AutoAnimateMatcherHost,
  clearManyToOneMorphArrangementClass,
  clearRevealAutoAnimateInlineLayout,
  elementIsInteractiveFigureDisposition,
  elementIsSourceGhostAnchor,
  elementIsTargetGhostAnchor,
  finalizeRevealAutoAnimateRestState,
  isManyToOneMorphTransition,
  isRevealAutoAnimatePairSource,
  isRevealSlideAutoAnimating,
  isSectionAutoAnimating,
  patchAutoAnimateUniformScale,
  patchPresentationAutoAnimateRunStyleSheet,
  patchPresentationAutoAnimateStyleSheet,
  prepareArrangementBeforeAutoAnimate,
  PRESENTATION_MANY_TO_ONE_MORPH_CLASS,
  presentationAutoAnimateMatcher,
  type PresentationAutoAnimateRunSlides,
  presentationMorphGhostAutoAnimateCss,
  relaxHiddenPreflight,
  resolveMorphAutoAnimateTo,
  resolvePresentationAutoAnimateRunSlides,
  resolveRevealSlideAt,
  revealInkMeasureForAutoAnimate,
  revealTextAutoAnimatePairOptions,
  slideChangedEventSlides,
  slidesShareAutoAnimateId,
  syncArrangementSettledState,
  syncManyToOneGhostMorphFramesFromDom,
  syncManyToOneMorphArrangementClass
} from "./🔨️modules/✨️auto-animate/🟦️.ts";

export {
  parsePresentationSlideCssSize
} from "./🔨️modules/🏷️morph-context/🟦️.ts";

export {
  pdfAdjacentPage,
  pdfCanGoToNextPage,
  pdfCanGoToPreviousPage,
  pdfCoverScale,
  pdfEmbodimentInitialPage,
  pdfEmbodimentPageList,
  pdfPageNavEnabled,
  pdfScrollCoverScale,
  resetPdfCanvasPort,
  setPdfCanvasPort
} from "./🔨️modules/📑️pdf-canvas-view/🟦️.ts";

export {
  applyFigureCropCssVars,
  applyMorphFrameCssVars,
  figureCropBackgroundPosition,
  figureCropBackgroundSize,
  figureCropBackgroundVars,
  figureCropBackgroundVarsTargetGhost,
  figureCropPhysicalAspect,
  figureMosaicBackgroundPosition,
  figureMosaicCellIndex,
  morphFrameCssVars,
  mosaicWindowedCoverVars,
  overflowAxisPosition,
  readPercentDispositionFrame,
  resolvePresentationAssetUrl
} from "./🔨️modules/📐️figure-frames/🟦️.ts";

export {
  estimateDispositionFramePixels,
  FIGURE_WHEEL_ZOOM_MAX,
  FIGURE_WHEEL_ZOOM_MIN,
  figureBackgroundSizeScrollAxis,
  figureBackgroundSizeZoomed,
  figureCoverOverflowAxis,
  figureCoverScrollContentSize,
  figureCoverScrollElementStyle,
  figureCropScrollBackgroundSize,
  figureCropScrollContentSize,
  figureEmbodimentScrollEnabled,
  figureScrollOffsetForBackgroundPosition,
  figureScrollOverlayThumbMetrics,
  figureWheelZoomAdjustScroll,
  figureWheelZoomStep,
  mediaTeaserActive,
  pdfEmbodimentScrollEnabled,
  useFigureZoom,
  videoEmbodimentScrollEnabled
} from "./🔨️modules/🖼️figure-scroll/🟦️.tsx";

export {
  buildInteractiveSlideLayout,
  clientToSectionFraction,
  declaredDispositionRect,
  dispositionFrameElement,
  dispositionHasEphemeralLayout,
  dispositionInteractionId,
  dispositionPlacementContainer,
  dispositionPositionChanged,
  type DispositionResizeHandle,
  type DispositionTransform,
  elementVisualScale,
  flowDispositionManipulationRect,
  flowDispositionOffsetStyle,
  flowDispositionReserveStyle,
  flowDragTransformElement,
  flowPixelOffsetToSectionRect,
  flowPointerDeltaToLocal,
  groupBoundingRect,
  interactiveDispositionChromeStyle,
  interactiveDispositionContentScale,
  interactiveDispositionContentScaleStyle,
  type InteractiveDispositionPlacement,
  type InteractiveRowBandPlacement,
  type InteractiveSlideLayout,
  isFlowPixelOffsetTransform,
  isNormalizedSlideFrame,
  isUsableMeasuredRect,
  marqueeSelectionRule,
  type MarqueeSelectionRule,
  marqueeSelects,
  measureDispositionBoundsInContainer,
  measureDispositionBoundsInSection,
  measureElementRectInSection,
  normalizeMarquee,
  pointerNearSlideResetHotspot,
  rectContains,
  rectsIntersect,
  resizeDispositionRect,
  revealDeckInOverview,
  rowBandInteractionId,
  scaleRectWithinGroup,
  sectionVisualScale,
  SLIDE_INTERACTIVE_ENLARGE_FRAME,
  slideCoordinateRoot,
  slideHasEphemeralLayout,
  slideLayoutBounds,
  suppressRevealOverviewSlideNavigation,
  tightElementBoundsRect,
  tileDispositionInteractionId,
  toggleEnlargeRect,
  transformFrameStyle,
  translateDispositionRect
} from "./🔨️modules/🧮️interaction-model/🟦️.ts";

export {
  usePresentationInteraction
} from "./🔨️modules/🫳️interaction/🟦️.tsx";

export {
  mountPresentation,
  PresentationDeck,
  unmountPresentation
} from "./🔨️modules/🎞️deck/🟦️.tsx";

export {
  jsonPreview,
  type JsonTreeRenderer,
  renderJsonTree,
  setJsonTreeRenderer
} from "./🔨️modules/🌳️json-tree/🟦️.tsx";

export {
  PdfCanvasResourceOwner,
  pdfCanvasBitmapSize,
  pdfCanvasStatusAnnouncement,
  type PdfCanvasDocument,
  type PdfCanvasLoadingTask,
  type PdfCanvasPage,
  type PdfCanvasPort,
  type PdfCanvasRenderTask,
  type PdfCanvasStatus,
  type PdfCanvasViewport,
} from "./🔨️modules/🔌️pdf-canvas-port/🟦️.ts";
export type {
  AffiliationEntry,
  AffiliationsEmbodiment,
  Arrangement,
  AuthorPerson,
  AuthorsEmbodiment,
  BulletEmbodiment,
  Chapter,
  Disposition,
  DispositionPosition,
  DispositionStyle,
  Embodiment,
  FigureEmbodiment,
  IframeEmbodiment,
  JsonEmbodiment,
  MarkdownEmbodiment,
  MediaTeaser,
  MediaScrollOrigin,
  Participant,
  ParticipantEmphasis,
  PdfEmbodiment,
  Presentation,
  RenderSlide,
  ResolvedDisposition,
  Sequence,
  Slide,
  TextEmbodiment,
  Thought,
  Transition,
  VideoEmbodiment,
} from "@semio-tech/presentation";
export {
  analogy,
  buildResolutionScope,
  collectPresentationSlides,
  countArrangements,
  expandThoughtSlides,
  formatPresentationUrlHash,
  intro,
  morphId,
  parsePresentationSlideHash,
  PRESENTATION_CHAPTER_QUERY_PARAM,
  PRESENTATION_SEQUENCE_QUERY_PARAM,
  PRESENTATION_SLIDE_QUERY_PARAM,
  PRESENTATION_THOUGHT_QUERY_PARAM,
  presentationEntityBookmarkName,
  presentationLanguage,
  presentationSequences,
  presentationSlideAt,
  presentationSlideBookmarkParamKeys,
  resolutionScopeForArrangement,
  resolveArrangement,
  resolveEmbodiment,
  resolveMediaScrollOrigin,
  resolveTextMorphRoot,
  split,
  splitFigureGrid,
  tile,
  unionSourceCrops,
} from "@semio-tech/presentation";
export type { MorphFromSlot, PresentationLanguageKind, PresentationSlideBookmark, PresentationSlideBookmarkParamKeys, PresentationSlideRef, TextMorphRoot } from "@semio-tech/presentation";

//#region 🔖️json
//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerPresentationReactDeckTests } = await import("../../🧪️tests/🎞️presentation-react-deck/🟦️.tsx");
  await registerPresentationReactDeckTests(import.meta.vitest, { FIGURE_WHEEL_ZOOM_MAX, jsonPreview, PRESENTATION_MANY_TO_ONE_MORPH_CLASS, PresentationDeck, SLIDE_INTERACTIVE_ENLARGE_FRAME, buildInteractiveSlideLayout, clearRevealAutoAnimateInlineLayout, clientToSectionFraction, dispositionHasEphemeralLayout, dispositionPlacementContainer, elementIsInteractiveFigureDisposition, elementIsSourceGhostAnchor, elementIsTargetGhostAnchor, figureBackgroundSizeScrollAxis, figureBackgroundSizeZoomed, figureCoverOverflowAxis, figureCoverScrollContentSize, figureCoverScrollElementStyle, figureCropBackgroundPosition, figureCropBackgroundSize, figureCropBackgroundVars, figureCropBackgroundVarsTargetGhost, figureCropScrollBackgroundSize, figureEmbodimentScrollEnabled, figureScrollOffsetForBackgroundPosition, figureScrollOverlayThumbMetrics, figureWheelZoomStep, finalizeRevealAutoAnimateRestState, flowDispositionManipulationRect, flowDispositionOffsetStyle, flowPixelOffsetToSectionRect, flowPointerDeltaToLocal, groupBoundingRect, interactiveDispositionChromeStyle, interactiveDispositionContentScale, interactiveDispositionContentScaleStyle, isFlowPixelOffsetTransform, isManyToOneMorphTransition, isNormalizedSlideFrame, isRevealAutoAnimatePairSource, isRevealSlideAutoAnimating, isSectionAutoAnimating, isUsableMeasuredRect, marqueeSelectionRule, marqueeSelects, measureDispositionBoundsInContainer, measureDispositionBoundsInSection, mediaTeaserActive, morphFrameCssVars, mosaicWindowedCoverVars, mountPresentation, normalizeMarquee, patchAutoAnimateUniformScale, patchPresentationAutoAnimateRunStyleSheet, patchPresentationAutoAnimateStyleSheet, pdfAdjacentPage, pdfCanGoToNextPage, pdfCanGoToPreviousPage, pdfCoverScale, pdfEmbodimentInitialPage, pdfPageNavEnabled, pdfScrollCoverScale, pointerNearSlideResetHotspot, prepareArrangementBeforeAutoAnimate, presentationAutoAnimateMatcher, readPresentationSlideIndicesFromUrl, rectContains, rectsIntersect, relaxHiddenPreflight, renderJsonTree, resetPdfCanvasPort, resizeDispositionRect, resolvePresentationAssetUrl, resolvePresentationAutoAnimateRunSlides, resolveRevealArrangement, revealDeckInOverview, revealInkMeasureForAutoAnimate, revealTextAutoAnimatePairOptions, scaleRectWithinGroup, setPdfCanvasPort, slideCoordinateRoot, slideHasEphemeralLayout, slidesShareAutoAnimateId, suppressRevealOverviewSlideNavigation, syncArrangementSettledState, syncManyToOneGhostMorphFramesFromDom, syncPresentationSlideUrl, tightElementBoundsRect, toggleEnlargeRect, transformFrameStyle, translateDispositionRect, unmountPresentation }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
//#endregion 🔖️json

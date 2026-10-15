/** 🏷️ Morph text and anchor class names, the slide epoch, aspect, enlarge and crop-frame contexts and the slide css size parser.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type { DispositionPosition, ParticipantEmphasis } from "@semio-tech/presentation";
import { createContext } from "@semio-tech/ui-react";

//#region 🔖️MorphView
const presentationMorphTextClass = "presentation-morph-text";

function morphTextSizeClass(morphId: string): string {
  return morphId === "title" ? "presentation-morph-text--title" : "presentation-morph-text--secondary";
}

export function morphTextClass(morphId: string, extra?: string): string {
  return [presentationMorphTextClass, morphTextSizeClass(morphId), extra].filter(Boolean).join(" ");
}

export function emphasisClass(emphasis: ParticipantEmphasis | undefined): string | undefined {
  return emphasis === "muted" ? "opacity-20" : undefined;
}

export function morphAnchorClass(emphasis: ParticipantEmphasis): string {
  return ["presentation-morph-anchor", emphasisClass(emphasis)].filter(Boolean).join(" ");
}

//#region 🔖️SlideEpoch
export const PresentationSlideEpochContext = createContext(0);

/** 🔗️ When true, the interactive canvas wrapper owns `data-id` for reveal.js auto-animate. */
export const MorphAnchorOnWrapperContext = createContext(false);

/** ⛶️ When true, {@link PdfMorphView} measures against the enlarged slide content box, not the declared catalogue frame. */
export const PresentationDispositionEnlargeContext = createContext(false);

/** 📐️ Live slide frame for {@link FigureMorphView} cover math (drag/resize updates this). */
export const PresentationFigureCropFrameContext = createContext<DispositionPosition | undefined>(undefined);

/** 📐️ Slide width÷height for mosaic windowed-cover (`undefined` → use {@link FigureEmbodiment.sourceAspect}). */
export const PresentationSlideAspectContext = createContext<number | undefined>(undefined);

/** 🆔️ Interactive disposition id for ephemeral pdf page navigation in {@link PdfMorphView}. */
export const PresentationInteractiveDispositionIdContext = createContext<string | undefined>(undefined);

export function parsePresentationSlideCssSize(revealEl: HTMLElement | null): { readonly width: number; readonly height: number } {
  const width = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-width") ?? "960");
  const height = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-height") ?? "700");
  return {
    width: Number.isFinite(width) && width > 0 ? width : 960,
    height: Number.isFinite(height) && height > 0 ? height : 700,
  };
}
//#endregion 🔖️SlideEpoch
//#endregion 🔖️MorphView

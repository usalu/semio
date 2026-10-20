/** 🟢️ The wrapper style phase of InteractiveDisposition. */
import { type CSSProperties, type RefObject } from "@semio-tech/ui-react";
import { type RevealResolvedDisposition } from "../../../🔀️reveal-morph/🟦️.ts";
import { isSectionAutoAnimating } from "../../../✨️auto-animate/🟦️.ts";
import { morphFrameCssVars } from "../../../📐️figure-frames/🟦️.ts";
import { dispositionHasEphemeralLayout, interactiveDispositionChromeStyle, interactiveDispositionContentScale, interactiveDispositionContentScaleStyle, SLIDE_INTERACTIVE_ENLARGE_FRAME, transformFrameStyle } from "../../../🧮️interaction-model/🟦️.ts";
import type { useSlideEpochState } from "../🔶️slide-epoch/🟦️.tsx";
import type { buildPointerDownState } from "../🟠️pointer-down/🟦️.tsx";

/** 🔵 Builds wrapperStyle, resizeContentBaseline, resizeContentScale, contentScale of InteractiveDisposition. */
export function buildWrapperStyleState({ canvasFramed, canvasAnchorRect, disposition, canvasLiveTransform, wrapperFrame, morphCropFrom, transformed, transform, flowPixelOffset, morphGhost, enlarged, measuredNatural, interactionRect, enlargedFlowContentScale, flowSectionFrame, effectiveRect, sectionRef, useFlowInkFrame, inkInWrapper, selected, gesturing, flowLayout }: Pick<ReturnType<typeof useSlideEpochState>, "canvasFramed" | "canvasAnchorRect" | "canvasLiveTransform" | "transformed" | "transform" | "flowPixelOffset" | "morphGhost" | "enlarged" | "measuredNatural" | "interactionRect" | "enlargedFlowContentScale" | "flowSectionFrame" | "effectiveRect" | "useFlowInkFrame" | "inkInWrapper" | "selected" | "gesturing" | "flowLayout"> & { readonly disposition: RevealResolvedDisposition; readonly sectionRef: RefObject<HTMLElement | null> } & Pick<ReturnType<typeof buildPointerDownState>, "wrapperFrame" | "morphCropFrom">) {
  if (canvasFramed && canvasAnchorRect) {
    const morphAnchorRect = disposition.revealMorphCompanion === "target" ? canvasAnchorRect : disposition.revealMorphCompanion === "source" ? canvasAnchorRect : (canvasLiveTransform ?? canvasAnchorRect);
    Object.assign(wrapperFrame, transformFrameStyle(morphAnchorRect));
    if (morphCropFrom && disposition.revealMorphFromFrame !== undefined && disposition.position !== undefined) {
      for (const [key, value] of Object.entries(morphFrameCssVars(disposition.revealMorphFromFrame, disposition.position))) {
        if (typeof value === "string") {
          (wrapperFrame as Record<string, string>)[key] = value;
        }
      }
    }
  } else if (transformed && transform && !flowPixelOffset) {
    Object.assign(wrapperFrame, transformFrameStyle(transform));
  }
  if (morphGhost) {
    Object.assign(wrapperFrame, { pointerEvents: "none", zIndex: 0 });
  }
  const wrapperStyle: CSSProperties | undefined = enlarged ? transformFrameStyle(SLIDE_INTERACTIVE_ENLARGE_FRAME) : Object.keys(wrapperFrame).length > 0 ? wrapperFrame : undefined;
  const resizeContentBaseline = measuredNatural ?? interactionRect;
  const resizeContentScale = !enlarged && !canvasFramed && !flowPixelOffset && transform && resizeContentBaseline ? interactiveDispositionContentScale(transform, resizeContentBaseline) : null;
  const contentScale = enlargedFlowContentScale ?? resizeContentScale;
  const contentStyle: CSSProperties | undefined = contentScale !== null ? interactiveDispositionContentScaleStyle(contentScale) : undefined;
  const hasContentStyle = contentStyle !== undefined;
  const chromeLayoutRect = flowSectionFrame && transform ? transform : (canvasLiveTransform ?? effectiveRect);
  const sectionAnimating = sectionRef.current !== null && isSectionAutoAnimating(sectionRef.current);
  const flowInkChromeRect = useFlowInkFrame ? inkInWrapper : undefined;
  const chromeStyle: CSSProperties | undefined = interactiveDispositionChromeStyle({
    selected: (selected || gesturing) && !sectionAnimating,
    effectiveRect: flowInkChromeRect ?? chromeLayoutRect,
    canvasFramed,
    enlarged,
  });
  const showControls = !morphGhost && (selected || gesturing || enlarged) && Boolean(enlarged || selected || gesturing || (useFlowInkFrame ? inkInWrapper : (chromeLayoutRect ?? effectiveRect)));
  const showHandles = showControls && !enlarged;
  const canResetPosition = enlarged || dispositionHasEphemeralLayout(transform, interactionRect, measuredNatural, flowLayout);
  return { wrapperStyle, hasContentStyle, contentStyle, showHandles, chromeStyle, showControls, canResetPosition };
}

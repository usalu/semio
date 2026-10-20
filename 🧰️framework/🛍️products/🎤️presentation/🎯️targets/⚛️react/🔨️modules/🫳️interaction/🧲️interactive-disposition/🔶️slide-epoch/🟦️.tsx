/** 🔶️ The slide epoch phase of InteractiveDisposition. */
import type { DispositionPosition } from "@semio-tech/presentation";
import { useCallback, useContext, useLayoutEffect, useRef, useState, type RefObject } from "@semio-tech/ui-react";
import { isRevealMorphCompanionOnly, type RevealResolvedDisposition } from "../../../🔀️reveal-morph/🟦️.ts";
import { isRevealSlideAutoAnimating } from "../../../✨️auto-animate/🟦️.ts";
import { PresentationSlideEpochContext } from "../../../🏷️morph-context/🟦️.ts";
import { dispositionFrameElement, dispositionPlacementContainer, dispositionPositionChanged, flowDispositionManipulationRect, flowDragTransformElement, flowPixelOffsetToSectionRect, interactiveDispositionContentScale, isFlowPixelOffsetTransform, isUsableMeasuredRect, measureDispositionBoundsInContainer, measureDispositionBoundsInSection, measureElementRectInSection, SLIDE_INTERACTIVE_ENLARGE_FRAME, usePresentationInteractionState } from "../../../🧮️interaction-model/🟦️.ts";
import { resolveEffectiveDispositionRect, useSlideDispositionRegistry } from "../../🗂️disposition-registry/🟦️.tsx";

/** 🔷 Builds slideEpoch, interaction, registry, rootRef of InteractiveDisposition. */
export function useSlideEpochState({ id, sectionDeclaredRect, declaredRect, disposition, sectionRef }: { readonly id: string; readonly sectionDeclaredRect: DispositionPosition | undefined; readonly declaredRect: DispositionPosition | undefined; readonly disposition: RevealResolvedDisposition; readonly sectionRef: RefObject<HTMLElement | null> }) {
  const slideEpoch = useContext(PresentationSlideEpochContext);
  const interaction = usePresentationInteractionState();
  const registry = useSlideDispositionRegistry();
  const rootRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const selected = interaction.isSelected(id);
  const interactionRect = sectionDeclaredRect ?? declaredRect;
  const flowLayout = interactionRect === undefined;
  const transform = interaction.getTransform(id);
  const transformed = transform !== undefined;
  const measuredNatural = registry.getRect(id);
  const flowPixelOffset = transformed && flowLayout && transform !== undefined && isFlowPixelOffsetTransform(transform, measuredNatural);
  const flowSectionFrame = transformed && flowLayout && !flowPixelOffset;
  const enlarged = interaction.isEnlarged(id);
  const morphGhost = isRevealMorphCompanionOnly(disposition);
  const canvasFramed = declaredRect !== undefined && !enlarged;
  const canvasPlacement = interactionRect !== undefined;
  const effectiveRect = resolveEffectiveDispositionRect(id, interactionRect, interaction, registry);
  const canvasAnchorRect = interactionRect;
  const canvasLiveTransform = canvasFramed && transform && canvasAnchorRect ? transform : undefined;
  const canvasDragActive = Boolean(canvasLiveTransform && canvasAnchorRect && !enlarged && dispositionPositionChanged(canvasAnchorRect, canvasLiveTransform));
  const pinned = !enlarged && ((transformed && !flowLayout && !canvasFramed) || flowSectionFrame || (canvasFramed && canvasDragActive && !flowPixelOffset));
  const [gesturing, setGesturing] = useState(false);
  const [flowReservePx, setFlowReservePx] = useState<{ readonly width: number; readonly height: number } | null>(null);
  const useFlowInkFrame = flowLayout && !flowSectionFrame;
  const [inkInWrapper, setInkInWrapper] = useState<DispositionPosition | null>(null);
  const [enlargedFlowContentScale, setEnlargedFlowContentScale] = useState<number | null>(null);
  const displayDisposition =
    enlarged && disposition.position !== undefined
      ? disposition.embodiment.kind === "figure" && disposition.embodiment.crop !== undefined
        ? { ...disposition, position: SLIDE_INTERACTIVE_ENLARGE_FRAME }
        : { ...disposition, position: undefined }
      : disposition;

  const measureDispositionRect = useCallback((): DispositionPosition | null => {
    const section = sectionRef.current;
    const root = rootRef.current;
    if (!root || !section) {
      return null;
    }
    const container = dispositionPlacementContainer(section, canvasPlacement);
    const measured = canvasPlacement ? measureElementRectInSection(dispositionFrameElement(root), container) : measureDispositionBoundsInSection(root, section);
    if (!measured || !isUsableMeasuredRect(measured)) {
      return null;
    }
    return measured;
  }, [canvasPlacement, sectionRef]);

  useLayoutEffect(() => {
    if (!transform) {
      setFlowReservePx(null);
    }
  }, [transform]);

  useLayoutEffect(() => {
    if (interactionRect) {
      registry.registerRect(id, null);
      return;
    }
    if (transform) {
      return;
    }
    const section = sectionRef.current;
    if (!section || !section.classList.contains("present") || isRevealSlideAutoAnimating(section)) {
      registry.registerRect(id, null);
      return;
    }
    const measured = measureDispositionRect();
    registry.registerRect(id, measured);
  }, [id, interactionRect, transform, registry, sectionRef, slideEpoch, measureDispositionRect]);

  useLayoutEffect(() => {
    if (!useFlowInkFrame || !(selected || gesturing || flowPixelOffset)) {
      setInkInWrapper(null);
      return;
    }
    if (gesturing) {
      return;
    }
    const section = sectionRef.current;
    if (section && isRevealSlideAutoAnimating(section)) {
      return;
    }
    const root = rootRef.current;
    if (!root) {
      setInkInWrapper(null);
      return;
    }
    setInkInWrapper(measureDispositionBoundsInContainer(root, root));
  }, [useFlowInkFrame, selected, gesturing, flowPixelOffset, transform, slideEpoch, disposition, sectionRef]);

  useLayoutEffect(() => {
    if (!enlarged || !flowLayout) {
      setEnlargedFlowContentScale(null);
      return;
    }
    const section = sectionRef.current;
    const root = rootRef.current;
    if (!section || !root || !section.classList.contains("present")) {
      setEnlargedFlowContentScale(null);
      return;
    }
    const baseline = measureDispositionBoundsInSection(root, section);
    if (!baseline) {
      setEnlargedFlowContentScale(null);
      return;
    }
    setEnlargedFlowContentScale(interactiveDispositionContentScale(SLIDE_INTERACTIVE_ENLARGE_FRAME, baseline));
  }, [enlarged, flowLayout, slideEpoch, sectionRef]);

  const ensureRectForManipulation = useCallback(
    (kind: "move" | "resize"): DispositionPosition | null => {
      const section = sectionRef.current;
      const existing = interaction.getTransform(id);
      const measuredNatural = registry.getRect(id) ?? measureDispositionRect();
      if (interactionRect) {
        return existing ?? interactionRect;
      }
      if (kind === "resize") {
        if (!measuredNatural) {
          return null;
        }
        registry.registerRect(id, measuredNatural);
        if (existing && section && isFlowPixelOffsetTransform(existing, measuredNatural)) {
          return flowPixelOffsetToSectionRect(measuredNatural, existing, section, flowDragTransformElement(section, rootRef.current, contentRef.current));
        }
        return existing ?? measuredNatural;
      }
      if (existing) {
        return existing;
      }
      if (!measuredNatural) {
        return null;
      }
      registry.registerRect(id, measuredNatural);
      return flowDispositionManipulationRect(measuredNatural, undefined);
    },
    [id, interactionRect, interaction, registry, measureDispositionRect, sectionRef],
  );
  return { canvasPlacement, registry, interaction, flowLayout, setGesturing, rootRef, contentRef, setFlowReservePx, interactionRect, selected, ensureRectForManipulation, enlarged, effectiveRect, flowPixelOffset, pinned, canvasFramed, gesturing, useFlowInkFrame, flowReservePx, transform, canvasAnchorRect, canvasLiveTransform, transformed, morphGhost, measuredNatural, enlargedFlowContentScale, flowSectionFrame, inkInWrapper, displayDisposition };
}

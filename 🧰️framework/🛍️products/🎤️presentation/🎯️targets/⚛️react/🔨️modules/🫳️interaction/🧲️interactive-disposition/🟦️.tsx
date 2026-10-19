/** 🧲️ The interactive disposition and row band: measuring, selecting, dragging, resizing and enlarging one disposition of a mounted slide. */
import type { DispositionPosition } from "@semio-tech/presentation";
import { Icon, useCallback, useContext, useLayoutEffect, useRef, useState, type CSSProperties, type FC, type RefObject } from "@semio-tech/ui-react";
import { isRevealMorphCompanionOnly, type RevealResolvedDisposition } from "../../🔀️reveal-morph/🟦️.ts";
import { isRevealSlideAutoAnimating, isSectionAutoAnimating } from "../../✨️auto-animate/🟦️.ts";
import { MorphAnchorOnWrapperContext, PresentationDispositionEnlargeContext, PresentationFigureCropFrameContext, PresentationInteractiveDispositionIdContext, PresentationSlideEpochContext } from "../../🏷️morph-context/🟦️.ts";
import { morphFrameCssVars } from "../../📐️figure-frames/🟦️.ts";
import { MorphDispositionView } from "../../🎭️morph-views/🟦️.tsx";
import { DISPOSITION_RESIZE_HANDLES, dispositionFrameElement, dispositionHasEphemeralLayout, dispositionPlacementContainer, dispositionPositionChanged, type DispositionResizeHandle, type DispositionTransform, flowDispositionManipulationRect, flowDispositionOffsetStyle, flowDispositionReserveStyle, flowDragTransformElement, flowPixelOffsetToSectionRect, flowPointerDeltaToLocal, groupBoundingRect, interactiveDispositionChromeStyle, interactiveDispositionContentScale, interactiveDispositionContentScaleStyle, isFlowPixelOffsetTransform, isUsableMeasuredRect, measureDispositionBoundsInContainer, measureDispositionBoundsInSection, measureElementRectInSection, POINTER_DRAG_THRESHOLD_PX, resizeDispositionRect, resolveDispositionDragGroupIds, scaleRectWithinGroup, SLIDE_INTERACTIVE_ENLARGE_FRAME, suppressRevealOverviewSlideNavigation, transformFrameStyle, translateDispositionRect, usePresentationInteractionState } from "../../🧮️interaction-model/🟦️.ts";
import { resolveEffectiveDispositionRect, useSlideDispositionRegistry } from "../🗂️disposition-registry/🟦️.tsx";

//#region 🔖️Interaction
export const InteractiveRowBand: FC<{
  readonly id: string;
  readonly frame: DispositionPosition;
  readonly tileIds: readonly string[];
}> = ({ id, frame, tileIds }) => {
  const interaction = usePresentationInteractionState();
  const rowSelected = tileIds.length > 0 && tileIds.every((tileId) => interaction.isSelected(tileId));
  const onPointerDown = (event: React.PointerEvent): void => {
    if (event.button !== 0) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    interaction.selectIds(tileIds, event.shiftKey);
  };
  return (
    <div
      className={["presentation-interactive-row-band", rowSelected ? "presentation-interactive-row-band--selected" : undefined].filter(Boolean).join(" ")}
      data-row-band={id}
      style={transformFrameStyle(frame)}
      onPointerDown={onPointerDown}
      aria-hidden
    />
  );
};

export const InteractiveDisposition: FC<{
  readonly id: string;
  readonly disposition: RevealResolvedDisposition;
  readonly declaredRect: DispositionPosition | undefined;
  readonly sectionDeclaredRect: DispositionPosition | undefined;
  readonly allDeclaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
  readonly sectionRef: RefObject<HTMLElement | null>;
  readonly revealMorphId?: string;
  readonly rowBandId?: string;
}> = ({ id, disposition, declaredRect, sectionDeclaredRect, allDeclaredRects, sectionRef, revealMorphId, rowBandId }) => {
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

  const attachPointerGesture = useCallback(
    (
      origin: {
        readonly pointerId: number;
        readonly clientX: number;
        readonly clientY: number;
        readonly captureEl: HTMLElement | null;
      },
      mode: "move" | DispositionResizeHandle,
      initialRect: DispositionPosition,
      requireDragThreshold: boolean,
      dragMemberIds: readonly string[],
    ): void => {
      const section = sectionRef.current;
      if (!section) {
        return;
      }
      const fractionContainer = dispositionPlacementContainer(section, canvasPlacement);
      const gesturePlacementBounds = fractionContainer.getBoundingClientRect();
      const pointerId = origin.pointerId;
      const startClient = { x: origin.clientX, y: origin.clientY };
      const groupIds = dragMemberIds.length > 1 ? [...dragMemberIds] : [id];
      const startRects = new Map<string, DispositionPosition>();
      for (const memberId of groupIds) {
        const memberDeclared = allDeclaredRects.get(memberId);
        const natural = memberId === id ? initialRect : registry.getRect(memberId);
        let rect: DispositionPosition | undefined = interaction.getTransform(memberId);
        if (!rect) {
          if (flowLayout && natural) {
            rect = flowDispositionManipulationRect(natural, undefined);
          } else if (natural) {
            rect = memberDeclared !== undefined ? natural : flowDispositionManipulationRect(natural, undefined);
          } else {
            rect = memberDeclared;
          }
        } else if (flowLayout && natural) {
          rect = isFlowPixelOffsetTransform(rect, natural) ? rect : flowDispositionManipulationRect(natural, undefined);
        }
        if (rect && (flowLayout || memberDeclared !== undefined || isUsableMeasuredRect(rect))) {
          startRects.set(memberId, rect);
        }
      }
      if (startRects.size === 0) {
        startRects.set(id, initialRect);
      }
      const startGroup = mode !== "move" && groupIds.length > 1 ? groupBoundingRect([...startRects.values()]) : null;
      let dragging = !requireDragThreshold;
      setGesturing(true);
      const flowTransformEl = flowDragTransformElement(section, rootRef.current, contentRef.current);
      if (flowLayout) {
        const root = rootRef.current;
        if (root && root.offsetWidth > 0 && root.offsetHeight > 0) {
          setFlowReservePx({ width: root.offsetWidth, height: root.offsetHeight });
        }
      }

      const onMove = (moveEvent: PointerEvent): void => {
        if (moveEvent.pointerId !== pointerId) {
          return;
        }
        if (!dragging) {
          if (Math.hypot(moveEvent.clientX - startClient.x, moveEvent.clientY - startClient.y) < POINTER_DRAG_THRESHOLD_PX) {
            return;
          }
          dragging = true;
        }
        moveEvent.preventDefault();
        const updates = new Map<string, DispositionTransform>();
        if (mode === "move") {
          for (const [memberId, rect] of startRects) {
            if (flowLayout) {
              const { dx, dy } = flowPointerDeltaToLocal(flowTransformEl, startClient.x, startClient.y, moveEvent.clientX, moveEvent.clientY);
              updates.set(memberId, {
                x: rect.x + dx,
                y: rect.y + dy,
                width: rect.width,
                height: rect.height,
              });
            } else {
              const container = dispositionPlacementContainer(section, allDeclaredRects.get(memberId) !== undefined);
              const bounds = container === fractionContainer && gesturePlacementBounds.width > 0 ? gesturePlacementBounds : container.getBoundingClientRect();
              const dx = bounds.width > 0 ? (moveEvent.clientX - startClient.x) / bounds.width : 0;
              const dy = bounds.height > 0 ? (moveEvent.clientY - startClient.y) / bounds.height : 0;
              updates.set(memberId, translateDispositionRect(rect, dx, dy));
            }
          }
        } else {
          const bounds = gesturePlacementBounds.width > 0 ? gesturePlacementBounds : fractionContainer.getBoundingClientRect();
          const dx = bounds.width > 0 ? (moveEvent.clientX - startClient.x) / bounds.width : 0;
          const dy = bounds.height > 0 ? (moveEvent.clientY - startClient.y) / bounds.height : 0;
          if (startGroup && groupIds.length > 1) {
            const resizedGroup = resizeDispositionRect(startGroup, mode, dx, dy);
            for (const [memberId, rect] of startRects) {
              updates.set(memberId, scaleRectWithinGroup(rect, startGroup, resizedGroup));
            }
          } else {
            const startRect = startRects.get(id) ?? initialRect;
            updates.set(id, resizeDispositionRect(startRect, mode, dx, dy));
          }
        }
        interaction.setTransforms(updates);
      };

      const onUp = (upEvent: PointerEvent): void => {
        if (upEvent.pointerId !== pointerId) {
          return;
        }
        if (dragging) {
          suppressRevealOverviewSlideNavigation(upEvent);
        }
        setGesturing(false);
        window.removeEventListener("pointermove", onMove);
        window.removeEventListener("pointerup", onUp);
        window.removeEventListener("pointercancel", onUp);
        try {
          origin.captureEl?.releasePointerCapture?.(pointerId);
        } catch {}
      };

      try {
        origin.captureEl?.setPointerCapture?.(pointerId);
      } catch {}
      window.addEventListener("pointermove", onMove);
      window.addEventListener("pointerup", onUp);
      window.addEventListener("pointercancel", onUp);
    },
    [id, flowLayout, canvasPlacement, allDeclaredRects, interaction, registry, sectionRef],
  );

  const seedCanvasTransform = useCallback((): DispositionPosition | null => {
    if (!interactionRect) {
      return null;
    }
    const existing = interaction.getTransform(id);
    if (existing) {
      return existing;
    }
    interaction.setTransform(id, interactionRect);
    return interactionRect;
  }, [id, interactionRect, interaction]);

  const onPointerDown = (event: React.PointerEvent): void => {
    if (event.button !== 0) {
      return;
    }
    const target = event.target instanceof Element ? event.target : null;
    if (target?.closest(".presentation-interaction-handle")) {
      return;
    }
    if (target?.closest(".presentation-interaction-enlarge")) {
      return;
    }
    if (target?.closest(".presentation-interaction-reset")) {
      return;
    }
    if (target?.closest(".presentation-pdf-page-nav")) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const additive = event.shiftKey;
    const dragMemberIds = resolveDispositionDragGroupIds(id, selected, additive, interaction.selectedIds);
    if (!selected) {
      interaction.selectIds([id], additive);
    } else if (additive) {
      interaction.selectIds([id], true);
    }
    seedCanvasTransform();
    const rect = ensureRectForManipulation("move");
    if (!rect) {
      return;
    }
    attachPointerGesture(
      {
        pointerId: event.pointerId,
        clientX: event.clientX,
        clientY: event.clientY,
        captureEl: rootRef.current,
      },
      "move",
      rect,
      true,
      dragMemberIds,
    );
  };

  const onHandlePointerDown = (handle: DispositionResizeHandle) => (event: React.PointerEvent) => {
    if (event.button !== 0) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const dragMemberIds = resolveDispositionDragGroupIds(id, selected, false, interaction.selectedIds);
    if (!selected) {
      interaction.selectIds([id], false);
    }
    seedCanvasTransform();
    const rect = ensureRectForManipulation("resize");
    if (!rect) {
      return;
    }
    attachPointerGesture(
      {
        pointerId: event.pointerId,
        clientX: event.clientX,
        clientY: event.clientY,
        captureEl: event.currentTarget as HTMLElement,
      },
      handle,
      rect,
      false,
      dragMemberIds,
    );
  };

  const onEnlargeClick = (event: React.MouseEvent): void => {
    event.preventDefault();
    event.stopPropagation();
    if (!selected) {
      interaction.selectIds([id], false);
    }
    if (enlarged) {
      const stashed = interaction.getEnlargeStash(id);
      interaction.toggleEnlarge(id);
      interaction.clearEnlargeStash(id);
      if (stashed) {
        if (interactionRect && !dispositionPositionChanged(interactionRect, stashed)) {
          interaction.clearTransform(id);
        } else {
          interaction.setTransform(id, stashed);
        }
      }
      return;
    }
    const preEnlargeRect = effectiveRect ?? interactionRect;
    if (preEnlargeRect) {
      interaction.stashEnlargeRect(id, preEnlargeRect);
    }
    interaction.toggleEnlarge(id);
  };

  const onResetClick = (event: React.MouseEvent): void => {
    event.preventDefault();
    event.stopPropagation();
    if (!selected) {
      interaction.selectIds([id], false);
    }
    if (enlarged) {
      const stashed = interaction.getEnlargeStash(id);
      interaction.toggleEnlarge(id);
      interaction.clearEnlargeStash(id);
      if (stashed && interactionRect && !dispositionPositionChanged(interactionRect, stashed)) {
        interaction.clearTransform(id);
      } else if (stashed) {
        interaction.setTransform(id, stashed);
      } else {
        interaction.clearTransform(id);
      }
      return;
    }
    interaction.clearTransform(id);
  };

  const morphCropFrom = disposition.revealMorphCompanion === "target" && disposition.revealMorphFromFrame !== undefined && disposition.position !== undefined;
  const morphCropTo = disposition.position !== undefined && (disposition.revealMorphFromMorphToFrame !== undefined || disposition.revealMorphToFrame !== undefined);
  const wrapperClass = [
    "presentation-interactive-disposition",
    `presentation-interactive-disposition--kind-${disposition.embodiment.kind}`,
    selected ? "presentation-interactive-disposition--selected" : undefined,
    flowPixelOffset ? "presentation-interactive-disposition--offset" : undefined,
    pinned ? "presentation-interactive-disposition--pinned" : undefined,
    canvasFramed ? "presentation-interactive-disposition--canvas-framed" : undefined,
    gesturing ? "presentation-interactive-disposition--gesturing" : undefined,
    enlarged ? "presentation-interactive-disposition--enlarged" : undefined,
    disposition.revealMorphCompanion === "target" ? "presentation-target-ghost" : undefined,
    disposition.revealMorphCompanion === "source" ? "presentation-source-ghost" : undefined,
    morphCropFrom ? "presentation-morph-crop-from" : undefined,
    morphCropTo ? "presentation-morph-crop-to" : undefined,
    (disposition.morphFrom?.length ?? 0) > 0 ? "presentation-morph-target" : undefined,
    (disposition.morphTo?.length ?? 0) > 0 ? "presentation-morph-one" : undefined,
  ]
    .filter(Boolean)
    .join(" ");

  const flowInkActive = useFlowInkFrame && (selected || gesturing || flowPixelOffset);
  const wrapperFrame: CSSProperties = flowInkActive ? { position: "relative" } : {};
  if (flowReservePx) {
    Object.assign(wrapperFrame, flowDispositionReserveStyle(flowReservePx));
  }
  const flowDragOffsetStyle = flowPixelOffset && transform ? flowDispositionOffsetStyle(transform) : undefined;
  if (flowDragOffsetStyle) {
    Object.assign(wrapperFrame, flowDragOffsetStyle);
  }
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

  return (
    <div
      ref={rootRef}
      data-disposition-id={id}
      {...(revealMorphId ? { "data-id": revealMorphId } : {})}
      {...(rowBandId ? { "data-row-band": rowBandId } : {})}
      className={wrapperClass}
      style={wrapperStyle}
      onPointerDown={morphGhost ? undefined : onPointerDown}
      aria-hidden={morphGhost ? true : undefined}
    >
      <div ref={contentRef} className="presentation-interactive-disposition__content" style={hasContentStyle ? contentStyle : undefined}>
        <PresentationInteractiveDispositionIdContext.Provider value={id}>
          <PresentationDispositionEnlargeContext.Provider value={enlarged}>
            <MorphAnchorOnWrapperContext.Provider value={Boolean(revealMorphId && declaredRect !== undefined)}>
              <PresentationFigureCropFrameContext.Provider value={canvasLiveTransform ?? displayDisposition.position}>
                <MorphDispositionView disposition={displayDisposition} />
              </PresentationFigureCropFrameContext.Provider>
            </MorphAnchorOnWrapperContext.Provider>
          </PresentationDispositionEnlargeContext.Provider>
        </PresentationInteractiveDispositionIdContext.Provider>
        {showHandles ? (
          <div className="presentation-interactive-disposition__chrome" style={chromeStyle} aria-hidden>
            {DISPOSITION_RESIZE_HANDLES.map((handle) => (
              <div key={handle} className={`presentation-interaction-handle presentation-interaction-handle--${handle}`} onPointerDown={onHandlePointerDown(handle)} />
            ))}
          </div>
        ) : null}
      </div>
      {showControls ? (
        <div className="presentation-interaction-actions">
          {canResetPosition ? (
            <button type="button" className="presentation-interaction-reset" title="Reset position" onClick={onResetClick}>
              <Icon icon="rotate-ccw" size="small" />
            </button>
          ) : null}
          <button type="button" className="presentation-interaction-enlarge" title={enlarged ? "Exit enlarge" : "Enlarge"} aria-pressed={enlarged} onClick={onEnlargeClick}>
            <Icon icon="maximize-2" size="small" />
          </button>
        </div>
      ) : null}
    </div>
  );
};
//#endregion 🔖️Interaction

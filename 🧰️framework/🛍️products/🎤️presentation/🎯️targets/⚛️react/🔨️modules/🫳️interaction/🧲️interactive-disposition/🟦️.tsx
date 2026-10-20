import type { DispositionPosition } from "@semio-tech/presentation";
import { Icon, type FC, type RefObject } from "@semio-tech/ui-react";
import { type RevealResolvedDisposition } from "../../🔀️reveal-morph/🟦️.ts";
import { MorphAnchorOnWrapperContext, PresentationDispositionEnlargeContext, PresentationFigureCropFrameContext, PresentationInteractiveDispositionIdContext } from "../../🏷️morph-context/🟦️.ts";
import { MorphDispositionView } from "../../🎭️morph-views/🟦️.tsx";
import { DISPOSITION_RESIZE_HANDLES, transformFrameStyle, usePresentationInteractionState } from "../../🧮️interaction-model/🟦️.ts";
import { useSlideEpochState } from "./🔶️slide-epoch/🟦️.tsx";
import { useAttachPointerGestureState } from "./🔸️attach-pointer-gesture/🟦️.tsx";
import { buildPointerDownState } from "./🟠️pointer-down/🟦️.tsx";
import { buildWrapperStyleState } from "./🟢️wrapper-style/🟦️.tsx";

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
  const { canvasPlacement, registry, interaction, flowLayout, setGesturing, rootRef, contentRef, setFlowReservePx, interactionRect, selected, ensureRectForManipulation, enlarged, effectiveRect, flowPixelOffset, pinned, canvasFramed, gesturing, useFlowInkFrame, flowReservePx, transform, canvasAnchorRect, canvasLiveTransform, transformed, morphGhost, measuredNatural, enlargedFlowContentScale, flowSectionFrame, inkInWrapper, displayDisposition } = useSlideEpochState({ id, sectionDeclaredRect, declaredRect, disposition, sectionRef });
  const { seedCanvasTransform, attachPointerGesture } = useAttachPointerGestureState({ sectionRef, canvasPlacement, id, allDeclaredRects, registry, interaction, flowLayout, setGesturing, rootRef, contentRef, setFlowReservePx, interactionRect });
  const { wrapperFrame, morphCropFrom, wrapperClass, onPointerDown, onHandlePointerDown, onResetClick, onEnlargeClick } = buildPointerDownState({ id, selected, interaction, seedCanvasTransform, ensureRectForManipulation, attachPointerGesture, rootRef, enlarged, interactionRect, effectiveRect, disposition, flowPixelOffset, pinned, canvasFramed, gesturing, useFlowInkFrame, flowReservePx, transform });
  const { wrapperStyle, hasContentStyle, contentStyle, showHandles, chromeStyle, showControls, canResetPosition } = buildWrapperStyleState({ canvasFramed, canvasAnchorRect, disposition, canvasLiveTransform, wrapperFrame, morphCropFrom, transformed, transform, flowPixelOffset, morphGhost, enlarged, measuredNatural, interactionRect, enlargedFlowContentScale, flowSectionFrame, effectiveRect, sectionRef, useFlowInkFrame, inkInWrapper, selected, gesturing, flowLayout });
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

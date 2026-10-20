/** 🟠️ The pointer down phase of InteractiveDisposition. */
import { type CSSProperties } from "@semio-tech/ui-react";
import { type RevealResolvedDisposition } from "../../../🔀️reveal-morph/🟦️.ts";
import { dispositionPositionChanged, type DispositionResizeHandle, flowDispositionOffsetStyle, flowDispositionReserveStyle, resolveDispositionDragGroupIds } from "../../../🧮️interaction-model/🟦️.ts";
import type { useSlideEpochState } from "../🔶️slide-epoch/🟦️.tsx";
import type { useAttachPointerGestureState } from "../🔸️attach-pointer-gesture/🟦️.tsx";

/** 🟡 Builds onPointerDown, onHandlePointerDown, onEnlargeClick, onResetClick of InteractiveDisposition. */
export function buildPointerDownState({ id, selected, interaction, seedCanvasTransform, ensureRectForManipulation, attachPointerGesture, rootRef, enlarged, interactionRect, effectiveRect, disposition, flowPixelOffset, pinned, canvasFramed, gesturing, useFlowInkFrame, flowReservePx, transform }: { readonly id: string; readonly disposition: RevealResolvedDisposition } & Pick<ReturnType<typeof useSlideEpochState>, "selected" | "interaction" | "ensureRectForManipulation" | "rootRef" | "enlarged" | "interactionRect" | "effectiveRect" | "flowPixelOffset" | "pinned" | "canvasFramed" | "gesturing" | "useFlowInkFrame" | "flowReservePx" | "transform"> & Pick<ReturnType<typeof useAttachPointerGestureState>, "seedCanvasTransform" | "attachPointerGesture">) {
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
  return { wrapperFrame, morphCropFrom, wrapperClass, onPointerDown, onHandlePointerDown, onResetClick, onEnlargeClick };
}

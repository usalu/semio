/** 🔸️ The attach pointer gesture phase of InteractiveDisposition. */
import type { DispositionPosition } from "@semio-tech/presentation";
import { useCallback, type RefObject } from "@semio-tech/ui-react";
import { dispositionPlacementContainer, type DispositionResizeHandle, type DispositionTransform, flowDispositionManipulationRect, flowDragTransformElement, flowPointerDeltaToLocal, groupBoundingRect, isFlowPixelOffsetTransform, isUsableMeasuredRect, POINTER_DRAG_THRESHOLD_PX, resizeDispositionRect, scaleRectWithinGroup, suppressRevealOverviewSlideNavigation, translateDispositionRect } from "../../../🧮️interaction-model/🟦️.ts";
import type { useSlideEpochState } from "../🔶️slide-epoch/🟦️.tsx";

/** 🔹 Builds attachPointerGesture, seedCanvasTransform of InteractiveDisposition. */
export function useAttachPointerGestureState({ sectionRef, canvasPlacement, id, allDeclaredRects, registry, interaction, flowLayout, setGesturing, rootRef, contentRef, setFlowReservePx, interactionRect }: { readonly sectionRef: RefObject<HTMLElement | null>; readonly id: string; readonly allDeclaredRects: ReadonlyMap<string, DispositionPosition | undefined> } & Pick<ReturnType<typeof useSlideEpochState>, "canvasPlacement" | "registry" | "interaction" | "flowLayout" | "setGesturing" | "rootRef" | "contentRef" | "setFlowReservePx" | "interactionRect">) {
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
  return { seedCanvasTransform, attachPointerGesture };
}

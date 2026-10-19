import type * as React from "react";
import { reactHostPort } from "../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";

/** 🖱️ Pointer-drag props for a host element (replaces imperative drag controllers). */
export function usePointerDrag<TElement extends HTMLElement = HTMLDivElement>(handlers: {
  onStart?: (event: React.PointerEvent<TElement>) => void;
  onMove?: (event: React.PointerEvent<TElement>) => void;
  onEnd?: (event: React.PointerEvent<TElement>) => void;
  onCancel?: (event: React.PointerEvent<TElement>) => void;
}): Pick<React.HTMLAttributes<TElement>, "onPointerCancel" | "onPointerDown" | "onPointerMove" | "onPointerUp"> {
  const activePointerIdRef = reactHostPort.useRef<number | null>(null);
  return reactHostPort.useMemo(
    () => ({
      onPointerDown: (event: React.PointerEvent<TElement>) => {
        activePointerIdRef.current = event.pointerId;
        event.currentTarget.setPointerCapture(event.pointerId);
        handlers.onStart?.(event);
      },
      onPointerMove: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        handlers.onMove?.(event);
      },
      onPointerUp: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        activePointerIdRef.current = null;
        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
          event.currentTarget.releasePointerCapture(event.pointerId);
        }
        handlers.onEnd?.(event);
      },
      onPointerCancel: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        activePointerIdRef.current = null;
        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
          event.currentTarget.releasePointerCapture(event.pointerId);
        }
        handlers.onCancel?.(event);
      },
    }),
    [handlers],
  );
}


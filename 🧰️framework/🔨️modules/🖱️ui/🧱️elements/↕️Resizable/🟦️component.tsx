// #region 🧲️Header
// 💻️ framework/ui/elements/↕️Resizable/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as ResizablePrimitive from "react-resizable-panels";
import { domSizePx } from "@semio-tech/ui-styling";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🪬️Resizable

/** @emoji ↔ Fine-pointer hit target for splitters and corner joins (VS Code–style). */
export const RESIZABLE_HIT_TARGET_MIN_FINE_PX = 20;

/** @emoji ↔ Coarse-pointer hit target for splitters and corner joins. */
export const RESIZABLE_HIT_TARGET_MIN_COARSE_PX = 28;

/** @emoji ↔ Corner grab square at perpendicular split intersections. */
export const RESIZABLE_CORNER_GRAB_PX = domSizePx("resizableCornerGrabUiSpacing");

const RESIZABLE_HIT_TARGET_MINIMUM_SIZE = {
  fine: RESIZABLE_HIT_TARGET_MIN_FINE_PX,
  coarse: RESIZABLE_HIT_TARGET_MIN_COARSE_PX,
} as const;

export type ResizableJoinCornerResizeHandler = (spec: ResizableJoinCornerSpec, deltaXPx: number, deltaYPx: number) => void;

type ResizableJoinCornerElement = HTMLDivElement & {
  __composeResizableJoinCornerResize?: ResizableJoinCornerResizeHandler;
};

type ResizableCornerWindow = Window &
  typeof globalThis & {
    __composeResizableCornerInterceptorV2?: boolean;
  };

export function readResizableJoinCornerSpec(element: HTMLElement): ResizableJoinCornerSpec | null {
  const raw = element.dataset.joinSpec;
  if (!raw) return null;
  try {
    return JSON.parse(raw) as ResizableJoinCornerSpec;
  } catch {
    return null;
  }
}

function writeResizableJoinCornerSpec(element: HTMLElement, spec: ResizableJoinCornerSpec): void {
  element.dataset.joinSpec = JSON.stringify(spec);
}

//#region ↔ResizableCornerInterceptor

/** @emoji ↔ Window capture hook so corner grabs win over react-resizable-panels and survive hot reloads. */
function installResizableCornerInterceptor(): void {
  if (typeof window === "undefined") return;
  const interceptorWindow = window as ResizableCornerWindow;
  if (interceptorWindow.__composeResizableCornerInterceptorV2) return;
  interceptorWindow.__composeResizableCornerInterceptorV2 = true;
  let drag: { pointerId: number; x: number; y: number; corner: ResizableJoinCornerElement; spec: ResizableJoinCornerSpec } | null = null;
  const onPointerMove = (event: PointerEvent) => {
    if (!drag || drag.pointerId !== event.pointerId) return;
    const deltaXPx = event.clientX - drag.x;
    const deltaYPx = event.clientY - drag.y;
    if (deltaXPx !== 0 || deltaYPx !== 0) {
      drag.corner.__composeResizableJoinCornerResize?.(drag.spec, deltaXPx, deltaYPx);
      drag = { ...drag, x: event.clientX, y: event.clientY };
    }
  };
  const endDrag = (event: PointerEvent) => {
    if (!drag || drag.pointerId !== event.pointerId) return;
    drag = null;
    document.body.style.removeProperty("cursor");
    window.removeEventListener("pointermove", onPointerMove, true);
    window.removeEventListener("pointerup", endDrag, true);
    window.removeEventListener("pointercancel", endDrag, true);
  };
  window.addEventListener(
    "pointerdown",
    (event) => {
      if (event.button !== 0) return;
      const corner = event.target instanceof Element ? event.target.closest<ResizableJoinCornerElement>('[data-slot="resizable-corner"]') : null;
      if (!corner) return;
      const spec = readResizableJoinCornerSpec(corner);
      if (!spec || !corner.__composeResizableJoinCornerResize) return;
      event.preventDefault();
      event.stopImmediatePropagation();
      drag = { pointerId: event.pointerId, x: event.clientX, y: event.clientY, corner, spec };
      document.body.style.cursor = "move";
      window.addEventListener("pointermove", onPointerMove, true);
      window.addEventListener("pointerup", endDrag, true);
      window.addEventListener("pointercancel", endDrag, true);
    },
    true,
  );
}

installResizableCornerInterceptor();

//#endregion ↔ResizableCornerInterceptor

/** @emoji ↔ Corner grab at a perpendicular split intersection (dual-axis resize). */
export type ResizableJoinEdgeSide = "leading" | "trailing";

/** @emoji ↔ Corner join wiring for a main-axis separator and a cross-axis child split. */
export interface ResizableJoinCornerSpec {
  parentKind: "row" | "column";
  mainAxisPath: string;
  mainSeparatorIndex: number;
  crossAxisPath: string;
  crossSeparatorIndex: number;
  edgeSide: ResizableJoinEdgeSide;
  alongFraction: number;
}

/** @emoji ↔ Pixel placement for a corner grab on a separator strip. */
export function resizableJoinCornerPlacementStyle(orientation: "horizontal" | "vertical", edgeSide: ResizableJoinEdgeSide, alongFraction: number, sizePx = RESIZABLE_CORNER_GRAB_PX): React.CSSProperties {
  const along = `${Math.round(Math.min(1, Math.max(0, alongFraction)) * 100)}%`;
  if (orientation === "horizontal") {
    return {
      top: along,
      [edgeSide === "leading" ? "left" : "right"]: 0,
      width: sizePx,
      height: sizePx,
      transform: `translate(${edgeSide === "leading" ? "-50%" : "50%"}, -50%)`,
    };
  }
  return {
    left: along,
    [edgeSide === "leading" ? "top" : "bottom"]: 0,
    width: sizePx,
    height: sizePx,
    transform: `translate(-50%, ${edgeSide === "leading" ? "-50%" : "50%"})`,
  };
}

function ResizableJoinCornerGrab({
  orientation,
  edgeSide,
  alongFraction,
  spec,
  onResize,
}: {
  orientation: "horizontal" | "vertical";
  edgeSide: ResizableJoinEdgeSide;
  alongFraction: number;
  spec: ResizableJoinCornerSpec;
  onResize: ResizableJoinCornerResizeHandler;
}) {
  const elementRef = reactHostPort.useRef<ResizableJoinCornerElement>(null);
  reactHostPort.useLayoutEffect(() => {
    installResizableCornerInterceptor();
    const element = elementRef.current;
    if (!element) return;
    writeResizableJoinCornerSpec(element, spec);
    element.__composeResizableJoinCornerResize = onResize;
    return () => {
      if (element.__composeResizableJoinCornerResize === onResize) delete element.__composeResizableJoinCornerResize;
    };
  }, [onResize, spec]);

  return <div ref={elementRef} data-slot="resizable-corner" data-edge={edgeSide} className="absolute z-50 cursor-move touch-none" style={resizableJoinCornerPlacementStyle(orientation, edgeSide, alongFraction)} />;
}

function ResizablePanelGroup({ className, orientation = "horizontal", resizeTargetMinimumSize = RESIZABLE_HIT_TARGET_MINIMUM_SIZE, defaultLayout, ...props }: React.ComponentProps<typeof ResizablePrimitive.Group>) {
  // react-resizable-panels Layout is Record<panelId, number>; reject/omit mismatched array leftovers from callers.
  const safeDefaultLayout =
    defaultLayout == null
      ? undefined
      : Array.isArray(defaultLayout)
        ? undefined
        : typeof defaultLayout === "object"
          ? defaultLayout
          : undefined;
  return (
    <ResizablePrimitive.Group
      data-slot="resizable-panel-group"
      data-panel-group-direction={orientation}
      className={cn("flex h-full w-full", orientation === "vertical" ? "flex-col" : "flex-row", className)}
      orientation={orientation}
      resizeTargetMinimumSize={resizeTargetMinimumSize}
      defaultLayout={safeDefaultLayout}
      {...props}
    />
  );
}

/**
 * ResizablePanel holds the data fields for a ResizablePanel record.
 **/
function ResizablePanel({ ...props }: React.ComponentProps<typeof ResizablePrimitive.Panel>) {
  return <ResizablePrimitive.Panel data-slot="resizable-panel" {...props} />;
}

function ResizableHandle({
  className,
  orientation = "horizontal",
  joinCorners,
  onJoinCornerResize,
  style,
  ...props
}: React.ComponentProps<typeof ResizablePrimitive.Separator> & {
  orientation?: "horizontal" | "vertical";
  joinCorners?: readonly ResizableJoinCornerSpec[];
  onJoinCornerResize?: ResizableJoinCornerResizeHandler;
}) {
  const horizontal = orientation === "horizontal";

  return (
    <ResizablePrimitive.Separator
      data-slot="resizable-handle"
      data-resize-orientation={orientation}
      className={cn(
        "relative flex shrink-0 items-center justify-center border-0 bg-transparent",
        horizontal ? "h-full min-h-0 w-single cursor-ew-resize" : "w-full min-w-0 h-single cursor-ns-resize",
        "data-[separator=hover]:bg-accent/25 data-[separator=active]:bg-accent/25",
        "focus-visible:ring-ring focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none",
        "after:hidden",
        className,
      )}
      style={{
        ...(horizontal ? { width: "var(--spacing-single)" } : { height: "var(--spacing-single)" }),
        ...style,
      }}
      {...(props as any)}
    >
      {onJoinCornerResize
        ? joinCorners?.map((spec) => (
            <ResizableJoinCornerGrab
              key={`${spec.mainAxisPath}-${spec.mainSeparatorIndex}-${spec.crossAxisPath}-${spec.crossSeparatorIndex}-${spec.edgeSide}-${spec.alongFraction}`}
              alongFraction={spec.alongFraction}
              edgeSide={spec.edgeSide}
              onResize={onJoinCornerResize}
              orientation={orientation}
              spec={spec}
            />
          ))
        : null}
    </ResizablePrimitive.Separator>
  );
}

export { ResizableHandle, ResizablePanel, ResizablePanelGroup };

// #endregion 🪬️Resizable

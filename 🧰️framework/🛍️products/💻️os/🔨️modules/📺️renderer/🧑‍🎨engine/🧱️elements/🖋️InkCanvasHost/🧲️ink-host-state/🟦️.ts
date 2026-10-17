/** 🧲️ InkCanvasHost state shapes: the drag, text edit and table edit state types and the host constants. */
import { type SelectionMarqueePoint } from "@semio-tech/ui-react";
import type { InkBounds, InkCamera, InkResizeHandle } from "../🧾️ink-model/🟦️.ts";

//#region 🔖️InkCanvasHost
//#region DragState
export type InkDragState =
  | { readonly kind: "pan"; readonly startX: number; readonly startY: number; readonly camera: InkCamera }
  | { readonly kind: "move"; readonly origins: Readonly<Record<string, { readonly x: number; readonly y: number }>>; readonly startX: number; readonly startY: number }
  | { readonly kind: "marquee"; readonly start: SelectionMarqueePoint }
  | { readonly kind: "stroke"; readonly blockId: string }
  | { readonly kind: "eraser"; readonly mode: "eraserStroke" | "eraserPoint" }
  | { readonly kind: "resize"; readonly handle: InkResizeHandle; readonly fromBounds: InkBounds; readonly startX: number; readonly startY: number; readonly selectedIds: readonly string[] };

export type InkTextEditState = { readonly blockId: string; readonly created?: boolean };
export type InkTableEditState = { readonly blockId: string; readonly row: number; readonly col: number };

const INK_MARQUEE_THRESHOLD_PX = 4;

//#endregion DragState
//#region InkCanvasHost
/** 🔍️ The ink camera's zoom limits — the wheel and the shared-recognizer pinch clamp to ONE range. */
export const INK_CAMERA_ZOOM_BOUNDS = { min: 0.1, max: 8 } as const;

/** 🪜️ The ink surface's root class. `isolate` keeps its selection frame (`z-20`), resize handles, block editors
 * and inline text editor (`z-30`) inside its own stacking context, so a selected block under a window chip can never
 * paint over the window chrome (ticket 26/09/23 S15; same law as `NODE_GRAPH_HOST_CLASS`). */
export const INK_CANVAS_HOST_CLASS = "isolate relative h-full w-full touch-none overflow-hidden outline-none";
//#endregion InkCanvasHost
//#endregion 🔖️InkCanvasHost

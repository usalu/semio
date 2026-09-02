/** 🖼️ Drawing viewer — Canvas window: typed twin of `🦀️.rs`'s view-model. Read-only mirror of
 * the 2D canvas scene payload `render()` produces — no mutation-shaped fields (no active utility, no
 * gesture/engagement session), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ The Canvas window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `DrawingSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface DrawingViewCanvasViewModel {
  windowKindId: "drawing-view-canvas";
  bodyKey: "drawing.view.canvas";
  surfaceId: "drawing.view.composite";
}

export const DRAWING_VIEW_CANVAS_WINDOW_KIND_ID = "drawing-view-canvas" as const;
export const DRAWING_VIEW_CANVAS_BODY_KEY = "drawing.view.canvas" as const;
export const DRAWING_VIEW_CANVAS_SURFACE_ID = "drawing.view.composite" as const;

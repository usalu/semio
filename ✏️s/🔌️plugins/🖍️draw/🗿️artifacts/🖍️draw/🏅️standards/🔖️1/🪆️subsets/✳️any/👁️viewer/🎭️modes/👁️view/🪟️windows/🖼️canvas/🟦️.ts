/** 🖼️ Draw viewer — Canvas window: typed twin of `🦀️.rs`'s view-model. Read-only mirror of
 * the 2D canvas scene payload `render()` produces — no mutation-shaped fields (no active utility, no
 * gesture/engagement session), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ The Canvas window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `DrawSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface DrawViewCanvasViewModel {
  windowKindId: "draw-view-canvas";
  bodyKey: "draw.view.canvas";
  surfaceId: "draw.view.composite";
}

export const DRAW_VIEW_CANVAS_WINDOW_KIND_ID = "draw-view-canvas" as const;
export const DRAW_VIEW_CANVAS_BODY_KEY = "draw.view.canvas" as const;
export const DRAW_VIEW_CANVAS_SURFACE_ID = "draw.view.composite" as const;

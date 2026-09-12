import type { Viewport2d } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 🖼️ Drawing editor — Canvas window: typed twin of `🦀️.rs`'s view-model. Mirrors the pane's
 * `render(document: &DrawingSnapshot, config: &DrawingCanvasWindowConfig, gesture: &drawing_gesture::Snapshot,
 * active_utility: &str)` boundary — the 2D canvas scene payload plus the live gesture/utility session
 * state a mutation-capable surface carries (absent entirely from the viewer's read-only twin, see
 * `👁️viewer/…/🟦️.ts`). */

/** ✏️ The Canvas window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface DrawingCanvasViewModel {
  windowKindId: "drawing-composite";
  bodyKey: "drawing.play.composite";
  surfaceId: "drawing.play.composite";
  camera: Viewport2d;
  activeUtilityId: string;
}

export const DRAWING_PLAY_WINDOW_CANVAS = "drawing-composite" as const;
export const DRAWING_PLAY_BODY_COMPOSITE = "drawing.play.composite" as const;
export const DRAWING_PLAY_SURFACE_ID = "drawing.play.composite" as const;

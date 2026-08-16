/** 🖼️ Draw editor — Canvas window: typed twin of `🦀️component.rs`'s view-model. Mirrors the pane's
 * `render(document: &DrawSnapshot, config: &DrawConfig, gesture: &draw_gesture::Snapshot,
 * active_utility: &str)` boundary — the 2D canvas scene payload plus the live gesture/utility session
 * state a mutation-capable surface carries (absent entirely from the viewer's read-only twin, see
 * `👁️viewer/…/🟦️component.ts`). */

/** ✏️ Session camera pose (pan + zoom) — mirrors Rust `DrawCamera`. */
export interface DrawCameraViewModel {
  x: number;
  y: number;
  zoom: number;
}

/** ✏️ The Canvas window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface DrawCanvasViewModel {
  windowKindId: "draw-composite";
  bodyKey: "draw.play.composite";
  surfaceId: "draw.play.composite";
  camera: DrawCameraViewModel;
  activeUtilityId: string;
}

export const DRAW_PLAY_WINDOW_CANVAS = "draw-composite" as const;
export const DRAW_PLAY_BODY_COMPOSITE = "draw.play.composite" as const;
export const DRAW_PLAY_SURFACE_ID = "draw.play.composite" as const;

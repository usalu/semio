/** 📷️ Canvas2dHost camera math: the zoom bounds and wheel factors, the pinch step, the clamp and the world and screen conversions shared with `Paint2dHost`. */
import { applyPinchToCamera, type PinchStep } from "@semio-tech/framework";

//#region 🔖️Canvas2dHost
//#region CanvasCameraMath
export const CANVAS_CAMERA_ZOOM_MIN = 0.05;
export const CANVAS_CAMERA_ZOOM_MAX = 32;
const WHEEL_ZOOM_IN_FACTOR = 1.1;
const WHEEL_ZOOM_OUT_FACTOR = 0.9;

export type CanvasCamera = {
  x: number;
  y: number;
  zoom: number;
};

/** 🔍️ The canvas camera's zoom limits as `👆️gesture` bounds — the pinch and the wheel clamp to ONE range. */
export const CANVAS_CAMERA_ZOOM_BOUNDS = { min: CANVAS_CAMERA_ZOOM_MIN, max: CANVAS_CAMERA_ZOOM_MAX } as const;

/** 🤏️ One shared-recognizer pinch step on a canvas camera (`Canvas2dHost`, `Paint2dHost`): the finger-anchored
 * `applyPinchToCamera` law in this camera's own centred transform and zoom limits. */
export function canvasPinchCamera(camera: CanvasCamera, step: PinchStep, viewportWidth: number, viewportHeight: number): CanvasCamera {
  const next = applyPinchToCamera(camera, step, { w: viewportWidth, h: viewportHeight }, CANVAS_CAMERA_ZOOM_BOUNDS);
  return { x: next.x, y: next.y, zoom: next.zoom };
}

export function clampCanvasZoom(zoom: number): number {
  return Math.min(CANVAS_CAMERA_ZOOM_MAX, Math.max(CANVAS_CAMERA_ZOOM_MIN, zoom));
}

/** 🧭️ Maps world coordinates to logical (CSS-pixel) screen space — matches `infinite_canvas::camera::world_to_screen`. */
export function worldToScreenLogical(worldX: number, worldY: number, camera: CanvasCamera, viewportWidth: number, viewportHeight: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom || 1;
  return {
    x: (worldX - camera.x) * zoom + viewportWidth * 0.5,
    y: (worldY - camera.y) * zoom + viewportHeight * 0.5,
  };
}

/** 🧭️ Maps logical (CSS-pixel) screen space to world coordinates — matches `infinite_canvas::camera::screen_to_world`. */
export function screenToWorldLogical(screenX: number, screenY: number, camera: CanvasCamera, viewportWidth: number, viewportHeight: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom || 1;
  return {
    x: (screenX - viewportWidth * 0.5) / zoom + camera.x,
    y: (screenY - viewportHeight * 0.5) / zoom + camera.y,
  };
}

/** 🔍️ Cursor-anchored wheel zoom — matches `infinite_canvas::camera::wheel_screen`. */
export function wheelCameraAtScreen(camera: CanvasCamera, screenX: number, screenY: number, deltaY: number, viewportWidth: number, viewportHeight: number): CanvasCamera {
  const zoomFactor = deltaY < 0 ? WHEEL_ZOOM_IN_FACTOR : WHEEL_ZOOM_OUT_FACTOR;
  const nextZoom = clampCanvasZoom((camera.zoom || 1) * zoomFactor);
  const worldBefore = screenToWorldLogical(screenX, screenY, camera, viewportWidth, viewportHeight);
  return {
    x: worldBefore.x - (screenX - viewportWidth * 0.5) / nextZoom,
    y: worldBefore.y - (screenY - viewportHeight * 0.5) / nextZoom,
    zoom: nextZoom,
  };
}
//#endregion CanvasCameraMath
//#endregion 🔖️Canvas2dHost

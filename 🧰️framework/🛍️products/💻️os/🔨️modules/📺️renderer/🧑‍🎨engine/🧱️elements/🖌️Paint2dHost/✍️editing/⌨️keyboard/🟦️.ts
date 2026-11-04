/** ⌨️ Keyboard operation of the pixel tools: a pixel cursor, marquee, stamping, camera keys and the stroke length limit. */
import { CANVAS_CAMERA_ZOOM_MAX, CANVAS_CAMERA_ZOOM_MIN, type CanvasCamera } from "../../../📐️Canvas2dHost/📷️camera-math/🟦️.ts";

export type PixelCursor = { readonly x: number; readonly y: number };
export type PixelExtent = { readonly width: number; readonly height: number };
export type PixelStrokePoint = readonly [number, number];

/** 🌊️ Points a pointer or keyboard stroke buffers before it streams the buffer as one history-neutral segment. */
export const STROKE_POINT_CAP = 2048;
/** ⚠️ The count from which the user is told the stroke is nearing the segment boundary. */
export const STROKE_WARNING_POINTS = Math.floor(STROKE_POINT_CAP * 0.9);

const ARROWS: Readonly<Record<string, readonly [number, number]>> = { ArrowLeft: [-1, 0], ArrowRight: [1, 0], ArrowUp: [0, -1], ArrowDown: [0, 1] };
const ZOOM_FACTOR = 1.25;

/** 🧭️ Moves the cursor by `stride` pixels along an arrow key and clamps it to the image; other keys answer `null`. */
export function stepKeyboardCursor(cursor: PixelCursor, key: string, extent: PixelExtent, stride = 1): PixelCursor | null {
  const direction = ARROWS[key];
  if (!direction) return null;
  const clamp = (value: number, size: number) => Math.min(Math.max(0, size - 1), Math.max(0, value));
  return { x: clamp(cursor.x + direction[0] * stride, extent.width), y: clamp(cursor.y + direction[1] * stride, extent.height) };
}

/** 🖌️ A stamp is a two-point stroke through the pixel center, the exact payload shape a pointer click produces. */
export function keyboardStampPath(cursor: PixelCursor): readonly [PixelStrokePoint, PixelStrokePoint] {
  const point: PixelStrokePoint = [cursor.x + 0.5, cursor.y + 0.5];
  return [point, point];
}

/** ▭ The inclusive rectangle spanned by the anchor pixel and the cursor pixel. */
export function keyboardMarquee(anchor: PixelCursor, cursor: PixelCursor): { kind: "rectangle"; x: number; y: number; width: number; height: number } {
  const x = Math.min(anchor.x, cursor.x), y = Math.min(anchor.y, cursor.y);
  return { kind: "rectangle", x, y, width: Math.abs(anchor.x - cursor.x) + 1, height: Math.abs(anchor.y - cursor.y) + 1 };
}

/** 📣️ The live-region text for the cursor, one-based and localized. */
export function cursorAnnouncement(locale: "en" | "de", cursor: PixelCursor, extent: PixelExtent): string {
  return locale === "de"
    ? `Pixel Spalte ${cursor.x + 1} von ${extent.width}, Zeile ${cursor.y + 1} von ${extent.height}`
    : `Pixel column ${cursor.x + 1} of ${extent.width}, row ${cursor.y + 1} of ${extent.height}`;
}

/** 🔍️ Zoom (`+`, `=`, `-`), reset (`0`) and pan (arrows, `screenStride` screen pixels) when no tool owns the arrows. */
export function keyboardCameraStep(camera: CanvasCamera, key: string, screenStride = 40): CanvasCamera | null {
  const zoom = (value: number) => Math.min(CANVAS_CAMERA_ZOOM_MAX, Math.max(CANVAS_CAMERA_ZOOM_MIN, value));
  if (key === "+" || key === "=") return { ...camera, zoom: zoom(camera.zoom * ZOOM_FACTOR) };
  if (key === "-") return { ...camera, zoom: zoom(camera.zoom / ZOOM_FACTOR) };
  if (key === "0") return { ...camera, zoom: 1 };
  const direction = ARROWS[key];
  return direction ? { ...camera, x: camera.x + (direction[0] * screenStride) / camera.zoom, y: camera.y + (direction[1] * screenStride) / camera.zoom } : null;
}

/** 🚦️ How close a buffered stroke is to its segment boundary. */
export function strokeLimitState(count: number): "ok" | "warn" | "full" {
  return count >= STROKE_POINT_CAP ? "full" : count >= STROKE_WARNING_POINTS ? "warn" : "ok";
}

/** ✂️ At the cap, hands out the unsent points as one stream batch and restarts the buffer at the last point so the gesture stays one history step. */
export function rolloverStroke(points: readonly PixelStrokePoint[], sent: number, cap = STROKE_POINT_CAP): { batch: PixelStrokePoint[]; points: PixelStrokePoint[]; sent: number } | null {
  if (points.length < cap) return null;
  return { batch: points.slice(sent), points: [points[points.length - 1]!], sent: 1 };
}

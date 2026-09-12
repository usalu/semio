import { viewportNumber, viewportRecord, viewportZoom } from "../../🟦️.ts";

/** 🪟️ Navigation retained by one concrete two-dimensional window. */
export interface Viewport2d { x: number; y: number; zoom: number }

/** 📥️ Admits the closed two-dimensional navigation schema. */
export function parseViewport2d(value: unknown): Viewport2d {
  const record = viewportRecord(value, ["x", "y", "zoom"]);
  return { x: viewportNumber(record.x), y: viewportNumber(record.y), zoom: viewportZoom(record.zoom) };
}

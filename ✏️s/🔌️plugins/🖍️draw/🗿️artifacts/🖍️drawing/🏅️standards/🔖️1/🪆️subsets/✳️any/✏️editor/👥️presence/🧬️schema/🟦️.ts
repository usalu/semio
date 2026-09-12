import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 👥️ Shareable live Drawing presence. */
export interface DrawingPresence {
  engagementInput: string;
  camera: Viewport2d;
}

/** 🚪️ Parses one exact Drawing presence value. */
export function parseDrawingPresence(value: unknown): DrawingPresence {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  const keys = Object.keys(row);
  if (keys.length !== 2 || !keys.includes("engagementInput") || !keys.includes("camera")) throw new TypeError("$ must contain only engagementInput and camera");
  if (typeof row.engagementInput !== "string") throw new TypeError("$.engagementInput must be a string");
  return { engagementInput: row.engagementInput, camera: parseViewport2d(row.camera) };
}

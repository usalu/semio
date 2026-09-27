import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 🎚️ Persisted navigation for one exact Drawing Canvas window. */
export interface DrawingCanvasWindowConfig {
  viewport: Viewport2d;
  framed: boolean;
}

/** 🧬️ Whole-record Drawing Canvas window configuration mutation. */
export type DrawingCanvasWindowConfigMutation = { kind: "snapshot"; config: DrawingCanvasWindowConfig };

/** 🚪️ Parses one exact Drawing Canvas window configuration. */
export function parseDrawingCanvasWindowConfig(value: unknown): DrawingCanvasWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  const keys = Object.keys(row);
  if (keys.length !== 2 || keys.some(key => key !== "viewport" && key !== "framed") || typeof row.framed !== "boolean") throw new TypeError("$ must contain viewport and framed");
  return { viewport: parseViewport2d(row.viewport),framed: row.framed };
}

/** 🔁️ Applies one exact Drawing Canvas configuration mutation. */
export function applyDrawingCanvasWindowConfigMutation(_base: DrawingCanvasWindowConfig, mutation: DrawingCanvasWindowConfigMutation): DrawingCanvasWindowConfig {
  return parseDrawingCanvasWindowConfig(mutation.config);
}

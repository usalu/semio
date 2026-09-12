/** 🫧️ Ephemeral interaction state for one exact Drawing Canvas window. */
export interface DrawingCanvasWindowTransient {
  engagementInput: string;
  tracePointerGeneration: number;
  tracePointerCompletedWork: number;
  tracePointerPendingWork: number;
}

/** 🧬️ Whole-record Drawing Canvas window transient mutation. */
export type DrawingCanvasWindowTransientMutation = { kind: "snapshot"; transient: DrawingCanvasWindowTransient };

/** 🚪️ Parses one exact Drawing Canvas transient value. */
export function parseDrawingCanvasWindowTransient(value: unknown): DrawingCanvasWindowTransient {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  const keys = ["engagementInput", "tracePointerGeneration", "tracePointerCompletedWork", "tracePointerPendingWork"] as const;
  if (Object.keys(row).length !== keys.length || keys.some((key) => !(key in row))) throw new TypeError("$ must contain the exact Drawing Canvas transient fields");
  if (typeof row.engagementInput !== "string") throw new TypeError("$.engagementInput must be a string");
  for (const key of keys.slice(1)) if (!Number.isSafeInteger(row[key]) || (row[key] as number) < 0) throw new TypeError(`$.${key} must be a non-negative safe integer`);
  return row as unknown as DrawingCanvasWindowTransient;
}

/** 🔁️ Applies one exact Drawing Canvas transient mutation. */
export function applyDrawingCanvasWindowTransientMutation(_base: DrawingCanvasWindowTransient, mutation: DrawingCanvasWindowTransientMutation): DrawingCanvasWindowTransient {
  return parseDrawingCanvasWindowTransient(mutation.transient);
}

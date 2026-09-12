/** 🪟️ Validates a declared viewport record at its native/UI protocol boundary. */
export function viewportRecord(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("Expected viewport object");
  const record = value as Record<string, unknown>;
  if (required.some(key => !Object.hasOwn(record, key)) || Object.keys(record).some(key => !required.includes(key) && !optional.includes(key))) throw new TypeError("Unknown or missing viewport field");
  return record;
}

/** 📐️ Admits finite coordinates without coercion. */
export function viewportNumber(value: unknown): number {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError("Expected finite viewport coordinate");
  return value;
}

/** 🔎️ Admits a positive finite zoom. */
export function viewportZoom(value: unknown): number {
  const zoom = viewportNumber(value);
  if (zoom <= 0) throw new TypeError("Expected positive viewport zoom");
  return zoom;
}

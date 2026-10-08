import schema from "./🧬️schema/🔣️.json";

export interface PixelSelectionSpanV1 { start: number; length: number; coverage: number }
export interface PixelLayerSelectionV1 { layerId: string; target: "pixels" | "mask"; width: number; height: number; spans: PixelSelectionSpanV1[] }

/** 🎯️ Admits finite ordered coverage without allocating an image-sized mask. */
export function parsePixelLayerSelectionV1(value: unknown, at = "$"): PixelLayerSelectionV1 {
  const refuse = (why: string): never => { throw new Error(`${at}: ${why}`); };
  const object = (input: unknown, keys: readonly string[]): Record<string, unknown> => {
    if (!input || typeof input !== "object" || Array.isArray(input)) return refuse("selection is not an object");
    const row = input as Record<string, unknown>;
    if (Object.keys(row).some(key => !keys.includes(key))) return refuse("unexpected field");
    return row;
  };
  const integer = (input: unknown, min: number, max: number): number => Number.isSafeInteger(input) && (input as number) >= min && (input as number) <= max ? input as number : refuse("integer outside selection bounds");
  const row = object(value, Object.keys(schema.properties));
  if (typeof row.layerId !== "string" || [...row.layerId].length < schema.properties.layerId.minLength || [...row.layerId].length > schema.properties.layerId.maxLength) return refuse("invalid layer identity");
  if (row.target !== "pixels" && row.target !== "mask") return refuse("invalid selection target");
  const width = integer(row.width, 1, schema["x-semio-admission"].maxDimension), height = integer(row.height, 1, schema["x-semio-admission"].maxDimension), count = width * height;
  if (count > schema["x-semio-admission"].maxPixels) return refuse("selection exceeds pixel budget");
  if (!Array.isArray(row.spans) || row.spans.length > schema["x-semio-admission"].maxSpans) return refuse("selection exceeds span budget");
  let previous = 0;
  const spans = row.spans.map(value => {
    const span = object(value, Object.keys(schema.$defs.PixelSelectionSpanV1.properties)), start = integer(span.start, previous, count - 1), length = integer(span.length, 1, count - start), coverage = integer(span.coverage, 0, 255);
    previous = start + length;
    return { start, length, coverage };
  });
  return { layerId: row.layerId, target: row.target, width, height, spans };
}

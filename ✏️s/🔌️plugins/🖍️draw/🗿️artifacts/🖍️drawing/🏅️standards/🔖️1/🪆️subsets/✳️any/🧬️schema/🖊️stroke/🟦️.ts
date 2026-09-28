export type StrokeCap = "butt" | "round" | "square";
export type StrokeJoin = "miter" | "round" | "bevel";

/** 🎩️ Reject unsupported line caps at the document boundary. */
export function parseStrokeCap(value: unknown): StrokeCap {
  if (value === "butt" || value === "round" || value === "square") return value;
  throw new Error("Invalid stroke cap");
}

/** 📐️ Reject unsupported line joins at the document boundary. */
export function parseStrokeJoin(value: unknown): StrokeJoin {
  if (value === "miter" || value === "round" || value === "bevel") return value;
  throw new Error("Invalid stroke join");
}

/** 🖊️ Parse a bounded sequence of nonnegative lengths separated by spaces. */
export function parseStrokeDash(value: string): number[] | null {
  if (value.length > 128 || !/^[ \t\r\n]*(?:(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[ \t\r\n]+(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+))*)?[ \t\r\n]*$/.test(value)) throw new Error("Invalid dash pattern");
  const dash = value.trim() ? value.trim().split(/[ \t\r\n]+/).map(Number) : [];
  return dash.some(length => length > 0) ? dash : null;
}

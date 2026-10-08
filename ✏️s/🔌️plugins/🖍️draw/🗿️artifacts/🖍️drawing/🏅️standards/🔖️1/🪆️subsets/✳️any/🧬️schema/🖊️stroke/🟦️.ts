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


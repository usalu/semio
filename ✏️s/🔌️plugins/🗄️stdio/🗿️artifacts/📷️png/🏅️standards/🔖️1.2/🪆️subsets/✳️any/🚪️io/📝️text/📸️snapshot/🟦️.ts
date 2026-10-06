/** 📝️ Logical PNG Record text preserves arbitrary schema and octets. */
export type PngSnapshotText = string;
/** 🚪️ Admits the declared textual carrier without parsing a raw PNG file. */
export function parsePngSnapshotText(value: unknown, at = "$" ): PngSnapshotText {
  if (typeof value !== "string") throw new TypeError(`${at}: value is not text`);
  return value;
}

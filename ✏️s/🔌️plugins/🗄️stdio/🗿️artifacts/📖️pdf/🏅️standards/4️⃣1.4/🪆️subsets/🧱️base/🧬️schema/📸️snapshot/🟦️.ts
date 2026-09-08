/** 🧬️ PdfSnapshot (1.4) schema — the document's real page tree, mirroring the Rust
 *  `PdfSnapshot` shape 1:1. `width`/`height` are the page's /MediaBox extent; `text` is its shown
 *  text (the operand bytes of the text-showing operators, not font-decoded). */
export interface PageDoc {
  width: number;
  height: number;
  text: string;
}
export interface PdfSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ pages: PageDoc[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf14BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf14BaseSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioPdf14BaseSnapshotGuardRefusal(at, why);
};

type stdioPdf14BaseSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf14BaseSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf14BaseSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf14BaseSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf14BaseSnapshotGuardReject(at, "value is not an object");
export const stdioPdf14BaseSnapshotGuardArray = (value: unknown, at: string, bounds: stdioPdf14BaseSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf14BaseSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf14BaseSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf14BaseSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf14BaseSnapshotGuardString = (value: unknown, at: string, bounds: stdioPdf14BaseSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf14BaseSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf14BaseSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf14BaseSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf14BaseSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf14BaseSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf14BaseSnapshotGuardReject(at, "value is not a boolean"));
export const stdioPdf14BaseSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioPdf14BaseSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf14BaseSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf14BaseSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf14BaseSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf14BaseSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioPdf14BaseSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf14BaseSnapshotGuardNumber(value, at, bounds) : stdioPdf14BaseSnapshotGuardReject(at, "value is not an integer");
export const stdioPdf14BaseSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf14BaseSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf14BaseSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf14BaseSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdfSnapshot(value: unknown, at = "$"): PdfSnapshot {
  const row = stdioPdf14BaseSnapshotGuardObject(value, at);
  return {
    schema: stdioPdf14BaseSnapshotGuardString(row["schema"], `${at}.schema`),
    pages: stdioPdf14BaseSnapshotGuardArray(row["pages"], `${at}.pages`).map((item, index) => parsePageDoc(item, `${at}.pages[${index}]`)),
  };
}

export function parsePageDoc(value: unknown, at = "$"): PageDoc {
  const row = stdioPdf14BaseSnapshotGuardObject(value, at);
  return {
    width: stdioPdf14BaseSnapshotGuardNumber(row["width"], `${at}.width`),
    height: stdioPdf14BaseSnapshotGuardNumber(row["height"], `${at}.height`),
    text: stdioPdf14BaseSnapshotGuardString(row["text"], `${at}.text`),
  };
}

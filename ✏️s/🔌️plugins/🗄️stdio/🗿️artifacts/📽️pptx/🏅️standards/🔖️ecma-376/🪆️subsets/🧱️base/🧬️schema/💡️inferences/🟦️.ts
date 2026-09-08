/** 💡️ Pptx inference schema — document outline (slide/shape/word counts). */

export interface PptxOutline {
  slideCount: number;
  shapeCount: number;
  wordCount: number;
}

export interface PptxInference {
  /** @derived */
  outline: PptxOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPptxEcma376BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPptxEcma376BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioPptxEcma376BaseInferenceGuardRefusal(at, why);
};

type stdioPptxEcma376BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPptxEcma376BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPptxEcma376BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPptxEcma376BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPptxEcma376BaseInferenceGuardReject(at, "value is not an object");
export const stdioPptxEcma376BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPptxEcma376BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPptxEcma376BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPptxEcma376BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPptxEcma376BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPptxEcma376BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPptxEcma376BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPptxEcma376BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPptxEcma376BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPptxEcma376BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPptxEcma376BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioPptxEcma376BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPptxEcma376BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPptxEcma376BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPptxEcma376BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPptxEcma376BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPptxEcma376BaseInferenceGuardNumber(value, at, bounds) : stdioPptxEcma376BaseInferenceGuardReject(at, "value is not an integer");
export const stdioPptxEcma376BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPptxEcma376BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPptxEcma376BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPptxEcma376BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePptxInference(value: unknown, at = "$"): PptxInference {
  const row = stdioPptxEcma376BaseInferenceGuardObject(value, at);
  return {
    outline: parsePptxOutline(row["outline"], `${at}.outline`),
  };
}

export function parsePptxOutline(value: unknown, at = "$"): PptxOutline {
  const row = stdioPptxEcma376BaseInferenceGuardObject(value, at);
  return {
    slideCount: stdioPptxEcma376BaseInferenceGuardInteger(row["slideCount"], `${at}.slideCount`),
    shapeCount: stdioPptxEcma376BaseInferenceGuardInteger(row["shapeCount"], `${at}.shapeCount`),
    wordCount: stdioPptxEcma376BaseInferenceGuardInteger(row["wordCount"], `${at}.wordCount`),
  };
}

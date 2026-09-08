/** 🧬️ SemioTextArtifact schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export type SemioTextMarkKind = "bold" | "italic" | "code" | "link";
export interface SemioTextMark {
  kind: SemioTextMarkKind;
  href: string;
}
export interface SemioTextRun {
  language: string;
  content: string;
  marks: SemioTextMark[];
}
export interface SemioTextArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ runs: SemioTextRun[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TextArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TextArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TextArtifactGuardRefusal(at, why);
};

type stdioSemioV1TextArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TextArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TextArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TextArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TextArtifactGuardReject(at, "value is not an object");
export const stdioSemioV1TextArtifactGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TextArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TextArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TextArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TextArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TextArtifactGuardString = (value: unknown, at: string, bounds: stdioSemioV1TextArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TextArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TextArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TextArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TextArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TextArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TextArtifactGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TextArtifactGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TextArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TextArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TextArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TextArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TextArtifactGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TextArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TextArtifactGuardNumber(value, at, bounds) : stdioSemioV1TextArtifactGuardReject(at, "value is not an integer");
export const stdioSemioV1TextArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TextArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TextArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TextArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioTextArtifact(value: unknown, at = "$"): SemioTextArtifact {
  const row = stdioSemioV1TextArtifactGuardObject(value, at);
  return {
    schema: stdioSemioV1TextArtifactGuardString(row["schema"], `${at}.schema`),
    runs: stdioSemioV1TextArtifactGuardArray(row["runs"], `${at}.runs`).map((item, index) => stdioSemioV1TextArtifactGuardObject(item, `${at}.runs[${index}]`)),
  };
}

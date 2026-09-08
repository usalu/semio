/** 🧬️ WavDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the WavDiff
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavDiffEntry {
  key: string;
  value: string;
}
export interface WavDiff {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: WavDiffEntry[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioWavRiffpcmAnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioWavRiffpcmAnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioWavRiffpcmAnyDiffGuardRefusal(at, why);
};

type stdioWavRiffpcmAnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioWavRiffpcmAnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioWavRiffpcmAnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioWavRiffpcmAnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioWavRiffpcmAnyDiffGuardReject(at, "value is not an object");
export const stdioWavRiffpcmAnyDiffGuardArray = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioWavRiffpcmAnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioWavRiffpcmAnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioWavRiffpcmAnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioWavRiffpcmAnyDiffGuardString = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioWavRiffpcmAnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioWavRiffpcmAnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioWavRiffpcmAnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioWavRiffpcmAnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioWavRiffpcmAnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioWavRiffpcmAnyDiffGuardReject(at, "value is not a boolean"));
export const stdioWavRiffpcmAnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioWavRiffpcmAnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioWavRiffpcmAnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioWavRiffpcmAnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioWavRiffpcmAnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioWavRiffpcmAnyDiffGuardNumber(value, at, bounds) : stdioWavRiffpcmAnyDiffGuardReject(at, "value is not an integer");
export const stdioWavRiffpcmAnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioWavRiffpcmAnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioWavRiffpcmAnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioWavRiffpcmAnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWavDiff(value: unknown, at = "$"): WavDiff {
  const row = stdioWavRiffpcmAnyDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioWavRiffpcmAnyDiffGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioWavRiffpcmAnyDiffGuardArray(row["entries"], `${at}.entries`).map((item, index) => stdioWavRiffpcmAnyDiffGuardObject(item, `${at}.entries[${index}]`)),
  };
}

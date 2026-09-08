/** 🧬️ WavArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the WavArtifact
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavArtifactEntry {
  key: string;
  value: string;
}
export interface WavArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: WavArtifactEntry[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioWavRiffpcmAnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioWavRiffpcmAnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioWavRiffpcmAnyArtifactGuardRefusal(at, why);
};

type stdioWavRiffpcmAnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioWavRiffpcmAnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioWavRiffpcmAnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioWavRiffpcmAnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioWavRiffpcmAnyArtifactGuardReject(at, "value is not an object");
export const stdioWavRiffpcmAnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioWavRiffpcmAnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioWavRiffpcmAnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioWavRiffpcmAnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioWavRiffpcmAnyArtifactGuardString = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioWavRiffpcmAnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioWavRiffpcmAnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioWavRiffpcmAnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioWavRiffpcmAnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioWavRiffpcmAnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioWavRiffpcmAnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioWavRiffpcmAnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioWavRiffpcmAnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioWavRiffpcmAnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioWavRiffpcmAnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioWavRiffpcmAnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioWavRiffpcmAnyArtifactGuardNumber(value, at, bounds) : stdioWavRiffpcmAnyArtifactGuardReject(at, "value is not an integer");
export const stdioWavRiffpcmAnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioWavRiffpcmAnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioWavRiffpcmAnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioWavRiffpcmAnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWavArtifact(value: unknown, at = "$"): WavArtifact {
  const row = stdioWavRiffpcmAnyArtifactGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioWavRiffpcmAnyArtifactGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioWavRiffpcmAnyArtifactGuardArray(row["entries"], `${at}.entries`).map((item, index) => stdioWavRiffpcmAnyArtifactGuardObject(item, `${at}.entries[${index}]`)),
  };
}

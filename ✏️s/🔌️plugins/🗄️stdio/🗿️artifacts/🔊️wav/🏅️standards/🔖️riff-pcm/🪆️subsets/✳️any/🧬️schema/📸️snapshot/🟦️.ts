/** 🧬️ WavSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the WavSnapshot
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavSnapshotEntry {
  key: string;
  value: string;
}
export interface WavSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: WavSnapshotEntry[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioWavRiffpcmAnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioWavRiffpcmAnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioWavRiffpcmAnySnapshotGuardRefusal(at, why);
};

type stdioWavRiffpcmAnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioWavRiffpcmAnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioWavRiffpcmAnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioWavRiffpcmAnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioWavRiffpcmAnySnapshotGuardReject(at, "value is not an object");
export const stdioWavRiffpcmAnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioWavRiffpcmAnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioWavRiffpcmAnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioWavRiffpcmAnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioWavRiffpcmAnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioWavRiffpcmAnySnapshotGuardString = (value: unknown, at: string, bounds: stdioWavRiffpcmAnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioWavRiffpcmAnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioWavRiffpcmAnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioWavRiffpcmAnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioWavRiffpcmAnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioWavRiffpcmAnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioWavRiffpcmAnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioWavRiffpcmAnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioWavRiffpcmAnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioWavRiffpcmAnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioWavRiffpcmAnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioWavRiffpcmAnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioWavRiffpcmAnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioWavRiffpcmAnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioWavRiffpcmAnySnapshotGuardNumber(value, at, bounds) : stdioWavRiffpcmAnySnapshotGuardReject(at, "value is not an integer");
export const stdioWavRiffpcmAnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioWavRiffpcmAnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioWavRiffpcmAnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioWavRiffpcmAnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWavSnapshot(value: unknown, at = "$"): WavSnapshot {
  const row = stdioWavRiffpcmAnySnapshotGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioWavRiffpcmAnySnapshotGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioWavRiffpcmAnySnapshotGuardArray(row["entries"], `${at}.entries`).map((item, index) => stdioWavRiffpcmAnySnapshotGuardObject(item, `${at}.entries[${index}]`)),
  };
}

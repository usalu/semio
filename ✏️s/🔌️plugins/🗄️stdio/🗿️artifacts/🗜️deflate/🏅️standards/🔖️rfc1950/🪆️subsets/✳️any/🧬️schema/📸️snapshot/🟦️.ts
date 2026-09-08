/** 🧬️ DeflateSnapshot schema — typed RFC1950 zlib container. */
export type DeflateLevelHint = "fastest" | "fast" | "default" | "maximum";

export interface DeflateSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact — CMF low nibble (CM); RFC1950 defines only 8 (deflate) */
  compressionMethod: number;
  /** @state artifact — CMF high nibble (CINFO); window = 2^(cinfo+8) */
  windowBits: number;
  /** @state artifact — FLG.FLEVEL */
  compressionLevelHint: DeflateLevelHint;
  /** @state artifact — FLG.FDICT + DICTID; present only when a preset dictionary is declared */
  dictId?: number;
  /** @state artifact — decompressed payload */
  payload: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDeflateRfc1950AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDeflateRfc1950AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioDeflateRfc1950AnySnapshotGuardRefusal(at, why);
};

type stdioDeflateRfc1950AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDeflateRfc1950AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDeflateRfc1950AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDeflateRfc1950AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDeflateRfc1950AnySnapshotGuardReject(at, "value is not an object");
export const stdioDeflateRfc1950AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDeflateRfc1950AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDeflateRfc1950AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDeflateRfc1950AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDeflateRfc1950AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDeflateRfc1950AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDeflateRfc1950AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDeflateRfc1950AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDeflateRfc1950AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDeflateRfc1950AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDeflateRfc1950AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioDeflateRfc1950AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDeflateRfc1950AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDeflateRfc1950AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDeflateRfc1950AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDeflateRfc1950AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDeflateRfc1950AnySnapshotGuardNumber(value, at, bounds) : stdioDeflateRfc1950AnySnapshotGuardReject(at, "value is not an integer");
export const stdioDeflateRfc1950AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDeflateRfc1950AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDeflateRfc1950AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDeflateRfc1950AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDeflateSnapshot(value: unknown, at = "$"): DeflateSnapshot {
  const row = stdioDeflateRfc1950AnySnapshotGuardObject(value, at);
  return {
    schema: stdioDeflateRfc1950AnySnapshotGuardString(row["schema"], `${at}.schema`),
    compressionMethod: stdioDeflateRfc1950AnySnapshotGuardInteger(row["compressionMethod"], `${at}.compressionMethod`, {"minimum": 0, "maximum": 15}),
    windowBits: stdioDeflateRfc1950AnySnapshotGuardInteger(row["windowBits"], `${at}.windowBits`, {"minimum": 0, "maximum": 15}),
    compressionLevelHint: stdioDeflateRfc1950AnySnapshotGuardMember(row["compressionLevelHint"], `${at}.compressionLevelHint`, ["fastest", "fast", "default", "maximum"] as const),
    dictId: row["dictId"] === undefined ? undefined : stdioDeflateRfc1950AnySnapshotGuardInteger(row["dictId"], `${at}.dictId`, {"minimum": 0, "maximum": 4294967295}),
    payload: stdioDeflateRfc1950AnySnapshotGuardArray(row["payload"], `${at}.payload`).map((item, index) => stdioDeflateRfc1950AnySnapshotGuardInteger(item, `${at}.payload[${index}]`, {"minimum": 0, "maximum": 255})),
  };
}

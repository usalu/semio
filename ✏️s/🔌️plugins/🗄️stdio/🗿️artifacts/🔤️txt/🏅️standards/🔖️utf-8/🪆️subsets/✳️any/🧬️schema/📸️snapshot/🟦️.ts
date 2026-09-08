/** ⏎️ Which newline sequence terminates each line. */
export type LineEnding = 'lf' | 'crLf';

/** 🧬️ TxtSnapshot schema — a text file as a sequence of lines. */
export interface TxtSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ lines: string[];
  /** @state artifact */ trailingNewline: boolean;
  /** @state artifact */ lineEnding: LineEnding;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTxtUtf8AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTxtUtf8AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioTxtUtf8AnySnapshotGuardRefusal(at, why);
};

type stdioTxtUtf8AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTxtUtf8AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTxtUtf8AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTxtUtf8AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTxtUtf8AnySnapshotGuardReject(at, "value is not an object");
export const stdioTxtUtf8AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTxtUtf8AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTxtUtf8AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTxtUtf8AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTxtUtf8AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTxtUtf8AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTxtUtf8AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTxtUtf8AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTxtUtf8AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTxtUtf8AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTxtUtf8AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioTxtUtf8AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTxtUtf8AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTxtUtf8AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTxtUtf8AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTxtUtf8AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTxtUtf8AnySnapshotGuardNumber(value, at, bounds) : stdioTxtUtf8AnySnapshotGuardReject(at, "value is not an integer");
export const stdioTxtUtf8AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTxtUtf8AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTxtUtf8AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTxtUtf8AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTxtSnapshot(value: unknown, at = "$"): TxtSnapshot {
  const row = stdioTxtUtf8AnySnapshotGuardObject(value, at);
  return {
    schema: stdioTxtUtf8AnySnapshotGuardString(row["schema"], `${at}.schema`),
    lines: stdioTxtUtf8AnySnapshotGuardArray(row["lines"], `${at}.lines`).map((item, index) => stdioTxtUtf8AnySnapshotGuardString(item, `${at}.lines[${index}]`)),
    trailingNewline: stdioTxtUtf8AnySnapshotGuardBoolean(row["trailingNewline"], `${at}.trailingNewline`),
    lineEnding: stdioTxtUtf8AnySnapshotGuardMember(row["lineEnding"], `${at}.lineEnding`, ["lf", "crLf"] as const),
  };
}

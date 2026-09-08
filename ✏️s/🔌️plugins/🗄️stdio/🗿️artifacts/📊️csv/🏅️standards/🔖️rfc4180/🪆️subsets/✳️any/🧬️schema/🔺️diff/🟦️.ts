/** 🔺️ CsvDiff schema facet — mirrors 🦀️.rs field-for-field. No full-replace slot:
 * every field is a sparse patch, `records` is an index-keyed removed/modified/added triple. */

export interface CsvFieldDiff {
  value?: string;
  quoted?: boolean;
}

/** Positional per-field patch list; `null` at a position means that field is unchanged. */
export interface CsvRecordDiff {
  fields?: (CsvFieldDiff | null)[];
}

export interface CsvRecordModified {
  index: number;
  diff: CsvRecordDiff;
}

export interface CsvRecordAdded {
  index: number;
  record: import('../📸️snapshot/🟦️.ts').CsvRecord;
}

export interface CsvRecordsDiff {
  removed?: number[];
  modified?: CsvRecordModified[];
  added?: CsvRecordAdded[];
}

export interface CsvDiff {
  hasHeader?: boolean;
  records?: CsvRecordsDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioCsvRfc4180AnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioCsvRfc4180AnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioCsvRfc4180AnyDiffGuardRefusal(at, why);
};

type stdioCsvRfc4180AnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioCsvRfc4180AnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioCsvRfc4180AnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioCsvRfc4180AnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioCsvRfc4180AnyDiffGuardReject(at, "value is not an object");
export const stdioCsvRfc4180AnyDiffGuardArray = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioCsvRfc4180AnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioCsvRfc4180AnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioCsvRfc4180AnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioCsvRfc4180AnyDiffGuardString = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioCsvRfc4180AnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioCsvRfc4180AnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioCsvRfc4180AnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioCsvRfc4180AnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioCsvRfc4180AnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioCsvRfc4180AnyDiffGuardReject(at, "value is not a boolean"));
export const stdioCsvRfc4180AnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioCsvRfc4180AnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioCsvRfc4180AnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioCsvRfc4180AnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioCsvRfc4180AnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioCsvRfc4180AnyDiffGuardNumber(value, at, bounds) : stdioCsvRfc4180AnyDiffGuardReject(at, "value is not an integer");
export const stdioCsvRfc4180AnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioCsvRfc4180AnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioCsvRfc4180AnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioCsvRfc4180AnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCsvDiff(value: unknown, at = "$"): CsvDiff {
  const row = stdioCsvRfc4180AnyDiffGuardObject(value, at);
  return {
    hasHeader: row["hasHeader"] === undefined ? undefined : stdioCsvRfc4180AnyDiffGuardBoolean(row["hasHeader"], `${at}.hasHeader`),
    records: row["records"] === undefined ? undefined : parseCsvRecordsDiff(row["records"], `${at}.records`),
  };
}

export function parseCsvFieldDiff(value: unknown, at = "$"): CsvFieldDiff {
  const row = stdioCsvRfc4180AnyDiffGuardObject(value, at);
  return {
    value: row["value"] === undefined ? undefined : stdioCsvRfc4180AnyDiffGuardString(row["value"], `${at}.value`),
    quoted: row["quoted"] === undefined ? undefined : stdioCsvRfc4180AnyDiffGuardBoolean(row["quoted"], `${at}.quoted`),
  };
}

export function parseCsvRecordModified(value: unknown, at = "$"): CsvRecordModified {
  const row = stdioCsvRfc4180AnyDiffGuardObject(value, at);
  return {
    index: stdioCsvRfc4180AnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseCsvRecordDiff(row["diff"], `${at}.diff`),
  };
}

export function parseCsvRecordsDiff(value: unknown, at = "$"): CsvRecordsDiff {
  const row = stdioCsvRfc4180AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioCsvRfc4180AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioCsvRfc4180AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioCsvRfc4180AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseCsvRecordModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioCsvRfc4180AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseCsvRecordAdded(item, `${at}.added[${index}]`)),
  };
}

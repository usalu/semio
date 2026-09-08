import type { ZipEntry } from '../📸️snapshot/🟦️.ts';

export interface ZipEntryDiff { name?: string; data?: number[]; }
export interface ZipEntryModified { name: string; diff: ZipEntryDiff; }
export interface ZipEntriesDiff { removed: string[]; modified: ZipEntryModified[]; added: ZipEntry[]; }
export interface ZipDiff { comment?: string; entries?: ZipEntriesDiff; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioZip20BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioZip20BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioZip20BaseDiffGuardRefusal(at, why);
};

type stdioZip20BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioZip20BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioZip20BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioZip20BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioZip20BaseDiffGuardReject(at, "value is not an object");
export const stdioZip20BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioZip20BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioZip20BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioZip20BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioZip20BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioZip20BaseDiffGuardString = (value: unknown, at: string, bounds: stdioZip20BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioZip20BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioZip20BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioZip20BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioZip20BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioZip20BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioZip20BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioZip20BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioZip20BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioZip20BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioZip20BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioZip20BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioZip20BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioZip20BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioZip20BaseDiffGuardNumber(value, at, bounds) : stdioZip20BaseDiffGuardReject(at, "value is not an integer");
export const stdioZip20BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioZip20BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioZip20BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioZip20BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseZipDiff(value: unknown, at = "$"): ZipDiff {
  const row = stdioZip20BaseDiffGuardObject(value, at);
  return {
    comment: row["comment"] === undefined ? undefined : stdioZip20BaseDiffGuardString(row["comment"], `${at}.comment`),
    entries: row["entries"] === undefined ? undefined : parseZipEntriesDiff(row["entries"], `${at}.entries`),
  };
}

export interface ZipEntry {
  readonly name: string;
  readonly data: readonly number[];
}

export function parseZipEntry(value: unknown, at = "$"): ZipEntry {
  const row = stdioZip20BaseDiffGuardObject(value, at);
  return {
    name: stdioZip20BaseDiffGuardString(row["name"], `${at}.name`),
    data: stdioZip20BaseDiffGuardArray(row["data"], `${at}.data`).map((item, index) => stdioZip20BaseDiffGuardInteger(item, `${at}.data[${index}]`)),
  };
}

export function parseZipEntryDiff(value: unknown, at = "$"): ZipEntryDiff {
  const row = stdioZip20BaseDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : stdioZip20BaseDiffGuardString(row["name"], `${at}.name`),
    data: row["data"] === undefined ? undefined : stdioZip20BaseDiffGuardArray(row["data"], `${at}.data`).map((item, index) => stdioZip20BaseDiffGuardInteger(item, `${at}.data[${index}]`)),
  };
}

export function parseZipEntryModified(value: unknown, at = "$"): ZipEntryModified {
  const row = stdioZip20BaseDiffGuardObject(value, at);
  return {
    name: stdioZip20BaseDiffGuardString(row["name"], `${at}.name`),
    diff: parseZipEntryDiff(row["diff"], `${at}.diff`),
  };
}

export function parseZipEntriesDiff(value: unknown, at = "$"): ZipEntriesDiff {
  const row = stdioZip20BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioZip20BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioZip20BaseDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioZip20BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseZipEntryModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioZip20BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseZipEntry(item, `${at}.added[${index}]`)),
  };
}

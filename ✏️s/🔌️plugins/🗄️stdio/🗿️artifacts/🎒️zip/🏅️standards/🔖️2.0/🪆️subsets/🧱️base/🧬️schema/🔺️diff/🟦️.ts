import { parseZipEntry as parseSnapshotZipEntry, parseZipEntryMetadata, type ZipSnapshot, type ZipEntry, type ZipEntryMetadata } from '../📸️snapshot/🟦️.ts';
export type { ZipEntry, ZipEntryMetadata } from '../📸️snapshot/🟦️.ts';

export interface ZipEntryDiff { name?: string; data?: number[]; metadata?: ZipEntryMetadata; }
export interface ZipEntryModified { name: string; diff: ZipEntryDiff; }
export interface ZipEntriesDiff { removed: string[]; modified: ZipEntryModified[]; added: ZipEntry[]; order?: string[]; }
export interface ZipDiff { comment?: string; commentUtf8?: boolean; entries?: ZipEntriesDiff; }

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
    commentUtf8: row["commentUtf8"] === undefined ? undefined : stdioZip20BaseDiffGuardBoolean(row["commentUtf8"], `${at}.commentUtf8`),
    entries: row["entries"] === undefined ? undefined : parseZipEntriesDiff(row["entries"], `${at}.entries`),
  };
}

export function parseZipEntry(value: unknown, at = "$"): ZipEntry {
  return parseSnapshotZipEntry(value, at);
}

export function parseZipEntryDiff(value: unknown, at = "$"): ZipEntryDiff {
  const row = stdioZip20BaseDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : stdioZip20BaseDiffGuardString(row["name"], `${at}.name`),
    data: row["data"] === undefined ? undefined : stdioZip20BaseDiffGuardArray(row["data"], `${at}.data`).map((item, index) => stdioZip20BaseDiffGuardInteger(item, `${at}.data[${index}]`, { minimum: 0, maximum: 255 })),
    metadata: row["metadata"] === undefined ? undefined : parseZipEntryMetadata(row["metadata"], `${at}.metadata`),
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
    removed: row["removed"] === undefined ? [] : stdioZip20BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioZip20BaseDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? [] : stdioZip20BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseZipEntryModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? [] : stdioZip20BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseZipEntry(item, `${at}.added[${index}]`)),
    order: row["order"] === undefined ? undefined : stdioZip20BaseDiffGuardArray(row["order"], `${at}.order`).map((item, index) => stdioZip20BaseDiffGuardString(item, `${at}.order[${index}]`)),
  };
}

/** 🔺️ Applies sparse member edits without changing the order of untouched entries. */
export function applyZipDiff(base: ZipSnapshot, diff: ZipDiff): ZipSnapshot {
  const next = structuredClone(base);
  if (diff.comment !== undefined) next.comment = diff.comment;
  if (diff.commentUtf8 !== undefined) next.commentUtf8 = diff.commentUtf8;
  if (diff.entries === undefined) return next;
  const changes = diff.entries;
  const names = new Set(base.entries.map((entry) => entry.name));
  if (names.size !== base.entries.length) throw new Error("mutation.apply.duplicate-target");
  const removed = new Set<string>();
  for (const name of changes.removed) {
    if (!names.has(name) || removed.has(name)) throw new Error("mutation.apply.missing-target");
    removed.add(name);
  }
  const occupied = new Set([...names].filter((name) => !removed.has(name)));
  const modified = new Set<string>();
  const renamed = new Set<string>();
  for (const entry of changes.modified) {
    if (!names.has(entry.name) || modified.has(entry.name) || removed.has(entry.name)) throw new Error("mutation.apply.conflicting-target");
    modified.add(entry.name);
    const name = entry.diff.name;
    if (name !== undefined) {
      if (name.length === 0 || (name !== entry.name && occupied.has(name)) || renamed.has(name)) throw new Error("mutation.apply.duplicate-target");
      renamed.add(name);
      occupied.delete(entry.name);
      occupied.add(name);
    }
  }
  for (const entry of changes.added) {
    if (entry.name.length === 0 || occupied.has(entry.name)) throw new Error("mutation.apply.duplicate-target");
    occupied.add(entry.name);
  }
  if (changes.order !== undefined && (changes.order.length !== occupied.size || new Set(changes.order).size !== changes.order.length || changes.order.some((name) => !occupied.has(name)))) throw new Error("mutation.apply.invalid-order");
  next.entries = next.entries.filter((entry) => !removed.has(entry.name));
  const byName = new Map(next.entries.map((entry) => [entry.name, entry]));
  for (const entry of changes.modified) {
    const target = byName.get(entry.name)!;
    if (entry.diff.name !== undefined) target.name = entry.diff.name;
    if (entry.diff.data !== undefined) target.data = [...entry.diff.data];
    if (entry.diff.metadata !== undefined) target.metadata = structuredClone(entry.diff.metadata);
  }
  next.entries.push(...changes.added.map((entry) => structuredClone(entry)));
  if (changes.order !== undefined) {
    const finalEntries = new Map(next.entries.map((entry) => [entry.name, entry]));
    next.entries = changes.order.map((name) => finalEntries.get(name)!);
  }
  return next;
}

/** ➕️ Addresses insertion by the existing following member so deletion can undo exactly. */
export function zipInsertionDiff(base: ZipSnapshot, entry: ZipEntry, before?: string): ZipDiff {
  if (before !== undefined && !base.entries.some((existing) => existing.name === before)) throw new Error("mutation.target-missing");
  const order = before === undefined ? undefined : base.entries.flatMap((existing) => existing.name === before ? [entry.name, existing.name] : [existing.name]);
  return { entries: { removed: [], modified: [], added: [structuredClone(entry)], order } };
}

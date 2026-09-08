/** 🎒️ One logical ZIP member with decompressed semantic content. */
export interface ZipEntry {
  name: string;
  data: number[];
}

/** 📸️ Logical `stdio.zip` snapshot. Entries are keyed and normalized by name. */
export interface ZipSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: ZipEntry[];
  /** @state artifact */ comment: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioZip20BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioZip20BaseSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioZip20BaseSnapshotGuardRefusal(at, why);
};

type stdioZip20BaseSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioZip20BaseSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioZip20BaseSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioZip20BaseSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioZip20BaseSnapshotGuardReject(at, "value is not an object");
export const stdioZip20BaseSnapshotGuardArray = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioZip20BaseSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioZip20BaseSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioZip20BaseSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioZip20BaseSnapshotGuardString = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioZip20BaseSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioZip20BaseSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioZip20BaseSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioZip20BaseSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioZip20BaseSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioZip20BaseSnapshotGuardReject(at, "value is not a boolean"));
export const stdioZip20BaseSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioZip20BaseSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioZip20BaseSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioZip20BaseSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioZip20BaseSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioZip20BaseSnapshotGuardNumber(value, at, bounds) : stdioZip20BaseSnapshotGuardReject(at, "value is not an integer");
export const stdioZip20BaseSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioZip20BaseSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioZip20BaseSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioZip20BaseSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseZipSnapshot(value: unknown, at = "$"): ZipSnapshot {
  const row = stdioZip20BaseSnapshotGuardObject(value, at);
  return {
    schema: stdioZip20BaseSnapshotGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioZip20BaseSnapshotGuardArray(row["entries"], `${at}.entries`).map((item, index) => parseZipEntry(item, `${at}.entries[${index}]`)),
    comment: row["comment"] === undefined ? undefined : stdioZip20BaseSnapshotGuardString(row["comment"], `${at}.comment`),
  };
}

export function parseZipEntry(value: unknown, at = "$"): ZipEntry {
  const row = stdioZip20BaseSnapshotGuardObject(value, at);
  return {
    name: stdioZip20BaseSnapshotGuardString(row["name"], `${at}.name`),
    data: stdioZip20BaseSnapshotGuardArray(row["data"], `${at}.data`).map((item, index) => stdioZip20BaseSnapshotGuardInteger(item, `${at}.data[${index}]`)),
  };
}

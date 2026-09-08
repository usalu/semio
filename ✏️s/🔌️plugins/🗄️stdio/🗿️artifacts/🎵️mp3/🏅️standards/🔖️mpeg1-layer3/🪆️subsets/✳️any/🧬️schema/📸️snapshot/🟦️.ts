/** 🧬️ Mp3Snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Snapshot
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3SnapshotEntry {
  key: string;
  value: string;
}
export interface Mp3Snapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Mp3SnapshotEntry[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMp3Mpeg1layer3AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMp3Mpeg1layer3AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioMp3Mpeg1layer3AnySnapshotGuardRefusal(at, why);
};

type stdioMp3Mpeg1layer3AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMp3Mpeg1layer3AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMp3Mpeg1layer3AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMp3Mpeg1layer3AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, "value is not an object");
export const stdioMp3Mpeg1layer3AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMp3Mpeg1layer3AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioMp3Mpeg1layer3AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMp3Mpeg1layer3AnySnapshotGuardNumber(value, at, bounds) : stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, "value is not an integer");
export const stdioMp3Mpeg1layer3AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMp3Mpeg1layer3AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMp3Mpeg1layer3AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMp3Snapshot(value: unknown, at = "$"): Mp3Snapshot {
  const row = stdioMp3Mpeg1layer3AnySnapshotGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioMp3Mpeg1layer3AnySnapshotGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioMp3Mpeg1layer3AnySnapshotGuardArray(row["entries"], `${at}.entries`).map((item, index) => stdioMp3Mpeg1layer3AnySnapshotGuardObject(item, `${at}.entries[${index}]`)),
  };
}

/** 🧬️ Mp3Diff schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Diff
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3DiffEntry {
  key: string;
  value: string;
}
export interface Mp3Diff {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Mp3DiffEntry[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMp3Mpeg1layer3AnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMp3Mpeg1layer3AnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioMp3Mpeg1layer3AnyDiffGuardRefusal(at, why);
};

type stdioMp3Mpeg1layer3AnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMp3Mpeg1layer3AnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMp3Mpeg1layer3AnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMp3Mpeg1layer3AnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMp3Mpeg1layer3AnyDiffGuardReject(at, "value is not an object");
export const stdioMp3Mpeg1layer3AnyDiffGuardArray = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMp3Mpeg1layer3AnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyDiffGuardString = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMp3Mpeg1layer3AnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMp3Mpeg1layer3AnyDiffGuardReject(at, "value is not a boolean"));
export const stdioMp3Mpeg1layer3AnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMp3Mpeg1layer3AnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMp3Mpeg1layer3AnyDiffGuardNumber(value, at, bounds) : stdioMp3Mpeg1layer3AnyDiffGuardReject(at, "value is not an integer");
export const stdioMp3Mpeg1layer3AnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMp3Mpeg1layer3AnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMp3Mpeg1layer3AnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMp3Diff(value: unknown, at = "$"): Mp3Diff {
  const row = stdioMp3Mpeg1layer3AnyDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioMp3Mpeg1layer3AnyDiffGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioMp3Mpeg1layer3AnyDiffGuardArray(row["entries"], `${at}.entries`).map((item, index) => stdioMp3Mpeg1layer3AnyDiffGuardObject(item, `${at}.entries[${index}]`)),
  };
}

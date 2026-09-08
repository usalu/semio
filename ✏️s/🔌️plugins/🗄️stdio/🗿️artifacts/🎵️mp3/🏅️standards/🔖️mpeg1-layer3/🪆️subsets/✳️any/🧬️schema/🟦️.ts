/** 🧬️ Mp3Artifact schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Artifact
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3ArtifactEntry {
  key: string;
  value: string;
}
export interface Mp3Artifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Mp3ArtifactEntry[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMp3Mpeg1layer3AnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMp3Mpeg1layer3AnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioMp3Mpeg1layer3AnyArtifactGuardRefusal(at, why);
};

type stdioMp3Mpeg1layer3AnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMp3Mpeg1layer3AnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMp3Mpeg1layer3AnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMp3Mpeg1layer3AnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, "value is not an object");
export const stdioMp3Mpeg1layer3AnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyArtifactGuardString = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioMp3Mpeg1layer3AnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMp3Mpeg1layer3AnyArtifactGuardNumber(value, at, bounds) : stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, "value is not an integer");
export const stdioMp3Mpeg1layer3AnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMp3Mpeg1layer3AnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMp3Mpeg1layer3AnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMp3Artifact(value: unknown, at = "$"): Mp3Artifact {
  const row = stdioMp3Mpeg1layer3AnyArtifactGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioMp3Mpeg1layer3AnyArtifactGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioMp3Mpeg1layer3AnyArtifactGuardArray(row["entries"], `${at}.entries`).map((item, index) => stdioMp3Mpeg1layer3AnyArtifactGuardObject(item, `${at}.entries[${index}]`)),
  };
}

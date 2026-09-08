/** 📝️ Text representation mirror for `stdio.semio.video` (snapshot): envelope header line +
 * hex(JSON) body. The JSON body's own structure is `../🟦️.ts`'s `SemioVideoSnapshot`. */
export interface SemioVideoSnapshotTextEnvelope {
  header: "schema stdio.semio.video";
  /** hex-encoded UTF-8 JSON, decodes as `SemioVideoSnapshot` (see ../🟦️.ts) */
  bodyHex: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1VideoSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1VideoSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1VideoSnapshotTextGuardRefusal(at, why);
};

type stdioSemioV1VideoSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1VideoSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1VideoSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1VideoSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1VideoSnapshotTextGuardReject(at, "value is not an object");
export const stdioSemioV1VideoSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1VideoSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1VideoSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1VideoSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1VideoSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1VideoSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1VideoSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1VideoSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1VideoSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1VideoSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1VideoSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1VideoSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1VideoSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1VideoSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1VideoSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1VideoSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1VideoSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1VideoSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1VideoSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1VideoSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1VideoSnapshotTextGuardNumber(value, at, bounds) : stdioSemioV1VideoSnapshotTextGuardReject(at, "value is not an integer");
export const stdioSemioV1VideoSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1VideoSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1VideoSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1VideoSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioVideoSnapshotTextEnvelope(value: unknown, at = "$"): SemioVideoSnapshotTextEnvelope {
  const row = stdioSemioV1VideoSnapshotTextGuardObject(value, at);
  return {
    header: stdioSemioV1VideoSnapshotTextGuardConstant(row["header"], `${at}.header`, "schema stdio.semio.video"),
    bodyHex: stdioSemioV1VideoSnapshotTextGuardString(row["bodyHex"], `${at}.bodyHex`),
  };
}

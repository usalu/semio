/** 🧬️ JpgArtifact schema — reduced UI-editable view: identity + the raster the user is directly
 * manipulating. `pixels` is canonical 8-bit-per-channel RGBA, `width * height * 4` bytes. */
export interface JpgArtifact {
  schema: string;
  width: number;
  height: number;
  pixels: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJpgJfif101DocumentArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJpgJfif101DocumentArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioJpgJfif101DocumentArtifactGuardRefusal(at, why);
};

type stdioJpgJfif101DocumentArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJpgJfif101DocumentArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJpgJfif101DocumentArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJpgJfif101DocumentArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJpgJfif101DocumentArtifactGuardReject(at, "value is not an object");
export const stdioJpgJfif101DocumentArtifactGuardArray = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJpgJfif101DocumentArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJpgJfif101DocumentArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJpgJfif101DocumentArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJpgJfif101DocumentArtifactGuardString = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJpgJfif101DocumentArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJpgJfif101DocumentArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJpgJfif101DocumentArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJpgJfif101DocumentArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJpgJfif101DocumentArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJpgJfif101DocumentArtifactGuardReject(at, "value is not a boolean"));
export const stdioJpgJfif101DocumentArtifactGuardNumber = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJpgJfif101DocumentArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJpgJfif101DocumentArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJpgJfif101DocumentArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJpgJfif101DocumentArtifactGuardInteger = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJpgJfif101DocumentArtifactGuardNumber(value, at, bounds) : stdioJpgJfif101DocumentArtifactGuardReject(at, "value is not an integer");
export const stdioJpgJfif101DocumentArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJpgJfif101DocumentArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJpgJfif101DocumentArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJpgJfif101DocumentArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJpgArtifact(value: unknown, at = "$"): JpgArtifact {
  const row = stdioJpgJfif101DocumentArtifactGuardObject(value, at);
  return {
    schema: stdioJpgJfif101DocumentArtifactGuardString(row["schema"], `${at}.schema`),
    width: stdioJpgJfif101DocumentArtifactGuardInteger(row["width"], `${at}.width`),
    height: stdioJpgJfif101DocumentArtifactGuardInteger(row["height"], `${at}.height`),
    pixels: stdioJpgJfif101DocumentArtifactGuardArray(row["pixels"], `${at}.pixels`).map((item, index) => stdioJpgJfif101DocumentArtifactGuardInteger(item, `${at}.pixels[${index}]`)),
  };
}

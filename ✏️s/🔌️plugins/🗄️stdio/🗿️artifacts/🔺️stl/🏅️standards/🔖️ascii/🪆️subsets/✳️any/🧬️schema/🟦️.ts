/** 🧬️ StlArtifact schema — full `stdio.stl` artifact state (mirrors `StlSnapshot`). */
export interface StlTriangle {
  normal: [number, number, number];
  vertices: [[number, number, number], [number, number, number], [number, number, number]];
}
export interface StlArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ solidName: string;
  /** @state artifact */ triangles: StlTriangle[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStlAsciiAnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStlAsciiAnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioStlAsciiAnyArtifactGuardRefusal(at, why);
};

type stdioStlAsciiAnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStlAsciiAnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStlAsciiAnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStlAsciiAnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStlAsciiAnyArtifactGuardReject(at, "value is not an object");
export const stdioStlAsciiAnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioStlAsciiAnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStlAsciiAnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStlAsciiAnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStlAsciiAnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStlAsciiAnyArtifactGuardString = (value: unknown, at: string, bounds: stdioStlAsciiAnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStlAsciiAnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStlAsciiAnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStlAsciiAnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStlAsciiAnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStlAsciiAnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStlAsciiAnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioStlAsciiAnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioStlAsciiAnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStlAsciiAnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStlAsciiAnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStlAsciiAnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStlAsciiAnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioStlAsciiAnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStlAsciiAnyArtifactGuardNumber(value, at, bounds) : stdioStlAsciiAnyArtifactGuardReject(at, "value is not an integer");
export const stdioStlAsciiAnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStlAsciiAnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStlAsciiAnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStlAsciiAnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStlArtifact(value: unknown, at = "$"): StlArtifact {
  const row = stdioStlAsciiAnyArtifactGuardObject(value, at);
  return {
    schema: stdioStlAsciiAnyArtifactGuardString(row["schema"], `${at}.schema`),
    solidName: row["solidName"] === undefined ? undefined : stdioStlAsciiAnyArtifactGuardString(row["solidName"], `${at}.solidName`),
    triangles: row["triangles"] === undefined ? undefined : stdioStlAsciiAnyArtifactGuardArray(row["triangles"], `${at}.triangles`).map((item, index) => parseStlTriangle(item, `${at}.triangles[${index}]`)),
  };
}

export function parseStlTriangle(value: unknown, at = "$"): StlTriangle {
  const row = stdioStlAsciiAnyArtifactGuardObject(value, at);
  return {
    normal: stdioStlAsciiAnyArtifactGuardArray(row["normal"], `${at}.normal`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyArtifactGuardNumber(item, `${at}.normal[${index}]`)),
    vertices: stdioStlAsciiAnyArtifactGuardArray(row["vertices"], `${at}.vertices`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyArtifactGuardArray(item, `${at}.vertices[${index}]`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyArtifactGuardNumber(item, `${at}.vertices[${index}][${index}]`))),
  };
}

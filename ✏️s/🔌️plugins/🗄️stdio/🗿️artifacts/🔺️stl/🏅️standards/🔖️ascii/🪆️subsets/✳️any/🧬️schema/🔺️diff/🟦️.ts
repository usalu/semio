/** 🔺️ StlDiff — handcrafted sparse diff. `solidName` plus an index-keyed `triangles` triple. */

export interface StlTriangle {
  normal: [number, number, number];
  vertices: [[number, number, number], [number, number, number], [number, number, number]];
}

/** 🔺️ Sparse per-field patch for one `StlTriangle`; both fields whole-value replace. */
export interface StlTriangleDiff {
  normal?: [number, number, number];
  vertices?: [[number, number, number], [number, number, number], [number, number, number]];
}

/** 📦️ One `triangles.modified[]` entity — `index` is the triangle's position in BASE. */
export interface StlTriangleModified {
  index: number;
  diff: StlTriangleDiff;
}

/** 📦️ One `triangles.added[]` entity — `index` is the triangle's position in the FINAL sequence. */
export interface StlTriangleAdded {
  index: number;
  triangle: StlTriangle;
}

export interface StlTrianglesDiff {
  removed: number[];
  modified: StlTriangleModified[];
  added: StlTriangleAdded[];
}

/** 🔺️ Diff for `stdio.stl`. `schema` is an identity field and never appears here. */
export interface StlDiff {
  solidName?: string;
  triangles?: StlTrianglesDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStlAsciiAnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStlAsciiAnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioStlAsciiAnyDiffGuardRefusal(at, why);
};

type stdioStlAsciiAnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStlAsciiAnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStlAsciiAnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStlAsciiAnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStlAsciiAnyDiffGuardReject(at, "value is not an object");
export const stdioStlAsciiAnyDiffGuardArray = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStlAsciiAnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStlAsciiAnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStlAsciiAnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStlAsciiAnyDiffGuardString = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStlAsciiAnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStlAsciiAnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStlAsciiAnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStlAsciiAnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStlAsciiAnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStlAsciiAnyDiffGuardReject(at, "value is not a boolean"));
export const stdioStlAsciiAnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStlAsciiAnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStlAsciiAnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStlAsciiAnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStlAsciiAnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStlAsciiAnyDiffGuardNumber(value, at, bounds) : stdioStlAsciiAnyDiffGuardReject(at, "value is not an integer");
export const stdioStlAsciiAnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStlAsciiAnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStlAsciiAnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStlAsciiAnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStlDiff(value: unknown, at = "$"): StlDiff {
  const row = stdioStlAsciiAnyDiffGuardObject(value, at);
  return {
    solidName: row["solidName"] === undefined ? undefined : stdioStlAsciiAnyDiffGuardString(row["solidName"], `${at}.solidName`),
    triangles: row["triangles"] === undefined ? undefined : parseStlTrianglesDiff(row["triangles"], `${at}.triangles`),
  };
}

export function parseStlTriangleDiff(value: unknown, at = "$"): StlTriangleDiff {
  const row = stdioStlAsciiAnyDiffGuardObject(value, at);
  return {
    normal: row["normal"] === undefined ? undefined : stdioStlAsciiAnyDiffGuardArray(row["normal"], `${at}.normal`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyDiffGuardNumber(item, `${at}.normal[${index}]`)),
    vertices: row["vertices"] === undefined ? undefined : stdioStlAsciiAnyDiffGuardArray(row["vertices"], `${at}.vertices`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyDiffGuardArray(item, `${at}.vertices[${index}]`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyDiffGuardNumber(item, `${at}.vertices[${index}][${index}]`))),
  };
}

export function parseStlTriangleModified(value: unknown, at = "$"): StlTriangleModified {
  const row = stdioStlAsciiAnyDiffGuardObject(value, at);
  return {
    index: stdioStlAsciiAnyDiffGuardInteger(row["index"], `${at}.index`),
    diff: parseStlTriangleDiff(row["diff"], `${at}.diff`),
  };
}

export function parseStlTrianglesDiff(value: unknown, at = "$"): StlTrianglesDiff {
  const row = stdioStlAsciiAnyDiffGuardObject(value, at);
  return {
    removed: stdioStlAsciiAnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioStlAsciiAnyDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: stdioStlAsciiAnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseStlTriangleModified(item, `${at}.modified[${index}]`)),
    added: stdioStlAsciiAnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseStlTriangleAdded(item, `${at}.added[${index}]`)),
  };
}

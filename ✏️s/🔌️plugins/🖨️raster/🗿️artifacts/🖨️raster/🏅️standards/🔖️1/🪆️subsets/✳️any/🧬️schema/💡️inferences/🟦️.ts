/** 💡️ Raster inference schema — layer-tree topology (pre-order + nesting depth). */

export interface RasterTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface RasterInference {
  /** @derived */
  topology: RasterTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterInferenceGuardReject = (at: string, why: string): never => {
  throw new rasterRasterInferenceGuardRefusal(at, why);
};

type rasterRasterInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterInferenceGuardReject(at, "value is not an object");
export const rasterRasterInferenceGuardArray = (value: unknown, at: string, bounds: rasterRasterInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterInferenceGuardString = (value: unknown, at: string, bounds: rasterRasterInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterInferenceGuardReject(at, "value is not a boolean"));
export const rasterRasterInferenceGuardNumber = (value: unknown, at: string, bounds: rasterRasterInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterInferenceGuardInteger = (value: unknown, at: string, bounds: rasterRasterInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterInferenceGuardNumber(value, at, bounds) : rasterRasterInferenceGuardReject(at, "value is not an integer");
export const rasterRasterInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterInference(value: unknown, at = "$"): RasterInference {
  const row = rasterRasterInferenceGuardObject(value, at);
  return {
    topology: parseRasterTopology(row["topology"], `${at}.topology`),
  };
}

export function parseRasterTopology(value: unknown, at = "$"): RasterTopology {
  const row = rasterRasterInferenceGuardObject(value, at);
  return {
    topoOrder: rasterRasterInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => rasterRasterInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: rasterRasterInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: rasterRasterInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: rasterRasterInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}

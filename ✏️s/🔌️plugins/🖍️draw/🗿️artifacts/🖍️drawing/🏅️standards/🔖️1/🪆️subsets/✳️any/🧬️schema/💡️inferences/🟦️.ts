/** 💡️ Drawing inference schema — layer-tree topology (pre-order + nesting depth). */

export interface DrawingTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface DrawingInference {
  /** @derived */
  topology: DrawingTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class drawingDrawingInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const drawingDrawingInferenceGuardReject = (at: string, why: string): never => {
  throw new drawingDrawingInferenceGuardRefusal(at, why);
};

type drawingDrawingInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type drawingDrawingInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type drawingDrawingInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const drawingDrawingInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : drawingDrawingInferenceGuardReject(at, "value is not an object");
export const drawingDrawingInferenceGuardArray = (value: unknown, at: string, bounds: drawingDrawingInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return drawingDrawingInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) drawingDrawingInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) drawingDrawingInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const drawingDrawingInferenceGuardString = (value: unknown, at: string, bounds: drawingDrawingInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return drawingDrawingInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) drawingDrawingInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) drawingDrawingInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) drawingDrawingInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const drawingDrawingInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : drawingDrawingInferenceGuardReject(at, "value is not a boolean"));
export const drawingDrawingInferenceGuardNumber = (value: unknown, at: string, bounds: drawingDrawingInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return drawingDrawingInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) drawingDrawingInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) drawingDrawingInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const drawingDrawingInferenceGuardInteger = (value: unknown, at: string, bounds: drawingDrawingInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? drawingDrawingInferenceGuardNumber(value, at, bounds) : drawingDrawingInferenceGuardReject(at, "value is not an integer");
export const drawingDrawingInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : drawingDrawingInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const drawingDrawingInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : drawingDrawingInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDrawingInference(value: unknown, at = "$"): DrawingInference {
  const row = drawingDrawingInferenceGuardObject(value, at);
  return {
    topology: parseDrawingTopology(row["topology"], `${at}.topology`),
  };
}

export function parseDrawingTopology(value: unknown, at = "$"): DrawingTopology {
  const row = drawingDrawingInferenceGuardObject(value, at);
  return {
    topoOrder: drawingDrawingInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => drawingDrawingInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: drawingDrawingInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: drawingDrawingInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: drawingDrawingInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}

/** 🧬️ DrawingConfig */
export interface DrawingConfig {
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: DrawingCamera;
  /** @state config */
  tracePointerGeneration: number;
  /** @state config */
  tracePointerCompletedWork: number;
  /** @state config */
  tracePointerPendingWork: number;
  /** @state config */
  /** @state config */
}
export interface DrawingCamera { x: number; y: number; zoom: number; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class drawingDrawingConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const drawingDrawingConfigGuardReject = (at: string, why: string): never => {
  throw new drawingDrawingConfigGuardRefusal(at, why);
};

type drawingDrawingConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type drawingDrawingConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type drawingDrawingConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const drawingDrawingConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : drawingDrawingConfigGuardReject(at, "value is not an object");
export const drawingDrawingConfigGuardArray = (value: unknown, at: string, bounds: drawingDrawingConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return drawingDrawingConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) drawingDrawingConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) drawingDrawingConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const drawingDrawingConfigGuardString = (value: unknown, at: string, bounds: drawingDrawingConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return drawingDrawingConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) drawingDrawingConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) drawingDrawingConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) drawingDrawingConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const drawingDrawingConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : drawingDrawingConfigGuardReject(at, "value is not a boolean"));
export const drawingDrawingConfigGuardNumber = (value: unknown, at: string, bounds: drawingDrawingConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return drawingDrawingConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) drawingDrawingConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) drawingDrawingConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const drawingDrawingConfigGuardInteger = (value: unknown, at: string, bounds: drawingDrawingConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? drawingDrawingConfigGuardNumber(value, at, bounds) : drawingDrawingConfigGuardReject(at, "value is not an integer");
export const drawingDrawingConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : drawingDrawingConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const drawingDrawingConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : drawingDrawingConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDrawingConfig(value: unknown, at = "$"): DrawingConfig {
  const row = drawingDrawingConfigGuardObject(value, at);
  return {
    engagementInput: drawingDrawingConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
    camera: parseDrawingCamera(row["camera"], `${at}.camera`),
    tracePointerGeneration: drawingDrawingConfigGuardInteger(row["tracePointerGeneration"], `${at}.tracePointerGeneration`, {"minimum": 0}),
    tracePointerCompletedWork: drawingDrawingConfigGuardInteger(row["tracePointerCompletedWork"], `${at}.tracePointerCompletedWork`, {"minimum": 0}),
    tracePointerPendingWork: drawingDrawingConfigGuardInteger(row["tracePointerPendingWork"], `${at}.tracePointerPendingWork`, {"minimum": 0}),
  };
}

export function parseDrawingCamera(value: unknown, at = "$"): DrawingCamera {
  const row = drawingDrawingConfigGuardObject(value, at);
  return {
    x: drawingDrawingConfigGuardNumber(row["x"], `${at}.x`),
    y: drawingDrawingConfigGuardNumber(row["y"], `${at}.y`),
    zoom: drawingDrawingConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

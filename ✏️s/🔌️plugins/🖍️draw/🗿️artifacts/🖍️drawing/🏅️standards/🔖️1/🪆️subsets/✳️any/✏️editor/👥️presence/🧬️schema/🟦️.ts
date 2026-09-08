/** 🧬️ DrawingPresence */
export interface DrawingPresence {
  /** @state presence */
  engagementInput: string;
  /** @state presence */
  camera: DrawingCamera;
  /** @state presence */
  activeUtilityId: string;
}
export interface DrawingCamera { x: number; y: number; zoom: number; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class drawingDrawingPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const drawingDrawingPresenceGuardReject = (at: string, why: string): never => {
  throw new drawingDrawingPresenceGuardRefusal(at, why);
};

type drawingDrawingPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type drawingDrawingPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type drawingDrawingPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const drawingDrawingPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : drawingDrawingPresenceGuardReject(at, "value is not an object");
export const drawingDrawingPresenceGuardArray = (value: unknown, at: string, bounds: drawingDrawingPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return drawingDrawingPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) drawingDrawingPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) drawingDrawingPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const drawingDrawingPresenceGuardString = (value: unknown, at: string, bounds: drawingDrawingPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return drawingDrawingPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) drawingDrawingPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) drawingDrawingPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) drawingDrawingPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const drawingDrawingPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : drawingDrawingPresenceGuardReject(at, "value is not a boolean"));
export const drawingDrawingPresenceGuardNumber = (value: unknown, at: string, bounds: drawingDrawingPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return drawingDrawingPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) drawingDrawingPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) drawingDrawingPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const drawingDrawingPresenceGuardInteger = (value: unknown, at: string, bounds: drawingDrawingPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? drawingDrawingPresenceGuardNumber(value, at, bounds) : drawingDrawingPresenceGuardReject(at, "value is not an integer");
export const drawingDrawingPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : drawingDrawingPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const drawingDrawingPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : drawingDrawingPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDrawingPresence(value: unknown, at = "$"): DrawingPresence {
  const row = drawingDrawingPresenceGuardObject(value, at);
  return {
    engagementInput: drawingDrawingPresenceGuardString(row["engagementInput"], `${at}.engagementInput`),
    camera: parseDrawingCamera(row["camera"], `${at}.camera`),
    activeUtilityId: drawingDrawingPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
  };
}

export function parseDrawingCamera(value: unknown, at = "$"): DrawingCamera {
  const row = drawingDrawingPresenceGuardObject(value, at);
  return {
    x: drawingDrawingPresenceGuardNumber(row["x"], `${at}.x`),
    y: drawingDrawingPresenceGuardNumber(row["y"], `${at}.y`),
    zoom: drawingDrawingPresenceGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

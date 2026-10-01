/** 💡️ SemioDrawing inference schema — flattenedScene (world transform + resolved style) per
 * scene-graph entity, keyed by the same `"<layer>:<p0>.<p1>..."` structural address every
 * mutation triad in this facet uses in place of a stable node id. */
import {parseDrawStyle,parseTransform,type DrawStyle,type Transform} from "../📸️snapshot/🟦️.ts";
export {parseDrawStyle,parseTransform,parseRgba} from "../📸️snapshot/🟦️.ts";
export type {DrawStyle,Transform,Rgba} from "../📸️snapshot/🟦️.ts";

export interface FlattenedNode {
  worldTransform: Transform;
  resolvedStyle?: DrawStyle;
}

export interface SemioDrawingInference {
  /** @derived */
  flattenedScene: Record<string, FlattenedNode>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1DrawingInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1DrawingInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1DrawingInferenceGuardRefusal(at, why);
};

type stdioSemioV1DrawingInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1DrawingInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1DrawingInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1DrawingInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1DrawingInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1DrawingInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1DrawingInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1DrawingInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1DrawingInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1DrawingInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1DrawingInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1DrawingInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1DrawingInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1DrawingInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1DrawingInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1DrawingInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1DrawingInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1DrawingInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1DrawingInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1DrawingInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1DrawingInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1DrawingInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1DrawingInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1DrawingInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1DrawingInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1DrawingInferenceGuardNumber(value, at, bounds) : stdioSemioV1DrawingInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1DrawingInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1DrawingInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1DrawingInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1DrawingInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioDrawingInference(value: unknown, at = "$"): SemioDrawingInference {
  const row = stdioSemioV1DrawingInferenceGuardObject(value, at);
  return {
    flattenedScene: Object.fromEntries(Object.entries(stdioSemioV1DrawingInferenceGuardObject(row["flattenedScene"], `${at}.flattenedScene`)).map(([key,value])=>[key,parseFlattenedNode(value,`${at}.flattenedScene.${key}`)])),
  };
}

export function parseFlattenedNode(value: unknown, at = "$"): FlattenedNode {
  const row = stdioSemioV1DrawingInferenceGuardObject(value, at);
  return {
    worldTransform: parseTransform(row["worldTransform"], `${at}.worldTransform`),
    resolvedStyle: row["resolvedStyle"] === undefined ? undefined : parseDrawStyle(row["resolvedStyle"], `${at}.resolvedStyle`),
  };
}

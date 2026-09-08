/** 🧬️ Rewriting artifact schema — every field with its state class. */

export interface RewritingArtifact {
  /** @state artifact */
  beforeFixtureJson: string;
  /** @state artifact */
  lhsJson: string;
  /** @state artifact */
  rhsJson: string;
  /** @state artifact */
  parameterBindings: Record<string, PropertyValue>;
  /** @state artifact */
  ruleLayout: Record<string, LayoutPoint>;
  /** @state config */
  lodModeByWindow: Record<string, string>;
  /** @state config */
  beforePaneCamera: Camera;
  /** @state config */
  /** @state config */
}

export type PropertyValue =
  | null
  | boolean
  | number
  | string
  | PropertyValue[]
  | { [key: string]: PropertyValue };

export interface LayoutPoint {
  x: number;
  y: number;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityRewritingArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityRewritingArtifactGuardReject = (at: string, why: string): never => {
  throw new trinityRewritingArtifactGuardRefusal(at, why);
};

type trinityRewritingArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityRewritingArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityRewritingArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityRewritingArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityRewritingArtifactGuardReject(at, "value is not an object");
export const trinityRewritingArtifactGuardArray = (value: unknown, at: string, bounds: trinityRewritingArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityRewritingArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityRewritingArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityRewritingArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityRewritingArtifactGuardString = (value: unknown, at: string, bounds: trinityRewritingArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityRewritingArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityRewritingArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityRewritingArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityRewritingArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityRewritingArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityRewritingArtifactGuardReject(at, "value is not a boolean"));
export const trinityRewritingArtifactGuardNumber = (value: unknown, at: string, bounds: trinityRewritingArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityRewritingArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityRewritingArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityRewritingArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityRewritingArtifactGuardInteger = (value: unknown, at: string, bounds: trinityRewritingArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityRewritingArtifactGuardNumber(value, at, bounds) : trinityRewritingArtifactGuardReject(at, "value is not an integer");
export const trinityRewritingArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityRewritingArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityRewritingArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityRewritingArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRewritingArtifact(value: unknown, at = "$"): RewritingArtifact {
  const row = trinityRewritingArtifactGuardObject(value, at);
  return {
    beforeFixtureJson: trinityRewritingArtifactGuardString(row["beforeFixtureJson"], `${at}.beforeFixtureJson`),
    lhsJson: trinityRewritingArtifactGuardString(row["lhsJson"], `${at}.lhsJson`),
    rhsJson: trinityRewritingArtifactGuardString(row["rhsJson"], `${at}.rhsJson`),
    parameterBindings: trinityRewritingArtifactGuardObject(row["parameterBindings"], `${at}.parameterBindings`),
    ruleLayout: trinityRewritingArtifactGuardObject(row["ruleLayout"], `${at}.ruleLayout`),
    lodModeByWindow: trinityRewritingArtifactGuardObject(row["lodModeByWindow"], `${at}.lodModeByWindow`),
    beforePaneCamera: parseCamera(row["beforePaneCamera"], `${at}.beforePaneCamera`),
  };
}

export function parseLayoutPoint(value: unknown, at = "$"): LayoutPoint {
  const row = trinityRewritingArtifactGuardObject(value, at);
  return {
    x: trinityRewritingArtifactGuardNumber(row["x"], `${at}.x`),
    y: trinityRewritingArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseCamera(value: unknown, at = "$"): Camera {
  const row = trinityRewritingArtifactGuardObject(value, at);
  return {
    x: trinityRewritingArtifactGuardNumber(row["x"], `${at}.x`),
    y: trinityRewritingArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: trinityRewritingArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

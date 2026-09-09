/** 🧬️ Imperative artifact schema — every field with its state class. */

export interface ProcedureArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  path: ProcedurePath;
  /** @state artifact */
  seed: Record<string, unknown>;
}

export interface ProcedurePath {
  steps: ProcedureStep[];
}

export interface ProcedureStep {
  id: string;
  kind: string;
  params?: Record<string, unknown>;
  bodies?: Record<string, ProcedurePath>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeArtifactGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeArtifactGuardRefusal(at, why);
};

type imperativeImperativeArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeArtifactGuardReject(at, "value is not an object");
export const imperativeImperativeArtifactGuardArray = (value: unknown, at: string, bounds: imperativeImperativeArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeArtifactGuardString = (value: unknown, at: string, bounds: imperativeImperativeArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeArtifactGuardReject(at, "value is not a boolean"));
export const imperativeImperativeArtifactGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeArtifactGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeArtifactGuardNumber(value, at, bounds) : imperativeImperativeArtifactGuardReject(at, "value is not an integer");
export const imperativeImperativeArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcedureArtifact(value: unknown, at = "$"): ProcedureArtifact {
  const row = imperativeImperativeArtifactGuardObject(value, at);
  return {
    schema: imperativeImperativeArtifactGuardString(row["schema"], `${at}.schema`),
    path: parseProcedurePath(row["path"], `${at}.path`),
    seed: imperativeImperativeArtifactGuardObject(row["seed"], `${at}.seed`),
  };
}

export function parseProcedurePath(value: unknown, at = "$"): ProcedurePath {
  const row = imperativeImperativeArtifactGuardObject(value, at);
  return {
    steps: imperativeImperativeArtifactGuardArray(row["steps"], `${at}.steps`).map((item, index) => parseProcedureStep(item, `${at}.steps[${index}]`)),
  };
}

export function parseProcedureStep(value: unknown, at = "$"): ProcedureStep {
  const row = imperativeImperativeArtifactGuardObject(value, at);
  return {
    id: imperativeImperativeArtifactGuardString(row["id"], `${at}.id`),
    kind: imperativeImperativeArtifactGuardString(row["kind"], `${at}.kind`),
    params: row["params"] === undefined ? undefined : imperativeImperativeArtifactGuardObject(row["params"], `${at}.params`),
    bodies: row["bodies"] === undefined ? undefined : imperativeImperativeArtifactGuardObject(row["bodies"], `${at}.bodies`),
  };
}

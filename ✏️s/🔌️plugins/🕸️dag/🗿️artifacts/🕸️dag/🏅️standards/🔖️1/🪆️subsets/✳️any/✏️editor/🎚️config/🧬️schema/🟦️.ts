/** 🧬️ DagConfig */
export interface DagConfig {
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class dagDagConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const dagDagConfigGuardReject = (at: string, why: string): never => {
  throw new dagDagConfigGuardRefusal(at, why);
};

type dagDagConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type dagDagConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type dagDagConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const dagDagConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : dagDagConfigGuardReject(at, "value is not an object");
export const dagDagConfigGuardArray = (value: unknown, at: string, bounds: dagDagConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return dagDagConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) dagDagConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) dagDagConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const dagDagConfigGuardString = (value: unknown, at: string, bounds: dagDagConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return dagDagConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) dagDagConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) dagDagConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) dagDagConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const dagDagConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : dagDagConfigGuardReject(at, "value is not a boolean"));
export const dagDagConfigGuardNumber = (value: unknown, at: string, bounds: dagDagConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return dagDagConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) dagDagConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) dagDagConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const dagDagConfigGuardInteger = (value: unknown, at: string, bounds: dagDagConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? dagDagConfigGuardNumber(value, at, bounds) : dagDagConfigGuardReject(at, "value is not an integer");
export const dagDagConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : dagDagConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const dagDagConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : dagDagConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDagConfig(value: unknown, at = "$"): DagConfig {
  const row = dagDagConfigGuardObject(value, at);
  return {
    cameraX: dagDagConfigGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: dagDagConfigGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: dagDagConfigGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
  };
}

/** 🧬️ DagPresence */
export interface DagPresence {
  /** @state presence */
  cameraX: number;
  /** @state presence */
  cameraY: number;
  /** @state presence */
  cameraZoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class dagDagPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const dagDagPresenceGuardReject = (at: string, why: string): never => {
  throw new dagDagPresenceGuardRefusal(at, why);
};

type dagDagPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type dagDagPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type dagDagPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const dagDagPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : dagDagPresenceGuardReject(at, "value is not an object");
export const dagDagPresenceGuardArray = (value: unknown, at: string, bounds: dagDagPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return dagDagPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) dagDagPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) dagDagPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const dagDagPresenceGuardString = (value: unknown, at: string, bounds: dagDagPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return dagDagPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) dagDagPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) dagDagPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) dagDagPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const dagDagPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : dagDagPresenceGuardReject(at, "value is not a boolean"));
export const dagDagPresenceGuardNumber = (value: unknown, at: string, bounds: dagDagPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return dagDagPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) dagDagPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) dagDagPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const dagDagPresenceGuardInteger = (value: unknown, at: string, bounds: dagDagPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? dagDagPresenceGuardNumber(value, at, bounds) : dagDagPresenceGuardReject(at, "value is not an integer");
export const dagDagPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : dagDagPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const dagDagPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : dagDagPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDagPresence(value: unknown, at = "$"): DagPresence {
  const row = dagDagPresenceGuardObject(value, at);
  return {
    cameraX: dagDagPresenceGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: dagDagPresenceGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: dagDagPresenceGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
  };
}

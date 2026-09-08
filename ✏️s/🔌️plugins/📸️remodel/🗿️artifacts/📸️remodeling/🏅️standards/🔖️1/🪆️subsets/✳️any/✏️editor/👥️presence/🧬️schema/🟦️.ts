/** 🧬️ RemodelingPresence */
export interface RemodelingPresence {
  /** @state presence */
  worldCameraPosition: number[];
  /** @state presence */
  worldCameraTarget: number[];
  /** @state presence */
  worldCameraFov: number;
  /** @state presence */
  frameStreamId?: string;
  /** @state presence */
  frameIndex: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  reportTable: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingPresenceGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingPresenceGuardRefusal(at, why);
};

type remodelRemodelingPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingPresenceGuardReject(at, "value is not an object");
export const remodelRemodelingPresenceGuardArray = (value: unknown, at: string, bounds: remodelRemodelingPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingPresenceGuardString = (value: unknown, at: string, bounds: remodelRemodelingPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingPresenceGuardReject(at, "value is not a boolean"));
export const remodelRemodelingPresenceGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingPresenceGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingPresenceGuardNumber(value, at, bounds) : remodelRemodelingPresenceGuardReject(at, "value is not an integer");
export const remodelRemodelingPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRemodelingPresence(value: unknown, at = "$"): RemodelingPresence {
  const row = remodelRemodelingPresenceGuardObject(value, at);
  return {
    worldCameraPosition: remodelRemodelingPresenceGuardArray(row["worldCameraPosition"], `${at}.worldCameraPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingPresenceGuardNumber(item, `${at}.worldCameraPosition[${index}]`)),
    worldCameraTarget: remodelRemodelingPresenceGuardArray(row["worldCameraTarget"], `${at}.worldCameraTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingPresenceGuardNumber(item, `${at}.worldCameraTarget[${index}]`)),
    worldCameraFov: remodelRemodelingPresenceGuardNumber(row["worldCameraFov"], `${at}.worldCameraFov`),
    frameStreamId: row["frameStreamId"] === undefined ? undefined : remodelRemodelingPresenceGuardString(row["frameStreamId"], `${at}.frameStreamId`),
    frameIndex: remodelRemodelingPresenceGuardInteger(row["frameIndex"], `${at}.frameIndex`, {"minimum": 0}),
    activeUtilityId: remodelRemodelingPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    reportTable: remodelRemodelingPresenceGuardString(row["reportTable"], `${at}.reportTable`),
  };
}

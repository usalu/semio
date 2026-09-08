/** 🧬️ SourcingCurationPresence */
export interface SourcingCurationPresence {
  /** @state presence */
  worldCameraPosition: number[];
  /** @state presence */
  worldCameraTarget: number[];
  /** @state presence */
  worldCameraFov: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sourcingCurationPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sourcingCurationPresenceGuardReject = (at: string, why: string): never => {
  throw new sourcingCurationPresenceGuardRefusal(at, why);
};

type sourcingCurationPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sourcingCurationPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sourcingCurationPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sourcingCurationPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sourcingCurationPresenceGuardReject(at, "value is not an object");
export const sourcingCurationPresenceGuardArray = (value: unknown, at: string, bounds: sourcingCurationPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sourcingCurationPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sourcingCurationPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sourcingCurationPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sourcingCurationPresenceGuardString = (value: unknown, at: string, bounds: sourcingCurationPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sourcingCurationPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sourcingCurationPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sourcingCurationPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sourcingCurationPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sourcingCurationPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sourcingCurationPresenceGuardReject(at, "value is not a boolean"));
export const sourcingCurationPresenceGuardNumber = (value: unknown, at: string, bounds: sourcingCurationPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sourcingCurationPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sourcingCurationPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sourcingCurationPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sourcingCurationPresenceGuardInteger = (value: unknown, at: string, bounds: sourcingCurationPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sourcingCurationPresenceGuardNumber(value, at, bounds) : sourcingCurationPresenceGuardReject(at, "value is not an integer");
export const sourcingCurationPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sourcingCurationPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sourcingCurationPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sourcingCurationPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSourcingCurationPresence(value: unknown, at = "$"): SourcingCurationPresence {
  const row = sourcingCurationPresenceGuardObject(value, at);
  return {
    worldCameraPosition: sourcingCurationPresenceGuardArray(row["worldCameraPosition"], `${at}.worldCameraPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => sourcingCurationPresenceGuardNumber(item, `${at}.worldCameraPosition[${index}]`)),
    worldCameraTarget: sourcingCurationPresenceGuardArray(row["worldCameraTarget"], `${at}.worldCameraTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => sourcingCurationPresenceGuardNumber(item, `${at}.worldCameraTarget[${index}]`)),
    worldCameraFov: sourcingCurationPresenceGuardNumber(row["worldCameraFov"], `${at}.worldCameraFov`),
  };
}

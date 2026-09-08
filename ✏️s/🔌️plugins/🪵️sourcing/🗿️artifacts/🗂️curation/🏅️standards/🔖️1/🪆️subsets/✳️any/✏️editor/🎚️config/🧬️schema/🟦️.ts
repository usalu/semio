/** 🧬️ SourcingCurationConfig */
export type SortDirection = "asc" | "desc";

export interface TableSort {
  columnId: string;
  direction: SortDirection;
}

export interface Filters {
  query: string;
  moduleIds: string[];
  typologyPath: string[];
  minAvailability: number;
  sort?: TableSort | null;
}

export interface SourcingCurationConfig {
  /** @state config */
  filters: Filters;
  /** @state config */
  /** @state config */
  contributionsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sourcingCurationConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sourcingCurationConfigGuardReject = (at: string, why: string): never => {
  throw new sourcingCurationConfigGuardRefusal(at, why);
};

type sourcingCurationConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sourcingCurationConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sourcingCurationConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sourcingCurationConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sourcingCurationConfigGuardReject(at, "value is not an object");
export const sourcingCurationConfigGuardArray = (value: unknown, at: string, bounds: sourcingCurationConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sourcingCurationConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sourcingCurationConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sourcingCurationConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sourcingCurationConfigGuardString = (value: unknown, at: string, bounds: sourcingCurationConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sourcingCurationConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sourcingCurationConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sourcingCurationConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sourcingCurationConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sourcingCurationConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sourcingCurationConfigGuardReject(at, "value is not a boolean"));
export const sourcingCurationConfigGuardNumber = (value: unknown, at: string, bounds: sourcingCurationConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sourcingCurationConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sourcingCurationConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sourcingCurationConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sourcingCurationConfigGuardInteger = (value: unknown, at: string, bounds: sourcingCurationConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sourcingCurationConfigGuardNumber(value, at, bounds) : sourcingCurationConfigGuardReject(at, "value is not an integer");
export const sourcingCurationConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sourcingCurationConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sourcingCurationConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sourcingCurationConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSourcingCurationConfig(value: unknown, at = "$"): SourcingCurationConfig {
  const row = sourcingCurationConfigGuardObject(value, at);
  return {
    filters: parseFilters(row["filters"], `${at}.filters`),
    contributionsJson: sourcingCurationConfigGuardString(row["contributionsJson"], `${at}.contributionsJson`),
  };
}

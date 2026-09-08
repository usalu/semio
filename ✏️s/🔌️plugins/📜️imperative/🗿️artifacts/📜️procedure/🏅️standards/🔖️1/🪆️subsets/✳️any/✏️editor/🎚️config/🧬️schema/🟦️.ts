/** 🧬️ ImperativeConfig */
export interface ImperativeConfig {
  /** @state config */
  runOutputJson: string;
  /** @state config */
  /** @state config */
  contributionsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeConfigGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeConfigGuardRefusal(at, why);
};

type imperativeImperativeConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeConfigGuardReject(at, "value is not an object");
export const imperativeImperativeConfigGuardArray = (value: unknown, at: string, bounds: imperativeImperativeConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeConfigGuardString = (value: unknown, at: string, bounds: imperativeImperativeConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeConfigGuardReject(at, "value is not a boolean"));
export const imperativeImperativeConfigGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeConfigGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeConfigGuardNumber(value, at, bounds) : imperativeImperativeConfigGuardReject(at, "value is not an integer");
export const imperativeImperativeConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseImperativeConfig(value: unknown, at = "$"): ImperativeConfig {
  const row = imperativeImperativeConfigGuardObject(value, at);
  return {
    runOutputJson: imperativeImperativeConfigGuardString(row["runOutputJson"], `${at}.runOutputJson`),
    contributionsJson: imperativeImperativeConfigGuardString(row["contributionsJson"], `${at}.contributionsJson`),
  };
}

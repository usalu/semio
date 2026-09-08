/** 🧬️ HomeConfig */
export interface HomeConfig {
  /** @state config */
  activePanelTab: string;
  /** @state config */
  /** @state config */
  directoryJson: string;
  /** @state config */
  directorySessionBindingSha256: string;
  /** @state config */
  directoryAuthorizationGeneration: number;
  /** @state config */
  directoryReceiptSha256: string;
  /** @state config */
  clientId: string;
  /** @state config */
  clientName: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeConfigGuardReject = (at: string, why: string): never => {
  throw new spaceHomeConfigGuardRefusal(at, why);
};

type spaceHomeConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeConfigGuardReject(at, "value is not an object");
export const spaceHomeConfigGuardArray = (value: unknown, at: string, bounds: spaceHomeConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeConfigGuardString = (value: unknown, at: string, bounds: spaceHomeConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeConfigGuardReject(at, "value is not a boolean"));
export const spaceHomeConfigGuardNumber = (value: unknown, at: string, bounds: spaceHomeConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeConfigGuardInteger = (value: unknown, at: string, bounds: spaceHomeConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeConfigGuardNumber(value, at, bounds) : spaceHomeConfigGuardReject(at, "value is not an integer");
export const spaceHomeConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHomeConfig(value: unknown, at = "$"): HomeConfig {
  const row = spaceHomeConfigGuardObject(value, at);
  return {
    activePanelTab: spaceHomeConfigGuardString(row["activePanelTab"], `${at}.activePanelTab`),
    directoryJson: spaceHomeConfigGuardString(row["directoryJson"], `${at}.directoryJson`),
    directorySessionBindingSha256: spaceHomeConfigGuardString(row["directorySessionBindingSha256"], `${at}.directorySessionBindingSha256`),
    directoryAuthorizationGeneration: spaceHomeConfigGuardInteger(row["directoryAuthorizationGeneration"], `${at}.directoryAuthorizationGeneration`, {"minimum": 0, "maximum": 9007199254740991}),
    directoryReceiptSha256: spaceHomeConfigGuardString(row["directoryReceiptSha256"], `${at}.directoryReceiptSha256`),
    clientId: spaceHomeConfigGuardString(row["clientId"], `${at}.clientId`),
    clientName: spaceHomeConfigGuardString(row["clientName"], `${at}.clientName`),
  };
}

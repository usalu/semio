/** 🧬️ SpacePresence */
export interface SpacePresence {
  /** @state presence */
  camera: Record<string, SpaceWindowCamera>;
  /** @state presence */
  activeNodeId?: string;
  /** @state presence */
  focusedNodeId?: string;
  /** @state presence */
  collapsedNodeIds: string[];
  /** @state presence */
  previewOffNodeIds: string[];
}

export interface SpaceWindowCamera {
  x: number;
  y: number;
  zoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceSpacePresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceSpacePresenceGuardReject = (at: string, why: string): never => {
  throw new spaceSpacePresenceGuardRefusal(at, why);
};

type spaceSpacePresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceSpacePresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceSpacePresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceSpacePresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceSpacePresenceGuardReject(at, "value is not an object");
export const spaceSpacePresenceGuardArray = (value: unknown, at: string, bounds: spaceSpacePresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceSpacePresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceSpacePresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceSpacePresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceSpacePresenceGuardString = (value: unknown, at: string, bounds: spaceSpacePresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceSpacePresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceSpacePresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceSpacePresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceSpacePresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceSpacePresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceSpacePresenceGuardReject(at, "value is not a boolean"));
export const spaceSpacePresenceGuardNumber = (value: unknown, at: string, bounds: spaceSpacePresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceSpacePresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceSpacePresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceSpacePresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceSpacePresenceGuardInteger = (value: unknown, at: string, bounds: spaceSpacePresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceSpacePresenceGuardNumber(value, at, bounds) : spaceSpacePresenceGuardReject(at, "value is not an integer");
export const spaceSpacePresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceSpacePresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceSpacePresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceSpacePresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSpacePresence(value: unknown, at = "$"): SpacePresence {
  const row = spaceSpacePresenceGuardObject(value, at);
  return {
    camera: spaceSpacePresenceGuardObject(row["camera"], `${at}.camera`),
    activeNodeId: row["activeNodeId"] === undefined ? undefined : spaceSpacePresenceGuardString(row["activeNodeId"], `${at}.activeNodeId`),
    focusedNodeId: row["focusedNodeId"] === undefined ? undefined : spaceSpacePresenceGuardString(row["focusedNodeId"], `${at}.focusedNodeId`),
    collapsedNodeIds: spaceSpacePresenceGuardArray(row["collapsedNodeIds"], `${at}.collapsedNodeIds`).map((item, index) => spaceSpacePresenceGuardString(item, `${at}.collapsedNodeIds[${index}]`)),
    previewOffNodeIds: spaceSpacePresenceGuardArray(row["previewOffNodeIds"], `${at}.previewOffNodeIds`).map((item, index) => spaceSpacePresenceGuardString(item, `${at}.previewOffNodeIds[${index}]`)),
  };
}

export function parseSpaceWindowCamera(value: unknown, at = "$"): SpaceWindowCamera {
  const row = spaceSpacePresenceGuardObject(value, at);
  return {
    x: spaceSpacePresenceGuardNumber(row["x"], `${at}.x`),
    y: spaceSpacePresenceGuardNumber(row["y"], `${at}.y`),
    zoom: spaceSpacePresenceGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

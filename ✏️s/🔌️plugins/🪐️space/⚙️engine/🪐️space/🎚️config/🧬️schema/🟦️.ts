/** 🧬️ SpaceConfig */
export interface SpaceConfig {
  /** @state config */
  camera: Record<string, SpaceWindowCamera>;
  /** @state config */
  collapsedNodeIds: string[];
  /** @state config */
  previewOffNodeIds: string[];
  /** @state config */
  activeNodeId?: string;
  /** @state config */
  focusedNodeId?: string;
  /** @state config */
  clipboardNodeIds: string[];
  /** @state config */
  workflowEngagementInput: string;
  /** @state config */
  compiledDagEngagementInput: string;
  /** @state config */
  pendingImportNodeId?: string;
  /** @state config */
  pendingImportFormat?: string;
  /** @state config */
  activePanelTab: string;
  /** @state config */
  spaceId?: string;
  /** @state config */
  clientId?: string;
  /** @state config */
  clientName?: string;
  /** @state config */
}

export interface SpaceWindowCamera {
  x: number;
  y: number;
  zoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceSpaceConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceSpaceConfigGuardReject = (at: string, why: string): never => {
  throw new spaceSpaceConfigGuardRefusal(at, why);
};

type spaceSpaceConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceSpaceConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceSpaceConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceSpaceConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceSpaceConfigGuardReject(at, "value is not an object");
export const spaceSpaceConfigGuardArray = (value: unknown, at: string, bounds: spaceSpaceConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceSpaceConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceSpaceConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceSpaceConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceSpaceConfigGuardString = (value: unknown, at: string, bounds: spaceSpaceConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceSpaceConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceSpaceConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceSpaceConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceSpaceConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceSpaceConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceSpaceConfigGuardReject(at, "value is not a boolean"));
export const spaceSpaceConfigGuardNumber = (value: unknown, at: string, bounds: spaceSpaceConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceSpaceConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceSpaceConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceSpaceConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceSpaceConfigGuardInteger = (value: unknown, at: string, bounds: spaceSpaceConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceSpaceConfigGuardNumber(value, at, bounds) : spaceSpaceConfigGuardReject(at, "value is not an integer");
export const spaceSpaceConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceSpaceConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceSpaceConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceSpaceConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSpaceConfig(value: unknown, at = "$"): SpaceConfig {
  const row = spaceSpaceConfigGuardObject(value, at);
  return {
    camera: spaceSpaceConfigGuardObject(row["camera"], `${at}.camera`),
    collapsedNodeIds: spaceSpaceConfigGuardArray(row["collapsedNodeIds"], `${at}.collapsedNodeIds`).map((item, index) => spaceSpaceConfigGuardString(item, `${at}.collapsedNodeIds[${index}]`)),
    previewOffNodeIds: spaceSpaceConfigGuardArray(row["previewOffNodeIds"], `${at}.previewOffNodeIds`).map((item, index) => spaceSpaceConfigGuardString(item, `${at}.previewOffNodeIds[${index}]`)),
    activeNodeId: row["activeNodeId"] === undefined ? undefined : spaceSpaceConfigGuardString(row["activeNodeId"], `${at}.activeNodeId`),
    focusedNodeId: row["focusedNodeId"] === undefined ? undefined : spaceSpaceConfigGuardString(row["focusedNodeId"], `${at}.focusedNodeId`),
    clipboardNodeIds: spaceSpaceConfigGuardArray(row["clipboardNodeIds"], `${at}.clipboardNodeIds`).map((item, index) => spaceSpaceConfigGuardString(item, `${at}.clipboardNodeIds[${index}]`)),
    workflowEngagementInput: spaceSpaceConfigGuardString(row["workflowEngagementInput"], `${at}.workflowEngagementInput`),
    compiledDagEngagementInput: spaceSpaceConfigGuardString(row["compiledDagEngagementInput"], `${at}.compiledDagEngagementInput`),
    pendingImportNodeId: row["pendingImportNodeId"] === undefined ? undefined : spaceSpaceConfigGuardString(row["pendingImportNodeId"], `${at}.pendingImportNodeId`),
    pendingImportFormat: row["pendingImportFormat"] === undefined ? undefined : spaceSpaceConfigGuardString(row["pendingImportFormat"], `${at}.pendingImportFormat`),
    activePanelTab: spaceSpaceConfigGuardString(row["activePanelTab"], `${at}.activePanelTab`),
    spaceId: row["spaceId"] === undefined ? undefined : spaceSpaceConfigGuardString(row["spaceId"], `${at}.spaceId`),
    clientId: row["clientId"] === undefined ? undefined : spaceSpaceConfigGuardString(row["clientId"], `${at}.clientId`),
    clientName: row["clientName"] === undefined ? undefined : spaceSpaceConfigGuardString(row["clientName"], `${at}.clientName`),
  };
}

export function parseSpaceWindowCamera(value: unknown, at = "$"): SpaceWindowCamera {
  const row = spaceSpaceConfigGuardObject(value, at);
  return {
    x: spaceSpaceConfigGuardNumber(row["x"], `${at}.x`),
    y: spaceSpaceConfigGuardNumber(row["y"], `${at}.y`),
    zoom: spaceSpaceConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

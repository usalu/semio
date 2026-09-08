/** 🧬️ Puzzle2dConfig */
export type Puzzle2dFillLifecycle =
  | "idle"
  | "capturing"
  | "queued"
  | "running"
  | "checkpointReady"
  | "applying"
  | "awaitingAdoption"
  | "closing"
  | "completed"
  | "cancelled"
  | "faulted"
  | "discarded";

export interface Puzzle2dConfig {
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  lodModeByPane: Record<string, string>;
  /** @state config */
  engagementInputByPane: Record<string, string>;
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  brushCandidates: unknown[];
  /** @state config */
  brushCandidateSourceHandleId: string;
  /** @state config */
  fillCount: number;
  /** @state config */
  fillJobOperation: number;
  /** @state config */
  fillJobGeneration: number;
  /** @state config */
  fillJobSeed: number;
  /** @state config */
  fillJobBaseRevision: number;
  /** @state config */
  fillJobCheckpointSequence: number;
  /** @state config */
  fillJobAcceptedCount: number;
  /** @state config */
  fillJobSearchCount: number;
  /** @state config */
  fillJobStage: string;
  /** @state config */
  fillJobLifecycle: Puzzle2dFillLifecycle;
  /** @state config */
  fillJobFaultCode?: string;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  suggestionOffset: number;
  /** @state config */
  nodeKindWeights: Record<string, number>;
  /** @state config */
  handleKindWeights: Record<string, number>;
  /** @state config */
  activeUtilityByWindowId: Record<string, string>;
  /** @state config */
  /** @state config */
  /** @state config */
  exampleLoadGeneration: number;
  /** @state config */
  exampleLoadId?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dConfigGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dConfigGuardRefusal(at, why);
};

type puzzlePuzzle2dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dConfigGuardReject(at, "value is not an object");
export const puzzlePuzzle2dConfigGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dConfigGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dConfigGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dConfigGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dConfigGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dConfigGuardNumber(value, at, bounds) : puzzlePuzzle2dConfigGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dConfig(value: unknown, at = "$"): Puzzle2dConfig {
  const row = puzzlePuzzle2dConfigGuardObject(value, at);
  return {
    cameraX: puzzlePuzzle2dConfigGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: puzzlePuzzle2dConfigGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: puzzlePuzzle2dConfigGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    lodModeByPane: puzzlePuzzle2dConfigGuardObject(row["lodModeByPane"], `${at}.lodModeByPane`),
    engagementInputByPane: puzzlePuzzle2dConfigGuardObject(row["engagementInputByPane"], `${at}.engagementInputByPane`),
    brushCandidateIndex: puzzlePuzzle2dConfigGuardInteger(row["brushCandidateIndex"], `${at}.brushCandidateIndex`, {"minimum": 0}),
    brushCandidates: puzzlePuzzle2dConfigGuardArray(row["brushCandidates"], `${at}.brushCandidates`).map((item, index) => puzzlePuzzle2dConfigGuardObject(item, `${at}.brushCandidates[${index}]`)),
    brushCandidateSourceHandleId: puzzlePuzzle2dConfigGuardString(row["brushCandidateSourceHandleId"], `${at}.brushCandidateSourceHandleId`),
    fillCount: puzzlePuzzle2dConfigGuardInteger(row["fillCount"], `${at}.fillCount`, {"minimum": 0}),
    fillJobOperation: puzzlePuzzle2dConfigGuardInteger(row["fillJobOperation"], `${at}.fillJobOperation`, {"minimum": 0}),
    fillJobGeneration: puzzlePuzzle2dConfigGuardInteger(row["fillJobGeneration"], `${at}.fillJobGeneration`, {"minimum": 0}),
    fillJobSeed: puzzlePuzzle2dConfigGuardInteger(row["fillJobSeed"], `${at}.fillJobSeed`, {"minimum": 0}),
    fillJobBaseRevision: puzzlePuzzle2dConfigGuardInteger(row["fillJobBaseRevision"], `${at}.fillJobBaseRevision`, {"minimum": 0}),
    fillJobCheckpointSequence: puzzlePuzzle2dConfigGuardInteger(row["fillJobCheckpointSequence"], `${at}.fillJobCheckpointSequence`, {"minimum": 0}),
    fillJobAcceptedCount: puzzlePuzzle2dConfigGuardInteger(row["fillJobAcceptedCount"], `${at}.fillJobAcceptedCount`, {"minimum": 0}),
    fillJobSearchCount: puzzlePuzzle2dConfigGuardInteger(row["fillJobSearchCount"], `${at}.fillJobSearchCount`, {"minimum": 0}),
    fillJobStage: puzzlePuzzle2dConfigGuardString(row["fillJobStage"], `${at}.fillJobStage`),
    fillJobLifecycle: puzzlePuzzle2dConfigGuardMember(row["fillJobLifecycle"], `${at}.fillJobLifecycle`, ["idle", "capturing", "queued", "running", "checkpointReady", "applying", "awaitingAdoption", "closing", "completed", "cancelled", "faulted", "discarded"] as const),
    fillJobFaultCode: row["fillJobFaultCode"] === undefined ? undefined : puzzlePuzzle2dConfigGuardString(row["fillJobFaultCode"], `${at}.fillJobFaultCode`),
    gridSnapEnabled: puzzlePuzzle2dConfigGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridFactor: puzzlePuzzle2dConfigGuardNumber(row["gridFactor"], `${at}.gridFactor`),
    suggestionOffset: puzzlePuzzle2dConfigGuardNumber(row["suggestionOffset"], `${at}.suggestionOffset`),
    nodeKindWeights: puzzlePuzzle2dConfigGuardObject(row["nodeKindWeights"], `${at}.nodeKindWeights`),
    handleKindWeights: puzzlePuzzle2dConfigGuardObject(row["handleKindWeights"], `${at}.handleKindWeights`),
    activeUtilityByWindowId: puzzlePuzzle2dConfigGuardObject(row["activeUtilityByWindowId"], `${at}.activeUtilityByWindowId`),
    exampleLoadGeneration: puzzlePuzzle2dConfigGuardInteger(row["exampleLoadGeneration"], `${at}.exampleLoadGeneration`, {"minimum": 0}),
    exampleLoadId: row["exampleLoadId"] === undefined ? undefined : puzzlePuzzle2dConfigGuardString(row["exampleLoadId"], `${at}.exampleLoadId`),
  };
}

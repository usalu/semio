/** 💡️ Playbook inference schema — topology (step/block dependency order). */

export interface PlaybookTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface PlaybookInference {
  /** @derived */
  topology: PlaybookTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookInferenceGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookInferenceGuardRefusal(at, why);
};

type playbookPlaybookInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookInferenceGuardReject(at, "value is not an object");
export const playbookPlaybookInferenceGuardArray = (value: unknown, at: string, bounds: playbookPlaybookInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookInferenceGuardString = (value: unknown, at: string, bounds: playbookPlaybookInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookInferenceGuardReject(at, "value is not a boolean"));
export const playbookPlaybookInferenceGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookInferenceGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookInferenceGuardNumber(value, at, bounds) : playbookPlaybookInferenceGuardReject(at, "value is not an integer");
export const playbookPlaybookInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookInference(value: unknown, at = "$"): PlaybookInference {
  const row = playbookPlaybookInferenceGuardObject(value, at);
  return {
    topology: parsePlaybookTopology(row["topology"], `${at}.topology`),
  };
}

export function parsePlaybookTopology(value: unknown, at = "$"): PlaybookTopology {
  const row = playbookPlaybookInferenceGuardObject(value, at);
  return {
    topoOrder: playbookPlaybookInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => playbookPlaybookInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: playbookPlaybookInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: playbookPlaybookInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: playbookPlaybookInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}

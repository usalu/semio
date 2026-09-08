/** 🧬️ Playbook artifact schema — every field with its state class. */

export interface PlaybookArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  steps: PlaybookStep[];
  /** @state presence */
  selectedIds: string[];
  /** @state config */
  /** @state config */
  contributionsJson: string;
}

export interface PlaybookStep {
  id: string;
  title: string;
  description?: string;
  blocks: PlaybookBlock[];
}

export interface PlaybookBlock {
  id: string;
  label: string;
  kind: string;
  description?: string;
  required?: boolean;
  placeholder?: string;
  text?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookArtifactGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookArtifactGuardRefusal(at, why);
};

type playbookPlaybookArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookArtifactGuardReject(at, "value is not an object");
export const playbookPlaybookArtifactGuardArray = (value: unknown, at: string, bounds: playbookPlaybookArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookArtifactGuardString = (value: unknown, at: string, bounds: playbookPlaybookArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookArtifactGuardReject(at, "value is not a boolean"));
export const playbookPlaybookArtifactGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookArtifactGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookArtifactGuardNumber(value, at, bounds) : playbookPlaybookArtifactGuardReject(at, "value is not an integer");
export const playbookPlaybookArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookArtifact(value: unknown, at = "$"): PlaybookArtifact {
  const row = playbookPlaybookArtifactGuardObject(value, at);
  return {
    schema: playbookPlaybookArtifactGuardString(row["schema"], `${at}.schema`),
    id: playbookPlaybookArtifactGuardString(row["id"], `${at}.id`),
    version: playbookPlaybookArtifactGuardString(row["version"], `${at}.version`),
    title: row["title"] === undefined ? undefined : playbookPlaybookArtifactGuardString(row["title"], `${at}.title`),
    steps: playbookPlaybookArtifactGuardArray(row["steps"], `${at}.steps`).map((item, index) => parsePlaybookStep(item, `${at}.steps[${index}]`)),
    selectedIds: playbookPlaybookArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => playbookPlaybookArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
    contributionsJson: playbookPlaybookArtifactGuardString(row["contributionsJson"], `${at}.contributionsJson`),
  };
}

export function parsePlaybookStep(value: unknown, at = "$"): PlaybookStep {
  const row = playbookPlaybookArtifactGuardObject(value, at);
  return {
    id: playbookPlaybookArtifactGuardString(row["id"], `${at}.id`),
    title: playbookPlaybookArtifactGuardString(row["title"], `${at}.title`),
    description: row["description"] === undefined ? undefined : playbookPlaybookArtifactGuardString(row["description"], `${at}.description`),
    blocks: playbookPlaybookArtifactGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => parsePlaybookBlock(item, `${at}.blocks[${index}]`)),
  };
}

export function parsePlaybookBlock(value: unknown, at = "$"): PlaybookBlock {
  const row = playbookPlaybookArtifactGuardObject(value, at);
  return {
    id: playbookPlaybookArtifactGuardString(row["id"], `${at}.id`),
    label: playbookPlaybookArtifactGuardString(row["label"], `${at}.label`),
    kind: playbookPlaybookArtifactGuardString(row["kind"], `${at}.kind`),
  };
}

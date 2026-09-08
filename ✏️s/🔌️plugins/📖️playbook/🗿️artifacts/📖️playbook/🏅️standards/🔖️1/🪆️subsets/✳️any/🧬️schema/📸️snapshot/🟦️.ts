/** 🧬️ Playbook snapshot schema — artifact-lane fields only. */

export interface PlaybookSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  document: ArtifactChildHandle;
  /** @state artifact */
  flow: ArtifactChildHandle;
}

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookSnapshotGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookSnapshotGuardRefusal(at, why);
};

type playbookPlaybookSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookSnapshotGuardReject(at, "value is not an object");
export const playbookPlaybookSnapshotGuardArray = (value: unknown, at: string, bounds: playbookPlaybookSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookSnapshotGuardString = (value: unknown, at: string, bounds: playbookPlaybookSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookSnapshotGuardReject(at, "value is not a boolean"));
export const playbookPlaybookSnapshotGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookSnapshotGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookSnapshotGuardNumber(value, at, bounds) : playbookPlaybookSnapshotGuardReject(at, "value is not an integer");
export const playbookPlaybookSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookSnapshot(value: unknown, at = "$"): PlaybookSnapshot {
  const row = playbookPlaybookSnapshotGuardObject(value, at);
  return {
    schema: playbookPlaybookSnapshotGuardString(row["schema"], `${at}.schema`),
    value: row["value"] === undefined ? undefined : playbookPlaybookSnapshotGuardString(row["value"], `${at}.value`),
  };
}

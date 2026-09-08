/** 🧬️ Playbook diff schema — sparse field delta over the artifact. */

export interface PlaybookDiff {
  /** @state artifact */
  artifact?: PlaybookArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  version?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  document?: ArtifactChildHandle;
  /** @state artifact */
  flow?: ArtifactChildHandle;
  /** @state presence */
  selectedIds?: PlaybookStringList;
  /** @state config */
  /** @state config */
  contributionsJson?: string;
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

export interface PlaybookStringList {
  values: string[];
}

export interface PlaybookArtifact { [key: string]: unknown; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookDiffGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookDiffGuardRefusal(at, why);
};

type playbookPlaybookDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookDiffGuardReject(at, "value is not an object");
export const playbookPlaybookDiffGuardArray = (value: unknown, at: string, bounds: playbookPlaybookDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookDiffGuardString = (value: unknown, at: string, bounds: playbookPlaybookDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookDiffGuardReject(at, "value is not a boolean"));
export const playbookPlaybookDiffGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookDiffGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookDiffGuardNumber(value, at, bounds) : playbookPlaybookDiffGuardReject(at, "value is not an integer");
export const playbookPlaybookDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookDiff(value: unknown, at = "$"): PlaybookDiff {
  const row = playbookPlaybookDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : playbookPlaybookDiffGuardString(row["schema"], `${at}.schema`),
    value: row["value"] === undefined ? undefined : playbookPlaybookDiffGuardString(row["value"], `${at}.value`),
  };
}

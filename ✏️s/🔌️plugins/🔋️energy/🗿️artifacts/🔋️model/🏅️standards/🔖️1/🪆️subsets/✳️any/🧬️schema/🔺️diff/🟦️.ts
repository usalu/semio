/** 🧬️ EnergyModel diff schema — sparse field delta. */

/** 🔗️ A link slot delta; an absent field means the slot did not change at all. */
export type EnergyLinkSlotDelta = { readonly kind: "detached" } | { readonly kind: "attached"; readonly link: unknown };

export interface EnergyModelDiff {
  /** @state artifact */
  artifact?: EnergyModelArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  model?: unknown;
  structure?: unknown;
  zones?: unknown;
  referencedModel?: EnergyLinkSlotDelta;
  weatherLink?: EnergyLinkSlotDelta;
  /** @state artifact */
  resultsJson?: string;
}

export interface EnergyModelArtifact {
  schema: string;
  model: unknown;
  structure: unknown;
  zones: unknown;
  referencedModel?: unknown;
  weatherLink?: unknown;
  resultsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class energyModelDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const energyModelDiffGuardReject = (at: string, why: string): never => {
  throw new energyModelDiffGuardRefusal(at, why);
};

type energyModelDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type energyModelDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type energyModelDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const energyModelDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : energyModelDiffGuardReject(at, "value is not an object");
export const energyModelDiffGuardArray = (value: unknown, at: string, bounds: energyModelDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return energyModelDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) energyModelDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) energyModelDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const energyModelDiffGuardString = (value: unknown, at: string, bounds: energyModelDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return energyModelDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) energyModelDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) energyModelDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) energyModelDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const energyModelDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : energyModelDiffGuardReject(at, "value is not a boolean"));
export const energyModelDiffGuardNumber = (value: unknown, at: string, bounds: energyModelDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return energyModelDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) energyModelDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) energyModelDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const energyModelDiffGuardInteger = (value: unknown, at: string, bounds: energyModelDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? energyModelDiffGuardNumber(value, at, bounds) : energyModelDiffGuardReject(at, "value is not an integer");
export const energyModelDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : energyModelDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const energyModelDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : energyModelDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEnergyModelDiff(value: unknown, at = "$"): EnergyModelDiff {
  const row = energyModelDiffGuardObject(value, at);
  return {
    artifact: row["artifact"] === undefined ? undefined : parseEnergyModelArtifact(row["artifact"], `${at}.artifact`),
    schema: row["schema"] === undefined ? undefined : energyModelDiffGuardString(row["schema"], `${at}.schema`),
    model: row["model"] === undefined ? undefined : energyModelDiffGuardObject(row["model"], `${at}.model`),
    structure: row["structure"] === undefined ? undefined : energyModelDiffGuardObject(row["structure"], `${at}.structure`),
    zones: row["zones"] === undefined ? undefined : energyModelDiffGuardObject(row["zones"], `${at}.zones`),
    referencedModel: row["referencedModel"] === undefined ? undefined : parseEnergyLinkSlotDelta(row["referencedModel"], `${at}.referencedModel`),
    weatherLink: row["weatherLink"] === undefined ? undefined : parseEnergyLinkSlotDelta(row["weatherLink"], `${at}.weatherLink`),
    resultsJson: row["resultsJson"] === undefined ? undefined : energyModelDiffGuardString(row["resultsJson"], `${at}.resultsJson`),
  };
}

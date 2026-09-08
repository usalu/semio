/** 💡️ Bcf inference schema — topic/comment/viewpoint/author counts derived from the topic tree. */

export interface BcfTopicStats {
  topicCount: number;
  commentCount: number;
  viewpointCount: number;
  authorCount: number;
}

export interface BcfInference {
  /** @derived */
  topicStats: BcfTopicStats;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBcf21MarkupInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBcf21MarkupInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioBcf21MarkupInferenceGuardRefusal(at, why);
};

type stdioBcf21MarkupInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBcf21MarkupInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBcf21MarkupInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBcf21MarkupInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBcf21MarkupInferenceGuardReject(at, "value is not an object");
export const stdioBcf21MarkupInferenceGuardArray = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBcf21MarkupInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBcf21MarkupInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBcf21MarkupInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBcf21MarkupInferenceGuardString = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBcf21MarkupInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBcf21MarkupInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBcf21MarkupInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBcf21MarkupInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBcf21MarkupInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBcf21MarkupInferenceGuardReject(at, "value is not a boolean"));
export const stdioBcf21MarkupInferenceGuardNumber = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBcf21MarkupInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBcf21MarkupInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBcf21MarkupInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBcf21MarkupInferenceGuardInteger = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBcf21MarkupInferenceGuardNumber(value, at, bounds) : stdioBcf21MarkupInferenceGuardReject(at, "value is not an integer");
export const stdioBcf21MarkupInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBcf21MarkupInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBcf21MarkupInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBcf21MarkupInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBcfInference(value: unknown, at = "$"): BcfInference {
  const row = stdioBcf21MarkupInferenceGuardObject(value, at);
  return {
    topicStats: parseBcfTopicStats(row["topicStats"], `${at}.topicStats`),
  };
}

export function parseBcfTopicStats(value: unknown, at = "$"): BcfTopicStats {
  const row = stdioBcf21MarkupInferenceGuardObject(value, at);
  return {
    topicCount: stdioBcf21MarkupInferenceGuardInteger(row["topicCount"], `${at}.topicCount`, {"minimum": 0}),
    commentCount: stdioBcf21MarkupInferenceGuardInteger(row["commentCount"], `${at}.commentCount`, {"minimum": 0}),
    viewpointCount: stdioBcf21MarkupInferenceGuardInteger(row["viewpointCount"], `${at}.viewpointCount`, {"minimum": 0}),
    authorCount: stdioBcf21MarkupInferenceGuardInteger(row["authorCount"], `${at}.authorCount`, {"minimum": 0}),
  };
}

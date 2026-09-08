/** 🔺️ SemioAnimationDiff schema — real mirror of `🦀️.rs` (the source of truth). No
 * `snapshot: SemioAnimationSnapshot` full-replace slot anywhere. Collections
 * (timelines/channels/keyframes) are index-keyed removed/modified/added triples
 * (`engine::triples::IndexedTripleDiff<D,T>`), kept generic here since TS supports it. */
import type { AnimTarget, AnimInterpolation, AnimValue, AnimTimeline, AnimChannel, AnimKeyframe } from "../📸️snapshot/🟦️";

export interface IndexModified<D> { index: number; diff: D; }
export interface IndexAdded<T> { index: number; item: T; }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: IndexModified<D>[]; added: IndexAdded<T>[]; }

export interface AnimKeyframeDiff {
  t?: number;
  value?: AnimValue;
}

export interface AnimChannelDiff {
  target?: AnimTarget;
  interpolation?: AnimInterpolation;
  keyframes?: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe>;
}

export interface AnimTimelineDiff {
  /** tri-state: absent = unchanged, null = name cleared, string = renamed */
  name?: string | null;
  channels?: IndexedTripleDiff<AnimChannelDiff, AnimChannel>;
}

export interface SemioAnimationDiff {
  timelines?: IndexedTripleDiff<AnimTimelineDiff, AnimTimeline>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AnimationDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AnimationDiffGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AnimationDiffGuardRefusal(at, why);
};

type stdioSemioV1AnimationDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AnimationDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AnimationDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AnimationDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AnimationDiffGuardReject(at, "value is not an object");
export const stdioSemioV1AnimationDiffGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AnimationDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AnimationDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AnimationDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AnimationDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AnimationDiffGuardString = (value: unknown, at: string, bounds: stdioSemioV1AnimationDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AnimationDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AnimationDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AnimationDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AnimationDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AnimationDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AnimationDiffGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AnimationDiffGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AnimationDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AnimationDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AnimationDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AnimationDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AnimationDiffGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AnimationDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AnimationDiffGuardNumber(value, at, bounds) : stdioSemioV1AnimationDiffGuardReject(at, "value is not an integer");
export const stdioSemioV1AnimationDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AnimationDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AnimationDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AnimationDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioAnimationDiff(value: unknown, at = "$"): SemioAnimationDiff {
  const row = stdioSemioV1AnimationDiffGuardObject(value, at);
  return {
    timelines: row["timelines"] === undefined ? undefined : stdioSemioV1AnimationDiffGuardObject(row["timelines"], `${at}.timelines`),
  };
}

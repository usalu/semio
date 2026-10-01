import type { UiDocumentLimits, UiNumberLimits, UiNumberScale } from "@semio-tech/framework";
import { uiNumberCrossedBound } from "../🧩️component/🟦️.ts";

/** 🛡️ Cross-renderer document admission limits shared by browser and native UI hosts. */
export const DEFAULT_UI_DOCUMENT_LIMITS = {
  maxNodes: 20_000,
  maxDepth: 128,
  maxChildren: 4_096,
  maxTextBytes: 65_536,
  maxPatchOps: 4_096,
  maxPatchBytes: 1_048_576,
} satisfies UiDocumentLimits;

/** 🧲️ The detent law: finite, strictly ascending, inside `min..=max` — the twin of the Rust admission gate `snaps_are_valid`. */
export function snapsAreValid(snaps: readonly number[], min: number, max: number): boolean {
  let previous = Number.NEGATIVE_INFINITY;
  return snaps.every((snap) => {
    const valid = Number.isFinite(snap) && snap > previous && snap >= min && snap <= max;
    previous = snap;
    return valid;
  });
}

/** 🎚️ The number-range law: a positive display factor, a strictly positive travel on a log scale, declared limits ordered (equal only when both include the bound) and admitting the key range and every detent — the twin of the Rust admission gate `number_range_is_valid`. */
export function numberRangeIsValid(min: number | null, max: number | null, scale: UiNumberScale, displayFactor: number | null, limits: UiNumberLimits | null, snaps: readonly number[]): boolean {
  const factor = displayFactor == null || displayFactor > 0;
  const axis = scale === "linear" || (min != null && max != null && min > 0 && max > 0);
  if (!limits) return factor && axis;
  const low = limits.min;
  const high = limits.max;
  const ordered = !low || !high || low.value < high.value || (low.value === high.value && !low.exclusive && !high.exclusive);
  const admitted = (value: number): boolean => uiNumberCrossedBound(value, null, null, limits) === null;
  return factor && axis && ordered && (min == null || admitted(min)) && (max == null || admitted(max)) && snaps.every(admitted);
}

/** 🔢️ The limit values of a numeric control's hard range, for the finiteness gate. */
export function numberLimitValues(limits: UiNumberLimits | null | undefined): number[] {
  return [limits?.min?.value, limits?.max?.value].filter((value): value is number => value !== undefined);
}

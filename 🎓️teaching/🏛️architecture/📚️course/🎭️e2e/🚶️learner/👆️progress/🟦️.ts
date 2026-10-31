/** 👆️ The directed content progress made by a real navigation gesture. */
export interface SwipeScrollProgress { readonly position: number; readonly remaining: number; }

/** 🧭️ One gesture's observed page and actual directed scroll progress. */
export interface SwipeNavigationObservation {
  readonly from: string;
  readonly next: string;
  readonly observed: string | null;
  readonly before: SwipeScrollProgress;
  readonly after: SwipeScrollProgress;
}

/** 🧮️ Bounds scrolling gestures by the measured content range, plus its final edge gesture. */
export function swipeAttemptLimit(range: number): number {
  const count = Math.ceil(range) + 1;
  if (!Number.isFinite(range) || range < 0 || !Number.isSafeInteger(count)) throw new RangeError("Bounded swipe scroll range required");
  return count;
}

/** 🧭️ Accepts the destination or monotonic content progress; a stalled gesture cannot authorize another. */
export function swipeNavigationProgress(value: SwipeNavigationObservation): "advance" | "scroll" | "pending" {
  if ([value.before.position, value.before.remaining, value.after.position, value.after.remaining].some(number => !Number.isFinite(number) || number < 0)) throw new RangeError("Actual directed swipe progress required");
  if (value.observed === value.next) return "advance";
  if (value.observed === null) return "pending";
  if (value.observed !== value.from) throw new Error("Swipe reached an unexpected page");
  const minimum = Math.min(1, value.before.remaining);
  return minimum > 0 && value.after.position - value.before.position >= minimum && value.before.remaining - value.after.remaining >= minimum ? "scroll" : "pending";
}

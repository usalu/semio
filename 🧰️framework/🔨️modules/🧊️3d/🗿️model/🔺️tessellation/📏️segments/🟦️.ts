/** 📏️ Segment counts of a full circle from the chord tolerance: the sagitta `r (1 - cos(pi / n))` stays within `chordM`, clamped to the quality bounds.
 * @see ../../🧬️schema/🟦️.ts#Quality */
import type { Quality } from "../../🧬️schema/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

export function circleSegments(radius: number, quality: Quality): number {
  const r = Math.max(radius, 1e-9);
  const ratio = Math.min(1, quality.chordM / r);
  const n = ratio >= 1 ? quality.minSegments : Math.ceil(Math.PI / portable.acos(1 - ratio));
  return Math.max(quality.minSegments, Math.min(quality.maxSegments, n));
}

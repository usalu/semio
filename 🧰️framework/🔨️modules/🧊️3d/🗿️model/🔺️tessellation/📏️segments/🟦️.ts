/** 📏️ Segment counts of a full circle from the chord tolerance: the sagitta `r (1 - cos(pi / n))` stays within `chordM`, a circle whose radius exceeds `round.radiusM` never falls below `round.segments`, and the result is clamped to the quality bounds.
 * @see ../../🧬️schema/🟦️.ts#Quality */
import type { Quality } from "../../🧬️schema/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

export function circleSegments(radius: number, quality: Quality): number {
  const r = Math.max(radius, 1e-9);
  const ratio = Math.min(1, quality.chordM / r);
  const chord = ratio >= 1 ? quality.minSegments : Math.ceil(Math.PI / portable.acos(1 - ratio));
  const n = quality.round !== undefined && r > quality.round.radiusM ? Math.max(chord, quality.round.segments) : chord;
  return Math.max(quality.minSegments, Math.min(quality.maxSegments, n));
}

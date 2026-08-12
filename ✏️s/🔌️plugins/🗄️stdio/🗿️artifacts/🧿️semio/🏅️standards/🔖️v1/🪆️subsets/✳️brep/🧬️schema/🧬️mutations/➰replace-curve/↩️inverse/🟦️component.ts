/** ↩️ inverse for `ReplaceCurve` — undoes to another `ReplaceCurve` restoring the prior curve. */
export interface ReplaceCurveInverseReplaceCurve {
  edgeId: string;
  newCurve: unknown;
}

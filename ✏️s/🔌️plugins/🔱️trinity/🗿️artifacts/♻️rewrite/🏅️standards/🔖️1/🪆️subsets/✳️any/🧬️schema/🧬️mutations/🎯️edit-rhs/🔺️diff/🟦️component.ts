/** 🔺️ rewrite edit-rhs/🔺️diff — mirror of the single-field diff builder. */
import type { EditRhs } from "../🟦️component.ts";

export function diff(payload: EditRhs): { rhsJson: string } {
  return { rhsJson: payload.newRhsJson };
}

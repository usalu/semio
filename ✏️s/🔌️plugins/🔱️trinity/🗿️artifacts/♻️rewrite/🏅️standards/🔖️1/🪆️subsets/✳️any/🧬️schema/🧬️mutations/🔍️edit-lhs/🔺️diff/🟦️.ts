/** 🔺️ rewrite edit-lhs/🔺️diff — mirror of the single-field diff builder. */
import type { EditLhs } from "../🟦️.ts";

export function diff(payload: EditLhs): { lhsJson: string } {
  return { lhsJson: payload.newLhsJson };
}

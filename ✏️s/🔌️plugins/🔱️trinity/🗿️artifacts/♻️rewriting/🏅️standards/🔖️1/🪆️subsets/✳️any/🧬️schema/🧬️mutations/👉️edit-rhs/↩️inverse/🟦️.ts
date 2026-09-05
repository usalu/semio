/** ↩️ rewriting edit-rhs/↩️inverse — mirror of the BASE-lookup old-body inverse builder. */
import type { EditRhs } from "../🟦️.ts";

export function inverse(_payload: EditRhs, baseRhsJson: string): EditRhs[] {
  return [{ newRhsJson: baseRhsJson }];
}

/** ↩️ rewrite edit-lhs/↩️inverse — mirror of the BASE-lookup old-body inverse builder. */
import type { EditLhs } from "../🦠️mutation/🟦️component.ts";

export function inverse(_payload: EditLhs, baseLhsJson: string): EditLhs[] {
  return [{ newLhsJson: baseLhsJson }];
}

/** 🔺️ sourcing curate delete-curated-item/🔺️diff — mirror of the curated-item removal sparse diff
 * builder. */
import type { DeleteCuratedItem } from "../🟦️component.ts";
import type { CurateCuratedDelta } from "../../../🔺️diff/🟦️component.ts";

export function diff(payload: DeleteCuratedItem): { curated: CurateCuratedDelta } {
  return { curated: { removed: [payload.objectId] } };
}

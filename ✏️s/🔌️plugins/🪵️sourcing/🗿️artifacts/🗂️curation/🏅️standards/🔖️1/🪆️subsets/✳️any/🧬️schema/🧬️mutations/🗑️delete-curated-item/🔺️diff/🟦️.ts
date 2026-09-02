/** 🔺️ sourcing curation delete-curated-item/🔺️diff — mirror of the curated-item removal sparse diff
 * builder. */
import type { DeleteCuratedItem } from "../🟦️.ts";
import type { CurationCuratedDelta } from "../../../🔺️diff/🟦️.ts";

export function diff(payload: DeleteCuratedItem): { curated: CurationCuratedDelta } {
  return { curated: { removed: [payload.objectId] } };
}

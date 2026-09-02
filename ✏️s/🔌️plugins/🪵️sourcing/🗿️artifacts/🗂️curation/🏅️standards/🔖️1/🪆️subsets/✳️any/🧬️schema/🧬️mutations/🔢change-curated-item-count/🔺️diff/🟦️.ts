/** 🔺️ sourcing curation change-curated-item-count/🔺️diff — mirror of the single-field count patch
 * delta builder. */
import type { ChangeCuratedItemCount } from "../🟦️.ts";
import type { CurationCuratedDelta } from "../../../🔺️diff/🟦️.ts";

export function diff(payload: ChangeCuratedItemCount): { curated: CurationCuratedDelta } {
  return { curated: { patched: [{ objectId: payload.objectId, count: payload.newCount }] } };
}

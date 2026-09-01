/** 🔺️ sourcing curate change-curated-item-count/🔺️diff — mirror of the single-field count patch
 * delta builder. */
import type { ChangeCuratedItemCount } from "../🟦️component.ts";
import type { CurateCuratedDelta } from "../../../🔺️diff/🟦️component.ts";

export function diff(payload: ChangeCuratedItemCount): { curated: CurateCuratedDelta } {
  return { curated: { patched: [{ objectId: payload.objectId, count: payload.newCount }] } };
}

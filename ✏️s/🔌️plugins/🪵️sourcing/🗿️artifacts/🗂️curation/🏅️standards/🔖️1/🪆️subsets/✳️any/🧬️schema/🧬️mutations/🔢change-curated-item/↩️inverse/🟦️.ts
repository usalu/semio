/** ↩️ sourcing curation change-curated-item-count/↩️inverse — mirror of the BASE-lookup old-count
 * restore inverse. */
import type { ChangeCuratedItemCount } from "../🟦️.ts";

export function inverse(payload: ChangeCuratedItemCount, baseCount: number | undefined): ChangeCuratedItemCount[] {
  return baseCount === undefined ? [] : [{ objectId: payload.objectId, newCount: baseCount }];
}

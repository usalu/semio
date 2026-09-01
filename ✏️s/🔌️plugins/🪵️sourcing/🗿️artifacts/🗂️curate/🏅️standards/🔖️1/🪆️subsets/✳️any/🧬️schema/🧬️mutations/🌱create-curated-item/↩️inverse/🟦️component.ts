/** ↩️ sourcing curate create-curated-item/↩️inverse — mirror of the id-only delete-curated-item
 * inverse. */
import type { CreateCuratedItem } from "../🟦️component.ts";
import type { DeleteCuratedItem } from "../../🗑️delete-curated-item/🟦️component.ts";

export function inverse(payload: CreateCuratedItem): [DeleteCuratedItem] {
  return [{ objectId: payload.item.objectId }];
}

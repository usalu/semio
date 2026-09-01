/** ↩️ sourcing curate delete-curated-item/↩️inverse — mirror of the BASE-lookup recreate
 * curated-item inverse. */
import type { DeleteCuratedItem } from "../🟦️component.ts";
import type { CreateCuratedItem } from "../../🌱create-curated-item/🟦️component.ts";
import type { CuratedItem } from "../../../📸️snapshot/🟦️component.ts";

export function inverse(_payload: DeleteCuratedItem, baseItem: CuratedItem | undefined): CreateCuratedItem[] {
  return baseItem ? [{ item: baseItem }] : [];
}

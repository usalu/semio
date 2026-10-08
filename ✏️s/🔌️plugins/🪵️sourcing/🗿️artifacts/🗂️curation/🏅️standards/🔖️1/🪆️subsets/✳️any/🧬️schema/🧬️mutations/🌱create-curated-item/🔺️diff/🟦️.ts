/** 🔺️ sourcing curation create-curated-item/🔺️diff — mirror of the append-only curated-item insert
 * delta builder. */
import type { CreateCuratedItem } from "../🟦️.ts";
import type { CurationCuratedDelta } from "../../../🔺️diff/🟦️.ts";

export function diff(payload: CreateCuratedItem, base: { curated: { objectId: string }[] }): { curated: CurationCuratedDelta } {
  return { curated: { inserted: [{ index: Math.min(payload.index ?? base.curated.length, base.curated.length), row: payload.item }] } };
}

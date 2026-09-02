/** 🔺️ sourcing curation create-curated-item/🔺️diff — mirror of the append-only curated-item insert
 * delta builder. */
import type { CreateCuratedItem } from "../🟦️.ts";
import type { CurationCuratedDelta } from "../../../🔺️diff/🟦️.ts";

export function diff(payload: CreateCuratedItem): { curated: CurationCuratedDelta } {
  return { curated: { added: [payload.item] } };
}

/** 🔺️ sourcing curate create-curated-item/🔺️diff — mirror of the append-only curated-item insert
 * delta builder. */
import type { CreateCuratedItem } from "../🟦️component.ts";
import type { CurateCuratedDelta } from "../../../🔺️diff/🟦️component.ts";

export function diff(payload: CreateCuratedItem): { curated: CurateCuratedDelta } {
  return { curated: { added: [payload.item] } };
}

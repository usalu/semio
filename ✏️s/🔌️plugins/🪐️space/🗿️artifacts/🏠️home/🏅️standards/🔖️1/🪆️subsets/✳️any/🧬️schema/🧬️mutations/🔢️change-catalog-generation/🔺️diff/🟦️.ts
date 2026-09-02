/** 🔺️ home change-catalog-generation/🔺️diff — mirror of the sparse `SHomeDiff` builder. */
import type { ChangeCatalogGeneration } from "../🟦️.ts";
import type { SHomeDiff } from "../../../🔺️diff/🟦️.ts";

export function diff(payload: ChangeCatalogGeneration): Partial<SHomeDiff> {
  return { catalogGeneration: payload.newCatalogGeneration };
}

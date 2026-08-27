/** 🔺️ home change-catalog-generation/🔺️diff — mirror of the sparse `SHomeDiff` builder. */
import type { ChangeCatalogGeneration } from "../🟦️component.ts";
import type { SHomeDiff } from "../../../🔺️diff/🟦️component.ts";

export function diff(payload: ChangeCatalogGeneration): Partial<SHomeDiff> {
  return { catalogGeneration: payload.newCatalogGeneration };
}

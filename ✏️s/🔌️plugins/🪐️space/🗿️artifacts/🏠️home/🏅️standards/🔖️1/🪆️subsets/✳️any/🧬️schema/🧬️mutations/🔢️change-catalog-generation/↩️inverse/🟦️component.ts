/** ↩️ home change-catalog-generation/↩️inverse — mirror of the BASE-lookup inverse builder. */
import type { ChangeCatalogGeneration } from "../🦠️mutation/🟦️component.ts";

export function inverse(_payload: ChangeCatalogGeneration, base: { catalogGeneration: number }): ChangeCatalogGeneration[] {
  return [{ newCatalogGeneration: base.catalogGeneration }];
}

/** 🗂️ Sourcing curate direct mutation aggregate. */
import type { CreateCuratedItem } from "./🌱create-curated-item/🟦️component.ts";
import type { DeleteCuratedItem } from "./🗑️delete-curated-item/🟦️component.ts";
import type { ChangeCuratedItemCount } from "./🔢change-curated-item-count/🟦️component.ts";

export type SourcingMutation =
  | ({ mutation: "createCuratedItem" } & CreateCuratedItem)
  | ({ mutation: "deleteCuratedItem" } & DeleteCuratedItem)
  | ({ mutation: "changeCuratedItemCount" } & ChangeCuratedItemCount);

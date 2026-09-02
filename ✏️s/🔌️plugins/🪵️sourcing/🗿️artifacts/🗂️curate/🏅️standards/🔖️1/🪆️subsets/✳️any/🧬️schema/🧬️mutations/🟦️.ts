/** 🗂️ Sourcing curate direct mutation aggregate. */
import type { CreateCuratedItem } from "./🌱create-curated-item/🟦️.ts";
import type { DeleteCuratedItem } from "./🗑️delete-curated-item/🟦️.ts";
import type { ChangeCuratedItemCount } from "./🔢change-curated-item-count/🟦️.ts";

export type SourcingMutation =
  | ({ mutation: "createCuratedItem" } & CreateCuratedItem)
  | ({ mutation: "deleteCuratedItem" } & DeleteCuratedItem)
  | ({ mutation: "changeCuratedItemCount" } & ChangeCuratedItemCount);

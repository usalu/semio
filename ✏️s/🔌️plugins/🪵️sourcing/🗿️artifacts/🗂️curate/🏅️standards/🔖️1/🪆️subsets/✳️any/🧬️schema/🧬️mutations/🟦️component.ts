/** 🧬️ Sourcing curate document mutations — the closed semantic vocabulary derived from
 * `CurateSnapshot.curated`'s id-keyed shape. `stock` is a bulk-populated reference catalogue and is
 * not represented here (whole-catalogue population goes through a non-history document reset).
 */
export interface CuratedItem {
  objectId: string;
  count: number;
}

export interface CreateCuratedItem {
  mutation: "createCuratedItem";
  item: CuratedItem;
}

export interface DeleteCuratedItem {
  mutation: "deleteCuratedItem";
  objectId: string;
}

export interface ChangeCuratedItemCount {
  mutation: "changeCuratedItemCount";
  objectId: string;
  newCount: number;
}

export type SourcingMutation = CreateCuratedItem | DeleteCuratedItem | ChangeCuratedItemCount;

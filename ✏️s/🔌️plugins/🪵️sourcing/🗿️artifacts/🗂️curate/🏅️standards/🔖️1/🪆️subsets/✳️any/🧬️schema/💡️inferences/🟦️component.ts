/** 💡️ Curate inference schema — a real census over the stock catalog and curated bill of quantities. */

export interface CurateEntries {
  stockCount: number;
  entryCount: number;
  totalCount: number;
}

export interface CurateInference {
  /** @state inferred */
  entries: CurateEntries;
}

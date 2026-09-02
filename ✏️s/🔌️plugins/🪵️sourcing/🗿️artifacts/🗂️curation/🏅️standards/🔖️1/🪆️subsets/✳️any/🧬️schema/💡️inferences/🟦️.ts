/** 💡️ Curation inference schema — a real census over the stock catalog and curated bill of quantities. */

export interface CurationEntries {
  stockCount: number;
  entryCount: number;
  totalCount: number;
}

export interface CurationInference {
  /** @derived */
  entries: CurationEntries;
}

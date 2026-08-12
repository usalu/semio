/** 💡️ Tsv inference schema — document outline (record/column counts). */

export interface TsvOutline {
  recordCount: number;
  columnCount: number;
}

export interface TsvInference {
  /** @state inferred */
  outline: TsvOutline;
}

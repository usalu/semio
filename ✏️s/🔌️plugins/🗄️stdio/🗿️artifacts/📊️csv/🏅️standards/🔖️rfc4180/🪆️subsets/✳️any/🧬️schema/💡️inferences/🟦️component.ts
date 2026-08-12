/** 💡️ Csv inference schema — document outline (record/column counts + hasHeader). */

export interface CsvOutline {
  recordCount: number;
  columnCount: number;
  hasHeader: boolean;
}

export interface CsvInference {
  /** @state inferred */
  outline: CsvOutline;
}

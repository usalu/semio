/** 💡️ Xlsx inference schema — document outline (sheet names/count, cell count). */

export interface XlsxOutline {
  sheetNames: string[];
  sheetCount: number;
  cellCount: number;
}

export interface XlsxInference {
  /** @derived */
  outline: XlsxOutline;
}

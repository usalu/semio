/** 🧾 `outline` — one named inference: this RFC 4180 table's own row/column structure. */

export interface CsvOutline {
  recordCount: number;
  columnCount: number;
  hasHeader: boolean;
}

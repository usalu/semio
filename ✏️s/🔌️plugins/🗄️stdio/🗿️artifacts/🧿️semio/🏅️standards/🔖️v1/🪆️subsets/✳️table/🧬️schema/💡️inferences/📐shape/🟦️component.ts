/** 📐 `shape` — the semio table's own dimensions + declared column-kind census. */

export interface SemioTableShape {
  columnCount: number;
  rowCount: number;
  nullColumnCount: number;
  boolColumnCount: number;
  intColumnCount: number;
  floatColumnCount: number;
  strColumnCount: number;
  bytesColumnCount: number;
}

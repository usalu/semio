/** ↩️ inverse for `DeleteColumn` — a `CreateColumn` at the original index, plus one `EditCell` per
 * row to restore its original cell value (a bare re-create only fills Null). */
export interface DeleteColumnInverseCreateColumn {
  name: string;
  kind: "null" | "bool" | "int" | "float" | "str" | "bytes";
  index: number;
}
export interface DeleteColumnInverseEditCell {
  rowIndex: number;
  columnName: string;
  newValue: unknown;
}

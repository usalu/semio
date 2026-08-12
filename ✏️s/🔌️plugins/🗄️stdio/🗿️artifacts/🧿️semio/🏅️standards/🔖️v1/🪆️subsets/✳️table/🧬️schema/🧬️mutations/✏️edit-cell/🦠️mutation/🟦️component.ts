/** mutation payload — mirrors `EditCell`. */
export interface EditCell {
  rowIndex: number;
  columnName: string;
  newValue: unknown;
}

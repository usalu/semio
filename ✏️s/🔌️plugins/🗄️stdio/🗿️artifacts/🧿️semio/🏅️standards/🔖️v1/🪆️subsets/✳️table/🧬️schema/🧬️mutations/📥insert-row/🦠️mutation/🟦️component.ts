/** mutation payload — mirrors `InsertRow`. */
export interface InsertRow {
  index: number;
  row: { cells: unknown[] };
}
